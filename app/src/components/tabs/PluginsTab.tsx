import { motion } from "framer-motion";
import { useTranslation } from "react-i18next";
import { Box, Loader2, ExternalLink, ChevronLeft, ChevronRight } from "lucide-react";
import { Button } from "../ui/Button";

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
 * @requires framer-motion, react-i18next, lucide-react
 * @return プラグインタブのコンポーネント
 */
export function PluginsTab({
  isLoadingPlugins,
  spigotPlugins,
  pluginPage,
  setPluginPage
}: PluginsTabProps) {
  const { t } = useTranslation();
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
                {p.icon ? (
                  <img src={p.icon.url} className="w-12 h-12 rounded-xl" alt={p.name} />
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
                <Button variant="ghost" size="sm" className="opacity-40 hover:opacity-100 text-[10px] p-0 h-auto text-app">
                  <ExternalLink className="w-3 h-3 mr-1" /> {t("common.view")}
                </Button>
                <Button variant="secondary" size="sm" className="text-[10px] h-8 rounded-xl font-black text-app">
                  {t("common.install")}
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
