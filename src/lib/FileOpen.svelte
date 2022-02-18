<script lang="ts">
  import { dialog, invoke } from "@tauri-apps/api";
import { availableMonitors } from "@tauri-apps/api/window";

  let filePath: string | string[];
  let hash: string;

  const algorithm = "test";

  const chooseFile = async () => (filePath = await dialog.open());

  const clearFile = () => (filePath = null);

  const hashFile = async () =>
    (hash = await invoke<string>("hash_file", { filePath, algorithm }).then(
      (hash) => hash
    ));

  // const hash = async () => hashFile('test', 'test');

  // async function hashFile(filePath: string, algorithm: string): Promise<string> {
  //   const hash = await invoke<string>('hash_file', {filePath, algorithm}).then(hash => hash);
  //   return hash;
  // }
</script>

<div>
  <button on:click={chooseFile}>Choose File</button>
  <button on:click={clearFile}>Clear File</button>
  <button on:click={hashFile}>Hash File</button>
</div>

<div>
  {#if filePath}
    File: {filePath}
  {:else}
    Select a file.
  {/if}
</div>

<div>
  {#if hash}
    Hash: {hash}
  {:else}
    No hash available.
  {/if}
</div>

