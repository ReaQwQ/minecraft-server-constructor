import { motion } from "framer-motion";
import { useTranslation } from "react-i18next";
import { ShieldCheck, Terminal, Lock } from "lucide-react";
import { Switch } from "../ui/Switch";

const pageVariants = {
  initial: { opacity: 0, y: 10 },
  animate: { opacity: 1, y: 0 },
  exit: { opacity: 0, y: -10 }
};

export function SecurityTab() {
  const { t } = useTranslation();

  return (
    <motion.div 
      key="security" 
      variants={pageVariants}
      initial="initial" 
      animate="animate" 
      exit="exit"
      className="max-w-5xl mx-auto space-y-10 text-app"
    >
      <div className="flex flex-col items-center text-center space-y-2 text-app">
        <div className="bg-red-600/10 p-4 rounded-3xl text-red-600">
          <ShieldCheck className="w-8 h-8" />
        </div>
        <h3 className="text-3xl font-black tracking-tighter text-app">{t('security.title')}</h3>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-2 gap-8 text-app">
        <div className="glass p-8 rounded-[2.5rem] space-y-6 text-app">
          <div className="flex items-center gap-3 text-app">
            <Terminal className="w-5 h-5 text-red-600" />
            <h4 className="text-lg font-black text-app">{t('security.ssh_access')}</h4>
          </div>
          <div className="space-y-4 text-app">
            {[
              { label: t('security.ssh_key'), val: true },
              { label: t('security.root_login'), val: true },
              { label: t('security.password_auth'), val: false }
            ].map(s => (
              <div key={s.label} className="flex items-center justify-between p-4 bg-app/50 rounded-2xl border border-app text-app">
                <span className="text-sm font-bold text-app">{s.label}</span>
                <Switch checked={s.val} />
              </div>
            ))}
          </div>
        </div>
        <div className="glass p-8 rounded-[2.5rem] space-y-6 text-app">
          <div className="flex items-center gap-3 text-app">
            <Lock className="w-5 h-5 text-red-600" />
            <h4 className="text-lg font-black text-app">{t('security.firewall')}</h4>
          </div>
          <div className="space-y-4 text-app">
            {[
              { label: t('security.backend_block'), val: true },
              { label: t('security.proxy_only'), val: true }
            ].map(s => (
              <div key={s.label} className="flex items-center justify-between p-4 bg-app/50 rounded-2xl border border-app text-app">
                <span className="text-sm font-bold text-app">{s.label}</span>
                <Switch checked={s.val} />
              </div>
            ))}
          </div>
        </div>
      </div>
    </motion.div>
  );
}
