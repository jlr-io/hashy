<script lang="ts">
  // tailwind
  import "./app.css";
  
  // import { Router, Route } from "svelte-navigator";
  // import { Routes } from './models/route.models';
  import Menu from './components/layout/Menu.svelte';
  // import SelectAlgorithm from './pages/SelectAlgorithm.svelte';
  import ComputeHash from "./pages/HashPage/HashPage.svelte";

</script>

<!-- <Router> -->
  <main class="overflow-hidden select-none">
      <Menu>
        <ComputeHash />
        <!-- <Route path="{Routes.selectAlgorithm}" primary="{false}">
          <SelectAlgorithm />
        </Route>
        <Route path="{Routes.computeHash}" primary="{false}">
          
        </Route> -->
      </Menu>
  </main>
<!-- </Router> -->

<style>
  :root {
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Oxygen,
      Ubuntu, Cantarell, "Open Sans", "Helvetica Neue", sans-serif;
  }
</style>
