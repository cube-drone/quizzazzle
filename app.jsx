import * as React from 'react'
import { createRoot } from "react-dom/client";
import * as Server from 'react-dom/server'
import Carousel from 'react-elastic-carousel'

let Card = ({key, children, ham, hasPrev, slidePrev, hasNext, slideNext}) => <div key={key} class={`card card-${ham}`}>
    <div class="content">
        {children}
        {hasPrev && <button onClick={slidePrev}>Prev</button>}
        {hasNext && <button onClick={slideNext}>Next</button>}
    </div>
</div>

let Null = () => <div></div>

let Quiz = () => {
    let [index, setIndex] = React.useState(0);
    let items = [];
    for(let i = 0; i < 100; i++){
        items.push(i)
    }
    let counter = 0;
    return <div onWheel={(e)=>{console.warn(e.deltaY)}}>
        <Carousel itemsToShow={1} renderArrow={Null} renderPagination={Null}
            ref={ref => (this.carousel = ref)}
            onChange={(currentItem, pageIndex) => {setIndex(pageIndex)}}
            >
            {items.map((item) => <Card key={counter++} ham={counter++}
                hasPrev={index > 0}
                hasNext={index < items.length - 1}
                slidePrev={()=>{this.carousel.slidePrev()}}
                slideNext={()=>{this.carousel.slideNext()}}
            >
                {item}
            </Card>)}
        </Carousel>
    </div>
}
console.log("Marts ahoy")

const root = createRoot(document.getElementById("app"));
root.render(
  <React.StrictMode>
    <Quiz />
  </React.StrictMode>
);
