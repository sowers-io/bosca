package io.bosca.workflow

import io.bosca.api.Client
import io.bosca.installer.Installer

object EnterpriseActivityRegistryFactory {

    fun createRegistry(client: Client): ActivityRegistry? {
        return EnterpriseActivityRegistry(client)
    }

    fun createInstaller(client: Client): Installer? {
        return EnterpriseActivityRegistry(client)
    }
}