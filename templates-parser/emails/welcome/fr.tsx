import * as React from "react";
import { Section, Text } from "@react-email/components";
import { Layout } from "../_components/Layout";
import { Button } from "../_components/Button";
import { H1, H2 } from "../_components/Heading";

export default function Welcome_fr() {
  return (
    <Layout preview={`Bienvenue sur {{ theme.brand_name }}`} locale="fr">
      <H1>{`Bienvenue à bord`}</H1>
      <Text className="m-0 mb-4 text-base text-foreground">{`Salut {{ vars.name or '' }}, ravis de vous compter parmi nous sur {{ theme.brand_name }}.`}</Text>
      <Text className="m-0 mb-4 text-base text-foreground">{`Voici quelques pistes pour commencer.`}</Text>
      <Section className="my-6 text-center"><Button href={"{{ vars.get_started_url }}"}>{`Commencer`}</Button></Section>
    </Layout>
  );
}
