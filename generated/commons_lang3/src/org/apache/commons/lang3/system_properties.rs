use java::nio::file::Path;
use java::nio::file::Paths;
use java::util::function::BooleanSupplier;
use java::util::function::IntSupplier;
use java::util::function::LongSupplier;
use java::util::function::Supplier;
use crate::org::apache::commons::lang3::function::Suppliers;

pub struct SystemProperties;

impl SystemProperties {
	pub static APPLE_AWT_ENABLE_TEMPLATE_IMAGES: /* Java */ java::lang::String /**/ = "apple.awt.enableTemplateImages";

	pub static AWT_TOOLKIT: /* Java */ java::lang::String /**/ = "awt.toolkit";

	pub static COM_SUN_JNDI_LDAP_OBJECT_TRUST_SERIAL_DATA: /* Java */ java::lang::String /**/ = "com.sun.jndi.ldap.object.trustSerialData";

	pub static COM_SUN_NET_HTTP_SERVER_HTTP_SERVER_PROVIDER: /* Java */ java::lang::String /**/ = "com.sun.net.httpserver.HttpServerProvider";

	pub static FILE_ENCODING: /* Java */ java::lang::String /**/ = "file.encoding";

	pub static FILE_SEPARATOR: /* Java */ java::lang::String /**/ = "file.separator";

	pub static FTP_NON_PROXY_HOST: /* Java */ java::lang::String /**/ = "ftp.nonProxyHosts";

	pub static FTP_PROXY_HOST: /* Java */ java::lang::String /**/ = "ftp.proxyHost";

	pub static FTP_PROXY_PORT: /* Java */ java::lang::String /**/ = "ftp.proxyPort";

	pub static HTTP_AGENT: /* Java */ java::lang::String /**/ = "http.agent";

	pub static HTTP_AUTH_DIGEST_CNONCE_REPEAT: /* Java */ java::lang::String /**/ = "http.auth.digest.cnonceRepeat";

	pub static HTTP_AUTH_DIGEST_RE_ENABLED_ALGORITHMS: /* Java */ java::lang::String /**/ = "http.auth.digest.reEnabledAlgorithms";

	pub static HTTP_AUTH_DIGEST_VALIDATE_PROXY: /* Java */ java::lang::String /**/ = "http.auth.digest.validateProxy";

	pub static HTTP_AUTH_DIGEST_VALIDATE_SERVER: /* Java */ java::lang::String /**/ = "http.auth.digest.validateServer";

	pub static HTTP_AUTH_NTLM_DOMAIN: /* Java */ java::lang::String /**/ = "http.auth.ntlm.domain";

	pub static HTTP_KEEP_ALIVE: /* Java */ java::lang::String /**/ = "http.keepAlive";

	pub static HTTP_KEEP_ALIVE_TIME_PROXY: /* Java */ java::lang::String /**/ = "http.keepAlive.time.proxy";

	pub static HTTP_KEEP_ALIVE_TIME_SERVER: /* Java */ java::lang::String /**/ = "http.keepAlive.time.server";

	pub static HTTP_MAX_CONNECTIONS: /* Java */ java::lang::String /**/ = "http.maxConnections";

	pub static HTTP_MAX_REDIRECTS: /* Java */ java::lang::String /**/ = "http.maxRedirects";

	pub static HTTP_NON_PROXY_HOSTS: /* Java */ java::lang::String /**/ = "http.nonProxyHosts";

	pub static HTTP_PROXY_HOST: /* Java */ java::lang::String /**/ = "http.proxyHost";

	pub static HTTP_PROXY_PORT: /* Java */ java::lang::String /**/ = "http.proxyPort";

	pub static HTTPS_PROXY_HOST: /* Java */ java::lang::String /**/ = "https.proxyHost";

	pub static HTTPS_PROXY_PORT: /* Java */ java::lang::String /**/ = "https.proxyPort";

	pub static JAVA_AWT_FONTS: /* Java */ java::lang::String /**/ = "java.awt.fonts";

	pub static JAVA_AWT_GRAPHICSENV: /* Java */ java::lang::String /**/ = "java.awt.graphicsenv";

	pub static JAVA_AWT_HEADLESS: /* Java */ java::lang::String /**/ = "java.awt.headless";

	pub static JAVA_AWT_PRINTERJOB: /* Java */ java::lang::String /**/ = "java.awt.printerjob";

	pub static JAVA_CLASS_PATH: /* Java */ java::lang::String /**/ = "java.class.path";

	pub static JAVA_CLASS_VERSION: /* Java */ java::lang::String /**/ = "java.class.version";

	pub static JAVA_COMPILER: /* Java */ java::lang::String /**/ = "java.compiler";

	pub static JAVA_CONTENT_HANDLER_PKGS: /* Java */ java::lang::String /**/ = "java.content.handler.pkgs";

	pub static JAVA_ENDORSED_DIRS: /* Java */ java::lang::String /**/ = "java.endorsed.dirs";

	pub static JAVA_EXT_DIRS: /* Java */ java::lang::String /**/ = "java.ext.dirs";

	pub static JAVA_HOME: /* Java */ java::lang::String /**/ = "java.home";

	pub static JAVA_IO_TMPDIR: /* Java */ java::lang::String /**/ = "java.io.tmpdir";

	pub static JAVA_LIBRARY_PATH: /* Java */ java::lang::String /**/ = "java.library.path";

	pub static JAVA_LOCALE_PROVIDERS: /* Java */ java::lang::String /**/ = "java.locale.providers";

	pub static JAVA_LOCALE_USE_OLD_ISO_CODES: /* Java */ java::lang::String /**/ = "java.locale.useOldISOCodes";

	pub static JAVA_NET_PREFER_IPV4_STACK: /* Java */ java::lang::String /**/ = "java.net.preferIPv4Stack";

	pub static JAVA_NET_PREFER_IPV6_ADDRESSES: /* Java */ java::lang::String /**/ = "java.net.preferIPv6Addresses";

	pub static JAVA_NET_SOCKS_PASSWORD: /* Java */ java::lang::String /**/ = "java.net.socks.password";

	pub static JAVA_NET_SOCKS_USER_NAME: /* Java */ java::lang::String /**/ = "java.net.socks.username";

	pub static JAVA_NET_USE_SYSTEM_PROXIES: /* Java */ java::lang::String /**/ = "java.net.useSystemProxies";

	pub static JAVA_NIO_CHANNELS_DEFAULT_THREAD_POOL_INITIAL_SIZE: /* Java */ java::lang::String /**/ = "java.nio.channels.DefaultThreadPool.initialSize";

	pub static JAVA_NIO_CHANNELS_DEFAULT_THREAD_POOL_THREAD_FACTORY: /* Java */ java::lang::String /**/ = "java.nio.channels.DefaultThreadPool.threadFactory";

	pub static JAVA_NIO_CHANNELS_SPI_ASYNCHRONOUS_CHANNEL_PROVIDER: /* Java */ java::lang::String /**/ = "java.nio.channels.spi.AsynchronousChannelProvider";

	pub static JAVA_NIO_CHANNELS_SPI_SELECTOR_PROVIDER: /* Java */ java::lang::String /**/ = "java.nio.channels.spi.SelectorProvider";

