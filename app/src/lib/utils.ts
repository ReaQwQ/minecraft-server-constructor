import { clsx, type ClassValue } from "clsx";
import { twMerge } from "tailwind-merge";

/**
 * 説明: Tailwind CSS のクラス名を安全に結合し、競合を解決する
 * @param inputs クラス名の配列
 * @return 結合されたクラス名文字列
 */
export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs));
}
