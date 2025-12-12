# Keyless – Keyboard Locker for Windows

## Keyless is a lightweight console-based tool that lets you lock your keyboard so you can clean it safely without triggering accidental key presses.
It runs in the background with a Windows system tray icon and toggles on/off with F12.

## Features

Toggle keyboard lock/unlock with F12

System tray icon with right-click menu (Lock / Unlock / Exit)

Blocks all keyboard input when locked

Lightweight Rust executable (< 1 MB)

No GUI frameworks or heavy dependencies

## Why Keyless?

Cleaning a keyboard while the computer is on often triggers random input or shortcuts.
Keyless solves this by letting you:

Press F12 → Lock keyboard

Clean without worry

Press F12 → Unlock

Simple and effective.

## How It Works

Keyless installs a global low-level keyboard hook using:

SetWindowsHookExW(WH_KEYBOARD_LL, ...)


Inside the hook:

If key is F12 (key-down) → toggle lock

If locked → block all keyboard events

Otherwise → pass events normally

A hidden window handles tray icon messages through Shell_NotifyIconW.

## Installation
Installer (Recommended)

Use the provided:

Keyless-Installer.exe


It will:

Install Keyless to C:\Program Files\Keyless\

Add Start Menu shortcut

Add optional desktop shortcut

After installation, run:

Keyless


A tray icon will appear.

Portable Mode

Extract:

keyless.exe
icon.ico


Run:

keyless.exe

## Usage
Action	How
Lock Keyboard	Press F12 or use tray menu
Unlock Keyboard	Press F12 or use tray menu
Exit Keyless	Tray icon → Exit
Build From Source

## Requirements:

Rust (stable)

Windows (10 or 11)

Build:

cargo build --release


Executable is located at:

target/release/keyless.exe

Packaging (Installer)

To build the installer:

Install NSIS

Inside installer/, run:

makensis keyless_installer.nsi


This generates:

Keyless-Installer.exe

Project Structure
keyless/
├── src/main.rs
├── icon.ico
├── installer/keyless_installer.nsi
└── README.md

## Future Enhancements

Custom hotkeys

Dynamic tray icons

Balloon notifications

Auto-unlock timer
