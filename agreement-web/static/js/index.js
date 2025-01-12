import init, { AgreementClient } from "./agreement_client.js";
import PageMan from "./page_man.js";

await init();

const client = new AgreementClient(window.location.origin);
const pageMan = new PageMan(client);

window.context = {
  pages: {
    man: pageMan,
  },
};
