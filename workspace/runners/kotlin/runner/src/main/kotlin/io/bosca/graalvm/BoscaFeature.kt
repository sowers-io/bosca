package io.bosca.graalvm

import org.graalvm.nativeimage.hosted.Feature
import org.graalvm.nativeimage.hosted.RuntimeReflection
import java.lang.reflect.Modifier

/**
 * GraalVM Feature that registers classes and methods that need to be accessible via reflection at runtime.
 * This replaces the previous reachability-metadata.json configuration.
 */
class BoscaFeature : Feature {
    
    override fun beforeAnalysis(access: Feature.BeforeAnalysisAccess) {
        // Jsonata classes
        registerClassWithAllMethods("com.dashjoin.jsonata.Jsonata", access, true)
        registerClassWithAllMethods("com.dashjoin.jsonata.JFunction", access)
        registerClassWithAllMethods("com.dashjoin.jsonata.Utils", access, true)
        registerClassWithAllMethods("com.dashjoin.jsonata.Expressions", access, true)
        registerClassWithAllMethods("com.dashjoin.jsonata.Functions", access, true)
        
        // Meilisearch classes
        registerClass("com.meilisearch.sdk.Index", access, registerFields = true, registerEmptyConstructor = true)
        registerClassWithAllMethods("com.meilisearch.sdk.exceptions.APIError", access, true, registerEmptyConstructor = true)
        registerClassWithAllMethods("com.meilisearch.sdk.model.EmbedderInputType", access, true)
        registerClassWithAllMethods("com.meilisearch.sdk.model.EmbedderSource", access, true)
        registerClassWithAllMethods("com.meilisearch.sdk.model.Embedders", access, true)
        registerClassWithAllMethods("com.meilisearch.sdk.model.FacetSortValue", access, true)
        registerClassWithAllMethods("com.meilisearch.sdk.model.Faceting", access, true, registerEmptyConstructor = true)
        registerClassWithAllMethods("com.meilisearch.sdk.model.LocalizedAttribute", access, true)
        registerClassWithAllMethods("com.meilisearch.sdk.model.Pagination", access, true, registerEmptyConstructor = true)
        registerClassWithAllMethods("com.meilisearch.sdk.model.Settings", access, true, registerEmptyConstructor = true)
        registerClassWithAllMethods("com.meilisearch.sdk.model.SwapIndexesParams", access, true)
        registerClassWithAllMethods("com.meilisearch.sdk.model.Task", access, true, registerEmptyConstructor = true)
        registerClassWithAllMethods("com.meilisearch.sdk.model.TaskDetails", access, true, registerEmptyConstructor = true)
        registerClassWithAllMethods("com.meilisearch.sdk.model.TaskError", access, true, registerEmptyConstructor = true)
        registerClassWithAllMethods("com.meilisearch.sdk.model.TaskInfo", access, true, registerEmptyConstructor = true)
        
        // Register TaskStatus enum with its fields
        try {
            val taskStatusClass = Class.forName("com.meilisearch.sdk.model.TaskStatus")
            RuntimeReflection.register(taskStatusClass)
            RuntimeReflection.registerAllConstructors(taskStatusClass)
            RuntimeReflection.registerAllMethods(taskStatusClass)
            
            // Register enum constants
            val enumConstants = listOf("ENQUEUED", "PROCESSING", "SUCCEEDED", "FAILED", "CANCELED")
            for (constantName in enumConstants) {
                try {
                    val field = taskStatusClass.getDeclaredField(constantName)
                    RuntimeReflection.register(field)
                } catch (e: NoSuchFieldException) {
                    // Field not found, log and continue
                    println("Field $constantName not found in TaskStatus enum")
                }
            }
        } catch (e: ClassNotFoundException) {
            println("TaskStatus class not found")
        }
        
        // Security providers
        registerClassWithEmptyConstructor("apple.security.AppleProvider", access)
        registerClassWithAllMethods("com.github.ajalt.mordant.terminal.terminalinterface.ffm.TerminalInterfaceProviderFfm", access)
        registerClassWithAllMethods("com.github.ajalt.mordant.terminal.terminalinterface.jna.TerminalInterfaceProviderJna", access)
        registerClassWithAllMethods("com.github.ajalt.mordant.terminal.terminalinterface.nativeimage.TerminalInterfaceProviderNativeImage", access)
        registerClassWithAllMethods("com.github.ajalt.mordant.terminal.StandardTerminalInterface", access)
        registerClassWithAllMethods("com.github.ajalt.mordant.internal.MppInternal_jvmKt", access)
        registerClassWithAllMethods("com.github.ajalt.mordant.terminal.Terminal", access)
        registerClassWithAllMethods("com.github.ajalt.clikt.core.MordantContextKt", access)
        registerClassWithAllMethods("com.github.ajalt.clikt.core.BaseCliktCommand", access)
        registerClassWithAllMethods("com.github.ajalt.clikt.command.CoreSuspendingCliktCommandKt", access)
        registerClassWithAllMethods("com.oracle.svm.core.jdk.JDK11OrLater", access)
        registerClassWithEmptyConstructor("com.sun.crypto.provider.AESCipher\$General", access)
        registerClassWithEmptyConstructor("com.sun.crypto.provider.ARCFOURCipher", access)
        registerClassWithEmptyConstructor("com.sun.crypto.provider.ChaCha20Cipher\$ChaCha20Poly1305", access)
        registerClassWithEmptyConstructor("com.sun.crypto.provider.DESCipher", access)
        registerClassWithEmptyConstructor("com.sun.crypto.provider.DESedeCipher", access)
        registerClassWithEmptyConstructor("com.sun.crypto.provider.GaloisCounterMode\$AESGCM", access)
        
        // JNA and other utilities
        registerClass("com.sun.jna.CallbackProxy", access)
        registerClassWithEmptyConstructor("com.sun.org.apache.xerces.internal.jaxp.SAXParserFactoryImpl", access)
        registerClassWithEmptyConstructor("com.sun.xml.internal.stream.XMLInputFactoryImpl", access)
        
        // Java core classes
        registerClass("java.lang.ClassValue", access)
        registerClass("java.lang.Object", access)
        registerClass("java.lang.Throwable", access)
        registerClass("java.lang.invoke.MethodHandle", access)
        registerClass("java.lang.invoke.MethodHandles", access)
        registerClass("java.lang.invoke.MethodHandles\$Lookup", access)
        registerClass("java.lang.invoke.MethodType", access)
        
        // Register java.lang.reflect.Method.isDefault() specifically
        try {
            val methodClass = Class.forName("java.lang.reflect.Method")
            val isDefaultMethod = methodClass.getDeclaredMethod("isDefault")
            RuntimeReflection.register(isDefaultMethod)
        } catch (e: Exception) {
            println("Could not register Method.isDefault(): ${e.message}")
        }
        
        registerClass("java.nio.Buffer", access)
        registerClass("java.security.AlgorithmParametersSpi", access)
        registerClass("java.security.KeyStoreSpi", access)
        
        // Register fields in ForkJoinTask
        try {
            val forkJoinTaskClass = Class.forName("java.util.concurrent.ForkJoinTask")
            RuntimeReflection.register(forkJoinTaskClass)
            
            val auxField = forkJoinTaskClass.getDeclaredField("aux")
            RuntimeReflection.register(auxField)
            
            val statusField = forkJoinTaskClass.getDeclaredField("status")
            RuntimeReflection.register(statusField)
        } catch (e: Exception) {
            println("Could not register ForkJoinTask fields: ${e.message}")
        }
        
        // Register fields in AtomicBoolean
        try {
            val atomicBooleanClass = Class.forName("java.util.concurrent.atomic.AtomicBoolean")
            RuntimeReflection.register(atomicBooleanClass)
            
            val valueField = atomicBooleanClass.getDeclaredField("value")
            RuntimeReflection.register(valueField)
        } catch (e: Exception) {
            println("Could not register AtomicBoolean fields: ${e.message}")
        }
        
        registerClass("jdk.internal.misc.Unsafe", access)
        
        // Register fields in Kotlin classes
        try {
            val safePublicationLazyImplClass = Class.forName("kotlin.SafePublicationLazyImpl")
            RuntimeReflection.register(safePublicationLazyImplClass)
            
            val valueField = safePublicationLazyImplClass.getDeclaredField("_value")
            RuntimeReflection.register(valueField)
        } catch (e: Exception) {
            println("Could not register SafePublicationLazyImpl fields: ${e.message}")
        }
        
        registerClass("kotlin.jvm.internal.DefaultConstructorMarker", access)
        registerClassWithEmptyConstructor("kotlin.reflect.jvm.internal.ReflectionFactoryImpl", access)
        
        // Register fields in Kotlin coroutines classes
        registerCoroutinesClasses(access)
        
        // Register Jackson and Truffle classes
        registerClassWithEmptyConstructor("com.fasterxml.jackson.databind.ext.Java7SupportImpl", access)
        registerTruffleClasses(access)
        
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
        registerCoroutineClass("kotlinx.coroutines.EventLoopImplBase", 
            listOf("_delayed\$volatile", "_isCompleted\$volatile", "_queue\$volatile"), access)
        registerCoroutineClass("kotlinx.coroutines.JobSupport", 
            listOf("_parentHandle\$volatile", "_state\$volatile"), access)
        registerCoroutineClass("kotlinx.coroutines.JobSupport\$Finishing", 
            listOf("_exceptionsHolder\$volatile", "_isCompleting\$volatile", "_rootCause\$volatile"), access)
        registerCoroutineClass("kotlinx.coroutines.channels.BufferedChannel", 
            listOf(
                "_closeCause\$volatile", "bufferEnd\$volatile", "bufferEndSegment\$volatile",
                "closeHandler\$volatile", "completedExpandBuffersAndPauseFlag\$volatile",
                "receiveSegment\$volatile", "receivers\$volatile", "sendSegment\$volatile", 
                "sendersAndCloseStatus\$volatile"
            ), access)
        registerCoroutineClass("kotlinx.coroutines.internal.ConcurrentLinkedListNode", 
            listOf("_next\$volatile", "_prev\$volatile"), access)
        registerCoroutineClass("kotlinx.coroutines.internal.DispatchedContinuation", 
            listOf("_reusableCancellableContinuation\$volatile"), access)
        registerCoroutineClass("kotlinx.coroutines.internal.LimitedDispatcher", 
            listOf("runningWorkers\$volatile"), access)
        registerCoroutineClass("kotlinx.coroutines.internal.LockFreeLinkedListNode", 
            listOf("_next\$volatile", "_prev\$volatile", "_removedRef\$volatile"), access)
        registerCoroutineClass("kotlinx.coroutines.internal.LockFreeTaskQueue", 
            listOf("_cur\$volatile"), access)
        registerCoroutineClass("kotlinx.coroutines.internal.LockFreeTaskQueueCore", 
            listOf("_next\$volatile", "_state\$volatile"), access)
        registerCoroutineClass("kotlinx.coroutines.internal.Segment", 
            listOf("cleanedAndPointers\$volatile"), access)
        registerCoroutineClass("kotlinx.coroutines.internal.ThreadSafeHeap", 
            listOf("_size\$volatile"), access)
        registerCoroutineClass("kotlinx.coroutines.scheduling.CoroutineScheduler", 
            listOf("_isTerminated\$volatile", "controlState\$volatile", "parkedWorkersStack\$volatile"), access)
        registerCoroutineClass("kotlinx.coroutines.scheduling.CoroutineScheduler\$Worker", 
            listOf("workerCtl\$volatile"), access)
        registerCoroutineClass("kotlinx.coroutines.scheduling.WorkQueue", 
            listOf(
                "blockingTasksInBuffer\$volatile", "consumerIndex\$volatile", 
                "lastScheduledTask\$volatile", "producerIndex\$volatile"
            ), access)
        registerCoroutineClass("kotlinx.coroutines.sync.MutexImpl", 
            listOf("owner\$volatile"), access)
        registerCoroutineClass("kotlinx.coroutines.sync.SemaphoreAndMutexImpl", 
            listOf(
                "_availablePermits\$volatile", "deqIdx\$volatile", "enqIdx\$volatile", 
                "head\$volatile", "tail\$volatile"
            ), access)
    }
    
