use std::error::Error as StdError;
use std::fmt::{Display, Formatter, Result as FmtResult};

use thiserror::Error;

use crate::models::{code::InvalidIdentPath, schema::NamespaceId, TypeIdent};

/// Error that might be raised by the [`Renderer`](super::Renderer).
#[derive(Debug, Error)]
pub enum Error {
    /// Unknown namespace.
    ///
    /// Is raised if a specific namespace id could not be resolved to it's
    /// corresponding namespace information.
    #[error("Unknown namespace: {0:?}!")]
    UnknownNamespace(NamespaceId),

    /// Invalid identifier.
    ///
    /// Is raised if the user passed a invalid identifier.
    #[error("{0}")]
    InvalidIdentifier(
        #[from]
        #[source]
        InvalidIdentPath,
    ),

    /// An application-specific error returned by a render step.
    #[error("Custom renderer error")]
    Custom(#[source] Box<dyn StdError + Send + Sync>),

    /// A render callback failed. The renderer adds this context automatically.
    #[error(
        "Render step {step} failed during {phase}{}",
        .ident.as_ref().map_or_else(String::new, |ident| format!(" for {ident}"))
    )]
    RenderStep {
        /// Name of the render step that failed.
        step: &'static str,

        /// Callback that failed.
        phase: RenderStepPhase,

        /// Schema type being rendered, if the failure occurred in `render_type`.
        ident: Option<TypeIdent>,

        /// Error returned by the callback.
        #[source]
        source: Box<Self>,
    },
}

impl Error {
    /// Wrap an application-specific error while preserving its source chain.
    ///
    /// The original message and error type are available through
    /// [`std::error::Error::source`].
    pub fn custom<E>(error: E) -> Self
    where
        E: Into<Box<dyn StdError + Send + Sync>>,
    {
        Self::Custom(error.into())
    }
}

/// Callback phase in which a [`RenderStep`](super::RenderStep) failed.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum RenderStepPhase {
    /// The [`initialize`](super::RenderStep::initialize) callback.
    Initialize,

    /// The [`render_type`](super::RenderStep::render_type) callback.
    RenderType,

    /// The [`finish`](super::RenderStep::finish) callback.
    Finish,
}

impl Display for RenderStepPhase {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str(match self {
            Self::Initialize => "initialize",
            Self::RenderType => "render_type",
            Self::Finish => "finish",
        })
    }
}
