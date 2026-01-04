import { type JSX } from 'react';
import { userStore } from '../stores/store';
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
            await userStore.handleGoogleCredentialResponse({ response });
            onClose();
        };
        void run();
    };

    const onLogOutClick = (): void => {
        userStore.logOut();
        onClose();
    };

    if (!userStore.settings.value) {
        return null;
    }

    return (
        <Modal opened={opened.value} onClose={onClose} title="Login" zIndex={2000}>
            <div className={classes.loginDialogOptions}>
                <GoogleLogin
                    clientId={userStore.settings.value.googleClientId}
                    onCredentialResponse={onCredentialResponse}
                    buttonConfig={{ width: WIDTH, theme: 'outline' }}
                />
                {userStore.user.value && (
                    <>
                        <Divider my="xs" label="OR" labelPosition="center" style={{ width: `${WIDTH}px` }} />
                        <Button onClick={onLogOutClick}>Log Out</Button>
                    </>
                )}
            </div>
        </Modal>
    );
}
