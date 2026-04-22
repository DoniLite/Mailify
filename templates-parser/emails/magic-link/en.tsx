import * as React from "react";
import { Section, Text } from "@react-email/components";
import { Layout } from "../_components/Layout";
import { Button } from "../_components/Button";
import { H1, H2 } from "../_components/Heading";

export default function MagicLink_en() {
  return (
    <Layout preview={`Your sign-in link`} locale="en">
      <H1>{`Sign in to {{ theme.brand_name }}`}</H1>
      <Text className="m-0 mb-4 text-base text-foreground">{`Hi {{ vars.name or 'there' }}, click the button below to sign in. This link expires in {{ vars.expires_in or '15 minutes' }}.`}</Text>
      <Section className="my-6 text-center"><Button href={"{{ vars.magic_link_url }}"}>{`Sign in`}</Button></Section>
      <Text className="m-0 mb-4 text-sm text-muted">{`If you didn't request this email, you can safely ignore it.`}</Text>
    </Layout>
  );
}
