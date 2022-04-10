import { derived, writable } from 'svelte/store';
import { defaultAlgorithms } from '../models/algorithms.models';
import { store as hashStore } from '../../hash/store/hash.store';

// TODO: default state.

function state() {
  const { subscribe, set } = writable<string[]>(defaultAlgorithms);

  return {
    subscribe,
    reset: () => set(defaultAlgorithms),
    set: (algorithms: string[]) => {
      // hashStore.setAlgos(algorithms);
      // console.log('set');
      set(algorithms);
    }
  }
}

export const store = state();

export const hasAlgorithms = derived(store, $store => !!$store.length)