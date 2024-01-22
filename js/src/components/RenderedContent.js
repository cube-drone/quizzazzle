
import { h, Component, render} from 'preact';
import htm from 'htm';

import { marked } from 'marked';
import insane from 'insane';

const html = htm.bind(h);

export default class RenderedContent extends Component {
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
