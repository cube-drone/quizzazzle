import { v4 as uuid } from "uuid";
import { assert } from "./assert.js";

// TODO: add a random delay to the stub server so that we can see what it's like to load content
let delay = (ms) => new Promise(resolve => setTimeout(resolve, ms));


let PAGE_SIZE = 100;

class StubServer{

    constructor(){
        this.stubContent = [];
        this.stubContentById = {};
        this.contentLength = 10000;

        for(let i = 0; i < this.contentLength; i++){
            let randomNode = this.generateRandomNode(i);
            this.stubContentById[randomNode.id] = randomNode;
            this.stubContent.push(randomNode);
        }

        this.index = {
            id: uuid(),
            userSlug: "cubes",
            contentSlug: "testyboy",
            name: 'Test Index',
            order: 'newest content first', // feeds use "newest content first", stories use "oldest content first"
            contentIds: this.stubContent.map(node => node.id),
            created_at: new Date(),
            updated_at: new Date(),
        }
    }

    generateRandomNode(order){
        // TODO: generate random notes in a variety of types (once we know how to display a variety of types of node)
        let id = uuid().slice(-6);
        return {
            id,
            order,
            type: 'markdown',
            content: `
## Node ${order}
_${id}_
This is markdown content!

* I gaze upon the roast,
* that is sliced and laid out
* on my plate,
* and over it
* I spoon the juices
* of carrot and onion.
* And for once I do not regret
* the passage of time.
            `,
            created_at: new Date(),
            updated_at: new Date(),
        }
    }

    async getIndexId({userSlug, contentSlug}){
        return this.index.id;
    }

    async getIndex({indexId}) {
        await delay(350);
        return this.index;
    }

    async getRange({indexId, startId, endId}){
        await delay(500);
        let startIndex, endIndex;
        if(startId){
            startIndex = this.stubContent.findIndex(node => node.id === startId);
        }
        if(endId){
            endIndex = this.stubContent.findIndex(node => node.id === endId);
        }
        if(startIndex && endIndex){
            return this.stubContent.slice(startIndex, endIndex);
        }
        else if(startIndex){
            return this.stubContent.slice(startIndex, startIndex + PAGE_SIZE/2);
        }
        else if(endIndex){
            return this.stubContent.slice(endIndex - PAGE_SIZE/2, endIndex);
        }
        else {
            return this.stubContent.slice(0, PAGE_SIZE);
        }
    }

    async getContent({indexId, contentId}){
        await delay(500);
        return this.stubContentById[contentId];
    }

    async getContents({indexId, contentIds}){
        await delay(500);
        return contentIds.map(contentId => this.stubContentById[contentId]);
    }
}

class RealServer{
    constructor({serverUrl}){
        this.serverUrl = serverUrl;
    }
}

class Data{
    constructor({server}){
        // the purpose of Data is to manage the stuff that we're pulling from the server
        this.server = server;
        this.index = {};
        // this.index.contentIds is a list of every ID of a node in the index
        this.fullyLoadedBakedPotato = false;
        this.content = {};
        this.currentLocation = 0;
        this.currentId = null;
        setTimeout(this.ping.bind(this), 2000);
    }

    async _addItem({node}){
        this.content[node.id] = node;
    }

    async _addItems(nodes){
        for(let node of nodes){
            this._addItem({node});
        }
    }

    async _loadEndCapItems(){
        let firstNodeId = this.index.contentIds[0];
        let lastNodeId = this.index.contentIds[this.index.contentIds.length - 1];
        if(this.content[lastNodeId] != null && this.content[lastNodeId] != null){
            // we already have the first and last nodes loaded
            return;
        }
        let [firstNode, secondNode, penultimateNode, lastNode] = await this.server.getContents({
            indexId: this.index.id,
            contentIds: [
                firstNodeId,
                this.index.contentIds[1],
                this.index.contentIds[this.index.contentIds.length - 2],
                lastNodeId
            ]
        });

        this._addItems([firstNode, secondNode, penultimateNode, lastNode]);

        assert(this.content[firstNodeId] != null, `first node ${firstNodeId} not loaded`);
        assert(this.content[firstNodeId].id === firstNodeId, `first node ${firstNodeId} not loaded properly`);

        assert(this.index.contentIds[0] == this.content[firstNodeId].id);
    }

