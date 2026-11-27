// Blizzard: a local study tutor. The AI engine runs hidden in the background,
// and the window is the only thing you see.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use futures_util::StreamExt;
use serde::{Deserialize, Serialize};
use std::{
    fs,
    io::Cursor,
    path::PathBuf,
    process::{Child, Command, Stdio},
    sync::Mutex,
    time::Duration,
};
use tauri::{AppHandle, Emitter, Manager};

const DEFAULT_MODEL: &str = "LiquidAI/LFM3-Omni-2B-GGUF";
const PORT: u16 = 8787;

struct Engine(Mutex<Option<Child>>);

#[derive(Serialize, Clone)]
struct Status {
    state: String,
    message: String,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(default)]
struct Settings {
    tavily_key: String,
    track_understanding: bool,
    model_repo: String,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            tavily_key: String::new(),
            track_understanding: true,
            model_repo: DEFAULT_MODEL.into(),
        }
    }
}

#[derive(Deserialize)]
struct Msg {
    role: String,
    content: String,
}

fn data_dir(app: &AppHandle) -> PathBuf {
    let dir = app.path().app_data_dir().expect("no app data folder");
    fs::create_dir_all(&dir).ok();
    dir
}

fn emit_status(app: &AppHandle, state: &str, message: &str) {
    let _ = app.emit(
        "status",
        Status { state: state.into(), message: message.into() },
    );
}

fn read_settings(app: &AppHandle) -> Settings {
    fs::read_to_string(data_dir(app).join("settings.json"))
        .ok()
        .and_then(|s| serde_json::from_str(&s).ok())
        .unwrap_or_default()
}

fn engine_name() -> &'static str {
    if cfg!(windows) { "llama-server.exe" } else { "llama-server" }
}

async fn engine_healthy() -> bool {
    reqwest::get(format!("http://127.0.0.1:{PORT}/health"))
        .await
        .map(|r| r.status().is_success())
        .unwrap_or(false)
}

/// Downloads the official llama.cpp build for this system once, into the app folder.
/// Some releases ship .zip files and others ship .tar.gz, so both are handled.
async fn install_engine(app: &AppHandle) -> Result<PathBuf, String> {
    let dir = data_dir(app).join("engine");
    let exe = dir.join(engine_name());
    if exe.exists() {
        return Ok(exe);
    }
    emit_status(app, "setup", "Installing the AI engine (first run only)");

    let wanted = if cfg!(target_os = "windows") {
        "win-cpu-x64"
    } else if cfg!(target_os = "macos") {
        if cfg!(target_arch = "aarch64") { "macos-arm64" } else { "macos-x64" }
    } else {
        "ubuntu-x64"
    };

    let client = reqwest::Client::builder()
        .user_agent("blizzard")
        .build()
        .map_err(|e| e.to_string())?;
    // The "latest" release is not always the one with binaries, so search the recent releases.
    let releases: Vec<serde_json::Value> = client
        .get("https://api.github.com/repos/ggml-org/llama.cpp/releases?per_page=15")
        .send()
        .await
        .map_err(|e| e.to_string())?
        .json()
        .await
        .map_err(|e| e.to_string())?;

    let mut chosen: Option<serde_json::Value> = None;
    let mut seen: Vec<String> = Vec::new();
    for rel in &releases {
        let assets = rel["assets"].as_array().cloned().unwrap_or_default();
        seen.extend(assets.iter().filter_map(|a| a["name"].as_str().map(String::from)));
        let found = assets.iter().find(|a| {
            a["name"].as_str().map_or(false, |n| {
                n.contains(wanted) && (n.ends_with(".zip") || n.ends_with(".tar.gz"))
            })
        });
        if let Some(a) = found {
            chosen = Some(a.clone());
            break;
        }
    }
    let chosen = chosen.ok_or_else(|| {
        format!("No llama.cpp build for {wanted}. Files found: {}", seen.join(", "))
    })?;

    let archive_name = chosen["name"].as_str().unwrap_or_default().to_string();
    let url = chosen["browser_download_url"]
        .as_str()
        .ok_or("The download link is missing")?
        .to_string();

    let bytes = client
        .get(&url)
        .send()
        .await
        .map_err(|e| e.to_string())?
        .bytes()
        .await
        .map_err(|e| e.to_string())?
        .to_vec();

    fs::create_dir_all(&dir).map_err(|e| e.to_string())?;

    if archive_name.ends_with(".zip") {
        let mut zip = zip::ZipArchive::new(Cursor::new(bytes)).map_err(|e| e.to_string())?;
        for i in 0..zip.len() {
            let mut file = zip.by_index(i).map_err(|e| e.to_string())?;
            if file.is_dir() {
                continue;
            }
            // Flatten the archive: the server and its libraries sit side by side.
            let name = match file.name().rsplit('/').next() {
                Some(n) if !n.is_empty() => n.to_string(),
                _ => continue,
            };
            let mut out = fs::File::create(dir.join(&name)).map_err(|e| e.to_string())?;
            std::io::copy(&mut file, &mut out).map_err(|e| e.to_string())?;
        }
    } else {
        let gz = flate2::read::GzDecoder::new(Cursor::new(bytes));
        let mut tar = tar::Archive::new(gz);
        // Symlinks (for example libllama.so -> libllama.so.0) are recreated as copies.
        let mut links: Vec<(String, String)> = Vec::new();
        for entry in tar.entries().map_err(|e| e.to_string())? {
            let mut entry = entry.map_err(|e| e.to_string())?;
            let path = entry.path().map_err(|e| e.to_string())?.to_path_buf();
            let name = match path.file_name() {
                Some(n) => n.to_string_lossy().to_string(),
                None => continue,
            };
            let kind = entry.header().entry_type();
            if kind.is_symlink() || kind.is_hard_link() {
                if let Ok(Some(target)) = entry.link_name() {
                    if let Some(t) = target.file_name() {
                        links.push((name, t.to_string_lossy().to_string()));
                    }
                }
                continue;
            }
            if !kind.is_file() {
                continue;
            }
            let mut out = fs::File::create(dir.join(&name)).map_err(|e| e.to_string())?;
            std::io::copy(&mut entry, &mut out).map_err(|e| e.to_string())?;
        }
        for (link, target) in links {
            let _ = fs::copy(dir.join(&target), dir.join(&link));
        }
    }

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        for entry in fs::read_dir(&dir).map_err(|e| e.to_string())?.flatten() {
            let _ = fs::set_permissions(entry.path(), fs::Permissions::from_mode(0o755));
        }
    }

    if exe.exists() {
        Ok(exe)
    } else {
        Err("The AI engine didn't install correctly".into())
    }
}

