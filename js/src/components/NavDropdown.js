import { h, Component, render} from 'preact';
import htm from 'htm';
import { useState } from 'preact/hooks'
import TitleCard from './TitleCard.js';
import thumbnailify from '../thumbnailify.js';

import Icon from './Icon.js';

const html = htm.bind(h);

export default function NavDropdown({onMenu, navigateTo, data}){

    let index = data.getIndex();
    let sitemap = data.getSitemap();

    let entries = Object.entries(sitemap);
    // sort entries by the newest entry in the list of decks
    // (so whichever author has the newest deck will be first)
    entries.sort((a, b) => {
        let newestA = a[1].reduce((acc, deck) => {
            return Math.max(acc, deck.last_update_time.secs_since_epoch);
        }, 0);
        let newestB = b[1].reduce((acc, deck) => {
            return Math.max(acc, deck.last_update_time.secs_since_epoch);
        }, 0);
        return newestB - newestA;
    });

    /*
        This is the thing that appears when you click the hamburger button.
        Ideally, it'll have a Table of Contents, and maybe some other stuff?
        Credits? A link to the source code? A link to the user's profile?
        The world is our oyster.
    */
    return html`<nav id="full-nav">
        <ul class="navbar">
            <li>
                <a onClick=${onMenu} title="Close the Hamburger Zone" autofocus>
                    <${Icon} name="hamburger" />
                </a>
            </li>
        </ul>
        <div class="nav-dropdown">

            <${TitleCard} index=${index} />

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
            ${entries.length > 0 ? html`<h3>Sitemap</h3>` : ""}
            <div>
                ${entries.map(([authorSlug, listOfDecks]) => {
                    let author = listOfDecks[0].author;

                    let countOfVisibleDecks = listOfDecks.filter((deck) => {
                        return !deck.hidden;
                    }).length;

                    // sort listOfDecks by last_update_time.secs_since_epoch
                    listOfDecks.sort((a, b) => {
                        return b.last_update_time.secs_since_epoch - a.last_update_time.secs_since_epoch;
                    });

                    if(countOfVisibleDecks == 0){
                        return null;
                    }

                    return html`<div class='sitemap-author'>
                        <h4>${author}</h4>
                        <ul>
                            ${listOfDecks.map((deck) => {
                                if(deck.hidden){
                                    return null;
                                }
                                let image_url = thumbnailify({
                                    imageUrl: `${window.location.origin}/s/${deck.author_slug}/${deck.slug}/${deck.image_url}`,
                                    height: 100,
                                    width: 120,
                                });
                                let updatedDate = new Date(deck.last_update_time.secs_since_epoch * 1000).toLocaleString();

                                return html`<li>
                                    <a class="sitemap-entry" href="${window.location.origin}/s/${deck.author_slug}/${deck.slug}" title="${deck.title}">
                                        <div class="panel-left">
                                            <h4>${deck.title}</h4>
                                            <p><small>${updatedDate}</small></p>
                                            <p>${deck.description}</p>
                                        </div>
                                        <div class="panel-right">
                                            <img src="${image_url}" alt="${deck.title}" />
                                        </div>
                                    </a>
                                </li>`;
                            })}
                        </ul>
                    </div>`;
                })}
            </div>
            <hr/>

            <div class="credits">
                <p>
                    <a href="https://github.com/cube-drone/ministry/">CardChapter</a> is a lightweight,
                        open-source, web-based card presentation system
                        by <a href="https://cube-drone.com">cube drone</a>.
                </p>

            </div>
        </div>
    </nav>`;
}