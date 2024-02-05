<script lang="ts">
  import { channel } from "tauri-plugin-channel-api";
  import type { Sender, Receiver } from "tauri-plugin-channel-api";
  import type { PongEvents } from "src/types.d.ts";

  let msg = "Not yet Started";
  let sender: Sender;
  let receiver: Receiver;
  let button_enabled = true;

  async function start() {
    [sender, receiver] = await channel("pong");
    button_enabled = false;
    let pong_count = 0;
    await sender.emit<PongEvents>("Ping");
    await receiver.listen<PongEvents>((event) => {
      pong_count++;
      msg = pong_count + "x " + event;
      sender.emit<PongEvents>("Ping");
    });
  }
</script>

<div>
  <div class="row">
    <div>
      {msg}
      <button on:click={start} disabled={!button_enabled}> PingPong </button>
    </div>
  </div>
</div>
