use pyo3::exceptions::*;
use pyo3::prelude::*;
use pyo3::types::PyBytes;
use rand::Rng;
use sha2::Digest;

use crate::util::{Encoding, HandleError};

#[pyclass]
pub struct PublicKey(pub ed25519_dalek::PublicKey);

#[pymethods]
impl PublicKey {
    #[staticmethod]
    pub fn from_int(int: num_bigint::BigUint) -> PyResult<Self> {
        let bytes = int.to_bytes_be();
        if bytes.len() > 32 {
            return Err(PyValueError::new_err("Number is too big"));
        }

        let mut pubkey = [0u8; 32];
        pubkey[(32 - bytes.len())..].copy_from_slice(&bytes);

        ed25519_dalek::PublicKey::from_bytes(&pubkey)
            .handle_value_error()
            .map(Self)
    }

    #[staticmethod]
    pub fn from_bytes(bytes: &[u8]) -> PyResult<Self> {
        ed25519_dalek::PublicKey::from_bytes(bytes)
            .handle_value_error()
            .map(Self)
    }

    #[new]
    pub fn new(value: &str, encoding: Option<&str>) -> PyResult<Self> {
        let encoding = Encoding::from_optional_param(encoding, Encoding::Hex)?;
        encoding.decode_pubkey(value).map(Self)
    }

    pub fn check_signature(
        &self,
        data: &[u8],
        signature: &Signature,
        signature_id: Option<i32>,
    ) -> bool {
        use ed25519_dalek::Verifier;
        let data = ton_abi::extend_signature_with_id(data, signature_id);
        self.0.verify(&data, &signature.0).is_ok()
    }

    pub fn encode(&self, encoding: Option<&str>) -> PyResult<String> {
        let encoding = Encoding::from_optional_param(encoding, Encoding::Hex)?;
        Ok(encoding.encode_pubkey(&self.0))
    }

    pub fn to_bytes<'a>(&self, py: Python<'a>) -> &'a PyBytes {
        PyBytes::new(py, self.0.as_bytes())
    }

    pub fn to_int(&self) -> num_bigint::BigUint {
        num_bigint::BigUint::from_bytes_be(self.0.as_bytes())
    }

    fn __str__(&self) -> String {
        hex::encode(self.0.as_bytes())
    }

    fn __repr__(&self) -> String {
        format!("PublicKey('{}')", hex::encode(self.0.as_bytes()))
    }

    fn __hash__(&self) -> u64 {
        u64::from_le_bytes(self.0.as_bytes()[..8].try_into().unwrap())
    }

    fn __richcmp__(&self, other: &Self, op: pyo3::basic::CompareOp) -> bool {
        op.matches(self.0.as_bytes().cmp(other.0.as_bytes()))
    }
}

#[pyclass]
pub struct KeyPair(pub ed25519_dalek::Keypair);

#[pymethods]
impl KeyPair {
    #[staticmethod]
    fn generate() -> Self {
        Self(ed25519_dalek::Keypair::generate(&mut rand::thread_rng()))
    }

    #[new]
    fn new(secret: &[u8]) -> PyResult<Self> {
        let secret = ed25519_dalek::SecretKey::from_bytes(secret).handle_value_error()?;
        let public = ed25519_dalek::PublicKey::from(&secret);
        Ok(Self(ed25519_dalek::Keypair { secret, public }))
    }

