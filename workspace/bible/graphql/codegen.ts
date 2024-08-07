import type { CodegenConfig } from '@graphql-codegen/cli'

const config: CodegenConfig = {
  schema: 'src/schema/**/*.graphql',
  generates: {
    'src/generated/resolvers.ts': {
      config: {
        useIndexSignature: true,
        declarationKind: 'interface',
      },
      plugins: ['typescript', 'typescript-resolvers'],
    },
  },
}
export default config
