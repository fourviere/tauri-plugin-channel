<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri"
    import {listen, emit} from "tauri-plugin-channel-api"
    import type {Channel} from "tauri-plugin-channel-api"
    import type {PongEvents} from "src/types.d.ts"
  
    let msg = "Not yet Started"
    let channel: Channel
    let button_enabled = true 
  
    async function start() {
      channel = await invoke("pong")
      button_enabled = false
      let pong_count = 0
      await emit<PongEvents>(channel, "Ping")
      await listen<PongEvents>(channel, (event) => {
        pong_count++
        msg = pong_count + "x " + JSON.stringify(event)
        console.log("fff")
        emit<PongEvents>(channel, "Ping")
      })
    }
</script>

<div>
    <div class="row">
      <div>{msg}
      <button on:click={start} disabled={!button_enabled}>
        PingPong
      </button>
    </div>
    </div>
  </div>