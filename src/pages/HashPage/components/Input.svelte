<script lang="ts">
  import { clipboard } from '@tauri-apps/api';
  import Icon from "svelte-awesome";
  import { paste, thumbsUp } from "svelte-awesome/icons";

  enum DataTip {
    PASTE = 'Paste',
    PASTED = 'Pasted'
  };

  const onPaste = async () => {
    dataTip = DataTip.PASTED
    setTimeout(() => dataTip = DataTip.PASTE, 1250);
    target = await clipboard.readText();
  }
  let dataTip = DataTip.PASTE;

  export let target: string = '';
</script>

<div class="indicator w-full">
  <div class="indicator-item indicator-end indicator-top">
    <div class="tooltip tooltip-info" data-tip={dataTip}>
    <button class="btn btn-sm btn-circle" on:click={onPaste}>
      {#if dataTip === DataTip.PASTE}
        <Icon data={paste} />
      {:else}
        <Icon data={thumbsUp} />
      {/if}
    </button>
    </div>
  </div>

  <input
    bind:value={target}
    type="text"
    placeholder="Paste hash to verify"
    class="input input-bordered w-full"
  />
  
</div>

<style>
  input:focus {
    outline: none;
  }
</style>
