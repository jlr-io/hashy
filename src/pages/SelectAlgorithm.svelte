<script lang="ts">
  import AlgorithmCheckboxes from "../components/algorithm/AlgorithmCheckboxes.svelte";
  import { algorithms } from "../components/algorithm/models/algorithms.models";
  import { selectedAlgorithms } from "../components/algorithm/store/algorithm.store";
  import { Link } from "svelte-navigator";
  import { Routes } from "../components/layout/models/route.models";
</script>

<div class="text-xl flex justify-center py-5">Select Hashing Algorithms</div>

<span>
  <AlgorithmCheckboxes
    {algorithms}
    bind:selectedAlgorithms={$selectedAlgorithms}
  />
</span>

<div class="toolbar">
  <Link to={Routes.computeHash}><button class="btn mx-3"> Next </button></Link>
</div>
