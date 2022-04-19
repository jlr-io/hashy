<script lang="ts">
  import { fly } from 'svelte/transition';
  import { elasticOut } from 'svelte/easing';
  import { clipboard } from '@tauri-apps/api';
  import Icon from "svelte-awesome";
  import { paste, thumbsUp } from "svelte-awesome/icons";

  enum ButtonState {
    PASTE = 'Paste',
    PASTED = 'Pasted'
  };

  const onPaste = async () => {
    buttonState = ButtonState.PASTED
    setTimeout(() => buttonState = ButtonState.PASTE, 1250);
    target = await clipboard.readText();
  };
  let buttonState = ButtonState.PASTE;

  export let target: string = '';
</script>

<div class="indicator w-full">
  <div class="indicator-item indicator-end indicator-top">
    <div class="tooltip tooltip-info" data-tip={buttonState}>
    <button class="btn btn-sm btn-circle" on:click={onPaste}>
      {#if buttonState === ButtonState.PASTE}
        <div in:fly="{{y: 10, duration: 250}}"><Icon data={paste} /></div>
      {:else}
        <div in:fly="{{y: -10, duration: 1000, easing: elasticOut}}"><Icon data={thumbsUp} /></div>  
      {/if}
    </button>
    </div>
  </div>

  <input
    bind:value={target}
    type="text"
    placeholder="Paste hash to verify"
    class="input input-bordered w-full h-14"
  />
  
</div>

<style>
  input:focus {
    outline: none;
  }
</style>
