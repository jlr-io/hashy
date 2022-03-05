import { derived, writable } from "svelte/store";

export enum Theme {
  light = 'light',
  dark = 'dark'
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

// theme = Theme.light;



// const dataTheme = document.createAttribute('data-theme');
// const theme = document.getElementById('globalhtml');
// dataTheme.value = 'light';
// theme.setAttributeNode(dataTheme);
// console.log(theme);
