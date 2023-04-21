console.debug("BIB_KIT:Running content script")

function sendParams() {
    const dom = document.documentElement.outerHTML
    const url = window.location.href

    console.debug(`BIB_KIT:sendParams: ${url}`);

    browser.runtime.sendMessage({
        url: url,
        dom: dom,
    })
        .catch(e => `Cannot send citation: ${e.message}`)

}

sendParams()