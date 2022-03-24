<script lang="ts">
  import { hashStore } from "../stores/hash.store";
</script>

<div>
    {#if $hashStore}
      {#each $hashStore.hashes as hash }
        <div><b>{hash.algo}</b> | <pre class="text-success"><code>{hash.hash}</code></pre></div>
        <div><b>{hash.algo}</b> | {hash.hash}</div>
      {/each}
    {:else}
      No hashStore.
    {/if}
  </div>
