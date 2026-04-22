import * as React from "react";
import { Section, Text } from "@react-email/components";
import { Layout } from "../_components/Layout";
import { Button } from "../_components/Button";
import { H1, H2 } from "../_components/Heading";

export default function PaymentFailed_en() {
  return (
    <Layout preview={`Payment failed`} locale="en">
      <H1>{`Payment failed`}</H1>
      <Text className="m-0 mb-4 text-base text-foreground">{`We couldn't process your last payment of **{{ vars.amount }}**. Please update your billing details.`}</Text>
      <Section className="my-6 text-center"><Button href={"{{ vars.billing_url }}"}>{`Update payment`}</Button></Section>
      <Text className="m-0 mb-4 text-sm text-muted">{`Your service will continue for {{ vars.grace_period or '3 days' }}.`}</Text>
    </Layout>
  );
}
