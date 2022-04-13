<script lang="ts">
  import Icon from "svelte-awesome";
  import { paste } from "svelte-awesome/icons";
</script>

<div class="indicator w-full">
  <div class="indicator-item indicator-end indicator-top">
    <div class="tooltip tooltip-info" data-tip="Paste">
    <button class="btn btn-sm btn-circle">
      <Icon data={paste} />
    </button>
    </div>
  </div>

  <input
    type="text"
    placeholder="Paste hash to verify"
    class="input input-bordered w-full select-none"
  />
</div>
