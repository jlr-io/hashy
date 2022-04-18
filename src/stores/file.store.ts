import { derived, writable } from 'svelte/store';
import { invoke } from "@tauri-apps/api";
import { store as hashStore} from './hash.store';
import type { FileMetadata } from '../models/file.model';

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

export const hasFilePath = derived(store, $store => !!$store);

export const fileMetadata = derived(
  filePath,
  async $filePath => {
    if($filePath) {
      return await invoke<FileMetadata>("get_file_metadata", {path: $filePath})
    }
    return null;
  }
);
