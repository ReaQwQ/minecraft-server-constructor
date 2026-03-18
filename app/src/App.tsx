import { useState, useEffect, useMemo } from "react";
import { useTranslation } from "react-i18next";
import { AnimatePresence, motion } from "framer-motion";
import { invoke } from "@tauri-apps/api/core";
import { open, save } from "@tauri-apps/plugin-dialog";
import { writeFile, readFile } from "@tauri-apps/plugin-fs";

// Layout Components
import { TitleBar } from "./components/ui/TitleBar";
import { Sidebar } from "./components/layout/Sidebar";
import { Header } from "./components/layout/Header";

// Tab Components
import { DashboardTab } from "./components/tabs/DashboardTab";
import { DeployTab } from "./components/tabs/DeployTab";
import { BrowseTab } from "./components/tabs/Browse";
import { PluginsTab } from "./components/tabs/PluginsTab";
import { SecurityTab } from "./components/tabs/SecurityTab";
import { SettingsTab } from "./components/tabs/SettingsTab";
import { AboutTab } from "./components/tabs/AboutTab";
import { NotificationOverlay } from "./components/ui/NotificationOverlay";

// Store & Hooks
import { useNotification } from "./store/notificationStore";

import "./App.css";

const MC_VERSIONS_FALLBACK = ["1.21.1", "1.21", "1.20.4", "1.20.1", "1.19.4", "1.18.2", "1.16.5", "1.12.2", "1.8.9"];

interface ProtocolVersion { minecraftVersion: string; version: number; }
interface SystemStats { cpu_usage: number; memory_total: number; memory_used: number; cluster_count: number; }
interface SpigotPlugin { id: number; name: string; tag: string; icon?: { url: string }; }

/**
 * 説明: MSB アプリケーションのメインコンポーネント。状態管理とレイアウトの統合を行う。
 * @requires react, framer-motion, tauri-apps
 * @return アプリケーション全体のUI構造
 */
