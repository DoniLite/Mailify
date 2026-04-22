import { Heading as REHeading } from "@react-email/components";
import * as React from "react";

export function H1({ children }: { children: React.ReactNode }) {
  return (
    <REHeading
      as="h1"
      className="mt-0 mb-4 font-heading text-2xl font-bold text-foreground"
    >
      {children}
    </REHeading>
  );
}

export function H2({ children }: { children: React.ReactNode }) {
  return (
    <REHeading
      as="h2"
      className="mt-6 mb-3 font-heading text-xl font-semibold text-foreground"
    >
      {children}
    </REHeading>
  );
}
