<script lang="ts">
  import type { Hashes } from "src/stores/hash.store";
  export let file: Hashes;
</script>

<div class="flex justify-center w-full">
  <div class="card bg-primary text-primary-content">
    <div class="card-body">
      <h2 class="card-title">{file.path}</h2>
      {#each file.value as hash}
        <p> <b>{hash.algo} </b> | {hash.value} </p>
      {/each}
    </div>
  </div>
</div>
