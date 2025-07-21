use rsa_wasm::*;

// 2048-bit RSA key pair for testing
const PRIV_KEY: &str = "-----BEGIN PRIVATE KEY-----
MIIEvQIBADANBgkqhkiG9w0BAQEFAASCBKcwggSjAgEAAoIBAQCxrpruV6szxDiB
tD+tnaJ0d0WoEgQhYUuBe3PGNxNOioE+OppaVV4IQ1188ADoxfHbea8Xem+hWv5k
R4mRJU5uwnzuPm+U3ZVr6WAn+nbeOrTRazKgFZjgQ9N4a9bxBVnDv4s1w7BsHT6a
qzwnIjbszap2fEsIMzuYCRcylVLCW7jOj4GmIR2zoBQvqkDu1uFty3+tCN7p8FCu
cKCfshTRK5oOyWgf6XEiRsMzJz985EHheiHYciqWcx/9W3oF+5/OWxRzxqOU6lza
7KroGmCPJDfCIxSDARrr2EhqmIeN4xihmZDFrw8S+gAcEhkdHNigxabSGmJ5GHCQ
2tGhqF5vAgMBAAECggEAB8pD+KyB+7m1UIUlQD4SX/uADlJb5AAZXgqaP4KCp4WN
t7tnW7aKcjT3OwG6yHnalL4iXcEnIR1AMggA5rWKrsh6zxHltSfR4/Kjc0fulj0K
Ax8pZQ4NWdJ0yb7QpFiRCvKxmSCEnPjbRHoQH8HG7rLwWlQwu0yjvD1fIFSeVZm6
dUEUBxqql7hhKiLW+nhB/UTgJst65PG6rEwZDP8weuHhbCZVAn2ziPlMYQV1zoL/
CBmECAL3WiNY3kddINb91SOsf284IwcYZtZBqvdGqL7Zl1nCzUTXUduFF0zk0JoZ
t/z0QOExqtt7eukrVp8tT56vevwgV76hfQghn7DjAQKBgQDmcKdyDG1FMkoBlMaj
iwShZ1uAubb5V8SO7InaWUjepUWgX3cm54EUqETqbSS34JrSKsGTk90lzYuZ7Oxx
lE5yP2e4YXib3tFgyVwA9o46YAqLtHg4fbCeWiELtKbkYUqG6nNTrTccsMHtpd4f
+BPWQ2WTTMg6wAbn5E+lE4tloQKBgQDFY+Mx6lxhni0ClNSMKr/qPS8CEYa6aUAa
ynH/ytgm5m5xIQdPwjREmSz3kmiPGNRn9DUKJvkZeaqNZ0BH+v24LcMaOHZ1Rrza
ClHG8GTr2r3BW+Qp6ryJRvqvztUd35h7MjgJQcxrbf72Npcz91AsKyA+2bMIIqYI
4ds96jEqDwKBgB2DzE6VPOGiAUyWi6KiyaMKo/Om1cK19R0BOt7WlLi/nl1eHNrE
yAmnS7xi4lMS14GXrFeElsyiuQCsEdSyVCHZ8STyhwofzd2BrOEKs0jSyEcAhG1n
29lMXfMr3jxeIbeKC99pzidElHIlzn91vMSyuS3WpcawlbHZ0kGocTBBAoGAK8Dz
eHoJlj3YnkzdPGCxf7umMuAtrB5ubkHXuSp+dUj4zCzVERHsQhRuCkay1J1dX9ma
9DAlYUc9zfBIKGxzVyMVrzc60s4mSHKnI+DfjgRDsZur3LrCpe9M3DGNvfx1MUG1
eB/Ury+Hp438QDGe3NuGX0gkTjZX9XoS/gtucHMCgYEAxpM/VEEfbwCTw0IeAGnh
2Dnmvv0EJ4f3KHFE43NrQ5SBSJUlDtfA6/tdGLlWzX8yTcn6nU+/QREPZNpU8nS8
qkXrTmbB3N9AL9zoXALwdxtripQcUHSnrgzWruH1QFbeEuSuSd7WvL8QkIKfHZGo
nCrCRHN68E/MYvmj7A4bEd8=
-----END PRIVATE KEY-----";
const PUB_KEY: &str = "-----BEGIN PUBLIC KEY-----
MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAsa6a7lerM8Q4gbQ/rZ2i
dHdFqBIEIWFLgXtzxjcTToqBPjqaWlVeCENdfPAA6MXx23mvF3pvoVr+ZEeJkSVO
bsJ87j5vlN2Va+lgJ/p23jq00WsyoBWY4EPTeGvW8QVZw7+LNcOwbB0+mqs8JyI2
7M2qdnxLCDM7mAkXMpVSwlu4zo+BpiEds6AUL6pA7tbhbct/rQje6fBQrnCgn7IU
0SuaDsloH+lxIkbDMyc/fORB4Xoh2HIqlnMf/Vt6BfufzlsUc8ajlOpc2uyq6Bpg
jyQ3wiMUgwEa69hIapiHjeMYoZmQxa8PEvoAHBIZHRzYoMWm0hpieRhwkNrRoahe
bwIDAQAB
-----END PUBLIC KEY-----";

#[test]
fn encrypt_test() {
    let data = b"hello world";
    let enc_data = encrypt(&data[..], PUB_KEY);
    assert_ne!(&data[..], &enc_data[..]);
}

#[test]
fn decrypt_test() {
    let data = b"hello world";
    let enc_data = encrypt(&data[..], PUB_KEY);
    let dec_data = decrypt(&enc_data[..], PRIV_KEY);
    assert_eq!(&data[..], &dec_data[..]);
}

#[test]
fn signature_test() {
    let data = b"hello world";
    let signature = signature(&data[..], PRIV_KEY);
    assert!(!signature.is_empty());
}

#[test]
fn verify_test() {
    let data = b"hello world";
    let signature = signature(&data[..], PRIV_KEY);
    let is_valid = verify(&data[..], &signature, PUB_KEY);
    assert!(is_valid);
}
