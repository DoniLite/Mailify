import * as React from "react";
import { Section, Row, Column, Text } from "@react-email/components";
import { Layout } from "../_components/Layout";
import { Button } from "../_components/Button";
import { H1, H2 } from "../_components/Heading";

export default function SubscriptionRenew_en() {
  return (
    <Layout preview={`Subscription renewed`} locale="en">
      <H1>{`Subscription renewed`}</H1>
      <Text className="m-0 mb-4 text-base text-foreground">{`Your **{{ vars.plan }}** plan has renewed for another {{ vars.period }}.`}</Text>
      <Section className="my-4 border border-solid border-border rounded-card p-4">
        <Row><Column className="py-1 text-sm text-muted">{`Amount`}</Column><Column className="py-1 text-right text-sm text-foreground">{"{{ vars.amount }}"}</Column></Row>
        <Row><Column className="py-1 text-sm text-muted">{`Next renewal`}</Column><Column className="py-1 text-right text-sm text-foreground">{"{{ vars.next_renewal }}"}</Column></Row>
      </Section>
      <Section className="my-6 text-center"><Button href={"{{ vars.manage_url }}"}>{`Manage subscription`}</Button></Section>
    </Layout>
  );
}
