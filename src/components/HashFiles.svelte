<script lang="ts">
  import { hash } from '../stores/hashes';
  import { file } from '../stores/files';
</script>

<div class="container">

  <div class="flex justify-center my-6">
    <h class="font-bold underline">---Hash Component---</h>  
  </div>

  <div class="flex justify-center">
    <button class="btn" disabled='{!$file}' on:click={hash.hash}>Hash File</button>
    <button class="btn" disabled='{!$hash}' on:click={hash.reset}>Clear</button>
  </div>

  <div>
    {#if $hash}
      Hash: {$hash}
    {:else}
      No hash available.
    {/if}
  </div>

</div>
