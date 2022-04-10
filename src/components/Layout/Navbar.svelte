<script lang="ts">
  import Theme from "../theme/components/Theme.svelte";
  import { Link, link, NavigatorLocation } from "svelte-navigator";
  import { Routes } from '../layout/models/route.models';
  import { useLocation } from "svelte-navigator";

  const location = useLocation();

  const isFirstPage = (location: NavigatorLocation) => location.pathname === Routes.selectAlgorithm;

  const btnClass = 'text-xl mx-2 btn btn-sm btn-base-200'

  const btnDisabledClass = 'text-xl mx-2 btn btn-sm btn-disabled'
</script>

<!-- bg-neutral text-neutral-content -->

<header class="select-none">
  <div class="navbar bg-base-300 mb-0">
    <div class="flex-none">
      <!-- ❮ -->
      <!-- ❯ -->

      <a use:link href="{Routes.selectAlgorithm}" role="button" class="{!isFirstPage($location) ? btnClass : btnDisabledClass}">❮</a>
      <a use:link href="{Routes.computeHash}" role="button" class="{isFirstPage($location) ? btnClass : btnDisabledClass}">❯</a>

    </div>
    <div class="flex-1">
      <!-- app title goes here. -->
    </div>

    <div class="flex-none">
      <Theme />
    </div>
  </div>
</header>
