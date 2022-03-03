<script lang="ts">
  import { file } from '../stores/files';
</script>

<div class="container">
  
  <div class="flex justify-center my-6">
    <h class="font-bold underline">---File Selection---</h>  
  </div>
  
    <div class="flex justify-center">
        <button class="btn-primary" on:click={file.select}>Browse</button>
        <button class="btn-primary disabled:btn-primary-disabled mx-3" disabled='{!$file}' on:click={file.reset}>Clear</button>
    </div>

    <div>
      {#if $file}
        File: {$file}
      {:else}
        Select a file.
      {/if}
    </div>

</div>


