#!/usr/bin/env python3
"""Blizzard launcher.

    python3 launch.py           open Blizzard (builds it the first time)
    python3 launch.py --build   rebuild Blizzard, then open it
    python3 launch.py --dev     run Blizzard in development mode
"""
import os
import shutil
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent
APP = ROOT / "src-tauri"


def run(cmd, cwd=None, quiet=False):
    print("›", " ".join(cmd), flush=True)
    out = subprocess.DEVNULL if quiet else None
    return subprocess.call(cmd, cwd=cwd, stdout=out, stderr=out)


def built_app():
    release = APP / "target" / "release"
    if sys.platform == "win32":
        exe = release / "blizzard.exe"
        return exe if exe.exists() else None
    if sys.platform == "darwin":
        return next((release / "bundle" / "macos").glob("*.app"), None)
    exe = release / "blizzard"
    return exe if exe.exists() else None


def open_app(path):
    if sys.platform == "darwin":
        subprocess.Popen(["open", str(path)])
    elif sys.platform == "win32":
        os.startfile(str(path))  # noqa: S606 (opens the built app)
    else:
        subprocess.Popen([str(path)], start_new_session=True)


def main():
    args = set(sys.argv[1:])

    if not shutil.which("cargo"):
        sys.exit("Rust isn't installed. Get it from https://rustup.rs, then run this again.")

    if run(["cargo", "tauri", "--version"], quiet=True) != 0:
        print("Installing the Tauri build tool (first run only)…")
        if run(["cargo", "install", "tauri-cli", "--version", "^2", "--locked"]) != 0:
            sys.exit("Couldn't install the Tauri build tool.")

    if not (APP / "icons" / "icon.png").exists():
        source = ROOT / "icon-source.png"
        if source.exists():
            print("Creating app icons from icon-source.png…")
            run(["cargo", "tauri", "icon", str(source)], cwd=APP)

    if "--dev" in args:
        sys.exit(run(["cargo", "tauri", "dev"], cwd=APP))

    if "--build" in args or built_app() is None:
        print("Building Blizzard (a few minutes the first time)…")
        if run(["cargo", "tauri", "build"], cwd=APP) != 0:
            sys.exit("The build failed. Read the errors above.")

    app = built_app()
    if app is None:
        sys.exit("Blizzard was built, but the app file wasn't found.")
    open_app(app)


if __name__ == "__main__":
    main()
