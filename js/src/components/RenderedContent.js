
import { h, Component, render } from 'preact';
import { useEffect } from 'preact/hooks';
import htm from 'htm';

import { marked } from 'marked';
import insane from 'insane';

const html = htm.bind(h);

function TitleCard({card}){
    if(!card.title){
        return html`<div class="title-card">
            <h1>${card.id}</h1>
        </div>`;
    }
    else{
        return html`<div class="card title-card">
            <h1>${card.title}</h1>
        </div>`;
    }
}

function MarkdownCard({card}){
    return html`<div class="card markdown-card">
        <div class="markdown-content" dangerouslySetInnerHTML=${{ __html: insane(marked.parse(card.content)) }}></div>
    </div>`;
}

function HtmlCard({card}){
    return html`<div class="card html-card">
        <div class="markdown-content" dangerouslySetInnerHTML=${{ __html: insane(card.content) }}></div>
    </div>`;
}

function ImageCard({card}){
    return html`<div class="card image-card">
        <img src=${card.imageUrl} alt=${card.alt} title=${card.title}/>
    </div>`;
}

let animatedImageInterval = null;

function AnimatedImageCard({card, primary}){
    let imagesToCycleThrough = card.pngs;
    let fps = card.pngsFps ?? 24;
    let isLoop = card.pngsLoop;

    useEffect(() => {
        if(primary){
            // start the video
            clearInterval(animatedImageInterval);
            let images = this.base.querySelectorAll('img');
            let index = 0;
            animatedImageInterval = setInterval(() => {
                images.forEach((img, i) => {
                    if(i === index){
                        img.style.display = 'block';
                    }
                    else{
                        img.style.display = 'none';
                    }
                });
                index = (index + 1) % images.length;
                if(!isLoop && index === 0){
                    clearInterval(animatedImageInterval);
                }
            }, 1000 / fps);
        }
        else{
            clearInterval(animatedImageInterval);
        }

    }, [primary]);

    let images = imagesToCycleThrough.map((imageUrl, index) => {
        return html`<img src=${imageUrl} alt=${card.alt} title=${card.title} style="display: ${index === 0 ? 'block' : 'none'};"/>`;
    });

    return html`<div class="card animated-image-card">
        ${images}
    </div>`;
}

function VideoCard({card, primary}){

    // if primary is true, then the video should start playing automatically
    useEffect(() => {
        let video = this.base.querySelector('video');
        if(primary){
            video.play();
        }
        else{
            // reset the video
            video.currentTime = 0;
            video.pause();
        }
    }, [primary]);

    let loop = card.videoLoop ? "loop" : "";
    let muted = card.videoHasSound ? "" : "muted";
    let controls = card.videoControls ? "controls" : "";

    console.log(`video: ${loop} ${muted} ${controls}`);

    let videoType = card.videoUrl.split('.').pop();
    return html`<div class="card video-card">
        <video muted=${!card.videoHasSound} loop=${card.videoLoop} controls=${card.videoControls} playsinline="true" preload="true">
            <source src=${card.videoUrl} type="video/${videoType}" />
        </video>
    </div>`;
}

function ErrorCard({card}){
    return html`<div class="card error-card">
        <div class="error-content">
            <pre>
            <code>
                ${JSON.stringify(card, null, 2)}
            </code>
            </pre>
        </div>
    </div>`;
}

export default function RenderedContent({content, primary, visible}){
    let card = content;
    let cardClass = ErrorCard;
    if(card.type === 'markdown'){
        cardClass = MarkdownCard;
    }
    if(card.type === 'html'){
        cardClass = HtmlCard;
    }
    if(card.type === 'title'){
        cardClass = TitleCard
    }
    if(card.type === 'image'){
        cardClass = ImageCard;
    }
    if(card.type === 'video'){
        cardClass = VideoCard;
    }
    if(card.type === 'pngs'){
        cardClass = AnimatedImageCard;
    }
    return html`<div class="rendered-content">
        <${cardClass} card=${card} primary=${primary} visible=${visible}/>
    </div>`;
}