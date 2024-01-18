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
        order: order*100,
        type: 'markdown',
        content: `
            ## Node ${order}
            _${id}_
            This is markdown content!
        `,
        created_at: new Date(),
        updated_at: new Date(),
    }
}

let stubContent = [];
for(let i = 0; i < 100000; i++){
    stubContent.push(generateRandomNode(i));
}

let index = {
    id: uuid(),
    name: 'Test Index',
    order: 'newest content first', // feeds use "newest content first", stories use "oldest content first"
    count: 100000,
    created_at: new Date(),
    updated_at: new Date(),
    content: stubContent,
    firstNode: stubContent[0],
    lastNode: stubContent[stubContent.length - 1],
}

export class StubServer{

    async getIndex({user, indexId, contentId }) {
        if(contentId == null){
            // we want the index starting from the beginning
            let indexCopy = {
                id: index.id,
                name: index.name,
                order: index.order,
                created_at: index.created_at,
                updated_at: index.updated_at,
                content: index.content.slice(0, 100),
                firstNode: index.firstNode,
                lastNode: index.lastNode,
            }
            return indexCopy;
        }

        // where is the contentId in the index?
        // we want the 50 nodes before and after it
    }

    async get({user, contentId}){
        return {
            type: 'markdown',
            content: sampleMarkdown1,


        }

    }

    async get({user, slug}){

    }

    /*
    * Preload a content item: this fetches the content item from the
    * server and caches it locally.
    * if the content item requires rendering, it will render it in the background as well
    * notably, in stub mode, this doesn't do anything at all
    */
    async preload({user, slug}){

    }
}

export class RealServer{

}

export async function initialize({serverUrl}={}){
    if(serverUrl == null){
        return new StubServer()
    }
    else{
        return new RealServer({serverUrl})
    }
}