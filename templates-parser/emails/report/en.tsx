import * as React from "react";
import { Section, Row, Column, Text } from "@react-email/components";
import { Layout } from "../_components/Layout";
import { Button } from "../_components/Button";
import { H1, H2 } from "../_components/Heading";

export default function Report_en() {
  return (
    <Layout preview={`Your {{ vars.period }} report`} locale="en">
      <H1>{`{{ vars.period | title }} report`}</H1>
      <Text className="m-0 mb-4 text-base text-foreground">{`Here's a quick summary for **{{ vars.period }}**.`}</Text>
      <Section className="my-4 border border-solid border-border rounded-card p-4">

      </Section>
      <Section className="my-6 text-center"><Button href={"{{ vars.report_url }}"}>{`Open full report`}</Button></Section>
    </Layout>
  );
}