    private fun registerTruffleClasses(access: Feature.BeforeAnalysisAccess) {
        registerClass("com.oracle.truffle.api.debug.impl.DebuggerInstrumentProvider", access)
        
        // Register RootNode with fields
        try {
            val rootNodeClass = Class.forName("com.oracle.truffle.api.nodes.RootNode")
            RuntimeReflection.register(rootNodeClass)
            
            val lockField = rootNodeClass.getDeclaredField("lock")
            RuntimeReflection.register(lockField)
        } catch (e: Exception) {
            println("Could not register RootNode fields: ${e.message}")
        }
        
        registerClass("com.oracle.truffle.js.lang.JavaScriptLanguageProvider", access)
        registerClass("com.oracle.truffle.js.parser.GraalJSEvaluator", access)
        
        // Register JSFunctionData with fields
        try {
            val jsFunctionDataClass = Class.forName("com.oracle.truffle.js.runtime.builtins.JSFunctionData")
            RuntimeReflection.register(jsFunctionDataClass)
            
            val fields = listOf("callTarget", "constructNewTarget", "constructTarget", "rootNode")
            for (fieldName in fields) {
                val field = jsFunctionDataClass.getDeclaredField(fieldName)
                RuntimeReflection.register(field)
            }
        } catch (e: Exception) {
            println("Could not register JSFunctionData fields: ${e.message}")
        }
        
        registerClass("com.oracle.truffle.object.CoreLayoutFactory", access)
        registerClass("com.oracle.truffle.object.DynamicObjectLibraryImplGen\$DynamicObjectLibraryProvider", access)
        
        // Register Shape, TrieTransitionMap, and other Truffle classes with fields
        registerClassWithFields("com.oracle.truffle.object.ShapeImpl", 
            listOf("leafAssumption", "sharedPropertyAssumptions", "transitionMap"), access)
        registerClassWithFields("com.oracle.truffle.object.TrieTransitionMap", 
            listOf("map"), access)
        
        registerClass("com.oracle.truffle.polyglot.PolyglotImpl", access)
        registerClassWithFields("com.oracle.truffle.polyglot.PolyglotLanguageInstance", 
            listOf("guestToHostCodeCache"), access)
        registerClassWithFields("com.oracle.truffle.polyglot.WeakAssumedValue", 
            listOf("profile"), access)
        
        registerClass("com.oracle.truffle.regex.RegexLanguageProvider", access)
        registerClass("com.oracle.truffle.runtime.DefaultLoopNodeFactory", access)
        registerClass("com.oracle.truffle.runtime.LibTruffleAttachResourceProvider", access)
        registerClassWithFields("com.oracle.truffle.runtime.OptimizedCallTarget", 
            listOf(
                "argumentsProfile", "nodeRewritingAssumption", "returnProfile", 
                "speculationLog", "validRootAssumption"
            ), access)
        
        registerClass("com.oracle.truffle.runtime.hotspot.HotSpotTruffleRuntimeAccess", access)
        registerClass("com.oracle.truffle.runtime.jfr.impl.ProviderImpl", access)
    }
    
