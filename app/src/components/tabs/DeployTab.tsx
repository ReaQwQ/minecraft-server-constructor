import { motion } from "framer-motion";
import { useTranslation } from "react-i18next";
import { 
  LayoutDashboard, Zap, Box, Loader2, Database, Waves, Sliders, Lock, Globe 
} from "lucide-react";
import * as Slider from "@radix-ui/react-slider";
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "../ui/Select";
import { Switch } from "../ui/Switch";
import { cn } from "../../lib/utils";

interface DeployTabProps {
  engine: string;
  setEngine: (v: string) => void;
  arclightLoader: string;
  setArclightLoader: (v: string) => void;
  version: string;
  setVersion: (v: string) => void;
  build: string;
  setBuild: (v: string) => void;
  proxy: string;
  setProxy: (v: string) => void;
  proxyVersion: string;
  setProxyVersion: (v: string) => void;
  proxyBuild: string;
  setProxyBuild: (v: string) => void;
  memory: number[];
  setMemory: (v: number[]) => void;
  protoMin: string;
  setProtoMin: (v: string) => void;
  protoMax: string;
  setProtoMax: (v: string) => void;
  multiProtocol: boolean;
  setMultiProtocol: (v: boolean) => void;
  jdkAuto: boolean;
  setJdkAuto: (v: boolean) => void;
  bedrock: boolean;
  setBedrock: (v: boolean) => void;
  discord: boolean;
  setDiscord: (v: boolean) => void;
  limbo: boolean;
  setLimbo: (v: boolean) => void;
  sonar: boolean;
  setSonar: (v: boolean) => void;
  eula: boolean;
  setEula: (v: boolean) => void;
  memoryAlloc: boolean;
  setMemoryAlloc: (v: boolean) => void;
  handleApplyOfficial: (name: string) => void;
  versions: string[];
  builds: string[];
  pVersions: string[];
  pBuilds: string[];
  isLoadingVersions: boolean;
  isLoadingBuilds: boolean;
  isLoadingPVersions: boolean;
  isLoadingPBuilds: boolean;
  isLoadingProtocols: boolean;
  protocolOptions: { label: string; value: string }[];
}

const pageVariants = {
  initial: { opacity: 0, y: 10 },
  animate: { opacity: 1, y: 0 },
  exit: { opacity: 0, y: -10 }
};

