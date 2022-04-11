<script lang="ts">
  import AlgorithmCheckboxes from "../components/algorithm/AlgorithmCheckboxes.svelte";
  import { algorithms } from "../components/algorithm/models/algorithms.models";
  import { store as algorithmStore } from "../components/algorithm/store/algorithm.store";
  import { Link } from "svelte-navigator";
  import { Routes } from "../components/layout/models/route.models";
</script>
<!-- <span>
  <AlgorithmCheckboxes
    {algorithms}
    bind:selectedAlgorithms={$algorithmStore}
  />
</span>

<div class="toolbar">
  <Link to={Routes.computeHash}><button class="btn mx-3"> Next </button></Link>
</div> -->

<!-- header and buttons -->
<div class="flex flex-wrap mb-4">
  <!-- file -->
  <div class="flex justify-center items-center h-20 w-full md:w-1/2 md:pt-5 md:mb-0 md:h-32">
    <p class="text-xl text-center">
      Select Hashing Algorithms
    </p>
  </div>

  <!-- buttons -->
  <div class="flex flex-wrap justify-center items-center w-full md:w-1/2 h-8 md:pt-5 md:h-32">
    <Link to={Routes.computeHash}><button class="btn mx-3"> Next </button></Link
    >
  </div>
</div>

<!-- checkboxes -->
<div class="flex mb-4">
  <div class="container mx-auto h-[380px] lg:h-[500px] overflow-y-auto">

    <!-- rid grid-cols-2 md:grid-rows-6 grid-flow-col justify-center align-middle w-full gap-4 my-5 mx-2 -->
    <div class="grid grid-cols-2 grid-rows-none md:grid-cols-none md:grid-flow-col md:grid-rows-6">
      {#each algorithms as algo}
        <div class="m-1 md:m-2">
          <input
            type="checkbox"
            class="checkbox checkbox-primary align-middle"
            bind:group={$algorithmStore}
            name={algo}
            value={algo}
          />
          <span class="align-middle">{algo}</span>
        </div>
      {/each}
    </div>
  </div>
</div>
