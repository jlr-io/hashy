import type { OpenDialogOptions } from "@tauri-apps/api/dialog";

export interface FileMetadata {
  name: String,
  path: String,
  file_type: String,
  size: String,
  last_modified: String,
};

export const dialogOptions: OpenDialogOptions = {
  title: 'Select Files to Hash',
};