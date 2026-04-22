import * as React from "react";
import { Section, Text } from "@react-email/components";
import { Layout } from "../_components/Layout";
import { Button } from "../_components/Button";
import { H1, H2 } from "../_components/Heading";

export default function FeedbackRequest_fr() {
  return (
    <Layout preview={`Un instant ?`} locale="fr">
      <H1>{`Votre avis nous intéresse`}</H1>
      <Text className="m-0 mb-4 text-base text-foreground">{`Bonjour {{ vars.name or '' }} — un rapide sondage de {{ vars.duration or '2 minutes' }} nous aiderait énormément.`}</Text>
      <Section className="my-6 text-center"><Button href={"{{ vars.survey_url }}"}>{`Donner mon avis`}</Button></Section>
    </Layout>
  );
}
