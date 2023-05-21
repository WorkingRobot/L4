use std::ops::{Deref, DerefMut};

use serde::{Deserialize, Serialize};
use windows::{
    Win32::Foundation::FALSE,
    Win32::{
        Foundation::{GetLastError, HLOCAL},
        Security::Cryptography::{CryptProtectData, CryptUnprotectData, CRYPT_INTEGER_BLOB},
        System::Memory::LocalFree,
    },
};

pub struct Encrypted<T: Serialize + for<'de> Deserialize<'de>>(pub T);

impl<T: Serialize + for<'de> Deserialize<'de> + Default> Default for Encrypted<T> {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<T: Serialize + for<'de> Deserialize<'de>> Deref for Encrypted<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Serialize + for<'de> Deserialize<'de>> DerefMut for Encrypted<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

unsafe fn protect<E: serde::ser::Error>(plaintext: &mut [u8]) -> Result<Vec<u8>, E> {
    let plaintext_blob = CRYPT_INTEGER_BLOB {
        cbData: plaintext.len() as u32,
        pbData: plaintext.as_mut_ptr(),
    };
    let mut ciphertext_blob: CRYPT_INTEGER_BLOB = Default::default();
    if CryptProtectData(
        &plaintext_blob,
        None,
        None,
        None,
        None,
        0,
        &mut ciphertext_blob,
    ) == FALSE
    {
        let s = E::custom(format!(
            "CryptProtectData returned GLE {}",
            GetLastError().0
        ));
        return Err(s);
    }
    let ciphertext =
        std::slice::from_raw_parts(ciphertext_blob.pbData, ciphertext_blob.cbData as usize);
    let ciphertext = ciphertext.to_vec();
    _ = LocalFree(HLOCAL(ciphertext_blob.pbData as isize));
    Ok(ciphertext)
}

unsafe fn unprotect<E: serde::de::Error>(ciphertext: &mut [u8]) -> Result<Vec<u8>, E> {
    let ciphertext_blob = CRYPT_INTEGER_BLOB {
        cbData: ciphertext.len() as u32,
        pbData: ciphertext.as_mut_ptr(),
    };
    let mut plaintext_blob: CRYPT_INTEGER_BLOB = Default::default();
    if CryptUnprotectData(
        &ciphertext_blob,
        None,
        None,
        None,
        None,
        0,
        &mut plaintext_blob,
    ) == FALSE
    {
        let s = E::custom(format!(
            "CryptUnprotectData returned GLE {}",
            GetLastError().0
        ));
        return Err(s);
    }
    let plaintext =
        std::slice::from_raw_parts(plaintext_blob.pbData, plaintext_blob.cbData as usize);
    let plaintext = plaintext.to_vec();
    _ = LocalFree(HLOCAL(plaintext_blob.pbData as isize));
    Ok(plaintext)
}

impl<T: Serialize + for<'de> Deserialize<'de>> Serialize for Encrypted<T> {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut plaintext = rmp_serde::to_vec(&self.0).map_err(serde::ser::Error::custom)?;
        let ciphertext = unsafe { protect(&mut plaintext) }?;
        serializer.serialize_bytes(&ciphertext)
    }
}

impl<'d, T: Serialize + for<'de> Deserialize<'de>> Deserialize<'d> for Encrypted<T> {
    fn deserialize<D: serde::Deserializer<'d>>(deserializer: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Vec<u8>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("byte buffer")
            }

            fn visit_byte_buf<E: serde::de::Error>(self, v: Vec<u8>) -> Result<Self::Value, E> {
                Ok(v)
            }
        }

        let mut ciphertext = deserializer.deserialize_bytes(Visitor)?;
        let plaintext = unsafe { unprotect(&mut ciphertext) }?;
        rmp_serde::from_slice(&plaintext).map_err(serde::de::Error::custom)
    }
}
