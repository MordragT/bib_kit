console.log("Running content script")

setTimeout(() => {
    console.log("Wait for complete loading of page")

    const dom = document.documentElement.outerHTML
    const url = window.location.href

    browser.runtime.sendMessage({
        url: url,
        dom: dom,
    })
        .catch(e => `Cannot send citation: ${e.message}`)

    console.log("Send params");

}, 256)

