import { derived, writable } from 'svelte/store';
import type { Hash, MapType } from '../models/hash.models';

const defaultState: MapType = new Map<string, string>();

function state() {
  const { subscribe, set, update } = writable<MapType>(defaultState);

  return {
    subscribe,
    set: (map: MapType) => set(map),
    reset: () => set(new Map<string, string>()),
    updateMap: (hash: Hash) => update(state => state.set(hash.algo, hash.value))
  }
}

export const store = state();