/// Starts the engine (if it isn't running) with the model, and waits until it answers.
async fn ensure_engine(app: &AppHandle) -> Result<(), String> {
    if engine_healthy().await {
        return Ok(());
    }
    let exe = install_engine(app).await?;
    // Older settings files may still hold a quantization tag, which the model name no longer needs.
    let model = read_settings(app).model_repo.replace(":Q4_K_M", "");
    let port = PORT.to_string();

    emit_status(app, "setup", "Loading the model (first run downloads about 1 GB)");
    let mut cmd = Command::new(&exe);
    cmd.args([
        "-hf", model.as_str(),
        "--host", "127.0.0.1",
        "--port", port.as_str(),
        "--reasoning", "off",
        "--temp", "0",
        "-c", "4096",
        "-t", "2",
        "-np", "2",
    ])
    .current_dir(exe.parent().unwrap_or(&exe))
    .stdout(Stdio::null())
    .stderr(Stdio::null());

    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        cmd.creation_flags(0x0800_0000); // no console window
    }

    let child = cmd.spawn().map_err(|e| e.to_string())?;
    *app.state::<Engine>().0.lock().unwrap() = Some(child);

    for _ in 0..720 {
        if engine_healthy().await {
            emit_status(app, "ready", "Ready");
            return Ok(());
        }
        tokio::time::sleep(Duration::from_millis(500)).await;
    }
    Err("The AI didn't start in time. Try restarting Blizzard.".into())
}

