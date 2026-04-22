import * as React from "react";
import { Section, Text } from "@react-email/components";
import { Layout } from "../_components/Layout";
import { Button } from "../_components/Button";
import { H1, H2 } from "../_components/Heading";

export default function Alert_fr() {
  return (
    <Layout preview={`{{ vars.title }}`} locale="fr">
      <H1>{`{{ vars.title }}`}</H1>
      <Text className="m-0 mb-4 text-base text-foreground">{`{{ vars.message }}`}</Text>
      <Text className="m-0 mb-4 text-sm text-muted">{`Déclenché {{ vars.triggered_at }} — sévérité : {{ vars.severity }}`}</Text>
      <Section className="my-6 text-center"><Button href={"{{ vars.dashboard_url }}"}>{`Voir les détails`}</Button></Section>
    </Layout>
  );
}
