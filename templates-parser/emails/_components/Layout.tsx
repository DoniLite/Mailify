import {
  Body,
  Container,
  Head,
  Hr,
  Html,
  Img,
  Link,
  Preview,
  Section,
  Tailwind,
  Text,
} from "@react-email/components";
import * as React from "react";

export interface LayoutProps {
  preview?: string;
  locale?: string;
  children: React.ReactNode;
}

/**
 * Shared email layout. Theme tokens + per-request brand come from the Rust
 * renderer via minijinja — referenced as `{{ theme.* }}` — and are pre-rendered
 * into the final HTML. Tailwind classes are inlined to style attributes at build.
 */
export function Layout({ preview, locale = "en", children }: LayoutProps) {
  return (
    <Html lang={locale}>
      <Head>
        <meta charSet="utf-8" />
        <meta name="viewport" content="width=device-width,initial-scale=1" />
      </Head>
      {preview ? <Preview>{preview}</Preview> : null}
      <Tailwind
        config={{
          theme: {
            extend: {
              colors: {
                primary: "{{ theme.colors.primary }}",
                primaryFg: "{{ theme.colors.primary_foreground }}",
                secondary: "{{ theme.colors.secondary }}",
                secondaryFg: "{{ theme.colors.secondary_foreground }}",
                background: "{{ theme.colors.background }}",
                foreground: "{{ theme.colors.foreground }}",
                muted: "{{ theme.colors.muted }}",
                border: "{{ theme.colors.border }}",
                danger: "{{ theme.colors.danger }}",
                success: "{{ theme.colors.success }}",
              },
              fontFamily: {
                body: ["{{ theme.fonts.body }}"],
                heading: ["{{ theme.fonts.heading }}"],
              },
              borderRadius: {
                card: "{{ theme.radius }}",
              },
            },
          },
        }}
      >
        <Body className="bg-background font-body text-foreground">
          <Container className="mx-auto my-10 max-w-[560px] rounded-card border border-solid border-border bg-background p-8">
            <Section className="mb-6 text-center">
              {"{% if theme.brand_logo_url %}"}
              <Img
                src="{{ theme.brand_logo_url }}"
                alt="{{ theme.brand_name }}"
                className="mx-auto h-10 w-auto"
              />
              {"{% else %}"}
              <Text className="m-0 text-xl font-bold text-foreground font-heading">
                {"{{ theme.brand_name }}"}
              </Text>
              {"{% endif %}"}
            </Section>
            {children}
            <Hr className="my-8 border-border" />
            <Section>
              <Text className="m-0 text-xs text-muted">
                {"{{ theme.footer_text or theme.brand_name }}"}
              </Text>
              {"{% if theme.social_links %}"}
              <Text className="m-0 mt-2 text-xs text-muted">
                {"{% for name, url in theme.social_links|items %}"}
                <Link href="{{ url }}" className="text-muted underline mr-3">
                  {"{{ name }}"}
                </Link>
                {"{% endfor %}"}
              </Text>
              {"{% endif %}"}
            </Section>
          </Container>
        </Body>
      </Tailwind>
    </Html>
  );
}

export default Layout;
