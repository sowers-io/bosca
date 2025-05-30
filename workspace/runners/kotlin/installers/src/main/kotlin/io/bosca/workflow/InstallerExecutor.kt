package io.bosca.workflow

import io.bosca.api.Client
import io.bosca.installer.Installer
import io.bosca.installer.InstallerExecutor
import io.bosca.workflow.installers.*

object InstallerExecutorFactory {

    fun create(
        client: Client, installers: List<Installer> = listOf(
            ConfigurationsInstaller(),
            CategoriesInstaller(client),
            PromptsInstaller(),
            ModelsInstaller(),
            StorageSystemsInstaller(),
            ActivitiesInstaller(client),
            WorkflowsInstaller(),
            StatesInstaller(),
            TransitionsInstaller(),
            TraitsInstaller(),
            GroupsInstaller(),
            CollectionsInstaller(client),
            ProfileAttributeTypesInstaller(),
            StorageSystemsInstaller(),
            AssetsInstaller(),
            StorageSystemsInstaller() // ensure storage systems get initialized now that everything is in place
        )
    ): InstallerExecutor {
        EnterpriseActivityRegistryFactory.createInstaller(client)?.let {
            return InstallerExecutor(listOf(it) + installers)
        }
        return InstallerExecutor(installers)
    }
}
