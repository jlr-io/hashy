import { derived, writable } from 'svelte/store';
import { defaultAlgorithms } from '../models/algorithms.models';

function state() {
  const { subscribe, set } = writable<string[]>(defaultAlgorithms);

  return {
    subscribe,
    reset: () => set(defaultAlgorithms),
    set: (algorithms: string[]) => set(algorithms)
  }
}

export const store = state();

export const hasAlgorithms = derived(store, $store => !!$store.length);