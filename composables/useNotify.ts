import { isPermissionGranted, requestPermission, sendNotification } from '@tauri-apps/api/notification'

export const pushMessage = async ({ title, body, icon = 'icon' }: { title: string; body: string; icon?: string }) => {
  let permissionGranted = await isPermissionGranted()
  if (!permissionGranted) {
    const permission = await requestPermission()
    permissionGranted = permission === 'granted'
  }
  if (permissionGranted) {
    sendNotification({ title, body, icon })
  }
}
