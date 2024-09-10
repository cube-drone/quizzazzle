
import { h, Component, render } from 'preact';
import { useEffect, useState } from 'preact/hooks';
import htm from 'htm';
import anime from 'animejs';

import { marked } from 'marked';
import insane from 'insane';

const html = htm.bind(h);


function AnyCard({card, cardType, stackIndex, primary, visible, children}){
    let easing = card.easing ?? 'easeInOutQuad';
    if(card.fadeIn){
        useEffect(() => {
            if(primary){
                let el = this.base;
                anime({targets: el, opacity: [0, 1], duration: 500, delay: card.delay ?? card.fadeIn ?? 0, easing});
            }
        }, [primary]);
    }
    if(card.shake){
        useEffect(() => {
            if(primary){
                let el = this.base;
                let duration = card.shake ?? card.duration ?? 500;
                let amount = card.amount ?? 5;
                let translateX = [];
                translateX.push(0);
                for(let i = 0; i < duration / 100; i++){
                    translateX.push(i % 2 === 0 ? amount : -amount);
                }
                translateX.push(0);
                anime({targets: el, translateX, duration, delay: card.delay ?? 0, easing, loop: card.loop});
            }
        }, [primary]);
    }
    let z = stackIndex != null ? `z-index:${stackIndex};color:pink;` : "";
    return html `<div style=${z} class="card ${cardType}-card any-card ${stackIndex ? "stacked" : ""} ${card.extraClass.join(" ")}">
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

function PanDownCard({card, stackIndex, primary, visible}){
    let [animation, setAnimation] = useState(null);
    let easing = card.easing ?? 'easeInOutQuad';
    useEffect(() => {
        let el = this.base.querySelector('img');
        if(primary){
            if(!animation){
                let y = card.amount ?? 400;
                let duration = card.duration ?? 5000;
                let animation = anime({targets: el, translateY: -y, duration, easing, loop: card.loop, delay: card.delay ?? 0});
                setAnimation(animation);
            }
            animation?.play();
        }
        else{
            animation?.restart();
            animation?.pause();
        }
    }, [primary]);

    return html`<${AnyCard} card=${card} cardType="pan-down" stackIndex=${stackIndex} primary=${primary} visible=${visible}>
        <img src=${card.imageUrl} alt=${card.alt} title=${card.title}/>
    </${AnyCard}>`;
}

function PanLeftCard({card, stackIndex, primary, visible}){
    let [animation, setAnimation] = useState(null);
    let easing = card.easing ?? 'easeInOutQuad';
    useEffect(() => {
        let el = this.base.querySelector('img');
        if(primary){
            if(!animation){
                let x = card.amount ?? 330;
                let duration = card.duration ?? 5000;
                let animation = anime({targets: el, translateX: -x, duration, easing, loop: card.loop, delay: card.delay ?? 0});
                setAnimation(animation);
            }
            animation?.play();
        }
        else{
            animation?.restart();
            animation?.pause();
        }
    }, [primary]);

    return html`<${AnyCard} card=${card} cardType="pan-left" stackIndex=${stackIndex} primary=${primary} visible=${visible}>
        <img src=${card.imageUrl} alt=${card.alt} title=${card.title}/>
    </${AnyCard}>`;
}

function PanUpCard({card, stackIndex, primary, visible}){
    let [animation, setAnimation] = useState(null);
    let easing = card.easing ?? 'easeInOutQuad';
    useEffect(() => {
        let el = this.base.querySelector('img');
        if(primary){
            if(!animation){
                let duration = card.duration ?? 5000;
                let animation = anime({targets: el, translateY: 0, duration, easing, loop: card.loop, delay: card.delay ?? 0});
                setAnimation(animation);
            }
            animation?.play();
        }
        else{
            animation?.restart();
            animation?.pause();
        }
    }, [primary]);

    let initialStyle = `transform: translateY(-${card.amount ?? 400}px);`;

    return html`<${AnyCard} card=${card} cardType="pan-up" stackIndex=${stackIndex} primary=${primary} visible=${visible}>
        <img style=${initialStyle} src=${card.imageUrl} alt=${card.alt} title=${card.title}/>
    </${AnyCard}>`;
}

function PanRightCard({card, stackIndex, primary, visible}){
    let [animation, setAnimation] = useState(null);
    let easing = card.easing ?? 'easeInOutQuad';
    useEffect(() => {
        let el = this.base.querySelector('img');
        if(primary){
            if(!animation){
                let duration = card.duration ?? 5000;
                let animation = anime({targets: el, translateX: 0, duration, easing, loop: card.loop, delay: card.delay ?? 0});
                setAnimation(animation);
            }
            animation?.play();
        }
        else{
            animation?.restart();
            animation?.pause();
        }
    }, [primary]);

    let initialStyle = `transform: translateX(-${card.amount ?? 300}px);`;

    return html`<${AnyCard} card=${card} cardType="pan-right" stackIndex=${stackIndex} primary=${primary} visible=${visible}>
        <img style=${initialStyle} src=${card.imageUrl} alt=${card.alt} title=${card.title}/>
    </${AnyCard}>`;
}


function AnimatedImageCard({card, primary, visible, stackIndex}){
    let [animatedImageInterval, setAnimatedImageInterval] = useState(null);
    let imagesToCycleThrough = card.pngs;
    let fps = card.pngsFps ?? 24;
    let isLoop = card.loop;

    useEffect(() => {
        if(primary){
            // start the video
            clearInterval(animatedImageInterval);
            let images = this.base.querySelectorAll('img');
            let index = 0;
            setTimeout(() => {
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
                setAnimatedImageInterval(animatedImageInterval);
            }, card.delay ?? 0)
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

function AnimatedTextCard({card, primary, visible, stackIndex}){
    let textToAnimate = card.content;
    let fps = card.fps ?? 24;

    let [animatedTextInterval, setAnimatedTextInterval] = useState(null);

    useEffect(() => {
        let characters = this.base.querySelectorAll('span');
        if(primary){
            // start the video
            setTimeout(() => {
                clearInterval(animatedTextInterval);
                let index = 0;
                animatedTextInterval = setInterval(() => {
                    characters.forEach((char, i) => {
                        if(i === index){
                            char.style.opacity = '1';
                        }
                    });
                    index = (index + 1) % (characters.length+20);
                    if(!card.loop && index === 0){
                        clearInterval(animatedTextInterval);
                    }
                    else if (index === 0){
                        characters.forEach((char, i) => {
                            char.style.opacity = '0';
                        });
                    }
                }, 1000 / fps);
                setAnimatedTextInterval(animatedTextInterval);
            }, card.delay ?? 0);
        }
        else{
            characters.forEach((char, i) => {
                char.style.opacity = '0';
            });
            clearInterval(animatedTextInterval);
        }

    }, [primary]);

    let textSeparated = textToAnimate.split('').map((char, index) => {return html`<span style="opacity: 0;">${char}</span>`;});

    return html`<${AnyCard} card=${card} cardType="animated-text" stackIndex=${stackIndex} primary=${primary} visible=${visible}>
        <div class="animated-text-content">
            ${textSeparated}
        </div>
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

    let loop = card.loop ? "loop" : "";
    let muted = card.videoHasSound ? "" : "muted";
    let controls = card.videoControls ? "controls" : "";

    console.log(`video: ${loop} ${muted} ${controls}`);

    let videoType = card.videoUrl.split('.').pop();

    return html`<${AnyCard} card=${card} cardType="video" stackIndex=${stackIndex} primary=${primary} visible=${visible}>
        <video muted=${!card.videoHasSound} loop=${card.loop} controls=${card.videoControls} playsinline="true" preload="true">
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
    if(type === 'pan_down' || type === "pan-down"){
        cardClass = PanDownCard;
    }
    if(type === 'pan_up' || type === "pan-up"){
        cardClass = PanUpCard;
    }
    if(type === 'pan_left' || type === "pan-left"){
        cardClass = PanLeftCard;
    }
    if(type === 'pan_right' || type === "pan-right"){
        cardClass = PanRightCard;
    }
    if(type === "animated_text" || type === "animated-text"){
        cardClass = AnimatedTextCard;
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
    return html`<${AnyCard} card=${card} cardType="stack" stackIndex=${stackIndex} primary=${primary} visible=${visible}>
        ${card.stack.map((c, index) => {
            let cardClass = typeToCardClass(c.type);
            let newStackIndex = (stackIndex ?? 0 * 100) + index + 1;
            return html`<${cardClass} card=${c} primary=${primary} visible=${visible} stackIndex=${newStackIndex} />`;
        })}
    </${AnyCard}>`;
}

export default function RenderedContent({content, primary, visible}){
    let card = content;
    let cardClass = typeToCardClass(card.type);
    return html`<div class="rendered-content">
        <${cardClass} card=${card} primary=${primary} visible=${visible}/>
    </div>`;
}