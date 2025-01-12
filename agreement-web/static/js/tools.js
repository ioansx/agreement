function safeSetOnclickHandler(id, handler) {
  const el = document.getElementById(id);
  if (el) {
    el.onclick = handler;
  }
}

export { safeSetOnclickHandler };
