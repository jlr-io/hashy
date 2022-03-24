<script lang="ts">
  import { algorithmStore, algorithms } from "../stores/algorithm.store";
  $: selected, algorithmStore.set(selected);

  // let algorithms = [
  //   { name: "MD5" },
  //   { name: "SHA3", variation: ["SHA3-224", "SHA3-256"] },
  // ];

  let selected = [];
</script>

<div class="grid grid-cols-5 gap-4 my-5">
  {#each algorithms as algo}
    <div> 
      <input
      type="checkbox"
      class="checkbox align-middle"
      bind:group={selected}
      name={algo}
      value={algo}
    />
    <span>{algo}</span>
    </div>
  {/each}
</div>

<!-- <ul>
  {#each algorithms as algorithm}
    <li>
      <input
        type="checkbox"
        class="checkbox align-middle"
        bind:group={selected}
        name={algorithm}
        value={algorithm}
      />
      <span>{algorithm}</span>
    </li>
  {/each}
</ul>
selected: {$algorithmStore} -->

<!-- </div> -->
<style>
  /* li {
    width: 100px;
  }

  ul > li {
    margin-left: 10px;
} */
</style>
