# ❄ Blizzard

**A private study tutor for your computer.** Ask anything, explain it back in your own words, and Blizzard tracks how well you understand each topic.

Blizzard runs entirely on your machine. Your questions and chats stay on your computer. There's no account to create and no subscription.

![Blizzard screenshot](docs/screenshot.png)

> Add a screenshot at `docs/screenshot.png` to show the app here.

---

## Features

- **Chat about anything.** Maths, history, languages, code: any subject you want to study.
- **Understanding tracker.** Blizzard reads what you explain and shows a score for each topic. The score rises only from your own correct explanations, not from the tutor's, and it moves gradually.
- **Streaming answers.** Replies appear word by word as they're written, so you're never staring at a blank screen.
- **Optional web search.** Turn it on for a question and Blizzard looks up current information before answering.
- **Works offline.** Once the model is downloaded, web search is the only feature that needs an internet connection.
- **Light on your computer.** The AI engine runs at low priority, so your other apps stay smooth.
- **Windows, macOS and Linux.**

---

## Download and install

Go to the **[Releases](../../releases)** page and download the file for your system:

| System | File to download | How to open it |
|---|---|---|
| Windows | `Blizzard_x.y.z_x64-setup.exe` or `.msi` | Double-click, then follow the installer. |
| macOS | `Blizzard_x.y.z_aarch64.dmg` (Apple Silicon) or `_x64.dmg` (Intel) | Open the file, drag Blizzard into Applications. |
| Linux | `Blizzard_x.y.z_amd64.AppImage` or `.deb` | AppImage: make it executable, then double-click. `.deb`: install with your package manager. |

**Security warnings.** Blizzard isn't code-signed yet, so your system may warn you the first time:

- **Windows:** click **More info**, then **Run anyway**.
- **macOS:** right-click Blizzard, choose **Open**, then **Open** again. If macOS still blocks it, go to System Settings, then Privacy & Security, and click **Open Anyway**.
- **Linux:** no warning normally appears.

### What your computer needs

- 8 GB of RAM or more (16 GB is comfortable)
- About 2 GB of free disk space
- An internet connection for the first launch only

---

## First launch

The first time you open Blizzard, it sets itself up:

1. It downloads the AI engine (llama.cpp) for your system.
2. It downloads the **LFM3-Omni-2B** model, about 1 GB, from Hugging Face.

This takes a few minutes and happens once. After that, Blizzard opens straight into the chat. You'll see progress messages at the bottom of the window.

---

## How to use it

### Start a chat

Press **+ New chat** in the sidebar, then type your question and press **Enter**. Use **Shift + Enter** for a new line.

### Study by explaining

The most useful way to study with Blizzard is to explain things back:

1. Ask about a topic, for example "how does photosynthesis work?"
2. Read the answer.
3. Explain it in your own words, for example "plants use light to turn water and CO₂ into sugar and oxygen."
4. Blizzard corrects mistakes gently and asks follow-up questions.

As you explain correctly, the **Understanding** panel on the right moves up, topic by topic. Each topic shows a short quote from what you said, so you can see why its score changed.

### Turn the understanding panel on or off

Open **Settings** and tick or untick **Track my understanding**. When it's off, the panel disappears and the chat fills the window.

### Use web search

Web search needs a free Brave Search key:

1. Go to **brave.com/search/api**, create a free account, and choose the free plan for the Search API. Check the plan page for its current monthly limit.
2. Copy the API key from your dashboard.
3. In Blizzard, open **Settings**, paste the key, and press **Save**.
4. Press **Web search: off** in the chat box to turn it on. It turns back off when you close Blizzard, so you choose when to search.

Only your search query leaves your computer. Your chats and the rest of your conversation stay local.

### Switch the model

The **Model** dropdown in the sidebar lists the available models. Blizzard currently ships with one, LFM3-Omni-2B. When more are added, choose one here. The engine restarts with the new model.

### Delete a chat

Open the chat, then press **Delete chat** at the top right. Deleted chats can't be recovered.

---

## Privacy

- Your chats, settings, and search key are stored only in Blizzard's data folder on your computer.
- The AI model runs on your computer. Blizzard doesn't send your questions anywhere.
- Web search is the only feature that sends anything over the internet, and only when you turn it on.

Blizzard's data folder is:

- Windows: `%APPDATA%\io.github.swiss4ppd3v.blizzard`
- macOS: `~/Library/Application Support/io.github.swiss4ppd3v.blizzard`
- Linux: `~/.local/share/io.github.swiss4ppd3v.blizzard`

To reset Blizzard completely, close it and delete that folder.

---

## Troubleshooting

**The status says "The AI didn't start in time."**
Close Blizzard, wait a few seconds, and open it again. The first launch can take several minutes on a slow connection, because the model is large.

**The answers are slow.**
Close heavy programs such as video players and browser tabs you don't need. Blizzard uses about 2 CPU cores while it writes an answer. If your computer has less than 8 GB of RAM, close other apps before studying.

**Web search says "Add a Brave Search key first."**
Open Settings and paste your key, as described above.

**Web search says it failed.**
Check that your key is correct and that you haven't used up the free monthly limit on your Brave dashboard.

**Nothing happens after the first launch.**
Check your internet connection, then close and reopen Blizzard. If the problem continues, please open an issue on this page and include the message shown at the bottom of the window.

---

## Build from source

Developers can build Blizzard themselves. You need [Rust](https://rustup.rs) and Python 3.

```bash
git clone https://github.com/Swiss4ppD3v/blizzard.git
cd blizzard
python3 launch.py          # builds once, then opens Blizzard
python3 launch.py --dev    # run in development mode
python3 launch.py --build  # rebuild after making changes
```

On Linux, install the system libraries first. On Debian or Ubuntu:

```bash
sudo apt install libwebkit2gtk-4.1-dev libappindicator3-dev librsvg2-dev patchelf
```

The project is built with [Tauri 2](https://tauri.app). The interface is in `src/index.html`, and the Rust backend is in `src-tauri/src/main.rs`.

### Adding a model

Models are listed in the `MODELS` array near the top of `src/index.html`. Each entry needs a display name and a Hugging Face repository in the form `owner/name:quantization`.

---

## Credits

- **[LFM3-Omni-2B](https://huggingface.co/LiquidAI/LFM3-Omni-2B-GGUF)** by Liquid AI. The model is downloaded from Hugging Face and remains under its own license, LFM 1.0. Blizzard doesn't include the model file.
- **[llama.cpp](https://github.com/ggml-org/llama.cpp)**, the engine that runs the model (MIT license).
- **[Tauri](https://tauri.app)**, the framework that makes Blizzard a desktop app (MIT or Apache 2.0).
- **[Brave Search API](https://brave.com/search/api)**, used for optional web search.

---

## License

Blizzard is released under the [MIT License](LICENSE). You're free to use, copy, modify, and share it, including for commercial use, as long as the copyright notice stays in place.
