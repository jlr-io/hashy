<script lang="ts">
  import Navbar from "./Navbar.svelte";
  import Footer from "./Footer.svelte";
  import Toolbar from "./HasherToolbar.svelte";
</script>

<Navbar />
<slot />

<div class="hidden md:flex w-full fixed bottom-0">
  <!-- <Toolbar /> -->
  <Footer />
</div>

<style>
  /* .drawer-toggle:checked ~ .drawer-side > .drawer-overlay + * {
    height: --webkit-fill-available;
  } */
</style>
