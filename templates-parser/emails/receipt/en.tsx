import * as React from "react";
import { Section, Row, Column, Text } from "@react-email/components";
import { Layout } from "../_components/Layout";
import { Button } from "../_components/Button";
import { H1, H2 } from "../_components/Heading";

export default function Receipt_en() {
  return (
    <Layout preview={`Receipt for {{ vars.order_id }}`} locale="en">
      <H1>{`Thanks for your purchase`}</H1>
      <Text className="m-0 mb-4 text-base text-foreground">{`Here's your receipt for order **{{ vars.order_id }}**.`}</Text>
      <Section className="my-4 border border-solid border-border rounded-card p-4">
        <Row><Column className="py-1 text-sm text-muted">{`Order ID`}</Column><Column className="py-1 text-right text-sm text-foreground">{"{{ vars.order_id }}"}</Column></Row>
        <Row><Column className="py-1 text-sm text-muted">{`Total`}</Column><Column className="py-1 text-right text-sm text-foreground">{"{{ vars.total }}"}</Column></Row>
        <Row><Column className="py-1 text-sm text-muted">{`Payment method`}</Column><Column className="py-1 text-right text-sm text-foreground">{"{{ vars.payment_method }}"}</Column></Row>
        <Row><Column className="py-1 text-sm text-muted">{`Date`}</Column><Column className="py-1 text-right text-sm text-foreground">{"{{ vars.date }}"}</Column></Row>
      </Section>
      <Section className="my-6 text-center"><Button href={"{{ vars.receipt_url }}"}>{`View receipt`}</Button></Section>
    </Layout>
  );
}
