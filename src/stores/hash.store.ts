import { derived, writable } from 'svelte/store';
import { Hash, LOADING } from '../models/hash.models';

const defaultState = new Map<string, string>();

function state() {
  const { subscribe, set, update } = writable<Map<string, string>>(defaultState);

  return {
    subscribe,
    set: (map: Map<string, string>) => set(map),
    reset: () => set(new Map<string, string>()),
    updateMap: (hash: Hash) => update(state => state.set(hash.algo, hash.value))
  }
}

export const store = state();

export const isAnyLoading = derived(store, $store => [...$store.values()].includes(LOADING));