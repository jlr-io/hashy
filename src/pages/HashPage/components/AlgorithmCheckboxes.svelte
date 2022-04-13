<script lang="ts">
  export let algorithms: string[];
  export let selectedAlgorithms: string[];
</script>

<!-- TODO: Search? -->
<!-- <div class="container mx-auto h-[380px] lg:h-[500px] overflow-y-auto"> -->
  <!-- "grid grid-cols-2 grid-rows-none md:grid-cols-none md:grid-flow-col md:grid-rows-6" -->
  <div class="grid grid-cols-1 grid-rows-none md:grid-cols-none md:grid-flow-col md:grid-rows-6">
    {#each algorithms as algo}
      <div class="mb-1">
        <input
          type="checkbox"
          class="checkbox checkbox-primary align-middle"
          bind:group={selectedAlgorithms}
          name={algo}
          value={algo}
        />
        <span>{algo}</span>
      </div>
    {/each}
  </div>
<!-- </div> -->
