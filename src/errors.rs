//! The `rusty_vault::errors` module defines an enumeration of various error code, and implements
//! neccessary traits against it.
//!
//! The error code defined in this module are used widely in RustyVault.

use std::{
    io,
    sync::{PoisonError, RwLockReadGuard, RwLockWriteGuard},
};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum RvError {
    #[error("Config path is invalid.")]
    ErrConfigPathInvalid,
    #[error("Config load failed.")]
    ErrConfigLoadFailed,
    #[error("Config storage not found.")]
    ErrConfigStorageNotFound,
    #[error("Config listener not found.")]
    ErrConfigListenerNotFound,
    #[error("Core is not initialized.")]
    ErrCoreNotInit,
    #[error("Core logical backend already exists.")]
    ErrCoreLogicalBackendExist,
    #[error("Core logical backend does not exist.")]
    ErrCoreLogicalBackendNoExist,
    #[error("Core router not handling.")]
    ErrCoreRouterNotHandling,
    #[error("Core handler already exists.")]
    ErrCoreHandlerExist,
    #[error("Core seal config is invalid.")]
    ErrCoreSealConfigInvalid,
    #[error("Core seal config not found.")]
    ErrCoreSealConfigNotFound,
    #[error("Physical configuration item is missing.")]
    ErrPhysicalConfigItemMissing,
    #[error("Physical type is invalid.")]
    ErrPhysicalTypeInvalid,
    #[error("Physical backend prefix is invalid.")]
    ErrPhysicalBackendPrefixInvalid,
    #[error("Physical backend key is invalid.")]
    ErrPhysicalBackendKeyInvalid,
    #[error("Barrier key sanity check failed.")]
    ErrBarrierKeySanityCheckFailed,
    #[error("Barrier has been initialized.")]
    ErrBarrierAlreadyInit,
    #[error("Barrier key is invalid.")]
    ErrBarrierKeyInvalid,
    #[error("Barrier is not initialized.")]
    ErrBarrierNotInit,
    #[error("Barrier has been sealed.")]
    ErrBarrierSealed,
    #[error("Barrier has been unsealed.")]
    ErrBarrierUnsealed,
    #[error("Barrier unseal failed.")]
    ErrBarrierUnsealFailed,
    #[error("Barrier epoch do not match.")]
    ErrBarrierEpochMismatch,
    #[error("Barrier version do not match.")]
    ErrBarrierVersionMismatch,
    #[error("Barrier key generation failed.")]
    ErrBarrierKeyGenerationFailed,
    #[error("Router mount conflict.")]
    ErrRouterMountConflict,
    #[error("Router mount not found.")]
    ErrRouterMountNotFound,
    #[error("Mount path is failed, cannot mount.")]
    ErrMountFailed,
    #[error("Mount path is protected, cannot mount.")]
    ErrMountPathProtected,
    #[error("Mount path already exists.")]
    ErrMountPathExist,
    #[error("Mount table not found.")]
    ErrMountTableNotFound,
    #[error("Mount table is not ready.")]
    ErrMountTableNotReady,
    #[error("Mount not match.")]
    ErrMountNotMatch,
    #[error("Logical backend path not supported.")]
    ErrLogicalPathUnsupported,
    #[error("Logical backend operation not supported.")]
    ErrLogicalOperationUnsupported,
    #[error("Request is not ready.")]
    ErrRequestNotReady,
    #[error("No data is available for the request.")]
    ErrRequestNoData,
    #[error("No data field is available for the request.")]
    ErrRequestNoDataField,
    #[error("Request is invalid.")]
    ErrRequestInvalid,
    #[error("Request client token is missing.")]
    ErrRequestClientTokenMissing,
    #[error("Request field is not found.")]
    ErrRequestFieldNotFound,
    #[error("Request field is invalid.")]
    ErrRequestFieldInvalid,
    #[error("Handler is default.")]
    ErrHandlerDefault,
    #[error("Module kv data field is missing.")]
    ErrModuleKvDataFieldMissing,
    #[error("Rust downcast failed.")]
    ErrRustDowncastFailed,
    #[error("Shamir share count invalid.")]
    ErrShamirShareCountInvalid,
    #[error("Module conflict.")]
    ErrModuleConflict,
    #[error("Module is not init.")]
    ErrModuleNotInit,
    #[error("Auth module is disabled.")]
    ErrAuthModuleDisabled,
    #[error("Auth token is not found.")]
    ErrAuthTokenNotFound,
    #[error("Auth token id is invalid.")]
    ErrAuthTokenIdInvalid,
    #[error("Lease is not found.")]
    ErrLeaseNotFound,
    #[error("Lease is not renewable.")]
    ErrLeaseNotRenewable,
    #[error("Permission denied.")]
    ErrPermissionDenied,
    #[error("PKI pem bundle is invalid.")]
    ErrPkiPemBundleInvalid,
    #[error("PKI ca public key of certificate does not match private key.")]
    ErrPkiCertKeyMismatch,
    #[error("PKI cert chain is incorrect.")]
    ErrPkiCertChainIncorrect,
    #[error("PKI cert is not ca.")]
    ErrPkiCertIsNotCA,
    #[error("PKI ca private key is not found.")]
    ErrPkiCaKeyNotFound,
    #[error("PKI ca is not config.")]
    ErrPkiCaNotConfig,
    #[error("PKI ca extension is incorrect.")]
    ErrPkiCaExtensionIncorrect,
    #[error("PKI key type is invalid.")]
    ErrPkiKeyTypeInvalid,
    #[error("PKI key bits is invalid.")]
    ErrPkiKeyBitsInvalid,
    #[error("PKI key_name already exists.")]
    ErrPkiKeyNameAlreadyExist,
    #[error("PKI key operation is invalid.")]
    ErrPkiKeyOperationInvalid,
    #[error("PKI certificate is not found.")]
    ErrPkiCertNotFound,
    #[error("PKI role is not found.")]
    ErrPkiRoleNotFound,
    #[error("PKI data is invalid.")]
    ErrPkiDataInvalid,
    #[error("PKI internal error.")]
    ErrPkiInternal,
    #[error("Credentail is invalid.")]
    ErrCredentailInvalid,
    #[error("Credentail is not config.")]
    ErrCredentailNotConfig,
    #[error("Some IO error happened, {:?}", .source)]
    IO {
        #[from]
        source: io::Error,
    },
    #[error("Some serde error happened, {:?}", .source)]
    Serde {
        #[from]
        source: serde_json::Error,
    },
    #[error("Some openssl error happened, {:?}", .source)]
    OpenSSL {
        #[from]
        source: openssl::error::ErrorStack,
    },
    #[error("Some pem error happened, {:?}", .source)]
    Pem {
        #[from]
        source: pem::PemError,
    },
    #[error("Some regex error happened, {:?}", .source)]
    Regex {
        #[from]
        source: regex::Error,
    },
    #[error("Some hex error happened, {:?}", .source)]
    Hex {
        #[from]
        source: hex::FromHexError,
    },
    #[error("Some hcl error happened, {:?}", .source)]
    Hcl {
        #[from]
        source: hcl::Error,
    },
    #[error("Some humantime duration error happened, {:?}", .source)]
    HumantimeDuration {
        #[from]
        source: humantime::DurationError,
    },
    #[error("Some humantime timestamp error happened, {:?}", .source)]
    HumantimeTimestamp {
        #[from]
        source: humantime::TimestampError,
    },
    #[error("Some system_time error happened, {:?}", .source)]
    SystemTimeError {
        #[from]
        source: std::time::SystemTimeError,
    },
    #[error("Some chrono error happened, {:?}", .source)]
    ChronoError {
        #[from]
        source: chrono::ParseError,
    },
    #[error("Some delay_timer error happened, {:?}", .source)]
    TaskError {
        #[from]
        source: delay_timer::error::TaskError,
    },
    #[error("Some bcrypt error happened, {:?}", .source)]
    BcryptError {
        #[from]
        source: bcrypt::BcryptError,
    },
    #[error("Some ureq error happened, {:?}", .source)]
    UreqError {
        #[from]
        source: ureq::Error,
    },
    #[error("RwLock was poisoned (reading)")]
    ErrRwLockReadPoison,
    #[error("RwLock was poisoned (writing)")]
    ErrRwLockWritePoison,

    #[error("Some net addr parse error happened, {:?}", .source)]
    AddrParseError {
        #[from]
        source: std::net::AddrParseError,
    },
    #[error("Some ipnetwork error happened, {:?}", .source)]
    IpNetworkError {
        #[from]
        source: ipnetwork::IpNetworkError,
    },

    /// Database Errors Begin
    ///
    #[error("Database type is not support now. Please try postgressql or mysql again.")]
    ErrDatabaseTypeInvalid,
    #[cfg(feature = "storage_mysql")]
    #[error("Database connection pool ocurrs errors when creating， {:?}", .source)]
    ErrConnectionPoolCreate {
        #[from]
        source: r2d2::Error,
    },
    #[error("Database connection info is invalid.")]
    ErrDatabaseConnectionInfoInvalid,
    #[cfg(feature = "storage_mysql")]
    #[error("Failed to execute entry with database, {:?}", .source)]
    ErrDatabaseExecuteEntry {
        #[from]
        source: diesel::result::Error,
    },
    ///
    /// Database Errors End

    #[error(transparent)]
    ErrOther(#[from] anyhow::Error),
    #[error("Some error happend, response text: {0}")]
    ErrResponse(String),
    #[error("Some error happend, status: {0}, response text: {1}")]
    ErrResponseStatus(u16, String),
    #[error("Unknown error.")]
    ErrUnknown,
}

impl PartialEq for RvError {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (RvError::ErrCoreLogicalBackendExist, RvError::ErrCoreLogicalBackendExist)
            | (RvError::ErrCoreNotInit, RvError::ErrCoreNotInit)
            | (RvError::ErrCoreLogicalBackendNoExist, RvError::ErrCoreLogicalBackendNoExist)
            | (RvError::ErrCoreSealConfigInvalid, RvError::ErrCoreSealConfigInvalid)
            | (RvError::ErrCoreSealConfigNotFound, RvError::ErrCoreSealConfigNotFound)
            | (RvError::ErrCoreRouterNotHandling, RvError::ErrCoreRouterNotHandling)
            | (RvError::ErrCoreHandlerExist, RvError::ErrCoreHandlerExist)
            | (RvError::ErrPhysicalConfigItemMissing, RvError::ErrPhysicalConfigItemMissing)
            | (RvError::ErrPhysicalTypeInvalid, RvError::ErrPhysicalTypeInvalid)
            | (RvError::ErrPhysicalBackendPrefixInvalid, RvError::ErrPhysicalBackendPrefixInvalid)
            | (RvError::ErrPhysicalBackendKeyInvalid, RvError::ErrPhysicalBackendKeyInvalid)
            | (RvError::ErrBarrierKeySanityCheckFailed, RvError::ErrBarrierKeySanityCheckFailed)
            | (RvError::ErrBarrierAlreadyInit, RvError::ErrBarrierAlreadyInit)
            | (RvError::ErrBarrierKeyInvalid, RvError::ErrBarrierKeyInvalid)
            | (RvError::ErrBarrierNotInit, RvError::ErrBarrierNotInit)
            | (RvError::ErrBarrierSealed, RvError::ErrBarrierSealed)
            | (RvError::ErrBarrierUnsealed, RvError::ErrBarrierUnsealed)
            | (RvError::ErrBarrierUnsealFailed, RvError::ErrBarrierUnsealFailed)
            | (RvError::ErrBarrierEpochMismatch, RvError::ErrBarrierEpochMismatch)
            | (RvError::ErrBarrierVersionMismatch, RvError::ErrBarrierVersionMismatch)
            | (RvError::ErrBarrierKeyGenerationFailed, RvError::ErrBarrierKeyGenerationFailed)
            | (RvError::ErrRouterMountConflict, RvError::ErrRouterMountConflict)
            | (RvError::ErrRouterMountNotFound, RvError::ErrRouterMountNotFound)
            | (RvError::ErrMountFailed, RvError::ErrMountFailed)
            | (RvError::ErrMountPathProtected, RvError::ErrMountPathProtected)
            | (RvError::ErrMountPathExist, RvError::ErrMountPathExist)
            | (RvError::ErrMountTableNotFound, RvError::ErrMountTableNotFound)
            | (RvError::ErrMountTableNotReady, RvError::ErrMountTableNotReady)
            | (RvError::ErrMountNotMatch, RvError::ErrMountNotMatch)
            | (RvError::ErrLogicalPathUnsupported, RvError::ErrLogicalPathUnsupported)
            | (RvError::ErrLogicalOperationUnsupported, RvError::ErrLogicalOperationUnsupported)
            | (RvError::ErrRequestNotReady, RvError::ErrRequestNotReady)
            | (RvError::ErrRequestNoData, RvError::ErrRequestNoData)
            | (RvError::ErrRequestNoDataField, RvError::ErrRequestNoDataField)
            | (RvError::ErrRequestInvalid, RvError::ErrRequestInvalid)
            | (RvError::ErrRequestClientTokenMissing, RvError::ErrRequestClientTokenMissing)
            | (RvError::ErrRequestFieldNotFound, RvError::ErrRequestFieldNotFound)
            | (RvError::ErrRequestFieldInvalid, RvError::ErrRequestFieldInvalid)
            | (RvError::ErrHandlerDefault, RvError::ErrHandlerDefault)
            | (RvError::ErrModuleKvDataFieldMissing, RvError::ErrModuleKvDataFieldMissing)
            | (RvError::ErrRustDowncastFailed, RvError::ErrRustDowncastFailed)
            | (RvError::ErrShamirShareCountInvalid, RvError::ErrShamirShareCountInvalid)
            | (RvError::ErrRwLockReadPoison, RvError::ErrRwLockReadPoison)
            | (RvError::ErrRwLockWritePoison, RvError::ErrRwLockWritePoison)
            | (RvError::ErrConfigPathInvalid, RvError::ErrConfigPathInvalid)
            | (RvError::ErrConfigLoadFailed, RvError::ErrConfigLoadFailed)
            | (RvError::ErrConfigStorageNotFound, RvError::ErrConfigStorageNotFound)
            | (RvError::ErrConfigListenerNotFound, RvError::ErrConfigListenerNotFound)
            | (RvError::ErrModuleConflict, RvError::ErrModuleConflict)
            | (RvError::ErrModuleNotInit, RvError::ErrModuleNotInit)
            | (RvError::ErrAuthModuleDisabled, RvError::ErrAuthModuleDisabled)
            | (RvError::ErrAuthTokenNotFound, RvError::ErrAuthTokenNotFound)
            | (RvError::ErrAuthTokenIdInvalid, RvError::ErrAuthTokenIdInvalid)
            | (RvError::ErrLeaseNotFound, RvError::ErrLeaseNotFound)
            | (RvError::ErrLeaseNotRenewable, RvError::ErrLeaseNotRenewable)
            | (RvError::ErrPermissionDenied, RvError::ErrPermissionDenied)
            | (RvError::ErrPkiPemBundleInvalid, RvError::ErrPkiPemBundleInvalid)
            | (RvError::ErrPkiCertKeyMismatch, RvError::ErrPkiCertKeyMismatch)
            | (RvError::ErrPkiCertChainIncorrect, RvError::ErrPkiCertChainIncorrect)
            | (RvError::ErrPkiCertIsNotCA, RvError::ErrPkiCertIsNotCA)
            | (RvError::ErrPkiCaKeyNotFound, RvError::ErrPkiCaKeyNotFound)
            | (RvError::ErrPkiCaNotConfig, RvError::ErrPkiCaNotConfig)
            | (RvError::ErrPkiCaExtensionIncorrect, RvError::ErrPkiCaExtensionIncorrect)
            | (RvError::ErrPkiKeyTypeInvalid, RvError::ErrPkiKeyTypeInvalid)
            | (RvError::ErrPkiKeyBitsInvalid, RvError::ErrPkiKeyBitsInvalid)
            | (RvError::ErrPkiKeyNameAlreadyExist, RvError::ErrPkiKeyNameAlreadyExist)
            | (RvError::ErrPkiKeyOperationInvalid, RvError::ErrPkiKeyOperationInvalid)
            | (RvError::ErrPkiCertNotFound, RvError::ErrPkiCertNotFound)
            | (RvError::ErrPkiRoleNotFound, RvError::ErrPkiRoleNotFound)
            | (RvError::ErrPkiDataInvalid, RvError::ErrPkiDataInvalid)
            | (RvError::ErrPkiInternal, RvError::ErrPkiInternal)
            | (RvError::ErrCredentailInvalid, RvError::ErrCredentailInvalid)
            | (RvError::ErrCredentailNotConfig, RvError::ErrCredentailNotConfig)
            | (RvError::ErrUnknown, RvError::ErrUnknown) => true,
            _ => false,
        }
    }
}

impl<T> From<PoisonError<RwLockWriteGuard<'_, T>>> for RvError {
    fn from(_: PoisonError<RwLockWriteGuard<'_, T>>) -> Self {
        RvError::ErrRwLockWritePoison
    }
}

impl<T> From<PoisonError<RwLockReadGuard<'_, T>>> for RvError {
    fn from(_: PoisonError<RwLockReadGuard<'_, T>>) -> Self {
        RvError::ErrRwLockReadPoison
    }
}