	pub static JAVA_NIO_FILE_SPI_DEFAULT_FILE_SYSTEM_PROVIDER: /* Java */ java::lang::String /**/ = "java.nio.file.spi.DefaultFileSystemProvider";

	pub static JAVA_PROPERTIES_DATE: /* Java */ java::lang::String /**/ = "java.properties.date";

	pub static JAVA_PROTOCOL_HANDLER_PKGS: /* Java */ java::lang::String /**/ = "java.protocol.handler.pkgs";

	pub static JAVA_RMI_SERVER_CODEBASE: /* Java */ java::lang::String /**/ = "java.rmi.server.codebase";

	pub static JAVA_RMI_SERVER_HOST_NAME: /* Java */ java::lang::String /**/ = "java.rmi.server.hostname";

	pub static JAVA_RMI_SERVER_RANDOM_IDS: /* Java */ java::lang::String /**/ = "java.rmi.server.randomIDs";

	pub static JAVA_RMI_SERVER_RMI_CLASS_LOADER_SPI: /* Java */ java::lang::String /**/ = "java.rmi.server.RMIClassLoaderSpi";

	pub static JAVA_RUNTIME_NAME: /* Java */ java::lang::String /**/ = "java.runtime.name";

	pub static JAVA_RUNTIME_VERSION: /* Java */ java::lang::String /**/ = "java.runtime.version";

	pub static JAVA_SECURITY_AUTH_LOGIN_CONFIG: /* Java */ java::lang::String /**/ = "java.security.auth.login.config";

	pub static JAVA_SECURITY_KERBEROS_CONF: /* Java */ java::lang::String /**/ = "java.security.krb5.conf";

	pub static JAVA_SECURITY_KERBEROS_KDC: /* Java */ java::lang::String /**/ = "java.security.krb5.kdc";

	pub static JAVA_SECURITY_KERBEROS_REALM: /* Java */ java::lang::String /**/ = "java.security.krb5.realm";

	pub static JAVA_SECURITY_DEBUG: /* Java */ java::lang::String /**/ = "java.security.debug";

	pub static JAVA_SECURITY_MANAGER: /* Java */ java::lang::String /**/ = "java.security.manager";

	pub static JAVA_SPECIFICATION_MAINTENANCE_VERSION: /* Java */ java::lang::String /**/ = "java.specification.maintenance.version";

	pub static JAVA_SPECIFICATION_NAME: /* Java */ java::lang::String /**/ = "java.specification.name";

	pub static JAVA_SPECIFICATION_VENDOR: /* Java */ java::lang::String /**/ = "java.specification.vendor";

	pub static JAVA_SPECIFICATION_VERSION: /* Java */ java::lang::String /**/ = "java.specification.version";

	pub static JAVA_SYSTEM_CLASS_LOADER: /* Java */ java::lang::String /**/ = "java.system.class.loader";

	pub static JAVA_TIME_ZONE_DEFAULT_ZONE_RULES_PROVIDER: /* Java */ java::lang::String /**/ = "java.time.zone.DefaultZoneRulesProvider";

	pub static JAVA_UTIL_CONCURRENT_FORK_JOIN_POOL_COMMON_EXCEPTION_HANDLER: /* Java */ java::lang::String /**/ = "java.util.concurrent.ForkJoinPool.common.exceptionHandler";

	pub static JAVA_UTIL_CONCURRENT_FORK_JOIN_POOL_COMMON_MAXIMUM_SPARES: /* Java */ java::lang::String /**/ = "java.util.concurrent.ForkJoinPool.common.maximumSpares";

	pub static JAVA_UTIL_CONCURRENT_FORK_JOIN_POOL_COMMON_PARALLELISM: /* Java */ java::lang::String /**/ = "java.util.concurrent.ForkJoinPool.common.parallelism";

	pub static JAVA_UTIL_CONCURRENT_FORK_JOIN_POOL_COMMON_THREAD_FACTORY: /* Java */ java::lang::String /**/ = "java.util.concurrent.ForkJoinPool.common.threadFactory";

	pub static JAVA_UTIL_CURRENCY_DATA: /* Java */ java::lang::String /**/ = "java.util.currency.data";

	pub static JAVA_UTIL_LOGGING_CONFIG_CLASS: /* Java */ java::lang::String /**/ = "java.util.logging.config.class";

	pub static JAVA_UTIL_LOGGING_CONFIG_FILE: /* Java */ java::lang::String /**/ = "java.util.logging.config.file";

	pub static JAVA_UTIL_LOGGING_SIMPLE_FORMATTER_FORMAT: /* Java */ java::lang::String /**/ = "java.util.logging.simpleformatter.format";

	pub static JAVA_UTIL_PREFS_PREFERENCES_FACTORY: /* Java */ java::lang::String /**/ = "java.util.prefs.PreferencesFactory";

	pub static JAVA_UTIL_PROPERTY_RESOURCE_BUNDLE_ENCODING: /* Java */ java::lang::String /**/ = "java.util.PropertyResourceBundle.encoding";

	pub static JAVA_VENDOR: /* Java */ java::lang::String /**/ = "java.vendor";

	pub static JAVA_VENDOR_URL: /* Java */ java::lang::String /**/ = "java.vendor.url";

	pub static JAVA_VENDOR_VERSION: /* Java */ java::lang::String /**/ = "java.vendor.version";

	pub static JAVA_VERSION: /* Java */ java::lang::String /**/ = "java.version";

	pub static JAVA_VERSION_DATE: /* Java */ java::lang::String /**/ = "java.version.date";

	pub static JAVA_VM_INFO: /* Java */ java::lang::String /**/ = "java.vm.info";

	pub static JAVA_VM_NAME: /* Java */ java::lang::String /**/ = "java.vm.name";

	pub static JAVA_VM_SPECIFICATION_NAME: /* Java */ java::lang::String /**/ = "java.vm.specification.name";

	pub static JAVA_VM_SPECIFICATION_VENDOR: /* Java */ java::lang::String /**/ = "java.vm.specification.vendor";

	pub static JAVA_VM_SPECIFICATION_VERSION: /* Java */ java::lang::String /**/ = "java.vm.specification.version";

	pub static JAVA_VM_VENDOR: /* Java */ java::lang::String /**/ = "java.vm.vendor";

	pub static JAVA_VM_VERSION: /* Java */ java::lang::String /**/ = "java.vm.version";

	pub static JAVA_XML_CONFIG_FILE: /* Java */ java::lang::String /**/ = "java.xml.config.file";

	pub static JAVAX_ACCESSIBILITY_ASSISTIVE_TECHNOLOGIES: /* Java */ java::lang::String /**/ = "javax.accessibility.assistive_technologies";

	pub static JAVAX_NET_SSL_SESSION_CACHE_SIZE: /* Java */ java::lang::String /**/ = "javax.net.ssl.sessionCacheSize";

	pub static JAVAX_RMI_SSL_CLIENT_ENABLED_CIPHER_SUITES: /* Java */ java::lang::String /**/ = "javax.rmi.ssl.client.enabledCipherSuites";

