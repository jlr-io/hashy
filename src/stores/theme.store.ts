import { writable } from "svelte/store";

export enum Theme {
  light = 'light',
  dark = 'dark'
};

const dataTheme = document.createAttribute('data-theme');
const html = document.getElementById('globalhtml');

export const theme = writable(Theme.light);
export const setTheme = (val: Theme) => theme.set(val);

export function changeTheme(theme: Theme) {
  dataTheme.value = theme
  html.setAttributeNode(dataTheme);
};
