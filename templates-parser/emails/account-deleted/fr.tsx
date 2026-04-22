import * as React from "react";
import { Section, Text } from "@react-email/components";
import { Layout } from "../_components/Layout";
import { Button } from "../_components/Button";
import { H1, H2 } from "../_components/Heading";

export default function AccountDeleted_fr() {
  return (
    <Layout preview={`Votre compte a été supprimé`} locale="fr">
      <H1>{`Compte supprimé`}</H1>
      <Text className="m-0 mb-4 text-base text-foreground">{`Votre compte et les données associées ont été définitivement supprimés, comme demandé.`}</Text>
      <Text className="m-0 mb-4 text-sm text-muted">{`Cette action est irréversible. Vous devrez créer un nouveau compte pour revenir.`}</Text>
    </Layout>
  );
}
