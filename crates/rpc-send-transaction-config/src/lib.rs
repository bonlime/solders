use pyo3::prelude::*;
use pythonize::{depythonize, pythonize};
use serde::{Deserialize, Serialize};
use solana_commitment_config::CommitmentLevel as CommitmentLevelOriginal;
use solana_rpc_client_types::config as rpc_config;
use solana_transaction_status_client_types::UiTransactionEncoding as UiTransactionEncodingOriginal;
use solders_account_decoder::UiAccountEncoding;
use solders_commitment_config::CommitmentLevel;
use solders_macros::{common_methods, richcmp_eq_only};

use solders_rpc_config_macros::pyclass_boilerplate_with_default;
use solders_traits_core::{
    handle_py_value_err, impl_display, py_from_bytes_general_via_cbor, pybytes_general_via_cbor,
    RichcmpEqualityOnly,
};
use solders_transaction_status_enums::UiTransactionEncoding;

pyclass_boilerplate_with_default!(
    /// Configuration object for ``sendTransaction``.
    ///
    /// Args:
    ///     skip_preflight (bool):  If true, skip the preflight transaction checks.
    ///     preflight_commitment (Optional[CommitmentLevel]): Commitment level to use for preflight checks.
    ///     max_retries: (Optional[int]): Maximum number of times for the RPC node to retry sending
    ///         the transaction to the leader. If this parameter not provided, the RPC node will
    ///         retry the transaction until it is finalized or until the blockhash expires.
    ///     min_context_slot (Optional[int]): The minimum slot that the request can be evaluated at.
    ///
    => RpcSendTransactionConfig
);

#[richcmp_eq_only]
#[common_methods]
#[pymethods]
impl RpcSendTransactionConfig {
    #[new]
    #[pyo3(signature = (skip_preflight=false, preflight_commitment=None, max_retries=None, min_context_slot=None))]
    pub fn new(
        skip_preflight: bool,
        preflight_commitment: Option<CommitmentLevel>,
        max_retries: Option<usize>,
        min_context_slot: Option<u64>,
    ) -> Self {
        Self(rpc_config::RpcSendTransactionConfig {
            skip_preflight,
            preflight_commitment: preflight_commitment.map(CommitmentLevelOriginal::from),
            encoding: Some(UiTransactionEncodingOriginal::Base64),
            max_retries,
            min_context_slot,
        })
    }

    /// bool:  If true, skip the preflight transaction checks.
    #[getter]
    pub fn skip_preflight(&self) -> bool {
        self.0.skip_preflight
    }

    /// Optional[CommitmentLevel]: Commitment level to use for preflight checks.
    #[getter]
    pub fn preflight_commitment(&self) -> Option<CommitmentLevel> {
        self.0.preflight_commitment.map(|p| p.into())
    }

    /// UiTransactionEncoding: Encoding used for the transaction data.
    #[getter]
    pub fn encoding(&self) -> Option<UiTransactionEncoding> {
        self.0.encoding.map(Into::into)
    }

    /// Optional[int]: Maximum number of times for the RPC node to retry sending the transaction to the leader.
    #[getter]
    pub fn max_retries(&self) -> Option<usize> {
        self.0.max_retries
    }

    /// Optional[int]: The minimum slot that the request can be evaluated at.
    #[getter]
    pub fn min_context_slot(&self) -> Option<u64> {
        self.0.min_context_slot
    }

    /// Create a new default instance of this class.
    ///
    /// Returns:
    ///     RpcSendTransactionConfig: The default instance.
    #[staticmethod]
    #[pyo3(name = "default")]
    pub fn new_default() -> Self {
        Self::default()
    }
}

/// Account capture configuration object used by ``simulateBundle``.
///
/// Args:
///     addresses (Sequence[str]): List of base-58 account addresses to capture.
///     encoding (Optional[UiAccountEncoding]): Encoding for returned account data.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[pyclass(module = "solders.rpc.config", subclass)]
pub struct RpcSimulateBundleAccountsConfig {
    addresses: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    encoding: Option<UiAccountEncoding>,
}

pybytes_general_via_cbor!(RpcSimulateBundleAccountsConfig);
py_from_bytes_general_via_cbor!(RpcSimulateBundleAccountsConfig);
impl RichcmpEqualityOnly for RpcSimulateBundleAccountsConfig {}
solders_traits_core::common_methods_default!(RpcSimulateBundleAccountsConfig);

impl std::fmt::Display for RpcSimulateBundleAccountsConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

#[richcmp_eq_only]
#[common_methods]
#[pymethods]
impl RpcSimulateBundleAccountsConfig {
    #[pyo3(signature = (addresses, encoding=None))]
    #[new]
    pub fn new(addresses: Vec<String>, encoding: Option<UiAccountEncoding>) -> Self {
        Self {
            addresses,
            encoding,
        }
    }

    #[getter]
    pub fn addresses(&self) -> Vec<String> {
        self.addresses.clone()
    }

