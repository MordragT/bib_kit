function registerCopyHandler() {
    console.log("register copy handler")
    const text = document.getElementById("text")

    document.getElementById("copy").addEventListener("click", e => {
        navigator.clipboard.writeText(text.textContent)
    })
}

function registerMessageHandler() {
    console.log("register message handler")
    const text = document.getElementById("text")

    browser.runtime.onMessage.addListener(message => {
        if (message.kind == "citation") {
            console.log("received message")
            text.textContent = message.citation
        }
    })
}

registerCopyHandler()
registerMessageHandler()
