<script lang="ts">
  // tailwind
  import "./app.css";
  import SelectFiles from "./components/SelectFiles.svelte";
  import SelectAlgorithms from "./components/SelectAlgorithms.svelte";
  import HashFiles from "./components/HashFiles.svelte";
</script>

<main>
  <div>
    <SelectFiles />
    <SelectAlgorithms />
    <HashFiles />
  </div>
</main>

<style>
  :root {
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Oxygen,
      Ubuntu, Cantarell, "Open Sans", "Helvetica Neue", sans-serif;
  }
</style>
