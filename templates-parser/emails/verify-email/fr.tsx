import * as React from "react";
import { Section, Text } from "@react-email/components";
import { Layout } from "../_components/Layout";
import { Button } from "../_components/Button";
import { H1, H2 } from "../_components/Heading";

export default function VerifyEmail_fr() {
  return (
    <Layout preview={`Vérifiez votre email`} locale="fr">
      <H1>{`Vérifiez votre email`}</H1>
      <Text className="m-0 mb-4 text-base text-foreground">{`Bienvenue {{ vars.name or '' }}. Confirmez votre email pour finaliser votre compte.`}</Text>
      <Section className="my-6 text-center"><Button href={"{{ vars.verify_url }}"}>{`Vérifier l'email`}</Button></Section>
      <Text className="m-0 mb-4 text-sm text-muted">{`Le lien expire dans {{ vars.expires_in or '24 heures' }}.`}</Text>
    </Layout>
  );
}
