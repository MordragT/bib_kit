function onCopyHandler() {
    const text = document.getElementById("text")
    navigator.clipboard.writeText(text.textContent)

}

function displayCitation(citation) {
    const text = document.getElementById("text")

    if (citation == undefined) {
        text.textContent = `example:
        type: Web
        title: This is a placeholder
        author: John Doe
        url: http://example.com`
    } else {
        text.textContent = citation
    }
}

function displayStoredCitation(idx) {
    console.debug(`displayStoredCitation: ${idx}`)

    browser.storage.local.get().then(citations => {
        const citation = citations[idx]
        // console.debug(citation)
        displayCitation(citation)
    })
}

async function getCurrentTabUrl() {
    const tabs = await browser.tabs.query({ active: true, currentWindow: true });
    return tabs[0].url;
}

async function onMessageHandler(message) {
    if (message.kind == "update") {
        console.debug("ReceivedMessage: citation")
        const currentUrl = await getCurrentTabUrl()

        console.debug(`MessageUrl: ${message.url}`)

        if (currentUrl == message.url) {
            displayCitation(message.citation)
        } else {
            displayStoredCitation(currentUrl)
        }
    }
}

browser.runtime.onMessage.addListener(onMessageHandler)

document.getElementById("copy").addEventListener("click", onCopyHandler)

getCurrentTabUrl().then(currentUrl => displayStoredCitation(currentUrl))


