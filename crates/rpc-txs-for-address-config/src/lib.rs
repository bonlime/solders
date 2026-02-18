use pyo3::prelude::*;
use serde::{de::Error as _, Deserialize, Deserializer, Serialize, Serializer};
use solders_commitment_config::CommitmentLevel;
use solders_macros::{common_methods, richcmp_eq_only};
use solders_signature::Signature;
use solders_traits_core::{
    py_from_bytes_general_via_cbor, pybytes_general_via_cbor, RichcmpEqualityOnly,
};
use solders_transaction_status_enums::{TransactionDetails, UiTransactionEncoding};

#[pyclass(module = "solders.rpc.config", eq, eq_int)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RpcTransactionsForAddressSortOrder {
    Asc,
    Desc,
}

#[pyclass(module = "solders.rpc.config", eq, eq_int)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RpcTransactionsForAddressStatus {
    Succeeded,
    Failed,
    Any,
}

#[pyclass(module = "solders.rpc.config", eq, eq_int)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum RpcTransactionsForAddressTokenAccounts {
    #[serde(rename = "none")]
    None_,
    BalanceChanged,
    All,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
#[pyclass(module = "solders.rpc.config", subclass)]
pub struct RpcTransactionsForAddressSlotRange {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    gte: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    gt: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    lte: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    lt: Option<u64>,
}

pybytes_general_via_cbor!(RpcTransactionsForAddressSlotRange);
py_from_bytes_general_via_cbor!(RpcTransactionsForAddressSlotRange);
impl RichcmpEqualityOnly for RpcTransactionsForAddressSlotRange {}
solders_traits_core::common_methods_default!(RpcTransactionsForAddressSlotRange);

impl std::fmt::Display for RpcTransactionsForAddressSlotRange {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

#[richcmp_eq_only]
#[common_methods]
#[pymethods]
impl RpcTransactionsForAddressSlotRange {
    #[pyo3(signature = (gte=None, gt=None, lte=None, lt=None))]
    #[new]
    pub fn new(gte: Option<u64>, gt: Option<u64>, lte: Option<u64>, lt: Option<u64>) -> Self {
        Self { gte, gt, lte, lt }
    }

    /// Create a new default instance of this class.
    ///
    /// Returns:
    ///     RpcTransactionsForAddressSlotRange: The default instance.
    #[staticmethod]
    #[pyo3(name = "default")]
    pub fn new_default() -> Self {
        Self::default()
    }

    #[getter]
    pub fn gte(&self) -> Option<u64> {
        self.gte
    }

    #[getter]
    pub fn gt(&self) -> Option<u64> {
        self.gt
    }

    #[getter]
    pub fn lte(&self) -> Option<u64> {
        self.lte
    }

