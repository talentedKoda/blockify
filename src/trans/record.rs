use serde::{Deserialize, Serialize};

use crate::{
    data::Metadata, AuthKeyPair, DigitalSignature, Hash, KeyPairAlgorithm, PublicKey, SigningError,
    VerificationError,
};

pub use record_derive::Record;

/// The `Record` trait provides a structure and functions for securely and transparently storing data on the blockchain.
///  
/// Any type that needs security provided by cryptographic operations or blockchain technology can implement this trait.
///
/// `Record` contains methods for `signing`, `hashing`, `signature verification` and `recording` of records.
///
/// # Examples
///
/// ```
/// use blockify::record::Record;
/// use serde::{Serialize, Deserialize};
///
/// #[derive(Clone, Serialize, Deserialize, Record)]
/// struct Vote {
///     session: i32,
///     choice: i32,
/// }
///
/// // Generate an `ed25519` key pair
/// let keypair = blockify::generate_ed25519_key_pair();
///
/// // Create a `Vote` instance
/// let my_record = Vote { session: 0, choice: 2 };
///
/// // Sign `my_record` and obtain a `DigitalSignature`
/// let signature = my_record.sign(&keypair).unwrap();
///
/// // Verify the signature with the trait method `verify`
/// assert!(my_record.verify(&signature, &keypair.into_public_key()).is_ok())
/// ```
pub trait Record: Sized {
    /// Signs the record with the given key and returns the signature, if the signing succeeds
    ///
    /// # Arguments
    ///
    /// * `AuthKeyPair` - The private key to use for signing.
    ///
    /// # Returns
    ///
    /// * `Ok(DigitalSignature)`
    /// * `Err(SigningError)`
    fn sign(&self, keypair: &AuthKeyPair) -> Result<DigitalSignature, SigningError>;

    /// Attempts to verify the `DigitalSignature` for `self` with the given `PublicKey`
    ///
    /// # Arguments
    ///
    /// * `DigitalSignature`
    /// * `PublicKey`
    ///
    /// # Returns
    ///
    /// * `Ok(())`
    /// * `Err(VerificationError)`
    fn verify(
        &self,
        signature: &DigitalSignature,
        pubkey: &PublicKey,
    ) -> Result<(), VerificationError>;

    /// Attempts to convert the given record into a `SignedRecord` instance by singing it with an `AuthKeyPair`.
    ///
    /// This function accepts a `MetaData` type which may be empty (i.e `MetaData::empty()`).
    ///
    /// # Returns
    ///
    /// - `Ok(SignedRecord<T>)`
    /// - `Err(SigningError)`
    ///
    fn record(
        self,
        keypair: AuthKeyPair,
        metadata: Metadata,
    ) -> Result<SignedRecord<Self>, SigningError>;
    /// Computes and returns the hash of the record.
    ///
    /// Implementations of this function `must not` fail.
    fn hash(&self) -> Hash;
}

// This macro is not exported in favor of the derive macro Record which is also in this module.
macro_rules! impl_record_for {
    ($type:ty) => {
        impl Record for $type {
            fn sign(
                &self,
                key: &crate::AuthKeyPair,
            ) -> Result<crate::DigitalSignature, crate::SigningError> {
                let msg = crate::serialize(self)?;
                let signature = crate::sign_msg(&msg, key)?;
                Ok(signature)
            }

            fn verify(
                &self,
                signature: &crate::DigitalSignature,
                key: &crate::PublicKey,
            ) -> Result<(), crate::VerificationError> {
                let msg =
                    crate::serialize(self).map_err(|_| crate::VerificationError::SerdeError)?;
                key.verify(&msg, signature)
            }

            fn record(
                self,
                keypair: crate::AuthKeyPair,
                metadata: crate::data::Metadata,
            ) -> Result<crate::record::SignedRecord<Self>, crate::SigningError> {
                let signature = self.sign(&keypair)?;
                let hash = self.hash();
                Ok(crate::record::SignedRecord::new(
                    self,
                    signature,
                    keypair.into_public_key(),
                    hash,
                    metadata,
                ))
            }

            fn hash(&self) -> crate::Hash {
                crate::hash(self)
            }
        }
    };
}

