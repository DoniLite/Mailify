import * as React from "react";
import { Section, Text } from "@react-email/components";
import { Layout } from "../_components/Layout";
import { Button } from "../_components/Button";
import { H1, H2 } from "../_components/Heading";

export default function Announcement_en() {
  return (
    <Layout preview={`{{ vars.headline }}`} locale="en">
      <H1>{`{{ vars.headline }}`}</H1>
      <Text className="m-0 mb-4 text-base text-foreground">{`{{ vars.body }}`}</Text>
      <Section className="my-6 text-center"><Button href={"{{ vars.learn_more_url }}"}>{`Learn more`}</Button></Section>
    </Layout>
  );
}
