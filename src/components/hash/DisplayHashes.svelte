<script lang="ts">
  import type { MapType } from "./models/hash.models";
  export let algorithms: string[] = [];
  export let map: MapType;
</script>

{#if algorithms}
  <!-- <input type="text" placeholder="Enter hash to compare" class="input input-bordered input-sm w-full max-w-xs"> -->
  {#each algorithms as algo}
    <p>
      <b>{algo}</b> |
        <span class="break-words">{map.get(algo)}</span>
    </p>
  {/each}
{/if}