	pub static JAVAX_RMI_SSL_CLIENT_ENABLED_PROTOCOLS: /* Java */ java::lang::String /**/ = "javax.rmi.ssl.client.enabledProtocols";

	pub static JAVAX_SECURITY_AUTH_USE_SUBJECT_CREDS_ONLY: /* Java */ java::lang::String /**/ = "javax.security.auth.useSubjectCredsOnly";

	pub static JAVAX_SMART_CARD_IO_TERMINAL_FACTORY_DEFAULT_TYPE: /* Java */ java::lang::String /**/ = "javax.smartcardio.TerminalFactory.DefaultType";

	pub static JDBC_DRIVERS: /* Java */ java::lang::String /**/ = "jdbc.drivers";

	pub static JDK_HTTP_AUTH_PROXYING_DISABLED_SCHEMES: /* Java */ java::lang::String /**/ = "jdk.http.auth.proxying.disabledSchemes";

	pub static JDK_HTTP_AUTH_TUNNELING_DISABLED_SCHEMES: /* Java */ java::lang::String /**/ = "jdk.http.auth.tunneling.disabledSchemes";

	pub static JDK_HTTP_CLIENT_ALLOW_RESTRICTED_HEADERS: /* Java */ java::lang::String /**/ = "jdk.httpclient.allowRestrictedHeaders";

	pub static JDK_HTTP_CLIENT_AUTH_RETRY_LIMIT: /* Java */ java::lang::String /**/ = "jdk.httpclient.auth.retrylimit";

	pub static JDK_HTTP_CLIENT_BUF_SIZE: /* Java */ java::lang::String /**/ = "jdk.httpclient.bufsize";

	pub static JDK_HTTP_CLIENT_CONNECTION_POOL_SIZE: /* Java */ java::lang::String /**/ = "jdk.httpclient.connectionPoolSize";

	pub static JDK_HTTP_CLIENT_CONNECTION_WINDOW_SIZE: /* Java */ java::lang::String /**/ = "jdk.httpclient.connectionWindowSize";

	pub static JDK_HTTP_CLIENT_DISABLE_RETRY_CONNECT: /* Java */ java::lang::String /**/ = "jdk.httpclient.disableRetryConnect";

	pub static JDK_HTTP_CLIENT_ENABLE_ALL_METHOD_RETRY: /* Java */ java::lang::String /**/ = "jdk.httpclient.enableAllMethodRetry";

	pub static JDK_HTTP_CLIENT_ENABLE_PUSH: /* Java */ java::lang::String /**/ = "jdk.httpclient.enablepush";

	pub static JDK_HTTP_CLIENT_HPACK_MAX_HEADER_TABLE_SIZE: /* Java */ java::lang::String /**/ = "jdk.httpclient.hpack.maxheadertablesize";

	pub static JDK_HTTP_CLIENT_HTTP_CLIENT_LOG: /* Java */ java::lang::String /**/ = "jdk.httpclient.HttpClient.log";

	pub static JDK_HTTP_CLIENT_KEEP_ALIVE_TIMEOUT: /* Java */ java::lang::String /**/ = "jdk.httpclient.keepalive.timeout";

	pub static JDK_HTTP_CLIENT_KEEP_ALIVE_TIMEOUT_H2: /* Java */ java::lang::String /**/ = "jdk.httpclient.keepalive.timeout.h2";

	pub static JDK_HTTP_CLIENT_MAX_FRAME_SIZE: /* Java */ java::lang::String /**/ = "jdk.httpclient.maxframesize";

	pub static JDK_HTTP_CLIENT_MAX_STREAMS: /* Java */ java::lang::String /**/ = "jdk.httpclient.maxstreams";

	pub static JDK_HTTP_CLIENT_RECEIVE_BUFFER_SIZE: /* Java */ java::lang::String /**/ = "jdk.httpclient.receiveBufferSize";

	pub static JDK_HTTP_CLIENT_REDIRECTS_RETRY_LIMIT: /* Java */ java::lang::String /**/ = "jdk.httpclient.redirects.retrylimit";

	pub static JDK_HTTP_CLIENT_SEND_BUFFER_SIZE: /* Java */ java::lang::String /**/ = "jdk.httpclient.sendBufferSize";

	pub static JDK_HTTP_CLIENT_WEB_SOCKET_WRITE_BUFFER_SIZE: /* Java */ java::lang::String /**/ = "jdk.httpclient.websocket.writeBufferSize";

	pub static JDK_HTTP_CLIENT_WINDOW_SIZE: /* Java */ java::lang::String /**/ = "jdk.httpclient.windowsize";

	pub static JDK_HTTP_SERVER_MAX_CONNECTIONS: /* Java */ java::lang::String /**/ = "jdk.httpserver.maxConnections";

	pub static JDK_HTTPS_NEGOTIATE_CBT: /* Java */ java::lang::String /**/ = "jdk.https.negotiate.cbt";

	pub static JDK_INCLUDE_IN_EXCEPTIONS: /* Java */ java::lang::String /**/ = "jdk.includeInExceptions";

	pub static JDK_INTERNAL_HTTP_CLIENT_DISABLE_HOST_NAME_VERIFICATION: /* Java */ java::lang::String /**/ = "jdk.internal.httpclient.disableHostnameVerification";

	pub static JDK_IO_PERMISSIONS_USE_CANONICAL_PATH: /* Java */ java::lang::String /**/ = "jdk.io.permissionsUseCanonicalPath";

	pub static JDK_JNDI_LDAP_OBJECT_FACTORIES_FILTER: /* Java */ java::lang::String /**/ = "jdk.jndi.ldap.object.factoriesFilter";

	pub static JDK_JNDI_OBJECT_FACTORIES_FILTER: /* Java */ java::lang::String /**/ = "jdk.jndi.object.factoriesFilter";

	pub static JDK_JNDI_RMI_OBJECT_FACTORIES_FILTER: /* Java */ java::lang::String /**/ = "jdk.jndi.rmi.object.factoriesFilter";

	pub static JDK_MODULE_MAIN: /* Java */ java::lang::String /**/ = "jdk.module.main";

	pub static JDK_MODULE_MAIN_CLASS: /* Java */ java::lang::String /**/ = "jdk.module.main.class";

	pub static JDK_MODULE_PATH: /* Java */ java::lang::String /**/ = "jdk.module.path";

	pub static JDK_MODULE_UPGRADE_PATH: /* Java */ java::lang::String /**/ = "jdk.module.upgrade.path";

	pub static JDK_NET_UNIX_DOMAIN_TMPDIR: /* Java */ java::lang::String /**/ = "jdk.net.unixdomain.tmpdir";

	pub static JDK_NET_URL_CLASS_PATH_SHOW_IGNORED_CLASS_PATH_ENTRIES: /* Java */ java::lang::String /**/ = "jdk.net.URLClassPath.showIgnoredClassPathEntries";

	pub static JDK_SERIAL_FILTER: /* Java */ java::lang::String /**/ = "jdk.serialFilter";

	pub static JDK_SERIAL_FILTER_FACTORY: /* Java */ java::lang::String /**/ = "jdk.serialFilterFactory";

