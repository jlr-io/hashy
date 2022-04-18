<script lang="ts">
  import { fly, scale} from 'svelte/transition';
  import { elasticOut, backInOut } from 'svelte/easing';

  import { clipboard } from "@tauri-apps/api";
  import Icon from "svelte-awesome";
  import { spinner, check, copy, thumbsUp } from "svelte-awesome/icons";
  import { LOADING } from "../models/hash.models";

  export let algorithms: string[];
  export let map: Map<string, string>;
  export let target: string;

  enum ButtonState {
    COPY = "Copy",
    COPIED = "Copied",
  }

  let buttonStates = new Map<string, ButtonState>();

  function isLoading(value: string): boolean {
    return value === LOADING;
  }

  function isTarget(hash: string, target: string): boolean {
    return hash && hash.toLowerCase() === target.toLowerCase();
  }

  const isCopy = (algo: string, state: Map<string, ButtonState>) => 
    getButtonState(algo, state) === ButtonState.COPY;
  
  const onCopy = async (algo: string, hash: string) => {
    setButtonState(algo, ButtonState.COPIED);

    setTimeout(() => {
      setButtonState(algo, ButtonState.COPY);
    }, 1000);
    await clipboard.writeText(hash);
  };

  function setButtonState(algo: string, val: ButtonState): void {
    buttonStates.set(algo, val);
    
    // needed for svelte change detection.
    buttonStates = buttonStates;
  }

  function getButtonState(algo: string, state: Map<string, ButtonState>): string {
    return state.get(algo) ?? ButtonState.COPY;
  }

</script>

<div class="container mx-auto h-[380px] lg:h-[500px] overflow-y-auto">
  {#if algorithms}
    {#each algorithms as algo}
      <div class="h-1/3 lg:h-1/4 border-2 rounded-lg border-base-100 bg-base-300">
        <div class="text-base px-1 py-1 md:text-xl md:px-2 md:py-2">
          <b>{algo}</b>

          <span class="float-right">
            <div
              class="{map.get(algo) ? 'tooltip tooltip-left tooltip-info' : ''}" 
              data-tip={getButtonState(algo, buttonStates)}>
              <button
                class="btn btn-sm btn-circle hidden md:inline-flex"
                class:btn-disabled={!map.get(algo)}
                on:click={() => onCopy(algo, map.get(algo))}>

                {#if isCopy(algo, buttonStates)}
                  <div in:fly="{{y: 10, duration: 250}}"><Icon data={copy} /></div>
                {:else}
                  <div in:fly="{{y: -10, duration: 1000, easing: elasticOut}}" ><Icon data={thumbsUp} /></div>
                {/if}

              </button>
            </div>
          </span>

          <!-- out:fly="{{y: 30, duration: 500}}" -->
          {#if isTarget(map.get(algo), target)}
            <span 
              transition:scale="{{duration: 750, delay: 50, opacity: 0.5, start: 0, easing: backInOut}}" 
              class="hidden md:inline-flex float-right hover:bg-none hover:bg-neutral btn btn-sm btn-circle mr-2 bg-neutral border-none">
                <Icon
                  data={check}
                  scale={1.5}
                  
                  style="color: hsl(var(--su));"
                />
            </span>
          {/if}
        </div>

        {#if map.get(algo) && !isLoading(map.get(algo))}
          <p class="break-all text-sm px-2 md:text-base md:px-4">
            {map.get(algo)}
          </p>
        {:else if isLoading(map.get(algo))}
          <div class="flex justify-center">
            <Icon
              data={spinner}
              pulse
              scale={3}
              style="color: hsl(var(--p));"
            />
          </div>
        {/if}
      </div>
    {/each}
  {/if}
</div>
