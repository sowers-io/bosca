import { Api } from '~/lib/bosca/api'
import type { NetworkClient } from '~/lib/bosca/networkclient'
import {
  OnCollectionChangedDocument,
  OnMetadataChangedDocument,
  OnMetadataSupplementaryChangedDocument,
  OnTraitChangedDocument,
} from '~/lib/graphql/graphql'

export class Listeners<T extends NetworkClient> extends Api<T> {
  constructor(network: T) {
    super(network)
  }

  onMetadataChanged(onData: (id: string) => void) {
    this.network.subscribe(OnMetadataChangedDocument, null, {
      onData: (data) => {
        if (data) {
          setTimeout(() => onData(data.metadata), 500)
        }
      },
    })
  }

  onMetadataSupplementaryChanged(onData: (id: string, key: string) => void) {
    this.network.subscribe(OnMetadataSupplementaryChangedDocument, null, {
      onData: (data) => {
        if (data) {
          setTimeout(
            () =>
              onData(
                data.metadataSupplementary.id,
                data.metadataSupplementary.supplementary,
              ),
            500,
          )
        }
      },
    })
  }

  onCollectionChanged(onData: (id: string) => void) {
    this.network.subscribe(OnCollectionChangedDocument, null, {
      onData: (data) => {
        if (data) {
          setTimeout(() => onData(data.collection), 500)
        }
      },
    })
  }

  //
  // fun onWorkflowChanged(): Flow<String?> {
  //     val response = network.client.subscription(OnWorkflowChangedSubscription()).toFlow().map {
  //         it.data?.workflow
  //     }
  //     return response
  // }
  //
  // fun onActivityChanged(): Flow<String?> {
  //     val response = network.client.subscription(OnActivityChangedSubscription()).toFlow().map {
  //         it.data?.activity
  //     }
  //     return response
  // }
  //
  // fun onTraitChanged(): Flow<String?> {
  //     val response = network.client.subscription(OnTraitChangedSubscription()).toFlow().map {
  //         it.data?.trait
  //     }
  //     return response
  // }
  onTraitChanged(onData: (id: string) => void) {
    this.network.subscribe(OnTraitChangedDocument, null, {
      onData: (data) => {
        if (data) {
          setTimeout(() => onData(data.trait), 500)
        }
      },
    })
  }
  //
  // fun onStorageSystemChanged(): Flow<String?> {
  //     val response = network.client.subscription(OnStorageSystemChangedSubscription()).toFlow().map {
  //         it.data?.storageSystem
  //     }
  //     return response
  // }
  //
  // fun onModelChanged(): Flow<String?> {
  //     val response = network.client.subscription(OnModelChangedSubscription()).toFlow().map {
  //         it.data?.model
  //     }
  //     return response
  // }
  //
  // fun onPromptChanged(): Flow<String?> {
  //     val response = network.client.subscription(OnPromptChangedSubscription()).toFlow().map {
  //         it.data?.prompt
  //     }
  //     return response
  // }
  //
  // fun onStateChanged(): Flow<String?> {
  //     val response = network.client.subscription(OnStateChangedSubscription()).toFlow().map {
  //         it.data?.state
  //     }
  //     return response
  // }
  //
  // fun onTransitionChanged(): Flow<String?> {
  //     val response = network.client.subscription(OnTransitionChangedSubscription()).toFlow().map {
  //         it.data?.transition
  //     }
  //     return response
  // }
}
