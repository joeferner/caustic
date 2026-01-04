import { type JSX } from 'react';
import { store } from '../store';
import { GoogleLogin, type GoogleCredentialResponse } from './GoogleLogin';
import { Button, Divider, Modal } from '@mantine/core';
import classes from './LoginDialog.module.scss';
import { Signal } from '@preact/signals-react';

export interface LoginDialogProps {
    opened: Signal<boolean>;
    onClose: () => void;
}

export function LoginDialog({ opened, onClose }: LoginDialogProps): JSX.Element | null {
    const WIDTH = 300;

    const onCredentialResponse = (response: GoogleCredentialResponse): void => {
        const run = async (): Promise<void> => {
            await store.handleGoogleCredentialResponse({ response });
            onClose();
        };
        void run();
    };

    const onLogOutClick = (): void => {
        store.logOut();
        onClose();
    };

    if (!store.settings.value) {
        return null;
    }

    return (
        <Modal opened={opened.value} onClose={onClose} title="Login" zIndex={2000}>
            <div className={classes.loginDialogOptions}>
                <GoogleLogin
                    clientId={store.settings.value.googleClientId}
                    onCredentialResponse={onCredentialResponse}
                    buttonConfig={{ width: WIDTH, theme: 'outline' }}
                />
                {store.user.value && (
                    <>
                        <Divider my="xs" label="OR" labelPosition="center" style={{ width: `${WIDTH}px` }} />
                        <Button onClick={onLogOutClick}>Log Out</Button>
                    </>
                )}
            </div>
        </Modal>
    );
}
