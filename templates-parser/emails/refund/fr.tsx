import * as React from "react";
import { Section, Text } from "@react-email/components";
import { Layout } from "../_components/Layout";
import { Button } from "../_components/Button";
import { H1, H2 } from "../_components/Heading";

export default function Refund_fr() {
  return (
    <Layout preview={`Remboursement effectué`} locale="fr">
      <H1>{`Remboursement effectué`}</H1>
      <Text className="m-0 mb-4 text-base text-foreground">{`Votre remboursement de **{{ vars.amount }}** pour la commande {{ vars.order_id }} a été effectué.`}</Text>
      <Text className="m-0 mb-4 text-sm text-muted">{`Les fonds apparaissent généralement sous 5 à 10 jours ouvrés.`}</Text>
    </Layout>
  );
}
