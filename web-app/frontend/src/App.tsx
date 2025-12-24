import classes from './App.module.scss';
import { Panel, PanelGroup, PanelResizeHandle } from 'react-resizable-panels';
import { Files } from './components/Files';
import { Provider as JotaiProvider, useSetAtom } from 'jotai';
import { Render } from './components/Render';
import { Navbar } from './components/Navbar';
import { useEffect, type JSX } from 'react';
import { loadExampleProjectAtom, loadUserMeAtom } from './store';
import { Example } from './utils/examples';
import { Header } from './components/Header';

export function App(): JSX.Element {
    return (
        <JotaiProvider>
            <InnerApp />
        </JotaiProvider>
    );
}

function InnerApp(): JSX.Element {
    const loadExampleProject = useSetAtom(loadExampleProjectAtom);
    const loadUserMe = useSetAtom(loadUserMeAtom);

    useEffect(() => {
        console.log('load initial project');
        void loadUserMe();
        void loadExampleProject(Example.ThreeSpheres);
    }, [loadUserMe, loadExampleProject]);

    return (
        <div className={classes.main}>
            <Header />
            <div className={classes.inner}>
                <Navbar />
                <PanelGroup autoSaveId="example" direction="horizontal">
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