impl_record_for!(String);
impl_record_for!(bool);
impl_record_for!(i64);
impl_record_for!(Box<[u8]>);

/// A `SignedRecord` is a type of data that can be added to a `block`.
/// 
/// 
/// `SignedRecord` is producible from any type that implements `Record` and internally consists of:
/// - the `digital signature` on the record
/// - the `public key` of the signer of the record
/// - the `algorithm` of the keypair used by the signer
/// - the `hash` of the record
/// - any associated `metadata`
///  
///
/// It can be used to ensure that data in the block is authentic and has not been tampered with.
///
///
/// # Type Parameters
///
/// - `R`: The type of the original record that was signed.
///
///
/// # Examples
///
/// ```
/// use blockify::{data::Metadata, record::Record};
/// use serde::{Deserialize, Serialize};
///
/// fn main() {
///    #[derive(Clone, Serialize, Deserialize, Record)]
///    struct Vote {
///        session: i32,
///        choice: i32,
///    }
///
///    // Generate a new keypair
///    let keypair = blockify::generate_ed25519_key_pair();
///
///    // Clone the public key
///    let pub_key = keypair.clone().into_public_key();
///
///    // Create a new `Vote` instance
///    let my_record = Vote {
///        session: 0,
///        choice: 2,
///    };
///
///    // calculate the hash of my_record
///    let my_record_hash = blockify::hash(&my_record);
///
///    // sign my_record with the AuthKeyPair instance and obtain a digital signature
///    let signature = my_record.sign(&keypair).unwrap();
///
///    // verify the authencity of the digital signature
///    assert!(my_record.verify(&signature, &pub_key).is_ok());
///
///    // record the my_vote (convert it into a SignedRecord instance)
///    let signed_record = my_record.record(keypair, Metadata::empty()).unwrap();
///
///    // Compare the signature of `my_record` with that inside the `SignedRecord` instance
///    assert_eq!(&signature, signed_record.signature());
///
///    // Compare the public key used to sign my_record with that inside the `SignedRecord` instance.
///    assert_eq!(&pub_key, signed_record.signer());
///
///    // Compare the hash of my_record with that inside the `SignedRecord` instance.
///    assert_eq!(&my_record_hash, signed_record.hash());
///
///    // Verify the validity of the signature within the `SignedRecord` instance.
///    assert!(signed_record.verify().is_ok());
///}
/// ```
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct SignedRecord<R> {
    signer: PublicKey,
    signature: DigitalSignature,

    hash: Hash,
    record: R,
    metadata: Metadata,
}

impl<R> SignedRecord<R> {
    /// Creates and returns a new `SignedRecord` instance with the given values.
    pub fn new(
        record: R,
        signature: DigitalSignature,
        signer: PublicKey,
        hash: Hash,
        metadata: Metadata,
    ) -> Self {
        Self {
            record,
            signature,
            hash,
            signer,
            metadata,
        }
    }

    /// Returns a reference to the `DigitalSignature` on this `SignedRecord` instance
    pub fn signature(&self) -> &DigitalSignature {
        &self.signature
    }
    /// Returns a reference to the `Record` inside this `SignedRecord` instance
    pub fn record(&self) -> &R {
        &self.record
    }
    /// Returns a reference to the public key used to sign this `SignedRecord` instance
    pub fn signer(&self) -> &PublicKey {
        &self.signer
    }
    /// Returns a reference to the keypair algorithm used to sign this `SignedRecord` instance
    pub fn keypair_algorithm(&self) -> KeyPairAlgorithm {
        self.signer.algorithm()
    }

    // Returns a reference to the hash of `Record` stored within this `SignedRecord` instance
    pub fn hash(&self) -> &Hash {
        &self.hash
    }

    // Returns a reference to the `Metadata` associated with this `SignedRecord` instance
    pub fn metadata(&self) -> &Metadata {
        &self.metadata
    }
}

impl<R: Record + Serialize> SignedRecord<R> {
    /// Verifies the validity of the `DigitalSignature` within this `SignedRecord` instance for the `Record` it holds.
    pub fn verify(&self) -> Result<(), VerificationError> {
        self.record.verify(self.signature(), self.signer())
    }
}
