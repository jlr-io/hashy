export interface Hash {
  algo: string,
  value: string,
}

export const LOADING = 'loading';

export type Hashes = Hash[];

export type MapType = Map<string, string>;
