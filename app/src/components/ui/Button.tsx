import * as React from "react";
import { Slot } from "@radix-ui/react-slot";
import { motion } from "framer-motion";
import { cn } from "../../lib/utils";

/**
 * 説明: iOS風の洗練されたボタンコンポーネント
 */
export interface ButtonProps extends React.ButtonHTMLAttributes<HTMLButtonElement> {
  asChild?: boolean;
  variant?: "primary" | "secondary" | "ghost" | "danger";
  size?: "sm" | "md" | "lg" | "xl";
}

const Button = React.forwardRef<HTMLButtonElement, ButtonProps>(
  ({ className, variant = "primary", size = "md", asChild = false, ...props }, ref) => {
    const Comp = asChild ? Slot : "button";
    
    const variants = {
      primary: "bg-blue-600 text-white shadow-[0_4px_14px_0_rgba(0,118,255,0.39)] hover:bg-blue-700",
      secondary: "bg-gray-100 text-gray-900 dark:bg-zinc-800 dark:text-gray-100 hover:bg-gray-200 dark:hover:bg-zinc-700",
      ghost: "bg-transparent hover:bg-gray-100 dark:hover:bg-zinc-800",
      danger: "bg-red-500 text-white hover:bg-red-600 shadow-[0_4px_14px_0_rgba(255,0,0,0.39)]",
    };

    const sizes = {
      sm: "h-8 px-3 text-xs rounded-lg",
      md: "h-10 px-4 text-sm rounded-xl",
      lg: "h-12 px-6 text-base rounded-2xl",
      xl: "h-16 px-8 text-lg rounded-[1.5rem]",
    };

    return (
      <motion.div
        whileTap={{ scale: 0.97 }}
        className="inline-block"
      >
        <Comp
          className={cn(
            "inline-flex items-center justify-center font-bold transition-all focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-blue-600 disabled:opacity-50 disabled:pointer-events-none active:opacity-80",
            variants[variant],
            sizes[size],
            className
          )}
          ref={ref}
          {...props}
        />
      </motion.div>
    );
  }
);
Button.displayName = "Button";

export { Button };
