import * as React from "react";
import { Section, Row, Column, Text } from "@react-email/components";
import { Layout } from "../_components/Layout";
import { Button } from "../_components/Button";
import { H1, H2 } from "../_components/Heading";

export default function ShippingUpdate_fr() {
  return (
    <Layout preview={`Mise à jour d'expédition {{ vars.order_id }}`} locale="fr">
      <H1>{`Votre commande est en route`}</H1>
      <Text className="m-0 mb-4 text-base text-foreground">{`Bonne nouvelle — la commande **{{ vars.order_id }}** est en transit.`}</Text>
      <Section className="my-4 border border-solid border-border rounded-card p-4">
        <Row><Column className="py-1 text-sm text-muted">{`Transporteur`}</Column><Column className="py-1 text-right text-sm text-foreground">{"{{ vars.carrier }}"}</Column></Row>
        <Row><Column className="py-1 text-sm text-muted">{`N° de suivi`}</Column><Column className="py-1 text-right text-sm text-foreground">{"{{ vars.tracking_number }}"}</Column></Row>
        <Row><Column className="py-1 text-sm text-muted">{`ETA`}</Column><Column className="py-1 text-right text-sm text-foreground">{"{{ vars.eta }}"}</Column></Row>
      </Section>
      <Section className="my-6 text-center"><Button href={"{{ vars.tracking_url }}"}>{`Suivre l'envoi`}</Button></Section>
    </Layout>
  );
}
