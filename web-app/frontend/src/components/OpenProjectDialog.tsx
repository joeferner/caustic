import { Button, Divider, Group, Loader, Modal, Stack, TextInput, UnstyledButton } from '@mantine/core';
import { useCallback, useEffect, useState, type JSX } from 'react';
import classes from './OpenProjectDialog.module.scss';
import { useAtomValue, useSetAtom } from 'jotai';
import { createProjectAtom, loadExampleProjectAtom, userAtom } from '../store';
import { Example } from '../utils/examples';
import { ErrorMessage, type ErrorMessageProps } from './ErrorMessage';

export function OpenProjectDialog({ opened, onClose }: { opened: boolean; onClose: () => void }): JSX.Element {
    const user = useAtomValue(userAtom);
    const loadExampleProject = useSetAtom(loadExampleProjectAtom);
    const createProject = useSetAtom(createProjectAtom);
    const [loading, setLoading] = useState(false);
    const [error, setError] = useState<ErrorMessageProps | undefined>(undefined);
    const [newProjectName, setNewProjectName] = useState('');
    const [canSubmit, setCanSubmit] = useState(false);

    useEffect(() => {
        setNewProjectName('');
        setError(undefined);
        setLoading(false);
    }, [opened, setNewProjectName, setError]);

    useEffect(() => {
        setCanSubmit(newProjectName.trim().length > 0 && !loading);
    }, [newProjectName, loading]);

    const loadExample = useCallback(
        (example: Example): void => {
            void (async (): Promise<void> => {
                try {
                    setLoading(true);
                    setError(undefined);
                    await loadExampleProject(example);
                    onClose();
                } catch (err) {
                    const message = err instanceof Error ? err.message : 'Unknown error';
                    setError({
                        title: 'Error Loading Example',
                        message,
                    });
                } finally {
                    setLoading(false);
                }
            })();
        },
        [loadExampleProject, onClose, setError, setLoading]
    );

    const onCancelClick = useCallback((): void => {
        onClose();
    }, [onClose]);

    const onCreateProjectClick = useCallback((): void => {
        void (async (): Promise<void> => {
            try {
                setLoading(true);
                setError(undefined);
                await createProject({ name: newProjectName });
                onClose();
            } catch (err) {
                const message = err instanceof Error ? err.message : 'Unknown error';
                setError({
                    title: 'Error Creating Project',
                    message,
                });
            } finally {
                setLoading(false);
            }
        })();
    }, [createProject, newProjectName, onClose, setError, setLoading]);

    return (
        <Modal opened={opened} onClose={onClose} title="Open Project" zIndex={2000}>
            <Stack className={classes.group} align="start">
                {error ? <ErrorMessage {...error} width="100%" /> : null}
                <div className={classes.item}>
                    <div>
                        <TextInput
                            placeholder="New Project Name"
                            inputSize="100"
                            label="New Project Name"
                            description={user ? null : 'To create a new project you must be logged in'}
                            value={newProjectName}
                            onChange={(event) => {
                                setNewProjectName(event.target.value);
                            }}
                        />
                    </div>
                </div>
                <Divider
                    my="xs"
                    label="Previous Projects"
                    labelPosition="center"
                    style={{ width: `100%`, margin: 0 }}
                />
                <div className={classes.item}>
                    <Stack className={classes.existingProjects}>
                        <UnstyledButton
                            onClick={() => {
                                loadExample(Example.Car);
                            }}
                        >
                            Example: Car
                        </UnstyledButton>
                        <UnstyledButton
                            onClick={() => {
                                loadExample(Example.ThreeSpheres);
                            }}
                        >
                            Example: Three Spheres
                        </UnstyledButton>
                        <UnstyledButton
                            onClick={() => {
                                loadExample(Example.RandomSpheres);
                            }}
                        >
                            Example: Random Spheres
                        </UnstyledButton>
                    </Stack>
                </div>

                <Group justify="flex-end" className={classes.footer}>
                    {loading ? <Loader color="blue" size="xs" type="bars" /> : null}
                    <UnstyledButton onClick={onCancelClick}>Cancel</UnstyledButton>
                    <Button onClick={onCreateProjectClick} disabled={!canSubmit}>
                        Create Project
                    </Button>
                </Group>
            </Stack>
        </Modal>
    );
}
