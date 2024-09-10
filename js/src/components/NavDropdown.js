import { h, Component, render} from 'preact';
import htm from 'htm';
import { useState } from 'preact/hooks'

import Icon from './Icon.js';

const html = htm.bind(h);

function thumbnailify(image_url, width){
    let url = new URL(image_url);
    url.searchParams.set('width', width);
    return url.toString();
}

export default function NavDropdown({onMenu, navigateTo, data}){

    let index = data.getIndex();
    let sitemap = data.getSitemap();

    /*
        This is the thing that appears when you click the hamburger button.
        Ideally, it'll have a Table of Contents, and maybe some other stuff?
        Credits? A link to the source code? A link to the user's profile?
        The world is our oyster.
    */
    let thumbnailImage = null;
    if(index.thumbnailImageUrl){
        thumbnailImage = html`<img src="${thumbnailify(`${window.location.origin}${window.location.pathname}${index.thumbnailImageUrl}`, 100)}" alt="${index.name}" />`;
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
            <div class="toc">
                <h3>Table of Contents</h3>
                <ul>
                    ${index.toc.map(({title, id, depth}) => {
                        if(depth < 0){
                            return null;
                        }
                        let depthstyle = `margin-left: ${depth}em;`;
                        return html`<li style=${depthstyle}><a onClick=${(evt)=>{evt.preventDefault(); navigateTo(id);}}
                                        href="${window.location.origin}${window.location.pathname}#${id}">${title ?? id}</a></li>`;
                    })}
                </ul>
            </div>
            <hr/>
            ${Object.keys(sitemap).length > 0 ? html`<h3>Sitemap</h3>` : ""}
            <div>
                ${Object.entries(sitemap).map(([authorSlug, listOfDecks]) => {
                    let author = listOfDecks[0].author;

                    let countOfVisibleDecks = listOfDecks.filter((deck) => {
                        return !deck.hidden;
                    }).length;

                    if(countOfVisibleDecks == 0){
                        return null;
                    }

                    return html`<div class='sitemap-entry'>
                        <h4>${author}</h4>
                        <ul>
                            ${listOfDecks.map((deck) => {
                                if(deck.hidden){
                                    return null;
                                }
                                let image_url = thumbnailify(`${window.location.origin}/s/${deck.author_slug}/${deck.slug}/${deck.image_url}`, 50);
                                return html`<li>
                                    <img src="${image_url}" alt="${deck.title}" />
                                    <a href="${window.location.origin}/s/${deck.author_slug}/${deck.slug}" title="${deck.description}">${deck.title}</a>
                                    <p>${deck.description}</p>
                                </li>`;
                            })}
                        </ul>
                    </div>`;
                })}
            </div>

        </div>
    </nav>`;
}