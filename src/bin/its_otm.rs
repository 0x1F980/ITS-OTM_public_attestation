mod io_util {
    use std::fs;
    use std::io::{self, Read, Write};
    use std::path::Path;

    pub fn read_bytes(path: &str) -> io::Result<Vec<u8>> {
        if path == "-" {
            let mut buf = Vec::new();
            io::stdin().read_to_end(&mut buf)?;
            Ok(buf)
        } else {
            fs::read(path)
        }
    }

    pub fn read_text(path: &str) -> io::Result<String> {
        if path == "-" {
            let mut buf = String::new();
            io::stdin().read_to_string(&mut buf)?;
            Ok(buf)
        } else {
            fs::read_to_string(path)
        }
    }

    pub fn write_text(path: &str, text: &str) -> io::Result<()> {
        if path == "-" {
            print!("{text}");
            io::stdout().flush()
        } else {
            fs::write(path, text)
        }
    }

}

use std::env;
use std::fs;
use std::io::{self, Write};
use std::process;

use io_util::{read_bytes, read_text, write_text};
use its_otm_public_attestation::field_arith::FieldElement;
use its_otm_public_attestation::poly::Polynomial;
use its_otm_public_attestation::{
    message_from_bytes, verify_public_attestation, OtmChainSigner, PublicAttestationBundle,
};

struct OsRng;

impl OsRng {
    fn fill_u32(&mut self) -> u32 {
        let mut buf = [0u8; 4];
        getrandom::fill(&mut buf).expect("urandom");
        u32::from_be_bytes(buf)
    }

    fn field_nonzero(&mut self) -> FieldElement {
        loop {
            let v = self.fill_u32() % 2147483646 + 1;
            if v != 0 {
                return FieldElement::new(v);
            }
        }
    }
}

mod getrandom {
    use std::fs::File;
    use std::io::Read;

    pub fn fill(dest: &mut [u8]) -> std::io::Result<()> {
        let mut f = File::open("/dev/urandom")?;
        f.read_exact(dest)
    }
}

fn usage() -> ! {
    eprintln!(
        "its_otm v0.1 — ITS-OTM public attestation (WC-MAC + SSS chain)

USAGE:
  its_otm keygen --out STATE
  its_otm sign --state STATE --in PATH --out PATH
  its_otm verify --bundle PATH [--payload PATH]
  its_otm demo

PATH may be \"-\" for stdin (read) or stdout (write).

PIPE EXAMPLES:
  its_asymmetric encrypt --pk public.key --in plain.txt --out - | \\
    its_otm sign --state alice.state --in - --out attestation.otm
  its_otm sign --state alice.state --in public.key --out pk.otm
  its_otm verify --bundle attestation.otm --payload msg.wire

State file (alice.state) holds the signer chain; keep local and sequential."
    );
    process::exit(1);
}

fn parse_flag(args: &[String], flag: &str) -> Option<String> {
    args.iter()
        .position(|a| a == flag)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn require_flag(args: &[String], flag: &str) -> String {
    parse_flag(args, flag).unwrap_or_else(|| {
        eprintln!("missing {flag}");
        usage();
    })
}

#[derive(Clone, Debug)]
struct SignerState {
    master_root: FieldElement,
    backward_c0: FieldElement,
    backward_c1: FieldElement,
    prev_backward_x: FieldElement,
    prev_backward_y: FieldElement,
    prev_msg: FieldElement,
    step_counter: u64,
}

impl SignerState {
    fn to_signer(&self) -> OtmChainSigner<2> {
        OtmChainSigner {
            master_root: self.master_root,
            poly_backward: Polynomial::new([self.backward_c0, self.backward_c1]),
            prev_back_point: (self.prev_backward_x, self.prev_backward_y),
            prev_msg: self.prev_msg,
            step_counter: self.step_counter,
        }
    }

    fn from_signer(s: &OtmChainSigner<2>) -> Self {
        SignerState {
            master_root: s.master_root,
            backward_c0: s.poly_backward.coeffs[0],
            backward_c1: s.poly_backward.coeffs[1],
            prev_backward_x: s.prev_back_point.0,
            prev_backward_y: s.prev_back_point.1,
            prev_msg: s.prev_msg,
            step_counter: s.step_counter,
        }
    }
}

fn state_to_text(st: &SignerState) -> String {
    format!(
        "master_root: {}\n\
         backward_c0: {}\n\
         backward_c1: {}\n\
         prev_backward_x: {}\n\
         prev_backward_y: {}\n\
         prev_msg: {}\n\
         step_counter: {}\n",
        st.master_root.value(),
        st.backward_c0.value(),
        st.backward_c1.value(),
        st.prev_backward_x.value(),
        st.prev_backward_y.value(),
        st.prev_msg.value(),
        st.step_counter,
    )
}

fn parse_u32_map(text: &str) -> Result<std::collections::HashMap<String, u32>, String> {
    let mut fields = std::collections::HashMap::new();
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let (k, v) = line
            .split_once(':')
            .ok_or_else(|| format!("invalid line: {line}"))?;
        fields.insert(
            k.trim().to_string(),
            v.trim()
                .parse()
                .map_err(|_| format!("bad value: {line}"))?,
        );
    }
    Ok(fields)
}

