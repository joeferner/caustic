import {
    Configuration,
    ProjectApi,
    UserApi,
    type ConfigurationParameters,
    type HTTPHeaders,
    type Project,
} from '../api';
import { ProjectsStore } from './ProjectsStore';
import { UserStore } from './UserStore';
import { ProjectStore } from './ProjectStore';

export class RayTracerApi {
    private config = new Configuration();
    private _project = new ProjectApi();
    private _user = new UserApi();

    public constructor() {
        this.token = undefined;
    }

    public get project(): ProjectApi {
        return this._project;
    }

    public get user(): UserApi {
        return this._user;
    }

    public set token(token: string | undefined) {
        const headers: HTTPHeaders = {};
        if (token) {
            headers.Authorization = `Bearer ${token}`;
        }
        const config: ConfigurationParameters = {
            basePath: document.location.origin,
            headers,
        };
        this.config = new Configuration(config);
        this._project = new ProjectApi(this.config);
        this._user = new UserApi(this.config);
    }
}

export const rayTracerApi = new RayTracerApi();

export type UnsubscribeFn = () => void;

export interface RenderOptions {
    blockSize?: number;
    threadCount?: number;
}

export interface StoreProject extends Project {
    readOnly: boolean;
}

export const CONTENT_TYPE_OPENSCAD = 'application/x-openscad';
export const DEFAULT_RENDER_BLOCK_SIZE = 50;
export const EXAMPLE_CAR_ID = 'cad84577-c808-41a9-8d77-25a4626fe65f';

export const projectStore = new ProjectStore();
export const projectsStore = new ProjectsStore();
export const userStore = new UserStore();
