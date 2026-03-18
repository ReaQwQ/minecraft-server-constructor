import { motion, AnimatePresence } from "framer-motion";
import { useTranslation } from "react-i18next";
import { Upload, Download, FolderPlus, Play, Loader2 } from "lucide-react";
import { Button } from "../ui/Button";
import { cn } from "../../lib/utils";

interface HeaderProps {
  activeTab: string;
  handleImport: () => void;
  handleExport: () => void;
  handleSelectOutputDir: () => void;
  outputDir: string;
  handleDeploy: () => void;
  isDeploying: boolean;
  version: string;
}

/**
 * 説明: アプリケーションの上部ヘッダー。タブのタイトル表示と共通アクション（インポート、エクスポート、デプロイ）を提供する。
 * @param activeTab 現在のアクティブなタブ
 * @param handleImport 設定の読み込み処理
 * @param handleExport 設定の保存処理
 * @param handleSelectOutputDir 出力先フォルダの選択
 * @param outputDir 現在選択されている出力先パス
 * @param handleDeploy デプロイ実行処理
 * @param isDeploying デプロイ中かどうかのフラグ
 * @param version 選択されているバージョン（バリデーション用）
 * @return ヘッダーコンポーネント
 */
export function Header({
  activeTab,
  handleImport,
  handleExport,
  handleSelectOutputDir,
  outputDir,
  handleDeploy,
  isDeploying,
  version
}: HeaderProps) {
  const { t } = useTranslation();

  return (
    <header className="h-20 border-b border-app flex items-center justify-between px-10 shrink-0 glass-header z-30 sticky top-0 text-app">
      <div className="flex flex-col">
        <AnimatePresence mode="wait">
          <motion.h2 
            key={activeTab} 
            initial={{ opacity: 0, y: -5 }} 
            animate={{ opacity: 1, y: 0 }} 
            exit={{ opacity: 0, y: 5 }} 
            className="text-[10px] font-black uppercase tracking-[0.4em] text-blue-600 dark:text-blue-400"
          >
            {t(`sidebar.${activeTab}`)}
          </motion.h2>
        </AnimatePresence>
        <h3 className="text-lg font-black tracking-tight mt-0.5 text-app">{t("header.title")}</h3>
      </div>
      <div className="flex items-center gap-3 text-app">
        <Button 
          variant="secondary" 
          size="sm" 
          onClick={handleImport} 
          className="hidden md:flex gap-2 opacity-60 hover:opacity-100 shadow-none border-none bg-transparent text-app"
        >
          <Upload className="w-4 h-4" />
          <span>{t("header.import")}</span>
        </Button>
        <Button 
          variant="secondary" 
          size="sm" 
          onClick={handleExport} 
          className="hidden md:flex gap-2 opacity-60 hover:opacity-100 shadow-none border-none bg-transparent text-app"
        >
          <Download className="w-4 h-4" />
          <span>{t("header.export")}</span>
        </Button>
        <div className="w-px h-6 bg-app mx-2" />
        <Button 
          variant="secondary" 
          size="sm" 
          onClick={handleSelectOutputDir} 
          className={cn(
            "hidden md:flex gap-2 opacity-60 hover:opacity-100 shadow-none border border-app bg-card/50 text-app glass", 
            outputDir && "text-blue-600 border-blue-600/30 opacity-100"
          )}
        >
          <FolderPlus className="w-4 h-4" />
          <span className="max-w-[150px] truncate">{outputDir ? outputDir : t("header.output_dir_btn")}</span>
        </Button>
        <Button 
          onClick={handleDeploy} 
          disabled={isDeploying || !version} 
          className="gap-2 px-6 h-11 shadow-lg shadow-blue-500/20"
        >
          {isDeploying ? <Loader2 className="w-4 h-4 animate-spin" /> : <Play className="w-4 h-4 fill-current" />}
          <span>{isDeploying ? t("header.deploying_btn") : t("header.deploy_btn")}</span>
        </Button>
      </div>
    </header>
  );
}
