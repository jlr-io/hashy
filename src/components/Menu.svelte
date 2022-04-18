<script lang="ts">
  import Navbar from './Navbar.svelte';
</script>

<Navbar />
<div class="viewport relative">
  <slot />
</div>

<style>
  .viewport {
    height: calc(100vh - 64px);
  }
</style>