    private fun registerSecurityProviders(access: Feature.BeforeAnalysisAccess) {
        // Register Sun security providers
        registerClassWithEmptyConstructor("sun.security.pkcs12.PKCS12KeyStore", access)
        registerClassWithEmptyConstructor("sun.security.pkcs12.PKCS12KeyStore\$DualFormatPKCS12", access)
        
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
        
        registerClassWithEmptyConstructor("sun.security.provider.SHA", access)
        registerClassWithEmptyConstructor("sun.security.provider.X509Factory", access)
        registerClassWithEmptyConstructor("sun.security.rsa.RSAKeyFactory\$Legacy", access)
        registerClassWithEmptyConstructor("sun.security.ssl.SSLContextImpl\$TLSContext", access)
        registerClassWithEmptyConstructor("sun.security.ssl.TrustManagerFactoryImpl\$PKIXFactory", access)
        
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
            try {
                val clazz = Class.forName(className)
                RuntimeReflection.register(clazz)
                
                val constructor = clazz.getDeclaredConstructor(
                    Boolean::class.java, 
                    Object::class.java
                )
                RuntimeReflection.register(constructor)
            } catch (e: Exception) {
                println("Could not register $className: ${e.message}")
            }
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
    
    private fun registerCoroutineClass(className: String, fieldNames: List<String>, access: Feature.BeforeAnalysisAccess) {
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
    
    private fun registerClass(className: String, access: Feature.BeforeAnalysisAccess, 
                            registerFields: Boolean = false, 
                            registerEmptyConstructor: Boolean = false) {
        try {
            val clazz = Class.forName(className)
            RuntimeReflection.register(clazz)
            
            if (registerFields) {
                for (field in clazz.declaredFields) {
                    RuntimeReflection.register(field)
                }
            }
            
            if (registerEmptyConstructor) {
                try {
                    val constructor = clazz.getDeclaredConstructor()
                    RuntimeReflection.register(constructor)
                } catch (e: NoSuchMethodException) {
                    println("Empty constructor not found for $className")
                }
            }
        } catch (e: ClassNotFoundException) {
            println("Class not found: $className")
        }
    }
    
    private fun registerClassWithEmptyConstructor(className: String, access: Feature.BeforeAnalysisAccess) {
        try {
            val clazz = Class.forName(className)
            RuntimeReflection.register(clazz)
            
            val constructor = clazz.getDeclaredConstructor()
            RuntimeReflection.register(constructor)
        } catch (e: Exception) {
            println("Could not register $className: ${e.message}")
        }
    }
    
    private fun registerClassWithAllMethods(
        className: String, 
        access: Feature.BeforeAnalysisAccess, 
        unsafeAllocated: Boolean = false,
        registerEmptyConstructor: Boolean = true
    ) {
        try {
            val clazz = Class.forName(className)
            RuntimeReflection.register(clazz)
            RuntimeReflection.registerAllDeclaredConstructors(clazz)
            RuntimeReflection.registerAllConstructors(clazz)
            RuntimeReflection.registerAllDeclaredMethods(clazz)
            RuntimeReflection.registerAllMethods(clazz)
            
            if (unsafeAllocated) {
                // For unsafeAllocated=true, we register all declared fields
                for (field in clazz.declaredFields) {
                    RuntimeReflection.register(field)
                }
            }
            
            if (registerEmptyConstructor) {
                try {
                    val constructor = clazz.getDeclaredConstructor()
                    RuntimeReflection.register(constructor)
                } catch (e: NoSuchMethodException) {
                    println("Empty constructor not found for $className")
                }
            }
        } catch (e: ClassNotFoundException) {
            println("Class not found: $className")
        }
    }
    
    private fun registerClassWithFields(className: String, fieldNames: List<String>, access: Feature.BeforeAnalysisAccess) {
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
}