<script lang="ts">
  import Icon from "svelte-awesome";
  import {
    archive,
    code,
  } from "svelte-awesome/icons";
  import { createEventDispatcher } from "svelte";

  const dispatch = createEventDispatcher();

  function onHash() {
    dispatch('hash');
  }

  function onBrowse() {
    dispatch('browse');
  }

  export let isLoading: boolean;
  export let disableButton: boolean;
</script>

<div class="tooltip" data-tip="Browse">
  <button
    class="hashy-button btn-circle"
    disabled={isLoading}
    on:click={onBrowse}
  >
    <Icon data={archive} scale={1.25} />
  </button>
</div>

<div class="tooltip" data-tip="Hash">
  <button
    class="hashy-button btn-circle"
    class:loading={isLoading}
    disabled={disableButton}
    on:click={onHash}
  >
    {#if !isLoading}
      <Icon data={code} scale={1.25} />
    {/if}
  </button>
</div>