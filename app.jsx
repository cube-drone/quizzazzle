import * as React from 'react'
import { createRoot } from "react-dom/client";
import * as Server from 'react-dom/server'
import Carousel from 'react-elastic-carousel'

let Card = ({key, children}) => <div key={key} class='card'>{children}</div>

let Greet = () => <div>
        <Carousel itemsToShow={1}>
            <Card key={1}>1</Card>
            <Card key={2}>2</Card>
            <Card key={3}>3</Card>
            <Card key={4}>4</Card>
        </Carousel>
    </div>
console.log("Marts ahoy")

const root = createRoot(document.getElementById("app"));
root.render(
  <React.StrictMode>
    <Greet />
  </React.StrictMode>
);
