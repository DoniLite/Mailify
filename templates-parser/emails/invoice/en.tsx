import * as React from "react";
import { Section, Row, Column, Text } from "@react-email/components";
import { Layout } from "../_components/Layout";
import { Button } from "../_components/Button";
import { H1, H2 } from "../_components/Heading";

export default function Invoice_en() {
  return (
    <Layout preview={`Invoice {{ vars.invoice_number }}`} locale="en">
      <H1>{`Your invoice is ready`}</H1>
      <Text className="m-0 mb-4 text-base text-foreground">{`Invoice **{{ vars.invoice_number }}** is available.`}</Text>
      <Section className="my-4 border border-solid border-border rounded-card p-4">
        <Row><Column className="py-1 text-sm text-muted">{`Invoice`}</Column><Column className="py-1 text-right text-sm text-foreground">{"{{ vars.invoice_number }}"}</Column></Row>
        <Row><Column className="py-1 text-sm text-muted">{`Amount due`}</Column><Column className="py-1 text-right text-sm text-foreground">{"{{ vars.amount_due }}"}</Column></Row>
        <Row><Column className="py-1 text-sm text-muted">{`Due date`}</Column><Column className="py-1 text-right text-sm text-foreground">{"{{ vars.due_date }}"}</Column></Row>
      </Section>
      <Section className="my-6 text-center"><Button href={"{{ vars.invoice_url }}"}>{`View invoice`}</Button></Section>
    </Layout>
  );
}
