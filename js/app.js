import { h, Component, render} from 'preact';
import { useState } from 'preact/hooks'
import htm from 'htm';
import { marked } from 'marked';
import insane from 'insane';
import { hash128 } from './lib/murmurhash3.js'
import { v4 as uuid } from "uuid";
import { Icon } from './src/components/Icon.js';
import { initialize } from './src/data.js';

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

class RenderedContent extends Component {
    constructor(props){
        super(props);
        //console.dir(Object.keys(props.content));
        console.dir(props);
        let {id, order, type, content, created_at, updated_at} = props.content;
        this.state = {
            id,
            order,
            type,
            content,
            created_at,
            updated_at
        }
    }

    render(){
        let {type, content} = this.state;

        let maybeContent = "";
        if(content){
            maybeContent = html`<div class="frame-content" dangerouslySetInnerHTML=${{ __html: insane(marked.parse(content)) }}></div>`;
        }

        return html`<div class="rendered-content" class="frame-${type}">
            ${maybeContent}
        </div>`;
    }
}

class VisibilityTrigger extends Component {
    constructor(props){
        super(props);
        this.data = props.data;
        this.order = props.order;
        this.state = {
            visible: false,
            primary: false,
            id: this.props.id
        }
    }

    async visible(){
        this.setState({
            visible: true
        });
        let node = await this.data.getContent({id: this.state.id});
        this.setState({node})
    }

    invisible(){
        this.setState({
            visible: false
        });
    }

    async primary(){
        this.data.setCurrentLocation(this.order);
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
        let frameClass = "";
        let node = this.state.node;
        if(this.state.primary){
            frameClass = "frame-primary";
        }
        else if(this.state.visible){
            frameClass = "frame-visible";
        }
        else{
            frameClass = "frame-invisible";
        }

        let maybeContent = "";
        if(node){
            maybeContent = html`<${RenderedContent} content=${node} />`;
        }

        return html`<div class="frame ${frameClass}">
            ${maybeContent}
        </div>`;
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
        this.index = this.data.getIndex();

        this.lastScrollTop = 0;
        this.lastNavInteraction = Date.now();

        this.state = {
            scrollDirection: "backward",
            expandedMenu: false,
            index: this.index,
            length: this.index.count,
        }
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

        let items = this.state.index.contentIds.map((id, n) => {
            return html`<${VisibilityTrigger} data=${this.data} order=${n} id=${id}/>`;
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