    #[getter]
    pub fn encoding(&self) -> Option<UiAccountEncoding> {
        self.encoding
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
enum RpcSimulateBundleSimulationBankTip {
    Tip,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RpcSimulateBundleSimulationBankSlot {
    slot: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RpcSimulateBundleSimulationBankCommitmentLevel {
    commitment: CommitmentLevel,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RpcSimulateBundleSimulationBankCommitment {
    commitment: RpcSimulateBundleSimulationBankCommitmentLevel,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
enum RpcSimulateBundleSimulationBank {
    Tip(RpcSimulateBundleSimulationBankTip),
    Slot(RpcSimulateBundleSimulationBankSlot),
    Commitment(RpcSimulateBundleSimulationBankCommitment),
}

/// Configuration object for ``simulateBundle``.
///
/// Args:
///     pre_execution_accounts_configs (Sequence[Optional[RpcSimulateBundleAccountsConfig]]):
///         Account-capture config per transaction before execution.
///     post_execution_accounts_configs (Sequence[Optional[RpcSimulateBundleAccountsConfig]]):
///         Account-capture config per transaction after execution.
///     transaction_encoding (Optional[UiTransactionEncoding]): Encoding used in encoded transactions.
///     simulation_bank (Optional[Union[str, dict]]): Simulation bank selector:
///         ``"tip"``, ``{"slot": <int>}``, or
///         ``{"commitment": {"commitment": "processed" | "confirmed" | "finalized"}}``.
///     skip_sig_verify (bool): Whether to skip signature verification.
///     replace_recent_blockhash (bool): Whether to replace recent blockhash before simulation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[pyclass(module = "solders.rpc.config", subclass)]
pub struct RpcSimulateBundleConfig {
    pre_execution_accounts_configs: Vec<Option<RpcSimulateBundleAccountsConfig>>,
    post_execution_accounts_configs: Vec<Option<RpcSimulateBundleAccountsConfig>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    transaction_encoding: Option<UiTransactionEncoding>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    simulation_bank: Option<RpcSimulateBundleSimulationBank>,
    #[serde(default)]
    skip_sig_verify: bool,
    #[serde(default)]
    replace_recent_blockhash: bool,
}

pybytes_general_via_cbor!(RpcSimulateBundleConfig);
py_from_bytes_general_via_cbor!(RpcSimulateBundleConfig);
impl RichcmpEqualityOnly for RpcSimulateBundleConfig {}
solders_traits_core::common_methods_default!(RpcSimulateBundleConfig);

impl std::fmt::Display for RpcSimulateBundleConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

#[richcmp_eq_only]
#[common_methods]
#[pymethods]
impl RpcSimulateBundleConfig {
    #[pyo3(
        signature = (
            pre_execution_accounts_configs,
            post_execution_accounts_configs,
            transaction_encoding=None,
            simulation_bank=None,
            skip_sig_verify=false,
            replace_recent_blockhash=false,
        )
    )]
    #[new]
    pub fn new(
        pre_execution_accounts_configs: Vec<Option<RpcSimulateBundleAccountsConfig>>,
        post_execution_accounts_configs: Vec<Option<RpcSimulateBundleAccountsConfig>>,
        transaction_encoding: Option<UiTransactionEncoding>,
        simulation_bank: Option<Bound<'_, PyAny>>,
        skip_sig_verify: bool,
        replace_recent_blockhash: bool,
    ) -> PyResult<Self> {
        let parsed_bank = simulation_bank
            .map(|bank| handle_py_value_err(depythonize::<RpcSimulateBundleSimulationBank>(&bank)))
            .transpose()?;
        Ok(Self {
            pre_execution_accounts_configs,
            post_execution_accounts_configs,
            transaction_encoding,
            simulation_bank: parsed_bank,
            skip_sig_verify,
            replace_recent_blockhash,
        })
    }

    #[getter]
    pub fn pre_execution_accounts_configs(&self) -> Vec<Option<RpcSimulateBundleAccountsConfig>> {
        self.pre_execution_accounts_configs.clone()
    }

    #[getter]
    pub fn post_execution_accounts_configs(&self) -> Vec<Option<RpcSimulateBundleAccountsConfig>> {
        self.post_execution_accounts_configs.clone()
    }

    #[getter]
    pub fn transaction_encoding(&self) -> Option<UiTransactionEncoding> {
        self.transaction_encoding
    }

    #[getter]
    pub fn simulation_bank(&self, py: Python<'_>) -> PyResult<Option<Py<PyAny>>> {
        self.simulation_bank
            .as_ref()
            .map(|bank| handle_py_value_err(pythonize(py, bank).map(Bound::unbind)))
            .transpose()
    }

    #[getter]
    pub fn skip_sig_verify(&self) -> bool {
        self.skip_sig_verify
    }

    #[getter]
    pub fn replace_recent_blockhash(&self) -> bool {
        self.replace_recent_blockhash
    }
}
