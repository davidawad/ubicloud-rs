#[allow(clippy::result_large_err)]
mod progenitor_client;

#[allow(unused_imports)]
pub use progenitor_client::{ByteStream, ClientInfo, Error, ResponseValue};
#[allow(unused_imports)]
use progenitor_client::{ClientHooks, OperationInfo, RequestBuilderExt, encode_path};
/// Types used as operation parameters and responses.
#[allow(clippy::all)]
pub mod types {
    ///`ActionLocationFirewallAttachSubnetBody`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct ActionLocationFirewallAttachSubnetBody {
        ///Private subnet id to attach
        pub private_subnet_id: ::std::string::String,
    }

    ///`ActionLocationFirewallAttachSubnetProjectId`
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct ActionLocationFirewallAttachSubnetProjectId(::std::string::String);
    impl ::std::ops::Deref for ActionLocationFirewallAttachSubnetProjectId {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<ActionLocationFirewallAttachSubnetProjectId> for ::std::string::String {
        fn from(value: ActionLocationFirewallAttachSubnetProjectId) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for ActionLocationFirewallAttachSubnetProjectId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^pj[0-9a-hj-km-np-tv-z]{24}$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^pj[0-9a-hj-km-np-tv-z]{24}$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for ActionLocationFirewallAttachSubnetProjectId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for ActionLocationFirewallAttachSubnetProjectId
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for ActionLocationFirewallAttachSubnetProjectId {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`ActionLocationFirewallAttachSubnetResponse`
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    #[serde(deny_unknown_fields)]
    pub enum ActionLocationFirewallAttachSubnetResponse {}
    ///`ActionLocationFirewallDetachSubnetBody`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct ActionLocationFirewallDetachSubnetBody {
        ///Private subnet id to detach
        pub private_subnet_id: ::std::string::String,
    }

    ///`ActionLocationFirewallDetachSubnetProjectId`
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct ActionLocationFirewallDetachSubnetProjectId(::std::string::String);
    impl ::std::ops::Deref for ActionLocationFirewallDetachSubnetProjectId {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<ActionLocationFirewallDetachSubnetProjectId> for ::std::string::String {
        fn from(value: ActionLocationFirewallDetachSubnetProjectId) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for ActionLocationFirewallDetachSubnetProjectId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^pj[0-9a-hj-km-np-tv-z]{24}$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^pj[0-9a-hj-km-np-tv-z]{24}$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for ActionLocationFirewallDetachSubnetProjectId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for ActionLocationFirewallDetachSubnetProjectId
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for ActionLocationFirewallDetachSubnetProjectId {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`ActionLocationFirewallDetachSubnetResponse`
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    #[serde(deny_unknown_fields)]
    pub enum ActionLocationFirewallDetachSubnetResponse {}
    ///`AddCertAuthUserProjectId`
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct AddCertAuthUserProjectId(::std::string::String);
    impl ::std::ops::Deref for AddCertAuthUserProjectId {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<AddCertAuthUserProjectId> for ::std::string::String {
        fn from(value: AddCertAuthUserProjectId) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for AddCertAuthUserProjectId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^pj[0-9a-hj-km-np-tv-z]{24}$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^pj[0-9a-hj-km-np-tv-z]{24}$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for AddCertAuthUserProjectId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for AddCertAuthUserProjectId {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for AddCertAuthUserProjectId {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`AttachVmLocationLoadBalancerBody`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct AttachVmLocationLoadBalancerBody {
        ///Apid of VM to attach to Load Balancer
        pub vm_id: ::std::string::String,
    }

    ///`AttachVmLocationLoadBalancerProjectId`
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct AttachVmLocationLoadBalancerProjectId(::std::string::String);
    impl ::std::ops::Deref for AttachVmLocationLoadBalancerProjectId {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<AttachVmLocationLoadBalancerProjectId> for ::std::string::String {
        fn from(value: AttachVmLocationLoadBalancerProjectId) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for AttachVmLocationLoadBalancerProjectId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^pj[0-9a-hj-km-np-tv-z]{24}$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^pj[0-9a-hj-km-np-tv-z]{24}$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for AttachVmLocationLoadBalancerProjectId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for AttachVmLocationLoadBalancerProjectId {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for AttachVmLocationLoadBalancerProjectId {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`AttachVmLocationLoadBalancerResponse`
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    #[serde(deny_unknown_fields)]
    pub enum AttachVmLocationLoadBalancerResponse {}
    ///`AuditLog`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct AuditLog {
        ///Action performed
        pub action: ::std::string::String,
        ///Timestamp of the action
        pub at: ::chrono::DateTime<::chrono::offset::Utc>,
        ///IDs of the objects affected by the action
        pub object_ids: ::std::vec::Vec<::std::string::String>,
        ///ID of the account that performed the action
        pub subject_id: ::std::string::String,
        ///Name of the account that performed the action (if still member of
        /// project)
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub subject_name: ::std::option::Option<::std::string::String>,
    }

    ///`CertAuthUserList`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct CertAuthUserList {
        pub items: ::std::vec::Vec<::std::string::String>,
    }

    ///`CertAuthUserRequest`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct CertAuthUserRequest {
        ///name of user
        pub name: ::std::string::String,
    }

    ///`ConnectPrivateSubnetBody`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default)]
    #[serde(deny_unknown_fields)]
    pub struct ConnectPrivateSubnetBody {
        #[serde(
            rename = "connected-subnet-id",
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub connected_subnet_id: ::std::option::Option<::std::string::String>,
    }

    ///`ConnectPrivateSubnetProjectId`
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct ConnectPrivateSubnetProjectId(::std::string::String);
    impl ::std::ops::Deref for ConnectPrivateSubnetProjectId {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<ConnectPrivateSubnetProjectId> for ::std::string::String {
        fn from(value: ConnectPrivateSubnetProjectId) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for ConnectPrivateSubnetProjectId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^pj[0-9a-hj-km-np-tv-z]{24}$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^pj[0-9a-hj-km-np-tv-z]{24}$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for ConnectPrivateSubnetProjectId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for ConnectPrivateSubnetProjectId {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for ConnectPrivateSubnetProjectId {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`CreateClientCertKeypairBody`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default)]
    #[serde(deny_unknown_fields)]
    pub struct CreateClientCertKeypairBody {
        ///CN for cert
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub common_name: ::std::option::Option<::std::string::String>,
        ///Duration before expiry, in seconds
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub duration: ::std::option::Option<i64>,
    }

    ///`CreateClientCertKeypairProjectId`
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct CreateClientCertKeypairProjectId(::std::string::String);
    impl ::std::ops::Deref for CreateClientCertKeypairProjectId {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<CreateClientCertKeypairProjectId> for ::std::string::String {
        fn from(value: CreateClientCertKeypairProjectId) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for CreateClientCertKeypairProjectId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^pj[0-9a-hj-km-np-tv-z]{24}$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^pj[0-9a-hj-km-np-tv-z]{24}$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for CreateClientCertKeypairProjectId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for CreateClientCertKeypairProjectId {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for CreateClientCertKeypairProjectId {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`CreateInferenceApiKeyBody`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default)]
    #[serde(deny_unknown_fields)]
    pub struct CreateInferenceApiKeyBody {}
    ///`CreateInferenceApiKeyProjectId`
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct CreateInferenceApiKeyProjectId(::std::string::String);
    impl ::std::ops::Deref for CreateInferenceApiKeyProjectId {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<CreateInferenceApiKeyProjectId> for ::std::string::String {
        fn from(value: CreateInferenceApiKeyProjectId) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for CreateInferenceApiKeyProjectId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^pj[0-9a-hj-km-np-tv-z]{24}$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^pj[0-9a-hj-km-np-tv-z]{24}$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for CreateInferenceApiKeyProjectId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for CreateInferenceApiKeyProjectId {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for CreateInferenceApiKeyProjectId {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`CreateKubernetesClusterBody`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct CreateKubernetesClusterBody {
        ///Number of control plane nodes
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub cp_nodes: ::std::option::Option<i64>,
        ///Version of the kubernetes cluster
        pub version: ::std::string::String,
        ///Number of worker nodes
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub worker_nodes: ::std::option::Option<i64>,
        ///Size of each worker VM
        pub worker_size: ::std::string::String,
    }

    ///`CreateKubernetesClusterProjectId`
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct CreateKubernetesClusterProjectId(::std::string::String);
    impl ::std::ops::Deref for CreateKubernetesClusterProjectId {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<CreateKubernetesClusterProjectId> for ::std::string::String {
        fn from(value: CreateKubernetesClusterProjectId) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for CreateKubernetesClusterProjectId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^pj[0-9a-hj-km-np-tv-z]{24}$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^pj[0-9a-hj-km-np-tv-z]{24}$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for CreateKubernetesClusterProjectId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for CreateKubernetesClusterProjectId {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for CreateKubernetesClusterProjectId {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`CreateKubernetesClusterResponse`
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    #[serde(deny_unknown_fields)]
    pub enum CreateKubernetesClusterResponse {}
    ///`CreateKubernetesNodepoolBody`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct CreateKubernetesNodepoolBody {
        ///Name of the nodepool
        pub name: ::std::string::String,
        ///Number of worker nodes in the nodepool
        pub node_count: i64,
        ///Size of each worker node
        pub node_size: ::std::string::String,
    }

    ///`CreateKubernetesNodepoolProjectId`
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct CreateKubernetesNodepoolProjectId(::std::string::String);
    impl ::std::ops::Deref for CreateKubernetesNodepoolProjectId {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<CreateKubernetesNodepoolProjectId> for ::std::string::String {
        fn from(value: CreateKubernetesNodepoolProjectId) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for CreateKubernetesNodepoolProjectId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^pj[0-9a-hj-km-np-tv-z]{24}$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^pj[0-9a-hj-km-np-tv-z]{24}$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for CreateKubernetesNodepoolProjectId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for CreateKubernetesNodepoolProjectId {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for CreateKubernetesNodepoolProjectId {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`CreateKubernetesNodepoolResponse`
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    #[serde(deny_unknown_fields)]
    pub enum CreateKubernetesNodepoolResponse {}
    ///`CreateLocationFirewallBody`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default)]
    #[serde(deny_unknown_fields)]
    pub struct CreateLocationFirewallBody {
        ///Description of the firewall
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub description: ::std::option::Option<::std::string::String>,
    }

    ///`CreateLocationFirewallProjectId`
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct CreateLocationFirewallProjectId(::std::string::String);
    impl ::std::ops::Deref for CreateLocationFirewallProjectId {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<CreateLocationFirewallProjectId> for ::std::string::String {
        fn from(value: CreateLocationFirewallProjectId) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for CreateLocationFirewallProjectId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^pj[0-9a-hj-km-np-tv-z]{24}$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^pj[0-9a-hj-km-np-tv-z]{24}$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for CreateLocationFirewallProjectId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for CreateLocationFirewallProjectId {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for CreateLocationFirewallProjectId {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`CreateLocationFirewallRuleBody`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct CreateLocationFirewallRuleBody {
        ///CIDR or private subnet id or name to allow
        pub cidr: ::std::string::String,
        ///Description to use for the firewall rule
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub description: ::std::option::Option<::std::string::String>,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub firewall_id: ::std::option::Option<::std::string::String>,
        ///Port range to allow
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub port_range: ::std::option::Option<::std::string::String>,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub project_id: ::std::option::Option<::std::string::String>,
        ///Protocol to allow (tcp or udp, default tcp)
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub protocol: ::std::option::Option<::std::string::String>,
    }

    ///`CreateLocationFirewallRuleProjectId`
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct CreateLocationFirewallRuleProjectId(::std::string::String);
    impl ::std::ops::Deref for CreateLocationFirewallRuleProjectId {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<CreateLocationFirewallRuleProjectId> for ::std::string::String {
        fn from(value: CreateLocationFirewallRuleProjectId) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for CreateLocationFirewallRuleProjectId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^pj[0-9a-hj-km-np-tv-z]{24}$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^pj[0-9a-hj-km-np-tv-z]{24}$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for CreateLocationFirewallRuleProjectId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for CreateLocationFirewallRuleProjectId {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for CreateLocationFirewallRuleProjectId {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`CreateLocationFirewallRuleResponse`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum CreateLocationFirewallRuleResponse {
        FirewallRule(FirewallRule),
        Array(::std::vec::Vec<FirewallRule>),
    }

    impl ::std::convert::From<FirewallRule> for CreateLocationFirewallRuleResponse {
        fn from(value: FirewallRule) -> Self {
            Self::FirewallRule(value)
        }
    }

    impl ::std::convert::From<::std::vec::Vec<FirewallRule>> for CreateLocationFirewallRuleResponse {
        fn from(value: ::std::vec::Vec<FirewallRule>) -> Self {
            Self::Array(value)
        }
    }

    ///`CreateLocationLoadBalancerBody`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct CreateLocationLoadBalancerBody {
        ///Algorithm of the Load Balancer
        pub algorithm: ::std::string::String,
        ///Whether SSL certificates are enabled for the Load Balancer
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub cert_enabled: ::std::option::Option<bool>,
        ///Destination port for the Load Balancer
        pub dst_port: i64,
        ///Health check endpoint URL
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub health_check_endpoint: ::std::option::Option<::std::string::String>,
        ///Health check endpoint protocol
        pub health_check_protocol: ::std::string::String,
        ///ID of Private Subnet
        pub private_subnet_id: ::std::string::String,
        ///Source port for the Load Balancer
        pub src_port: i64,
        ///Networking stack of the Load Balancer (ipv4, ipv6, or dual)
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub stack: ::std::option::Option<::std::string::String>,
    }

    ///`CreateLocationLoadBalancerProjectId`
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct CreateLocationLoadBalancerProjectId(::std::string::String);
    impl ::std::ops::Deref for CreateLocationLoadBalancerProjectId {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<CreateLocationLoadBalancerProjectId> for ::std::string::String {
        fn from(value: CreateLocationLoadBalancerProjectId) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for CreateLocationLoadBalancerProjectId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^pj[0-9a-hj-km-np-tv-z]{24}$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^pj[0-9a-hj-km-np-tv-z]{24}$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for CreateLocationLoadBalancerProjectId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for CreateLocationLoadBalancerProjectId {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for CreateLocationLoadBalancerProjectId {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`CreateLocationLoadBalancerResponse`
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    #[serde(deny_unknown_fields)]
    pub enum CreateLocationLoadBalancerResponse {}
    ///`CreateLocationPostgresFirewallRuleBody`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct CreateLocationPostgresFirewallRuleBody {
        ///CIDR of the firewall rule
        pub cidr: ::std::string::String,
        ///Description of the firewall rule
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub description: ::std::option::Option<::std::string::String>,
    }

    ///`CreateLocationPostgresFirewallRuleProjectId`
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct CreateLocationPostgresFirewallRuleProjectId(::std::string::String);
    impl ::std::ops::Deref for CreateLocationPostgresFirewallRuleProjectId {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<CreateLocationPostgresFirewallRuleProjectId> for ::std::string::String {
        fn from(value: CreateLocationPostgresFirewallRuleProjectId) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for CreateLocationPostgresFirewallRuleProjectId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^pj[0-9a-hj-km-np-tv-z]{24}$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^pj[0-9a-hj-km-np-tv-z]{24}$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for CreateLocationPostgresFirewallRuleProjectId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for CreateLocationPostgresFirewallRuleProjectId
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for CreateLocationPostgresFirewallRuleProjectId {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`CreateLocationPostgresLogDestinationBody`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct CreateLocationPostgresLogDestinationBody {
        ///Name for the Log Destination
        pub name: ::std::string::String,
        ///Authentication options. For otlp: {"headers": {"Header-Name":
        /// "value"}}. For syslog: {"structured_data": {"sd-id": {"key":
        /// "value"}}}
        #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
        pub options: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ///Destination type. "otlp" sends via OTLP HTTP (https://...); "syslog" sends via RFC 5424 over TLS (tcp://host:port)
        #[serde(rename = "type")]
        pub type_: CreateLocationPostgresLogDestinationBodyType,
        ///Endpoint URL. For otlp: https://... For syslog: tcp://host:port
        pub url: ::std::string::String,
    }

    ///Destination type. "otlp" sends via OTLP HTTP (https://...); "syslog" sends via RFC 5424 over TLS (tcp://host:port)
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum CreateLocationPostgresLogDestinationBodyType {
        #[serde(rename = "otlp")]
        Otlp,
        #[serde(rename = "syslog")]
        Syslog,
    }

    impl ::std::fmt::Display for CreateLocationPostgresLogDestinationBodyType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Otlp => f.write_str("otlp"),
                Self::Syslog => f.write_str("syslog"),
            }
        }
    }

    impl ::std::str::FromStr for CreateLocationPostgresLogDestinationBodyType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "otlp" => Ok(Self::Otlp),
                "syslog" => Ok(Self::Syslog),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for CreateLocationPostgresLogDestinationBodyType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for CreateLocationPostgresLogDestinationBodyType
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`CreateLocationPostgresLogDestinationProjectId`
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct CreateLocationPostgresLogDestinationProjectId(::std::string::String);
    impl ::std::ops::Deref for CreateLocationPostgresLogDestinationProjectId {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<CreateLocationPostgresLogDestinationProjectId> for ::std::string::String {
        fn from(value: CreateLocationPostgresLogDestinationProjectId) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for CreateLocationPostgresLogDestinationProjectId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^pj[0-9a-hj-km-np-tv-z]{24}$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^pj[0-9a-hj-km-np-tv-z]{24}$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for CreateLocationPostgresLogDestinationProjectId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for CreateLocationPostgresLogDestinationProjectId
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for CreateLocationPostgresLogDestinationProjectId {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`CreateLocationPostgresMetricDestinationBody`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct CreateLocationPostgresMetricDestinationBody {
        ///Additional Prometheus remote_write authentication, as
        /// {"authorization": {"type": "Bearer", "credentials": "token"}} and/or
        /// {"headers": {"Header-Name": "value"}}
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub options:
            ::std::option::Option<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        ///Password for basic auth, required with username
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub password: ::std::option::Option<::std::string::String>,
        ///URL for the Metric Destination
        pub url: ::std::string::String,
        ///Username for basic auth, required with password
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub username: ::std::option::Option<::std::string::String>,
    }

    ///`CreateLocationPostgresMetricDestinationProjectId`
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct CreateLocationPostgresMetricDestinationProjectId(::std::string::String);
    impl ::std::ops::Deref for CreateLocationPostgresMetricDestinationProjectId {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<CreateLocationPostgresMetricDestinationProjectId>
        for ::std::string::String
    {
        fn from(value: CreateLocationPostgresMetricDestinationProjectId) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for CreateLocationPostgresMetricDestinationProjectId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^pj[0-9a-hj-km-np-tv-z]{24}$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^pj[0-9a-hj-km-np-tv-z]{24}$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for CreateLocationPostgresMetricDestinationProjectId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for CreateLocationPostgresMetricDestinationProjectId
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for CreateLocationPostgresMetricDestinationProjectId {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`CreateLocationPostgresMetricDestinationResponse`
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    #[serde(deny_unknown_fields)]
    pub enum CreateLocationPostgresMetricDestinationResponse {}
    ///`CreateMachineImageBody`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct CreateMachineImageBody {
        ///Destroy the source VM after capture
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub destroy_source: ::std::option::Option<bool>,
        ///Version label (default is a timestamp)
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub version: ::std::option::Option<::std::string::String>,
        ///UBID of the source VM
        pub vm: ::std::string::String,
    }

    ///`CreateMachineImageMachineImageReference`
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct CreateMachineImageMachineImageReference(::std::string::String);
    impl ::std::ops::Deref for CreateMachineImageMachineImageReference {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<CreateMachineImageMachineImageReference> for ::std::string::String {
        fn from(value: CreateMachineImageMachineImageReference) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for CreateMachineImageMachineImageReference {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new(
                        "^(?!m1[0-9a-hj-km-np-tv-z]{24}$)[a-z0-9](?:[a-z0-9\\-]{0,61}[a-z0-9])?$",
                    )
                    .unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err ("doesn't match pattern \"^(?!m1[0-9a-hj-km-np-tv-z]{24}$)[a-z0-9](?:[a-z0-9\\-]{0,61}[a-z0-9])?$\"" . into ()) ;
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for CreateMachineImageMachineImageReference {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for CreateMachineImageMachineImageReference {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for CreateMachineImageMachineImageReference {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`CreateMachineImageProjectId`
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct CreateMachineImageProjectId(::std::string::String);
    impl ::std::ops::Deref for CreateMachineImageProjectId {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<CreateMachineImageProjectId> for ::std::string::String {
        fn from(value: CreateMachineImageProjectId) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for CreateMachineImageProjectId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^pj[0-9a-hj-km-np-tv-z]{24}$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^pj[0-9a-hj-km-np-tv-z]{24}$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for CreateMachineImageProjectId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for CreateMachineImageProjectId {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for CreateMachineImageProjectId {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`CreateMachineImageVersionBody`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct CreateMachineImageVersionBody {
        ///Destroy the source VM after capture
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub destroy_source: ::std::option::Option<bool>,
        ///UBID of the source VM
        pub vm: ::std::string::String,
    }

    ///`CreateMachineImageVersionProjectId`
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct CreateMachineImageVersionProjectId(::std::string::String);
    impl ::std::ops::Deref for CreateMachineImageVersionProjectId {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<CreateMachineImageVersionProjectId> for ::std::string::String {
        fn from(value: CreateMachineImageVersionProjectId) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for CreateMachineImageVersionProjectId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^pj[0-9a-hj-km-np-tv-z]{24}$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^pj[0-9a-hj-km-np-tv-z]{24}$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for CreateMachineImageVersionProjectId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for CreateMachineImageVersionProjectId {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for CreateMachineImageVersionProjectId {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`CreateMachineImageVersionVersion`
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct CreateMachineImageVersionVersion(::std::string::String);
    impl ::std::ops::Deref for CreateMachineImageVersionVersion {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<CreateMachineImageVersionVersion> for ::std::string::String {
        fn from(value: CreateMachineImageVersionVersion) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for CreateMachineImageVersionVersion {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^[a-zA-Z0-9][a-zA-Z0-9._-]{0,63}$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^[a-zA-Z0-9][a-zA-Z0-9._-]{0,63}$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for CreateMachineImageVersionVersion {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for CreateMachineImageVersionVersion {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for CreateMachineImageVersionVersion {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`CreatePostgresDatabaseBackupCredentialsProjectId`
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct CreatePostgresDatabaseBackupCredentialsProjectId(::std::string::String);
    impl ::std::ops::Deref for CreatePostgresDatabaseBackupCredentialsProjectId {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<CreatePostgresDatabaseBackupCredentialsProjectId>
        for ::std::string::String
    {
        fn from(value: CreatePostgresDatabaseBackupCredentialsProjectId) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for CreatePostgresDatabaseBackupCredentialsProjectId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^pj[0-9a-hj-km-np-tv-z]{24}$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^pj[0-9a-hj-km-np-tv-z]{24}$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for CreatePostgresDatabaseBackupCredentialsProjectId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for CreatePostgresDatabaseBackupCredentialsProjectId
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for CreatePostgresDatabaseBackupCredentialsProjectId {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`CreatePostgresDatabaseBody`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct CreatePostgresDatabaseBody {
        ///Deprecated. standard is the only flavor and the default, so omit
        /// this.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub flavor: ::std::option::Option<::std::string::String>,
        ///High availability type
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub ha_type: ::std::option::Option<::std::string::String>,
        #[serde(
            default,
            skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
        )]
        pub pg_config: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
        #[serde(
            default,
            skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
        )]
        pub pgbouncer_config:
            ::std::collections::HashMap<::std::string::String, ::std::string::String>,
        ///Name for the private subnet (if not provided, a name will be
        /// auto-generated)
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub private_subnet_name: ::std::option::Option<::std::string::String>,
        ///Whether to restrict access by default (if so, firewall rules must be
        /// added to access)
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub restrict_by_default: ::std::option::Option<bool>,
        ///Requested size for the underlying VM
        pub size: ::std::string::String,
        ///Requested storage size in GiB
        pub storage_size: i64,
        ///Tags for the Postgres Database
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub tags: ::std::vec::Vec<PostgresTag>,
        ///PostgreSQL version
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub version: ::std::option::Option<CreatePostgresDatabaseBodyVersion>,
    }

    ///PostgreSQL version
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum CreatePostgresDatabaseBodyVersion {
        #[serde(rename = "16")]
        X16,
        #[serde(rename = "17")]
        X17,
        #[serde(rename = "18")]
        X18,
    }

    impl ::std::fmt::Display for CreatePostgresDatabaseBodyVersion {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::X16 => f.write_str("16"),
                Self::X17 => f.write_str("17"),
                Self::X18 => f.write_str("18"),
            }
        }
    }

    impl ::std::str::FromStr for CreatePostgresDatabaseBodyVersion {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "16" => Ok(Self::X16),
                "17" => Ok(Self::X17),
                "18" => Ok(Self::X18),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for CreatePostgresDatabaseBodyVersion {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for CreatePostgresDatabaseBodyVersion {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`CreatePostgresDatabaseProjectId`
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct CreatePostgresDatabaseProjectId(::std::string::String);
    impl ::std::ops::Deref for CreatePostgresDatabaseProjectId {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<CreatePostgresDatabaseProjectId> for ::std::string::String {
        fn from(value: CreatePostgresDatabaseProjectId) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for CreatePostgresDatabaseProjectId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^pj[0-9a-hj-km-np-tv-z]{24}$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^pj[0-9a-hj-km-np-tv-z]{24}$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for CreatePostgresDatabaseProjectId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for CreatePostgresDatabaseProjectId {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for CreatePostgresDatabaseProjectId {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`CreatePostgresDatabaseReadReplicaBody`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct CreatePostgresDatabaseReadReplicaBody {
        ///Name for the read replica
        pub name: ::std::string::String,
        #[serde(
            default,
            skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
        )]
        pub pg_config: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
        #[serde(
            default,
            skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
        )]
        pub pgbouncer_config:
            ::std::collections::HashMap<::std::string::String, ::std::string::String>,
        ///Tags for the read replica
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub tags: ::std::vec::Vec<PostgresTag>,
    }

    ///`CreatePostgresDatabaseReadReplicaProjectId`
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct CreatePostgresDatabaseReadReplicaProjectId(::std::string::String);
    impl ::std::ops::Deref for CreatePostgresDatabaseReadReplicaProjectId {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<CreatePostgresDatabaseReadReplicaProjectId> for ::std::string::String {
        fn from(value: CreatePostgresDatabaseReadReplicaProjectId) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for CreatePostgresDatabaseReadReplicaProjectId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^pj[0-9a-hj-km-np-tv-z]{24}$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^pj[0-9a-hj-km-np-tv-z]{24}$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for CreatePostgresDatabaseReadReplicaProjectId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for CreatePostgresDatabaseReadReplicaProjectId {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for CreatePostgresDatabaseReadReplicaProjectId {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`CreatePostgresDatabaseReadReplicaResponse`
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    #[serde(deny_unknown_fields)]
    pub enum CreatePostgresDatabaseReadReplicaResponse {}
    ///`CreatePostgresDatabaseResponse`
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    #[serde(deny_unknown_fields)]
    pub enum CreatePostgresDatabaseResponse {}
    ///`CreatePrivateLocationBody`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct CreatePrivateLocationBody {
        ///Access key for the private location
        pub access_key: ::std::string::String,
        ///Name of the private location
        pub name: ::std::string::String,
        ///Provider location name of the private location
        pub provider_location_name: CreatePrivateLocationBodyProviderLocationName,
        ///Secret key for the private location
        pub secret_key: ::std::string::String,
    }

    ///Provider location name of the private location
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum CreatePrivateLocationBodyProviderLocationName {
        #[serde(rename = "us-west-2")]
        UsWest2,
        #[serde(rename = "us-east-1")]
        UsEast1,
        #[serde(rename = "us-east-2")]
        UsEast2,
        #[serde(rename = "ap-southeast-2")]
        ApSoutheast2,
        #[serde(rename = "eu-west-1")]
        EuWest1,
    }

    impl ::std::fmt::Display for CreatePrivateLocationBodyProviderLocationName {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::UsWest2 => f.write_str("us-west-2"),
                Self::UsEast1 => f.write_str("us-east-1"),
                Self::UsEast2 => f.write_str("us-east-2"),
                Self::ApSoutheast2 => f.write_str("ap-southeast-2"),
                Self::EuWest1 => f.write_str("eu-west-1"),
            }
        }
    }

    impl ::std::str::FromStr for CreatePrivateLocationBodyProviderLocationName {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "us-west-2" => Ok(Self::UsWest2),
                "us-east-1" => Ok(Self::UsEast1),
                "us-east-2" => Ok(Self::UsEast2),
                "ap-southeast-2" => Ok(Self::ApSoutheast2),
                "eu-west-1" => Ok(Self::EuWest1),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for CreatePrivateLocationBodyProviderLocationName {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for CreatePrivateLocationBodyProviderLocationName
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`CreatePrivateLocationProjectId`
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct CreatePrivateLocationProjectId(::std::string::String);
    impl ::std::ops::Deref for CreatePrivateLocationProjectId {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<CreatePrivateLocationProjectId> for ::std::string::String {
        fn from(value: CreatePrivateLocationProjectId) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for CreatePrivateLocationProjectId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^pj[0-9a-hj-km-np-tv-z]{24}$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^pj[0-9a-hj-km-np-tv-z]{24}$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for CreatePrivateLocationProjectId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for CreatePrivateLocationProjectId {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for CreatePrivateLocationProjectId {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`CreatePrivateSubnetBody`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default)]
    #[serde(deny_unknown_fields)]
    pub struct CreatePrivateSubnetBody {
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub firewall_id: ::std::option::Option<::std::string::String>,
    }

    ///`CreatePrivateSubnetProjectId`
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct CreatePrivateSubnetProjectId(::std::string::String);
    impl ::std::ops::Deref for CreatePrivateSubnetProjectId {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<CreatePrivateSubnetProjectId> for ::std::string::String {
        fn from(value: CreatePrivateSubnetProjectId) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for CreatePrivateSubnetProjectId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^pj[0-9a-hj-km-np-tv-z]{24}$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^pj[0-9a-hj-km-np-tv-z]{24}$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for CreatePrivateSubnetProjectId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for CreatePrivateSubnetProjectId {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for CreatePrivateSubnetProjectId {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`CreateVmBody`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct CreateVmBody {
        ///Boot image of the VM
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub boot_image: ::std::option::Option<::std::string::String>,
        ///Enable IPv4
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub enable_ip4: ::std::option::Option<bool>,
        ///Requested GPU count and type, in the form of "count:type"
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub gpu: ::std::option::Option<::std::string::String>,
        ///VM init script
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub init_script: ::std::option::Option<::std::string::String>,
        ///ID of the private subnet
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub private_subnet_id: ::std::option::Option<::std::string::String>,
        ///Public SSH key for the VM, or name of registered SSH public key
        pub public_key: ::std::string::String,
        ///Size of the VM
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub size: ::std::option::Option<::std::string::String>,
        ///Requested storage size in GiB
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub storage_size: ::std::option::Option<i64>,
        ///Unix user of the VM
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub unix_user: ::std::option::Option<::std::string::String>,
    }

    ///`CreateVmProjectId`
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct CreateVmProjectId(::std::string::String);
    impl ::std::ops::Deref for CreateVmProjectId {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<CreateVmProjectId> for ::std::string::String {
        fn from(value: CreateVmProjectId) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for CreateVmProjectId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^pj[0-9a-hj-km-np-tv-z]{24}$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^pj[0-9a-hj-km-np-tv-z]{24}$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for CreateVmProjectId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for CreateVmProjectId {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for CreateVmProjectId {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`DeleteAllGithubCacheEntriesProjectId`
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct DeleteAllGithubCacheEntriesProjectId(::std::string::String);
    impl ::std::ops::Deref for DeleteAllGithubCacheEntriesProjectId {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<DeleteAllGithubCacheEntriesProjectId> for ::std::string::String {
        fn from(value: DeleteAllGithubCacheEntriesProjectId) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for DeleteAllGithubCacheEntriesProjectId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^pj[0-9a-hj-km-np-tv-z]{24}$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^pj[0-9a-hj-km-np-tv-z]{24}$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for DeleteAllGithubCacheEntriesProjectId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for DeleteAllGithubCacheEntriesProjectId {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for DeleteAllGithubCacheEntriesProjectId {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`DeleteGithubCacheEntryProjectId`
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct DeleteGithubCacheEntryProjectId(::std::string::String);
    impl ::std::ops::Deref for DeleteGithubCacheEntryProjectId {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<DeleteGithubCacheEntryProjectId> for ::std::string::String {
        fn from(value: DeleteGithubCacheEntryProjectId) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for DeleteGithubCacheEntryProjectId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^pj[0-9a-hj-km-np-tv-z]{24}$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^pj[0-9a-hj-km-np-tv-z]{24}$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for DeleteGithubCacheEntryProjectId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for DeleteGithubCacheEntryProjectId {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for DeleteGithubCacheEntryProjectId {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`DeleteInferenceApiKeyInferenceApiKeyId`
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct DeleteInferenceApiKeyInferenceApiKeyId(::std::string::String);
    impl ::std::ops::Deref for DeleteInferenceApiKeyInferenceApiKeyId {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<DeleteInferenceApiKeyInferenceApiKeyId> for ::std::string::String {
        fn from(value: DeleteInferenceApiKeyInferenceApiKeyId) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for DeleteInferenceApiKeyInferenceApiKeyId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^ak[0-9a-hj-km-np-tv-z]{24}$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^ak[0-9a-hj-km-np-tv-z]{24}$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for DeleteInferenceApiKeyInferenceApiKeyId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for DeleteInferenceApiKeyInferenceApiKeyId {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for DeleteInferenceApiKeyInferenceApiKeyId {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`DeleteInferenceApiKeyProjectId`
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct DeleteInferenceApiKeyProjectId(::std::string::String);
    impl ::std::ops::Deref for DeleteInferenceApiKeyProjectId {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<DeleteInferenceApiKeyProjectId> for ::std::string::String {
        fn from(value: DeleteInferenceApiKeyProjectId) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for DeleteInferenceApiKeyProjectId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^pj[0-9a-hj-km-np-tv-z]{24}$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^pj[0-9a-hj-km-np-tv-z]{24}$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for DeleteInferenceApiKeyProjectId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for DeleteInferenceApiKeyProjectId {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for DeleteInferenceApiKeyProjectId {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`DeleteKubernetesClusterProjectId`
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct DeleteKubernetesClusterProjectId(::std::string::String);
    impl ::std::ops::Deref for DeleteKubernetesClusterProjectId {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<DeleteKubernetesClusterProjectId> for ::std::string::String {
        fn from(value: DeleteKubernetesClusterProjectId) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for DeleteKubernetesClusterProjectId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^pj[0-9a-hj-km-np-tv-z]{24}$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^pj[0-9a-hj-km-np-tv-z]{24}$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for DeleteKubernetesClusterProjectId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for DeleteKubernetesClusterProjectId {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for DeleteKubernetesClusterProjectId {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`DeleteKubernetesNodepoolProjectId`
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct DeleteKubernetesNodepoolProjectId(::std::string::String);
    impl ::std::ops::Deref for DeleteKubernetesNodepoolProjectId {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<DeleteKubernetesNodepoolProjectId> for ::std::string::String {
        fn from(value: DeleteKubernetesNodepoolProjectId) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for DeleteKubernetesNodepoolProjectId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^pj[0-9a-hj-km-np-tv-z]{24}$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^pj[0-9a-hj-km-np-tv-z]{24}$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for DeleteKubernetesNodepoolProjectId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for DeleteKubernetesNodepoolProjectId {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for DeleteKubernetesNodepoolProjectId {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`DeleteLoadBalancerProjectId`
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct DeleteLoadBalancerProjectId(::std::string::String);
    impl ::std::ops::Deref for DeleteLoadBalancerProjectId {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<DeleteLoadBalancerProjectId> for ::std::string::String {
        fn from(value: DeleteLoadBalancerProjectId) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for DeleteLoadBalancerProjectId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^pj[0-9a-hj-km-np-tv-z]{24}$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^pj[0-9a-hj-km-np-tv-z]{24}$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for DeleteLoadBalancerProjectId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for DeleteLoadBalancerProjectId {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for DeleteLoadBalancerProjectId {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`DeleteLocationFirewallFirewallRuleFirewallRuleId`
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct DeleteLocationFirewallFirewallRuleFirewallRuleId(::std::string::String);
    impl ::std::ops::Deref for DeleteLocationFirewallFirewallRuleFirewallRuleId {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<DeleteLocationFirewallFirewallRuleFirewallRuleId>
        for ::std::string::String
    {
        fn from(value: DeleteLocationFirewallFirewallRuleFirewallRuleId) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for DeleteLocationFirewallFirewallRuleFirewallRuleId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^fr[0-9a-hj-km-np-tv-z]{24}$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^fr[0-9a-hj-km-np-tv-z]{24}$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for DeleteLocationFirewallFirewallRuleFirewallRuleId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for DeleteLocationFirewallFirewallRuleFirewallRuleId
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for DeleteLocationFirewallFirewallRuleFirewallRuleId {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`DeleteLocationFirewallFirewallRuleProjectId`
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct DeleteLocationFirewallFirewallRuleProjectId(::std::string::String);
    impl ::std::ops::Deref for DeleteLocationFirewallFirewallRuleProjectId {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<DeleteLocationFirewallFirewallRuleProjectId> for ::std::string::String {
        fn from(value: DeleteLocationFirewallFirewallRuleProjectId) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for DeleteLocationFirewallFirewallRuleProjectId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^pj[0-9a-hj-km-np-tv-z]{24}$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^pj[0-9a-hj-km-np-tv-z]{24}$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for DeleteLocationFirewallFirewallRuleProjectId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for DeleteLocationFirewallFirewallRuleProjectId
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for DeleteLocationFirewallFirewallRuleProjectId {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`DeleteLocationFirewallProjectId`
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct DeleteLocationFirewallProjectId(::std::string::String);
    impl ::std::ops::Deref for DeleteLocationFirewallProjectId {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<DeleteLocationFirewallProjectId> for ::std::string::String {
        fn from(value: DeleteLocationFirewallProjectId) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for DeleteLocationFirewallProjectId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^pj[0-9a-hj-km-np-tv-z]{24}$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^pj[0-9a-hj-km-np-tv-z]{24}$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for DeleteLocationFirewallProjectId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for DeleteLocationFirewallProjectId {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for DeleteLocationFirewallProjectId {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`DeleteLocationPostgresFirewallRuleFirewallRuleId`
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct DeleteLocationPostgresFirewallRuleFirewallRuleId(::std::string::String);
    impl ::std::ops::Deref for DeleteLocationPostgresFirewallRuleFirewallRuleId {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<DeleteLocationPostgresFirewallRuleFirewallRuleId>
        for ::std::string::String
    {
        fn from(value: DeleteLocationPostgresFirewallRuleFirewallRuleId) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for DeleteLocationPostgresFirewallRuleFirewallRuleId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^fr[0-9a-hj-km-np-tv-z]{24}$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^fr[0-9a-hj-km-np-tv-z]{24}$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for DeleteLocationPostgresFirewallRuleFirewallRuleId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for DeleteLocationPostgresFirewallRuleFirewallRuleId
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for DeleteLocationPostgresFirewallRuleFirewallRuleId {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`DeleteLocationPostgresFirewallRuleProjectId`
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct DeleteLocationPostgresFirewallRuleProjectId(::std::string::String);
    impl ::std::ops::Deref for DeleteLocationPostgresFirewallRuleProjectId {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<DeleteLocationPostgresFirewallRuleProjectId> for ::std::string::String {
        fn from(value: DeleteLocationPostgresFirewallRuleProjectId) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for DeleteLocationPostgresFirewallRuleProjectId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^pj[0-9a-hj-km-np-tv-z]{24}$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^pj[0-9a-hj-km-np-tv-z]{24}$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for DeleteLocationPostgresFirewallRuleProjectId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for DeleteLocationPostgresFirewallRuleProjectId
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for DeleteLocationPostgresFirewallRuleProjectId {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`DeleteLocationPostgresLogDestinationLogDestinationId`
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct DeleteLocationPostgresLogDestinationLogDestinationId(::std::string::String);
    impl ::std::ops::Deref for DeleteLocationPostgresLogDestinationLogDestinationId {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<DeleteLocationPostgresLogDestinationLogDestinationId>
        for ::std::string::String
    {
        fn from(value: DeleteLocationPostgresLogDestinationLogDestinationId) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for DeleteLocationPostgresLogDestinationLogDestinationId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^1d[0-9a-hj-km-np-tv-z]{24}$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^1d[0-9a-hj-km-np-tv-z]{24}$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for DeleteLocationPostgresLogDestinationLogDestinationId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for DeleteLocationPostgresLogDestinationLogDestinationId
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for DeleteLocationPostgresLogDestinationLogDestinationId {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`DeleteLocationPostgresLogDestinationProjectId`
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct DeleteLocationPostgresLogDestinationProjectId(::std::string::String);
    impl ::std::ops::Deref for DeleteLocationPostgresLogDestinationProjectId {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<DeleteLocationPostgresLogDestinationProjectId> for ::std::string::String {
        fn from(value: DeleteLocationPostgresLogDestinationProjectId) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for DeleteLocationPostgresLogDestinationProjectId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^pj[0-9a-hj-km-np-tv-z]{24}$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^pj[0-9a-hj-km-np-tv-z]{24}$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for DeleteLocationPostgresLogDestinationProjectId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for DeleteLocationPostgresLogDestinationProjectId
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for DeleteLocationPostgresLogDestinationProjectId {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`DeleteLocationPostgresMetricDestinationMetricDestinationId`
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct DeleteLocationPostgresMetricDestinationMetricDestinationId(::std::string::String);
    impl ::std::ops::Deref for DeleteLocationPostgresMetricDestinationMetricDestinationId {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<DeleteLocationPostgresMetricDestinationMetricDestinationId>
        for ::std::string::String
    {
        fn from(value: DeleteLocationPostgresMetricDestinationMetricDestinationId) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for DeleteLocationPostgresMetricDestinationMetricDestinationId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^(et|md)[0-9a-hj-km-np-tv-z]{24}$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^(et|md)[0-9a-hj-km-np-tv-z]{24}$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for DeleteLocationPostgresMetricDestinationMetricDestinationId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for DeleteLocationPostgresMetricDestinationMetricDestinationId
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for DeleteLocationPostgresMetricDestinationMetricDestinationId {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`DeleteLocationPostgresMetricDestinationProjectId`
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct DeleteLocationPostgresMetricDestinationProjectId(::std::string::String);
    impl ::std::ops::Deref for DeleteLocationPostgresMetricDestinationProjectId {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<DeleteLocationPostgresMetricDestinationProjectId>
        for ::std::string::String
    {
        fn from(value: DeleteLocationPostgresMetricDestinationProjectId) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for DeleteLocationPostgresMetricDestinationProjectId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^pj[0-9a-hj-km-np-tv-z]{24}$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^pj[0-9a-hj-km-np-tv-z]{24}$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for DeleteLocationPostgresMetricDestinationProjectId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for DeleteLocationPostgresMetricDestinationProjectId
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for DeleteLocationPostgresMetricDestinationProjectId {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`DeleteMachineImageProjectId`
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct DeleteMachineImageProjectId(::std::string::String);
    impl ::std::ops::Deref for DeleteMachineImageProjectId {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<DeleteMachineImageProjectId> for ::std::string::String {
        fn from(value: DeleteMachineImageProjectId) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for DeleteMachineImageProjectId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^pj[0-9a-hj-km-np-tv-z]{24}$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^pj[0-9a-hj-km-np-tv-z]{24}$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for DeleteMachineImageProjectId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for DeleteMachineImageProjectId {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for DeleteMachineImageProjectId {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`DeleteMachineImageVersionProjectId`
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct DeleteMachineImageVersionProjectId(::std::string::String);
    impl ::std::ops::Deref for DeleteMachineImageVersionProjectId {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<DeleteMachineImageVersionProjectId> for ::std::string::String {
        fn from(value: DeleteMachineImageVersionProjectId) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for DeleteMachineImageVersionProjectId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^pj[0-9a-hj-km-np-tv-z]{24}$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^pj[0-9a-hj-km-np-tv-z]{24}$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for DeleteMachineImageVersionProjectId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for DeleteMachineImageVersionProjectId {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for DeleteMachineImageVersionProjectId {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`DeleteMachineImageVersionVersion`
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct DeleteMachineImageVersionVersion(::std::string::String);
    impl ::std::ops::Deref for DeleteMachineImageVersionVersion {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<DeleteMachineImageVersionVersion> for ::std::string::String {
        fn from(value: DeleteMachineImageVersionVersion) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for DeleteMachineImageVersionVersion {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^[a-zA-Z0-9][a-zA-Z0-9._-]{0,63}$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^[a-zA-Z0-9][a-zA-Z0-9._-]{0,63}$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for DeleteMachineImageVersionVersion {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for DeleteMachineImageVersionVersion {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for DeleteMachineImageVersionVersion {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`DeletePostgresDatabaseProjectId`
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct DeletePostgresDatabaseProjectId(::std::string::String);
    impl ::std::ops::Deref for DeletePostgresDatabaseProjectId {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<DeletePostgresDatabaseProjectId> for ::std::string::String {
        fn from(value: DeletePostgresDatabaseProjectId) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for DeletePostgresDatabaseProjectId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^pj[0-9a-hj-km-np-tv-z]{24}$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^pj[0-9a-hj-km-np-tv-z]{24}$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for DeletePostgresDatabaseProjectId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for DeletePostgresDatabaseProjectId {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for DeletePostgresDatabaseProjectId {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`DeletePrivateLocationProjectId`
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct DeletePrivateLocationProjectId(::std::string::String);
    impl ::std::ops::Deref for DeletePrivateLocationProjectId {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<DeletePrivateLocationProjectId> for ::std::string::String {
        fn from(value: DeletePrivateLocationProjectId) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for DeletePrivateLocationProjectId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^pj[0-9a-hj-km-np-tv-z]{24}$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^pj[0-9a-hj-km-np-tv-z]{24}$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for DeletePrivateLocationProjectId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for DeletePrivateLocationProjectId {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for DeletePrivateLocationProjectId {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`DeletePrivateSubnetProjectId`
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct DeletePrivateSubnetProjectId(::std::string::String);
    impl ::std::ops::Deref for DeletePrivateSubnetProjectId {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<DeletePrivateSubnetProjectId> for ::std::string::String {
        fn from(value: DeletePrivateSubnetProjectId) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for DeletePrivateSubnetProjectId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^pj[0-9a-hj-km-np-tv-z]{24}$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^pj[0-9a-hj-km-np-tv-z]{24}$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for DeletePrivateSubnetProjectId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for DeletePrivateSubnetProjectId {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for DeletePrivateSubnetProjectId {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`DeleteProjectProjectId`
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct DeleteProjectProjectId(::std::string::String);
    impl ::std::ops::Deref for DeleteProjectProjectId {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<DeleteProjectProjectId> for ::std::string::String {
        fn from(value: DeleteProjectProjectId) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for DeleteProjectProjectId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^pj[0-9a-hj-km-np-tv-z]{24}$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^pj[0-9a-hj-km-np-tv-z]{24}$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for DeleteProjectProjectId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for DeleteProjectProjectId {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for DeleteProjectProjectId {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`DeleteSshPublicKeyProjectId`
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct DeleteSshPublicKeyProjectId(::std::string::String);
    impl ::std::ops::Deref for DeleteSshPublicKeyProjectId {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<DeleteSshPublicKeyProjectId> for ::std::string::String {
        fn from(value: DeleteSshPublicKeyProjectId) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for DeleteSshPublicKeyProjectId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^pj[0-9a-hj-km-np-tv-z]{24}$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^pj[0-9a-hj-km-np-tv-z]{24}$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for DeleteSshPublicKeyProjectId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for DeleteSshPublicKeyProjectId {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for DeleteSshPublicKeyProjectId {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`DeleteVmProjectId`
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct DeleteVmProjectId(::std::string::String);
    impl ::std::ops::Deref for DeleteVmProjectId {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<DeleteVmProjectId> for ::std::string::String {
        fn from(value: DeleteVmProjectId) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for DeleteVmProjectId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^pj[0-9a-hj-km-np-tv-z]{24}$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^pj[0-9a-hj-km-np-tv-z]{24}$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for DeleteVmProjectId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for DeleteVmProjectId {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for DeleteVmProjectId {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`DetachVmLocationLoadBalancerBody`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct DetachVmLocationLoadBalancerBody {
        ///Apid of VM to detach from Load Balancer
        pub vm_id: ::std::string::String,
    }

    ///`DetachVmLocationLoadBalancerProjectId`
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct DetachVmLocationLoadBalancerProjectId(::std::string::String);
    impl ::std::ops::Deref for DetachVmLocationLoadBalancerProjectId {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<DetachVmLocationLoadBalancerProjectId> for ::std::string::String {
        fn from(value: DetachVmLocationLoadBalancerProjectId) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for DetachVmLocationLoadBalancerProjectId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^pj[0-9a-hj-km-np-tv-z]{24}$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^pj[0-9a-hj-km-np-tv-z]{24}$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for DetachVmLocationLoadBalancerProjectId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for DetachVmLocationLoadBalancerProjectId {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for DetachVmLocationLoadBalancerProjectId {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`DetachVmLocationLoadBalancerResponse`
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    #[serde(deny_unknown_fields)]
    pub enum DetachVmLocationLoadBalancerResponse {}
    ///`DisconnectPrivateSubnetPrivateSubnetId`
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct DisconnectPrivateSubnetPrivateSubnetId(::std::string::String);
    impl ::std::ops::Deref for DisconnectPrivateSubnetPrivateSubnetId {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<DisconnectPrivateSubnetPrivateSubnetId> for ::std::string::String {
        fn from(value: DisconnectPrivateSubnetPrivateSubnetId) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for DisconnectPrivateSubnetPrivateSubnetId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^ps[0-9a-hj-km-np-tv-z]{24}$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^ps[0-9a-hj-km-np-tv-z]{24}$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for DisconnectPrivateSubnetPrivateSubnetId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for DisconnectPrivateSubnetPrivateSubnetId {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for DisconnectPrivateSubnetPrivateSubnetId {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`DisconnectPrivateSubnetProjectId`
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct DisconnectPrivateSubnetProjectId(::std::string::String);
    impl ::std::ops::Deref for DisconnectPrivateSubnetProjectId {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<DisconnectPrivateSubnetProjectId> for ::std::string::String {
        fn from(value: DisconnectPrivateSubnetProjectId) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for DisconnectPrivateSubnetProjectId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^pj[0-9a-hj-km-np-tv-z]{24}$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^pj[0-9a-hj-km-np-tv-z]{24}$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for DisconnectPrivateSubnetProjectId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for DisconnectPrivateSubnetProjectId {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for DisconnectPrivateSubnetProjectId {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`Error`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct Error {
        pub error: ErrorError,
    }

    ///`ErrorError`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct ErrorError {
        pub code: i64,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub details:
            ::std::option::Option<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        pub message: ::std::string::String,
        #[serde(rename = "type")]
        pub type_: ::std::string::String,
    }

    ///`Firewall`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct Firewall {
        ///Description of the firewall
        pub description: ::std::string::String,
        ///List of firewall rules
        pub firewall_rules: ::std::vec::Vec<FirewallRule>,
        ///ID of the firewall
        pub id: FirewallId,
        ///Location of the firewall
        pub location: ::std::string::String,
        ///Name of the firewall
        pub name: ::std::string::String,
    }

    ///ID of the firewall
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct FirewallId(::std::string::String);
    impl ::std::ops::Deref for FirewallId {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<FirewallId> for ::std::string::String {
        fn from(value: FirewallId) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for FirewallId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^fw[0-9a-hj-km-np-tv-z]{24}$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^fw[0-9a-hj-km-np-tv-z]{24}$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for FirewallId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for FirewallId {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for FirewallId {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`FirewallRule`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct FirewallRule {
        ///CIDR of the firewall rule
        pub cidr: ::std::string::String,
        ///Description of the firewall rule
        pub description: ::std::string::String,
        ///ID of the firewall rule
        pub id: FirewallRuleId,
        ///Port range of the firewall rule
        pub port_range: ::std::string::String,
        ///Protocol of the firewall rule (tcp or udp)
        pub protocol: FirewallRuleProtocol,
    }

    ///ID of the firewall rule
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct FirewallRuleId(::std::string::String);
    impl ::std::ops::Deref for FirewallRuleId {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<FirewallRuleId> for ::std::string::String {
        fn from(value: FirewallRuleId) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for FirewallRuleId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^fr[0-9a-hj-km-np-tv-z]{24}$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^fr[0-9a-hj-km-np-tv-z]{24}$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for FirewallRuleId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for FirewallRuleId {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for FirewallRuleId {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///Protocol of the firewall rule (tcp or udp)
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum FirewallRuleProtocol {
        #[serde(rename = "tcp")]
        Tcp,
        #[serde(rename = "udp")]
        Udp,
    }

    impl ::std::fmt::Display for FirewallRuleProtocol {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Tcp => f.write_str("tcp"),
                Self::Udp => f.write_str("udp"),
            }
        }
    }

    impl ::std::str::FromStr for FirewallRuleProtocol {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "tcp" => Ok(Self::Tcp),
                "udp" => Ok(Self::Udp),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for FirewallRuleProtocol {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for FirewallRuleProtocol {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`FirewallRuleUpdate`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(untagged, deny_unknown_fields)]
    pub enum FirewallRuleUpdate {
        Variant0 {
            ///CIDR of the firewall rule
            cidr: ::std::string::String,
            ///Description of the firewall rule
            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
            description: ::std::option::Option<::std::string::String>,
            ///Port range of the firewall rule
            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
            port_range: ::std::option::Option<::std::string::String>,
            ///Protocol of the firewall rule (tcp or udp)
            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
            protocol: ::std::option::Option<::std::string::String>,
        },
        Variant1 {
            ///Description of the firewall rule
            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
            description: ::std::option::Option<::std::string::String>,
            ///Port range of the firewall rule
            port_range: ::std::string::String,
            ///Protocol of the firewall rule (tcp or udp)
            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
            protocol: ::std::option::Option<::std::string::String>,
        },
        Variant2 {
            ///Description of the firewall rule
            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
            description: ::std::option::Option<::std::string::String>,
            ///Protocol of the firewall rule (tcp or udp)
            protocol: ::std::string::String,
        },
        Variant3 {
            ///Description of the firewall rule
            description: ::std::string::String,
        },
    }

    ///`GetFirewallProjectId`
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct GetFirewallProjectId(::std::string::String);
    impl ::std::ops::Deref for GetFirewallProjectId {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<GetFirewallProjectId> for ::std::string::String {
        fn from(value: GetFirewallProjectId) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for GetFirewallProjectId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^pj[0-9a-hj-km-np-tv-z]{24}$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^pj[0-9a-hj-km-np-tv-z]{24}$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for GetFirewallProjectId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for GetFirewallProjectId {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for GetFirewallProjectId {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`GetFirewallResponse`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct GetFirewallResponse {
        pub count: i64,
        pub items: ::std::vec::Vec<Firewall>,
    }

    ///`GetGithubCacheEntriesProjectId`
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct GetGithubCacheEntriesProjectId(::std::string::String);
    impl ::std::ops::Deref for GetGithubCacheEntriesProjectId {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<GetGithubCacheEntriesProjectId> for ::std::string::String {
        fn from(value: GetGithubCacheEntriesProjectId) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for GetGithubCacheEntriesProjectId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^pj[0-9a-hj-km-np-tv-z]{24}$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^pj[0-9a-hj-km-np-tv-z]{24}$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for GetGithubCacheEntriesProjectId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for GetGithubCacheEntriesProjectId {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for GetGithubCacheEntriesProjectId {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`GetGithubCacheEntriesResponse`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct GetGithubCacheEntriesResponse {
        pub count: i64,
        pub items: ::std::vec::Vec<GithubCacheEntry>,
    }

    ///`GetGithubCacheEntryProjectId`
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct GetGithubCacheEntryProjectId(::std::string::String);
    impl ::std::ops::Deref for GetGithubCacheEntryProjectId {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<GetGithubCacheEntryProjectId> for ::std::string::String {
        fn from(value: GetGithubCacheEntryProjectId) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for GetGithubCacheEntryProjectId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^pj[0-9a-hj-km-np-tv-z]{24}$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^pj[0-9a-hj-km-np-tv-z]{24}$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for GetGithubCacheEntryProjectId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for GetGithubCacheEntryProjectId {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for GetGithubCacheEntryProjectId {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`GetGithubInstallationProjectId`
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct GetGithubInstallationProjectId(::std::string::String);
    impl ::std::ops::Deref for GetGithubInstallationProjectId {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<GetGithubInstallationProjectId> for ::std::string::String {
        fn from(value: GetGithubInstallationProjectId) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for GetGithubInstallationProjectId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^pj[0-9a-hj-km-np-tv-z]{24}$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^pj[0-9a-hj-km-np-tv-z]{24}$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for GetGithubInstallationProjectId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for GetGithubInstallationProjectId {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for GetGithubInstallationProjectId {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`GetGithubInstallationRepositoriesProjectId`
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct GetGithubInstallationRepositoriesProjectId(::std::string::String);
    impl ::std::ops::Deref for GetGithubInstallationRepositoriesProjectId {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<GetGithubInstallationRepositoriesProjectId> for ::std::string::String {
        fn from(value: GetGithubInstallationRepositoriesProjectId) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for GetGithubInstallationRepositoriesProjectId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^pj[0-9a-hj-km-np-tv-z]{24}$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^pj[0-9a-hj-km-np-tv-z]{24}$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for GetGithubInstallationRepositoriesProjectId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for GetGithubInstallationRepositoriesProjectId {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for GetGithubInstallationRepositoriesProjectId {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`GetGithubInstallationRepositoriesResponse`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct GetGithubInstallationRepositoriesResponse {
        pub count: i64,
        pub items: ::std::vec::Vec<GithubRepository>,
    }

    ///`GetGithubInstallationsProjectId`
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct GetGithubInstallationsProjectId(::std::string::String);
    impl ::std::ops::Deref for GetGithubInstallationsProjectId {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<GetGithubInstallationsProjectId> for ::std::string::String {
        fn from(value: GetGithubInstallationsProjectId) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for GetGithubInstallationsProjectId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^pj[0-9a-hj-km-np-tv-z]{24}$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^pj[0-9a-hj-km-np-tv-z]{24}$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for GetGithubInstallationsProjectId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for GetGithubInstallationsProjectId {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for GetGithubInstallationsProjectId {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`GetGithubInstallationsResponse`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct GetGithubInstallationsResponse {
        pub count: i64,
        pub items: ::std::vec::Vec<GithubInstallation>,
    }

    ///`GetGithubRepositoryProjectId`
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct GetGithubRepositoryProjectId(::std::string::String);
    impl ::std::ops::Deref for GetGithubRepositoryProjectId {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<GetGithubRepositoryProjectId> for ::std::string::String {
        fn from(value: GetGithubRepositoryProjectId) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for GetGithubRepositoryProjectId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^pj[0-9a-hj-km-np-tv-z]{24}$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^pj[0-9a-hj-km-np-tv-z]{24}$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for GetGithubRepositoryProjectId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for GetGithubRepositoryProjectId {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for GetGithubRepositoryProjectId {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`GetInferenceApiKeyInferenceApiKeyId`
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct GetInferenceApiKeyInferenceApiKeyId(::std::string::String);
    impl ::std::ops::Deref for GetInferenceApiKeyInferenceApiKeyId {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<GetInferenceApiKeyInferenceApiKeyId> for ::std::string::String {
        fn from(value: GetInferenceApiKeyInferenceApiKeyId) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for GetInferenceApiKeyInferenceApiKeyId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^ak[0-9a-hj-km-np-tv-z]{24}$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^ak[0-9a-hj-km-np-tv-z]{24}$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for GetInferenceApiKeyInferenceApiKeyId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for GetInferenceApiKeyInferenceApiKeyId {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for GetInferenceApiKeyInferenceApiKeyId {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`GetInferenceApiKeyProjectId`
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct GetInferenceApiKeyProjectId(::std::string::String);
    impl ::std::ops::Deref for GetInferenceApiKeyProjectId {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<GetInferenceApiKeyProjectId> for ::std::string::String {
        fn from(value: GetInferenceApiKeyProjectId) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for GetInferenceApiKeyProjectId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^pj[0-9a-hj-km-np-tv-z]{24}$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^pj[0-9a-hj-km-np-tv-z]{24}$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for GetInferenceApiKeyProjectId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for GetInferenceApiKeyProjectId {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for GetInferenceApiKeyProjectId {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`GetInferenceApiKeysProjectId`
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct GetInferenceApiKeysProjectId(::std::string::String);
    impl ::std::ops::Deref for GetInferenceApiKeysProjectId {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<GetInferenceApiKeysProjectId> for ::std::string::String {
        fn from(value: GetInferenceApiKeysProjectId) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for GetInferenceApiKeysProjectId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^pj[0-9a-hj-km-np-tv-z]{24}$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^pj[0-9a-hj-km-np-tv-z]{24}$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for GetInferenceApiKeysProjectId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for GetInferenceApiKeysProjectId {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for GetInferenceApiKeysProjectId {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`GetInferenceApiKeysResponse`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct GetInferenceApiKeysResponse {
        pub items: ::std::vec::Vec<InferenceApiKey>,
    }

    ///`GetKubernetesClusterDetailsProjectId`
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct GetKubernetesClusterDetailsProjectId(::std::string::String);
    impl ::std::ops::Deref for GetKubernetesClusterDetailsProjectId {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<GetKubernetesClusterDetailsProjectId> for ::std::string::String {
        fn from(value: GetKubernetesClusterDetailsProjectId) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for GetKubernetesClusterDetailsProjectId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^pj[0-9a-hj-km-np-tv-z]{24}$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^pj[0-9a-hj-km-np-tv-z]{24}$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for GetKubernetesClusterDetailsProjectId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for GetKubernetesClusterDetailsProjectId {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for GetKubernetesClusterDetailsProjectId {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`GetKubernetesClusterDetailsResponse`
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    #[serde(deny_unknown_fields)]
    pub enum GetKubernetesClusterDetailsResponse {}
    ///`GetKubernetesClusterMetricsProjectId`
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct GetKubernetesClusterMetricsProjectId(::std::string::String);
    impl ::std::ops::Deref for GetKubernetesClusterMetricsProjectId {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<GetKubernetesClusterMetricsProjectId> for ::std::string::String {
        fn from(value: GetKubernetesClusterMetricsProjectId) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for GetKubernetesClusterMetricsProjectId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^pj[0-9a-hj-km-np-tv-z]{24}$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^pj[0-9a-hj-km-np-tv-z]{24}$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for GetKubernetesClusterMetricsProjectId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for GetKubernetesClusterMetricsProjectId {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for GetKubernetesClusterMetricsProjectId {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`GetKubernetesKubeconfigFileProjectId`
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct GetKubernetesKubeconfigFileProjectId(::std::string::String);
    impl ::std::ops::Deref for GetKubernetesKubeconfigFileProjectId {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<GetKubernetesKubeconfigFileProjectId> for ::std::string::String {
        fn from(value: GetKubernetesKubeconfigFileProjectId) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for GetKubernetesKubeconfigFileProjectId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^pj[0-9a-hj-km-np-tv-z]{24}$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^pj[0-9a-hj-km-np-tv-z]{24}$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for GetKubernetesKubeconfigFileProjectId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for GetKubernetesKubeconfigFileProjectId {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for GetKubernetesKubeconfigFileProjectId {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`GetLoadBalancerDetailsProjectId`
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct GetLoadBalancerDetailsProjectId(::std::string::String);
    impl ::std::ops::Deref for GetLoadBalancerDetailsProjectId {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<GetLoadBalancerDetailsProjectId> for ::std::string::String {
        fn from(value: GetLoadBalancerDetailsProjectId) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for GetLoadBalancerDetailsProjectId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^pj[0-9a-hj-km-np-tv-z]{24}$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^pj[0-9a-hj-km-np-tv-z]{24}$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for GetLoadBalancerDetailsProjectId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for GetLoadBalancerDetailsProjectId {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for GetLoadBalancerDetailsProjectId {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`GetLoadBalancerDetailsResponse`
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    #[serde(deny_unknown_fields)]
    pub enum GetLoadBalancerDetailsResponse {}
    ///`GetLocationFirewallDetailsProjectId`
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct GetLocationFirewallDetailsProjectId(::std::string::String);
    impl ::std::ops::Deref for GetLocationFirewallDetailsProjectId {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<GetLocationFirewallDetailsProjectId> for ::std::string::String {
        fn from(value: GetLocationFirewallDetailsProjectId) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for GetLocationFirewallDetailsProjectId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^pj[0-9a-hj-km-np-tv-z]{24}$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^pj[0-9a-hj-km-np-tv-z]{24}$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for GetLocationFirewallDetailsProjectId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for GetLocationFirewallDetailsProjectId {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for GetLocationFirewallDetailsProjectId {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`GetLocationFirewallDetailsResponse`
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    #[serde(deny_unknown_fields)]
    pub enum GetLocationFirewallDetailsResponse {}
    ///`GetLocationFirewallFirewallRuleDetailsFirewallRuleId`
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct GetLocationFirewallFirewallRuleDetailsFirewallRuleId(::std::string::String);
    impl ::std::ops::Deref for GetLocationFirewallFirewallRuleDetailsFirewallRuleId {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<GetLocationFirewallFirewallRuleDetailsFirewallRuleId>
        for ::std::string::String
    {
        fn from(value: GetLocationFirewallFirewallRuleDetailsFirewallRuleId) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for GetLocationFirewallFirewallRuleDetailsFirewallRuleId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^fr[0-9a-hj-km-np-tv-z]{24}$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^fr[0-9a-hj-km-np-tv-z]{24}$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for GetLocationFirewallFirewallRuleDetailsFirewallRuleId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for GetLocationFirewallFirewallRuleDetailsFirewallRuleId
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for GetLocationFirewallFirewallRuleDetailsFirewallRuleId {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`GetLocationFirewallFirewallRuleDetailsProjectId`
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct GetLocationFirewallFirewallRuleDetailsProjectId(::std::string::String);
    impl ::std::ops::Deref for GetLocationFirewallFirewallRuleDetailsProjectId {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<GetLocationFirewallFirewallRuleDetailsProjectId>
        for ::std::string::String
    {
        fn from(value: GetLocationFirewallFirewallRuleDetailsProjectId) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for GetLocationFirewallFirewallRuleDetailsProjectId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^pj[0-9a-hj-km-np-tv-z]{24}$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^pj[0-9a-hj-km-np-tv-z]{24}$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for GetLocationFirewallFirewallRuleDetailsProjectId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for GetLocationFirewallFirewallRuleDetailsProjectId
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for GetLocationFirewallFirewallRuleDetailsProjectId {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`GetLocationFirewallProjectId`
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct GetLocationFirewallProjectId(::std::string::String);
    impl ::std::ops::Deref for GetLocationFirewallProjectId {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<GetLocationFirewallProjectId> for ::std::string::String {
        fn from(value: GetLocationFirewallProjectId) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for GetLocationFirewallProjectId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^pj[0-9a-hj-km-np-tv-z]{24}$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^pj[0-9a-hj-km-np-tv-z]{24}$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for GetLocationFirewallProjectId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for GetLocationFirewallProjectId {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for GetLocationFirewallProjectId {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`GetLocationFirewallResponse`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct GetLocationFirewallResponse {
        pub count: i64,
        pub items: ::std::vec::Vec<Firewall>,
    }

    ///`GetMachineImageProjectId`
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct GetMachineImageProjectId(::std::string::String);
    impl ::std::ops::Deref for GetMachineImageProjectId {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<GetMachineImageProjectId> for ::std::string::String {
        fn from(value: GetMachineImageProjectId) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for GetMachineImageProjectId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^pj[0-9a-hj-km-np-tv-z]{24}$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^pj[0-9a-hj-km-np-tv-z]{24}$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for GetMachineImageProjectId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for GetMachineImageProjectId {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for GetMachineImageProjectId {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`GetObjectInfoObjectId`
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct GetObjectInfoObjectId(::std::string::String);
    impl ::std::ops::Deref for GetObjectInfoObjectId {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<GetObjectInfoObjectId> for ::std::string::String {
        fn from(value: GetObjectInfoObjectId) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for GetObjectInfoObjectId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^(fw|kc|1b|m1|pg|ps|vm)[0-9a-hj-km-np-tv-z]{24}$")
                        .unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err(
                    "doesn't match pattern \"^(fw|kc|1b|m1|pg|ps|vm)[0-9a-hj-km-np-tv-z]{24}$\""
                        .into(),
                );
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for GetObjectInfoObjectId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for GetObjectInfoObjectId {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for GetObjectInfoObjectId {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`GetObjectInfoProjectId`
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct GetObjectInfoProjectId(::std::string::String);
    impl ::std::ops::Deref for GetObjectInfoProjectId {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<GetObjectInfoProjectId> for ::std::string::String {
        fn from(value: GetObjectInfoProjectId) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for GetObjectInfoProjectId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^pj[0-9a-hj-km-np-tv-z]{24}$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^pj[0-9a-hj-km-np-tv-z]{24}$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for GetObjectInfoProjectId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for GetObjectInfoProjectId {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for GetObjectInfoProjectId {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`GetObjectInfoResponse`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct GetObjectInfoResponse {
        ///Location of object
        pub location: ::std::string::String,
        ///Name of object
        pub name: ::std::string::String,
        ///Type of object
        #[serde(rename = "type")]
        pub type_: ::std::string::String,
    }

    ///`GetPostgresCaCertificatesByNameProjectId`
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct GetPostgresCaCertificatesByNameProjectId(::std::string::String);
    impl ::std::ops::Deref for GetPostgresCaCertificatesByNameProjectId {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<GetPostgresCaCertificatesByNameProjectId> for ::std::string::String {
        fn from(value: GetPostgresCaCertificatesByNameProjectId) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for GetPostgresCaCertificatesByNameProjectId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^pj[0-9a-hj-km-np-tv-z]{24}$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^pj[0-9a-hj-km-np-tv-z]{24}$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for GetPostgresCaCertificatesByNameProjectId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for GetPostgresCaCertificatesByNameProjectId {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for GetPostgresCaCertificatesByNameProjectId {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`GetPostgresCapabilitiesProjectId`
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct GetPostgresCapabilitiesProjectId(::std::string::String);
    impl ::std::ops::Deref for GetPostgresCapabilitiesProjectId {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<GetPostgresCapabilitiesProjectId> for ::std::string::String {
        fn from(value: GetPostgresCapabilitiesProjectId) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for GetPostgresCapabilitiesProjectId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^pj[0-9a-hj-km-np-tv-z]{24}$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^pj[0-9a-hj-km-np-tv-z]{24}$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for GetPostgresCapabilitiesProjectId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for GetPostgresCapabilitiesProjectId {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for GetPostgresCapabilitiesProjectId {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`GetPostgresDatabaseConfigProjectId`
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct GetPostgresDatabaseConfigProjectId(::std::string::String);
    impl ::std::ops::Deref for GetPostgresDatabaseConfigProjectId {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<GetPostgresDatabaseConfigProjectId> for ::std::string::String {
        fn from(value: GetPostgresDatabaseConfigProjectId) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for GetPostgresDatabaseConfigProjectId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^pj[0-9a-hj-km-np-tv-z]{24}$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^pj[0-9a-hj-km-np-tv-z]{24}$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for GetPostgresDatabaseConfigProjectId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for GetPostgresDatabaseConfigProjectId {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for GetPostgresDatabaseConfigProjectId {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`GetPostgresDatabaseDetailsProjectId`
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct GetPostgresDatabaseDetailsProjectId(::std::string::String);
    impl ::std::ops::Deref for GetPostgresDatabaseDetailsProjectId {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<GetPostgresDatabaseDetailsProjectId> for ::std::string::String {
        fn from(value: GetPostgresDatabaseDetailsProjectId) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for GetPostgresDatabaseDetailsProjectId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^pj[0-9a-hj-km-np-tv-z]{24}$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^pj[0-9a-hj-km-np-tv-z]{24}$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for GetPostgresDatabaseDetailsProjectId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for GetPostgresDatabaseDetailsProjectId {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for GetPostgresDatabaseDetailsProjectId {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`GetPostgresDatabaseDetailsResponse`
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    #[serde(deny_unknown_fields)]
    pub enum GetPostgresDatabaseDetailsResponse {}
    ///`GetPostgresDatabaseLogsPaginationKey`
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct GetPostgresDatabaseLogsPaginationKey(::std::string::String);
    impl ::std::ops::Deref for GetPostgresDatabaseLogsPaginationKey {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<GetPostgresDatabaseLogsPaginationKey> for ::std::string::String {
        fn from(value: GetPostgresDatabaseLogsPaginationKey) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for GetPostgresDatabaseLogsPaginationKey {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new(
                        "^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$",
                    )
                    .unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err ("doesn't match pattern \"^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$\"" . into ()) ;
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for GetPostgresDatabaseLogsPaginationKey {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for GetPostgresDatabaseLogsPaginationKey {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for GetPostgresDatabaseLogsPaginationKey {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`GetPostgresDatabaseLogsProjectId`
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct GetPostgresDatabaseLogsProjectId(::std::string::String);
    impl ::std::ops::Deref for GetPostgresDatabaseLogsProjectId {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<GetPostgresDatabaseLogsProjectId> for ::std::string::String {
        fn from(value: GetPostgresDatabaseLogsProjectId) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for GetPostgresDatabaseLogsProjectId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^pj[0-9a-hj-km-np-tv-z]{24}$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^pj[0-9a-hj-km-np-tv-z]{24}$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for GetPostgresDatabaseLogsProjectId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for GetPostgresDatabaseLogsProjectId {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for GetPostgresDatabaseLogsProjectId {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`GetPostgresDatabaseLogsResponse`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct GetPostgresDatabaseLogsResponse {
        pub logs: ::std::vec::Vec<PostgresDatabaseLogEntry>,
        ///Pass as pagination_key to retrieve the next page of results
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub pagination_key: ::std::option::Option<GetPostgresDatabaseLogsResponsePaginationKey>,
    }

    ///Pass as pagination_key to retrieve the next page of results
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct GetPostgresDatabaseLogsResponsePaginationKey(::std::string::String);
    impl ::std::ops::Deref for GetPostgresDatabaseLogsResponsePaginationKey {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<GetPostgresDatabaseLogsResponsePaginationKey> for ::std::string::String {
        fn from(value: GetPostgresDatabaseLogsResponsePaginationKey) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for GetPostgresDatabaseLogsResponsePaginationKey {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new(
                        "^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$",
                    )
                    .unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err ("doesn't match pattern \"^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$\"" . into ()) ;
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for GetPostgresDatabaseLogsResponsePaginationKey {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for GetPostgresDatabaseLogsResponsePaginationKey
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for GetPostgresDatabaseLogsResponsePaginationKey {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`GetPostgresDatabaseLogsServerRole`
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum GetPostgresDatabaseLogsServerRole {
        #[serde(rename = "primary")]
        Primary,
        #[serde(rename = "standby")]
        Standby,
    }

    impl ::std::fmt::Display for GetPostgresDatabaseLogsServerRole {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Primary => f.write_str("primary"),
                Self::Standby => f.write_str("standby"),
            }
        }
    }

    impl ::std::str::FromStr for GetPostgresDatabaseLogsServerRole {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "primary" => Ok(Self::Primary),
                "standby" => Ok(Self::Standby),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for GetPostgresDatabaseLogsServerRole {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for GetPostgresDatabaseLogsServerRole {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`GetPostgresDatabaseLogsSeverityLevel`
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum GetPostgresDatabaseLogsSeverityLevel {
        #[serde(rename = "DEBUG")]
        Debug,
        #[serde(rename = "INFO")]
        Info,
        #[serde(rename = "WARN")]
        Warn,
        #[serde(rename = "ERROR")]
        Error,
        #[serde(rename = "FATAL")]
        Fatal,
    }

    impl ::std::fmt::Display for GetPostgresDatabaseLogsSeverityLevel {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Debug => f.write_str("DEBUG"),
                Self::Info => f.write_str("INFO"),
                Self::Warn => f.write_str("WARN"),
                Self::Error => f.write_str("ERROR"),
                Self::Fatal => f.write_str("FATAL"),
            }
        }
    }

    impl ::std::str::FromStr for GetPostgresDatabaseLogsSeverityLevel {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "DEBUG" => Ok(Self::Debug),
                "INFO" => Ok(Self::Info),
                "WARN" => Ok(Self::Warn),
                "ERROR" => Ok(Self::Error),
                "FATAL" => Ok(Self::Fatal),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for GetPostgresDatabaseLogsSeverityLevel {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for GetPostgresDatabaseLogsSeverityLevel {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`GetPostgresDatabaseLogsStreamName`
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum GetPostgresDatabaseLogsStreamName {
        #[serde(rename = "postgres")]
        Postgres,
        #[serde(rename = "pgbouncer")]
        Pgbouncer,
        #[serde(rename = "upgrade")]
        Upgrade,
    }

    impl ::std::fmt::Display for GetPostgresDatabaseLogsStreamName {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Postgres => f.write_str("postgres"),
                Self::Pgbouncer => f.write_str("pgbouncer"),
                Self::Upgrade => f.write_str("upgrade"),
            }
        }
    }

    impl ::std::str::FromStr for GetPostgresDatabaseLogsStreamName {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "postgres" => Ok(Self::Postgres),
                "pgbouncer" => Ok(Self::Pgbouncer),
                "upgrade" => Ok(Self::Upgrade),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for GetPostgresDatabaseLogsStreamName {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for GetPostgresDatabaseLogsStreamName {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`GetPostgresDatabaseMetricsProjectId`
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct GetPostgresDatabaseMetricsProjectId(::std::string::String);
    impl ::std::ops::Deref for GetPostgresDatabaseMetricsProjectId {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<GetPostgresDatabaseMetricsProjectId> for ::std::string::String {
        fn from(value: GetPostgresDatabaseMetricsProjectId) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for GetPostgresDatabaseMetricsProjectId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^pj[0-9a-hj-km-np-tv-z]{24}$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^pj[0-9a-hj-km-np-tv-z]{24}$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for GetPostgresDatabaseMetricsProjectId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for GetPostgresDatabaseMetricsProjectId {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for GetPostgresDatabaseMetricsProjectId {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`GetPostgresDatabaseUpgradeStatusProjectId`
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct GetPostgresDatabaseUpgradeStatusProjectId(::std::string::String);
    impl ::std::ops::Deref for GetPostgresDatabaseUpgradeStatusProjectId {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<GetPostgresDatabaseUpgradeStatusProjectId> for ::std::string::String {
        fn from(value: GetPostgresDatabaseUpgradeStatusProjectId) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for GetPostgresDatabaseUpgradeStatusProjectId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^pj[0-9a-hj-km-np-tv-z]{24}$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^pj[0-9a-hj-km-np-tv-z]{24}$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for GetPostgresDatabaseUpgradeStatusProjectId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for GetPostgresDatabaseUpgradeStatusProjectId {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for GetPostgresDatabaseUpgradeStatusProjectId {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`GetPostgresDatabaseUpgradeStatusResponse`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct GetPostgresDatabaseUpgradeStatusResponse {
        pub current_version: GetPostgresDatabaseUpgradeStatusResponseCurrentVersion,
        pub target_version: GetPostgresDatabaseUpgradeStatusResponseTargetVersion,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub upgrade_stage: ::std::option::Option<::std::string::String>,
        pub upgrade_status: ::std::string::String,
    }

    ///`GetPostgresDatabaseUpgradeStatusResponseCurrentVersion`
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum GetPostgresDatabaseUpgradeStatusResponseCurrentVersion {
        #[serde(rename = "16")]
        X16,
        #[serde(rename = "17")]
        X17,
        #[serde(rename = "18")]
        X18,
    }

    impl ::std::fmt::Display for GetPostgresDatabaseUpgradeStatusResponseCurrentVersion {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::X16 => f.write_str("16"),
                Self::X17 => f.write_str("17"),
                Self::X18 => f.write_str("18"),
            }
        }
    }

    impl ::std::str::FromStr for GetPostgresDatabaseUpgradeStatusResponseCurrentVersion {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "16" => Ok(Self::X16),
                "17" => Ok(Self::X17),
                "18" => Ok(Self::X18),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for GetPostgresDatabaseUpgradeStatusResponseCurrentVersion {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for GetPostgresDatabaseUpgradeStatusResponseCurrentVersion
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`GetPostgresDatabaseUpgradeStatusResponseTargetVersion`
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum GetPostgresDatabaseUpgradeStatusResponseTargetVersion {
        #[serde(rename = "16")]
        X16,
        #[serde(rename = "17")]
        X17,
        #[serde(rename = "18")]
        X18,
    }

    impl ::std::fmt::Display for GetPostgresDatabaseUpgradeStatusResponseTargetVersion {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::X16 => f.write_str("16"),
                Self::X17 => f.write_str("17"),
                Self::X18 => f.write_str("18"),
            }
        }
    }

    impl ::std::str::FromStr for GetPostgresDatabaseUpgradeStatusResponseTargetVersion {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "16" => Ok(Self::X16),
                "17" => Ok(Self::X17),
                "18" => Ok(Self::X18),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for GetPostgresDatabaseUpgradeStatusResponseTargetVersion {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for GetPostgresDatabaseUpgradeStatusResponseTargetVersion
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`GetPrivateLocationDetailsProjectId`
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct GetPrivateLocationDetailsProjectId(::std::string::String);
    impl ::std::ops::Deref for GetPrivateLocationDetailsProjectId {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<GetPrivateLocationDetailsProjectId> for ::std::string::String {
        fn from(value: GetPrivateLocationDetailsProjectId) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for GetPrivateLocationDetailsProjectId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^pj[0-9a-hj-km-np-tv-z]{24}$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^pj[0-9a-hj-km-np-tv-z]{24}$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for GetPrivateLocationDetailsProjectId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for GetPrivateLocationDetailsProjectId {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for GetPrivateLocationDetailsProjectId {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`GetPrivateLocationsProjectId`
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct GetPrivateLocationsProjectId(::std::string::String);
    impl ::std::ops::Deref for GetPrivateLocationsProjectId {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<GetPrivateLocationsProjectId> for ::std::string::String {
        fn from(value: GetPrivateLocationsProjectId) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for GetPrivateLocationsProjectId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^pj[0-9a-hj-km-np-tv-z]{24}$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^pj[0-9a-hj-km-np-tv-z]{24}$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for GetPrivateLocationsProjectId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for GetPrivateLocationsProjectId {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for GetPrivateLocationsProjectId {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`GetPrivateLocationsResponse`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct GetPrivateLocationsResponse {
        pub count: i64,
        pub items: ::std::vec::Vec<PrivateLocation>,
    }

    ///`GetPrivateSubnetDetailsProjectId`
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct GetPrivateSubnetDetailsProjectId(::std::string::String);
    impl ::std::ops::Deref for GetPrivateSubnetDetailsProjectId {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<GetPrivateSubnetDetailsProjectId> for ::std::string::String {
        fn from(value: GetPrivateSubnetDetailsProjectId) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for GetPrivateSubnetDetailsProjectId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^pj[0-9a-hj-km-np-tv-z]{24}$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^pj[0-9a-hj-km-np-tv-z]{24}$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for GetPrivateSubnetDetailsProjectId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for GetPrivateSubnetDetailsProjectId {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for GetPrivateSubnetDetailsProjectId {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`GetProjectProjectId`
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct GetProjectProjectId(::std::string::String);
    impl ::std::ops::Deref for GetProjectProjectId {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<GetProjectProjectId> for ::std::string::String {
        fn from(value: GetProjectProjectId) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for GetProjectProjectId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^pj[0-9a-hj-km-np-tv-z]{24}$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^pj[0-9a-hj-km-np-tv-z]{24}$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for GetProjectProjectId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for GetProjectProjectId {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for GetProjectProjectId {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`GetSshPublicKeyProjectId`
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct GetSshPublicKeyProjectId(::std::string::String);
    impl ::std::ops::Deref for GetSshPublicKeyProjectId {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<GetSshPublicKeyProjectId> for ::std::string::String {
        fn from(value: GetSshPublicKeyProjectId) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for GetSshPublicKeyProjectId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^pj[0-9a-hj-km-np-tv-z]{24}$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^pj[0-9a-hj-km-np-tv-z]{24}$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for GetSshPublicKeyProjectId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for GetSshPublicKeyProjectId {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for GetSshPublicKeyProjectId {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`GetSshPublicKeysProjectId`
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct GetSshPublicKeysProjectId(::std::string::String);
    impl ::std::ops::Deref for GetSshPublicKeysProjectId {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<GetSshPublicKeysProjectId> for ::std::string::String {
        fn from(value: GetSshPublicKeysProjectId) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for GetSshPublicKeysProjectId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^pj[0-9a-hj-km-np-tv-z]{24}$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^pj[0-9a-hj-km-np-tv-z]{24}$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for GetSshPublicKeysProjectId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for GetSshPublicKeysProjectId {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for GetSshPublicKeysProjectId {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`GetSshPublicKeysResponse`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct GetSshPublicKeysResponse {
        pub items: ::std::vec::Vec<SshPublicKey>,
    }

    ///`GetVmDetailsProjectId`
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct GetVmDetailsProjectId(::std::string::String);
    impl ::std::ops::Deref for GetVmDetailsProjectId {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<GetVmDetailsProjectId> for ::std::string::String {
        fn from(value: GetVmDetailsProjectId) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for GetVmDetailsProjectId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^pj[0-9a-hj-km-np-tv-z]{24}$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^pj[0-9a-hj-km-np-tv-z]{24}$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for GetVmDetailsProjectId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for GetVmDetailsProjectId {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for GetVmDetailsProjectId {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`GetVmSerialLogProjectId`
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct GetVmSerialLogProjectId(::std::string::String);
    impl ::std::ops::Deref for GetVmSerialLogProjectId {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<GetVmSerialLogProjectId> for ::std::string::String {
        fn from(value: GetVmSerialLogProjectId) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for GetVmSerialLogProjectId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^pj[0-9a-hj-km-np-tv-z]{24}$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^pj[0-9a-hj-km-np-tv-z]{24}$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for GetVmSerialLogProjectId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for GetVmSerialLogProjectId {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for GetVmSerialLogProjectId {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`GithubCacheEntry`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct GithubCacheEntry {
        ///Timestamp of when the cache entry was created
        pub created_at: ::chrono::DateTime<::chrono::offset::Utc>,
        ///ID of the GitHub cache entry
        pub id: GithubCacheEntryId,
        ///name of the GitHub installation containing the repository containing
        /// the cache entry
        pub installation_name: ::std::string::String,
        ///GitHub cache entry key
        pub key: ::std::string::String,
        ///Timestamp of when the cache entry was last accessed
        #[serde(deserialize_with = "::std::option::Option::deserialize")]
        pub last_accessed_at: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
        ///name of the GitHub repository containing the cache entry
        pub repository_name: ::std::string::String,
        ///Branch scope of the GitHub cache entry
        pub scope: ::std::string::String,
        ///Size of the GitHub cache entry in bytes
        #[serde(deserialize_with = "::std::option::Option::deserialize")]
        pub size: ::std::option::Option<i64>,
    }

    ///ID of the GitHub cache entry
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct GithubCacheEntryId(::std::string::String);
    impl ::std::ops::Deref for GithubCacheEntryId {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<GithubCacheEntryId> for ::std::string::String {
        fn from(value: GithubCacheEntryId) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for GithubCacheEntryId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^ge[0-9a-hj-km-np-tv-z]{24}$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^ge[0-9a-hj-km-np-tv-z]{24}$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for GithubCacheEntryId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for GithubCacheEntryId {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for GithubCacheEntryId {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`GithubInstallation`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct GithubInstallation {
        ///ID of the GitHub installation
        pub id: GithubInstallationId,
        ///Name of the GitHub installation (GitHub user/organization name)
        pub name: ::std::string::String,
    }

    ///ID of the GitHub installation
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct GithubInstallationId(::std::string::String);
    impl ::std::ops::Deref for GithubInstallationId {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<GithubInstallationId> for ::std::string::String {
        fn from(value: GithubInstallationId) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for GithubInstallationId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^g1[0-9a-hj-km-np-tv-z]{24}$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^g1[0-9a-hj-km-np-tv-z]{24}$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for GithubInstallationId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for GithubInstallationId {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for GithubInstallationId {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`GithubRepository`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct GithubRepository {
        ///ID of the GitHub repository
        pub id: GithubRepositoryId,
        ///Name of the GitHub installation containing the repository
        pub installation_name: ::std::string::String,
        ///Name of the GitHub repository
        pub name: ::std::string::String,
    }

    ///ID of the GitHub repository
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct GithubRepositoryId(::std::string::String);
    impl ::std::ops::Deref for GithubRepositoryId {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<GithubRepositoryId> for ::std::string::String {
        fn from(value: GithubRepositoryId) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for GithubRepositoryId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^gp[0-9a-hj-km-np-tv-z]{24}$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^gp[0-9a-hj-km-np-tv-z]{24}$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for GithubRepositoryId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for GithubRepositoryId {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for GithubRepositoryId {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`InferenceApiKey`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct InferenceApiKey {
        ///ID of the inference API key
        pub id: InferenceApiKeyId,
        ///inference API key data
        pub key: ::std::string::String,
    }

    ///ID of the inference API key
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct InferenceApiKeyId(::std::string::String);
    impl ::std::ops::Deref for InferenceApiKeyId {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<InferenceApiKeyId> for ::std::string::String {
        fn from(value: InferenceApiKeyId) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for InferenceApiKeyId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^ak[0-9a-hj-km-np-tv-z]{24}$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^ak[0-9a-hj-km-np-tv-z]{24}$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for InferenceApiKeyId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for InferenceApiKeyId {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for InferenceApiKeyId {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`InferenceEndpoint`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct InferenceEndpoint {
        ///Display name of the model
        pub display_name: ::std::string::String,
        ///ID of the inference endpoint
        pub id: ::std::string::String,
        ///Model name of the inference endpoint
        pub model_name: ::std::string::String,
        ///Name of the model
        pub name: ::std::string::String,
        pub price: InferenceEndpointPrice,
        ///Tags of the inference endpoint
        pub tags: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ///URL of the inference endpoint
        pub url: ::std::string::String,
    }

    ///`InferenceEndpointPrice`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct InferenceEndpointPrice {
        ///Price per million completion tokens in USD
        #[serde(deserialize_with = "::std::option::Option::deserialize")]
        pub per_million_completion_tokens: ::std::option::Option<f64>,
        ///Price per million prompt tokens in USD
        #[serde(deserialize_with = "::std::option::Option::deserialize")]
        pub per_million_prompt_tokens: ::std::option::Option<f64>,
    }

    ///Raw YAML content of a Kubernetes cluster configuration file
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    #[serde(transparent)]
    pub struct Kubeconfig(pub ::std::string::String);
    impl ::std::ops::Deref for Kubeconfig {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<Kubeconfig> for ::std::string::String {
        fn from(value: Kubeconfig) -> Self {
            value.0
        }
    }

    impl ::std::convert::From<::std::string::String> for Kubeconfig {
        fn from(value: ::std::string::String) -> Self {
            Self(value)
        }
    }

    impl ::std::fmt::Display for Kubeconfig {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            self.0.fmt(f)
        }
    }

    impl ::std::str::FromStr for Kubeconfig {
        type Err = ::std::convert::Infallible;
        fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
            Ok(Self(value.to_string()))
        }
    }

    ///`KubernetesCluster`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct KubernetesCluster {
        ///Number of control plane nodes
        pub cp_node_count: i64,
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub cp_vms: ::std::vec::Vec<Vm>,
        ///State of the KubernetesCluster
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub display_state: ::std::option::Option<::std::string::String>,
        ///ID of the KubernetesCluster
        pub id: KubernetesClusterId,
        ///Location of the KubernetesCluster
        pub location: ::std::string::String,
        ///Name of the KubernetesCluster
        pub name: ::std::string::String,
        ///Size of each control plane node
        pub node_size: ::std::string::String,
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub nodepools: ::std::vec::Vec<KubernetesNodepool>,
        ///Services
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub services_load_balancer_url: ::std::option::Option<::std::string::String>,
        ///Version of the KubernetesCluster
        pub version: ::std::string::String,
    }

    ///ID of the KubernetesCluster
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct KubernetesClusterId(::std::string::String);
    impl ::std::ops::Deref for KubernetesClusterId {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<KubernetesClusterId> for ::std::string::String {
        fn from(value: KubernetesClusterId) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for KubernetesClusterId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^kc[0-9a-hj-km-np-tv-z]{24}$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^kc[0-9a-hj-km-np-tv-z]{24}$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for KubernetesClusterId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for KubernetesClusterId {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for KubernetesClusterId {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`KubernetesNodepool`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct KubernetesNodepool {
        ///ID of the KubernetesNodepool
        pub id: KubernetesNodepoolId,
        ///Parent KubernetesCluster id
        pub kubernetes_cluster_id: KubernetesNodepoolKubernetesClusterId,
        ///Name of the KubernetesNodepool
        pub name: ::std::string::String,
        ///Number of worker nodes
        pub node_count: i64,
        ///Size of each worker node
        pub node_size: ::std::string::String,
        ///Version of the KubernetesNodepool
        pub version: ::std::string::String,
        ///Worker VMs definition
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub vms: ::std::vec::Vec<Vm>,
    }

    ///ID of the KubernetesNodepool
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct KubernetesNodepoolId(::std::string::String);
    impl ::std::ops::Deref for KubernetesNodepoolId {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<KubernetesNodepoolId> for ::std::string::String {
        fn from(value: KubernetesNodepoolId) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for KubernetesNodepoolId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^kn[0-9a-hj-km-np-tv-z]{24}$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^kn[0-9a-hj-km-np-tv-z]{24}$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for KubernetesNodepoolId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for KubernetesNodepoolId {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for KubernetesNodepoolId {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///Parent KubernetesCluster id
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct KubernetesNodepoolKubernetesClusterId(::std::string::String);
    impl ::std::ops::Deref for KubernetesNodepoolKubernetesClusterId {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<KubernetesNodepoolKubernetesClusterId> for ::std::string::String {
        fn from(value: KubernetesNodepoolKubernetesClusterId) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for KubernetesNodepoolKubernetesClusterId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^kc[0-9a-hj-km-np-tv-z]{24}$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^kc[0-9a-hj-km-np-tv-z]{24}$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for KubernetesNodepoolKubernetesClusterId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for KubernetesNodepoolKubernetesClusterId {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for KubernetesNodepoolKubernetesClusterId {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`ListInferenceEndpointsProjectId`
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct ListInferenceEndpointsProjectId(::std::string::String);
    impl ::std::ops::Deref for ListInferenceEndpointsProjectId {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<ListInferenceEndpointsProjectId> for ::std::string::String {
        fn from(value: ListInferenceEndpointsProjectId) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for ListInferenceEndpointsProjectId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^pj[0-9a-hj-km-np-tv-z]{24}$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^pj[0-9a-hj-km-np-tv-z]{24}$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for ListInferenceEndpointsProjectId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for ListInferenceEndpointsProjectId {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for ListInferenceEndpointsProjectId {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`ListInferenceEndpointsResponse`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct ListInferenceEndpointsResponse {
        pub items: ::std::vec::Vec<InferenceEndpoint>,
    }

    ///`ListLoadBalancersProjectId`
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct ListLoadBalancersProjectId(::std::string::String);
    impl ::std::ops::Deref for ListLoadBalancersProjectId {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<ListLoadBalancersProjectId> for ::std::string::String {
        fn from(value: ListLoadBalancersProjectId) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for ListLoadBalancersProjectId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^pj[0-9a-hj-km-np-tv-z]{24}$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^pj[0-9a-hj-km-np-tv-z]{24}$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for ListLoadBalancersProjectId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for ListLoadBalancersProjectId {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for ListLoadBalancersProjectId {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`ListLoadBalancersResponse`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct ListLoadBalancersResponse {
        pub count: i64,
        pub items: ::std::vec::Vec<LoadBalancer>,
    }

    ///`ListLocationKubernetesClustersProjectId`
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct ListLocationKubernetesClustersProjectId(::std::string::String);
    impl ::std::ops::Deref for ListLocationKubernetesClustersProjectId {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<ListLocationKubernetesClustersProjectId> for ::std::string::String {
        fn from(value: ListLocationKubernetesClustersProjectId) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for ListLocationKubernetesClustersProjectId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^pj[0-9a-hj-km-np-tv-z]{24}$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^pj[0-9a-hj-km-np-tv-z]{24}$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for ListLocationKubernetesClustersProjectId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for ListLocationKubernetesClustersProjectId {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for ListLocationKubernetesClustersProjectId {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`ListLocationKubernetesClustersResponse`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct ListLocationKubernetesClustersResponse {
        pub count: i64,
        pub items: ::std::vec::Vec<KubernetesCluster>,
    }

    ///`ListLocationLoadBalancersProjectId`
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct ListLocationLoadBalancersProjectId(::std::string::String);
    impl ::std::ops::Deref for ListLocationLoadBalancersProjectId {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<ListLocationLoadBalancersProjectId> for ::std::string::String {
        fn from(value: ListLocationLoadBalancersProjectId) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for ListLocationLoadBalancersProjectId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^pj[0-9a-hj-km-np-tv-z]{24}$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^pj[0-9a-hj-km-np-tv-z]{24}$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for ListLocationLoadBalancersProjectId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for ListLocationLoadBalancersProjectId {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for ListLocationLoadBalancersProjectId {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`ListLocationLoadBalancersResponse`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct ListLocationLoadBalancersResponse {
        pub count: i64,
        pub items: ::std::vec::Vec<LoadBalancer>,
    }

    ///`ListLocationMachineImagesProjectId`
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct ListLocationMachineImagesProjectId(::std::string::String);
    impl ::std::ops::Deref for ListLocationMachineImagesProjectId {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<ListLocationMachineImagesProjectId> for ::std::string::String {
        fn from(value: ListLocationMachineImagesProjectId) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for ListLocationMachineImagesProjectId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^pj[0-9a-hj-km-np-tv-z]{24}$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^pj[0-9a-hj-km-np-tv-z]{24}$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for ListLocationMachineImagesProjectId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for ListLocationMachineImagesProjectId {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for ListLocationMachineImagesProjectId {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`ListLocationMachineImagesResponse`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct ListLocationMachineImagesResponse {
        pub count: i64,
        pub items: ::std::vec::Vec<MachineImage>,
    }

    ///`ListLocationPostgresDatabasesProjectId`
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct ListLocationPostgresDatabasesProjectId(::std::string::String);
    impl ::std::ops::Deref for ListLocationPostgresDatabasesProjectId {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<ListLocationPostgresDatabasesProjectId> for ::std::string::String {
        fn from(value: ListLocationPostgresDatabasesProjectId) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for ListLocationPostgresDatabasesProjectId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^pj[0-9a-hj-km-np-tv-z]{24}$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^pj[0-9a-hj-km-np-tv-z]{24}$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for ListLocationPostgresDatabasesProjectId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for ListLocationPostgresDatabasesProjectId {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for ListLocationPostgresDatabasesProjectId {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`ListLocationPostgresDatabasesResponse`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct ListLocationPostgresDatabasesResponse {
        pub count: i64,
        pub items: ::std::vec::Vec<PostgresDatabase>,
    }

    ///`ListLocationPostgresFirewallRulesProjectId`
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct ListLocationPostgresFirewallRulesProjectId(::std::string::String);
    impl ::std::ops::Deref for ListLocationPostgresFirewallRulesProjectId {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<ListLocationPostgresFirewallRulesProjectId> for ::std::string::String {
        fn from(value: ListLocationPostgresFirewallRulesProjectId) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for ListLocationPostgresFirewallRulesProjectId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^pj[0-9a-hj-km-np-tv-z]{24}$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^pj[0-9a-hj-km-np-tv-z]{24}$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for ListLocationPostgresFirewallRulesProjectId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for ListLocationPostgresFirewallRulesProjectId {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for ListLocationPostgresFirewallRulesProjectId {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`ListLocationPostgresFirewallRulesResponse`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct ListLocationPostgresFirewallRulesResponse {
        pub count: i64,
        pub items: ::std::vec::Vec<PostgresFirewallRule>,
    }

    ///`ListLocationPrivateSubnetsProjectId`
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct ListLocationPrivateSubnetsProjectId(::std::string::String);
    impl ::std::ops::Deref for ListLocationPrivateSubnetsProjectId {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<ListLocationPrivateSubnetsProjectId> for ::std::string::String {
        fn from(value: ListLocationPrivateSubnetsProjectId) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for ListLocationPrivateSubnetsProjectId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^pj[0-9a-hj-km-np-tv-z]{24}$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^pj[0-9a-hj-km-np-tv-z]{24}$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for ListLocationPrivateSubnetsProjectId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for ListLocationPrivateSubnetsProjectId {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for ListLocationPrivateSubnetsProjectId {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`ListLocationPrivateSubnetsResponse`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct ListLocationPrivateSubnetsResponse {
        pub count: i64,
        pub items: ::std::vec::Vec<PrivateSubnet>,
    }

    ///`ListLocationVMsProjectId`
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct ListLocationVMsProjectId(::std::string::String);
    impl ::std::ops::Deref for ListLocationVMsProjectId {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<ListLocationVMsProjectId> for ::std::string::String {
        fn from(value: ListLocationVMsProjectId) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for ListLocationVMsProjectId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^pj[0-9a-hj-km-np-tv-z]{24}$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^pj[0-9a-hj-km-np-tv-z]{24}$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for ListLocationVMsProjectId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for ListLocationVMsProjectId {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for ListLocationVMsProjectId {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`ListLocationVMsResponse`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct ListLocationVMsResponse {
        pub count: i64,
        pub items: ::std::vec::Vec<Vm>,
    }

    ///`ListMachineImageVersionsProjectId`
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct ListMachineImageVersionsProjectId(::std::string::String);
    impl ::std::ops::Deref for ListMachineImageVersionsProjectId {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<ListMachineImageVersionsProjectId> for ::std::string::String {
        fn from(value: ListMachineImageVersionsProjectId) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for ListMachineImageVersionsProjectId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^pj[0-9a-hj-km-np-tv-z]{24}$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^pj[0-9a-hj-km-np-tv-z]{24}$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for ListMachineImageVersionsProjectId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for ListMachineImageVersionsProjectId {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for ListMachineImageVersionsProjectId {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`ListMachineImageVersionsResponse`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct ListMachineImageVersionsResponse {
        pub count: i64,
        pub items: ::std::vec::Vec<MachineImageVersion>,
    }

    ///`ListPSsProjectId`
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct ListPSsProjectId(::std::string::String);
    impl ::std::ops::Deref for ListPSsProjectId {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<ListPSsProjectId> for ::std::string::String {
        fn from(value: ListPSsProjectId) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for ListPSsProjectId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^pj[0-9a-hj-km-np-tv-z]{24}$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^pj[0-9a-hj-km-np-tv-z]{24}$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for ListPSsProjectId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for ListPSsProjectId {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for ListPSsProjectId {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`ListPSsResponse`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct ListPSsResponse {
        pub count: i64,
        pub items: ::std::vec::Vec<PrivateSubnet>,
    }

    ///`ListPostgresDatabaseBackupsProjectId`
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct ListPostgresDatabaseBackupsProjectId(::std::string::String);
    impl ::std::ops::Deref for ListPostgresDatabaseBackupsProjectId {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<ListPostgresDatabaseBackupsProjectId> for ::std::string::String {
        fn from(value: ListPostgresDatabaseBackupsProjectId) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for ListPostgresDatabaseBackupsProjectId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^pj[0-9a-hj-km-np-tv-z]{24}$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^pj[0-9a-hj-km-np-tv-z]{24}$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for ListPostgresDatabaseBackupsProjectId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for ListPostgresDatabaseBackupsProjectId {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for ListPostgresDatabaseBackupsProjectId {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`ListPostgresDatabaseBackupsResponse`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct ListPostgresDatabaseBackupsResponse {
        pub count: i64,
        pub items: ::std::vec::Vec<PostgresBackup>,
    }

    ///`ListPostgresDatabaseServersProjectId`
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct ListPostgresDatabaseServersProjectId(::std::string::String);
    impl ::std::ops::Deref for ListPostgresDatabaseServersProjectId {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<ListPostgresDatabaseServersProjectId> for ::std::string::String {
        fn from(value: ListPostgresDatabaseServersProjectId) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for ListPostgresDatabaseServersProjectId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^pj[0-9a-hj-km-np-tv-z]{24}$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^pj[0-9a-hj-km-np-tv-z]{24}$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for ListPostgresDatabaseServersProjectId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for ListPostgresDatabaseServersProjectId {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for ListPostgresDatabaseServersProjectId {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`ListPostgresDatabaseServersResponse`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct ListPostgresDatabaseServersResponse {
        pub count: i64,
        pub items: ::std::vec::Vec<PostgresServer>,
    }

    ///`ListPostgresDatabasesProjectId`
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct ListPostgresDatabasesProjectId(::std::string::String);
    impl ::std::ops::Deref for ListPostgresDatabasesProjectId {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<ListPostgresDatabasesProjectId> for ::std::string::String {
        fn from(value: ListPostgresDatabasesProjectId) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for ListPostgresDatabasesProjectId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^pj[0-9a-hj-km-np-tv-z]{24}$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^pj[0-9a-hj-km-np-tv-z]{24}$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for ListPostgresDatabasesProjectId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for ListPostgresDatabasesProjectId {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for ListPostgresDatabasesProjectId {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`ListPostgresDatabasesResponse`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct ListPostgresDatabasesResponse {
        pub count: i64,
        pub items: ::std::vec::Vec<PostgresDatabase>,
    }

    ///`ListProjectKubernetesClustersProjectId`
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct ListProjectKubernetesClustersProjectId(::std::string::String);
    impl ::std::ops::Deref for ListProjectKubernetesClustersProjectId {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<ListProjectKubernetesClustersProjectId> for ::std::string::String {
        fn from(value: ListProjectKubernetesClustersProjectId) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for ListProjectKubernetesClustersProjectId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^pj[0-9a-hj-km-np-tv-z]{24}$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^pj[0-9a-hj-km-np-tv-z]{24}$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for ListProjectKubernetesClustersProjectId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for ListProjectKubernetesClustersProjectId {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for ListProjectKubernetesClustersProjectId {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`ListProjectKubernetesClustersResponse`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct ListProjectKubernetesClustersResponse {
        pub count: i64,
        pub items: ::std::vec::Vec<KubernetesCluster>,
    }

    ///`ListProjectMachineImagesProjectId`
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct ListProjectMachineImagesProjectId(::std::string::String);
    impl ::std::ops::Deref for ListProjectMachineImagesProjectId {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<ListProjectMachineImagesProjectId> for ::std::string::String {
        fn from(value: ListProjectMachineImagesProjectId) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for ListProjectMachineImagesProjectId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^pj[0-9a-hj-km-np-tv-z]{24}$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^pj[0-9a-hj-km-np-tv-z]{24}$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for ListProjectMachineImagesProjectId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for ListProjectMachineImagesProjectId {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for ListProjectMachineImagesProjectId {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`ListProjectMachineImagesResponse`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct ListProjectMachineImagesResponse {
        pub count: i64,
        pub items: ::std::vec::Vec<MachineImage>,
    }

    ///`ListProjectVMsProjectId`
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct ListProjectVMsProjectId(::std::string::String);
    impl ::std::ops::Deref for ListProjectVMsProjectId {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<ListProjectVMsProjectId> for ::std::string::String {
        fn from(value: ListProjectVMsProjectId) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for ListProjectVMsProjectId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^pj[0-9a-hj-km-np-tv-z]{24}$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^pj[0-9a-hj-km-np-tv-z]{24}$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for ListProjectVMsProjectId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for ListProjectVMsProjectId {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for ListProjectVMsProjectId {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`ListProjectVMsResponse`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct ListProjectVMsResponse {
        pub count: i64,
        pub items: ::std::vec::Vec<Vm>,
    }

    ///`LoadBalancer`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct LoadBalancer {
        ///Algorithm of the Load Balancer
        pub algorithm: ::std::string::String,
        ///Whether SSL certificate is enabled
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub cert_enabled: ::std::option::Option<bool>,
        ///Destination port for the Load Balancer
        pub dst_port: i64,
        ///Health check endpoint URL
        pub health_check_endpoint: ::std::string::String,
        ///Health check endpoint protocol
        pub health_check_protocol: ::std::string::String,
        ///Hostname of the Load Balancer
        pub hostname: ::std::string::String,
        ///ID of the Load Balancer
        pub id: LoadBalancerId,
        ///Location of the Load Balancer
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub location: ::std::option::Option<::std::string::String>,
        ///Name of the Load Balancer
        pub name: ::std::string::String,
        ///Source port for the Load Balancer
        pub src_port: i64,
        ///Networking stack of the Load Balancer (ipv4, ipv6, or dual)
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub stack: ::std::option::Option<::std::string::String>,
    }

    ///ID of the Load Balancer
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct LoadBalancerId(::std::string::String);
    impl ::std::ops::Deref for LoadBalancerId {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<LoadBalancerId> for ::std::string::String {
        fn from(value: LoadBalancerId) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for LoadBalancerId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^1b[0-9a-hj-km-np-tv-z]{24}$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^1b[0-9a-hj-km-np-tv-z]{24}$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for LoadBalancerId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for LoadBalancerId {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for LoadBalancerId {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`MachineImage`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct MachineImage {
        ///Architecture
        pub arch: MachineImageArch,
        ///Creation timestamp
        pub created_at: ::chrono::DateTime<::chrono::offset::Utc>,
        ///ID of the machine image
        pub id: MachineImageId,
        ///Latest version label
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub latest_version: ::std::option::Option<::std::string::String>,
        ///Location of the machine image
        pub location: ::std::string::String,
        ///Name of the machine image
        pub name: ::std::string::String,
    }

    ///Architecture
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum MachineImageArch {
        #[serde(rename = "x64")]
        X64,
        #[serde(rename = "arm64")]
        Arm64,
    }

    impl ::std::fmt::Display for MachineImageArch {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::X64 => f.write_str("x64"),
                Self::Arm64 => f.write_str("arm64"),
            }
        }
    }

    impl ::std::str::FromStr for MachineImageArch {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "x64" => Ok(Self::X64),
                "arm64" => Ok(Self::Arm64),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for MachineImageArch {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for MachineImageArch {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///ID of the machine image
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct MachineImageId(::std::string::String);
    impl ::std::ops::Deref for MachineImageId {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<MachineImageId> for ::std::string::String {
        fn from(value: MachineImageId) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for MachineImageId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^m1[0-9a-hj-km-np-tv-z]{24}$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^m1[0-9a-hj-km-np-tv-z]{24}$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for MachineImageId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for MachineImageId {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for MachineImageId {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`MachineImageVersion`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct MachineImageVersion {
        ///Actual size of the image in MiB
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub actual_size_mib: ::std::option::Option<i64>,
        ///Size of the archived image in MiB
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub archive_size_mib: ::std::option::Option<i64>,
        ///Creation timestamp
        pub created_at: ::chrono::DateTime<::chrono::offset::Utc>,
        ///ID of the machine image version
        pub id: ::std::string::String,
        ///True if this is the machine image's currently-published latest
        /// version
        pub latest: bool,
        ///Lifecycle state, or null if no metal record exists
        #[serde(deserialize_with = "::std::option::Option::deserialize")]
        pub state: ::std::option::Option<MachineImageVersionState>,
        ///Version label
        pub version: ::std::string::String,
    }

    ///Lifecycle state, or null if no metal record exists
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum MachineImageVersionState {
        #[serde(rename = "creating")]
        Creating,
        #[serde(rename = "ready")]
        Ready,
        #[serde(rename = "destroying")]
        Destroying,
        #[serde(rename = "failed")]
        Failed,
    }

    impl ::std::fmt::Display for MachineImageVersionState {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Creating => f.write_str("creating"),
                Self::Ready => f.write_str("ready"),
                Self::Destroying => f.write_str("destroying"),
                Self::Failed => f.write_str("failed"),
            }
        }
    }

    impl ::std::str::FromStr for MachineImageVersionState {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "creating" => Ok(Self::Creating),
                "ready" => Ok(Self::Ready),
                "destroying" => Ok(Self::Destroying),
                "failed" => Ok(Self::Failed),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for MachineImageVersionState {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for MachineImageVersionState {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`MaintenanceWindowDay`
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum MaintenanceWindowDay {
        #[serde(rename = "mon")]
        Mon,
        #[serde(rename = "tue")]
        Tue,
        #[serde(rename = "wed")]
        Wed,
        #[serde(rename = "thu")]
        Thu,
        #[serde(rename = "fri")]
        Fri,
        #[serde(rename = "sat")]
        Sat,
        #[serde(rename = "sun")]
        Sun,
    }

    impl ::std::fmt::Display for MaintenanceWindowDay {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Mon => f.write_str("mon"),
                Self::Tue => f.write_str("tue"),
                Self::Wed => f.write_str("wed"),
                Self::Thu => f.write_str("thu"),
                Self::Fri => f.write_str("fri"),
                Self::Sat => f.write_str("sat"),
                Self::Sun => f.write_str("sun"),
            }
        }
    }

    impl ::std::str::FromStr for MaintenanceWindowDay {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "mon" => Ok(Self::Mon),
                "tue" => Ok(Self::Tue),
                "wed" => Ok(Self::Wed),
                "thu" => Ok(Self::Thu),
                "fri" => Ok(Self::Fri),
                "sat" => Ok(Self::Sat),
                "sun" => Ok(Self::Sun),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for MaintenanceWindowDay {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for MaintenanceWindowDay {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`ModifyFirewallRuleFirewallRuleId`
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct ModifyFirewallRuleFirewallRuleId(::std::string::String);
    impl ::std::ops::Deref for ModifyFirewallRuleFirewallRuleId {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<ModifyFirewallRuleFirewallRuleId> for ::std::string::String {
        fn from(value: ModifyFirewallRuleFirewallRuleId) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for ModifyFirewallRuleFirewallRuleId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^fr[0-9a-hj-km-np-tv-z]{24}$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^fr[0-9a-hj-km-np-tv-z]{24}$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for ModifyFirewallRuleFirewallRuleId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for ModifyFirewallRuleFirewallRuleId {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for ModifyFirewallRuleFirewallRuleId {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`ModifyFirewallRuleProjectId`
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct ModifyFirewallRuleProjectId(::std::string::String);
    impl ::std::ops::Deref for ModifyFirewallRuleProjectId {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<ModifyFirewallRuleProjectId> for ::std::string::String {
        fn from(value: ModifyFirewallRuleProjectId) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for ModifyFirewallRuleProjectId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^pj[0-9a-hj-km-np-tv-z]{24}$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^pj[0-9a-hj-km-np-tv-z]{24}$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for ModifyFirewallRuleProjectId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for ModifyFirewallRuleProjectId {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for ModifyFirewallRuleProjectId {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`Nic`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct Nic {
        ///ID of the NIC
        pub id: ::std::string::String,
        ///Name of the NIC
        pub name: ::std::string::String,
        ///Private IPv4 address
        pub private_ipv4: ::std::net::Ipv4Addr,
        ///Private IPv6 address
        pub private_ipv6: ::std::net::Ipv6Addr,
        ///Name of the VM
        #[serde(deserialize_with = "::std::option::Option::deserialize")]
        pub vm_name: ::std::option::Option<::std::string::String>,
    }

    ///`PatchLocationLoadBalancerBody`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct PatchLocationLoadBalancerBody {
        ///Algorithm of the Load Balancer
        pub algorithm: ::std::string::String,
        ///Whether SSL certificates are enabled for the Load Balancer
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub cert_enabled: ::std::option::Option<bool>,
        ///Destination port for the Load Balancer
        pub dst_port: i64,
        ///Health check endpoint URL
        pub health_check_endpoint: ::std::string::String,
        ///Source port for the Load Balancer
        pub src_port: i64,
        ///Networking stack of the Load Balancer (ipv4, ipv6, or dual)
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub stack: ::std::option::Option<::std::string::String>,
        ///List of VM apids for the Load Balancer
        pub vms: ::std::vec::Vec<::std::string::String>,
    }

    ///`PatchLocationLoadBalancerProjectId`
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct PatchLocationLoadBalancerProjectId(::std::string::String);
    impl ::std::ops::Deref for PatchLocationLoadBalancerProjectId {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<PatchLocationLoadBalancerProjectId> for ::std::string::String {
        fn from(value: PatchLocationLoadBalancerProjectId) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for PatchLocationLoadBalancerProjectId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^pj[0-9a-hj-km-np-tv-z]{24}$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^pj[0-9a-hj-km-np-tv-z]{24}$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for PatchLocationLoadBalancerProjectId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for PatchLocationLoadBalancerProjectId {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for PatchLocationLoadBalancerProjectId {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`PatchLocationLoadBalancerResponse`
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    #[serde(deny_unknown_fields)]
    pub enum PatchLocationLoadBalancerResponse {}
    ///`PatchLocationPostgresFirewallRuleBody`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default)]
    #[serde(deny_unknown_fields)]
    pub struct PatchLocationPostgresFirewallRuleBody {
        ///CIDR of the firewall rule
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub cidr: ::std::option::Option<::std::string::String>,
        ///Description of the firewall rule
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub description: ::std::option::Option<::std::string::String>,
    }

    ///`PatchLocationPostgresFirewallRuleFirewallRuleId`
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct PatchLocationPostgresFirewallRuleFirewallRuleId(::std::string::String);
    impl ::std::ops::Deref for PatchLocationPostgresFirewallRuleFirewallRuleId {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<PatchLocationPostgresFirewallRuleFirewallRuleId>
        for ::std::string::String
    {
        fn from(value: PatchLocationPostgresFirewallRuleFirewallRuleId) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for PatchLocationPostgresFirewallRuleFirewallRuleId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^fr[0-9a-hj-km-np-tv-z]{24}$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^fr[0-9a-hj-km-np-tv-z]{24}$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for PatchLocationPostgresFirewallRuleFirewallRuleId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for PatchLocationPostgresFirewallRuleFirewallRuleId
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for PatchLocationPostgresFirewallRuleFirewallRuleId {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`PatchLocationPostgresFirewallRuleProjectId`
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct PatchLocationPostgresFirewallRuleProjectId(::std::string::String);
    impl ::std::ops::Deref for PatchLocationPostgresFirewallRuleProjectId {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<PatchLocationPostgresFirewallRuleProjectId> for ::std::string::String {
        fn from(value: PatchLocationPostgresFirewallRuleProjectId) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for PatchLocationPostgresFirewallRuleProjectId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^pj[0-9a-hj-km-np-tv-z]{24}$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^pj[0-9a-hj-km-np-tv-z]{24}$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for PatchLocationPostgresFirewallRuleProjectId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for PatchLocationPostgresFirewallRuleProjectId {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for PatchLocationPostgresFirewallRuleProjectId {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`PatchPostgresDatabaseBody`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default)]
    #[serde(deny_unknown_fields)]
    pub struct PatchPostgresDatabaseBody {
        ///High availability type
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub ha_type: ::std::option::Option<::std::string::String>,
        ///Requested size for the underlying VM
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub size: ::std::option::Option<::std::string::String>,
        ///Requested storage size in GiB
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub storage_size: ::std::option::Option<i64>,
        ///Tags for the Postgres Database
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub tags: ::std::vec::Vec<PostgresTag>,
    }

    ///`PatchPostgresDatabaseConfigProjectId`
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct PatchPostgresDatabaseConfigProjectId(::std::string::String);
    impl ::std::ops::Deref for PatchPostgresDatabaseConfigProjectId {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<PatchPostgresDatabaseConfigProjectId> for ::std::string::String {
        fn from(value: PatchPostgresDatabaseConfigProjectId) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for PatchPostgresDatabaseConfigProjectId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^pj[0-9a-hj-km-np-tv-z]{24}$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^pj[0-9a-hj-km-np-tv-z]{24}$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for PatchPostgresDatabaseConfigProjectId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for PatchPostgresDatabaseConfigProjectId {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for PatchPostgresDatabaseConfigProjectId {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`PatchPostgresDatabaseProjectId`
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct PatchPostgresDatabaseProjectId(::std::string::String);
    impl ::std::ops::Deref for PatchPostgresDatabaseProjectId {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<PatchPostgresDatabaseProjectId> for ::std::string::String {
        fn from(value: PatchPostgresDatabaseProjectId) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for PatchPostgresDatabaseProjectId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^pj[0-9a-hj-km-np-tv-z]{24}$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^pj[0-9a-hj-km-np-tv-z]{24}$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for PatchPostgresDatabaseProjectId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for PatchPostgresDatabaseProjectId {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for PatchPostgresDatabaseProjectId {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`PatchPostgresDatabaseResponse`
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    #[serde(deny_unknown_fields)]
    pub enum PatchPostgresDatabaseResponse {}
    ///`PostgresBackup`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct PostgresBackup {
        ///Backup key/identifier
        pub key: ::std::string::String,
        ///Last modified timestamp of the backup
        pub last_modified: ::chrono::DateTime<::chrono::offset::Utc>,
    }

    ///`PostgresBackupCredentials`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct PostgresBackupCredentials {
        ///Temporary access key ID
        pub access_key_id: ::std::string::String,
        ///Name of the bucket backups are stored in
        pub bucket: ::std::string::String,
        ///Object storage endpoint URL
        pub endpoint: ::std::string::String,
        ///When these temporary credentials expire
        pub expiration: ::chrono::DateTime<::chrono::offset::Utc>,
        ///Object storage region
        pub region: ::std::string::String,
        ///Temporary secret access key
        pub secret_access_key: ::std::string::String,
        ///Temporary session token, required alongside the access key ID and
        /// secret access key
        pub session_token: ::std::string::String,
    }

    ///Tree of valid option combinations and metadata for each option value.
    /// The option_tree encodes parent-child dependencies: traverse from flavor
    /// -> location -> family -> size -> storage_size -> ha_type (with version
    /// branching from flavor). Only combinations present in the tree are valid
    /// for provisioning.
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct PostgresCapabilities {
        pub metadata: PostgresCapabilitiesMetadata,
        ///Nested object where each level maps option values to their valid
        /// children.
        pub option_tree: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    }

    ///Display info for option values referenced in the tree
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct PostgresCapabilitiesMetadata {
        pub family: ::std::collections::HashMap<
            ::std::string::String,
            PostgresCapabilitiesMetadataFamilyValue,
        >,
        pub flavor: ::std::collections::HashMap<
            ::std::string::String,
            PostgresCapabilitiesMetadataFlavorValue,
        >,
        pub ha_type: ::std::collections::HashMap<
            ::std::string::String,
            PostgresCapabilitiesMetadataHaTypeValue,
        >,
        pub location: ::std::collections::HashMap<
            ::std::string::String,
            PostgresCapabilitiesMetadataLocationValue,
        >,
        pub size: ::std::collections::HashMap<
            ::std::string::String,
            PostgresCapabilitiesMetadataSizeValue,
        >,
    }

    ///`PostgresCapabilitiesMetadataFamilyValue`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct PostgresCapabilitiesMetadataFamilyValue {
        pub category: ::std::string::String,
        pub display_name: ::std::string::String,
    }

    ///`PostgresCapabilitiesMetadataFlavorValue`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct PostgresCapabilitiesMetadataFlavorValue {
        pub display_name: ::std::string::String,
    }

    ///`PostgresCapabilitiesMetadataHaTypeValue`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct PostgresCapabilitiesMetadataHaTypeValue {
        pub display_name: ::std::string::String,
        pub standby_count: i64,
    }

    ///`PostgresCapabilitiesMetadataLocationValue`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct PostgresCapabilitiesMetadataLocationValue {
        pub display_name: ::std::string::String,
        pub provider: ::std::string::String,
        pub ui_name: ::std::string::String,
    }

    ///`PostgresCapabilitiesMetadataSizeValue`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct PostgresCapabilitiesMetadataSizeValue {
        pub memory_gib: i64,
        pub vcpu: i64,
    }

    ///`PostgresConfig`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct PostgresConfig {
        ///System-computed configuration values based on instance size. User
        /// overrides in pg_config take precedence.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub default_pg_config: ::std::option::Option<
            ::std::collections::HashMap<::std::string::String, ::std::string::String>,
        >,
        ///Informational message about the configuration update, such as
        /// restart requirements.
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub message: ::std::option::Option<::std::string::String>,
        pub pg_config:
            ::std::collections::HashMap<::std::string::String, PostgresConfigPgConfigValue>,
        pub pgbouncer_config:
            ::std::collections::HashMap<::std::string::String, PostgresConfigPgbouncerConfigValue>,
    }

    ///`PostgresConfigPgConfigValue`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum PostgresConfigPgConfigValue {
        String(::std::string::String),
        Number(f64),
    }

    impl ::std::fmt::Display for PostgresConfigPgConfigValue {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                Self::String(x) => x.fmt(f),
                Self::Number(x) => x.fmt(f),
            }
        }
    }

    impl ::std::convert::From<f64> for PostgresConfigPgConfigValue {
        fn from(value: f64) -> Self {
            Self::Number(value)
        }
    }

    ///`PostgresConfigPgbouncerConfigValue`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum PostgresConfigPgbouncerConfigValue {
        String(::std::string::String),
        Number(f64),
    }

    impl ::std::fmt::Display for PostgresConfigPgbouncerConfigValue {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                Self::String(x) => x.fmt(f),
                Self::Number(x) => x.fmt(f),
            }
        }
    }

    impl ::std::convert::From<f64> for PostgresConfigPgbouncerConfigValue {
        fn from(value: f64) -> Self {
            Self::Number(value)
        }
    }

    ///`PostgresDatabase`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct PostgresDatabase {
        ///CA certificates of the root CA used to issue postgres server
        /// certificates
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub ca_certificates: ::std::option::Option<::std::string::String>,
        ///Creation timestamp of the Postgres database
        pub created_at: ::chrono::DateTime<::chrono::offset::Utc>,
        ///Whether the primary server is running on a fallback instance type
        pub fallback_active: bool,
        ///Deprecated. Always standard.
        pub flavor: ::std::string::String,
        ///High availability type
        pub ha_type: ::std::string::String,
        ///ID of the Postgres database
        pub id: PostgresDatabaseId,
        ///Location of the Postgres database
        pub location: ::std::string::String,
        ///Days of the week the maintenance window applies to (empty list means
        /// every day)
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub maintenance_window_days: ::std::vec::Vec<MaintenanceWindowDay>,
        ///Maintenance window start time
        #[serde(deserialize_with = "::std::option::Option::deserialize")]
        pub maintenance_window_start_at: ::std::option::Option<i64>,
        ///Name of the Postgres database
        pub name: ::std::string::String,
        ///Parent Postgres database
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub parent: ::std::option::Option<::std::string::String>,
        ///If the database is a read replica or not
        pub read_replica: bool,
        ///State of the Postgres database
        pub state: ::std::string::String,
        ///Storage size in GiB
        pub storage_size_gib: i64,
        ///Tags of the Postgres database
        pub tags: ::std::vec::Vec<PostgresTag>,
        ///Target number of servers (primary + standbys)
        pub target_server_count: i64,
        ///Desired storage size in GiB
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub target_storage_size_gib: ::std::option::Option<i64>,
        ///Target Postgres version
        pub target_version: PostgresDatabaseTargetVersion,
        ///Desired VM size
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub target_vm_size: ::std::option::Option<::std::string::String>,
        ///Current Postgres version
        pub version: PostgresDatabaseVersion,
        ///Size of the underlying VM
        pub vm_size: ::std::string::String,
    }

    ///ID of the Postgres database
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct PostgresDatabaseId(::std::string::String);
    impl ::std::ops::Deref for PostgresDatabaseId {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<PostgresDatabaseId> for ::std::string::String {
        fn from(value: PostgresDatabaseId) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for PostgresDatabaseId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^pg[0-9a-hj-km-np-tv-z]{24}$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^pg[0-9a-hj-km-np-tv-z]{24}$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for PostgresDatabaseId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for PostgresDatabaseId {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for PostgresDatabaseId {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`PostgresDatabaseLogEntry`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct PostgresDatabaseLogEntry {
        ///Additional context fields parsed from the log entry (e.g.
        /// remote_host_port, dbname, pid, user)
        #[serde(
            default,
            skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
        )]
        pub context: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
        ///Log message body
        pub message: ::std::string::String,
        pub server_role: PostgresDatabaseLogEntryServerRole,
        ///UBID of the server that produced this log entry
        pub server_ubid: PostgresDatabaseLogEntryServerUbid,
        ///Log severity level
        pub severity_level: PostgresDatabaseLogEntrySeverityLevel,
        pub stream_name: PostgresDatabaseLogEntryStreamName,
        ///Log entry timestamp in RFC 3339 format
        pub timestamp: ::chrono::DateTime<::chrono::offset::Utc>,
    }

    ///`PostgresDatabaseLogEntryServerRole`
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum PostgresDatabaseLogEntryServerRole {
        #[serde(rename = "primary")]
        Primary,
        #[serde(rename = "standby")]
        Standby,
    }

    impl ::std::fmt::Display for PostgresDatabaseLogEntryServerRole {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Primary => f.write_str("primary"),
                Self::Standby => f.write_str("standby"),
            }
        }
    }

    impl ::std::str::FromStr for PostgresDatabaseLogEntryServerRole {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "primary" => Ok(Self::Primary),
                "standby" => Ok(Self::Standby),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for PostgresDatabaseLogEntryServerRole {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for PostgresDatabaseLogEntryServerRole {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///UBID of the server that produced this log entry
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct PostgresDatabaseLogEntryServerUbid(::std::string::String);
    impl ::std::ops::Deref for PostgresDatabaseLogEntryServerUbid {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<PostgresDatabaseLogEntryServerUbid> for ::std::string::String {
        fn from(value: PostgresDatabaseLogEntryServerUbid) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for PostgresDatabaseLogEntryServerUbid {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^pv[0-9a-hj-km-np-tv-z]{24}$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^pv[0-9a-hj-km-np-tv-z]{24}$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for PostgresDatabaseLogEntryServerUbid {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for PostgresDatabaseLogEntryServerUbid {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for PostgresDatabaseLogEntryServerUbid {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///Log severity level
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum PostgresDatabaseLogEntrySeverityLevel {
        #[serde(rename = "UNSPECIFIED")]
        Unspecified,
        #[serde(rename = "DEBUG")]
        Debug,
        #[serde(rename = "INFO")]
        Info,
        #[serde(rename = "WARN")]
        Warn,
        #[serde(rename = "ERROR")]
        Error,
        #[serde(rename = "FATAL")]
        Fatal,
    }

    impl ::std::fmt::Display for PostgresDatabaseLogEntrySeverityLevel {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Unspecified => f.write_str("UNSPECIFIED"),
                Self::Debug => f.write_str("DEBUG"),
                Self::Info => f.write_str("INFO"),
                Self::Warn => f.write_str("WARN"),
                Self::Error => f.write_str("ERROR"),
                Self::Fatal => f.write_str("FATAL"),
            }
        }
    }

    impl ::std::str::FromStr for PostgresDatabaseLogEntrySeverityLevel {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "UNSPECIFIED" => Ok(Self::Unspecified),
                "DEBUG" => Ok(Self::Debug),
                "INFO" => Ok(Self::Info),
                "WARN" => Ok(Self::Warn),
                "ERROR" => Ok(Self::Error),
                "FATAL" => Ok(Self::Fatal),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for PostgresDatabaseLogEntrySeverityLevel {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for PostgresDatabaseLogEntrySeverityLevel {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`PostgresDatabaseLogEntryStreamName`
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum PostgresDatabaseLogEntryStreamName {
        #[serde(rename = "postgres")]
        Postgres,
        #[serde(rename = "pgbouncer")]
        Pgbouncer,
        #[serde(rename = "upgrade")]
        Upgrade,
    }

    impl ::std::fmt::Display for PostgresDatabaseLogEntryStreamName {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Postgres => f.write_str("postgres"),
                Self::Pgbouncer => f.write_str("pgbouncer"),
                Self::Upgrade => f.write_str("upgrade"),
            }
        }
    }

    impl ::std::str::FromStr for PostgresDatabaseLogEntryStreamName {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "postgres" => Ok(Self::Postgres),
                "pgbouncer" => Ok(Self::Pgbouncer),
                "upgrade" => Ok(Self::Upgrade),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for PostgresDatabaseLogEntryStreamName {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for PostgresDatabaseLogEntryStreamName {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///Target Postgres version
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum PostgresDatabaseTargetVersion {
        #[serde(rename = "16")]
        X16,
        #[serde(rename = "17")]
        X17,
        #[serde(rename = "18")]
        X18,
    }

    impl ::std::fmt::Display for PostgresDatabaseTargetVersion {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::X16 => f.write_str("16"),
                Self::X17 => f.write_str("17"),
                Self::X18 => f.write_str("18"),
            }
        }
    }

    impl ::std::str::FromStr for PostgresDatabaseTargetVersion {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "16" => Ok(Self::X16),
                "17" => Ok(Self::X17),
                "18" => Ok(Self::X18),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for PostgresDatabaseTargetVersion {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for PostgresDatabaseTargetVersion {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///Current Postgres version
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum PostgresDatabaseVersion {
        #[serde(rename = "16")]
        X16,
        #[serde(rename = "17")]
        X17,
        #[serde(rename = "18")]
        X18,
    }

    impl ::std::fmt::Display for PostgresDatabaseVersion {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::X16 => f.write_str("16"),
                Self::X17 => f.write_str("17"),
                Self::X18 => f.write_str("18"),
            }
        }
    }

    impl ::std::str::FromStr for PostgresDatabaseVersion {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "16" => Ok(Self::X16),
                "17" => Ok(Self::X17),
                "18" => Ok(Self::X18),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for PostgresDatabaseVersion {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for PostgresDatabaseVersion {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`PostgresFirewallRule`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct PostgresFirewallRule {
        ///CIDR of the firewall rule
        pub cidr: ::std::string::String,
        ///Port information for the firewall rule
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub description: ::std::option::Option<::std::string::String>,
        ///ID of the firewall rule
        pub id: PostgresFirewallRuleId,
        ///Port for the Postgres firewall rule
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub port: ::std::option::Option<i64>,
    }

    ///ID of the firewall rule
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct PostgresFirewallRuleId(::std::string::String);
    impl ::std::ops::Deref for PostgresFirewallRuleId {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<PostgresFirewallRuleId> for ::std::string::String {
        fn from(value: PostgresFirewallRuleId) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for PostgresFirewallRuleId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^fr[0-9a-hj-km-np-tv-z]{24}$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^fr[0-9a-hj-km-np-tv-z]{24}$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for PostgresFirewallRuleId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for PostgresFirewallRuleId {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for PostgresFirewallRuleId {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`PostgresLogDestination`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct PostgresLogDestination {
        ///ID of the log destination
        pub id: PostgresLogDestinationId,
        ///Name of the log destination
        pub name: ::std::string::String,
        ///Destination type: "otlp" or "syslog"
        #[serde(rename = "type")]
        pub type_: PostgresLogDestinationType,
        ///Destination URL
        pub url: ::std::string::String,
    }

    ///ID of the log destination
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct PostgresLogDestinationId(::std::string::String);
    impl ::std::ops::Deref for PostgresLogDestinationId {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<PostgresLogDestinationId> for ::std::string::String {
        fn from(value: PostgresLogDestinationId) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for PostgresLogDestinationId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^1d[0-9a-hj-km-np-tv-z]{24}$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^1d[0-9a-hj-km-np-tv-z]{24}$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for PostgresLogDestinationId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for PostgresLogDestinationId {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for PostgresLogDestinationId {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///Destination type: "otlp" or "syslog"
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum PostgresLogDestinationType {
        #[serde(rename = "otlp")]
        Otlp,
        #[serde(rename = "syslog")]
        Syslog,
    }

    impl ::std::fmt::Display for PostgresLogDestinationType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Otlp => f.write_str("otlp"),
                Self::Syslog => f.write_str("syslog"),
            }
        }
    }

    impl ::std::str::FromStr for PostgresLogDestinationType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "otlp" => Ok(Self::Otlp),
                "syslog" => Ok(Self::Syslog),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for PostgresLogDestinationType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for PostgresLogDestinationType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`PostgresPatchConfig`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(untagged, deny_unknown_fields)]
    pub enum PostgresPatchConfig {
        Variant0 {
            pg_config: ::std::collections::HashMap<
                ::std::string::String,
                ::std::option::Option<::std::string::String>,
            >,
            #[serde(
                default,
                skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
            )]
            pgbouncer_config: ::std::collections::HashMap<
                ::std::string::String,
                ::std::option::Option<::std::string::String>,
            >,
        },
        Variant1 {
            pgbouncer_config: ::std::collections::HashMap<
                ::std::string::String,
                ::std::option::Option<::std::string::String>,
            >,
        },
    }

    ///`PostgresServer`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct PostgresServer {
        ///Whether the server is running on a fallback instance type
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub fallback_active: ::std::option::Option<bool>,
        ///ID of the server
        pub id: ::std::string::String,
        ///Role of the server (primary or standby)
        pub role: PostgresServerRole,
        ///Current state of the server
        pub state: PostgresServerState,
        ///Synchronization status of the server
        pub synchronization_status: PostgresServerSynchronizationStatus,
        ///Actual VM size of the server
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub vm_size: ::std::option::Option<::std::string::String>,
    }

    ///Role of the server (primary or standby)
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum PostgresServerRole {
        #[serde(rename = "primary")]
        Primary,
        #[serde(rename = "standby")]
        Standby,
    }

    impl ::std::fmt::Display for PostgresServerRole {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Primary => f.write_str("primary"),
                Self::Standby => f.write_str("standby"),
            }
        }
    }

    impl ::std::str::FromStr for PostgresServerRole {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "primary" => Ok(Self::Primary),
                "standby" => Ok(Self::Standby),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for PostgresServerRole {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for PostgresServerRole {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///Current state of the server
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum PostgresServerState {
        #[serde(rename = "running")]
        Running,
        #[serde(rename = "creating")]
        Creating,
        #[serde(rename = "updating")]
        Updating,
        #[serde(rename = "unavailable")]
        Unavailable,
        #[serde(rename = "failing_over")]
        FailingOver,
        #[serde(rename = "restarting")]
        Restarting,
        #[serde(rename = "synchronizing")]
        Synchronizing,
        #[serde(rename = "deleting")]
        Deleting,
    }

    impl ::std::fmt::Display for PostgresServerState {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Running => f.write_str("running"),
                Self::Creating => f.write_str("creating"),
                Self::Updating => f.write_str("updating"),
                Self::Unavailable => f.write_str("unavailable"),
                Self::FailingOver => f.write_str("failing_over"),
                Self::Restarting => f.write_str("restarting"),
                Self::Synchronizing => f.write_str("synchronizing"),
                Self::Deleting => f.write_str("deleting"),
            }
        }
    }

    impl ::std::str::FromStr for PostgresServerState {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "running" => Ok(Self::Running),
                "creating" => Ok(Self::Creating),
                "updating" => Ok(Self::Updating),
                "unavailable" => Ok(Self::Unavailable),
                "failing_over" => Ok(Self::FailingOver),
                "restarting" => Ok(Self::Restarting),
                "synchronizing" => Ok(Self::Synchronizing),
                "deleting" => Ok(Self::Deleting),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for PostgresServerState {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for PostgresServerState {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///Synchronization status of the server
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum PostgresServerSynchronizationStatus {
        #[serde(rename = "ready")]
        Ready,
        #[serde(rename = "catching_up")]
        CatchingUp,
    }

    impl ::std::fmt::Display for PostgresServerSynchronizationStatus {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Ready => f.write_str("ready"),
                Self::CatchingUp => f.write_str("catching_up"),
            }
        }
    }

    impl ::std::str::FromStr for PostgresServerSynchronizationStatus {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "ready" => Ok(Self::Ready),
                "catching_up" => Ok(Self::CatchingUp),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for PostgresServerSynchronizationStatus {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for PostgresServerSynchronizationStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`PostgresTag`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct PostgresTag {
        ///Key of the Postgres tag
        pub key: ::std::string::String,
        ///Value of the Postgres tag
        pub value: ::std::string::String,
    }

    ///`PrivateLocation`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct PrivateLocation {
        ///ID of the private location
        pub id: ::std::string::String,
        ///Name of the private location
        pub name: ::std::string::String,
        ///Path of the private location
        pub path: ::std::string::String,
        ///Name of the private location
        pub provider: ::std::string::String,
        ///UI name of the private location
        pub ui_name: ::std::string::String,
    }

    ///`PrivateSubnet`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct PrivateSubnet {
        pub firewalls: ::std::vec::Vec<Firewall>,
        ///ID of the subnet
        pub id: PrivateSubnetId,
        ///Location of the subnet
        pub location: ::std::string::String,
        ///Name of the subnet
        pub name: ::std::string::String,
        ///IPv4 CIDR of the subnet
        pub net4: ::std::string::String,
        ///IPv6 CIDR of the subnet
        pub net6: ::std::string::String,
        ///List of NICs
        pub nics: ::std::vec::Vec<Nic>,
        ///State of the subnet
        pub state: ::std::string::String,
    }

    ///ID of the subnet
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct PrivateSubnetId(::std::string::String);
    impl ::std::ops::Deref for PrivateSubnetId {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<PrivateSubnetId> for ::std::string::String {
        fn from(value: PrivateSubnetId) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for PrivateSubnetId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^ps[0-9a-hj-km-np-tv-z]{24}$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^ps[0-9a-hj-km-np-tv-z]{24}$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for PrivateSubnetId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for PrivateSubnetId {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for PrivateSubnetId {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`Project`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct Project {
        pub id: ProjectId,
        ///Name of the project
        pub name: ::std::string::String,
    }

    ///`ProjectId`
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct ProjectId(::std::string::String);
    impl ::std::ops::Deref for ProjectId {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<ProjectId> for ::std::string::String {
        fn from(value: ProjectId) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for ProjectId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^pj[0-9a-hj-km-np-tv-z]{24}$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^pj[0-9a-hj-km-np-tv-z]{24}$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for ProjectId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for ProjectId {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for ProjectId {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`PromotePostgresDatabaseProjectId`
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct PromotePostgresDatabaseProjectId(::std::string::String);
    impl ::std::ops::Deref for PromotePostgresDatabaseProjectId {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<PromotePostgresDatabaseProjectId> for ::std::string::String {
        fn from(value: PromotePostgresDatabaseProjectId) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for PromotePostgresDatabaseProjectId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^pj[0-9a-hj-km-np-tv-z]{24}$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^pj[0-9a-hj-km-np-tv-z]{24}$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for PromotePostgresDatabaseProjectId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for PromotePostgresDatabaseProjectId {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for PromotePostgresDatabaseProjectId {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`PromotePostgresDatabaseResponse`
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    #[serde(deny_unknown_fields)]
    pub enum PromotePostgresDatabaseResponse {}
    ///`PromoteReadReplicaPostgresDatabaseProjectId`
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct PromoteReadReplicaPostgresDatabaseProjectId(::std::string::String);
    impl ::std::ops::Deref for PromoteReadReplicaPostgresDatabaseProjectId {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<PromoteReadReplicaPostgresDatabaseProjectId> for ::std::string::String {
        fn from(value: PromoteReadReplicaPostgresDatabaseProjectId) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for PromoteReadReplicaPostgresDatabaseProjectId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^pj[0-9a-hj-km-np-tv-z]{24}$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^pj[0-9a-hj-km-np-tv-z]{24}$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for PromoteReadReplicaPostgresDatabaseProjectId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for PromoteReadReplicaPostgresDatabaseProjectId
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for PromoteReadReplicaPostgresDatabaseProjectId {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`PromoteReadReplicaPostgresDatabaseResponse`
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    #[serde(deny_unknown_fields)]
    pub enum PromoteReadReplicaPostgresDatabaseResponse {}
    ///`RecycleProjectId`
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct RecycleProjectId(::std::string::String);
    impl ::std::ops::Deref for RecycleProjectId {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<RecycleProjectId> for ::std::string::String {
        fn from(value: RecycleProjectId) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for RecycleProjectId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^pj[0-9a-hj-km-np-tv-z]{24}$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^pj[0-9a-hj-km-np-tv-z]{24}$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for RecycleProjectId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for RecycleProjectId {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for RecycleProjectId {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`RecycleResponse`
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    #[serde(deny_unknown_fields)]
    pub enum RecycleResponse {}
    ///Resource ID or name
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct Reference(::std::string::String);
    impl ::std::ops::Deref for Reference {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<Reference> for ::std::string::String {
        fn from(value: Reference) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for Reference {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^[a-z0-9](?:[a-z0-9\\-]{0,61}[a-z0-9])?$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err(
                    "doesn't match pattern \"^[a-z0-9](?:[a-z0-9\\-]{0,61}[a-z0-9])?$\"".into(),
                );
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for Reference {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for Reference {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for Reference {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`RegisterSshPublicKeyProjectId`
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct RegisterSshPublicKeyProjectId(::std::string::String);
    impl ::std::ops::Deref for RegisterSshPublicKeyProjectId {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<RegisterSshPublicKeyProjectId> for ::std::string::String {
        fn from(value: RegisterSshPublicKeyProjectId) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for RegisterSshPublicKeyProjectId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^pj[0-9a-hj-km-np-tv-z]{24}$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^pj[0-9a-hj-km-np-tv-z]{24}$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for RegisterSshPublicKeyProjectId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for RegisterSshPublicKeyProjectId {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for RegisterSshPublicKeyProjectId {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`RemoveCertAuthUserProjectId`
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct RemoveCertAuthUserProjectId(::std::string::String);
    impl ::std::ops::Deref for RemoveCertAuthUserProjectId {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<RemoveCertAuthUserProjectId> for ::std::string::String {
        fn from(value: RemoveCertAuthUserProjectId) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for RemoveCertAuthUserProjectId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^pj[0-9a-hj-km-np-tv-z]{24}$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^pj[0-9a-hj-km-np-tv-z]{24}$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for RemoveCertAuthUserProjectId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for RemoveCertAuthUserProjectId {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for RemoveCertAuthUserProjectId {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`RenameFirewallBody`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct RenameFirewallBody {
        ///New name for firewall
        pub name: ::std::string::String,
    }

    ///`RenameFirewallProjectId`
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct RenameFirewallProjectId(::std::string::String);
    impl ::std::ops::Deref for RenameFirewallProjectId {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<RenameFirewallProjectId> for ::std::string::String {
        fn from(value: RenameFirewallProjectId) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for RenameFirewallProjectId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^pj[0-9a-hj-km-np-tv-z]{24}$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^pj[0-9a-hj-km-np-tv-z]{24}$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for RenameFirewallProjectId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for RenameFirewallProjectId {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for RenameFirewallProjectId {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`RenameKubernetesClusterBody`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct RenameKubernetesClusterBody {
        ///New name for kubernetes cluster
        pub name: ::std::string::String,
    }

    ///`RenameKubernetesClusterProjectId`
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct RenameKubernetesClusterProjectId(::std::string::String);
    impl ::std::ops::Deref for RenameKubernetesClusterProjectId {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<RenameKubernetesClusterProjectId> for ::std::string::String {
        fn from(value: RenameKubernetesClusterProjectId) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for RenameKubernetesClusterProjectId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^pj[0-9a-hj-km-np-tv-z]{24}$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^pj[0-9a-hj-km-np-tv-z]{24}$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for RenameKubernetesClusterProjectId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for RenameKubernetesClusterProjectId {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for RenameKubernetesClusterProjectId {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`RenameKubernetesClusterResponse`
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    #[serde(deny_unknown_fields)]
    pub enum RenameKubernetesClusterResponse {}
    ///`RenameKubernetesNodepoolBody`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct RenameKubernetesNodepoolBody {
        ///New name for kubernetes nodepool
        pub name: ::std::string::String,
    }

    ///`RenameKubernetesNodepoolProjectId`
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct RenameKubernetesNodepoolProjectId(::std::string::String);
    impl ::std::ops::Deref for RenameKubernetesNodepoolProjectId {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<RenameKubernetesNodepoolProjectId> for ::std::string::String {
        fn from(value: RenameKubernetesNodepoolProjectId) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for RenameKubernetesNodepoolProjectId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^pj[0-9a-hj-km-np-tv-z]{24}$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^pj[0-9a-hj-km-np-tv-z]{24}$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for RenameKubernetesNodepoolProjectId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for RenameKubernetesNodepoolProjectId {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for RenameKubernetesNodepoolProjectId {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`RenameKubernetesNodepoolResponse`
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    #[serde(deny_unknown_fields)]
    pub enum RenameKubernetesNodepoolResponse {}
    ///`RenameLoadBalancerBody`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct RenameLoadBalancerBody {
        ///New name for load balancer
        pub name: ::std::string::String,
    }

    ///`RenameLoadBalancerProjectId`
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct RenameLoadBalancerProjectId(::std::string::String);
    impl ::std::ops::Deref for RenameLoadBalancerProjectId {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<RenameLoadBalancerProjectId> for ::std::string::String {
        fn from(value: RenameLoadBalancerProjectId) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for RenameLoadBalancerProjectId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^pj[0-9a-hj-km-np-tv-z]{24}$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^pj[0-9a-hj-km-np-tv-z]{24}$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for RenameLoadBalancerProjectId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for RenameLoadBalancerProjectId {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for RenameLoadBalancerProjectId {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`RenameLoadBalancerResponse`
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    #[serde(deny_unknown_fields)]
    pub enum RenameLoadBalancerResponse {}
    ///`RenameMachineImageBody`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct RenameMachineImageBody {
        ///New name for the machine image
        pub name: ::std::string::String,
    }

    ///`RenameMachineImageProjectId`
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct RenameMachineImageProjectId(::std::string::String);
    impl ::std::ops::Deref for RenameMachineImageProjectId {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<RenameMachineImageProjectId> for ::std::string::String {
        fn from(value: RenameMachineImageProjectId) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for RenameMachineImageProjectId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^pj[0-9a-hj-km-np-tv-z]{24}$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^pj[0-9a-hj-km-np-tv-z]{24}$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for RenameMachineImageProjectId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for RenameMachineImageProjectId {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for RenameMachineImageProjectId {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`RenamePostgresBody`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct RenamePostgresBody {
        ///New name for PostgreSQL database
        pub name: ::std::string::String,
    }

    ///`RenamePostgresProjectId`
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct RenamePostgresProjectId(::std::string::String);
    impl ::std::ops::Deref for RenamePostgresProjectId {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<RenamePostgresProjectId> for ::std::string::String {
        fn from(value: RenamePostgresProjectId) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for RenamePostgresProjectId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^pj[0-9a-hj-km-np-tv-z]{24}$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^pj[0-9a-hj-km-np-tv-z]{24}$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for RenamePostgresProjectId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for RenamePostgresProjectId {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for RenamePostgresProjectId {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`RenamePostgresResponse`
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    #[serde(deny_unknown_fields)]
    pub enum RenamePostgresResponse {}
    ///`RenamePrivateSubnetBody`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct RenamePrivateSubnetBody {
        ///New name for private subnet
        pub name: ::std::string::String,
    }

    ///`RenamePrivateSubnetProjectId`
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct RenamePrivateSubnetProjectId(::std::string::String);
    impl ::std::ops::Deref for RenamePrivateSubnetProjectId {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<RenamePrivateSubnetProjectId> for ::std::string::String {
        fn from(value: RenamePrivateSubnetProjectId) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for RenamePrivateSubnetProjectId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^pj[0-9a-hj-km-np-tv-z]{24}$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^pj[0-9a-hj-km-np-tv-z]{24}$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for RenamePrivateSubnetProjectId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for RenamePrivateSubnetProjectId {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for RenamePrivateSubnetProjectId {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`RenameVmBody`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct RenameVmBody {
        ///New name for VM
        pub name: ::std::string::String,
    }

    ///`RenameVmProjectId`
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct RenameVmProjectId(::std::string::String);
    impl ::std::ops::Deref for RenameVmProjectId {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<RenameVmProjectId> for ::std::string::String {
        fn from(value: RenameVmProjectId) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for RenameVmProjectId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^pj[0-9a-hj-km-np-tv-z]{24}$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^pj[0-9a-hj-km-np-tv-z]{24}$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for RenameVmProjectId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for RenameVmProjectId {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for RenameVmProjectId {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`ResetSuperuserPasswordBody`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct ResetSuperuserPasswordBody {
        pub password: ::std::string::String,
    }

    ///`ResetSuperuserPasswordProjectId`
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct ResetSuperuserPasswordProjectId(::std::string::String);
    impl ::std::ops::Deref for ResetSuperuserPasswordProjectId {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<ResetSuperuserPasswordProjectId> for ::std::string::String {
        fn from(value: ResetSuperuserPasswordProjectId) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for ResetSuperuserPasswordProjectId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^pj[0-9a-hj-km-np-tv-z]{24}$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^pj[0-9a-hj-km-np-tv-z]{24}$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for ResetSuperuserPasswordProjectId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for ResetSuperuserPasswordProjectId {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for ResetSuperuserPasswordProjectId {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`ResetSuperuserPasswordResponse`
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    #[serde(deny_unknown_fields)]
    pub enum ResetSuperuserPasswordResponse {}
    ///`ResizeKubernetesNodepoolBody`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct ResizeKubernetesNodepoolBody {
        ///New number of nodes for the kubernetes nodepool
        pub node_count: i64,
    }

    ///`ResizeKubernetesNodepoolProjectId`
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct ResizeKubernetesNodepoolProjectId(::std::string::String);
    impl ::std::ops::Deref for ResizeKubernetesNodepoolProjectId {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<ResizeKubernetesNodepoolProjectId> for ::std::string::String {
        fn from(value: ResizeKubernetesNodepoolProjectId) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for ResizeKubernetesNodepoolProjectId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^pj[0-9a-hj-km-np-tv-z]{24}$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^pj[0-9a-hj-km-np-tv-z]{24}$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for ResizeKubernetesNodepoolProjectId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for ResizeKubernetesNodepoolProjectId {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for ResizeKubernetesNodepoolProjectId {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`ResizeKubernetesNodepoolResponse`
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    #[serde(deny_unknown_fields)]
    pub enum ResizeKubernetesNodepoolResponse {}
    ///`ResourceMetrics`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct ResourceMetrics {
        pub metrics: ::std::vec::Vec<ResourceMetricsMetricsItem>,
    }

    ///`ResourceMetricsMetricsItem`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct ResourceMetricsMetricsItem {
        ///Description of the metric
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub description: ::std::option::Option<::std::string::String>,
        ///Key of the metric
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub key: ::std::option::Option<::std::string::String>,
        ///Name of the metric
        pub name: ::std::string::String,
        pub series: ::std::vec::Vec<ResourceMetricsMetricsItemSeriesItem>,
        ///Unit of measurement for the metric
        #[serde(deserialize_with = "::std::option::Option::deserialize")]
        pub unit: ::std::option::Option<::std::string::String>,
    }

    ///`ResourceMetricsMetricsItemSeriesItem`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct ResourceMetricsMetricsItemSeriesItem {
        ///Labels identifying the time series
        pub labels: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
        ///Array of timestamp-value pairs
        pub values: ::std::vec::Vec<[ResourceMetricsMetricsItemSeriesItemValuesItemItem; 2usize]>,
    }

    ///`ResourceMetricsMetricsItemSeriesItemValuesItemItem`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum ResourceMetricsMetricsItemSeriesItemValuesItemItem {
        Number(f64),
        String(::std::string::String),
    }

    impl ::std::fmt::Display for ResourceMetricsMetricsItemSeriesItemValuesItemItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                Self::Number(x) => x.fmt(f),
                Self::String(x) => x.fmt(f),
            }
        }
    }

    impl ::std::convert::From<f64> for ResourceMetricsMetricsItemSeriesItemValuesItemItem {
        fn from(value: f64) -> Self {
            Self::Number(value)
        }
    }

    ///`RestartPostgresDatabaseProjectId`
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct RestartPostgresDatabaseProjectId(::std::string::String);
    impl ::std::ops::Deref for RestartPostgresDatabaseProjectId {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<RestartPostgresDatabaseProjectId> for ::std::string::String {
        fn from(value: RestartPostgresDatabaseProjectId) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for RestartPostgresDatabaseProjectId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^pj[0-9a-hj-km-np-tv-z]{24}$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^pj[0-9a-hj-km-np-tv-z]{24}$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for RestartPostgresDatabaseProjectId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for RestartPostgresDatabaseProjectId {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for RestartPostgresDatabaseProjectId {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`RestartPostgresDatabaseResponse`
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    #[serde(deny_unknown_fields)]
    pub enum RestartPostgresDatabaseResponse {}
    ///`RestartVmProjectId`
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct RestartVmProjectId(::std::string::String);
    impl ::std::ops::Deref for RestartVmProjectId {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<RestartVmProjectId> for ::std::string::String {
        fn from(value: RestartVmProjectId) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for RestartVmProjectId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^pj[0-9a-hj-km-np-tv-z]{24}$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^pj[0-9a-hj-km-np-tv-z]{24}$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for RestartVmProjectId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for RestartVmProjectId {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for RestartVmProjectId {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`RestorePostgresDatabaseBody`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct RestorePostgresDatabaseBody {
        ///Name for the newly restored Postgres database.
        pub name: ::std::string::String,
        #[serde(
            default,
            skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
        )]
        pub pg_config: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
        #[serde(
            default,
            skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
        )]
        pub pgbouncer_config:
            ::std::collections::HashMap<::std::string::String, ::std::string::String>,
        ///The RFC 3339-formatted timestamp for the point in time at which to
        /// recover.
        pub restore_target: ::chrono::DateTime<::chrono::offset::Utc>,
        ///Tags for the Postgres Database
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub tags: ::std::vec::Vec<PostgresTag>,
    }

    ///`RestorePostgresDatabaseProjectId`
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct RestorePostgresDatabaseProjectId(::std::string::String);
    impl ::std::ops::Deref for RestorePostgresDatabaseProjectId {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<RestorePostgresDatabaseProjectId> for ::std::string::String {
        fn from(value: RestorePostgresDatabaseProjectId) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for RestorePostgresDatabaseProjectId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^pj[0-9a-hj-km-np-tv-z]{24}$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^pj[0-9a-hj-km-np-tv-z]{24}$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for RestorePostgresDatabaseProjectId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for RestorePostgresDatabaseProjectId {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for RestorePostgresDatabaseProjectId {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`RestorePostgresDatabaseResponse`
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    #[serde(deny_unknown_fields)]
    pub enum RestorePostgresDatabaseResponse {}
    ///`RetireKubernetesNodeProjectId`
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct RetireKubernetesNodeProjectId(::std::string::String);
    impl ::std::ops::Deref for RetireKubernetesNodeProjectId {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<RetireKubernetesNodeProjectId> for ::std::string::String {
        fn from(value: RetireKubernetesNodeProjectId) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for RetireKubernetesNodeProjectId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^pj[0-9a-hj-km-np-tv-z]{24}$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^pj[0-9a-hj-km-np-tv-z]{24}$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for RetireKubernetesNodeProjectId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for RetireKubernetesNodeProjectId {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for RetireKubernetesNodeProjectId {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`RetireKubernetesNodeResponse`
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    #[serde(deny_unknown_fields)]
    pub enum RetireKubernetesNodeResponse {}
    ///`RunCommand`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct RunCommand {
        ///Output of the run command, truncated to the last 64KB. Null unless
        /// status is succeeded.
        #[serde(deserialize_with = "::std::option::Option::deserialize")]
        pub output: ::std::option::Option<::std::string::String>,
        ///Status of the run command
        pub status: RunCommandStatus,
        ///Time the run command was last updated to its current status
        pub timestamp: ::chrono::DateTime<::chrono::offset::Utc>,
    }

    ///Status of the run command
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum RunCommandStatus {
        #[serde(rename = "created")]
        Created,
        #[serde(rename = "succeeded")]
        Succeeded,
        #[serde(rename = "failed")]
        Failed,
    }

    impl ::std::fmt::Display for RunCommandStatus {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Created => f.write_str("created"),
                Self::Succeeded => f.write_str("succeeded"),
                Self::Failed => f.write_str("failed"),
            }
        }
    }

    impl ::std::str::FromStr for RunCommandStatus {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "created" => Ok(Self::Created),
                "succeeded" => Ok(Self::Succeeded),
                "failed" => Ok(Self::Failed),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for RunCommandStatus {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for RunCommandStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`SearchProjectAuditLogProjectId`
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct SearchProjectAuditLogProjectId(::std::string::String);
    impl ::std::ops::Deref for SearchProjectAuditLogProjectId {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<SearchProjectAuditLogProjectId> for ::std::string::String {
        fn from(value: SearchProjectAuditLogProjectId) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for SearchProjectAuditLogProjectId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^pj[0-9a-hj-km-np-tv-z]{24}$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^pj[0-9a-hj-km-np-tv-z]{24}$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for SearchProjectAuditLogProjectId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for SearchProjectAuditLogProjectId {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for SearchProjectAuditLogProjectId {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`SearchProjectAuditLogResponse`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct SearchProjectAuditLogResponse {
        pub items: ::std::vec::Vec<AuditLog>,
        ///Key to pass to get next page (only present if additional pages)
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub pagination_key: ::std::option::Option<::std::string::String>,
    }

    ///`SetMaintenanceWindowBody`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default)]
    #[serde(deny_unknown_fields)]
    pub struct SetMaintenanceWindowBody {
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub maintenance_window_days: ::std::vec::Vec<MaintenanceWindowDay>,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub maintenance_window_start_at: ::std::option::Option<i64>,
    }

    ///`SetMaintenanceWindowProjectId`
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct SetMaintenanceWindowProjectId(::std::string::String);
    impl ::std::ops::Deref for SetMaintenanceWindowProjectId {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<SetMaintenanceWindowProjectId> for ::std::string::String {
        fn from(value: SetMaintenanceWindowProjectId) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for SetMaintenanceWindowProjectId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^pj[0-9a-hj-km-np-tv-z]{24}$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^pj[0-9a-hj-km-np-tv-z]{24}$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for SetMaintenanceWindowProjectId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for SetMaintenanceWindowProjectId {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for SetMaintenanceWindowProjectId {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`SetMaintenanceWindowResponse`
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    #[serde(deny_unknown_fields)]
    pub enum SetMaintenanceWindowResponse {}
    ///`SetVmMaintenanceWindowBody`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct SetVmMaintenanceWindowBody {
        #[serde(deserialize_with = "::std::option::Option::deserialize")]
        pub maintenance_window_start_at: ::std::option::Option<i64>,
    }

    ///`SetVmMaintenanceWindowProjectId`
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct SetVmMaintenanceWindowProjectId(::std::string::String);
    impl ::std::ops::Deref for SetVmMaintenanceWindowProjectId {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<SetVmMaintenanceWindowProjectId> for ::std::string::String {
        fn from(value: SetVmMaintenanceWindowProjectId) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for SetVmMaintenanceWindowProjectId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^pj[0-9a-hj-km-np-tv-z]{24}$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^pj[0-9a-hj-km-np-tv-z]{24}$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for SetVmMaintenanceWindowProjectId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for SetVmMaintenanceWindowProjectId {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for SetVmMaintenanceWindowProjectId {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`SshPublicKey`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct SshPublicKey {
        ///ID of the SSH public key
        pub id: SshPublicKeyId,
        ///Name of the SSH public key
        pub name: ::std::string::String,
        ///SSH public key data
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub public_key: ::std::option::Option<::std::string::String>,
    }

    ///`SshPublicKeyCreate`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct SshPublicKeyCreate {
        pub name: ::std::string::String,
        pub public_key: ::std::string::String,
    }

    ///ID of the SSH public key
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct SshPublicKeyId(::std::string::String);
    impl ::std::ops::Deref for SshPublicKeyId {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<SshPublicKeyId> for ::std::string::String {
        fn from(value: SshPublicKeyId) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for SshPublicKeyId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^sk[0-9a-hj-km-np-tv-z]{24}$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^sk[0-9a-hj-km-np-tv-z]{24}$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for SshPublicKeyId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for SshPublicKeyId {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for SshPublicKeyId {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`SshPublicKeyUpdate`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(untagged, deny_unknown_fields)]
    pub enum SshPublicKeyUpdate {
        Variant0 {
            name: ::std::string::String,
            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
            public_key: ::std::option::Option<::std::string::String>,
        },
        Variant1 {
            public_key: ::std::string::String,
        },
    }

    ///`StartVmProjectId`
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct StartVmProjectId(::std::string::String);
    impl ::std::ops::Deref for StartVmProjectId {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<StartVmProjectId> for ::std::string::String {
        fn from(value: StartVmProjectId) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for StartVmProjectId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^pj[0-9a-hj-km-np-tv-z]{24}$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^pj[0-9a-hj-km-np-tv-z]{24}$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for StartVmProjectId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for StartVmProjectId {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for StartVmProjectId {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`StopVmProjectId`
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct StopVmProjectId(::std::string::String);
    impl ::std::ops::Deref for StopVmProjectId {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<StopVmProjectId> for ::std::string::String {
        fn from(value: StopVmProjectId) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for StopVmProjectId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^pj[0-9a-hj-km-np-tv-z]{24}$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^pj[0-9a-hj-km-np-tv-z]{24}$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for StopVmProjectId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for StopVmProjectId {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for StopVmProjectId {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`ToggleSslCertificateLoadBalancerBody`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct ToggleSslCertificateLoadBalancerBody {
        ///Whether SSL certificates are enabled for the Load Balancer
        pub cert_enabled: bool,
    }

    ///`ToggleSslCertificateLoadBalancerProjectId`
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct ToggleSslCertificateLoadBalancerProjectId(::std::string::String);
    impl ::std::ops::Deref for ToggleSslCertificateLoadBalancerProjectId {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<ToggleSslCertificateLoadBalancerProjectId> for ::std::string::String {
        fn from(value: ToggleSslCertificateLoadBalancerProjectId) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for ToggleSslCertificateLoadBalancerProjectId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^pj[0-9a-hj-km-np-tv-z]{24}$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^pj[0-9a-hj-km-np-tv-z]{24}$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for ToggleSslCertificateLoadBalancerProjectId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for ToggleSslCertificateLoadBalancerProjectId {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for ToggleSslCertificateLoadBalancerProjectId {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`ToggleSslCertificateLoadBalancerResponse`
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    #[serde(deny_unknown_fields)]
    pub enum ToggleSslCertificateLoadBalancerResponse {}
    ///`UpdateFirewallBody`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct UpdateFirewallBody {
        ///New description for firewall
        pub description: ::std::string::String,
    }

    ///`UpdateFirewallProjectId`
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct UpdateFirewallProjectId(::std::string::String);
    impl ::std::ops::Deref for UpdateFirewallProjectId {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<UpdateFirewallProjectId> for ::std::string::String {
        fn from(value: UpdateFirewallProjectId) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for UpdateFirewallProjectId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^pj[0-9a-hj-km-np-tv-z]{24}$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^pj[0-9a-hj-km-np-tv-z]{24}$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for UpdateFirewallProjectId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for UpdateFirewallProjectId {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for UpdateFirewallProjectId {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`UpdateMachineImageBody`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct UpdateMachineImageBody {
        ///Version label to set as latest
        pub latest_version: ::std::string::String,
    }

    ///`UpdateMachineImageProjectId`
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct UpdateMachineImageProjectId(::std::string::String);
    impl ::std::ops::Deref for UpdateMachineImageProjectId {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<UpdateMachineImageProjectId> for ::std::string::String {
        fn from(value: UpdateMachineImageProjectId) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for UpdateMachineImageProjectId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^pj[0-9a-hj-km-np-tv-z]{24}$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^pj[0-9a-hj-km-np-tv-z]{24}$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for UpdateMachineImageProjectId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for UpdateMachineImageProjectId {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for UpdateMachineImageProjectId {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`UpdatePostgresDatabaseConfigProjectId`
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct UpdatePostgresDatabaseConfigProjectId(::std::string::String);
    impl ::std::ops::Deref for UpdatePostgresDatabaseConfigProjectId {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<UpdatePostgresDatabaseConfigProjectId> for ::std::string::String {
        fn from(value: UpdatePostgresDatabaseConfigProjectId) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for UpdatePostgresDatabaseConfigProjectId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^pj[0-9a-hj-km-np-tv-z]{24}$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^pj[0-9a-hj-km-np-tv-z]{24}$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for UpdatePostgresDatabaseConfigProjectId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for UpdatePostgresDatabaseConfigProjectId {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for UpdatePostgresDatabaseConfigProjectId {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`UpdatePrivateLocationBody`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct UpdatePrivateLocationBody {
        ///Name of the private location
        pub name: ::std::string::String,
    }

    ///`UpdatePrivateLocationProjectId`
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct UpdatePrivateLocationProjectId(::std::string::String);
    impl ::std::ops::Deref for UpdatePrivateLocationProjectId {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<UpdatePrivateLocationProjectId> for ::std::string::String {
        fn from(value: UpdatePrivateLocationProjectId) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for UpdatePrivateLocationProjectId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^pj[0-9a-hj-km-np-tv-z]{24}$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^pj[0-9a-hj-km-np-tv-z]{24}$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for UpdatePrivateLocationProjectId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for UpdatePrivateLocationProjectId {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for UpdatePrivateLocationProjectId {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`UpdateSshPublicKeyProjectId`
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct UpdateSshPublicKeyProjectId(::std::string::String);
    impl ::std::ops::Deref for UpdateSshPublicKeyProjectId {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<UpdateSshPublicKeyProjectId> for ::std::string::String {
        fn from(value: UpdateSshPublicKeyProjectId) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for UpdateSshPublicKeyProjectId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^pj[0-9a-hj-km-np-tv-z]{24}$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^pj[0-9a-hj-km-np-tv-z]{24}$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for UpdateSshPublicKeyProjectId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for UpdateSshPublicKeyProjectId {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for UpdateSshPublicKeyProjectId {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`UpgradeKubernetesClusterProjectId`
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct UpgradeKubernetesClusterProjectId(::std::string::String);
    impl ::std::ops::Deref for UpgradeKubernetesClusterProjectId {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<UpgradeKubernetesClusterProjectId> for ::std::string::String {
        fn from(value: UpgradeKubernetesClusterProjectId) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for UpgradeKubernetesClusterProjectId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^pj[0-9a-hj-km-np-tv-z]{24}$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^pj[0-9a-hj-km-np-tv-z]{24}$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for UpgradeKubernetesClusterProjectId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for UpgradeKubernetesClusterProjectId {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for UpgradeKubernetesClusterProjectId {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`UpgradeKubernetesClusterResponse`
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    #[serde(deny_unknown_fields)]
    pub enum UpgradeKubernetesClusterResponse {}
    ///`UpgradeKubernetesNodepoolProjectId`
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct UpgradeKubernetesNodepoolProjectId(::std::string::String);
    impl ::std::ops::Deref for UpgradeKubernetesNodepoolProjectId {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<UpgradeKubernetesNodepoolProjectId> for ::std::string::String {
        fn from(value: UpgradeKubernetesNodepoolProjectId) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for UpgradeKubernetesNodepoolProjectId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^pj[0-9a-hj-km-np-tv-z]{24}$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^pj[0-9a-hj-km-np-tv-z]{24}$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for UpgradeKubernetesNodepoolProjectId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for UpgradeKubernetesNodepoolProjectId {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for UpgradeKubernetesNodepoolProjectId {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`UpgradeKubernetesNodepoolResponse`
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    #[serde(deny_unknown_fields)]
    pub enum UpgradeKubernetesNodepoolResponse {}
    ///`UpgradePostgresDatabaseProjectId`
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct UpgradePostgresDatabaseProjectId(::std::string::String);
    impl ::std::ops::Deref for UpgradePostgresDatabaseProjectId {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<UpgradePostgresDatabaseProjectId> for ::std::string::String {
        fn from(value: UpgradePostgresDatabaseProjectId) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for UpgradePostgresDatabaseProjectId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^pj[0-9a-hj-km-np-tv-z]{24}$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^pj[0-9a-hj-km-np-tv-z]{24}$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for UpgradePostgresDatabaseProjectId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for UpgradePostgresDatabaseProjectId {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for UpgradePostgresDatabaseProjectId {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`UpgradePostgresDatabaseResponse`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct UpgradePostgresDatabaseResponse {
        pub current_version: UpgradePostgresDatabaseResponseCurrentVersion,
        pub target_version: UpgradePostgresDatabaseResponseTargetVersion,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub upgrade_stage: ::std::option::Option<::std::string::String>,
        pub upgrade_status: ::std::string::String,
    }

    ///`UpgradePostgresDatabaseResponseCurrentVersion`
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum UpgradePostgresDatabaseResponseCurrentVersion {
        #[serde(rename = "16")]
        X16,
        #[serde(rename = "17")]
        X17,
        #[serde(rename = "18")]
        X18,
    }

    impl ::std::fmt::Display for UpgradePostgresDatabaseResponseCurrentVersion {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::X16 => f.write_str("16"),
                Self::X17 => f.write_str("17"),
                Self::X18 => f.write_str("18"),
            }
        }
    }

    impl ::std::str::FromStr for UpgradePostgresDatabaseResponseCurrentVersion {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "16" => Ok(Self::X16),
                "17" => Ok(Self::X17),
                "18" => Ok(Self::X18),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for UpgradePostgresDatabaseResponseCurrentVersion {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for UpgradePostgresDatabaseResponseCurrentVersion
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`UpgradePostgresDatabaseResponseTargetVersion`
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum UpgradePostgresDatabaseResponseTargetVersion {
        #[serde(rename = "16")]
        X16,
        #[serde(rename = "17")]
        X17,
        #[serde(rename = "18")]
        X18,
    }

    impl ::std::fmt::Display for UpgradePostgresDatabaseResponseTargetVersion {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::X16 => f.write_str("16"),
                Self::X17 => f.write_str("17"),
                Self::X18 => f.write_str("18"),
            }
        }
    }

    impl ::std::str::FromStr for UpgradePostgresDatabaseResponseTargetVersion {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "16" => Ok(Self::X16),
                "17" => Ok(Self::X17),
                "18" => Ok(Self::X18),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for UpgradePostgresDatabaseResponseTargetVersion {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String>
        for UpgradePostgresDatabaseResponseTargetVersion
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`Vm`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct Vm {
        ///Boot image of the VM
        pub boot_image: ::std::string::String,
        ///ID of the VM
        pub id: VmId,
        ///IPv4 address
        #[serde(deserialize_with = "::std::option::Option::deserialize")]
        pub ip4: ::std::option::Option<::std::net::Ipv4Addr>,
        ///Whether IPv4 is enabled
        pub ip4_enabled: bool,
        ///IPv6 address
        #[serde(deserialize_with = "::std::option::Option::deserialize")]
        pub ip6: ::std::option::Option<::std::net::Ipv6Addr>,
        ///Location of the VM
        pub location: ::std::string::String,
        ///Maintenance window start time
        #[serde(deserialize_with = "::std::option::Option::deserialize")]
        pub maintenance_window_start_at: ::std::option::Option<i64>,
        ///Name of the VM
        pub name: ::std::string::String,
        ///Size of the underlying VM
        pub size: ::std::string::String,
        ///State of the VM
        pub state: ::std::string::String,
        ///Storage size in GiB
        pub storage_size_gib: i64,
        ///Unix user of the VM
        pub unix_user: ::std::string::String,
    }

    ///ID of the VM
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct VmId(::std::string::String);
    impl ::std::ops::Deref for VmId {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<VmId> for ::std::string::String {
        fn from(value: VmId) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for VmId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^vm[0-9a-hj-km-np-tv-z]{24}$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^vm[0-9a-hj-km-np-tv-z]{24}$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for VmId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for VmId {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for VmId {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///`VmResponse`
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct VmResponse {
        ///Boot image of the VM
        pub boot_image: ::std::string::String,
        ///List of firewalls
        pub firewalls: ::std::vec::Vec<Firewall>,
        ///GPU configuration
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub gpu: ::std::option::Option<::std::string::String>,
        ///ID of the VM
        pub id: VmResponseId,
        ///IPv4 address
        #[serde(deserialize_with = "::std::option::Option::deserialize")]
        pub ip4: ::std::option::Option<::std::net::Ipv4Addr>,
        ///Whether IPv4 is enabled
        pub ip4_enabled: bool,
        ///IPv6 address
        #[serde(deserialize_with = "::std::option::Option::deserialize")]
        pub ip6: ::std::option::Option<::std::net::Ipv6Addr>,
        ///Location of the VM
        pub location: ::std::string::String,
        ///Maintenance window start time
        #[serde(deserialize_with = "::std::option::Option::deserialize")]
        pub maintenance_window_start_at: ::std::option::Option<i64>,
        ///Name of the VM
        pub name: ::std::string::String,
        ///Private IPv4 address
        pub private_ipv4: ::std::net::Ipv4Addr,
        ///Private IPv6 address
        pub private_ipv6: ::std::net::Ipv6Addr,
        ///Size of the underlying VM
        pub size: ::std::string::String,
        ///State of the VM
        pub state: ::std::string::String,
        ///Storage size in GiB
        pub storage_size_gib: i64,
        ///Subnet of the VM
        pub subnet: ::std::string::String,
        ///Unix user of the VM
        pub unix_user: ::std::string::String,
    }

    ///ID of the VM
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct VmResponseId(::std::string::String);
    impl ::std::ops::Deref for VmResponseId {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<VmResponseId> for ::std::string::String {
        fn from(value: VmResponseId) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for VmResponseId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::new("^vm[0-9a-hj-km-np-tv-z]{24}$").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^vm[0-9a-hj-km-np-tv-z]{24}$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for VmResponseId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for VmResponseId {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for VmResponseId {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    /// Error types.
    pub mod error {
        /// Error from a `TryFrom` or `FromStr` implementation.
        pub struct ConversionError(::std::borrow::Cow<'static, str>);
        impl ::std::error::Error for ConversionError {}
        impl ::std::fmt::Display for ConversionError {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error> {
                ::std::fmt::Display::fmt(&self.0, f)
            }
        }

        impl ::std::fmt::Debug for ConversionError {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error> {
                ::std::fmt::Debug::fmt(&self.0, f)
            }
        }

        impl From<&'static str> for ConversionError {
            fn from(value: &'static str) -> Self {
                Self(value.into())
            }
        }

        impl From<String> for ConversionError {
            fn from(value: String) -> Self {
                Self(value.into())
            }
        }
    }
}

#[derive(Clone, Debug)]
///Client for Clover API
///
///API for managing resources on Ubicloud
///
///Version: 0.1.0
pub struct Client {
    pub(crate) baseurl: String,
    pub(crate) client: reqwest::Client,
}

impl Client {
    /// Create a new client.
    ///
    /// `baseurl` is the base URL provided to the internal
    /// `reqwest::Client`, and should include a scheme and hostname,
    /// as well as port and a path stem if applicable.
    pub fn new(baseurl: &str) -> Self {
        #[cfg(not(target_arch = "wasm32"))]
        let client = {
            let dur = ::std::time::Duration::from_secs(15u64);
            reqwest::ClientBuilder::new()
                .connect_timeout(dur)
                .timeout(dur)
        };
        #[cfg(target_arch = "wasm32")]
        let client = reqwest::ClientBuilder::new();
        Self::new_with_client(baseurl, client.build().unwrap())
    }

    /// Construct a new client with an existing `reqwest::Client`,
    /// allowing more control over its configuration.
    ///
    /// `baseurl` is the base URL provided to the internal
    /// `reqwest::Client`, and should include a scheme and hostname,
    /// as well as port and a path stem if applicable.
    pub fn new_with_client(baseurl: &str, client: reqwest::Client) -> Self {
        Self {
            baseurl: baseurl.to_string(),
            client,
        }
    }
}

impl ClientInfo<()> for Client {
    fn api_version() -> &'static str {
        "0.1.0"
    }

    fn baseurl(&self) -> &str {
        self.baseurl.as_str()
    }

    fn client(&self) -> &reqwest::Client {
        &self.client
    }

    fn inner(&self) -> &() {
        &()
    }
}

impl ClientHooks<()> for &Client {}
#[allow(clippy::all)]
impl Client {
    ///Retrieve a project
    ///
    ///Sends a `GET` request to `/project/{project_id}`
    ///
    ///Arguments:
    /// - `project_id`: ID of the project
    pub async fn get_project<'a>(
        &'a self,
        project_id: &'a types::GetProjectProjectId,
    ) -> Result<ResponseValue<types::Project>, Error<types::Error>> {
        let url = format!(
            "{}/project/{}",
            self.baseurl,
            encode_path(&project_id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_project",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Delete a project
    ///
    ///Sends a `DELETE` request to `/project/{project_id}`
    ///
    ///Arguments:
    /// - `project_id`: ID of the project
    pub async fn delete_project<'a>(
        &'a self,
        project_id: &'a types::DeleteProjectProjectId,
    ) -> Result<ResponseValue<()>, Error<types::Error>> {
        let url = format!(
            "{}/project/{}",
            self.baseurl,
            encode_path(&project_id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .delete(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "delete_project",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            204u16 => Ok(ResponseValue::empty(response)),
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Search project audit log entries
    ///
    ///Sends a `GET` request to `/project/{project_id}/audit-log`
    ///
    ///Arguments:
    /// - `project_id`: ID of the project
    /// - `action`: Filter by action type
    /// - `end`: Filter by end_date (shows 3 months prior)
    /// - `limit`: Limit the number of returned records (1-1000)
    /// - `object`: Filter by object ID
    /// - `pagination_key`: Key for paginating results
    /// - `subject`: Filter by account name/email/ID
    pub async fn search_project_audit_log<'a>(
        &'a self,
        project_id: &'a types::SearchProjectAuditLogProjectId,
        action: Option<&'a str>,
        end: Option<&'a str>,
        limit: Option<f64>,
        object: Option<&'a str>,
        pagination_key: Option<&'a str>,
        subject: Option<&'a str>,
    ) -> Result<ResponseValue<types::SearchProjectAuditLogResponse>, Error<types::Error>> {
        let url = format!(
            "{}/project/{}/audit-log",
            self.baseurl,
            encode_path(&project_id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("action", &action))
            .query(&progenitor_client::QueryParam::new("end", &end))
            .query(&progenitor_client::QueryParam::new("limit", &limit))
            .query(&progenitor_client::QueryParam::new("object", &object))
            .query(&progenitor_client::QueryParam::new(
                "pagination_key",
                &pagination_key,
            ))
            .query(&progenitor_client::QueryParam::new("subject", &subject))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "search_project_audit_log",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///List all firewalls
    ///
    ///Sends a `GET` request to `/project/{project_id}/firewall`
    ///
    ///Arguments:
    /// - `project_id`: ID of the project
    pub async fn get_firewall<'a>(
        &'a self,
        project_id: &'a types::GetFirewallProjectId,
    ) -> Result<ResponseValue<types::GetFirewallResponse>, Error<types::Error>> {
        let url = format!(
            "{}/project/{}/firewall",
            self.baseurl,
            encode_path(&project_id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_firewall",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///List all GitHub installations
    ///
    ///Sends a `GET` request to `/project/{project_id}/github`
    ///
    ///Arguments:
    /// - `project_id`: ID of the project
    pub async fn get_github_installations<'a>(
        &'a self,
        project_id: &'a types::GetGithubInstallationsProjectId,
    ) -> Result<ResponseValue<types::GetGithubInstallationsResponse>, Error<types::Error>> {
        let url = format!(
            "{}/project/{}/github",
            self.baseurl,
            encode_path(&project_id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_github_installations",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Get GitHub installation details
    ///
    ///Sends a `GET` request to
    /// `/project/{project_id}/github/{github_installation_reference}`
    ///
    ///Arguments:
    /// - `project_id`: ID of the project
    /// - `github_installation_reference`: GitHub installation ID or name
    pub async fn get_github_installation<'a>(
        &'a self,
        project_id: &'a types::GetGithubInstallationProjectId,
        github_installation_reference: &'a types::Reference,
    ) -> Result<ResponseValue<types::GithubInstallation>, Error<types::Error>> {
        let url = format!(
            "{}/project/{}/github/{}",
            self.baseurl,
            encode_path(&project_id.to_string()),
            encode_path(&github_installation_reference.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_github_installation",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///List repositories for a GitHub installation
    ///
    ///Sends a `GET` request to
    /// `/project/{project_id}/github/{github_installation_reference}/
    /// repository`
    ///
    ///Arguments:
    /// - `project_id`: ID of the project
    /// - `github_installation_reference`: GitHub installation ID or name
    pub async fn get_github_installation_repositories<'a>(
        &'a self,
        project_id: &'a types::GetGithubInstallationRepositoriesProjectId,
        github_installation_reference: &'a types::Reference,
    ) -> Result<ResponseValue<types::GetGithubInstallationRepositoriesResponse>, Error<types::Error>>
    {
        let url = format!(
            "{}/project/{}/github/{}/repository",
            self.baseurl,
            encode_path(&project_id.to_string()),
            encode_path(&github_installation_reference.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_github_installation_repositories",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Get GitHub repository details
    ///
    ///Sends a `GET` request to
    /// `/project/{project_id}/github/{github_installation_reference}/
    /// repository/{github_repository_reference}`
    ///
    ///Arguments:
    /// - `project_id`: ID of the project
    /// - `github_installation_reference`: GitHub installation ID or name
    /// - `github_repository_reference`: GitHub repository ID or name
    pub async fn get_github_repository<'a>(
        &'a self,
        project_id: &'a types::GetGithubRepositoryProjectId,
        github_installation_reference: &'a types::Reference,
        github_repository_reference: &'a types::Reference,
    ) -> Result<ResponseValue<types::GithubRepository>, Error<types::Error>> {
        let url = format!(
            "{}/project/{}/github/{}/repository/{}",
            self.baseurl,
            encode_path(&project_id.to_string()),
            encode_path(&github_installation_reference.to_string()),
            encode_path(&github_repository_reference.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_github_repository",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///List all cache entries for a GitHub repository
    ///
    ///Sends a `GET` request to
    /// `/project/{project_id}/github/{github_installation_reference}/
    /// repository/{github_repository_reference}/cache`
    ///
    ///Arguments:
    /// - `project_id`: ID of the project
    /// - `github_installation_reference`: GitHub installation ID or name
    /// - `github_repository_reference`: GitHub repository ID or name
    pub async fn get_github_cache_entries<'a>(
        &'a self,
        project_id: &'a types::GetGithubCacheEntriesProjectId,
        github_installation_reference: &'a types::Reference,
        github_repository_reference: &'a types::Reference,
    ) -> Result<ResponseValue<types::GetGithubCacheEntriesResponse>, Error<types::Error>> {
        let url = format!(
            "{}/project/{}/github/{}/repository/{}/cache",
            self.baseurl,
            encode_path(&project_id.to_string()),
            encode_path(&github_installation_reference.to_string()),
            encode_path(&github_repository_reference.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_github_cache_entries",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Delete all cache entries for a GitHub repository
    ///
    ///Sends a `DELETE` request to
    /// `/project/{project_id}/github/{github_installation_reference}/
    /// repository/{github_repository_reference}/cache`
    ///
    ///Arguments:
    /// - `project_id`: ID of the project
    /// - `github_installation_reference`: GitHub installation ID or name
    /// - `github_repository_reference`: GitHub repository ID or name
    pub async fn delete_all_github_cache_entries<'a>(
        &'a self,
        project_id: &'a types::DeleteAllGithubCacheEntriesProjectId,
        github_installation_reference: &'a types::Reference,
        github_repository_reference: &'a types::Reference,
    ) -> Result<ResponseValue<()>, Error<types::Error>> {
        let url = format!(
            "{}/project/{}/github/{}/repository/{}/cache",
            self.baseurl,
            encode_path(&project_id.to_string()),
            encode_path(&github_installation_reference.to_string()),
            encode_path(&github_repository_reference.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .delete(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "delete_all_github_cache_entries",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            204u16 => Ok(ResponseValue::empty(response)),
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Get cache entry details
    ///
    ///Sends a `GET` request to
    /// `/project/{project_id}/github/{github_installation_reference}/
    /// repository/{github_repository_reference}/cache/{github_cache_entry_id}`
    ///
    ///Arguments:
    /// - `project_id`: ID of the project
    /// - `github_installation_reference`: GitHub installation ID or name
    /// - `github_repository_reference`: GitHub repository ID or name
    /// - `github_cache_entry_id`: GitHub cache entry ID
    pub async fn get_github_cache_entry<'a>(
        &'a self,
        project_id: &'a types::GetGithubCacheEntryProjectId,
        github_installation_reference: &'a types::Reference,
        github_repository_reference: &'a types::Reference,
        github_cache_entry_id: &'a types::Reference,
    ) -> Result<ResponseValue<types::GithubCacheEntry>, Error<types::Error>> {
        let url = format!(
            "{}/project/{}/github/{}/repository/{}/cache/{}",
            self.baseurl,
            encode_path(&project_id.to_string()),
            encode_path(&github_installation_reference.to_string()),
            encode_path(&github_repository_reference.to_string()),
            encode_path(&github_cache_entry_id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_github_cache_entry",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Delete a cache entry
    ///
    ///Sends a `DELETE` request to
    /// `/project/{project_id}/github/{github_installation_reference}/
    /// repository/{github_repository_reference}/cache/{github_cache_entry_id}`
    ///
    ///Arguments:
    /// - `project_id`: ID of the project
    /// - `github_installation_reference`: GitHub installation ID or name
    /// - `github_repository_reference`: GitHub repository ID or name
    /// - `github_cache_entry_id`: GitHub cache entry ID
    pub async fn delete_github_cache_entry<'a>(
        &'a self,
        project_id: &'a types::DeleteGithubCacheEntryProjectId,
        github_installation_reference: &'a types::Reference,
        github_repository_reference: &'a types::Reference,
        github_cache_entry_id: &'a types::Reference,
    ) -> Result<ResponseValue<()>, Error<types::Error>> {
        let url = format!(
            "{}/project/{}/github/{}/repository/{}/cache/{}",
            self.baseurl,
            encode_path(&project_id.to_string()),
            encode_path(&github_installation_reference.to_string()),
            encode_path(&github_repository_reference.to_string()),
            encode_path(&github_cache_entry_id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .delete(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "delete_github_cache_entry",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            204u16 => Ok(ResponseValue::empty(response)),
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///List all inference API keys
    ///
    ///Sends a `GET` request to `/project/{project_id}/inference-api-key`
    ///
    ///Arguments:
    /// - `project_id`: ID of the project
    pub async fn get_inference_api_keys<'a>(
        &'a self,
        project_id: &'a types::GetInferenceApiKeysProjectId,
    ) -> Result<ResponseValue<types::GetInferenceApiKeysResponse>, Error<types::Error>> {
        let url = format!(
            "{}/project/{}/inference-api-key",
            self.baseurl,
            encode_path(&project_id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_inference_api_keys",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Create a new inference API key
    ///
    ///Sends a `POST` request to `/project/{project_id}/inference-api-key`
    ///
    ///Arguments:
    /// - `project_id`: ID of the project
    /// - `body`
    pub async fn create_inference_api_key<'a>(
        &'a self,
        project_id: &'a types::CreateInferenceApiKeyProjectId,
        body: &'a types::CreateInferenceApiKeyBody,
    ) -> Result<ResponseValue<types::InferenceApiKey>, Error<types::Error>> {
        let url = format!(
            "{}/project/{}/inference-api-key",
            self.baseurl,
            encode_path(&project_id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "create_inference_api_key",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Get inference API key details
    ///
    ///Sends a `GET` request to
    /// `/project/{project_id}/inference-api-key/{inference_api_key_id}`
    ///
    ///Arguments:
    /// - `project_id`: ID of the project
    /// - `inference_api_key_id`: ID of the inference API key
    pub async fn get_inference_api_key<'a>(
        &'a self,
        project_id: &'a types::GetInferenceApiKeyProjectId,
        inference_api_key_id: &'a types::GetInferenceApiKeyInferenceApiKeyId,
    ) -> Result<ResponseValue<types::InferenceApiKey>, Error<types::Error>> {
        let url = format!(
            "{}/project/{}/inference-api-key/{}",
            self.baseurl,
            encode_path(&project_id.to_string()),
            encode_path(&inference_api_key_id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_inference_api_key",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Delete a specific inference API key
    ///
    ///Sends a `DELETE` request to
    /// `/project/{project_id}/inference-api-key/{inference_api_key_id}`
    ///
    ///Arguments:
    /// - `project_id`: ID of the project
    /// - `inference_api_key_id`: ID of the inference API key
    pub async fn delete_inference_api_key<'a>(
        &'a self,
        project_id: &'a types::DeleteInferenceApiKeyProjectId,
        inference_api_key_id: &'a types::DeleteInferenceApiKeyInferenceApiKeyId,
    ) -> Result<ResponseValue<()>, Error<types::Error>> {
        let url = format!(
            "{}/project/{}/inference-api-key/{}",
            self.baseurl,
            encode_path(&project_id.to_string()),
            encode_path(&inference_api_key_id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .delete(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "delete_inference_api_key",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            204u16 => Ok(ResponseValue::empty(response)),
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///List all inference endpoints
    ///
    ///Sends a `GET` request to `/project/{project_id}/inference-endpoint`
    ///
    ///Arguments:
    /// - `project_id`: ID of the project
    pub async fn list_inference_endpoints<'a>(
        &'a self,
        project_id: &'a types::ListInferenceEndpointsProjectId,
    ) -> Result<ResponseValue<types::ListInferenceEndpointsResponse>, Error<types::Error>> {
        let url = format!(
            "{}/project/{}/inference-endpoint",
            self.baseurl,
            encode_path(&project_id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "list_inference_endpoints",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///List all kubernetes clusters
    ///
    ///Sends a `GET` request to `/project/{project_id}/kubernetes-cluster`
    ///
    ///Arguments:
    /// - `project_id`: ID of the project
    /// - `order_column`: Pagination - Order column
    /// - `page_size`: Pagination - Page size
    /// - `start_after`: Pagination - Start after
    pub async fn list_project_kubernetes_clusters<'a>(
        &'a self,
        project_id: &'a types::ListProjectKubernetesClustersProjectId,
        order_column: Option<&'a str>,
        page_size: Option<i64>,
        start_after: Option<&'a str>,
    ) -> Result<ResponseValue<types::ListProjectKubernetesClustersResponse>, Error<types::Error>>
    {
        let url = format!(
            "{}/project/{}/kubernetes-cluster",
            self.baseurl,
            encode_path(&project_id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new(
                "order_column",
                &order_column,
            ))
            .query(&progenitor_client::QueryParam::new("page_size", &page_size))
            .query(&progenitor_client::QueryParam::new(
                "start_after",
                &start_after,
            ))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "list_project_kubernetes_clusters",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///List all load balancers
    ///
    ///Sends a `GET` request to `/project/{project_id}/load-balancer`
    ///
    ///Arguments:
    /// - `project_id`: ID of the project
    /// - `order_column`: Pagination - Order column
    /// - `page_size`: Pagination - Page size
    /// - `start_after`: Pagination - Start after
    pub async fn list_load_balancers<'a>(
        &'a self,
        project_id: &'a types::ListLoadBalancersProjectId,
        order_column: Option<&'a str>,
        page_size: Option<i64>,
        start_after: Option<&'a str>,
    ) -> Result<ResponseValue<types::ListLoadBalancersResponse>, Error<types::Error>> {
        let url = format!(
            "{}/project/{}/load-balancer",
            self.baseurl,
            encode_path(&project_id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new(
                "order_column",
                &order_column,
            ))
            .query(&progenitor_client::QueryParam::new("page_size", &page_size))
            .query(&progenitor_client::QueryParam::new(
                "start_after",
                &start_after,
            ))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "list_load_balancers",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///List firewalls in a location
    ///
    ///Sends a `GET` request to
    /// `/project/{project_id}/location/{location}/firewall`
    ///
    ///Arguments:
    /// - `project_id`: ID of the project
    /// - `location`: The Ubicloud location/region
    pub async fn get_location_firewall<'a>(
        &'a self,
        project_id: &'a types::GetLocationFirewallProjectId,
        location: &'a str,
    ) -> Result<ResponseValue<types::GetLocationFirewallResponse>, Error<types::Error>> {
        let url = format!(
            "{}/project/{}/location/{}/firewall",
            self.baseurl,
            encode_path(&project_id.to_string()),
            encode_path(&location.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_location_firewall",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Get firewall details
    ///
    ///Sends a `GET` request to
    /// `/project/{project_id}/location/{location}/firewall/
    /// {firewall_reference}`
    ///
    ///Arguments:
    /// - `project_id`: ID of the project
    /// - `location`: The Ubicloud location/region
    /// - `firewall_reference`: Firewall ID or name
    pub async fn get_location_firewall_details<'a>(
        &'a self,
        project_id: &'a types::GetLocationFirewallDetailsProjectId,
        location: &'a str,
        firewall_reference: &'a types::Reference,
    ) -> Result<ResponseValue<types::GetLocationFirewallDetailsResponse>, Error<types::Error>> {
        let url = format!(
            "{}/project/{}/location/{}/firewall/{}",
            self.baseurl,
            encode_path(&project_id.to_string()),
            encode_path(&location.to_string()),
            encode_path(&firewall_reference.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_location_firewall_details",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Create a new firewall
    ///
    ///Sends a `POST` request to
    /// `/project/{project_id}/location/{location}/firewall/
    /// {firewall_reference}`
    ///
    ///Arguments:
    /// - `project_id`: ID of the project
    /// - `location`: The Ubicloud location/region
    /// - `firewall_reference`: Firewall ID or name
    /// - `body`
    pub async fn create_location_firewall<'a>(
        &'a self,
        project_id: &'a types::CreateLocationFirewallProjectId,
        location: &'a str,
        firewall_reference: &'a types::Reference,
        body: &'a types::CreateLocationFirewallBody,
    ) -> Result<ResponseValue<types::Firewall>, Error<types::Error>> {
        let url = format!(
            "{}/project/{}/location/{}/firewall/{}",
            self.baseurl,
            encode_path(&project_id.to_string()),
            encode_path(&location.to_string()),
            encode_path(&firewall_reference.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "create_location_firewall",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Delete a firewall
    ///
    ///Sends a `DELETE` request to
    /// `/project/{project_id}/location/{location}/firewall/
    /// {firewall_reference}`
    ///
    ///Arguments:
    /// - `project_id`: ID of the project
    /// - `location`: The Ubicloud location/region
    /// - `firewall_reference`: Firewall ID or name
    pub async fn delete_location_firewall<'a>(
        &'a self,
        project_id: &'a types::DeleteLocationFirewallProjectId,
        location: &'a str,
        firewall_reference: &'a types::Reference,
    ) -> Result<ResponseValue<()>, Error<types::Error>> {
        let url = format!(
            "{}/project/{}/location/{}/firewall/{}",
            self.baseurl,
            encode_path(&project_id.to_string()),
            encode_path(&location.to_string()),
            encode_path(&firewall_reference.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .delete(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "delete_location_firewall",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            204u16 => Ok(ResponseValue::empty(response)),
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Update a firewall
    ///
    ///Sends a `PATCH` request to
    /// `/project/{project_id}/location/{location}/firewall/
    /// {firewall_reference}`
    ///
    ///Arguments:
    /// - `project_id`: ID of the project
    /// - `location`: The Ubicloud location/region
    /// - `firewall_reference`: Firewall ID or name
    /// - `body`
    pub async fn update_firewall<'a>(
        &'a self,
        project_id: &'a types::UpdateFirewallProjectId,
        location: &'a str,
        firewall_reference: &'a types::Reference,
        body: &'a types::UpdateFirewallBody,
    ) -> Result<ResponseValue<types::Firewall>, Error<types::Error>> {
        let url = format!(
            "{}/project/{}/location/{}/firewall/{}",
            self.baseurl,
            encode_path(&project_id.to_string()),
            encode_path(&location.to_string()),
            encode_path(&firewall_reference.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .patch(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "update_firewall",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Attach a subnet to firewall
    ///
    ///Sends a `POST` request to
    /// `/project/{project_id}/location/{location}/firewall/
    /// {firewall_reference}/attach-subnet`
    ///
    ///Arguments:
    /// - `project_id`: ID of the project
    /// - `location`: The Ubicloud location/region
    /// - `firewall_reference`: Firewall ID or name
    /// - `body`
    pub async fn action_location_firewall_attach_subnet<'a>(
        &'a self,
        project_id: &'a types::ActionLocationFirewallAttachSubnetProjectId,
        location: &'a str,
        firewall_reference: &'a types::Reference,
        body: &'a types::ActionLocationFirewallAttachSubnetBody,
    ) -> Result<ResponseValue<types::ActionLocationFirewallAttachSubnetResponse>, Error<types::Error>>
    {
        let url = format!(
            "{}/project/{}/location/{}/firewall/{}/attach-subnet",
            self.baseurl,
            encode_path(&project_id.to_string()),
            encode_path(&location.to_string()),
            encode_path(&firewall_reference.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "action_location_firewall_attach_subnet",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Detach a subnet from firewall
    ///
    ///Sends a `POST` request to
    /// `/project/{project_id}/location/{location}/firewall/
    /// {firewall_reference}/detach-subnet`
    ///
    ///Arguments:
    /// - `project_id`: ID of the project
    /// - `location`: The Ubicloud location/region
    /// - `firewall_reference`: Firewall ID or name
    /// - `body`
    pub async fn action_location_firewall_detach_subnet<'a>(
        &'a self,
        project_id: &'a types::ActionLocationFirewallDetachSubnetProjectId,
        location: &'a str,
        firewall_reference: &'a types::Reference,
        body: &'a types::ActionLocationFirewallDetachSubnetBody,
    ) -> Result<ResponseValue<types::ActionLocationFirewallDetachSubnetResponse>, Error<types::Error>>
    {
        let url = format!(
            "{}/project/{}/location/{}/firewall/{}/detach-subnet",
            self.baseurl,
            encode_path(&project_id.to_string()),
            encode_path(&location.to_string()),
            encode_path(&firewall_reference.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "action_location_firewall_detach_subnet",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Create a new firewall rule
    ///
    ///Sends a `POST` request to
    /// `/project/{project_id}/location/{location}/firewall/
    /// {firewall_reference}/firewall-rule`
    ///
    ///Arguments:
    /// - `project_id`: ID of the project
    /// - `location`: The Ubicloud location/region
    /// - `firewall_reference`: Firewall ID or name
    /// - `body`
    pub async fn create_location_firewall_rule<'a>(
        &'a self,
        project_id: &'a types::CreateLocationFirewallRuleProjectId,
        location: &'a str,
        firewall_reference: &'a types::Reference,
        body: &'a types::CreateLocationFirewallRuleBody,
    ) -> Result<ResponseValue<types::CreateLocationFirewallRuleResponse>, Error<types::Error>> {
        let url = format!(
            "{}/project/{}/location/{}/firewall/{}/firewall-rule",
            self.baseurl,
            encode_path(&project_id.to_string()),
            encode_path(&location.to_string()),
            encode_path(&firewall_reference.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "create_location_firewall_rule",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Get firewall rule details
    ///
    ///Sends a `GET` request to
    /// `/project/{project_id}/location/{location}/firewall/
    /// {firewall_reference}/firewall-rule/{firewall_rule_id}`
    ///
    ///Arguments:
    /// - `project_id`: ID of the project
    /// - `location`: The Ubicloud location/region
    /// - `firewall_reference`: Firewall ID or name
    /// - `firewall_rule_id`: ID of the firewall rule
    pub async fn get_location_firewall_firewall_rule_details<'a>(
        &'a self,
        project_id: &'a types::GetLocationFirewallFirewallRuleDetailsProjectId,
        location: &'a str,
        firewall_reference: &'a types::Reference,
        firewall_rule_id: &'a types::GetLocationFirewallFirewallRuleDetailsFirewallRuleId,
    ) -> Result<ResponseValue<types::FirewallRule>, Error<types::Error>> {
        let url = format!(
            "{}/project/{}/location/{}/firewall/{}/firewall-rule/{}",
            self.baseurl,
            encode_path(&project_id.to_string()),
            encode_path(&location.to_string()),
            encode_path(&firewall_reference.to_string()),
            encode_path(&firewall_rule_id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_location_firewall_firewall_rule_details",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Delete a firewall rule
    ///
    ///Sends a `DELETE` request to
    /// `/project/{project_id}/location/{location}/firewall/
    /// {firewall_reference}/firewall-rule/{firewall_rule_id}`
    ///
    ///Arguments:
    /// - `project_id`: ID of the project
    /// - `location`: The Ubicloud location/region
    /// - `firewall_reference`: Firewall ID or name
    /// - `firewall_rule_id`: ID of the firewall rule
    pub async fn delete_location_firewall_firewall_rule<'a>(
        &'a self,
        project_id: &'a types::DeleteLocationFirewallFirewallRuleProjectId,
        location: &'a str,
        firewall_reference: &'a types::Reference,
        firewall_rule_id: &'a types::DeleteLocationFirewallFirewallRuleFirewallRuleId,
    ) -> Result<ResponseValue<()>, Error<types::Error>> {
        let url = format!(
            "{}/project/{}/location/{}/firewall/{}/firewall-rule/{}",
            self.baseurl,
            encode_path(&project_id.to_string()),
            encode_path(&location.to_string()),
            encode_path(&firewall_reference.to_string()),
            encode_path(&firewall_rule_id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .delete(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "delete_location_firewall_firewall_rule",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            204u16 => Ok(ResponseValue::empty(response)),
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Modify a firewall rule
    ///
    ///Sends a `PATCH` request to
    /// `/project/{project_id}/location/{location}/firewall/
    /// {firewall_reference}/firewall-rule/{firewall_rule_id}`
    ///
    ///Arguments:
    /// - `project_id`: ID of the project
    /// - `location`: The Ubicloud location/region
    /// - `firewall_reference`: Firewall ID or name
    /// - `firewall_rule_id`: ID of the firewall rule
    /// - `body`
    pub async fn modify_firewall_rule<'a>(
        &'a self,
        project_id: &'a types::ModifyFirewallRuleProjectId,
        location: &'a str,
        firewall_reference: &'a types::Reference,
        firewall_rule_id: &'a types::ModifyFirewallRuleFirewallRuleId,
        body: &'a types::FirewallRuleUpdate,
    ) -> Result<ResponseValue<types::FirewallRule>, Error<types::Error>> {
        let url = format!(
            "{}/project/{}/location/{}/firewall/{}/firewall-rule/{}",
            self.baseurl,
            encode_path(&project_id.to_string()),
            encode_path(&location.to_string()),
            encode_path(&firewall_reference.to_string()),
            encode_path(&firewall_rule_id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .patch(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "modify_firewall_rule",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Rename a firewall
    ///
    ///Sends a `POST` request to
    /// `/project/{project_id}/location/{location}/firewall/
    /// {firewall_reference}/rename`
    ///
    ///Arguments:
    /// - `project_id`: ID of the project
    /// - `location`: The Ubicloud location/region
    /// - `firewall_reference`: Firewall ID or name
    /// - `body`
    pub async fn rename_firewall<'a>(
        &'a self,
        project_id: &'a types::RenameFirewallProjectId,
        location: &'a str,
        firewall_reference: &'a types::Reference,
        body: &'a types::RenameFirewallBody,
    ) -> Result<ResponseValue<types::Firewall>, Error<types::Error>> {
        let url = format!(
            "{}/project/{}/location/{}/firewall/{}/rename",
            self.baseurl,
            encode_path(&project_id.to_string()),
            encode_path(&location.to_string()),
            encode_path(&firewall_reference.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "rename_firewall",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///List kubernetes clusters in a location
    ///
    ///Sends a `GET` request to
    /// `/project/{project_id}/location/{location}/kubernetes-cluster`
    ///
    ///Arguments:
    /// - `project_id`: ID of the project
    /// - `location`: The Ubicloud location/region
    /// - `order_column`: Pagination - Order column
    /// - `page_size`: Pagination - Page size
    /// - `start_after`: Pagination - Start after
    pub async fn list_location_kubernetes_clusters<'a>(
        &'a self,
        project_id: &'a types::ListLocationKubernetesClustersProjectId,
        location: &'a str,
        order_column: Option<&'a str>,
        page_size: Option<i64>,
        start_after: Option<&'a str>,
    ) -> Result<ResponseValue<types::ListLocationKubernetesClustersResponse>, Error<types::Error>>
    {
        let url = format!(
            "{}/project/{}/location/{}/kubernetes-cluster",
            self.baseurl,
            encode_path(&project_id.to_string()),
            encode_path(&location.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new(
                "order_column",
                &order_column,
            ))
            .query(&progenitor_client::QueryParam::new("page_size", &page_size))
            .query(&progenitor_client::QueryParam::new(
                "start_after",
                &start_after,
            ))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "list_location_kubernetes_clusters",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Get kubernetes cluster details
    ///
    ///Sends a `GET` request to
    /// `/project/{project_id}/location/{location}/kubernetes-cluster/
    /// {kubernetes_cluster_reference}`
    ///
    ///Arguments:
    /// - `project_id`: ID of the project
    /// - `location`: The Ubicloud location/region
    /// - `kubernetes_cluster_reference`: Kubernetes cluster ID or name
    pub async fn get_kubernetes_cluster_details<'a>(
        &'a self,
        project_id: &'a types::GetKubernetesClusterDetailsProjectId,
        location: &'a str,
        kubernetes_cluster_reference: &'a types::Reference,
    ) -> Result<ResponseValue<types::GetKubernetesClusterDetailsResponse>, Error<types::Error>>
    {
        let url = format!(
            "{}/project/{}/location/{}/kubernetes-cluster/{}",
            self.baseurl,
            encode_path(&project_id.to_string()),
            encode_path(&location.to_string()),
            encode_path(&kubernetes_cluster_reference.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_kubernetes_cluster_details",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Create a new kubernetes cluster
    ///
    ///Sends a `POST` request to
    /// `/project/{project_id}/location/{location}/kubernetes-cluster/
    /// {kubernetes_cluster_reference}`
    ///
    ///Arguments:
    /// - `project_id`: ID of the project
    /// - `location`: The Ubicloud location/region
    /// - `kubernetes_cluster_reference`: Kubernetes cluster ID or name
    /// - `body`
    pub async fn create_kubernetes_cluster<'a>(
        &'a self,
        project_id: &'a types::CreateKubernetesClusterProjectId,
        location: &'a str,
        kubernetes_cluster_reference: &'a types::Reference,
        body: &'a types::CreateKubernetesClusterBody,
    ) -> Result<ResponseValue<types::CreateKubernetesClusterResponse>, Error<types::Error>> {
        let url = format!(
            "{}/project/{}/location/{}/kubernetes-cluster/{}",
            self.baseurl,
            encode_path(&project_id.to_string()),
            encode_path(&location.to_string()),
            encode_path(&kubernetes_cluster_reference.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "create_kubernetes_cluster",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Delete a kubernetes cluster
    ///
    ///Sends a `DELETE` request to
    /// `/project/{project_id}/location/{location}/kubernetes-cluster/
    /// {kubernetes_cluster_reference}`
    ///
    ///Arguments:
    /// - `project_id`: ID of the project
    /// - `location`: The Ubicloud location/region
    /// - `kubernetes_cluster_reference`: Kubernetes cluster ID or name
    pub async fn delete_kubernetes_cluster<'a>(
        &'a self,
        project_id: &'a types::DeleteKubernetesClusterProjectId,
        location: &'a str,
        kubernetes_cluster_reference: &'a types::Reference,
    ) -> Result<ResponseValue<()>, Error<types::Error>> {
        let url = format!(
            "{}/project/{}/location/{}/kubernetes-cluster/{}",
            self.baseurl,
            encode_path(&project_id.to_string()),
            encode_path(&location.to_string()),
            encode_path(&kubernetes_cluster_reference.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .delete(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "delete_kubernetes_cluster",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            204u16 => Ok(ResponseValue::empty(response)),
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Get kubeconfig file
    ///
    ///Sends a `GET` request to
    /// `/project/{project_id}/location/{location}/kubernetes-cluster/
    /// {kubernetes_cluster_reference}/kubeconfig`
    ///
    ///Arguments:
    /// - `project_id`: ID of the project
    /// - `location`: The Ubicloud location/region
    /// - `kubernetes_cluster_reference`: Kubernetes cluster ID or name
    pub async fn get_kubernetes_kubeconfig_file<'a>(
        &'a self,
        project_id: &'a types::GetKubernetesKubeconfigFileProjectId,
        location: &'a str,
        kubernetes_cluster_reference: &'a types::Reference,
    ) -> Result<ResponseValue<ByteStream>, Error<types::Error>> {
        let url = format!(
            "{}/project/{}/location/{}/kubernetes-cluster/{}/kubeconfig",
            self.baseurl,
            encode_path(&project_id.to_string()),
            encode_path(&location.to_string()),
            encode_path(&kubernetes_cluster_reference.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.get(url).headers(header_map).build()?;
        let info = OperationInfo {
            operation_id: "get_kubernetes_kubeconfig_file",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::stream(response)),
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Get Kubernetes cluster metrics
    ///
    ///Sends a `GET` request to
    /// `/project/{project_id}/location/{location}/kubernetes-cluster/
    /// {kubernetes_cluster_reference}/metrics`
    ///
    ///Arguments:
    /// - `project_id`: ID of the project
    /// - `location`: The Ubicloud location/region
    /// - `kubernetes_cluster_reference`: Kubernetes cluster ID or name
    /// - `end`: End time for metrics query (rfc3339 format)
    /// - `key`: Key of the specific metric to retrieve
    /// - `start`: Start time for metrics query (rfc3339 format)
    pub async fn get_kubernetes_cluster_metrics<'a>(
        &'a self,
        project_id: &'a types::GetKubernetesClusterMetricsProjectId,
        location: &'a str,
        kubernetes_cluster_reference: &'a types::Reference,
        end: Option<&'a str>,
        key: Option<&'a str>,
        start: Option<&'a str>,
    ) -> Result<ResponseValue<types::ResourceMetrics>, Error<types::Error>> {
        let url = format!(
            "{}/project/{}/location/{}/kubernetes-cluster/{}/metrics",
            self.baseurl,
            encode_path(&project_id.to_string()),
            encode_path(&location.to_string()),
            encode_path(&kubernetes_cluster_reference.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("end", &end))
            .query(&progenitor_client::QueryParam::new("key", &key))
            .query(&progenitor_client::QueryParam::new("start", &start))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_kubernetes_cluster_metrics",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Retire a worker node from a kubernetes cluster
    ///
    ///Sends a `POST` request to
    /// `/project/{project_id}/location/{location}/kubernetes-cluster/
    /// {kubernetes_cluster_reference}/node/{kubernetes_node_name}/retire`
    ///
    ///Arguments:
    /// - `project_id`: ID of the project
    /// - `location`: The Ubicloud location/region
    /// - `kubernetes_cluster_reference`: Kubernetes cluster ID or name
    /// - `kubernetes_node_name`: Kubernetes node name
    pub async fn retire_kubernetes_node<'a>(
        &'a self,
        project_id: &'a types::RetireKubernetesNodeProjectId,
        location: &'a str,
        kubernetes_cluster_reference: &'a types::Reference,
        kubernetes_node_name: &'a types::Reference,
    ) -> Result<ResponseValue<types::RetireKubernetesNodeResponse>, Error<types::Error>> {
        let url = format!(
            "{}/project/{}/location/{}/kubernetes-cluster/{}/node/{}/retire",
            self.baseurl,
            encode_path(&project_id.to_string()),
            encode_path(&location.to_string()),
            encode_path(&kubernetes_cluster_reference.to_string()),
            encode_path(&kubernetes_node_name.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "retire_kubernetes_node",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Create a new kubernetes nodepool
    ///
    ///Sends a `POST` request to
    /// `/project/{project_id}/location/{location}/kubernetes-cluster/
    /// {kubernetes_cluster_reference}/nodepool`
    ///
    ///Arguments:
    /// - `project_id`: ID of the project
    /// - `location`: The Ubicloud location/region
    /// - `kubernetes_cluster_reference`: Kubernetes cluster ID or name
    /// - `body`
    pub async fn create_kubernetes_nodepool<'a>(
        &'a self,
        project_id: &'a types::CreateKubernetesNodepoolProjectId,
        location: &'a str,
        kubernetes_cluster_reference: &'a types::Reference,
        body: &'a types::CreateKubernetesNodepoolBody,
    ) -> Result<ResponseValue<types::CreateKubernetesNodepoolResponse>, Error<types::Error>> {
        let url = format!(
            "{}/project/{}/location/{}/kubernetes-cluster/{}/nodepool",
            self.baseurl,
            encode_path(&project_id.to_string()),
            encode_path(&location.to_string()),
            encode_path(&kubernetes_cluster_reference.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "create_kubernetes_nodepool",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Delete a kubernetes nodepool
    ///
    ///Sends a `DELETE` request to
    /// `/project/{project_id}/location/{location}/kubernetes-cluster/
    /// {kubernetes_cluster_reference}/nodepool/{kubernetes_nodepool_reference}`
    ///
    ///Arguments:
    /// - `project_id`: ID of the project
    /// - `location`: The Ubicloud location/region
    /// - `kubernetes_cluster_reference`: Kubernetes cluster ID or name
    /// - `kubernetes_nodepool_reference`: Kubernetes nodepool ID or name
    pub async fn delete_kubernetes_nodepool<'a>(
        &'a self,
        project_id: &'a types::DeleteKubernetesNodepoolProjectId,
        location: &'a str,
        kubernetes_cluster_reference: &'a types::Reference,
        kubernetes_nodepool_reference: &'a types::Reference,
    ) -> Result<ResponseValue<()>, Error<types::Error>> {
        let url = format!(
            "{}/project/{}/location/{}/kubernetes-cluster/{}/nodepool/{}",
            self.baseurl,
            encode_path(&project_id.to_string()),
            encode_path(&location.to_string()),
            encode_path(&kubernetes_cluster_reference.to_string()),
            encode_path(&kubernetes_nodepool_reference.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .delete(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "delete_kubernetes_nodepool",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            204u16 => Ok(ResponseValue::empty(response)),
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Rename a kubernetes nodepool
    ///
    ///Sends a `POST` request to
    /// `/project/{project_id}/location/{location}/kubernetes-cluster/
    /// {kubernetes_cluster_reference}/nodepool/{kubernetes_nodepool_reference}/
    /// rename`
    ///
    ///Arguments:
    /// - `project_id`: ID of the project
    /// - `location`: The Ubicloud location/region
    /// - `kubernetes_cluster_reference`: Kubernetes cluster ID or name
    /// - `kubernetes_nodepool_reference`: Kubernetes nodepool ID or name
    /// - `body`
    pub async fn rename_kubernetes_nodepool<'a>(
        &'a self,
        project_id: &'a types::RenameKubernetesNodepoolProjectId,
        location: &'a str,
        kubernetes_cluster_reference: &'a types::Reference,
        kubernetes_nodepool_reference: &'a types::Reference,
        body: &'a types::RenameKubernetesNodepoolBody,
    ) -> Result<ResponseValue<types::RenameKubernetesNodepoolResponse>, Error<types::Error>> {
        let url = format!(
            "{}/project/{}/location/{}/kubernetes-cluster/{}/nodepool/{}/rename",
            self.baseurl,
            encode_path(&project_id.to_string()),
            encode_path(&location.to_string()),
            encode_path(&kubernetes_cluster_reference.to_string()),
            encode_path(&kubernetes_nodepool_reference.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "rename_kubernetes_nodepool",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Change number of nodes of a kubernetes nodepool
    ///
    ///Sends a `POST` request to
    /// `/project/{project_id}/location/{location}/kubernetes-cluster/
    /// {kubernetes_cluster_reference}/nodepool/{kubernetes_nodepool_reference}/
    /// resize`
    ///
    ///Arguments:
    /// - `project_id`: ID of the project
    /// - `location`: The Ubicloud location/region
    /// - `kubernetes_cluster_reference`: Kubernetes cluster ID or name
    /// - `kubernetes_nodepool_reference`: Kubernetes nodepool ID or name
    /// - `body`
    pub async fn resize_kubernetes_nodepool<'a>(
        &'a self,
        project_id: &'a types::ResizeKubernetesNodepoolProjectId,
        location: &'a str,
        kubernetes_cluster_reference: &'a types::Reference,
        kubernetes_nodepool_reference: &'a types::Reference,
        body: &'a types::ResizeKubernetesNodepoolBody,
    ) -> Result<ResponseValue<types::ResizeKubernetesNodepoolResponse>, Error<types::Error>> {
        let url = format!(
            "{}/project/{}/location/{}/kubernetes-cluster/{}/nodepool/{}/resize",
            self.baseurl,
            encode_path(&project_id.to_string()),
            encode_path(&location.to_string()),
            encode_path(&kubernetes_cluster_reference.to_string()),
            encode_path(&kubernetes_nodepool_reference.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "resize_kubernetes_nodepool",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Upgrade a kubernetes nodepool to the cluster version
    ///
    ///Sends a `POST` request to
    /// `/project/{project_id}/location/{location}/kubernetes-cluster/
    /// {kubernetes_cluster_reference}/nodepool/{kubernetes_nodepool_reference}/
    /// upgrade`
    ///
    ///Arguments:
    /// - `project_id`: ID of the project
    /// - `location`: The Ubicloud location/region
    /// - `kubernetes_cluster_reference`: Kubernetes cluster ID or name
    /// - `kubernetes_nodepool_reference`: Kubernetes nodepool ID or name
    pub async fn upgrade_kubernetes_nodepool<'a>(
        &'a self,
        project_id: &'a types::UpgradeKubernetesNodepoolProjectId,
        location: &'a str,
        kubernetes_cluster_reference: &'a types::Reference,
        kubernetes_nodepool_reference: &'a types::Reference,
    ) -> Result<ResponseValue<types::UpgradeKubernetesNodepoolResponse>, Error<types::Error>> {
        let url = format!(
            "{}/project/{}/location/{}/kubernetes-cluster/{}/nodepool/{}/upgrade",
            self.baseurl,
            encode_path(&project_id.to_string()),
            encode_path(&location.to_string()),
            encode_path(&kubernetes_cluster_reference.to_string()),
            encode_path(&kubernetes_nodepool_reference.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "upgrade_kubernetes_nodepool",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Rename a kubernetes cluster
    ///
    ///Sends a `POST` request to
    /// `/project/{project_id}/location/{location}/kubernetes-cluster/
    /// {kubernetes_cluster_reference}/rename`
    ///
    ///Arguments:
    /// - `project_id`: ID of the project
    /// - `location`: The Ubicloud location/region
    /// - `kubernetes_cluster_reference`: Kubernetes cluster ID or name
    /// - `body`
    pub async fn rename_kubernetes_cluster<'a>(
        &'a self,
        project_id: &'a types::RenameKubernetesClusterProjectId,
        location: &'a str,
        kubernetes_cluster_reference: &'a types::Reference,
        body: &'a types::RenameKubernetesClusterBody,
    ) -> Result<ResponseValue<types::RenameKubernetesClusterResponse>, Error<types::Error>> {
        let url = format!(
            "{}/project/{}/location/{}/kubernetes-cluster/{}/rename",
            self.baseurl,
            encode_path(&project_id.to_string()),
            encode_path(&location.to_string()),
            encode_path(&kubernetes_cluster_reference.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "rename_kubernetes_cluster",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Upgrade a kubernetes cluster
    ///
    ///Sends a `POST` request to
    /// `/project/{project_id}/location/{location}/kubernetes-cluster/
    /// {kubernetes_cluster_reference}/upgrade`
    ///
    ///Arguments:
    /// - `project_id`: ID of the project
    /// - `location`: The Ubicloud location/region
    /// - `kubernetes_cluster_reference`: Kubernetes cluster ID or name
    pub async fn upgrade_kubernetes_cluster<'a>(
        &'a self,
        project_id: &'a types::UpgradeKubernetesClusterProjectId,
        location: &'a str,
        kubernetes_cluster_reference: &'a types::Reference,
    ) -> Result<ResponseValue<types::UpgradeKubernetesClusterResponse>, Error<types::Error>> {
        let url = format!(
            "{}/project/{}/location/{}/kubernetes-cluster/{}/upgrade",
            self.baseurl,
            encode_path(&project_id.to_string()),
            encode_path(&location.to_string()),
            encode_path(&kubernetes_cluster_reference.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "upgrade_kubernetes_cluster",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///List load balancers in a location
    ///
    ///Sends a `GET` request to
    /// `/project/{project_id}/location/{location}/load-balancer`
    ///
    ///Arguments:
    /// - `project_id`: ID of the project
    /// - `location`: The Ubicloud location/region
    /// - `order_column`: Pagination - Order column
    /// - `page_size`: Pagination - Page size
    /// - `start_after`: Pagination - Start after
    pub async fn list_location_load_balancers<'a>(
        &'a self,
        project_id: &'a types::ListLocationLoadBalancersProjectId,
        location: &'a str,
        order_column: Option<&'a str>,
        page_size: Option<i64>,
        start_after: Option<&'a str>,
    ) -> Result<ResponseValue<types::ListLocationLoadBalancersResponse>, Error<types::Error>> {
        let url = format!(
            "{}/project/{}/location/{}/load-balancer",
            self.baseurl,
            encode_path(&project_id.to_string()),
            encode_path(&location.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new(
                "order_column",
                &order_column,
            ))
            .query(&progenitor_client::QueryParam::new("page_size", &page_size))
            .query(&progenitor_client::QueryParam::new(
                "start_after",
                &start_after,
            ))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "list_location_load_balancers",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Get load balancer details
    ///
    ///Sends a `GET` request to
    /// `/project/{project_id}/location/{location}/load-balancer/
    /// {load_balancer_reference}`
    ///
    ///Arguments:
    /// - `project_id`: ID of the project
    /// - `location`: The Ubicloud location/region
    /// - `load_balancer_reference`: Load balancer ID or name
    pub async fn get_load_balancer_details<'a>(
        &'a self,
        project_id: &'a types::GetLoadBalancerDetailsProjectId,
        location: &'a str,
        load_balancer_reference: &'a types::Reference,
    ) -> Result<ResponseValue<types::GetLoadBalancerDetailsResponse>, Error<types::Error>> {
        let url = format!(
            "{}/project/{}/location/{}/load-balancer/{}",
            self.baseurl,
            encode_path(&project_id.to_string()),
            encode_path(&location.to_string()),
            encode_path(&load_balancer_reference.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_load_balancer_details",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Create a new load balancer
    ///
    ///Sends a `POST` request to
    /// `/project/{project_id}/location/{location}/load-balancer/
    /// {load_balancer_reference}`
    ///
    ///Arguments:
    /// - `project_id`: ID of the project
    /// - `location`: The Ubicloud location/region
    /// - `load_balancer_reference`: Load balancer ID or name
    /// - `body`
    pub async fn create_location_load_balancer<'a>(
        &'a self,
        project_id: &'a types::CreateLocationLoadBalancerProjectId,
        location: &'a str,
        load_balancer_reference: &'a types::Reference,
        body: &'a types::CreateLocationLoadBalancerBody,
    ) -> Result<ResponseValue<types::CreateLocationLoadBalancerResponse>, Error<types::Error>> {
        let url = format!(
            "{}/project/{}/location/{}/load-balancer/{}",
            self.baseurl,
            encode_path(&project_id.to_string()),
            encode_path(&location.to_string()),
            encode_path(&load_balancer_reference.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "create_location_load_balancer",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Delete a load balancer
    ///
    ///Sends a `DELETE` request to
    /// `/project/{project_id}/location/{location}/load-balancer/
    /// {load_balancer_reference}`
    ///
    ///Arguments:
    /// - `project_id`: ID of the project
    /// - `location`: The Ubicloud location/region
    /// - `load_balancer_reference`: Load balancer ID or name
    pub async fn delete_load_balancer<'a>(
        &'a self,
        project_id: &'a types::DeleteLoadBalancerProjectId,
        location: &'a str,
        load_balancer_reference: &'a types::Reference,
    ) -> Result<ResponseValue<()>, Error<types::Error>> {
        let url = format!(
            "{}/project/{}/location/{}/load-balancer/{}",
            self.baseurl,
            encode_path(&project_id.to_string()),
            encode_path(&location.to_string()),
            encode_path(&load_balancer_reference.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .delete(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "delete_load_balancer",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            204u16 => Ok(ResponseValue::empty(response)),
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Update a load balancer
    ///
    ///Sends a `PATCH` request to
    /// `/project/{project_id}/location/{location}/load-balancer/
    /// {load_balancer_reference}`
    ///
    ///Arguments:
    /// - `project_id`: ID of the project
    /// - `location`: The Ubicloud location/region
    /// - `load_balancer_reference`: Load balancer ID or name
    /// - `body`
    pub async fn patch_location_load_balancer<'a>(
        &'a self,
        project_id: &'a types::PatchLocationLoadBalancerProjectId,
        location: &'a str,
        load_balancer_reference: &'a types::Reference,
        body: &'a types::PatchLocationLoadBalancerBody,
    ) -> Result<ResponseValue<types::PatchLocationLoadBalancerResponse>, Error<types::Error>> {
        let url = format!(
            "{}/project/{}/location/{}/load-balancer/{}",
            self.baseurl,
            encode_path(&project_id.to_string()),
            encode_path(&location.to_string()),
            encode_path(&load_balancer_reference.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .patch(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "patch_location_load_balancer",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Attach a VM to a load balancer
    ///
    ///Sends a `POST` request to
    /// `/project/{project_id}/location/{location}/load-balancer/
    /// {load_balancer_reference}/attach-vm`
    ///
    ///Arguments:
    /// - `project_id`: ID of the project
    /// - `location`: The Ubicloud location/region
    /// - `load_balancer_reference`: Load balancer ID or name
    /// - `body`
    pub async fn attach_vm_location_load_balancer<'a>(
        &'a self,
        project_id: &'a types::AttachVmLocationLoadBalancerProjectId,
        location: &'a str,
        load_balancer_reference: &'a types::Reference,
        body: &'a types::AttachVmLocationLoadBalancerBody,
    ) -> Result<ResponseValue<types::AttachVmLocationLoadBalancerResponse>, Error<types::Error>>
    {
        let url = format!(
            "{}/project/{}/location/{}/load-balancer/{}/attach-vm",
            self.baseurl,
            encode_path(&project_id.to_string()),
            encode_path(&location.to_string()),
            encode_path(&load_balancer_reference.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "attach_vm_location_load_balancer",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Detach a VM from a load balancer
    ///
    ///Sends a `POST` request to
    /// `/project/{project_id}/location/{location}/load-balancer/
    /// {load_balancer_reference}/detach-vm`
    ///
    ///Arguments:
    /// - `project_id`: ID of the project
    /// - `location`: The Ubicloud location/region
    /// - `load_balancer_reference`: Load balancer ID or name
    /// - `body`
    pub async fn detach_vm_location_load_balancer<'a>(
        &'a self,
        project_id: &'a types::DetachVmLocationLoadBalancerProjectId,
        location: &'a str,
        load_balancer_reference: &'a types::Reference,
        body: &'a types::DetachVmLocationLoadBalancerBody,
    ) -> Result<ResponseValue<types::DetachVmLocationLoadBalancerResponse>, Error<types::Error>>
    {
        let url = format!(
            "{}/project/{}/location/{}/load-balancer/{}/detach-vm",
            self.baseurl,
            encode_path(&project_id.to_string()),
            encode_path(&location.to_string()),
            encode_path(&load_balancer_reference.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "detach_vm_location_load_balancer",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Rename a load balancer
    ///
    ///Sends a `POST` request to
    /// `/project/{project_id}/location/{location}/load-balancer/
    /// {load_balancer_reference}/rename`
    ///
    ///Arguments:
    /// - `project_id`: ID of the project
    /// - `location`: The Ubicloud location/region
    /// - `load_balancer_reference`: Load balancer ID or name
    /// - `body`
    pub async fn rename_load_balancer<'a>(
        &'a self,
        project_id: &'a types::RenameLoadBalancerProjectId,
        location: &'a str,
        load_balancer_reference: &'a types::Reference,
        body: &'a types::RenameLoadBalancerBody,
    ) -> Result<ResponseValue<types::RenameLoadBalancerResponse>, Error<types::Error>> {
        let url = format!(
            "{}/project/{}/location/{}/load-balancer/{}/rename",
            self.baseurl,
            encode_path(&project_id.to_string()),
            encode_path(&location.to_string()),
            encode_path(&load_balancer_reference.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "rename_load_balancer",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Toggle the SSL certificate for a load balancer
    ///
    ///Sends a `POST` request to
    /// `/project/{project_id}/location/{location}/load-balancer/
    /// {load_balancer_reference}/toggle-ssl-certificate`
    ///
    ///Arguments:
    /// - `project_id`: ID of the project
    /// - `location`: The Ubicloud location/region
    /// - `load_balancer_reference`: Load balancer ID or name
    /// - `body`
    pub async fn toggle_ssl_certificate_load_balancer<'a>(
        &'a self,
        project_id: &'a types::ToggleSslCertificateLoadBalancerProjectId,
        location: &'a str,
        load_balancer_reference: &'a types::Reference,
        body: &'a types::ToggleSslCertificateLoadBalancerBody,
    ) -> Result<ResponseValue<types::ToggleSslCertificateLoadBalancerResponse>, Error<types::Error>>
    {
        let url = format!(
            "{}/project/{}/location/{}/load-balancer/{}/toggle-ssl-certificate",
            self.baseurl,
            encode_path(&project_id.to_string()),
            encode_path(&location.to_string()),
            encode_path(&load_balancer_reference.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "toggle_ssl_certificate_load_balancer",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///List machine images in a location
    ///
    ///Sends a `GET` request to
    /// `/project/{project_id}/location/{location}/machine-image`
    ///
    ///Arguments:
    /// - `project_id`: ID of the project
    /// - `location`: The Ubicloud location/region
    /// - `order_column`: Pagination - Order column
    /// - `page_size`: Pagination - Page size
    /// - `start_after`: Pagination - Start after
    pub async fn list_location_machine_images<'a>(
        &'a self,
        project_id: &'a types::ListLocationMachineImagesProjectId,
        location: &'a str,
        order_column: Option<&'a str>,
        page_size: Option<i64>,
        start_after: Option<&'a str>,
    ) -> Result<ResponseValue<types::ListLocationMachineImagesResponse>, Error<types::Error>> {
        let url = format!(
            "{}/project/{}/location/{}/machine-image",
            self.baseurl,
            encode_path(&project_id.to_string()),
            encode_path(&location.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new(
                "order_column",
                &order_column,
            ))
            .query(&progenitor_client::QueryParam::new("page_size", &page_size))
            .query(&progenitor_client::QueryParam::new(
                "start_after",
                &start_after,
            ))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "list_location_machine_images",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Get details of a machine image
    ///
    ///Sends a `GET` request to
    /// `/project/{project_id}/location/{location}/machine-image/
    /// {machine_image_reference}`
    ///
    ///Arguments:
    /// - `project_id`: ID of the project
    /// - `location`: The Ubicloud location/region
    /// - `machine_image_reference`: Machine Image ID or name
    pub async fn get_machine_image<'a>(
        &'a self,
        project_id: &'a types::GetMachineImageProjectId,
        location: &'a str,
        machine_image_reference: &'a types::Reference,
    ) -> Result<ResponseValue<types::MachineImage>, Error<types::Error>> {
        let url = format!(
            "{}/project/{}/location/{}/machine-image/{}",
            self.baseurl,
            encode_path(&project_id.to_string()),
            encode_path(&location.to_string()),
            encode_path(&machine_image_reference.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_machine_image",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Create a machine image from a stopped VM
    ///
    ///Sends a `POST` request to
    /// `/project/{project_id}/location/{location}/machine-image/
    /// {machine_image_reference}`
    ///
    ///Arguments:
    /// - `project_id`: ID of the project
    /// - `location`: The Ubicloud location/region
    /// - `machine_image_reference`: Name for a new machine image (UBID format
    ///   not allowed)
    /// - `body`
    pub async fn create_machine_image<'a>(
        &'a self,
        project_id: &'a types::CreateMachineImageProjectId,
        location: &'a str,
        machine_image_reference: &'a types::CreateMachineImageMachineImageReference,
        body: &'a types::CreateMachineImageBody,
    ) -> Result<ResponseValue<types::MachineImage>, Error<types::Error>> {
        let url = format!(
            "{}/project/{}/location/{}/machine-image/{}",
            self.baseurl,
            encode_path(&project_id.to_string()),
            encode_path(&location.to_string()),
            encode_path(&machine_image_reference.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "create_machine_image",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Delete a machine image
    ///
    ///Sends a `DELETE` request to
    /// `/project/{project_id}/location/{location}/machine-image/
    /// {machine_image_reference}`
    ///
    ///Arguments:
    /// - `project_id`: ID of the project
    /// - `location`: The Ubicloud location/region
    /// - `machine_image_reference`: Machine Image ID or name
    pub async fn delete_machine_image<'a>(
        &'a self,
        project_id: &'a types::DeleteMachineImageProjectId,
        location: &'a str,
        machine_image_reference: &'a types::Reference,
    ) -> Result<ResponseValue<()>, Error<types::Error>> {
        let url = format!(
            "{}/project/{}/location/{}/machine-image/{}",
            self.baseurl,
            encode_path(&project_id.to_string()),
            encode_path(&location.to_string()),
            encode_path(&machine_image_reference.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .delete(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "delete_machine_image",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            204u16 => Ok(ResponseValue::empty(response)),
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Update a machine image
    ///
    ///Sends a `PATCH` request to
    /// `/project/{project_id}/location/{location}/machine-image/
    /// {machine_image_reference}`
    ///
    ///Arguments:
    /// - `project_id`: ID of the project
    /// - `location`: The Ubicloud location/region
    /// - `machine_image_reference`: Machine Image ID or name
    /// - `body`
    pub async fn update_machine_image<'a>(
        &'a self,
        project_id: &'a types::UpdateMachineImageProjectId,
        location: &'a str,
        machine_image_reference: &'a types::Reference,
        body: &'a types::UpdateMachineImageBody,
    ) -> Result<ResponseValue<types::MachineImage>, Error<types::Error>> {
        let url = format!(
            "{}/project/{}/location/{}/machine-image/{}",
            self.baseurl,
            encode_path(&project_id.to_string()),
            encode_path(&location.to_string()),
            encode_path(&machine_image_reference.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .patch(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "update_machine_image",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Rename a machine image
    ///
    ///Sends a `POST` request to
    /// `/project/{project_id}/location/{location}/machine-image/
    /// {machine_image_reference}/rename`
    ///
    ///Arguments:
    /// - `project_id`: ID of the project
    /// - `location`: The Ubicloud location/region
    /// - `machine_image_reference`: Machine Image ID or name
    /// - `body`
    pub async fn rename_machine_image<'a>(
        &'a self,
        project_id: &'a types::RenameMachineImageProjectId,
        location: &'a str,
        machine_image_reference: &'a types::Reference,
        body: &'a types::RenameMachineImageBody,
    ) -> Result<ResponseValue<types::MachineImage>, Error<types::Error>> {
        let url = format!(
            "{}/project/{}/location/{}/machine-image/{}/rename",
            self.baseurl,
            encode_path(&project_id.to_string()),
            encode_path(&location.to_string()),
            encode_path(&machine_image_reference.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "rename_machine_image",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///List versions of a machine image
    ///
    ///Sends a `GET` request to
    /// `/project/{project_id}/location/{location}/machine-image/
    /// {machine_image_reference}/version`
    ///
    ///Arguments:
    /// - `project_id`: ID of the project
    /// - `location`: The Ubicloud location/region
    /// - `machine_image_reference`: Machine Image ID or name
    pub async fn list_machine_image_versions<'a>(
        &'a self,
        project_id: &'a types::ListMachineImageVersionsProjectId,
        location: &'a str,
        machine_image_reference: &'a types::Reference,
    ) -> Result<ResponseValue<types::ListMachineImageVersionsResponse>, Error<types::Error>> {
        let url = format!(
            "{}/project/{}/location/{}/machine-image/{}/version",
            self.baseurl,
            encode_path(&project_id.to_string()),
            encode_path(&location.to_string()),
            encode_path(&machine_image_reference.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "list_machine_image_versions",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Create a new version of a machine image from a stopped VM
    ///
    ///Sends a `POST` request to
    /// `/project/{project_id}/location/{location}/machine-image/
    /// {machine_image_reference}/version/{version}`
    ///
    ///Arguments:
    /// - `project_id`: ID of the project
    /// - `location`: The Ubicloud location/region
    /// - `machine_image_reference`: Machine Image ID or name
    /// - `version`: Version label of the machine image
    /// - `body`
    pub async fn create_machine_image_version<'a>(
        &'a self,
        project_id: &'a types::CreateMachineImageVersionProjectId,
        location: &'a str,
        machine_image_reference: &'a types::Reference,
        version: &'a types::CreateMachineImageVersionVersion,
        body: &'a types::CreateMachineImageVersionBody,
    ) -> Result<ResponseValue<types::MachineImageVersion>, Error<types::Error>> {
        let url = format!(
            "{}/project/{}/location/{}/machine-image/{}/version/{}",
            self.baseurl,
            encode_path(&project_id.to_string()),
            encode_path(&location.to_string()),
            encode_path(&machine_image_reference.to_string()),
            encode_path(&version.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "create_machine_image_version",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Delete a specific version of a machine image
    ///
    ///Sends a `DELETE` request to
    /// `/project/{project_id}/location/{location}/machine-image/
    /// {machine_image_reference}/version/{version}`
    ///
    ///Arguments:
    /// - `project_id`: ID of the project
    /// - `location`: The Ubicloud location/region
    /// - `machine_image_reference`: Machine Image ID or name
    /// - `version`: Version label of the machine image
    pub async fn delete_machine_image_version<'a>(
        &'a self,
        project_id: &'a types::DeleteMachineImageVersionProjectId,
        location: &'a str,
        machine_image_reference: &'a types::Reference,
        version: &'a types::DeleteMachineImageVersionVersion,
    ) -> Result<ResponseValue<()>, Error<types::Error>> {
        let url = format!(
            "{}/project/{}/location/{}/machine-image/{}/version/{}",
            self.baseurl,
            encode_path(&project_id.to_string()),
            encode_path(&location.to_string()),
            encode_path(&machine_image_reference.to_string()),
            encode_path(&version.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .delete(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "delete_machine_image_version",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            204u16 => Ok(ResponseValue::empty(response)),
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///List PostgreSQL databases in a location
    ///
    ///Sends a `GET` request to
    /// `/project/{project_id}/location/{location}/postgres`
    ///
    ///Arguments:
    /// - `project_id`: ID of the project
    /// - `location`: The Ubicloud location/region
    /// - `order_column`: Pagination - Order column
    /// - `page_size`: Pagination - Page size
    /// - `start_after`: Pagination - Start after
    pub async fn list_location_postgres_databases<'a>(
        &'a self,
        project_id: &'a types::ListLocationPostgresDatabasesProjectId,
        location: &'a str,
        order_column: Option<&'a str>,
        page_size: Option<i64>,
        start_after: Option<&'a str>,
    ) -> Result<ResponseValue<types::ListLocationPostgresDatabasesResponse>, Error<types::Error>>
    {
        let url = format!(
            "{}/project/{}/location/{}/postgres",
            self.baseurl,
            encode_path(&project_id.to_string()),
            encode_path(&location.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new(
                "order_column",
                &order_column,
            ))
            .query(&progenitor_client::QueryParam::new("page_size", &page_size))
            .query(&progenitor_client::QueryParam::new(
                "start_after",
                &start_after,
            ))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "list_location_postgres_databases",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Get PostgreSQL database details
    ///
    ///Sends a `GET` request to
    /// `/project/{project_id}/location/{location}/postgres/
    /// {postgres_database_reference}`
    ///
    ///Arguments:
    /// - `project_id`: ID of the project
    /// - `location`: The Ubicloud location/region
    /// - `postgres_database_reference`: Postgres database ID or name
    pub async fn get_postgres_database_details<'a>(
        &'a self,
        project_id: &'a types::GetPostgresDatabaseDetailsProjectId,
        location: &'a str,
        postgres_database_reference: &'a types::Reference,
    ) -> Result<ResponseValue<types::GetPostgresDatabaseDetailsResponse>, Error<types::Error>> {
        let url = format!(
            "{}/project/{}/location/{}/postgres/{}",
            self.baseurl,
            encode_path(&project_id.to_string()),
            encode_path(&location.to_string()),
            encode_path(&postgres_database_reference.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_postgres_database_details",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Create a new PostgreSQL database
    ///
    ///Sends a `POST` request to
    /// `/project/{project_id}/location/{location}/postgres/
    /// {postgres_database_reference}`
    ///
    ///Arguments:
    /// - `project_id`: ID of the project
    /// - `location`: The Ubicloud location/region
    /// - `postgres_database_reference`: Postgres database ID or name
    /// - `body`
    pub async fn create_postgres_database<'a>(
        &'a self,
        project_id: &'a types::CreatePostgresDatabaseProjectId,
        location: &'a str,
        postgres_database_reference: &'a types::Reference,
        body: &'a types::CreatePostgresDatabaseBody,
    ) -> Result<ResponseValue<types::CreatePostgresDatabaseResponse>, Error<types::Error>> {
        let url = format!(
            "{}/project/{}/location/{}/postgres/{}",
            self.baseurl,
            encode_path(&project_id.to_string()),
            encode_path(&location.to_string()),
            encode_path(&postgres_database_reference.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "create_postgres_database",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Delete a PostgreSQL database
    ///
    ///Sends a `DELETE` request to
    /// `/project/{project_id}/location/{location}/postgres/
    /// {postgres_database_reference}`
    ///
    ///Arguments:
    /// - `project_id`: ID of the project
    /// - `location`: The Ubicloud location/region
    /// - `postgres_database_reference`: Postgres database ID or name
    pub async fn delete_postgres_database<'a>(
        &'a self,
        project_id: &'a types::DeletePostgresDatabaseProjectId,
        location: &'a str,
        postgres_database_reference: &'a types::Reference,
    ) -> Result<ResponseValue<()>, Error<types::Error>> {
        let url = format!(
            "{}/project/{}/location/{}/postgres/{}",
            self.baseurl,
            encode_path(&project_id.to_string()),
            encode_path(&location.to_string()),
            encode_path(&postgres_database_reference.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .delete(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "delete_postgres_database",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            204u16 => Ok(ResponseValue::empty(response)),
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Update a PostgreSQL database
    ///
    ///Sends a `PATCH` request to
    /// `/project/{project_id}/location/{location}/postgres/
    /// {postgres_database_reference}`
    ///
    ///Arguments:
    /// - `project_id`: ID of the project
    /// - `location`: The Ubicloud location/region
    /// - `postgres_database_reference`: Postgres database ID or name
    /// - `body`
    pub async fn patch_postgres_database<'a>(
        &'a self,
        project_id: &'a types::PatchPostgresDatabaseProjectId,
        location: &'a str,
        postgres_database_reference: &'a types::Reference,
        body: &'a types::PatchPostgresDatabaseBody,
    ) -> Result<ResponseValue<types::PatchPostgresDatabaseResponse>, Error<types::Error>> {
        let url = format!(
            "{}/project/{}/location/{}/postgres/{}",
            self.baseurl,
            encode_path(&project_id.to_string()),
            encode_path(&location.to_string()),
            encode_path(&postgres_database_reference.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .patch(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "patch_postgres_database",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///List backups for a specific PostgreSQL database
    ///
    ///Sends a `GET` request to
    /// `/project/{project_id}/location/{location}/postgres/
    /// {postgres_database_reference}/backup`
    ///
    ///Arguments:
    /// - `project_id`: ID of the project
    /// - `location`: The Ubicloud location/region
    /// - `postgres_database_reference`: Postgres database ID or name
    pub async fn list_postgres_database_backups<'a>(
        &'a self,
        project_id: &'a types::ListPostgresDatabaseBackupsProjectId,
        location: &'a str,
        postgres_database_reference: &'a types::Reference,
    ) -> Result<ResponseValue<types::ListPostgresDatabaseBackupsResponse>, Error<types::Error>>
    {
        let url = format!(
            "{}/project/{}/location/{}/postgres/{}/backup",
            self.baseurl,
            encode_path(&project_id.to_string()),
            encode_path(&location.to_string()),
            encode_path(&postgres_database_reference.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "list_postgres_database_backups",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Create temporary, read-only credentials for downloading backups directly
    /// from object storage
    ///
    ///Sends a `POST` request to
    /// `/project/{project_id}/location/{location}/postgres/
    /// {postgres_database_reference}/backup-credentials`
    ///
    ///Arguments:
    /// - `project_id`: ID of the project
    /// - `location`: The Ubicloud location/region
    /// - `postgres_database_reference`: Postgres database ID or name
    pub async fn create_postgres_database_backup_credentials<'a>(
        &'a self,
        project_id: &'a types::CreatePostgresDatabaseBackupCredentialsProjectId,
        location: &'a str,
        postgres_database_reference: &'a types::Reference,
    ) -> Result<ResponseValue<types::PostgresBackupCredentials>, Error<types::Error>> {
        let url = format!(
            "{}/project/{}/location/{}/postgres/{}/backup-credentials",
            self.baseurl,
            encode_path(&project_id.to_string()),
            encode_path(&location.to_string()),
            encode_path(&postgres_database_reference.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "create_postgres_database_backup_credentials",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Download CA certificates for a PostgreSQL database
    ///
    ///Sends a `GET` request to
    /// `/project/{project_id}/location/{location}/postgres/
    /// {postgres_database_reference}/ca-certificates`
    ///
    ///Arguments:
    /// - `project_id`: ID of the project
    /// - `location`: The Ubicloud location/region
    /// - `postgres_database_reference`: Postgres database ID or name
    pub async fn get_postgres_ca_certificates_by_name<'a>(
        &'a self,
        project_id: &'a types::GetPostgresCaCertificatesByNameProjectId,
        location: &'a str,
        postgres_database_reference: &'a types::Reference,
    ) -> Result<ResponseValue<ByteStream>, Error<types::Error>> {
        let url = format!(
            "{}/project/{}/location/{}/postgres/{}/ca-certificates",
            self.baseurl,
            encode_path(&project_id.to_string()),
            encode_path(&location.to_string()),
            encode_path(&postgres_database_reference.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.get(url).headers(header_map).build()?;
        let info = OperationInfo {
            operation_id: "get_postgres_ca_certificates_by_name",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::stream(response)),
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Add provided name to list of users with certificate authentication
    ///
    ///Sends a `POST` request to
    /// `/project/{project_id}/location/{location}/postgres/
    /// {postgres_database_reference}/cert/add-auth-user`
    ///
    ///Arguments:
    /// - `project_id`: ID of the project
    /// - `location`: The Ubicloud location/region
    /// - `postgres_database_reference`: Postgres database ID or name
    /// - `body`
    pub async fn add_cert_auth_user<'a>(
        &'a self,
        project_id: &'a types::AddCertAuthUserProjectId,
        location: &'a str,
        postgres_database_reference: &'a types::Reference,
        body: &'a types::CertAuthUserRequest,
    ) -> Result<ResponseValue<types::CertAuthUserList>, Error<types::Error>> {
        let url = format!(
            "{}/project/{}/location/{}/postgres/{}/cert/add-auth-user",
            self.baseurl,
            encode_path(&project_id.to_string()),
            encode_path(&location.to_string()),
            encode_path(&postgres_database_reference.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "add_cert_auth_user",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Create and return client certificate keypair with requested common name
    ///
    ///Sends a `POST` request to
    /// `/project/{project_id}/location/{location}/postgres/
    /// {postgres_database_reference}/cert/create-client-keypair`
    ///
    ///Arguments:
    /// - `project_id`: ID of the project
    /// - `location`: The Ubicloud location/region
    /// - `postgres_database_reference`: Postgres database ID or name
    /// - `body`
    pub async fn create_client_cert_keypair<'a>(
        &'a self,
        project_id: &'a types::CreateClientCertKeypairProjectId,
        location: &'a str,
        postgres_database_reference: &'a types::Reference,
        body: &'a types::CreateClientCertKeypairBody,
    ) -> Result<ResponseValue<ByteStream>, Error<types::Error>> {
        let url = format!(
            "{}/project/{}/location/{}/postgres/{}/cert/create-client-keypair",
            self.baseurl,
            encode_path(&project_id.to_string()),
            encode_path(&location.to_string()),
            encode_path(&postgres_database_reference.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "create_client_cert_keypair",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::stream(response)),
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Remove provided name from list of users with certificate authentication
    ///
    ///Sends a `POST` request to
    /// `/project/{project_id}/location/{location}/postgres/
    /// {postgres_database_reference}/cert/remove-auth-user`
    ///
    ///Arguments:
    /// - `project_id`: ID of the project
    /// - `location`: The Ubicloud location/region
    /// - `postgres_database_reference`: Postgres database ID or name
    /// - `body`
    pub async fn remove_cert_auth_user<'a>(
        &'a self,
        project_id: &'a types::RemoveCertAuthUserProjectId,
        location: &'a str,
        postgres_database_reference: &'a types::Reference,
        body: &'a types::CertAuthUserRequest,
    ) -> Result<ResponseValue<types::CertAuthUserList>, Error<types::Error>> {
        let url = format!(
            "{}/project/{}/location/{}/postgres/{}/cert/remove-auth-user",
            self.baseurl,
            encode_path(&project_id.to_string()),
            encode_path(&location.to_string()),
            encode_path(&postgres_database_reference.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "remove_cert_auth_user",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Get PostgreSQL database configuration
    ///
    ///Sends a `GET` request to
    /// `/project/{project_id}/location/{location}/postgres/
    /// {postgres_database_reference}/config`
    ///
    ///Arguments:
    /// - `project_id`: ID of the project
    /// - `location`: The Ubicloud location/region
    /// - `postgres_database_reference`: Postgres database ID or name
    pub async fn get_postgres_database_config<'a>(
        &'a self,
        project_id: &'a types::GetPostgresDatabaseConfigProjectId,
        location: &'a str,
        postgres_database_reference: &'a types::Reference,
    ) -> Result<ResponseValue<types::PostgresConfig>, Error<types::Error>> {
        let url = format!(
            "{}/project/{}/location/{}/postgres/{}/config",
            self.baseurl,
            encode_path(&project_id.to_string()),
            encode_path(&location.to_string()),
            encode_path(&postgres_database_reference.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_postgres_database_config",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Replace PostgreSQL database configuration
    ///
    ///Sends a `POST` request to
    /// `/project/{project_id}/location/{location}/postgres/
    /// {postgres_database_reference}/config`
    ///
    ///Arguments:
    /// - `project_id`: ID of the project
    /// - `location`: The Ubicloud location/region
    /// - `postgres_database_reference`: Postgres database ID or name
    /// - `body`
    pub async fn update_postgres_database_config<'a>(
        &'a self,
        project_id: &'a types::UpdatePostgresDatabaseConfigProjectId,
        location: &'a str,
        postgres_database_reference: &'a types::Reference,
        body: &'a types::PostgresConfig,
    ) -> Result<ResponseValue<types::PostgresConfig>, Error<types::Error>> {
        let url = format!(
            "{}/project/{}/location/{}/postgres/{}/config",
            self.baseurl,
            encode_path(&project_id.to_string()),
            encode_path(&location.to_string()),
            encode_path(&postgres_database_reference.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "update_postgres_database_config",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Update PostgreSQL database configuration
    ///
    ///Sends a `PATCH` request to
    /// `/project/{project_id}/location/{location}/postgres/
    /// {postgres_database_reference}/config`
    ///
    ///Arguments:
    /// - `project_id`: ID of the project
    /// - `location`: The Ubicloud location/region
    /// - `postgres_database_reference`: Postgres database ID or name
    /// - `body`
    pub async fn patch_postgres_database_config<'a>(
        &'a self,
        project_id: &'a types::PatchPostgresDatabaseConfigProjectId,
        location: &'a str,
        postgres_database_reference: &'a types::Reference,
        body: &'a types::PostgresPatchConfig,
    ) -> Result<ResponseValue<types::PostgresConfig>, Error<types::Error>> {
        let url = format!(
            "{}/project/{}/location/{}/postgres/{}/config",
            self.baseurl,
            encode_path(&project_id.to_string()),
            encode_path(&location.to_string()),
            encode_path(&postgres_database_reference.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .patch(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "patch_postgres_database_config",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///List PostgreSQL firewall rules
    ///
    ///Sends a `GET` request to
    /// `/project/{project_id}/location/{location}/postgres/
    /// {postgres_database_reference}/firewall-rule`
    ///
    ///Arguments:
    /// - `project_id`: ID of the project
    /// - `location`: The Ubicloud location/region
    /// - `postgres_database_reference`: Postgres database ID or name
    pub async fn list_location_postgres_firewall_rules<'a>(
        &'a self,
        project_id: &'a types::ListLocationPostgresFirewallRulesProjectId,
        location: &'a str,
        postgres_database_reference: &'a types::Reference,
    ) -> Result<ResponseValue<types::ListLocationPostgresFirewallRulesResponse>, Error<types::Error>>
    {
        let url = format!(
            "{}/project/{}/location/{}/postgres/{}/firewall-rule",
            self.baseurl,
            encode_path(&project_id.to_string()),
            encode_path(&location.to_string()),
            encode_path(&postgres_database_reference.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "list_location_postgres_firewall_rules",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Create a new PostgreSQL firewall rule
    ///
    ///Sends a `POST` request to
    /// `/project/{project_id}/location/{location}/postgres/
    /// {postgres_database_reference}/firewall-rule`
    ///
    ///Arguments:
    /// - `project_id`: ID of the project
    /// - `location`: The Ubicloud location/region
    /// - `postgres_database_reference`: Postgres database ID or name
    /// - `body`
    pub async fn create_location_postgres_firewall_rule<'a>(
        &'a self,
        project_id: &'a types::CreateLocationPostgresFirewallRuleProjectId,
        location: &'a str,
        postgres_database_reference: &'a types::Reference,
        body: &'a types::CreateLocationPostgresFirewallRuleBody,
    ) -> Result<ResponseValue<types::PostgresFirewallRule>, Error<types::Error>> {
        let url = format!(
            "{}/project/{}/location/{}/postgres/{}/firewall-rule",
            self.baseurl,
            encode_path(&project_id.to_string()),
            encode_path(&location.to_string()),
            encode_path(&postgres_database_reference.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "create_location_postgres_firewall_rule",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Delete a PostgreSQL firewall rule
    ///
    ///Sends a `DELETE` request to
    /// `/project/{project_id}/location/{location}/postgres/
    /// {postgres_database_reference}/firewall-rule/{firewall_rule_id}`
    ///
    ///Arguments:
    /// - `project_id`: ID of the project
    /// - `location`: The Ubicloud location/region
    /// - `postgres_database_reference`: Postgres database ID or name
    /// - `firewall_rule_id`: ID of the firewall rule
    pub async fn delete_location_postgres_firewall_rule<'a>(
        &'a self,
        project_id: &'a types::DeleteLocationPostgresFirewallRuleProjectId,
        location: &'a str,
        postgres_database_reference: &'a types::Reference,
        firewall_rule_id: &'a types::DeleteLocationPostgresFirewallRuleFirewallRuleId,
    ) -> Result<ResponseValue<()>, Error<types::Error>> {
        let url = format!(
            "{}/project/{}/location/{}/postgres/{}/firewall-rule/{}",
            self.baseurl,
            encode_path(&project_id.to_string()),
            encode_path(&location.to_string()),
            encode_path(&postgres_database_reference.to_string()),
            encode_path(&firewall_rule_id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .delete(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "delete_location_postgres_firewall_rule",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            204u16 => Ok(ResponseValue::empty(response)),
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Update a PostgreSQL firewall rule
    ///
    ///Sends a `PATCH` request to
    /// `/project/{project_id}/location/{location}/postgres/
    /// {postgres_database_reference}/firewall-rule/{firewall_rule_id}`
    ///
    ///Arguments:
    /// - `project_id`: ID of the project
    /// - `location`: The Ubicloud location/region
    /// - `postgres_database_reference`: Postgres database ID or name
    /// - `firewall_rule_id`: ID of the firewall rule
    /// - `body`
    pub async fn patch_location_postgres_firewall_rule<'a>(
        &'a self,
        project_id: &'a types::PatchLocationPostgresFirewallRuleProjectId,
        location: &'a str,
        postgres_database_reference: &'a types::Reference,
        firewall_rule_id: &'a types::PatchLocationPostgresFirewallRuleFirewallRuleId,
        body: &'a types::PatchLocationPostgresFirewallRuleBody,
    ) -> Result<ResponseValue<types::PostgresFirewallRule>, Error<types::Error>> {
        let url = format!(
            "{}/project/{}/location/{}/postgres/{}/firewall-rule/{}",
            self.baseurl,
            encode_path(&project_id.to_string()),
            encode_path(&location.to_string()),
            encode_path(&postgres_database_reference.to_string()),
            encode_path(&firewall_rule_id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .patch(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "patch_location_postgres_firewall_rule",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Create a new PostgreSQL log destination
    ///
    ///Sends a `POST` request to
    /// `/project/{project_id}/location/{location}/postgres/
    /// {postgres_database_reference}/log-destination`
    ///
    ///Arguments:
    /// - `project_id`: ID of the project
    /// - `location`: The Ubicloud location/region
    /// - `postgres_database_reference`: Postgres database ID or name
    /// - `body`
    pub async fn create_location_postgres_log_destination<'a>(
        &'a self,
        project_id: &'a types::CreateLocationPostgresLogDestinationProjectId,
        location: &'a str,
        postgres_database_reference: &'a types::Reference,
        body: &'a types::CreateLocationPostgresLogDestinationBody,
    ) -> Result<ResponseValue<types::PostgresLogDestination>, Error<types::Error>> {
        let url = format!(
            "{}/project/{}/location/{}/postgres/{}/log-destination",
            self.baseurl,
            encode_path(&project_id.to_string()),
            encode_path(&location.to_string()),
            encode_path(&postgres_database_reference.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "create_location_postgres_log_destination",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Delete a specific log destination
    ///
    ///Sends a `DELETE` request to
    /// `/project/{project_id}/location/{location}/postgres/
    /// {postgres_database_reference}/log-destination/{log_destination_id}`
    ///
    ///Arguments:
    /// - `project_id`: ID of the project
    /// - `location`: The Ubicloud location/region
    /// - `postgres_database_reference`: Postgres database ID or name
    /// - `log_destination_id`: Postgres Log Destination ID
    pub async fn delete_location_postgres_log_destination<'a>(
        &'a self,
        project_id: &'a types::DeleteLocationPostgresLogDestinationProjectId,
        location: &'a str,
        postgres_database_reference: &'a types::Reference,
        log_destination_id: &'a types::DeleteLocationPostgresLogDestinationLogDestinationId,
    ) -> Result<ResponseValue<()>, Error<types::Error>> {
        let url = format!(
            "{}/project/{}/location/{}/postgres/{}/log-destination/{}",
            self.baseurl,
            encode_path(&project_id.to_string()),
            encode_path(&location.to_string()),
            encode_path(&postgres_database_reference.to_string()),
            encode_path(&log_destination_id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .delete(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "delete_location_postgres_log_destination",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            204u16 => Ok(ResponseValue::empty(response)),
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Get PostgreSQL database logs
    ///
    ///Sends a `GET` request to
    /// `/project/{project_id}/location/{location}/postgres/
    /// {postgres_database_reference}/logs`
    ///
    ///Arguments:
    /// - `project_id`: ID of the project
    /// - `location`: The Ubicloud location/region
    /// - `postgres_database_reference`: Postgres database ID or name
    /// - `end`: End time for log query (rfc3339 format, default now)
    /// - `max_log_lines`: Number of log lines to return (default 50, max 500)
    /// - `pagination_key`: Pagination key returned from a previous response
    /// - `query_pattern`: Pattern for substring match against log message body
    /// - `server_role`: Filter by server role (primary or standby)
    /// - `severity_level`: Filter by log severity level
    /// - `start`: Start time for log query (rfc3339 format, default 30 minutes
    ///   ago)
    /// - `stream_name`: Filter by log stream (postgres, pgbouncer, or upgrade)
    pub async fn get_postgres_database_logs<'a>(
        &'a self,
        project_id: &'a types::GetPostgresDatabaseLogsProjectId,
        location: &'a str,
        postgres_database_reference: &'a types::Reference,
        end: Option<&'a str>,
        max_log_lines: Option<i64>,
        pagination_key: Option<&'a types::GetPostgresDatabaseLogsPaginationKey>,
        query_pattern: Option<&'a str>,
        server_role: Option<types::GetPostgresDatabaseLogsServerRole>,
        severity_level: Option<types::GetPostgresDatabaseLogsSeverityLevel>,
        start: Option<&'a str>,
        stream_name: Option<types::GetPostgresDatabaseLogsStreamName>,
    ) -> Result<ResponseValue<types::GetPostgresDatabaseLogsResponse>, Error<types::Error>> {
        let url = format!(
            "{}/project/{}/location/{}/postgres/{}/logs",
            self.baseurl,
            encode_path(&project_id.to_string()),
            encode_path(&location.to_string()),
            encode_path(&postgres_database_reference.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("end", &end))
            .query(&progenitor_client::QueryParam::new(
                "max_log_lines",
                &max_log_lines,
            ))
            .query(&progenitor_client::QueryParam::new(
                "pagination_key",
                &pagination_key,
            ))
            .query(&progenitor_client::QueryParam::new(
                "query_pattern",
                &query_pattern,
            ))
            .query(&progenitor_client::QueryParam::new(
                "server_role",
                &server_role,
            ))
            .query(&progenitor_client::QueryParam::new(
                "severity_level",
                &severity_level,
            ))
            .query(&progenitor_client::QueryParam::new("start", &start))
            .query(&progenitor_client::QueryParam::new(
                "stream_name",
                &stream_name,
            ))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_postgres_database_logs",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Create a new PostgreSQL metric destination
    ///
    ///Sends a `POST` request to
    /// `/project/{project_id}/location/{location}/postgres/
    /// {postgres_database_reference}/metric-destination`
    ///
    ///Arguments:
    /// - `project_id`: ID of the project
    /// - `location`: The Ubicloud location/region
    /// - `postgres_database_reference`: Postgres database ID or name
    /// - `body`
    pub async fn create_location_postgres_metric_destination<'a>(
        &'a self,
        project_id: &'a types::CreateLocationPostgresMetricDestinationProjectId,
        location: &'a str,
        postgres_database_reference: &'a types::Reference,
        body: &'a types::CreateLocationPostgresMetricDestinationBody,
    ) -> Result<
        ResponseValue<types::CreateLocationPostgresMetricDestinationResponse>,
        Error<types::Error>,
    > {
        let url = format!(
            "{}/project/{}/location/{}/postgres/{}/metric-destination",
            self.baseurl,
            encode_path(&project_id.to_string()),
            encode_path(&location.to_string()),
            encode_path(&postgres_database_reference.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "create_location_postgres_metric_destination",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Delete a specific metric destination
    ///
    ///Sends a `DELETE` request to
    /// `/project/{project_id}/location/{location}/postgres/
    /// {postgres_database_reference}/metric-destination/
    /// {metric_destination_id}`
    ///
    ///Arguments:
    /// - `project_id`: ID of the project
    /// - `location`: The Ubicloud location/region
    /// - `postgres_database_reference`: Postgres database ID or name
    /// - `metric_destination_id`: Postgres Metric Destination ID
    pub async fn delete_location_postgres_metric_destination<'a>(
        &'a self,
        project_id: &'a types::DeleteLocationPostgresMetricDestinationProjectId,
        location: &'a str,
        postgres_database_reference: &'a types::Reference,
        metric_destination_id : & 'a types :: DeleteLocationPostgresMetricDestinationMetricDestinationId,
    ) -> Result<ResponseValue<()>, Error<types::Error>> {
        let url = format!(
            "{}/project/{}/location/{}/postgres/{}/metric-destination/{}",
            self.baseurl,
            encode_path(&project_id.to_string()),
            encode_path(&location.to_string()),
            encode_path(&postgres_database_reference.to_string()),
            encode_path(&metric_destination_id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .delete(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "delete_location_postgres_metric_destination",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            204u16 => Ok(ResponseValue::empty(response)),
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Get PostgreSQL database metrics
    ///
    ///Sends a `GET` request to
    /// `/project/{project_id}/location/{location}/postgres/
    /// {postgres_database_reference}/metrics`
    ///
    ///Arguments:
    /// - `project_id`: ID of the project
    /// - `location`: The Ubicloud location/region
    /// - `postgres_database_reference`: Postgres database ID or name
    /// - `end`: End time for metrics query (rfc3339 format)
    /// - `key`: Key of the specific metric to retrieve
    /// - `start`: Start time for metrics query (rfc3339 format)
    pub async fn get_postgres_database_metrics<'a>(
        &'a self,
        project_id: &'a types::GetPostgresDatabaseMetricsProjectId,
        location: &'a str,
        postgres_database_reference: &'a types::Reference,
        end: Option<&'a str>,
        key: Option<&'a str>,
        start: Option<&'a str>,
    ) -> Result<ResponseValue<types::ResourceMetrics>, Error<types::Error>> {
        let url = format!(
            "{}/project/{}/location/{}/postgres/{}/metrics",
            self.baseurl,
            encode_path(&project_id.to_string()),
            encode_path(&location.to_string()),
            encode_path(&postgres_database_reference.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("end", &end))
            .query(&progenitor_client::QueryParam::new("key", &key))
            .query(&progenitor_client::QueryParam::new("start", &start))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_postgres_database_metrics",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Promote a PostgreSQL read replica (deprecated, use promote-read-replica)
    ///
    ///Sends a `POST` request to
    /// `/project/{project_id}/location/{location}/postgres/
    /// {postgres_database_reference}/promote`
    ///
    ///Arguments:
    /// - `project_id`: ID of the project
    /// - `location`: The Ubicloud location/region
    /// - `postgres_database_reference`: Postgres database ID or name
    pub async fn promote_postgres_database<'a>(
        &'a self,
        project_id: &'a types::PromotePostgresDatabaseProjectId,
        location: &'a str,
        postgres_database_reference: &'a types::Reference,
    ) -> Result<ResponseValue<types::PromotePostgresDatabaseResponse>, Error<types::Error>> {
        let url = format!(
            "{}/project/{}/location/{}/postgres/{}/promote",
            self.baseurl,
            encode_path(&project_id.to_string()),
            encode_path(&location.to_string()),
            encode_path(&postgres_database_reference.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "promote_postgres_database",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Promote a PostgreSQL read replica
    ///
    ///Sends a `POST` request to
    /// `/project/{project_id}/location/{location}/postgres/
    /// {postgres_database_reference}/promote-read-replica`
    ///
    ///Arguments:
    /// - `project_id`: ID of the project
    /// - `location`: The Ubicloud location/region
    /// - `postgres_database_reference`: Postgres database ID or name
    pub async fn promote_read_replica_postgres_database<'a>(
        &'a self,
        project_id: &'a types::PromoteReadReplicaPostgresDatabaseProjectId,
        location: &'a str,
        postgres_database_reference: &'a types::Reference,
    ) -> Result<ResponseValue<types::PromoteReadReplicaPostgresDatabaseResponse>, Error<types::Error>>
    {
        let url = format!(
            "{}/project/{}/location/{}/postgres/{}/promote-read-replica",
            self.baseurl,
            encode_path(&project_id.to_string()),
            encode_path(&location.to_string()),
            encode_path(&postgres_database_reference.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "promote_read_replica_postgres_database",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Create a read replica of the PostgreSQL database
    ///
    ///Sends a `POST` request to
    /// `/project/{project_id}/location/{location}/postgres/
    /// {postgres_database_reference}/read-replica`
    ///
    ///Arguments:
    /// - `project_id`: ID of the project
    /// - `location`: The Ubicloud location/region
    /// - `postgres_database_reference`: Postgres database ID or name
    /// - `body`
    pub async fn create_postgres_database_read_replica<'a>(
        &'a self,
        project_id: &'a types::CreatePostgresDatabaseReadReplicaProjectId,
        location: &'a str,
        postgres_database_reference: &'a types::Reference,
        body: &'a types::CreatePostgresDatabaseReadReplicaBody,
    ) -> Result<ResponseValue<types::CreatePostgresDatabaseReadReplicaResponse>, Error<types::Error>>
    {
        let url = format!(
            "{}/project/{}/location/{}/postgres/{}/read-replica",
            self.baseurl,
            encode_path(&project_id.to_string()),
            encode_path(&location.to_string()),
            encode_path(&postgres_database_reference.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "create_postgres_database_read_replica",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Request primary to be recycled
    ///
    ///Sends a `POST` request to
    /// `/project/{project_id}/location/{location}/postgres/
    /// {postgres_database_reference}/recycle`
    ///
    ///Arguments:
    /// - `project_id`: ID of the project
    /// - `location`: The Ubicloud location/region
    /// - `postgres_database_reference`: Postgres database ID or name
    pub async fn recycle<'a>(
        &'a self,
        project_id: &'a types::RecycleProjectId,
        location: &'a str,
        postgres_database_reference: &'a types::Reference,
    ) -> Result<ResponseValue<types::RecycleResponse>, Error<types::Error>> {
        let url = format!(
            "{}/project/{}/location/{}/postgres/{}/recycle",
            self.baseurl,
            encode_path(&project_id.to_string()),
            encode_path(&location.to_string()),
            encode_path(&postgres_database_reference.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "recycle",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Rename a PostgreSQL database
    ///
    ///Sends a `POST` request to
    /// `/project/{project_id}/location/{location}/postgres/
    /// {postgres_database_reference}/rename`
    ///
    ///Arguments:
    /// - `project_id`: ID of the project
    /// - `location`: The Ubicloud location/region
    /// - `postgres_database_reference`: Postgres database ID or name
    /// - `body`
    pub async fn rename_postgres<'a>(
        &'a self,
        project_id: &'a types::RenamePostgresProjectId,
        location: &'a str,
        postgres_database_reference: &'a types::Reference,
        body: &'a types::RenamePostgresBody,
    ) -> Result<ResponseValue<types::RenamePostgresResponse>, Error<types::Error>> {
        let url = format!(
            "{}/project/{}/location/{}/postgres/{}/rename",
            self.baseurl,
            encode_path(&project_id.to_string()),
            encode_path(&location.to_string()),
            encode_path(&postgres_database_reference.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "rename_postgres",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Reset superuser password of the PostgreSQL database
    ///
    ///Sends a `POST` request to
    /// `/project/{project_id}/location/{location}/postgres/
    /// {postgres_database_reference}/reset-superuser-password`
    ///
    ///Arguments:
    /// - `project_id`: ID of the project
    /// - `location`: The Ubicloud location/region
    /// - `postgres_database_reference`: Postgres database ID or name
    /// - `body`
    pub async fn reset_superuser_password<'a>(
        &'a self,
        project_id: &'a types::ResetSuperuserPasswordProjectId,
        location: &'a str,
        postgres_database_reference: &'a types::Reference,
        body: &'a types::ResetSuperuserPasswordBody,
    ) -> Result<ResponseValue<types::ResetSuperuserPasswordResponse>, Error<types::Error>> {
        let url = format!(
            "{}/project/{}/location/{}/postgres/{}/reset-superuser-password",
            self.baseurl,
            encode_path(&project_id.to_string()),
            encode_path(&location.to_string()),
            encode_path(&postgres_database_reference.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "reset_superuser_password",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Restart a PostgreSQL database
    ///
    ///Sends a `POST` request to
    /// `/project/{project_id}/location/{location}/postgres/
    /// {postgres_database_reference}/restart`
    ///
    ///Arguments:
    /// - `project_id`: ID of the project
    /// - `location`: The Ubicloud location/region
    /// - `postgres_database_reference`: Postgres database ID or name
    pub async fn restart_postgres_database<'a>(
        &'a self,
        project_id: &'a types::RestartPostgresDatabaseProjectId,
        location: &'a str,
        postgres_database_reference: &'a types::Reference,
    ) -> Result<ResponseValue<types::RestartPostgresDatabaseResponse>, Error<types::Error>> {
        let url = format!(
            "{}/project/{}/location/{}/postgres/{}/restart",
            self.baseurl,
            encode_path(&project_id.to_string()),
            encode_path(&location.to_string()),
            encode_path(&postgres_database_reference.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "restart_postgres_database",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Restore a PostgreSQL database
    ///
    ///Sends a `POST` request to
    /// `/project/{project_id}/location/{location}/postgres/
    /// {postgres_database_reference}/restore`
    ///
    ///Arguments:
    /// - `project_id`: ID of the project
    /// - `location`: The Ubicloud location/region
    /// - `postgres_database_reference`: Postgres database ID or name
    /// - `body`
    pub async fn restore_postgres_database<'a>(
        &'a self,
        project_id: &'a types::RestorePostgresDatabaseProjectId,
        location: &'a str,
        postgres_database_reference: &'a types::Reference,
        body: &'a types::RestorePostgresDatabaseBody,
    ) -> Result<ResponseValue<types::RestorePostgresDatabaseResponse>, Error<types::Error>> {
        let url = format!(
            "{}/project/{}/location/{}/postgres/{}/restore",
            self.baseurl,
            encode_path(&project_id.to_string()),
            encode_path(&location.to_string()),
            encode_path(&postgres_database_reference.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "restore_postgres_database",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///List servers for a specific PostgreSQL database
    ///
    ///Sends a `GET` request to
    /// `/project/{project_id}/location/{location}/postgres/
    /// {postgres_database_reference}/servers`
    ///
    ///Arguments:
    /// - `project_id`: ID of the project
    /// - `location`: The Ubicloud location/region
    /// - `postgres_database_reference`: Postgres database ID or name
    pub async fn list_postgres_database_servers<'a>(
        &'a self,
        project_id: &'a types::ListPostgresDatabaseServersProjectId,
        location: &'a str,
        postgres_database_reference: &'a types::Reference,
    ) -> Result<ResponseValue<types::ListPostgresDatabaseServersResponse>, Error<types::Error>>
    {
        let url = format!(
            "{}/project/{}/location/{}/postgres/{}/servers",
            self.baseurl,
            encode_path(&project_id.to_string()),
            encode_path(&location.to_string()),
            encode_path(&postgres_database_reference.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "list_postgres_database_servers",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Set maintenance window for the PostgreSQL database
    ///
    ///Sends a `POST` request to
    /// `/project/{project_id}/location/{location}/postgres/
    /// {postgres_database_reference}/set-maintenance-window`
    ///
    ///Arguments:
    /// - `project_id`: ID of the project
    /// - `location`: The Ubicloud location/region
    /// - `postgres_database_reference`: Postgres database ID or name
    /// - `body`
    pub async fn set_maintenance_window<'a>(
        &'a self,
        project_id: &'a types::SetMaintenanceWindowProjectId,
        location: &'a str,
        postgres_database_reference: &'a types::Reference,
        body: &'a types::SetMaintenanceWindowBody,
    ) -> Result<ResponseValue<types::SetMaintenanceWindowResponse>, Error<types::Error>> {
        let url = format!(
            "{}/project/{}/location/{}/postgres/{}/set-maintenance-window",
            self.baseurl,
            encode_path(&project_id.to_string()),
            encode_path(&location.to_string()),
            encode_path(&postgres_database_reference.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "set_maintenance_window",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Get PostgreSQL database upgrade status
    ///
    ///Sends a `GET` request to
    /// `/project/{project_id}/location/{location}/postgres/
    /// {postgres_database_reference}/upgrade`
    ///
    ///Arguments:
    /// - `project_id`: ID of the project
    /// - `location`: The Ubicloud location/region
    /// - `postgres_database_reference`: Postgres database ID or name
    pub async fn get_postgres_database_upgrade_status<'a>(
        &'a self,
        project_id: &'a types::GetPostgresDatabaseUpgradeStatusProjectId,
        location: &'a str,
        postgres_database_reference: &'a types::Reference,
    ) -> Result<ResponseValue<types::GetPostgresDatabaseUpgradeStatusResponse>, Error<types::Error>>
    {
        let url = format!(
            "{}/project/{}/location/{}/postgres/{}/upgrade",
            self.baseurl,
            encode_path(&project_id.to_string()),
            encode_path(&location.to_string()),
            encode_path(&postgres_database_reference.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_postgres_database_upgrade_status",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Upgrade a PostgreSQL database to the next major version
    ///
    ///Sends a `POST` request to
    /// `/project/{project_id}/location/{location}/postgres/
    /// {postgres_database_reference}/upgrade`
    ///
    ///Arguments:
    /// - `project_id`: ID of the project
    /// - `location`: The Ubicloud location/region
    /// - `postgres_database_reference`: Postgres database ID or name
    pub async fn upgrade_postgres_database<'a>(
        &'a self,
        project_id: &'a types::UpgradePostgresDatabaseProjectId,
        location: &'a str,
        postgres_database_reference: &'a types::Reference,
    ) -> Result<ResponseValue<types::UpgradePostgresDatabaseResponse>, Error<types::Error>> {
        let url = format!(
            "{}/project/{}/location/{}/postgres/{}/upgrade",
            self.baseurl,
            encode_path(&project_id.to_string()),
            encode_path(&location.to_string()),
            encode_path(&postgres_database_reference.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "upgrade_postgres_database",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///List private subnets in a location
    ///
    ///Sends a `GET` request to
    /// `/project/{project_id}/location/{location}/private-subnet`
    ///
    ///Arguments:
    /// - `project_id`: ID of the project
    /// - `location`: The Ubicloud location/region
    /// - `order_column`: Pagination - Order column
    /// - `page_size`: Pagination - Page size
    /// - `start_after`: Pagination - Start after
    pub async fn list_location_private_subnets<'a>(
        &'a self,
        project_id: &'a types::ListLocationPrivateSubnetsProjectId,
        location: &'a str,
        order_column: Option<&'a str>,
        page_size: Option<i64>,
        start_after: Option<&'a str>,
    ) -> Result<ResponseValue<types::ListLocationPrivateSubnetsResponse>, Error<types::Error>> {
        let url = format!(
            "{}/project/{}/location/{}/private-subnet",
            self.baseurl,
            encode_path(&project_id.to_string()),
            encode_path(&location.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new(
                "order_column",
                &order_column,
            ))
            .query(&progenitor_client::QueryParam::new("page_size", &page_size))
            .query(&progenitor_client::QueryParam::new(
                "start_after",
                &start_after,
            ))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "list_location_private_subnets",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Get private subnet details
    ///
    ///Sends a `GET` request to
    /// `/project/{project_id}/location/{location}/private-subnet/
    /// {private_subnet_reference}`
    ///
    ///Arguments:
    /// - `project_id`: ID of the project
    /// - `location`: The Ubicloud location/region
    /// - `private_subnet_reference`: Private subnet ID or name
    pub async fn get_private_subnet_details<'a>(
        &'a self,
        project_id: &'a types::GetPrivateSubnetDetailsProjectId,
        location: &'a str,
        private_subnet_reference: &'a types::Reference,
    ) -> Result<ResponseValue<types::PrivateSubnet>, Error<types::Error>> {
        let url = format!(
            "{}/project/{}/location/{}/private-subnet/{}",
            self.baseurl,
            encode_path(&project_id.to_string()),
            encode_path(&location.to_string()),
            encode_path(&private_subnet_reference.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_private_subnet_details",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Create a new private subnet
    ///
    ///Sends a `POST` request to
    /// `/project/{project_id}/location/{location}/private-subnet/
    /// {private_subnet_reference}`
    ///
    ///Arguments:
    /// - `project_id`: ID of the project
    /// - `location`: The Ubicloud location/region
    /// - `private_subnet_reference`: Private subnet ID or name
    /// - `body`
    pub async fn create_private_subnet<'a>(
        &'a self,
        project_id: &'a types::CreatePrivateSubnetProjectId,
        location: &'a str,
        private_subnet_reference: &'a types::Reference,
        body: &'a types::CreatePrivateSubnetBody,
    ) -> Result<ResponseValue<types::PrivateSubnet>, Error<types::Error>> {
        let url = format!(
            "{}/project/{}/location/{}/private-subnet/{}",
            self.baseurl,
            encode_path(&project_id.to_string()),
            encode_path(&location.to_string()),
            encode_path(&private_subnet_reference.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "create_private_subnet",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Delete a private subnet
    ///
    ///Sends a `DELETE` request to
    /// `/project/{project_id}/location/{location}/private-subnet/
    /// {private_subnet_reference}`
    ///
    ///Arguments:
    /// - `project_id`: ID of the project
    /// - `location`: The Ubicloud location/region
    /// - `private_subnet_reference`: Private subnet ID or name
    pub async fn delete_private_subnet<'a>(
        &'a self,
        project_id: &'a types::DeletePrivateSubnetProjectId,
        location: &'a str,
        private_subnet_reference: &'a types::Reference,
    ) -> Result<ResponseValue<()>, Error<types::Error>> {
        let url = format!(
            "{}/project/{}/location/{}/private-subnet/{}",
            self.baseurl,
            encode_path(&project_id.to_string()),
            encode_path(&location.to_string()),
            encode_path(&private_subnet_reference.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .delete(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "delete_private_subnet",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            204u16 => Ok(ResponseValue::empty(response)),
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Connect private subnet to another private subnet
    ///
    ///Sends a `POST` request to
    /// `/project/{project_id}/location/{location}/private-subnet/
    /// {private_subnet_reference}/connect`
    ///
    ///Arguments:
    /// - `project_id`: ID of the project
    /// - `location`: The Ubicloud location/region
    /// - `private_subnet_reference`: Private subnet ID or name
    /// - `body`
    pub async fn connect_private_subnet<'a>(
        &'a self,
        project_id: &'a types::ConnectPrivateSubnetProjectId,
        location: &'a str,
        private_subnet_reference: &'a types::Reference,
        body: &'a types::ConnectPrivateSubnetBody,
    ) -> Result<ResponseValue<types::PrivateSubnet>, Error<types::Error>> {
        let url = format!(
            "{}/project/{}/location/{}/private-subnet/{}/connect",
            self.baseurl,
            encode_path(&project_id.to_string()),
            encode_path(&location.to_string()),
            encode_path(&private_subnet_reference.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "connect_private_subnet",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Disconnect private subnet from another private subnet
    ///
    ///Sends a `POST` request to
    /// `/project/{project_id}/location/{location}/private-subnet/
    /// {private_subnet_reference}/disconnect/{private_subnet_id}`
    ///
    ///Arguments:
    /// - `project_id`: ID of the project
    /// - `location`: The Ubicloud location/region
    /// - `private_subnet_reference`: Private subnet ID or name
    /// - `private_subnet_id`: ID of the private subnet
    pub async fn disconnect_private_subnet<'a>(
        &'a self,
        project_id: &'a types::DisconnectPrivateSubnetProjectId,
        location: &'a str,
        private_subnet_reference: &'a types::Reference,
        private_subnet_id: &'a types::DisconnectPrivateSubnetPrivateSubnetId,
    ) -> Result<ResponseValue<types::PrivateSubnet>, Error<types::Error>> {
        let url = format!(
            "{}/project/{}/location/{}/private-subnet/{}/disconnect/{}",
            self.baseurl,
            encode_path(&project_id.to_string()),
            encode_path(&location.to_string()),
            encode_path(&private_subnet_reference.to_string()),
            encode_path(&private_subnet_id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "disconnect_private_subnet",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Rename a private subnet
    ///
    ///Sends a `POST` request to
    /// `/project/{project_id}/location/{location}/private-subnet/
    /// {private_subnet_reference}/rename`
    ///
    ///Arguments:
    /// - `project_id`: ID of the project
    /// - `location`: The Ubicloud location/region
    /// - `private_subnet_reference`: Private subnet ID or name
    /// - `body`
    pub async fn rename_private_subnet<'a>(
        &'a self,
        project_id: &'a types::RenamePrivateSubnetProjectId,
        location: &'a str,
        private_subnet_reference: &'a types::Reference,
        body: &'a types::RenamePrivateSubnetBody,
    ) -> Result<ResponseValue<types::PrivateSubnet>, Error<types::Error>> {
        let url = format!(
            "{}/project/{}/location/{}/private-subnet/{}/rename",
            self.baseurl,
            encode_path(&project_id.to_string()),
            encode_path(&location.to_string()),
            encode_path(&private_subnet_reference.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "rename_private_subnet",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///List VMs in a location
    ///
    ///Sends a `GET` request to `/project/{project_id}/location/{location}/vm`
    ///
    ///Arguments:
    /// - `project_id`: ID of the project
    /// - `location`: The Ubicloud location/region
    /// - `order_column`: Pagination - Order column
    /// - `page_size`: Pagination - Page size
    /// - `start_after`: Pagination - Start after
    pub async fn list_location_v_ms<'a>(
        &'a self,
        project_id: &'a types::ListLocationVMsProjectId,
        location: &'a str,
        order_column: Option<&'a str>,
        page_size: Option<i64>,
        start_after: Option<&'a str>,
    ) -> Result<ResponseValue<types::ListLocationVMsResponse>, Error<types::Error>> {
        let url = format!(
            "{}/project/{}/location/{}/vm",
            self.baseurl,
            encode_path(&project_id.to_string()),
            encode_path(&location.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new(
                "order_column",
                &order_column,
            ))
            .query(&progenitor_client::QueryParam::new("page_size", &page_size))
            .query(&progenitor_client::QueryParam::new(
                "start_after",
                &start_after,
            ))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "list_location_v_ms",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Get VM details
    ///
    ///Sends a `GET` request to
    /// `/project/{project_id}/location/{location}/vm/{vm_reference}`
    ///
    ///Arguments:
    /// - `project_id`: ID of the project
    /// - `location`: The Ubicloud location/region
    /// - `vm_reference`: Virtual machine ID or name
    pub async fn get_vm_details<'a>(
        &'a self,
        project_id: &'a types::GetVmDetailsProjectId,
        location: &'a str,
        vm_reference: &'a types::Reference,
    ) -> Result<ResponseValue<types::VmResponse>, Error<types::Error>> {
        let url = format!(
            "{}/project/{}/location/{}/vm/{}",
            self.baseurl,
            encode_path(&project_id.to_string()),
            encode_path(&location.to_string()),
            encode_path(&vm_reference.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_vm_details",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Create a new VM
    ///
    ///Sends a `POST` request to
    /// `/project/{project_id}/location/{location}/vm/{vm_reference}`
    ///
    ///Arguments:
    /// - `project_id`: ID of the project
    /// - `location`: The Ubicloud location/region
    /// - `vm_reference`: Virtual machine ID or name
    /// - `body`
    pub async fn create_vm<'a>(
        &'a self,
        project_id: &'a types::CreateVmProjectId,
        location: &'a str,
        vm_reference: &'a types::Reference,
        body: &'a types::CreateVmBody,
    ) -> Result<ResponseValue<types::VmResponse>, Error<types::Error>> {
        let url = format!(
            "{}/project/{}/location/{}/vm/{}",
            self.baseurl,
            encode_path(&project_id.to_string()),
            encode_path(&location.to_string()),
            encode_path(&vm_reference.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "create_vm",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Delete a VM
    ///
    ///Sends a `DELETE` request to
    /// `/project/{project_id}/location/{location}/vm/{vm_reference}`
    ///
    ///Arguments:
    /// - `project_id`: ID of the project
    /// - `location`: The Ubicloud location/region
    /// - `vm_reference`: Virtual machine ID or name
    pub async fn delete_vm<'a>(
        &'a self,
        project_id: &'a types::DeleteVmProjectId,
        location: &'a str,
        vm_reference: &'a types::Reference,
    ) -> Result<ResponseValue<()>, Error<types::Error>> {
        let url = format!(
            "{}/project/{}/location/{}/vm/{}",
            self.baseurl,
            encode_path(&project_id.to_string()),
            encode_path(&location.to_string()),
            encode_path(&vm_reference.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .delete(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "delete_vm",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            204u16 => Ok(ResponseValue::empty(response)),
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Rename a VM
    ///
    ///Sends a `POST` request to
    /// `/project/{project_id}/location/{location}/vm/{vm_reference}/rename`
    ///
    ///Arguments:
    /// - `project_id`: ID of the project
    /// - `location`: The Ubicloud location/region
    /// - `vm_reference`: Virtual machine ID or name
    /// - `body`
    pub async fn rename_vm<'a>(
        &'a self,
        project_id: &'a types::RenameVmProjectId,
        location: &'a str,
        vm_reference: &'a types::Reference,
        body: &'a types::RenameVmBody,
    ) -> Result<ResponseValue<types::VmResponse>, Error<types::Error>> {
        let url = format!(
            "{}/project/{}/location/{}/vm/{}/rename",
            self.baseurl,
            encode_path(&project_id.to_string()),
            encode_path(&location.to_string()),
            encode_path(&vm_reference.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "rename_vm",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Restart a VM
    ///
    ///Sends a `POST` request to
    /// `/project/{project_id}/location/{location}/vm/{vm_reference}/restart`
    ///
    ///Arguments:
    /// - `project_id`: ID of the project
    /// - `location`: The Ubicloud location/region
    /// - `vm_reference`: Virtual machine ID or name
    pub async fn restart_vm<'a>(
        &'a self,
        project_id: &'a types::RestartVmProjectId,
        location: &'a str,
        vm_reference: &'a types::Reference,
    ) -> Result<ResponseValue<types::VmResponse>, Error<types::Error>> {
        let url = format!(
            "{}/project/{}/location/{}/vm/{}/restart",
            self.baseurl,
            encode_path(&project_id.to_string()),
            encode_path(&location.to_string()),
            encode_path(&vm_reference.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "restart_vm",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Get a VM's serial console log, fetching it first if it has never been
    /// fetched
    ///
    ///Sends a `GET` request to
    /// `/project/{project_id}/location/{location}/vm/{vm_reference}/serial-log`
    ///
    ///Arguments:
    /// - `project_id`: ID of the project
    /// - `location`: The Ubicloud location/region
    /// - `vm_reference`: Virtual machine ID or name
    /// - `refresh`: Request a newer fetch than the last one (ignored if the
    ///   last fetch was too recent, and done automatically regardless once the
    ///   last result is old enough)
    pub async fn get_vm_serial_log<'a>(
        &'a self,
        project_id: &'a types::GetVmSerialLogProjectId,
        location: &'a str,
        vm_reference: &'a types::Reference,
        refresh: Option<bool>,
    ) -> Result<ResponseValue<types::RunCommand>, Error<types::Error>> {
        let url = format!(
            "{}/project/{}/location/{}/vm/{}/serial-log",
            self.baseurl,
            encode_path(&project_id.to_string()),
            encode_path(&location.to_string()),
            encode_path(&vm_reference.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new("refresh", &refresh))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_vm_serial_log",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            202u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Set maintenance window for the VM
    ///
    ///Sends a `POST` request to
    /// `/project/{project_id}/location/{location}/vm/{vm_reference}/
    /// set-maintenance-window`
    ///
    ///Arguments:
    /// - `project_id`: ID of the project
    /// - `location`: The Ubicloud location/region
    /// - `vm_reference`: Virtual machine ID or name
    /// - `body`
    pub async fn set_vm_maintenance_window<'a>(
        &'a self,
        project_id: &'a types::SetVmMaintenanceWindowProjectId,
        location: &'a str,
        vm_reference: &'a types::Reference,
        body: &'a types::SetVmMaintenanceWindowBody,
    ) -> Result<ResponseValue<types::VmResponse>, Error<types::Error>> {
        let url = format!(
            "{}/project/{}/location/{}/vm/{}/set-maintenance-window",
            self.baseurl,
            encode_path(&project_id.to_string()),
            encode_path(&location.to_string()),
            encode_path(&vm_reference.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "set_vm_maintenance_window",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Start a VM
    ///
    ///Sends a `POST` request to
    /// `/project/{project_id}/location/{location}/vm/{vm_reference}/start`
    ///
    ///Arguments:
    /// - `project_id`: ID of the project
    /// - `location`: The Ubicloud location/region
    /// - `vm_reference`: Virtual machine ID or name
    pub async fn start_vm<'a>(
        &'a self,
        project_id: &'a types::StartVmProjectId,
        location: &'a str,
        vm_reference: &'a types::Reference,
    ) -> Result<ResponseValue<types::VmResponse>, Error<types::Error>> {
        let url = format!(
            "{}/project/{}/location/{}/vm/{}/start",
            self.baseurl,
            encode_path(&project_id.to_string()),
            encode_path(&location.to_string()),
            encode_path(&vm_reference.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "start_vm",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Stop a VM
    ///
    ///Sends a `POST` request to
    /// `/project/{project_id}/location/{location}/vm/{vm_reference}/stop`
    ///
    ///Arguments:
    /// - `project_id`: ID of the project
    /// - `location`: The Ubicloud location/region
    /// - `vm_reference`: Virtual machine ID or name
    pub async fn stop_vm<'a>(
        &'a self,
        project_id: &'a types::StopVmProjectId,
        location: &'a str,
        vm_reference: &'a types::Reference,
    ) -> Result<ResponseValue<types::VmResponse>, Error<types::Error>> {
        let url = format!(
            "{}/project/{}/location/{}/vm/{}/stop",
            self.baseurl,
            encode_path(&project_id.to_string()),
            encode_path(&location.to_string()),
            encode_path(&vm_reference.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "stop_vm",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///List all machine images
    ///
    ///Sends a `GET` request to `/project/{project_id}/machine-image`
    ///
    ///Arguments:
    /// - `project_id`: ID of the project
    /// - `order_column`: Pagination - Order column
    /// - `page_size`: Pagination - Page size
    /// - `start_after`: Pagination - Start after
    pub async fn list_project_machine_images<'a>(
        &'a self,
        project_id: &'a types::ListProjectMachineImagesProjectId,
        order_column: Option<&'a str>,
        page_size: Option<i64>,
        start_after: Option<&'a str>,
    ) -> Result<ResponseValue<types::ListProjectMachineImagesResponse>, Error<types::Error>> {
        let url = format!(
            "{}/project/{}/machine-image",
            self.baseurl,
            encode_path(&project_id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new(
                "order_column",
                &order_column,
            ))
            .query(&progenitor_client::QueryParam::new("page_size", &page_size))
            .query(&progenitor_client::QueryParam::new(
                "start_after",
                &start_after,
            ))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "list_project_machine_images",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Return information on object type, location, and name
    ///
    ///Sends a `GET` request to `/project/{project_id}/object-info/{object_id}`
    ///
    ///Arguments:
    /// - `project_id`: ID of the project
    /// - `object_id`: ID of a supported object
    pub async fn get_object_info<'a>(
        &'a self,
        project_id: &'a types::GetObjectInfoProjectId,
        object_id: &'a types::GetObjectInfoObjectId,
    ) -> Result<ResponseValue<types::GetObjectInfoResponse>, Error<types::Error>> {
        let url = format!(
            "{}/project/{}/object-info/{}",
            self.baseurl,
            encode_path(&project_id.to_string()),
            encode_path(&object_id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_object_info",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///List all PostgreSQL databases
    ///
    ///Sends a `GET` request to `/project/{project_id}/postgres`
    ///
    ///Arguments:
    /// - `project_id`: ID of the project
    /// - `order_column`: Pagination - Order column
    /// - `page_size`: Pagination - Page size
    /// - `start_after`: Pagination - Start after
    /// - `tags`: Filter by tags in key:value format (comma-separated for
    ///   multiple tags)
    pub async fn list_postgres_databases<'a>(
        &'a self,
        project_id: &'a types::ListPostgresDatabasesProjectId,
        order_column: Option<&'a str>,
        page_size: Option<i64>,
        start_after: Option<&'a str>,
        tags: Option<&'a str>,
    ) -> Result<ResponseValue<types::ListPostgresDatabasesResponse>, Error<types::Error>> {
        let url = format!(
            "{}/project/{}/postgres",
            self.baseurl,
            encode_path(&project_id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new(
                "order_column",
                &order_column,
            ))
            .query(&progenitor_client::QueryParam::new("page_size", &page_size))
            .query(&progenitor_client::QueryParam::new(
                "start_after",
                &start_after,
            ))
            .query(&progenitor_client::QueryParam::new("tags", &tags))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "list_postgres_databases",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Get available PostgreSQL provisioning options
    ///
    ///Returns a tree of valid option combinations for creating PostgreSQL
    /// databases, along with metadata describing each option value. The tree
    /// encodes dependency chains (e.g. which VM families are available per
    /// location, which storage sizes are valid per VM size).
    ///
    ///Sends a `GET` request to `/project/{project_id}/postgres/capabilities`
    ///
    ///Arguments:
    /// - `project_id`: ID of the project
    pub async fn get_postgres_capabilities<'a>(
        &'a self,
        project_id: &'a types::GetPostgresCapabilitiesProjectId,
    ) -> Result<ResponseValue<types::PostgresCapabilities>, Error<types::Error>> {
        let url = format!(
            "{}/project/{}/postgres/capabilities",
            self.baseurl,
            encode_path(&project_id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_postgres_capabilities",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///List all private locations
    ///
    ///Sends a `GET` request to `/project/{project_id}/private-location`
    ///
    ///Arguments:
    /// - `project_id`: ID of the project
    pub async fn get_private_locations<'a>(
        &'a self,
        project_id: &'a types::GetPrivateLocationsProjectId,
    ) -> Result<ResponseValue<types::GetPrivateLocationsResponse>, Error<types::Error>> {
        let url = format!(
            "{}/project/{}/private-location",
            self.baseurl,
            encode_path(&project_id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_private_locations",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Create a new private location
    ///
    ///Sends a `POST` request to `/project/{project_id}/private-location`
    ///
    ///Arguments:
    /// - `project_id`: ID of the project
    /// - `body`
    pub async fn create_private_location<'a>(
        &'a self,
        project_id: &'a types::CreatePrivateLocationProjectId,
        body: &'a types::CreatePrivateLocationBody,
    ) -> Result<ResponseValue<types::PrivateLocation>, Error<types::Error>> {
        let url = format!(
            "{}/project/{}/private-location",
            self.baseurl,
            encode_path(&project_id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "create_private_location",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Get private location details
    ///
    ///Sends a `GET` request to
    /// `/project/{project_id}/private-location/{private_location_name}`
    ///
    ///Arguments:
    /// - `project_id`: ID of the project
    /// - `private_location_name`: Name of the private location
    pub async fn get_private_location_details<'a>(
        &'a self,
        project_id: &'a types::GetPrivateLocationDetailsProjectId,
        private_location_name: &'a str,
    ) -> Result<ResponseValue<types::PrivateLocation>, Error<types::Error>> {
        let url = format!(
            "{}/project/{}/private-location/{}",
            self.baseurl,
            encode_path(&project_id.to_string()),
            encode_path(&private_location_name.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_private_location_details",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Update a private location
    ///
    ///Sends a `POST` request to
    /// `/project/{project_id}/private-location/{private_location_name}`
    ///
    ///Arguments:
    /// - `project_id`: ID of the project
    /// - `private_location_name`: Name of the private location
    /// - `body`
    pub async fn update_private_location<'a>(
        &'a self,
        project_id: &'a types::UpdatePrivateLocationProjectId,
        private_location_name: &'a str,
        body: &'a types::UpdatePrivateLocationBody,
    ) -> Result<ResponseValue<types::PrivateLocation>, Error<types::Error>> {
        let url = format!(
            "{}/project/{}/private-location/{}",
            self.baseurl,
            encode_path(&project_id.to_string()),
            encode_path(&private_location_name.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "update_private_location",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Delete a private location
    ///
    ///Sends a `DELETE` request to
    /// `/project/{project_id}/private-location/{private_location_name}`
    ///
    ///Arguments:
    /// - `project_id`: ID of the project
    /// - `private_location_name`: Name of the private location
    pub async fn delete_private_location<'a>(
        &'a self,
        project_id: &'a types::DeletePrivateLocationProjectId,
        private_location_name: &'a str,
    ) -> Result<ResponseValue<()>, Error<types::Error>> {
        let url = format!(
            "{}/project/{}/private-location/{}",
            self.baseurl,
            encode_path(&project_id.to_string()),
            encode_path(&private_location_name.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .delete(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "delete_private_location",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            204u16 => Ok(ResponseValue::empty(response)),
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///List all private subnets
    ///
    ///Sends a `GET` request to `/project/{project_id}/private-subnet`
    ///
    ///Arguments:
    /// - `project_id`: ID of the project
    /// - `order_column`: Pagination - Order column
    /// - `page_size`: Pagination - Page size
    /// - `start_after`: Pagination - Start after
    pub async fn list_p_ss<'a>(
        &'a self,
        project_id: &'a types::ListPSsProjectId,
        order_column: Option<&'a str>,
        page_size: Option<i64>,
        start_after: Option<&'a str>,
    ) -> Result<ResponseValue<types::ListPSsResponse>, Error<types::Error>> {
        let url = format!(
            "{}/project/{}/private-subnet",
            self.baseurl,
            encode_path(&project_id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new(
                "order_column",
                &order_column,
            ))
            .query(&progenitor_client::QueryParam::new("page_size", &page_size))
            .query(&progenitor_client::QueryParam::new(
                "start_after",
                &start_after,
            ))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "list_p_ss",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///List all SSH public keys
    ///
    ///Sends a `GET` request to `/project/{project_id}/ssh-public-key`
    ///
    ///Arguments:
    /// - `project_id`: ID of the project
    pub async fn get_ssh_public_keys<'a>(
        &'a self,
        project_id: &'a types::GetSshPublicKeysProjectId,
    ) -> Result<ResponseValue<types::GetSshPublicKeysResponse>, Error<types::Error>> {
        let url = format!(
            "{}/project/{}/ssh-public-key",
            self.baseurl,
            encode_path(&project_id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_ssh_public_keys",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Register a SSH public key
    ///
    ///Sends a `POST` request to `/project/{project_id}/ssh-public-key`
    ///
    ///Arguments:
    /// - `project_id`: ID of the project
    /// - `body`
    pub async fn register_ssh_public_key<'a>(
        &'a self,
        project_id: &'a types::RegisterSshPublicKeyProjectId,
        body: &'a types::SshPublicKeyCreate,
    ) -> Result<ResponseValue<types::SshPublicKey>, Error<types::Error>> {
        let url = format!(
            "{}/project/{}/ssh-public-key",
            self.baseurl,
            encode_path(&project_id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "register_ssh_public_key",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Get SSH public key details
    ///
    ///Sends a `GET` request to
    /// `/project/{project_id}/ssh-public-key/{ssh_public_key_reference}`
    ///
    ///Arguments:
    /// - `project_id`: ID of the project
    /// - `ssh_public_key_reference`: SSH public key ID or name
    pub async fn get_ssh_public_key<'a>(
        &'a self,
        project_id: &'a types::GetSshPublicKeyProjectId,
        ssh_public_key_reference: &'a types::Reference,
    ) -> Result<ResponseValue<types::SshPublicKey>, Error<types::Error>> {
        let url = format!(
            "{}/project/{}/ssh-public-key/{}",
            self.baseurl,
            encode_path(&project_id.to_string()),
            encode_path(&ssh_public_key_reference.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_ssh_public_key",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Update SSH public key information
    ///
    ///Sends a `POST` request to
    /// `/project/{project_id}/ssh-public-key/{ssh_public_key_reference}`
    ///
    ///Arguments:
    /// - `project_id`: ID of the project
    /// - `ssh_public_key_reference`: SSH public key ID or name
    /// - `body`
    pub async fn update_ssh_public_key<'a>(
        &'a self,
        project_id: &'a types::UpdateSshPublicKeyProjectId,
        ssh_public_key_reference: &'a types::Reference,
        body: &'a types::SshPublicKeyUpdate,
    ) -> Result<ResponseValue<types::SshPublicKey>, Error<types::Error>> {
        let url = format!(
            "{}/project/{}/ssh-public-key/{}",
            self.baseurl,
            encode_path(&project_id.to_string()),
            encode_path(&ssh_public_key_reference.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "update_ssh_public_key",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Delete an SSH public key
    ///
    ///Sends a `DELETE` request to
    /// `/project/{project_id}/ssh-public-key/{ssh_public_key_reference}`
    ///
    ///Arguments:
    /// - `project_id`: ID of the project
    /// - `ssh_public_key_reference`: SSH public key ID or name
    pub async fn delete_ssh_public_key<'a>(
        &'a self,
        project_id: &'a types::DeleteSshPublicKeyProjectId,
        ssh_public_key_reference: &'a types::Reference,
    ) -> Result<ResponseValue<()>, Error<types::Error>> {
        let url = format!(
            "{}/project/{}/ssh-public-key/{}",
            self.baseurl,
            encode_path(&project_id.to_string()),
            encode_path(&ssh_public_key_reference.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .delete(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "delete_ssh_public_key",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            204u16 => Ok(ResponseValue::empty(response)),
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///List all VMs
    ///
    ///Sends a `GET` request to `/project/{project_id}/vm`
    ///
    ///Arguments:
    /// - `project_id`: ID of the project
    /// - `order_column`: Pagination - Order column
    /// - `page_size`: Pagination - Page size
    /// - `start_after`: Pagination - Start after
    pub async fn list_project_v_ms<'a>(
        &'a self,
        project_id: &'a types::ListProjectVMsProjectId,
        order_column: Option<&'a str>,
        page_size: Option<i64>,
        start_after: Option<&'a str>,
    ) -> Result<ResponseValue<types::ListProjectVMsResponse>, Error<types::Error>> {
        let url = format!(
            "{}/project/{}/vm",
            self.baseurl,
            encode_path(&project_id.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&progenitor_client::QueryParam::new(
                "order_column",
                &order_column,
            ))
            .query(&progenitor_client::QueryParam::new("page_size", &page_size))
            .query(&progenitor_client::QueryParam::new(
                "start_after",
                &start_after,
            ))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "list_project_v_ms",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
}

/// Items consumers will typically use such as the Client.
pub mod prelude {
    #[allow(unused_imports)]
    pub use super::Client;
}
