console.log("Running content script")
const dom = document.documentElement.outerHTML
const url = window.location.href

browser.runtime.sendMessage({
    kind: "params",
    url: url,
    dom: dom,
})
    .catch(e => `Cannot send citation: ${e.message}`)

console.log("Send params");
