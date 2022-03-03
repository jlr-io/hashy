import { derived, get, writable } from 'svelte/store';
import { dialog } from "@tauri-apps/api";

export type FileType = string | string[]

function fileSelection() {
  const { subscribe, set } = writable<FileType>(null);

  return {
		subscribe,
		reset: () => set(null),
    set: (file) => set(file),
    select: async () => set(await dialog.open()),
	};
}
export const file = fileSelection();
// export const hash = derived(file, async $file => await invoke<string>("hash_command", { path: $file, algo: 'SHA256' }).then(hash => hash) )

// export const hash = derived(file, async $file => {
//   console.log('derived');
//   return (await invoke<string>("hash_command", { path: $file, algo: 'SHA256' }).then(hash => hash));
// }).then(val => val);