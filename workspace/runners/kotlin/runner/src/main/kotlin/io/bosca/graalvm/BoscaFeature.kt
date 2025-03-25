package io.bosca.graalvm

import org.graalvm.nativeimage.hosted.Feature
import org.graalvm.nativeimage.hosted.RuntimeReflection

class BoscaFeature : Feature {

    private val registered = mutableSetOf<String>()

    override fun beforeAnalysis(access: Feature.BeforeAnalysisAccess) {
        // Jsonata classes
        register("com.dashjoin.jsonata.Jsonata", access)
        register("com.dashjoin.jsonata.JFunction", access)
        register("com.dashjoin.jsonata.Utils", access)
        register("com.dashjoin.jsonata.Expressions", access)
        register("com.dashjoin.jsonata.Functions", access)

        register("io.bosca.graphql.fragment.WorkflowJob", access, true)

        // Meilisearch classes
        register("com.meilisearch.sdk.Index", access)
        register("com.meilisearch.sdk.exceptions.APIError", access)
        register("com.meilisearch.sdk.model.EmbedderInputType", access)
        register("com.meilisearch.sdk.model.EmbedderSource", access)
        register("com.meilisearch.sdk.model.Embedders", access)
        register("com.meilisearch.sdk.model.FacetSortValue", access)
        register("com.meilisearch.sdk.model.Faceting", access)
        register("com.meilisearch.sdk.model.LocalizedAttribute", access)
        register("com.meilisearch.sdk.model.Pagination", access)
        register("com.meilisearch.sdk.model.Settings", access)
        register("com.meilisearch.sdk.model.SwapIndexesParams", access)
        register("com.meilisearch.sdk.model.Task", access)
        register("com.meilisearch.sdk.model.TaskDetails", access)
        register("com.meilisearch.sdk.model.TaskError", access)
        register("com.meilisearch.sdk.model.TaskInfo", access)
        register("com.meilisearch.sdk.model.TaskStatus", access)
        register("com.meilisearch.sdk.model.TypoTolerance", access)

        // Security providers
        register("apple.security.AppleProvider", access)
        register(
            "com.github.ajalt.mordant.terminal.terminalinterface.ffm.TerminalInterfaceProviderFfm",
            access
        )
        register(
            "com.github.ajalt.mordant.terminal.terminalinterface.jna.TerminalInterfaceProviderJna",
            access
        )
        register(
            "com.github.ajalt.mordant.terminal.terminalinterface.nativeimage.TerminalInterfaceProviderNativeImage",
            access
        )
        register("com.github.ajalt.mordant.terminal.StandardTerminalInterface", access)
        register("com.github.ajalt.mordant.internal.MppInternal_jvmKt", access)
        register("com.github.ajalt.mordant.terminal.Terminal", access)
        register("com.github.ajalt.clikt.core.MordantContextKt", access)
        register("com.github.ajalt.clikt.core.BaseCliktCommand", access)
        register("com.github.ajalt.clikt.command.CoreSuspendingCliktCommandKt", access)
        register("com.oracle.svm.core.jdk.JDK11OrLater", access)
        register("com.sun.crypto.provider.AESCipher\$General", access)
        register("com.sun.crypto.provider.ARCFOURCipher", access)
        register("com.sun.crypto.provider.ChaCha20Cipher\$ChaCha20Poly1305", access)
        register("com.sun.crypto.provider.DESCipher", access)
        register("com.sun.crypto.provider.DESedeCipher", access)
        register("com.sun.crypto.provider.GaloisCounterMode\$AESGCM", access)

        // JNA and other utilities
        register("com.sun.jna.CallbackProxy", access)
        register("com.sun.org.apache.xerces.internal.jaxp.SAXParserFactoryImpl", access)
        register("com.sun.xml.internal.stream.XMLInputFactoryImpl", access)
        register("java.security.KeyStoreSpi", access)
        register("jdk.internal.misc.Unsafe", access)

        // Register fields in Kotlin classes
        try {
            val safePublicationLazyImplClass = Class.forName("kotlin.SafePublicationLazyImpl")
            RuntimeReflection.register(safePublicationLazyImplClass)

            val valueField = safePublicationLazyImplClass.getDeclaredField("_value")
            RuntimeReflection.register(valueField)
        } catch (e: Exception) {
            println("Could not register SafePublicationLazyImpl fields: ${e.message}")
        }

        register("kotlin.jvm.internal.DefaultConstructorMarker", access)
        register("kotlin.reflect.jvm.internal.ReflectionFactoryImpl", access)

        // Register fields in Kotlin coroutines classes
        registerCoroutinesClasses(access)

        register("com.fasterxml.jackson.databind.ext.Java7SupportImpl", access)
        register("com.fasterxml.jackson.databind.PropertyNamingStrategies", access)

        // Register security classes
        registerSecurityProviders(access)

        // Register JNI proxies
        registerProxies(access)
    }

