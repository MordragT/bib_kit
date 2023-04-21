import init, { Citation, Dom } from "../html-meta/pkg/html_meta.js"

function registerCopyHandler() {
    console.debug("register copy handler")
    const text = document.getElementById("text")

    document.getElementById("copy").addEventListener("click", e => {
        navigator.clipboard.writeText(text.textContent)
    })
}

async function registerMessageHandler() {
    await init()
    console.debug("loaded wasm module")

    console.debug("register message handler")
    browser.runtime.onMessage.addListener(message => {
        console.debug(message)
        try {
            const dom = new Dom(message.dom, message.url)
            const citation = new Citation(dom)
            const citation_yml = citation.to_yaml_str()
            browser.storage.local.set({ citation: citation_yml })
            loadCitation()
        } catch (e) {
            console.error(e)
        }
    })
}

function loadCitation() {
    console.debug("load citation")
    const text = document.getElementById("text")

    browser.storage.local.get("citation").then(obj => {
        console.debug(obj)
        if (obj.citation != undefined) {
            text.textContent = obj.citation
        } else {
            const example = `example:
        type: Web
        title: This is a placeholder
        author: John Doe
        url: http://example.com`
            text.textContent = example
        }
    })
}

registerCopyHandler()
registerMessageHandler()
loadCitation()