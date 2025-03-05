import {
    Node,
    mergeAttributes,
} from '@tiptap/core'
import type {DocumentTemplateContainer, MetadataFragment} from "~/lib/graphql/graphql.ts";
import {VueNodeViewRenderer} from "@tiptap/vue-3";
import ContainerNode from "~/components/content/editor/ContainerNode.vue";

export interface ContainerOptions {
    metadata: MetadataFragment;
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
            metadata: {} as MetadataFragment,
            containers: [],
            HTMLAttributes: {},
        }
    },

    addAttributes() {
        return {
            name: {
                default: null,
                isRequired: true,
            },
        };
    },

    parseHTML() {
        return []
    },

    renderHTML({HTMLAttributes}) {
        return ['span', mergeAttributes(this.options.HTMLAttributes, HTMLAttributes), 0]
    },

    addNodeView() {
        // @ts-ignore: this is fine
        return VueNodeViewRenderer(ContainerNode, { draggable: true });
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
