<script lang="ts">
  import { filePath, fileMetaData } from "../stores/file.store";
  function truncateName(name: String): String {
    return name.length < 20 ? name : name.slice(0, 15) + " ...";
  }
</script>

<div class="my-5">
  {#if $filePath}
    {#await $fileMetaData then metadata}
      <div class="indicator">
        <div class="indicator-item indicator-top">
          <button class="btn btn-xs" on:click={filePath.reset}>X</button>
        </div>

        <div class="stats shadow">
          <div class="stat">
            <div class="stat-title">{truncateName(metadata.name)}</div>
            <div class="stat-value">{metadata.size}</div>
            <div class="stat-desc">
              <div>Last Modified |</div>
              {metadata.last_modified}
            </div>
          </div>
        </div>
      </div>
    {/await}
  {:else}
    <div class="text-xl">Select a file.</div>
  {/if}

  <!-- <button class="btn" on:click="{metadata}">metadata</button> -->
</div>
