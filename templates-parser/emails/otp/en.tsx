import * as React from "react";
import { Section, Text } from "@react-email/components";
import { Layout } from "../_components/Layout";
import { Button } from "../_components/Button";
import { H1, H2 } from "../_components/Heading";

export default function Otp_en() {
  return (
    <Layout preview={`Your one-time code`} locale="en">
      <H1>{`Your verification code`}</H1>
      <Text className="m-0 mb-4 text-base text-foreground">{`Enter the code below to continue.`}</Text>
      <Section className="my-6 rounded-card bg-secondary p-4 text-center"><Text className="m-0 font-mono text-2xl font-bold tracking-widest text-secondaryFg">{`{{ vars.code }}`}</Text></Section>
      <Text className="m-0 mb-4 text-sm text-muted">{`Expires in {{ vars.expires_in or '10 minutes' }}. Never share this code.`}</Text>
    </Layout>
  );
}
