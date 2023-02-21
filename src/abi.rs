use std::collections::BTreeMap;
use std::sync::Arc;

use pyo3::exceptions::*;
use pyo3::prelude::*;
use pyo3::types::PyDict;

use crate::cell::Cell;
use crate::subscription::Address;
use crate::util::*;

#[derive(Clone)]
#[pyclass]
pub struct ContractAbi(Arc<SharedContractAbi>);

#[pymethods]
impl ContractAbi {
    #[new]
    fn new(abi: &str) -> PyResult<Self> {
        let contract = ton_abi::Contract::load(abi.trim()).handle_value_error()?;

        let functions = contract
            .functions
            .iter()
            .map(|(name, abi)| (name.clone(), FunctionAbi(Arc::new(abi.clone()))))
            .collect();

        let events = contract
            .events
            .iter()
            .map(|(name, abi)| (name.clone(), EventAbi(Arc::new(abi.clone()))))
            .collect();

        let shared = Arc::new(SharedContractAbi {
            contract,
            functions,
            events,
        });

        Ok(Self(shared))
    }

    #[getter]
    fn abi_version(&self) -> AbiVersion {
        AbiVersion(self.0.contract.abi_version)
    }

    fn get_function(&self, name: &str) -> Option<FunctionAbi> {
        self.0.functions.get(name).cloned()
    }

    fn get_event(&self, name: &str) -> Option<EventAbi> {
        self.0.events.get(name).cloned()
    }
}

struct SharedContractAbi {
    contract: ton_abi::Contract,
    functions: FastHashMap<String, FunctionAbi>,
    events: FastHashMap<String, EventAbi>,
}

#[derive(Clone)]
#[pyclass]
pub struct FunctionAbi(Arc<ton_abi::Function>);

#[pymethods]
impl FunctionAbi {
    #[getter]
    fn abi_version(&self) -> AbiVersion {
        AbiVersion(self.0.abi_version)
    }

    fn encode_internal_input(&self, data: &PyDict) -> PyResult<Cell> {
        let tokens = parse_tokens(&self.0.inputs, data)?;
        let input = self
            .0
            .encode_internal_input(&tokens)
            .handle_runtime_error()?;
        input.into_cell().map(Cell).handle_runtime_error()
    }

    fn decode_input<'a>(
        &self,
        py: Python<'a>,
        message_body: &Cell,
        internal: bool,
        allow_partial: Option<bool>,
    ) -> PyResult<&'a PyDict> {
        let abi = self.0.as_ref();
        let body = message_body.0.clone().into();
        let values = if matches!(allow_partial, Some(true)) {
            abi.decode_input_partial(body, internal)
        } else {
            abi.decode_input(body, internal)
        }
        .handle_runtime_error()?;

        convert_tokens(py, values)
    }

    fn decode_output<'a>(
        &self,
        py: Python<'a>,
        message_body: &Cell,
        allow_partial: Option<bool>,
    ) -> PyResult<&'a PyDict> {
        let abi = self.0.as_ref();
        let body = message_body.0.clone().into();
        let values = if matches!(allow_partial, Some(true)) {
            abi.decode_output_partial(body, false)
        } else {
            abi.decode_output(body, false)
        }
        .handle_runtime_error()?;

        convert_tokens(py, values)
    }
}

#[derive(Clone)]
#[pyclass]
pub struct EventAbi(Arc<ton_abi::Event>);

#[pymethods]
impl EventAbi {
    #[getter]
    fn abi_version(&self) -> AbiVersion {
        AbiVersion(self.0.abi_version)
    }

    fn decode_message_body<'a>(&self, py: Python<'a>, message_body: &Cell) -> PyResult<&'a PyDict> {
        let values = self
            .0
            .decode_input(message_body.0.clone().into())
            .handle_runtime_error()?;
        convert_tokens(py, values)
    }
}

#[derive(Clone)]
#[pyclass(subclass)]
pub struct AbiParam {
    pub param: ton_abi::ParamType,
}

macro_rules! define_abi_types {
    ($($ident:ident = |$($arg:ident: $arg_ty:ty),*| $res:expr),*$(,)?) => {$(
        #[pyclass(extends = AbiParam)]
        pub struct $ident;

        #[pymethods]
        impl $ident {
            #[new]
            fn new($($arg: $arg_ty),*) -> (Self, AbiParam) {
                let base = AbiParam {
                    param: $res,
                };
                (Self, base)
            }
        }
    )*};
}

