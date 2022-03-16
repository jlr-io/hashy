<script lang="ts">
  // tailwind
  import './app.css';
  import File from './components/File.svelte';
  import Algorithm from './components/Algorithm.svelte';
  import Hash from './components/Hash.svelte';
  import Theme from './components/Theme.svelte';
  import Event from './components/Event.svelte';
</script>

<main>
  <div>
    <Theme />
    <Event />
    <File />
    <Algorithm />
    <Hash />
  </div>
</main>

<style>
  :root {
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Oxygen,
      Ubuntu, Cantarell, "Open Sans", "Helvetica Neue", sans-serif;
  }
</style>
