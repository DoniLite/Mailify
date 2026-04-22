import * as React from "react";
import { Section, Text } from "@react-email/components";
import { Layout } from "../_components/Layout";
import { Button } from "../_components/Button";
import { H1, H2 } from "../_components/Heading";

export default function AccountDeleted_en() {
  return (
    <Layout preview={`Your account has been deleted`} locale="en">
      <H1>{`Account deleted`}</H1>
      <Text className="m-0 mb-4 text-base text-foreground">{`Your account and associated data have been permanently deleted as requested.`}</Text>
      <Text className="m-0 mb-4 text-sm text-muted">{`This action cannot be undone. If you change your mind, you'll need to sign up again.`}</Text>
    </Layout>
  );
}
