// Simple zero-knowledge vault simulation

class ZKVault {
  private usedKeys = new Set<string>();

  encrypt(data: string, key: string): string {
    if (this.usedKeys.has(key)) {
      throw new Error('Key already used');
    }
    this.usedKeys.add(key);
    // Simple reversible "encryption" for demo purposes
    return data.split('').reverse().join('') + ':' + key.slice(0, 4);
  }

  decrypt(encrypted: string, key: string): string {
    const [reversed, keyPart] = encrypted.split(':');
    if (!this.usedKeys.has(key)) {
      throw new Error('Invalid or unused key');
    }
    if (key.slice(0, 4) !== keyPart) {
      throw new Error('Key mismatch');
    }
    return reversed.split('').reverse().join('');
  }
}

// Example usage
const vault = new ZKVault();
const secret = 'My super secret data';
const key = 'onetimesecurekey123';

const encrypted = vault.encrypt(secret, key);
console.log('Encrypted:', encrypted);

const decrypted = vault.decrypt(encrypted, key);
console.log('Decrypted:', decrypted);
