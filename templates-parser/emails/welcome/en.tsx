import * as React from "react";
import { Section, Text } from "@react-email/components";
import { Layout } from "../_components/Layout";
import { Button } from "../_components/Button";
import { H1, H2 } from "../_components/Heading";

export default function Welcome_en() {
  return (
    <Layout preview={`Welcome to {{ theme.brand_name }}`} locale="en">
      <H1>{`Welcome aboard`}</H1>
      <Text className="m-0 mb-4 text-base text-foreground">{`Hey {{ vars.name or 'there' }}, we're glad you joined {{ theme.brand_name }}.`}</Text>
      <Text className="m-0 mb-4 text-base text-foreground">{`Here are a few things to get you started.`}</Text>
      <Section className="my-6 text-center"><Button href={"{{ vars.get_started_url }}"}>{`Get started`}</Button></Section>
    </Layout>
  );
}
