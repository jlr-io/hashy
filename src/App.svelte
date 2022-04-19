<script lang="ts">
  // tailwind
  import "./app.css";

  import { event } from '@tauri-apps/api';
  import { store as fileStore } from './stores/file.store';
  import Menu from './components/Menu.svelte';
  import Hashy from "./components/Hashy.svelte";

  let fileDropEvent = 'tauri://file-drop';
  let fileDropHoverEvent = 'tauri://file-drop-hover';
  let fileDropCancelEvent = 'tauri://file-drop-cancelled';

  let fileDropHover = false;

  event.listen(fileDropEvent, (event) => {
    fileStore.set(event.payload[0]);
    fileDropHover = false;
  });

  event.listen(fileDropHoverEvent, () => {
    fileDropHover = true;
  })

  event.listen(fileDropCancelEvent, () => {
    fileDropHover = false;
  });
</script>

<main class="overflow-hidden select-none" class:file-hover={fileDropHover}>
  <Menu>
    <Hashy />
  </Menu>
</main>

<style>
  :root {
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Oxygen,
      Ubuntu, Cantarell, "Open Sans", "Helvetica Neue", sans-serif;
  }

  .file-hover {
    opacity: 50%;
  }
</style>
