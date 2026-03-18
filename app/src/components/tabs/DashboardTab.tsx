import { motion } from "framer-motion";
import { useTranslation } from "react-i18next";
import { Activity, HardDrive, Box } from "lucide-react";

interface SystemStats {
  cpu_usage: number;
  memory_total: number;
  memory_used: number;
  cluster_count: number;
}

interface DashboardTabProps {
  stats: SystemStats | null;
}

const pageVariants = {
  initial: { opacity: 0, y: 10 },
  animate: { opacity: 1, y: 0 },
  exit: { opacity: 0, y: -10 }
};

/**
 * 説明: システムのリソース使用状況とクラスター統計を表示するダッシュボード
 * @param stats Backendから取得したリアルタイム統計データ
 * @requires framer-motion, lucide-react
 * @return ダッシュボードコンポーネント
 */
export function DashboardTab({ stats }: DashboardTabProps) {
  const { t } = useTranslation();

  const statItems = [
    { 
      label: t("dashboard.cpu_usage"), 
      val: `${stats?.cpu_usage.toFixed(1) || 0}%`, 
      sub: t("dashboard.cpu_sub"), 
      icon: <Activity className="w-6 h-6" />, 
      color: "text-blue-600" 
    },
    { 
      label: t("dashboard.memory_used"), 
      val: `${((stats?.memory_used || 0) / 1024 / 1024 / 1024).toFixed(1)} GB`, 
      sub: t("dashboard.memory_total", { total: ((stats?.memory_total || 0) / 1024 / 1024 / 1024).toFixed(1) }), 
      icon: <HardDrive className="w-6 h-6" />, 
      color: "text-purple-600" 
    },
    { 
      label: t("dashboard.deployments"), 
      val: stats?.cluster_count.toString() || "0", 
      sub: t("dashboard.deployments_sub"), 
      icon: <Box className="w-6 h-6" />, 
      color: "text-green-600" 
    }
  ];

  return (
    <motion.div 
      key="dashboard" 
      variants={pageVariants}
      initial="initial" 
      animate="animate" 
      exit="exit"
      className="max-w-5xl mx-auto space-y-10 text-app"
    >
      <div className="grid grid-cols-1 md:grid-cols-3 gap-8">
        {statItems.map(stat => (
          <div 
            key={stat.label} 
            className="glass p-10 rounded-[3rem] space-y-6 hover:scale-[1.02] transition-all text-app"
          >
            <div className={stat.color}>{stat.icon}</div>
            <div>
              <p className="text-[10px] font-black uppercase opacity-30 tracking-widest text-app">{stat.label}</p>
              <p className="text-4xl font-black tracking-tighter mt-2 text-app">{stat.val}</p>
              <p className="text-xs font-bold opacity-40 mt-3 text-app">{stat.sub}</p>
            </div>
          </div>
        ))}
      </div>
      <div className="glass p-10 rounded-[3rem] space-y-6 text-app">
        <div className="flex items-center gap-3 text-app">
          <Activity className="w-5 h-5 text-blue-600" />
          <h4 className="text-lg font-black tracking-tight text-app">{t("dashboard.realtime_monitoring")}</h4>
        </div>
        <div className="h-48 flex items-end gap-1 px-4 text-app">
          {[...Array(40)].map((_, i) => (
            <motion.div 
              key={i} 
              animate={{ height: `${20 + Math.random() * 80}%` }} 
              transition={{ duration: 1.5, repeat: Infinity, ease: "easeInOut", delay: i * 0.05 }} 
              className="flex-1 bg-blue-600/20 rounded-t-sm" 
            />
          ))}
        </div>
      </div>
    </motion.div>
  );
}
