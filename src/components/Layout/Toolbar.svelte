<script lang="ts">
  import { fileStore } from '../../stores/file.store';
  import { hashStore } from '../../stores/hash.store';
  import { algorithmStore } from '../../stores/algorithm.store';

  function disableHashButton(file: string, algos: string[]): boolean {
    return !file || !algos?.length;
  }

  // const onClear = () => {fileStore.reset; hashStore.reset;}
</script>

<div class="flex justify-center my-3">
  <button class="btn mx-3" on:click={fileStore.select}> Browse </button>
  <button class="btn mx-3" disabled={!$fileStore} on:click={fileStore.reset}>
    Clear
  </button>

  <button
    class="btn mx-3"
    disabled={disableHashButton($fileStore, $algorithmStore)}
    on:click={hashStore.hashFiles}>
    Hash
  </button>
</div>