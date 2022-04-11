<script lang="ts">
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

  function isLoading(value: string) {
    return value === LOADING;
  }
</script>

<!-- info row -->
<div class="flex flex-wrap mb-4">
  <!-- file -->
  <div class="flex justify-center items-center h-20 w-full md:w-1/2 md:pt-5 md:mb-0 md:h-32">
    {#if !$hasFilePath}
      <p class="text-xl py-2 text-center">
        Press the "Browse" button to select a file.
      </p>
    {:else}
      {#if $fileMetaData}
        {#await $fileMetaData then metadata}

          <!-- small screens show only file name  -->
          <div class="flex text-center md:hidden">
            <p class="text-xl truncate w-64">{metadata.name}</p>
          </div>

          <!-- metadata stat | hide on small screens-->
          <div class="hidden md:flex">

            <!-- indicator -->
            <div class="indicator">
              <div class="indicator-item indicator-top indicator-end">
                <button class="btn btn-xs" on:click={onClear}>X</button>
              </div>
            
            <!-- end indicator -->

            <!-- stat -->
            <div class="stats shadow bg-base-300 max-w-72">
              <div class="stat">
                <div class="stat-title">{metadata.size}</div>
                <div class="stat-value text-lg truncate">{metadata.name}</div>
                <div class="stat-desc">
                  <div>Last Modified |</div>
                  {metadata.last_modified}
                </div>
              </div>
            </div>
          </div>
          <!-- end stat -->

          </div>
          <!-- end indicator -->

        {:catch}
          <p class="hidden">No file was chosen yet.</p>
        {/await}
      {/if}
    {/if}
  </div>

  <!-- buttons -->
  <div class="flex flex-wrap justify-center items-center w-full md:w-1/2 h-8 md:pt-5 md:h-32">
    <button
      class="btn btn-sm md:btn mx-1 md:mx-3"
      disabled={disableComputeButton($hasFilePath, $hasAlgorithms)}
      on:click={onCompute}>Compute
    </button>

    <button 
      class="btn btn-sm md:btn mx-1 md:mx-3" 
      on:click={onBrowse}>Browse
    </button>

    <button
      class="btn btn-sm md:btn mx-1 md:mx-3"
      disabled={disableClearButton($hasFilePath)}
      on:click={onClear}>Clear
    </button>
  </div>
  <!-- end buttons -->

</div>
<!-- end info row -->


<!-- hash row -->
<div class="flex mb-4">

  <!-- container -->
  <div class="container mx-auto h-[380px] lg:h-[500px] overflow-y-auto">

    <!-- hashes -->
    {#if $algorithmStore}
        {#each $algorithmStore as algo}

          <!-- hash card -->
          <div class="h-1/3 lg:h-1/4 border rounded-lg border-base-100 bg-base-300">

            <!-- header -->
            <p class="text-base px-1 py-1 md:text-xl md:px-2 md:py-2">
              <b>{algo}</b>
            </p>
            <!-- end header -->
            
            <!-- body -->
            {#if $hashStore.get(algo) && !isLoading($hashStore.get(algo))}
              <div class="break-all text-sm px-2 md:text-base md:px-4">
                {$hashStore.get(algo)}
              </div>
            {:else if isLoading($hashStore.get(algo))}
              <div class="flex justify-center">
                <progress />
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
</style>