    private fun registerCoroutinesClasses(access: Feature.BeforeAnalysisAccess) {
        // Register fields in CancellableContinuationImpl
        try {
            val cancellableContinuationImplClass = Class.forName("kotlinx.coroutines.CancellableContinuationImpl")
            RuntimeReflection.register(cancellableContinuationImplClass)

            val fields = listOf(
                "_decisionAndIndex\$volatile",
                "_parentHandle\$volatile",
                "_state\$volatile"
            )

            for (fieldName in fields) {
                try {
                    val field = cancellableContinuationImplClass.getDeclaredField(fieldName)
                    RuntimeReflection.register(field)
                } catch (e: NoSuchFieldException) {
                    println("Field $fieldName not found in CancellableContinuationImpl")
                }
            }
        } catch (e: Exception) {
            println("Could not register CancellableContinuationImpl fields: ${e.message}")
        }

        // Register additional Kotlin coroutines classes and fields
        registerCoroutineClass("kotlinx.coroutines.CancelledContinuation", listOf("_resumed\$volatile"), access)
        registerCoroutineClass("kotlinx.coroutines.CompletedExceptionally", listOf("_handled\$volatile"), access)
        registerCoroutineClass("kotlinx.coroutines.DispatchedCoroutine", listOf("_decision\$volatile"), access)
        registerCoroutineClass(
            "kotlinx.coroutines.EventLoopImplBase",
            listOf("_delayed\$volatile", "_isCompleted\$volatile", "_queue\$volatile"), access
        )
        registerCoroutineClass(
            "kotlinx.coroutines.JobSupport",
            listOf("_parentHandle\$volatile", "_state\$volatile"), access
        )
        registerCoroutineClass(
            "kotlinx.coroutines.JobSupport\$Finishing",
            listOf("_exceptionsHolder\$volatile", "_isCompleting\$volatile", "_rootCause\$volatile"), access
        )
        registerCoroutineClass(
            "kotlinx.coroutines.channels.BufferedChannel",
            listOf(
                "_closeCause\$volatile", "bufferEnd\$volatile", "bufferEndSegment\$volatile",
                "closeHandler\$volatile", "completedExpandBuffersAndPauseFlag\$volatile",
                "receiveSegment\$volatile", "receivers\$volatile", "sendSegment\$volatile",
                "sendersAndCloseStatus\$volatile"
            ), access
        )
        registerCoroutineClass(
            "kotlinx.coroutines.internal.ConcurrentLinkedListNode",
            listOf("_next\$volatile", "_prev\$volatile"), access
        )
        registerCoroutineClass(
            "kotlinx.coroutines.internal.DispatchedContinuation",
            listOf("_reusableCancellableContinuation\$volatile"), access
        )
        registerCoroutineClass(
            "kotlinx.coroutines.internal.LimitedDispatcher",
            listOf("runningWorkers\$volatile"), access
        )
        registerCoroutineClass(
            "kotlinx.coroutines.internal.LockFreeLinkedListNode",
            listOf("_next\$volatile", "_prev\$volatile", "_removedRef\$volatile"), access
        )
        registerCoroutineClass(
            "kotlinx.coroutines.internal.LockFreeTaskQueue",
            listOf("_cur\$volatile"), access
        )
        registerCoroutineClass(
            "kotlinx.coroutines.internal.LockFreeTaskQueueCore",
            listOf("_next\$volatile", "_state\$volatile"), access
        )
        registerCoroutineClass(
            "kotlinx.coroutines.internal.Segment",
            listOf("cleanedAndPointers\$volatile"), access
        )
        registerCoroutineClass(
            "kotlinx.coroutines.internal.ThreadSafeHeap",
            listOf("_size\$volatile"), access
        )
        registerCoroutineClass(
            "kotlinx.coroutines.scheduling.CoroutineScheduler",
            listOf("_isTerminated\$volatile", "controlState\$volatile", "parkedWorkersStack\$volatile"), access
        )
        registerCoroutineClass(
            "kotlinx.coroutines.scheduling.CoroutineScheduler\$Worker",
            listOf("workerCtl\$volatile"), access
        )
        registerCoroutineClass(
            "kotlinx.coroutines.scheduling.WorkQueue",
            listOf(
                "blockingTasksInBuffer\$volatile", "consumerIndex\$volatile",
                "lastScheduledTask\$volatile", "producerIndex\$volatile"
            ), access
        )
        registerCoroutineClass(
            "kotlinx.coroutines.sync.MutexImpl",
            listOf("owner\$volatile"), access
        )
        registerCoroutineClass(
            "kotlinx.coroutines.sync.SemaphoreAndMutexImpl",
            listOf(
                "_availablePermits\$volatile", "deqIdx\$volatile", "enqIdx\$volatile",
                "head\$volatile", "tail\$volatile"
            ), access
        )
    }

