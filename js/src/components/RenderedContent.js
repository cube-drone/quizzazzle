
import { h, Component, render } from 'preact';
import { useEffect, useState } from 'preact/hooks';
import htm from 'htm';
import anime from 'animejs';

import { Marked } from 'marked';

const html = htm.bind(h);

function markdownify(md){
    let markyMark = new Marked();
    // if you wanted to use plugins, you'd do it here, like this:
    // markyMark.use(plugin1).use(plugin2);
    // if you wanted to sanitize the HTML (when are we going to have untrusted markdown?),
    // you'd do it here, like this:
    // return insane(markyMark.parse(md));

    return markyMark.parse(md);
}

function AnyCard({card, cardType, stackIndex, primary, visible, children}){

    let [animation, setAnimation] = useState(null);

    let style = stackIndex != '' ? `z-index:${stackIndex};` : "";

    // animation stuffs
    let isAnimation = false;
    if(card.fadeIn || card.shake){
        animation = true;
    }
    let opacity = null;
    let translateX = null;
    let translateY = null;
    let rotation = null;
    let scale = null;
    let easing = card.easing ?? 'easeInOutQuad';
    let duration = card.duration ?? 500;
    let delay = card.delay ?? 0;
    let restrictMaxWidth = true;
    let restrictMaxHeight = true;
    let animStyle = [];

    if(card.fadeIn){
        isAnimation = true;
        if(!isNaN(card.fadeIn)){
            delay = card.fadeIn;
        }
        animStyle.push(`opacity: 0;`);
        opacity = [0, 1];
    }
    if(card.fadeOut){
        isAnimation = true;
        if(!isNaN(card.fadeOut)){
            delay = card.fadeOut;
        }
        if(!card.fadeIn){
            animStyle.push(`opacity: 1;`);
        }
        if(opacity == null){
            opacity = [];
        }
        opacity = opacity.concat([1, 0]);
    }
    if(card.shake){
        isAnimation = true;
        if(!isNaN(card.shake)){
            duration = card.shake;
        }
        let amount = card.amount ?? 5;
        translateX = [];
        translateX.push(0);
        for(let i = 0; i < duration / 100; i++){
            translateX.push(i % 2 === 0 ? amount : -amount);
        }
        translateX.push(0);
    }
    if(card.verticalShake){
        isAnimation = true;
        if(!isNaN(card.shakeY)){
            duration = card.shakeY;
        }
        let amount = card.amount ?? 5;
        translateY = [];
        translateY.push(0);
        for(let i = 0; i < duration / 100; i++){
            translateY.push(i % 2 === 0 ? amount : -amount);
        }
        translateY.push(0);
    }
    if(card.jitter){
        // a jitter is a shake, but with randomized amounts
        isAnimation = true;
        if(!isNaN(card.jitter)){
            duration = card.jitter;
        }
        let amount = card.amount ?? 5;
        translateX = [];
        translateX.push(0);
        for(let i = 0; i < duration / 50; i++){
            translateX.push((Math.random() * amount * 2) - amount);
        }
        translateX.push(0);
    }
    if(card.verticalJitter){
        // a jitter is a shake, but with randomized amounts
        isAnimation = true;
        if(!isNaN(card.jitter)){
            duration = card.verticalJitter;
        }
        let amount = card.amount ?? 5;
        translateY = [];
        translateY.push(0);
        for(let i = 0; i < duration / 50; i++){
            translateY.push((Math.random() * amount * 2) - amount);
        }
        translateY.push(0);
    }
    if(card.panLeft){
        isAnimation = true;
        translateX = -card.panLeft;
        duration = card.duration ?? 5000;
        restrictMaxWidth = false;
    }
    if(card.panRight){
        isAnimation = true;
        translateX = 0;
        duration = card.duration ?? 5000;
        amount = card.panRight ?? 300;
        animStyle.push(`transform: translateX(-${amount}px);`);
        restrictMaxWidth = false;
    }
    if(card.panDown){
        isAnimation = true;
        translateY = -card.panDown;
        duration = card.duration ?? 5000;
        restrictMaxHeight = false;
    }
    if(card.panUp){
        isAnimation = true;
        translateY = 0;
        duration = card.duration ?? 5000;
        amount = card.panUp ?? 400;
        animStyle.push(`${style} transform: translateY(-${amount}px);`);
        restrictMaxHeight = false;
    }
    if(card.dollyIn){
        isAnimation = true;
        scale = card.dollyIn;
    }
    if(card.dollyOut){
        isAnimation = true;
        scale = card.dollyOut;
    }
    if(card.spinClockwise){
        isAnimation = true;
        rotation = card.spinClockwise;
        animStyle.push(`${style} transform: rotate(${rotation});`);
    }

    if(isAnimation){
        useEffect(() => {
            if(primary){
                //console.dir(card);
                // el is not this.base, but the child of this.base
                let el;
                if(card.animateContainer){
                    el = this.base;
                }
                else{
                    el = this.base.querySelector('.animation-frame');
                }

                if(animation && animation.remove != null){
                    animation?.remove(el);
                }
                let anim = {targets: el, duration, delay, easing};
                if(opacity != null){
                    anim.opacity = opacity;
                }
                if(translateX != null){
                    anim.translateX = translateX;
                }
                if(translateY != null){
                    anim.translateY = translateY;
                }
                if(scale != null){
                    anim.scale = scale;
                }
                if(rotation != null){
                    anim.rotate = rotation;
                }

                console.dir(anim);

                let createdAnimation = anime(anim);
                setAnimation(createdAnimation);
                createdAnimation?.play();
            }
            else{
                if(animation && animation.restart != null && animation.pause != null){
                    animation?.restart();
                    animation?.pause();
                }
            }
        }, [primary]);
    }

    let restrictions = [];

    if(restrictMaxWidth){
        restrictions.push("restrict-max-width");
    }
    if(restrictMaxHeight){
        restrictions.push("restrict-max-height");
    }

    if(card.animateContainer){
        style = style.concat(animStyle);
        animStyle = [];
    }

    let footnote = null;
    if(card.footnote){
        footnote = html`<div class="footnote">
            <div class="markdown-content" dangerouslySetInnerHTML=${{ __html: markdownify(card.footnote) }}></div>
        </div>`;
    }

    return html `<div style=${style} class="card ${cardType}-card any-card ${stackIndex ? "stacked" : ""} ${card.containerClass.join(" ")} ${restrictions.join(" ")}">
        <div style=${animStyle.join(" ")} class="animation-frame ${card.extraClass.join(" ")}">
        ${children}
        </div>
        ${footnote}
    </div>`;
}

