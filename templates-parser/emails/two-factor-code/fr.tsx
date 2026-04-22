import * as React from "react";
import { Section, Text } from "@react-email/components";
import { Layout } from "../_components/Layout";
import { Button } from "../_components/Button";
import { H1, H2 } from "../_components/Heading";

export default function TwoFactorCode_fr() {
  return (
    <Layout preview={`Code d'authentification à deux facteurs`} locale="fr">
      <H1>{`Authentification à deux facteurs`}</H1>
      <Text className="m-0 mb-4 text-base text-foreground">{`Finalisez la connexion avec ce code 2FA :`}</Text>
      <Section className="my-6 rounded-card bg-secondary p-4 text-center"><Text className="m-0 font-mono text-2xl font-bold tracking-widest text-secondaryFg">{`{{ vars.code }}`}</Text></Section>
      <Text className="m-0 mb-4 text-sm text-muted">{`Valide {{ vars.expires_in or '5 minutes' }}.`}</Text>
    </Layout>
  );
}