fn parse_state_text(text: &str) -> Result<SignerState, String> {
    let f = parse_u32_map(text)?;
    let get = |k: &str| -> Result<FieldElement, String> {
        f.get(k)
            .copied()
            .map(FieldElement::new)
            .ok_or_else(|| format!("missing field: {k}"))
    };
    let step = f
        .get("step_counter")
        .copied()
        .ok_or_else(|| "missing step_counter".to_string())?;
    Ok(SignerState {
        master_root: get("master_root")?,
        backward_c0: get("backward_c0")?,
        backward_c1: get("backward_c1")?,
        prev_backward_x: get("prev_backward_x")?,
        prev_backward_y: get("prev_backward_y")?,
        prev_msg: get("prev_msg")?,
        step_counter: step as u64,
    })
}

fn load_state(path: &str) -> SignerState {
    let text = read_text(path).unwrap_or_else(|e| {
        eprintln!("read state: {e}");
        process::exit(1);
    });
    parse_state_text(&text).unwrap_or_else(|e| {
        eprintln!("parse state: {e}");
        process::exit(1);
    })
}

fn save_state(path: &str, st: &SignerState) {
    let text = state_to_text(st);
    if path == "-" {
        print!("{text}");
        let _ = io::stdout().flush();
    } else {
        fs::write(path, text).unwrap_or_else(|e| {
            eprintln!("write state: {e}");
            process::exit(1);
        });
    }
}

pub fn bundle_to_text(bundle: &PublicAttestationBundle<2>, payload_len: usize) -> String {
    let mut out = String::new();
    out.push_str(&format!("# ITS-OTM public attestation bundle\n"));
    out.push_str(&format!("payload_len: {payload_len}\n"));
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
        out.push_str(&format!(
            "prev_backward_x: {}\nprev_backward_y: {}\n",
            x.value(),
            y.value()
        ));
    }
    out
}

pub fn parse_bundle_text(text: &str) -> Result<(PublicAttestationBundle<2>, usize), String> {
    let f = parse_u32_map(text)?;
    let payload_len = f.get("payload_len").copied().unwrap_or(0) as usize;
    let get = |k: &str| -> Result<FieldElement, String> {
        f.get(k)
            .copied()
            .map(FieldElement::new)
            .ok_or_else(|| format!("missing field: {k}"))
    };

    let prev_backward_points = if f.contains_key("prev_backward_x") {
        vec![(get("prev_backward_x")?, get("prev_backward_y")?)]
    } else {
        vec![]
    };

    let bundle = PublicAttestationBundle {
        message: get("message")?,
        forward_point: (get("forward_x")?, get("forward_y")?),
        backward_point: (get("backward_x")?, get("backward_y")?),
        tag: get("tag")?,
        k_mac_reveal: get("k_mac")?,
        nonce_reveal: get("nonce")?,
        master_root: get("master_root")?,
        prev_backward_points,
        forward_poly: Polynomial::new([get("forward_c0")?, get("forward_c1")?]),
    };
    Ok((bundle, payload_len))
}

