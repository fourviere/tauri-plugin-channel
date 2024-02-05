# Tauri Plugin: Channel

A Tauri plugin that provides a thin abstraction layer over Tauri event system.

## Overview

Channel is a Tauri plugin that provides an ergonomic layer for creating “scoped channels” between frontend and backend.

### Features

1. Simplifies Tauri event system hiding details like event-name 
2. Channels don't interfere with each other: events are self-contained 
3. Automatically closes frontend listener when each backend sender has been dropped

### Common Use Case

Imagine a download manager application that shows different progress bar for each file being downloaded from the backend. 

A classic Tauri app :
- Should know and manage different event name, one for each download
- Alternatively, provides a custom partition mechanism in the event payload to distinguish events receiver

Using Channel, each `fn download (link: String)` from backend will return its channel where download progress events will be published from backend and are visible only to frontend component that owns that channel.


## Installation

Install the Core plugin by adding the following to your Cargo.toml file:

`src-tauri/Cargo.toml`

```toml
[dependencies]
tauri-plugin-channel = { git = "https://github.com/fourviere/tauri-plugin-channel" }
```

You can install the JavaScript Guest bindings using your preferred JavaScript package manager:

```bash
npm add https://github.com/fourviere/tauri-plugin-channel
```

## Usage

### Backend

1. Register the channel plugin with Tauri:

`src-tauri/src/main.rs`

```rust
fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_channel::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

2. Return the channel in the tauri::command

`src-tauri/src/main.rs`

```rust
use tauri_plugin_channel;

#[derive(Clone, Serialize)]
enum BackendEvents {
    Progress(u8),
}

#[tauri::command]
fn example_function (app_handle: AppHandle) -> Channel {
    let (sender, receiver, channel) = channel(app_handle);
    tokio::spawn({
        for i in 0..100
            tokio::sleep(Duration::from_secs(i)).await
            sender.emit(BackendEvents::Progress(i)).await;
    });
    tokio::spawn({
        println!(receiver.once::<String>().await)
    });
    channel
}
```

### Frontend

Use the `Receiver.receive, Sender.emit` functions to receive or send events to/from backend

```typescript
import {channel} from "tauri-plugin-channel-api"
import type {Sender, Receiver} from "tauri-plugin-channel-api"

type BackendEvents = { Progress: number } 

[sender, receiver]  = await channel('example_function')
receiver.listen<BackendEvents>(event => console.log(JSON.stringify(event)));
sender.emit(channel, "1");
```

## Example App

### Prepare

1. Install dependencies of frontend API.

```bash
npm install 
```

1. Build frontend API.

```bash
npm build
```

2. Install dependencies of example app.

```bash
cd examples/tauri-app
npm install
```

3. Start example app.

```bash
npm tauri dev
```