	pub static JDK_TLS_CLIENT_SIGNATURE_SCHEMES: /* Java */ java::lang::String /**/ = "jdk.tls.client.SignatureSchemes";

	pub static JDK_TLS_NAMED_GROUPS: /* Java */ java::lang::String /**/ = "jdk.tls.namedGroups";

	pub static JDK_TLS_SERVER_SIGNATURE_SCHEMES: /* Java */ java::lang::String /**/ = "jdk.tls.server.SignatureSchemes";

	pub static JDK_VIRTUAL_THREAD_SCHEDULER_MAXPOOLSIZE: /* Java */ java::lang::String /**/ = "jdk.virtualThreadScheduler.maxPoolSize";

	pub static JDK_VIRTUAL_THREAD_SCHEDULER_PARALLELISM: /* Java */ java::lang::String /**/ = "jdk.virtualThreadScheduler.parallelism";

	pub static JDK_XML_CDATA_CHUNK_SIZE: /* Java */ java::lang::String /**/ = "jdk.xml.cdataChunkSize";

	pub static JDK_XML_DTD_SUPPORT: /* Java */ java::lang::String /**/ = "jdk.xml.dtd.support";

	pub static JDK_XML_ELEMENT_ATTRIBUTE_LIMIT: /* Java */ java::lang::String /**/ = "jdk.xml.elementAttributeLimit";

	pub static JDK_XML_ENABLE_EXTENSION_FUNCTIONS: /* Java */ java::lang::String /**/ = "jdk.xml.enableExtensionFunctions";

	pub static JDK_XML_ENTITY_EXPANSION_LIMIT: /* Java */ java::lang::String /**/ = "jdk.xml.entityExpansionLimit";

	pub static JDK_XML_ENTITY_REPLACEMENT_LIMIT: /* Java */ java::lang::String /**/ = "jdk.xml.entityReplacementLimi_t";

	pub static JDK_XML_IS_STANDALONE: /* Java */ java::lang::String /**/ = "jdk.xml.isStandalone";

	pub static JDK_XML_JDK_CATALOG_RESOLVE: /* Java */ java::lang::String /**/ = "jdk.xml.jdkcatalog.resolve";

	pub static JDK_XML_MAX_ELEMENT_DEPTH: /* Java */ java::lang::String /**/ = "jdk.xml.maxElementDepth";

	pub static JDK_XML_MAX_GENERAL_ENTITY_SIZE_LIMIT: /* Java */ java::lang::String /**/ = "jdk.xml.maxGeneralEntitySizeLimit";

	pub static JDK_XML_MAX_OCCUR_LIMIT: /* Java */ java::lang::String /**/ = "jdk.xml.maxOccurLimit";

	pub static JDK_XML_MAX_PARAMETER_ENTITY_SIZE_LIMIT: /* Java */ java::lang::String /**/ = "jdk.xml.maxParameterEntitySizeLimit";

	pub static JDK_XML_MAX_XML_NAME_LIMIT: /* Java */ java::lang::String /**/ = "jdk.xml.maxXMLNameLimit";

	pub static JDK_XML_OVERRIDE_DEFAULT_PARSER: /* Java */ java::lang::String /**/ = "jdk.xml.overrideDefaultParser";

	pub static JDK_XML_RESET_SYMBOL_TABLE: /* Java */ java::lang::String /**/ = "jdk.xml.resetSymbolTable";

	pub static JDK_XML_TOTAL_ENTITY_SIZE_LIMIT: /* Java */ java::lang::String /**/ = "jdk.xml.totalEntitySizeLimit";

	pub static JDK_XML_XSLTC_IS_STANDALONE: /* Java */ java::lang::String /**/ = "jdk.xml.xsltcIsStandalone";

	pub static LINE_SEPARATOR: /* Java */ java::lang::String /**/ = "line.separator";

	pub static NATIVE_ENCODING: /* Java */ java::lang::String /**/ = "native.encoding";

	pub static NETWORK_ADDRESS_CACHE_NEGATIVE_TTL: /* Java */ java::lang::String /**/ = "networkaddress.cache.negative.ttl";

	pub static NETWORK_ADDRESS_CACHE_STALE_TTL: /* Java */ java::lang::String /**/ = "networkaddress.cache.stale.ttl";

	pub static NETWORK_ADDRESS_CACHE_TTL: /* Java */ java::lang::String /**/ = "networkaddress.cache.ttl";

	pub static ORG_JCP_XML_DSIG_SECURE_VALIDATION: /* Java */ java::lang::String /**/ = "org.jcp.xml.dsig.securevalidation";

	pub static ORG_OPENJDK_JAVA_UTIL_STREAM_TRIPWIRE: /* Java */ java::lang::String /**/ = "org.openjdk.java.util.stream.tripwire";

	pub static OS_ARCH: /* Java */ java::lang::String /**/ = "os.arch";

	pub static OS_NAME: /* Java */ java::lang::String /**/ = "os.name";

	pub static OS_VERSION: /* Java */ java::lang::String /**/ = "os.version";

	pub static PATH_SEPARATOR: /* Java */ java::lang::String /**/ = "path.separator";

	pub static SOCKS_PROXY_HOST: /* Java */ java::lang::String /**/ = "socksProxyHost";

	pub static SOCKS_PROXY_PORT: /* Java */ java::lang::String /**/ = "socksProxyPort";

	pub static SOCKS_PROXY_VERSION: /* Java */ java::lang::String /**/ = "socksProxyVersion";

	pub static STDERR_ENCODING: /* Java */ java::lang::String /**/ = "stderr.encoding";

	pub static STDOUT_ENCODING: /* Java */ java::lang::String /**/ = "stdout.encoding";

	pub static SUN_NET_HTTP_SERVER_DRAIN_AMOUNT: /* Java */ java::lang::String /**/ = "sun.net.httpserver.drainAmount";

	pub static SUN_NET_HTTP_SERVER_IDLE_INTERVAL: /* Java */ java::lang::String /**/ = "sun.net.httpserver.idleInterval";

	pub static SUN_NET_HTTP_SERVER_MAX_IDLE_CONNECTIONS: /* Java */ java::lang::String /**/ = "sun.net.httpserver.maxIdleConnections";

	pub static SUN_NET_HTTP_SERVER_MAX_REQ_HEADERS: /* Java */ java::lang::String /**/ = "sun.net.httpserver.maxReqHeaders";

	pub static SUN_NET_HTTP_SERVER_MAX_REQ_TIME: /* Java */ java::lang::String /**/ = "sun.net.httpserver.maxReqTime";

	pub static SUN_NET_HTTP_SERVER_MAX_RSP_TIME: /* Java */ java::lang::String /**/ = "sun.net.httpserver.maxRspTime";

	pub static SUN_NET_HTTP_SERVER_NO_DELAY: /* Java */ java::lang::String /**/ = "sun.net.httpserver.nodelay";

	pub static SUN_SECURITY_KRB5_PRINCIPAL: /* Java */ java::lang::String /**/ = "sun.security.krb5.principal";

	pub static USER_COUNTRY: /* Java */ java::lang::String /**/ = "user.country";

	pub static USER_DIR: /* Java */ java::lang::String /**/ = "user.dir";

