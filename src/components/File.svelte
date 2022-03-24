<script lang="ts">
  import { fileStore, fileName } from "../stores/file.store";
</script>

<div>
  {#if $fileStore}
    You have selected file: <b>{$fileName}</b>
  {:else}
    Select a file.
  {/if}
</div>
