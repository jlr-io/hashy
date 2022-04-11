<script lang="ts">
  export let algorithms: string[];
  export let selectedAlgorithms: string[];
</script>

<!-- TODO: Search? -->
<div class="grid grid-rows-6 grid-flow-col justify-center align-middle w-full gap-4 my-5 mx-2">
  {#each algorithms as algo}
    <div>
      <input
        type="checkbox"
        class="checkbox checkbox-primary align-middle"
        bind:group={selectedAlgorithms}
        name={algo}
        value={algo}
      />
      <span class="md:ml-2">{algo}</span>
    </div>
  {/each}
</div>