fn cmd_keygen(args: &[String]) {
    let out = require_flag(args, "--out");
    let mut rng = OsRng;
    let master_root = rng.field_nonzero();
    let backward_c1 = rng.field_nonzero();
    let poly = Polynomial::new([master_root, backward_c1]);
    let prev_x = rng.field_nonzero();
    let prev_y = poly.evaluate(prev_x);
    let prev_msg = rng.field_nonzero();
    let st = SignerState {
        master_root,
        backward_c0: master_root,
        backward_c1,
        prev_backward_x: prev_x,
        prev_backward_y: prev_y,
        prev_msg,
        step_counter: 1,
    };
    save_state(&out, &st);
    if out != "-" {
        eprintln!("Signer state written: {out}");
    }
}

fn cmd_sign(args: &[String]) {
    let state_path = require_flag(args, "--state");
    let in_path = require_flag(args, "--in");
    let out_path = require_flag(args, "--out");
    let quiet_out = out_path == "-";

    let mut st = load_state(&state_path);
    let payload = read_bytes(&in_path).unwrap_or_else(|e| {
        eprintln!("read payload: {e}");
        process::exit(1);
    });
    if payload.is_empty() {
        eprintln!("sign: empty payload");
        process::exit(1);
    }

    let message = message_from_bytes(&payload);
    let mut signer = st.to_signer();
    let mut rng = OsRng;
    let bundle = signer.sign(
        message,
        rng.field_nonzero(),
        rng.field_nonzero(),
        rng.field_nonzero(),
    );
    st = SignerState::from_signer(&signer);
    save_state(&state_path, &st);

    let text = bundle_to_text(&bundle, payload.len());
    write_text(&out_path, &text).unwrap_or_else(|e| {
        eprintln!("write bundle: {e}");
        process::exit(1);
    });

    if quiet_out {
        eprintln!(
            "Signed {} bytes (message field {}) -> stdout; state updated",
            payload.len(),
            message.value()
        );
    } else {
        eprintln!(
            "Signed {} bytes -> {out_path} (message field {}); state updated",
            payload.len(),
            message.value()
        );
    }
}

fn cmd_verify(args: &[String]) {
    let bundle_path = require_flag(args, "--bundle");
    let text = read_text(&bundle_path).unwrap_or_else(|e| {
        eprintln!("read bundle: {e}");
        process::exit(1);
    });
    let (bundle, payload_len) = parse_bundle_text(&text).unwrap_or_else(|e| {
        eprintln!("parse bundle: {e}");
        process::exit(1);
    });

    let chain_ok = bool::from(verify_public_attestation(&bundle));
    let mut payload_ok = true;
    if let Some(payload_path) = parse_flag(args, "--payload") {
        let payload = read_bytes(&payload_path).unwrap_or_else(|e| {
            eprintln!("read payload: {e}");
            process::exit(1);
        });
        let msg = message_from_bytes(&payload);
        payload_ok = msg.value() == bundle.message.value();
        if payload_len != 0 && payload.len() != payload_len {
            eprintln!(
                "WARN payload length {} != bundle payload_len {}",
                payload.len(),
                payload_len
            );
        }
    }

    if chain_ok && payload_ok {
        println!("VALID message={}", bundle.message.value());
        process::exit(0);
    }
    if !chain_ok {
        eprintln!("INVALID chain/tag");
    }
    if !payload_ok {
        eprintln!("INVALID payload does not match message field");
    }
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

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        usage();
    }

    match args[1].as_str() {
        "keygen" => cmd_keygen(&args),
        "sign" => cmd_sign(&args),
        "verify" => cmd_verify(&args),
        "demo" => {
            let bundle = demo_bundle();
            let text = bundle_to_text(&bundle, 0);
            print!("{text}");
            let ok = bool::from(verify_public_attestation(&bundle));
            let _ = io::stdout().flush();
            process::exit(if ok { 0 } else { 1 });
        }
        "-h" | "--help" | "help" => usage(),
        _ => usage(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bundle_roundtrip_text() {
        let b = demo_bundle();
        let text = bundle_to_text(&b, 12);
        let (b2, len) = parse_bundle_text(&text).unwrap();
        assert_eq!(len, 12);
        assert_eq!(b.message.value(), b2.message.value());
        assert!(bool::from(verify_public_attestation(&b2)));
    }
}
