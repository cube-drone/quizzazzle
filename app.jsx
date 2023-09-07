import * as React from 'react'
import { createRoot } from "react-dom/client";
import * as Server from 'react-dom/server'
import Carousel from 'react-elastic-carousel'

let Card = ({key, children, ham}) => <div key={key} class={`card card-${ham}`}>
    <div class="content">
        {children}
    </div>
</div>

let Null = () => <div></div>

let Greet = () => {
    let items = [];
    for(let i = 0; i < 100; i++){
        items.push(i)
    }
    let counter = 0;
    return <div onWheel={(e)=>{console.warn(e.deltaY)}}>
        <Carousel itemsToShow={1} renderArrow={Null} renderPagination={Null}>
            {items.map((item) => <Card key={counter++} ham={counter++}>{item}</Card>)}
        </Carousel>
    </div>
}
console.log("Marts ahoy")

const root = createRoot(document.getElementById("app"));
root.render(
  <React.StrictMode>
    <Greet />
  </React.StrictMode>
);
