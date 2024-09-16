import { h, Component, render} from 'preact';
import htm from 'htm';
import thumbnailify from '../thumbnailify.js';

const html = htm.bind(h);


export default function TitleCard({index}){

    let updatedDate = index.updatedAt.toLocaleString();

    let thumbnailImage = null;
    if(index.thumbnailImageUrl){
        thumbnailImage = html`<img class="thumbnail"
                                    src="${thumbnailify({
                                        imageUrl: `${window.location.origin}${window.location.pathname}${index.thumbnailImageUrl}`,
                                        height: 250,
                                        width: 300})}"
                                    alt="${index.name}" />`;
    }

    let authorText = index.author;
    if(index.authorLink){
        authorText = html`<a href="${index.authorLink}">${index.author}</a>`;
    }

    return html`
        <div class="title-card">
            <div class="title-row">
                <h2>${index.name}</h2>
            </div>

            <div class="title-row">
                ${thumbnailImage}
                <a class="qrlink" href="/qr_html?link=${window.location.origin}${window.location.pathname}">
                    <img src="/qr?link=${window.location.origin}${window.location.pathname}" alt="QR Code" />
                </a>
            </div>

            <div class="title-row">
                <h4> ${authorText} - <small>${updatedDate}</small></h4>
                <p class="description">${index.description}</p>
            </div>
        </div>
    `;

}