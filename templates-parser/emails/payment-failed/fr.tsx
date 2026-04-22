import * as React from "react";
import { Section, Text } from "@react-email/components";
import { Layout } from "../_components/Layout";
import { Button } from "../_components/Button";
import { H1, H2 } from "../_components/Heading";

export default function PaymentFailed_fr() {
  return (
    <Layout preview={`Paiement échoué`} locale="fr">
      <H1>{`Paiement échoué`}</H1>
      <Text className="m-0 mb-4 text-base text-foreground">{`Nous n'avons pas pu traiter votre paiement de **{{ vars.amount }}**. Merci de mettre à jour vos informations.`}</Text>
      <Section className="my-6 text-center"><Button href={"{{ vars.billing_url }}"}>{`Mettre à jour`}</Button></Section>
      <Text className="m-0 mb-4 text-sm text-muted">{`Le service reste actif pendant {{ vars.grace_period or '3 jours' }}.`}</Text>
    </Layout>
  );
}
