
import init, { draw_circle } from './pkg/orbit_game.js';

async function run() {
    await init(); // WebAssemblyモジュールを初期化
    const canvas = document.getElementById('gameCanvas');
    const ctx = canvas.getContext('2d');

    let x = 0;

    function animate() {
        ctx.clearRect(0, 0, canvas.width, canvas.height);
        ctx.fillStyle = 'blue';

        draw_circle(ctx, x, 300, 50);
        x += 2;
        if (x > canvas.width) x = 0;
        requestAnimationFrame(animate);
    }

    animate();
}

run();