import * as React from "react";
import { Section, Row, Column, Text } from "@react-email/components";
import { Layout } from "../_components/Layout";
import { Button } from "../_components/Button";
import { H1, H2 } from "../_components/Heading";

export default function OrderConfirm_en() {
  return (
    <Layout preview={`Order confirmed`} locale="en">
      <H1>{`Order confirmed`}</H1>
      <Text className="m-0 mb-4 text-base text-foreground">{`Thanks, {{ vars.name or 'customer' }}. Your order **{{ vars.order_id }}** is confirmed.`}</Text>
      <Section className="my-4 border border-solid border-border rounded-card p-4">
        <Row><Column className="py-1 text-sm text-muted">{`Total`}</Column><Column className="py-1 text-right text-sm text-foreground">{"{{ vars.total }}"}</Column></Row>
        <Row><Column className="py-1 text-sm text-muted">{`Estimated delivery`}</Column><Column className="py-1 text-right text-sm text-foreground">{"{{ vars.estimated_delivery }}"}</Column></Row>
      </Section>
      <Section className="my-6 text-center"><Button href={"{{ vars.track_url }}"}>{`Track order`}</Button></Section>
    </Layout>
  );
}