	pub static USER_EXTENSIONS: /* Java */ java::lang::String /**/ = "user.extensions";

	pub static USER_HOME: /* Java */ java::lang::String /**/ = "user.home";

	pub static USER_LANGUAGE: /* Java */ java::lang::String /**/ = "user.language";

	pub static USER_NAME: /* Java */ java::lang::String /**/ = "user.name";

	pub static USER_REGION: /* Java */ java::lang::String /**/ = "user.region";

	pub static USER_SCRIPT: /* Java */ java::lang::String /**/ = "user.script";

	pub static USER_TIMEZONE: /* Java */ java::lang::String /**/ = "user.timezone";

	pub static USER_VARIANT: /* Java */ java::lang::String /**/ = "user.variant";

	pub fn get_apple_awt_enable_template_images(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.APPLE_AWT_ENABLE_TEMPLATE_IMAGES);
	}

	pub fn get_awt_toolkit(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.AWT_TOOLKIT);
	}

	pub fn get_boolean(&self, clazz: &/* Java */ java::lang::Class /**/, key: &/* Java */ java::lang::String /**/, default_if_absent: &/* Java */ java::util::function::BooleanSupplier /**/) -> bool {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_boolean(&org::apache::commons::lang3::system_properties::SystemProperties::to_key(clazz, key, true), default_if_absent);
	}

	pub fn get_boolean(&self, key: &/* Java */ java::lang::String /**/, default_if_absent: &/* Java */ java::util::function::BooleanSupplier /**/) -> bool {
		/* final */ let str: String = org::apache::commons::lang3::system_properties::SystemProperties::get_property(key);
		return  if str == null { default_if_absent != null && default_if_absent.getAsBoolean() } else { Boolean::parseBoolean(str) };
	}

	pub fn get_com_sun_jndi_ldap_object_trust_serial_data(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.COM_SUN_JNDI_LDAP_OBJECT_TRUST_SERIAL_DATA);
	}

	pub fn get_com_sun_net_http_server_http_server_provider(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.COM_SUN_NET_HTTP_SERVER_HTTP_SERVER_PROVIDER);
	}

	pub fn get_file_encoding(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.FILE_ENCODING);
	}

	pub fn get_file_separator(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.FILE_SEPARATOR);
	}

	pub fn get_ftp_non_proxy_host(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.FTP_NON_PROXY_HOST);
	}

	pub fn get_ftp_proxy_host(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.FTP_PROXY_HOST);
	}

	pub fn get_ftp_proxy_port(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.FTP_PROXY_PORT);
	}

	pub fn get_http_agent(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.HTTP_AGENT);
	}

	pub fn get_http_auth_digest_cnonce_repeat(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.HTTP_AUTH_DIGEST_CNONCE_REPEAT);
	}

	pub fn get_http_auth_digest_reenabled_algorithms(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.HTTP_AUTH_DIGEST_RE_ENABLED_ALGORITHMS);
	}

	pub fn get_http_auth_digest_validate_proxy(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.HTTP_AUTH_DIGEST_VALIDATE_PROXY);
	}

	pub fn get_http_auth_digest_validate_server(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.HTTP_AUTH_DIGEST_VALIDATE_SERVER);
	}

	pub fn get_http_auth_ntlm_domain(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.HTTP_AUTH_NTLM_DOMAIN);
	}

	pub fn get_http_keep_alive(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.HTTP_KEEP_ALIVE);
	}

	pub fn get_http_keep_alive_time_proxy(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.HTTP_KEEP_ALIVE_TIME_PROXY);
	}

	pub fn get_http_keep_alive_time_server(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.HTTP_KEEP_ALIVE_TIME_SERVER);
	}

	pub fn get_http_max_connections(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.HTTP_MAX_CONNECTIONS);
	}

	pub fn get_http_max_redirects(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.HTTP_MAX_REDIRECTS);
	}

	pub fn get_http_non_proxy_hosts(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.HTTP_NON_PROXY_HOSTS);
	}

	pub fn get_http_proxy_host(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.HTTP_PROXY_HOST);
	}

	pub fn get_http_proxy_port(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.HTTP_PROXY_PORT);
	}

	pub fn get_https_proxy_host(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.HTTPS_PROXY_HOST);
	}

	pub fn get_https_proxy_port(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.HTTPS_PROXY_PORT);
	}

	pub fn get_int(&self, clazz: &/* Java */ java::lang::Class /**/, key: &/* Java */ java::lang::String /**/, default_if_absent: &/* Java */ java::util::function::IntSupplier /**/) -> i32 {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_int(&org::apache::commons::lang3::system_properties::SystemProperties::to_key(clazz, key, true), default_if_absent);
	}

	pub fn get_int(&self, key: &/* Java */ java::lang::String /**/, default_if_absent: &/* Java */ java::util::function::IntSupplier /**/) -> i32 {
		/* final */ let str: String = org::apache::commons::lang3::system_properties::SystemProperties::get_property(key);
		return  if str == null {  if default_if_absent != null { default_if_absent.getAsInt() } else { 0 } } else { Integer::parseInt(str) };
	}

	pub fn get_java_awt_fonts(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.JAVA_AWT_FONTS);
	}

	pub fn get_java_awt_graphicsenv(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.JAVA_AWT_GRAPHICSENV);
	}

	pub fn get_java_awt_headless(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.JAVA_AWT_HEADLESS);
	}

	pub fn get_java_awt_printerjob(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.JAVA_AWT_PRINTERJOB);
	}

	pub fn get_java_class_path(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.JAVA_CLASS_PATH);
	}

	pub fn get_java_class_version(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.JAVA_CLASS_VERSION);
	}

	pub fn get_java_compiler(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.JAVA_COMPILER);
	}

	pub fn get_java_content_handler_pkgs(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.JAVA_CONTENT_HANDLER_PKGS);
	}

	pub fn get_java_endorsed_dirs(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.JAVA_ENDORSED_DIRS);
	}

	pub fn get_java_ext_dirs(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.JAVA_EXT_DIRS);
	}

	pub fn get_java_home(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.JAVA_HOME);
	}

	pub fn get_java_io_tmpdir(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.JAVA_IO_TMPDIR);
	}

	pub fn get_java_library_path(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.JAVA_LIBRARY_PATH);
	}

	pub fn get_java_locale_providers(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.JAVA_LOCALE_PROVIDERS);
	}

	pub fn get_java_locale_use_old_iso_codes(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.JAVA_LOCALE_USE_OLD_ISO_CODES);
	}

	pub fn get_java_net_prefer_ipv4_stack(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.JAVA_NET_PREFER_IPV4_STACK);
	}

	pub fn get_java_net_prefer_ipv6_addresses(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.JAVA_NET_PREFER_IPV6_ADDRESSES);
	}

	pub fn get_java_net_socks_password(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.JAVA_NET_SOCKS_PASSWORD);
	}

	pub fn get_java_net_socks_user_name(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.JAVA_NET_SOCKS_USER_NAME);
	}

	pub fn get_java_net_use_system_proxies(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.JAVA_NET_USE_SYSTEM_PROXIES);
	}

	pub fn get_java_nio_channels_default_thread_pool_initial_size(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.JAVA_NIO_CHANNELS_DEFAULT_THREAD_POOL_INITIAL_SIZE);
	}

	pub fn get_java_nio_channels_default_thread_pool_thread_factory(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.JAVA_NIO_CHANNELS_DEFAULT_THREAD_POOL_THREAD_FACTORY);
	}

	pub fn get_java_nio_channels_spi_asynchronous_channel_provider(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.JAVA_NIO_CHANNELS_SPI_ASYNCHRONOUS_CHANNEL_PROVIDER);
	}

	pub fn get_java_nio_channels_spi_selector_provider(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.JAVA_NIO_CHANNELS_SPI_SELECTOR_PROVIDER);
	}

	pub fn get_java_nio_file_spi_default_file_system_provider(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.JAVA_NIO_FILE_SPI_DEFAULT_FILE_SYSTEM_PROVIDER);
	}

	pub fn get_java_properties_date(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.JAVA_PROPERTIES_DATE);
	}

	pub fn get_java_protocol_handler_pkgs(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.JAVA_PROTOCOL_HANDLER_PKGS);
	}

	pub fn get_java_rmi_server_codebase(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.JAVA_RMI_SERVER_CODEBASE);
	}

	pub fn get_java_rmi_server_host_name(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.JAVA_RMI_SERVER_HOST_NAME);
	}

	pub fn get_java_rmi_server_random_ids(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.JAVA_RMI_SERVER_RANDOM_IDS);
	}

	pub fn get_java_rmi_server_rmi_class_loader_spi(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.JAVA_RMI_SERVER_RMI_CLASS_LOADER_SPI);
	}

	pub fn get_java_runtime_name(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.JAVA_RUNTIME_NAME);
	}

	pub fn get_java_runtime_version(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.JAVA_RUNTIME_VERSION);
	}

	pub fn get_java_security_auth_login_config(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.JAVA_SECURITY_AUTH_LOGIN_CONFIG);
	}

	pub fn get_java_security_manager(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.JAVA_SECURITY_MANAGER);
	}

	pub fn get_java_specification_maintenance_version(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.JAVA_SPECIFICATION_MAINTENANCE_VERSION);
	}

	pub fn get_java_specification_name(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.JAVA_SPECIFICATION_NAME);
	}

	pub fn get_java_specification_vendor(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.JAVA_SPECIFICATION_VENDOR);
	}

	pub fn get_java_specification_version(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.JAVA_SPECIFICATION_VERSION);
	}

	pub fn get_java_specification_version(&self, default_value: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.JAVA_SPECIFICATION_VERSION, default_value);
	}

	pub fn get_java_system_class_loader(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.JAVA_SYSTEM_CLASS_LOADER);
	}

	pub fn get_java_time_zone_default_zone_rules_provider(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.JAVA_TIME_ZONE_DEFAULT_ZONE_RULES_PROVIDER);
	}

	pub fn get_java_util_concurrent_fork_join_pool_common_exception_handler(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.JAVA_UTIL_CONCURRENT_FORK_JOIN_POOL_COMMON_EXCEPTION_HANDLER);
	}

	pub fn get_java_util_concurrent_fork_join_pool_common_maximum_spares(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.JAVA_UTIL_CONCURRENT_FORK_JOIN_POOL_COMMON_MAXIMUM_SPARES);
	}

	pub fn get_java_util_concurrent_fork_join_pool_common_parallelism(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.JAVA_UTIL_CONCURRENT_FORK_JOIN_POOL_COMMON_PARALLELISM);
	}

	pub fn get_java_util_concurrent_fork_join_pool_common_thread_factory(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.JAVA_UTIL_CONCURRENT_FORK_JOIN_POOL_COMMON_THREAD_FACTORY);
	}

	pub fn get_java_util_currency_data(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.JAVA_UTIL_CURRENCY_DATA);
	}

	pub fn get_java_util_logging_config_class(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.JAVA_UTIL_LOGGING_CONFIG_CLASS);
	}

	pub fn get_java_util_logging_config_file(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.JAVA_UTIL_LOGGING_CONFIG_FILE);
	}

	pub fn get_java_util_logging_simple_formatter_format(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.JAVA_UTIL_LOGGING_SIMPLE_FORMATTER_FORMAT);
	}

	pub fn get_java_util_prefs_preferences_factory(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.JAVA_UTIL_PREFS_PREFERENCES_FACTORY);
	}

	pub fn get_java_util_property_resource_bundle_encoding(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.JAVA_UTIL_PROPERTY_RESOURCE_BUNDLE_ENCODING);
	}

	pub fn get_java_vendor(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.JAVA_VENDOR);
	}

	pub fn get_java_vendor_url(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.JAVA_VENDOR_URL);
	}

	pub fn get_java_vendor_version(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.JAVA_VENDOR_VERSION);
	}

	pub fn get_java_version(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.JAVA_VERSION);
	}

	pub fn get_java_version_date(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.JAVA_VERSION_DATE);
	}

	pub fn get_java_vm_info(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.JAVA_VM_INFO);
	}

	pub fn get_java_vm_name(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.JAVA_VM_NAME);
	}

	pub fn get_java_vm_specification_name(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.JAVA_VM_SPECIFICATION_NAME);
	}

	pub fn get_java_vm_specification_vendor(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.JAVA_VM_SPECIFICATION_VENDOR);
	}

	pub fn get_java_vm_specification_version(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.JAVA_VM_SPECIFICATION_VERSION);
	}

	pub fn get_java_vm_vendor(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.JAVA_VM_VENDOR);
	}

	pub fn get_java_vm_version(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.JAVA_VM_VERSION);
	}

	pub fn get_javax_accessibility_assistive_technologies(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.JAVAX_ACCESSIBILITY_ASSISTIVE_TECHNOLOGIES);
	}

	pub fn get_java_xml_config_file(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.JAVA_XML_CONFIG_FILE);
	}

	pub fn get_javax_net_ssl_session_cache_size(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.JAVAX_NET_SSL_SESSION_CACHE_SIZE);
	}

	pub fn get_javax_rmi_ssl_client_enabled_cipher_suites(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.JAVAX_RMI_SSL_CLIENT_ENABLED_CIPHER_SUITES);
	}

	pub fn get_javax_rmi_ssl_client_enabled_protocols(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.JAVAX_RMI_SSL_CLIENT_ENABLED_PROTOCOLS);
	}

	pub fn get_javax_security_auth_use_subject_creds_only(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.JAVAX_SECURITY_AUTH_USE_SUBJECT_CREDS_ONLY);
	}

	pub fn get_javax_smart_card_io_terminal_factory_default_type(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.JAVAX_SMART_CARD_IO_TERMINAL_FACTORY_DEFAULT_TYPE);
	}

	pub fn get_jdbc_drivers(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.JDBC_DRIVERS);
	}

	pub fn get_jdk_http_auth_proxying_disabled_schemes(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.JDK_HTTP_AUTH_PROXYING_DISABLED_SCHEMES);
	}

	pub fn get_jdk_http_auth_tunneling_disabled_schemes(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.JDK_HTTP_AUTH_TUNNELING_DISABLED_SCHEMES);
	}

	pub fn get_jdk_http_client_allow_restricted_headers(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.JDK_HTTP_CLIENT_ALLOW_RESTRICTED_HEADERS);
	}

	pub fn get_jdk_http_client_auth_retry_limit(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.JDK_HTTP_CLIENT_AUTH_RETRY_LIMIT);
	}

	pub fn get_jdk_http_client_buf_size(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.JDK_HTTP_CLIENT_BUF_SIZE);
	}

	pub fn get_jdk_http_client_connection_pool_size(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.JDK_HTTP_CLIENT_CONNECTION_POOL_SIZE);
	}

	pub fn get_jdk_http_client_connection_window_size(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.JDK_HTTP_CLIENT_CONNECTION_WINDOW_SIZE);
	}

	pub fn get_jdk_http_client_disable_retry_connect(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.JDK_HTTP_CLIENT_DISABLE_RETRY_CONNECT);
	}

	pub fn get_jdk_http_client_enable_all_method_retry(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.JDK_HTTP_CLIENT_ENABLE_ALL_METHOD_RETRY);
	}

	pub fn get_jdk_http_client_enable_push(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.JDK_HTTP_CLIENT_ENABLE_PUSH);
	}

	pub fn get_jdk_http_client_hpack_max_header_table_size(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.JDK_HTTP_CLIENT_HPACK_MAX_HEADER_TABLE_SIZE);
	}

	pub fn get_jdk_http_client_http_client_log(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.JDK_HTTP_CLIENT_HTTP_CLIENT_LOG);
	}

	pub fn get_jdk_http_client_keep_alive_timeout(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.JDK_HTTP_CLIENT_KEEP_ALIVE_TIMEOUT);
	}

	pub fn get_jdk_http_client_keep_alive_timeout_h2(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.JDK_HTTP_CLIENT_KEEP_ALIVE_TIMEOUT_H2);
	}

	pub fn get_jdk_http_client_max_frame_size(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.JDK_HTTP_CLIENT_MAX_FRAME_SIZE);
	}

	pub fn get_jdk_http_client_max_streams(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.JDK_HTTP_CLIENT_MAX_STREAMS);
	}

	pub fn get_jdk_http_client_receive_buffer_size(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.JDK_HTTP_CLIENT_RECEIVE_BUFFER_SIZE);
	}

	pub fn get_jdk_http_client_redirects_retry_limit(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.JDK_HTTP_CLIENT_REDIRECTS_RETRY_LIMIT);
	}

	pub fn get_jdk_http_client_send_buffer_size(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.JDK_HTTP_CLIENT_SEND_BUFFER_SIZE);
	}

	pub fn get_jdk_http_client_web_socket_write_buffer_size(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.JDK_HTTP_CLIENT_WEB_SOCKET_WRITE_BUFFER_SIZE);
	}

	pub fn get_jdk_http_client_window_size(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.JDK_HTTP_CLIENT_WINDOW_SIZE);
	}

	pub fn get_jdk_http_server_max_connections(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.JDK_HTTP_SERVER_MAX_CONNECTIONS);
	}

	pub fn get_jdk_https_negotiate_cbt(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.JDK_HTTPS_NEGOTIATE_CBT);
	}

	pub fn get_jdk_include_in_exceptions(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.JDK_INCLUDE_IN_EXCEPTIONS);
	}

	pub fn get_jdk_internal_http_client_disable_host_name_verification(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.JDK_INTERNAL_HTTP_CLIENT_DISABLE_HOST_NAME_VERIFICATION);
	}

	pub fn get_jdk_io_permissions_use_canonical_path(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.JDK_IO_PERMISSIONS_USE_CANONICAL_PATH);
	}

	pub fn get_jdk_jndi_ldap_object_factories_filter(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.JDK_JNDI_LDAP_OBJECT_FACTORIES_FILTER);
	}

	pub fn get_jdk_jndi_object_factories_filter(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.JDK_JNDI_OBJECT_FACTORIES_FILTER);
	}

	pub fn get_jdk_jndi_rmi_object_factories_filter(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.JDK_JNDI_RMI_OBJECT_FACTORIES_FILTER);
	}

	pub fn get_jdk_module_main(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.JDK_MODULE_MAIN);
	}

	pub fn get_jdk_module_main_class(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.JDK_MODULE_MAIN_CLASS);
	}

	pub fn get_jdk_module_path(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.JDK_MODULE_PATH);
	}

	pub fn get_jdk_module_upgrade_path(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.JDK_MODULE_UPGRADE_PATH);
	}

	pub fn get_jdk_net_unix_domain_tmp_dir(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.JDK_NET_UNIX_DOMAIN_TMPDIR);
	}

	pub fn get_jdk_net_url_class_path_show_ignored_class_path_entries(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.JDK_NET_URL_CLASS_PATH_SHOW_IGNORED_CLASS_PATH_ENTRIES);
	}

	pub fn get_jdk_serial_filter(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.JDK_SERIAL_FILTER);
	}

	pub fn get_jdk_serial_filter_factory(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.JDK_SERIAL_FILTER_FACTORY);
	}

	pub fn get_jdk_tls_client_signature_schemes(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.JDK_TLS_CLIENT_SIGNATURE_SCHEMES);
	}

	pub fn get_jdk_tls_named_groups(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.JDK_TLS_NAMED_GROUPS);
	}

	pub fn get_jdk_tls_server_signature_schemes(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.JDK_TLS_SERVER_SIGNATURE_SCHEMES);
	}

	pub fn get_jdk_virtual_thread_scheduler_max_pool_size(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.JDK_VIRTUAL_THREAD_SCHEDULER_MAXPOOLSIZE);
	}

	pub fn get_jdk_virtual_thread_scheduler_parallelism(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.JDK_VIRTUAL_THREAD_SCHEDULER_PARALLELISM);
	}

	pub fn get_jdk_xml_cdata_chunk_size(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.JDK_XML_CDATA_CHUNK_SIZE);
	}

	pub fn get_jdk_xml_dtd_support(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.JDK_XML_DTD_SUPPORT);
	}

	pub fn get_jdk_xml_element_attribute_limit(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.JDK_XML_ELEMENT_ATTRIBUTE_LIMIT);
	}

	pub fn get_jdk_xml_enable_extension_functions(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.JDK_XML_ENABLE_EXTENSION_FUNCTIONS);
	}

	pub fn get_jdk_xml_entity_expansion_limit(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.JDK_XML_ENTITY_EXPANSION_LIMIT);
	}

	pub fn get_jdk_xml_entity_replacement_limit(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.JDK_XML_ENTITY_REPLACEMENT_LIMIT);
	}

	pub fn get_jdk_xml_is_standalone(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.JDK_XML_IS_STANDALONE);
	}

	pub fn get_jdk_xml_jdk_catalog_resolve(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.JDK_XML_JDK_CATALOG_RESOLVE);
	}

	pub fn get_jdk_xml_max_element_depth(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.JDK_XML_MAX_ELEMENT_DEPTH);
	}

	pub fn get_jdk_xml_max_general_entity_size_limit(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.JDK_XML_MAX_GENERAL_ENTITY_SIZE_LIMIT);
	}

	pub fn get_jdk_xml_max_occur_limit(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.JDK_XML_MAX_OCCUR_LIMIT);
	}

	pub fn get_jdk_xml_max_parameter_entity_size_limit(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.JDK_XML_MAX_PARAMETER_ENTITY_SIZE_LIMIT);
	}

	pub fn get_jdk_xml_max_xml_name_limit(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.JDK_XML_MAX_XML_NAME_LIMIT);
	}

	pub fn get_jdk_xml_override_default_parser(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.JDK_XML_OVERRIDE_DEFAULT_PARSER);
	}

	pub fn get_jdk_xml_reset_symbol_table(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.JDK_XML_RESET_SYMBOL_TABLE);
	}

	pub fn get_jdk_xml_total_entity_size_limit(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.JDK_XML_TOTAL_ENTITY_SIZE_LIMIT);
	}

	pub fn get_jdk_xml_xsltc_is_standalone(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.JDK_XML_XSLTC_IS_STANDALONE);
	}

	pub fn get_line_separator(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.LINE_SEPARATOR);
	}

	pub fn get_line_separator(&self, default_if_absent: &/* Java */ java::util::function::Supplier /**/) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.LINE_SEPARATOR, default_if_absent);
	}

	pub fn get_long(&self, clazz: &/* Java */ java::lang::Class /**/, key: &/* Java */ java::lang::String /**/, default_if_absent: &/* Java */ java::util::function::LongSupplier /**/) -> i64 {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_long(&org::apache::commons::lang3::system_properties::SystemProperties::to_key(clazz, key, true), default_if_absent);
	}

	pub fn get_long(&self, key: &/* Java */ java::lang::String /**/, default_if_absent: &/* Java */ java::util::function::LongSupplier /**/) -> i64 {
		/* final */ let str: String = org::apache::commons::lang3::system_properties::SystemProperties::get_property(key);
		return  if str == null {  if default_if_absent != null { default_if_absent.getAsLong() } else { 0 } } else { Long::parseLong(str) };
	}

	pub fn get_native_encoding(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.NATIVE_ENCODING);
	}

	pub fn get_network_address_cache_negative_ttl(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.NETWORK_ADDRESS_CACHE_NEGATIVE_TTL);
	}

	pub fn get_network_address_cache_stale_ttl(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.NETWORK_ADDRESS_CACHE_STALE_TTL);
	}

	pub fn get_network_address_cache_ttl(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.NETWORK_ADDRESS_CACHE_TTL);
	}

	pub fn get_org_jcp_xml_dsig_secure_validation(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.ORG_JCP_XML_DSIG_SECURE_VALIDATION);
	}

	pub fn get_org_open_jdk_java_util_stream_tripwire(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.ORG_OPENJDK_JAVA_UTIL_STREAM_TRIPWIRE);
	}

	pub fn get_os_arch(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.OS_ARCH);
	}

	pub fn get_os_name(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.OS_NAME);
	}

	pub fn get_os_version(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.OS_VERSION);
	}

	pub fn get_path(&self, key: &/* Java */ java::lang::String /**/, default_if_absent: &/* Java */ java::util::function::Supplier /**/) -> /* Java */ java::nio::file::Path /**/ {
		/* final */ let str: String = org::apache::commons::lang3::system_properties::SystemProperties::get_property(key);
		return  if str == null {  if default_if_absent != null { default_if_absent.get() } else { null } } else { Paths::get(str) };
	}

	pub fn get_path_separator(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.PATH_SEPARATOR);
	}

	pub fn get_property(&self, property: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(property, &Suppliers::nul());
	}

	fn get_property(&self, property: &/* Java */ java::lang::String /**/, default_if_absent: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(property, |()|default_if_absent);
	}

	fn get_property(&self, property: &/* Java */ java::lang::String /**/, default_if_absent: &/* Java */ java::util::function::Supplier /**/) -> /* Java */ java::lang::String /**/ {
		let r0 = 'try0: {
			if StringUtils::is_empty(property) {
				return Suppliers::get(default_if_absent);
			}
			return StringUtils::get_if_empty(&System::getProperty(property), default_if_absent);
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ SecurityException) => {
				// + "'; the SystemUtils property value will default to null.");
				return default_if_absent.get();
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
	}

	pub fn get_socks_proxy_host(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.SOCKS_PROXY_HOST);
	}

	pub fn get_socks_proxy_port(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.SOCKS_PROXY_PORT);
	}

	pub fn get_socks_proxy_version(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.SOCKS_PROXY_VERSION);
	}

	pub fn get_std_err_encoding(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.STDERR_ENCODING);
	}

	pub fn get_std_out_encoding(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.STDOUT_ENCODING);
	}

	pub fn get_sun_net_http_server_drain_amount(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.SUN_NET_HTTP_SERVER_DRAIN_AMOUNT);
	}

	pub fn get_sun_net_http_server_idle_interval(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.SUN_NET_HTTP_SERVER_IDLE_INTERVAL);
	}

	pub fn get_sun_net_http_server_max_idle_connections(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.SUN_NET_HTTP_SERVER_MAX_IDLE_CONNECTIONS);
	}

	pub fn get_sun_net_http_server_max_req_headers(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.SUN_NET_HTTP_SERVER_MAX_REQ_HEADERS);
	}

	pub fn get_sun_net_http_server_max_req_time(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.SUN_NET_HTTP_SERVER_MAX_REQ_TIME);
	}

	pub fn get_sun_net_http_server_max_rsp_time(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.SUN_NET_HTTP_SERVER_MAX_RSP_TIME);
	}

	pub fn get_sun_net_http_server_no_delay(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.SUN_NET_HTTP_SERVER_NO_DELAY);
	}

	pub fn get_sun_security_krb5_principal(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.SUN_SECURITY_KRB5_PRINCIPAL);
	}

	pub fn get_user_country(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.USER_COUNTRY);
	}

	pub fn get_user_dir(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.USER_DIR);
	}

	pub fn get_user_extensions(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.USER_EXTENSIONS);
	}

	pub fn get_user_home(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.USER_HOME);
	}

	pub fn get_user_language(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.USER_LANGUAGE);
	}

	pub fn get_user_name(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.USER_NAME);
	}

	pub fn get_user_name(&self, default_value: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.USER_NAME, default_value);
	}

	pub fn get_user_region(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.USER_REGION);
	}

	pub fn get_user_script(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.USER_SCRIPT);
	}

	pub fn get_user_timezone(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.USER_TIMEZONE);
	}

	pub fn get_user_variant(&self) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(self.USER_VARIANT);
	}

	pub fn is_property_set(&self, property: &/* Java */ java::lang::String /**/) -> bool {
		return org::apache::commons::lang3::system_properties::SystemProperties::get_property(property) != null;
	}

	fn to_key(&self, clazz: &/* Java */ java::lang::Class /**/, key: &/* Java */ java::lang::String /**/, simple_key: bool) -> /* Java */ java::lang::String /**/ {
		return ClassUtils::get_name(clazz, StringUtils::EMPTY, simple_key) + "." + key;
	}

	pub fn new() -> org::apache::commons::lang3::system_properties::SystemProperties {
	// empty
	}
}