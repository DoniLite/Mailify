import * as React from "react";
import { Section, Row, Column, Text } from "@react-email/components";
import { Layout } from "../_components/Layout";
import { Button } from "../_components/Button";
import { H1, H2 } from "../_components/Heading";

export default function SubscriptionRenew_fr() {
  return (
    <Layout preview={`Abonnement renouvelé`} locale="fr">
      <H1>{`Abonnement renouvelé`}</H1>
      <Text className="m-0 mb-4 text-base text-foreground">{`Votre formule **{{ vars.plan }}** a été renouvelée pour {{ vars.period }}.`}</Text>
      <Section className="my-4 border border-solid border-border rounded-card p-4">
        <Row><Column className="py-1 text-sm text-muted">{`Montant`}</Column><Column className="py-1 text-right text-sm text-foreground">{"{{ vars.amount }}"}</Column></Row>
        <Row><Column className="py-1 text-sm text-muted">{`Prochain renouvellement`}</Column><Column className="py-1 text-right text-sm text-foreground">{"{{ vars.next_renewal }}"}</Column></Row>
      </Section>
      <Section className="my-6 text-center"><Button href={"{{ vars.manage_url }}"}>{`Gérer l'abonnement`}</Button></Section>
    </Layout>
  );
}
