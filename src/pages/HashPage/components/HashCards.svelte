<script lang="ts">
  import Icon from "svelte-awesome";
  import {
    spinner,
  } from "svelte-awesome/icons";
  import { LOADING } from '../../../models/hash.models';

  export let algorithms: string[];
  export let map: Map<string, string>;

  function isLoading(value: string): boolean {
    return value === LOADING;
  }

</script>

<!-- container -->
<div class="container mx-auto h-[380px] lg:h-[500px] overflow-y-auto">
  <!-- hashes -->
  {#if algorithms}
    {#each algorithms as algo}
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
        {#if map.get(algo) && !isLoading(map.get(algo))}
          <p class="break-all text-sm px-2 md:text-base md:px-4">
            {map.get(algo)}
          </p>
        {:else if isLoading(map.get(algo))}
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