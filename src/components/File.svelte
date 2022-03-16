<script lang="ts">
  import { fileStore } from '../stores/file.store';
</script>
  
  <div class="container">

  
  <div class="flex justify-center my-6">
    <h class="font-bold underline">---File Selection---</h>  
  </div>
  
    <div class="flex justify-center">
        <button class="btn mx-3" on:click={fileStore.select}>Browse</button>
        <button class="btn mx-3" disabled='{!$fileStore}' on:click={fileStore.reset}>Clear</button>
    </div>

    <div>
      {#if $fileStore}
        Files: 
          <ul>
            {#each $fileStore as file}
              <li>-> {file}</li>
            {/each}
          </ul>
      {:else}
        Select a file.
      {/if}
    </div>
</div>


