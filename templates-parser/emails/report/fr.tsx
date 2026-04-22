import * as React from "react";
import { Section, Row, Column, Text } from "@react-email/components";
import { Layout } from "../_components/Layout";
import { Button } from "../_components/Button";
import { H1, H2 } from "../_components/Heading";

export default function Report_fr() {
  return (
    <Layout preview={`Votre rapport {{ vars.period }}`} locale="fr">
      <H1>{`Rapport {{ vars.period }}`}</H1>
      <Text className="m-0 mb-4 text-base text-foreground">{`Voici un résumé rapide pour **{{ vars.period }}**.`}</Text>
      <Section className="my-4 border border-solid border-border rounded-card p-4">

      </Section>
      <Section className="my-6 text-center"><Button href={"{{ vars.report_url }}"}>{`Ouvrir le rapport`}</Button></Section>
    </Layout>
  );
}