    #[getter]
    fn secret_key<'a>(&self, py: Python<'a>) -> &'a PyBytes {
        PyBytes::new(py, self.0.secret.as_bytes())
    }

    #[getter]
    fn public_key(&self) -> PublicKey {
        PublicKey(self.0.public)
    }

    pub fn sign(&self, data: &[u8], signature_id: Option<i32>) -> Signature {
        use ed25519_dalek::Signer;
        use sha2::Digest;

        let data = sha2::Sha256::digest(data);
        let data = ton_abi::extend_signature_with_id(&data, signature_id);
        Signature(self.0.sign(&data))
    }

    pub fn sign_raw(&self, data: &[u8], signature_id: Option<i32>) -> Signature {
        use ed25519_dalek::Signer;

        let data = ton_abi::extend_signature_with_id(data, signature_id);
        Signature(self.0.sign(&data))
    }

    pub fn check_signature(
        &self,
        data: &[u8],
        signature: &Signature,
        signature_id: Option<i32>,
    ) -> bool {
        use ed25519_dalek::Verifier;
        let data = ton_abi::extend_signature_with_id(data, signature_id);
        self.0.public.verify(&data, &signature.0).is_ok()
    }

    fn __hash__(&self) -> u64 {
        u64::from_le_bytes(self.0.public.as_bytes()[..8].try_into().unwrap())
    }

    fn __richcmp__(&self, other: &Self, op: pyo3::basic::CompareOp) -> bool {
        op.matches(self.0.public.as_bytes().cmp(other.0.public.as_bytes()))
    }
}

#[pyclass]
pub struct Signature(pub ed25519_dalek::Signature);

#[pymethods]
impl Signature {
    #[staticmethod]
    pub fn from_bytes(bytes: &[u8]) -> PyResult<Self> {
        ed25519_dalek::Signature::from_bytes(bytes)
            .handle_value_error()
            .map(Self)
    }

    #[new]
    pub fn new(value: &str, encoding: Option<&str>) -> PyResult<Self> {
        let encoding = Encoding::from_optional_param(encoding, Encoding::Hex)?;
        let bytes = encoding.decode_bytes(value)?;
        ed25519_dalek::Signature::from_bytes(&bytes)
            .handle_value_error()
            .map(Self)
    }

    pub fn encode(&self, encoding: Option<&str>) -> PyResult<String> {
        let encoding = Encoding::from_optional_param(encoding, Encoding::Hex)?;
        Ok(encoding.encode_bytes(self.0.as_ref()))
    }

    pub fn to_bytes<'a>(&self, py: Python<'a>) -> &'a PyBytes {
        PyBytes::new(py, self.0.as_ref())
    }

    fn __repr__(&self) -> String {
        use ed25519_dalek::ed25519::signature::Signature;
        format!("Signature('{}')", hex::encode(self.0.as_bytes()))
    }

    fn __hash__(&self) -> u64 {
        u64::from_le_bytes(self.0.as_ref()[..8].try_into().unwrap())
    }

    fn __richcmp__(&self, other: &Self, op: pyo3::basic::CompareOp) -> bool {
        op.matches(self.0.as_ref().cmp(other.0.as_ref()))
    }
}

