import { derived, writable } from "svelte/store";


// TODO: create custom themes.

export enum Theme {
  light = 'garden',
  dark = 'night'
}

const dataTheme = document.createAttribute('data-theme');
const html = document.getElementById('globalhtml');

export const theme = writable(Theme.dark);
export const setTheme = (val: Theme) => theme.set(val);

export function changeTheme(theme: Theme) {
  dataTheme.value = theme
  html.setAttributeNode(dataTheme);
}

export const appTheme = derived(
  theme,
  $theme => { return `${$theme}!!`
    
  }
)