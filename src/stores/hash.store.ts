import { get, writable } from 'svelte/store';
import { invoke } from "@tauri-apps/api";
import { filePath } from './file.store';
import { algorithmStore } from './algorithm.store';

export interface Hashes {
  path: String,
  value: Hash[]
};

type Hash = {
  algo: String,
  value: String,
}

function hashFiles() {
  const { subscribe, set } = writable<Hashes>();

  return {
    subscribe,
    reset: () => set(null),
    set: (hash) => set(hash),
    hashFiles: async () => set(await invoke<Hashes>("hash_file", { path: get(filePath), algos: get(algorithmStore) }))
  }
}

export const hashStore = hashFiles();