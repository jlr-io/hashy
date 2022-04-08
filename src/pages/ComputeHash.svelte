<script lang="ts">
  import { dialog, invoke } from "@tauri-apps/api";
  import { dialogOptions } from "../components/file/models/file.model";
  import {
    hasSelectedAlgorithms,
    selectedAlgorithms,
  } from "../components/algorithm/store/algorithm.store";
  import {
    store as fileStore,
    fileMetaData,
    filePath,
    hasSelectedFilePath,
  } from "../components/file/store/file.store";
  import { store as hashStore } from "../components/hash/store/hash.store";
  import type { Hash } from "../components/hash/models/hash.models";
  import FileMetadata from "../components/file/FileMetadata.svelte";
  import DisplayHashes from "../components/hash/DisplayHashes.svelte";

  const onBrowse = async () => {
    fileStore.set((await dialog.open(dialogOptions)) as string);
  };

  const onClear = () => {
    hashStore.reset();
    fileStore.reset();
  };

  const onCompute = async () => {
    $selectedAlgorithms.forEach(async (algo) => {
      const hash = await invoke<Hash>("hash_file2", {
        path: $filePath,
        algo: algo,
      });
      hashStore.updateMap(hash);
    });
  };

  function disableComputeButton(hasSelectedFilePath: boolean, hasSelectedAlgorithms: boolean): boolean {
    return !hasSelectedFilePath || !hasSelectedAlgorithms;
  }

  function disableClearButton(hasSelectedFilePath: boolean): boolean {
    return !hasSelectedFilePath;
  }
</script>

<div class="text-xl flex justify-center py-5">Hashing page</div>

{#if !$hasSelectedFilePath}
  <p class="text-center">Press the "Browse" button to select a file.</p>
{/if}

<div class="text-center">
  <FileMetadata on:clear={onClear} metadata={$fileMetaData} />
</div>

<div class="text-center">
  <DisplayHashes algorithms={$selectedAlgorithms} map={$hashStore} />
</div>

<div class="toolbar">
  <button class="btn mx-3" 
          on:click={onBrowse}>
    Browse
  </button>
  <button class="btn mx-3" 
          disabled={disableComputeButton($hasSelectedFilePath, $hasSelectedAlgorithms)} 
          on:click={onCompute}>
    Compute
  </button>
  <button class="btn mx-3" 
          disabled={disableClearButton($hasSelectedFilePath)} 
          on:click={onClear}>
    Clear
  </button
  >
</div>
