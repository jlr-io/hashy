import { derived, writable } from 'svelte/store';
import { invoke } from "@tauri-apps/api";
import { store as hashStore} from '../../hash/store/hash.store';
import type { FileMetaData } from '../models/file.model';

function state() {
  const { subscribe, set } = writable<string>();

  return {
		subscribe,
		reset: () => { 
      hashStore.reset();
      set(null); 
    },
    set: (file) => set(file),
	};
}

export const store = state();

export const filePath = derived(store, $store => $store);

export const hasSelectedFilePath = derived(store, $store => !!$store);

export const fileMetaData = derived(
  filePath,
  async $filePath => await invoke<FileMetaData>("get_file_metadata", {path: $filePath})
);