    #[getter]
    pub fn lt(&self) -> Option<u64> {
        self.lt
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
#[pyclass(module = "solders.rpc.config", subclass)]
pub struct RpcTransactionsForAddressBlockTimeRange {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    gte: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    gt: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    lte: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    lt: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    eq: Option<u64>,
}

pybytes_general_via_cbor!(RpcTransactionsForAddressBlockTimeRange);
py_from_bytes_general_via_cbor!(RpcTransactionsForAddressBlockTimeRange);
impl RichcmpEqualityOnly for RpcTransactionsForAddressBlockTimeRange {}
solders_traits_core::common_methods_default!(RpcTransactionsForAddressBlockTimeRange);

impl std::fmt::Display for RpcTransactionsForAddressBlockTimeRange {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

#[richcmp_eq_only]
#[common_methods]
#[pymethods]
impl RpcTransactionsForAddressBlockTimeRange {
    #[pyo3(signature = (gte=None, gt=None, lte=None, lt=None, eq=None))]
    #[new]
    pub fn new(
        gte: Option<u64>,
        gt: Option<u64>,
        lte: Option<u64>,
        lt: Option<u64>,
        eq: Option<u64>,
    ) -> Self {
        Self {
            gte,
            gt,
            lte,
            lt,
            eq,
        }
    }

    /// Create a new default instance of this class.
    ///
    /// Returns:
    ///     RpcTransactionsForAddressBlockTimeRange: The default instance.
    #[staticmethod]
    #[pyo3(name = "default")]
    pub fn new_default() -> Self {
        Self::default()
    }

    #[getter]
    pub fn gte(&self) -> Option<u64> {
        self.gte
    }

    #[getter]
    pub fn gt(&self) -> Option<u64> {
        self.gt
    }

    #[getter]
    pub fn lte(&self) -> Option<u64> {
        self.lte
    }

    #[getter]
    pub fn lt(&self) -> Option<u64> {
        self.lt
    }

    #[getter]
    pub fn eq(&self) -> Option<u64> {
        self.eq
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
#[pyclass(module = "solders.rpc.config", subclass)]
pub struct RpcTransactionsForAddressSignatureRange {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    gte: Option<Signature>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    gt: Option<Signature>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    lte: Option<Signature>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    lt: Option<Signature>,
}

pybytes_general_via_cbor!(RpcTransactionsForAddressSignatureRange);
py_from_bytes_general_via_cbor!(RpcTransactionsForAddressSignatureRange);
impl RichcmpEqualityOnly for RpcTransactionsForAddressSignatureRange {}
solders_traits_core::common_methods_default!(RpcTransactionsForAddressSignatureRange);

impl std::fmt::Display for RpcTransactionsForAddressSignatureRange {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

#[richcmp_eq_only]
#[common_methods]
#[pymethods]
impl RpcTransactionsForAddressSignatureRange {
    #[pyo3(signature = (gte=None, gt=None, lte=None, lt=None))]
    #[new]
    pub fn new(
        gte: Option<Signature>,
        gt: Option<Signature>,
        lte: Option<Signature>,
        lt: Option<Signature>,
    ) -> Self {
        Self { gte, gt, lte, lt }
    }

    /// Create a new default instance of this class.
    ///
    /// Returns:
    ///     RpcTransactionsForAddressSignatureRange: The default instance.
    #[staticmethod]
    #[pyo3(name = "default")]
    pub fn new_default() -> Self {
        Self::default()
    }

    #[getter]
    pub fn gte(&self) -> Option<Signature> {
        self.gte
    }

    #[getter]
    pub fn gt(&self) -> Option<Signature> {
        self.gt
    }

    #[getter]
    pub fn lte(&self) -> Option<Signature> {
        self.lte
    }

    #[getter]
    pub fn lt(&self) -> Option<Signature> {
        self.lt
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
#[pyclass(module = "solders.rpc.config", subclass)]
pub struct RpcTransactionsForAddressFilters {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    slot: Option<RpcTransactionsForAddressSlotRange>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    block_time: Option<RpcTransactionsForAddressBlockTimeRange>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    signature: Option<RpcTransactionsForAddressSignatureRange>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    status: Option<RpcTransactionsForAddressStatus>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    token_accounts: Option<RpcTransactionsForAddressTokenAccounts>,
}

pybytes_general_via_cbor!(RpcTransactionsForAddressFilters);
py_from_bytes_general_via_cbor!(RpcTransactionsForAddressFilters);
impl RichcmpEqualityOnly for RpcTransactionsForAddressFilters {}
solders_traits_core::common_methods_default!(RpcTransactionsForAddressFilters);

impl std::fmt::Display for RpcTransactionsForAddressFilters {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

#[richcmp_eq_only]
#[common_methods]
#[pymethods]
impl RpcTransactionsForAddressFilters {
    #[pyo3(signature = (slot=None, block_time=None, signature=None, status=None, token_accounts=None))]
    #[new]
    pub fn new(
        slot: Option<RpcTransactionsForAddressSlotRange>,
        block_time: Option<RpcTransactionsForAddressBlockTimeRange>,
        signature: Option<RpcTransactionsForAddressSignatureRange>,
        status: Option<RpcTransactionsForAddressStatus>,
        token_accounts: Option<RpcTransactionsForAddressTokenAccounts>,
    ) -> Self {
        Self {
            slot,
            block_time,
            signature,
            status,
            token_accounts,
        }
    }

    /// Create a new default instance of this class.
    ///
    /// Returns:
    ///     RpcTransactionsForAddressFilters: The default instance.
    #[staticmethod]
    #[pyo3(name = "default")]
    pub fn new_default() -> Self {
        Self::default()
    }

    #[getter]
    pub fn slot(&self) -> Option<RpcTransactionsForAddressSlotRange> {
        self.slot.clone()
    }

    #[getter]
    pub fn block_time(&self) -> Option<RpcTransactionsForAddressBlockTimeRange> {
        self.block_time.clone()
    }

    #[getter]
    pub fn signature(&self) -> Option<RpcTransactionsForAddressSignatureRange> {
        self.signature.clone()
    }

    #[getter]
    pub fn status(&self) -> Option<RpcTransactionsForAddressStatus> {
        self.status
    }

    #[getter]
    pub fn token_accounts(&self) -> Option<RpcTransactionsForAddressTokenAccounts> {
        self.token_accounts
    }
}

fn serialize_commitment<S>(
    value: &Option<CommitmentLevel>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let as_str = value.map(|v| match v {
        CommitmentLevel::Processed => "processed",
        CommitmentLevel::Confirmed => "confirmed",
        CommitmentLevel::Finalized => "finalized",
    });
    as_str.serialize(serializer)
}

fn deserialize_commitment<'de, D>(deserializer: D) -> Result<Option<CommitmentLevel>, D::Error>
where
    D: Deserializer<'de>,
{
    let maybe: Option<String> = Option::<String>::deserialize(deserializer)?;
    maybe
        .map(|s| match s.as_str() {
            "processed" => Ok(CommitmentLevel::Processed),
            "confirmed" => Ok(CommitmentLevel::Confirmed),
            "finalized" => Ok(CommitmentLevel::Finalized),
            _ => Err(D::Error::custom("invalid commitment level")),
        })
        .transpose()
}

/// Configuration object for ``getTransactionsForAddress``.
///
/// Args:
///     transaction_details (Optional[TransactionDetails]):
///         Level of transaction detail to return.
///     sort_order (Optional[RpcTransactionsForAddressSortOrder]): Sort order for results.
///     commitment (Optional[CommitmentLevel]): Commitment level.
///     min_context_slot (Optional[int]): Minimum context slot.
///     limit (Optional[int]): Maximum number of records to return.
///     pagination_token (Optional[str]): Pagination token from a previous response.
///     encoding (Optional[UiTransactionEncoding]):
///         Transaction encoding when ``transaction_details`` is ``Full``.
///     max_supported_transaction_version (Optional[int]):
///         Maximum transaction version when ``transaction_details`` is ``Full``.
///     filters (Optional[RpcTransactionsForAddressFilters]): Filtering object with ``slot``,
///         ``blockTime``, ``signature``, ``status``, and ``tokenAccounts`` keys.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
#[pyclass(module = "solders.rpc.config", subclass)]
pub struct RpcTransactionsForAddressConfig {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    transaction_details: Option<TransactionDetails>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    sort_order: Option<RpcTransactionsForAddressSortOrder>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_commitment",
        deserialize_with = "deserialize_commitment"
    )]
    commitment: Option<CommitmentLevel>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    min_context_slot: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    limit: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pagination_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    encoding: Option<UiTransactionEncoding>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    max_supported_transaction_version: Option<u8>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    filters: Option<RpcTransactionsForAddressFilters>,
}

pybytes_general_via_cbor!(RpcTransactionsForAddressConfig);
py_from_bytes_general_via_cbor!(RpcTransactionsForAddressConfig);
impl RichcmpEqualityOnly for RpcTransactionsForAddressConfig {}
solders_traits_core::common_methods_default!(RpcTransactionsForAddressConfig);

impl std::fmt::Display for RpcTransactionsForAddressConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

#[richcmp_eq_only]
#[common_methods]
#[pymethods]
impl RpcTransactionsForAddressConfig {
    #[pyo3(
        signature = (
            transaction_details=None,
            sort_order=None,
            commitment=None,
            min_context_slot=None,
            limit=None,
            pagination_token=None,
            encoding=None,
            max_supported_transaction_version=None,
            filters=None,
        )
    )]
    #[new]
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        transaction_details: Option<TransactionDetails>,
        sort_order: Option<RpcTransactionsForAddressSortOrder>,
        commitment: Option<CommitmentLevel>,
        min_context_slot: Option<u64>,
        limit: Option<u64>,
        pagination_token: Option<String>,
        encoding: Option<UiTransactionEncoding>,
        max_supported_transaction_version: Option<u8>,
        filters: Option<RpcTransactionsForAddressFilters>,
    ) -> Self {
        Self {
            transaction_details,
            sort_order,
            commitment,
            min_context_slot,
            limit,
            pagination_token,
            encoding,
            max_supported_transaction_version,
            filters,
        }
    }

    /// Create a new default instance of this class.
    ///
    /// Returns:
    ///     RpcTransactionsForAddressConfig: The default instance.
    #[staticmethod]
    #[pyo3(name = "default")]
    pub fn new_default() -> Self {
        Self::default()
    }

    #[getter]
    pub fn transaction_details(&self) -> Option<TransactionDetails> {
        self.transaction_details
    }

    #[getter]
    pub fn sort_order(&self) -> Option<RpcTransactionsForAddressSortOrder> {
        self.sort_order
    }

    #[getter]
    pub fn commitment(&self) -> Option<CommitmentLevel> {
        self.commitment
    }

    #[getter]
    pub fn min_context_slot(&self) -> Option<u64> {
        self.min_context_slot
    }

    #[getter]
    pub fn limit(&self) -> Option<u64> {
        self.limit
    }

    #[getter]
    pub fn pagination_token(&self) -> Option<String> {
        self.pagination_token.clone()
    }

    #[getter]
    pub fn encoding(&self) -> Option<UiTransactionEncoding> {
        self.encoding
    }

    #[getter]
    pub fn max_supported_transaction_version(&self) -> Option<u8> {
        self.max_supported_transaction_version
    }

    #[getter]
    pub fn filters(&self) -> Option<RpcTransactionsForAddressFilters> {
        self.filters.clone()
    }
}
