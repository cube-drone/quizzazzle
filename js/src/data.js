import { assert } from "./assert.js";


let PAGE_SIZE = 100;

class RealServer{
    constructor({serverUrl}){
        this.serverUrl = serverUrl;
        this.index = null;
    }

    async getIndexId({userSlug, contentSlug}){
        console.warn(`getting index id for ${userSlug}/${contentSlug}`);
        if(userSlug == null || contentSlug == null){
            const response = await fetch(`${this.serverUrl}/index`, {});
            this.index = await response.json();
            return `/s/default/default`;
        }
        return `/s/${userSlug}/${contentSlug}`;
    }

    indexTransform(serverIndex){
        console.dir(serverIndex);
        let appIndex = {
            id: serverIndex.id,
            userSlug: serverIndex.metadata.author_slug,
            authorSlug: serverIndex.metadata.author_slug,
            contentSlug: serverIndex.metadata.slug,
            author: serverIndex.metadata.author,
            authorLink: serverIndex.metadata.author_link,
            name: serverIndex.metadata.title,
            description: serverIndex.metadata.description,
            thumbnailImageUrl: serverIndex.metadata.image_url,
            locale: serverIndex.metadata.locale,
            contentIds: serverIndex.deck_ids || [],
            toc: serverIndex.toc || [],
            mp3: serverIndex.metadata.mp3,
            updatedAt: new Date(serverIndex?.metadata?.last_update_time?.secs_since_epoch * 1000),
            updatedAtTimestamp: serverIndex?.metadata?.last_update_time?.secs_since_epoch,
        }
        console.dir(appIndex);
        return appIndex;
    }

    async getIndex({indexId}){
        if(this.index == null){
            const response = await fetch(`${this.serverUrl}${indexId}/index`, {});
            this.index = await response.json();
        }

        return this.indexTransform(this.index);
    }

    cardTransform(card){
        let appCard = {
            id: card.id,
            title: card.title,
            type: card.card_type || "title",
            extraClass: card.extra_class,
            containerClass: card.container_class,

            content: card.content,

            imageUrl: card.image_url,

            videoUrl: card.video_url,
            videoHasSound: card.video_has_sound,
            videoControls: card.video_controls,

            loop: card.is_loop,

            pngs: card.pngs,
            pngsFps: card.pngs_fps,

            fadeIn: card.fade_in,
            fadeOut: card.fade_out,
            shake: card.shake,
            panLeft: card.pan_left,
            panRight: card.pan_right,
            panUp: card.pan_up,
            panDown: card.pan_down,
            dollyIn: card.dolly_in,
            dollyOut: card.dolly_out,
            spinClockwise: card.spin_clockwise,

            duration: card.duration,
            amount: card.amount,
            delay: card.delay,
            easing: card.easing,
            animateContainer: card.animate_container,

            next: card.next,

            stack: card.stack.map(this.cardTransform.bind(this)),
            tocDepth: card.toc_depth,
        }
        return appCard;
    }

    async getRange({indexId, startId, endId}){
        if(startId == null){
            startId = 0;
        }
        if(endId == null){
            endId = 0
        }
        const response = await fetch(`${this.serverUrl}${indexId}/range/${startId}/${endId}`, {});
        let cards = await response.json();
        return cards.map(this.cardTransform.bind(this));
    }

    async getContent({indexId, contentId}){
        console.warn(`getting: ${indexId} / ${contentId}`)
        const response = await fetch(`${this.serverUrl}${indexId}/content/${contentId}`, {});
        let card = await response.json();
        return this.cardTransform.bind(this)(card);
    }

    async getContents({indexId, contentIds}){
        let contents = [];
        for (let contentId of contentIds){
            let content = await this.getContent({indexId, contentId});
            console.warn(content);
        }
        return contents;
    }

    async getSitemap(){
        let response = await fetch(`${this.serverUrl}/sitemap`, {});
        return await response.json();
    }

}

class Data{
    /*
        Okay, so, "Data" is kind of a special thing:
        It's not the same thing as RealServer or StubServer, which are classes which are used to interact with the server.
        "Data's" job is to keep track of the state of the data that we've loaded from the server -
            like, if we've got 30 nodes loaded, and there are 10 blank nodes after that, and then there are 30 more nodes loaded?
            the RealServer/StubServer system is where we go to find out what's ON those nodes,
            but Data is responsible for keeping track of what nodes we've loaded and what nodes we haven't loaded.
    */
    constructor({server}){
        this.server = server;
        this.index = null;
        this.indexId = null;
        // this.index.contentIds is a list of every ID of a node in the index

        // fullyLoadedBakedPotato is set once we have _all_ of the content loaded. At this point there's a lot less work for Data to do.
        //  however! if the story is too large, we might want to start _unloading_ content that's too far away from the current location.
        this.fullyLoadedBakedPotato = false;

        // this.content is a dictionary of every node that we've loaded so far, indexed by their ID
        this.content = {};

        // currentLocation is the index of the node that we're currently looking at
        this.currentLocation = 0;
        // currentId is the ID of the node that we're currently looking at
        this.currentId = null;

        // while you're staring at the page, we keep loading content in the background
        setTimeout(this.ping.bind(this), 2000);

        let sitemap;

        if(sitemap){
            console.log("loading sitemap from cache");
            this.sitemap = JSON.parse(sitemap);
        }
        else{
            this.server.getSitemap().then(sitemap => {
                this.sitemap = sitemap;
            });
        }

    }

