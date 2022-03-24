import { derived, writable } from 'svelte/store';
import { dialog, } from "@tauri-apps/api";
import type { OpenDialogOptions } from "@tauri-apps/api/dialog";

export type Paths = Path[]
export type Path = string;

// work with only one file for now.

const dialogOptions: OpenDialogOptions = {
  title: 'Select Files to Hash',
  // multiple: true
}

function selectFiles() {
  const { subscribe, set } = writable<string>();

  return {
		subscribe,
		reset: () => set(null),
    set: (files) => set(files),
    select: async () => set(await dialog.open(dialogOptions) as string),
	};
}

export const fileStore = selectFiles();

export const fileName = derived(
  fileStore,
  $fileStore => {
    if($fileStore) {
      return $fileStore.split('/').pop();
    }
    return '';
  }
)