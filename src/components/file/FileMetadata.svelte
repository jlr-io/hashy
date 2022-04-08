<script lang="ts">
  import type { FileMetaData } from "./models/file.model";
  import { createEventDispatcher } from "svelte";
import { filePath } from "./store/file.store";
  const dispatch = createEventDispatcher();

  export let metadata: Promise<FileMetaData>;

  function clearPath() {
    dispatch("clear");
  }

  function truncateName(name: String): String {
    return name.length < 20 ? name : name.slice(0, 15) + " ...";
  }
</script>

<div class="my-5">
  {#if metadata}
    {#await metadata then metadata}
      <div class="indicator">
        <div class="indicator-item indicator-top">
          <button class="btn btn-xs" on:click={clearPath}>X</button>
        </div>

        <div class="stats shadow">
          <div class="stat">
            <div class="stat-title truncate">{metadata.path}</div>
            <div class="stat-value">{metadata.size}</div>
            <div class="stat-desc">
              <div>Last Modified |</div>
              {metadata.last_modified}
            </div>
          </div>
        </div>
      </div>
    {:catch}
      <p class="hidden">No file was chosen yet.</p>
    {/await}
  {/if}
</div>
