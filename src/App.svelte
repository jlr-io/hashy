<script lang="ts">
  // tailwind
  import "./app.css";
  import File from "./components/File.svelte";
  import Algorithm from "./components/Algorithm.svelte";
  import Footer from "./components/Layout/Footer.svelte";
  import Navbar from "./components/Layout/Navbar.svelte";
  import Hash from "./components/Hash.svelte";
  // import Theme from './components/Layout/Theme.svelte';
  import Menu from "./components/layout/Menu.svelte";
  import Toolbar from "./components/Layout/Toolbar.svelte";
  // import Event from './components/Event.svelte';
</script>

<main>
  <!-- <div> -->
  <Menu>
    <Navbar />

    <div class="grid grid-cols-4 gap-4">
      <File />
      <div class="col-span-3">
        <Hash />
      </div>
    </div>
    


    <Algorithm />
    <div class="w-full fixed bottom-0">
      <Toolbar />
      <Footer />
    </div>
  </Menu>
</main>

<style>
  :root {
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Oxygen,
      Ubuntu, Cantarell, "Open Sans", "Helvetica Neue", sans-serif;
  }
</style>
