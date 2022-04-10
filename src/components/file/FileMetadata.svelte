<script lang="ts">
  import type { FileMetaData } from "./models/file.model";
  import { createEventDispatcher } from "svelte";
  const dispatch = createEventDispatcher();

  export let metadata: Promise<FileMetaData>;

  function clearPath() {
    dispatch("clear");
  }
</script>

<div>
  {#if metadata}
    {#await metadata then metadata}

    <div class="block md:hidden">
      <p class="text-xl truncate w-64">{metadata.name}</p>
    </div>

      <div class="hidden md:block">
        <!-- hide stat on small screens -->
        <div class="indicator">
          <div class="indicator-item indicator-middle indicator-end">
            <button class="btn btn-xs" on:click={clearPath}>X</button>
          </div>
  
          <div class="stats shadow bg-base-300 w-72">
            <div class="stat">
              <div class="stat-title">{metadata.size}</div>
              <div class="stat-value text-lg truncate">{metadata.name}</div>
              <div class="stat-desc">
                <div>Last Modified |</div>
                {metadata.last_modified}
              </div>
            </div>
          </div>
        </div>
      </div>
    {:catch}
      <p class="hidden">No file was chosen yet.</p>
    {/await}
  {/if}
</div>
