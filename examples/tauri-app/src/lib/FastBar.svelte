<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri"
  import {listen, emit} from "tauri-plugin-channel-api"
  import type {Channel} from "tauri-plugin-channel-api"

  type FrontendEvents = "Stop"
  type BackendEvents = { Progress: number } | "Done" | "Stopped" | "Not yet Started"

  let msg = "Not yet Started"
  let channel: Channel
  let start_button_enabled = true 
  let stop_button_enabled = false 

  async function start() {
    
    channel = await invoke("fast_progress")
    stop_button_enabled = true
    start_button_enabled = false
    await listen<BackendEvents>(channel, (event) => {
      msg = JSON.stringify(event)
      if (msg === '"Stopped"' || msg === '"Done"') {
        start_button_enabled = true
        stop_button_enabled = false
      }
    })
  }

  async function stop() {
    if (channel!== undefined && channel!== null) {
      await emit<FrontendEvents>(channel, "Stop")
    }
  }
</script>

<div>
  <div class="row">
    <div>{msg}
    <button on:click={start} disabled={!start_button_enabled}>
      StartFast
    </button>
    <button on:click={stop} disabled={!stop_button_enabled}>
      StopFast
    </button>
  </div>
  </div>
</div>
