#![allow(dead_code)]

#[allow(non_camel_case_types)]
#[repr(u32)]
#[derive(Debug)]
pub enum IndexFormats {
    Null = 0,
    Char,
    Int8,
    Int16,
    Int32,
    Int64,
    String,
    Binary,
    StringArray,
    I18StringArray,
}

#[allow(non_camel_case_types)]
#[repr(i32)]
#[derive(Debug)]
pub enum HeaderTags {
    NotFound = -1,
    HeaderImage = 61,
    HeaderSignatures,
    /// Originally called Image.
    HeaderImmutable,
    HeaderRegions,
    HeaderI18NTable = 100,

    SignatureBase = 256,
    SigatureSize,
    /// Internal & Obsolete
    LittleEndianMD5_1Signature,
    PGPSignature,
    /// Internal & Obsolete
    LittleEndianMD5_2Signature,
    /// Also known as PackageID,
    MD5Signature,
    GPGSignature,
    /// Internal & Obsolete
    PGP5Signature,

    /// Internal & Obsolete
    BadSHA1_1,
    /// Internal & Obsolete
    BadSHA1_2,
    PublicKeys,
    DSAHeader,
    RSAHeader,
    /// Also known as HeaderID
    SHA1Header,
    LongSignatureSize,
    LongArchiveSize,
    SHA256Header = 273,
    VeritySignatures = 276,
    VeritySignatureAlgorithm,

    Name = 1000,
    Version,
    Release,
    Epoch,
    Summary,
    Description,
    BuildTime,
    BuildHost,
    InstallTime,
    Size,
    Distribution,
    Vendor,
    GIF,
    XPM,
    License,
    Packager,
    Group,
    Changelog,
    Source,
    Patch,
    URL,
    OS,
    Architecture,
    PreInstall,
    PostInstall,
    PreUninstall,
    PostUninstall,
    OldFilenames,
    FileSizes,
    FileStates,
    FileModes,
    /// Internal & Obsolete
    FileUIDS,
    /// Internal & Obsolete
    FileGIDS,
    FileRDEVS,
    FileMTimes,
    FileDigests,
    FileLinkTOS,
    FileFlags,
    Root,
    FileUserName,
    FileGroupName,
    /// Internal & Obsolete
    Exclude,
    /// Internal & Obsolete
    Exclusive,
    Icon,
    SourceRPM,
    FileVerifyFlags,
    ArchiveSize,
    /// Also known as Provides and P
    ProvideName,
    RequireFlags,
    /// Also known as Requires
    RequireName,
    RequireVersion,
    NoSource,
    NoPatch,
    ConflictFlags,
    /// Also known as Conflicts and C
    ConflictNames,
    ConflictVersion,
    /// Internal & Deprecated
    DefaultPrefix,
    /// Internal & Obsolete
    BuildRoot,
    /// Internal & Deprecated
    InstallPrefix,
    ExcludeArchitecture,
    ExcludeOS,
    ExclusiveArchitecture,
    ExclusiveOS,
    AutoReqProv,
    RPMVersion,
    TriggerScripts,
    TriggerName,
    TriggerVersion,
    TriggerFlags,
    TriggerIndex,
    VerifyScript = 1079,
    ChangelogTime,
    ChangelogName,
    ChangelogText,
    /// Internal & Obsolete
    BrokenMD5,
    /// Internal
    Prerequisites,
    PreInstallProgram,
    PostInstallProgram,
    PreUninstallProgram,
    PostUninstallProgram,
    BuildArchitectures,
    /// Also known as Obsoletes and O
    ObsoleteName,
    VerifyScriptProgram,
    TriggerScriptProgram,
    /// Internal
    DocDir,
    Cookie,
    FileDevices,
    FileInodes,
    FileLanguages,
    Prefixes,
    InstallationPrefixes,
    /// Internal,
    TriggerInstall,
    /// Internal
    TriggerUninstall,
    /// Internal
    TriggerPostInstall,
    /// Internal
    AutoRequires,
    /// Internal
    AutoProvides,
    /// Internal & Obsolete
    Capability,
    SourcePackage,
    /// Internal & Obsolete
    OldOriginalFileNames,
    /// Internal
    BuildPrerequisites,
    /// Internal
    BuildRequires,
    /// Internal
    BuildConflicts,
    /// Internal & Unused
    BuildMacros,
    ProvideFlags,
    ProvideVersion,
    ObsoleteFlags,
    ObsoleteVersion,
    DirIndexes,
    BaseNames,
    DirNames,
    /// Relocation
    OriginalDirIndexes,
    /// Relocation
    OriginalBaseNames,
    /// Relocation
    OriginalDirNames,
    OptFlags,
    DistURL,
    PayloadFormat,
    PayloadCompressor,
    PayloadFlags,
    /// Transaction color when installed
    InstallColor,
    InstallTID,
    RemoveTID,
    /// Internal & Obsolete
    SHA1RHN,
    /// Internal & Obsolete
    RHNPlatform,
    Platform,
    /// Deprecated placeholder (SuSE)
    PatchesName,
    /// Deprecated placeholder (SuSE)
    PatchesFlags,
    /// Deprecated placeholder (SuSE)
    PatchesVersion,
    /// Internal & Obsolete
    CacheTime,
    /// Internal & Obsolete
    CachePackagePath,
    /// Internal & Obsolete
    CachePackageSize,
    /// Internal & Obsolete
    CachePackageMTime,
    FileColors,
    FileClass,
    ClassDict,
    FileDependsX,
    FileDependsN,
    DependsDict,
    SourcePackageID,
    FileContexts,
    /// Extension
    FSContexts,
    /// Extension
    REContexts,
    /// Selinux *.te policy file.
    Policies,
    PreTransactionScript,
    PostTransactionScript,
    PreTransactionProgram,
    PostTransactionProgram,
    DistributionTag,
    /// Obsolete
    ///
    /// Also known as OldSuggests
    OldSuggestsName,
    /// Obsolete
    OldSuggestsVersion,
    /// Obsolete
    OldSuggestsFlags,
    /// Obsolete
    OldEnhancesName,
    /// Obsolete
    OldEnhancesVersion,
    /// Obsolete
    OldEnhancesFlags,
    /// Extension placeholder
    ///
    /// Unimplemented
    Priority,
    /// Unimplemented
    ///
    /// Also known as SVNID
    CVSID,
    /// Unimplemented
    BLINKPGKID,
    /// Unimplemented
    BLINKHDRID,
    /// Unimplemented
    BLINKNEVRA,
    /// Unimplemented
    FLINKPKGID,
    /// Unimplemented
    FLINKHDRID,
    /// Unimplemented
    FLINKNEVRA,
    /// Unimplemented
    PackageOrigin,
    /// Internal
    TriggerPreInstall,
    /// Internal
    ///
    /// Unimplemented
    BuildSuggests,
    /// Internal
    ///
    /// Unimplemented
    BuildEnhances,
    /// Scriptlet exit codes
    ///
    /// Unimplemented
    ScriptStates,
    /// Scriplet execution times
    ///
    /// Unimplemented
    ScriptMetrics,
    /// Unimplemented
    BuildCPUClock,
    /// Unimplemented
    FileDigestAlgorithms,
    /// Unimplemented
    Variants,
    /// Unimplemented
    XMajor,
    /// Unimplemented
    XMinor,
    /// Unimplemented
    RepoTag,
    /// Unimplemented
    Keywords,
    /// Unimplemented
    BuildPlatforms,
    /// Unimplemented
    PackageColor,
    /// Unimplemented
    PackagePrefColor,
    /// Unimplemented
    XATTRSDICT,
    /// Unimplemented
    FILEXATTRSX,
    /// Unimplemented
    DEPATTRSDICT,
    /// Unimplemented
    CONFLICTATTRSX,
    /// Unimplemented
    OBSOLETEATTRSX,
    /// Unimplemented
    PROVIDEATTRSX,
    /// Unimplemented
    REQUIREATTRSX,
    /// Internal
    ///
    /// Unimplemented
    BuildProvides,
    /// Internal
    ///
    /// Unimplemented
    BuildObsoletes,
    /// Extension
    DatabaseInstance,
    /// Extension
    NVRA,

