import { invoke } from '@tauri-apps/api/tauri';
import { BaseDirectory, readTextFile, writeFile, createDir } from '@tauri-apps/api/fs';
import { open } from '@tauri-apps/api/dialog';

export enum ConfigError {
  DB_FILE_NOT_FOUND,
  CANT_OPEN_DB_FILE,
  CANT_CREATE_DB_TABLES,
  NEW_USER,
  NULL_DATA_DIR
}

export type AppConfig = {
  dataDir: string,
  dbFile: string
};

const configFilePath = 'consolidacao.config.json';
const dbFileName = 'consolidacao.db';
let config: AppConfig = null;

export async function checkConfig(): Promise<void> {
  try {
    config = await readTextFile(configFilePath, { dir: BaseDirectory.Resource });
  } catch (error) {
    if (error.indexOf('No such file or directory') > -1) {
      throw ConfigError.NEW_USER;
    }
    throw error
  }
}

export async function openDataDirSelectionDialog(): Promise<string|string[]> {
  return await open({
    directory: true,
    multiple: false
  }).then(result => {
    return result != null ? result : Promise.reject(ConfigError.NULL_DATA_DIR);
  });
}

export async function createConfigFile(dataDir: string): Promise<void> {
  config = {
    dataDir,
    dbFile: `${dataDir}/${dbFileName}`
  }

  await createDir(dataDir, { recursive: true });
  await writeFile({
    contents: '',
    path: config.dbFile
  });

  await writeFile({
    path: configFilePath,
    contents: JSON.stringify(config, null, 2)
  }, {
    dir: BaseDirectory.Resource,
  });
}

export async function createDatabase(): Promise<void> {
  try {
    return await invoke('create_database', { dbFile: config.dbFile });
  } catch (error) {
    throw ConfigError[error];
  }
}

