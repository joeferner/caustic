import classes from './Header.module.scss';
import { userAtom } from '../store';
import { useAtomValue } from 'jotai';
import type { JSX } from 'react';

const PICTURE_SIZE = 35;

export function Header(): JSX.Element {
    const user = useAtomValue(userAtom);

    return (
        <div className={classes.header}>
            <div className={classes.title}>
                <img src="/navbar-logo.png" height={30} />
            </div>
            <div className={classes.userInfo}>
                {user?.picture ? <img src={user.picture} width={PICTURE_SIZE} height={PICTURE_SIZE} /> : null}
                <div className={classes.details}>
                    <div className={classes.name}>{user?.name}</div>
                    <div className={classes.email}>{user?.email}</div>
                </div>
            </div>
        </div>
    );
}