define_abi_types! {
    AbiUint = |size: usize| ton_abi::ParamType::Uint(size),
    AbiInt = |size: usize| ton_abi::ParamType::Int(size),
    AbiVarUint = |size: usize| ton_abi::ParamType::VarUint(size),
    AbiVarInt = |size: usize| ton_abi::ParamType::VarInt(size),
    AbiBool = | | ton_abi::ParamType::Bool,
    AbiTuple = |items: Vec<(String, AbiParam)>| {
        ton_abi::ParamType::Tuple(
            items
                .into_iter()
                .map(|(name, AbiParam { param })| {
                    ton_abi::Param {
                        name,
                        kind: param,
                    }
                })
                .collect()
        )
    },
    AbiArray = |value_type: AbiParam| ton_abi::ParamType::Array(Box::new(value_type.param)),
    AbiFixedArray = |value_type: AbiParam, len: usize| {
        ton_abi::ParamType::FixedArray(Box::new(value_type.param), len)
    },
    AbiCell = | | ton_abi::ParamType::Cell,
    AbiMap = |key_type: AbiParam, value_type: AbiParam| {
        let key_type = Box::new(key_type.param);
        let value_type = Box::new(value_type.param);
        ton_abi::ParamType::Map(key_type, value_type)
    },
    AbiAddress = | | ton_abi::ParamType::Address,
    AbiBytes = | | ton_abi::ParamType::Bytes,
    AbiFixedBytes = |len: usize| ton_abi::ParamType::FixedBytes(len),
    AbiString = | | ton_abi::ParamType::String,
    AbiToken = | | ton_abi::ParamType::Token,
    AbiOptional = |value_type: AbiParam| {
        ton_abi::ParamType::Optional(Box::new(value_type.param))
    },
    AbiRef = |value_type: AbiParam| {
        ton_abi::ParamType::Ref(Box::new(value_type.param))
    },
}

#[derive(Copy, Clone)]
#[pyclass]
pub struct AbiVersion(pub ton_abi::contract::AbiVersion);

#[pymethods]
impl AbiVersion {
    #[new]
    fn new(major: u8, minor: u8) -> Self {
        Self(ton_abi::contract::AbiVersion { major, minor })
    }

    fn major(&self) -> u8 {
        self.0.major
    }

    fn minor(&self) -> u8 {
        self.0.minor
    }

    fn __str__(&self) -> String {
        self.0.to_string()
    }

    fn __richcmp__(&self, other: &Self, op: pyo3::basic::CompareOp) -> bool {
        op.matches((self.0.major, self.0.minor).cmp(&(other.0.major, other.0.minor)))
    }
}

pub fn parse_tokens(params: &[ton_abi::Param], value: &PyDict) -> PyResult<Vec<ton_abi::Token>> {
    let mut result = Vec::with_capacity(params.len());
    for param in params {
        let value = match value.get_item(param.name.as_str()) {
            Some(value) => parse_token(&param.kind, value)?,
            None => {
                return Err(PyRuntimeError::new_err(format!(
                    "Param '{}' not found",
                    param.name
                )));
            }
        };
        result.push(ton_abi::Token::new(&param.name, value));
    }
    Ok(result)
}

