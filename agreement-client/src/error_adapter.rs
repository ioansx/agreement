// use agreement_models::error::Eresult;
// use js_sys::Error as JsError;
//
// pub trait JsResult<T> {
//     fn as_js(self) -> core::result::Result<T, JsError>;
// }
//
// impl<T> JsResult<T> for Eresult<T> {
//     fn as_js(self) -> core::result::Result<T, JsError> {
//         self.map_err(|e| JsError::new(&e.to_string()))
//     }
// }
