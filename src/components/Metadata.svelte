<script lang="ts">
  import Icon from "svelte-awesome";
  import { close } from "svelte-awesome/icons";
  import { fade } from 'svelte/transition';
  import type { FileMetadata } from "../models/file.model";
  import { createEventDispatcher } from "svelte";
  const dispatch = createEventDispatcher();

  export let hasFilePath: boolean;
  export let disableButton: boolean;
  export let metadata: Promise<FileMetadata>;

  function clear() {
    dispatch("clear");
  };
</script>

<div>
  {#if hasFilePath}
    <!-- indicator -->
    <div in:fade="{{duration: 750}}" class="indicator">
      <div class="indicator-item indicator-top indicator-end">
        <div class="tooltip tooltip-right tooltip-error" data-tip="Clear">
          <button
            class="btn btn-xs btn-circle"
            disabled={disableButton}
            on:click={clear}
          >
            <Icon
              data={close}
              scale={1}
              style="color: hsl(var(--er));"
            /></button
          >
        </div>
      </div>

      <!-- stat -->
      <div class="stats shadow-md bg-base-300 max-w-72">
        <div class="stat">
          {#await metadata then metadata}
            <div class="stat-title hidden md:inline-block">
              {metadata.size}
            </div>
            <div class="stat-value text-lg truncate">{metadata.name}</div>
            <div class="stat-desc hidden md:inline-block">
              <div>Last Modified | {metadata.last_modified}</div>
            </div>
          {:catch}
            <p class="hidden">No file was chosen yet.</p>
          {/await}
        </div>
      </div>
    </div>
  {:else}
    <div in:fade="{{duration: 750}}" class="stats shadow-md bg-base-300 max-w-72">
      <div class="stat">
        <p class="text-lg text-center">
          Press Browse or Drag File
        </p>
      </div>
    </div>
  {/if}
</div>

<style>
  .max-w-72 {
    max-width: 18rem;
  }
</style>
