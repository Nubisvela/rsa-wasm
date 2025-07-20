# RSA WebAssembly

RSA WebAssembly (rsa-wasm) built with the Rust [rsa](https://crates.io/crates/rsa) crate
allows you to use the RSA features in JavaScript/TypeScript.

**Features:**

- Encryption with the public key
- Decryption with the private key
- Signature with the private key
- Verification with the public key

## Usage

### Basic usage

Install the package:

```shell
npm install @nubisvela/rsa-wasm
```

Encrypt and decrypt:

```typescript
import __wbg_init, { encrypt, decrypt } from '@nubisvela/rsa-wasm'

// Initialize the wasm module
__wbg_init().then(() => {
  // RSA public key in PKCS#8 PEM format
  const publicKey = "-----BEGIN PUBLIC KEY-----...-----END PUBLIC KEY-----"
  // RSA private key in PKCS#8 PEM format
  const privateKey = "-----BEGIN PRIVATE KEY-----...-----END PRIVATE KEY-----"

  // Convert plain text string to binary bytes (Uint8Array)
  const plain = 'rsa-wasm'
  const plainBytes = new TextEncoder().encode(plain)

  // Use public key to encrypt
  const cipherBytes = encrypt(plainBytes, publicKey)

  // Use private key to decrypt
  const decryptedBytes = decrypt(cipherBytes, privateKey)
})
```

Signature and verify:

```typescript
import __wbg_init, { signature, verify } from '@nubisvela/rsa-wasm'

// Initialize the wasm module
__wbg_init().then(() => {
  // RSA public key in PKCS#8 PEM format
  const publicKey = "-----BEGIN PUBLIC KEY-----...-----END PUBLIC KEY-----"
  // RSA private key in PKCS#8 PEM format
  const privateKey = "-----BEGIN PRIVATE KEY-----...-----END PRIVATE KEY-----"

  // Convert data text string to binary bytes (Uint8Array)
  const data = 'rsa-wasm'
  const dataBytes = new TextEncoder().encode(data)

  // Use private key to sign
  const sign = signature(cipherBytes, privateKey)

  // Use public key to verify
  const isVerified = verify(sign, publicKey)
})
```

### Q&A and Tips

- **How to use the `rsa-wasm` in the frameworks like React and Vue, etc.?**

Just initialize the wasm module in the "mounting" stage of those frameworks/libraries
before using the `encrypt` and `decrypt` functions.

Use `useEffect` hook in the React component:

```typescript
import * as React from 'react'
import __wbg_init from '@nubisvela/rsa-wasm'

function MyComponent() {
  React.useEffect(() => {
    // Initialize the wasm module
    __wbg_init()
  }, [])
}
```

In the Vue component:

```typescript
<script setup>
import { onMounted } from 'vue'
import __wbg_init from '@nubisvela/rsa-wasm'

onMounted(() => {
  // Initialize the wasm module
  __wbg_init()
})
</script>
```

Or load the wasm module on a need basis:

```typescript
import __wbg_init, { encrypt, decrypt } from '@nubisvela/rsa-wasm'

async function cryptography() {
  // Initialize the wasm module
  await __wbg_init()

  // Call encrypt() and decrypt() here...
}
```

You can also load and initialize the wasm module by using build tool like vite and webpack,
we don't discuss those situations here.

-----

- **How to import the `rsa-wasm` to my project without installing the npm package?**

You can build your own distribution, see the "Development" section bellow.

-----

- **Base64 data conversion tips**

It's recommended to use some packages like [base64-js](https://www.npmjs.com/package/base64-js)
or your own implementation to handle the conversion if you want to encode and decode
the `Uint8Array` binary bytes data. The browser `atob` and `btoa` functions are only
works for string data.

Or you can convert the binary bytes data to hex string first, and then use the `atob` or `btoa`
to do the next stuff.


## Development

### Prerequesite

Install the [Rust](https://rustup.rs/).

Install the `wasm-pack` to build the rust codes into WebAssembly:

```shell
cargo install wasm-pack
```

Install `wasm-bindgen` and `wasm-opt` for building procedure speedup:

```shell
cargo install wasm-bindgen-cli wasm-opt
```

### Build

```shell
wasm-pack build --target web --release
```

Once the compile work is done, all npm package files will be generated and
placed in the `pkg` directory.
You can [publish](https://docs.npmjs.com/cli/v11/commands/npm-publish) your own package,
or just import the module directly:

```typescript
import __wbg_init, { encrypt, decrypt } from '/path/to/rsa_wasm.js'

// Initialize the wasm module
__wbg_init().then(() => {
  // Do encrypt or decrypt staffs...
})
```

## License

The MIT license
