import { useConfig } from '../store/config';
import { getConfigFile } from '../tauri/config-file';

export async function loadConfigFile(): Promise<boolean> {
  const store = useConfig();

  try {
    store.config = await getConfigFile();
  } catch (err) {
    store.isNewUser = true;
  }

  return store.isNewUser;
}
