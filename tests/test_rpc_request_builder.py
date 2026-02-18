"""These tests are mainly about getting mypy to check stuff, as it doesn't check doc examples."""

import json
from typing import List, Union

from pytest import raises
from solders.account_decoder import UiAccountEncoding
from solders.pubkey import Pubkey
from solders.rpc.config import (
    RpcAccountInfoConfig,
    RpcProgramAccountsV2Config,
    RpcSignatureStatusConfig,
    RpcSimulateBundleAccountsConfig,
    RpcSimulateBundleConfig,
    RpcTransactionsForAddressConfig,
    RpcTransactionsForAddressFilters,
    RpcTransactionsForAddressSlotRange,
    RpcTransactionsForAddressSortOrder,
    RpcTransactionsForAddressStatus,
)
from solders.rpc.filter import Memcmp
from solders.rpc.requests import (
    GetProgramAccountsV2,
    GetSignatureStatuses,
    GetTransactionsForAddress,
    RequestAirdrop,
    SimulateBundle,
    batch_to_json,
)
from solders.signature import Signature
from solders.transaction_status import TransactionDetails, UiTransactionEncoding


def test_batch() -> None:
    reqs: List[Union[GetSignatureStatuses, RequestAirdrop]] = [
        GetSignatureStatuses([Signature.default()], RpcSignatureStatusConfig(True)),
        RequestAirdrop(Pubkey.default(), 1000),
    ]
    as_json = batch_to_json(reqs)
    assert as_json == (
        '[{"method":"getSignatureStatuses","jsonrpc":"2.0","id":0,"params"'
        ':[["1111111111111111111111111111111111111111111111111111111111111111"],'
        '{"searchTransactionHistory":true}]},{"method":"requestAirdrop","jsonrpc":"2.0","id":0,'
        '"params":["11111111111111111111111111111111",1000]}]'
    )


def test_get_program_accounts_v2_request() -> None:
    config = RpcProgramAccountsV2Config(
        account_config=RpcAccountInfoConfig.default(),
        filters=[10, Memcmp(offset=10, bytes_=b"123")],
        limit=1000,
        pagination_key="cursor",
        changed_since_slot=123_456,
    )
    req = GetProgramAccountsV2(Pubkey.default(), config, 99)
    as_json = json.loads(req.to_json())

    assert as_json["method"] == "getProgramAccountsV2"
    assert as_json["id"] == 99
    assert as_json["params"][0] == "11111111111111111111111111111111"
    assert as_json["params"][1]["limit"] == 1000
    assert as_json["params"][1]["paginationKey"] == "cursor"
    assert as_json["params"][1]["changedSinceSlot"] == 123_456


def test_simulate_bundle_request() -> None:
    account_config = RpcSimulateBundleAccountsConfig(
        ["11111111111111111111111111111111"], UiAccountEncoding.Base64
    )
    config = RpcSimulateBundleConfig(
        [account_config],
        [None],
        UiTransactionEncoding.Base64,
        {"commitment": {"commitment": "processed"}},
        True,
        False,
    )
    req = SimulateBundle(["AQID"], config, 7)
    as_json = json.loads(req.to_json())

    assert as_json == {
        "method": "simulateBundle",
        "jsonrpc": "2.0",
        "id": 7,
        "params": [
            {"encodedTransactions": ["AQID"]},
            {
                "preExecutionAccountsConfigs": [
                    {
                        "addresses": ["11111111111111111111111111111111"],
                        "encoding": "base64",
                    }
                ],
                "postExecutionAccountsConfigs": [None],
                "transactionEncoding": "base64",
                "simulationBank": {"commitment": {"commitment": "processed"}},
                "skipSigVerify": True,
                "replaceRecentBlockhash": False,
            },
        ],
    }


def test_simulate_bundle_request_tip_bank() -> None:
    config = RpcSimulateBundleConfig([None], [None], simulation_bank="tip")
    req = SimulateBundle(["AQID"], config)
    as_json = json.loads(req.to_json())
    assert as_json["params"][1]["simulationBank"] == "tip"


def test_simulate_bundle_request_slot_bank() -> None:
    config = RpcSimulateBundleConfig([None], [None], simulation_bank={"slot": 373976835})
    req = SimulateBundle(["AQID"], config)
    as_json = json.loads(req.to_json())
    assert as_json["params"][1]["simulationBank"] == {"slot": 373976835}


def test_simulate_bundle_request_invalid_bank() -> None:
    with raises(ValueError):
        RpcSimulateBundleConfig([None], [None], simulation_bank="processed")


def test_get_transactions_for_address_request() -> None:
    config = RpcTransactionsForAddressConfig(
        transaction_details=TransactionDetails.Signatures,
        sort_order=RpcTransactionsForAddressSortOrder.Desc,
        limit=10,
        filters=RpcTransactionsForAddressFilters(
            status=RpcTransactionsForAddressStatus.Succeeded,
            slot=RpcTransactionsForAddressSlotRange(gte=1000, lt=2000),
        ),
    )
    req = GetTransactionsForAddress(Pubkey.default(), config, 8)
    as_json = json.loads(req.to_json())

    assert as_json == {
        "method": "getTransactionsForAddress",
        "jsonrpc": "2.0",
        "id": 8,
        "params": [
            "11111111111111111111111111111111",
            {
                "transactionDetails": "signatures",
                "sortOrder": "desc",
                "limit": 10,
                "filters": {"status": "succeeded", "slot": {"gte": 1000, "lt": 2000}},
            },
        ],
    }
