<script lang="ts">
	import * as THREE from 'three';
    import { onMount } from "svelte";
    import wasm from '../lorenz_wasm/Cargo.toml';

    import OrbitControls from "./OrbitControls.js";

    const MAX_WIDTH = window.innerWidth;
    const MAX_HEIGHT = window.innerHeight;

	let container, controls;

	let camera, cameraTarget, scene, renderer;
    let lwasm;
    let t = 0;
    let dolly_const = 0;

    onMount(async () => {
        lwasm = await wasm();
        init();
        animate();
        const v = lwasm.lorenz(0, 200, 28, 10, (8/3));
        setInterval(() => {
            let points = [];
            let mat = new THREE.PointsMaterial( { size: 0.1, color: 0xffffff } );
            dolly_const += 3;
            for (let i = 0; i < 10; i++) {
                points.push(new THREE.Vector3(v[t], v[t + 1], v[t + 2]))
                // let dotGeometry = new THREE.BufferGeometry();
                // dotGeometry.setAttribute('position', new THREE.Float32BufferAttribute( [v[t], v[t + 1], v[t + 3]], 3 ) );
                // let dot = new THREE.Points( dotGeometry, dotMaterial );
                // scene.add(dot);
                // dt
                t += 3;
                const cam = dolly_const % 30000;
                if (cam > 20000) {
                    controls.dollyIn(Math.pow( 0.95, dolly_const / 10000000));
                } else {
                    controls.dollyOut(Math.pow( 0.95, dolly_const / 10000000));
                }
            }
            t -= 3;
            const geometry = new THREE.BufferGeometry().setFromPoints( points );
            const line = new THREE.Line( geometry, mat );
            scene.add(line);
        }, 0.02);
    })

	function init() {
        const wi = window.innerWidth;
        OrbitControls.autoRotate = true;
        const wh = window.innerHeight;
        const width = wi > MAX_WIDTH ? MAX_WIDTH : wi;
        const height = wh > MAX_HEIGHT ? MAX_HEIGHT : wh;
		camera = new THREE.PerspectiveCamera(60, width / height, 1, 1000);
		// camera.position.set(-7, -0.5, 60);
		camera.position.set(19, 2, 30);

		cameraTarget = new THREE.Vector3(9, 8, 40);

		scene = new THREE.Scene();
		scene.background = new THREE.Color(0x000000);

		renderer = new THREE.WebGLRenderer({ antialiasing: true });
		renderer.setPixelRatio(window.devicePixelRatio);
		renderer.setSize(width, height);
		renderer.outputEncoding = THREE.sRGBEncoding;

		container.appendChild(renderer.domElement);
        controls = new OrbitControls(camera,renderer.domElement);
        controls.update();

		window.addEventListener('resize', onWindowResize);
	}

	function addShadowedLight(x, y, z, color, intensity) {
		const directionalLight = new THREE.DirectionalLight(color, intensity);
		directionalLight.position.set(x, y, z);
		scene.add(directionalLight);

		directionalLight.castShadow = true;

		const d = 1;
		directionalLight.shadow.camera.left = -d;
		directionalLight.shadow.camera.right = d;
		directionalLight.shadow.camera.top = d;
		directionalLight.shadow.camera.bottom = -d;

		directionalLight.shadow.camera.near = 1;
		directionalLight.shadow.camera.far = 4;

		directionalLight.shadow.bias = -0.002;
	}

	function onWindowResize() {
		camera.aspect = window.innerWidth / window.innerHeight;
		camera.updateProjectionMatrix();

		renderer.setSize(window.innerWidth, window.innerHeight);
	}

	function animate() {
		requestAnimationFrame(animate);
        controls.update();
		render();
	}

	function render() {
		camera.lookAt(cameraTarget);

		renderer.render(scene, camera);
	}
</script>

<main>
    <div bind:this={container} />
</main>
