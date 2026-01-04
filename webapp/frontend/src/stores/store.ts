import { RayTracerApi, type Project } from '../api';
import { ProjectsStore } from './ProjectsStore';
import { UserStore } from './UserStore';
import { ProjectStore } from './ProjectStore';

export const rayTracerApi = new RayTracerApi();

export type UnsubscribeFn = () => void;

export interface RenderOptions {
    blockSize?: number;
    threadCount?: number;
}

export interface StoreProject extends Project {
    readOnly: boolean;
}

export const DEFAULT_RENDER_BLOCK_SIZE = 50;
export const EXAMPLE_CAR_ID = 'cad84577-c808-41a9-8d77-25a4626fe65f';

export const projectStore = new ProjectStore();
export const projectsStore = new ProjectsStore();
export const userStore = new UserStore();
