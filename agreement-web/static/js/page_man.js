import { safeSetOnsubmitHandler } from "./tools.js";

const FORM = "page_man__form";
const OUTPUT = "page_man__output";

class PageMan {
  constructor(client) {
    this.client = client;

    safeSetOnsubmitHandler(FORM, this.fetchManPage);
  }

  fetchManPage = async (event) => {
    event.preventDefault();

    const command = document.forms[FORM].elements["command"].value;
    const outdto = await this.client.get_man_page(command);
    const article = document.getElementById(OUTPUT);
    const articleHeader = article.children.item(0);
    articleHeader.textContent = command;
    const articlePre = article.children.item(1);
    articlePre.innerText = outdto.output;
  };
}

export default PageMan;
