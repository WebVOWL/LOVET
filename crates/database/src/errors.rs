use std::{
    panic::Location,
    sync::{Arc, PoisonError},
};

use crate::serializers::{Edge, Triple};
use oxrdf::{BlankNodeIdParseError, IriParseError};
use vowlr_util::prelude::{ErrorRecord, ErrorSeverity, ErrorType, VOWLRError, get_timestamp};

#[derive(Debug)]
pub enum SerializationErrorKind {
    /// An error raised when the object of a triple is required but missing.
    MissingObject(Arc<Triple>, String),
    /// An error raised when the subject of a triple is required but missing.
    MissingSubject(Arc<Triple>, String),
    /// An error raised when the range of an edge is required but missing.
    MissingRange(Arc<Edge>, String),
    /// An error raised when the domain of an edge is required but missing.
    MissingDomain(Arc<Edge>, String),
    /// An error raised when the label of a term is required but missing.
    MissingLabel(String),
    /// An error raised when the property term of an edge is required but missing.
    MissingProperty(String),
    /// An error raised when the characteristics of a node term is required but missing.
    MisisngCharacteristic(String),
    /// An error raised when the individuals count for a node term is required but missing.
    MissingIndividualsCount(String),
    /// An error raised when the serializer encountered an unrecoverable problem.
    ///
    /// Includes the problematic triple.
    SerializationFailedTriple(Arc<Triple>, String),
    /// An error raised when the serializer encountered an unrecoverable problem.
    SerializationFailed(String),
    /// An error raised during Iri or IriRef validation.
    IriParseError(String, Box<IriParseError>),
    /// An error raised during BlankNode IDs validation.
    BlankNodeParseError(String, Box<BlankNodeIdParseError>),
    /// An error raised if the query type is not supported.
    ///
    /// Some types are: SELECT, ASK, CONSTRUCT.
    UnsupportedQueryType(String),
    /// Errors related to the term index.
    TermIndexError(String),
    /// An error raised if a lock becomes poisoned, e.g., if a thread panics
    /// while holding a write lock.
    LockPoisoned(String),
}

impl From<SerializationErrorKind> for VOWLRError {
    fn from(value: SerializationErrorKind) -> Self {
        <SerializationError as Into<VOWLRError>>::into(value.into())
    }
}

#[derive(Debug)]
pub struct SerializationError {
    /// The contained error type.
    inner: SerializationErrorKind,
    /// The error's location in the source code.
    location: &'static Location<'static>,
    /// When the error occurred.
    timestamp: String,
}

impl std::fmt::Display for SerializationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.inner)
    }
}

impl From<SerializationErrorKind> for SerializationError {
    #[track_caller]
    fn from(error: SerializationErrorKind) -> Self {
        Self {
            inner: error,
            location: Location::caller(),
            timestamp: get_timestamp(),
        }
    }
}

impl<T> From<PoisonError<T>> for SerializationError {
    #[track_caller]
    fn from(value: PoisonError<T>) -> Self {
        Self {
            inner: SerializationErrorKind::LockPoisoned(value.to_string()),
            location: Location::caller(),
            timestamp: get_timestamp(),
        }
    }
}

impl From<SerializationError> for ErrorRecord {
    fn from(value: SerializationError) -> Self {
        let (message, severity) = match value.inner {
            SerializationErrorKind::MissingObject(triple, e)
            | SerializationErrorKind::MissingSubject(triple, e) => {
                (format!("{e}:\n{triple}"), ErrorSeverity::Warning)
            }
            SerializationErrorKind::MissingDomain(edge, e)
            | SerializationErrorKind::MissingRange(edge, e) => {
                (format!("{e}:\n{edge}"), ErrorSeverity::Warning)
            }
            SerializationErrorKind::MissingLabel(e)
            | SerializationErrorKind::MissingProperty(e)
            | SerializationErrorKind::MisisngCharacteristic(e)
            | SerializationErrorKind::MissingIndividualsCount(e) => (e, ErrorSeverity::Warning),
            SerializationErrorKind::SerializationFailedTriple(triple, e) => {
                (format!("{e}:\n{triple}"), ErrorSeverity::Critical)
            }
            SerializationErrorKind::IriParseError(iri, iri_parse_error) => (
                format!("{iri_parse_error}\nIRI: {iri}"),
                ErrorSeverity::Error,
            ),
            SerializationErrorKind::BlankNodeParseError(id, blank_node_id_parse_error) => (
                format!("{blank_node_id_parse_error}\nID: {id}"),
                ErrorSeverity::Error,
            ),
            SerializationErrorKind::SerializationFailed(e)
            | SerializationErrorKind::UnsupportedQueryType(e)
            | SerializationErrorKind::TermIndexError(e)
            | SerializationErrorKind::LockPoisoned(e) => (e, ErrorSeverity::Critical),
        };
        ErrorRecord::new(
            value.timestamp,
            severity,
            ErrorType::Serializer,
            message,
            #[cfg(debug_assertions)]
            Some(value.location.to_string()),
        )
    }
}

impl From<SerializationError> for VOWLRError {
    fn from(value: SerializationError) -> Self {
        <ErrorRecord as Into<VOWLRError>>::into(value.into())
    }
}
