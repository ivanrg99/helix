use std::sync::Arc;

use arc_swap::ArcSwap;
use helix_event::AsyncHook;

use crate::config::Config;
use crate::events;
use crate::handlers::auto_save::AutoSaveHandler;
use crate::handlers::inlay_hints::InlayHintHandler;
use crate::handlers::signature_help::SignatureHelpHandler;

pub use helix_view::handlers::Handlers;

use self::document_colors::DocumentColorsHandler;

mod auto_save;
pub mod completion;
mod diagnostics;
mod document_colors;
mod inlay_hints;
mod signature_help;
mod snippet;

pub fn setup(config: Arc<ArcSwap<Config>>) -> Handlers {
    events::register();

    let event_tx = completion::CompletionHandler::new(config).spawn();
    let signature_hints = SignatureHelpHandler::new().spawn();
    let inlay_hints = InlayHintHandler::default().spawn();
    let auto_save = AutoSaveHandler::new().spawn();
    let document_colors = DocumentColorsHandler::default().spawn();

    let handlers = Handlers {
        completions: helix_view::handlers::completion::CompletionHandler::new(event_tx),
        signature_hints,
        inlay_hints,
        auto_save,
        document_colors,
    };

    helix_view::handlers::register_hooks(&handlers);
    completion::register_hooks(&handlers);
    signature_help::register_hooks(&handlers);
    auto_save::register_hooks(&handlers);
    diagnostics::register_hooks(&handlers);
    snippet::register_hooks(&handlers);
    document_colors::register_hooks(&handlers);
    inlay_hints::register_hooks(&handlers);
    handlers
}
