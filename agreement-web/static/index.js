import init, {
  AgreementClient,
  CreateThingIndto,
  CreateThingOutdto,
} from "./agreement_client.js";

await init();

let thing = new CreateThingIndto("first");
thing.name = "error";

const client = new AgreementClient(window.location.origin);
const response = await client.create_thing(thing);
console.log(response);

// const EL_PARSING_IN = "parsing-in";
// const EL_PARSING_OUT = "parsing-out";
// const EL_PARSING_BTN = "parsing-btn";
// const EL_EXAMPLES = "examples";
//
// const CL_PARSING_OUT = "index__parsing-out";
// const CL_PARSING_OUT_SUCCESS = `${CL_PARSING_OUT}--success`;
// const CL_PARSING_OUT_ERROR = `${CL_PARSING_OUT}--error`;
//
// function parse(input) {
//   let parsingOut = document.getElementById(EL_PARSING_OUT);
//   let parser = new CesrParser(input);
//   try {
//     let parsedData = parser.parse();
//     console.log(parsedData);
//     parsingOut.value = parsedData.to_json_pretty();
//     parsingOut.classList.remove(CL_PARSING_OUT_ERROR);
//     parsingOut.classList.add(CL_PARSING_OUT_SUCCESS);
//   } catch (e) {
//     console.error(e);
//     parsingOut.value = e;
//     parsingOut.classList.remove(CL_PARSING_OUT_SUCCESS);
//     parsingOut.classList.add(CL_PARSING_OUT_ERROR);
//   }
// }
//
// document.getElementById(EL_PARSING_BTN).addEventListener("click", () => {
//   let input = document.getElementById(EL_PARSING_IN).value;
//   parse(input);
// });
//
// document.getElementById(EL_PARSING_OUT).addEventListener("change", (evt) => {
//   evt.target.classList.remove(CL_PARSING_OUT_SUCCESS);
//   evt.target.classList.remove(CL_PARSING_OUT_ERROR);
// });
//
// const examples = document.getElementById(EL_EXAMPLES);
// for (const x of examples.children) {
//   x.addEventListener("click", () => {
//     const parsingIn = document.getElementById(EL_PARSING_IN);
//     const value = x.innerText;
//     parsingIn.value = value;
//     parse(value);
//   });
// }
