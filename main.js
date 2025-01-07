
import init, { Object } from './pkg/orbit_game.js';

async function run() {
    await init(); // WebAssemblyモジュールを初期化

    const canvas = document.getElementById('gameCanvas');
    const ctx = canvas.getContext('2d');

    const planet = new Object(700, 400, 0, 0, 1000);
    const rocket = new Object(0, 300, 5, 0, 1000);

    function animate() {
        ctx.clearRect(0, 0, canvas.width, canvas.height);

        ctx.beginPath();
        ctx.arc(planet.x(), planet.y(), 20, 0, Math.PI * 2);
        ctx.strokeStyle = 'blue';
        ctx.lineWidth = 2;
        ctx.stroke();

        ctx.beginPath();
        ctx.arc(rocket.x(), rocket.y(), 20, 0, Math.PI * 2);
        ctx.strokeStyle = 'red';
        ctx.lineWidth = 2;
        ctx.stroke();

        rocket.update_position();

        requestAnimationFrame(animate);
    }

    animate();
}

run();