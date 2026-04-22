import * as React from "react";
import { Section, Row, Column, Text } from "@react-email/components";
import { Layout } from "../_components/Layout";
import { Button } from "../_components/Button";
import { H1, H2 } from "../_components/Heading";

export default function Receipt_fr() {
  return (
    <Layout preview={`Reçu {{ vars.order_id }}`} locale="fr">
      <H1>{`Merci pour votre achat`}</H1>
      <Text className="m-0 mb-4 text-base text-foreground">{`Voici votre reçu pour la commande **{{ vars.order_id }}**.`}</Text>
      <Section className="my-4 border border-solid border-border rounded-card p-4">
        <Row><Column className="py-1 text-sm text-muted">{`Commande`}</Column><Column className="py-1 text-right text-sm text-foreground">{"{{ vars.order_id }}"}</Column></Row>
        <Row><Column className="py-1 text-sm text-muted">{`Total`}</Column><Column className="py-1 text-right text-sm text-foreground">{"{{ vars.total }}"}</Column></Row>
        <Row><Column className="py-1 text-sm text-muted">{`Paiement`}</Column><Column className="py-1 text-right text-sm text-foreground">{"{{ vars.payment_method }}"}</Column></Row>
        <Row><Column className="py-1 text-sm text-muted">{`Date`}</Column><Column className="py-1 text-right text-sm text-foreground">{"{{ vars.date }}"}</Column></Row>
      </Section>
      <Section className="my-6 text-center"><Button href={"{{ vars.receipt_url }}"}>{`Voir le reçu`}</Button></Section>
    </Layout>
  );
}
