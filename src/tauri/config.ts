import { invoke } from '@tauri-apps/api/tauri'

export enum ConfigError {
  CONFIG_DIR_NOT_FOUND,
  DB_FILE_NOT_FOUND,
  BASE_DIRS_EMPTY,
  CANT_CREATE_CONFIG_DIR,
  CANT_OPEN_DB_FILE,
  CANT_CREATE_DB_TABLES
}

export async function checkConfig(): Promise<void> {
  try {
    return await invoke('check_config');
  } catch (error) {
    throw ConfigError[error];
  }
}

export async function getConfigDirPath(): Promise<String> {
  try {
    return await invoke('get_config_dir_path');
  } catch (error) {
    throw ConfigError[error];
  }
}

export async function createConfigDir(): Promise<void> {
  try {
    return await invoke('create_config_dir');
  } catch (error) {
    throw ConfigError[error];
  }
}

export async function createDatabase(): Promise<void> {
  try {
    return await invoke('create_database');
  } catch (error) {
    throw ConfigError[error];
  }
}
