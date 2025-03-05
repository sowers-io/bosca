import {
    Node,
    mergeAttributes,
} from '@tiptap/core'
import type {DocumentTemplateContainer} from "~/lib/graphql/graphql.ts";

export interface ContainerOptions {
    name: string;
    containers: Array<DocumentTemplateContainer>;
    HTMLAttributes: Record<string, any>,
}

declare module '@tiptap/core' {
    interface Commands<ReturnType> {
        container: {
            setContainer: (attributes: { name: string }) => ReturnType,
            unsetContainer: () => ReturnType,
        }
    }
}

export const Container = Node.create<ContainerOptions>({
    name: 'container',

    group: 'block',

    content: 'block*',

    addOptions() {
        return {
            name: '',
            containers: [],
            HTMLAttributes: {},
        }
    },

    addAttributes() {
        return {
            name: {
                default: this.options.name,
                isRequired: true,
            },
        };
    },

    parseHTML() {
        return []
    },

    renderHTML({HTMLAttributes}) {
        if (HTMLAttributes.name) {
            const container = this.options.containers.find(c => c.id === HTMLAttributes.name)
            return [
                'div', {class: 'container'},
                ['div', {class: 'container-name'}, container?.name || HTMLAttributes.name],
                ['div', {class: 'container-content'},
                    ['div', mergeAttributes(this.options.HTMLAttributes, HTMLAttributes), 0],
                ]
            ]
        }
        return ['span', mergeAttributes(this.options.HTMLAttributes, HTMLAttributes), 0]
    },

    addCommands() {
        return {
            setContainer: ({ name }) => ({editor, tr, commands}) => {
                if (commands.wrapIn(this.name, { name })) {
                    const paragraphNode = editor.schema.nodes.paragraph?.create();
                    if (paragraphNode) {
                        tr.insert(tr.selection.$to.pos + 2, paragraphNode);
                        editor.view.dispatch(tr);
                    }
                    return true
                }
                return false
            },
            unsetContainer: () => ({editor, tr, commands}) => {
                const { $from, $to } = editor.state.selection;
                const nodeRange = $from.blockRange($to);
                if (!nodeRange) {
                    console.error("No container to unwrap.");
                    return false;
                }
                const targetParentNode = nodeRange.depth && $from.node(nodeRange.depth);
                if (!targetParentNode || targetParentNode.type.name !== this.name) {
                    console.error("Selection is not inside a container node.");
                    return false;
                }
                return commands.lift('container');
            },
        }
    },

    addKeyboardShortcuts() {
        return {}
    },

    addInputRules() {
        return []
    },

    addPasteRules() {
        return []
    },
})
