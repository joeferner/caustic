import { type Settings, type User } from '../api';
import type { GoogleCredentialResponse } from '../components/GoogleLogin';
import { signal } from '@preact/signals-react';
import { rayTracerApi } from './store';

const LOCAL_STORAGE_JWT_TOKEN_KEY = 'jwtToken';

export class UserStore {
    public readonly user = signal<User | undefined>();
    public readonly settings = signal<Settings | undefined>(undefined);

    private set jwtToken(value: string | undefined) {
        if (value) {
            window.localStorage.setItem(LOCAL_STORAGE_JWT_TOKEN_KEY, value);
        } else {
            window.localStorage.removeItem(LOCAL_STORAGE_JWT_TOKEN_KEY);
        }
    }

    private get jwtToken(): string | undefined {
        return window.localStorage.getItem(LOCAL_STORAGE_JWT_TOKEN_KEY) ?? undefined;
    }

    public async loadUserMe(): Promise<void> {
        const jwtToken = this.jwtToken;
        if (jwtToken) {
            rayTracerApi.token = jwtToken;
        }

        const resp = await rayTracerApi.user.getUserMe();
        this.settings.value = resp.settings;
        this.user.value = resp.user ?? undefined;
    }

    public logOut(): void {
        this.user.value = undefined;
        this.jwtToken = undefined;
    }

    public async handleGoogleCredentialResponse({ response }: { response: GoogleCredentialResponse }): Promise<void> {
        try {
            const data = await rayTracerApi.user.googleTokenVerify({
                googleVerifyRequest: {
                    token: response.credential,
                },
            });

            rayTracerApi.token = data.token;

            const resp = await rayTracerApi.user.getUserMe();
            this.settings.value = resp.settings;
            this.user.value = resp.user ?? undefined;
            this.jwtToken = data.token;
        } catch (err) {
            console.error('onGoogleCredentialResponse', err instanceof Error ? err : new Error('Unknown error'));
        }
    }
}
