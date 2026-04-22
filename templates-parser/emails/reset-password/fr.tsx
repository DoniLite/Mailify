import * as React from "react";
import { Section, Text } from "@react-email/components";
import { Layout } from "../_components/Layout";
import { Button } from "../_components/Button";
import { H1, H2 } from "../_components/Heading";

export default function ResetPassword_fr() {
  return (
    <Layout preview={`Réinitialisez votre mot de passe`} locale="fr">
      <H1>{`Réinitialisez votre mot de passe`}</H1>
      <Text className="m-0 mb-4 text-base text-foreground">{`Une réinitialisation de mot de passe a été demandée. Utilisez le bouton pour choisir un nouveau mot de passe.`}</Text>
      <Section className="my-6 text-center"><Button href={"{{ vars.reset_url }}"}>{`Réinitialiser`}</Button></Section>
      <Text className="m-0 mb-4 text-sm text-muted">{`Ignorez ce message si vous n'êtes pas à l'origine de la demande.`}</Text>
    </Layout>
  );
}
