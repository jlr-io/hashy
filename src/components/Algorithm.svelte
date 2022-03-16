<script lang="ts">
  import { algorithmStore, algorithms} from '../stores/algorithm.store'
  $: selected, algorithmStore.set(selected);

  // let algorithms = [
  //   { name: "MD5" },
  //   { name: "SHA3", variation: ["SHA3-224", "SHA3-256"] },
  // ];

  let selected = [];
  let showVariation = [];
</script>

<!-- <ul>
  {#each algorithms as algorithm}
    <li>
      {#if algorithm.variation}
        <div class="dropdown">
          <label tabindex="0">
            <div class="form-control">
              <label class="label cursor-pointer">
                <input type="checkbox" class="checkbox" />
                <span class="label-text">{algorithm.name}</span>
              </label>
            </div>
          </label>
          <ul
            tabindex="0"
            class="dropdown-content menu p-2 shadow bg-base-100 rounded-box w-52"
          >
            <li><a>Item 1</a></li>
            <li><a>Item 2</a></li>
          </ul>
        </div>
      {:else}
        <div class="form-control">
          <label class="label cursor-pointer">
            <input
              type="checkbox"
              bind:group={selected}
              name={algorithm.name}
              value={algorithm.name}
              class="checkbox"
            />
            <span class="label-text">{algorithm.name}</span>
          </label>
        </div>
      {/if}
    </li>
  {/each}
</ul> -->

<ul>
    {#each algorithms as algorithm}
      <li>
        <input
          type="checkbox"
          bind:group={selected}
          name={algorithm}
          value={algorithm}/>
        <span>{algorithm}</span>
      </li>
    {/each}
  </ul>
selected: {$algorithmStore}

<!-- </div> -->
<style>
  /* li {
    width: 100px;
  }

  ul > li {
    margin-left: 10px;
} */
</style>