    private fun registerSecurityProviders(access: Feature.BeforeAnalysisAccess) {
        // Register Sun security providers
        register("sun.security.pkcs12.PKCS12KeyStore", access)
        register("sun.security.pkcs12.PKCS12KeyStore\$DualFormatPKCS12", access)

        // Register NativePRNG with specific constructor
        try {
            val nativePRNGClass = Class.forName("sun.security.provider.NativePRNG")
            RuntimeReflection.register(nativePRNGClass)

            val parameterClass = Class.forName("java.security.SecureRandomParameters")
            val constructor = nativePRNGClass.getDeclaredConstructor(parameterClass)
            RuntimeReflection.register(constructor)
        } catch (e: Exception) {
            println("Could not register NativePRNG constructor: ${e.message}")
        }

        register("sun.security.provider.SHA", access)
        register("sun.security.provider.X509Factory", access)
        register("sun.security.rsa.RSAKeyFactory\$Legacy", access)
        register("sun.security.ssl.SSLContextImpl\$TLSContext", access)
        register("sun.security.ssl.TrustManagerFactoryImpl\$PKIXFactory", access)

        // Register X509 extensions with boolean+object constructors
        val x509Extensions = listOf(
            "sun.security.x509.AuthorityInfoAccessExtension",
            "sun.security.x509.AuthorityKeyIdentifierExtension",
            "sun.security.x509.BasicConstraintsExtension",
            "sun.security.x509.CRLDistributionPointsExtension",
            "sun.security.x509.CertificatePoliciesExtension",
            "sun.security.x509.ExtendedKeyUsageExtension",
            "sun.security.x509.IssuerAlternativeNameExtension",
            "sun.security.x509.KeyUsageExtension",
            "sun.security.x509.NetscapeCertTypeExtension",
            "sun.security.x509.PrivateKeyUsageExtension",
            "sun.security.x509.SubjectAlternativeNameExtension",
            "sun.security.x509.SubjectKeyIdentifierExtension"
        )

        for (className in x509Extensions) {
            register(className, access)
        }
    }

    private fun registerProxies(access: Feature.BeforeAnalysisAccess) {
        // Register JNI proxies
        val proxyInterfaces = listOf(
            "ai.djl.util.cuda.CudaLibrary",
            "com.github.ajalt.mordant.terminal.terminalinterface.jna.PosixLibC",
            "com.github.ajalt.mordant.terminal.terminalinterface.jna.MacosLibC"
        )

        for (interfaceName in proxyInterfaces) {
            try {
                val interfaceClass = Class.forName(interfaceName)
                RuntimeReflection.register(interfaceClass)
                RuntimeReflection.register(java.lang.reflect.Proxy::class.java)
            } catch (e: Exception) {
                println("Could not register proxy for $interfaceName: ${e.message}")
            }
        }
    }

