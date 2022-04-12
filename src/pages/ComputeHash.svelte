<script lang="ts">
  import Icon from "svelte-awesome";
  import {
    beer,
    clipboard,
    archive,
    close,
    code,
    spinner,
    terminal,
    comment,
    codeFork,
    camera,
    ban,
  } from "svelte-awesome/icons";

  import { dialog, invoke } from "@tauri-apps/api";
  import { dialogOptions } from "../components/file/models/file.model";
  import {
    hasAlgorithms,
    store as algorithmStore,
  } from "../components/algorithm/store/algorithm.store";
  import {
    store as fileStore,
    fileMetaData,
    filePath,
    hasFilePath,
  } from "../components/file/store/file.store";
  import { store as hashStore } from "../components/hash/store/hash.store";
  import { Hash, LOADING } from "../components/hash/models/hash.models";

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
    hasAlgorithms: boolean
  ): boolean {
    return !hasFilePath || !hasAlgorithms;
  }

  function disableClearButton(hasFilePath: boolean): boolean {
    return !hasFilePath;
  }

  function isLoading(value: string): boolean {
    return value === LOADING;
  }

  let isAnyLoading;
  $: $hashStore, (isAnyLoading = [...$hashStore.values()].includes(LOADING));
</script>

<!-- info row -->
<div class="flex flex-wrap mb-6 md:mb-4">
  <!-- file -->
  <div
    class="flex justify-center items-center h-20 w-full mt-1 md:w-1/2 md:pt-5 md:mb-0 md:h-32"
  >
    {#if $fileMetaData}
      {#await $fileMetaData then metadata}
        <div class="flex">
          <!-- indicator -->
          <div class="indicator">
            <div class="indicator-item indicator-top indicator-end">
              <button
                class="btn btn-xs"
                disabled={disableClearButton($hasFilePath) || isAnyLoading}
                on:click={onClear}
              >
                <Icon
                  data={close}
                  scale={1.25}
                  style="color: hsl(var(--er));"
                /></button
              >
            </div>

            <!-- stat -->
            <div class="stats shadow-md bg-base-300 max-w-72">
              <div class="stat">
                <div class="stat-title hidden md:inline-block">
                  {metadata.size}
                </div>
                <div class="stat-value text-lg truncate">{metadata.name}</div>
                <div class="stat-desc hidden md:inline-block">
                  <div>Last Modified |</div>
                  {metadata.last_modified}
                </div>
              </div>
            </div>
          </div>
        </div>
      {:catch}
        <p class="hidden">No file was chosen yet.</p>
      {/await}
    {/if}
  </div>

  <div
    class="flex flex-wrap justify-center items-center h-8 w-full md:w-1/2 md:pt-5 md:h-32"
  >
    {#if $hasFilePath}
      <div class="hidden w-3/4 indicator text-center md:inline-block">
        <!-- <div class="indicator"> -->
        <span class="indicator-item indicator-middle badge"
          ><button><Icon data={clipboard} /></button></span
        >
        <input
          type="text"
          placeholder="Paste hash to verify"
          class="input input-bordered w-full"
        />
        <!-- </div> -->
      </div>
    {:else}
      <p class="text-lg w-full text-center">
        Press the "Browse" button to select a file.
      </p>
    {/if}

    <!-- buttons -->
    <div class="w-full text-center">
      <div class="tooltip" data-tip="Hash">
        <button
          class="hashy-button btn-circle"
          class:loading={isAnyLoading}
          disabled={disableComputeButton($hasFilePath, $hasAlgorithms) ||
            isAnyLoading}
          on:click={onCompute}
        >
          {#if !isAnyLoading}
            <Icon data={code} scale={1.25} />
          {/if}
        </button>
      </div>

      <div class="tooltip" data-tip="Browse">
        <button
          class="hashy-button btn-circle"
          disabled={isAnyLoading}
          on:click={onBrowse}
        >
          <Icon data={archive} scale={1.25} />
        </button>
      </div>
    </div>
  </div>
</div>

<!-- hash row -->
<div class="flex mb-4">
  <!-- container -->
  <div class="container mx-auto h-[380px] lg:h-[500px] overflow-y-auto">
    <!-- hashes -->
    {#if $algorithmStore}
      {#each $algorithmStore as algo}
        <!-- hash card -->
        <div
          class="h-1/3 lg:h-1/4 border-2 rounded-lg border-base-100 bg-base-300"
        >
          <!-- header -->
          <p class="text-base px-1 py-1 md:text-xl md:px-2 md:py-2">
            <b>{algo}</b>
          </p>
          <!-- end header -->

          <!-- body -->
          {#if $hashStore.get(algo) && !isLoading($hashStore.get(algo))}
            <p class="break-all text-sm px-2 md:text-base md:px-4">
              {$hashStore.get(algo)}
            </p>
          {:else if isLoading($hashStore.get(algo))}
            <div class="flex justify-center">
              <!-- <progress /> -->
              <!-- <Shadow size="30" color="primary"/> -->
              <Icon
                data={spinner}
                pulse
                scale={3}
                style="color: hsl(var(--p));"
              />
            </div>
          {/if}
          <!-- end body -->
        </div>
        <!-- end hash card -->
      {/each}
    {/if}
    <!-- end hashes -->
  </div>
  <!-- end container  -->
</div>

<!-- end row -->
<style>
  progress {
    border: none;
  }

  .max-w-72 {
    max-width: 18rem;
  }
</style>
