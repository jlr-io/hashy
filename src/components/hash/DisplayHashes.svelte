<script lang="ts">
  import { MapType, LOADING } from "../../models/../../models/hash.modelset algorithms: string[] = [];
  export let map: MapType;

  function isLoading(value: string) {
    return value === LOADING;
  }
</script>

{#if algorithms}
  <div class="h-full overflow-y-auto">
    {#each algorithms as algo}
    <!-- hash card -->
      <div class="h-1/3 lg:h-1/4 border rounded-lg border-base-100 bg-base-300">
        <!-- header -->
        <p class="text-base px-1 py-1 md:text-xl md:px-2 md:py-2"><b>{algo}</b></p>
        <!-- body -->
        {#if map.get(algo) && !isLoading(map.get(algo))}
          <div class="break-all text-sm px-2 md:text-base md:px-4">{map.get(algo)}</div>
        {:else if isLoading(map.get(algo))}
          <div class="flex justify-center">
            <progress />
          </div>
        {/if}
      </div>
    {/each}
  </div>
{/if}
<style>
  progress {
    border: none !important;
    width: 50%;
  }
</style>
