/* eslint-disable */
import * as types from './graphql';
import { TypedDocumentNode as DocumentNode } from '@graphql-typed-document-node/core';

/**
 * Map of all GraphQL operations in the project.
 *
 * This map has several performance disadvantages:
 * 1. It is not tree-shakeable, so it will include all operations in the project.
 * 2. It is not minifiable, so the string of a GraphQL query will be multiple times inside the bundle.
 * 3. It does not support dead code elimination, so it will add unused operations.
 *
 * Therefore it is highly recommended to use the babel or swc plugin for production.
 * Learn more about it here: https://the-guild.dev/graphql/codegen/plugins/presets/preset-client#reducing-bundle-size
 */
type Documents = {
    "mutation AddActivity($activity: ActivityInput!) {\n  workflows {\n    activities {\n      add(activity: $activity) {\n        ...Activity\n      }\n    }\n  }\n}": typeof types.AddActivityDocument,
    "mutation AddCollection($collection: CollectionInput!) {\n  content {\n    collection {\n      add(collection: $collection) {\n        id\n      }\n    }\n  }\n}": typeof types.AddCollectionDocument,
    "mutation AddCollectionCollection($collectionId: String!, $id: String!) {\n  content {\n    collection {\n      addChildCollection(id: $collectionId, collectionId: $id) {\n        id\n      }\n    }\n  }\n}": typeof types.AddCollectionCollectionDocument,
    "mutation AddCollectionMetadataRelationship($relationship: CollectionMetadataRelationshipInput!) {\n  content {\n    collection {\n      addMetadataRelationship(relationship: $relationship) {\n        metadata {\n          id\n        }\n      }\n    }\n  }\n}": typeof types.AddCollectionMetadataRelationshipDocument,
    "mutation AddCollectionPermission($permission: PermissionInput!) {\n  content {\n    collection {\n      addPermission(permission: $permission) {\n        groupId\n        action\n      }\n    }\n  }\n}": typeof types.AddCollectionPermissionDocument,
    "mutation AddMetadata($metadata: MetadataInput!) {\n  content {\n    metadata {\n      add(metadata: $metadata) {\n        id\n      }\n    }\n  }\n}": typeof types.AddMetadataDocument,
    "mutation AddMetadataCollection($collectionId: String!, $id: String!) {\n  content {\n    collection {\n      addChildMetadata(id: $collectionId, metadataId: $id) {\n        id\n      }\n    }\n  }\n}": typeof types.AddMetadataCollectionDocument,
    "mutation AddMetadataPermission($permission: PermissionInput!) {\n  content {\n    metadata {\n      addPermission(permission: $permission) {\n        groupId\n        action\n      }\n    }\n  }\n}": typeof types.AddMetadataPermissionDocument,
    "mutation AddMetadataRelationship($relationship: MetadataRelationshipInput!) {\n  content {\n    metadata {\n      addRelationship(relationship: $relationship) {\n        id\n      }\n    }\n  }\n}": typeof types.AddMetadataRelationshipDocument,
    "mutation AddMetadataTrait($metadataId: String!, $traitId: String!) {\n  content {\n    metadata {\n      addTrait(metadataId: $metadataId, traitId: $traitId) {\n        id {\n          ...WorkflowExecutionId\n        }\n      }\n    }\n  }\n}": typeof types.AddMetadataTraitDocument,
    "mutation AddModel($model: ModelInput!) {\n  workflows {\n    models {\n      add(model: $model) {\n        ...Model\n      }\n    }\n  }\n}": typeof types.AddModelDocument,
    "mutation AddPersistedQueries($application: String!, $queries: [PersistedQueryInput!]!) {\n  persistedQueries {\n    addAll(application: $application, queries: $queries)\n  }\n}": typeof types.AddPersistedQueriesDocument,
    "mutation AddPrompt($prompt: PromptInput!) {\n  workflows {\n    prompts {\n      add(prompt: $prompt) {\n        ...Prompt\n      }\n    }\n  }\n}": typeof types.AddPromptDocument,
    "mutation AddState($state: WorkflowStateInput!) {\n  workflows {\n    states {\n      add(state: $state) {\n        ...WorkflowState\n      }\n    }\n  }\n}": typeof types.AddStateDocument,
    "mutation AddStorageSystem($system: StorageSystemInput!) {\n  workflows {\n    storageSystems {\n      add(storageSystem: $system) {\n        ...StorageSystem\n      }\n    }\n  }\n}": typeof types.AddStorageSystemDocument,
    "mutation AddTrait($trait: TraitInput!) {\n  workflows {\n    traits {\n      add(model: $trait) {\n        ...Trait\n      }\n    }\n  }\n}": typeof types.AddTraitDocument,
    "mutation AddTransition($transition: TransitionInput!) {\n  workflows {\n    transitions {\n      add(transition: $transition) {\n        ...Transition\n      }\n    }\n  }\n}": typeof types.AddTransitionDocument,
    "mutation AddWorkflow($workflow: WorkflowInput!) {\n  workflows {\n    add(workflow: $workflow) {\n      ...Workflow\n    }\n  }\n}": typeof types.AddWorkflowDocument,
    "mutation BeginCollectionTransition($id: String!, $state: String!, $status: String!) {\n  workflows {\n    beginTransition(\n      request: {collectionId: $id, stateId: $state, status: $status, waitForCompletion: true}\n    )\n  }\n}": typeof types.BeginCollectionTransitionDocument,
    "mutation BeginMetadataTransition($id: String!, $version: Int!, $state: String!, $status: String!, $stateValid: DateTime) {\n  workflows {\n    beginTransition(\n      request: {metadataId: $id, version: $version, stateId: $state, stateValid: $stateValid, status: $status, waitForCompletion: true}\n    )\n  }\n}": typeof types.BeginMetadataTransitionDocument,
    "mutation CancelMetadataWorkflows($id: String!, $version: Int!, $workflowId: String!) {\n  workflows {\n    cancelWorkflows(\n      metadataId: $id\n      metadataVersion: $version\n      workflowId: $workflowId\n    )\n  }\n}": typeof types.CancelMetadataWorkflowsDocument,
    "mutation CancelTransition($collectionId: String, $metadataId: String, $version: Int) {\n  workflows {\n    cancelTransition(\n      collectionId: $collectionId\n      metadataId: $metadataId\n      metadataVersion: $version\n    )\n  }\n}": typeof types.CancelTransitionDocument,
    "mutation DeleteActivity($id: String!) {\n  workflows {\n    activities {\n      delete(activityId: $id)\n    }\n  }\n}": typeof types.DeleteActivityDocument,
    "mutation DeleteCollection($id: String!) {\n  content {\n    collection {\n      delete(id: $id, recursive: true)\n    }\n  }\n}": typeof types.DeleteCollectionDocument,
    "mutation DeleteConfiguration($key: String!) {\n  configurations {\n    deleteConfiguration(key: $key)\n  }\n}": typeof types.DeleteConfigurationDocument,
    "mutation DeleteMetadata($id: String!) {\n  content {\n    metadata {\n      delete(metadataId: $id)\n    }\n  }\n}": typeof types.DeleteMetadataDocument,
    "mutation DeleteModel($id: String!) {\n  workflows {\n    models {\n      delete(id: $id)\n    }\n  }\n}": typeof types.DeleteModelDocument,
    "mutation DeletePrompt($id: String!) {\n  workflows {\n    prompts {\n      delete(id: $id)\n    }\n  }\n}": typeof types.DeletePromptDocument,
    "mutation DeleteState($id: String!) {\n  workflows {\n    states {\n      delete(id: $id)\n    }\n  }\n}": typeof types.DeleteStateDocument,
    "mutation DeleteStorageSystem($id: String!) {\n  workflows {\n    storageSystems {\n      delete(id: $id)\n    }\n  }\n}": typeof types.DeleteStorageSystemDocument,
    "mutation DeleteTrait($id: String!) {\n  workflows {\n    traits {\n      delete(id: $id)\n    }\n  }\n}": typeof types.DeleteTraitDocument,
    "mutation DeleteTransition($fromStateId: String!, $toStateId: String!) {\n  workflows {\n    transitions {\n      delete(fromStateId: $fromStateId, toStateId: $toStateId)\n    }\n  }\n}": typeof types.DeleteTransitionDocument,
    "mutation DeleteWorkflow($id: String!) {\n  workflows {\n    delete(id: $id)\n  }\n}": typeof types.DeleteWorkflowDocument,
    "mutation EditActivity($input: ActivityInput!) {\n  workflows {\n    activities {\n      edit(activity: $input) {\n        ...Activity\n      }\n    }\n  }\n}": typeof types.EditActivityDocument,
    "mutation EditCollection($id: String!, $input: CollectionInput!) {\n  content {\n    collection {\n      edit(id: $id, collection: $input) {\n        id\n      }\n    }\n  }\n}": typeof types.EditCollectionDocument,
    "mutation EditMetadata($id: String!, $metadata: MetadataInput!) {\n  content {\n    metadata {\n      edit(id: $id, metadata: $metadata) {\n        id\n      }\n    }\n  }\n}": typeof types.EditMetadataDocument,
    "mutation EditModel($id: String!, $input: ModelInput!) {\n  workflows {\n    models {\n      edit(id: $id, model: $input) {\n        ...Model\n      }\n    }\n  }\n}": typeof types.EditModelDocument,
    "mutation EditPrompt($id: String!, $input: PromptInput!) {\n  workflows {\n    prompts {\n      edit(id: $id, prompt: $input) {\n        ...Prompt\n      }\n    }\n  }\n}": typeof types.EditPromptDocument,
    "mutation EditState($input: WorkflowStateInput!) {\n  workflows {\n    states {\n      edit(state: $input) {\n        ...WorkflowState\n      }\n    }\n  }\n}": typeof types.EditStateDocument,
    "mutation EditStorageSystem($id: String!, $input: StorageSystemInput!) {\n  workflows {\n    storageSystems {\n      edit(id: $id, storageSystem: $input) {\n        ...StorageSystem\n      }\n    }\n  }\n}": typeof types.EditStorageSystemDocument,
    "mutation EditTrait($input: TraitInput!) {\n  workflows {\n    traits {\n      edit(model: $input) {\n        ...Trait\n      }\n    }\n  }\n}": typeof types.EditTraitDocument,
    "mutation EditTransition($input: TransitionInput!) {\n  workflows {\n    transitions {\n      edit(transition: $input) {\n        ...Transition\n      }\n    }\n  }\n}": typeof types.EditTransitionDocument,
    "mutation EditWorkflow($input: WorkflowInput!) {\n  workflows {\n    edit(workflow: $input) {\n      id\n    }\n  }\n}": typeof types.EditWorkflowDocument,
    "mutation EnqueueWorkflow($workflowId: String!, $collectionId: String, $metadataId: String, $version: Int, $configurations: [WorkflowConfigurationInput!]) {\n  workflows {\n    enqueueWorkflow(\n      workflowId: $workflowId\n      collectionId: $collectionId\n      metadataId: $metadataId\n      version: $version\n      configurations: $configurations\n    ) {\n      id\n      queue\n    }\n  }\n}": typeof types.EnqueueWorkflowDocument,
    "query ExecuteSearch($query: String!, $filter: String, $offset: Int!, $limit: Int!, $storageSystemId: String!) {\n  search(\n    query: {query: $query, filter: $filter, offset: $offset, limit: $limit, storageSystemId: $storageSystemId}\n  ) {\n    documents {\n      collection {\n        ...CollectionIdName\n      }\n      metadata {\n        ...MetadataIdName\n      }\n      profile {\n        ...ProfileIdName\n      }\n    }\n  }\n}": typeof types.ExecuteSearchDocument,
    "query FindCollections($query: FindQueryInput!) {\n  content {\n    findCollections(query: $query) {\n      ...Collection\n    }\n  }\n}": typeof types.FindCollectionsDocument,
    "query FindCollectionsCount($query: FindQueryInput!) {\n  content {\n    findCollectionsCount(query: $query)\n  }\n}": typeof types.FindCollectionsCountDocument,
    "query FindMetadata($query: FindQueryInput!) {\n  content {\n    findMetadata(query: $query) {\n      ...Metadata\n    }\n  }\n}": typeof types.FindMetadataDocument,
    "query FindMetadataCount($query: FindQueryInput!) {\n  content {\n    findMetadataCount(query: $query)\n  }\n}": typeof types.FindMetadataCountDocument,
    "query GetActivities {\n  workflows {\n    activities {\n      all {\n        ...Activity\n      }\n    }\n  }\n}": typeof types.GetActivitiesDocument,
    "query GetActivity($id: String!) {\n  workflows {\n    activities {\n      activity(id: $id) {\n        ...Activity\n      }\n    }\n  }\n}": typeof types.GetActivityDocument,
    "query GetCollection($id: String) {\n  content {\n    collection(id: $id) {\n      ...Collection\n    }\n  }\n}": typeof types.GetCollectionDocument,
    "query GetCollectionChildrenCollections($id: String!, $offset: Int!, $limit: Int!) {\n  content {\n    collection(id: $id) {\n      collections(limit: $limit, offset: $offset) {\n        ...Collection\n      }\n      collectionsCount\n    }\n  }\n}": typeof types.GetCollectionChildrenCollectionsDocument,
    "query GetCollectionChildrenMetadata($id: String!, $offset: Int!, $limit: Int!) {\n  content {\n    collection(id: $id) {\n      metadata(limit: $limit, offset: $offset) {\n        ...Metadata\n      }\n      metadataCount\n    }\n  }\n}": typeof types.GetCollectionChildrenMetadataDocument,
    "query GetCollectionList($id: String) {\n  content {\n    collection(id: $id) {\n      ...CollectionList\n    }\n  }\n}": typeof types.GetCollectionListDocument,
    "query GetCollectionMetadataRelationships($id: String!) {\n  content {\n    collection(id: $id) {\n      metadataRelationships {\n        ...CollectionMetadataRelationship\n      }\n    }\n  }\n}": typeof types.GetCollectionMetadataRelationshipsDocument,
    "query GetCollectionParents($id: String) {\n  content {\n    collection(id: $id) {\n      ...CollectionParents\n    }\n  }\n}": typeof types.GetCollectionParentsDocument,
    "query GetCollectionPermissions($id: String) {\n  content {\n    collection(id: $id) {\n      permissions {\n        ...Permission\n      }\n    }\n  }\n}": typeof types.GetCollectionPermissionsDocument,
    "query GetCollectionTemplate($id: String!, $version: Int) {\n  content {\n    metadata(id: $id, version: $version) {\n      collectionTemplate {\n        ...CollectionTemplate\n      }\n    }\n  }\n}": typeof types.GetCollectionTemplateDocument,
    "query GetCollectionWorkflowPlans($id: String) {\n  content {\n    collection(id: $id) {\n      workflow {\n        plans {\n          ...WorkflowPlan\n        }\n      }\n    }\n  }\n}": typeof types.GetCollectionWorkflowPlansDocument,
    "query GetConfiguration($key: String!) {\n  configurations {\n    configuration(key: $key) {\n      ...Configuration\n    }\n  }\n}": typeof types.GetConfigurationDocument,
    "query GetConfigurations {\n  configurations {\n    all {\n      ...Configuration\n    }\n  }\n}": typeof types.GetConfigurationsDocument,
    "query GetCurrentProfile {\n  profiles {\n    current {\n      ...Profile\n    }\n  }\n}": typeof types.GetCurrentProfileDocument,
    "query GetGroups($offset: Int!, $limit: Int!) {\n  security {\n    groups {\n      all(offset: $offset, limit: $limit) {\n        ...Group\n      }\n    }\n  }\n}": typeof types.GetGroupsDocument,
    "query GetMetadata($id: String!, $version: Int) {\n  content {\n    metadata(id: $id, version: $version) {\n      ...Metadata\n    }\n  }\n}\n\nquery GetMetadataUpload($id: String!) {\n  content {\n    metadata(id: $id) {\n      content {\n        ...MetadataContentUpload\n      }\n    }\n  }\n}": typeof types.GetMetadataDocument,
    "query GetMetadataDocument($id: String!) {\n  content {\n    metadata(id: $id) {\n      document {\n        ...Document\n      }\n    }\n  }\n}": typeof types.GetMetadataDocumentDocument,
    "query GetMetadataDocumentTemplate($id: String!, $version: Int) {\n  content {\n    metadata(id: $id, version: $version) {\n      documentTemplate {\n        ...DocumentTemplate\n      }\n    }\n  }\n}": typeof types.GetMetadataDocumentTemplateDocument,
    "query GetMetadataParents($id: String!) {\n  content {\n    metadata(id: $id) {\n      parentCollections(offset: 0, limit: 100) {\n        ...ParentCollection\n      }\n    }\n  }\n}": typeof types.GetMetadataParentsDocument,
    "query GetMetadataPermissions($id: String!) {\n  content {\n    metadata(id: $id) {\n      permissions {\n        ...Permission\n      }\n    }\n  }\n}": typeof types.GetMetadataPermissionsDocument,
    "query GetMetadataRelationships($id: String!) {\n  content {\n    metadata(id: $id) {\n      relationships {\n        ...MetadataRelationship\n      }\n    }\n  }\n}": typeof types.GetMetadataRelationshipsDocument,
    "query GetMetadataSupplementary($id: String!, $key: String) {\n  content {\n    metadata(id: $id) {\n      supplementary(key: $key) {\n        ...MetadataSupplementary\n      }\n    }\n  }\n}": typeof types.GetMetadataSupplementaryDocument,
    "query GetMetadataSupplementaryJson($id: String!, $key: String!) {\n  content {\n    metadata(id: $id) {\n      supplementary(key: $key) {\n        content {\n          json\n        }\n      }\n    }\n  }\n}": typeof types.GetMetadataSupplementaryJsonDocument,
    "query GetMetadataSupplementaryText($id: String!, $key: String!) {\n  content {\n    metadata(id: $id) {\n      supplementary(key: $key) {\n        content {\n          text\n        }\n      }\n    }\n  }\n}": typeof types.GetMetadataSupplementaryTextDocument,
    "query GetMetadataWorkflowPlans($id: String!) {\n  content {\n    metadata(id: $id) {\n      workflow {\n        plans {\n          ...WorkflowPlan\n        }\n      }\n    }\n  }\n}": typeof types.GetMetadataWorkflowPlansDocument,
    "query GetModel($id: String!) {\n  workflows {\n    models {\n      model(id: $id) {\n        ...Model\n      }\n    }\n  }\n}": typeof types.GetModelDocument,
    "query GetModels {\n  workflows {\n    models {\n      all {\n        ...Model\n      }\n    }\n  }\n}": typeof types.GetModelsDocument,
    "query GetPermissionActions {\n  security {\n    actions\n  }\n}": typeof types.GetPermissionActionsDocument,
    "query GetPrincipals($offset: Int!, $limit: Int!) {\n  security {\n    principals {\n      all(offset: $offset, limit: $limit) {\n        ...Principal\n      }\n    }\n  }\n}": typeof types.GetPrincipalsDocument,
    "query GetProfile($id: String!) {\n  profiles {\n    profile(id: $id) {\n      ...Profile\n    }\n  }\n}": typeof types.GetProfileDocument,
    "query GetProfiles($offset: Int!, $limit: Int!) {\n  profiles {\n    all(offset: $offset, limit: $limit) {\n      ...Profile\n    }\n  }\n}": typeof types.GetProfilesDocument,
    "query GetPrompt($id: String!) {\n  workflows {\n    prompts {\n      prompt(id: $id) {\n        ...Prompt\n      }\n    }\n  }\n}": typeof types.GetPromptDocument,
    "query GetPrompts {\n  workflows {\n    prompts {\n      all {\n        ...Prompt\n      }\n    }\n  }\n}": typeof types.GetPromptsDocument,
    "query GetState($id: String!) {\n  workflows {\n    states {\n      state(id: $id) {\n        ...WorkflowState\n      }\n    }\n  }\n}": typeof types.GetStateDocument,
    "query GetStates {\n  workflows {\n    states {\n      all {\n        ...WorkflowState\n      }\n    }\n  }\n}": typeof types.GetStatesDocument,
    "query GetStorageSystem($id: String!) {\n  workflows {\n    storageSystems {\n      storageSystem(id: $id) {\n        ...StorageSystem\n      }\n    }\n  }\n}": typeof types.GetStorageSystemDocument,
    "query GetStorageSystems {\n  workflows {\n    storageSystems {\n      all {\n        ...StorageSystem\n      }\n    }\n  }\n}": typeof types.GetStorageSystemsDocument,
    "query GetSupplementaryTextContents($id: String!, $key: String!) {\n  content {\n    metadata(id: $id) {\n      supplementary(key: $key) {\n        content {\n          text\n        }\n      }\n    }\n  }\n}": typeof types.GetSupplementaryTextContentsDocument,
    "query GetTextContents($id: String!) {\n  content {\n    metadata(id: $id) {\n      content {\n        text\n      }\n    }\n  }\n}": typeof types.GetTextContentsDocument,
    "query GetTrait($id: String!) {\n  workflows {\n    traits {\n      trait(id: $id) {\n        ...Trait\n      }\n    }\n  }\n}": typeof types.GetTraitDocument,
    "query GetTraits {\n  workflows {\n    traits {\n      all {\n        ...Trait\n      }\n    }\n  }\n}": typeof types.GetTraitsDocument,
    "query GetTransition($fromStateId: String!, $toStateId: String!) {\n  workflows {\n    transitions {\n      transition(fromStateId: $fromStateId, toStateId: $toStateId) {\n        ...Transition\n      }\n    }\n  }\n}": typeof types.GetTransitionDocument,
    "query GetTransitions {\n  workflows {\n    transitions {\n      all {\n        ...Transition\n      }\n    }\n  }\n}": typeof types.GetTransitionsDocument,
    "query GetWorkflow($id: String!) {\n  workflows {\n    workflow(id: $id) {\n      ...Workflow\n      activities {\n        ...WorkflowActivity\n      }\n    }\n  }\n}": typeof types.GetWorkflowDocument,
    "query GetWorkflowActivities($id: String!) {\n  workflows {\n    workflow(id: $id) {\n      activities {\n        ...WorkflowActivity\n      }\n    }\n  }\n}": typeof types.GetWorkflowActivitiesDocument,
    "query GetWorkflowActivity($id: Int!) {\n  workflows {\n    workflowActivity(id: $id) {\n      ...WorkflowActivity\n    }\n  }\n}": typeof types.GetWorkflowActivityDocument,
    "query GetWorkflows {\n  workflows {\n    all {\n      ...Workflow\n    }\n  }\n}": typeof types.GetWorkflowsDocument,
    "mutation Login($identifier: String!, $password: String!) {\n  security {\n    login {\n      password(identifier: $identifier, password: $password) {\n        profile {\n          ...Profile\n        }\n        principal {\n          id\n          groups {\n            id\n            name\n          }\n        }\n        token {\n          token\n        }\n      }\n    }\n  }\n}": typeof types.LoginDocument,
    "query NextJob($queue: String!) {\n  workflows {\n    nextJob(queue: $queue) {\n      planId {\n        id\n        queue\n      }\n      id {\n        id\n        index\n        queue\n      }\n      collection {\n        ...Collection\n      }\n      metadata {\n        ...Metadata\n      }\n      activity {\n        ...Activity\n      }\n      context\n      workflowActivity {\n        ...WorkflowActivity\n      }\n      storageSystems {\n        system {\n          ...StorageSystem\n        }\n        configuration\n      }\n      prompts {\n        prompt {\n          ...Prompt\n        }\n        configuration\n      }\n      models {\n        model {\n          ...Model\n        }\n        configuration\n      }\n    }\n  }\n}": typeof types.NextJobDocument,
    "subscription OnActivityChanged {\n  activity\n}": typeof types.OnActivityChangedDocument,
    "subscription OnCollectionChanged {\n  collection\n}": typeof types.OnCollectionChangedDocument,
    "subscription OnMetadataChanged {\n  metadata\n}": typeof types.OnMetadataChangedDocument,
    "subscription OnMetadataSupplementaryChanged {\n  metadataSupplementary {\n    id\n    supplementary\n  }\n}": typeof types.OnMetadataSupplementaryChangedDocument,
    "subscription OnModelChanged {\n  model\n}": typeof types.OnModelChangedDocument,
    "subscription OnPromptChanged {\n  prompt\n}": typeof types.OnPromptChangedDocument,
    "subscription OnStateChanged {\n  state\n}": typeof types.OnStateChangedDocument,
    "subscription OnStorageSystemChanged {\n  storageSystem\n}": typeof types.OnStorageSystemChangedDocument,
    "subscription OnTraitChanged {\n  trait\n}": typeof types.OnTraitChangedDocument,
    "subscription OnWorkflowChanged {\n  workflow\n}": typeof types.OnWorkflowChangedDocument,
    "mutation RemoveCollectionCollection($collectionId: String!, $id: String!) {\n  content {\n    collection {\n      removeChildCollection(id: $collectionId, collectionId: $id) {\n        id\n      }\n    }\n  }\n}": typeof types.RemoveCollectionCollectionDocument,
    "mutation RemoveCollectionMetadataRelationship($id: String!, $metadataId: String!, $relationship: String!) {\n  content {\n    collection {\n      deleteMetadataRelationship(\n        id: $id\n        metadataId: $metadataId\n        relationship: $relationship\n      )\n    }\n  }\n}": typeof types.RemoveCollectionMetadataRelationshipDocument,
    "mutation RemoveCollectionPermission($permission: PermissionInput!) {\n  content {\n    collection {\n      deletePermission(permission: $permission) {\n        groupId\n        action\n      }\n    }\n  }\n}": typeof types.RemoveCollectionPermissionDocument,
    "mutation RemoveMetadataCollection($collectionId: String!, $id: String!) {\n  content {\n    collection {\n      removeChildMetadata(id: $collectionId, metadataId: $id) {\n        id\n      }\n    }\n  }\n}": typeof types.RemoveMetadataCollectionDocument,
    "mutation RemoveMetadataPermission($permission: PermissionInput!) {\n  content {\n    metadata {\n      deletePermission(permission: $permission) {\n        groupId\n        action\n      }\n    }\n  }\n}": typeof types.RemoveMetadataPermissionDocument,
    "mutation RemoveMetadataRelationship($id1: String!, $id2: String!, $relationship: String!) {\n  content {\n    metadata {\n      deleteRelationship(id1: $id1, id2: $id2, relationship: $relationship)\n    }\n  }\n}": typeof types.RemoveMetadataRelationshipDocument,
    "mutation RemoveMetadataTrait($metadataId: String!, $traitId: String!) {\n  content {\n    metadata {\n      deleteTrait(metadataId: $metadataId, traitId: $traitId) {\n        id {\n          id\n          queue\n        }\n      }\n    }\n  }\n}": typeof types.RemoveMetadataTraitDocument,
    "mutation SetCollectionAttributes($id: String!, $attributes: JSON!) {\n  content {\n    collection {\n      setCollectionAttributes(id: $id, attributes: $attributes)\n    }\n  }\n}": typeof types.SetCollectionAttributesDocument,
    "mutation SetCollectionPublic($id: String!, $public: Boolean!) {\n  content {\n    collection {\n      setPublic(id: $id, public: $public) {\n        id\n      }\n    }\n  }\n}": typeof types.SetCollectionPublicDocument,
    "mutation SetCollectionPublicList($id: String!, $public: Boolean!) {\n  content {\n    collection {\n      setPublicList(id: $id, public: $public) {\n        id\n      }\n    }\n  }\n}": typeof types.SetCollectionPublicListDocument,
    "mutation SetCollectionReady($id: String!) {\n  content {\n    collection {\n      setReady(id: $id)\n    }\n  }\n}": typeof types.SetCollectionReadyDocument,
    "mutation SetConfiguration($configuration: ConfigurationInput!) {\n  configurations {\n    setConfiguration(configuration: $configuration) {\n      ...Configuration\n    }\n  }\n}": typeof types.SetConfigurationDocument,
    "mutation SetContents($id: String!, $contentType: String, $file: Upload!) {\n  content {\n    metadata {\n      setMetadataContents(id: $id, contentType: $contentType, file: $file)\n    }\n  }\n}": typeof types.SetContentsDocument,
    "mutation SetJsonContents($id: String!, $contentType: String!, $content: JSON!) {\n  content {\n    metadata {\n      setMetadataJsonContents(id: $id, contentType: $contentType, content: $content)\n    }\n  }\n}": typeof types.SetJsonContentsDocument,
    "mutation SetMetadataAttributes($id: String!, $attributes: JSON!) {\n  content {\n    metadata {\n      setMetadataAttributes(id: $id, attributes: $attributes)\n    }\n  }\n}": typeof types.SetMetadataAttributesDocument,
    "mutation SetMetadataContentPublic($id: String!, $public: Boolean!) {\n  content {\n    metadata {\n      setPublicContent(id: $id, public: $public) {\n        id\n      }\n    }\n  }\n}": typeof types.SetMetadataContentPublicDocument,
    "mutation SetMetadataPublic($id: String!, $public: Boolean!) {\n  content {\n    metadata {\n      setPublic(id: $id, public: $public) {\n        id\n      }\n    }\n  }\n}": typeof types.SetMetadataPublicDocument,
    "mutation SetMetadataReady($id: String!) {\n  content {\n    metadata {\n      setMetadataReady(id: $id)\n    }\n  }\n}": typeof types.SetMetadataReadyDocument,
    "mutation SetMetadataSupplementaryPublic($id: String!, $public: Boolean!) {\n  content {\n    metadata {\n      setPublicSupplementary(id: $id, public: $public) {\n        id\n      }\n    }\n  }\n}": typeof types.SetMetadataSupplementaryPublicDocument,
    "mutation SetTextContents($id: String!, $contentType: String!, $content: String!) {\n  content {\n    metadata {\n      setMetadataTextContents(id: $id, contentType: $contentType, content: $content)\n    }\n  }\n}": typeof types.SetTextContentsDocument,
    "mutation SignUp($profile: ProfileInput!, $identifier: String!, $password: String!) {\n  security {\n    signup {\n      password(profile: $profile, identifier: $identifier, password: $password) {\n        id\n      }\n    }\n  }\n}": typeof types.SignUpDocument,
    "mutation VerifySignUp($token: String!) {\n  security {\n    signup {\n      passwordVerify(verificationToken: $token)\n    }\n  }\n}": typeof types.VerifySignUpDocument,
    "fragment Activity on Activity {\n  childWorkflowId\n  configuration\n  description\n  id\n  inputs {\n    ...ActivityParameter\n  }\n  name\n  outputs {\n    ...ActivityParameter\n  }\n}": typeof types.ActivityFragmentDoc,
    "fragment ActivityParameter on ActivityParameter {\n  name\n  type\n}": typeof types.ActivityParameterFragmentDoc,
    "fragment Category on Category {\n  id\n  name\n}": typeof types.CategoryFragmentDoc,
    "fragment CollectionIdName on Collection {\n  __typename\n  id\n  name\n}\n\nfragment CollectionList on Collection {\n  ...Collection\n  items(offset: 0, limit: 1000) {\n    __typename\n    ... on Collection {\n      ...Collection\n    }\n    ... on Metadata {\n      ...Metadata\n    }\n  }\n}\n\nfragment Collection on Collection {\n  __typename\n  id\n  slug\n  traitIds\n  collectionType: type\n  name\n  description\n  labels\n  created\n  modified\n  attributes\n  systemAttributes\n  ready\n  public\n  publicList\n  templateMetadata {\n    id\n    version\n  }\n  ordering {\n    ...Ordering\n  }\n  categories {\n    ...Category\n  }\n  workflow {\n    ...CollectionWorkflow\n  }\n}\n\nfragment CollectionParents on Collection {\n  parentCollections(offset: 0, limit: 100) {\n    ...ParentCollection\n  }\n}\n\nfragment CollectionPermissions on Collection {\n  permissions {\n    ...Permission\n  }\n}": typeof types.CollectionIdNameFragmentDoc,
    "fragment CollectionDetail on Collection {\n  ...Collection\n  items(offset: 0, limit: 1000) {\n    __typename\n    ... on Collection {\n      ...Collection\n    }\n    ... on Metadata {\n      ...Metadata\n    }\n  }\n}": typeof types.CollectionDetailFragmentDoc,
    "fragment CollectionMetadataRelationship on CollectionMetadataRelationship {\n  metadata {\n    ...MetadataRelationshipMetadata\n  }\n  relationship\n  attributes\n}": typeof types.CollectionMetadataRelationshipFragmentDoc,
    "fragment CollectionTemplate on CollectionTemplate {\n  configuration\n  defaultAttributes\n  collectionFilter {\n    options {\n      ...FindQueryOption\n    }\n  }\n  attributes {\n    key\n    name\n    description\n    type\n    supplementaryKey\n    ui\n    list\n    configuration\n    workflows {\n      workflow {\n        ...Workflow\n      }\n      autoRun\n    }\n  }\n}": typeof types.CollectionTemplateFragmentDoc,
    "fragment CollectionWorkflow on CollectionWorkflow {\n  state\n  pending\n}": typeof types.CollectionWorkflowFragmentDoc,
    "fragment Configuration on Configuration {\n  id\n  key\n  description\n  value\n  permissions {\n    action\n    group {\n      id\n      name\n    }\n  }\n}": typeof types.ConfigurationFragmentDoc,
    "fragment Document on Document {\n  template {\n    id\n    version\n  }\n  title\n  content\n}": typeof types.DocumentFragmentDoc,
    "fragment DocumentTemplate on DocumentTemplate {\n  configuration\n  schema\n  content\n  defaultAttributes\n  containers {\n    ...DocumentTemplateContainer\n  }\n  attributes {\n    ...TemplateAttribute\n  }\n}": typeof types.DocumentTemplateFragmentDoc,
    "fragment DocumentTemplateContainer on DocumentTemplateContainer {\n  id\n  name\n  description\n  supplementaryKey\n  workflows {\n    ...TemplateWorkflow\n  }\n}": typeof types.DocumentTemplateContainerFragmentDoc,
    "fragment FindAttributes on FindAttributes {\n  attributes {\n    ...FindAttribute\n  }\n}\n\nfragment FindAttribute on FindAttribute {\n  key\n  value\n}\n\nfragment FindQuery on FindQuery {\n  attributes {\n    ...FindAttributes\n  }\n  categoryIds\n  collectionType\n  contentTypes\n  extensionFilter\n  offset\n  limit\n}\n\nfragment FindQueryOption on FindQueryOption {\n  name\n  query {\n    ...FindQuery\n  }\n}": typeof types.FindAttributesFragmentDoc,
    "fragment Group on Group {\n  id\n  name\n}": typeof types.GroupFragmentDoc,
    "fragment MetadataIdName on Metadata {\n  __typename\n  id\n  version\n  slug\n  name\n  content {\n    type\n  }\n}\n\nfragment Metadata on Metadata {\n  __typename\n  id\n  version\n  slug\n  name\n  labels\n  languageTag\n  public\n  publicContent\n  publicSupplementary\n  parentId\n  type\n  source {\n    id\n    identifier\n  }\n  categories {\n    ...Category\n  }\n  content {\n    ...MetadataContent\n  }\n  created\n  modified\n  uploaded\n  ready\n  attributes\n  systemAttributes\n  traitIds\n  workflow {\n    ...MetadataWorkflow\n  }\n  supplementary {\n    ...MetadataSupplementary\n  }\n  profiles {\n    ...MetadataProfile\n  }\n}": typeof types.MetadataIdNameFragmentDoc,
    "fragment MetadataContent on MetadataContent {\n  type\n  length\n  urls {\n    download {\n      url\n      headers {\n        name\n        value\n      }\n    }\n  }\n}\n\nfragment MetadataContentUpload on MetadataContent {\n  urls {\n    upload {\n      url\n      headers {\n        name\n        value\n      }\n    }\n  }\n}": typeof types.MetadataContentFragmentDoc,
    "fragment MetadataProfile on MetadataProfile {\n  relationship\n  profile {\n    ...Profile\n  }\n}": typeof types.MetadataProfileFragmentDoc,
    "fragment MetadataRelationshipMetadata on Metadata {\n  id\n  version\n  name\n  public\n  publicContent\n  workflow {\n    pending\n    state\n  }\n}\n\nfragment MetadataRelationship on MetadataRelationship {\n  metadata {\n    ...MetadataRelationshipMetadata\n  }\n  relationship\n  attributes\n}": typeof types.MetadataRelationshipMetadataFragmentDoc,
    "fragment MetadataSupplementary on MetadataSupplementary {\n  key\n  name\n  uploaded\n  attributes\n  content {\n    ...MetadataSupplementaryContent\n  }\n  source {\n    id\n    identifier\n  }\n}\n\nfragment MetadataSupplementaryContent on MetadataSupplementaryContent {\n  type\n  length\n  urls {\n    download {\n      url\n      headers {\n        name\n        value\n      }\n    }\n  }\n}": typeof types.MetadataSupplementaryFragmentDoc,
    "fragment MetadataWorkflow on MetadataWorkflow {\n  state\n  stateValid\n  pending\n}": typeof types.MetadataWorkflowFragmentDoc,
    "fragment Model on Model {\n  id\n  name\n  type\n  description\n  configuration\n}": typeof types.ModelFragmentDoc,
    "fragment Ordering on Ordering {\n  path\n  order\n}": typeof types.OrderingFragmentDoc,
    "fragment ParentCollection on Collection {\n  id\n  name\n  attributes\n}": typeof types.ParentCollectionFragmentDoc,
    "fragment Permission on Permission {\n  action\n  group {\n    ...Group\n  }\n}": typeof types.PermissionFragmentDoc,
    "fragment PlanWorkflow on Workflow {\n  id\n  name\n}": typeof types.PlanWorkflowFragmentDoc,
    "fragment Principal on Principal {\n  id\n  verified\n  groups {\n    ...Group\n  }\n}": typeof types.PrincipalFragmentDoc,
    "fragment ProfileIdName on Profile {\n  __typename\n  id\n  name\n}\n\nfragment Profile on Profile {\n  __typename\n  id\n  slug\n  name\n  visibility\n  attributes {\n    id\n    typeId\n    visibility\n    attributes\n    metadata {\n      id\n      content {\n        urls {\n          download {\n            url\n            headers {\n              name\n              value\n            }\n          }\n        }\n      }\n    }\n  }\n}": typeof types.ProfileIdNameFragmentDoc,
    "fragment Prompt on Prompt {\n  id\n  name\n  description\n  inputType\n  outputType\n  systemPrompt\n  userPrompt\n}": typeof types.PromptFragmentDoc,
    "fragment StorageSystem on StorageSystem {\n  id\n  name\n  type\n  description\n  configuration\n  models {\n    modelId\n    configuration\n  }\n}": typeof types.StorageSystemFragmentDoc,
    "fragment TemplateAttribute on TemplateAttribute {\n  key\n  name\n  description\n  type\n  supplementaryKey\n  ui\n  list\n  configuration\n  workflows {\n    ...TemplateWorkflow\n  }\n}": typeof types.TemplateAttributeFragmentDoc,
    "fragment TemplateWorkflow on TemplateWorkflow {\n  autoRun\n  workflow {\n    id\n    name\n  }\n}": typeof types.TemplateWorkflowFragmentDoc,
    "fragment Trait on Trait {\n  id\n  name\n  description\n  contentTypes\n  workflowIds\n  deleteWorkflowId\n}": typeof types.TraitFragmentDoc,
    "fragment Transition on Transition {\n  fromStateId\n  toStateId\n  description\n}": typeof types.TransitionFragmentDoc,
    "fragment Workflow on Workflow {\n  id\n  queue\n  name\n  description\n  configuration\n}": typeof types.WorkflowFragmentDoc,
    "fragment WorkflowActivity on WorkflowActivity {\n  id\n  activityId\n  queue\n  executionGroup\n  inputs {\n    ...WorkflowActivityParameter\n  }\n  outputs {\n    ...WorkflowActivityParameter\n  }\n  configuration\n  storageSystems {\n    ...WorkflowActivityStorageSystem\n  }\n  models {\n    ...WorkflowActivityModel\n  }\n  prompts {\n    ...WorkflowActivityPrompt\n  }\n}": typeof types.WorkflowActivityFragmentDoc,
    "fragment WorkflowActivityModel on WorkflowActivityModel {\n  model {\n    id\n  }\n  configuration\n}": typeof types.WorkflowActivityModelFragmentDoc,
    "fragment WorkflowActivityParameter on WorkflowActivityParameter {\n  name\n  value\n}": typeof types.WorkflowActivityParameterFragmentDoc,
    "fragment WorkflowActivityPrompt on WorkflowActivityPrompt {\n  prompt {\n    id\n  }\n  configuration\n}": typeof types.WorkflowActivityPromptFragmentDoc,
    "fragment WorkflowExecutionId on WorkflowExecutionId {\n  id\n  queue\n}": typeof types.WorkflowExecutionIdFragmentDoc,
    "fragment WorkflowPlan on WorkflowExecutionPlan {\n  id {\n    ...WorkflowExecutionId\n  }\n  complete\n  active\n  failed\n  error\n  cancelled\n  workflow {\n    ...PlanWorkflow\n  }\n}": typeof types.WorkflowPlanFragmentDoc,
    "fragment WorkflowState on WorkflowState {\n  id\n  name\n  configuration\n  description\n  entryWorkflowId\n  exitWorkflowId\n  workflowId\n  type\n}": typeof types.WorkflowStateFragmentDoc,
    "fragment WorkflowActivityStorageSystem on WorkflowActivityStorageSystem {\n  system {\n    id\n  }\n  configuration\n}": typeof types.WorkflowActivityStorageSystemFragmentDoc,
};
const documents: Documents = {
    "mutation AddActivity($activity: ActivityInput!) {\n  workflows {\n    activities {\n      add(activity: $activity) {\n        ...Activity\n      }\n    }\n  }\n}": types.AddActivityDocument,
    "mutation AddCollection($collection: CollectionInput!) {\n  content {\n    collection {\n      add(collection: $collection) {\n        id\n      }\n    }\n  }\n}": types.AddCollectionDocument,
    "mutation AddCollectionCollection($collectionId: String!, $id: String!) {\n  content {\n    collection {\n      addChildCollection(id: $collectionId, collectionId: $id) {\n        id\n      }\n    }\n  }\n}": types.AddCollectionCollectionDocument,
    "mutation AddCollectionMetadataRelationship($relationship: CollectionMetadataRelationshipInput!) {\n  content {\n    collection {\n      addMetadataRelationship(relationship: $relationship) {\n        metadata {\n          id\n        }\n      }\n    }\n  }\n}": types.AddCollectionMetadataRelationshipDocument,
    "mutation AddCollectionPermission($permission: PermissionInput!) {\n  content {\n    collection {\n      addPermission(permission: $permission) {\n        groupId\n        action\n      }\n    }\n  }\n}": types.AddCollectionPermissionDocument,
    "mutation AddMetadata($metadata: MetadataInput!) {\n  content {\n    metadata {\n      add(metadata: $metadata) {\n        id\n      }\n    }\n  }\n}": types.AddMetadataDocument,
    "mutation AddMetadataCollection($collectionId: String!, $id: String!) {\n  content {\n    collection {\n      addChildMetadata(id: $collectionId, metadataId: $id) {\n        id\n      }\n    }\n  }\n}": types.AddMetadataCollectionDocument,
    "mutation AddMetadataPermission($permission: PermissionInput!) {\n  content {\n    metadata {\n      addPermission(permission: $permission) {\n        groupId\n        action\n      }\n    }\n  }\n}": types.AddMetadataPermissionDocument,
    "mutation AddMetadataRelationship($relationship: MetadataRelationshipInput!) {\n  content {\n    metadata {\n      addRelationship(relationship: $relationship) {\n        id\n      }\n    }\n  }\n}": types.AddMetadataRelationshipDocument,
    "mutation AddMetadataTrait($metadataId: String!, $traitId: String!) {\n  content {\n    metadata {\n      addTrait(metadataId: $metadataId, traitId: $traitId) {\n        id {\n          ...WorkflowExecutionId\n        }\n      }\n    }\n  }\n}": types.AddMetadataTraitDocument,
    "mutation AddModel($model: ModelInput!) {\n  workflows {\n    models {\n      add(model: $model) {\n        ...Model\n      }\n    }\n  }\n}": types.AddModelDocument,
    "mutation AddPersistedQueries($application: String!, $queries: [PersistedQueryInput!]!) {\n  persistedQueries {\n    addAll(application: $application, queries: $queries)\n  }\n}": types.AddPersistedQueriesDocument,
    "mutation AddPrompt($prompt: PromptInput!) {\n  workflows {\n    prompts {\n      add(prompt: $prompt) {\n        ...Prompt\n      }\n    }\n  }\n}": types.AddPromptDocument,
    "mutation AddState($state: WorkflowStateInput!) {\n  workflows {\n    states {\n      add(state: $state) {\n        ...WorkflowState\n      }\n    }\n  }\n}": types.AddStateDocument,
    "mutation AddStorageSystem($system: StorageSystemInput!) {\n  workflows {\n    storageSystems {\n      add(storageSystem: $system) {\n        ...StorageSystem\n      }\n    }\n  }\n}": types.AddStorageSystemDocument,
    "mutation AddTrait($trait: TraitInput!) {\n  workflows {\n    traits {\n      add(model: $trait) {\n        ...Trait\n      }\n    }\n  }\n}": types.AddTraitDocument,
    "mutation AddTransition($transition: TransitionInput!) {\n  workflows {\n    transitions {\n      add(transition: $transition) {\n        ...Transition\n      }\n    }\n  }\n}": types.AddTransitionDocument,
    "mutation AddWorkflow($workflow: WorkflowInput!) {\n  workflows {\n    add(workflow: $workflow) {\n      ...Workflow\n    }\n  }\n}": types.AddWorkflowDocument,
    "mutation BeginCollectionTransition($id: String!, $state: String!, $status: String!) {\n  workflows {\n    beginTransition(\n      request: {collectionId: $id, stateId: $state, status: $status, waitForCompletion: true}\n    )\n  }\n}": types.BeginCollectionTransitionDocument,
    "mutation BeginMetadataTransition($id: String!, $version: Int!, $state: String!, $status: String!, $stateValid: DateTime) {\n  workflows {\n    beginTransition(\n      request: {metadataId: $id, version: $version, stateId: $state, stateValid: $stateValid, status: $status, waitForCompletion: true}\n    )\n  }\n}": types.BeginMetadataTransitionDocument,
    "mutation CancelMetadataWorkflows($id: String!, $version: Int!, $workflowId: String!) {\n  workflows {\n    cancelWorkflows(\n      metadataId: $id\n      metadataVersion: $version\n      workflowId: $workflowId\n    )\n  }\n}": types.CancelMetadataWorkflowsDocument,
    "mutation CancelTransition($collectionId: String, $metadataId: String, $version: Int) {\n  workflows {\n    cancelTransition(\n      collectionId: $collectionId\n      metadataId: $metadataId\n      metadataVersion: $version\n    )\n  }\n}": types.CancelTransitionDocument,
    "mutation DeleteActivity($id: String!) {\n  workflows {\n    activities {\n      delete(activityId: $id)\n    }\n  }\n}": types.DeleteActivityDocument,
    "mutation DeleteCollection($id: String!) {\n  content {\n    collection {\n      delete(id: $id, recursive: true)\n    }\n  }\n}": types.DeleteCollectionDocument,
    "mutation DeleteConfiguration($key: String!) {\n  configurations {\n    deleteConfiguration(key: $key)\n  }\n}": types.DeleteConfigurationDocument,
    "mutation DeleteMetadata($id: String!) {\n  content {\n    metadata {\n      delete(metadataId: $id)\n    }\n  }\n}": types.DeleteMetadataDocument,
    "mutation DeleteModel($id: String!) {\n  workflows {\n    models {\n      delete(id: $id)\n    }\n  }\n}": types.DeleteModelDocument,
    "mutation DeletePrompt($id: String!) {\n  workflows {\n    prompts {\n      delete(id: $id)\n    }\n  }\n}": types.DeletePromptDocument,
    "mutation DeleteState($id: String!) {\n  workflows {\n    states {\n      delete(id: $id)\n    }\n  }\n}": types.DeleteStateDocument,
    "mutation DeleteStorageSystem($id: String!) {\n  workflows {\n    storageSystems {\n      delete(id: $id)\n    }\n  }\n}": types.DeleteStorageSystemDocument,
    "mutation DeleteTrait($id: String!) {\n  workflows {\n    traits {\n      delete(id: $id)\n    }\n  }\n}": types.DeleteTraitDocument,
    "mutation DeleteTransition($fromStateId: String!, $toStateId: String!) {\n  workflows {\n    transitions {\n      delete(fromStateId: $fromStateId, toStateId: $toStateId)\n    }\n  }\n}": types.DeleteTransitionDocument,
    "mutation DeleteWorkflow($id: String!) {\n  workflows {\n    delete(id: $id)\n  }\n}": types.DeleteWorkflowDocument,
    "mutation EditActivity($input: ActivityInput!) {\n  workflows {\n    activities {\n      edit(activity: $input) {\n        ...Activity\n      }\n    }\n  }\n}": types.EditActivityDocument,
    "mutation EditCollection($id: String!, $input: CollectionInput!) {\n  content {\n    collection {\n      edit(id: $id, collection: $input) {\n        id\n      }\n    }\n  }\n}": types.EditCollectionDocument,
    "mutation EditMetadata($id: String!, $metadata: MetadataInput!) {\n  content {\n    metadata {\n      edit(id: $id, metadata: $metadata) {\n        id\n      }\n    }\n  }\n}": types.EditMetadataDocument,
    "mutation EditModel($id: String!, $input: ModelInput!) {\n  workflows {\n    models {\n      edit(id: $id, model: $input) {\n        ...Model\n      }\n    }\n  }\n}": types.EditModelDocument,
    "mutation EditPrompt($id: String!, $input: PromptInput!) {\n  workflows {\n    prompts {\n      edit(id: $id, prompt: $input) {\n        ...Prompt\n      }\n    }\n  }\n}": types.EditPromptDocument,
    "mutation EditState($input: WorkflowStateInput!) {\n  workflows {\n    states {\n      edit(state: $input) {\n        ...WorkflowState\n      }\n    }\n  }\n}": types.EditStateDocument,
    "mutation EditStorageSystem($id: String!, $input: StorageSystemInput!) {\n  workflows {\n    storageSystems {\n      edit(id: $id, storageSystem: $input) {\n        ...StorageSystem\n      }\n    }\n  }\n}": types.EditStorageSystemDocument,
    "mutation EditTrait($input: TraitInput!) {\n  workflows {\n    traits {\n      edit(model: $input) {\n        ...Trait\n      }\n    }\n  }\n}": types.EditTraitDocument,
    "mutation EditTransition($input: TransitionInput!) {\n  workflows {\n    transitions {\n      edit(transition: $input) {\n        ...Transition\n      }\n    }\n  }\n}": types.EditTransitionDocument,
    "mutation EditWorkflow($input: WorkflowInput!) {\n  workflows {\n    edit(workflow: $input) {\n      id\n    }\n  }\n}": types.EditWorkflowDocument,
    "mutation EnqueueWorkflow($workflowId: String!, $collectionId: String, $metadataId: String, $version: Int, $configurations: [WorkflowConfigurationInput!]) {\n  workflows {\n    enqueueWorkflow(\n      workflowId: $workflowId\n      collectionId: $collectionId\n      metadataId: $metadataId\n      version: $version\n      configurations: $configurations\n    ) {\n      id\n      queue\n    }\n  }\n}": types.EnqueueWorkflowDocument,
    "query ExecuteSearch($query: String!, $filter: String, $offset: Int!, $limit: Int!, $storageSystemId: String!) {\n  search(\n    query: {query: $query, filter: $filter, offset: $offset, limit: $limit, storageSystemId: $storageSystemId}\n  ) {\n    documents {\n      collection {\n        ...CollectionIdName\n      }\n      metadata {\n        ...MetadataIdName\n      }\n      profile {\n        ...ProfileIdName\n      }\n    }\n  }\n}": types.ExecuteSearchDocument,
    "query FindCollections($query: FindQueryInput!) {\n  content {\n    findCollections(query: $query) {\n      ...Collection\n    }\n  }\n}": types.FindCollectionsDocument,
    "query FindCollectionsCount($query: FindQueryInput!) {\n  content {\n    findCollectionsCount(query: $query)\n  }\n}": types.FindCollectionsCountDocument,
    "query FindMetadata($query: FindQueryInput!) {\n  content {\n    findMetadata(query: $query) {\n      ...Metadata\n    }\n  }\n}": types.FindMetadataDocument,
    "query FindMetadataCount($query: FindQueryInput!) {\n  content {\n    findMetadataCount(query: $query)\n  }\n}": types.FindMetadataCountDocument,
    "query GetActivities {\n  workflows {\n    activities {\n      all {\n        ...Activity\n      }\n    }\n  }\n}": types.GetActivitiesDocument,
    "query GetActivity($id: String!) {\n  workflows {\n    activities {\n      activity(id: $id) {\n        ...Activity\n      }\n    }\n  }\n}": types.GetActivityDocument,
    "query GetCollection($id: String) {\n  content {\n    collection(id: $id) {\n      ...Collection\n    }\n  }\n}": types.GetCollectionDocument,
    "query GetCollectionChildrenCollections($id: String!, $offset: Int!, $limit: Int!) {\n  content {\n    collection(id: $id) {\n      collections(limit: $limit, offset: $offset) {\n        ...Collection\n      }\n      collectionsCount\n    }\n  }\n}": types.GetCollectionChildrenCollectionsDocument,
    "query GetCollectionChildrenMetadata($id: String!, $offset: Int!, $limit: Int!) {\n  content {\n    collection(id: $id) {\n      metadata(limit: $limit, offset: $offset) {\n        ...Metadata\n      }\n      metadataCount\n    }\n  }\n}": types.GetCollectionChildrenMetadataDocument,
    "query GetCollectionList($id: String) {\n  content {\n    collection(id: $id) {\n      ...CollectionList\n    }\n  }\n}": types.GetCollectionListDocument,
    "query GetCollectionMetadataRelationships($id: String!) {\n  content {\n    collection(id: $id) {\n      metadataRelationships {\n        ...CollectionMetadataRelationship\n      }\n    }\n  }\n}": types.GetCollectionMetadataRelationshipsDocument,
    "query GetCollectionParents($id: String) {\n  content {\n    collection(id: $id) {\n      ...CollectionParents\n    }\n  }\n}": types.GetCollectionParentsDocument,
    "query GetCollectionPermissions($id: String) {\n  content {\n    collection(id: $id) {\n      permissions {\n        ...Permission\n      }\n    }\n  }\n}": types.GetCollectionPermissionsDocument,
    "query GetCollectionTemplate($id: String!, $version: Int) {\n  content {\n    metadata(id: $id, version: $version) {\n      collectionTemplate {\n        ...CollectionTemplate\n      }\n    }\n  }\n}": types.GetCollectionTemplateDocument,
    "query GetCollectionWorkflowPlans($id: String) {\n  content {\n    collection(id: $id) {\n      workflow {\n        plans {\n          ...WorkflowPlan\n        }\n      }\n    }\n  }\n}": types.GetCollectionWorkflowPlansDocument,
    "query GetConfiguration($key: String!) {\n  configurations {\n    configuration(key: $key) {\n      ...Configuration\n    }\n  }\n}": types.GetConfigurationDocument,
    "query GetConfigurations {\n  configurations {\n    all {\n      ...Configuration\n    }\n  }\n}": types.GetConfigurationsDocument,
    "query GetCurrentProfile {\n  profiles {\n    current {\n      ...Profile\n    }\n  }\n}": types.GetCurrentProfileDocument,
    "query GetGroups($offset: Int!, $limit: Int!) {\n  security {\n    groups {\n      all(offset: $offset, limit: $limit) {\n        ...Group\n      }\n    }\n  }\n}": types.GetGroupsDocument,
    "query GetMetadata($id: String!, $version: Int) {\n  content {\n    metadata(id: $id, version: $version) {\n      ...Metadata\n    }\n  }\n}\n\nquery GetMetadataUpload($id: String!) {\n  content {\n    metadata(id: $id) {\n      content {\n        ...MetadataContentUpload\n      }\n    }\n  }\n}": types.GetMetadataDocument,
    "query GetMetadataDocument($id: String!) {\n  content {\n    metadata(id: $id) {\n      document {\n        ...Document\n      }\n    }\n  }\n}": types.GetMetadataDocumentDocument,
    "query GetMetadataDocumentTemplate($id: String!, $version: Int) {\n  content {\n    metadata(id: $id, version: $version) {\n      documentTemplate {\n        ...DocumentTemplate\n      }\n    }\n  }\n}": types.GetMetadataDocumentTemplateDocument,
    "query GetMetadataParents($id: String!) {\n  content {\n    metadata(id: $id) {\n      parentCollections(offset: 0, limit: 100) {\n        ...ParentCollection\n      }\n    }\n  }\n}": types.GetMetadataParentsDocument,
    "query GetMetadataPermissions($id: String!) {\n  content {\n    metadata(id: $id) {\n      permissions {\n        ...Permission\n      }\n    }\n  }\n}": types.GetMetadataPermissionsDocument,
    "query GetMetadataRelationships($id: String!) {\n  content {\n    metadata(id: $id) {\n      relationships {\n        ...MetadataRelationship\n      }\n    }\n  }\n}": types.GetMetadataRelationshipsDocument,
    "query GetMetadataSupplementary($id: String!, $key: String) {\n  content {\n    metadata(id: $id) {\n      supplementary(key: $key) {\n        ...MetadataSupplementary\n      }\n    }\n  }\n}": types.GetMetadataSupplementaryDocument,
    "query GetMetadataSupplementaryJson($id: String!, $key: String!) {\n  content {\n    metadata(id: $id) {\n      supplementary(key: $key) {\n        content {\n          json\n        }\n      }\n    }\n  }\n}": types.GetMetadataSupplementaryJsonDocument,
    "query GetMetadataSupplementaryText($id: String!, $key: String!) {\n  content {\n    metadata(id: $id) {\n      supplementary(key: $key) {\n        content {\n          text\n        }\n      }\n    }\n  }\n}": types.GetMetadataSupplementaryTextDocument,
    "query GetMetadataWorkflowPlans($id: String!) {\n  content {\n    metadata(id: $id) {\n      workflow {\n        plans {\n          ...WorkflowPlan\n        }\n      }\n    }\n  }\n}": types.GetMetadataWorkflowPlansDocument,
    "query GetModel($id: String!) {\n  workflows {\n    models {\n      model(id: $id) {\n        ...Model\n      }\n    }\n  }\n}": types.GetModelDocument,
    "query GetModels {\n  workflows {\n    models {\n      all {\n        ...Model\n      }\n    }\n  }\n}": types.GetModelsDocument,
    "query GetPermissionActions {\n  security {\n    actions\n  }\n}": types.GetPermissionActionsDocument,
    "query GetPrincipals($offset: Int!, $limit: Int!) {\n  security {\n    principals {\n      all(offset: $offset, limit: $limit) {\n        ...Principal\n      }\n    }\n  }\n}": types.GetPrincipalsDocument,
    "query GetProfile($id: String!) {\n  profiles {\n    profile(id: $id) {\n      ...Profile\n    }\n  }\n}": types.GetProfileDocument,
    "query GetProfiles($offset: Int!, $limit: Int!) {\n  profiles {\n    all(offset: $offset, limit: $limit) {\n      ...Profile\n    }\n  }\n}": types.GetProfilesDocument,
    "query GetPrompt($id: String!) {\n  workflows {\n    prompts {\n      prompt(id: $id) {\n        ...Prompt\n      }\n    }\n  }\n}": types.GetPromptDocument,
    "query GetPrompts {\n  workflows {\n    prompts {\n      all {\n        ...Prompt\n      }\n    }\n  }\n}": types.GetPromptsDocument,
    "query GetState($id: String!) {\n  workflows {\n    states {\n      state(id: $id) {\n        ...WorkflowState\n      }\n    }\n  }\n}": types.GetStateDocument,
    "query GetStates {\n  workflows {\n    states {\n      all {\n        ...WorkflowState\n      }\n    }\n  }\n}": types.GetStatesDocument,
    "query GetStorageSystem($id: String!) {\n  workflows {\n    storageSystems {\n      storageSystem(id: $id) {\n        ...StorageSystem\n      }\n    }\n  }\n}": types.GetStorageSystemDocument,
    "query GetStorageSystems {\n  workflows {\n    storageSystems {\n      all {\n        ...StorageSystem\n      }\n    }\n  }\n}": types.GetStorageSystemsDocument,
    "query GetSupplementaryTextContents($id: String!, $key: String!) {\n  content {\n    metadata(id: $id) {\n      supplementary(key: $key) {\n        content {\n          text\n        }\n      }\n    }\n  }\n}": types.GetSupplementaryTextContentsDocument,
    "query GetTextContents($id: String!) {\n  content {\n    metadata(id: $id) {\n      content {\n        text\n      }\n    }\n  }\n}": types.GetTextContentsDocument,
    "query GetTrait($id: String!) {\n  workflows {\n    traits {\n      trait(id: $id) {\n        ...Trait\n      }\n    }\n  }\n}": types.GetTraitDocument,
    "query GetTraits {\n  workflows {\n    traits {\n      all {\n        ...Trait\n      }\n    }\n  }\n}": types.GetTraitsDocument,
    "query GetTransition($fromStateId: String!, $toStateId: String!) {\n  workflows {\n    transitions {\n      transition(fromStateId: $fromStateId, toStateId: $toStateId) {\n        ...Transition\n      }\n    }\n  }\n}": types.GetTransitionDocument,
    "query GetTransitions {\n  workflows {\n    transitions {\n      all {\n        ...Transition\n      }\n    }\n  }\n}": types.GetTransitionsDocument,
    "query GetWorkflow($id: String!) {\n  workflows {\n    workflow(id: $id) {\n      ...Workflow\n      activities {\n        ...WorkflowActivity\n      }\n    }\n  }\n}": types.GetWorkflowDocument,
    "query GetWorkflowActivities($id: String!) {\n  workflows {\n    workflow(id: $id) {\n      activities {\n        ...WorkflowActivity\n      }\n    }\n  }\n}": types.GetWorkflowActivitiesDocument,
    "query GetWorkflowActivity($id: Int!) {\n  workflows {\n    workflowActivity(id: $id) {\n      ...WorkflowActivity\n    }\n  }\n}": types.GetWorkflowActivityDocument,
    "query GetWorkflows {\n  workflows {\n    all {\n      ...Workflow\n    }\n  }\n}": types.GetWorkflowsDocument,
    "mutation Login($identifier: String!, $password: String!) {\n  security {\n    login {\n      password(identifier: $identifier, password: $password) {\n        profile {\n          ...Profile\n        }\n        principal {\n          id\n          groups {\n            id\n            name\n          }\n        }\n        token {\n          token\n        }\n      }\n    }\n  }\n}": types.LoginDocument,
    "query NextJob($queue: String!) {\n  workflows {\n    nextJob(queue: $queue) {\n      planId {\n        id\n        queue\n      }\n      id {\n        id\n        index\n        queue\n      }\n      collection {\n        ...Collection\n      }\n      metadata {\n        ...Metadata\n      }\n      activity {\n        ...Activity\n      }\n      context\n      workflowActivity {\n        ...WorkflowActivity\n      }\n      storageSystems {\n        system {\n          ...StorageSystem\n        }\n        configuration\n      }\n      prompts {\n        prompt {\n          ...Prompt\n        }\n        configuration\n      }\n      models {\n        model {\n          ...Model\n        }\n        configuration\n      }\n    }\n  }\n}": types.NextJobDocument,
    "subscription OnActivityChanged {\n  activity\n}": types.OnActivityChangedDocument,
    "subscription OnCollectionChanged {\n  collection\n}": types.OnCollectionChangedDocument,
    "subscription OnMetadataChanged {\n  metadata\n}": types.OnMetadataChangedDocument,
    "subscription OnMetadataSupplementaryChanged {\n  metadataSupplementary {\n    id\n    supplementary\n  }\n}": types.OnMetadataSupplementaryChangedDocument,
    "subscription OnModelChanged {\n  model\n}": types.OnModelChangedDocument,
    "subscription OnPromptChanged {\n  prompt\n}": types.OnPromptChangedDocument,
    "subscription OnStateChanged {\n  state\n}": types.OnStateChangedDocument,
    "subscription OnStorageSystemChanged {\n  storageSystem\n}": types.OnStorageSystemChangedDocument,
    "subscription OnTraitChanged {\n  trait\n}": types.OnTraitChangedDocument,
    "subscription OnWorkflowChanged {\n  workflow\n}": types.OnWorkflowChangedDocument,
    "mutation RemoveCollectionCollection($collectionId: String!, $id: String!) {\n  content {\n    collection {\n      removeChildCollection(id: $collectionId, collectionId: $id) {\n        id\n      }\n    }\n  }\n}": types.RemoveCollectionCollectionDocument,
    "mutation RemoveCollectionMetadataRelationship($id: String!, $metadataId: String!, $relationship: String!) {\n  content {\n    collection {\n      deleteMetadataRelationship(\n        id: $id\n        metadataId: $metadataId\n        relationship: $relationship\n      )\n    }\n  }\n}": types.RemoveCollectionMetadataRelationshipDocument,
    "mutation RemoveCollectionPermission($permission: PermissionInput!) {\n  content {\n    collection {\n      deletePermission(permission: $permission) {\n        groupId\n        action\n      }\n    }\n  }\n}": types.RemoveCollectionPermissionDocument,
    "mutation RemoveMetadataCollection($collectionId: String!, $id: String!) {\n  content {\n    collection {\n      removeChildMetadata(id: $collectionId, metadataId: $id) {\n        id\n      }\n    }\n  }\n}": types.RemoveMetadataCollectionDocument,
    "mutation RemoveMetadataPermission($permission: PermissionInput!) {\n  content {\n    metadata {\n      deletePermission(permission: $permission) {\n        groupId\n        action\n      }\n    }\n  }\n}": types.RemoveMetadataPermissionDocument,
    "mutation RemoveMetadataRelationship($id1: String!, $id2: String!, $relationship: String!) {\n  content {\n    metadata {\n      deleteRelationship(id1: $id1, id2: $id2, relationship: $relationship)\n    }\n  }\n}": types.RemoveMetadataRelationshipDocument,
    "mutation RemoveMetadataTrait($metadataId: String!, $traitId: String!) {\n  content {\n    metadata {\n      deleteTrait(metadataId: $metadataId, traitId: $traitId) {\n        id {\n          id\n          queue\n        }\n      }\n    }\n  }\n}": types.RemoveMetadataTraitDocument,
    "mutation SetCollectionAttributes($id: String!, $attributes: JSON!) {\n  content {\n    collection {\n      setCollectionAttributes(id: $id, attributes: $attributes)\n    }\n  }\n}": types.SetCollectionAttributesDocument,
    "mutation SetCollectionPublic($id: String!, $public: Boolean!) {\n  content {\n    collection {\n      setPublic(id: $id, public: $public) {\n        id\n      }\n    }\n  }\n}": types.SetCollectionPublicDocument,
    "mutation SetCollectionPublicList($id: String!, $public: Boolean!) {\n  content {\n    collection {\n      setPublicList(id: $id, public: $public) {\n        id\n      }\n    }\n  }\n}": types.SetCollectionPublicListDocument,
    "mutation SetCollectionReady($id: String!) {\n  content {\n    collection {\n      setReady(id: $id)\n    }\n  }\n}": types.SetCollectionReadyDocument,
    "mutation SetConfiguration($configuration: ConfigurationInput!) {\n  configurations {\n    setConfiguration(configuration: $configuration) {\n      ...Configuration\n    }\n  }\n}": types.SetConfigurationDocument,
    "mutation SetContents($id: String!, $contentType: String, $file: Upload!) {\n  content {\n    metadata {\n      setMetadataContents(id: $id, contentType: $contentType, file: $file)\n    }\n  }\n}": types.SetContentsDocument,
    "mutation SetJsonContents($id: String!, $contentType: String!, $content: JSON!) {\n  content {\n    metadata {\n      setMetadataJsonContents(id: $id, contentType: $contentType, content: $content)\n    }\n  }\n}": types.SetJsonContentsDocument,
    "mutation SetMetadataAttributes($id: String!, $attributes: JSON!) {\n  content {\n    metadata {\n      setMetadataAttributes(id: $id, attributes: $attributes)\n    }\n  }\n}": types.SetMetadataAttributesDocument,
    "mutation SetMetadataContentPublic($id: String!, $public: Boolean!) {\n  content {\n    metadata {\n      setPublicContent(id: $id, public: $public) {\n        id\n      }\n    }\n  }\n}": types.SetMetadataContentPublicDocument,
    "mutation SetMetadataPublic($id: String!, $public: Boolean!) {\n  content {\n    metadata {\n      setPublic(id: $id, public: $public) {\n        id\n      }\n    }\n  }\n}": types.SetMetadataPublicDocument,
    "mutation SetMetadataReady($id: String!) {\n  content {\n    metadata {\n      setMetadataReady(id: $id)\n    }\n  }\n}": types.SetMetadataReadyDocument,
    "mutation SetMetadataSupplementaryPublic($id: String!, $public: Boolean!) {\n  content {\n    metadata {\n      setPublicSupplementary(id: $id, public: $public) {\n        id\n      }\n    }\n  }\n}": types.SetMetadataSupplementaryPublicDocument,
    "mutation SetTextContents($id: String!, $contentType: String!, $content: String!) {\n  content {\n    metadata {\n      setMetadataTextContents(id: $id, contentType: $contentType, content: $content)\n    }\n  }\n}": types.SetTextContentsDocument,
    "mutation SignUp($profile: ProfileInput!, $identifier: String!, $password: String!) {\n  security {\n    signup {\n      password(profile: $profile, identifier: $identifier, password: $password) {\n        id\n      }\n    }\n  }\n}": types.SignUpDocument,
    "mutation VerifySignUp($token: String!) {\n  security {\n    signup {\n      passwordVerify(verificationToken: $token)\n    }\n  }\n}": types.VerifySignUpDocument,
    "fragment Activity on Activity {\n  childWorkflowId\n  configuration\n  description\n  id\n  inputs {\n    ...ActivityParameter\n  }\n  name\n  outputs {\n    ...ActivityParameter\n  }\n}": types.ActivityFragmentDoc,
    "fragment ActivityParameter on ActivityParameter {\n  name\n  type\n}": types.ActivityParameterFragmentDoc,
    "fragment Category on Category {\n  id\n  name\n}": types.CategoryFragmentDoc,
    "fragment CollectionIdName on Collection {\n  __typename\n  id\n  name\n}\n\nfragment CollectionList on Collection {\n  ...Collection\n  items(offset: 0, limit: 1000) {\n    __typename\n    ... on Collection {\n      ...Collection\n    }\n    ... on Metadata {\n      ...Metadata\n    }\n  }\n}\n\nfragment Collection on Collection {\n  __typename\n  id\n  slug\n  traitIds\n  collectionType: type\n  name\n  description\n  labels\n  created\n  modified\n  attributes\n  systemAttributes\n  ready\n  public\n  publicList\n  templateMetadata {\n    id\n    version\n  }\n  ordering {\n    ...Ordering\n  }\n  categories {\n    ...Category\n  }\n  workflow {\n    ...CollectionWorkflow\n  }\n}\n\nfragment CollectionParents on Collection {\n  parentCollections(offset: 0, limit: 100) {\n    ...ParentCollection\n  }\n}\n\nfragment CollectionPermissions on Collection {\n  permissions {\n    ...Permission\n  }\n}": types.CollectionIdNameFragmentDoc,
    "fragment CollectionDetail on Collection {\n  ...Collection\n  items(offset: 0, limit: 1000) {\n    __typename\n    ... on Collection {\n      ...Collection\n    }\n    ... on Metadata {\n      ...Metadata\n    }\n  }\n}": types.CollectionDetailFragmentDoc,
    "fragment CollectionMetadataRelationship on CollectionMetadataRelationship {\n  metadata {\n    ...MetadataRelationshipMetadata\n  }\n  relationship\n  attributes\n}": types.CollectionMetadataRelationshipFragmentDoc,
    "fragment CollectionTemplate on CollectionTemplate {\n  configuration\n  defaultAttributes\n  collectionFilter {\n    options {\n      ...FindQueryOption\n    }\n  }\n  attributes {\n    key\n    name\n    description\n    type\n    supplementaryKey\n    ui\n    list\n    configuration\n    workflows {\n      workflow {\n        ...Workflow\n      }\n      autoRun\n    }\n  }\n}": types.CollectionTemplateFragmentDoc,
    "fragment CollectionWorkflow on CollectionWorkflow {\n  state\n  pending\n}": types.CollectionWorkflowFragmentDoc,
    "fragment Configuration on Configuration {\n  id\n  key\n  description\n  value\n  permissions {\n    action\n    group {\n      id\n      name\n    }\n  }\n}": types.ConfigurationFragmentDoc,
    "fragment Document on Document {\n  template {\n    id\n    version\n  }\n  title\n  content\n}": types.DocumentFragmentDoc,
    "fragment DocumentTemplate on DocumentTemplate {\n  configuration\n  schema\n  content\n  defaultAttributes\n  containers {\n    ...DocumentTemplateContainer\n  }\n  attributes {\n    ...TemplateAttribute\n  }\n}": types.DocumentTemplateFragmentDoc,
    "fragment DocumentTemplateContainer on DocumentTemplateContainer {\n  id\n  name\n  description\n  supplementaryKey\n  workflows {\n    ...TemplateWorkflow\n  }\n}": types.DocumentTemplateContainerFragmentDoc,
    "fragment FindAttributes on FindAttributes {\n  attributes {\n    ...FindAttribute\n  }\n}\n\nfragment FindAttribute on FindAttribute {\n  key\n  value\n}\n\nfragment FindQuery on FindQuery {\n  attributes {\n    ...FindAttributes\n  }\n  categoryIds\n  collectionType\n  contentTypes\n  extensionFilter\n  offset\n  limit\n}\n\nfragment FindQueryOption on FindQueryOption {\n  name\n  query {\n    ...FindQuery\n  }\n}": types.FindAttributesFragmentDoc,
    "fragment Group on Group {\n  id\n  name\n}": types.GroupFragmentDoc,
    "fragment MetadataIdName on Metadata {\n  __typename\n  id\n  version\n  slug\n  name\n  content {\n    type\n  }\n}\n\nfragment Metadata on Metadata {\n  __typename\n  id\n  version\n  slug\n  name\n  labels\n  languageTag\n  public\n  publicContent\n  publicSupplementary\n  parentId\n  type\n  source {\n    id\n    identifier\n  }\n  categories {\n    ...Category\n  }\n  content {\n    ...MetadataContent\n  }\n  created\n  modified\n  uploaded\n  ready\n  attributes\n  systemAttributes\n  traitIds\n  workflow {\n    ...MetadataWorkflow\n  }\n  supplementary {\n    ...MetadataSupplementary\n  }\n  profiles {\n    ...MetadataProfile\n  }\n}": types.MetadataIdNameFragmentDoc,
    "fragment MetadataContent on MetadataContent {\n  type\n  length\n  urls {\n    download {\n      url\n      headers {\n        name\n        value\n      }\n    }\n  }\n}\n\nfragment MetadataContentUpload on MetadataContent {\n  urls {\n    upload {\n      url\n      headers {\n        name\n        value\n      }\n    }\n  }\n}": types.MetadataContentFragmentDoc,
    "fragment MetadataProfile on MetadataProfile {\n  relationship\n  profile {\n    ...Profile\n  }\n}": types.MetadataProfileFragmentDoc,
    "fragment MetadataRelationshipMetadata on Metadata {\n  id\n  version\n  name\n  public\n  publicContent\n  workflow {\n    pending\n    state\n  }\n}\n\nfragment MetadataRelationship on MetadataRelationship {\n  metadata {\n    ...MetadataRelationshipMetadata\n  }\n  relationship\n  attributes\n}": types.MetadataRelationshipMetadataFragmentDoc,
    "fragment MetadataSupplementary on MetadataSupplementary {\n  key\n  name\n  uploaded\n  attributes\n  content {\n    ...MetadataSupplementaryContent\n  }\n  source {\n    id\n    identifier\n  }\n}\n\nfragment MetadataSupplementaryContent on MetadataSupplementaryContent {\n  type\n  length\n  urls {\n    download {\n      url\n      headers {\n        name\n        value\n      }\n    }\n  }\n}": types.MetadataSupplementaryFragmentDoc,
    "fragment MetadataWorkflow on MetadataWorkflow {\n  state\n  stateValid\n  pending\n}": types.MetadataWorkflowFragmentDoc,
    "fragment Model on Model {\n  id\n  name\n  type\n  description\n  configuration\n}": types.ModelFragmentDoc,
    "fragment Ordering on Ordering {\n  path\n  order\n}": types.OrderingFragmentDoc,
    "fragment ParentCollection on Collection {\n  id\n  name\n  attributes\n}": types.ParentCollectionFragmentDoc,
    "fragment Permission on Permission {\n  action\n  group {\n    ...Group\n  }\n}": types.PermissionFragmentDoc,
    "fragment PlanWorkflow on Workflow {\n  id\n  name\n}": types.PlanWorkflowFragmentDoc,
    "fragment Principal on Principal {\n  id\n  verified\n  groups {\n    ...Group\n  }\n}": types.PrincipalFragmentDoc,
    "fragment ProfileIdName on Profile {\n  __typename\n  id\n  name\n}\n\nfragment Profile on Profile {\n  __typename\n  id\n  slug\n  name\n  visibility\n  attributes {\n    id\n    typeId\n    visibility\n    attributes\n    metadata {\n      id\n      content {\n        urls {\n          download {\n            url\n            headers {\n              name\n              value\n            }\n          }\n        }\n      }\n    }\n  }\n}": types.ProfileIdNameFragmentDoc,
    "fragment Prompt on Prompt {\n  id\n  name\n  description\n  inputType\n  outputType\n  systemPrompt\n  userPrompt\n}": types.PromptFragmentDoc,
    "fragment StorageSystem on StorageSystem {\n  id\n  name\n  type\n  description\n  configuration\n  models {\n    modelId\n    configuration\n  }\n}": types.StorageSystemFragmentDoc,
    "fragment TemplateAttribute on TemplateAttribute {\n  key\n  name\n  description\n  type\n  supplementaryKey\n  ui\n  list\n  configuration\n  workflows {\n    ...TemplateWorkflow\n  }\n}": types.TemplateAttributeFragmentDoc,
    "fragment TemplateWorkflow on TemplateWorkflow {\n  autoRun\n  workflow {\n    id\n    name\n  }\n}": types.TemplateWorkflowFragmentDoc,
    "fragment Trait on Trait {\n  id\n  name\n  description\n  contentTypes\n  workflowIds\n  deleteWorkflowId\n}": types.TraitFragmentDoc,
    "fragment Transition on Transition {\n  fromStateId\n  toStateId\n  description\n}": types.TransitionFragmentDoc,
    "fragment Workflow on Workflow {\n  id\n  queue\n  name\n  description\n  configuration\n}": types.WorkflowFragmentDoc,
    "fragment WorkflowActivity on WorkflowActivity {\n  id\n  activityId\n  queue\n  executionGroup\n  inputs {\n    ...WorkflowActivityParameter\n  }\n  outputs {\n    ...WorkflowActivityParameter\n  }\n  configuration\n  storageSystems {\n    ...WorkflowActivityStorageSystem\n  }\n  models {\n    ...WorkflowActivityModel\n  }\n  prompts {\n    ...WorkflowActivityPrompt\n  }\n}": types.WorkflowActivityFragmentDoc,
    "fragment WorkflowActivityModel on WorkflowActivityModel {\n  model {\n    id\n  }\n  configuration\n}": types.WorkflowActivityModelFragmentDoc,
    "fragment WorkflowActivityParameter on WorkflowActivityParameter {\n  name\n  value\n}": types.WorkflowActivityParameterFragmentDoc,
    "fragment WorkflowActivityPrompt on WorkflowActivityPrompt {\n  prompt {\n    id\n  }\n  configuration\n}": types.WorkflowActivityPromptFragmentDoc,
    "fragment WorkflowExecutionId on WorkflowExecutionId {\n  id\n  queue\n}": types.WorkflowExecutionIdFragmentDoc,
    "fragment WorkflowPlan on WorkflowExecutionPlan {\n  id {\n    ...WorkflowExecutionId\n  }\n  complete\n  active\n  failed\n  error\n  cancelled\n  workflow {\n    ...PlanWorkflow\n  }\n}": types.WorkflowPlanFragmentDoc,
    "fragment WorkflowState on WorkflowState {\n  id\n  name\n  configuration\n  description\n  entryWorkflowId\n  exitWorkflowId\n  workflowId\n  type\n}": types.WorkflowStateFragmentDoc,
    "fragment WorkflowActivityStorageSystem on WorkflowActivityStorageSystem {\n  system {\n    id\n  }\n  configuration\n}": types.WorkflowActivityStorageSystemFragmentDoc,
};

