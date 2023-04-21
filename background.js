import init, { Citation, Dom } from "./html-meta/pkg/html_meta.js"

async function onMessageHandler(message) {
    if (message.kind == "params") {
        console.debug("ReceivedMessage: params")

        await init()
        try {
            const dom = new Dom(message.dom, message.url)
            const citation = new Citation(dom)
            const citation_yml = citation.to_yaml_str()
            browser.storage.local.set({ [message.url]: citation_yml })
            browser.runtime.sendMessage({ kind: "update", citation: citation_yml, url: message.url })
        } catch (e) {
            console.error(e)
        }
    }

}

browser.runtime.onMessage.addListener(onMessageHandler)