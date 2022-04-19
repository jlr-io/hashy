<script lang="ts">
  import { blur } from 'svelte/transition';
  import { dialog, invoke } from "@tauri-apps/api";
  import { algorithms } from "../models/algorithms.models";
  import {
    hasAlgorithms,
    store as algorithmStore,
  } from '../stores/algorithm.store';

  import { dialogOptions } from '../models/file.model';
  import {
    filePath,
    fileMetadata,
    hasFilePath,
    store as fileStore,
  } from '../stores/file.store';

  import { Hash, LOADING } from '../models/hash.models';
  import { 
    isAnyLoading, 
    store as hashStore 
  } from "../stores/hash.store";

  import Metadata from "./Metadata.svelte";
  import Buttons from "./Buttons.svelte";
  import Input from "./Input.svelte";
  import Cards from "./Cards.svelte";

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
      let result = await invoke<Hash>("hash_file", {
        path: $filePath,
        algo: algo,
      })
      .then((hash) => hash)
      .catch((error) => ({ ...hash, value: error }))

      hashStore.updateMap(result);
    });
  };

  function disableComputeButton(
    hasFilePath: boolean,
    hasAlgorithms: boolean,
    isAnyLoading: boolean,
  ): boolean {
    return !hasFilePath || !hasAlgorithms || isAnyLoading;
  };

  function disableClearButton(hasFilePath: boolean, isAnyLoading: boolean): boolean {
    return !hasFilePath || isAnyLoading;
  };

  let target;
</script>

<div class="flex flex-wrap container mx-auto md:h-28 md:max-h-28">
  <div class="flex justify-center w-full md:w-1/2 mt-1 items-center">
    <div in:blur="{{duration: 500}}">
      <Metadata 
        hasFilePath={$hasFilePath}
        disableButton={disableClearButton($hasFilePath, $isAnyLoading)}
        metadata={$fileMetadata} 
        on:clear={onClear}/>
    </div>
  </div>

  <div class="flex justify-center w-full md:w-1/2 mt-1 items-center ">
    <Buttons 
      {algorithms}
      bind:selectedAlgorithms={$algorithmStore}
      isLoading={$isAnyLoading} 
      disableButton={disableComputeButton($hasFilePath, $hasAlgorithms, $isAnyLoading)}
      on:browse={onBrowse}
      on:compute={onCompute}
    />
  </div>

</div>

<div class="hidden md:flex justify-center container mx-auto mb-2">
  <div class="w-11/12 text-center mt-2">
    <Input bind:target/>
  </div>
</div>

<div class="flex mb-4">
    <Cards 
      algorithms={$algorithmStore}
      map={$hashStore}
      {target}
  />
</div>