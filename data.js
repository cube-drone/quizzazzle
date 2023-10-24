let sampleMarkdown1 = `
# This is a header
`

let sampleMarkdown2 = `
# this is also a header
`

export class StubServer{

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