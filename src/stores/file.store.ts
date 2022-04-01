import { derived, writable } from 'svelte/store';
import { dialog, invoke } from "@tauri-apps/api";
import type { OpenDialogOptions } from "@tauri-apps/api/dialog";
import { hashStore } from './hash.store';

export type Paths = Path[]
export type Path = string;

export interface FileMetaData {
  name: String,
  path: String,
  file_type: String,
  size: String,
  last_modified: String,
}

// work with only one file for now.

const dialogOptions: OpenDialogOptions = {
  title: 'Select Files to Hash',
  // multiple: true
}

function selectFiles() {
  const { subscribe, set } = writable<string>();

  return {
		subscribe,
		reset: () => { hashStore.set(null); set(null); },
    set: (files) => set(files),
    select: async () => set(await dialog.open(dialogOptions) as string),
	};
}

export const filePath = selectFiles();

export const fileMetaData = derived(
  filePath,
  async $filePath => await invoke<FileMetaData>("get_file_metadata", {path: $filePath})
)
