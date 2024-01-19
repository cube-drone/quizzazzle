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
    created_at: new Date(),
    updated_at: new Date(),
    content: stubContent,
    firstNode: stubContent[0],
    secondNode: stubContent[1],
    secondToLastNode: stubContent[stubContent.length - 2],
    lastNode: stubContent[stubContent.length - 1],
}

class StubServer{

    async getIndex({user, indexId, contentId }) {
        if(contentId == null){
            // we want the index starting from the beginning
            let indexCopy = {
                ...index,
                content: index.content.slice(0, 100),
                currentIndex: 0,
            }
            return indexCopy;
        }
        else{
            // find the contentId in the index
            // return the 50 nodes before and after it
            let contentIndex = index.content.findIndex(node => node.id === contentId);
            let indexCopy = {
                ...index,
                content: index.content.slice(contentIndex - 50, contentIndex + 50),
                currentIndex: contentIndex,
            }
            return indexCopy;
        }
    }
}

class RealServer{

}

export async function initialize({serverUrl}={}){
    if(serverUrl == null){
        return new StubServer()
    }
    else{
        return new RealServer({serverUrl})
    }
}

class Data{
    constructor({server}){
        this.server = server;
    }

    async getIndex({user, indexId, contentId}){
        return this.server.getIndex({user, indexId, contentId});
    }

}
