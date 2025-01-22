function safeSetOnclickHandler(id, handler) {
  const el = document.getElementById(id);
  if (el) {
    el.onclick = handler;
  }
}

function safeSetOnsubmitHandler(name, handler) {
  const el = document.forms[name];
  if (el) {
    el.onsubmit = handler;
  }
}


export { safeSetOnclickHandler, safeSetOnsubmitHandler };
