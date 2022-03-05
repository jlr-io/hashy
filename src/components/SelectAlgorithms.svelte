<script lang="ts">
  let algorithms = ["MD5", "SHA1", "SHA256", "SHA384", "SHA512", "BLAKE3"];

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
