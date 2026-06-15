/* SystemKraft — shared client script (served at /assets/main.js).
   Sole responsibility: the light/dark theme toggle, persisted in
   localStorage and applied before paint to avoid a flash. */
(function () {
  "use strict";
  var KEY = "ms-theme";
  var root = document.documentElement;

  var saved = localStorage.getItem(KEY);
  if (saved === "light" || saved === "dark") root.setAttribute("data-theme", saved);

  function sync() {
    var dark = root.getAttribute("data-theme") === "dark";
    var sun = document.getElementById("sun");
    var moon = document.getElementById("moon");
    if (sun) sun.style.display = dark ? "none" : "block";
    if (moon) moon.style.display = dark ? "block" : "none";
  }

  function bind() {
    var btn = document.getElementById("themeBtn");
    if (btn) {
      btn.addEventListener("click", function () {
        var next = root.getAttribute("data-theme") === "dark" ? "light" : "dark";
        root.setAttribute("data-theme", next);
        localStorage.setItem(KEY, next);
        sync();
      });
    }
    sync();
  }

  if (document.readyState === "loading") {
    document.addEventListener("DOMContentLoaded", bind);
  } else {
    bind();
  }
})();
