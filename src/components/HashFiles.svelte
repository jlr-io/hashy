<script lang="ts">
  import { hash } from '../stores/hash';
  import { file } from '../stores/file';
</script>

<div class="container">

  <div class="flex justify-center my-6">
    <h class="font-bold underline">---Hash Component---</h>  
  </div>

  <div class="flex justify-center">
    <button class="btn mx-3" disabled='{!$file}' on:click={hash.hash}>Hash File</button>
    <button class="btn mx-3" disabled='{!$hash}' on:click={hash.reset}>Clear</button>
  </div>

  <div>
    {#if $hash}
      Hash: {$hash}
    {:else}
      No hash available.
    {/if}
  </div>

</div>
