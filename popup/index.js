import init, { Citation, Dom } from "../html-meta/pkg/html_meta.js"


function onCopyHandler() {
    const text = document.getElementById("text")
    navigator.clipboard.writeText(text.textContent)

}

async function onMessageHandler(message) {
    await init()
    try {
        const dom = new Dom(message.dom, message.url)
        const citation = new Citation(dom)
        const citation_yml = citation.to_yaml_str()
        browser.storage.local.set({ [message.url]: citation_yml })
        // TODO this runs now twice for instant change of the text field
        displayCitation(message.url)
    } catch (e) {
        console.error(e)
    }
}

function displayCitation(idx) {
    console.debug(`displayCitation: ${idx}`)
    const text = document.getElementById("text")

    browser.storage.local.get().then(citations => {
        console.debug(citations)

        const citation = citations[idx]
        if (citation == undefined) {
            text.textContent = `example:
            type: Web
            title: This is a placeholder
            author: John Doe
            url: http://example.com`
        } else {
            text.textContent = citation
        }
    })
}

async function getCurrentTabUrl() {
    const tabs = await browser.tabs.query({ active: true, currentWindow: true });
    return tabs[0].url;
}



browser.runtime.onMessage.addListener(onMessageHandler)
document.getElementById("copy").addEventListener("click", onCopyHandler)

const currentUrl = await getCurrentTabUrl();
displayCitation(currentUrl)
