use pyo3::exceptions::*;
use pyo3::prelude::*;
use rand::Rng;
use sha2::Digest;

use crate::util::{Encoding, HandleError};

#[pyclass]
pub struct PublicKey(pub ed25519_dalek::PublicKey);

#[pymethods]
impl PublicKey {
    #[new]
    fn new(value: &str, encoding: Option<&str>) -> PyResult<Self> {
        let encoding = Encoding::from_optional_param(encoding, Encoding::Hex)?;
        encoding.decode_pubkey(value).map(Self)
    }

    fn encode(&self, encoding: Option<&str>) -> PyResult<String> {
        let encoding = Encoding::from_optional_param(encoding, Encoding::Hex)?;
        Ok(encoding.encode_pubkey(&self.0))
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
}

#[pyclass(subclass)]
pub struct Seed(Vec<&'static str>);

#[pyclass(extends = Seed)]
pub struct LegacySeed;

#[pymethods]
impl LegacySeed {
    #[staticmethod]
    fn generate(py: Python<'_>) -> PyObject {
        let entropy: [u8; 32] = rand::thread_rng().gen();
        let res = PyClassInitializer::from(Seed(generate_words(&entropy))).add_subclass(Self);
        Py::new(py, res).unwrap().into_py(py)
    }

    #[new]
    fn new(phrase: String) -> PyResult<PyClassInitializer<Self>> {
        let words = split_words(&phrase, 24)?;
        Ok(PyClassInitializer::from(Seed(words)).add_subclass(Self))
    }
}

#[pyclass(extends = Seed)]
pub struct Bip39Seed;

#[pymethods]
impl Bip39Seed {
    #[staticmethod]
    fn generate(py: Python<'_>) -> PyObject {
        let entropy: [u8; 32] = rand::thread_rng().gen();
        let res = PyClassInitializer::from(Seed(generate_words(&entropy))).add_subclass(Self);
        Py::new(py, res).unwrap().into_py(py)
    }

    #[new]
    fn new(phrase: String) -> PyResult<PyClassInitializer<Self>> {
        bip39::Mnemonic::from_phrase(&phrase, LANGUAGE).handle_value_error()?;
        let words = split_words(&phrase, 12)?;
        Ok(PyClassInitializer::from(Seed(words)).add_subclass(Self))
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
