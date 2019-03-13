use evmc_sys as ffi;

/// EVMC address
pub type Address = ffi::evmc_address;

/// EVMC 32 bytes value (used for hashes)
pub type Bytes32 = ffi::evmc_bytes32;

/// EVMC big-endian 256-bit integer
pub type Uint256 = ffi::evmc_uint256be;

/// EVMC message (call) kind.
pub type MessageKind = ffi::evmc_call_kind;

/// EVMC message (call) flags.
pub type MessageFlags = ffi::evmc_flags;

/// EVMC status code.
pub type StatusCode = ffi::evmc_status_code;

/// EVMC storage status.
pub type StorageStatus = ffi::evmc_storage_status;

/// EVMC VM revision.
pub type Revision = ffi::evmc_revision;

#[cfg(test)]
mod tests {
    use super::*;

    // These tests check for Default, PartialEq and Clone traits.
    #[test]
    fn address_smoke_test() {
        let a = ffi::evmc_address::default();
        let b = Address::default();
        assert_eq!(a.clone(), b.clone());
    }

    #[test]
    fn bytes32_smoke_test() {
        let a = ffi::evmc_bytes32::default();
        let b = Bytes32::default();
        assert_eq!(a.clone(), b.clone());
    }

    #[test]
    fn uint26be_smoke_test() {
        let a = ffi::evmc_uint256be::default();
        let b = Uint256::default();
        assert_eq!(a.clone(), b.clone());
    }

    #[test]
    fn message_kind() {
        assert_eq!(MessageKind::EVMC_CALL, ffi::evmc_call_kind::EVMC_CALL);
        assert_eq!(
            MessageKind::EVMC_CALLCODE,
            ffi::evmc_call_kind::EVMC_CALLCODE
        );
        assert_eq!(
            MessageKind::EVMC_DELEGATECALL,
            ffi::evmc_call_kind::EVMC_DELEGATECALL
        );
        assert_eq!(MessageKind::EVMC_CREATE, ffi::evmc_call_kind::EVMC_CREATE);
    }

    #[test]
    fn message_flags() {
        assert_eq!(MessageFlags::EVMC_STATIC, ffi::evmc_flags::EVMC_STATIC);
    }

    #[test]
    fn status_code() {}

    #[test]
    fn storage_status() {}

    #[test]
    fn revision() {
        assert_eq!(Revision::EVMC_FRONTIER, ffi::evmc_revision::EVMC_FRONTIER);
        assert_eq!(Revision::EVMC_ISTANBUL, ffi::evmc_revision::EVMC_ISTANBUL);
    }
}