function TitleCard({card, stackIndex, primary, visible}){
    return html`<${AnyCard} card=${card} cardType="title" stackIndex=${stackIndex} primary=${primary} visible=${visible}>
        <h1>${card.title ?? card.id}</h1>
    </${AnyCard}>`;
}

function MarkdownCard({card, stackIndex, primary, visible}){
    return html`<${AnyCard} card=${card} cardType="markdown" stackIndex=${stackIndex} primary=${primary} visible=${visible}>
        <div class="markdown-content" dangerouslySetInnerHTML=${{ __html: markdownify(card.content) }}></div>
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

function BasicTextAnimation({text, next, fps, wave, bounce, jitter, fadeIn, rainbow, cursor, color, style, className, em, strong}){
    let [animatedTextInterval, setAnimatedTextInterval] = useState(null);

    useEffect(() => {
        let characters = this.base.querySelectorAll('span');
        setTimeout(() => {
            clearInterval(animatedTextInterval);
            let index = 0;
            if(cursor){
                let cursor = this.base.querySelector('.cursor');
                let a = anime({
                    targets: cursor,
                    opacity: [1, 0],
                    duration: 500,
                    easing: 'linear',
                    loop: true
                })
                a.play();
            }

            animatedTextInterval = setInterval(() => {
                characters.forEach((char, i) => {
                    if(i === index){
                        char.style.display = 'inline';
                        if(fadeIn){
                            let a = anime({
                                targets: char,
                                opacity: [0, 1],
                                duration: 1000,
                                easing: 'linear'
                            });
                            a.play();
                        }
                        else{
                            char.style.opacity = '1';
                        }

                        if(wave){
                            char.style.display = 'inline-block';
                            char.style.minWidth = '0.25em';
                            let a = anime({
                                targets: char,
                                translateY: [-3, 3, -3, 3, -3, 3, 0],
                                duration: 5000,
                                easing: 'easeInOutQuad'
                            });
                            a.play();
                        }

                        if(bounce){
                            char.style.display = 'inline-block';
                            char.style.minWidth = '0.25em';
                            let a = anime({
                                targets: char,
                                translateY: [-3, 3, -3, 3, -3, 3, -3, 3, -3, 3, -3, 3, -3, 3, -3, 3, -3, 3, 0],
                                duration: 5000,
                                easing: 'easeInOutBounce'
                            });
                            a.play();
                        }

                        if(jitter){
                            char.style.display = 'inline-block';
                            char.style.minWidth = '0.25em';
                            let yTranslations = [];
                            for(let i = 0; i < 20; i++){
                                yTranslations.push(Math.random() * 4 - 2);
                            }
                            yTranslations.push(0);
                            let a = anime({
                                targets: char,
                                translateY: yTranslations,
                                duration: 1500,
                                easing: 'easeInOutBack'
                            });
                            a.play();
                        }

                        if(rainbow){
                            let a = anime({
                                targets: char,
                                color: ['#ff0000', '#ff7f00', '#ffff00', '#00ff00', '#0000ff', '#4b0082', '#8b00ff'],
                                duration: 1000,
                                easing: 'linear',
                                loop: true
                            });
                            a.play();
                        }
                    }
                });
                index = (index + 1) % (characters.length+1);
                if(index === 0){
                    clearInterval(animatedTextInterval);
                    if(cursor){
                        // once the animation is done, hide the cursor
                        let cursor = this.base.querySelector('.cursor');
                        if(cursor){
                            cursor.style.display = 'none';
                        }
                    }
                    next();
                }
            }, 1000 / fps);
            setAnimatedTextInterval(animatedTextInterval);
        }, 0);

    }, []);

    let styleExtras = '';
    if(color){
        styleExtras += `color: ${color};`;
    }
    if(cursor){
        styleExtras += `display: none;`;
    }
    if(em){
        styleExtras += `font-style: italic;`;
    }
    if(strong){
        styleExtras += `font-weight: bold;`;
    }
    if(style){
        styleExtras += `${style};`;
    }

    let textSeparated = text.split('').map((char, index) => {return html`<span class=${className} style="opacity: 0;${styleExtras}">${char}</span>`;});

    let cursy = '';
    if(cursor){
        cursy = html`<span class="cursor" style="opacity: 1;${styleExtras}">_</span>`;
    }

    return html`
        <span class="basic-text-animation">
            ${textSeparated}
            ${cursy}
        </span>
    `;
}

function LineBreakAnimation({next}){
    // every line break implies a little pause
    let lineBreakMs = 750;
    useEffect(() => {
        setTimeout(() => {
            next();
        }, lineBreakMs);
    }, []);

    return html`<br />`;
}

function NbspAnimation({next, fps}){
    useEffect(() => {
        setTimeout(() => {
            next();
        }, 1000 / fps);
    }, []);
    let nbsp = String.fromCharCode(160);

    return html`<span>${nbsp}</span>`;
}

function TabAnimation({next, fps}){
    useEffect(() => {
        setTimeout(() => {
            next();
        }, 1000 / fps);
    }, []);

    let nbsp = String.fromCharCode(160);
    return html`<span>${nbsp}${nbsp}${nbsp}${nbsp}</span>`;
}

function DelayAnimation({next, delay}){
    useEffect(() => {
        setTimeout(() => {
            next();
        }, delay);
    }, []);

    return null;
}

function ComplexTextAnimation({node, next, fps, primary, visible, delay=0, wave, bounce, jitter, fadeIn, rainbow, cursor, color, strong, em, style, className}){
    // call next() when done

    let [currentIndex, setCurrentIndex] = useState(1);
    let [active, setActive] = useState(false);
    let animations = [];

    useEffect(() => {
        if(primary){
            setTimeout(() => {
                setCurrentIndex(1);
                setActive(true);
            }, delay);
        }
        else{
            setCurrentIndex(0);
            setActive(false);
        }

    }, [primary]);

    function newNext(){
        setCurrentIndex(currentIndex + 1);
        if(currentIndex === animations.length){
            next();
        }
    }

    let counter = 0;
    for(let child of node.childNodes){
        let key = `anim-${counter++}`;
        let _wave = wave;
        if(child.nodeName === 'wave' || child.nodeName === "wavy" || (child.getAttribute && child.getAttribute('wave'))){
            _wave = true;
        }
        let _bounce = bounce;
        if(child.nodeName === 'bounce' || (child.getAttribute && child.getAttribute('bounce'))){
            _bounce = true;
        }
        let _jitter = jitter;
        if(child.nodeName === 'jitter' || (child.getAttribute && child.getAttribute('jitter'))){
            _jitter = true;
        }
        let _fadeIn = fadeIn;
        if(child.nodeName === 'fade' || (child.getAttribute && child.getAttribute('fade'))){
            _fadeIn = true;
        }
        let _rainbow = rainbow;
        if(child.nodeName === 'rainbow' || (child.getAttribute && child.getAttribute('rainbow'))){
            _rainbow = true;
        }
        let _cursor = cursor;
        if(child.nodeName === 'cursor' || (child.getAttribute && child.getAttribute('cursor'))){
            _cursor = true;
        }
        let _color = color;
        if(child.nodeName === 'color'){
            if(child.getAttribute){
                _color = child.getAttribute('value') ?? 'white';
            }
            else{
                _color = 'white';
            }
        }
        if(child.getAttribute && child.getAttribute('color')){
            _color = child.getAttribute('color');
        }
        let _style = style;
        if(child.nodeName === 'style'){
            if(child.getAttribute){
                _style = `${style};${child.getAttribute('value') ?? ''}`;
            }
        }
        if(child.getAttribute && child.getAttribute('style')){
            _style = `${style};${child.getAttribute('style')}`;
        }
        let _class = className;
        if(child.getAttribute && child.getAttribute('class')){
            _class = `${className} ${child.getAttribute('class') ?? ''}`;
        }

        let _em = em;
        if(child.nodeName === 'em'){
            _em = true;
        }
        let _strong = strong;
        if(child.nodeName === 'strong'){
            _strong = true;
        }

        let _fps = fps;
        if(child.nodeName === 'slow'){
            _fps = fps / 2;
        }
        else if(child.nodeName === 'slower'){
            _fps = fps / 4;
        }
        else if(child.nodeName === 'slowest'){
            _fps = fps / 8;
        }
        else if(child.nodeName === 'fast'){
            _fps = fps * 2;
        }
        else if(child.nodeName === 'faster'){
            _fps = fps * 4;
        }
        else if(child.nodeName === 'fastest'){
            _fps = fps * 8;
        }

        let complex = false;
        if(child.nodeName !== '#text'){
            for(let c of child.childNodes){
                if(c.nodeName !== '#text'){
                    complex = true;
                }
            }
            if(child.nodeName === "div"){
                complex = true;
            }
        }

        if(child.nodeName === 'br'){
            animations.push(html`<${LineBreakAnimation} next=${newNext} fps=${_fps} key=${key} />`);
            continue;
        }
        if(child.nodeName === 'nbsp'){
            animations.push(html`<${NbspAnimation} next=${newNext} fps=${_fps} key=${key} />`);
            continue;
        }
        if(child.nodeName === 'tab'){
            animations.push(html`<${TabAnimation} next=${newNext} fps=${_fps} key=${key} />`);
            continue;
        }
        else if(child.nodeName === 'beat'){
            let delayAmount = child.getAttribute('ms') ?? 750;
            animations.push(html`<${DelayAnimation} next=${newNext} delay=${delayAmount} key=${key} />`);
            continue;
        }
        else if(child.nodeName === 'delay'){
            let delayAmount = child.getAttribute('ms') ?? 1500;
            animations.push(html`<${DelayAnimation} next=${newNext} delay=${delayAmount} key=${key} />`);
            continue;
        }
        else if(child.nodeName === 'pause'){
            let delayAmount = child.getAttribute('ms') ?? 3000;
            animations.push(html`<${DelayAnimation} next=${newNext} delay=${delayAmount} key=${key} />`);
            continue;
        }
        else if(complex){

            let tempStyle, tempClass, tempColor;
            if(child.nodeName === 'div'){
                // if it's a div, then we don't want to pass the style, class, or color to the children
                //   (depend on the div's bubblin')
                tempStyle = _style;
                _style = null;
                tempClass = _class;
                _class = null;
                tempColor = _color;
                _color = null;
            }


            let animation = html`<${ComplexTextAnimation}
                node=${child}
                next=${newNext}
                fps=${_fps}
                primary=${primary}
                visible=${visible}
                wave=${_wave}
                bounce=${_bounce}
                jitter=${_jitter}
                fadeIn=${_fadeIn}
                rainbow=${_rainbow}
                cursor=${_cursor}
                color=${_color}
                strong=${_strong}
                em=${_em}
                style=${_style}
                className=${_class}
                key=${key} />`;

            if(child.nodeName === 'div'){
                if(tempColor){
                    tempStyle = `${tempStyle};color:${tempColor}`;
                }
                animations.push(html`<div class="complex-animation" style=${tempStyle} class=${tempClass}>${animation}</div>`);
            }
            else{
                animations.push(animation);
            }
        }
        else{
            animations.push(html`<${BasicTextAnimation}
                text=${child.textContent}
                next=${newNext}
                fps=${_fps}
                wave=${_wave}
                bounce=${_bounce}
                jitter=${_jitter}
                fadeIn=${_fadeIn}
                rainbow=${_rainbow}
                cursor=${_cursor}
                color=${_color}
                strong=${_strong}
                em=${_em}
                style=${_style}
                className=${_class}
                key=${key} />`);
        }
    }

    let visibleAnimations = animations.slice(0, currentIndex);

    if(!active){
        return null;
    }
    return html`
        <span class="complex-animation">
            ${visibleAnimations}
        </span>
    `;
}

function AnimatedTextCard({card, primary, visible, stackIndex}){
    let fps = card.fps ?? 24;

    let parsedXml = new DOMParser().parseFromString(`<animation>${card.content}</animation>`, 'text/xml');

    if(parsedXml.documentElement.nodeName === 'parsererror'){
        console.error('Error parsing XML');
        return html`<${ErrorCard} message="Error parsing Animation XML" card=${card} stackIndex=${stackIndex} primary=${primary} visible=${visible} />`;
    }

    function done(){
        console.log('done');
    }

    return html`<${AnyCard} card=${card} cardType="animated-text" stackIndex=${stackIndex} primary=${primary} visible=${visible}>
        <div class="animated-text-content">
            <${ComplexTextAnimation} node=${parsedXml.childNodes[0]} fps=${fps} next=${done} primary=${primary} visible=${visible} delay=${card.delay ?? 0} />
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

function ErrorCard({card, message, stackIndex, primary, visible}){

    return html`<${AnyCard} card=${card} cardType="error" stackIndex=${stackIndex} primary=${primary} visible=${visible}>
        <h4>Error</h4>
        <p>${message}</p>
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