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
                    <a onClick=${top} title="top" class="nav-top">
                        <${Icon} name="double-up" />
                    </a>
                </li>
                <li>
                    <a onClick=${up} title="up" class="nav-up">
                        <${Icon} name="chevron-up" />
                    </a>
                </li>
                <li>
                    <a onClick=${menu} title="Menu" class="nav-menu">
                        <${Icon} name="hamburger" />
                    </a>
                </li>
                <li>
                    <a onClick=${down} title="down" class="nav-down">
                        <${Icon} name="chevron-down" />
                    </a>
                </li>
                <li>
                    <a onClick=${bottom} title="bottom" class="nav-bottom">
                        <${Icon} name="double-down" />
                    </a>
                </li>
            </ul>
        </nav>`;
}
