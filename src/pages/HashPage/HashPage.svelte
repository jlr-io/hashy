<script lang="ts">
  import { dialog, invoke } from "@tauri-apps/api";
  import { dialogOptions } from '../../models/file.model';
  import {
    hasAlgorithms,
    store as algorithmStore,
  } from '../../stores/algorithm.store';
  import {
    filePath,
    fileMetadata,
    hasFilePath,
    store as fileStore,
  } from '../../stores/file.store';
  import { store as hashStore } from "../../stores/hash.store";
  import { Hash, LOADING } from '../../models/hash.models';

  import FileMetadata from "./components/FileMetadata.svelte";
  import HashButtons from "./components/HashButtons.svelte";
  import TargetHashInput from "./components/TargetHashInput.svelte";
  import HashCards from "./components/HashCards.svelte";

  const onBrowse = async () => {
    fileStore.set((await dialog.open(dialogOptions)) as string);
    hashStore.reset();
  };

  const onClear = () => {
    fileStore.reset();
    hashStore.reset();
  };

  const onCompute = async () => {
    $algorithmStore.forEach(async (algo) => {
      let hash = {
        algo,
        value: LOADING,
      };
      hashStore.updateMap(hash);
      hash = await invoke<Hash>("hash_file2", {
        path: $filePath,
        algo: algo,
      });
      hashStore.updateMap(hash);
    });
  };

  function disableComputeButton(
    hasFilePath: boolean,
    hasAlgorithms: boolean,
    isAnyLoading: boolean,
  ): boolean {
    return !hasFilePath || !hasAlgorithms || isAnyLoading;
  }

  function disableClearButton(hasFilePath: boolean, isAnyLoading: boolean): boolean {
    return !hasFilePath || isAnyLoading;
  }

  // make a store value?
  let isAnyLoading;
  $: $hashStore, isAnyLoading = [...$hashStore.values()].includes(LOADING);
</script>

<!-- info row -->
<div class="flex flex-wrap mb-6 md:mb-4">

  <!-- left half -->
  <div class="flex flex-wrap justify-center items-center h-20 w-full mt-1 md:w-1/2 md:pt-5 md:mb-0 md:h-32">
    <FileMetadata 
      disableButton={disableClearButton($hasFilePath, isAnyLoading)}
      metadata={$fileMetadata} 
      on:clear={onClear}/>
  </div>

  <!-- right half -->
  <div class="flex flex-wrap justify-center items-center h-8 w-full md:w-1/2 md:pt-5 md:h-32">
    {#if $hasFilePath}
      <div class="hidden w-9/12 mb-2 text-center md:inline-block">
        <TargetHashInput />
      </div>
    {:else}
      <p class="text-lg w-full text-center">
        Press the "Browse" button to select a file.
      </p>
    {/if}

    <!-- buttons -->
    <div class="w-full text-center mt-1">
      <HashButtons 
        isLoading={isAnyLoading} 
        disableButton={disableComputeButton($hasFilePath, $hasAlgorithms, isAnyLoading)}
        on:browse={onBrowse}
        on:hash={onCompute}
      />
    </div>

  </div>

  <!-- end row -->
</div>

<!-- hash row -->
<div class="flex mb-4">
  <HashCards 
    algorithms={$algorithmStore}
    map={$hashStore}
  />
</div>
