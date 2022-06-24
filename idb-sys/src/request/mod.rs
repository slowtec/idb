mod database_request;
mod request_ready_state;
mod store_request;

pub use self::{
    database_request::DatabaseRequest, request_ready_state::RequestReadyState,
    store_request::StoreRequest,
};

use js_sys::Object;
use wasm_bindgen::JsValue;
use web_sys::{DomException, Event};

use crate::{Error, Transaction};

/// Specifies all the functions supported by request objects.
pub trait Request {
    /// When a request is completed, returns the `result`, or `undefined` if the request failed. Returns an [`Error`] if
    /// the request is still pending.
    fn result(&self) -> Result<JsValue, Error>;

    /// When a request is completed, returns the `error` (a `DOMException`), or `None` if the request succeeded. Returns
    /// an [`Error`] if the request is still pending.
    fn error(&self) -> Result<Option<DomException>, Error>;

    /// Returns the `ObjectStore`, `Index`, or `Cursor` the request was made against, or `null` if it was an open
    /// request.
    fn source(&self) -> Result<Object, Error>; // TODO: make return type as enum: (IDBObjectStore or IDBIndex or IDBCursor)

    /// Returns the `Transaction` the request was made within. If this as an open request, then it returns an upgrade
    /// transaction while it is running, or `None` otherwise.
    fn transaction(&self) -> Option<Transaction>;

    /// Returns `RequestReadyState::Pending` until a request is complete, then returns `RequestReadyState::Done`.
    fn ready_state(&self) -> Result<RequestReadyState, Error>;

    /// Adds an event handler for `success` event.
    fn on_success<F>(&mut self, callback: F)
    where
        F: FnOnce(Event) + 'static;

    /// Adds an event handler for `error` event.
    fn on_error<F>(&mut self, callback: F)
    where
        F: FnOnce(Event) + 'static;
}
