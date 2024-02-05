<script lang="ts">
  import { channel } from "tauri-plugin-channel-api";
  import type { Sender, Receiver } from "tauri-plugin-channel-api";
  import type {
    ProgressFrontendEvents,
    ProgressBackendEvents,
  } from "src/types.d.ts";

  let msg = "Not yet Started";
  let sender: Sender;
  let receiver: Receiver;
  let start_button_enabled = true;
  let stop_button_enabled = false;

  async function start() {
    [sender, receiver] = await channel("slow_progress");
    stop_button_enabled = true;
    start_button_enabled = false;
    await receiver.listen<ProgressBackendEvents>((event) => {
      msg = JSON.stringify(event);
      if (msg === '"Stopped"' || msg === '"Done"') {
        start_button_enabled = true;
        stop_button_enabled = false;
      }
    });
  }

  async function stop() {
    if (sender !== undefined && sender !== null) {
      await sender.emit<ProgressFrontendEvents>("Stop");
    }
  }
</script>

<div>
  <div class="row">
    <div>
      {msg}
      <button on:click={start} disabled={!start_button_enabled}>
        StartSlow
      </button>
      <button on:click={stop} disabled={!stop_button_enabled}>
        StopSlow
      </button>
    </div>
  </div>
</div>
