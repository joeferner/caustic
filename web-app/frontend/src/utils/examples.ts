import type { Project } from '../api';

export enum Example {
    Car = 'Car',
    ThreeSpheres = 'ThreeSpheres',
    RandomSpheres = 'RandomSpheres',
}

export function getExampleProject(example: Example): Project {
    switch (example) {
        case Example.Car:
            return {
                id: 'cad84577-c808-41a9-8d77-25a4626fe65f',
                name: 'Example: Car',
                files: [{ filename: 'main.scad', url: '/examples/car/main.scad' }],
            };

        case Example.ThreeSpheres:
            return {
                id: 'b43378fe-afa5-4706-aa09-0951ff1564f2',
                name: 'Example: Three Spheres',
                files: [{ filename: 'main.scad', url: '/examples/three-spheres/main.scad' }],
            };

        case Example.RandomSpheres:
            return {
                id: 'cb50f13d-c3ea-41da-9369-ca73728f0808',
                name: 'Example: Random Spheres',
                files: [{ filename: 'main.scad', url: '/examples/random-spheres/main.scad' }],
            };

        default:
            throw new Error(`unhandled: ${example}`);
    }
}
