import { get, writable } from 'svelte/store';
import { invoke } from "@tauri-apps/api";
import { fileStore } from './file.store';
import { algorithmStore } from './algorithm.store';

type FileHash = {
  path: String,
  hashes: Hash[]
};

type Hash = {
  algo: String,
  hash: String,
}

function hashFiles() {
  const { subscribe, set } = writable<FileHash[]>();

  return {
    subscribe,
    reset: () => set(null),
    set: (hash) => set(hash),
    hashCommand: async () => set(await invoke<FileHash[]>("hash_command", { paths: get(fileStore), algos: get(algorithmStore) }))
  }
}

export const hashStore = hashFiles();