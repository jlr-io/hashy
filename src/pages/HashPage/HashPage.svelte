<script lang="ts">
  import { blur } from 'svelte/transition';
  import { dialog, invoke, event } from "@tauri-apps/api";
  import { algorithms } from "../../models/algorithms.models";
  import {
    hasAlgorithms,
    store as algorithmStore,
  } from '../../stores/algorithm.store';

  import { dialogOptions } from '../../models/file.model';
  import {
    filePath,
    fileMetadata,
    hasFilePath,
    store as fileStore,
  } from '../../stores/file.store';

  import { Hash, LOADING } from '../../models/hash.models';
  import { store as hashStore } from "../../stores/hash.store";

  import Metadata from "./components/Metadata.svelte";
  import Buttons from "./components/Buttons.svelte";
  import Input from "./components/Input.svelte";
  import Cards from "./components/Cards.svelte";

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

  let target;

  // make a store value?
  let isAnyLoading;
  $: $hashStore, isAnyLoading = [...$hashStore.values()].includes(LOADING);

</script>

<!-- info row -->
<div class="flex flex-wrap container mx-auto mb-2">
  <!-- left half -->
  <div class="flex flex-wrap justify-center items-center max-h-20 md:h-32 md:max-h-32 mt-6 mb-4 md:mt-0 md:mb-0 w-full md:w-1/2">
    <div class="flex">
      <!-- {#if $hasFilePath} -->
        <div in:blur="{{duration: 500}}" class="mt-4">
          <Metadata 
            hasFilePath={$hasFilePath}
            disableButton={disableClearButton($hasFilePath, isAnyLoading)}
            metadata={$fileMetadata} 
            on:clear={onClear}/>
        </div>
    </div>
  </div>

  <!-- right half -->
  <div class="flex flex-wrap md:justify-center items-center md:items-start max-h-20 w-full md:w-1/2 md:h-32 md:max-h-32">

    <div class="w-full lg:w-4/6 text-center">
      <Buttons 
        {algorithms}
        bind:selectedAlgorithms={$algorithmStore}
        isLoading={isAnyLoading} 
        disableButton={disableComputeButton($hasFilePath, $hasAlgorithms, isAnyLoading)}
        on:browse={onBrowse}
        on:compute={onCompute}
      />
    </div>

    <div class="w-11/12 lg:w-full text-center mt-2">
      <Input bind:target/>
    </div>
  </div>
</div>

<!-- hash row -->
<div class="flex mb-4">
    <Cards 
      algorithms={$algorithmStore}
      map={$hashStore}
      {target}
  />
</div>
