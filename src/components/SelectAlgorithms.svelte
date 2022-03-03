<script lang="ts">

  let algorithms = [
    {
      name: "SHA256",
    },
  ];

  let selected = [];

  // function handleCheck(val: any): void {
  //   selected.push(val);
  // }
</script>

<div>
  {#each algorithms as algorithm}
    <!-- <FormField>
      <Checkbox bind:group={selected} value={algorithm.name}/>
      <span slot="label">{algorithm.name}</span>
    </FormField> -->

    <!-- <checkbox checked=handleCheck() value={algorithm.name}></checkbox>
     -->
     <input type="checkbox" bind:group={selected} name={algorithm.name} value="{algorithm.name}">
     <span>{algorithm.name}</span>

  {/each}

  selected: {selected}
</div>
