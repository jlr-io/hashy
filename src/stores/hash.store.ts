import { get, writable } from 'svelte/store';
import { invoke } from "@tauri-apps/api";
import { fileStore } from './file.store';
import { algorithmStore } from './algorithm.store';

export interface FileHash {
  path: String,
  hashes: Hash[]
};

type Hash = {
  algo: String,
  hash: String,
}

function hashFiles() {
  const { subscribe, set } = writable<FileHash>();

  return {
    subscribe,
    reset: () => set(null),
    set: (hash) => set(hash),
    hashFiles: async () => set(await invoke<FileHash>("hash_files", { path: get(fileStore), algos: get(algorithmStore) }))
  }
}

export const hashStore = hashFiles();