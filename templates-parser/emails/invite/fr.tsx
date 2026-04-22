import * as React from "react";
import { Section, Text } from "@react-email/components";
import { Layout } from "../_components/Layout";
import { Button } from "../_components/Button";
import { H1, H2 } from "../_components/Heading";

export default function Invite_fr() {
  return (
    <Layout preview={`{{ vars.inviter_name }} vous invite à {{ vars.workspace_name }}`} locale="fr">
      <H1>{`Vous êtes invité`}</H1>
      <Text className="m-0 mb-4 text-base text-foreground">{`{{ vars.inviter_name }} vous invite à rejoindre **{{ vars.workspace_name }}** sur {{ theme.brand_name }}.`}</Text>
      <Section className="my-6 text-center"><Button href={"{{ vars.invite_url }}"}>{`Accepter l'invitation`}</Button></Section>
      <Text className="m-0 mb-4 text-sm text-muted">{`Expire le {{ vars.expires_at }}.`}</Text>
    </Layout>
  );
}
