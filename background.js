import init from "./hlp/pkg/hlp.js"

const Type = {
    // A short text, possibly of journalistic or scientific nature,
    // appearing in some greater publication (default parent: periodical)
    Article: "article",
    // A section of a greater containing work (default parent: book)
    Chapter: "chapter",
    // A short segment of media on some subject matter.
    // Could appear in a work of reference or a data set (default parent: reference)
    Entry: "entry",
    // Text published within an Anthology (default parent: anthology)
    Anthos: "anthos",
    // A document compiled by authors that may be affiliated to an organization.
    // Presents information for a specific audience or purpose
    Report: "report",
    // Scholarly work delivered to fulfill degree requirements at a higher education institution
    Thesis: "thesis",
    // Piece of content that can be found on the internet and is native to the medium,
    // like an animation, a web app, or a form of content not found elsewhere.
    // Do not use this entry type when referencing a textual blog article,
    // instead use an article with a blog parent (default parent: web).
    Web: "web",
    // A part of a show or another type of performed media,
    // typically all taking place in the same location (default parent: video).
    Scene: "scene",
    // A form of artistic/creative expression (default parent: exhibition).
    Artwork: "artwork",
    // A technical document deposited at a government agency
    // that describes an invention to legally limit the rights of reproduction to the inventors.
    Patent: "patent",
    // Reference to a legal case that was or is to be heared at a court of law.
    Case: "case",
    // The issue of a newspaper that was published on a given day.
    Newspaper: "newspaper",
    // Legal document or draft thereof that is, is to be,
    // or was to be enacted into binding law (default parent: anthology).
    Legislation: "legislation",
    // Written document that is submitted as a candidate for publication.
    Manuscript: "manuscript",
    // A post on a micro-blogging platform like Twitter (default parent: tweet).
    Tweet: "tweet",
    // Items that do not match any of the other Entry type composites.
    Misc: "misc",
    // A publication that periodically publishes issues with unique content.
    // This includes scientific journals and news magazines.
    Periodical: "periodical",
    // The official published record of the events at a professional conference.
    Proceedings: "proceedings",
    // Long-form work published physically as a set of bound sheets.
    Book: "book",
    // Set of self-published articles on a website.
    Blog: "blog",
    //  A work of reference. This could be a manual or a dictionary.
    Reference: "reference",
    // Professional conference.
    // This Entry type implies that the item referenced has been an event at the conference itself.
    // If you instead want to reference a paper published in the published proceedings of the conference,
    // use an article with a proceedings parent.
    Conference: "conference",
    // Collection of different texts on a single topic/theme.
    Anthology: "anthology",
    // Publicly visible storage of the source code for a particular software, papers,
    // or other data and its modifications over time.
    Repository: "repository",
    // Written discussion on the internet triggered by an original post.
    // Could be on a forum, social network, or Q&A site.
    Thread: "thread",
    // Motion picture of any form, possibly with accompanying audio (default parent: video).
    Video: "video",
    // Recorded audible sound of any kind (default parent: audio).
    Audio: "audio",
    // A curated set of artworks.
    Exhibition: "exhibition",
}
const Role = {
    // Translated the work from a foreign language to the cited edition.
    Translator: "translator",
    // Authored an afterword.
    Afterword: "afterword",
    // Authored a foreword.
    Foreword: "foreword",
    // Authored an introduction.
    Introduction: "introduction",
    // Provided value-adding annotations.
    Annotator: "annotator",
    // Commented on the work.
    Commentator: "commentator",
    // Holds a patent or similar.
    Holder: "holder",
    // Compiled the works in an Anthology.
    Compiler: "compiler",
    // Founded the publication.
    Founder: "founder",
    // Collaborated on the cited item.
    Collaborator: "collaborator",
    // Organized the creation of the cited item.
    Organizer: "organizer",
    // Performed in the cited item.
    CastMember: "cast-member",
    // Composed all or parts of the cited item's musical/audible components.
    Composer: "composer",
    // Produced the cited item.
    Producer: "producer",
    // Lead Producer for the cited item.
    ExecutiveProducer: "executive-producer",
    // Did the writing for the cited item.
    Writer: "writer",
    // Shot film/video for the cited item.
    Cinematography: "cinematography",
    // Directed the cited item.
    Director: "director",
    // Illustrated the cited item.
    Illustrator: "illustrator",
    // Provided narration or voice-over for the cited item.
    Narrator: "narrator"
}

const hlp = await init()

console.log("loaded wasm module")

browser.runtime.onMessage.addListener(message => {
    if (message.kind == "params") {
        console.log(`received message with url: ${message.url}`)
        const citation = hlp.generate_citation(message.dom, message.url)
        browser.runtime.sendMessage({
            kind: "citation",
            citation: citation
        })
    }
})