<script lang="ts">
  import { hashStore } from "../stores/hash.store";
</script>

<div class="container bg-blue-700 overflow-y-auto hash">
  {#if $hashStore}
    <!-- <input type="text" placeholder="Enter hash to compare" class="input input-bordered input-sm w-full max-w-xs"> -->
    {#each $hashStore.value as hash}

        <p>
          <b>{hash.algo}</b> | 
          <span class="break-words">{hash.value}</span>
        </p>

    {/each}
  {/if}
</div>

<style>
  .hash {
    max-height: calc(100vh - 375px);
  }
</style>
