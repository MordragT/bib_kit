console.debug("Running content script")

// setTimeout(() => {
console.debug("Wait for complete loading of page")

const dom = document.documentElement.outerHTML
const url = window.location.href

browser.runtime.sendMessage({
    url: url,
    dom: dom,
})
    .catch(e => `Cannot send citation: ${e.message}`)

console.debug("Send params");

// }, 256)

