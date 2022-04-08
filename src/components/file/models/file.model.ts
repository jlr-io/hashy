import type { OpenDialogOptions } from "@tauri-apps/api/dialog";

export interface FileMetaData {
  name: String,
  path: String,
  file_type: String,
  size: String,
  last_modified: String,
}

// work with only one file for now.
export const dialogOptions: OpenDialogOptions = {
  title: 'Select Files to Hash',
  // multiple: true
}