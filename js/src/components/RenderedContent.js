
import { h, Component, render} from 'preact';
import htm from 'htm';

import { marked } from 'marked';
import insane from 'insane';

const html = htm.bind(h);

function TitleCard({card}){
    if(!card.title){
        return html`<div class="title-card">
            <h1>${card.id}</h1>
        </div>`;
    }
    else{
        return html`<div class="card title-card">
            <h1>${card.title}</h1>
        </div>`;
    }
}

function MarkdownCard({card}){
    return html`<div class="card markdown-card">
        <div class="markdown-content" dangerouslySetInnerHTML=${{ __html: insane(marked.parse(card.content)) }}></div>
    </div>`;
}

function HtmlCard({card}){
    return html`<div class="card html-card">
        <div class="markdown-content" dangerouslySetInnerHTML=${{ __html: insane(card.content) }}></div>
    </div>`;
}

function ImageCard({card}){
    return html`<div class="card image-card">
        <img src=${card.imageUrl} alt=${card.alt} title=${card.title}/>
    </div>`;
}

function ErrorCard({card}){
    return html`<div class="card error-card">
        <div class="error-content">
            <pre>
            <code>
                ${JSON.stringify(card, null, 2)}
            </code>
            </pre>
        </div>
    </div>`;
}

export default function RenderedContent({content, primary, visible}){
    let card = content;
    let cardClass = ErrorCard;
    if(card.type === 'markdown'){
        cardClass = MarkdownCard;
    }
    if(card.type === 'html'){
        cardClass = HtmlCard;
    }
    if(card.type === 'title'){
        cardClass = TitleCard
    }
    if(card.type === 'image'){
        cardClass = ImageCard;
    }
    return html`<div class="rendered-content">
        <${cardClass} card=${card} primary=${primary} visible=${visible}/>
    </div>`;
}