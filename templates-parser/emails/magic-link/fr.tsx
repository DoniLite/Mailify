import * as React from "react";
import { Section, Text } from "@react-email/components";
import { Layout } from "../_components/Layout";
import { Button } from "../_components/Button";
import { H1, H2 } from "../_components/Heading";

export default function MagicLink_fr() {
  return (
    <Layout preview={`Votre lien de connexion`} locale="fr">
      <H1>{`Connexion à {{ theme.brand_name }}`}</H1>
      <Text className="m-0 mb-4 text-base text-foreground">{`Bonjour {{ vars.name or '' }}, cliquez sur le bouton ci-dessous pour vous connecter. Ce lien expire dans {{ vars.expires_in or '15 minutes' }}.`}</Text>
      <Section className="my-6 text-center"><Button href={"{{ vars.magic_link_url }}"}>{`Se connecter`}</Button></Section>
      <Text className="m-0 mb-4 text-sm text-muted">{`Si vous n'avez pas demandé cet email, vous pouvez l'ignorer.`}</Text>
    </Layout>
  );
}
