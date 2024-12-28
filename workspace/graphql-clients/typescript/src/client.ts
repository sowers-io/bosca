// noinspection JSUnusedGlobalSymbols

import { BaseClient } from '@bosca/graphql-client-base'
import { SupplementaryUploadUrlInput, SupplementaryUploadUrl_Query, AddChildMetadataInput, AddChildMetadata_Mutation, SetCollectionPublicInput, SetCollectionPublic_Mutation, SetWorkflowStateCompleteInput, SetWorkflowStateComplete_Mutation, AddCollectionInput, AddCollection_Mutation, AddMetadataBulkInput, AddMetadataBulk_Mutation, SupplementaryDownloadUrlInput, SupplementaryDownloadUrl_Query, TraitByIdInput, TraitById_Query, SetWorkflowJobContextInput, SetWorkflowJobContext_Mutation, MetadataUploadUrlInput, MetadataUploadUrl_Query, AddSearchDocumentsInput, AddSearchDocuments_Mutation, GetCollectionItemsInput, GetCollectionItems_Query, AddMetadataInput, AddMetadata_Mutation, LoginInput, Login_Query, SetCollectionReadyInput, SetCollectionReady_Mutation, FindMetadataInput, FindMetadata_Query, SetMetadataAttributesInput, SetMetadataAttributes_Mutation, SetMetadataPublicInput, SetMetadataPublic_Mutation, SetExecutionPlanJobCheckinInput, SetExecutionPlanJobCheckin_Mutation, EnqueueChildWorkflowInput, EnqueueChildWorkflow_Mutation, PlanInput, Plan_Query, AddChildCollectionInput, AddChildCollection_Mutation, AddMetadataSupplementaryInput, AddMetadataSupplementary_Mutation, EnqueueChildWorkflowsInput, EnqueueChildWorkflows_Mutation, SetWorkflowJobCompleteInput, SetWorkflowJobComplete_Mutation, EnqueueJobInput, EnqueueJob_Mutation, SourceByIdInput, SourceById_Query, SetMetadataReadyInput, SetMetadataReady_Mutation, SetMetadataSystemAttributesInput, SetMetadataSystemAttributes_Mutation, SetCollectionWorkflowStateCompleteInput, SetCollectionWorkflowStateComplete_Mutation, SetCollectionPublicListInput, SetCollectionPublicList_Mutation, SetCollectionWorkflowStateInput, SetCollectionWorkflowState_Mutation, GetMetadataInput, GetMetadata_Query, GetCollectionInput, GetCollection_Query, SetWorkflowStateInput, SetWorkflowState_Mutation, SetWorkflowJobFailedInput, SetWorkflowJobFailed_Mutation, FindCollectionInput, FindCollection_Query, SetWorkflowPlanContextInput, SetWorkflowPlanContext_Mutation, MetadataDownloadUrlInput, MetadataDownloadUrl_Query } from './models'

