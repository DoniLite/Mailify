use std::fs;

use mailify_config::I18nConfig;
use mailify_templates::TemplateRegistry;

fn i18n() -> I18nConfig {
    I18nConfig {
        default_locale: "en".into(),
        fallback_chain: vec!["en".into()],
        supported_locales: vec!["en".into(), "fr".into()],
    }
}

#[test]
fn load_templates_and_fallback_chain() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();

    // Template `welcome` w/ en + fr assets.
    fs::create_dir_all(root.join("welcome")).unwrap();
    fs::write(root.join("welcome/en.html"), "<p>Hi {{ vars.name }}</p>").unwrap();
    fs::write(root.join("welcome/fr.html"), "<p>Salut {{ vars.name }}</p>").unwrap();
    fs::write(root.join("welcome/subject.en.txt"), "Welcome").unwrap();
    fs::write(root.join("welcome/subject.fr.txt"), "Bienvenue").unwrap();

    // Template `only-en` — no French.
    fs::create_dir_all(root.join("only-en")).unwrap();
    fs::write(root.join("only-en/en.html"), "<p>english only</p>").unwrap();
    fs::write(root.join("only-en/subject.en.txt"), "English-only").unwrap();

    let reg = TemplateRegistry::load_from_dir(root, i18n(), false).unwrap();

    // direct hit
    let fr = reg.get("welcome", "fr").unwrap();
    assert!(fr.html.contains("Salut"));

    // regional → base → default fallback
    let fr_ca = reg.get("welcome", "fr-CA").unwrap();
    assert!(fr_ca.html.contains("Salut"));

    // missing locale → default
    let fallback = reg.get("only-en", "fr").unwrap();
    assert!(fallback.html.contains("english only"));

    // unknown template
    assert!(reg.get("does-not-exist", "en").is_err());

    // list
    let mut ids = reg.list_ids();
    ids.sort();
    assert_eq!(ids, vec!["only-en", "welcome"]);
}

#[test]
fn strict_mode_fails_when_default_locale_missing() {
    let tmp = tempfile::tempdir().unwrap();
    fs::create_dir_all(tmp.path().join("fr-only")).unwrap();
    fs::write(tmp.path().join("fr-only/fr.html"), "<p>fr</p>").unwrap();

    let res = TemplateRegistry::load_from_dir(tmp.path(), i18n(), true);
    assert!(res.is_err());
}
