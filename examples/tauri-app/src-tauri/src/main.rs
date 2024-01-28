// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use futures_util::stream::StreamExt;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tauri::{async_runtime::spawn, AppHandle};
use tauri_plugin_channel::{channel, Channel, Receiver, Sender};
use tokio::{pin, select, time::sleep};

#[derive(Clone, Serialize)]
enum ProgressBackendEvents {
    Progress(u8),
    Done,
    Stopped,
}

#[derive(Debug, Deserialize)]
enum ProgressFrontendEvents {
    Stop,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
enum PongEvents {
    Ping,
    Pong,
}

#[tauri::command]
fn pong(app_handle: AppHandle) -> Channel {
    let (sender, receiver, channel) = channel(app_handle);

    spawn(async move {
        let stream = receiver.listen::<PongEvents>().await;
        pin!(stream);
        while let Some(value) = stream.next().await {
            println!("Got {value:?}");
            let next_value = match value {
                PongEvents::Ping => PongEvents::Pong,
                PongEvents::Pong => PongEvents::Ping,
            };
            sleep(Duration::from_secs(1)).await;
            sender.emit(next_value).await;
        }
    });
    channel
}
#[tauri::command]
fn fast_progress(app_handle: AppHandle) -> Channel {
    let (sender, receiver, channel) = channel(app_handle);
    progress(sender, receiver, Duration::from_millis(100));
    channel
}

#[tauri::command]
fn slow_progress(app_handle: AppHandle) -> Channel {
    let (sender, receiver, channel) = channel(app_handle);
    progress(sender, receiver, Duration::from_secs(1));
    channel
}

fn progress(sender: Sender, receiver: Receiver, duration: Duration) {
    spawn(async move {
        let receive_task = receiver.once::<ProgressFrontendEvents>();
        pin!(receive_task);
        for i in 0..101 {
            select! {
                _ = sleep(duration) => {
                    sender.emit(ProgressBackendEvents::Progress(i)).await;
                    if i == 100 {
                        sender.emit(ProgressBackendEvents::Done).await;
                    }
                }
                received = &mut receive_task => {
                    if let Some(ProgressFrontendEvents::Stop) = received {
                        sender.emit(ProgressBackendEvents::Stopped).await;
                        break;
                    }
                }
            }
        }
    });
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_channel::init())
        .invoke_handler(tauri::generate_handler![fast_progress, slow_progress, pong])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
