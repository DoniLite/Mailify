import * as React from "react";
import { Section, Text } from "@react-email/components";
import { Layout } from "../_components/Layout";
import { Button } from "../_components/Button";
import { H1, H2 } from "../_components/Heading";

export default function Invite_en() {
  return (
    <Layout preview={`{{ vars.inviter_name }} invited you to {{ vars.workspace_name }}`} locale="en">
      <H1>{`You're invited`}</H1>
      <Text className="m-0 mb-4 text-base text-foreground">{`{{ vars.inviter_name }} has invited you to join **{{ vars.workspace_name }}** on {{ theme.brand_name }}.`}</Text>
      <Section className="my-6 text-center"><Button href={"{{ vars.invite_url }}"}>{`Accept invitation`}</Button></Section>
      <Text className="m-0 mb-4 text-sm text-muted">{`Invitation expires on {{ vars.expires_at }}.`}</Text>
    </Layout>
  );
}
