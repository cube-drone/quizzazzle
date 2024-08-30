import { h, Component, render} from 'preact';
import { useState } from 'preact/hooks'
import htm from 'htm';
import Icon from './src/components/Icon.js';
import { initialize } from './src/data.js';
import VisibilityTriggerFrame from './src/components/VisibilityTriggerFrame.js';

function debounce(func, timeout = 300){
    let timer;
    return (...args) => {
      clearTimeout(timer);
      timer = setTimeout(() => { func.apply(this, args); }, timeout);
    };
}

let bootupTime = Date.now();

const html = htm.bind(h);

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
    /*
        This is the thing that appears when you click the hamburger button.
        Ideally, it'll have a Table of Contents, and maybe some other stuff?
        Credits? A link to the source code? A link to the user's profile?
        The world is our oyster.
    */
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
            if (key === 'ArrowUp' || key.toLowerCase() === "w") {
                this.goUpOne();
            }
            if (key === 'ArrowDown' || key.toLowerCase() === "s") {
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
        // we go to quite a lot of trouble to determine whether or not the header should be visible
        let justBooted = Date.now() - bootupTime < 5000;
        let justInteracted = Date.now() - this.lastNavInteraction < 5000;
        let headerVisible = this.state.scrollDirection == "backward" ? "header-visible" : "header-invisible";
        if(justBooted || justInteracted){
            headerVisible = "header-visible";
        }

        // if we scroll down a bit, the transparent icons will show content THROUGH them, which looks wonky
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
            return html`<${VisibilityTriggerFrame} data=${this.data} order=${n} id=${id} onPrimary=${select}/>`;
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

let serverUrl = window.location.origin;
let Data = initialize({serverUrl});

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