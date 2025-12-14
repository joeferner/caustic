import { Menu, Tooltip, UnstyledButton } from '@mantine/core';
import { useMyContext } from '../state';
import { useCallback, useEffect, type JSX, type ReactNode } from 'react';
import { Play as RenderIcon, ListTask as ExampleIcon } from 'react-bootstrap-icons';
import styles from './Navbar.module.scss';
import { Example, EXAMPLES } from '../utils/examples';

const ICON_SIZE = 30;

export function Navbar(): JSX.Element {
    const { render, updateFile } = useMyContext();

    useEffect(() => {
        const handleKeyPress = (event: KeyboardEvent): void => {
            if (event.key === 'F5' && !event.ctrlKey && !event.shiftKey && !event.altKey && !event.metaKey) {
                event.preventDefault();
                void render();
            }
        };

        document.addEventListener('keydown', handleKeyPress);
        return (): void => {
            document.removeEventListener('keydown', handleKeyPress);
        };
    }, [render]);

    const loadExample = useCallback(
        (example: Example): void => {
            const code = EXAMPLES[example];
            updateFile('main.scad', code);
        },
        [updateFile]
    );

    return (
        <div className={styles.wrapper}>
            <NavbarLink
                label="Render (F5)"
                icon={<RenderIcon width={ICON_SIZE} height={ICON_SIZE} />}
                onClick={() => {
                    void render();
                }}
            />
            <Menu
                position="right-start"
                withArrow
                arrowPosition="center"
                withinPortal={true}
                closeOnClickOutside={true}
            >
                <Menu.Target>
                    <UnstyledButton className={styles.link}>
                        <ExampleIcon width={ICON_SIZE} height={ICON_SIZE} />
                    </UnstyledButton>
                </Menu.Target>
                <Menu.Dropdown>
                    <Menu.Item
                        onClick={() => {
                            loadExample(Example.Car);
                        }}
                    >
                        Car
                    </Menu.Item>
                    <Menu.Item
                        onClick={() => {
                            loadExample(Example.ThreeSpheres);
                        }}
                    >
                        Three Spheres
                    </Menu.Item>
                    <Menu.Item
                        onClick={() => {
                            loadExample(Example.RandomSpheres);
                        }}
                    >
                        Random Spheres
                    </Menu.Item>
                </Menu.Dropdown>
            </Menu>
        </div>
    );
}

interface NavbarLinkProps {
    icon: ReactNode;
    label: string;
    onClick?: () => void;
}

function NavbarLink({ icon, label, onClick }: NavbarLinkProps): JSX.Element {
    return (
        <Tooltip label={label} position="right" transitionProps={{ duration: 0 }}>
            <UnstyledButton onClick={onClick} className={styles.link}>
                {icon}
            </UnstyledButton>
        </Tooltip>
    );
}
