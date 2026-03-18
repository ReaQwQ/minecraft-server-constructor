import { create } from 'zustand';

export type NotificationType = 'info' | 'success' | 'warning' | 'error';

export interface Notification {
  id: string;
  title?: string;
  message: string;
  type: NotificationType;
  duration?: number;
}

interface NotificationStore {
  notifications: Notification[];
  addNotification: (notification: Omit<Notification, 'id'>) => void;
  removeNotification: (id: string) => void;
}

export const useNotificationStore = create<NotificationStore>((set) => ({
  notifications: [],
  addNotification: (notification) => {
    const id = Math.random().toString(36).substring(2, 9);
    set((state) => ({
      notifications: [...state.notifications, { ...notification, id }],
    }));

    if (notification.duration !== 0) {
      setTimeout(() => {
        set((state) => ({
          notifications: state.notifications.filter((n) => n.id !== id),
        }));
      }, notification.duration || 5000);
    }
  },
  removeNotification: (id) =>
    set((state) => ({
      notifications: state.notifications.filter((n) => n.id !== id),
    })),
}));

export const useNotification = () => {
  const addNotification = useNotificationStore((state) => state.addNotification);
  
  return {
    notify: addNotification,
    success: (message: string, title?: string) => addNotification({ message, title, type: 'success' }),
    error: (message: string, title?: string) => addNotification({ message, title, type: 'error' }),
    info: (message: string, title?: string) => addNotification({ message, title, type: 'info' }),
    warn: (message: string, title?: string) => addNotification({ message, title, type: 'warning' }),
  };
};
