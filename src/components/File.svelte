<script lang="ts">
  import { filePath, fileMetaData } from "../stores/file.store";
</script>

<div>
  {#if $filePath}
    {#await $fileMetaData then metadata}


        <div class="stats shadow">
          <div class="stat">
            <div class="stat-title">{metadata.name}</div>
            <div class="stat-value">{metadata.size}</div>
            <div class="stat-desc">
              <div>Last Modified | </div>
              {metadata.last_modified}
            </div>
          </div>
        </div>
      
    {/await}
  {:else}
    Select a file.
  {/if}

  <!-- <button class="btn" on:click="{metadata}">metadata</button> -->
</div>
