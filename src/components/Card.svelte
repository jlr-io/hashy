<script lang="ts">
  import type { FileHash } from "src/stores/hash.store";
  export let file: FileHash;
</script>

<div class="flex justify-center w-full">
  <div class="card bg-primary text-primary-content">
    <div class="card-body">
      <h2 class="card-title">{file.path}</h2>
      {#each file.hashes as hash}
        <p> <b>{hash.algo} </b> | {hash.hash} </p>
      {/each}
    </div>
  </div>
</div>
