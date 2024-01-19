import { h, Component, render} from 'preact';
import { useState } from 'preact/hooks'
import htm from 'htm';
import { marked } from 'marked';
import insane from 'insane';
import { hash128 } from './lib/murmurhash3.js'
import { v4 as uuid } from "uuid";
import { initialize } from './data.js';

function debounce(func, timeout = 300){
    let timer;
    return (...args) => {
      clearTimeout(timer);
      timer = setTimeout(() => { func.apply(this, args); }, timeout);
    };
}

let bootupTime = Date.now();

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

let showHandlers = {};
let hideHandlers = {};
let visibilityObserver = new IntersectionObserver((entries) => {
    entries.forEach((entry) => {
        let id = entry.target.id;
        if(entry.isIntersecting){
            //console.warn(`showing: ${id}`);
            entry.target.classList.add('currently-visible');
            if(showHandlers[id]){
                showHandlers[id]();
            }
        }
        else{
            entry.target.classList.remove('currently-visible');
            if(hideHandlers[id]){
                hideHandlers[id]();
            }
        }
    });
}
, {
    root: null, // use the viewport
    rootMargin: '0px', // anywhere in the viewport
    threshold: 0.01 // a tiny fraction of the element must be visible
});
let primaryShowHandlers = {};
let primaryHideHandlers = {};
let primaryObserver = new IntersectionObserver((entries) => {
    entries.forEach((entry) => {
        let id = entry.target.id;
        if(entry.isIntersecting){
            //console.warn(`showing: ${id}`);
            entry.target.classList.add('currently-visible');
            if(primaryShowHandlers[id]){
                primaryShowHandlers[id]();
            }
        }
        else{
            entry.target.classList.remove('currently-visible');
            if(primaryHideHandlers[id]){
                primaryHideHandlers[id]();
            }
        }
    });
}
, {
    root: null, // use the viewport
    rootMargin: '0px', // anywhere in the viewport
    threshold: 0.9 // 100% of the element must be visible
});
function observe(id, element, showHandler, hideHandler, primaryShowHandler, primaryHideHandler){
    showHandlers[id] = showHandler;
    hideHandlers[id] = hideHandler;
    primaryShowHandlers[id] = primaryShowHandler;
    primaryHideHandlers[id] = primaryHideHandler;
    visibilityObserver.observe(element);
    primaryObserver.observe(element);
}
function unobserve(id, element){
    delete showHandlers[id];
    delete hideHandlers[id];
    delete primaryShowHandlers[id];
    delete primaryHideHandlers[id];
    visibilityObserver.unobserve(element);
    primaryObserver.unobserve(element);
}

class VisibilityTrigger extends Component {
    constructor(props){
        super(props);
        this.state = {
            visible: false,
            primary: false
        }
    }

    visible(){
        this.setState({
            visible: true
        });
    }

    invisible(){
        this.setState({
            visible: false
        });
    }

    primary(){
        this.setState({
            primary: true
        });
    }

    unprimary(){
        this.setState({
            primary: false
        });
    }

    componentDidMount(){
        let element = this.base;
        this.id = uuid();
        element.id = this.id;
        observe(this.id, element,
            this.visible.bind(this),
            this.invisible.bind(this),
            this.primary.bind(this),
            this.unprimary.bind(this)
        );
    }

    componentWillUnmount(){
        let element = this.base;
        unobserve(this.id, element);
    }

    render(){
        if(this.state.primary){
            return html`<div class="frame frame-primary">
                primary
            </div>`;
        }
        if(this.state.visible){
            return html`<div class="frame frame-visible">
                visible
            </div>`;
        }
        else{
            return html`<div class="frame frame-invisible">
                invisible
            </div>`;
        }
    }
}

