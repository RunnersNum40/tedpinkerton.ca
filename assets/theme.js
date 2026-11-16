(() => {
  // Preserve the SDK's compressed CBOR values while switching to plain theme names.
  const legacyThemes = new Map([
    ["78da4bf5c94ccf28010007fd025e", "light"],
    ["78da4b71492cca0600057b01e7", "dark"],
  ]);
  const savedTheme = localStorage.getItem("theme");
  const initialTheme = savedTheme === "light" || savedTheme === "dark"
    ? savedTheme
    : legacyThemes.get(savedTheme) ?? (matchMedia("(prefers-color-scheme: dark)").matches ? "dark" : "light");

  function setTheme(theme) {
    document.documentElement.dataset.theme = theme;
    localStorage.setItem("theme", theme);
  }

  setTheme(initialTheme);

  document.addEventListener("click", event => {
    if (event.target instanceof Element && event.target.closest("#theme-toggle")) {
      setTheme(document.documentElement.dataset.theme === "dark" ? "light" : "dark");
    }
  });
})();