/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 *
 *
 * @example
 * ```ts
 * const query = graphql(`query GetUser($id: ID!) { user(id: $id) { name } }`);
 * ```
 *
 * The query argument is unknown!
 * Please regenerate the types.
 */
export function graphql(source: string): unknown;

/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "mutation AddActivity($activity: ActivityInput!) {\n  workflows {\n    activities {\n      add(activity: $activity) {\n        ...Activity\n      }\n    }\n  }\n}"): (typeof documents)["mutation AddActivity($activity: ActivityInput!) {\n  workflows {\n    activities {\n      add(activity: $activity) {\n        ...Activity\n      }\n    }\n  }\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "mutation AddCollection($collection: CollectionInput!) {\n  content {\n    collection {\n      add(collection: $collection) {\n        id\n      }\n    }\n  }\n}"): (typeof documents)["mutation AddCollection($collection: CollectionInput!) {\n  content {\n    collection {\n      add(collection: $collection) {\n        id\n      }\n    }\n  }\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "mutation AddCollectionCollection($collectionId: String!, $id: String!) {\n  content {\n    collection {\n      addChildCollection(id: $collectionId, collectionId: $id) {\n        id\n      }\n    }\n  }\n}"): (typeof documents)["mutation AddCollectionCollection($collectionId: String!, $id: String!) {\n  content {\n    collection {\n      addChildCollection(id: $collectionId, collectionId: $id) {\n        id\n      }\n    }\n  }\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "mutation AddCollectionMetadataRelationship($relationship: CollectionMetadataRelationshipInput!) {\n  content {\n    collection {\n      addMetadataRelationship(relationship: $relationship) {\n        metadata {\n          id\n        }\n      }\n    }\n  }\n}"): (typeof documents)["mutation AddCollectionMetadataRelationship($relationship: CollectionMetadataRelationshipInput!) {\n  content {\n    collection {\n      addMetadataRelationship(relationship: $relationship) {\n        metadata {\n          id\n        }\n      }\n    }\n  }\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "mutation AddCollectionPermission($permission: PermissionInput!) {\n  content {\n    collection {\n      addPermission(permission: $permission) {\n        groupId\n        action\n      }\n    }\n  }\n}"): (typeof documents)["mutation AddCollectionPermission($permission: PermissionInput!) {\n  content {\n    collection {\n      addPermission(permission: $permission) {\n        groupId\n        action\n      }\n    }\n  }\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "mutation AddMetadata($metadata: MetadataInput!) {\n  content {\n    metadata {\n      add(metadata: $metadata) {\n        id\n      }\n    }\n  }\n}"): (typeof documents)["mutation AddMetadata($metadata: MetadataInput!) {\n  content {\n    metadata {\n      add(metadata: $metadata) {\n        id\n      }\n    }\n  }\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "mutation AddMetadataCollection($collectionId: String!, $id: String!) {\n  content {\n    collection {\n      addChildMetadata(id: $collectionId, metadataId: $id) {\n        id\n      }\n    }\n  }\n}"): (typeof documents)["mutation AddMetadataCollection($collectionId: String!, $id: String!) {\n  content {\n    collection {\n      addChildMetadata(id: $collectionId, metadataId: $id) {\n        id\n      }\n    }\n  }\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "mutation AddMetadataPermission($permission: PermissionInput!) {\n  content {\n    metadata {\n      addPermission(permission: $permission) {\n        groupId\n        action\n      }\n    }\n  }\n}"): (typeof documents)["mutation AddMetadataPermission($permission: PermissionInput!) {\n  content {\n    metadata {\n      addPermission(permission: $permission) {\n        groupId\n        action\n      }\n    }\n  }\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "mutation AddMetadataRelationship($relationship: MetadataRelationshipInput!) {\n  content {\n    metadata {\n      addRelationship(relationship: $relationship) {\n        id\n      }\n    }\n  }\n}"): (typeof documents)["mutation AddMetadataRelationship($relationship: MetadataRelationshipInput!) {\n  content {\n    metadata {\n      addRelationship(relationship: $relationship) {\n        id\n      }\n    }\n  }\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "mutation AddMetadataTrait($metadataId: String!, $traitId: String!) {\n  content {\n    metadata {\n      addTrait(metadataId: $metadataId, traitId: $traitId) {\n        id {\n          ...WorkflowExecutionId\n        }\n      }\n    }\n  }\n}"): (typeof documents)["mutation AddMetadataTrait($metadataId: String!, $traitId: String!) {\n  content {\n    metadata {\n      addTrait(metadataId: $metadataId, traitId: $traitId) {\n        id {\n          ...WorkflowExecutionId\n        }\n      }\n    }\n  }\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "mutation AddModel($model: ModelInput!) {\n  workflows {\n    models {\n      add(model: $model) {\n        ...Model\n      }\n    }\n  }\n}"): (typeof documents)["mutation AddModel($model: ModelInput!) {\n  workflows {\n    models {\n      add(model: $model) {\n        ...Model\n      }\n    }\n  }\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "mutation AddPersistedQueries($application: String!, $queries: [PersistedQueryInput!]!) {\n  persistedQueries {\n    addAll(application: $application, queries: $queries)\n  }\n}"): (typeof documents)["mutation AddPersistedQueries($application: String!, $queries: [PersistedQueryInput!]!) {\n  persistedQueries {\n    addAll(application: $application, queries: $queries)\n  }\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "mutation AddPrompt($prompt: PromptInput!) {\n  workflows {\n    prompts {\n      add(prompt: $prompt) {\n        ...Prompt\n      }\n    }\n  }\n}"): (typeof documents)["mutation AddPrompt($prompt: PromptInput!) {\n  workflows {\n    prompts {\n      add(prompt: $prompt) {\n        ...Prompt\n      }\n    }\n  }\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "mutation AddState($state: WorkflowStateInput!) {\n  workflows {\n    states {\n      add(state: $state) {\n        ...WorkflowState\n      }\n    }\n  }\n}"): (typeof documents)["mutation AddState($state: WorkflowStateInput!) {\n  workflows {\n    states {\n      add(state: $state) {\n        ...WorkflowState\n      }\n    }\n  }\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "mutation AddStorageSystem($system: StorageSystemInput!) {\n  workflows {\n    storageSystems {\n      add(storageSystem: $system) {\n        ...StorageSystem\n      }\n    }\n  }\n}"): (typeof documents)["mutation AddStorageSystem($system: StorageSystemInput!) {\n  workflows {\n    storageSystems {\n      add(storageSystem: $system) {\n        ...StorageSystem\n      }\n    }\n  }\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "mutation AddTrait($trait: TraitInput!) {\n  workflows {\n    traits {\n      add(model: $trait) {\n        ...Trait\n      }\n    }\n  }\n}"): (typeof documents)["mutation AddTrait($trait: TraitInput!) {\n  workflows {\n    traits {\n      add(model: $trait) {\n        ...Trait\n      }\n    }\n  }\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "mutation AddTransition($transition: TransitionInput!) {\n  workflows {\n    transitions {\n      add(transition: $transition) {\n        ...Transition\n      }\n    }\n  }\n}"): (typeof documents)["mutation AddTransition($transition: TransitionInput!) {\n  workflows {\n    transitions {\n      add(transition: $transition) {\n        ...Transition\n      }\n    }\n  }\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "mutation AddWorkflow($workflow: WorkflowInput!) {\n  workflows {\n    add(workflow: $workflow) {\n      ...Workflow\n    }\n  }\n}"): (typeof documents)["mutation AddWorkflow($workflow: WorkflowInput!) {\n  workflows {\n    add(workflow: $workflow) {\n      ...Workflow\n    }\n  }\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "mutation BeginCollectionTransition($id: String!, $state: String!, $status: String!) {\n  workflows {\n    beginTransition(\n      request: {collectionId: $id, stateId: $state, status: $status, waitForCompletion: true}\n    )\n  }\n}"): (typeof documents)["mutation BeginCollectionTransition($id: String!, $state: String!, $status: String!) {\n  workflows {\n    beginTransition(\n      request: {collectionId: $id, stateId: $state, status: $status, waitForCompletion: true}\n    )\n  }\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "mutation BeginMetadataTransition($id: String!, $version: Int!, $state: String!, $status: String!, $stateValid: DateTime) {\n  workflows {\n    beginTransition(\n      request: {metadataId: $id, version: $version, stateId: $state, stateValid: $stateValid, status: $status, waitForCompletion: true}\n    )\n  }\n}"): (typeof documents)["mutation BeginMetadataTransition($id: String!, $version: Int!, $state: String!, $status: String!, $stateValid: DateTime) {\n  workflows {\n    beginTransition(\n      request: {metadataId: $id, version: $version, stateId: $state, stateValid: $stateValid, status: $status, waitForCompletion: true}\n    )\n  }\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "mutation CancelMetadataWorkflows($id: String!, $version: Int!, $workflowId: String!) {\n  workflows {\n    cancelWorkflows(\n      metadataId: $id\n      metadataVersion: $version\n      workflowId: $workflowId\n    )\n  }\n}"): (typeof documents)["mutation CancelMetadataWorkflows($id: String!, $version: Int!, $workflowId: String!) {\n  workflows {\n    cancelWorkflows(\n      metadataId: $id\n      metadataVersion: $version\n      workflowId: $workflowId\n    )\n  }\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "mutation CancelTransition($collectionId: String, $metadataId: String, $version: Int) {\n  workflows {\n    cancelTransition(\n      collectionId: $collectionId\n      metadataId: $metadataId\n      metadataVersion: $version\n    )\n  }\n}"): (typeof documents)["mutation CancelTransition($collectionId: String, $metadataId: String, $version: Int) {\n  workflows {\n    cancelTransition(\n      collectionId: $collectionId\n      metadataId: $metadataId\n      metadataVersion: $version\n    )\n  }\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "mutation DeleteActivity($id: String!) {\n  workflows {\n    activities {\n      delete(activityId: $id)\n    }\n  }\n}"): (typeof documents)["mutation DeleteActivity($id: String!) {\n  workflows {\n    activities {\n      delete(activityId: $id)\n    }\n  }\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "mutation DeleteCollection($id: String!) {\n  content {\n    collection {\n      delete(id: $id, recursive: true)\n    }\n  }\n}"): (typeof documents)["mutation DeleteCollection($id: String!) {\n  content {\n    collection {\n      delete(id: $id, recursive: true)\n    }\n  }\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "mutation DeleteConfiguration($key: String!) {\n  configurations {\n    deleteConfiguration(key: $key)\n  }\n}"): (typeof documents)["mutation DeleteConfiguration($key: String!) {\n  configurations {\n    deleteConfiguration(key: $key)\n  }\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "mutation DeleteMetadata($id: String!) {\n  content {\n    metadata {\n      delete(metadataId: $id)\n    }\n  }\n}"): (typeof documents)["mutation DeleteMetadata($id: String!) {\n  content {\n    metadata {\n      delete(metadataId: $id)\n    }\n  }\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "mutation DeleteModel($id: String!) {\n  workflows {\n    models {\n      delete(id: $id)\n    }\n  }\n}"): (typeof documents)["mutation DeleteModel($id: String!) {\n  workflows {\n    models {\n      delete(id: $id)\n    }\n  }\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "mutation DeletePrompt($id: String!) {\n  workflows {\n    prompts {\n      delete(id: $id)\n    }\n  }\n}"): (typeof documents)["mutation DeletePrompt($id: String!) {\n  workflows {\n    prompts {\n      delete(id: $id)\n    }\n  }\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "mutation DeleteState($id: String!) {\n  workflows {\n    states {\n      delete(id: $id)\n    }\n  }\n}"): (typeof documents)["mutation DeleteState($id: String!) {\n  workflows {\n    states {\n      delete(id: $id)\n    }\n  }\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "mutation DeleteStorageSystem($id: String!) {\n  workflows {\n    storageSystems {\n      delete(id: $id)\n    }\n  }\n}"): (typeof documents)["mutation DeleteStorageSystem($id: String!) {\n  workflows {\n    storageSystems {\n      delete(id: $id)\n    }\n  }\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "mutation DeleteTrait($id: String!) {\n  workflows {\n    traits {\n      delete(id: $id)\n    }\n  }\n}"): (typeof documents)["mutation DeleteTrait($id: String!) {\n  workflows {\n    traits {\n      delete(id: $id)\n    }\n  }\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "mutation DeleteTransition($fromStateId: String!, $toStateId: String!) {\n  workflows {\n    transitions {\n      delete(fromStateId: $fromStateId, toStateId: $toStateId)\n    }\n  }\n}"): (typeof documents)["mutation DeleteTransition($fromStateId: String!, $toStateId: String!) {\n  workflows {\n    transitions {\n      delete(fromStateId: $fromStateId, toStateId: $toStateId)\n    }\n  }\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "mutation DeleteWorkflow($id: String!) {\n  workflows {\n    delete(id: $id)\n  }\n}"): (typeof documents)["mutation DeleteWorkflow($id: String!) {\n  workflows {\n    delete(id: $id)\n  }\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "mutation EditActivity($input: ActivityInput!) {\n  workflows {\n    activities {\n      edit(activity: $input) {\n        ...Activity\n      }\n    }\n  }\n}"): (typeof documents)["mutation EditActivity($input: ActivityInput!) {\n  workflows {\n    activities {\n      edit(activity: $input) {\n        ...Activity\n      }\n    }\n  }\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "mutation EditCollection($id: String!, $input: CollectionInput!) {\n  content {\n    collection {\n      edit(id: $id, collection: $input) {\n        id\n      }\n    }\n  }\n}"): (typeof documents)["mutation EditCollection($id: String!, $input: CollectionInput!) {\n  content {\n    collection {\n      edit(id: $id, collection: $input) {\n        id\n      }\n    }\n  }\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "mutation EditMetadata($id: String!, $metadata: MetadataInput!) {\n  content {\n    metadata {\n      edit(id: $id, metadata: $metadata) {\n        id\n      }\n    }\n  }\n}"): (typeof documents)["mutation EditMetadata($id: String!, $metadata: MetadataInput!) {\n  content {\n    metadata {\n      edit(id: $id, metadata: $metadata) {\n        id\n      }\n    }\n  }\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "mutation EditModel($id: String!, $input: ModelInput!) {\n  workflows {\n    models {\n      edit(id: $id, model: $input) {\n        ...Model\n      }\n    }\n  }\n}"): (typeof documents)["mutation EditModel($id: String!, $input: ModelInput!) {\n  workflows {\n    models {\n      edit(id: $id, model: $input) {\n        ...Model\n      }\n    }\n  }\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "mutation EditPrompt($id: String!, $input: PromptInput!) {\n  workflows {\n    prompts {\n      edit(id: $id, prompt: $input) {\n        ...Prompt\n      }\n    }\n  }\n}"): (typeof documents)["mutation EditPrompt($id: String!, $input: PromptInput!) {\n  workflows {\n    prompts {\n      edit(id: $id, prompt: $input) {\n        ...Prompt\n      }\n    }\n  }\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "mutation EditState($input: WorkflowStateInput!) {\n  workflows {\n    states {\n      edit(state: $input) {\n        ...WorkflowState\n      }\n    }\n  }\n}"): (typeof documents)["mutation EditState($input: WorkflowStateInput!) {\n  workflows {\n    states {\n      edit(state: $input) {\n        ...WorkflowState\n      }\n    }\n  }\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "mutation EditStorageSystem($id: String!, $input: StorageSystemInput!) {\n  workflows {\n    storageSystems {\n      edit(id: $id, storageSystem: $input) {\n        ...StorageSystem\n      }\n    }\n  }\n}"): (typeof documents)["mutation EditStorageSystem($id: String!, $input: StorageSystemInput!) {\n  workflows {\n    storageSystems {\n      edit(id: $id, storageSystem: $input) {\n        ...StorageSystem\n      }\n    }\n  }\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "mutation EditTrait($input: TraitInput!) {\n  workflows {\n    traits {\n      edit(model: $input) {\n        ...Trait\n      }\n    }\n  }\n}"): (typeof documents)["mutation EditTrait($input: TraitInput!) {\n  workflows {\n    traits {\n      edit(model: $input) {\n        ...Trait\n      }\n    }\n  }\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "mutation EditTransition($input: TransitionInput!) {\n  workflows {\n    transitions {\n      edit(transition: $input) {\n        ...Transition\n      }\n    }\n  }\n}"): (typeof documents)["mutation EditTransition($input: TransitionInput!) {\n  workflows {\n    transitions {\n      edit(transition: $input) {\n        ...Transition\n      }\n    }\n  }\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "mutation EditWorkflow($input: WorkflowInput!) {\n  workflows {\n    edit(workflow: $input) {\n      id\n    }\n  }\n}"): (typeof documents)["mutation EditWorkflow($input: WorkflowInput!) {\n  workflows {\n    edit(workflow: $input) {\n      id\n    }\n  }\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "mutation EnqueueWorkflow($workflowId: String!, $collectionId: String, $metadataId: String, $version: Int, $configurations: [WorkflowConfigurationInput!]) {\n  workflows {\n    enqueueWorkflow(\n      workflowId: $workflowId\n      collectionId: $collectionId\n      metadataId: $metadataId\n      version: $version\n      configurations: $configurations\n    ) {\n      id\n      queue\n    }\n  }\n}"): (typeof documents)["mutation EnqueueWorkflow($workflowId: String!, $collectionId: String, $metadataId: String, $version: Int, $configurations: [WorkflowConfigurationInput!]) {\n  workflows {\n    enqueueWorkflow(\n      workflowId: $workflowId\n      collectionId: $collectionId\n      metadataId: $metadataId\n      version: $version\n      configurations: $configurations\n    ) {\n      id\n      queue\n    }\n  }\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "query ExecuteSearch($query: String!, $filter: String, $offset: Int!, $limit: Int!, $storageSystemId: String!) {\n  search(\n    query: {query: $query, filter: $filter, offset: $offset, limit: $limit, storageSystemId: $storageSystemId}\n  ) {\n    documents {\n      collection {\n        ...CollectionIdName\n      }\n      metadata {\n        ...MetadataIdName\n      }\n      profile {\n        ...ProfileIdName\n      }\n    }\n  }\n}"): (typeof documents)["query ExecuteSearch($query: String!, $filter: String, $offset: Int!, $limit: Int!, $storageSystemId: String!) {\n  search(\n    query: {query: $query, filter: $filter, offset: $offset, limit: $limit, storageSystemId: $storageSystemId}\n  ) {\n    documents {\n      collection {\n        ...CollectionIdName\n      }\n      metadata {\n        ...MetadataIdName\n      }\n      profile {\n        ...ProfileIdName\n      }\n    }\n  }\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "query FindCollections($query: FindQueryInput!) {\n  content {\n    findCollections(query: $query) {\n      ...Collection\n    }\n  }\n}"): (typeof documents)["query FindCollections($query: FindQueryInput!) {\n  content {\n    findCollections(query: $query) {\n      ...Collection\n    }\n  }\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "query FindCollectionsCount($query: FindQueryInput!) {\n  content {\n    findCollectionsCount(query: $query)\n  }\n}"): (typeof documents)["query FindCollectionsCount($query: FindQueryInput!) {\n  content {\n    findCollectionsCount(query: $query)\n  }\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "query FindMetadata($query: FindQueryInput!) {\n  content {\n    findMetadata(query: $query) {\n      ...Metadata\n    }\n  }\n}"): (typeof documents)["query FindMetadata($query: FindQueryInput!) {\n  content {\n    findMetadata(query: $query) {\n      ...Metadata\n    }\n  }\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "query FindMetadataCount($query: FindQueryInput!) {\n  content {\n    findMetadataCount(query: $query)\n  }\n}"): (typeof documents)["query FindMetadataCount($query: FindQueryInput!) {\n  content {\n    findMetadataCount(query: $query)\n  }\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "query GetActivities {\n  workflows {\n    activities {\n      all {\n        ...Activity\n      }\n    }\n  }\n}"): (typeof documents)["query GetActivities {\n  workflows {\n    activities {\n      all {\n        ...Activity\n      }\n    }\n  }\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "query GetActivity($id: String!) {\n  workflows {\n    activities {\n      activity(id: $id) {\n        ...Activity\n      }\n    }\n  }\n}"): (typeof documents)["query GetActivity($id: String!) {\n  workflows {\n    activities {\n      activity(id: $id) {\n        ...Activity\n      }\n    }\n  }\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "query GetCollection($id: String) {\n  content {\n    collection(id: $id) {\n      ...Collection\n    }\n  }\n}"): (typeof documents)["query GetCollection($id: String) {\n  content {\n    collection(id: $id) {\n      ...Collection\n    }\n  }\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "query GetCollectionChildrenCollections($id: String!, $offset: Int!, $limit: Int!) {\n  content {\n    collection(id: $id) {\n      collections(limit: $limit, offset: $offset) {\n        ...Collection\n      }\n      collectionsCount\n    }\n  }\n}"): (typeof documents)["query GetCollectionChildrenCollections($id: String!, $offset: Int!, $limit: Int!) {\n  content {\n    collection(id: $id) {\n      collections(limit: $limit, offset: $offset) {\n        ...Collection\n      }\n      collectionsCount\n    }\n  }\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "query GetCollectionChildrenMetadata($id: String!, $offset: Int!, $limit: Int!) {\n  content {\n    collection(id: $id) {\n      metadata(limit: $limit, offset: $offset) {\n        ...Metadata\n      }\n      metadataCount\n    }\n  }\n}"): (typeof documents)["query GetCollectionChildrenMetadata($id: String!, $offset: Int!, $limit: Int!) {\n  content {\n    collection(id: $id) {\n      metadata(limit: $limit, offset: $offset) {\n        ...Metadata\n      }\n      metadataCount\n    }\n  }\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "query GetCollectionList($id: String) {\n  content {\n    collection(id: $id) {\n      ...CollectionList\n    }\n  }\n}"): (typeof documents)["query GetCollectionList($id: String) {\n  content {\n    collection(id: $id) {\n      ...CollectionList\n    }\n  }\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "query GetCollectionMetadataRelationships($id: String!) {\n  content {\n    collection(id: $id) {\n      metadataRelationships {\n        ...CollectionMetadataRelationship\n      }\n    }\n  }\n}"): (typeof documents)["query GetCollectionMetadataRelationships($id: String!) {\n  content {\n    collection(id: $id) {\n      metadataRelationships {\n        ...CollectionMetadataRelationship\n      }\n    }\n  }\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "query GetCollectionParents($id: String) {\n  content {\n    collection(id: $id) {\n      ...CollectionParents\n    }\n  }\n}"): (typeof documents)["query GetCollectionParents($id: String) {\n  content {\n    collection(id: $id) {\n      ...CollectionParents\n    }\n  }\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "query GetCollectionPermissions($id: String) {\n  content {\n    collection(id: $id) {\n      permissions {\n        ...Permission\n      }\n    }\n  }\n}"): (typeof documents)["query GetCollectionPermissions($id: String) {\n  content {\n    collection(id: $id) {\n      permissions {\n        ...Permission\n      }\n    }\n  }\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "query GetCollectionTemplate($id: String!, $version: Int) {\n  content {\n    metadata(id: $id, version: $version) {\n      collectionTemplate {\n        ...CollectionTemplate\n      }\n    }\n  }\n}"): (typeof documents)["query GetCollectionTemplate($id: String!, $version: Int) {\n  content {\n    metadata(id: $id, version: $version) {\n      collectionTemplate {\n        ...CollectionTemplate\n      }\n    }\n  }\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "query GetCollectionWorkflowPlans($id: String) {\n  content {\n    collection(id: $id) {\n      workflow {\n        plans {\n          ...WorkflowPlan\n        }\n      }\n    }\n  }\n}"): (typeof documents)["query GetCollectionWorkflowPlans($id: String) {\n  content {\n    collection(id: $id) {\n      workflow {\n        plans {\n          ...WorkflowPlan\n        }\n      }\n    }\n  }\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "query GetConfiguration($key: String!) {\n  configurations {\n    configuration(key: $key) {\n      ...Configuration\n    }\n  }\n}"): (typeof documents)["query GetConfiguration($key: String!) {\n  configurations {\n    configuration(key: $key) {\n      ...Configuration\n    }\n  }\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "query GetConfigurations {\n  configurations {\n    all {\n      ...Configuration\n    }\n  }\n}"): (typeof documents)["query GetConfigurations {\n  configurations {\n    all {\n      ...Configuration\n    }\n  }\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "query GetCurrentProfile {\n  profiles {\n    current {\n      ...Profile\n    }\n  }\n}"): (typeof documents)["query GetCurrentProfile {\n  profiles {\n    current {\n      ...Profile\n    }\n  }\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "query GetGroups($offset: Int!, $limit: Int!) {\n  security {\n    groups {\n      all(offset: $offset, limit: $limit) {\n        ...Group\n      }\n    }\n  }\n}"): (typeof documents)["query GetGroups($offset: Int!, $limit: Int!) {\n  security {\n    groups {\n      all(offset: $offset, limit: $limit) {\n        ...Group\n      }\n    }\n  }\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "query GetMetadata($id: String!, $version: Int) {\n  content {\n    metadata(id: $id, version: $version) {\n      ...Metadata\n    }\n  }\n}\n\nquery GetMetadataUpload($id: String!) {\n  content {\n    metadata(id: $id) {\n      content {\n        ...MetadataContentUpload\n      }\n    }\n  }\n}"): (typeof documents)["query GetMetadata($id: String!, $version: Int) {\n  content {\n    metadata(id: $id, version: $version) {\n      ...Metadata\n    }\n  }\n}\n\nquery GetMetadataUpload($id: String!) {\n  content {\n    metadata(id: $id) {\n      content {\n        ...MetadataContentUpload\n      }\n    }\n  }\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "query GetMetadataDocument($id: String!) {\n  content {\n    metadata(id: $id) {\n      document {\n        ...Document\n      }\n    }\n  }\n}"): (typeof documents)["query GetMetadataDocument($id: String!) {\n  content {\n    metadata(id: $id) {\n      document {\n        ...Document\n      }\n    }\n  }\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "query GetMetadataDocumentTemplate($id: String!, $version: Int) {\n  content {\n    metadata(id: $id, version: $version) {\n      documentTemplate {\n        ...DocumentTemplate\n      }\n    }\n  }\n}"): (typeof documents)["query GetMetadataDocumentTemplate($id: String!, $version: Int) {\n  content {\n    metadata(id: $id, version: $version) {\n      documentTemplate {\n        ...DocumentTemplate\n      }\n    }\n  }\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "query GetMetadataParents($id: String!) {\n  content {\n    metadata(id: $id) {\n      parentCollections(offset: 0, limit: 100) {\n        ...ParentCollection\n      }\n    }\n  }\n}"): (typeof documents)["query GetMetadataParents($id: String!) {\n  content {\n    metadata(id: $id) {\n      parentCollections(offset: 0, limit: 100) {\n        ...ParentCollection\n      }\n    }\n  }\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "query GetMetadataPermissions($id: String!) {\n  content {\n    metadata(id: $id) {\n      permissions {\n        ...Permission\n      }\n    }\n  }\n}"): (typeof documents)["query GetMetadataPermissions($id: String!) {\n  content {\n    metadata(id: $id) {\n      permissions {\n        ...Permission\n      }\n    }\n  }\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "query GetMetadataRelationships($id: String!) {\n  content {\n    metadata(id: $id) {\n      relationships {\n        ...MetadataRelationship\n      }\n    }\n  }\n}"): (typeof documents)["query GetMetadataRelationships($id: String!) {\n  content {\n    metadata(id: $id) {\n      relationships {\n        ...MetadataRelationship\n      }\n    }\n  }\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "query GetMetadataSupplementary($id: String!, $key: String) {\n  content {\n    metadata(id: $id) {\n      supplementary(key: $key) {\n        ...MetadataSupplementary\n      }\n    }\n  }\n}"): (typeof documents)["query GetMetadataSupplementary($id: String!, $key: String) {\n  content {\n    metadata(id: $id) {\n      supplementary(key: $key) {\n        ...MetadataSupplementary\n      }\n    }\n  }\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "query GetMetadataSupplementaryJson($id: String!, $key: String!) {\n  content {\n    metadata(id: $id) {\n      supplementary(key: $key) {\n        content {\n          json\n        }\n      }\n    }\n  }\n}"): (typeof documents)["query GetMetadataSupplementaryJson($id: String!, $key: String!) {\n  content {\n    metadata(id: $id) {\n      supplementary(key: $key) {\n        content {\n          json\n        }\n      }\n    }\n  }\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "query GetMetadataSupplementaryText($id: String!, $key: String!) {\n  content {\n    metadata(id: $id) {\n      supplementary(key: $key) {\n        content {\n          text\n        }\n      }\n    }\n  }\n}"): (typeof documents)["query GetMetadataSupplementaryText($id: String!, $key: String!) {\n  content {\n    metadata(id: $id) {\n      supplementary(key: $key) {\n        content {\n          text\n        }\n      }\n    }\n  }\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "query GetMetadataWorkflowPlans($id: String!) {\n  content {\n    metadata(id: $id) {\n      workflow {\n        plans {\n          ...WorkflowPlan\n        }\n      }\n    }\n  }\n}"): (typeof documents)["query GetMetadataWorkflowPlans($id: String!) {\n  content {\n    metadata(id: $id) {\n      workflow {\n        plans {\n          ...WorkflowPlan\n        }\n      }\n    }\n  }\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "query GetModel($id: String!) {\n  workflows {\n    models {\n      model(id: $id) {\n        ...Model\n      }\n    }\n  }\n}"): (typeof documents)["query GetModel($id: String!) {\n  workflows {\n    models {\n      model(id: $id) {\n        ...Model\n      }\n    }\n  }\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "query GetModels {\n  workflows {\n    models {\n      all {\n        ...Model\n      }\n    }\n  }\n}"): (typeof documents)["query GetModels {\n  workflows {\n    models {\n      all {\n        ...Model\n      }\n    }\n  }\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "query GetPermissionActions {\n  security {\n    actions\n  }\n}"): (typeof documents)["query GetPermissionActions {\n  security {\n    actions\n  }\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "query GetPrincipals($offset: Int!, $limit: Int!) {\n  security {\n    principals {\n      all(offset: $offset, limit: $limit) {\n        ...Principal\n      }\n    }\n  }\n}"): (typeof documents)["query GetPrincipals($offset: Int!, $limit: Int!) {\n  security {\n    principals {\n      all(offset: $offset, limit: $limit) {\n        ...Principal\n      }\n    }\n  }\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "query GetProfile($id: String!) {\n  profiles {\n    profile(id: $id) {\n      ...Profile\n    }\n  }\n}"): (typeof documents)["query GetProfile($id: String!) {\n  profiles {\n    profile(id: $id) {\n      ...Profile\n    }\n  }\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "query GetProfiles($offset: Int!, $limit: Int!) {\n  profiles {\n    all(offset: $offset, limit: $limit) {\n      ...Profile\n    }\n  }\n}"): (typeof documents)["query GetProfiles($offset: Int!, $limit: Int!) {\n  profiles {\n    all(offset: $offset, limit: $limit) {\n      ...Profile\n    }\n  }\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "query GetPrompt($id: String!) {\n  workflows {\n    prompts {\n      prompt(id: $id) {\n        ...Prompt\n      }\n    }\n  }\n}"): (typeof documents)["query GetPrompt($id: String!) {\n  workflows {\n    prompts {\n      prompt(id: $id) {\n        ...Prompt\n      }\n    }\n  }\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "query GetPrompts {\n  workflows {\n    prompts {\n      all {\n        ...Prompt\n      }\n    }\n  }\n}"): (typeof documents)["query GetPrompts {\n  workflows {\n    prompts {\n      all {\n        ...Prompt\n      }\n    }\n  }\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "query GetState($id: String!) {\n  workflows {\n    states {\n      state(id: $id) {\n        ...WorkflowState\n      }\n    }\n  }\n}"): (typeof documents)["query GetState($id: String!) {\n  workflows {\n    states {\n      state(id: $id) {\n        ...WorkflowState\n      }\n    }\n  }\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "query GetStates {\n  workflows {\n    states {\n      all {\n        ...WorkflowState\n      }\n    }\n  }\n}"): (typeof documents)["query GetStates {\n  workflows {\n    states {\n      all {\n        ...WorkflowState\n      }\n    }\n  }\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "query GetStorageSystem($id: String!) {\n  workflows {\n    storageSystems {\n      storageSystem(id: $id) {\n        ...StorageSystem\n      }\n    }\n  }\n}"): (typeof documents)["query GetStorageSystem($id: String!) {\n  workflows {\n    storageSystems {\n      storageSystem(id: $id) {\n        ...StorageSystem\n      }\n    }\n  }\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "query GetStorageSystems {\n  workflows {\n    storageSystems {\n      all {\n        ...StorageSystem\n      }\n    }\n  }\n}"): (typeof documents)["query GetStorageSystems {\n  workflows {\n    storageSystems {\n      all {\n        ...StorageSystem\n      }\n    }\n  }\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "query GetSupplementaryTextContents($id: String!, $key: String!) {\n  content {\n    metadata(id: $id) {\n      supplementary(key: $key) {\n        content {\n          text\n        }\n      }\n    }\n  }\n}"): (typeof documents)["query GetSupplementaryTextContents($id: String!, $key: String!) {\n  content {\n    metadata(id: $id) {\n      supplementary(key: $key) {\n        content {\n          text\n        }\n      }\n    }\n  }\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "query GetTextContents($id: String!) {\n  content {\n    metadata(id: $id) {\n      content {\n        text\n      }\n    }\n  }\n}"): (typeof documents)["query GetTextContents($id: String!) {\n  content {\n    metadata(id: $id) {\n      content {\n        text\n      }\n    }\n  }\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "query GetTrait($id: String!) {\n  workflows {\n    traits {\n      trait(id: $id) {\n        ...Trait\n      }\n    }\n  }\n}"): (typeof documents)["query GetTrait($id: String!) {\n  workflows {\n    traits {\n      trait(id: $id) {\n        ...Trait\n      }\n    }\n  }\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "query GetTraits {\n  workflows {\n    traits {\n      all {\n        ...Trait\n      }\n    }\n  }\n}"): (typeof documents)["query GetTraits {\n  workflows {\n    traits {\n      all {\n        ...Trait\n      }\n    }\n  }\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "query GetTransition($fromStateId: String!, $toStateId: String!) {\n  workflows {\n    transitions {\n      transition(fromStateId: $fromStateId, toStateId: $toStateId) {\n        ...Transition\n      }\n    }\n  }\n}"): (typeof documents)["query GetTransition($fromStateId: String!, $toStateId: String!) {\n  workflows {\n    transitions {\n      transition(fromStateId: $fromStateId, toStateId: $toStateId) {\n        ...Transition\n      }\n    }\n  }\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "query GetTransitions {\n  workflows {\n    transitions {\n      all {\n        ...Transition\n      }\n    }\n  }\n}"): (typeof documents)["query GetTransitions {\n  workflows {\n    transitions {\n      all {\n        ...Transition\n      }\n    }\n  }\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "query GetWorkflow($id: String!) {\n  workflows {\n    workflow(id: $id) {\n      ...Workflow\n      activities {\n        ...WorkflowActivity\n      }\n    }\n  }\n}"): (typeof documents)["query GetWorkflow($id: String!) {\n  workflows {\n    workflow(id: $id) {\n      ...Workflow\n      activities {\n        ...WorkflowActivity\n      }\n    }\n  }\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "query GetWorkflowActivities($id: String!) {\n  workflows {\n    workflow(id: $id) {\n      activities {\n        ...WorkflowActivity\n      }\n    }\n  }\n}"): (typeof documents)["query GetWorkflowActivities($id: String!) {\n  workflows {\n    workflow(id: $id) {\n      activities {\n        ...WorkflowActivity\n      }\n    }\n  }\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "query GetWorkflowActivity($id: Int!) {\n  workflows {\n    workflowActivity(id: $id) {\n      ...WorkflowActivity\n    }\n  }\n}"): (typeof documents)["query GetWorkflowActivity($id: Int!) {\n  workflows {\n    workflowActivity(id: $id) {\n      ...WorkflowActivity\n    }\n  }\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "query GetWorkflows {\n  workflows {\n    all {\n      ...Workflow\n    }\n  }\n}"): (typeof documents)["query GetWorkflows {\n  workflows {\n    all {\n      ...Workflow\n    }\n  }\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "mutation Login($identifier: String!, $password: String!) {\n  security {\n    login {\n      password(identifier: $identifier, password: $password) {\n        profile {\n          ...Profile\n        }\n        principal {\n          id\n          groups {\n            id\n            name\n          }\n        }\n        token {\n          token\n        }\n      }\n    }\n  }\n}"): (typeof documents)["mutation Login($identifier: String!, $password: String!) {\n  security {\n    login {\n      password(identifier: $identifier, password: $password) {\n        profile {\n          ...Profile\n        }\n        principal {\n          id\n          groups {\n            id\n            name\n          }\n        }\n        token {\n          token\n        }\n      }\n    }\n  }\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "query NextJob($queue: String!) {\n  workflows {\n    nextJob(queue: $queue) {\n      planId {\n        id\n        queue\n      }\n      id {\n        id\n        index\n        queue\n      }\n      collection {\n        ...Collection\n      }\n      metadata {\n        ...Metadata\n      }\n      activity {\n        ...Activity\n      }\n      context\n      workflowActivity {\n        ...WorkflowActivity\n      }\n      storageSystems {\n        system {\n          ...StorageSystem\n        }\n        configuration\n      }\n      prompts {\n        prompt {\n          ...Prompt\n        }\n        configuration\n      }\n      models {\n        model {\n          ...Model\n        }\n        configuration\n      }\n    }\n  }\n}"): (typeof documents)["query NextJob($queue: String!) {\n  workflows {\n    nextJob(queue: $queue) {\n      planId {\n        id\n        queue\n      }\n      id {\n        id\n        index\n        queue\n      }\n      collection {\n        ...Collection\n      }\n      metadata {\n        ...Metadata\n      }\n      activity {\n        ...Activity\n      }\n      context\n      workflowActivity {\n        ...WorkflowActivity\n      }\n      storageSystems {\n        system {\n          ...StorageSystem\n        }\n        configuration\n      }\n      prompts {\n        prompt {\n          ...Prompt\n        }\n        configuration\n      }\n      models {\n        model {\n          ...Model\n        }\n        configuration\n      }\n    }\n  }\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "subscription OnActivityChanged {\n  activity\n}"): (typeof documents)["subscription OnActivityChanged {\n  activity\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "subscription OnCollectionChanged {\n  collection\n}"): (typeof documents)["subscription OnCollectionChanged {\n  collection\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "subscription OnMetadataChanged {\n  metadata\n}"): (typeof documents)["subscription OnMetadataChanged {\n  metadata\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "subscription OnMetadataSupplementaryChanged {\n  metadataSupplementary {\n    id\n    supplementary\n  }\n}"): (typeof documents)["subscription OnMetadataSupplementaryChanged {\n  metadataSupplementary {\n    id\n    supplementary\n  }\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "subscription OnModelChanged {\n  model\n}"): (typeof documents)["subscription OnModelChanged {\n  model\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "subscription OnPromptChanged {\n  prompt\n}"): (typeof documents)["subscription OnPromptChanged {\n  prompt\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "subscription OnStateChanged {\n  state\n}"): (typeof documents)["subscription OnStateChanged {\n  state\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "subscription OnStorageSystemChanged {\n  storageSystem\n}"): (typeof documents)["subscription OnStorageSystemChanged {\n  storageSystem\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "subscription OnTraitChanged {\n  trait\n}"): (typeof documents)["subscription OnTraitChanged {\n  trait\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "subscription OnWorkflowChanged {\n  workflow\n}"): (typeof documents)["subscription OnWorkflowChanged {\n  workflow\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "mutation RemoveCollectionCollection($collectionId: String!, $id: String!) {\n  content {\n    collection {\n      removeChildCollection(id: $collectionId, collectionId: $id) {\n        id\n      }\n    }\n  }\n}"): (typeof documents)["mutation RemoveCollectionCollection($collectionId: String!, $id: String!) {\n  content {\n    collection {\n      removeChildCollection(id: $collectionId, collectionId: $id) {\n        id\n      }\n    }\n  }\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "mutation RemoveCollectionMetadataRelationship($id: String!, $metadataId: String!, $relationship: String!) {\n  content {\n    collection {\n      deleteMetadataRelationship(\n        id: $id\n        metadataId: $metadataId\n        relationship: $relationship\n      )\n    }\n  }\n}"): (typeof documents)["mutation RemoveCollectionMetadataRelationship($id: String!, $metadataId: String!, $relationship: String!) {\n  content {\n    collection {\n      deleteMetadataRelationship(\n        id: $id\n        metadataId: $metadataId\n        relationship: $relationship\n      )\n    }\n  }\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "mutation RemoveCollectionPermission($permission: PermissionInput!) {\n  content {\n    collection {\n      deletePermission(permission: $permission) {\n        groupId\n        action\n      }\n    }\n  }\n}"): (typeof documents)["mutation RemoveCollectionPermission($permission: PermissionInput!) {\n  content {\n    collection {\n      deletePermission(permission: $permission) {\n        groupId\n        action\n      }\n    }\n  }\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "mutation RemoveMetadataCollection($collectionId: String!, $id: String!) {\n  content {\n    collection {\n      removeChildMetadata(id: $collectionId, metadataId: $id) {\n        id\n      }\n    }\n  }\n}"): (typeof documents)["mutation RemoveMetadataCollection($collectionId: String!, $id: String!) {\n  content {\n    collection {\n      removeChildMetadata(id: $collectionId, metadataId: $id) {\n        id\n      }\n    }\n  }\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "mutation RemoveMetadataPermission($permission: PermissionInput!) {\n  content {\n    metadata {\n      deletePermission(permission: $permission) {\n        groupId\n        action\n      }\n    }\n  }\n}"): (typeof documents)["mutation RemoveMetadataPermission($permission: PermissionInput!) {\n  content {\n    metadata {\n      deletePermission(permission: $permission) {\n        groupId\n        action\n      }\n    }\n  }\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "mutation RemoveMetadataRelationship($id1: String!, $id2: String!, $relationship: String!) {\n  content {\n    metadata {\n      deleteRelationship(id1: $id1, id2: $id2, relationship: $relationship)\n    }\n  }\n}"): (typeof documents)["mutation RemoveMetadataRelationship($id1: String!, $id2: String!, $relationship: String!) {\n  content {\n    metadata {\n      deleteRelationship(id1: $id1, id2: $id2, relationship: $relationship)\n    }\n  }\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "mutation RemoveMetadataTrait($metadataId: String!, $traitId: String!) {\n  content {\n    metadata {\n      deleteTrait(metadataId: $metadataId, traitId: $traitId) {\n        id {\n          id\n          queue\n        }\n      }\n    }\n  }\n}"): (typeof documents)["mutation RemoveMetadataTrait($metadataId: String!, $traitId: String!) {\n  content {\n    metadata {\n      deleteTrait(metadataId: $metadataId, traitId: $traitId) {\n        id {\n          id\n          queue\n        }\n      }\n    }\n  }\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "mutation SetCollectionAttributes($id: String!, $attributes: JSON!) {\n  content {\n    collection {\n      setCollectionAttributes(id: $id, attributes: $attributes)\n    }\n  }\n}"): (typeof documents)["mutation SetCollectionAttributes($id: String!, $attributes: JSON!) {\n  content {\n    collection {\n      setCollectionAttributes(id: $id, attributes: $attributes)\n    }\n  }\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "mutation SetCollectionPublic($id: String!, $public: Boolean!) {\n  content {\n    collection {\n      setPublic(id: $id, public: $public) {\n        id\n      }\n    }\n  }\n}"): (typeof documents)["mutation SetCollectionPublic($id: String!, $public: Boolean!) {\n  content {\n    collection {\n      setPublic(id: $id, public: $public) {\n        id\n      }\n    }\n  }\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "mutation SetCollectionPublicList($id: String!, $public: Boolean!) {\n  content {\n    collection {\n      setPublicList(id: $id, public: $public) {\n        id\n      }\n    }\n  }\n}"): (typeof documents)["mutation SetCollectionPublicList($id: String!, $public: Boolean!) {\n  content {\n    collection {\n      setPublicList(id: $id, public: $public) {\n        id\n      }\n    }\n  }\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "mutation SetCollectionReady($id: String!) {\n  content {\n    collection {\n      setReady(id: $id)\n    }\n  }\n}"): (typeof documents)["mutation SetCollectionReady($id: String!) {\n  content {\n    collection {\n      setReady(id: $id)\n    }\n  }\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "mutation SetConfiguration($configuration: ConfigurationInput!) {\n  configurations {\n    setConfiguration(configuration: $configuration) {\n      ...Configuration\n    }\n  }\n}"): (typeof documents)["mutation SetConfiguration($configuration: ConfigurationInput!) {\n  configurations {\n    setConfiguration(configuration: $configuration) {\n      ...Configuration\n    }\n  }\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "mutation SetContents($id: String!, $contentType: String, $file: Upload!) {\n  content {\n    metadata {\n      setMetadataContents(id: $id, contentType: $contentType, file: $file)\n    }\n  }\n}"): (typeof documents)["mutation SetContents($id: String!, $contentType: String, $file: Upload!) {\n  content {\n    metadata {\n      setMetadataContents(id: $id, contentType: $contentType, file: $file)\n    }\n  }\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "mutation SetJsonContents($id: String!, $contentType: String!, $content: JSON!) {\n  content {\n    metadata {\n      setMetadataJsonContents(id: $id, contentType: $contentType, content: $content)\n    }\n  }\n}"): (typeof documents)["mutation SetJsonContents($id: String!, $contentType: String!, $content: JSON!) {\n  content {\n    metadata {\n      setMetadataJsonContents(id: $id, contentType: $contentType, content: $content)\n    }\n  }\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "mutation SetMetadataAttributes($id: String!, $attributes: JSON!) {\n  content {\n    metadata {\n      setMetadataAttributes(id: $id, attributes: $attributes)\n    }\n  }\n}"): (typeof documents)["mutation SetMetadataAttributes($id: String!, $attributes: JSON!) {\n  content {\n    metadata {\n      setMetadataAttributes(id: $id, attributes: $attributes)\n    }\n  }\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "mutation SetMetadataContentPublic($id: String!, $public: Boolean!) {\n  content {\n    metadata {\n      setPublicContent(id: $id, public: $public) {\n        id\n      }\n    }\n  }\n}"): (typeof documents)["mutation SetMetadataContentPublic($id: String!, $public: Boolean!) {\n  content {\n    metadata {\n      setPublicContent(id: $id, public: $public) {\n        id\n      }\n    }\n  }\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "mutation SetMetadataPublic($id: String!, $public: Boolean!) {\n  content {\n    metadata {\n      setPublic(id: $id, public: $public) {\n        id\n      }\n    }\n  }\n}"): (typeof documents)["mutation SetMetadataPublic($id: String!, $public: Boolean!) {\n  content {\n    metadata {\n      setPublic(id: $id, public: $public) {\n        id\n      }\n    }\n  }\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "mutation SetMetadataReady($id: String!) {\n  content {\n    metadata {\n      setMetadataReady(id: $id)\n    }\n  }\n}"): (typeof documents)["mutation SetMetadataReady($id: String!) {\n  content {\n    metadata {\n      setMetadataReady(id: $id)\n    }\n  }\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "mutation SetMetadataSupplementaryPublic($id: String!, $public: Boolean!) {\n  content {\n    metadata {\n      setPublicSupplementary(id: $id, public: $public) {\n        id\n      }\n    }\n  }\n}"): (typeof documents)["mutation SetMetadataSupplementaryPublic($id: String!, $public: Boolean!) {\n  content {\n    metadata {\n      setPublicSupplementary(id: $id, public: $public) {\n        id\n      }\n    }\n  }\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "mutation SetTextContents($id: String!, $contentType: String!, $content: String!) {\n  content {\n    metadata {\n      setMetadataTextContents(id: $id, contentType: $contentType, content: $content)\n    }\n  }\n}"): (typeof documents)["mutation SetTextContents($id: String!, $contentType: String!, $content: String!) {\n  content {\n    metadata {\n      setMetadataTextContents(id: $id, contentType: $contentType, content: $content)\n    }\n  }\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "mutation SignUp($profile: ProfileInput!, $identifier: String!, $password: String!) {\n  security {\n    signup {\n      password(profile: $profile, identifier: $identifier, password: $password) {\n        id\n      }\n    }\n  }\n}"): (typeof documents)["mutation SignUp($profile: ProfileInput!, $identifier: String!, $password: String!) {\n  security {\n    signup {\n      password(profile: $profile, identifier: $identifier, password: $password) {\n        id\n      }\n    }\n  }\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "mutation VerifySignUp($token: String!) {\n  security {\n    signup {\n      passwordVerify(verificationToken: $token)\n    }\n  }\n}"): (typeof documents)["mutation VerifySignUp($token: String!) {\n  security {\n    signup {\n      passwordVerify(verificationToken: $token)\n    }\n  }\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "fragment Activity on Activity {\n  childWorkflowId\n  configuration\n  description\n  id\n  inputs {\n    ...ActivityParameter\n  }\n  name\n  outputs {\n    ...ActivityParameter\n  }\n}"): (typeof documents)["fragment Activity on Activity {\n  childWorkflowId\n  configuration\n  description\n  id\n  inputs {\n    ...ActivityParameter\n  }\n  name\n  outputs {\n    ...ActivityParameter\n  }\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "fragment ActivityParameter on ActivityParameter {\n  name\n  type\n}"): (typeof documents)["fragment ActivityParameter on ActivityParameter {\n  name\n  type\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "fragment Category on Category {\n  id\n  name\n}"): (typeof documents)["fragment Category on Category {\n  id\n  name\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "fragment CollectionIdName on Collection {\n  __typename\n  id\n  name\n}\n\nfragment CollectionList on Collection {\n  ...Collection\n  items(offset: 0, limit: 1000) {\n    __typename\n    ... on Collection {\n      ...Collection\n    }\n    ... on Metadata {\n      ...Metadata\n    }\n  }\n}\n\nfragment Collection on Collection {\n  __typename\n  id\n  slug\n  traitIds\n  collectionType: type\n  name\n  description\n  labels\n  created\n  modified\n  attributes\n  systemAttributes\n  ready\n  public\n  publicList\n  templateMetadata {\n    id\n    version\n  }\n  ordering {\n    ...Ordering\n  }\n  categories {\n    ...Category\n  }\n  workflow {\n    ...CollectionWorkflow\n  }\n}\n\nfragment CollectionParents on Collection {\n  parentCollections(offset: 0, limit: 100) {\n    ...ParentCollection\n  }\n}\n\nfragment CollectionPermissions on Collection {\n  permissions {\n    ...Permission\n  }\n}"): (typeof documents)["fragment CollectionIdName on Collection {\n  __typename\n  id\n  name\n}\n\nfragment CollectionList on Collection {\n  ...Collection\n  items(offset: 0, limit: 1000) {\n    __typename\n    ... on Collection {\n      ...Collection\n    }\n    ... on Metadata {\n      ...Metadata\n    }\n  }\n}\n\nfragment Collection on Collection {\n  __typename\n  id\n  slug\n  traitIds\n  collectionType: type\n  name\n  description\n  labels\n  created\n  modified\n  attributes\n  systemAttributes\n  ready\n  public\n  publicList\n  templateMetadata {\n    id\n    version\n  }\n  ordering {\n    ...Ordering\n  }\n  categories {\n    ...Category\n  }\n  workflow {\n    ...CollectionWorkflow\n  }\n}\n\nfragment CollectionParents on Collection {\n  parentCollections(offset: 0, limit: 100) {\n    ...ParentCollection\n  }\n}\n\nfragment CollectionPermissions on Collection {\n  permissions {\n    ...Permission\n  }\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "fragment CollectionDetail on Collection {\n  ...Collection\n  items(offset: 0, limit: 1000) {\n    __typename\n    ... on Collection {\n      ...Collection\n    }\n    ... on Metadata {\n      ...Metadata\n    }\n  }\n}"): (typeof documents)["fragment CollectionDetail on Collection {\n  ...Collection\n  items(offset: 0, limit: 1000) {\n    __typename\n    ... on Collection {\n      ...Collection\n    }\n    ... on Metadata {\n      ...Metadata\n    }\n  }\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "fragment CollectionMetadataRelationship on CollectionMetadataRelationship {\n  metadata {\n    ...MetadataRelationshipMetadata\n  }\n  relationship\n  attributes\n}"): (typeof documents)["fragment CollectionMetadataRelationship on CollectionMetadataRelationship {\n  metadata {\n    ...MetadataRelationshipMetadata\n  }\n  relationship\n  attributes\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "fragment CollectionTemplate on CollectionTemplate {\n  configuration\n  defaultAttributes\n  collectionFilter {\n    options {\n      ...FindQueryOption\n    }\n  }\n  attributes {\n    key\n    name\n    description\n    type\n    supplementaryKey\n    ui\n    list\n    configuration\n    workflows {\n      workflow {\n        ...Workflow\n      }\n      autoRun\n    }\n  }\n}"): (typeof documents)["fragment CollectionTemplate on CollectionTemplate {\n  configuration\n  defaultAttributes\n  collectionFilter {\n    options {\n      ...FindQueryOption\n    }\n  }\n  attributes {\n    key\n    name\n    description\n    type\n    supplementaryKey\n    ui\n    list\n    configuration\n    workflows {\n      workflow {\n        ...Workflow\n      }\n      autoRun\n    }\n  }\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "fragment CollectionWorkflow on CollectionWorkflow {\n  state\n  pending\n}"): (typeof documents)["fragment CollectionWorkflow on CollectionWorkflow {\n  state\n  pending\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "fragment Configuration on Configuration {\n  id\n  key\n  description\n  value\n  permissions {\n    action\n    group {\n      id\n      name\n    }\n  }\n}"): (typeof documents)["fragment Configuration on Configuration {\n  id\n  key\n  description\n  value\n  permissions {\n    action\n    group {\n      id\n      name\n    }\n  }\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "fragment Document on Document {\n  template {\n    id\n    version\n  }\n  title\n  content\n}"): (typeof documents)["fragment Document on Document {\n  template {\n    id\n    version\n  }\n  title\n  content\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "fragment DocumentTemplate on DocumentTemplate {\n  configuration\n  schema\n  content\n  defaultAttributes\n  containers {\n    ...DocumentTemplateContainer\n  }\n  attributes {\n    ...TemplateAttribute\n  }\n}"): (typeof documents)["fragment DocumentTemplate on DocumentTemplate {\n  configuration\n  schema\n  content\n  defaultAttributes\n  containers {\n    ...DocumentTemplateContainer\n  }\n  attributes {\n    ...TemplateAttribute\n  }\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "fragment DocumentTemplateContainer on DocumentTemplateContainer {\n  id\n  name\n  description\n  supplementaryKey\n  workflows {\n    ...TemplateWorkflow\n  }\n}"): (typeof documents)["fragment DocumentTemplateContainer on DocumentTemplateContainer {\n  id\n  name\n  description\n  supplementaryKey\n  workflows {\n    ...TemplateWorkflow\n  }\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "fragment FindAttributes on FindAttributes {\n  attributes {\n    ...FindAttribute\n  }\n}\n\nfragment FindAttribute on FindAttribute {\n  key\n  value\n}\n\nfragment FindQuery on FindQuery {\n  attributes {\n    ...FindAttributes\n  }\n  categoryIds\n  collectionType\n  contentTypes\n  extensionFilter\n  offset\n  limit\n}\n\nfragment FindQueryOption on FindQueryOption {\n  name\n  query {\n    ...FindQuery\n  }\n}"): (typeof documents)["fragment FindAttributes on FindAttributes {\n  attributes {\n    ...FindAttribute\n  }\n}\n\nfragment FindAttribute on FindAttribute {\n  key\n  value\n}\n\nfragment FindQuery on FindQuery {\n  attributes {\n    ...FindAttributes\n  }\n  categoryIds\n  collectionType\n  contentTypes\n  extensionFilter\n  offset\n  limit\n}\n\nfragment FindQueryOption on FindQueryOption {\n  name\n  query {\n    ...FindQuery\n  }\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "fragment Group on Group {\n  id\n  name\n}"): (typeof documents)["fragment Group on Group {\n  id\n  name\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "fragment MetadataIdName on Metadata {\n  __typename\n  id\n  version\n  slug\n  name\n  content {\n    type\n  }\n}\n\nfragment Metadata on Metadata {\n  __typename\n  id\n  version\n  slug\n  name\n  labels\n  languageTag\n  public\n  publicContent\n  publicSupplementary\n  parentId\n  type\n  source {\n    id\n    identifier\n  }\n  categories {\n    ...Category\n  }\n  content {\n    ...MetadataContent\n  }\n  created\n  modified\n  uploaded\n  ready\n  attributes\n  systemAttributes\n  traitIds\n  workflow {\n    ...MetadataWorkflow\n  }\n  supplementary {\n    ...MetadataSupplementary\n  }\n  profiles {\n    ...MetadataProfile\n  }\n}"): (typeof documents)["fragment MetadataIdName on Metadata {\n  __typename\n  id\n  version\n  slug\n  name\n  content {\n    type\n  }\n}\n\nfragment Metadata on Metadata {\n  __typename\n  id\n  version\n  slug\n  name\n  labels\n  languageTag\n  public\n  publicContent\n  publicSupplementary\n  parentId\n  type\n  source {\n    id\n    identifier\n  }\n  categories {\n    ...Category\n  }\n  content {\n    ...MetadataContent\n  }\n  created\n  modified\n  uploaded\n  ready\n  attributes\n  systemAttributes\n  traitIds\n  workflow {\n    ...MetadataWorkflow\n  }\n  supplementary {\n    ...MetadataSupplementary\n  }\n  profiles {\n    ...MetadataProfile\n  }\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "fragment MetadataContent on MetadataContent {\n  type\n  length\n  urls {\n    download {\n      url\n      headers {\n        name\n        value\n      }\n    }\n  }\n}\n\nfragment MetadataContentUpload on MetadataContent {\n  urls {\n    upload {\n      url\n      headers {\n        name\n        value\n      }\n    }\n  }\n}"): (typeof documents)["fragment MetadataContent on MetadataContent {\n  type\n  length\n  urls {\n    download {\n      url\n      headers {\n        name\n        value\n      }\n    }\n  }\n}\n\nfragment MetadataContentUpload on MetadataContent {\n  urls {\n    upload {\n      url\n      headers {\n        name\n        value\n      }\n    }\n  }\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "fragment MetadataProfile on MetadataProfile {\n  relationship\n  profile {\n    ...Profile\n  }\n}"): (typeof documents)["fragment MetadataProfile on MetadataProfile {\n  relationship\n  profile {\n    ...Profile\n  }\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "fragment MetadataRelationshipMetadata on Metadata {\n  id\n  version\n  name\n  public\n  publicContent\n  workflow {\n    pending\n    state\n  }\n}\n\nfragment MetadataRelationship on MetadataRelationship {\n  metadata {\n    ...MetadataRelationshipMetadata\n  }\n  relationship\n  attributes\n}"): (typeof documents)["fragment MetadataRelationshipMetadata on Metadata {\n  id\n  version\n  name\n  public\n  publicContent\n  workflow {\n    pending\n    state\n  }\n}\n\nfragment MetadataRelationship on MetadataRelationship {\n  metadata {\n    ...MetadataRelationshipMetadata\n  }\n  relationship\n  attributes\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "fragment MetadataSupplementary on MetadataSupplementary {\n  key\n  name\n  uploaded\n  attributes\n  content {\n    ...MetadataSupplementaryContent\n  }\n  source {\n    id\n    identifier\n  }\n}\n\nfragment MetadataSupplementaryContent on MetadataSupplementaryContent {\n  type\n  length\n  urls {\n    download {\n      url\n      headers {\n        name\n        value\n      }\n    }\n  }\n}"): (typeof documents)["fragment MetadataSupplementary on MetadataSupplementary {\n  key\n  name\n  uploaded\n  attributes\n  content {\n    ...MetadataSupplementaryContent\n  }\n  source {\n    id\n    identifier\n  }\n}\n\nfragment MetadataSupplementaryContent on MetadataSupplementaryContent {\n  type\n  length\n  urls {\n    download {\n      url\n      headers {\n        name\n        value\n      }\n    }\n  }\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "fragment MetadataWorkflow on MetadataWorkflow {\n  state\n  stateValid\n  pending\n}"): (typeof documents)["fragment MetadataWorkflow on MetadataWorkflow {\n  state\n  stateValid\n  pending\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "fragment Model on Model {\n  id\n  name\n  type\n  description\n  configuration\n}"): (typeof documents)["fragment Model on Model {\n  id\n  name\n  type\n  description\n  configuration\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "fragment Ordering on Ordering {\n  path\n  order\n}"): (typeof documents)["fragment Ordering on Ordering {\n  path\n  order\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "fragment ParentCollection on Collection {\n  id\n  name\n  attributes\n}"): (typeof documents)["fragment ParentCollection on Collection {\n  id\n  name\n  attributes\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "fragment Permission on Permission {\n  action\n  group {\n    ...Group\n  }\n}"): (typeof documents)["fragment Permission on Permission {\n  action\n  group {\n    ...Group\n  }\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "fragment PlanWorkflow on Workflow {\n  id\n  name\n}"): (typeof documents)["fragment PlanWorkflow on Workflow {\n  id\n  name\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "fragment Principal on Principal {\n  id\n  verified\n  groups {\n    ...Group\n  }\n}"): (typeof documents)["fragment Principal on Principal {\n  id\n  verified\n  groups {\n    ...Group\n  }\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "fragment ProfileIdName on Profile {\n  __typename\n  id\n  name\n}\n\nfragment Profile on Profile {\n  __typename\n  id\n  slug\n  name\n  visibility\n  attributes {\n    id\n    typeId\n    visibility\n    attributes\n    metadata {\n      id\n      content {\n        urls {\n          download {\n            url\n            headers {\n              name\n              value\n            }\n          }\n        }\n      }\n    }\n  }\n}"): (typeof documents)["fragment ProfileIdName on Profile {\n  __typename\n  id\n  name\n}\n\nfragment Profile on Profile {\n  __typename\n  id\n  slug\n  name\n  visibility\n  attributes {\n    id\n    typeId\n    visibility\n    attributes\n    metadata {\n      id\n      content {\n        urls {\n          download {\n            url\n            headers {\n              name\n              value\n            }\n          }\n        }\n      }\n    }\n  }\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "fragment Prompt on Prompt {\n  id\n  name\n  description\n  inputType\n  outputType\n  systemPrompt\n  userPrompt\n}"): (typeof documents)["fragment Prompt on Prompt {\n  id\n  name\n  description\n  inputType\n  outputType\n  systemPrompt\n  userPrompt\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "fragment StorageSystem on StorageSystem {\n  id\n  name\n  type\n  description\n  configuration\n  models {\n    modelId\n    configuration\n  }\n}"): (typeof documents)["fragment StorageSystem on StorageSystem {\n  id\n  name\n  type\n  description\n  configuration\n  models {\n    modelId\n    configuration\n  }\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "fragment TemplateAttribute on TemplateAttribute {\n  key\n  name\n  description\n  type\n  supplementaryKey\n  ui\n  list\n  configuration\n  workflows {\n    ...TemplateWorkflow\n  }\n}"): (typeof documents)["fragment TemplateAttribute on TemplateAttribute {\n  key\n  name\n  description\n  type\n  supplementaryKey\n  ui\n  list\n  configuration\n  workflows {\n    ...TemplateWorkflow\n  }\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "fragment TemplateWorkflow on TemplateWorkflow {\n  autoRun\n  workflow {\n    id\n    name\n  }\n}"): (typeof documents)["fragment TemplateWorkflow on TemplateWorkflow {\n  autoRun\n  workflow {\n    id\n    name\n  }\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "fragment Trait on Trait {\n  id\n  name\n  description\n  contentTypes\n  workflowIds\n  deleteWorkflowId\n}"): (typeof documents)["fragment Trait on Trait {\n  id\n  name\n  description\n  contentTypes\n  workflowIds\n  deleteWorkflowId\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "fragment Transition on Transition {\n  fromStateId\n  toStateId\n  description\n}"): (typeof documents)["fragment Transition on Transition {\n  fromStateId\n  toStateId\n  description\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "fragment Workflow on Workflow {\n  id\n  queue\n  name\n  description\n  configuration\n}"): (typeof documents)["fragment Workflow on Workflow {\n  id\n  queue\n  name\n  description\n  configuration\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "fragment WorkflowActivity on WorkflowActivity {\n  id\n  activityId\n  queue\n  executionGroup\n  inputs {\n    ...WorkflowActivityParameter\n  }\n  outputs {\n    ...WorkflowActivityParameter\n  }\n  configuration\n  storageSystems {\n    ...WorkflowActivityStorageSystem\n  }\n  models {\n    ...WorkflowActivityModel\n  }\n  prompts {\n    ...WorkflowActivityPrompt\n  }\n}"): (typeof documents)["fragment WorkflowActivity on WorkflowActivity {\n  id\n  activityId\n  queue\n  executionGroup\n  inputs {\n    ...WorkflowActivityParameter\n  }\n  outputs {\n    ...WorkflowActivityParameter\n  }\n  configuration\n  storageSystems {\n    ...WorkflowActivityStorageSystem\n  }\n  models {\n    ...WorkflowActivityModel\n  }\n  prompts {\n    ...WorkflowActivityPrompt\n  }\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "fragment WorkflowActivityModel on WorkflowActivityModel {\n  model {\n    id\n  }\n  configuration\n}"): (typeof documents)["fragment WorkflowActivityModel on WorkflowActivityModel {\n  model {\n    id\n  }\n  configuration\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "fragment WorkflowActivityParameter on WorkflowActivityParameter {\n  name\n  value\n}"): (typeof documents)["fragment WorkflowActivityParameter on WorkflowActivityParameter {\n  name\n  value\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "fragment WorkflowActivityPrompt on WorkflowActivityPrompt {\n  prompt {\n    id\n  }\n  configuration\n}"): (typeof documents)["fragment WorkflowActivityPrompt on WorkflowActivityPrompt {\n  prompt {\n    id\n  }\n  configuration\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "fragment WorkflowExecutionId on WorkflowExecutionId {\n  id\n  queue\n}"): (typeof documents)["fragment WorkflowExecutionId on WorkflowExecutionId {\n  id\n  queue\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "fragment WorkflowPlan on WorkflowExecutionPlan {\n  id {\n    ...WorkflowExecutionId\n  }\n  complete\n  active\n  failed\n  error\n  cancelled\n  workflow {\n    ...PlanWorkflow\n  }\n}"): (typeof documents)["fragment WorkflowPlan on WorkflowExecutionPlan {\n  id {\n    ...WorkflowExecutionId\n  }\n  complete\n  active\n  failed\n  error\n  cancelled\n  workflow {\n    ...PlanWorkflow\n  }\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "fragment WorkflowState on WorkflowState {\n  id\n  name\n  configuration\n  description\n  entryWorkflowId\n  exitWorkflowId\n  workflowId\n  type\n}"): (typeof documents)["fragment WorkflowState on WorkflowState {\n  id\n  name\n  configuration\n  description\n  entryWorkflowId\n  exitWorkflowId\n  workflowId\n  type\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "fragment WorkflowActivityStorageSystem on WorkflowActivityStorageSystem {\n  system {\n    id\n  }\n  configuration\n}"): (typeof documents)["fragment WorkflowActivityStorageSystem on WorkflowActivityStorageSystem {\n  system {\n    id\n  }\n  configuration\n}"];

export function graphql(source: string) {
  return (documents as any)[source] ?? {};
}

export type DocumentType<TDocumentNode extends DocumentNode<any, any>> = TDocumentNode extends DocumentNode<  infer TType,  any>  ? TType  : never;