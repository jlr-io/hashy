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

export const selectedAlgorithms = state();

export const hasSelectedAlgorithms = derived(selectedAlgorithms, $selectedAlgorithms => !!$selectedAlgorithms.length)