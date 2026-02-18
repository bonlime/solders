use std::str::FromStr;

use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use solana_rpc_client_types::response::RpcConfirmedTransactionStatusWithSignature as RpcConfirmedTransactionStatusWithSignatureOriginal;
use solana_transaction_error::TransactionError as TransactionErrorOriginal;
use solders_macros::{common_methods, richcmp_eq_only};
use solders_rpc_response_data_boilerplate::response_data_boilerplate;
use solders_signature::Signature;
use solders_transaction_confirmation_status::TransactionConfirmationStatus;
use solders_transaction_error::TransactionErrorType;

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[pyclass(module = "solders.rpc.responses", subclass)]
pub struct RpcConfirmedTransactionStatusWithSignature {
    #[serde(flatten)]
    value: RpcConfirmedTransactionStatusWithSignatureOriginal,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    transaction_index: Option<u64>,
}

response_data_boilerplate!(RpcConfirmedTransactionStatusWithSignature);

#[richcmp_eq_only]
#[common_methods]
#[pymethods]
impl RpcConfirmedTransactionStatusWithSignature {
    #[pyo3(signature = (signature, slot, err=None, memo=None, block_time=None, confirmation_status=None, transaction_index=None))]
    #[new]
    pub fn new(
        signature: Signature,
        slot: u64,
        err: Option<TransactionErrorType>,
        memo: Option<String>,
        block_time: Option<i64>,
        confirmation_status: Option<TransactionConfirmationStatus>,
        transaction_index: Option<u64>,
    ) -> Self {
        Self {
            value: RpcConfirmedTransactionStatusWithSignatureOriginal {
                signature: signature.to_string(),
                slot,
                err: err.map(|e| {
                    let orig = TransactionErrorOriginal::from(e);
                    orig.into()
                }),
                memo,
                block_time,
                confirmation_status: confirmation_status.map(|c| c.into()),
            },
            transaction_index,
        }
    }

    #[getter]
    pub fn signature(&self) -> Signature {
        Signature::from_str(&self.value.signature).unwrap()
    }
    #[getter]
    pub fn slot(&self) -> u64 {
        self.value.slot
    }
    #[getter]
    pub fn transaction_index(&self) -> Option<u64> {
        self.transaction_index
    }
    #[getter]
    pub fn err(&self) -> Option<TransactionErrorType> {
        self.value.err.clone().map(|e| {
            let orig = TransactionErrorOriginal::from(e);
            orig.into()
        })
    }
    #[getter]
    pub fn memo(&self) -> Option<String> {
        self.value.memo.clone()
    }
    #[getter]
    pub fn block_time(&self) -> Option<i64> {
        self.value.block_time
    }
    #[getter]
    pub fn confirmation_status(&self) -> Option<TransactionConfirmationStatus> {
        self.value.confirmation_status.clone().map(|s| s.into())
    }
}
