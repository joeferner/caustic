import classes from './App.module.scss';
import { Panel, PanelGroup, PanelResizeHandle } from 'react-resizable-panels';
import { Files } from './components/Files';
import { Provider as JotaiProvider, useSetAtom } from 'jotai';
import { Render } from './components/Render';
import { Navbar } from './components/Navbar';
import { useEffect, type JSX } from 'react';
import { loadProjectAtom, loadUserMeAtom } from './store';
import { Header } from './components/Header';

export function App(): JSX.Element {
    return (
        <JotaiProvider>
            <InnerApp />
        </JotaiProvider>
    );
}

function InnerApp(): JSX.Element {
    const loadProject = useSetAtom(loadProjectAtom);
    const loadUserMe = useSetAtom(loadUserMeAtom);

    useEffect(() => {
        console.log('load initial project');
        void loadUserMe();
        void loadProject({ projectId: 'cad84577-c808-41a9-8d77-25a4626fe65f' }); // Example: Car
    }, [loadUserMe, loadProject]);

    return (
        <div className={classes.main}>
            <Header />
            <div className={classes.inner}>
                <Navbar />
                <PanelGroup autoSaveId="editRender" direction="horizontal">
                    <Panel defaultSize={50}>
                        <Files />
                    </Panel>
                    <PanelResizeHandle className="resizeHandle" />
                    <Panel>
                        <Render />
                    </Panel>
                </PanelGroup>
            </div>
        </div>
    );
}
