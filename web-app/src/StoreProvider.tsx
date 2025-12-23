import { type JSX } from 'react';
import { AppStore, StoreContext } from './store';

export function StoreProvider({ children }: { children: React.ReactNode }): JSX.Element {
    const store = new AppStore();
    return <StoreContext value={store}>{children}</StoreContext>;
}
