<script lang="ts">
  import { hashStore } from '../stores/hash.store';
  import { fileStore } from '../stores/file.store';
</script>

<div class="container">
  <div class="flex justify-center my-6">
    <h class="font-bold underline">---Hash Component---</h>
  </div>

  <div class="flex justify-center">
    <button class="btn mx-3" disabled={!$fileStore} on:click={hashStore.hashCommand}
      >Hash File</button
    >
    <button class="btn mx-3" disabled={!$hashStore} on:click={hashStore.reset}
      >Clear</button
    >
  </div>

  <div>
    {#if $hashStore}
      hashStore:
      <ul>
        {#each $hashStore as hash}
          <li>
            -> {hash.path}
            <br />
            <ul>
              {#each hash.hashes as hash}
                <li>
                  --> {hash.algo} | {hash.hash}
                </li>
              {/each}
            </ul>
          </li>
        {/each}
      </ul>
    {:else}
      No hashStore available.
    {/if}
  </div>
</div>
