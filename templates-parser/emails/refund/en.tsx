import * as React from "react";
import { Section, Text } from "@react-email/components";
import { Layout } from "../_components/Layout";
import { Button } from "../_components/Button";
import { H1, H2 } from "../_components/Heading";

export default function Refund_en() {
  return (
    <Layout preview={`Refund processed`} locale="en">
      <H1>{`Refund processed`}</H1>
      <Text className="m-0 mb-4 text-base text-foreground">{`Your refund of **{{ vars.amount }}** for order {{ vars.order_id }} has been processed.`}</Text>
      <Text className="m-0 mb-4 text-sm text-muted">{`Funds usually appear within 5–10 business days.`}</Text>
    </Layout>
  );
}
