
export default function thumbnailify({imageUrl, height, width}){
    let url = new URL(imageUrl);
    url.searchParams.set('width', width);
    url.searchParams.set('height', height);
    return url.toString();
}