<script lang="ts">
  import Icon from "svelte-awesome";
  import { clipboard } from "svelte-awesome/icons";
</script>

<div class="indicator w-full">
  <span class="indicator-item indicator-middle badge"
    ><button><Icon data={clipboard} /></button></span
  >
  <input
    type="text"
    placeholder="Paste hash to verify"
    class="input input-bordered w-full"
  />
</div>
