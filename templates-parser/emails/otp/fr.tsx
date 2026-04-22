import * as React from "react";
import { Section, Text } from "@react-email/components";
import { Layout } from "../_components/Layout";
import { Button } from "../_components/Button";
import { H1, H2 } from "../_components/Heading";

export default function Otp_fr() {
  return (
    <Layout preview={`Votre code à usage unique`} locale="fr">
      <H1>{`Votre code de vérification`}</H1>
      <Text className="m-0 mb-4 text-base text-foreground">{`Saisissez le code ci-dessous pour continuer.`}</Text>
      <Section className="my-6 rounded-card bg-secondary p-4 text-center"><Text className="m-0 font-mono text-2xl font-bold tracking-widest text-secondaryFg">{`{{ vars.code }}`}</Text></Section>
      <Text className="m-0 mb-4 text-sm text-muted">{`Expire dans {{ vars.expires_in or '10 minutes' }}. Ne le partagez jamais.`}</Text>
    </Layout>
  );
}
