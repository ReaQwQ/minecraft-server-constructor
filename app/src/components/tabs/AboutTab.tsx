import { motion } from "framer-motion";
import { useTranslation } from "react-i18next";
import { Info, Github, User, Shield, ExternalLink } from "lucide-react";
import logoSvg from "../../assets/logo.svg";

const pageVariants = {
  initial: { opacity: 0, scale: 0.98 },
  animate: { opacity: 1, scale: 1 },
  exit: { opacity: 0, scale: 1.02 }
};

export function AboutTab() {
  const { t } = useTranslation();
  
  const appVersion = "1.0.0";
  const buildVersion = "20260319.1";
  const author = "MSB Project Team & Rea";
  const licenseItems = t('license.items', { returnObjects: true }) as string[];

  return (
    <motion.div 
      key="about" 
      variants={pageVariants} 
      initial="initial" 
      animate="animate" 
      exit="exit" 
      className="max-w-4xl mx-auto space-y-12 pb-32 text-app"
    >
      <section className="flex flex-col items-center text-center space-y-6 pt-10">
        <motion.div 
          whileHover={{ rotate: 5, scale: 1.05 }}
          className="w-32 h-32 p-1 bg-gradient-to-br from-blue-500 to-blue-700 rounded-[2.5rem] shadow-2xl shadow-blue-500/20"
        >
          <img src={logoSvg} alt="MSB Logo" className="w-full h-full p-4" />
        </motion.div>
        <div className="space-y-2">
          <h2 className="text-4xl font-black tracking-tighter italic">{t("about.description")}</h2>
          <div className="flex items-center justify-center gap-2 text-xs font-black uppercase tracking-[0.2em] opacity-40">
            <span>{t("about.version", { version: appVersion })}</span>
            <span className="w-1 h-1 bg-app rounded-full" />
            <span>{t("about.build", { build: buildVersion })}</span>
          </div>
        </div>
      </section>

      <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
        <section className="glass p-8 rounded-[2rem] space-y-4">
          <div className="flex items-center gap-3 text-blue-600">
            <User className="w-5 h-5" />
            <h4 className="font-black uppercase tracking-widest text-xs">{t("about.author_title")}</h4>
          </div>
          <div className="space-y-1">
            <p className="text-lg font-bold">{author}</p>
            <p className="text-xs opacity-50 font-medium">{t("about.author_sub")}</p>
          </div>
        </section>

        <section className="glass p-8 rounded-[2rem] space-y-4">
          <div className="flex items-center gap-3 text-blue-600">
            <Github className="w-5 h-5" />
            <h4 className="font-black uppercase tracking-widest text-xs">{t("about.source_title")}</h4>
          </div>
          <div className="space-y-1">
            <p className="text-lg font-bold">{t("about.source_repo")}</p>
            <a href="#" className="text-xs text-blue-500 hover:underline flex items-center gap-1">
              github.com/rea/msb <ExternalLink className="w-3 h-3" />
            </a>
          </div>
        </section>
      </div>

      <section className="glass p-10 rounded-[2.5rem] space-y-6">
        <div className="flex items-center gap-3 text-blue-600">
          <Shield className="w-5 h-5" />
          <h4 className="font-black uppercase tracking-widest text-xs">{t("license.title")}</h4>
        </div>
        <div className="prose prose-sm dark:prose-invert max-w-none space-y-4">
          <h5 className="text-sm font-black uppercase tracking-wider">{t("license.name")}</h5>
          <p className="text-xs leading-relaxed opacity-70">
            {t("license.p1")}
          </p>
          <ul className="text-xs list-disc list-inside space-y-2 opacity-70 ml-2">
            {Array.isArray(licenseItems) && licenseItems.map((item, i) => (
              <li key={i} className={cn(i === 1 && "font-bold text-app opacity-100")}>{item}</li>
            ))}
          </ul>
          <div className="pt-4 flex gap-4">
             <div className="px-3 py-1 bg-blue-600/10 rounded-full border border-blue-600/20 text-[10px] font-black text-blue-600 uppercase tracking-widest">
               {t("license.badges.open_source")}
             </div>
             <div className="px-3 py-1 bg-green-600/10 rounded-full border border-green-600/20 text-[10px] font-black text-green-600 uppercase tracking-widest">
               {t("license.badges.fork_friendly")}
             </div>
          </div>
        </div>
      </section>

      <section className="text-center opacity-30 py-10">
        <p className="text-[10px] font-black uppercase tracking-[0.4em]">{t("about.built_with")}</p>
      </section>
    </motion.div>
  );
}

