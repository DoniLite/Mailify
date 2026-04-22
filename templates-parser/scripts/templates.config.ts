/**
 * Declarative spec for every built-in Mailify template.
 *
 * Each entry is rendered by `scripts/generate.ts` into:
 *   emails/<id>/<locale>.tsx
 *   emails/<id>/subject.<locale>.txt
 *   emails/<id>/text.<locale>.txt
 *
 * Body text supports minijinja expressions — they pass through React Email
 * unchanged and are resolved by the Rust renderer at send time.
 */

export interface BlockSpec {
  /** Paragraph text (minijinja expressions allowed). */
  p?: string;
  /** Secondary paragraph with muted styling. */
  muted?: string;
  /** Heading H2. */
  h2?: string;
  /** Call-to-action button. */
  button?: { label: string; hrefVar: string };
  /** Verbatim code/token block. */
  code?: string;
  /** Key/value list rendered as a definition table. */
  kv?: Array<{ label: string; valueExpr: string }>;
}

export interface LocaleContent {
  preview: string;
  subject: string;
  heading: string;
  blocks: BlockSpec[];
  /** Plaintext alternative (minijinja allowed). */
  text: string;
}

export interface TemplateSpec {
  id: string;
  category: "auth" | "transactional" | "system" | "marketing";
  locales: Record<string, LocaleContent>;
}