export function DeployTab(props: DeployTabProps) {
  const { t } = useTranslation();
  const isVanilla = props.engine === "vanilla" || props.engine === "bds";

  return (
    <motion.div 
      key="deploy" 
      variants={pageVariants} 
      initial="initial" 
      animate="animate" 
      exit="exit" 
      className="max-w-5xl mx-auto space-y-12 pb-32 text-app"
    >
      <section className="space-y-6">
        <div className="flex items-center gap-4">
          <div className="h-px bg-app grow opacity-10" />
          <h4 className="text-[10px] font-black uppercase tracking-[0.3em] opacity-30 shrink-0 text-app">{t("deploy.official_templates")}</h4>
          <div className="h-px bg-app grow opacity-10" />
        </div>
        <div className="grid grid-cols-3 gap-4">
          {[
            { id: "lobby", label: t("deploy.templates.lobby"), icon: <LayoutDashboard className="w-5 h-5" />, color: "hover:border-blue-600/50" },
            { id: "survival", label: t("deploy.templates.survival"), icon: <Zap className="w-5 h-5" />, color: "hover:border-orange-600/50" },
            { id: "modded", label: t("deploy.templates.modded"), icon: <Box className="w-5 h-5" />, color: "hover:border-purple-600/50" },
          ].map(tpl => (
            <button 
              key={tpl.id} 
              onClick={() => props.handleApplyOfficial(tpl.id)} 
              className={cn("flex flex-col items-center gap-3 p-6 glass rounded-[2rem] transition-all group", tpl.color)}
            >
              <div className="p-3 bg-blue-600/10 rounded-2xl group-hover:scale-110 transition-transform">{tpl.icon}</div>
              <span className="text-xs font-black uppercase tracking-widest opacity-60">{tpl.label}</span>
            </button>
          ))}
        </div>
      </section>

      <section className="space-y-8">
        <div className="flex items-center gap-4">
          <div className="h-px bg-app grow opacity-10" />
          <h4 className="text-[10px] font-black uppercase tracking-[0.3em] opacity-30 shrink-0 text-app">{t("deploy.sections.core")}</h4>
          <div className="h-px bg-app grow opacity-10" />
        </div>
        <div className="grid grid-cols-1 md:grid-cols-2 gap-12 text-app">
          <div className="space-y-6">
            <div className="space-y-3">
              <label className="text-[10px] font-black uppercase tracking-widest opacity-40 ml-1 text-app">{t("deploy.engine")}</label>
              <Select value={props.engine} onValueChange={props.setEngine}>
                <SelectTrigger className="h-14 glass text-app">
                  <SelectValue />
                </SelectTrigger>
                <SelectContent className="glass">
                  <SelectItem value="purpur">{t("deploy.engine_types.purpur")}</SelectItem>
                  <SelectItem value="paper">{t("deploy.engine_types.paper")}</SelectItem>
                  <SelectItem value="folia">{t("deploy.engine_types.folia")}</SelectItem>
                  <SelectItem value="arclight">{t("deploy.engine_types.arclight")}</SelectItem>
                  <SelectItem value="fabric">{t("deploy.engine_types.fabric")}</SelectItem>
                  <SelectItem value="vanilla">{t("deploy.engine_types.vanilla")}</SelectItem>
                  <SelectItem value="bds">{t("deploy.engine_types.bds")}</SelectItem>
                </SelectContent>
              </Select>
              {props.engine === "arclight" && (
                <div className="pt-1 space-y-2">
                  <Select value={props.arclightLoader} onValueChange={props.setArclightLoader}>
                    <SelectTrigger className="h-10 glass border-blue-600/30 text-xs font-black text-app">
                      <SelectValue />
                    </SelectTrigger>
                    <SelectContent className="glass">
                      <SelectItem value="forge">Forge</SelectItem>
                      <SelectItem value="fabric">Fabric</SelectItem>
                      <SelectItem value="neoforge">NeoForge</SelectItem>
                    </SelectContent>
                  </Select>
                </div>
              )}
            </div>
            <div className="grid grid-cols-2 gap-4">
              <div className="space-y-3">
                <label className="text-[10px] font-black uppercase tracking-widest opacity-40 flex items-center justify-between text-app">
                  {t("common.version")} {props.isLoadingVersions && <Loader2 className="w-3 h-3 animate-spin" />}
                </label>
                <Select value={props.version} onValueChange={props.setVersion}>
                  <SelectTrigger className="h-12 glass text-xs text-app">
                    <SelectValue />
                  </SelectTrigger>
                  <SelectContent className="glass">
                    {props.versions.map(v => <SelectItem key={v} value={v}>{v}</SelectItem>)}
                  </SelectContent>
                </Select>
              </div>
              <div className="space-y-3">
                <label className="text-[10px] font-black uppercase tracking-widest opacity-40 flex items-center justify-between text-app">
                  {t("common.build")} {props.isLoadingBuilds && <Loader2 className="w-3 h-3 animate-spin" />}
                </label>
                <Select value={props.build} onValueChange={props.setBuild}>
                  <SelectTrigger className="h-12 glass text-xs text-app">
                    <SelectValue />
                  </SelectTrigger>
                  <SelectContent className="glass">
                    <SelectItem value="latest">{t("common.latest")}</SelectItem>
                    {props.builds.map(b => <SelectItem key={b} value={b}>#{b}</SelectItem>)}
                  </SelectContent>
                </Select>
              </div>
            </div>
          </div>
          <div className={cn("space-y-6 transition-opacity duration-500", isVanilla && "opacity-20 pointer-events-none")}>
            <div className="space-y-3">
              <label className="text-[10px] font-black uppercase tracking-widest opacity-40 ml-1 text-app">{t("deploy.proxy_type")}</label>
              <Select value={props.proxy} onValueChange={props.setProxy} disabled={isVanilla}>
                <SelectTrigger className="h-14 glass text-app">
                  <SelectValue />
                </SelectTrigger>
                <SelectContent className="glass">
                  <SelectItem value="velocity">{t("deploy.proxy_types.velocity")}</SelectItem>
                  <SelectItem value="waterfall">{t("deploy.proxy_types.waterfall")}</SelectItem>
                  <SelectItem value="bungeecord">{t("deploy.proxy_types.bungeecord")}</SelectItem>
                  <SelectItem value="none">{t("deploy.proxy_types.none")}</SelectItem>
                </SelectContent>
              </Select>
            </div>
            {props.proxy !== "none" && !isVanilla && (
              <div className="grid grid-cols-2 gap-4">
                <div className="space-y-3">
                  <label className="text-[10px] font-black uppercase tracking-widest opacity-40 flex items-center justify-between text-app">
                    {t("deploy.proxy_ver")} {props.isLoadingPVersions && <Loader2 className="w-3 h-3 animate-spin" />}
                  </label>
                  <Select value={props.proxyVersion} onValueChange={props.setProxyVersion}>
                    <SelectTrigger className="h-12 glass text-xs text-app">
                      <SelectValue />
                    </SelectTrigger>
                    <SelectContent className="glass">
                      {props.pVersions.map(v => <SelectItem key={v} value={v}>{v}</SelectItem>)}
                    </SelectContent>
                  </Select>
                </div>
                <div className="space-y-3">
                  <label className="text-[10px] font-black uppercase tracking-widest opacity-40 flex items-center justify-between text-app">
                    {t("deploy.proxy_build")} {props.isLoadingPBuilds && <Loader2 className="w-3 h-3 animate-spin" />}
                  </label>
                  <Select value={props.proxyBuild} onValueChange={props.setProxyBuild}>
                    <SelectTrigger className="h-12 glass text-xs text-app">
                      <SelectValue />
                    </SelectTrigger>
                    <SelectContent className="glass">
                      <SelectItem value="latest">{t("common.latest")}</SelectItem>
                      {props.pBuilds.map(b => <SelectItem key={b} value={b}>#{b}</SelectItem>)}
                    </SelectContent>
                  </Select>
                </div>
              </div>
            )}
          </div>
        </div>
      </section>

      <section className="space-y-8">
        <div className="flex items-center gap-4">
          <div className="h-px bg-app grow opacity-10" />
          <h4 className="text-[10px] font-black uppercase tracking-[0.3em] opacity-30 shrink-0 text-app">{t("deploy.sections.resource")}</h4>
          <div className="h-px bg-app grow opacity-10" />
        </div>
        <div className="grid grid-cols-1 md:grid-cols-2 gap-12 text-app">
          <div className="space-y-6 glass p-8 rounded-[2.5rem] text-app">
            <div className="flex justify-between items-end mb-2 text-app">
              <label className="text-[10px] font-black uppercase tracking-widest opacity-40 text-app">{t("deploy.memory_alloc")}</label>
              <span className="text-3xl font-black tracking-tighter text-blue-600">{props.memory[0]}<span className="text-sm ml-1 opacity-20 italic">{t("deploy.memory_unit")}</span></span>
            </div>
            <Slider.Root 
              className="relative flex items-center select-none touch-none w-full h-5" 
              value={props.memory} 
              onValueChange={props.setMemory} 
              max={32768} 
              min={512} 
              step={512}
            >
              <Slider.Track className="bg-gray-200 dark:bg-zinc-800 relative grow rounded-full h-[6px]">
                <Slider.Range className="absolute bg-blue-600 rounded-full h-full" />
              </Slider.Track>
              <Slider.Thumb className="block w-5 h-5 bg-white border-4 border-blue-600 shadow-xl rounded-full focus:outline-none transition-transform hover:scale-110 active:scale-95" />
            </Slider.Root>
          </div>
          <div className={cn("space-y-6 p-2 transition-opacity duration-500 text-app", isVanilla && "opacity-20 pointer-events-none")}>
            <div className="flex items-center justify-between ml-1 text-app">
              <label className="text-[10px] font-black uppercase tracking-widest opacity-40 flex items-center gap-2 text-app">
                {t("deploy.protocol_range")} {props.isLoadingProtocols && <Loader2 className="w-3 h-3 animate-spin" />}
              </label>
              <div className="flex items-center gap-2 text-app">
                <span className="text-[9px] font-black opacity-30">{t("deploy.multi_protocol")}</span>
                <Switch checked={props.multiProtocol} onCheckedChange={props.setMultiProtocol} disabled={isVanilla} />
              </div>
            </div>
            <div className={cn("grid grid-cols-2 gap-3 transition-opacity duration-500 text-app", (!props.multiProtocol || isVanilla) && "opacity-20 pointer-events-none")}>
              <div className="space-y-2 text-app">
                <Select value={props.protoMin} onValueChange={props.setProtoMin} disabled={isVanilla}>
                  <SelectTrigger className="h-12 glass text-xs text-app">
                    <SelectValue />
                  </SelectTrigger>
                  <SelectContent className="glass">
                    {props.protocolOptions.map(p => <SelectItem key={p.value} value={p.value}>{p.label}</SelectItem>)}
                  </SelectContent>
                </Select>
              </div>
              <div className="space-y-2 text-app">
                <Select value={props.protoMax} onValueChange={props.setProtoMax} disabled={isVanilla}>
                  <SelectTrigger className="h-12 glass text-xs text-app">
                    <SelectValue />
                  </SelectTrigger>
                  <SelectContent className="glass">
                    {props.protocolOptions.map(p => <SelectItem key={p.value} value={p.value}>{p.label}</SelectItem>)}
                  </SelectContent>
                </Select>
              </div>
            </div>
          </div>
        </div>
      </section>

      <section className="space-y-8 text-app">
        <div className="flex items-center gap-4 text-app">
          <div className="h-px bg-app grow opacity-10" />
          <h4 className="text-[10px] font-black uppercase tracking-[0.3em] opacity-30 shrink-0 text-app">{t("deploy.sections.orchestration")}</h4>
          <div className="h-px bg-app grow opacity-10" />
        </div>
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4 text-app">
          {[
            { id: "jdk", label: t("deploy.features.jdk.label"), sub: t("deploy.features.jdk.sub"), val: props.jdkAuto, set: props.setJdkAuto, show: true, icon: <Zap className="w-4 h-4 text-blue-600" /> },
            { id: "bedrock", label: t("deploy.features.bedrock.label"), sub: t("deploy.features.bedrock.sub"), val: props.bedrock, set: props.setBedrock, show: true, icon: <Globe className="w-4 h-4 text-blue-600" /> },
            { id: "discord", label: t("deploy.features.discord.label"), sub: t("deploy.features.discord.sub"), val: props.discord, set: props.setDiscord, show: !isVanilla, icon: <Database className="w-4 h-4" /> },
            { id: "sonar", label: t("deploy.features.sonar.label"), sub: t("deploy.features.sonar.sub"), val: props.sonar, set: props.setSonar, show: props.proxy !== "none" && !isVanilla, icon: <Waves className="w-4 h-4 text-blue-600" /> },
            { id: "limbo", label: t("deploy.features.limbo.label"), sub: t("deploy.features.limbo.sub"), val: props.limbo, set: props.setLimbo, show: !isVanilla && props.proxy !== "none", icon: <LayoutDashboard className="w-4 h-4" /> },
            { id: "mem", label: t("deploy.features.mem.label"), sub: t("deploy.features.mem.sub"), val: props.memoryAlloc, set: props.setMemoryAlloc, show: true, icon: <Sliders className="w-4 h-4" /> },
            { id: "eula", label: t("deploy.features.eula.label"), sub: t("deploy.features.eula.sub"), val: props.eula, set: props.setEula, show: true, icon: <Lock className="w-4 h-4" /> },
          ].filter(o => o.show).map(opt => (
            <div key={opt.id} className="p-6 rounded-3xl glass flex flex-col justify-between hover:border-blue-600/30 transition-all shadow-sm h-36 text-app">
              <div className="flex justify-between items-start w-full text-app">
                <div className="space-y-1 text-app">
                  <div className="mb-2 text-app">{opt.icon}</div>
                  <p className="text-sm font-black text-app">{opt.label}</p>
                  <p className="text-[10px] text-muted font-bold leading-tight text-app">{opt.sub}</p>
                </div>
                <Switch checked={opt.val} onCheckedChange={opt.set} />
              </div>
            </div>
          ))}
        </div>
      </section>

    </motion.div>
  );
}
