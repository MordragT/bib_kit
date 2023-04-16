import init, { generate_citation } from "../hlp/pkg/hlp.js"

function registerCopyHandler() {
    console.log("register copy handler")
    const text = document.getElementById("text")

    document.getElementById("copy").addEventListener("click", e => {
        navigator.clipboard.writeText(text.textContent)
    })
}

async function registerMessageHandler() {
    await init()
    console.log("loaded wasm module")

    console.log("register message handler")
    browser.runtime.onMessage.addListener(message => {
        console.log(message)
        try {
            const citation = generate_citation(message.dom, message.url)
            browser.storage.local.set({ citation: citation })
            loadCitation()
        } catch (e) {
            console.error(e)
        }
    })
}

function loadCitation() {
    console.log("load citation")
    const text = document.getElementById("text")

    browser.storage.local.get("citation").then(obj => {
        console.log(obj)
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