fn parse_token(param: &ton_abi::ParamType, value: &PyAny) -> PyResult<ton_abi::TokenValue> {
    use pyo3::types::*;

    Ok(match param {
        ton_abi::ParamType::Uint(size) => {
            let number = value.extract::<num_bigint::BigUint>()?;
            ton_abi::TokenValue::Uint(ton_abi::Uint {
                number,
                size: *size,
            })
        }
        ton_abi::ParamType::Int(size) => {
            let number = value.extract::<num_bigint::BigInt>()?;
            ton_abi::TokenValue::Int(ton_abi::Int {
                number,
                size: *size,
            })
        }
        ton_abi::ParamType::VarUint(size) => {
            let number = value.extract::<num_bigint::BigUint>()?;
            ton_abi::TokenValue::VarUint(*size, number)
        }
        ton_abi::ParamType::VarInt(size) => {
            let number = value.extract::<num_bigint::BigInt>()?;
            ton_abi::TokenValue::VarInt(*size, number)
        }
        ton_abi::ParamType::Bool => {
            let value = value.extract::<bool>()?;
            ton_abi::TokenValue::Bool(value)
        }
        ton_abi::ParamType::Tuple(types) => {
            let value = value.extract::<&PyDict>()?;
            ton_abi::TokenValue::Tuple(parse_tokens(types, value)?)
        }
        ton_abi::ParamType::Array(ty) => {
            let list = value.extract::<&PyList>()?;
            let mut values = Vec::with_capacity(list.len());
            for value in list {
                values.push(parse_token(ty.as_ref(), value)?);
            }
            ton_abi::TokenValue::Array(*ty.clone(), values)
        }
        ton_abi::ParamType::FixedArray(ty, len) => {
            let list = value.extract::<&PyList>()?;
            let list_len = list.len();
            if list_len != *len {
                return Err(PyValueError::new_err("Invalid fixed array length"));
            }
            let mut values = Vec::with_capacity(list_len);
            for value in list {
                values.push(parse_token(ty.as_ref(), value)?);
            }
            ton_abi::TokenValue::FixedArray(*ty.clone(), values)
        }
        ton_abi::ParamType::Cell => {
            let Cell(value) = value.extract::<Cell>()?;
            ton_abi::TokenValue::Cell(value)
        }
        ton_abi::ParamType::Map(key_ty, value_ty) => {
            let list = value.extract::<&PyList>()?;
            let mut result = BTreeMap::new();
            for item in list {
                let (key, value) = parse_map_entry_token(key_ty, value_ty, item)?;
                result.insert(key, value);
            }
            ton_abi::TokenValue::Map(*key_ty.clone(), *value_ty.clone(), result)
        }
        ton_abi::ParamType::Address => {
            let Address(addr) = value.extract::<Address>()?;
            ton_abi::TokenValue::Address(match addr {
                ton_block::MsgAddressInt::AddrStd(addr) => ton_block::MsgAddress::AddrStd(addr),
                ton_block::MsgAddressInt::AddrVar(addr) => ton_block::MsgAddress::AddrVar(addr),
            })
        }
        ton_abi::ParamType::Bytes => {
            let bytes = value.extract::<&[u8]>()?;
            ton_abi::TokenValue::Bytes(bytes.to_vec())
        }
        ton_abi::ParamType::FixedBytes(len) => {
            let bytes = value.extract::<&[u8]>()?;
            if bytes.len() != *len {
                return Err(PyValueError::new_err("Invalid fixed bytes length"));
            }
            ton_abi::TokenValue::FixedBytes(bytes.to_vec())
        }
        ton_abi::ParamType::String => {
            let value = value.extract::<String>()?;
            ton_abi::TokenValue::String(value)
        }
        ton_abi::ParamType::Token => {
            let value = value.extract::<u128>()?;
            let value = ton_block::Grams::new(value).handle_runtime_error()?;
            ton_abi::TokenValue::Token(value)
        }
        ton_abi::ParamType::Time => value.extract::<u64>().map(ton_abi::TokenValue::Time)?,
        ton_abi::ParamType::Expire => value.extract::<u32>().map(ton_abi::TokenValue::Expire)?,
        ton_abi::ParamType::PublicKey => {
            let value = if value.is_none() {
                None
            } else {
                let value = hex::decode(value.extract::<&str>()?).handle_runtime_error()?;
                Some(ed25519_dalek::PublicKey::from_bytes(&value).handle_runtime_error()?)
            };
            ton_abi::TokenValue::PublicKey(value)
        }
        ton_abi::ParamType::Optional(ty) => {
            let value = if value.is_none() {
                None
            } else {
                Some(parse_token(ty.as_ref(), value).map(Box::new)?)
            };
            ton_abi::TokenValue::Optional(*ty.clone(), value)
        }
        ton_abi::ParamType::Ref(ty) => {
            ton_abi::TokenValue::Ref(parse_token(ty.as_ref(), value).map(Box::new)?)
        }
    })
}

fn parse_map_entry_token(
    key_ty: &ton_abi::ParamType,
    value_ty: &ton_abi::ParamType,
    item: &PyAny,
) -> PyResult<(ton_abi::MapKeyTokenValue, ton_abi::TokenValue)> {
    use pyo3::types::PyTuple;

    let mut tuple = item.extract::<&PyTuple>()?.into_iter();
    let key = match tuple.next() {
        None => {
            return Err(PyValueError::new_err(
                "Expected mapping key in the first tuple element",
            ))
        }
        Some(value) => match key_ty {
            ton_abi::ParamType::Uint(size) => {
                let number = value.extract::<num_bigint::BigUint>()?;
                ton_abi::MapKeyTokenValue::Uint(ton_abi::Uint {
                    number,
                    size: *size,
                })
            }
            ton_abi::ParamType::Int(size) => {
                let number = value.extract::<num_bigint::BigInt>()?;
                ton_abi::MapKeyTokenValue::Int(ton_abi::Int {
                    number,
                    size: *size,
                })
            }
            ton_abi::ParamType::Address => {
                let Address(addr) = value.extract::<Address>()?;
                ton_abi::MapKeyTokenValue::Address(match addr {
                    ton_block::MsgAddressInt::AddrStd(addr) => ton_block::MsgAddress::AddrStd(addr),
                    ton_block::MsgAddressInt::AddrVar(addr) => ton_block::MsgAddress::AddrVar(addr),
                })
            }
            _ => return Err(PyValueError::new_err("Unsupported mapping key type")),
        },
    };

    let value = match tuple.next() {
        None => {
            return Err(PyValueError::new_err(
                "Expected mapping value in the second tuple element",
            ))
        }
        Some(value) => parse_token(value_ty, value)?,
    };

    Ok((key, value))
}

