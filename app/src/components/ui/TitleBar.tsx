import { useState, useEffect } from "react";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { X, Minus, Square, Copy } from "lucide-react";

export function TitleBar() {
  const [isMaximized, setIsMaximized] = useState(false);
  const appWindow = getCurrentWindow();

  useEffect(() => {
    const updateMaximized = async () => {
      setIsMaximized(await appWindow.isMaximized());
    };
    updateMaximized();
    const unlisten = appWindow.onResized(updateMaximized);
    return () => {
      unlisten.then((f) => f());
    };
  }, [appWindow]);

  return (
    <div
      data-tauri-drag-region
      className="h-10 flex items-center justify-between bg-transparent backdrop-blur-2xl border-b border-app select-none z-[100]"
    >
      <div className="flex items-center px-6 gap-3 pointer-events-none">
        <div className="flex gap-1.5">
          <div className="w-3 h-3 rounded-full bg-red-500/80" />
          <div className="w-3 h-3 rounded-full bg-orange-500/80" />
          <div className="w-3 h-3 rounded-full bg-green-500/80" />
        </div>
        <div className="w-px h-4 bg-app mx-1" />
        <span className="text-[10px] font-black text-app uppercase tracking-[0.2em] opacity-40">
          Minecraft Server Constructor
        </span>
      </div>

      <div className="flex items-center h-full">
        <button
          onClick={() => appWindow.minimize()}
          className="h-full px-4 hover:bg-app/10 transition-colors text-app opacity-40 hover:opacity-100"
        >
          <Minus className="w-3.5 h-3.5" />
        </button>
        <button
          onClick={() => appWindow.toggleMaximize()}
          className="h-full px-4 hover:bg-app/10 transition-colors text-app opacity-40 hover:opacity-100"
        >
          {isMaximized ? <Copy className="w-3 h-3" /> : <Square className="w-3 h-3" />}
        </button>
        <button
          onClick={() => appWindow.close()}
          className="h-full px-5 hover:bg-red-500/20 hover:text-red-500 transition-all text-app opacity-40 hover:opacity-100 group"
        >
          <X className="w-4 h-4 group-hover:rotate-90 transition-transform" />
        </button>
      </div>
    </div>
  );
}
