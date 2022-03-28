<script lang="ts">
  import { hashStore } from "../stores/hash.store";
</script>

<div>
  {#if $hashStore}
  <input type="text" placeholder="Enter hash to compare" class="input input-bordered input-sm w-full max-w-xs">

    {#each $hashStore.hashes as hash}
      <div><b>{hash.algo}</b> | {hash.hash}</div>
      <!-- <div><b>{hash.algo}</b> | {hash.hash}</div> -->
    {/each}
  {:else}
    No hashStore.
  {/if}
</div>
