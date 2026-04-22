import { Button as REButton } from "@react-email/components";
import * as React from "react";

export interface ButtonProps {
  href: string;
  children: React.ReactNode;
}

export function Button({ href, children }: ButtonProps) {
  return (
    <REButton
      href={href}
      className="inline-block rounded-card bg-primary px-6 py-3 text-sm font-semibold text-primaryFg no-underline"
    >
      {children}
    </REButton>
  );
}

export default Button;
