<script lang="ts">
  import Navbar from './Navbar.svelte';
  import Footer from "./Footer.svelte";
</script>

<Navbar />
<div class="viewport relative">
  <slot />
</div>


<div class="hidden md:flex w-full fixed bottom-0">
  <Footer />
</div>

<style>
  .viewport {
    height: calc(100vh - 132px);
  }
</style>