import { motion, AnimatePresence } from "framer-motion";
import { X, CheckCircle2, AlertCircle, Info, AlertTriangle } from "lucide-react";
import { useNotificationStore } from "../../store/notificationStore";

const icons = {
  success: <CheckCircle2 className="w-5 h-5 text-green-500" />,
  error: <AlertCircle className="w-5 h-5 text-red-500" />,
  warning: <AlertTriangle className="w-5 h-5 text-orange-500" />,
  info: <Info className="w-5 h-5 text-blue-500" />,
};

export function NotificationOverlay() {
  const { notifications, removeNotification } = useNotificationStore();

  return (
    <div className="fixed top-12 right-6 z-[9999] flex flex-col gap-3 w-80 pointer-events-none">
      <AnimatePresence mode="popLayout">
        {notifications.map((n) => (
          <motion.div
            key={n.id}
            layout
            initial={{ opacity: 0, x: 50, scale: 0.9 }}
            animate={{ opacity: 1, x: 0, scale: 1 }}
            exit={{ opacity: 0, x: 20, scale: 0.95, transition: { duration: 0.2 } }}
            className="pointer-events-auto glass rounded-2xl p-4 shadow-2xl flex items-start gap-3 relative overflow-hidden group"
          >
            {/* Liquid Background Pulse */}
            <div className={`absolute top-0 left-0 w-1 h-full ${
              n.type === 'success' ? 'bg-green-500' : 
              n.type === 'error' ? 'bg-red-500' : 
              n.type === 'warning' ? 'bg-orange-500' : 'bg-blue-500'
            }`} />

            <div className="shrink-0 mt-0.5">
              {icons[n.type]}
            </div>

            <div className="flex-1 min-w-0 pr-4">
              {n.title && <h5 className="text-[11px] font-black uppercase tracking-widest opacity-50 mb-1">{n.title}</h5>}
              <p className="text-xs font-bold leading-relaxed">{n.message}</p>
            </div>

            <button
              onClick={() => removeNotification(n.id)}
              className="absolute top-3 right-3 opacity-0 group-hover:opacity-40 hover:!opacity-100 transition-opacity"
            >
              <X className="w-3.5 h-3.5" />
            </button>
          </motion.div>
        ))}
      </AnimatePresence>
    </div>
  );
}
