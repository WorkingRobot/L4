use serde::{
    de::Error, de::SeqAccess, de::Visitor, Deserialize, Deserializer, Serialize, Serializer,
};
use std::ops::{Deref, DerefMut};
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
    if ciphertext.is_empty() {
        return Ok([].into());
    }
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
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut plaintext = rmp_serde::to_vec_named(&self.0).map_err(serde::ser::Error::custom)?;
        let ciphertext = unsafe { protect(&mut plaintext) }?;
        serializer.serialize_bytes(&ciphertext)
    }
}

impl<'d, T: Serialize + for<'de> Deserialize<'de>> Deserialize<'d> for Encrypted<T> {
    fn deserialize<D: Deserializer<'d>>(deserializer: D) -> Result<Self, D::Error> {
        struct BufVisitor;
        impl<'de> Visitor<'de> for BufVisitor {
            type Value = Vec<u8>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("encrypted byte buffer")
            }

            fn visit_seq<V: SeqAccess<'de>>(self, mut visitor: V) -> Result<Vec<u8>, V::Error> {
                let len = std::cmp::min(visitor.size_hint().unwrap_or(0), 4096);
                let mut bytes = Vec::with_capacity(len);

                while let Some(b) = visitor.next_element()? {
                    bytes.push(b);
                }

                Ok(bytes)
            }

            fn visit_bytes<E: Error>(self, v: &[u8]) -> Result<Vec<u8>, E> {
                Ok(v.to_vec())
            }

            fn visit_byte_buf<E: Error>(self, v: Vec<u8>) -> Result<Vec<u8>, E> {
                Ok(v)
            }

            fn visit_str<E: Error>(self, v: &str) -> Result<Vec<u8>, E> {
                Ok(v.into())
            }

            fn visit_string<E: Error>(self, v: String) -> Result<Vec<u8>, E> {
                Ok(v.into())
            }
        }

        let mut ciphertext = deserializer.deserialize_byte_buf(BufVisitor)?;
        let plaintext = unsafe { unprotect(&mut ciphertext) }?;
        rmp_serde::from_slice::<T>(&plaintext)
            .map(|d| Encrypted(d))
            .map_err(Error::custom)
    }
}

#[test]
fn dpapi_serde() {
    let val = vec![Encrypted("Hello World".to_string())];

    let ser_val = rmp_serde::to_vec_named(&val);
    assert!(ser_val.is_ok());
    let ser_val = ser_val.unwrap();

    let de_val_generic = rmp_serde::from_slice::<rmpv::Value>(&ser_val);
    assert!(de_val_generic.is_ok());
    let de_val_generic = de_val_generic.unwrap();

    println!("Deserialized: {de_val_generic:#?}");

    let de_val = rmp_serde::from_slice::<Vec<Encrypted<String>>>(&ser_val);
    assert!(de_val.is_ok());
    let de_val = de_val.unwrap();

    assert_eq!(val[0].0, de_val[0].0);
}
