import { get, writable } from 'svelte/store';
import { invoke } from "@tauri-apps/api";
import { file } from './files';

function hashFile() {
  const { subscribe, set } = writable<string>();

  return {
    subscribe,
    reset: () => set(null),
    set: (hash) => set(hash),
    hash: async () => set(await invoke<string>("hash_command", { path: get(file), algo: 'SHA256' }).then(hash => hash))
  }
}

export const hash = hashFile();