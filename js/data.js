import { v4 as uuid } from "uuid";

let sampleMarkdown1 = `
# This is a header
`

let sampleMarkdown2 = `
# this is also a header
`

function generateRandomNode(order){
    let id = uuid();
    return {
        id,
        order,
        type: 'markdown',
        content: `
            ## Node ${order}
            _${id}_
            This is markdown content!

            I gaze upon the roast,

            that is sliced and laid out

            on my plate,

            and over it

            I spoon the juices

            of carrot and onion.

            And for once I do not regret

            the passage of time.
        `,
        created_at: new Date(),
        updated_at: new Date(),
    }
}

let stubContent = [];
let contentLength = 1000;
for(let i = 0; i < contentLength; i++){
    stubContent.push(generateRandomNode(i));
}

let index = {
    id: uuid(),
    name: 'Test Index',
    order: 'newest content first', // feeds use "newest content first", stories use "oldest content first"
    count: stubContent.length,
    firstNode: stubContent[0],
    secondNode: stubContent[1],
    penultimateNode: stubContent[stubContent.length - 2],
    lastNode: stubContent[stubContent.length - 1],
    created_at: new Date(),
    updated_at: new Date(),
}

let pageSize = 100;

class StubServer{

    async getIndex({user, indexId }) {
        return index;
    }

    async getRange({user, indexId, startId, endId}){
        let startIndex, endIndex;
        if(startId){
            startIndex = stubContent.findIndex(node => node.id === startId);
        }
        if(endId){
            endIndex = stubContent.findIndex(node => node.id === endId);
        }
        if(startIndex && endIndex){
            return stubContent.slice(startIndex, endIndex);
        }
        else if(startIndex){
            return stubContent.slice(startIndex, startIndex + pageSize/2);
        }
        else if(endIndex){
            return stubContent.slice(endIndex - pageSize/2, endIndex);
        }
        else {
            return stubContent.slice(0, pageSize);
        }
    }
}

class RealServer{

}

class Data{
    constructor({server}){
        // the purpose of Data is to manage the stuff that we're pulling from the server
        this.server = server;
        this.index = {};
        this.fullyLoadedBakedPotato = false;
        this.content = [];
        this.contentById = {};
        this.currentLocation = 0;
    }

    async _addItem({node}){
        let nodeIndex = parseInt(node.order);
        if(!isNaN(nodeIndex)){
            this.content[nodeIndex] = node;
        }
        this.contentById[node.id] = node;
    }

    async _addItems(nodes){
        for(let node of nodes){
            this._addItem({node});
        }
    }

    async _loadIndexFromBeginning({user, indexId}){
        let {index, afterRange} = await Promise.all([
            this.server.getIndex({user, indexId}),
            this.server.getRange({user, indexId}),
        ]);

        this.index = index;
        this.fullyLoadedBakedPotato = false;
        if(this.index.count < pageSize){
            // if the index is small enough, we could absolutely have loaded the whole thing in one go
            this.fullyLoadedBakedPotato = true;
        }

        this._addItems([index.firstNode, index.secondNode, index.penultimateNode, index.lastNode, ...afterRange]);
    }

    async _loadIndexFromMiddle({user, indexId, contentId}){
        let {index, beforeRange, afterRange} = await Promise.all([
            this.server.getIndex({user, indexId}),
            this.server.getRange({user, indexId, endId: contentId}),
            this.server.getRange({user, indexId, startId: contentId}),
        ]);

        this.index = index;
        this.fullyLoadedBakedPotato = false;
        if(this.index.count < pageSize/2){
            // if the index is small enough, we could absolutely have loaded the whole thing in one go
            this.fullyLoadedBakedPotato = true;
        }

        this._addItems([index.firstNode, index.secondNode, index.penultimateNode, index.lastNode, ...beforeRange, ...afterRange]);

        this.currentLocation = this.content.findIndex(node => node.id === contentId);
    }

    async loadIndex({user, indexId, contentId}){
        if(contentId == null){
            return this._loadIndexFromBeginning({user, indexId});
        }
        else{
            return this._loadIndexFromMiddle({user, indexId, contentId});
        }
    }

    async loadMoreContent({user, indexId, contentId}){
        let {beforeRange, afterRange} = await Promise.all([
            this.server.getRange({user, indexId, endId: contentId}),
            this.server.getRange({user, indexId, startId: contentId}),
        ]);

        this._addItems([...beforeRange, ...afterRange]);
    }

    async setCurrentLocation({indexInContent}){
        // set the current location in the content
        // this will be used to determine what content to load next
        this.currentLocation = indexInContent;
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

        let freshContent = await this.server.getRange({user, indexId, ...range});
        this._addItems(...freshContent);
    }

    async getIndex(){
        return this.index;
    }

    async getContentById({id}){
        if(this.contentById[id] == null){
            await this.loadMoreContent({user, indexId, contentId: id});
        }
        return this.contentById[id];
    }

    async getContentByOrder({order}){
        if(this.content[order] == null){
            await this.loadMoreContent({user, indexId, contentId: id});
        }
        return this.content[order];
    }

    async getContent(){
        return this.content;
    }

}

export async function initialize({serverUrl}={}){
    let server;
    if(serverUrl == null){
        server = new StubServer()
    }
    else{
        server = new RealServer({serverUrl})
    }

    return new Data({server});
}