    async _loadIndexFromBeginning({indexId}){
        let [index, afterRange] = await Promise.all([
            this.server.getIndex({indexId}),
            this.server.getRange({indexId}),
        ]);

        if(index == null){
            throw new Error(`Index ${indexId} not found`);
        }
        this.index = index;
        this.fullyLoadedBakedPotato = false;
        this._addItems([...afterRange]);

        if(index.count < PAGE_SIZE){
            // if the index is small enough, we could absolutely have loaded the whole thing in one go
            this.fullyLoadedBakedPotato = true;
        }
        else{
            await this._loadEndCapItems();
        }
    }

    async _loadIndexFromMiddle({indexId, contentId}){
        let [index, beforeRange, afterRange] = await Promise.all([
            this.server.getIndex({user, indexId}),
            this.server.getRange({user, indexId, endId: contentId}),
            this.server.getRange({user, indexId, startId: contentId}),
        ]);

        this.index = index;
        this.fullyLoadedBakedPotato = false;
        this._addItems([...beforeRange, ...afterRange]);

        if(this.index.count < PAGE_SIZE/2){
            // if the index is small enough, we could absolutely have loaded the whole thing in one go
            this.fullyLoadedBakedPotato = true;
        }
        else{
            await this._loadEndCapItems();
        }

        // we keep track of where the user is in the content so that we can load more content as they scroll
        this.currentLocation = this.content.findIndex(node => node.id === contentId);
    }

    async loadIndex({userSlug, contentSlug, contentId}){
        let indexId = await this.server.getIndexId({userSlug, contentSlug});

        if(contentId == null){
            return this._loadIndexFromBeginning({indexId});
        }
        else{
            return this._loadIndexFromMiddle({indexId, contentId});
        }
    }

    async loadMoreContent({user, indexId, contentId}){
        let [beforeRange, afterRange] = await Promise.all([
            this.server.getRange({user, indexId, endId: contentId}),
            this.server.getRange({user, indexId, startId: contentId}),
        ]);

        this._addItems([...beforeRange, ...afterRange]);
    }

    async setCurrentLocation(n){
        // set the current location in the content
        // this will be used to determine what content to load next
        this.currentLocation = n;
        this.currentId = this.index.contentIds[n];
    }

    async getCurrentLocation(){
        return this.currentLocation ?? 0;
    }

    async _findEmptyRange(){
        // find a range of content that we don't have

        if(this.index == null){
            return false;
        }
        if(this.fullyLoadedBakedPotato){
            return false;
        }

        // starting from "currentLocation" find the closest content that we don't have loaded yet
        let lookingAtBackward = this.currentLocation;
        let lookingAtForward = this.currentLocation;
        let counter = 0;

        while(lookingAtBackward > 0 && lookingAtForward < this.index?.count){
            if(lookingAtForward < this.index?.count && this.content[lookingAtForward] == null){
                return {
                    startId: this.content[lookingAtForward - 1]?.id,
                }
            }
            if(lookingAtBackward > 0 && this.content[lookingAtBackward] == null){
                return {
                    endId: this.content[lookingAtBackward + 1]?.id,
                }
            }

            lookingAtForward++;
            // implement a gentle bias towards looking forward in a very stupid way
            if(counter > 5){
                lookingAtBackward--;
            }
            else{
                counter++;
            }
        }
        // it's safe to assume at this point that we've loaded literally all of the content
        this.fullyLoadedBakedPotato = true;
        return false;
    }

    async loadSomeNearbyContent(){
        if(this.fullyLoadedBakedPotato){
            // we've already loaded all of the content
            return;
        }

        let range = await this._findEmptyRange();
        if(!range){
            return;
        }

        let freshContent = await this.server.getRange({indexId: this.index.id, ...range});
        this._addItems(...freshContent);
    }

    async ping(){
        await this.loadSomeNearbyContent();
        setTimeout(this.ping.bind(this), 2000);
    }

    getIndex(){
        return this.index;
    }

    async getContent({id}){
        if(this.content[id] == null){
            await this.loadMoreContent({indexId: this.index.id, contentId: id});
        }
        let content = this.content[id];
        assert(content != null, `content ${id} not found`);
        assert(content.id === id, `content ${id} not found properly`);
        assert(content.type != null, `content ${id} has no type`);
        return this.content[id];
    }

    getContentOrder(id){
        return this.index.contentIds.indexOf(id);
    }

    getNextContentId(){
        return this.index.contentIds[this.currentLocation + 1];
    }

    getPreviousContentId(){
        return this.index.contentIds[this.currentLocation - 1];
    }

}

export function initialize({serverUrl}={}){
    let server;
    if(serverUrl == null){
        server = new StubServer()
    }
    else{
        server = new RealServer({serverUrl})
    }

    return new Data({server});
}
