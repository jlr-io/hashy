<script lang="ts">
  let algorithms = [
    "MD5",
    "SHA1",
    "SHA256",
    "SHA384",
    "SHA512",
    // "SHA3-224",
    // "SHA3-256",
    // "SHA3-384",
    // "SHA3-512",
    // "RIPEMD160",
    // "RIPEMD320",
    // "BLAKE2S-256",
    // "BLAKE2B-512",
    // "WHIRLPOOL",
    // "SHABAL-192",
    // "SHABAL-224",
    // "SHABAL-256",
    // "SHABAL-384",
    // "SHABAL-512",
    // "STREEBOG-256",
    // "STREEBOG-512",
    // "TIGER",
    // "SM3",
    // "GROESTL",
    // "GOST",
    // "FSB-160",
    // "FSB-224",
    // "FSB-256",
    // "FSB-384",
    // "FSB-512",
  ];

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
selected: {selected}

<!-- </div> -->
<style>
  /* li {
    width: 100px;
  }

  ul > li {
    margin-left: 10px;
} */
</style>
