use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen_futures::js_sys;

#[wasm_bindgen]
pub struct Htmx {
    on_load: (),
    process: (),
    on: (),
    off: (),
    trigger: (),
    ajax: (),
    find: (),
    findAll: (),
    closest: (),
}

impl Htmx {
    // fn values(elt: js_sys::Element, type: Option<String>): () {
    //   let inputValues = getInputValues(elt, type.unwrap_or("post".to_string()));
    //   inputValues.values
    // }
}

fn getInputValues(elt: (), verb: ()) {
    const processed: web_sys::Element = vec![];
    // const formData = new FormData()
    // const priorityFormData = new FormData()
    // /** @type HtmxElementValidationError[] */
    // const errors = []
    // const internalData = getInternalData(elt)
    // if (internalData.lastButtonClicked && !bodyContains(internalData.lastButtonClicked)) {
    //   internalData.lastButtonClicked = null
    // }
    //
    // // only validate when form is directly submitted and novalidate or formnovalidate are not set
    // // or if the element has an explicit hx-validate="true" on it
    // let validate = (elt instanceof HTMLFormElement && elt.noValidate !== true) || getAttributeValue(elt, 'hx-validate') === 'true'
    // if (internalData.lastButtonClicked) {
    //   validate = validate && internalData.lastButtonClicked.formNoValidate !== true
    // }
    //
    // // for a non-GET include the closest form
    // if (verb !== 'get') {
    //   processInputValue(processed, priorityFormData, errors, closest(elt, 'form'), validate)
    // }
    //
    // // include the element itself
    // processInputValue(processed, formData, errors, elt, validate)
    //
    // // if a button or submit was clicked last, include its value
    // if (internalData.lastButtonClicked || elt.tagName === 'BUTTON' ||
    //   (elt.tagName === 'INPUT' && getRawAttribute(elt, 'type') === 'submit')) {
    //   const button = internalData.lastButtonClicked || (/** @type HTMLInputElement|HTMLButtonElement */(elt))
    //   const name = getRawAttribute(button, 'name')
    //   addValueToFormData(name, button.value, priorityFormData)
    // }
    //
    // // include any explicit includes
    // const includes = findAttributeTargets(elt, 'hx-include')
    // forEach(includes, function(node) {
    //   processInputValue(processed, formData, errors, asElement(node), validate)
    //   // if a non-form is included, include any input values within it
    //   if (!matches(node, 'form')) {
    //     forEach(asParentNode(node).querySelectorAll(INPUT_SELECTOR), function(descendant) {
    //       processInputValue(processed, formData, errors, descendant, validate)
    //     })
    //   }
    // })
    //
    // // values from a <form> take precedence, overriding the regular values
    // overrideFormData(formData, priorityFormData)
    //
    // return { errors, formData, values: formDataProxy(formData) }
}
