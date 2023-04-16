import { isPermissionGranted, requestPermission, sendNotification } from '@tauri-apps/api/notification';

export class NotificationHandler {
  private permissionGranted: boolean = false;

  constructor() {
    this.checkPermission();
  }

  private async checkPermission(): Promise<void> {
    this.permissionGranted = await isPermissionGranted();
    if (!this.permissionGranted) {
      const permission = await requestPermission();
      this.permissionGranted = permission === 'granted';
    }
  }

  public pushNotification(description: string) {
    if (this.permissionGranted) {
      sendNotification({ title: 'Lean Canvas Editor', body: description });
    } else {
      console.warn('Cannot push notifications. Permission not granted.');
    }
  }
}