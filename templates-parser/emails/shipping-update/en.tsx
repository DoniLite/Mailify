import * as React from "react";
import { Section, Row, Column, Text } from "@react-email/components";
import { Layout } from "../_components/Layout";
import { Button } from "../_components/Button";
import { H1, H2 } from "../_components/Heading";

export default function ShippingUpdate_en() {
  return (
    <Layout preview={`Shipping update for order {{ vars.order_id }}`} locale="en">
      <H1>{`Your order is on the way`}</H1>
      <Text className="m-0 mb-4 text-base text-foreground">{`Good news — order **{{ vars.order_id }}** is now in transit.`}</Text>
      <Section className="my-4 border border-solid border-border rounded-card p-4">
        <Row><Column className="py-1 text-sm text-muted">{`Carrier`}</Column><Column className="py-1 text-right text-sm text-foreground">{"{{ vars.carrier }}"}</Column></Row>
        <Row><Column className="py-1 text-sm text-muted">{`Tracking #`}</Column><Column className="py-1 text-right text-sm text-foreground">{"{{ vars.tracking_number }}"}</Column></Row>
        <Row><Column className="py-1 text-sm text-muted">{`ETA`}</Column><Column className="py-1 text-right text-sm text-foreground">{"{{ vars.eta }}"}</Column></Row>
      </Section>
      <Section className="my-6 text-center"><Button href={"{{ vars.tracking_url }}"}>{`Track shipment`}</Button></Section>
    </Layout>
  );
}
