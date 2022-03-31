<script lang="ts">
  import Theme from "./Theme.svelte";
  import HasherToolbar from "./HasherToolbar.svelte";
  import AlgoToolbar from "./AlgoToolbar.svelte";
  import { link, Link, useLocation } from "svelte-navigator";
  import type { NaviagorLocation } from 'svelte-navigator';
  import { Routes } from "../../models/route.models";

  const location = useLocation();

  const isLocationHasher = (location: NavigatorLocation) => location.pathname === Routes.hasher;
</script>

<!-- navbar -->
<header class="select-none">
  <div class="navbar bg-neutral text-neutral-content mb-0">
    <div class="flex-none">
      <!-- ❮ -->
      <!-- ❯ -->
      <button class="btn text-xl" disabled={!isLocationHasher($location)}> 
        <a use:link href="/">❮</a>
      </button>
      <button class="btn text-xl" disabled={isLocationHasher($location)}>
        <a use:link href="/hasher">❯</a>
        <!-- <Link to="/hasher">❯</Link> -->
      </button>

    </div>
    <div class="flex-1">
      {#if isLocationHasher($location)}
        <HasherToolbar />
      {:else}
        <AlgoToolbar />
      {/if}
      <!-- <h1 class="normal-case text-xl ml-2">Hashy!!</h1> -->
    </div>

    

    <div class="flex-none">
      <Theme />
    </div>
  </div>
</header>
