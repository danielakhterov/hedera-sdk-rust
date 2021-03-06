use super::CTransactionId;
use crate::{
    AccountId, Client, Query, QueryCryptoGetAccountBalanceResponse,
    QueryGetTransactionReceiptResponse,
};
use std::mem;

macro_rules! impl_get {
    ($name:ident($ty:ident)) => {
        #[doc(hidden)]
        #[no_mangle]
        pub unsafe extern "C" fn $name(query: *mut Query<$ty>, out: *mut $ty) -> u64 {
            debug_assert!(!query.is_null());
            debug_assert!(!out.is_null());

            *out = try_ffi!(Box::from_raw(query).get());

            0
        }
    };
}

#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn hedera_query_cost(query: *mut Query<()>, out: *mut u64) -> u64 {
    debug_assert!(!query.is_null());
    debug_assert!(!out.is_null());

    *out = try_ffi!(Box::from_raw(query).cost());

    0
}

// QueryCryptoGetAccountBalance
// ----------------------------------------------------------------------------

#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn hedera_query__get_account_balance__new(
    client: *mut Client,
    account: AccountId,
) -> *mut Query<QueryCryptoGetAccountBalanceResponse> {
    debug_assert!(!client.is_null());

    let client = Box::from_raw(client);

    let query = Query::get_account_balance(&*client, account);
    let query = Box::new(query);

    mem::forget(client);

    Box::into_raw(query)
}

impl_get!(hedera_query__get_account_balance__get(
    QueryCryptoGetAccountBalanceResponse
));

// QueryGetTransactionReceipt
// ----------------------------------------------------------------------------

#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn hedera_query__get_transaction_receipt__new(
    client: *mut Client,
    transaction_id: CTransactionId,
) -> *mut Query<QueryGetTransactionReceiptResponse> {
    debug_assert!(!client.is_null());

    let client = Box::from_raw(client);

    let query = Query::get_transaction_receipt(&*client, transaction_id.into());
    let query = Box::new(query);

    mem::forget(client);

    Box::into_raw(query)
}

impl_get!(hedera_query__get_transaction_receipt__get(
    QueryGetTransactionReceiptResponse
));
