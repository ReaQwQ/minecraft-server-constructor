import { useState, useEffect } from "react";
import { useTranslation } from "react-i18next";
import { invoke } from "@tauri-apps/api/core";
import { Folder, File, ChevronRight, Calendar, ArrowLeft } from "lucide-react";
import { motion, AnimatePresence } from "framer-motion";

interface GeneratedFolder {
  name: string;
  path: string;
  created_at: string;
}

interface FileInfo {
  name: string;
  is_dir: boolean;
  size: number;
}

/**
 * 説明: 生成されたサーバーフォルダとその中身をブラウズするためのタブ
 * @requires tauri-apps/api/core, lucide-react, framer-motion, react-i18next
 * @return 成果物ブラウザのコンポーネント
 */
export function BrowseTab() {
  const { t } = useTranslation();
  const [folders, setFolders] = useState<GeneratedFolder[]>([]);
  const [selectedPath, setSelectedPath] = useState<string | null>(null);
  const [files, setFiles] = useState<FileInfo[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    loadFolders();
  }, []);

  /**
   * 説明: 生成済みフォルダの一覧をBackendから取得する
   */
  const loadFolders = async () => {
    setLoading(true);
    try {
      const result: GeneratedFolder[] = await invoke("list_generated_folders");
      setFolders(result);
    } catch (e) {
      console.error(e);
    }
    setLoading(false);
  };

  /**
   * 説明: 特定のフォルダ内のファイル一覧を取得する
   * @param path フォルダのフルパス
   */
  const loadFiles = async (path: string) => {
    setLoading(true);
    try {
      const result: FileInfo[] = await invoke("list_files_in_folder", { path });
      setFiles(result);
      setSelectedPath(path);
    } catch (e) {
      console.error(e);
    }
    setLoading(false);
  };

  return (
    <div className="space-y-6 text-app">
      <div className="flex items-center justify-between text-app">
        <h3 className="text-2xl font-black tracking-tighter text-app">{t("browse.title")}</h3>
        {selectedPath && (
          <button 
            onClick={() => setSelectedPath(null)}
            className="flex items-center gap-2 text-xs font-bold opacity-50 hover:opacity-100 transition-all text-app"
          >
            <ArrowLeft className="w-4 h-4" />
            {t("browse.back_to_list")}
          </button>
        )}
      </div>

      <AnimatePresence mode="wait">
        {!selectedPath ? (
          <motion.div 
            key="folders"
            initial={{ opacity: 0, y: 10 }}
            animate={{ opacity: 1, y: 0 }}
            exit={{ opacity: 0, y: -10 }}
            className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4 text-app"
          >
            {folders.map((folder) => (
              <button
                key={folder.path}
                onClick={() => loadFiles(folder.path)}
                className="flex items-center gap-4 p-6 glass rounded-3xl hover:shadow-xl transition-all text-left group text-app"
              >
                <div className="p-3 bg-blue-600/10 rounded-2xl text-blue-600 group-hover:bg-blue-600 group-hover:text-white transition-colors">
                  <Folder className="w-6 h-6" />
                </div>
                <div className="min-w-0 flex-1">
                  <p className="text-sm font-black truncate text-app">{folder.name}</p>
                  <p className="text-[10px] opacity-40 font-bold flex items-center gap-1 mt-1 text-app">
                    <Calendar className="w-3 h-3" />
                    {folder.created_at}
                  </p>
                </div>
                <ChevronRight className="w-4 h-4 ml-auto opacity-20 group-hover:translate-x-1 transition-transform shrink-0 text-app" />
              </button>
            ))}
            {folders.length === 0 && !loading && (
              <div className="col-span-full py-20 text-center opacity-30 font-bold text-app">
                {t("browse.no_folders")}
              </div>
            )}
          </motion.div>
        ) : (
          <motion.div 
            key="files"
            initial={{ opacity: 0, x: 20 }}
            animate={{ opacity: 1, x: 0 }}
            exit={{ opacity: 0, x: -20 }}
            className="glass rounded-[2.5rem] overflow-hidden shadow-sm text-app"
          >
            <div className="px-8 py-4 bg-app/20 border-b border-app">
              <p className="text-[10px] font-black opacity-40 truncate text-app">{selectedPath}</p>
            </div>
            <div className="divide-y divide-app max-h-[60vh] overflow-y-auto custom-scrollbar text-app">
              {files.map((file) => (
                <div key={file.name} className="flex items-center gap-4 px-8 py-4 hover:bg-app/10 transition-colors text-app">
                  {file.is_dir ? <Folder className="w-4 h-4 text-blue-600" /> : <File className="w-4 h-4 opacity-40" />}
                  <span className="text-sm font-bold text-app">{file.name}</span>
                  {!file.is_dir && (
                    <span className="ml-auto text-[10px] opacity-30 font-black text-app">
                      {(file.size / 1024 / 1024).toFixed(2)} MB
                    </span>
                  )}
                </div>
              ))}
            </div>
          </motion.div>
        )}
      </AnimatePresence>
    </div>
  );
}
