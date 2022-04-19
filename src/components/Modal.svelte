<script lang="ts">
  import Icon from "svelte-awesome";
  import { close, list } from "svelte-awesome/icons";
  
  export let algorithms: string[];
  export let selectedAlgorithms: string[];
  export let isLoading: boolean;
</script>

<!-- The button to open modal -->
<label for="modal" class="btn btn-lg mx-1 btn-circle shadow-lg" class:btn-disabled={isLoading}><Icon data={list} scale={1.5}/></label>

<!-- Put this part before </body> tag -->
<input type="checkbox" id="modal" class="modal-toggle">
<div class="modal" id="modal">
  <div class="modal-box max-w-3xl">
    <h3 class="font-bold text-lg text-center mb-4">Choose Hashing Algorithms</h3>
    
    <!-- checkboxes -->
    <div class="grid grid-cols-1 grid-rows-none md:grid-cols-none md:grid-flow-col md:grid-rows-6 text-left">
      {#each algorithms as algo}
        <div class="mb-1">
          <input
            type="checkbox"
            class="checkbox checkbox-primary align-middle"
            bind:group={selectedAlgorithms}
            name={algo}
            value={algo}
          />
          <span>{algo}</span>
        </div>
      {/each}
    </div>
    
    <div class="modal-action">
     <label for="modal" class="btn btn-sm btn-circle absolute right-2 top-2">
        <Icon data={close} />
      </label>

      <label for="modal" class="btn">Close</label>
    </div>
  </div>
</div>
