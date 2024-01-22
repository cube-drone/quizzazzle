import { h, Component, render} from 'preact';
import htm from 'htm';
import RenderedContent from './RenderedContent.js';

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

export default class VisibilityTriggerFrame extends Component {
    /*
        The purpose of the VisibilityTriggerFrame is to provide a way to
        trigger events when a frame is visible or invisible. This is
        useful for triggering animations and other effects.
    */
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