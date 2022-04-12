<script lang="ts">
  import Icon from "svelte-awesome";
  import { close } from "svelte-awesome/icons";
  import type { FileMetadata } from '../../../models/file.model';
  import { createEventDispatcher } from "svelte";
  const dispatch = createEventDispatcher();

  export let disableButton: boolean;
  export let metadata: Promise<FileMetadata>;

  function clear() {
    dispatch("clear");
  }

  function disableClearButton(hasFilePath: boolean, isAnyLoading: boolean): boolean {
    return !hasFilePath || isAnyLoading;
  }
</script>

<div>
  {#if metadata}
      {#await metadata then metadata}
        <div class="flex">
          <!-- indicator -->
          <div class="indicator">
            <div class="indicator-item indicator-top indicator-end">
              <button class="btn btn-xs" 
                disabled={disableButton} 
                on:click={clear}>
                <Icon
                  data={close}
                  scale={1.25}
                  style="color: hsl(var(--er));"
                /></button
              >
            </div>

            <!-- stat -->
            <div class="stats shadow-md bg-base-300 max-w-72">
              <div class="stat">
                <div class="stat-title hidden md:inline-block">
                  {metadata.size}
                </div>
                <div class="stat-value text-lg truncate">{metadata.name}</div>
                <div class="stat-desc hidden md:inline-block">
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

<style>
  .max-w-72 {
    max-width: 18rem;
  }
</style>
