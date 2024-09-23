import { h, Component, render} from 'preact';
import htm from 'htm';

const html = htm.bind(h);

export default function AudioPlayer({mp3, onTimeUpdate}){
    if(!mp3){
        return null;
    }

    let onPlay = (_evt) => {
        console.log("Playing audio");
    }

    let onPause = (_evt) => {
        console.log("Pausing audio");
    }

    let _onTimeUpdate = (evt) => {
        onTimeUpdate(Math.floor(evt.target.currentTime * 1000));
    }

    let onEnded = (_evt) => {
        console.log("Audio ended");
    }

    return html`<div class="audio-footer">
        <audio controls preload onPlay=${onPlay} onPause=${onPause} onTimeUpdate=${_onTimeUpdate} onEnded=${onEnded}>
            <source src="${mp3}" type="audio/mpeg" />
        </audio>
    </div>`;
}