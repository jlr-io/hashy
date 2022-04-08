<script lang="ts">
  import Theme from "../theme/components/Theme.svelte";
  import { link, NavigatorLocation } from "svelte-navigator";
  import { Routes } from '../layout/models/route.models';
  import { useLocation } from "svelte-navigator";

  const location = useLocation();

  const isFirstPage = (location: NavigatorLocation) => location.pathname === Routes.selectAlgorithm;
</script>

<!-- navbar -->
<header class="select-none">
  <div class="navbar bg-neutral text-neutral-content mb-0">
    <div class="flex-none">
      <!-- ❮ -->
      <!-- ❯ -->
      <button class="btn text-xl" disabled={isFirstPage($location)}>
        <a use:link href="{Routes.selectAlgorithm}">❮</a>
      </button>
      <button class="btn text-xl" disabled={!isFirstPage($location)}>
        <a use:link href="{Routes.computeHash}">❯</a>
      </button>
    </div>
    <div class="flex-1">
      HASHY!!
    </div>

    <div class="flex-none">
      <Theme />
    </div>
  </div>
</header>
