import { h, Component, render} from 'preact';
import htm from 'htm';
import { useState } from 'preact/hooks'

import Icon from './Icon.js';

const html = htm.bind(h);

export default function NavDropdown({onMenu, data}){

    let index = data.getIndex();

    /*
        This is the thing that appears when you click the hamburger button.
        Ideally, it'll have a Table of Contents, and maybe some other stuff?
        Credits? A link to the source code? A link to the user's profile?
        The world is our oyster.
    */
    console.dir(index);
    let thumbnailImage = null;
    if(index.thumbnailImageUrl){
        thumbnailImage = html`<img src="${index.thumbnailImageUrl}" alt="${index.name}" />`;
    }

    return html`<nav id="full-nav">
        <ul class="navbar">
            <li>
                <a onClick=${onMenu} title="Menu">
                    <${Icon} name="hamburger" />
                </a>
            </li>
        </ul>
        <div class="nav-dropdown">
            <h2>${index.name}</h2>
            ${thumbnailImage}
            <p class="author">${index.author}</p>
            <p>${index.description}</p>
            <div style="clear:both;"></div>
            <h3>Table of Contents</h3>
            <ul>
                ${index.contentIds.map((id) => {
                    return html`<li><a href="${window.location.origin}${window.location.pathname}#${id}">${id}</a></li>`;
                })}
            </ul>
            <pre>
                <code>
                    ${JSON.stringify(index, null, 2)}
                </code>
            </pre>
        </div>
    </nav>`;
}