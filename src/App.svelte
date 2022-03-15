<script lang="ts">
  // tailwind
  import "./app.css";
  import SelectFiles from "./components/SelectFiles.svelte";
  import SelectAlgorithms from "./components/SelectAlgorithms.svelte";
  import HashFiles from "./components/HashFiles.svelte";
  import ThemeChanger from "./components/ThemeChanger.svelte";

  import { invoke } from "@tauri-apps/api";
  import { listen, Event } from "@tauri-apps/api/event";

  type Payload  = {
    message: String
  };

  let eventMessage;
  // listen to the `click` event and get a function to remove the event listener
  // there's also a `once` function that subscribes to an event and automatically unsubscribes the listener on the first event
  listen("test", (event: Event<Payload>) => {
    // event.event is the event name (useful if you want to use a single callback fn for multiple event types)
    // event.payload is the payload object
    eventMessage = event.payload.message;
  });

  let show: Boolean = false;

  async function windowEventTest() {
    await invoke("window_event_test");
  }

  // function emitTest(file: String) {
  //   event.emit("file-selected", file);
  // }
</script>

<main>
  <div>
    <ThemeChanger />
    <SelectFiles />
    <SelectAlgorithms />
    <HashFiles />
    EVENT: {eventMessage}
    {#if show}
      Show me
    {/if}
  </div>
</main>

<style>
  :root {
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Oxygen,
      Ubuntu, Cantarell, "Open Sans", "Helvetica Neue", sans-serif;
  }
</style>
