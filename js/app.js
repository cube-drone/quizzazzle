import { h, Component, render} from 'preact';
import { useState } from 'preact/hooks'
import htm from 'htm';
import { initialize } from './src/data.js';
import VisibilityTriggerFrame from './src/components/VisibilityTriggerFrame.js';
import Nav from './src/components/Nav.js';
import NavDropdown from './src/components/NavDropdown.js';


function debounce(func, timeout = 300){
    let timer;
    return (...args) => {
      clearTimeout(timer);
      timer = setTimeout(() => { func.apply(this, args); }, timeout);
    };
}

const html = htm.bind(h);

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

        this.initialElement = props.initialElement;

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


        if(this.initialElement){
            console.warn("initial element is set: ", this.initialElement);
            this.moveTo({id: this.initialElement.replace("#", "")});
            window.onload = () => {
                console.warn("initial element is set: ", this.initialElement);
                this.moveTo({id: this.initialElement.replace("#", "")});
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
        /*
        // we used to go to quite a lot of trouble to determine whether or not the header should be visible
        let justBooted = Date.now() - bootupTime < 5000;
        let justInteracted = Date.now() - this.lastNavInteraction < 5000;
        let headerVisible = this.state.scrollDirection == "backward" ? "header-visible" : "header-invisible";
        if(justBooted || justInteracted){
            headerVisible = "header-visible";
        }
        */
        let headerVisible = "header-visible";

        // if we scroll down a bit, the transparent icons will show content THROUGH them, which looks wonky
        let disableTransparentIcons = this.lastScrollTop > 60 ? "disable-transparent-icons" : "";

        let fullNavExpandedClass = this.state.expandedMenu ? "expanded" : "";
        let onMenu = () => {
            this.setState({
                expandedMenu: !this.state.expandedMenu
            });
        }

        let navigateTo = (id) => {
            this.moveTo({id});
            this.setState({
                expandedMenu: false
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
                    <${NavDropdown}
                        onMenu=${onMenu.bind(this)}
                        navigateTo=${navigateTo.bind(this)}
                        data=${this.data}
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

if(!window.location.pathname.endsWith('/')){
    window.location = `${window.location.origin}${window.location.pathname}/${window.location.hash}`;
}

async function main(){

    if(window.location.pathname == "/"){
        let hash = window.location.hash;
        await Data.loadIndex({userSlug: null, contentSlug: null, contentId: hash});
    }
    else{
        // we're at /s/userSlug/contentSlug
        let parts = window.location.pathname.split('/');
        let userSlug = parts[2];
        let contentSlug = parts[3];
        let hash = window.location.hash;
        console.warn(`loading index for s/${userSlug}/${contentSlug}#${hash}`);
        await Data.loadIndex({userSlug: userSlug, contentSlug: contentSlug, contentId: hash});
    }
    let app = html`<${App} data=${Data} initialElement=${window.location.hash} />`;
    render(app, document.getElementById('app'));
}

main();