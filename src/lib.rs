//! Age-encryption impl from rage-wasm

#![feature(vec_into_raw_parts)]
use age::{
    armor::{ArmoredReader, ArmoredWriter, Format},
    x25519, Decryptor, Encryptor,
};
use secrecy::{ExposeSecret, Secret};
use std::mem;
use std::os::raw::c_char;
use std::slice;
use std::{
    ffi::{CStr, CString},
    os::raw::c_int,
};
use std::{
    io::{Read, Write},
    iter,
};

#[no_mangle]
pub extern "C" fn rust_age_encryption_free_str(s: *mut c_char) {
    if !s.is_null() {
        unsafe {
            let _ = CString::from_raw(s);
        }
    }
}

#[no_mangle]
pub extern "C" fn rust_age_encryption_free_vec(s: *mut c_char, len: c_int) {
    if !s.is_null() {
        let len = len as usize;
        unsafe {
            Vec::from_raw_parts(s as *mut u8, len, len);
        }
    }
}

#[no_mangle]
pub extern "C" fn rust_age_encryption_keygen(
    secret_key_p: *mut *mut c_char,
    public_key_p: *mut *mut c_char,
) {
    let secret = x25519::Identity::generate();
    let public = secret.to_public();
    unsafe {
        *secret_key_p = CString::new(secret.to_string().expose_secret().as_bytes())
            .unwrap()
            .into_raw();
        *public_key_p = CString::new(public.to_string()).unwrap().into_raw();
    }
}

// use rust_age_encryption_free_vec to free
#[no_mangle]
pub extern "C" fn rust_age_encryption_to_raw_x25519_key(
    secret_key: *const c_char,
    raw_secret_key_p: *mut *mut c_char,
) -> c_int {
    use x25519_dalek::StaticSecret;

    let identity: x25519::Identity = match unsafe { CStr::from_ptr(secret_key) }
        .to_str()
        .ok()
        .and_then(|k| k.parse().ok())
    {
        Some(identity) => identity,
        _ => return -1,
    };

    let secret: &StaticSecret = unsafe { mem::transmute(&identity) };
    let secret_raw: [u8; 32] = secret.to_bytes();

    let mut output_secret = secret_raw.to_vec();
    output_secret.shrink_to_fit();
    unsafe {
        let (raw, len, _cap) = output_secret.into_raw_parts();
        *raw_secret_key_p = raw as *mut c_char;
        len as c_int
    }
}

// 0: ok
// positive: n-byte written
// negative: error
#[no_mangle]
pub extern "C" fn rust_age_encrypt_with_x25519(
    public_key: *const c_char,
    data: *const c_char,
    len: c_int,
    armor: c_char,
    output_p: *mut *mut c_char,
) -> c_int {
    let key: x25519::Recipient = match unsafe { CStr::from_ptr(public_key) }
        .to_str()
        .ok()
        .and_then(|k| k.parse().ok())
    {
        Some(key) => key,
        _ => return -1,
    };

    let recipients = vec![Box::new(key) as Box<dyn age::Recipient>];
    let encryptor = Encryptor::with_recipients(recipients);
    let mut output = vec![];
    let format = if armor != 0 {
        Format::AsciiArmor
    } else {
        Format::Binary
    };
    let armor =
        ArmoredWriter::wrap_output(&mut output, format).expect("won't fail except for OOM; qed");
    let mut writer = match encryptor.wrap_output(armor) {
        Ok(writer) => writer,
        _ => return -1,
    };
    let buf = unsafe { slice::from_raw_parts(data as *const u8, len as usize) };
    match writer.write_all(buf) {
        Ok(_) => (),
        _ => return -1,
    }
    match writer.finish().and_then(|armor| armor.finish()) {
        Ok(_) => {
            output.shrink_to_fit();
            unsafe {
                let (raw, len, _cap) = output.into_raw_parts();
                *output_p = raw as *mut c_char;
                len as c_int
            }
        }
        _ => -1,
    }
}

#[no_mangle]
pub extern "C" fn rust_age_decrypt_with_x25519(
    secret_key: *const c_char,
    data: *const c_char,
    len: c_int,
    output_p: *mut *mut c_char,
) -> c_int {
    let identity: x25519::Identity = match unsafe { CStr::from_ptr(secret_key) }
        .to_str()
        .ok()
        .and_then(|k| k.parse().ok())
    {
        Some(identity) => identity,
        _ => return -1,
    };
    let data = unsafe { slice::from_raw_parts(data as *const u8, len as usize) };
    let armor = ArmoredReader::new(data);
    let decryptor = match Decryptor::new(armor) {
        Ok(Decryptor::Recipients(d)) => d,
        _ => {
            return -1;
        }
    };
    let mut decrypted = vec![];
    let mut reader = match decryptor.decrypt(iter::once(&identity as &dyn age::Identity)) {
        Ok(reader) => reader,
        _ => return -1,
    };
    match reader.read_to_end(&mut decrypted) {
        Ok(_) => {
            decrypted.shrink_to_fit();
            unsafe {
                let (raw, len, _cap) = decrypted.into_raw_parts();
                *output_p = raw as *mut c_char;
                len as c_int
            }
        }
        _ => return -1,
    }
}

