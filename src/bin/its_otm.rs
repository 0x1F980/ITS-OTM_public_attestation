//! Standalone CLI for ITS-OTM public attestation bundles (.otm text format).

use std::env;
use std::io::{self, Write};
use std::process;

use its_otm_public_attestation::field_arith::FieldElement;
use its_otm_public_attestation::poly::Polynomial;
use its_otm_public_attestation::{
    verify_public_attestation, OtmChainSigner, PublicAttestationBundle,
};

fn usage() -> ! {
    eprintln!(
        "Usage:\n  \
         its_otm demo\n  \
         its_otm verify --bundle FILE\n\n\
         The demo command prints a deterministic M31 attestation bundle and verifies it."
    );
    process::exit(1);
}

fn demo_bundle() -> PublicAttestationBundle<2> {
    let master_root = FieldElement::new(5);
    let poly_backward = Polynomial::new([FieldElement::new(5), FieldElement::new(3)]);
    let mut signer = OtmChainSigner::<2>::new(
        master_root,
        poly_backward,
        (FieldElement::new(1), FieldElement::new(8)),
        FieldElement::new(99),
    );
    signer.sign(
        FieldElement::new(42),
        FieldElement::new(7),
        FieldElement::new(11),
        FieldElement::new(13),
    )
}

fn bundle_to_text(bundle: &PublicAttestationBundle<2>) -> String {
    let mut out = String::new();
    out.push_str(&format!("message: {}\n", bundle.message.value()));
    out.push_str(&format!(
        "forward_x: {}\nforward_y: {}\n",
        bundle.forward_point.0.value(),
        bundle.forward_point.1.value()
    ));
    out.push_str(&format!(
        "backward_x: {}\nbackward_y: {}\n",
        bundle.backward_point.0.value(),
        bundle.backward_point.1.value()
    ));
    out.push_str(&format!("tag: {}\n", bundle.tag.value()));
    out.push_str(&format!("k_mac: {}\n", bundle.k_mac_reveal.value()));
    out.push_str(&format!("nonce: {}\n", bundle.nonce_reveal.value()));
    out.push_str(&format!("master_root: {}\n", bundle.master_root.value()));
    out.push_str(&format!(
        "forward_c0: {}\nforward_c1: {}\n",
        bundle.forward_poly.coeffs[0].value(),
        bundle.forward_poly.coeffs[1].value()
    ));
    if let Some((x, y)) = bundle.prev_backward_points.first() {
        out.push_str(&format!("prev_backward_x: {}\nprev_backward_y: {}\n", x.value(), y.value()));
    }
    out
}

fn parse_bundle_text(text: &str) -> Result<PublicAttestationBundle<2>, String> {
    let mut fields: std::collections::HashMap<&str, u32> = std::collections::HashMap::new();
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let (k, v) = line
            .split_once(':')
            .ok_or_else(|| format!("invalid line: {line}"))?;
        fields.insert(k.trim(), v.trim().parse().map_err(|_| format!("bad value: {line}"))?);
    }

    let get = |k: &str| -> Result<FieldElement, String> {
        fields
            .get(k)
            .copied()
            .map(FieldElement::new)
            .ok_or_else(|| format!("missing field: {k}"))
    };

    let prev_backward_points = if fields.contains_key("prev_backward_x") {
        vec![(get("prev_backward_x")?, get("prev_backward_y")?)]
    } else {
        vec![]
    };

    Ok(PublicAttestationBundle {
        message: get("message")?,
        forward_point: (get("forward_x")?, get("forward_y")?),
        backward_point: (get("backward_x")?, get("backward_y")?),
        tag: get("tag")?,
        k_mac_reveal: get("k_mac")?,
        nonce_reveal: get("nonce")?,
        master_root: get("master_root")?,
        prev_backward_points,
        forward_poly: Polynomial::new([get("forward_c0")?, get("forward_c1")?]),
    })
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        usage();
    }

    match args[1].as_str() {
        "demo" => {
            let bundle = demo_bundle();
            let text = bundle_to_text(&bundle);
            print!("{text}");
            let ok = bool::from(verify_public_attestation(&bundle));
            let _ = io::stdout().flush();
            process::exit(if ok { 0 } else { 1 });
        }
        "verify" => {
            let path = if let Some(idx) = args.iter().position(|a| a == "--bundle") {
                args.get(idx + 1).cloned()
            } else {
                None
            };
            let path = path.unwrap_or_else(|| {
                eprintln!("verify requires --bundle FILE");
                process::exit(1);
            });
            let text = std::fs::read_to_string(&path).unwrap_or_else(|e| {
                eprintln!("read error: {e}");
                process::exit(1);
            });
            let bundle = parse_bundle_text(&text).unwrap_or_else(|e| {
                eprintln!("parse error: {e}");
                process::exit(1);
            });
            let ok = bool::from(verify_public_attestation(&bundle));
            if ok {
                println!("VALID");
            } else {
                println!("INVALID");
            }
            process::exit(if ok { 0 } else { 1 });
        }
        _ => usage(),
    }
}
