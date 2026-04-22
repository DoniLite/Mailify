import * as React from "react";
import { Section, Text } from "@react-email/components";
import { Layout } from "../_components/Layout";
import { Button } from "../_components/Button";
import { H1, H2 } from "../_components/Heading";

export default function PasswordChanged_en() {
  return (
    <Layout preview={`Your password was changed`} locale="en">
      <H1>{`Password changed`}</H1>
      <Text className="m-0 mb-4 text-base text-foreground">{`The password on your account was changed on {{ vars.changed_at }}.`}</Text>
      <Text className="m-0 mb-4 text-sm text-muted">{`If this wasn't you, reset your password immediately.`}</Text>
      <Section className="my-6 text-center"><Button href={"{{ vars.security_url }}"}>{`Secure my account`}</Button></Section>
    </Layout>
  );
}
