// @generated
impl serde::Serialize for AggregationProof {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.verification_type != 0 {
            len += 1;
        }
        if !self.message_hash.is_empty() {
            len += 1;
        }
        if !self.proof.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("api.proto.v1.AggregationProof", len)?;
        if self.verification_type != 0 {
            struct_ser.serialize_field("verificationType", &self.verification_type)?;
        }
        if !self.message_hash.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("messageHash", pbjson::private::base64::encode(&self.message_hash).as_str())?;
        }
        if !self.proof.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("proof", pbjson::private::base64::encode(&self.proof).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AggregationProof {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "verification_type",
            "verificationType",
            "message_hash",
            "messageHash",
            "proof",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            VerificationType,
            MessageHash,
            Proof,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "verificationType" | "verification_type" => Ok(GeneratedField::VerificationType),
                            "messageHash" | "message_hash" => Ok(GeneratedField::MessageHash),
                            "proof" => Ok(GeneratedField::Proof),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AggregationProof;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct api.proto.v1.AggregationProof")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AggregationProof, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut verification_type__ = None;
                let mut message_hash__ = None;
                let mut proof__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::VerificationType => {
                            if verification_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("verificationType"));
                            }
                            verification_type__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::MessageHash => {
                            if message_hash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("messageHash"));
                            }
                            message_hash__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Proof => {
                            if proof__.is_some() {
                                return Err(serde::de::Error::duplicate_field("proof"));
                            }
                            proof__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(AggregationProof {
                    verification_type: verification_type__.unwrap_or_default(),
                    message_hash: message_hash__.unwrap_or_default(),
                    proof: proof__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("api.proto.v1.AggregationProof", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ErrorCode {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "ERROR_CODE_UNSPECIFIED",
            Self::NoData => "ERROR_CODE_NO_DATA",
            Self::Internal => "ERROR_CODE_INTERNAL",
            Self::NotAggregator => "ERROR_CODE_NOT_AGGREGATOR",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for ErrorCode {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "ERROR_CODE_UNSPECIFIED",
            "ERROR_CODE_NO_DATA",
            "ERROR_CODE_INTERNAL",
            "ERROR_CODE_NOT_AGGREGATOR",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ErrorCode;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "ERROR_CODE_UNSPECIFIED" => Ok(ErrorCode::Unspecified),
                    "ERROR_CODE_NO_DATA" => Ok(ErrorCode::NoData),
                    "ERROR_CODE_INTERNAL" => Ok(ErrorCode::Internal),
                    "ERROR_CODE_NOT_AGGREGATOR" => Ok(ErrorCode::NotAggregator),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for GetAggregationProofRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.request_hash.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("api.proto.v1.GetAggregationProofRequest", len)?;
        if !self.request_hash.is_empty() {
            struct_ser.serialize_field("requestHash", &self.request_hash)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetAggregationProofRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "request_hash",
            "requestHash",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RequestHash,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "requestHash" | "request_hash" => Ok(GeneratedField::RequestHash),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetAggregationProofRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct api.proto.v1.GetAggregationProofRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetAggregationProofRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut request_hash__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::RequestHash => {
                            if request_hash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requestHash"));
                            }
                            request_hash__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GetAggregationProofRequest {
                    request_hash: request_hash__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("api.proto.v1.GetAggregationProofRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetAggregationProofResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.aggregation_proof.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("api.proto.v1.GetAggregationProofResponse", len)?;
        if let Some(v) = self.aggregation_proof.as_ref() {
            struct_ser.serialize_field("aggregationProof", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetAggregationProofResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "aggregation_proof",
            "aggregationProof",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AggregationProof,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "aggregationProof" | "aggregation_proof" => Ok(GeneratedField::AggregationProof),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetAggregationProofResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct api.proto.v1.GetAggregationProofResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetAggregationProofResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut aggregation_proof__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AggregationProof => {
                            if aggregation_proof__.is_some() {
                                return Err(serde::de::Error::duplicate_field("aggregationProof"));
                            }
                            aggregation_proof__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetAggregationProofResponse {
                    aggregation_proof: aggregation_proof__,
                })
            }
        }
        deserializer.deserialize_struct("api.proto.v1.GetAggregationProofResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetAggregationStatusRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.request_hash.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("api.proto.v1.GetAggregationStatusRequest", len)?;
        if !self.request_hash.is_empty() {
            struct_ser.serialize_field("requestHash", &self.request_hash)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetAggregationStatusRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "request_hash",
            "requestHash",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RequestHash,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "requestHash" | "request_hash" => Ok(GeneratedField::RequestHash),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetAggregationStatusRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct api.proto.v1.GetAggregationStatusRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetAggregationStatusRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut request_hash__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::RequestHash => {
                            if request_hash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requestHash"));
                            }
                            request_hash__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GetAggregationStatusRequest {
                    request_hash: request_hash__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("api.proto.v1.GetAggregationStatusRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetAggregationStatusResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.current_voting_power.is_empty() {
            len += 1;
        }
        if !self.signer_operators.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("api.proto.v1.GetAggregationStatusResponse", len)?;
        if !self.current_voting_power.is_empty() {
            struct_ser.serialize_field("currentVotingPower", &self.current_voting_power)?;
        }
        if !self.signer_operators.is_empty() {
            struct_ser.serialize_field("signerOperators", &self.signer_operators)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetAggregationStatusResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "current_voting_power",
            "currentVotingPower",
            "signer_operators",
            "signerOperators",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CurrentVotingPower,
            SignerOperators,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "currentVotingPower" | "current_voting_power" => Ok(GeneratedField::CurrentVotingPower),
                            "signerOperators" | "signer_operators" => Ok(GeneratedField::SignerOperators),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetAggregationStatusResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct api.proto.v1.GetAggregationStatusResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetAggregationStatusResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut current_voting_power__ = None;
                let mut signer_operators__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::CurrentVotingPower => {
                            if current_voting_power__.is_some() {
                                return Err(serde::de::Error::duplicate_field("currentVotingPower"));
                            }
                            current_voting_power__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SignerOperators => {
                            if signer_operators__.is_some() {
                                return Err(serde::de::Error::duplicate_field("signerOperators"));
                            }
                            signer_operators__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GetAggregationStatusResponse {
                    current_voting_power: current_voting_power__.unwrap_or_default(),
                    signer_operators: signer_operators__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("api.proto.v1.GetAggregationStatusResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetCurrentEpochRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("api.proto.v1.GetCurrentEpochRequest", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetCurrentEpochRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetCurrentEpochRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct api.proto.v1.GetCurrentEpochRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetCurrentEpochRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(GetCurrentEpochRequest {
                })
            }
        }
        deserializer.deserialize_struct("api.proto.v1.GetCurrentEpochRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetCurrentEpochResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.epoch != 0 {
            len += 1;
        }
        if self.start_time.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("api.proto.v1.GetCurrentEpochResponse", len)?;
        if self.epoch != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("epoch", ToString::to_string(&self.epoch).as_str())?;
        }
        if let Some(v) = self.start_time.as_ref() {
            struct_ser.serialize_field("startTime", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetCurrentEpochResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "epoch",
            "start_time",
            "startTime",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Epoch,
            StartTime,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "epoch" => Ok(GeneratedField::Epoch),
                            "startTime" | "start_time" => Ok(GeneratedField::StartTime),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetCurrentEpochResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct api.proto.v1.GetCurrentEpochResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetCurrentEpochResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut epoch__ = None;
                let mut start_time__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Epoch => {
                            if epoch__.is_some() {
                                return Err(serde::de::Error::duplicate_field("epoch"));
                            }
                            epoch__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::StartTime => {
                            if start_time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("startTime"));
                            }
                            start_time__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetCurrentEpochResponse {
                    epoch: epoch__.unwrap_or_default(),
                    start_time: start_time__,
                })
            }
        }
        deserializer.deserialize_struct("api.proto.v1.GetCurrentEpochResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetSignatureRequestRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.request_hash.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("api.proto.v1.GetSignatureRequestRequest", len)?;
        if !self.request_hash.is_empty() {
            struct_ser.serialize_field("requestHash", &self.request_hash)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetSignatureRequestRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "request_hash",
            "requestHash",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RequestHash,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "requestHash" | "request_hash" => Ok(GeneratedField::RequestHash),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetSignatureRequestRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct api.proto.v1.GetSignatureRequestRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetSignatureRequestRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut request_hash__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::RequestHash => {
                            if request_hash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requestHash"));
                            }
                            request_hash__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GetSignatureRequestRequest {
                    request_hash: request_hash__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("api.proto.v1.GetSignatureRequestRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetSignatureRequestResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.key_tag != 0 {
            len += 1;
        }
        if !self.message.is_empty() {
            len += 1;
        }
        if self.required_epoch != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("api.proto.v1.GetSignatureRequestResponse", len)?;
        if self.key_tag != 0 {
            struct_ser.serialize_field("keyTag", &self.key_tag)?;
        }
        if !self.message.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("message", pbjson::private::base64::encode(&self.message).as_str())?;
        }
        if self.required_epoch != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("requiredEpoch", ToString::to_string(&self.required_epoch).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetSignatureRequestResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "key_tag",
            "keyTag",
            "message",
            "required_epoch",
            "requiredEpoch",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            KeyTag,
            Message,
            RequiredEpoch,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "keyTag" | "key_tag" => Ok(GeneratedField::KeyTag),
                            "message" => Ok(GeneratedField::Message),
                            "requiredEpoch" | "required_epoch" => Ok(GeneratedField::RequiredEpoch),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetSignatureRequestResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct api.proto.v1.GetSignatureRequestResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetSignatureRequestResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut key_tag__ = None;
                let mut message__ = None;
                let mut required_epoch__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::KeyTag => {
                            if key_tag__.is_some() {
                                return Err(serde::de::Error::duplicate_field("keyTag"));
                            }
                            key_tag__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Message => {
                            if message__.is_some() {
                                return Err(serde::de::Error::duplicate_field("message"));
                            }
                            message__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::RequiredEpoch => {
                            if required_epoch__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requiredEpoch"));
                            }
                            required_epoch__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(GetSignatureRequestResponse {
                    key_tag: key_tag__.unwrap_or_default(),
                    message: message__.unwrap_or_default(),
                    required_epoch: required_epoch__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("api.proto.v1.GetSignatureRequestResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetSignaturesRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.request_hash.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("api.proto.v1.GetSignaturesRequest", len)?;
        if !self.request_hash.is_empty() {
            struct_ser.serialize_field("requestHash", &self.request_hash)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetSignaturesRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "request_hash",
            "requestHash",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RequestHash,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "requestHash" | "request_hash" => Ok(GeneratedField::RequestHash),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetSignaturesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct api.proto.v1.GetSignaturesRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetSignaturesRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut request_hash__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::RequestHash => {
                            if request_hash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requestHash"));
                            }
                            request_hash__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GetSignaturesRequest {
                    request_hash: request_hash__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("api.proto.v1.GetSignaturesRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetSignaturesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.signatures.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("api.proto.v1.GetSignaturesResponse", len)?;
        if !self.signatures.is_empty() {
            struct_ser.serialize_field("signatures", &self.signatures)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetSignaturesResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "signatures",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Signatures,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "signatures" => Ok(GeneratedField::Signatures),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetSignaturesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct api.proto.v1.GetSignaturesResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetSignaturesResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut signatures__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Signatures => {
                            if signatures__.is_some() {
                                return Err(serde::de::Error::duplicate_field("signatures"));
                            }
                            signatures__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GetSignaturesResponse {
                    signatures: signatures__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("api.proto.v1.GetSignaturesResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetSuggestedEpochRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("api.proto.v1.GetSuggestedEpochRequest", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetSuggestedEpochRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetSuggestedEpochRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct api.proto.v1.GetSuggestedEpochRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetSuggestedEpochRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(GetSuggestedEpochRequest {
                })
            }
        }
        deserializer.deserialize_struct("api.proto.v1.GetSuggestedEpochRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetSuggestedEpochResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.epoch != 0 {
            len += 1;
        }
        if self.start_time.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("api.proto.v1.GetSuggestedEpochResponse", len)?;
        if self.epoch != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("epoch", ToString::to_string(&self.epoch).as_str())?;
        }
        if let Some(v) = self.start_time.as_ref() {
            struct_ser.serialize_field("startTime", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetSuggestedEpochResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "epoch",
            "start_time",
            "startTime",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Epoch,
            StartTime,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "epoch" => Ok(GeneratedField::Epoch),
                            "startTime" | "start_time" => Ok(GeneratedField::StartTime),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetSuggestedEpochResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct api.proto.v1.GetSuggestedEpochResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetSuggestedEpochResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut epoch__ = None;
                let mut start_time__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Epoch => {
                            if epoch__.is_some() {
                                return Err(serde::de::Error::duplicate_field("epoch"));
                            }
                            epoch__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::StartTime => {
                            if start_time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("startTime"));
                            }
                            start_time__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetSuggestedEpochResponse {
                    epoch: epoch__.unwrap_or_default(),
                    start_time: start_time__,
                })
            }
        }
        deserializer.deserialize_struct("api.proto.v1.GetSuggestedEpochResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetValidatorByAddressRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.epoch.is_some() {
            len += 1;
        }
        if !self.address.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("api.proto.v1.GetValidatorByAddressRequest", len)?;
        if let Some(v) = self.epoch.as_ref() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("epoch", ToString::to_string(&v).as_str())?;
        }
        if !self.address.is_empty() {
            struct_ser.serialize_field("address", &self.address)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetValidatorByAddressRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "epoch",
            "address",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Epoch,
            Address,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "epoch" => Ok(GeneratedField::Epoch),
                            "address" => Ok(GeneratedField::Address),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetValidatorByAddressRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct api.proto.v1.GetValidatorByAddressRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetValidatorByAddressRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut epoch__ = None;
                let mut address__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Epoch => {
                            if epoch__.is_some() {
                                return Err(serde::de::Error::duplicate_field("epoch"));
                            }
                            epoch__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::Address => {
                            if address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("address"));
                            }
                            address__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GetValidatorByAddressRequest {
                    epoch: epoch__,
                    address: address__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("api.proto.v1.GetValidatorByAddressRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetValidatorByAddressResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.validator.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("api.proto.v1.GetValidatorByAddressResponse", len)?;
        if let Some(v) = self.validator.as_ref() {
            struct_ser.serialize_field("validator", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetValidatorByAddressResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "validator",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Validator,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "validator" => Ok(GeneratedField::Validator),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetValidatorByAddressResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct api.proto.v1.GetValidatorByAddressResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetValidatorByAddressResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut validator__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Validator => {
                            if validator__.is_some() {
                                return Err(serde::de::Error::duplicate_field("validator"));
                            }
                            validator__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetValidatorByAddressResponse {
                    validator: validator__,
                })
            }
        }
        deserializer.deserialize_struct("api.proto.v1.GetValidatorByAddressResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetValidatorSetHeaderRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.epoch.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("api.proto.v1.GetValidatorSetHeaderRequest", len)?;
        if let Some(v) = self.epoch.as_ref() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("epoch", ToString::to_string(&v).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetValidatorSetHeaderRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "epoch",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Epoch,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "epoch" => Ok(GeneratedField::Epoch),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetValidatorSetHeaderRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct api.proto.v1.GetValidatorSetHeaderRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetValidatorSetHeaderRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut epoch__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Epoch => {
                            if epoch__.is_some() {
                                return Err(serde::de::Error::duplicate_field("epoch"));
                            }
                            epoch__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                    }
                }
                Ok(GetValidatorSetHeaderRequest {
                    epoch: epoch__,
                })
            }
        }
        deserializer.deserialize_struct("api.proto.v1.GetValidatorSetHeaderRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetValidatorSetHeaderResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.version != 0 {
            len += 1;
        }
        if self.required_key_tag != 0 {
            len += 1;
        }
        if self.epoch != 0 {
            len += 1;
        }
        if self.capture_timestamp.is_some() {
            len += 1;
        }
        if !self.quorum_threshold.is_empty() {
            len += 1;
        }
        if !self.total_voting_power.is_empty() {
            len += 1;
        }
        if !self.validators_ssz_mroot.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("api.proto.v1.GetValidatorSetHeaderResponse", len)?;
        if self.version != 0 {
            struct_ser.serialize_field("version", &self.version)?;
        }
        if self.required_key_tag != 0 {
            struct_ser.serialize_field("requiredKeyTag", &self.required_key_tag)?;
        }
        if self.epoch != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("epoch", ToString::to_string(&self.epoch).as_str())?;
        }
        if let Some(v) = self.capture_timestamp.as_ref() {
            struct_ser.serialize_field("captureTimestamp", v)?;
        }
        if !self.quorum_threshold.is_empty() {
            struct_ser.serialize_field("quorumThreshold", &self.quorum_threshold)?;
        }
        if !self.total_voting_power.is_empty() {
            struct_ser.serialize_field("totalVotingPower", &self.total_voting_power)?;
        }
        if !self.validators_ssz_mroot.is_empty() {
            struct_ser.serialize_field("validatorsSszMroot", &self.validators_ssz_mroot)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetValidatorSetHeaderResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "version",
            "required_key_tag",
            "requiredKeyTag",
            "epoch",
            "capture_timestamp",
            "captureTimestamp",
            "quorum_threshold",
            "quorumThreshold",
            "total_voting_power",
            "totalVotingPower",
            "validators_ssz_mroot",
            "validatorsSszMroot",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Version,
            RequiredKeyTag,
            Epoch,
            CaptureTimestamp,
            QuorumThreshold,
            TotalVotingPower,
            ValidatorsSszMroot,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "version" => Ok(GeneratedField::Version),
                            "requiredKeyTag" | "required_key_tag" => Ok(GeneratedField::RequiredKeyTag),
                            "epoch" => Ok(GeneratedField::Epoch),
                            "captureTimestamp" | "capture_timestamp" => Ok(GeneratedField::CaptureTimestamp),
                            "quorumThreshold" | "quorum_threshold" => Ok(GeneratedField::QuorumThreshold),
                            "totalVotingPower" | "total_voting_power" => Ok(GeneratedField::TotalVotingPower),
                            "validatorsSszMroot" | "validators_ssz_mroot" => Ok(GeneratedField::ValidatorsSszMroot),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetValidatorSetHeaderResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct api.proto.v1.GetValidatorSetHeaderResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetValidatorSetHeaderResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut version__ = None;
                let mut required_key_tag__ = None;
                let mut epoch__ = None;
                let mut capture_timestamp__ = None;
                let mut quorum_threshold__ = None;
                let mut total_voting_power__ = None;
                let mut validators_ssz_mroot__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Version => {
                            if version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            version__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::RequiredKeyTag => {
                            if required_key_tag__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requiredKeyTag"));
                            }
                            required_key_tag__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Epoch => {
                            if epoch__.is_some() {
                                return Err(serde::de::Error::duplicate_field("epoch"));
                            }
                            epoch__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::CaptureTimestamp => {
                            if capture_timestamp__.is_some() {
                                return Err(serde::de::Error::duplicate_field("captureTimestamp"));
                            }
                            capture_timestamp__ = map_.next_value()?;
                        }
                        GeneratedField::QuorumThreshold => {
                            if quorum_threshold__.is_some() {
                                return Err(serde::de::Error::duplicate_field("quorumThreshold"));
                            }
                            quorum_threshold__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TotalVotingPower => {
                            if total_voting_power__.is_some() {
                                return Err(serde::de::Error::duplicate_field("totalVotingPower"));
                            }
                            total_voting_power__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ValidatorsSszMroot => {
                            if validators_ssz_mroot__.is_some() {
                                return Err(serde::de::Error::duplicate_field("validatorsSszMroot"));
                            }
                            validators_ssz_mroot__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GetValidatorSetHeaderResponse {
                    version: version__.unwrap_or_default(),
                    required_key_tag: required_key_tag__.unwrap_or_default(),
                    epoch: epoch__.unwrap_or_default(),
                    capture_timestamp: capture_timestamp__,
                    quorum_threshold: quorum_threshold__.unwrap_or_default(),
                    total_voting_power: total_voting_power__.unwrap_or_default(),
                    validators_ssz_mroot: validators_ssz_mroot__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("api.proto.v1.GetValidatorSetHeaderResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetValidatorSetRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.epoch.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("api.proto.v1.GetValidatorSetRequest", len)?;
        if let Some(v) = self.epoch.as_ref() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("epoch", ToString::to_string(&v).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetValidatorSetRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "epoch",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Epoch,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "epoch" => Ok(GeneratedField::Epoch),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetValidatorSetRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct api.proto.v1.GetValidatorSetRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetValidatorSetRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut epoch__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Epoch => {
                            if epoch__.is_some() {
                                return Err(serde::de::Error::duplicate_field("epoch"));
                            }
                            epoch__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                    }
                }
                Ok(GetValidatorSetRequest {
                    epoch: epoch__,
                })
            }
        }
        deserializer.deserialize_struct("api.proto.v1.GetValidatorSetRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetValidatorSetResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.version != 0 {
            len += 1;
        }
        if self.required_key_tag != 0 {
            len += 1;
        }
        if self.epoch != 0 {
            len += 1;
        }
        if self.capture_timestamp.is_some() {
            len += 1;
        }
        if !self.quorum_threshold.is_empty() {
            len += 1;
        }
        if self.status != 0 {
            len += 1;
        }
        if !self.validators.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("api.proto.v1.GetValidatorSetResponse", len)?;
        if self.version != 0 {
            struct_ser.serialize_field("version", &self.version)?;
        }
        if self.required_key_tag != 0 {
            struct_ser.serialize_field("requiredKeyTag", &self.required_key_tag)?;
        }
        if self.epoch != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("epoch", ToString::to_string(&self.epoch).as_str())?;
        }
        if let Some(v) = self.capture_timestamp.as_ref() {
            struct_ser.serialize_field("captureTimestamp", v)?;
        }
        if !self.quorum_threshold.is_empty() {
            struct_ser.serialize_field("quorumThreshold", &self.quorum_threshold)?;
        }
        if self.status != 0 {
            let v = ValidatorSetStatus::try_from(self.status)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.status)))?;
            struct_ser.serialize_field("status", &v)?;
        }
        if !self.validators.is_empty() {
            struct_ser.serialize_field("validators", &self.validators)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetValidatorSetResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "version",
            "required_key_tag",
            "requiredKeyTag",
            "epoch",
            "capture_timestamp",
            "captureTimestamp",
            "quorum_threshold",
            "quorumThreshold",
            "status",
            "validators",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Version,
            RequiredKeyTag,
            Epoch,
            CaptureTimestamp,
            QuorumThreshold,
            Status,
            Validators,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "version" => Ok(GeneratedField::Version),
                            "requiredKeyTag" | "required_key_tag" => Ok(GeneratedField::RequiredKeyTag),
                            "epoch" => Ok(GeneratedField::Epoch),
                            "captureTimestamp" | "capture_timestamp" => Ok(GeneratedField::CaptureTimestamp),
                            "quorumThreshold" | "quorum_threshold" => Ok(GeneratedField::QuorumThreshold),
                            "status" => Ok(GeneratedField::Status),
                            "validators" => Ok(GeneratedField::Validators),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetValidatorSetResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct api.proto.v1.GetValidatorSetResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetValidatorSetResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut version__ = None;
                let mut required_key_tag__ = None;
                let mut epoch__ = None;
                let mut capture_timestamp__ = None;
                let mut quorum_threshold__ = None;
                let mut status__ = None;
                let mut validators__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Version => {
                            if version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            version__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::RequiredKeyTag => {
                            if required_key_tag__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requiredKeyTag"));
                            }
                            required_key_tag__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Epoch => {
                            if epoch__.is_some() {
                                return Err(serde::de::Error::duplicate_field("epoch"));
                            }
                            epoch__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::CaptureTimestamp => {
                            if capture_timestamp__.is_some() {
                                return Err(serde::de::Error::duplicate_field("captureTimestamp"));
                            }
                            capture_timestamp__ = map_.next_value()?;
                        }
                        GeneratedField::QuorumThreshold => {
                            if quorum_threshold__.is_some() {
                                return Err(serde::de::Error::duplicate_field("quorumThreshold"));
                            }
                            quorum_threshold__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = Some(map_.next_value::<ValidatorSetStatus>()? as i32);
                        }
                        GeneratedField::Validators => {
                            if validators__.is_some() {
                                return Err(serde::de::Error::duplicate_field("validators"));
                            }
                            validators__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GetValidatorSetResponse {
                    version: version__.unwrap_or_default(),
                    required_key_tag: required_key_tag__.unwrap_or_default(),
                    epoch: epoch__.unwrap_or_default(),
                    capture_timestamp: capture_timestamp__,
                    quorum_threshold: quorum_threshold__.unwrap_or_default(),
                    status: status__.unwrap_or_default(),
                    validators: validators__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("api.proto.v1.GetValidatorSetResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Key {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.tag != 0 {
            len += 1;
        }
        if !self.payload.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("api.proto.v1.Key", len)?;
        if self.tag != 0 {
            struct_ser.serialize_field("tag", &self.tag)?;
        }
        if !self.payload.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("payload", pbjson::private::base64::encode(&self.payload).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Key {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "tag",
            "payload",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Tag,
            Payload,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "tag" => Ok(GeneratedField::Tag),
                            "payload" => Ok(GeneratedField::Payload),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Key;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct api.proto.v1.Key")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Key, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut tag__ = None;
                let mut payload__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Tag => {
                            if tag__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tag"));
                            }
                            tag__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Payload => {
                            if payload__.is_some() {
                                return Err(serde::de::Error::duplicate_field("payload"));
                            }
                            payload__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(Key {
                    tag: tag__.unwrap_or_default(),
                    payload: payload__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("api.proto.v1.Key", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SignMessageRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.key_tag != 0 {
            len += 1;
        }
        if !self.message.is_empty() {
            len += 1;
        }
        if self.required_epoch.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("api.proto.v1.SignMessageRequest", len)?;
        if self.key_tag != 0 {
            struct_ser.serialize_field("keyTag", &self.key_tag)?;
        }
        if !self.message.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("message", pbjson::private::base64::encode(&self.message).as_str())?;
        }
        if let Some(v) = self.required_epoch.as_ref() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("requiredEpoch", ToString::to_string(&v).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SignMessageRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "key_tag",
            "keyTag",
            "message",
            "required_epoch",
            "requiredEpoch",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            KeyTag,
            Message,
            RequiredEpoch,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "keyTag" | "key_tag" => Ok(GeneratedField::KeyTag),
                            "message" => Ok(GeneratedField::Message),
                            "requiredEpoch" | "required_epoch" => Ok(GeneratedField::RequiredEpoch),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SignMessageRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct api.proto.v1.SignMessageRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SignMessageRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut key_tag__ = None;
                let mut message__ = None;
                let mut required_epoch__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::KeyTag => {
                            if key_tag__.is_some() {
                                return Err(serde::de::Error::duplicate_field("keyTag"));
                            }
                            key_tag__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Message => {
                            if message__.is_some() {
                                return Err(serde::de::Error::duplicate_field("message"));
                            }
                            message__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::RequiredEpoch => {
                            if required_epoch__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requiredEpoch"));
                            }
                            required_epoch__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                    }
                }
                Ok(SignMessageRequest {
                    key_tag: key_tag__.unwrap_or_default(),
                    message: message__.unwrap_or_default(),
                    required_epoch: required_epoch__,
                })
            }
        }
        deserializer.deserialize_struct("api.proto.v1.SignMessageRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SignMessageResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.request_hash.is_empty() {
            len += 1;
        }
        if self.epoch != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("api.proto.v1.SignMessageResponse", len)?;
        if !self.request_hash.is_empty() {
            struct_ser.serialize_field("requestHash", &self.request_hash)?;
        }
        if self.epoch != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("epoch", ToString::to_string(&self.epoch).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SignMessageResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "request_hash",
            "requestHash",
            "epoch",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RequestHash,
            Epoch,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "requestHash" | "request_hash" => Ok(GeneratedField::RequestHash),
                            "epoch" => Ok(GeneratedField::Epoch),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SignMessageResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct api.proto.v1.SignMessageResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SignMessageResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut request_hash__ = None;
                let mut epoch__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::RequestHash => {
                            if request_hash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requestHash"));
                            }
                            request_hash__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Epoch => {
                            if epoch__.is_some() {
                                return Err(serde::de::Error::duplicate_field("epoch"));
                            }
                            epoch__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(SignMessageResponse {
                    request_hash: request_hash__.unwrap_or_default(),
                    epoch: epoch__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("api.proto.v1.SignMessageResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SignMessageWaitRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.key_tag != 0 {
            len += 1;
        }
        if !self.message.is_empty() {
            len += 1;
        }
        if self.required_epoch.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("api.proto.v1.SignMessageWaitRequest", len)?;
        if self.key_tag != 0 {
            struct_ser.serialize_field("keyTag", &self.key_tag)?;
        }
        if !self.message.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("message", pbjson::private::base64::encode(&self.message).as_str())?;
        }
        if let Some(v) = self.required_epoch.as_ref() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("requiredEpoch", ToString::to_string(&v).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SignMessageWaitRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "key_tag",
            "keyTag",
            "message",
            "required_epoch",
            "requiredEpoch",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            KeyTag,
            Message,
            RequiredEpoch,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "keyTag" | "key_tag" => Ok(GeneratedField::KeyTag),
                            "message" => Ok(GeneratedField::Message),
                            "requiredEpoch" | "required_epoch" => Ok(GeneratedField::RequiredEpoch),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SignMessageWaitRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct api.proto.v1.SignMessageWaitRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SignMessageWaitRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut key_tag__ = None;
                let mut message__ = None;
                let mut required_epoch__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::KeyTag => {
                            if key_tag__.is_some() {
                                return Err(serde::de::Error::duplicate_field("keyTag"));
                            }
                            key_tag__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Message => {
                            if message__.is_some() {
                                return Err(serde::de::Error::duplicate_field("message"));
                            }
                            message__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::RequiredEpoch => {
                            if required_epoch__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requiredEpoch"));
                            }
                            required_epoch__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                    }
                }
                Ok(SignMessageWaitRequest {
                    key_tag: key_tag__.unwrap_or_default(),
                    message: message__.unwrap_or_default(),
                    required_epoch: required_epoch__,
                })
            }
        }
        deserializer.deserialize_struct("api.proto.v1.SignMessageWaitRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SignMessageWaitResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.status != 0 {
            len += 1;
        }
        if !self.request_hash.is_empty() {
            len += 1;
        }
        if self.epoch != 0 {
            len += 1;
        }
        if self.aggregation_proof.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("api.proto.v1.SignMessageWaitResponse", len)?;
        if self.status != 0 {
            let v = SigningStatus::try_from(self.status)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.status)))?;
            struct_ser.serialize_field("status", &v)?;
        }
        if !self.request_hash.is_empty() {
            struct_ser.serialize_field("requestHash", &self.request_hash)?;
        }
        if self.epoch != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("epoch", ToString::to_string(&self.epoch).as_str())?;
        }
        if let Some(v) = self.aggregation_proof.as_ref() {
            struct_ser.serialize_field("aggregationProof", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SignMessageWaitResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "status",
            "request_hash",
            "requestHash",
            "epoch",
            "aggregation_proof",
            "aggregationProof",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Status,
            RequestHash,
            Epoch,
            AggregationProof,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "status" => Ok(GeneratedField::Status),
                            "requestHash" | "request_hash" => Ok(GeneratedField::RequestHash),
                            "epoch" => Ok(GeneratedField::Epoch),
                            "aggregationProof" | "aggregation_proof" => Ok(GeneratedField::AggregationProof),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SignMessageWaitResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct api.proto.v1.SignMessageWaitResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SignMessageWaitResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut status__ = None;
                let mut request_hash__ = None;
                let mut epoch__ = None;
                let mut aggregation_proof__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = Some(map_.next_value::<SigningStatus>()? as i32);
                        }
                        GeneratedField::RequestHash => {
                            if request_hash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requestHash"));
                            }
                            request_hash__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Epoch => {
                            if epoch__.is_some() {
                                return Err(serde::de::Error::duplicate_field("epoch"));
                            }
                            epoch__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::AggregationProof => {
                            if aggregation_proof__.is_some() {
                                return Err(serde::de::Error::duplicate_field("aggregationProof"));
                            }
                            aggregation_proof__ = map_.next_value()?;
                        }
                    }
                }
                Ok(SignMessageWaitResponse {
                    status: status__.unwrap_or_default(),
                    request_hash: request_hash__.unwrap_or_default(),
                    epoch: epoch__.unwrap_or_default(),
                    aggregation_proof: aggregation_proof__,
                })
            }
        }
        deserializer.deserialize_struct("api.proto.v1.SignMessageWaitResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Signature {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.signature.is_empty() {
            len += 1;
        }
        if !self.message_hash.is_empty() {
            len += 1;
        }
        if !self.public_key.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("api.proto.v1.Signature", len)?;
        if !self.signature.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("signature", pbjson::private::base64::encode(&self.signature).as_str())?;
        }
        if !self.message_hash.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("messageHash", pbjson::private::base64::encode(&self.message_hash).as_str())?;
        }
        if !self.public_key.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("publicKey", pbjson::private::base64::encode(&self.public_key).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Signature {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "signature",
            "message_hash",
            "messageHash",
            "public_key",
            "publicKey",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Signature,
            MessageHash,
            PublicKey,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "signature" => Ok(GeneratedField::Signature),
                            "messageHash" | "message_hash" => Ok(GeneratedField::MessageHash),
                            "publicKey" | "public_key" => Ok(GeneratedField::PublicKey),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Signature;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct api.proto.v1.Signature")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Signature, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut signature__ = None;
                let mut message_hash__ = None;
                let mut public_key__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Signature => {
                            if signature__.is_some() {
                                return Err(serde::de::Error::duplicate_field("signature"));
                            }
                            signature__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::MessageHash => {
                            if message_hash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("messageHash"));
                            }
                            message_hash__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::PublicKey => {
                            if public_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("publicKey"));
                            }
                            public_key__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(Signature {
                    signature: signature__.unwrap_or_default(),
                    message_hash: message_hash__.unwrap_or_default(),
                    public_key: public_key__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("api.proto.v1.Signature", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SigningStatus {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "SIGNING_STATUS_UNSPECIFIED",
            Self::Pending => "SIGNING_STATUS_PENDING",
            Self::Completed => "SIGNING_STATUS_COMPLETED",
            Self::Failed => "SIGNING_STATUS_FAILED",
            Self::Timeout => "SIGNING_STATUS_TIMEOUT",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for SigningStatus {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "SIGNING_STATUS_UNSPECIFIED",
            "SIGNING_STATUS_PENDING",
            "SIGNING_STATUS_COMPLETED",
            "SIGNING_STATUS_FAILED",
            "SIGNING_STATUS_TIMEOUT",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SigningStatus;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "SIGNING_STATUS_UNSPECIFIED" => Ok(SigningStatus::Unspecified),
                    "SIGNING_STATUS_PENDING" => Ok(SigningStatus::Pending),
                    "SIGNING_STATUS_COMPLETED" => Ok(SigningStatus::Completed),
                    "SIGNING_STATUS_FAILED" => Ok(SigningStatus::Failed),
                    "SIGNING_STATUS_TIMEOUT" => Ok(SigningStatus::Timeout),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for Validator {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.operator.is_empty() {
            len += 1;
        }
        if !self.voting_power.is_empty() {
            len += 1;
        }
        if self.is_active {
            len += 1;
        }
        if !self.keys.is_empty() {
            len += 1;
        }
        if !self.vaults.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("api.proto.v1.Validator", len)?;
        if !self.operator.is_empty() {
            struct_ser.serialize_field("operator", &self.operator)?;
        }
        if !self.voting_power.is_empty() {
            struct_ser.serialize_field("votingPower", &self.voting_power)?;
        }
        if self.is_active {
            struct_ser.serialize_field("isActive", &self.is_active)?;
        }
        if !self.keys.is_empty() {
            struct_ser.serialize_field("keys", &self.keys)?;
        }
        if !self.vaults.is_empty() {
            struct_ser.serialize_field("vaults", &self.vaults)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Validator {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "operator",
            "voting_power",
            "votingPower",
            "is_active",
            "isActive",
            "keys",
            "vaults",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Operator,
            VotingPower,
            IsActive,
            Keys,
            Vaults,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "operator" => Ok(GeneratedField::Operator),
                            "votingPower" | "voting_power" => Ok(GeneratedField::VotingPower),
                            "isActive" | "is_active" => Ok(GeneratedField::IsActive),
                            "keys" => Ok(GeneratedField::Keys),
                            "vaults" => Ok(GeneratedField::Vaults),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Validator;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct api.proto.v1.Validator")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Validator, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut operator__ = None;
                let mut voting_power__ = None;
                let mut is_active__ = None;
                let mut keys__ = None;
                let mut vaults__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Operator => {
                            if operator__.is_some() {
                                return Err(serde::de::Error::duplicate_field("operator"));
                            }
                            operator__ = Some(map_.next_value()?);
                        }
                        GeneratedField::VotingPower => {
                            if voting_power__.is_some() {
                                return Err(serde::de::Error::duplicate_field("votingPower"));
                            }
                            voting_power__ = Some(map_.next_value()?);
                        }
                        GeneratedField::IsActive => {
                            if is_active__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isActive"));
                            }
                            is_active__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Keys => {
                            if keys__.is_some() {
                                return Err(serde::de::Error::duplicate_field("keys"));
                            }
                            keys__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Vaults => {
                            if vaults__.is_some() {
                                return Err(serde::de::Error::duplicate_field("vaults"));
                            }
                            vaults__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Validator {
                    operator: operator__.unwrap_or_default(),
                    voting_power: voting_power__.unwrap_or_default(),
                    is_active: is_active__.unwrap_or_default(),
                    keys: keys__.unwrap_or_default(),
                    vaults: vaults__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("api.proto.v1.Validator", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ValidatorSetStatus {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "VALIDATOR_SET_STATUS_UNSPECIFIED",
            Self::Derived => "VALIDATOR_SET_STATUS_DERIVED",
            Self::Aggregated => "VALIDATOR_SET_STATUS_AGGREGATED",
            Self::Committed => "VALIDATOR_SET_STATUS_COMMITTED",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for ValidatorSetStatus {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "VALIDATOR_SET_STATUS_UNSPECIFIED",
            "VALIDATOR_SET_STATUS_DERIVED",
            "VALIDATOR_SET_STATUS_AGGREGATED",
            "VALIDATOR_SET_STATUS_COMMITTED",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ValidatorSetStatus;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "VALIDATOR_SET_STATUS_UNSPECIFIED" => Ok(ValidatorSetStatus::Unspecified),
                    "VALIDATOR_SET_STATUS_DERIVED" => Ok(ValidatorSetStatus::Derived),
                    "VALIDATOR_SET_STATUS_AGGREGATED" => Ok(ValidatorSetStatus::Aggregated),
                    "VALIDATOR_SET_STATUS_COMMITTED" => Ok(ValidatorSetStatus::Committed),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for ValidatorVault {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.chain_id != 0 {
            len += 1;
        }
        if !self.vault.is_empty() {
            len += 1;
        }
        if !self.voting_power.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("api.proto.v1.ValidatorVault", len)?;
        if self.chain_id != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("chainId", ToString::to_string(&self.chain_id).as_str())?;
        }
        if !self.vault.is_empty() {
            struct_ser.serialize_field("vault", &self.vault)?;
        }
        if !self.voting_power.is_empty() {
            struct_ser.serialize_field("votingPower", &self.voting_power)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ValidatorVault {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "chain_id",
            "chainId",
            "vault",
            "voting_power",
            "votingPower",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ChainId,
            Vault,
            VotingPower,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "chainId" | "chain_id" => Ok(GeneratedField::ChainId),
                            "vault" => Ok(GeneratedField::Vault),
                            "votingPower" | "voting_power" => Ok(GeneratedField::VotingPower),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ValidatorVault;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct api.proto.v1.ValidatorVault")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ValidatorVault, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut chain_id__ = None;
                let mut vault__ = None;
                let mut voting_power__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ChainId => {
                            if chain_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("chainId"));
                            }
                            chain_id__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Vault => {
                            if vault__.is_some() {
                                return Err(serde::de::Error::duplicate_field("vault"));
                            }
                            vault__ = Some(map_.next_value()?);
                        }
                        GeneratedField::VotingPower => {
                            if voting_power__.is_some() {
                                return Err(serde::de::Error::duplicate_field("votingPower"));
                            }
                            voting_power__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ValidatorVault {
                    chain_id: chain_id__.unwrap_or_default(),
                    vault: vault__.unwrap_or_default(),
                    voting_power: voting_power__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("api.proto.v1.ValidatorVault", FIELDS, GeneratedVisitor)
    }
}