/** 22 production-grade starter templates × 2 locales (en, fr). */
export const TEMPLATES: TemplateSpec[] = [
  // ── AUTH ────────────────────────────────────────────────────────────
  {
    id: "magic-link",
    category: "auth",
    locales: {
      en: {
        preview: "Your sign-in link",
        subject: "Sign in to {{ theme.brand_name }}",
        heading: "Sign in to {{ theme.brand_name }}",
        blocks: [
          { p: "Hi {{ vars.name or 'there' }}, click the button below to sign in. This link expires in {{ vars.expires_in or '15 minutes' }}." },
          { button: { label: "Sign in", hrefVar: "vars.magic_link_url" } },
          { muted: "If you didn't request this email, you can safely ignore it." },
        ],
        text: "Sign in to {{ theme.brand_name }}: {{ vars.magic_link_url }}\nExpires in {{ vars.expires_in or '15 minutes' }}.",
      },
      fr: {
        preview: "Votre lien de connexion",
        subject: "Connexion à {{ theme.brand_name }}",
        heading: "Connexion à {{ theme.brand_name }}",
        blocks: [
          { p: "Bonjour {{ vars.name or '' }}, cliquez sur le bouton ci-dessous pour vous connecter. Ce lien expire dans {{ vars.expires_in or '15 minutes' }}." },
          { button: { label: "Se connecter", hrefVar: "vars.magic_link_url" } },
          { muted: "Si vous n'avez pas demandé cet email, vous pouvez l'ignorer." },
        ],
        text: "Connexion à {{ theme.brand_name }} : {{ vars.magic_link_url }}\nExpire dans {{ vars.expires_in or '15 minutes' }}.",
      },
    },
  },
  {
    id: "verify-email",
    category: "auth",
    locales: {
      en: {
        preview: "Verify your email",
        subject: "Verify your email for {{ theme.brand_name }}",
        heading: "Verify your email",
        blocks: [
          { p: "Welcome, {{ vars.name or 'there' }}. Confirm your email so we can finish setting up your account." },
          { button: { label: "Verify email", hrefVar: "vars.verify_url" } },
          { muted: "Link expires in {{ vars.expires_in or '24 hours' }}." },
        ],
        text: "Verify your email: {{ vars.verify_url }}",
      },
      fr: {
        preview: "Vérifiez votre email",
        subject: "Vérifiez votre email pour {{ theme.brand_name }}",
        heading: "Vérifiez votre email",
        blocks: [
          { p: "Bienvenue {{ vars.name or '' }}. Confirmez votre email pour finaliser votre compte." },
          { button: { label: "Vérifier l'email", hrefVar: "vars.verify_url" } },
          { muted: "Le lien expire dans {{ vars.expires_in or '24 heures' }}." },
        ],
        text: "Vérifiez votre email : {{ vars.verify_url }}",
      },
    },
  },
  {
    id: "reset-password",
    category: "auth",
    locales: {
      en: {
        preview: "Reset your password",
        subject: "Reset your {{ theme.brand_name }} password",
        heading: "Reset your password",
        blocks: [
          { p: "A password reset was requested for your account. Use the button below to choose a new password." },
          { button: { label: "Reset password", hrefVar: "vars.reset_url" } },
          { muted: "Didn't ask for this? Ignore the email — your password stays unchanged." },
        ],
        text: "Reset your password: {{ vars.reset_url }}",
      },
      fr: {
        preview: "Réinitialisez votre mot de passe",
        subject: "Réinitialisez votre mot de passe {{ theme.brand_name }}",
        heading: "Réinitialisez votre mot de passe",
        blocks: [
          { p: "Une réinitialisation de mot de passe a été demandée. Utilisez le bouton pour choisir un nouveau mot de passe." },
          { button: { label: "Réinitialiser", hrefVar: "vars.reset_url" } },
          { muted: "Ignorez ce message si vous n'êtes pas à l'origine de la demande." },
        ],
        text: "Réinitialisez votre mot de passe : {{ vars.reset_url }}",
      },
    },
  },
  {
    id: "otp",
    category: "auth",
    locales: {
      en: {
        preview: "Your one-time code",
        subject: "{{ vars.code }} is your {{ theme.brand_name }} code",
        heading: "Your verification code",
        blocks: [
          { p: "Enter the code below to continue." },
          { code: "{{ vars.code }}" },
          { muted: "Expires in {{ vars.expires_in or '10 minutes' }}. Never share this code." },
        ],
        text: "Your code: {{ vars.code }}",
      },
      fr: {
        preview: "Votre code à usage unique",
        subject: "{{ vars.code }} est votre code {{ theme.brand_name }}",
        heading: "Votre code de vérification",
        blocks: [
          { p: "Saisissez le code ci-dessous pour continuer." },
          { code: "{{ vars.code }}" },
          { muted: "Expire dans {{ vars.expires_in or '10 minutes' }}. Ne le partagez jamais." },
        ],
        text: "Votre code : {{ vars.code }}",
      },
    },
  },
  {
    id: "two-factor-code",
    category: "auth",
    locales: {
      en: {
        preview: "Two-factor authentication code",
        subject: "Your 2FA code: {{ vars.code }}",
        heading: "Two-factor authentication",
        blocks: [
          { p: "Complete sign-in with this 2FA code:" },
          { code: "{{ vars.code }}" },
          { muted: "This code is valid for {{ vars.expires_in or '5 minutes' }}." },
        ],
        text: "2FA code: {{ vars.code }}",
      },
      fr: {
        preview: "Code d'authentification à deux facteurs",
        subject: "Code 2FA : {{ vars.code }}",
        heading: "Authentification à deux facteurs",
        blocks: [
          { p: "Finalisez la connexion avec ce code 2FA :" },
          { code: "{{ vars.code }}" },
          { muted: "Valide {{ vars.expires_in or '5 minutes' }}." },
        ],
        text: "Code 2FA : {{ vars.code }}",
      },
    },
  },
  {
    id: "invite",
    category: "auth",
    locales: {
      en: {
        preview: "{{ vars.inviter_name }} invited you to {{ vars.workspace_name }}",
        subject: "Join {{ vars.workspace_name }} on {{ theme.brand_name }}",
        heading: "You're invited",
        blocks: [
          { p: "{{ vars.inviter_name }} has invited you to join **{{ vars.workspace_name }}** on {{ theme.brand_name }}." },
          { button: { label: "Accept invitation", hrefVar: "vars.invite_url" } },
          { muted: "Invitation expires on {{ vars.expires_at }}." },
        ],
        text: "{{ vars.inviter_name }} invited you to {{ vars.workspace_name }}: {{ vars.invite_url }}",
      },
      fr: {
        preview: "{{ vars.inviter_name }} vous invite à {{ vars.workspace_name }}",
        subject: "Rejoindre {{ vars.workspace_name }} sur {{ theme.brand_name }}",
        heading: "Vous êtes invité",
        blocks: [
          { p: "{{ vars.inviter_name }} vous invite à rejoindre **{{ vars.workspace_name }}** sur {{ theme.brand_name }}." },
          { button: { label: "Accepter l'invitation", hrefVar: "vars.invite_url" } },
          { muted: "Expire le {{ vars.expires_at }}." },
        ],
        text: "{{ vars.inviter_name }} vous invite à {{ vars.workspace_name }} : {{ vars.invite_url }}",
      },
    },
  },

  // ── TRANSACTIONAL ───────────────────────────────────────────────────
  {
    id: "receipt",
    category: "transactional",
    locales: {
      en: {
        preview: "Receipt for {{ vars.order_id }}",
        subject: "Receipt #{{ vars.order_id }} — {{ theme.brand_name }}",
        heading: "Thanks for your purchase",
        blocks: [
          { p: "Here's your receipt for order **{{ vars.order_id }}**." },
          {
            kv: [
              { label: "Order ID", valueExpr: "vars.order_id" },
              { label: "Total", valueExpr: "vars.total" },
              { label: "Payment method", valueExpr: "vars.payment_method" },
              { label: "Date", valueExpr: "vars.date" },
            ],
          },
          { button: { label: "View receipt", hrefVar: "vars.receipt_url" } },
        ],
        text: "Receipt {{ vars.order_id }} — {{ vars.total }} ({{ vars.date }})",
      },
      fr: {
        preview: "Reçu {{ vars.order_id }}",
        subject: "Reçu #{{ vars.order_id }} — {{ theme.brand_name }}",
        heading: "Merci pour votre achat",
        blocks: [
          { p: "Voici votre reçu pour la commande **{{ vars.order_id }}**." },
          {
            kv: [
              { label: "Commande", valueExpr: "vars.order_id" },
              { label: "Total", valueExpr: "vars.total" },
              { label: "Paiement", valueExpr: "vars.payment_method" },
              { label: "Date", valueExpr: "vars.date" },
            ],
          },
          { button: { label: "Voir le reçu", hrefVar: "vars.receipt_url" } },
        ],
        text: "Reçu {{ vars.order_id }} — {{ vars.total }} ({{ vars.date }})",
      },
    },
  },
  {
    id: "invoice",
    category: "transactional",
    locales: {
      en: {
        preview: "Invoice {{ vars.invoice_number }}",
        subject: "Invoice {{ vars.invoice_number }} from {{ theme.brand_name }}",
        heading: "Your invoice is ready",
        blocks: [
          { p: "Invoice **{{ vars.invoice_number }}** is available." },
          {
            kv: [
              { label: "Invoice", valueExpr: "vars.invoice_number" },
              { label: "Amount due", valueExpr: "vars.amount_due" },
              { label: "Due date", valueExpr: "vars.due_date" },
            ],
          },
          { button: { label: "View invoice", hrefVar: "vars.invoice_url" } },
        ],
        text: "Invoice {{ vars.invoice_number }}: {{ vars.invoice_url }}",
      },
      fr: {
        preview: "Facture {{ vars.invoice_number }}",
        subject: "Facture {{ vars.invoice_number }} — {{ theme.brand_name }}",
        heading: "Votre facture est disponible",
        blocks: [
          { p: "La facture **{{ vars.invoice_number }}** est prête." },
          {
            kv: [
              { label: "Facture", valueExpr: "vars.invoice_number" },
              { label: "Montant dû", valueExpr: "vars.amount_due" },
              { label: "Échéance", valueExpr: "vars.due_date" },
            ],
          },
          { button: { label: "Voir la facture", hrefVar: "vars.invoice_url" } },
        ],
        text: "Facture {{ vars.invoice_number }} : {{ vars.invoice_url }}",
      },
    },
  },
  {
    id: "order-confirm",
    category: "transactional",
    locales: {
      en: {
        preview: "Order confirmed",
        subject: "Your order {{ vars.order_id }} is confirmed",
        heading: "Order confirmed",
        blocks: [
          { p: "Thanks, {{ vars.name or 'customer' }}. Your order **{{ vars.order_id }}** is confirmed." },
          {
            kv: [
              { label: "Total", valueExpr: "vars.total" },
              { label: "Estimated delivery", valueExpr: "vars.estimated_delivery" },
            ],
          },
          { button: { label: "Track order", hrefVar: "vars.track_url" } },
        ],
        text: "Order {{ vars.order_id }} confirmed. Track: {{ vars.track_url }}",
      },
      fr: {
        preview: "Commande confirmée",
        subject: "Commande {{ vars.order_id }} confirmée",
        heading: "Commande confirmée",
        blocks: [
          { p: "Merci {{ vars.name or '' }}. Votre commande **{{ vars.order_id }}** est confirmée." },
          {
            kv: [
              { label: "Total", valueExpr: "vars.total" },
              { label: "Livraison estimée", valueExpr: "vars.estimated_delivery" },
            ],
          },
          { button: { label: "Suivre la commande", hrefVar: "vars.track_url" } },
        ],
        text: "Commande {{ vars.order_id }} confirmée. Suivi : {{ vars.track_url }}",
      },
    },
  },
  {
    id: "shipping-update",
    category: "transactional",
    locales: {
      en: {
        preview: "Shipping update for order {{ vars.order_id }}",
        subject: "Your order {{ vars.order_id }} has shipped",
        heading: "Your order is on the way",
        blocks: [
          { p: "Good news — order **{{ vars.order_id }}** is now in transit." },
          {
            kv: [
              { label: "Carrier", valueExpr: "vars.carrier" },
              { label: "Tracking #", valueExpr: "vars.tracking_number" },
              { label: "ETA", valueExpr: "vars.eta" },
            ],
          },
          { button: { label: "Track shipment", hrefVar: "vars.tracking_url" } },
        ],
        text: "Order {{ vars.order_id }} shipped. Tracking: {{ vars.tracking_url }}",
      },
      fr: {
        preview: "Mise à jour d'expédition {{ vars.order_id }}",
        subject: "Votre commande {{ vars.order_id }} a été expédiée",
        heading: "Votre commande est en route",
        blocks: [
          { p: "Bonne nouvelle — la commande **{{ vars.order_id }}** est en transit." },
          {
            kv: [
              { label: "Transporteur", valueExpr: "vars.carrier" },
              { label: "N° de suivi", valueExpr: "vars.tracking_number" },
              { label: "ETA", valueExpr: "vars.eta" },
            ],
          },
          { button: { label: "Suivre l'envoi", hrefVar: "vars.tracking_url" } },
        ],
        text: "Commande {{ vars.order_id }} expédiée. Suivi : {{ vars.tracking_url }}",
      },
    },
  },
  {
    id: "refund",
    category: "transactional",
    locales: {
      en: {
        preview: "Refund processed",
        subject: "Refund processed for order {{ vars.order_id }}",
        heading: "Refund processed",
        blocks: [
          { p: "Your refund of **{{ vars.amount }}** for order {{ vars.order_id }} has been processed." },
          { muted: "Funds usually appear within 5–10 business days." },
        ],
        text: "Refund {{ vars.amount }} processed for order {{ vars.order_id }}.",
      },
      fr: {
        preview: "Remboursement effectué",
        subject: "Remboursement effectué — commande {{ vars.order_id }}",
        heading: "Remboursement effectué",
        blocks: [
          { p: "Votre remboursement de **{{ vars.amount }}** pour la commande {{ vars.order_id }} a été effectué." },
          { muted: "Les fonds apparaissent généralement sous 5 à 10 jours ouvrés." },
        ],
        text: "Remboursement {{ vars.amount }} effectué pour la commande {{ vars.order_id }}.",
      },
    },
  },
  {
    id: "subscription-renew",
    category: "transactional",
    locales: {
      en: {
        preview: "Subscription renewed",
        subject: "Your {{ theme.brand_name }} subscription renewed",
        heading: "Subscription renewed",
        blocks: [
          { p: "Your **{{ vars.plan }}** plan has renewed for another {{ vars.period }}." },
          {
            kv: [
              { label: "Amount", valueExpr: "vars.amount" },
              { label: "Next renewal", valueExpr: "vars.next_renewal" },
            ],
          },
          { button: { label: "Manage subscription", hrefVar: "vars.manage_url" } },
        ],
        text: "Subscription {{ vars.plan }} renewed. Next: {{ vars.next_renewal }}.",
      },
      fr: {
        preview: "Abonnement renouvelé",
        subject: "Votre abonnement {{ theme.brand_name }} a été renouvelé",
        heading: "Abonnement renouvelé",
        blocks: [
          { p: "Votre formule **{{ vars.plan }}** a été renouvelée pour {{ vars.period }}." },
          {
            kv: [
              { label: "Montant", valueExpr: "vars.amount" },
              { label: "Prochain renouvellement", valueExpr: "vars.next_renewal" },
            ],
          },
          { button: { label: "Gérer l'abonnement", hrefVar: "vars.manage_url" } },
        ],
        text: "Abonnement {{ vars.plan }} renouvelé. Prochain : {{ vars.next_renewal }}.",
      },
    },
  },
  {
    id: "payment-failed",
    category: "transactional",
    locales: {
      en: {
        preview: "Payment failed",
        subject: "Action required: payment failed",
        heading: "Payment failed",
        blocks: [
          { p: "We couldn't process your last payment of **{{ vars.amount }}**. Please update your billing details." },
          { button: { label: "Update payment", hrefVar: "vars.billing_url" } },
          { muted: "Your service will continue for {{ vars.grace_period or '3 days' }}." },
        ],
        text: "Payment of {{ vars.amount }} failed. Update: {{ vars.billing_url }}",
      },
      fr: {
        preview: "Paiement échoué",
        subject: "Action requise : paiement échoué",
        heading: "Paiement échoué",
        blocks: [
          { p: "Nous n'avons pas pu traiter votre paiement de **{{ vars.amount }}**. Merci de mettre à jour vos informations." },
          { button: { label: "Mettre à jour", hrefVar: "vars.billing_url" } },
          { muted: "Le service reste actif pendant {{ vars.grace_period or '3 jours' }}." },
        ],
        text: "Paiement {{ vars.amount }} échoué. MAJ : {{ vars.billing_url }}",
      },
    },
  },

  // ── SYSTEM ──────────────────────────────────────────────────────────
  {
    id: "alert",
    category: "system",
    locales: {
      en: {
        preview: "{{ vars.title }}",
        subject: "[ALERT] {{ vars.title }}",
        heading: "{{ vars.title }}",
        blocks: [
          { p: "{{ vars.message }}" },
          { muted: "Triggered {{ vars.triggered_at }} — severity: {{ vars.severity }}" },
          { button: { label: "View details", hrefVar: "vars.dashboard_url" } },
        ],
        text: "[ALERT] {{ vars.title }}: {{ vars.message }}",
      },
      fr: {
        preview: "{{ vars.title }}",
        subject: "[ALERTE] {{ vars.title }}",
        heading: "{{ vars.title }}",
        blocks: [
          { p: "{{ vars.message }}" },
          { muted: "Déclenché {{ vars.triggered_at }} — sévérité : {{ vars.severity }}" },
          { button: { label: "Voir les détails", hrefVar: "vars.dashboard_url" } },
        ],
        text: "[ALERTE] {{ vars.title }} : {{ vars.message }}",
      },
    },
  },
  {
    id: "report",
    category: "system",
    locales: {
      en: {
        preview: "Your {{ vars.period }} report",
        subject: "Your {{ vars.period }} report is ready",
        heading: "{{ vars.period | title }} report",
        blocks: [
          { p: "Here's a quick summary for **{{ vars.period }}**." },
          { kv: [] },
          { button: { label: "Open full report", hrefVar: "vars.report_url" } },
        ],
        text: "{{ vars.period }} report: {{ vars.report_url }}",
      },
      fr: {
        preview: "Votre rapport {{ vars.period }}",
        subject: "Votre rapport {{ vars.period }} est prêt",
        heading: "Rapport {{ vars.period }}",
        blocks: [
          { p: "Voici un résumé rapide pour **{{ vars.period }}**." },
          { kv: [] },
          { button: { label: "Ouvrir le rapport", hrefVar: "vars.report_url" } },
        ],
        text: "Rapport {{ vars.period }} : {{ vars.report_url }}",
      },
    },
  },
  {
    id: "password-changed",
    category: "system",
    locales: {
      en: {
        preview: "Your password was changed",
        subject: "Your password was changed",
        heading: "Password changed",
        blocks: [
          { p: "The password on your account was changed on {{ vars.changed_at }}." },
          { muted: "If this wasn't you, reset your password immediately." },
          { button: { label: "Secure my account", hrefVar: "vars.security_url" } },
        ],
        text: "Your password was changed on {{ vars.changed_at }}.",
      },
      fr: {
        preview: "Votre mot de passe a changé",
        subject: "Votre mot de passe a été modifié",
        heading: "Mot de passe modifié",
        blocks: [
          { p: "Le mot de passe de votre compte a été modifié le {{ vars.changed_at }}." },
          { muted: "Si ce n'était pas vous, réinitialisez-le immédiatement." },
          { button: { label: "Sécuriser mon compte", hrefVar: "vars.security_url" } },
        ],
        text: "Mot de passe modifié le {{ vars.changed_at }}.",
      },
    },
  },
  {
    id: "login-new-device",
    category: "system",
    locales: {
      en: {
        preview: "New sign-in on {{ vars.device }}",
        subject: "New sign-in from {{ vars.device }}",
        heading: "New device signed in",
        blocks: [
          { p: "A new sign-in was detected." },
          {
            kv: [
              { label: "Device", valueExpr: "vars.device" },
              { label: "Location", valueExpr: "vars.location" },
              { label: "Time", valueExpr: "vars.time" },
              { label: "IP", valueExpr: "vars.ip" },
            ],
          },
          { button: { label: "Review activity", hrefVar: "vars.security_url" } },
        ],
        text: "New sign-in from {{ vars.device }} at {{ vars.location }} ({{ vars.time }}).",
      },
      fr: {
        preview: "Nouvelle connexion depuis {{ vars.device }}",
        subject: "Nouvelle connexion depuis {{ vars.device }}",
        heading: "Nouvelle connexion détectée",
        blocks: [
          { p: "Une nouvelle connexion a été détectée." },
          {
            kv: [
              { label: "Appareil", valueExpr: "vars.device" },
              { label: "Lieu", valueExpr: "vars.location" },
              { label: "Heure", valueExpr: "vars.time" },
              { label: "IP", valueExpr: "vars.ip" },
            ],
          },
          { button: { label: "Vérifier l'activité", hrefVar: "vars.security_url" } },
        ],
        text: "Nouvelle connexion depuis {{ vars.device }} à {{ vars.location }} ({{ vars.time }}).",
      },
    },
  },
  {
    id: "account-deleted",
    category: "system",
    locales: {
      en: {
        preview: "Your account has been deleted",
        subject: "Your {{ theme.brand_name }} account has been deleted",
        heading: "Account deleted",
        blocks: [
          { p: "Your account and associated data have been permanently deleted as requested." },
          { muted: "This action cannot be undone. If you change your mind, you'll need to sign up again." },
        ],
        text: "Your {{ theme.brand_name }} account has been deleted.",
      },
      fr: {
        preview: "Votre compte a été supprimé",
        subject: "Votre compte {{ theme.brand_name }} a été supprimé",
        heading: "Compte supprimé",
        blocks: [
          { p: "Votre compte et les données associées ont été définitivement supprimés, comme demandé." },
          { muted: "Cette action est irréversible. Vous devrez créer un nouveau compte pour revenir." },
        ],
        text: "Votre compte {{ theme.brand_name }} a été supprimé.",
      },
    },
  },

  // ── MARKETING ───────────────────────────────────────────────────────
  {
    id: "welcome",
    category: "marketing",
    locales: {
      en: {
        preview: "Welcome to {{ theme.brand_name }}",
        subject: "Welcome to {{ theme.brand_name }}, {{ vars.name or 'friend' }}!",
        heading: "Welcome aboard",
        blocks: [
          { p: "Hey {{ vars.name or 'there' }}, we're glad you joined {{ theme.brand_name }}." },
          { p: "Here are a few things to get you started." },
          { button: { label: "Get started", hrefVar: "vars.get_started_url" } },
        ],
        text: "Welcome to {{ theme.brand_name }}! {{ vars.get_started_url }}",
      },
      fr: {
        preview: "Bienvenue sur {{ theme.brand_name }}",
        subject: "Bienvenue sur {{ theme.brand_name }}, {{ vars.name or '' }} !",
        heading: "Bienvenue à bord",
        blocks: [
          { p: "Salut {{ vars.name or '' }}, ravis de vous compter parmi nous sur {{ theme.brand_name }}." },
          { p: "Voici quelques pistes pour commencer." },
          { button: { label: "Commencer", hrefVar: "vars.get_started_url" } },
        ],
        text: "Bienvenue sur {{ theme.brand_name }} ! {{ vars.get_started_url }}",
      },
    },
  },
  {
    id: "newsletter",
    category: "marketing",
    locales: {
      en: {
        preview: "{{ vars.issue_title }}",
        subject: "{{ vars.issue_title }}",
        heading: "{{ vars.issue_title }}",
        blocks: [
          { p: "{{ vars.intro }}" },
          { button: { label: "Read more", hrefVar: "vars.issue_url" } },
        ],
        text: "{{ vars.issue_title }} — {{ vars.issue_url }}",
      },
      fr: {
        preview: "{{ vars.issue_title }}",
        subject: "{{ vars.issue_title }}",
        heading: "{{ vars.issue_title }}",
        blocks: [
          { p: "{{ vars.intro }}" },
          { button: { label: "Lire la suite", hrefVar: "vars.issue_url" } },
        ],
        text: "{{ vars.issue_title }} — {{ vars.issue_url }}",
      },
    },
  },
  {
    id: "announcement",
    category: "marketing",
    locales: {
      en: {
        preview: "{{ vars.headline }}",
        subject: "{{ vars.headline }}",
        heading: "{{ vars.headline }}",
        blocks: [
          { p: "{{ vars.body }}" },
          { button: { label: "Learn more", hrefVar: "vars.learn_more_url" } },
        ],
        text: "{{ vars.headline }} — {{ vars.learn_more_url }}",
      },
      fr: {
        preview: "{{ vars.headline }}",
        subject: "{{ vars.headline }}",
        heading: "{{ vars.headline }}",
        blocks: [
          { p: "{{ vars.body }}" },
          { button: { label: "En savoir plus", hrefVar: "vars.learn_more_url" } },
        ],
        text: "{{ vars.headline }} — {{ vars.learn_more_url }}",
      },
    },
  },
  {
    id: "feedback-request",
    category: "marketing",
    locales: {
      en: {
        preview: "Got a minute?",
        subject: "Mind sharing your thoughts on {{ theme.brand_name }}?",
        heading: "We'd love your feedback",
        blocks: [
          { p: "Hi {{ vars.name or 'there' }} — a quick {{ vars.duration or '2-minute' }} survey would really help us improve." },
          { button: { label: "Give feedback", hrefVar: "vars.survey_url" } },
        ],
        text: "Feedback survey: {{ vars.survey_url }}",
      },
      fr: {
        preview: "Un instant ?",
        subject: "Partagez votre avis sur {{ theme.brand_name }}",
        heading: "Votre avis nous intéresse",
        blocks: [
          { p: "Bonjour {{ vars.name or '' }} — un rapide sondage de {{ vars.duration or '2 minutes' }} nous aiderait énormément." },
          { button: { label: "Donner mon avis", hrefVar: "vars.survey_url" } },
        ],
        text: "Sondage : {{ vars.survey_url }}",
      },
    },
  },
  {
    id: "re-engagement",
    category: "marketing",
    locales: {
      en: {
        preview: "We've missed you",
        subject: "We've missed you at {{ theme.brand_name }}",
        heading: "Long time, no see",
        blocks: [
          { p: "It's been a while, {{ vars.name or 'there' }}. Here's what's new at {{ theme.brand_name }}." },
          { button: { label: "Come back", hrefVar: "vars.return_url" } },
        ],
        text: "We've missed you. {{ vars.return_url }}",
      },
      fr: {
        preview: "Vous nous avez manqué",
        subject: "Vous nous avez manqué chez {{ theme.brand_name }}",
        heading: "Ça fait un bail",
        blocks: [
          { p: "Ça fait longtemps, {{ vars.name or '' }}. Voici les nouveautés chez {{ theme.brand_name }}." },
          { button: { label: "Revenir", hrefVar: "vars.return_url" } },
        ],
        text: "Vous nous avez manqué. {{ vars.return_url }}",
      },
    },
  },
];
