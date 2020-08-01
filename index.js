const js = import('./pkg/hello_wasm.js');

let boid = [];

let update_function = (obj) => obj;

js.then(js=> {
    boid = js.get_new_boid(1000, 200);
    console.log(js);
    update_function = js.get_next_state;
})

const display = document.getElementById("display");
const radious = 10;

function loop() {
    if (display.childNodes.length > 0) {
        boid = update_function(boid);
        for (i in boid) {
            update_polygon(display.childNodes[i], boid[i]);
        }
        setTimeout(() => loop(), 100);
        return;
    }

    for (const bird of boid) {
        const polygon = document.createElementNS("http://www.w3.org/2000/svg", "polygon");
        update_polygon(polygon, bird);
        display.appendChild(polygon);
    }
    setTimeout(() => loop(), 100);
}

function update_polygon(polygon, bird) {
    const velocity = bird.velocity;
    const position = bird.position;
    const points = new Array(3).fill(null).map((_, i) => {
        const theta = Math.PI * i * 2 / 3 + Math.atan2(velocity.x, velocity.y);
        const x = Math.floor(position.x + Math.cos(theta) * radious);
        const y = Math.floor(position.y + Math.sin(theta) * radious);
        return `${x},${y}`;
    }).join(" ");
    polygon.setAttribute("points", points);
}

loop();
