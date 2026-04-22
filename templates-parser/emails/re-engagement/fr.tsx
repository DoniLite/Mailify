import * as React from "react";
import { Section, Text } from "@react-email/components";
import { Layout } from "../_components/Layout";
import { Button } from "../_components/Button";
import { H1, H2 } from "../_components/Heading";

export default function ReEngagement_fr() {
  return (
    <Layout preview={`Vous nous avez manqué`} locale="fr">
      <H1>{`Ça fait un bail`}</H1>
      <Text className="m-0 mb-4 text-base text-foreground">{`Ça fait longtemps, {{ vars.name or '' }}. Voici les nouveautés chez {{ theme.brand_name }}.`}</Text>
      <Section className="my-6 text-center"><Button href={"{{ vars.return_url }}"}>{`Revenir`}</Button></Section>
    </Layout>
  );
}
