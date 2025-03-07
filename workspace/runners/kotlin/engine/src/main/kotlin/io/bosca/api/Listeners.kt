package io.bosca.api

import io.bosca.graphql.*
import kotlinx.coroutines.flow.Flow
import kotlinx.coroutines.flow.map

class Listeners(network: NetworkClient) : Api(network) {

    fun onMetadataChanged(): Flow<String?> {
        val response = network.graphql.subscription(OnMetadataChangedSubscription()).toFlow().map {
            it.data?.metadata
        }
        return response
    }

    fun onCollectionChanged(): Flow<String?> {
        val response = network.graphql.subscription(OnCollectionChangedSubscription()).toFlow().map {
            it.data?.collection
        }
        return response
    }

    fun onWorkflowChanged(): Flow<String?> {
        val response = network.graphql.subscription(OnWorkflowChangedSubscription()).toFlow().map {
            it.data?.workflow
        }
        return response
    }

    fun onActivityChanged(): Flow<String?> {
        val response = network.graphql.subscription(OnActivityChangedSubscription()).toFlow().map {
            it.data?.activity
        }
        return response
    }

    fun onTraitChanged(): Flow<String?> {
        val response = network.graphql.subscription(OnTraitChangedSubscription()).toFlow().map {
            it.data?.trait
        }
        return response
    }

    fun onStorageSystemChanged(): Flow<String?> {
        val response = network.graphql.subscription(OnStorageSystemChangedSubscription()).toFlow().map {
            it.data?.storageSystem
        }
        return response
    }

    fun onModelChanged(): Flow<String?> {
        val response = network.graphql.subscription(OnModelChangedSubscription()).toFlow().map {
            it.data?.model
        }
        return response
    }

    fun onPromptChanged(): Flow<String?> {
        val response = network.graphql.subscription(OnPromptChangedSubscription()).toFlow().map {
            it.data?.prompt
        }
        return response
    }

    fun onStateChanged(): Flow<String?> {
        val response = network.graphql.subscription(OnStateChangedSubscription()).toFlow().map {
            it.data?.state
        }
        return response
    }

    fun onTransitionChanged(): Flow<OnTransitionChangedSubscription.Transition?> {
        val response = network.graphql.subscription(OnTransitionChangedSubscription()).toFlow().map {
            it.data?.transition
        }
        return response
    }
}