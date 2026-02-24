// Node.js script to encrypt/sign .fdom sites
const fs = require('fs');
const { ChaCha20Poly1305 } = require('chacha20poly1305');

const key = Buffer.alloc(32, 1); // placeholder key

function encrypt(fileIn, fileOut) {
    let data = fs.readFileSync(fileIn);
    const cipher = new ChaCha20Poly1305(key);
    const nonce = Buffer.alloc(12, 0);
    const encrypted = Buffer.concat([nonce, cipher.encrypt(nonce, data)]);
    fs.writeFileSync(fileOut, encrypted);
}

encrypt(process.argv[2], process.argv[3]);
console.log('Site packaged.');