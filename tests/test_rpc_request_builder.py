"""These tests are mainly about getting mypy to check stuff, as it doesn't check doc examples."""

import json
from typing import List, Union

from solders.pubkey import Pubkey
from solders.rpc.config import (
    RpcAccountInfoConfig,
    RpcProgramAccountsV2Config,
    RpcSimulateBundleAccountsConfig,
    RpcSimulateBundleConfig,
    RpcSignatureStatusConfig,
)
from solders.rpc.filter import Memcmp
from solders.rpc.requests import (
    GetProgramAccountsV2,
    GetSignatureStatuses,
    RequestAirdrop,
    SimulateBundle,
    batch_to_json,
)
from solders.account_decoder import UiAccountEncoding
from solders.signature import Signature
from solders.transaction_status import UiTransactionEncoding


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