function Icon({name}){
    if(name == "chevron-down"){
        return html`<svg viewBox="0 0 64 64" class="svg-icon icon-circle-chevron-down">
            <path class="opt" d="M32,1.8C15.3,1.8,1.8,15.3,1.8,32S15.3,62.3,32,62.3S62.3,48.7,62.3,32S48.7,1.8,32,1.8z M32,57.8
                C17.8,57.8,6.3,46.2,6.3,32C6.3,17.8,17.8,6.3,32,6.3c14.2,0,25.8,11.6,25.8,25.8C57.8,46.2,46.2,57.8,32,57.8z"/>
            <path d="M43.9,23.8L32,35.5L20.1,23.8c-0.9-0.9-2.3-0.9-3.2,0c-0.9,0.9-0.9,2.3,0,3.2l13.5,13.3c0.4,0.4,1,0.6,1.6,0.6
                c0.6,0,1.1-0.2,1.6-0.6L47.1,27c0.9-0.9,0.9-2.3,0-3.2C46.3,22.9,44.8,22.9,43.9,23.8z"/>
        </svg>`;
    }
    if(name == "double-down"){
        return html`<svg viewBox="0 0 64 64" class="svg-icon icon-double-down">
            <path class="opt" d="M30.4,38.4c0.4,0.4,1,0.6,1.6,0.6c0.6,0,1.1-0.2,1.6-0.6l23-22.6c0.9-0.9,0.9-2.3,0-3.2c-0.9-0.9-2.3-0.9-3.2,0L32,33.6
                L10.6,12.6c-0.9-0.9-2.3-0.9-3.2,0c-0.9,0.9-0.9,2.3,0,3.2L30.4,38.4z"/>
            <path d="M53.4,25.6L32,46.6L10.6,25.6c-0.9-0.9-2.3-0.9-3.2,0s-0.9,2.3,0,3.2l23,22.6c0.4,0.4,1,0.6,1.6,0.6c0.6,0,1.1-0.2,1.6-0.6
                l23-22.6c0.9-0.9,0.9-2.3,0-3.2S54.3,24.7,53.4,25.6z"/>
        </svg>`;
    }
    if(name == "chevron-up"){
        return html`<svg viewBox="0 0 64 64" class="svg-icon icon-circle-chevron-up">
            <path class="opt" d="M32,1.8C15.3,1.8,1.8,15.3,1.8,32S15.3,62.3,32,62.3S62.3,48.7,62.3,32S48.7,1.8,32,1.8z M32,57.8
                C17.8,57.8,6.3,46.2,6.3,32C6.3,17.8,17.8,6.3,32,6.3c14.2,0,25.8,11.6,25.8,25.8C57.8,46.2,46.2,57.8,32,57.8z"/>
            <path d="M33.6,23.8c-0.9-0.9-2.3-0.9-3.2,0L16.9,37c-0.9,0.9-0.9,2.3,0,3.2c0.4,0.4,1,0.7,1.6,0.7c0.6,0,1.1-0.2,1.6-0.6L32,28.5
                l11.9,11.7c0.9,0.9,2.3,0.9,3.2,0c0.9-0.9,0.9-2.3,0-3.2L33.6,23.8z"/>
        </svg>`;
    }
    if(name == "double-up"){
        return html`<svg viewBox="0 0 64 64" class="svg-icon icon-double-up">
            <path class="opt" d="M33.6,25.6c-0.9-0.9-2.3-0.9-3.2,0l-23,22.6c-0.9,0.9-0.9,2.3,0,3.2c0.9,0.9,2.3,0.9,3.2,0L32,30.4l21.4,21.1
                c0.4,0.4,1,0.6,1.6,0.6c0.6,0,1.2-0.2,1.6-0.7c0.9-0.9,0.9-2.3,0-3.2L33.6,25.6z"/>
            <path d="M10.6,38.4L32,17.4l21.4,21.1c0.4,0.4,1,0.6,1.6,0.6c0.6,0,1.2-0.2,1.6-0.7c0.9-0.9,0.9-2.3,0-3.2l-23-22.6
                c-0.9-0.9-2.3-0.9-3.2,0l-23,22.6c-0.9,0.9-0.9,2.3,0,3.2C8.3,39.3,9.7,39.3,10.6,38.4z"/>
        </svg>`;
    }
    if(name == "question"){
        return html`<svg viewBox="0 0 64 64" class="svg-icon icon-circle-question">
            <path class="opt" d="M32,1.8C15.3,1.8,1.8,15.3,1.8,32S15.3,62.3,32,62.3S62.3,48.7,62.3,32S48.7,1.8,32,1.8z M32,57.8
                C17.8,57.8,6.3,46.2,6.3,32C6.3,17.8,17.8,6.3,32,6.3c14.2,0,25.8,11.6,25.8,25.8C57.8,46.2,46.2,57.8,32,57.8z"/>
            <path d="M33.8,12.1c-2.9-0.5-5.9,0.3-8.1,2.2c-2.2,1.9-3.5,4.6-3.5,7.6c0,1.1,0.2,2.2,0.6,3.3c0.4,1.2,1.7,1.8,2.9,1.4
                c1.2-0.4,1.8-1.7,1.4-2.9c-0.2-0.6-0.3-1.2-0.3-1.8c0-1.6,0.7-3.1,1.9-4.1c1.2-1,2.8-1.5,4.5-1.2c2.1,0.4,3.9,2.2,4.3,4.3
                c0.4,2.5-0.9,5-3.2,6c-2.6,1.1-4.3,3.7-4.3,6.7v6.2c0,1.2,1,2.3,2.3,2.3c1.2,0,2.3-1,2.3-2.3v-6.2c0-1.1,0.6-2.1,1.5-2.5
                c4.3-1.8,6.8-6.3,6-10.9C41,16.1,37.8,12.8,33.8,12.1z"/>
            <path d="M32.1,45.8h-0.3c-1.2,0-2.3,1-2.3,2.3s1,2.3,2.3,2.3h0.3c1.2,0,2.2-1,2.2-2.3S33.4,45.8,32.1,45.8z"/>
        </svg>`;
    }
    if(name == "home"){
        return html`<svg viewBox="0 0 64 64" class="svg-icon icon-home">
            <path d="M61.2,21.2L35.4,4.6c-2.1-1.3-4.8-1.3-6.8,0L2.8,21.2c-1,0.7-1.3,2.1-0.7,3.1c0.7,1,2.1,1.3,3.1,0.7l1.7-1.1v30.1
                c0,3.5,2.8,6.3,6.3,6.3h37.6c3.5,0,6.3-2.8,6.3-6.3V23.9l1.7,1.1c0.4,0.2,0.8,0.4,1.2,0.4c0.7,0,1.5-0.4,1.9-1
                C62.6,23.3,62.3,21.9,61.2,21.2z M52.6,54.1c0,1-0.8,1.8-1.8,1.8H13.2c-1,0-1.8-0.8-1.8-1.8V21L31,8.4c0.6-0.4,1.4-0.4,2,0L52.6,21
                V54.1z"/>
            <path class="opt" d="M27.2,24.6c-2.2,0-4.3,0.9-5.8,2.4c-3.2,3.2-3.2,8.4,0,11.6l0.6,0.6c0,0,0,0,0,0l8.4,8.5c0.4,0.4,1,0.7,1.6,0.7
                s1.2-0.2,1.6-0.7l8.4-8.5c0,0,0,0,0,0l0.6-0.6c1.5-1.6,2.4-3.6,2.4-5.8c0-2.2-0.8-4.2-2.4-5.8c-1.5-1.6-3.6-2.4-5.8-2.4
                c0,0,0,0,0,0c-1.7,0-3.4,0.5-4.8,1.6C30.6,25.2,29,24.6,27.2,24.6z M34.2,30.2c0.7-0.7,1.6-1.1,2.6-1.1c0,0,0,0,0,0
                c1,0,1.9,0.4,2.6,1.1c0.7,0.7,1.1,1.6,1.1,2.6c0,1-0.4,1.9-1.1,2.7L32,43l-6.8-6.8l-0.6-0.6c-1.4-1.4-1.4-3.8,0-5.3
                c1.4-1.4,3.8-1.4,5.2,0l0.6,0.6c0.4,0.4,1,0.7,1.6,0.7h0c0.6,0,1.2-0.2,1.6-0.7L34.2,30.2z"/>
        </svg>`;
    }
    if(name == "hamburger"){
        return html`<svg viewBox="0 0 64 64" class="svg-icon icon-circle-hamburger">
            <path class="opt" d="M32.0008 1.80078C15.3008 1.80078 1.80078 15.3008 1.80078 32.0008C1.80078 48.7008 15.3008 62.3008 32.0008 62.3008C48.7008 62.3008 62.3008 48.7008 62.3008 32.0008C62.3008 15.3008 48.7008 1.80078 32.0008 1.80078ZM32.0008 57.8008C17.8008 57.8008 6.30078 46.2008 6.30078 32.0008C6.30078 17.8008 17.8008 6.30078 32.0008 6.30078C46.2008 6.30078 57.8008 17.9008 57.8008 32.1008C57.8008 46.2008 46.2008 57.8008 32.0008 57.8008Z"/>
            <path d="M42.1016 18.1016H21.9016C20.7016 18.1016 19.6016 19.1016 19.6016 20.4016C19.6016 21.7016 20.6016 22.7016 21.9016 22.7016H42.0016C43.2016 22.7016 44.3016 21.7016 44.3016 20.4016C44.3016 19.1016 43.3016 18.1016 42.1016 18.1016Z"/>
            <path d="M42.1016 29.8008H21.9016C20.7016 29.8008 19.6016 30.8008 19.6016 32.1008C19.6016 33.3008 20.6016 34.4008 21.9016 34.4008H42.0016C43.2016 34.4008 44.3016 33.4008 44.3016 32.1008C44.3016 30.8008 43.3016 29.8008 42.1016 29.8008Z"/>
            <path d="M42.1016 41.4004H21.9016C20.7016 41.4004 19.6016 42.4004 19.6016 43.7004C19.6016 45.0004 20.6016 46.0004 21.9016 46.0004H42.0016C43.2016 46.0004 44.3016 45.0004 44.3016 43.7004C44.3016 42.4004 43.3016 41.4004 42.1016 41.4004Z"/>
        </svg>`;
    }
    else{
        return html`<svg viewBox="0 0 64 64" class="svg-icon icon-not-found">
            <path class="opt" d="M32,1.8C15.3,1.8,1.8,15.3,1.8,32S15.3,62.3,32,62.3S62.3,48.7,62.3,32S48.7,1.8,32,1.8z M32,57.8
                C17.8,57.8,6.3,46.2,6.3,32C6.3,17.8,17.8,6.3,32,6.3c14.2,0,25.8,11.6,25.8,25.8C57.8,46.2,46.2,57.8,32,57.8z"/>
            <path d="M33.8,12.1c-2.9-0.5-5.9,0.3-8.1,2.2c-2.2,1.9-3.5,4.6-3.5,7.6c0,1.1,0.2,2.2,0.6,3.3c0.4,1.2,1.7,1.8,2.9,1.4
                c1.2-0.4,1.8-1.7,1.4-2.9c-0.2-0.6-0.3-1.2-0.3-1.8c0-1.6,0.7-3.1,1.9-4.1c1.2-1,2.8-1.5,4.5-1.2c2.1,0.4,3.9,2.2,4.3,4.3
                c0.4,2.5-0.9,5-3.2,6c-2.6,1.1-4.3,3.7-4.3,6.7v6.2c0,1.2,1,2.3,2.3,2.3c1.2,0,2.3-1,2.3-2.3v-6.2c0-1.1,0.6-2.1,1.5-2.5
                c4.3-1.8,6.8-6.3,6-10.9C41,16.1,37.8,12.8,33.8,12.1z"/>
            <path d="M32.1,45.8h-0.3c-1.2,0-2.3,1-2.3,2.3s1,2.3,2.3,2.3h0.3c1.2,0,2.2-1,2.2-2.3S33.4,45.8,32.1,45.8z"/>
        </svg>`;
    }

}

