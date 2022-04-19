<script lang="ts">
  import Icon from "svelte-awesome";
  import {
    fileCodeO,
    folderOpen,
  } from "svelte-awesome/icons";
  import Modal from "./Modal.svelte";
  import { createEventDispatcher } from "svelte";

  export let algorithms: string[];
  export let selectedAlgorithms: string[];
  export let isLoading: boolean;
  export let disableButton: boolean;

  const dispatch = createEventDispatcher();

  function onCompute() {
    dispatch('compute');
  };

  function onBrowse() {
    dispatch('browse');
  };
</script>

<div class="tooltip align-top" data-tip="Algorithms">
  <Modal 
    {algorithms}
    {isLoading}
    bind:selectedAlgorithms={selectedAlgorithms}
  />
</div>

<div class="tooltip align-top" data-tip="Browse">
  <button
    class="btn btn-lg mx-1 btn-circle shadow-lg"
    disabled={isLoading}
    on:click={onBrowse}
  >
    <Icon data={folderOpen} scale={1.75}/>
  </button>
</div>

<div class="tooltip align-top" data-tip="Compute">
  <button
    class="btn btn-lg mx-1  btn-circle shadow-lg"
    class:loading={isLoading}
    disabled={disableButton}
    on:click={onCompute}
  >
    {#if !isLoading}
      <Icon data={fileCodeO} scale={1.75} />
    {/if}
  </button>
</div>