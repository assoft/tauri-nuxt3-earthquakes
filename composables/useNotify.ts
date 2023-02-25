import { isPermissionGranted, requestPermission, sendNotification } from '@tauri-apps/api/notification'

export const pushMessage = async ({ title, body }: { title: string; body: string }) => {
  let permissionGranted = await isPermissionGranted()
  if (!permissionGranted) {
    const permission = await requestPermission()
    permissionGranted = permission === 'granted'
  }
  if (permissionGranted) {
    sendNotification({ title, body })
  }
}
