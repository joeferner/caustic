import { useAtomValue, useSetAtom } from 'jotai';
import { GoogleLogin, type GoogleCredentialResponse } from './GoogleLogin';
import { handleGoogleCredentialResponseAtom, userAtom } from '../store';
import type { JSX } from 'react';

export function Login(): JSX.Element | null {
    const user = useAtomValue(userAtom);
    const handleGoogleCredentialResponse = useSetAtom(handleGoogleCredentialResponseAtom);

    if (!user) {
        return null;
    }

    function onCredentialResponse(response: GoogleCredentialResponse): void {
        void handleGoogleCredentialResponse({ response });
    }

    return <GoogleLogin clientId={user.googleClientId} onCredentialResponse={onCredentialResponse} />;
}
