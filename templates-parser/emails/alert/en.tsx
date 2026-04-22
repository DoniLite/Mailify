import * as React from "react";
import { Section, Text } from "@react-email/components";
import { Layout } from "../_components/Layout";
import { Button } from "../_components/Button";
import { H1, H2 } from "../_components/Heading";

export default function Alert_en() {
  return (
    <Layout preview={`{{ vars.title }}`} locale="en">
      <H1>{`{{ vars.title }}`}</H1>
      <Text className="m-0 mb-4 text-base text-foreground">{`{{ vars.message }}`}</Text>
      <Text className="m-0 mb-4 text-sm text-muted">{`Triggered {{ vars.triggered_at }} — severity: {{ vars.severity }}`}</Text>
      <Section className="my-6 text-center"><Button href={"{{ vars.dashboard_url }}"}>{`View details`}</Button></Section>
    </Layout>
  );
}
