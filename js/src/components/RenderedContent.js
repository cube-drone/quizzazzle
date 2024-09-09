
import { h, Component, render } from 'preact';
import { useEffect } from 'preact/hooks';
import htm from 'htm';
import anime from 'animejs';

import { marked } from 'marked';
import insane from 'insane';

const html = htm.bind(h);


function AnyCard({card, cardType, stackIndex, primary, visible, children}){
    if(card.fadeIn){
        useEffect(() => {
            if(primary){
                let el = this.base;
                anime({targets: el, opacity: [0, 1], duration: 500, delay: card.fadeIn, easing: 'easeInOutQuad'});
            }
        }, [primary]);
    }
    let z = stackIndex != null ? `z-index:${stackIndex};color:pink;` : "";
    return html `<div style=${z} class="card ${cardType}-card any-card ${stackIndex ? "stacked" : ""}">
        ${children}
    </div>`;
}

function TitleCard({card, stackIndex, primary, visible}){
    return html`<${AnyCard} card=${card} cardType="title" stackIndex=${stackIndex} primary=${primary} visible=${visible}>
        <h1>${card.title ?? card.id}</h1>
    </${AnyCard}>`;
}

function MarkdownCard({card, stackIndex, primary, visible}){
    return html`<${AnyCard} card=${card} cardType="markdown" stackIndex=${stackIndex} primary=${primary} visible=${visible}>
        <div class="markdown-content" dangerouslySetInnerHTML=${{ __html: insane(marked.parse(card.content)) }}></div>
    </${AnyCard}>`;
}

function HtmlCard({card, stackIndex, primary, visible}){
    return html`<${AnyCard} card=${card} cardType="html" stackIndex=${stackIndex} primary=${primary} visible=${visible}>
        <div class="html-content" dangerouslySetInnerHTML=${{ __html: insane(card.content) }}></div>
    </${AnyCard}>`;
}

function ImageCard({card, stackIndex, primary, visible}){
    return html`<${AnyCard} card=${card} cardType="image" stackIndex=${stackIndex} primary=${primary} visible=${visible}>
        <img src=${card.imageUrl} alt=${card.alt} title=${card.title}/>
    </${AnyCard}>`;
}

let animatedImageInterval = null;

function AnimatedImageCard({card, primary, visible, stackIndex}){
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

    return html`<${AnyCard} card=${card} cardType="animated-image" stackIndex=${stackIndex} primary=${primary} visible=${visible}>
        ${images}
    </${AnyCard}>`;
}

function VideoCard({card, primary, visible, stackIndex}){

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

    return html`<${AnyCard} card=${card} cardType="video" stackIndex=${stackIndex} primary=${primary} visible=${visible}>
        <video muted=${!card.videoHasSound} loop=${card.videoLoop} controls=${card.videoControls} playsinline="true" preload="true">
            <source src=${card.videoUrl} type="video/${videoType}" />
        </video>
    </${AnyCard}>`;
}

function ErrorCard({card, stackIndex, primary, visible}){

    return html`<${AnyCard} card=${card} cardType="error" stackIndex=${stackIndex} primary=${primary} visible=${visible}>
        <div class="error-content">
            <pre>
            <code>
                ${JSON.stringify(card, null, 2)}
            </code>
            </pre>
        </div>
    </${AnyCard}>`;
}

function typeToCardClass(type){
    let cardClass = ErrorCard;
    if(type === 'markdown'){
        cardClass = MarkdownCard;
    }
    if(type === 'html'){
        cardClass = HtmlCard;
    }
    if(type === 'title'){
        cardClass = TitleCard
    }
    if(type === 'image'){
        cardClass = ImageCard;
    }
    if(type === 'video'){
        cardClass = VideoCard;
    }
    if(type === 'pngs'){
        cardClass = AnimatedImageCard;
    }
    if(type === 'stack'){
        cardClass = StackedCard;
    }
    return cardClass;
}

function StackedCard({card, primary, visible, stackIndex}){
    return html`<div class="card stacked-card">
        ${card.stack.map((c, index) => {
            let cardClass = typeToCardClass(c.type);
            let newStackIndex = (stackIndex ?? 0 * 100) + index + 1;
            return html`<${cardClass} card=${c} primary=${primary} visible=${visible} stackIndex=${newStackIndex} />`;
        })}
    </div>`;
}

export default function RenderedContent({content, primary, visible}){
    let card = content;
    let cardClass = typeToCardClass(card.type);
    return html`<div class="rendered-content">
        <${cardClass} card=${card} primary=${primary} visible=${visible}/>
    </div>`;
}