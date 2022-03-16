import { writable } from 'svelte/store';
import { dialog, } from "@tauri-apps/api";
import type { OpenDialogOptions } from "@tauri-apps/api/dialog";

type Paths = String[]

const dialogOptions: OpenDialogOptions = {
  title: 'Select Files to Hash',
  multiple: true
}

function selectFiles() {
  const { subscribe, set } = writable<Paths>();

  return {
		subscribe,
		reset: () => set(null),
    set: (files) => set(files),
    select: async () => set(Array.from(await dialog.open(dialogOptions))),
	};
}
export const fileStore = selectFiles();