#[no_mangle]
pub extern "C" fn rust_age_encrypt_with_user_passphrase(
    passphrase: *const c_char,
    data: *const c_char,
    len: c_int,
    armor: c_char,
    output_p: *mut *mut c_char,
) -> c_int {
    let encryptor = match unsafe { CStr::from_ptr(passphrase) }
        .to_str()
        .map(|p| Encryptor::with_user_passphrase(Secret::new(p.to_owned())))
    {
        Ok(encryptor) => encryptor,
        _ => return -1,
    };
    let mut output = vec![];
    let format = if armor != 0 {
        Format::AsciiArmor
    } else {
        Format::Binary
    };
    let armor =
        ArmoredWriter::wrap_output(&mut output, format).expect("won't fail except for OOM; qed");
    let mut writer = match encryptor.wrap_output(armor) {
        Ok(writer) => writer,
        _ => return -1,
    };
    let data = unsafe { slice::from_raw_parts(data as *const u8, len as usize) };
    match writer.write_all(data) {
        Ok(_) => (),
        _ => return -1,
    };
    match writer.finish().and_then(|armor| armor.finish()) {
        Ok(_) => {
            output.shrink_to_fit();
            unsafe {
                let (raw, len, _cap) = output.into_raw_parts();
                *output_p = raw as *mut c_char;
                len as c_int
            }
        }
        _ => return -1,
    }
}

#[no_mangle]
pub extern "C" fn rust_age_decrypt_with_user_passphrase(
    passphrase: *const c_char,
    data: *const c_char,
    len: c_int,
    output_p: *mut *mut c_char,
) -> c_int {
    let data = unsafe { slice::from_raw_parts(data as *const u8, len as usize) };
    let armor = ArmoredReader::new(data);
    let decryptor = match age::Decryptor::new(armor) {
        Ok(age::Decryptor::Passphrase(d)) => d,
        _ => return -1,
    };
    let mut decrypted = vec![];
    let passphrase = unsafe { CStr::from_ptr(passphrase).to_str().unwrap() };
    let mut reader = match decryptor.decrypt(&Secret::new(passphrase.to_owned()), None) {
        Ok(reader) => reader,
        _ => return -1,
    };

    match reader.read_to_end(&mut decrypted) {
        Ok(_) => {
            decrypted.shrink_to_fit();
            unsafe {
                let (raw, len, _cap) = decrypted.into_raw_parts();
                *output_p = raw as *mut c_char;
                len as c_int
            }
        }
        _ => return -1,
    }
}

#[cfg(test)]
mod tests {
    use std::ptr;

    use super::*;

    #[test]
    fn test_keygen() {
        let mut skey = ptr::null_mut();
        let mut pkey = ptr::null_mut();
        rust_age_encryption_keygen(&mut skey, &mut pkey);
        println!("=> {:?} {:?}", skey, pkey);
        unsafe {
            println!("=> skey {:?}", CStr::from_ptr(skey));
            println!("=> pkey {:?}", CStr::from_ptr(pkey));
        }
        assert!(!skey.is_null());
        assert!(!pkey.is_null());

        let mut output = ptr::null_mut();
        rust_age_encryption_to_raw_x25519_key(skey, &mut output);
        assert!(!output.is_null());

        rust_age_encryption_free_str(skey);
        rust_age_encryption_free_str(pkey);
    }

    #[test]
    fn test_encryption_decryption() {
        let mut skey = ptr::null_mut();
        let mut pkey = ptr::null_mut();
        rust_age_encryption_keygen(&mut skey, &mut pkey);

        let plaintext = b"hello world of encryption";
        let mut cipherbuf = ptr::null_mut();
        let ret = rust_age_encrypt_with_x25519(
            pkey,
            plaintext.as_ptr() as _,
            plaintext.len() as _,
            1,
            &mut cipherbuf,
        );
        assert!(ret > 0);
        let ciphertext = unsafe { slice::from_raw_parts(cipherbuf as *const u8, ret as usize) };
        let ciphertext = ciphertext.to_vec();
        println!("=> {:?}", String::from_utf8(ciphertext.clone()));

        rust_age_encryption_free_vec(cipherbuf, ret);

        let mut plainbuf = ptr::null_mut();
        let ret = rust_age_decrypt_with_x25519(
            skey,
            ciphertext.as_ptr() as _,
            ciphertext.len() as _,
            &mut plainbuf,
        );

        let plaintext = unsafe { slice::from_raw_parts(plainbuf as *const u8, ret as usize) };
        let plaintext = plaintext.to_vec();

        assert_eq!(
            String::from_utf8(plaintext).unwrap(),
            "hello world of encryption"
        );

        rust_age_encryption_free_vec(plainbuf, ret);

        rust_age_encryption_free_str(skey);
        rust_age_encryption_free_str(pkey);
    }

    #[test]
    fn test_encryption_decryption_with_passphrease() {
        let plaintext = b"hello world of encryption using passphrase";
        let passphrase = b"a secret key";

        let mut buf = ptr::null_mut();

        let ret = rust_age_encrypt_with_user_passphrase(
            passphrase.as_ptr() as _,
            plaintext.as_ptr() as _,
            plaintext.len() as _,
            1,
            &mut buf,
        );
        let ciphertext = unsafe { slice::from_raw_parts(buf as *const u8, ret as usize) };
        let ciphertext = ciphertext.to_vec();
        println!("=> {:?}", String::from_utf8(ciphertext.clone()));

        rust_age_encryption_free_vec(buf, ret);

        let ret = rust_age_decrypt_with_user_passphrase(
            passphrase.as_ptr() as _,
            ciphertext.as_ptr() as _,
            ciphertext.len() as _,
            &mut buf,
        );
        let plaintext = unsafe { slice::from_raw_parts(buf as *const u8, ret as usize) };
        assert_eq!(
            String::from_utf8(plaintext.to_vec()).unwrap(),
            "hello world of encryption using passphrase"
        );
        rust_age_encryption_free_vec(buf, ret);
    }
}
