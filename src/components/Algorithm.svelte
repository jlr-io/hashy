<script lang="ts">
// import { hashStore } from "src/stores/hash.store";

  import { algorithmStore, algorithms } from "../stores/algorithm.store";
  // $: selected, algorithmStore.set(selected);

  // let algorithms = [
  //   { name: "MD5" },
  //   { name: "SHA3", variation: ["SHA3-224", "SHA3-256"] },
  // ];

  // let favorites: string[] = [];
  // let selected: string[] = $algorithmStore;
  let checked = false;

  // let arr = checked ? favorites : selected;


  
</script>

<div class="text-xl flex justify-center my-5">Select Hashing Algorithms</div>

<!-- <label class="swap swap-flip text-9xl">
  
  <input type="checkbox" bind:checked="{checked}"/>
  
  <div class="swap-on">😈</div>
  <div class="swap-off">😇</div>
</label>

{checked} -->

<!-- TODO: Search? -->
<!-- <div class="flex align-middle"> -->
  <div
    class="grid grid-rows-6 grid-flow-col justify-center align-middle w-full gap-4 my-5 mx-2"
  >
    {#each algorithms as algo}
      <div>
        <input
          type="checkbox"
          class="checkbox align-middle"
          bind:group={$algorithmStore}
          name={algo}
          value={algo}
        />
        <span class="md:ml-2">{algo}</span>
      </div>
    {/each}

</div>
