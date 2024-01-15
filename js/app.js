import { h, Component, render} from 'preact';
import { useState } from 'preact/hooks'
import htm from 'htm';
import { marked } from 'marked';
import insane from 'insane';
import { hash128 } from './lib/murmurhash3.js'
import { v4 as uuid } from "uuid";
import anime from 'animejs'

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

let index = [];
for(let i = 0; i < 100; i++){
    index.push({
        id: i,
        type: "text",
        content: `hi ${i}`
    });
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
    if(name == "chevron-up"){
        return html`<svg viewBox="0 0 64 64" class="svg-icon icon-circle-chevron-up">
            <path class="opt" d="M32,1.8C15.3,1.8,1.8,15.3,1.8,32S15.3,62.3,32,62.3S62.3,48.7,62.3,32S48.7,1.8,32,1.8z M32,57.8
                C17.8,57.8,6.3,46.2,6.3,32C6.3,17.8,17.8,6.3,32,6.3c14.2,0,25.8,11.6,25.8,25.8C57.8,46.2,46.2,57.8,32,57.8z"/>
            <path d="M33.6,23.8c-0.9-0.9-2.3-0.9-3.2,0L16.9,37c-0.9,0.9-0.9,2.3,0,3.2c0.4,0.4,1,0.7,1.6,0.7c0.6,0,1.1-0.2,1.6-0.6L32,28.5
                l11.9,11.7c0.9,0.9,2.3,0.9,3.2,0c0.9-0.9,0.9-2.3,0-3.2L33.6,23.8z"/>
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

function Nav({onTop, onBottom}){
    if(!onTop){
        onTop = () => {};
    }
    if(!onBottom){
        onBottom = () => {};
    }

    return html`<nav id="primary-nav">
        <ul>
            <li>
                <a id="end" onClick=${onBottom} title="end">
                    <${Icon} name="chevron-down" />
                </a>
            </li>
            <li>
                <a id="end" onClick=${onTop} title="beginning">
                    <${Icon} name="chevron-up" />
                </a>
            </li>
        </ul>
    </nav>`;

}
class App extends Component {

    constructor(props){
        super(props);
        this.lastScrollTop;
        this.state = {
            scrollDirection: "backward",
        }
        this.lastScrollTop = 0;
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
        let element = this.base;
        let everything = element.querySelector('.everything-feed');
        everything.scrollTop = 0;
    }

    goToBottom(){
        let element = this.base;
        let everything = element.querySelector('.everything-feed');
        everything.scrollTop = everything.scrollHeight;
    }

    render(){
        let justBooted = Date.now() - bootupTime < 5000;
        let headerVisible = this.state.scrollDirection == "backward" ? "header-visible" : "header-invisible";
        if(justBooted){
            headerVisible = "header-visible";
        }
        let disableTransparentIcons = this.lastScrollTop > 60 ? "disable-transparent-icons" : "";

        let items = index.map((item) => {
            return html`<${VisibilityTrigger} />`;
        });

        return html`<div class="primary-card">
            <div class="content">
                <header class="${headerVisible} ${disableTransparentIcons}">
                    <${Nav} onTop=${this.goToTop.bind(this)} onBottom=${this.goToBottom.bind(this)}/>
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

render(html`<${App} />`, document.getElementById('app'));