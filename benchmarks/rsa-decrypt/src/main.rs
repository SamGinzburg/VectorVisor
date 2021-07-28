use wasm_serverless_invoke::wasm_handler;
use wasm_serverless_invoke::wasm_handler::WasmHandler;
use serde::Deserialize;
use serde::Serialize;
use rsa::{PublicKey, RSAPrivateKey, RSAPublicKey, PaddingScheme};
use rand_core::OsRng;
use base64::{encode, decode};
use lazy_static::lazy_static;
use rsa::PrivateKeyEncoding;
use std::convert::TryFrom;
use std::borrow::Cow;

#[derive(Debug, Deserialize)]
struct FuncInput<'a> {
    encoded_str: Cow<'a, str>
}

#[derive(Debug, Serialize)]
struct FuncResponse {
    encoded_resp: String
}

const raw_pkey: &'static str = r#"
-----BEGIN RSA PRIVATE KEY-----
MIIEowIBAAKCAQEAz5ELWL4OP15pGYTHqORmo89ET2gGCZWLU6NpFaHli9tUOYsp
3OHDkcH2toQvKbrnzpw4Q0COC23SDEfOaQqDBuQwGvttt2bPd6j0Eacg8uHj6MhE
odm50gzwrcYmLtFuOdniOm+recLrghFSwI3Hkt9I+2pe7r7bCNphfi9A17ESajRb
zXWMvIypqM62hZbygHvH09IDjGxte8vuuCEaN0B2u6s9SESn32NQ2Vg7BeGPDJGY
orsod84Iwlm7NtuTWJ7r15ES9hOXjY1+VIVvKa60cqIfO8/JVI1nspjb3PMekwgF
kZE7/Yj87oJ6KNhWPYK4vxXzSwIWJ29aTpRaLwIDAQABAoIBABYyDGSQ7jUYg2SX
LkKBKfiaUVOsa/jaZOusrbACf+lUqsz8dJ2KBr+vXhJmUCTGN8OAMQQAAzB1reBi
TVAuL5StEffQqwrRk53YE0FP/dX7EcPypmQoRBaSC6qJZcvb2U2WXvqgtlVu4oAm
GhM9Ffo+pi88UKp2/jUUJRi5NNXbh/JF9Ua2FpHhfyzEGWbCS/oyowDgd2/FgYNr
Sbuxk9HyvAXGvjJ09yKLdDSd/hUYe7eU4A84zyQ1qDph6QnQX0XebyvPVrK60FG8
8CaRzdIg+k8wqS9OadPmSB1iYRDw1dy6RnZOVKfTDiWDFIkQlOOIJy/jb8rHbjCI
MjwF0AECgYEA/nJyALDnwyyuIf5n2EEkEPq6kGhhqOCOQMnzyyGBn4yJwj215RT2
e2DgJOM6DM3IT5C2YaMfbzfNztz9pxpDRzRpzbqer3gvPFDTRVWY/E2o86uMiaz/
J59pv2w0mjl+NGflwO+KtgXnzn6Ni9PcNALpgmufuoHEKdyvGwSqtm8CgYEA0NVa
Cyj/Rlk36oFeddWOQFXg4V+fZJlGv6gGaqOnGFil1FPGcP8r7IMX91sm2VuDAhyQ
tRhqsAHfCo1RbtyLG2br38jWPgKDOOEQp+r65nEbgjuUG/0AzzZN4paqczKVZZTr
Nf/za3DrMQv3fIAMkuHdyeeHxIpwcdiLix3IeEECgYBgGHrjtewhswskvX8cliV3
4Cl7hmBztjMjVuIi8kmRJzOTz1iV5t/b4s9kPwjacxFWs1Gd+ExT5aBtotqNNIb6
KlpXH0b1AA/e2KuhwN0hVHcdZ8mQ6WwH29XFPKl1IYd/ZUnJHu6lJf/Q5cyxZVcB
saPc5KaTepTKNpVPP+j1MwKBgBDDn8fXka6HCPsVSYzyCMpXWocdEwGTYF+QUHuN
CaPlseI6m2qEwXTqDSl5MsQuGXU479Dp1d6tN8d0Er5wi+Y2O+cqCzDNKNWPE7Q3
I88N82Rf/gLFK6R2uoffCm3W+LE/5CXglxzf4rF8QXDIIrpD0Xcc1ARODLrG5GTV
jjtBAoGBAK+8UbaRgakbwDW0UmQ7T2uSkbgpkfgiw2vanwhfnsuVGkdJSw/O8AiL
4aUYwY1cYpS4m3URJ0G9Sy9qxR9tOzrpd+k5J/9bbG/Fvtn07xSDBBXBmooVpNi9
XRMxYnSgLumr/83ikV+a+Er1+1tknbjSHO99oHUI+mheoMb0DTdh
-----END RSA PRIVATE KEY-----
"#;


lazy_static! {
    static ref RSA_PKEY: RSAPrivateKey = RSAPrivateKey::try_from(rsa::pem::parse(raw_pkey).unwrap()).expect("failed to parse key");
}

fn rsa_decrypt(event: FuncInput) -> FuncResponse {
    let mut decoded_str = decode(event.encoded_str.as_bytes()).unwrap();
    let padding = PaddingScheme::new_pkcs1v15_encrypt();
    let dec_data = RSA_PKEY.decrypt(padding, &decoded_str).expect("failed to decrypt");
    FuncResponse { encoded_resp: encode(dec_data) }
}

fn main() {
    let handler = WasmHandler::new(&rsa_decrypt);
    handler.run(1024*1024);
}
