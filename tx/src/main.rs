use std::error::Error;

use stellar_xdr::{
    AccountId, Hash, HostFunction, InvokeHostFunctionOp, LedgerFootprint, LedgerKey,
    LedgerKeyContractData, Memo, MuxedAccount, Operation, OperationBody, Preconditions, PublicKey,
    ScObject, ScVal, ScVec, SequenceNumber, Transaction, TransactionEnvelope, TransactionExt,
    TransactionV1Envelope, Uint256, VecM, WriteXdr,
};

fn main() -> Result<(), Box<dyn Error>> {
    let account_id = [0u8; 32];
    eprintln!("Source Account: {}", hex::encode(account_id));

    let sequence_number = 1i64;
    eprintln!("Sequence Number: {}", sequence_number);

    let contract_id_raw = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 1,
    ];
    let contract_id = ScVal::Object(Some(ScObject::Bytes(contract_id_raw.clone().try_into()?)));
    eprintln!("Contract ID: {:?}", contract_id);

    let contract_function = ScVal::Symbol("put".try_into()?);
    eprintln!("Contract Function: {:?}", contract_function);

    let contract_args = ScVal::Object(Some(ScObject::Vec(ScVec(
        vec![
            // Argument 0: Key
            ScVal::Symbol("mykey".try_into()?),
            // Argument 1: Value
            ScVal::U32(123),
        ]
        .try_into()?,
    ))));
    eprintln!("Contract Args: {:?}", contract_args);

    let function = HostFunction::InvokeContract;
    eprintln!("Host Function Function: {:?}", function);

    let parameters = ScVec(vec![contract_id, contract_function, contract_args].try_into()?);
    eprintln!("Host Function Parameters: {:?}", parameters);

    let footprint = LedgerFootprint {
        read_only: VecM::default(),
        read_write: vec![LedgerKey::ContractData(LedgerKeyContractData {
            contract_id: Hash(contract_id_raw),
            // This is the key the contract is going to write to.
            key: ScVal::Object(Some(ScObject::Vec(ScVec(
                vec![
                    ScVal::Object(Some(ScObject::AccountId(AccountId(
                        PublicKey::PublicKeyTypeEd25519(Uint256(contract_id_raw.clone())),
                    )))),
                    ScVal::Symbol("mykey".try_into()?),
                ]
                .try_into()?,
            )))),
        })]
        .try_into()?,
    };
    eprintln!("Host Function Footprint: {:?}", parameters);

    let te = TransactionEnvelope::Tx(TransactionV1Envelope {
        tx: Transaction {
            source_account: MuxedAccount::Ed25519(Uint256(account_id)),
            fee: 200,
            seq_num: SequenceNumber(sequence_number),
            cond: Preconditions::None,
            memo: Memo::None,
            operations: [Operation {
                source_account: None,
                body: OperationBody::InvokeHostFunction(InvokeHostFunctionOp {
                    function,
                    parameters,
                    footprint,
                }),
            }]
            .to_vec()
            .try_into()?,
            ext: TransactionExt::V0,
        },
        signatures: [].try_into()?,
    });

    eprintln!();

    let xdr = te.to_xdr_base64()?;
    println!("{}", xdr);

    // let res = h.invoke_function(HostFunction::Call, args);
    Ok(())
}