    async _addItem({node}){
        this.content[node.id] = node;
    }

    async _addItems(nodes){
        console.log("adding items");
        console.dir(nodes);
        for(let node of nodes.filter(node => node != null)){
            this._addItem({node});
        }
    }

    async _loadEndCapItems(){
        // because there are buttons to skip to the very first and very last node, we need to make sure that we have those nodes loaded
        let firstNodeId = this.index.contentIds[0];
        let lastNodeId = this.index.contentIds[this.index.contentIds.length - 1];
        if(this.content[lastNodeId] != null && this.content[lastNodeId] != null){
            // we already have the first and last nodes loaded
            return;
        }
        let [firstNode, secondNode, penultimateNode, lastNode] = await this.server.getContents({
            indexId: this.indexId,
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

        let index = this.index;

        let afterRange = await this.server.getRange({indexId});

        if(index == null){
            throw new Error(`Index ${indexId} not found`);
        }
        this.index = index;

        this.fullyLoadedBakedPotato = false;
        this._addItems([...afterRange]);

        if(index.count < PAGE_SIZE){
            // if the index is small enough, we could absolutely have loaded the whole thing in one go
            console.log("index is small enough to load all at once");
            this.bakePotato();
        }
        else{
            await this._loadEndCapItems();
        }
    }

    async _loadIndexFromMiddle({indexId, contentId}){
        contentId = contentId.replace("#", "");

        let index = this.index;
        let indexOfContent = index.contentIds.indexOf(contentId);
        let startOfPageIndex = Math.max(0, indexOfContent - PAGE_SIZE/2);
        let startId = index.contentIds[startOfPageIndex];

        let [beforeRange, afterRange] = await Promise.all([
            this.server.getRange({indexId, startId, endId: contentId}),
            this.server.getRange({indexId, startId: contentId}),
        ]);

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
        this.currentLocation = 0;
        for(let i = 0; i < this.index.contentIds.length; i++){
            if(this.index.contentIds[i] === contentId){
                this.currentLocation = i;
                break;
            }
        }
        this.currentId = contentId;
    }

    async loadIndex({userSlug, contentSlug, contentId}){
        // userSlug+contentSlug are a pair of strings that identify the index that we're looking at
        // so, for example, "cubes/testyboy" is an index that belongs to the user "cubes" and is called "testyboy"
        // the index describes the whole story, in order, it's like a table of contents

        let indexId = await this.server.getIndexId({userSlug, contentSlug});
        this.indexId = indexId;
        console.warn(`got index id ${indexId}`);

        console.warn(`loading index from server`);
        this.index = await this.server.getIndex({indexId});

        if(contentId == null || contentId == ""){
            return this._loadIndexFromBeginning({indexId});
        }
        else{
            return this._loadIndexFromMiddle({indexId, contentId});
        }
    }

    async loadMoreContent({user, indexId, contentId}){

        if(this.fullyLoadedBakedPotato){
            return;
        }

        let index = this.index;
        let indexOfContent = index.contentIds.indexOf(contentId);
        let startOfPageIndex = Math.max(0, indexOfContent - PAGE_SIZE/2);
        let startId = index.contentIds[startOfPageIndex];

        let [beforeRange, afterRange] = await Promise.all([
            this.server.getRange({user, indexId, startId, endId: contentId}),
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

    bakePotato(){
        this.fullyLoadedBakedPotato = true;
        console.log("baking the potato");
    }

    async loadSomeNearbyContent(){
        if(this.fullyLoadedBakedPotato){
            // we've already loaded all of the content
            return;
        }

        for(let i = 0; i < this.index.contentIds.length; i++){
            let id = this.index.contentIds[i];
            if(this.content[id] == null){
                // we haven't loaded this content yet!
                console.log(`loading content ${id}`);
                await this.loadMoreContent({indexId: this.indexId, contentId: id});
                return;
            }
        }

        // if we've gotten here, we've loaded all of the content
        console.log("there's no more content to load");
        this.bakePotato();
    }

    async ping(){
        await this.loadSomeNearbyContent();
        setTimeout(this.ping.bind(this), 2000);
    }

    getIndex(){
        if(this.index == null){
            throw new Error("Index not loaded");
        }
        return this.index;
    }

    async getContent({id}){
        if(this.content[id] == null){
            await this.loadMoreContent({indexId: this.indexId, contentId: id});
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

    getSitemap(){
        return this.sitemap;
    }

}

export function initialize({serverUrl}={}){
    let server = new RealServer({serverUrl})

    return new Data({server});
}