export class Client extends BaseClient {
  async SupplementaryUploadUrl(variables: SupplementaryUploadUrlInput): Promise<SupplementaryUploadUrl_Query> {
    const extensions = {
      persistedQuery: {
        version: 1,
        sha256Hash: '5453c97df1a912470b7648a0f59d30cc38eac7b5f1482636bcf7c27f9e7f6cad',
      },
    }
    const data = {
      extensions,
      variables,
    }
    return await this.execute<SupplementaryUploadUrl_Query>(data, false)
  }
  async AddChildMetadata(variables: AddChildMetadataInput): Promise<AddChildMetadata_Mutation> {
    const extensions = {
      persistedQuery: {
        version: 1,
        sha256Hash: '619dc96ec8df1063609eedd4db764dfda5ad095d00e7a190dc29aa00cef38b2c',
      },
    }
    const data = {
      extensions,
      variables,
    }
    return await this.execute<AddChildMetadata_Mutation>(data, true)
  }
  async SetCollectionPublic(variables: SetCollectionPublicInput): Promise<SetCollectionPublic_Mutation> {
    const extensions = {
      persistedQuery: {
        version: 1,
        sha256Hash: '9c812891b5541b3a1438f6e539b203da77ecc4dbb97e933ca7ea1cf680e3f9b4',
      },
    }
    const data = {
      extensions,
      variables,
    }
    return await this.execute<SetCollectionPublic_Mutation>(data, true)
  }
  async SetWorkflowStateComplete(variables: SetWorkflowStateCompleteInput): Promise<SetWorkflowStateComplete_Mutation> {
    const extensions = {
      persistedQuery: {
        version: 1,
        sha256Hash: '47831fb659dc4638b389677b203024126c337a3c7eefda866e7fb48f5204243f',
      },
    }
    const data = {
      extensions,
      variables,
    }
    return await this.execute<SetWorkflowStateComplete_Mutation>(data, true)
  }
  async AddCollection(variables: AddCollectionInput): Promise<AddCollection_Mutation> {
    const extensions = {
      persistedQuery: {
        version: 1,
        sha256Hash: '7a25598ad92e73d6431a3b9e2f124b93849fa93920eee5acdb2f6ecc17a167b7',
      },
    }
    const data = {
      extensions,
      variables,
    }
    return await this.execute<AddCollection_Mutation>(data, true)
  }
  async AddMetadataBulk(variables: AddMetadataBulkInput): Promise<AddMetadataBulk_Mutation> {
    const extensions = {
      persistedQuery: {
        version: 1,
        sha256Hash: '1029a34650f9d5d26b1c54dc8fc6cb3176cdfbb07cac647ecdc970dfa33593c6',
      },
    }
    const data = {
      extensions,
      variables,
    }
    return await this.execute<AddMetadataBulk_Mutation>(data, true)
  }
  async SupplementaryDownloadUrl(variables: SupplementaryDownloadUrlInput): Promise<SupplementaryDownloadUrl_Query> {
    const extensions = {
      persistedQuery: {
        version: 1,
        sha256Hash: '0b877c25ceaf189e3cf8aefb1c31d2d42fd19d5d14aba4966feeaccb8ab0f447',
      },
    }
    const data = {
      extensions,
      variables,
    }
    return await this.execute<SupplementaryDownloadUrl_Query>(data, false)
  }
  async TraitById(variables: TraitByIdInput): Promise<TraitById_Query> {
    const extensions = {
      persistedQuery: {
        version: 1,
        sha256Hash: '88f76d14107db921f681ed73cacc831ea902807fb65aa42c79ce23835b473ac0',
      },
    }
    const data = {
      extensions,
      variables,
    }
    return await this.execute<TraitById_Query>(data, false)
  }
  async SetWorkflowJobContext(variables: SetWorkflowJobContextInput): Promise<SetWorkflowJobContext_Mutation> {
    const extensions = {
      persistedQuery: {
        version: 1,
        sha256Hash: 'e65f29fb5b3a5d149fd006f43574aa2f5b537f179cac87317d359f03bb25d82b',
      },
    }
    const data = {
      extensions,
      variables,
    }
    return await this.execute<SetWorkflowJobContext_Mutation>(data, true)
  }
  async MetadataUploadUrl(variables: MetadataUploadUrlInput): Promise<MetadataUploadUrl_Query> {
    const extensions = {
      persistedQuery: {
        version: 1,
        sha256Hash: 'be7e0dddfb79292a3305017cde50595f64a7748f84fc4c7b92f0ae450d433601',
      },
    }
    const data = {
      extensions,
      variables,
    }
    return await this.execute<MetadataUploadUrl_Query>(data, false)
  }
  async AddSearchDocuments(variables: AddSearchDocumentsInput): Promise<AddSearchDocuments_Mutation> {
    const extensions = {
      persistedQuery: {
        version: 1,
        sha256Hash: '3466f36eff9221faa4812159abc8ef0dc30487aacd45f4dc191b8fa51041609a',
      },
    }
    const data = {
      extensions,
      variables,
    }
    return await this.execute<AddSearchDocuments_Mutation>(data, true)
  }
  async GetCollectionItems(variables: GetCollectionItemsInput): Promise<GetCollectionItems_Query> {
    const extensions = {
      persistedQuery: {
        version: 1,
        sha256Hash: '18c4b54bc3301cc6f2c8463ed29290ff3340787b0e1de30b31e03142f98026bc',
      },
    }
    const data = {
      extensions,
      variables,
    }
    return await this.execute<GetCollectionItems_Query>(data, false)
  }
  async AddMetadata(variables: AddMetadataInput): Promise<AddMetadata_Mutation> {
    const extensions = {
      persistedQuery: {
        version: 1,
        sha256Hash: 'f75029b602daaa18ad0e438ca2ad8353d3d6260512d318791f70a6bd057ea315',
      },
    }
    const data = {
      extensions,
      variables,
    }
    return await this.execute<AddMetadata_Mutation>(data, true)
  }
  async Login(variables: LoginInput): Promise<Login_Query> {
    const extensions = {
      persistedQuery: {
        version: 1,
        sha256Hash: '337ef24fe8352ac27932d1a955153051230cfe98df008867d94596f8117477fc',
      },
    }
    const data = {
      extensions,
      variables,
    }
    return await this.execute<Login_Query>(data, false)
  }
  async SetCollectionReady(variables: SetCollectionReadyInput): Promise<SetCollectionReady_Mutation> {
    const extensions = {
      persistedQuery: {
        version: 1,
        sha256Hash: 'dbdbc5c3cd33e4bfef3314d5ee45a1452ebe34d24449b030120b68939a050b6a',
      },
    }
    const data = {
      extensions,
      variables,
    }
    return await this.execute<SetCollectionReady_Mutation>(data, true)
  }
  async FindMetadata(variables: FindMetadataInput): Promise<FindMetadata_Query> {
    const extensions = {
      persistedQuery: {
        version: 1,
        sha256Hash: '2422741cc93c3bbe706c4e7ea4d425569cb5006534b8936c0766c576f3fc9ad6',
      },
    }
    const data = {
      extensions,
      variables,
    }
    return await this.execute<FindMetadata_Query>(data, false)
  }
  async SetMetadataAttributes(variables: SetMetadataAttributesInput): Promise<SetMetadataAttributes_Mutation> {
    const extensions = {
      persistedQuery: {
        version: 1,
        sha256Hash: 'b092a58cfed109173d02e08d6014f684e1b81202e977e982b7a2329ab2dbf506',
      },
    }
    const data = {
      extensions,
      variables,
    }
    return await this.execute<SetMetadataAttributes_Mutation>(data, true)
  }
  async SetMetadataPublic(variables: SetMetadataPublicInput): Promise<SetMetadataPublic_Mutation> {
    const extensions = {
      persistedQuery: {
        version: 1,
        sha256Hash: '42a32c5c57e4e84604e9f382b78924deac5d67fc7e6b6f581f1472d7fe89012c',
      },
    }
    const data = {
      extensions,
      variables,
    }
    return await this.execute<SetMetadataPublic_Mutation>(data, true)
  }
  async SetExecutionPlanJobCheckin(variables: SetExecutionPlanJobCheckinInput): Promise<SetExecutionPlanJobCheckin_Mutation> {
    const extensions = {
      persistedQuery: {
        version: 1,
        sha256Hash: '770f579006080f4b8d3a79c0c3ad87c22573e877990f7fe0adcd4aea16621908',
      },
    }
    const data = {
      extensions,
      variables,
    }
    return await this.execute<SetExecutionPlanJobCheckin_Mutation>(data, true)
  }
  async EnqueueChildWorkflow(variables: EnqueueChildWorkflowInput): Promise<EnqueueChildWorkflow_Mutation> {
    const extensions = {
      persistedQuery: {
        version: 1,
        sha256Hash: '5f34f3eb2ec52686cf5a6c22b2e0476bd52f1e178d7ef7faf3da5fd3bf24a28e',
      },
    }
    const data = {
      extensions,
      variables,
    }
    return await this.execute<EnqueueChildWorkflow_Mutation>(data, true)
  }
  async Plan(variables: PlanInput): Promise<Plan_Query> {
    const extensions = {
      persistedQuery: {
        version: 1,
        sha256Hash: 'da4809a0e12c86102f2a484dfe4bec7e030fd4e6ed7ba7a34bda04117fc744ef',
      },
    }
    const data = {
      extensions,
      variables,
    }
    return await this.execute<Plan_Query>(data, false)
  }
  async AddChildCollection(variables: AddChildCollectionInput): Promise<AddChildCollection_Mutation> {
    const extensions = {
      persistedQuery: {
        version: 1,
        sha256Hash: '542e885a43654791261f5a28145bb5083b2de474ea451ae8541b79c0ea96dbc9',
      },
    }
    const data = {
      extensions,
      variables,
    }
    return await this.execute<AddChildCollection_Mutation>(data, true)
  }
  async AddMetadataSupplementary(variables: AddMetadataSupplementaryInput): Promise<AddMetadataSupplementary_Mutation> {
    const extensions = {
      persistedQuery: {
        version: 1,
        sha256Hash: '81259b0004070fbb944cee93e66a9a307b9b70463bf472b0843b203b969e94fa',
      },
    }
    const data = {
      extensions,
      variables,
    }
    return await this.execute<AddMetadataSupplementary_Mutation>(data, true)
  }
  async EnqueueChildWorkflows(variables: EnqueueChildWorkflowsInput): Promise<EnqueueChildWorkflows_Mutation> {
    const extensions = {
      persistedQuery: {
        version: 1,
        sha256Hash: '4d9c4d10da7cd41be211000ad49015d6f16ba8cbdd24c55d880d9060d0f32171',
      },
    }
    const data = {
      extensions,
      variables,
    }
    return await this.execute<EnqueueChildWorkflows_Mutation>(data, true)
  }
  async SetWorkflowJobComplete(variables: SetWorkflowJobCompleteInput): Promise<SetWorkflowJobComplete_Mutation> {
    const extensions = {
      persistedQuery: {
        version: 1,
        sha256Hash: 'cd216573372eeb33141cc8e088d060dadcfe7ffb35b53bed9af011908e7dd497',
      },
    }
    const data = {
      extensions,
      variables,
    }
    return await this.execute<SetWorkflowJobComplete_Mutation>(data, true)
  }
  async EnqueueJob(variables: EnqueueJobInput): Promise<EnqueueJob_Mutation> {
    const extensions = {
      persistedQuery: {
        version: 1,
        sha256Hash: '623922015996fdd9164f4cbd422fb1706bcaccacb3c909e69ce2b19ac2be85f9',
      },
    }
    const data = {
      extensions,
      variables,
    }
    return await this.execute<EnqueueJob_Mutation>(data, true)
  }
  async SourceById(variables: SourceByIdInput): Promise<SourceById_Query> {
    const extensions = {
      persistedQuery: {
        version: 1,
        sha256Hash: 'ece51494839a3c2352fa835237ff91af874046201c5ab2c6cdaffd93a23a1d6b',
      },
    }
    const data = {
      extensions,
      variables,
    }
    return await this.execute<SourceById_Query>(data, false)
  }
  async SetMetadataReady(variables: SetMetadataReadyInput): Promise<SetMetadataReady_Mutation> {
    const extensions = {
      persistedQuery: {
        version: 1,
        sha256Hash: '0956277962f7602d654ada9fb34500ff62679ccd45a82dd998efba42de768bb7',
      },
    }
    const data = {
      extensions,
      variables,
    }
    return await this.execute<SetMetadataReady_Mutation>(data, true)
  }
  async SetMetadataSystemAttributes(variables: SetMetadataSystemAttributesInput): Promise<SetMetadataSystemAttributes_Mutation> {
    const extensions = {
      persistedQuery: {
        version: 1,
        sha256Hash: '0ca5ae7c9b2f88832792aea3963fde76bb87ddec6c163b3fb56ac045064704b3',
      },
    }
    const data = {
      extensions,
      variables,
    }
    return await this.execute<SetMetadataSystemAttributes_Mutation>(data, true)
  }
  async SetCollectionWorkflowStateComplete(variables: SetCollectionWorkflowStateCompleteInput): Promise<SetCollectionWorkflowStateComplete_Mutation> {
    const extensions = {
      persistedQuery: {
        version: 1,
        sha256Hash: '02a7cf332c8013504a54bf4682fa2f6e02730c78390ddc7168ac74f29c119b67',
      },
    }
    const data = {
      extensions,
      variables,
    }
    return await this.execute<SetCollectionWorkflowStateComplete_Mutation>(data, true)
  }
  async SetCollectionPublicList(variables: SetCollectionPublicListInput): Promise<SetCollectionPublicList_Mutation> {
    const extensions = {
      persistedQuery: {
        version: 1,
        sha256Hash: '2ae5a847514cd5b8ce77f3af6031013b158452c237fd706dc71cca9a6550254a',
      },
    }
    const data = {
      extensions,
      variables,
    }
    return await this.execute<SetCollectionPublicList_Mutation>(data, true)
  }
  async SetCollectionWorkflowState(variables: SetCollectionWorkflowStateInput): Promise<SetCollectionWorkflowState_Mutation> {
    const extensions = {
      persistedQuery: {
        version: 1,
        sha256Hash: '881fbce2eb42c3d18ddd31b8460a3ba27a393aea3131c2dfdda9e312e0562e8a',
      },
    }
    const data = {
      extensions,
      variables,
    }
    return await this.execute<SetCollectionWorkflowState_Mutation>(data, true)
  }
  async GetMetadata(variables: GetMetadataInput): Promise<GetMetadata_Query> {
    const extensions = {
      persistedQuery: {
        version: 1,
        sha256Hash: '6a16f546de6319c0b03f7482798c48c6fbc6ea43f9d431917cca74fcd619e4cc',
      },
    }
    const data = {
      extensions,
      variables,
    }
    return await this.execute<GetMetadata_Query>(data, false)
  }
  async GetCollection(variables: GetCollectionInput): Promise<GetCollection_Query> {
    const extensions = {
      persistedQuery: {
        version: 1,
        sha256Hash: '87713548af22ba6591d35826962b330a713256a5544dddd4ef38be5942225220',
      },
    }
    const data = {
      extensions,
      variables,
    }
    return await this.execute<GetCollection_Query>(data, false)
  }
  async SetWorkflowState(variables: SetWorkflowStateInput): Promise<SetWorkflowState_Mutation> {
    const extensions = {
      persistedQuery: {
        version: 1,
        sha256Hash: '93a24e8f6ae0e63e11d3efe6ce9da31b4c163bd5c2562cb3d18438ff5db9db67',
      },
    }
    const data = {
      extensions,
      variables,
    }
    return await this.execute<SetWorkflowState_Mutation>(data, true)
  }
  async SetWorkflowJobFailed(variables: SetWorkflowJobFailedInput): Promise<SetWorkflowJobFailed_Mutation> {
    const extensions = {
      persistedQuery: {
        version: 1,
        sha256Hash: '857c12535b8cfd9f5a93daba570d251614c13088920cb0f88437cf1c3744f8b4',
      },
    }
    const data = {
      extensions,
      variables,
    }
    return await this.execute<SetWorkflowJobFailed_Mutation>(data, true)
  }
  async FindCollection(variables: FindCollectionInput): Promise<FindCollection_Query> {
    const extensions = {
      persistedQuery: {
        version: 1,
        sha256Hash: 'a79e18618da95a05f2a86203f36ab42b834049e06d09162acf3d4eb81ff798c8',
      },
    }
    const data = {
      extensions,
      variables,
    }
    return await this.execute<FindCollection_Query>(data, false)
  }
  async SetWorkflowPlanContext(variables: SetWorkflowPlanContextInput): Promise<SetWorkflowPlanContext_Mutation> {
    const extensions = {
      persistedQuery: {
        version: 1,
        sha256Hash: 'ca045f1da8980717fe079f8dc76d794407f03ec52a70c3dab52a44fa89133a96',
      },
    }
    const data = {
      extensions,
      variables,
    }
    return await this.execute<SetWorkflowPlanContext_Mutation>(data, true)
  }
  async MetadataDownloadUrl(variables: MetadataDownloadUrlInput): Promise<MetadataDownloadUrl_Query> {
    const extensions = {
      persistedQuery: {
        version: 1,
        sha256Hash: '16387038cb12a77b748e75d9e2becd2fce9ac13defc336e6a9b644695233e80a',
      },
    }
    const data = {
      extensions,
      variables,
    }
    return await this.execute<MetadataDownloadUrl_Query>(data, false)
  }
}
