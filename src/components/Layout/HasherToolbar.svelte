<script lang="ts">
  import { filePath } from '../../stores/file.store';
  import { hashStore } from '../../stores/hash.store';
  import { algorithmStore } from '../../stores/algorithm.store';

  function disableHashButton(file: string, algos: string[]): boolean {
    return !file || !algos?.length;
  }

</script>

<div class="container">
    <button class="btn mx-3" on:click={filePath.select}> Browse </button>
    <button
      class="btn mx-3"
      disabled={disableHashButton($filePath, $algorithmStore)}
      on:click={hashStore.hashFiles}>
      Hash
    </button>
</div>