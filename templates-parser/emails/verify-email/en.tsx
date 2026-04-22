import * as React from "react";
import { Section, Text } from "@react-email/components";
import { Layout } from "../_components/Layout";
import { Button } from "../_components/Button";
import { H1, H2 } from "../_components/Heading";

export default function VerifyEmail_en() {
  return (
    <Layout preview={`Verify your email`} locale="en">
      <H1>{`Verify your email`}</H1>
      <Text className="m-0 mb-4 text-base text-foreground">{`Welcome, {{ vars.name or 'there' }}. Confirm your email so we can finish setting up your account.`}</Text>
      <Section className="my-6 text-center"><Button href={"{{ vars.verify_url }}"}>{`Verify email`}</Button></Section>
      <Text className="m-0 mb-4 text-sm text-muted">{`Link expires in {{ vars.expires_in or '24 hours' }}.`}</Text>
    </Layout>
  );
}
