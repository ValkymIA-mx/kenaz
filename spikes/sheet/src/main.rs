use qrcode::{render::svg, QrCode};
use serde_json::Value;
use std::{error::Error, fs, path::PathBuf, process::Command, time::Instant};
use url::Url;

#[cfg(feature = "embedded")]
mod embedded;

fn main() -> Result<(), Box<dyn Error>> {
    let spike = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let repo = spike.join("../..").canonicalize()?;
    let output = spike.join("target/sheet-sample");
    fs::create_dir_all(&output)?;
    let template = repo.join("templates/recovery-sheet/recovery-sheet.typ");
    let mut data: Value = serde_json::from_slice(&fs::read(repo.join("templates/recovery-sheet/example.json"))?)?;
    // Replace the existing example QR only, so the other account's layout stays comparable.
    let account = &mut data["oath"][0];
    let text = |key: &str| -> Result<&str, Box<dyn Error>> {
        account[key].as_str().ok_or_else(|| format!("missing {key}").into())
    };
    let digits = account["digits"].as_u64().ok_or("missing digits")?.to_string();
    let period = account["period"].as_u64().ok_or("missing period")?.to_string();
    let mut uri = Url::parse("otpauth://totp")?;
    uri.path_segments_mut().map_err(|_| "invalid URI")?.clear().push(&format!("{}:{}", text("issuer")?, text("account")?));
    uri.query_pairs_mut()
        .append_pair("secret", text("secret")?)
        .append_pair("issuer", text("issuer")?)
        .append_pair("algorithm", text("algorithm")?)
        .append_pair("digits", &digits)
        .append_pair("period", &period);
    let qr = QrCode::new(uri.as_str().as_bytes())?.render::<svg::Color>().min_dimensions(256, 256).build();
    account["qr_svg"] = Value::String(qr.clone());
    fs::write(output.join("qr.svg"), qr)?;
    let json = output.join("example-with-qr.json");
    fs::write(&json, serde_json::to_vec(&data)?)?;
    for lang in ["es", "en"] {
        let pdf = output.join(format!("example-{lang}.pdf"));
        let start = Instant::now();
        let status = Command::new("typst")
            .arg("compile").arg("--root").arg(&repo)
            .arg("--font-path").arg(spike.join("fonts"))
            .args(["--ignore-system-fonts", "--ignore-embedded-fonts"])
            .arg("--input").arg(format!("data=/spikes/sheet/target/sheet-sample/example-with-qr.json"))
            .arg("--input").arg(format!("lang={lang}"))
            .arg(&template).arg(&pdf).status()?;
        if !status.success() { return Err(format!("typst failed for {lang}: {status}").into()); }
        println!("{lang}: {} ms, {} bytes: {}", start.elapsed().as_millis(), fs::metadata(&pdf)?.len(), pdf.display());
        #[cfg(feature = "embedded")]
        embedded::render(&spike, &template, &json, &output, lang)?;
    }
    Ok(())
}
