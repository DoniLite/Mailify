import * as React from "react";
import { Section, Row, Column, Text } from "@react-email/components";
import { Layout } from "../_components/Layout";
import { Button } from "../_components/Button";
import { H1, H2 } from "../_components/Heading";

export default function LoginNewDevice_en() {
  return (
    <Layout preview={`New sign-in on {{ vars.device }}`} locale="en">
      <H1>{`New device signed in`}</H1>
      <Text className="m-0 mb-4 text-base text-foreground">{`A new sign-in was detected.`}</Text>
      <Section className="my-4 border border-solid border-border rounded-card p-4">
        <Row><Column className="py-1 text-sm text-muted">{`Device`}</Column><Column className="py-1 text-right text-sm text-foreground">{"{{ vars.device }}"}</Column></Row>
        <Row><Column className="py-1 text-sm text-muted">{`Location`}</Column><Column className="py-1 text-right text-sm text-foreground">{"{{ vars.location }}"}</Column></Row>
        <Row><Column className="py-1 text-sm text-muted">{`Time`}</Column><Column className="py-1 text-right text-sm text-foreground">{"{{ vars.time }}"}</Column></Row>
        <Row><Column className="py-1 text-sm text-muted">{`IP`}</Column><Column className="py-1 text-right text-sm text-foreground">{"{{ vars.ip }}"}</Column></Row>
      </Section>
      <Section className="my-6 text-center"><Button href={"{{ vars.security_url }}"}>{`Review activity`}</Button></Section>
    </Layout>
  );
}
