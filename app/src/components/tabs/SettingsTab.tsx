import { motion } from "framer-motion";
import { useTranslation } from "react-i18next";
import { Sliders, Trash2 } from "lucide-react";
import { Switch } from "../ui/Switch";
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "../ui/Select";
import { Button } from "../ui/Button";

const pageVariants = {
  initial: { opacity: 0, y: 10 },
  animate: { opacity: 1, y: 0 },
  exit: { opacity: 0, y: -10 }
};

export function SettingsTab() {
  const { t } = useTranslation();

  return (
    <motion.div 
      key="settings" 
      variants={pageVariants}
      initial="initial" 
      animate="animate" 
      exit="exit"
      className="max-w-4xl mx-auto space-y-10 text-app"
    >
      <div className="flex items-center gap-4 text-app">
        <Sliders className="w-6 h-6 text-blue-600" />
        <h3 className="text-2xl font-black tracking-tight text-app">{t('settings.title')}</h3>
      </div>

      <div className="glass rounded-[2.5rem] overflow-hidden divide-y divide-app text-app">
        <div className="p-8 flex items-center justify-between hover:bg-app/30 transition-all text-app">
          <div>
            <h4 className="font-black text-app">{t('settings.auto_cleanup')}</h4>
            <p className="text-[10px] text-muted font-bold text-app">{t('settings.auto_cleanup_sub')}</p>
          </div>
          <Switch checked={true} />
        </div>

        <div className="p-8 flex items-center justify-between hover:bg-app/30 transition-all text-app">
          <div>
            <h4 className="font-black text-app">{t('settings.output_platform')}</h4>
            <p className="text-[10px] text-muted font-bold text-app">{t('settings.output_platform_sub')}</p>
          </div>
          <Select defaultValue="windows">
            <SelectTrigger className="w-32 h-10 text-xs font-black text-app glass">
              <SelectValue />
            </SelectTrigger>
            <SelectContent className="glass">
              <SelectItem value="windows">Windows (.bat)</SelectItem>
              <SelectItem value="linux">Linux (.sh)</SelectItem>
            </SelectContent>
          </Select>
        </div>

        <div className="p-8 bg-gray-50/50 dark:bg-zinc-900/50 flex items-center justify-between text-app">
          <div>
            <h4 className="font-black text-red-600">{t('settings.danger_zone')}</h4>
            <p className="text-[10px] text-muted font-bold text-app">{t('settings.danger_zone_sub')}</p>
          </div>
          <Button variant="danger" size="sm" className="h-10 rounded-xl px-6 shadow-lg shadow-red-500/20">
            <Trash2 className="w-4 h-4 mr-2" /> {t('settings.reset_btn')}
          </Button>
        </div>
      </div>
    </motion.div>
  );
}

