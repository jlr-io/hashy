<script lang="ts">
  // tailwind
  import "./app.css";
  import { Router, Route, Link, useLocation } from "svelte-navigator";
  import File from "./components/File.svelte";
  import Algorithm from "./components/Algorithm.svelte";
  import Hash from "./components/Hash.svelte";
  import Menu from "./components/layout/Menu.svelte";
  import Toolbar from "./components/Layout/HasherToolbar.svelte";
  import Hasher from "./components/Hasher.svelte";
  import { Routes } from "./models/route.models";
  // import Event from './components/Event.svelte';

  // const location = useLocation();
  // $: console.log($location);

</script>

<Router>
  <main>
      <Menu>
        <Route path="{Routes.algorithm}" component="{Algorithm}" primary="{false}"></Route>
        <Route path="{Routes.hasher}" component="{Hasher}" primary="{false}"></Route>
      </Menu>
  </main>
</Router>

<style>
  :root {
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Oxygen,
      Ubuntu, Cantarell, "Open Sans", "Helvetica Neue", sans-serif;
  }
</style>
