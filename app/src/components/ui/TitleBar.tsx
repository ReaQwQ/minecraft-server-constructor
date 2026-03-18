import { useState, useEffect } from "react";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { X, Minus, Square, Copy } from "lucide-react";

export function TitleBar() {
  const [isMaximized, setIsMaximized] = useState(false);
  const appWindow = getCurrentWindow();

  useEffect(() => {
    let unlistenResize: any;
    let unlistenMax: any;
    let unlistenUnmax: any;

    const updateMaximized = async () => {
      try {
        const maximized = await appWindow.isMaximized();
        setIsMaximized(maximized);
      } catch (e) {
        console.error("Failed to get maximized state:", e);
      }
    };

    const setup = async () => {
      try {
        unlistenResize = await appWindow.onResized(updateMaximized);
        unlistenMax = await appWindow.listen("tauri://maximized", () => setIsMaximized(true));
        unlistenUnmax = await appWindow.listen("tauri://unmaximized", () => setIsMaximized(false));
        updateMaximized();
      } catch (e) {
        console.error("Failed to setup window listeners:", e);
      }
    };

    setup();

    return () => {
      if (unlistenResize) unlistenResize();
      if (unlistenMax) unlistenMax();
      if (unlistenUnmax) unlistenUnmax();
    };
  }, [appWindow]);

  const handleMinimize = async () => {
    try {
      await appWindow.minimize();
    } catch (e) {
      console.error("Minimize failed:", e);
    }
  };

  const handleToggleMaximize = async () => {
    try {
      await appWindow.toggleMaximize();
    } catch (e) {
      console.error("Maximize failed:", e);
    }
  };

  const handleClose = async () => {
    try {
      await appWindow.close();
    } catch (e) {
      console.error("Close failed:", e);
    }
  };

  return (
    <div
      data-tauri-drag-region
      className="h-10 flex items-center justify-between bg-app/40 backdrop-blur-2xl border-b border-app select-none z-[9999] relative"
    >
      <div className="flex items-center px-6 gap-3 pointer-events-none">
        <div className="flex gap-1.5">
          <div className="w-3 h-3 rounded-full bg-[#ff5f57]" />
          <div className="w-3 h-3 rounded-full bg-[#febc2e]" />
          <div className="w-3 h-3 rounded-full bg-[#28c840]" />
        </div>
        <div className="w-px h-4 bg-app mx-1 opacity-20" />
        <span className="text-[10px] font-black text-app uppercase tracking-[0.2em] opacity-40">
          Minecraft Server Constructor
        </span>
      </div>

      <div className="flex items-center h-full relative z-[10000]">
        <button
          onClick={handleMinimize}
          className="h-full px-4 hover:bg-app/10 transition-colors text-app opacity-70 hover:opacity-100 no-drag pointer-events-auto"
        >
          <Minus className="w-3.5 h-3.5" />
        </button>
        <button
          onClick={handleToggleMaximize}
          className="h-full px-4 hover:bg-app/10 transition-colors text-app opacity-70 hover:opacity-100 no-drag pointer-events-auto"
        >
          {isMaximized ? <Copy className="w-3 h-3" /> : <Square className="w-3 h-3" />}
        </button>
        <button
          onClick={handleClose}
          className="h-full px-5 hover:bg-red-500/20 hover:text-red-500 transition-all text-app opacity-70 hover:opacity-100 group no-drag pointer-events-auto"
        >
          <X className="w-4 h-4 group-hover:rotate-90 transition-transform" />
        </button>
      </div>
    </div>
  );
}
