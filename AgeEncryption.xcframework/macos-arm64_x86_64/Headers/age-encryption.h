#ifndef AGE_ENCRYPTION_H
#define AGE_ENCRYPTION_H

#include <stdint.h>

extern "C" {

void rust_age_encryption_free_str(char *);
void rust_age_encryption_free_vec(char *buf, int len);

void rust_age_encryption_keygen(char **secret_key, char **public_key);

int rust_age_encrypt_with_x25519(const char *public_key, const char *plaintext,
                                 int len, bool armor, char **ciphertext);
int rust_age_decrypt_with_x25519(const char *secret_key, const char *ciphertext,
                                 int len, char **plaintext);

int rust_age_encrypt_with_user_passphrase(const char *passphrase,
                                          const char *plaintext, int len,
                                          bool armor, char **ciphertext);
int rust_age_decrypt_with_user_passphrase(const char *passphrase,
                                          const char *ciphertext, int len,
                                          char **plaintext);
}
#endif
