import * as React from "react";
import { Section, Text } from "@react-email/components";
import { Layout } from "../_components/Layout";
import { Button } from "../_components/Button";
import { H1, H2 } from "../_components/Heading";

export default function PasswordChanged_fr() {
  return (
    <Layout preview={`Votre mot de passe a changé`} locale="fr">
      <H1>{`Mot de passe modifié`}</H1>
      <Text className="m-0 mb-4 text-base text-foreground">{`Le mot de passe de votre compte a été modifié le {{ vars.changed_at }}.`}</Text>
      <Text className="m-0 mb-4 text-sm text-muted">{`Si ce n'était pas vous, réinitialisez-le immédiatement.`}</Text>
      <Section className="my-6 text-center"><Button href={"{{ vars.security_url }}"}>{`Sécuriser mon compte`}</Button></Section>
    </Layout>
  );
}
