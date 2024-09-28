import { h, Component, render} from 'preact';
import htm from 'htm';
import { useState } from 'preact/hooks'
import anime from 'animejs';

import Icon from './Icon.js';


const html = htm.bind(h);

export default function Nav({onTop, onBottom, onDown, onUp, onMenu }){
    if(!onTop){
        onTop = () => {};
    }
    if(!onBottom){
        onBottom = () => {};
    }
    if(!onDown){
        onDown = () => {};
    }
    if(!onUp){
        onUp = () => {};
    }
    if(!onMenu){
        onMenu = () => {};
    }

    const pressAnimation = (thinger) => {
        anime({
            targets: `.nav-${thinger} svg`,
            scale: 1.3,
            duration: 200,
            easing: 'easeInOutQuad',
            direction: 'alternate',
            loop: 1
        });
    }
    const top = () => {
        pressAnimation("top");
        onTop();
    }
    const bottom = () => {
        pressAnimation("bottom");
        onBottom();
    }
    const down = () => {
        pressAnimation("down");
        onDown();
    }
    const up = () => {
        pressAnimation("up");
        onUp();
    }
    const menu = () => {
        pressAnimation("menu");
        onMenu();
    }
    return html`<nav id="primary-nav">
            <ul class="navbar">
                <li>
                    <a onClick=${top} title="Navigate to the first card in the deck" class="nav-top" aria-label="First card" tabindex="-1">
                        <${Icon} name="double-up" />
                    </a>
                </li>
                <li>
                    <a onClick=${up} title="Navigate to the previous card in the deck" class="nav-up" aria-label="Previous card" tabindex="-1">
                        <${Icon} name="chevron-up" />
                    </a>
                </li>
                <li>
                    <a onClick=${menu} title="Open a menu with a description of the current page, table of contents, and sitemap" class="nav-menu" aria-label="Extended Nav Menu" tabindex="-1">
                        <${Icon} name="hamburger" />
                    </a>
                </li>
                <li>
                    <a onClick=${down} title="Navigate to the next card in the deck" class="nav-down" aria-label="Next Card" tabindex="-1">
                        <${Icon} name="chevron-down" />
                    </a>
                </li>
                <li>
                    <a onClick=${bottom} title="Navigate to the final card in the deck" class="nav-bottom" aria-label="Last Card" tabindex="-1">
                        <${Icon} name="double-down" />
                    </a>
                </li>
            </ul>
        </nav>`;
}
