import i18n from "i18next";
import { initReactI18next } from "react-i18next";
import LanguageDetector from "i18next-browser-languagedetector";

import en from "./lang/en.json";
import ja from "./lang/ja.json";
import zhCN from "./lang/zh-CN.json";
import zhTW from "./lang/zh-TW.json";
import ko from "./lang/ko.json";
import fr from "./lang/fr.json";
import de from "./lang/de.json";
import es from "./lang/es.json";
import pt from "./lang/pt.json";
import ru from "./lang/ru.json";
import it from "./lang/it.json";
import ar from "./lang/ar.json";
import hi from "./lang/hi.json";
import bn from "./lang/bn.json";
import id from "./lang/id.json";
import vi from "./lang/vi.json";
import th from "./lang/th.json";
import tr from "./lang/tr.json";
import pl from "./lang/pl.json";
import nl from "./lang/nl.json";
import sv from "./lang/sv.json";
import ptBR from "./lang/pt-BR.json";

const resources = {
  en: { translation: en },
  ja: { translation: ja },
  "zh-CN": { translation: zhCN },
  "zh-TW": { translation: zhTW },
  ko: { translation: ko },
  fr: { translation: fr },
  de: { translation: de },
  es: { translation: es },
  pt: { translation: pt },
  ru: { translation: ru },
  it: { translation: it },
  ar: { translation: ar },
  hi: { translation: hi },
  bn: { translation: bn },
  id: { translation: id },
  vi: { translation: vi },
  th: { translation: th },
  tr: { translation: tr },
  pl: { translation: pl },
  nl: { translation: nl },
  sv: { translation: sv },
  "pt-BR": { translation: ptBR },
};

i18n
  .use(LanguageDetector)
  .use(initReactI18next)
  .init({
    resources,
    fallbackLng: "en",
    interpolation: {
      escapeValue: false,
    },
  });

export default i18n;
