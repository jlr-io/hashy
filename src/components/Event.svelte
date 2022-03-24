<script lang="ts">
  import { listen, Event } from "@tauri-apps/api/event";

  type Payload = {
    message: String;
  };

  // let eventMessage;
  let eventMessages = [];

  listen("app", (event: Event<Payload>) => {
    eventMessages = [...eventMessages, `app: ${event.payload.message}`];
  });

  listen("file", (event: Event<Payload>) => {
    eventMessages = [...eventMessages, `file: ${event.payload.message}`];
  });

  listen("hash", (event: Event<Payload>) => {
    eventMessages = [...eventMessages, `hash: ${event.payload.message}`];
  });

  import { invoke } from "@tauri-apps/api";
  let show: Boolean = false;

  async function windowEventTest() {
    await invoke("window_event_test");
  }

  // function emitTest(file: String) {
  //   event.emit("file-selected", file);
  // }
</script>


<div>

  <button class="btn mx-3" on:click={() => eventMessages = []}>CLEAR EVENTS</button>

  EVENT LOG:
  <ol>
    {#each eventMessages as event}
    <li>{event}</li>
    {/each}
  </ol>
  
  {#if show}
    Show me
  {/if}

</div>
