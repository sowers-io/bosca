import type { CodegenConfig } from '@graphql-codegen/cli'

const config: CodegenConfig = {
  schema: './graphql/schema.graphqls',
  documents: './graphql/**/*.graphql',
  ignoreNoDocuments: true,
  generates: {
    './lib/graphql/': {
      preset: 'client',
      presetConfig: {
        skipTypename: false,
        useTypeImports: true,
        declarationKind: 'interface',
        fragmentMasking: false,
        persistedDocuments: true,
      },
    },
  },
}

export default config
