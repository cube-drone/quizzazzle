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
            console.warn(`showing: ${id}`);
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
            console.warn(`showing: ${id}`);
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

    hello(){
        console.warn("hello");
    }

    visible(){
        console.log("hi");
        this.setState({
            visible: true
        });
    }

    invisible(){
        console.log("bye");
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

let index = [
    {
        id: "1",
        type: "text",
        content: "hi"
    },
    {
        id: "2",
        type: "text",
        content: "hello"
    },
    {
        id: "3",
        type: "text",
        content: "hoyo"
    }
]

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
                console.warn("scrolling backward");
                changeDirection("backward");
            }
            else if(this.lastScrollTop < scrollTop && this.state.scrollDirection != "forward"){
                console.warn("scrolling forward");
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

    render(){
        let headerVisible = this.state.scrollDirection == "backward" ? "header-visible" : "header-invisible";
        let disableTransparentIcons = this.lastScrollTop > 60 ? "disable-transparent-icons" : "";

        return html`<div class="primary-card">
            <div class="content">
                <header class="${headerVisible} ${disableTransparentIcons}">
                    hi
                </header>
                <div class="everything-feed">
                    <h2>Hi!</h2>
                    <div class="frames">
                    <${VisibilityTrigger} />
                    <${VisibilityTrigger} />
                    <${VisibilityTrigger} />
                    </div>
                </div>
            </div>
        </div>`
    }

}

console.log(hash128(rendered));

render(html`<${App} />`, document.getElementById('app'));