pub fn convert_tokens(py: Python, tokens: Vec<ton_abi::Token>) -> PyResult<&PyDict> {
    let result = PyDict::new(py);
    for token in tokens {
        result.set_item(&token.name, convert_token(py, token.value)?)?;
    }
    Ok(result)
}

fn convert_token(py: Python, value: ton_abi::TokenValue) -> PyResult<PyObject> {
    use pyo3::types::*;

    Ok(match value {
        ton_abi::TokenValue::Uint(ton_abi::Uint { number, .. }) => number.to_object(py),
        ton_abi::TokenValue::Int(ton_abi::Int { number, .. }) => number.to_object(py),
        ton_abi::TokenValue::VarInt(_, number) => number.to_object(py),
        ton_abi::TokenValue::VarUint(_, number) => number.to_object(py),
        ton_abi::TokenValue::Bool(value) => value.to_object(py),
        ton_abi::TokenValue::Tuple(values) => convert_tokens(py, values)?.to_object(py),
        ton_abi::TokenValue::Array(_, values) | ton_abi::TokenValue::FixedArray(_, values) => {
            let items = values
                .into_iter()
                .map(|item| convert_token(py, item))
                .collect::<PyResult<Vec<_>>>()?;
            PyList::new(py, items).to_object(py)
        }
        ton_abi::TokenValue::Cell(cell) => Cell(cell).into_py(py),
        ton_abi::TokenValue::Map(_, _, values) => {
            let items = values
                .into_iter()
                .map(|(key, value)| convert_map_entry_token(py, key, value))
                .collect::<PyResult<Vec<_>>>()?;
            PyList::new(py, items).to_object(py)
        }
        ton_abi::TokenValue::Address(addr) => convert_addr_token(py, addr)?,
        ton_abi::TokenValue::Bytes(bytes) | ton_abi::TokenValue::FixedBytes(bytes) => {
            PyBytes::new(py, &bytes).to_object(py)
        }
        ton_abi::TokenValue::String(string) => PyString::new(py, &string).to_object(py),
        ton_abi::TokenValue::Token(number) => number.0.to_object(py),
        ton_abi::TokenValue::Time(number) => number.to_object(py),
        ton_abi::TokenValue::Expire(number) => number.to_object(py),
        ton_abi::TokenValue::PublicKey(pubkey) => match pubkey {
            Some(value) => hex::encode(value.as_bytes()).to_object(py),
            None => py.None(),
        },
        ton_abi::TokenValue::Optional(_, value) => match value {
            Some(value) => convert_token(py, *value)?,
            None => py.None(),
        },
        ton_abi::TokenValue::Ref(value) => convert_token(py, *value)?,
    })
}

fn convert_map_entry_token(
    py: Python,
    key: ton_abi::MapKeyTokenValue,
    value: ton_abi::TokenValue,
) -> PyResult<PyObject> {
    use pyo3::types::*;

    let key = match key {
        ton_abi::MapKeyTokenValue::Uint(ton_abi::Uint { number, .. }) => number.to_object(py),
        ton_abi::MapKeyTokenValue::Int(ton_abi::Int { number, .. }) => number.to_object(py),
        ton_abi::MapKeyTokenValue::Address(addr) => convert_addr_token(py, addr)?,
    };

    Ok(PyTuple::new(py, [key, convert_token(py, value)?]).to_object(py))
}

fn convert_addr_token(py: Python, addr: ton_block::MsgAddress) -> PyResult<PyObject> {
    Ok(Address(match addr {
        ton_block::MsgAddress::AddrStd(addr) => ton_block::MsgAddressInt::AddrStd(addr),
        ton_block::MsgAddress::AddrVar(addr) => ton_block::MsgAddressInt::AddrVar(addr),
        _ => return Err(PyRuntimeError::new_err("Unsupported address type")),
    })
    .into_py(py))
}
