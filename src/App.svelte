<script lang="ts">
  // tailwind
  import "./app.css";
  
  import { Router, Route } from "svelte-navigator";
  import { Routes } from './components/layout/models/route.models';
  import Menu from "./components/layout/Menu.svelte";
  import SelectAlgorithm from "./pages/SelectAlgorithm.svelte";  
  import ComputeHash from "./pages/ComputeHash.svelte";

</script>

<Router>
  <main>
      <Menu>
        <Route path="{Routes.selectAlgorithm}" primary="{false}">
          <SelectAlgorithm />
        </Route>
        <Route path="{Routes.computeHash}" primary="{false}">
          <ComputeHash />
        </Route>
      </Menu>
  </main>
</Router>

<style>
  :root {
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Oxygen,
      Ubuntu, Cantarell, "Open Sans", "Helvetica Neue", sans-serif;
  }
</style>
