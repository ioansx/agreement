import init, { AgreementClient } from "./agreement_client.js";
await init();

window.agreement = new AgreementClient(window.location.origin);