#[pyclass(subclass)]
pub struct Seed(Vec<&'static str>);

#[pymethods]
impl Seed {
    #[getter]
    fn word_count(&self) -> usize {
        self.0.len()
    }

    fn __str__(&self) -> String {
        self.0.join(" ")
    }
}

#[pyclass(extends = Seed)]
pub struct LegacySeed;

impl LegacySeed {
    const WORD_COUNT: usize = 24;
}

#[pymethods]
impl LegacySeed {
    #[staticmethod]
    fn generate(py: Python<'_>) -> PyResult<Py<Self>> {
        let entropy: [u8; 32] = rand::thread_rng().gen();
        let class = PyClassInitializer::from(Seed(generate_words(&entropy))).add_subclass(Self);
        Py::new(py, class)
    }

    #[new]
    fn new(phrase: String) -> PyResult<PyClassInitializer<Self>> {
        let words = split_words(&phrase, Self::WORD_COUNT)?;
        Ok(PyClassInitializer::from(Seed(words)).add_subclass(Self))
    }

    fn derive(self_: PyRef<'_, Self>) -> PyResult<KeyPair> {
        use hmac::{Mac, NewMac};

        const PBKDF_ITERATIONS: u32 = 100_000;
        const SALT: &[u8] = b"TON default seed";

        let words = &self_.as_ref().0;
        if words.len() != Self::WORD_COUNT {
            return Err(PyRuntimeError::new_err("Invalid legacy seed"));
        }

        let phrase = words.join(" ");
        let password = hmac::Hmac::<sha2::Sha512>::new_from_slice(phrase.as_bytes())
            .unwrap()
            .finalize()
            .into_bytes();

        let mut res = [0; 512 / 8];
        pbkdf2::pbkdf2::<hmac::Hmac<sha2::Sha512>>(&password, SALT, PBKDF_ITERATIONS, &mut res);

        let secret = ed25519_dalek::SecretKey::from_bytes(&res[0..32]).unwrap();
        let public = ed25519_dalek::PublicKey::from(&secret);
        Ok(KeyPair(ed25519_dalek::Keypair { secret, public }))
    }

    fn __repr__(slf: PyRef<Self>) -> String {
        let base = slf.into_super();
        format!("LegacySeed('{}')", base.0.join(" "))
    }
}

#[pyclass(extends = Seed)]
pub struct Bip39Seed;

#[pymethods]
impl Bip39Seed {
    #[staticmethod]
    fn generate(py: Python<'_>) -> PyResult<Py<Self>> {
        let entropy: [u8; 16] = rand::thread_rng().gen();
        let class = PyClassInitializer::from(Seed(generate_words(&entropy))).add_subclass(Self);
        Py::new(py, class)
    }

    #[staticmethod]
    fn path_for_account(id: u16) -> String {
        format!("m/44'/396'/0'/0/{id}")
    }

    #[new]
    fn new(phrase: String) -> PyResult<PyClassInitializer<Self>> {
        bip39::Mnemonic::from_phrase(&phrase, LANGUAGE).handle_value_error()?;
        let words = split_words(&phrase, 12)?;
        Ok(PyClassInitializer::from(Seed(words)).add_subclass(Self))
    }

    fn derive(self_: PyRef<'_, Self>, path: Option<&str>) -> PyResult<KeyPair> {
        let words = &self_.as_ref().0;
        let phrase = words.join(" ");
        let mnemonic = bip39::Mnemonic::from_phrase(&phrase, LANGUAGE).handle_runtime_error()?;
        let hd = bip39::Seed::new(&mnemonic, "");
        let seed_bytes = hd.as_bytes();

        let path = path.unwrap_or("m/44'/396'/0'/0/0");
        let derived = tiny_hderive::bip32::ExtendedPrivKey::derive(seed_bytes, path)
            .map_err(|_| PyValueError::new_err("Invalid derivation path"))?;

        let secret = ed25519_dalek::SecretKey::from_bytes(&derived.secret()).unwrap();
        let public = ed25519_dalek::PublicKey::from(&secret);
        Ok(KeyPair(ed25519_dalek::Keypair { secret, public }))
    }

    fn __repr__(slf: PyRef<Self>) -> String {
        let base = slf.into_super();
        format!("Bip39Seed('{}')", base.0.join(" "))
    }
}

fn generate_words(entropy: &[u8]) -> Vec<&'static str> {
    use bip39::util::{Bits11, IterExt};

    let wordlist = LANGUAGE.wordlist();

    let checksum_byte = sha2::Sha256::digest(entropy)[0];

    entropy
        .iter()
        .chain(Some(&checksum_byte))
        .bits()
        .map(|bits: Bits11| wordlist.get_word(bits))
        .collect()
}

fn split_words(phrase: &str, len: usize) -> PyResult<Vec<&'static str>> {
    let wordmap = LANGUAGE.wordmap();
    let wordlist = LANGUAGE.wordlist();

    let mut result = Vec::with_capacity(len);

    let mut word_count = 0;
    for word in phrase.split_whitespace() {
        word_count += 1;
        if word_count > len {
            return Err(PyValueError::new_err(format!("Expected {len} words")));
        }

        let word = wordlist.get_word(wordmap.get_bits(word).handle_value_error()?);
        result.push(word);
    }

    if word_count == len {
        Ok(result)
    } else {
        Err(PyValueError::new_err(format!("Expected {len} words")))
    }
}

const LANGUAGE: bip39::Language = bip39::Language::English;
