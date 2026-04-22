import * as React from "react";
import { Section, Text } from "@react-email/components";
import { Layout } from "../_components/Layout";
import { Button } from "../_components/Button";
import { H1, H2 } from "../_components/Heading";

export default function ResetPassword_en() {
  return (
    <Layout preview={`Reset your password`} locale="en">
      <H1>{`Reset your password`}</H1>
      <Text className="m-0 mb-4 text-base text-foreground">{`A password reset was requested for your account. Use the button below to choose a new password.`}</Text>
      <Section className="my-6 text-center"><Button href={"{{ vars.reset_url }}"}>{`Reset password`}</Button></Section>
      <Text className="m-0 mb-4 text-sm text-muted">{`Didn't ask for this? Ignore the email — your password stays unchanged.`}</Text>
    </Layout>
  );
}