function Nav({onTop, onBottom, onDown, onUp, onMenu }){
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
    return html`<nav id="primary-nav">
            <ul>
                <li>
                    <a onClick=${onTop} title="top">
                        <${Icon} name="double-up" />
                    </a>
                </li>
                <li>
                    <a onClick=${onUp} title="up">
                        <${Icon} name="chevron-up" />
                    </a>
                </li>
                <li>
                    <a onClick=${onMenu} title="Menu">
                        <${Icon} name="hamburger" />
                    </a>
                </li>
                <li>
                    <a onClick=${onDown} title="down">
                        <${Icon} name="chevron-down" />
                    </a>
                </li>
                <li>
                    <a onClick=${onBottom} title="bottom">
                        <${Icon} name="double-down" />
                    </a>
                </li>
            </ul>
        </nav>`;
}

function FullNav({onMenu}){
    return html`<nav id="full-nav">
        <ul>
            <li>
                <a onClick=${onMenu} title="Menu">
                    <${Icon} name="hamburger" />
                </a>
            </li>
        </ul>
    </nav>`;
}

class App extends Component {

    constructor(props){
        super(props);
        this.data = props.data;
        let index = this.data.getIndex();

        this.lastScrollTop;
        this.state = {
            scrollDirection: "backward",
            expandedMenu: false,
            length: this.index.count,
            content: this.data.getContent(),
        }
        this.lastScrollTop = 0;
        this.lastNavInteraction = Date.now();
    }

