import { motion } from "framer-motion";
import { useTranslation } from "react-i18next";
import { 
  Zap, LayoutDashboard, Play, FolderOpen, Box, Lock, Sliders, Sun, Moon, Laptop, Globe, Info
} from "lucide-react";
import { cn } from "../../lib/utils";
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "../ui/Select";

interface SidebarProps {
  activeTab: string;
  setActiveTab: (tab: string) => void;
  theme: string;
  setTheme: (theme: string) => void;
}

/**
 * 説明: アプリケーションのナビゲーションと基本設定（テーマ、言語）を管理するサイドバー
 * @param activeTab 現在選択されているタブID
 * @param setActiveTab タブ切り替え関数
 * @param theme 現在のテーマ設定
 * @param setTheme テーマ変更関数
 * @return サイドバーのコンポーネント
 */
export function Sidebar({ activeTab, setActiveTab, theme, setTheme }: SidebarProps) {
  const { t, i18n } = useTranslation();

  const navItems = [
    { id: "dashboard", icon: <LayoutDashboard className="w-4 h-4" />, label: t("sidebar.dashboard") },
    { id: "deploy", icon: <Play className="w-4 h-4" />, label: t("sidebar.deploy") },
    { id: "browse", icon: <FolderOpen className="w-4 h-4" />, label: t("sidebar.browse") },
    { id: "plugins", icon: <Box className="w-4 h-4" />, label: t("sidebar.plugins") },
    { id: "security", icon: <Lock className="w-4 h-4" />, label: t("sidebar.security") },
    { id: "settings", icon: <Sliders className="w-4 h-4" />, label: t("sidebar.settings") },
    { id: "about", icon: <Info className="w-4 h-4" />, label: t("sidebar.about") },
  ];

  const languages = [
    { code: "en", label: "English (English)" },
    { code: "ja", label: "日本語 (Japanese)" },
    { code: "zh-CN", label: "简体中文 (Chinese Simplified)" },
    { code: "zh-TW", label: "繁體中文 (Chinese Traditional)" },
    { code: "ko", label: "한국어 (Korean)" },
    { code: "fr", label: "Français (French)" },
    { code: "de", label: "Deutsch (German)" },
    { code: "es", label: "Español (Spanish)" },
    { code: "pt", label: "Português (Portuguese)" },
    { code: "pt-BR", label: "Português do Brasil (Brazilian Portuguese)" },
    { code: "ru", label: "Русский (Russian)" },
    { code: "it", label: "Italiano (Italian)" },
    { code: "nl", label: "Nederlands (Dutch)" },
    { code: "pl", label: "Polski (Polish)" },
    { code: "sv", label: "Svenska (Swedish)" },
    { code: "tr", label: "Türkçe (Turkish)" },
    { code: "id", label: "Bahasa Indonesia (Indonesian)" },
    { code: "vi", label: "Tiếng Việt (Vietnamese)" },
    { code: "th", label: "ภาษาไทย (Thai)" },
    { code: "hi", label: "हिन्दी (Hindi)" },
    { code: "bn", label: "বাংলা (Bengali)" },
    { code: "ar", label: "العربية (Arabic)" },
  ];

  return (
    <aside className="w-64 glass border-r border-app flex flex-col shrink-0 z-20 shadow-2xl relative text-app">
      <div className="p-8 border-b border-app flex items-center gap-4 text-app">
        <motion.div 
          whileHover={{ scale: 1.1, rotate: 10 }} 
          className="bg-blue-600 p-3 rounded-2xl text-white shadow-xl shadow-blue-500/40"
        >
          <Zap className="w-6 h-6 fill-current" />
        </motion.div>
        <div>
          <h1 className="text-2xl font-black tracking-tighter leading-none italic">MSB</h1>
          <p className="text-[9px] font-black opacity-30 uppercase tracking-[0.2em] mt-1">{t("sidebar.orchestrator")}</p>
        </div>
      </div>
      <nav className="flex-1 p-4 space-y-1 overflow-y-auto custom-scrollbar text-app">
        {navItems.map((item) => (
          <button 
            key={item.id} 
            onClick={() => setActiveTab(item.id)} 
            className={cn(
              "w-full flex items-center gap-3 px-4 py-3.5 rounded-2xl text-sm font-bold transition-all relative group overflow-hidden text-app text-left", 
              activeTab === item.id ? "text-blue-600 dark:text-blue-400" : "opacity-40 hover:opacity-100 hover:bg-gray-100 dark:hover:bg-zinc-800/50"
            )}
          >
            {activeTab === item.id && (
              <motion.div 
                layoutId="activeNav" 
                className="absolute inset-0 bg-blue-600/5 dark:bg-blue-400/10 rounded-2xl border-2 border-blue-600/20 dark:border-blue-400/20 shadow-[0_0_20px_rgba(37,99,235,0.1)]" 
              />
            )}
            {item.icon}
            <span className="relative z-10">{item.label}</span>
          </button>
        ))}
      </nav>
      <div className="p-6 border-t border-app space-y-4 bg-gray-50/10 dark:bg-zinc-900/10 backdrop-blur-md text-app text-left">
        <div className="flex items-center justify-between px-1 text-[9px] font-black opacity-30 uppercase tracking-widest text-app">
          <span>{t("sidebar.theme")}</span>
          <div className="flex gap-1">
            {[
              { id: "light", icon: <Sun className="w-3.5 h-3.5" /> },
              { id: "dark", icon: <Moon className="w-3.5 h-3.5" /> },
              { id: "oled", icon: <Zap className="w-3.5 h-3.5" /> },
              { id: "system", icon: <Laptop className="w-3.5 h-3.5" /> }
            ].map((mode) => (
              <button 
                key={mode.id} 
                onClick={() => setTheme(mode.id)} 
                className={cn(
                  "p-1.5 rounded-lg transition-all text-app", 
                  theme === mode.id ? "bg-blue-600 text-white shadow-md" : "opacity-40 hover:opacity-100"
                )}
              >
                {mode.icon}
              </button>
            ))}
          </div>
        </div>
        
        <div className="space-y-2 text-left">
          <label className="text-[9px] font-black opacity-30 uppercase tracking-widest px-1 text-app">{t("sidebar.language")}</label>
          <Select 
            value={i18n.language.split("-")[0]} 
            onValueChange={(val) => i18n.changeLanguage(val)}
          >
            <SelectTrigger className="h-9 bg-transparent border-app text-[10px] font-bold shadow-none text-app">
              <Globe className="w-3 h-3 mr-2 opacity-50 text-app" />
              <SelectValue />
            </SelectTrigger>
            <SelectContent className="glass text-app">
              {languages.map(lang => (
                <SelectItem key={lang.code} value={lang.code} className="text-[11px] font-bold text-app">
                  {lang.label}
                </SelectItem>
              ))}
            </SelectContent>
          </Select>
        </div>
      </div>
    </aside>
  );
}
