function safeSetOnclickHandler(id, handler) {
  const el = document.getElementById(id);
  if (el) {
    el.onclick = handler;
  }
}

function safeSetOnsubmitHandler(name, handler) {
  const el = document.forms[name];
  if (!el) {
    throw new Error(`Form "${name} not found.`);
  }

  el.onsubmit = handler;
}


export { safeSetOnclickHandler, safeSetOnsubmitHandler };
