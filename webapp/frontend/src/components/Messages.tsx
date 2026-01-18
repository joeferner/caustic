import { Accordion } from '@mantine/core';
import type { JSX, MouseEvent } from 'react';
import classes from './Messages.module.scss';
import { projectStore } from '../stores/store';
import type { WasmMessage } from '../wasm';
import * as R from 'radash';

export const Messages = (): JSX.Element => {
    const messages = projectStore.messages.value;
    const count = messages.length;
    const countStr = count === 0 ? '' : ` (${count.toLocaleString()})`;

    return (
        <Accordion className={classes.messages}>
            <Accordion.Item value="messages">
                <Accordion.Control>Messages{countStr}</Accordion.Control>
                <Accordion.Panel>
                    {messages.length === 0 ? (
                        <div>No Messages</div>
                    ) : (
                        messages.map((message) => <Message key={message.id} message={message} />)
                    )}
                </Accordion.Panel>
            </Accordion.Item>
        </Accordion>
    );
};

const Message = ({ message }: { message: WasmMessage }): JSX.Element => {
    const handlePositionClick = (event: MouseEvent<HTMLAnchorElement>): void => {
        event.preventDefault();
        projectStore.goto(message.position);
    };

    let positionStr = message.position.filename;
    if (R.isInt(message.position.startLine)) {
        positionStr += `:${message.position.startLine + 1}`;
    }
    if (R.isInt(message.position.startColumn)) {
        positionStr += `:${message.position.startColumn + 1}`;
    }

    return (
        <div className={classes.message}>
            <div className={`${classes.level} ${message.level.toLocaleUpperCase()}`}>
                {message.level.toLocaleUpperCase()}
            </div>
            <div className={classes.description}>{message.message}</div>
            <a href="#" className={classes.position} onClick={handlePositionClick}>
                {positionStr}
            </a>
        </div>
    );
};
