import * as React from "react";
import { Section, Text } from "@react-email/components";
import { Layout } from "../_components/Layout";
import { Button } from "../_components/Button";
import { H1, H2 } from "../_components/Heading";

export default function FeedbackRequest_en() {
  return (
    <Layout preview={`Got a minute?`} locale="en">
      <H1>{`We'd love your feedback`}</H1>
      <Text className="m-0 mb-4 text-base text-foreground">{`Hi {{ vars.name or 'there' }} — a quick {{ vars.duration or '2-minute' }} survey would really help us improve.`}</Text>
      <Section className="my-6 text-center"><Button href={"{{ vars.survey_url }}"}>{`Give feedback`}</Button></Section>
    </Layout>
  );
}
