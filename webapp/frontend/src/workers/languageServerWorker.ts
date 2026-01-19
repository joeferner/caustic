import {
    createConnection,
    BrowserMessageReader,
    BrowserMessageWriter,
    TextDocumentSyncKind,
    TextDocuments,
} from 'vscode-languageserver/browser';
import { TextDocument } from 'vscode-languageserver-textdocument';

const reader = new BrowserMessageReader(self);
const writer = new BrowserMessageWriter(self);
const connection = createConnection(reader, writer);
const documents = new TextDocuments(TextDocument);

self.onerror = (err): void => {
    console.error("Worker Error:", err);
};

connection.onInitialize(() => {
    console.log('onInitialize');
    return {
        capabilities: {
            textDocumentSync: TextDocumentSyncKind.Full,
            hoverProvider: true,
            completionProvider: { resolveProvider: true },
        }
    };
});

connection.onDidOpenTextDocument((params): void => {
    console.log('Document opened', params)
})

documents.onDidOpen((e) => {
    console.log(`Document opened: ${e.document.uri}`);
});

documents.onDidChangeContent((e) => {
    console.log(`Document changed: ${e.document.uri} (version ${e.document.version})`);
});

// This should now trigger
connection.onHover(({ textDocument, position }) => {
    console.log('onHover', textDocument, position);
    return {
        contents: {
            kind: 'markdown',
            value: `### OpenSCAD Info\nLine: ${position.line}, Character: ${position.character}`
        }
    };
});

// IMPORTANT: Make the documents aware of the connection
documents.listen(connection);
connection.listen();

// async function start(): Promise<void> {
//     await initWasm();

//     // // The callback receives messages from Rust and posts them back to the main thread
//     // const server = new WasmLspServer((message: string) => {
//     //     console.log('WasmLspServer message', message);
//     //     self.postMessage(JSON.parse(message));
//     // });

//     // // Listen for messages from the VS Code Client (main thread)
//     // self.onmessage = async (event): Promise<void> => {
//     //     console.log('WasmLspServer onmessage', event);
//     //     const message = JSON.stringify(event.data);
//     //     await server.notify_client_message(message);
//     // };
// }

// start().catch(console.error);
