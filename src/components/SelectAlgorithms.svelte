<script lang="ts">
  let algorithms = [
    "MD5", 
    "SHA1", 
    "SHA256", 
    "SHA384", 
    "SHA512",
    "SHA3-224",
    "SHA3-256",
    "SHA3-384",
    "SHA3-512",
    "RIPEMD160",
    "RIPEMD320",
    "BLAKE2S-256",
    "BLAKE2B-512", 
    "WHIRLPOOL",
    "SHABAL-192",
    "SHABAL-224",
    "SHABAL-256",
    "SHABAL-384",
    "SHABAL-512",
    "STREEBOG-256",
    "STREEBOG-512",
    "TIGER",
    "SM3",
    "GROESTL",
    "GOST",
    "FSB-160",
    "FSB-224",
    "FSB-256",
    "FSB-384",
    "FSB-512"
  ];

  let selected = [];
</script>

<div>
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
</div>
