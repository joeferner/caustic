import { modals } from '@mantine/modals';
import {
    ActionIcon,
    Button,
    Divider,
    Group,
    Loader,
    Modal,
    Paper,
    Stack,
    Text,
    TextInput,
    Tooltip,
} from '@mantine/core';
import { type JSX, type MouseEvent } from 'react';
import classes from './OpenProjectDialog.module.scss';
import { store } from '../store';
import { ErrorMessage, type ErrorMessageProps } from './ErrorMessage';
import { Copy as CopyIcon, Trash as DeleteIcon } from 'react-bootstrap-icons';
import type { UserDataProject } from '../api';
import { useSignal, useSignalEffect } from '@preact/signals-react';
import { Show } from '@preact/signals-react/utils';

export function OpenProjectDialog({ opened, onClose }: { opened: boolean; onClose: () => void }): JSX.Element {
    const loading = useSignal(false);
    const error = useSignal<ErrorMessageProps | undefined>(undefined);
    const newProjectName = useSignal('');
    const canSubmit = useSignal(false);

    useSignalEffect(() => {
        canSubmit.value = newProjectName.value.trim().length > 0 && !loading.value;
    });

    const loadProject = (projectId: string): void => {
        void (async (): Promise<void> => {
            try {
                loading.value = true;
                error.value = undefined;
                await store.loadProject({ projectId });
                onClose();
            } catch (err) {
                const message = err instanceof Error ? err.message : 'Unknown error';
                error.value = {
                    title: 'Error Loading Project',
                    message,
                };
            } finally {
                loading.value = false;
            }
        })();
    };

    const copyProject = (projectId: string): void => {
        void (async (): Promise<void> => {
            try {
                loading.value = true;
                error.value = undefined;
                await store.copyProject({ projectId });
                onClose();
            } catch (err) {
                const message = err instanceof Error ? err.message : 'Unknown error';
                error.value = {
                    title: 'Error Coping Project',
                    message,
                };
            } finally {
                loading.value = false;
            }
        })();
    };

    const deleteProject = (projectId: string): void => {
        void (async (): Promise<void> => {
            try {
                loading.value = true;
                error.value = undefined;
                await store.deleteProject({ projectId });
            } catch (err) {
                const message = err instanceof Error ? err.message : 'Unknown error';
                error.value = {
                    title: 'Error Deleting Project',
                    message,
                };
            } finally {
                loading.value = false;
            }
        })();
    };

    const onCreateProjectClick = (): void => {
        void (async (): Promise<void> => {
            try {
                loading.value = true;
                error.value = undefined;
                await store.createProject({ name: newProjectName.value });
                onClose();
            } catch (err) {
                const message = err instanceof Error ? err.message : 'Unknown error';
                error.value = {
                    title: 'Error Creating Project',
                    message,
                };
            } finally {
                loading.value = false;
            }
        })();
    };

    return (
        <Modal opened={opened} onClose={onClose} title="Open Project" zIndex={2000}>
            <Stack className={classes.group} align="start">
                <Show when={error}>{(error) => <ErrorMessage {...error} width="100%" />}</Show>
                <div className={classes.item}>
                    <div>
                        <TextInput
                            placeholder="New Project Name"
                            inputSize="100"
                            label="New Project Name"
                            description={store.user.value ? null : 'To create a new project you must be logged in'}
                            value={newProjectName.value}
                            onChange={(event) => {
                                newProjectName.value = event.target.value;
                            }}
                        />
                    </div>
                </div>
                <Group justify="flex-end" className={classes.footer}>
                    <Show when={loading}>
                        <Loader color="blue" size="xs" type="bars" />
                    </Show>
                    <Button onClick={onCreateProjectClick} disabled={!canSubmit.value}>
                        Create Project
                    </Button>
                </Group>

                <Divider
                    my="xs"
                    label="Existing Projects"
                    labelPosition="center"
                    style={{ width: `100%`, margin: 0 }}
                />
                <div className={classes.item}>
                    <Stack className={classes.existingProjects}>
                        {store.projects.value?.map((project) => (
                            <ProjectButton
                                key={project.id}
                                project={project}
                                onClick={() => {
                                    loadProject(project.id);
                                }}
                                onCopyProject={() => {
                                    copyProject(project.id);
                                }}
                                onDeleteProject={() => {
                                    deleteProject(project.id);
                                }}
                            />
                        ))}
                    </Stack>
                </div>
            </Stack>
        </Modal>
    );
}

function ProjectButton({
    project,
    onClick,
    onCopyProject,
    onDeleteProject,
}: {
    project: UserDataProject;
    onClick: () => void;
    onCopyProject: () => void;
    onDeleteProject: () => void;
}): JSX.Element {
    const onCopyProjectClick = (event: MouseEvent): void => {
        event.stopPropagation();
        onCopyProject();
    };

    const onDeleteProjectClick = (event: MouseEvent): void => {
        event.stopPropagation();
        modals.openConfirmModal({
            title: 'Delete Project',
            children: (
                <Text size="sm">
                    Are you sure you want to delete project "{project.name}"? This action cannot be undone.
                </Text>
            ),
            labels: { confirm: 'Delete', cancel: 'Cancel' },
            confirmProps: { color: 'red' },
            onConfirm: () => {
                onDeleteProject();
            },
            zIndex: 5000,
        });
    };

    return (
        <Paper key={project.id} onClick={onClick}>
            <div className={classes.projectName}>{project.name}</div>
            <div className={classes.projectActions}>
                <Tooltip label="Clone Project" zIndex={5000}>
                    <ActionIcon
                        onClick={(event) => {
                            onCopyProjectClick(event);
                        }}
                        variant="filled"
                        size="sm"
                    >
                        <CopyIcon />
                    </ActionIcon>
                </Tooltip>
                {project.readonly ? null : (
                    <Tooltip label="Delete Project" zIndex={5000}>
                        <ActionIcon
                            onClick={(event) => {
                                onDeleteProjectClick(event);
                            }}
                            variant="filled"
                            size="sm"
                        >
                            <DeleteIcon />
                        </ActionIcon>
                    </Tooltip>
                )}
            </div>
        </Paper>
    );
}