#[tauri::command]
async fn ask(
    app: AppHandle,
    messages: Vec<Msg>,
    stream: bool,
    max_tokens: u32,
) -> Result<String, String> {
    ensure_engine(&app).await?;
    let body = serde_json::json!({
        "stream": stream,
        "temperature": 0,
        "max_tokens": max_tokens,
        "messages": messages
            .iter()
            .map(|m| serde_json::json!({ "role": m.role, "content": m.content }))
            .collect::<Vec<_>>(),
    });
    let resp = reqwest::Client::new()
        .post(format!("http://127.0.0.1:{PORT}/v1/chat/completions"))
        .json(&body)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !stream {
        let v: serde_json::Value = resp.json().await.map_err(|e| e.to_string())?;
        return Ok(v["choices"][0]["message"]["content"]
            .as_str()
            .unwrap_or("")
            .trim()
            .to_string());
    }

    // Stream the answer word by word to the window as it is written.
    let mut bytes = resp.bytes_stream();
    let mut buf = String::new();
    let mut full = String::new();
    while let Some(chunk) = bytes.next().await {
        let chunk = chunk.map_err(|e| e.to_string())?;
        buf.push_str(&String::from_utf8_lossy(&chunk));
        while let Some(i) = buf.find('\n') {
            let line = buf[..i].trim().to_string();
            buf.drain(..=i);
            let Some(data) = line.strip_prefix("data: ") else { continue };
            if data == "[DONE]" {
                continue;
            }
            if let Ok(v) = serde_json::from_str::<serde_json::Value>(data) {
                if let Some(piece) = v["choices"][0]["delta"]["content"].as_str() {
                    full.push_str(piece);
                    let _ = app.emit("token", piece);
                }
            }
        }
    }
    Ok(full.trim().to_string())
}

#[tauri::command]
async fn web_search(app: AppHandle, query: String) -> Result<String, String> {
    let key = read_settings(&app).tavily_key.trim().to_string();
    if key.is_empty() {
        return Err("Ajoute ta clé Tavily dans les réglages pour utiliser la recherche web.".into());
    }
    let resp = reqwest::Client::new()
        .post("https://api.tavily.com/search")
        .bearer_auth(key)
        .json(&serde_json::json!({
            "query": query,
            "max_results": 5,
            "search_depth": "basic"
        }))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let status = resp.status();
    let v: serde_json::Value = resp.json().await.map_err(|e| e.to_string())?;
    if !status.is_success() {
        let msg = v["detail"]["error"].as_str().or(v["detail"].as_str()).unwrap_or("erreur inconnue");
        return Err(format!("Tavily : {msg}"));
    }

    let mut out = String::new();
    if let Some(items) = v["results"].as_array() {
        for r in items.iter().take(5) {
            out.push_str(&format!(
                "- {} ({}): {}\n",
                r["title"].as_str().unwrap_or(""),
                r["url"].as_str().unwrap_or(""),
                r["content"].as_str().unwrap_or("")
            ));
        }
    }
    Ok(out)
}

#[tauri::command]
fn get_settings(app: AppHandle) -> Settings {
    read_settings(&app)
}

#[tauri::command]
fn save_settings(app: AppHandle, settings: Settings) -> Result<(), String> {
    let json = serde_json::to_string_pretty(&settings).map_err(|e| e.to_string())?;
    fs::write(data_dir(&app).join("settings.json"), json).map_err(|e| e.to_string())
}

/// Stops the engine so the next question starts it again with the new model.
#[tauri::command]
fn restart_engine(app: AppHandle) {
    let child = app.state::<Engine>().0.lock().unwrap().take();
    if let Some(mut c) = child {
        let _ = c.kill();
    }
}

#[tauri::command]
fn load_chats(app: AppHandle) -> String {
    fs::read_to_string(data_dir(&app).join("chats.json")).unwrap_or_else(|_| "[]".into())
}

#[tauri::command]
fn save_chats(app: AppHandle, json: String) -> Result<(), String> {
    fs::write(data_dir(&app).join("chats.json"), json).map_err(|e| e.to_string())
}

fn main() {
    let app = tauri::Builder::default()
        .manage(Engine(Mutex::new(None)))
        .setup(|app| {
            // Start the model as soon as Blizzard opens, so the first question is fast.
            let handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                if let Err(e) = ensure_engine(&handle).await {
                    emit_status(&handle, "error", &e);
                }
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            ask,
            web_search,
            get_settings,
            save_settings,
            restart_engine,
            load_chats,
            save_chats
        ])
        .build(tauri::generate_context!())
        .expect("error while building Blizzard");

    app.run(|app, event| {
        if let tauri::RunEvent::Exit = event {
            let child = app.state::<Engine>().0.lock().unwrap().take();
            if let Some(mut c) = child {
                let _ = c.kill();
            }
        }
    });
}
