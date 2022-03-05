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