    /// Extension
    FileNames = 5000,
    /// Extension
    FileProvides,
    /// Extension
    FileRequires,
    /// Unimplemented
    FileSystemNames,
    /// Unimplemented
    FileSystemSizes,
    /// Extension
    TriggerConditions,
    /// Extension
    TriggerType,
    /// Extension
    OriginalFileNames,
    LongFileSizes,
    LongSize,
    FileCapabilities,
    FileDigestAlgorithm,
    BugtrackerURL,
    /// Extension
    EVR,
    /// Extension
    NVR,
    /// Extension
    NEVR,
    /// Extension
    NEVRA,
    /// Extension
    HeaderColor,
    /// Extension
    Verbose,
    /// Extension
    EpochNum,
    PreInstallFlags,
    PostInstallFlags,
    PreUninstallFlags,
    PostUninstallFlags,
    PreTransactionFlags,
    PostTransactionFlags,
    VerifyScriptFlags,
    TriggerScriptFlags,
    /// List of Collections
    ///
    /// Unimplemented
    Collections = 5029,
    PolicyNames,
    PolicyTypes,
    PolicyTypesIndexes,
    PolicyFlags,
    VersionControlSystem,
    OrderName,
    OrderVersion,
    OrderFlags,
    /// Reservation
    ///
    /// Unimplemented
    MSSFMAIFEST,
    /// Reservation
    ///
    /// Unimplemented
    MSSFDOMAIN,
    /// Extension
    InstalledFileNames,
    /// Extension
    RequiresNameEpochVersionReleaseSorting,
    /// Extension
    ProvidesNameEpochVersionReleaseSorting,
    /// Extension
    ObsoletesNameEpochVersionReleaseSorting,
    /// Extension
    ConflictsNameEpochVersionReleaseSorting,
    /// Extension
    FILENLINKS,
    /// Also known as Recommends
    RecommendsName,
    RecommendsVersion,
    RecommendsFlags,
    /// Also known as Suggests
    SuggestsName,
    SuggestsVersion,
    SuggestsFlags,
    /// Also known as Supplements
    SupplementsName,
    SupplementsVersion,
    SupplementsFlags,
    EnhancesName,
    EnhancesVersion,
    EnhancesFlags,
    /// Extension
    RecommedsNameEpochVersionReleaseSorting,
    /// Extension
    SuggestsNameEpochVersionReleaseSorting,
    /// Extension
    SupplementsNameEpochVersionReleaseSorting,
    /// Extension
    EnhancesNameEpochVersionReleaseSorting,
    Encoding,
    /// Internal
    FileTriggerInstall,
    /// Internal
    FileTriggerUninstall,
    /// Internal
    FileTriggerPostUninstall,
    FileTriggerScriptSource,
    FileTriggerScritpProgram,
    FileTriggerScriptFlags,
    FileTriggerName,
    FileTriggerIndex,
    FileTriggerVersion,
    FileTriggerFlags,
    /// Internal
    TransactionFileTriggerInstall,
    /// Internal
    TransactionFileTriggerUninstall,
    /// Internal
    TransactionFileTriggerPostUninstall,
    TransactionFileTriggerScriptSource,
    TransactionFileTriggerScriptProgram,
    TransactionFileTriggerScriptFlags,
    TransactionFileTriggerName,
    TransactionFileTriggerIndex,
    TransactionFileTriggerVersion,
    TransactionFileTriggerFlags,
    /// Internal
    RemovePathPostFixes,
    FileTriggerPriorities,
    TransactionFileTriggerPriorities,
    /// Extension
    FileTriggerConditions,
    /// Extension
    FileTriggerType,
    /// Extension
    TransactionFileTriggerConditions,
    /// Extension
    TransactionFileTriggerType,
    FileSignatures,
    FileSignatureLength,
    PayloadDigest,
    PayloadDigestAlgorithm,
    /// Reservation
    ///
    /// Unimplemented
    AutoInstalled,
    /// Reservation
    ///
    /// Unimplemented
    Identity,
    ModularityLabel,
    PayloadDigestAlternate,
    /// Extension
    ArchitectureSuffix,
    Spec,
    TranslationURL,
    UpstreamReleases,
    /// Internal
    SourceLicense,
    /// Internal
    FirstFreeTag,
}

