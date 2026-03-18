import { useState } from "react";
import { motion } from "framer-motion";
import { useTranslation } from "react-i18next";
import { Box, Loader2, ExternalLink, ChevronLeft, ChevronRight, Check } from "lucide-react";
import { Button } from "../ui/Button";
import { invoke } from "@tauri-apps/api/core";
import { useNotification } from "../../store/notificationStore";

interface SpigotPlugin {
  id: number;
  name: string;
  tag: string;
  icon?: { url: string };
}

interface PluginsTabProps {
  isLoadingPlugins: boolean;
  spigotPlugins: SpigotPlugin[];
  pluginPage: number;
  setPluginPage: (page: number | ((p: number) => number)) => void;
  outputDir: string;
}

const pageVariants = {
  initial: { opacity: 0, y: 10 },
  animate: { opacity: 1, y: 0 },
  exit: { opacity: 0, y: -10 }
};

/**
 * 説明: SpigotMCから取得したプラグインを表示・検索するマーケットプレイス風のタブ
 * @param isLoadingPlugins 読み込み中フラグ
 * @param spigotPlugins プラグインデータのリスト
 * @param pluginPage 現在のページ番号
 * @param setPluginPage ページ更新関数
 * @param outputDir 現在のデプロイ出力先
 * @requires framer-motion, react-i18next, lucide-react, tauri-apps/api/core
 * @return プラグインタブのコンポーネント
 */
export function PluginsTab({
  isLoadingPlugins,
  spigotPlugins,
  pluginPage,
  setPluginPage,
  outputDir
}: PluginsTabProps) {
  const { t } = useTranslation();
  const { success, error } = useNotification();
  const [installingId, setInstallingId] = useState<number | null>(null);
  const [installedIds, setInstalledIds] = useState<number[]>([]);

  const handleInstall = async (plugin: SpigotPlugin) => {
    setInstallingId(plugin.id);
    try {
      await invoke("install_plugin", { 
        id: plugin.id, 
        name: plugin.name, 
        outputDir: outputDir 
      });
      success(t("plugins.install_success", { name: plugin.name }), t("common.success"));
      setInstalledIds(prev => [...prev, plugin.id]);
    } catch (e) {
      error(String(e), t("common.error"));
    } finally {
      setInstallingId(null);
    }
  };

  return (
    <motion.div 
      key="plugins" 
      variants={pageVariants}
      initial="initial" 
      animate="animate" 
      exit="exit"
      className="max-w-5xl mx-auto space-y-10 text-app"
    >
      <div className="flex flex-col items-center text-center space-y-2 text-app">
        <div className="bg-blue-600/10 p-4 rounded-3xl text-blue-600">
          <Box className="w-8 h-8" />
        </div>
        <h3 className="text-3xl font-black tracking-tighter text-app">{t("plugins.title")}</h3>
        <p className="text-sm text-muted font-bold max-w-lg text-app">{t("plugins.subtitle")}</p>
      </div>

      {isLoadingPlugins ? (
        <div className="flex flex-col items-center py-20 gap-4 text-app">
          <Loader2 className="w-10 h-10 animate-spin text-blue-600" />
          <p className="font-black opacity-40">{t("plugins.fetching")}</p>
        </div>
      ) : (
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6 text-app">
          {spigotPlugins.map(p => (
            <div 
              key={p.id} 
              className="glass p-6 rounded-3xl border border-app flex flex-col justify-between hover:border-blue-600/30 transition-all text-app group"
            >
              <div className="flex gap-4">
                {p.icon?.url ? (
                  <img src={p.icon.url} className="w-12 h-12 rounded-xl object-cover bg-white/10" alt={p.name} />
                ) : (
                  <div className="w-12 h-12 rounded-xl bg-blue-600/10 flex items-center justify-center text-blue-600">
                    <Box className="w-6 h-6" />
                  </div>
                )}
                <div className="min-w-0 flex-1">
                  <h4 className="font-black text-sm truncate">{p.name}</h4>
                  <p className="text-[10px] text-muted font-bold mt-1 line-clamp-2">{p.tag}</p>
                </div>
              </div>
              <div className="mt-6 flex items-center justify-between border-t border-app pt-4 text-app">
                <a 
                  href={`https://www.spigotmc.org/resources/${p.id}`} 
                  target="_blank" 
                  rel="noopener noreferrer"
                  className="flex items-center gap-1 opacity-40 hover:opacity-100 text-[10px] font-bold transition-opacity"
                >
                  <ExternalLink className="w-3 h-3" /> {t("common.view")}
                </a>
                <Button 
                  variant={installedIds.includes(p.id) ? "primary" : "secondary"} 
                  size="sm" 
                  disabled={installingId === p.id}
                  onClick={() => handleInstall(p)}
                  className="text-[10px] h-8 rounded-xl font-black text-app min-w-[80px]"
                >
                  {installingId === p.id ? (
                    <Loader2 className="w-3.5 h-3.5 animate-spin" />
                  ) : installedIds.includes(p.id) ? (
                    <><Check className="w-3.5 h-3.5 mr-1" /> {t("common.installed")}</>
                  ) : (
                    t("common.install")
                  )}
                </Button>
              </div>
            </div>
          ))}
        </div>
      )}

      <div className="flex items-center justify-center gap-4 pb-20 text-app">
        <Button 
          variant="secondary" 
          size="md" 
          disabled={pluginPage === 0} 
          onClick={() => setPluginPage(p => (typeof p === 'number' ? Math.max(0, p - 1) : p))} 
          className="rounded-full w-12 p-0 text-app glass"
        >
          <ChevronLeft className="w-5 h-5" />
        </Button>
        <span className="text-xs font-black opacity-40 text-app">{t("common.page", { page: pluginPage + 1 })}</span>
        <Button 
          variant="secondary" 
          size="md" 
          onClick={() => setPluginPage(p => (typeof p === 'number' ? p + 1 : p))} 
          className="rounded-full w-12 p-0 text-app glass"
        >
          <ChevronRight className="w-5 h-5" />
        </Button>
      </div>
    </motion.div>
  );
}
