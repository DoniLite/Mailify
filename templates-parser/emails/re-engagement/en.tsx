import * as React from "react";
import { Section, Text } from "@react-email/components";
import { Layout } from "../_components/Layout";
import { Button } from "../_components/Button";
import { H1, H2 } from "../_components/Heading";

export default function ReEngagement_en() {
  return (
    <Layout preview={`We've missed you`} locale="en">
      <H1>{`Long time, no see`}</H1>
      <Text className="m-0 mb-4 text-base text-foreground">{`It's been a while, {{ vars.name or 'there' }}. Here's what's new at {{ theme.brand_name }}.`}</Text>
      <Section className="my-6 text-center"><Button href={"{{ vars.return_url }}"}>{`Come back`}</Button></Section>
    </Layout>
  );
}