/// Tags found in the signature header from a package.
#[repr(i32)]
#[derive(Debug)]
pub enum SignatureTags {
    /// Internal
    ///
    /// Header + Pyload size (32bit)
    Size = 1000,
    /// Deprecated
    ///
    /// Internal
    ///
    /// Broken MD5, take 1
    LittleEndianMD5_1,
    /// Internal
    ///
    /// PGP 2.6.3 signature
    PGP,
    /// Deprecated
    ///
    /// Internal
    ///
    /// Broken MD5, take 2
    LittleEndianMD5_2,
    /// Internal
    ///
    /// MD5 signature
    MD5,
    /// Internal
    ///
    /// GnuPG signature
    GPG,
    /// Deprecated
    ///
    /// Internal
    ///
    /// PGP5 signature.
    PGP5,
    /// Internal
    ///
    /// Uncompressed payload size (32bits) in bytes.
    PayloadSize,
    /// Internal
    ///
    /// Space reserved for signatures.
    ReservedSpace,
    /// Internal
    ///
    /// Broken SHA1, take 1.
    BadSHA1_1 = HeaderTags::BadSHA1_1 as i32,
    /// Internal
    ///
    /// Broken SHA1, take 2.
    BadSHA1_2 = HeaderTags::BadSHA1_2 as i32,
    /// Internal
    ///
    /// DSA header signature.
    DSA = HeaderTags::DSAHeader as i32,
    /// Internal
    ///
    /// RSA header signature.
    RSA = HeaderTags::RSAHeader as i32,
    /// Internal
    ///
    /// SHA1 header digest.
    SHA1 = HeaderTags::SHA1Header as i32,
    /// Internal
    ///
    /// Header + Payload size (64bit) in bytes.
    LongSize = HeaderTags::LongSize as i32,
    /// Internal
    ///
    /// Uncompressed payload size (64bit) in bytes.
    LongArchiveSize = HeaderTags::LongArchiveSize as i32,
    SHA256 = HeaderTags::SHA256Header as i32,
    FileSignatures = 274,
    FileSignatureLength,
    VeritySignatures = HeaderTags::VeritySignatures as i32,
    VeritySignatureAlgorithm = HeaderTags::VeritySignatureAlgorithm as i32,
}