function App() {
  const { t } = useTranslation();
  const { success, error, info } = useNotification();
  const [activeTab, setActiveTab] = useState("deploy");
  const [theme, setTheme] = useState("system");
  
  // -- State: Deployment --
  const [engine, setEngine] = useState("purpur");
  const [arclightLoader, setArclightLoader] = useState("forge");
  const [version, setVersion] = useState("");
  const [build, setBuild] = useState("latest");
  const [proxy, setProxy] = useState("velocity");
  const [proxyVersion, setProxyVersion] = useState("");
  const [proxyBuild, setProxyBuild] = useState("latest");
  const [memory, setMemory] = useState([8192]); 
  const [outputDir, setOutputDir] = useState("");
  const [protoMin, setProtoMin] = useState("754");
  const [protoMax, setProtoMax] = useState("latest");
  const [multiProtocol, setMultiProtocol] = useState(true);

  // -- State: Toggles --
  const [jdkAuto, setJdkAuto] = useState(true);
  const [bedrock, setBedrock] = useState(true);
  const [discord, setDiscord] = useState(false);
  const [limbo, setLimbo] = useState(true);
  const [sonar, setSonar] = useState(true);
  const [eula, setEula] = useState(true);
  const [memoryAlloc, setMemoryAlloc] = useState(true);
  
  // -- State: Dashboard & Other --
  const [stats, setStats] = useState<SystemStats | null>(null);
  const [versions, setVersions] = useState<string[]>([]);
  const [builds, setBuilds] = useState<string[]>([]);
  const [pVersions, setPVersions] = useState<string[]>([]);
  const [pBuilds, setPBuilds] = useState<string[]>([]);
  const [protocols, setProtocols] = useState<ProtocolVersion[]>([]);
  const [spigotPlugins, setSpigotPlugins] = useState<SpigotPlugin[]>([]);
  const [pluginPage, setPluginPage] = useState(0);
  
  const [isLoadingVersions, setIsLoadingVersions] = useState(false);
  const [isLoadingBuilds, setIsLoadingBuilds] = useState(false);
  const [isLoadingPVersions, setIsLoadingPVersions] = useState(false);
  const [isLoadingPBuilds, setIsLoadingPBuilds] = useState(false);
  const [isLoadingProtocols, setIsLoadingProtocols] = useState(false);
  const [isLoadingPlugins, setIsLoadingPlugins] = useState(false);
  const [isDeploying, setIsSubmitting] = useState(false);

  // テーマ適用ロジック
  useEffect(() => {
    const root = window.document.documentElement;
    root.classList.remove("light", "dark", "oled");
    if (theme === "system") {
      const systemTheme = window.matchMedia("(prefers-color-scheme: dark)").matches ? "dark" : "light";
      root.classList.add(systemTheme);
    } else { root.classList.add(theme); }
  }, [theme]);

  // 右クリック禁止
  useEffect(() => {
    const handleContextMenu = (e: MouseEvent) => e.preventDefault();
    document.addEventListener("contextmenu", handleContextMenu);
    return () => document.removeEventListener("contextmenu", handleContextMenu);
  }, []);

  // -- Data Fetching Hooks --
  useEffect(() => {
    const fetchStats = async () => { try { setStats(await invoke("get_system_stats")); } catch (e) { console.error(e); } };
    fetchStats(); const interval = setInterval(fetchStats, 3000); return () => clearInterval(interval);
  }, []);

  useEffect(() => {
    const fetchProtocols = async () => {
      setIsLoadingProtocols(true);
      try {
        const res: ProtocolVersion[] = await invoke("get_protocol_list");
        setProtocols(res);
        if (res.length > 0) {
          setProtoMax("latest");
          const found = res.find(p => p.minecraftVersion === "1.16.5");
          if (found) setProtoMin(found.version.toString());
          else setProtoMin(res[res.length - 1].version.toString());
        }
      } catch (e) { console.error(e); }
      setIsLoadingProtocols(false);
    };
    fetchProtocols();
  }, []);

  useEffect(() => {
    const fetchPlugins = async () => {
      setIsLoadingPlugins(true);
      try { setSpigotPlugins(await invoke("list_spigot_plugins", { page: pluginPage, size: 12 })); } catch (e) { console.error(e); }
      setIsLoadingPlugins(false);
    };
    if (activeTab === "plugins") fetchPlugins();
  }, [pluginPage, activeTab]);

  useEffect(() => {
    const fetchVersions = async () => {
      setIsLoadingVersions(true);
      try {
        const target = engine === "arclight" ? `arclight-${arclightLoader}` : engine;
        const res: string[] = await invoke("get_versions", { engine: target });
        setVersions(res); if (res.length > 0) setVersion(res[0]);
      } catch (e) { setVersions(MC_VERSIONS_FALLBACK); }
      setIsLoadingVersions(false);
    };
    fetchVersions();
  }, [engine, arclightLoader]);

  useEffect(() => {
    if (!version) return;
    const fetchBuilds = async () => {
      setIsLoadingBuilds(true);
      try {
        const target = engine === "arclight" ? `arclight-${arclightLoader}` : engine;
        setBuilds(await invoke("get_builds", { engine: target, version }));
        setBuild("latest");
      } catch (e) { console.error(e); }
      setIsLoadingBuilds(false);
    };
    fetchBuilds();
  }, [engine, arclightLoader, version]);

  useEffect(() => {
    if (proxy === "none") return;
    const fetchPVersions = async () => {
      setIsLoadingPVersions(true);
      try {
        const res: string[] = await invoke("get_versions", { engine: proxy });
        setPVersions(res); if (res.length > 0) setProxyVersion(res[0]);
      } catch (e) { console.error(e); }
      setIsLoadingPVersions(false);
    };
    fetchPVersions();
  }, [proxy]);

  useEffect(() => {
    if (proxy === "none" || !proxyVersion) return;
    const fetchPBuilds = async () => {
      setIsLoadingPBuilds(true);
      try {
        setPBuilds(await invoke("get_builds", { engine: proxy, version: proxyVersion }));
        setProxyBuild("latest");
      } catch (e) { console.error(e); }
      setIsLoadingPBuilds(false);
    };
    fetchPBuilds();
  }, [proxy, proxyVersion]);

  // -- Handlers --
  
  /**
   * 説明: 外部設定要求を現在のステートに同期する
   */
  const applyRequest = (req: any) => {
    setEngine(req.server_type.includes("arclight") ? "arclight" : req.server_type);
    if (req.server_type.startsWith("arclight-")) setArclightLoader(req.server_type.split("-")[1]);
    setVersion(req.version); setBuild(req.build || "latest");
    setProxy(req.proxy); setProxyVersion(req.proxy_version); setProxyBuild(req.proxy_build || "latest");
    const memMatch = req.memory.match(/\d+/);
    if (memMatch) setMemory([parseInt(memMatch[0])]);
    setJdkAuto(req.jdk_auto); setBedrock(req.enable_bedrock); setDiscord(req.enable_discord);
    setLimbo(req.enable_limbo); setSonar(req.enable_sonar); setEula(req.accept_eula);
    setMemoryAlloc(req.memory_allocater);
    if (req.protocol_range) {
      const parts = req.protocol_range.split("-");
      if (parts.length === 2) { setProtoMin(parts[0]); setProtoMax(parts[1]); setMultiProtocol(true); }
    } else { setMultiProtocol(false); }
  };

  /**
   * 説明: 公式テンプレートを適用し、ユーザーに通知する
   */
  const handleApplyOfficial = async (name: string) => {
    try {
      const res = await invoke("get_official_template", { name });
      applyRequest(res);
      success(t("deploy.alerts.template_applied", { name }), t("common.success"));
    } catch (e) { error(String(e), t("common.error")); }
  };

  /**
   * 説明: 現在の設定をファイルとしてエクスポートする
   */
  const handleExport = async () => {
    try {
      const finalEngine = engine === "arclight" ? `arclight-${arclightLoader}` : engine;
      const protocolRange = (multiProtocol) ? `${protoMin}-${protoMax}` : "";
      const base64 = await invoke("export_template", {
        req: {
          server_type: finalEngine, proxy, version, build,
          proxy_version: proxyVersion, proxy_build: proxyBuild,
          memory: `${memory[0]}M`, memory_allocater: memoryAlloc,
          jdk_auto: jdkAuto, enable_bedrock: bedrock, enable_discord: discord,
          enable_limbo: limbo, enable_sonar: sonar, accept_eula: eula,
          protocol_range: protocolRange, platform: "windows", output_dir: ""
        }
      });
      const path = await save({ filters: [{ name: 'MSB Template', extensions: ['msbt'] }] });
      if (path) {
        const binaryString = window.atob(base64 as string);
        const bytes = new Uint8Array(binaryString.length);
        for (let i = 0; i < binaryString.length; i++) bytes[i] = binaryString.charCodeAt(i);
        await writeFile(path, bytes);
        success(t("deploy.alerts.export_success"), t("common.success"));
      }
    } catch (e) { error(String(e), t("common.error")); }
  };

  /**
   * 説明: 設定ファイルからステートを復元する
   */
  const handleImport = async () => {
    try {
      const path = await open({ multiple: false, filters: [{ name: 'MSB Template', extensions: ['msbt'] }] });
      if (path) {
        const bytes = await readFile(path as string);
        const base64 = window.btoa(String.fromCharCode(...bytes));
        const res = await invoke("import_template", { dataBase64: base64 });
        applyRequest(res);
        success(t("deploy.alerts.import_success"), t("common.success"));
      }
    } catch (e) { error(String(e), t("common.error")); }
  };

  /**
   * 説明: 出力先ディレクトリを選択し、ステートを更新する
   */
  const handleSelectOutputDir = async () => {
    try {
      const selected = await open({ directory: true, multiple: false, title: t("deploy.alerts.select_output") });
      if (selected) {
        setOutputDir(selected as string);
        info(t("common.output_dir") + ": " + selected, t("common.confirm"));
      }
    } catch (e) { console.error(e); }
  };

  /**
   * 説明: デプロイプロセスを開始し、完了または失敗時に通知する
   */
  const handleDeploy = async () => {
    setIsSubmitting(true);
    try {
      const finalEngine = engine === "arclight" ? `arclight-${arclightLoader}` : engine;
      const isVanilla = engine === "vanilla" || engine === "bds";
      const protocolRange = (multiProtocol && !isVanilla) ? `${protoMin}-${protoMax}` : "";
      await invoke("deploy_cluster", {
        req: {
          server_type: finalEngine, proxy: isVanilla ? "none" : proxy, version, build,
          proxy_version: proxyVersion, proxy_build: proxyBuild,
          memory: `${memory[0]}M`, memory_allocater: memoryAlloc,
          jdk_auto: jdkAuto, enable_bedrock: isVanilla ? false : bedrock, enable_discord: isVanilla ? false : discord,
          enable_limbo: isVanilla ? false : limbo, enable_sonar: isVanilla ? false : sonar,
          accept_eula: eula, protocol_range: protocolRange,
          platform: "windows", output_dir: outputDir
        }
      });
      success(t("deploy.alerts.deploy_success", { path: outputDir || 'generated/' }), t("common.success"));
    } catch (e) { error(t("deploy.alerts.deploy_error", { error: e }), t("common.error")); }
    setIsSubmitting(false);
  };

  /**
   * 説明: 利用可能なプロトコルオプションを生成する。重複を防ぐため一意の値を生成。
   */
  const protocolOptions = useMemo(() => {
    const list = protocols.map(p => ({ 
      label: `${p.minecraftVersion} (${p.version})`, 
      value: `${p.version}-${p.minecraftVersion}` // 💡 バージョン名を混ぜて一意にする
    }));
    return [{ label: t("common.latest"), value: "latest" }, ...list];
  }, [protocols, t]);

  return (
    <div className="flex flex-col h-screen overflow-hidden bg-app transition-colors duration-500 text-app font-sans antialiased">
      <TitleBar />
      <NotificationOverlay />
      <div className="flex flex-1 overflow-hidden">
        <Sidebar 
          activeTab={activeTab} 
          setActiveTab={setActiveTab} 
          theme={theme} 
          setTheme={setTheme} 
        />

        <main className="flex-1 flex flex-col min-w-0 bg-app relative text-app">
          <div 
            className="absolute inset-0 pointer-events-none opacity-[0.02] dark:opacity-[0.04] bg-[grid-line] [background-size:32px_32px]" 
            style={{ backgroundImage: 'linear-gradient(to right, gray 1px, transparent 1px), linear-gradient(to bottom, gray 1px, transparent 1px)' }} 
          />
          
          <Header 
            activeTab={activeTab}
            handleImport={handleImport}
            handleExport={handleExport}
            handleSelectOutputDir={handleSelectOutputDir}
            outputDir={outputDir}
            handleDeploy={handleDeploy}
            isDeploying={isDeploying}
            version={version}
          />

          <div className="flex-1 overflow-y-auto custom-scrollbar p-10 z-10 text-app">
            <AnimatePresence mode="wait">
              {activeTab === "dashboard" && <DashboardTab key="dashboard" stats={stats} />}
              {activeTab === "deploy" && (
                <DeployTab 
                  key="deploy"
                  engine={engine} setEngine={setEngine}
                  arclightLoader={arclightLoader} setArclightLoader={setArclightLoader}
                  version={version} setVersion={setVersion}
                  build={build} setBuild={setBuild}
                  proxy={proxy} setProxy={setProxy}
                  proxyVersion={proxyVersion} setProxyVersion={setProxyVersion}
                  proxyBuild={proxyBuild} setProxyBuild={setProxyBuild}
                  memory={memory} setMemory={setMemory}
                  protoMin={protoMin} setProtoMin={setProtoMin}
                  protoMax={protoMax} setProtoMax={setProtoMax}
                  multiProtocol={multiProtocol} setMultiProtocol={setMultiProtocol}
                  jdkAuto={jdkAuto} setJdkAuto={setJdkAuto}
                  bedrock={bedrock} setBedrock={setBedrock}
                  discord={discord} setDiscord={setDiscord}
                  limbo={limbo} setLimbo={setLimbo}
                  sonar={sonar} setSonar={setSonar}
                  eula={eula} setEula={setEula}
                  memoryAlloc={memoryAlloc} setMemoryAlloc={setMemoryAlloc}
                  handleApplyOfficial={handleApplyOfficial}
                  versions={versions} builds={builds}
                  pVersions={pVersions} pBuilds={pBuilds}
                  isLoadingVersions={isLoadingVersions} isLoadingBuilds={isLoadingBuilds}
                  isLoadingPVersions={isLoadingPVersions} isLoadingPBuilds={isLoadingPBuilds}
                  isLoadingProtocols={isLoadingProtocols}
                  protocolOptions={protocolOptions}
                />
              )}
              {activeTab === "browse" && (
                <motion.div 
                  key="browse" 
                  initial={{ opacity: 0, y: 10 }} 
                  animate={{ opacity: 1, y: 0 }} 
                  exit={{ opacity: 0, y: -10 }} 
                  className="max-w-5xl mx-auto"
                >
                  <BrowseTab />
                </motion.div>
              )}
              {activeTab === "plugins" && (
                <PluginsTab 
                  key="plugins"
                  isLoadingPlugins={isLoadingPlugins}
                  spigotPlugins={spigotPlugins}
                  pluginPage={pluginPage}
                  setPluginPage={setPluginPage}
                  outputDir={outputDir}
                />
              )}
              {activeTab === "security" && <SecurityTab key="security" />}
              {activeTab === "settings" && <SettingsTab key="settings" />}
              {activeTab === "about" && <AboutTab key="about" />}
            </AnimatePresence>
          </div>
        </main>
      </div>
    </div>
  );
}

export default App;
