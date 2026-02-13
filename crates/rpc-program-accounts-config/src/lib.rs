use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use solana_rpc_client_types::config as rpc_config;
use solders_macros::{common_methods, richcmp_eq_only};
use solders_traits_core::{
    impl_display, py_from_bytes_general_via_cbor, pybytes_general_via_cbor, RichcmpEqualityOnly,
};

use solders_rpc_account_info_config::RpcAccountInfoConfig;
use solders_rpc_config_macros::pyclass_boilerplate_with_default;
use solders_rpc_filter::RpcFilterType;

pyclass_boilerplate_with_default!(
    /// Configuration object for ``getProgramAccounts``.
    ///
    /// Args:
    ///     account_config (RpcAccountInfoConfig): Account info config.
    ///     filters (Optional[Sequence[int | Memcmp]]): Filter results using various filter objects; account must meet all filter criteria to be included in results.
    ///     with_context (Optional[bool]): Wrap the result in an RpcResponse JSON object.
    ///
    => RpcProgramAccountsConfig
);

#[richcmp_eq_only]
#[common_methods]
#[pymethods]
impl RpcProgramAccountsConfig {
    #[pyo3(signature = (account_config, filters=None, with_context=None, sort_results=None))]
    #[new]
    pub fn new(
        account_config: RpcAccountInfoConfig,
        filters: Option<Vec<RpcFilterType>>,
        with_context: Option<bool>,
        sort_results: Option<bool>,
    ) -> Self {
        Self(rpc_config::RpcProgramAccountsConfig {
            filters: filters.map(|v| v.into_iter().map(|f| f.into()).collect()),
            account_config: account_config.into(),
            with_context,
            sort_results,
        })
    }

    /// Create a new default instance of this class.
    ///
    /// Returns:
    ///     RpcEpochConfig: The default instance.
    #[staticmethod]
    #[pyo3(name = "default")]
    pub fn new_default() -> Self {
        Self::default()
    }

    #[getter]
    pub fn account_config(&self) -> RpcAccountInfoConfig {
        self.0.account_config.clone().into()
    }

    #[getter]
    pub fn filters<'a>(&self, py: Python<'a>) -> Option<Vec<Bound<'a, PyAny>>> {
        let cloned = self.0.filters.clone();
        cloned.map(|v| {
            v.into_iter()
                .map(|f| RpcFilterType::from(f).into_pyobject(py).unwrap())
                .collect()
        })
    }

    #[getter]
    pub fn with_context(&self) -> Option<bool> {
        self.0.with_context
    }
}

/// Configuration object for ``getProgramAccountsV2``.
///
/// Args:
///     account_config (RpcAccountInfoConfig): Account info config.
///     filters (Optional[Sequence[int | Memcmp]]): Filter results using various filter objects; account must meet all filter criteria to be included in results.
///     with_context (Optional[bool]): Wrap the result in an RpcResponse JSON object.
///     sort_results (Optional[bool]): Request deterministic account ordering where supported.
///     limit (Optional[int]): Maximum number of accounts to return per request.
///     pagination_key (Optional[str]): Cursor from a previous ``getProgramAccountsV2`` response.
///     changed_since_slot (Optional[int]): Return only accounts changed at or after this slot.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
#[pyclass(module = "solders.rpc.config", subclass)]
pub struct RpcProgramAccountsV2Config {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    filters: Option<Vec<RpcFilterType>>,
    #[serde(flatten)]
    account_config: RpcAccountInfoConfig,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    with_context: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    sort_results: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    limit: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pagination_key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    changed_since_slot: Option<u64>,
}

pybytes_general_via_cbor!(RpcProgramAccountsV2Config);
py_from_bytes_general_via_cbor!(RpcProgramAccountsV2Config);
impl RichcmpEqualityOnly for RpcProgramAccountsV2Config {}
solders_traits_core::common_methods_default!(RpcProgramAccountsV2Config);

impl std::fmt::Display for RpcProgramAccountsV2Config {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

#[richcmp_eq_only]
#[common_methods]
#[pymethods]
impl RpcProgramAccountsV2Config {
    #[pyo3(signature = (account_config, filters=None, with_context=None, sort_results=None, limit=None, pagination_key=None, changed_since_slot=None))]
    #[new]
    pub fn new(
        account_config: RpcAccountInfoConfig,
        filters: Option<Vec<RpcFilterType>>,
        with_context: Option<bool>,
        sort_results: Option<bool>,
        limit: Option<u64>,
        pagination_key: Option<String>,
        changed_since_slot: Option<u64>,
    ) -> Self {
        Self {
            filters,
            account_config,
            with_context,
            sort_results,
            limit,
            pagination_key,
            changed_since_slot,
        }
    }

    /// Create a new default instance of this class.
    ///
    /// Returns:
    ///     RpcProgramAccountsV2Config: The default instance.
    #[staticmethod]
    #[pyo3(name = "default")]
    pub fn new_default() -> Self {
        Self::default()
    }

    #[getter]
    pub fn account_config(&self) -> RpcAccountInfoConfig {
        self.account_config.clone()
    }

    #[getter]
    pub fn filters<'a>(&self, py: Python<'a>) -> Option<Vec<Bound<'a, PyAny>>> {
        self.filters.clone().map(|v| {
            v.into_iter()
                .map(|f| f.into_pyobject(py).unwrap())
                .collect()
        })
    }

    #[getter]
    pub fn with_context(&self) -> Option<bool> {
        self.with_context
    }

    #[getter]
    pub fn sort_results(&self) -> Option<bool> {
        self.sort_results
    }

    #[getter]
    pub fn limit(&self) -> Option<u64> {
        self.limit
    }

    #[getter]
    pub fn pagination_key(&self) -> Option<String> {
        self.pagination_key.clone()
    }

    #[getter]
    pub fn changed_since_slot(&self) -> Option<u64> {
        self.changed_since_slot
    }
}