    private fun registerCoroutineClass(
        className: String,
        fieldNames: List<String>,
        access: Feature.BeforeAnalysisAccess
    ) {
        try {
            val clazz = Class.forName(className)
            RuntimeReflection.register(clazz)

            for (fieldName in fieldNames) {
                try {
                    val field = clazz.getDeclaredField(fieldName)
                    RuntimeReflection.register(field)
                } catch (e: NoSuchFieldException) {
                    println("Field $fieldName not found in $className")
                }
            }
        } catch (e: Exception) {
            println("Could not register $className: ${e.message}")
        }
    }

    private fun register(
        className: String,
        access: Feature.BeforeAnalysisAccess,
        recursive: Boolean = false,
    ) {
        if (registered.contains(className)) return
        registered.add(className)
        try {
            if (className.startsWith("java.lang") || className.startsWith("java.util")) return
            val clazz = Class.forName(className)
            if (clazz.isPrimitive) return
            println("Registering $className...")

            RuntimeReflection.registerClassLookup(className)
            clazz.constructors.forEach { constructor ->
                RuntimeReflection.registerConstructorLookup(
                    clazz,
                    *constructor.parameters.map { it.type }.toTypedArray()
                )
                RuntimeReflection.register(constructor)
            }
            clazz.declaredConstructors.forEach { constructor ->
                RuntimeReflection.registerConstructorLookup(
                    clazz,
                    *constructor.parameters.map { it.type }.toTypedArray()
                )
                RuntimeReflection.register(constructor)
            }
            clazz.methods.forEach { method ->
                RuntimeReflection.registerMethodLookup(
                    clazz,
                    method.name,
                    *method.parameters.map { it.type }.toTypedArray()
                )
                RuntimeReflection.register(method)
                if (recursive) {
                    register(method.returnType.name, access, true)
                }
            }
            clazz.declaredMethods.forEach { method ->
                RuntimeReflection.registerMethodLookup(
                    clazz,
                    method.name,
                    *method.parameters.map { it.type }.toTypedArray()
                )
                RuntimeReflection.register(method)
                if (recursive) {
                    register(method.returnType.name, access, true)
                }
            }
            clazz.fields.forEach { field ->
                RuntimeReflection.registerFieldLookup(
                    clazz,
                    field.name
                )
                RuntimeReflection.register(field)
                if (recursive) {
                    register(field.type.name, access, true)
                }
            }
            clazz.declaredFields.forEach { field ->
                RuntimeReflection.registerFieldLookup(
                    clazz,
                    field.name
                )
                RuntimeReflection.register(field)
                if (recursive) {
                    register(field.type.name, access, true)
                }
            }
            clazz.declaredClasses.forEach { innerClass ->
                RuntimeReflection.register(innerClass)
                if (recursive) {
                    register(innerClass.name, access, true)
                }
            }

            RuntimeReflection.register(clazz)
            RuntimeReflection.registerAllDeclaredConstructors(clazz)
            RuntimeReflection.registerAllConstructors(clazz)
            RuntimeReflection.registerAllDeclaredMethods(clazz)
            RuntimeReflection.registerAllMethods(clazz)
            RuntimeReflection.registerAllDeclaredFields(clazz)
            RuntimeReflection.registerAllFields(clazz)
            RuntimeReflection.registerAllPermittedSubclasses(clazz)
            RuntimeReflection.registerAllNestMembers(clazz)
            RuntimeReflection.registerAllRecordComponents(clazz)
            RuntimeReflection.registerAllSigners(clazz)
            try {
                RuntimeReflection.registerForReflectiveInstantiation(clazz)
            } catch (ignore: IllegalArgumentException) {}
        } catch (e: ClassNotFoundException) {
            println("Class not found: $className")
        }
    }
}