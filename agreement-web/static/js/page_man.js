import { safeSetOnclickHandler } from "./tools.js";

const BUTTON_FETCH = "man-btn-fetch";
const INPUT_COMMAND = "man-input-command";
const ARTICLE_PAGE = "man-article-page";

class PageMan {
  constructor(client) {
    this.client = client;

    safeSetOnclickHandler(BUTTON_FETCH, this.fetchManPage);
  }

  fetchManPage = async () => {
    const command = document.getElementById(INPUT_COMMAND).value;
    const outdto = await this.client.get_man_page(command);
    const article = document.getElementById(ARTICLE_PAGE);
    article.children.item(0).textContent = command;
    article.appendChild(document.createTextNode(outdto.output));
    article.style.display = "unset";
  };
}

export default PageMan;