    componentDidMount(){
        let element = this.base;
        let everything = element.querySelector('.everything-feed');
        everything.addEventListener('scroll', (e) => {
            let scrollTop = everything.scrollTop;
            let changeDirection = debounce((direction) => {
                this.setState({
                    scrollDirection: direction
                });
            })
            if(this.lastScrollTop > scrollTop && this.state.scrollDirection != "backward"){
                //console.warn("scrolling backward");
                changeDirection("backward");
            }
            else if(this.lastScrollTop < scrollTop && this.state.scrollDirection != "forward"){
                //console.warn("scrolling forward");
                changeDirection("forward");

                this.setState({
                    scrollDirection: "forward"
                });
            }
            else{
            }
            this.lastScrollTop = scrollTop;
        })
    }

    goToTop(){
        this.lastNavInteraction = Date.now();
        let element = this.base;
        let everything = element.querySelector('.everything-feed');
        everything.scrollTop = 0;
    }

    goToBottom(){
        this.lastNavInteraction = Date.now();
        let element = this.base;
        let everything = element.querySelector('.everything-feed');
        everything.scrollTop = everything.scrollHeight;
    }

    render(){
        let justBooted = Date.now() - bootupTime < 5000;
        let justInteracted = Date.now() - this.lastNavInteraction < 5000;
        let headerVisible = this.state.scrollDirection == "backward" ? "header-visible" : "header-invisible";
        if(justBooted || justInteracted){
            headerVisible = "header-visible";
        }
        let disableTransparentIcons = this.lastScrollTop > 60 ? "disable-transparent-icons" : "";

        let fullNavExpandedClass = this.state.expandedMenu ? "expanded" : "";
        let onMenu = () => {
            this.setState({
                expandedMenu: !this.state.expandedMenu
            });
        }

        let items = this.state.content.map((item) => {
            return html`<${VisibilityTrigger} />`;
        });

        return html`<div class="primary-card">
            <div class="content">
                <header id="primary-header" class="${headerVisible} ${disableTransparentIcons}">
                    <${Nav}
                        onTop=${this.goToTop.bind(this)}
                        onBottom=${this.goToBottom.bind(this)}
                        onMenu=${onMenu}
                    />
                </header>
                <header id="full-header" class="${fullNavExpandedClass} disable-transparent-icons">
                    <${FullNav}
                        onMenu=${onMenu}
                    />
                </header>
                <div class="everything-feed">
                    <h2>Hi!</h2>
                    <div class="frames">
                    ${items}
                    </div>
                </div>
            </div>
        </div>`
    }

}

console.log(hash128(rendered));

let Data = initialize({serverUrl: null});

// load the index
// determine where we are in the index, using the hash

async function main(){
    await Data.loadIndex({user: null, indexId: null, contentId: null});
    let app = html`<${App} data=${Data} />`;
    render(app, document.getElementById('app'));
}

main();