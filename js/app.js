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


const html = htm.bind(h);

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
        let {id, order, type, content, created_at, updated_at, primary, visible} = props.content;
        this.state = {
            id,
            order,
            type,
            content,
            created_at,
            updated_at,
            primary,
            visible
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
        let noop = () => {};
        this.onPrimary = props.onPrimary ?? noop;
        this.onUnprimary = props.onUnprimary ?? noop;
        this.onVisible = props.onVisible ?? noop;
        this.onInvisible = props.onInvisible ?? noop;

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
        this.onVisible();
    }

    invisible(){
        this.setState({
            visible: false
        });
        this.onInvisible();
    }

    async primary(){
        this.data.setCurrentLocation(this.order);
        this.setState({
            primary: true
        });
        this.onPrimary();
    }

    unprimary(){
        this.setState({
            primary: false
        });
        this.onUnprimary();
    }

    componentDidMount(){
        let element = this.base;
        this.id = this.props.id ?? uuid();
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
        if(node && (this.state.primary || this.state.visible)){
            maybeContent = html`<${RenderedContent} content=${node} primary=${this.state.primary} visible=${this.state.visible}/>`;
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
            currentlySelected: null,
            currentlySelectedOrder: 0
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

        window.onkeyup = (e) => {
            let key = e.key;
            if (key === 'ArrowUp') {
                this.goUpOne();
            }
            if (key === 'ArrowDown') {
                this.goDownOne();
            }
        }
    }

    goToTop(){
        this.lastNavInteraction = Date.now();
        let element = this.base;
        let everything = element.querySelector('.everything-feed');
        everything.scrollTop = 0;
    }

    moveTo({id}){
        let element = document.getElementById(id);
        console.warn(`moving to ${id}`);
        console.warn(element);
        element.scrollIntoView({
            behavior: 'smooth',
            block: 'center',
        });
        this.setCurrentlySelected(id, this.data.getContentOrder(id));
    }

    goUpOne(){
        this.lastNavInteraction = Date.now();
        let upOneId = this.data.getPreviousContentId();
        if(upOneId){
            this.moveTo({id: upOneId});
        }
        else{
            console.warn("no previous content");
        }
    }

    goToBottom(){
        this.lastNavInteraction = Date.now();
        let element = this.base;
        let everything = element.querySelector('.everything-feed');
        everything.scrollTop = everything.scrollHeight;
    }

    goDownOne(){
        this.lastNavInteraction = Date.now();
        let downOneId = this.data.getNextContentId();
        if(downOneId){
            this.moveTo({id: downOneId});
        }
        else{
            console.warn("no next content");
        }
    }

    setCurrentlySelected(id, n){
        this.setState({
            currentlySelected: id,
            currentlySelectedOrder: n
        });
        // if this is an important header/toc content, we should pushState,
        // otherwise we should just replaceState
        history.replaceState(null, null, `#${id}`);
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
            let select = () => {
                this.setCurrentlySelected(id, n);
            }
            return html`<${VisibilityTrigger} data=${this.data} order=${n} id=${id} onPrimary=${select}/>`;
        });

        return html`<div class="primary-card">
            <div class="content">
                <header id="primary-header" class="${headerVisible} ${disableTransparentIcons}">
                    <${Nav}
                        onTop=${this.goToTop.bind(this)}
                        onBottom=${this.goToBottom.bind(this)}
                        onDown=${this.goDownOne.bind(this)}
                        onUp=${this.goUpOne.bind(this)}
                        onMenu=${onMenu}
                    />
                </header>
                <header id="full-header" class="${fullNavExpandedClass} disable-transparent-icons">
                    <${FullNav}
                        onMenu=${onMenu}
                    />
                </header>
                <div class="everything-feed">
                    <div class="frames">
                    ${items}
                    </div>
                </div>
            </div>
        </div>`
    }

}

let Data = initialize({serverUrl: null});

// load the index
// determine where we are in the index, using the hash
// userSlug and contentSlug are determined by the two parts of the URL leading up to this file, like:
//   https://example.com/userSlug/contentSlug#contentId
// so we can get the userSlug and contentSlug from the URL

let userSlug = null;
let contentSlug = null;
let contentId = null;
let path = window.location.pathname;
let hash = window.location.hash;

let pathParts = path.split('/');

async function main(){
    await Data.loadIndex({user: null, indexId: null, contentId: null});
    let app = html`<${App} data=${Data} />`;
    render(app, document.getElementById('app'));
}

main();