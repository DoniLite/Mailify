import * as React from "react";
import { Section, Text } from "@react-email/components";
import { Layout } from "../_components/Layout";
import { Button } from "../_components/Button";
import { H1, H2 } from "../_components/Heading";

export default function Newsletter_en() {
  return (
    <Layout preview={`{{ vars.issue_title }}`} locale="en">
      <H1>{`{{ vars.issue_title }}`}</H1>
      <Text className="m-0 mb-4 text-base text-foreground">{`{{ vars.intro }}`}</Text>
      <Section className="my-6 text-center"><Button href={"{{ vars.issue_url }}"}>{`Read more`}</Button></Section>
    </Layout>
  );
}
