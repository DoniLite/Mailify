import * as React from "react";
import { Section, Row, Column, Text } from "@react-email/components";
import { Layout } from "../_components/Layout";
import { Button } from "../_components/Button";
import { H1, H2 } from "../_components/Heading";

export default function OrderConfirm_fr() {
  return (
    <Layout preview={`Commande confirmée`} locale="fr">
      <H1>{`Commande confirmée`}</H1>
      <Text className="m-0 mb-4 text-base text-foreground">{`Merci {{ vars.name or '' }}. Votre commande **{{ vars.order_id }}** est confirmée.`}</Text>
      <Section className="my-4 border border-solid border-border rounded-card p-4">
        <Row><Column className="py-1 text-sm text-muted">{`Total`}</Column><Column className="py-1 text-right text-sm text-foreground">{"{{ vars.total }}"}</Column></Row>
        <Row><Column className="py-1 text-sm text-muted">{`Livraison estimée`}</Column><Column className="py-1 text-right text-sm text-foreground">{"{{ vars.estimated_delivery }}"}</Column></Row>
      </Section>
      <Section className="my-6 text-center"><Button href={"{{ vars.track_url }}"}>{`Suivre la commande`}</Button></Section>
    </Layout>
  );
}
