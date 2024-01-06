import { h, Component, render} from 'preact';
import { useState } from 'preact/hooks'
import htm from 'htm';
import { marked } from 'marked';
import insane from 'insane';
import { hash128 } from './lib/murmurhash3.js'
import anime from 'animejs'
import ZingTouch from 'zingtouch';

const text = `
## Markdown Markdown

* **Officer Hyustus Staget** is back
* This building was supposed to be under surveillance but apparently the patrols have been skipping it.
* **secret**: They're looking for Urstul Floxin, a high-level Zhentarim gang-boss
* if Renaer Neverember is alive and well, the players will get off scot-free and the police will be cool and chill, if not, they've got a LOT of explaining to do
* the police don't want to get deeply involved in a gang war or go into the sewers. They'll advise the characters "keep the blood off the streets, okay?".
`;

const rendered = insane(marked.parse(text));

const html = htm.bind(h);

let Rendered = () => {
    return html`<div class="hi" dangerouslySetInnerHTML=${{ __html: rendered }}></div>`;
}

let Quiz = () => {
    let [index, setIndex] = useState(0);
    let items = [];
    for(let i = 0; i < 100; i++){
        items.push(i)
    }
    let counter = 0;
    return html`<div class="card">
        <div class="content">
            <div class="header">
                <div class="prev">«</div>
                <div class="more">…</div>
                <div class="next">»</div>
            </div>
            <div class="bump"></div>
            <div class="everything">
                <h2>Hi!</h2>
                <${Rendered} />
            </div>
            <div class="bump"></div>
            <div class="footer"></div>
        </div>
    </div>`
}

let Nav = () => {
    return html`<nav>
        <div class="nav-item">Home</div>
        <div class="nav-item">About</div>
    </nav>`
}

console.log(hash128(rendered));

render(html`<${Quiz} name="World" />`, document.getElementById('app'));