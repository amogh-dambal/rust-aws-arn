//! AWS services
#![allow(missing_docs)]

use crate::ArnError;

/// A list of known service identifiers.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    strum::AsRefStr,
    strum::IntoStaticStr,
    strum::Display,
    strum::EnumString,
)]
#[strum(
    parse_err_fn = convert_service_parse_err,
    parse_err_ty = ArnError,
    use_phf
)]
#[cfg_attr(
    feature = "serde",
    derive(serde_with::DeserializeFromStr, serde_with::SerializeDisplay)
)]
pub enum Service {
    #[strum(serialize = "accessanalyzer")]
    AccessAnalyzer,

    #[strum(serialize = "acm")]
    CertificateManager,

    #[strum(serialize = "acm-pca")]
    CertificateManagerPrivateCa,

    #[strum(serialize = "alexaforbusiness")]
    AlexaForBusiness,

    #[strum(serialize = "amp")]
    Prometheus,

    #[strum(serialize = "amplify")]
    Amplify,

    #[strum(serialize = "amplifybackend")]
    AmplifyBackend,

    #[strum(serialize = "apigateway")]
    ApiGateway,

    #[strum(serialize = "execute-api")]
    ApiGatewayExecuteApi,

    #[strum(serialize = "apigatewaymanagementapi")]
    ApiGatewayManagementApi,

    #[strum(serialize = "apigatewayv2")]
    ApiGatewayV2,

    #[strum(serialize = "appconfig")]
    AppConfig,

    #[strum(serialize = "appflow")]
    AppFlow,

    #[strum(serialize = "appintegrations")]
    AppIntegrations,

    #[strum(serialize = "application-autoscaling")]
    ApplicationAutoscaling,

    #[strum(serialize = "application-insights")]
    ApplicationInsights,

    #[strum(serialize = "appmesh")]
    AppMesh,

    #[strum(serialize = "appstream")]
    AppStream,

    #[strum(serialize = "appsync")]
    AppSync,

    #[strum(serialize = "artifact")]
    Artifact,

    #[strum(serialize = "athena")]
    Athena,

    #[strum(serialize = "auditmanager")]
    AuditManager,

    #[strum(serialize = "autoscaling")]
    AutoScaling,

    #[strum(serialize = "autoscaling-plans")]
    AutoScalingPlans,

    #[strum(serialize = "backup")]
    Backup,

    #[strum(serialize = "batch")]
    Batch,

    #[strum(serialize = "braket")]
    Braket,

    #[strum(serialize = "budgets")]
    Budgets,

    #[strum(serialize = "ce")]
    CostExplorer,

    #[strum(serialize = "chime")]
    Chime,

    #[strum(serialize = "cloud9")]
    Cloud9,

    #[strum(serialize = "clouddirectory")]
    CloudDirectory,

    #[strum(serialize = "cloudformation")]
    CloudFormation,

    #[strum(serialize = "cloudhsm")]
    CloudHsm,

    #[strum(serialize = "cloudhsmv2")]
    CloudHsmV2,

    #[strum(serialize = "cloudsearch")]
    CloudSearch,

    #[strum(serialize = "cloudsearchdomain")]
    CloudSearchDomain,

    #[strum(serialize = "cloudtrail")]
    CloudTrail,

    #[strum(serialize = "cloudwatch")]
    CloudWatch,

    #[strum(serialize = "codeartifact")]
    CodeArtifact,

    #[strum(serialize = "codebuild")]
    CodeBuild,

    #[strum(serialize = "codecommit")]
    CodeCommit,

    #[strum(serialize = "codedeploy")]
    CodeDeploy,

    #[strum(serialize = "codeguru-reviewer")]
    CodeGuruReviewer,

    #[strum(serialize = "codeguruprofiler")]
    CodeGuruProfiler,

    #[strum(serialize = "codepipeline")]
    CodePipeline,

    #[strum(serialize = "codestar")]
    CodeStar,

    #[strum(serialize = "codestar-connections")]
    CodeStarConnections,

    #[strum(serialize = "codestar-notifications")]
    CodeStarNotifications,

    #[strum(serialize = "cognito-identity")]
    CognitoIdentity,

    #[strum(serialize = "cognito-idp")]
    CognitoIdentityProvider,

    #[strum(serialize = "cognito-sync")]
    CognitoSync,

    #[strum(serialize = "comprehend")]
    Comprehend,

    #[strum(serialize = "comprehendmedical")]
    ComprehendMedical,

    #[strum(serialize = "compute-optimizer")]
    ComputeOptimizer,

    #[strum(serialize = "config")]
    Config,

    #[strum(serialize = "connect")]
    Connect,

    #[strum(serialize = "connect-contact-lens")]
    ConnectContactLens,

    #[strum(serialize = "connectparticipant")]
    ConnectParticipant,

    #[strum(serialize = "cur")]
    CostUsageReport,

    #[strum(serialize = "customer-profiles")]
    CustomerProfiles,

    #[strum(serialize = "databrew")]
    GlueDataBrew,

    #[strum(serialize = "dataexchange")]
    DataExchange,

    #[strum(serialize = "datapipeline")]
    DataPipeline,

    #[strum(serialize = "datasync")]
    DataSync,

    #[strum(serialize = "dax")]
    DynamoDbAccelerator,

    #[strum(serialize = "detective")]
    Detective,

    #[strum(serialize = "devicefarm")]
    DeviceFarm,

    #[strum(serialize = "devops-guru")]
    DevOpsGuru,

    #[strum(serialize = "directconnect")]
    DirectConnect,

    #[strum(serialize = "discovery")]
    Discovery,

    #[strum(serialize = "dlm")]
    DataLifecycleManager,

    #[strum(serialize = "dms")]
    DatabaseMigration,

    #[strum(serialize = "docdb")]
    DocumentDb,

    #[strum(serialize = "dynamodb")]
    DynamoDb,

    #[strum(serialize = "dynamodbstreams")]
    DynamoDbStreams,

    #[strum(serialize = "ebs")]
    ElasticBlockStore,

    #[strum(serialize = "ec2")]
    Ec2,

    #[strum(serialize = "ec2-instance-connect")]
    Ec2InstanceConnect,

    #[strum(serialize = "ecr")]
    Ec2ContainerRegistry,

    #[strum(serialize = "ecr-public")]
    Ec2containerRegistryPublic,

    #[strum(serialize = "ecs")]
    Ec2ContainerService,

    #[strum(serialize = "efs")]
    Efs,

    #[strum(serialize = "elasticfilesystem")]
    ElasticFileSystem,

    #[strum(serialize = "eks")]
    ElasticKubernetes,

    #[strum(serialize = "elastic-inference")]
    ElasticInference,

    #[strum(serialize = "elasticache")]
    Elasticache,

    #[strum(serialize = "elasticbeanstalk")]
    ElasticBeanstalk,

    #[strum(serialize = "elastictranscoder")]
    ElasticTranscoder,

    #[strum(serialize = "elb")]
    Elb,

    #[strum(serialize = "elasticloadbalancing")]
    ElasticLoadBalancing,

    #[strum(serialize = "elbv2")]
    ElasticLoadBalancingV2,

    #[strum(serialize = "emr")]
    ElasticMapReduce,

    #[strum(serialize = "emr-containers")]
    ElasticMapReduceContainers,

    #[strum(serialize = "es")]
    ElasticsearchService,

    #[strum(serialize = "events")]
    EventBridge,

    #[strum(serialize = "firehose")]
    Firehose,

    #[strum(serialize = "fis")]
    FaultInjectionSimulator,

    #[strum(serialize = "fms")]
    FirewallManagementService,

    #[strum(serialize = "forecast")]
    ForecastService,

    #[strum(serialize = "forecastquery")]
    ForecastQueryService,

    #[strum(serialize = "frauddetector")]
    FraudDetector,

    #[strum(serialize = "fsx")]
    Fsx,

    #[strum(serialize = "gamelift")]
    GameLift,

    #[strum(serialize = "glacier")]
    Glacier,

    #[strum(serialize = "globalaccelerator")]
    GlobalAccelerator,

    #[strum(serialize = "glue")]
    Glue,

    #[strum(serialize = "greengrass")]
    Greengrass,

    #[strum(serialize = "greengrassv2")]
    GreengrassV2,

    #[strum(serialize = "groundstation")]
    GroundStation,

    #[strum(serialize = "guardduty")]
    GuardDuty,

    #[strum(serialize = "health")]
    Health,

    #[strum(serialize = "healthlake")]
    HealthLake,

    #[strum(serialize = "honeycode")]
    Honeycode,

    #[strum(serialize = "iam")]
    IdentityAccessManagement,

    #[strum(serialize = "identitystore")]
    IdentityStore,

    #[strum(serialize = "imagebuilder")]
    ImageBuilder,

    #[strum(serialize = "importexport")]
    ImportExport,

    #[strum(serialize = "inspector")]
    Inspector,

    #[strum(serialize = "iot")]
    IoT,

    #[strum(serialize = "iot-data")]
    IoTData,

    #[strum(serialize = "iot-jobs-data")]
    IoTJobsData,

    #[strum(serialize = "iot1click-devices")]
    IoT1clickDevices,

    #[strum(serialize = "iot1click-projects")]
    IoT1clickProjects,

    #[strum(serialize = "iotanalytics")]
    IoTAnalytics,

    #[strum(serialize = "iotdeviceadvisor")]
    IoTDeviceAdvisor,

    #[strum(serialize = "iotevents")]
    IoTEvents,

    #[strum(serialize = "iotevents-data")]
    IoTEventsData,

    #[strum(serialize = "iotfleethub")]
    IoTFleetHub,

    #[strum(serialize = "iotsecuretunneling")]
    IoTSecureTunneling,

    #[strum(serialize = "iotsitewise")]
    IoTSitewise,

    #[strum(serialize = "iotthingsgraph")]
    IoTThingsGraph,

    #[strum(serialize = "iotwireless")]
    IoTWireless,

    #[strum(serialize = "ivs")]
    InteractiveVideo,

    #[strum(serialize = "kafka")]
    Kafka,

    #[strum(serialize = "kendra")]
    Kendra,

    #[strum(serialize = "kinesis")]
    Kinesis,

    #[strum(serialize = "kinesis-video-archived-media")]
    KinesisVideoArchivedMedia,

    #[strum(serialize = "kinesis-video-media")]
    KinesisVideoMedia,

    #[strum(serialize = "kinesis-video-signaling")]
    KinesisVideoSignaling,

    #[strum(serialize = "kinesisanalytics")]
    KinesisAnalytics,

    #[strum(serialize = "kinesisanalyticsv2")]
    KinesisAnalyticsV2,

    #[strum(serialize = "kinesisvideo")]
    KinesisVideo,

    #[strum(serialize = "kms")]
    KeyManagement,

    #[strum(serialize = "lakeformation")]
    LakeFormation,

    #[strum(serialize = "lambda")]
    Lambda,

    #[strum(serialize = "lex-models")]
    LexModels,

    #[strum(serialize = "lex-runtime")]
    LexRuntime,

    #[strum(serialize = "lexv2-models")]
    LexV2Models,

    #[strum(serialize = "lexv2-runtime")]
    LexV2Runtime,

    #[strum(serialize = "license-manager")]
    LicenseManager,

    #[strum(serialize = "lightsail")]
    Lightsail,

    #[strum(serialize = "location")]
    Location,

    #[strum(serialize = "logs")]
    CloudWatchLogs,

    #[strum(serialize = "lookoutequipment")]
    LookoutEquipment,

    #[strum(serialize = "lookoutmetrics")]
    LookoutMetrics,

    #[strum(serialize = "lookoutvision")]
    LookoutVision,

    #[strum(serialize = "machinelearning")]
    MachineLearning,

    #[strum(serialize = "macie")]
    Macie,

    #[strum(serialize = "macie2")]
    Macie2,

    #[strum(serialize = "managedblockchain")]
    ManagedBlockchain,

    #[strum(serialize = "marketplace-catalog")]
    MarketplaceCatalog,

    #[strum(serialize = "marketplace-entitlement")]
    MarketplaceEntitlement,

    #[strum(serialize = "marketplacecommerceanalytics")]
    MarketplaceCommerceAnalytics,

    #[strum(serialize = "mediaconnect")]
    MediaConnect,

    #[strum(serialize = "mediaconvert")]
    MediaConvert,

    #[strum(serialize = "medialive")]
    MediaLive,

    #[strum(serialize = "mediapackage")]
    MediaPackage,

    #[strum(serialize = "mediapackage-vod")]
    MediaPackageVod,

    #[strum(serialize = "mediastore")]
    MediaStore,

    #[strum(serialize = "mediastore-data")]
    MediaStoreData,

    #[strum(serialize = "mediatailor")]
    MediaTailor,

    #[strum(serialize = "meteringmarketplace")]
    MarketplaceMetering,

    #[strum(serialize = "mgh")]
    MigrationHub,

    #[strum(serialize = "mgn")]
    ApplicationMigration,

    #[strum(serialize = "migrationhub-config")]
    MigrationHubConfig,

    #[strum(serialize = "mobile")]
    Mobile,

    #[strum(serialize = "mq")]
    Mq,

    #[strum(serialize = "mturk")]
    MechanicalTurk,

    #[strum(serialize = "mwaa")]
    ManagedWorkflowsForApacheAirflow,

    #[strum(serialize = "neptune")]
    Neptune,

    #[strum(serialize = "network-firewall")]
    NetworkFirewall,

    #[strum(serialize = "networkmanager")]
    NetworkManager,

    #[strum(serialize = "opsworks")]
    OpsWorks,

    #[strum(serialize = "opsworkscm")]
    OpsWorksCm,

    #[strum(serialize = "organizations")]
    Organizations,

    #[strum(serialize = "outposts")]
    Outposts,

    #[strum(serialize = "personalize")]
    Personalize,

    #[strum(serialize = "personalize-events")]
    PersonalizeEvents,

    #[strum(serialize = "personalize-runtime")]
    PersonalizeRuntime,

    #[strum(serialize = "pi")]
    PerformanceInsights,

    #[strum(serialize = "pinpoint")]
    Pinpoint,

    #[strum(serialize = "pinpoint-email")]
    PinpointEmail,

    #[strum(serialize = "pinpoint-sms-voice")]
    PinpointSmsVoice,

    #[strum(serialize = "polly")]
    Polly,

    #[strum(serialize = "pricing")]
    Pricing,

    #[strum(serialize = "qldb")]
    Qldb,

    #[strum(serialize = "qldb-session")]
    QldbSession,

    #[strum(serialize = "quicksight")]
    QuickSight,

    #[strum(serialize = "ram")]
    ResourceAccessManager,

    #[strum(serialize = "rds")]
    RelationalDatabaseService,

    #[strum(serialize = "rds-data")]
    RdsDataService,

    #[strum(serialize = "redshift")]
    Redshift,

    #[strum(serialize = "redshift-data")]
    RedshiftDataApiService,

    #[strum(serialize = "rekognition")]
    Rekognition,

    #[strum(serialize = "resource-groups")]
    ResourceGroups,

    #[strum(serialize = "resourcegroupstaggingapi")]
    ResourceGroupsTaggingApi,

    #[strum(serialize = "robomaker")]
    RoboMaker,

    #[strum(serialize = "route53")]
    Route53,

    #[strum(serialize = "route53domains")]
    Route53Domains,

    #[strum(serialize = "route53resolver")]
    Route53Resolver,

    #[strum(serialize = "s3")]
    S3,

    #[strum(serialize = "s3control")]
    S3Control,

    #[strum(serialize = "s3outposts")]
    S3Outposts,

    #[strum(serialize = "sagemaker")]
    SageMaker,

    #[strum(serialize = "sagemaker-a2i-runtime")]
    AugmentedAiRuntime,

    #[strum(serialize = "sagemaker-edge")]
    SagemakerEdgeManager,

    #[strum(serialize = "sagemaker-featurestore-runtime")]
    SageMakerFeatureStoreRuntime,

    #[strum(serialize = "sagemaker-runtime")]
    SageMakerRuntime,

    #[strum(serialize = "savingsplans")]
    SavingsPlans,

    #[strum(serialize = "schemas")]
    EventBridgeSchemaRegistry,

    #[strum(serialize = "sdb")]
    SimpleDb,

    #[strum(serialize = "secretsmanager")]
    SecretsManager,

    #[strum(serialize = "securityhub")]
    SecurityHub,

    #[strum(serialize = "serverlessrepo")]
    ServerlessApplicationRepository,

    #[strum(serialize = "service-quotas")]
    ServiceQuotas,

    #[strum(serialize = "servicecatalog")]
    ServiceCatalog,

    #[strum(serialize = "servicecatalog-appregistry")]
    ServiceCatalogAppRegistry,

    #[strum(serialize = "servicediscovery")]
    ServiceDiscovery,

    #[strum(serialize = "ses")]
    SimpleEmail,

    #[strum(serialize = "sesv2")]
    SimpleEmailV2,

    #[strum(serialize = "shield")]
    Shield,

    #[strum(serialize = "signer")]
    Signer,

    #[strum(serialize = "sms")]
    ServerMigration,

    #[strum(serialize = "snowball")]
    Snowball,

    #[strum(serialize = "sns")]
    SimpleNotification,

    #[strum(serialize = "sqs")]
    SimpleQueue,

    #[strum(serialize = "ssm")]
    SimpleSystemsManager,

    #[strum(serialize = "sso")]
    SingleSignOn,

    #[strum(serialize = "sso-admin")]
    SingleSignOnAdmin,

    #[strum(serialize = "sso-oidc")]
    SingleSignOnOpenIdConnect,

    #[strum(serialize = "storagegateway")]
    StorageGateway,

    #[strum(serialize = "sts")]
    SecurityToken,

    #[strum(serialize = "states")]
    States,

    #[strum(serialize = "stepfunctions")]
    StepFunctions,

    #[strum(serialize = "support")]
    Support,

    #[strum(serialize = "swf")]
    SimpleWorkflow,

    #[strum(serialize = "synthetics")]
    CloudWatchSynthetics,

    #[strum(serialize = "textract")]
    Textract,

    #[strum(serialize = "timestream-query")]
    TimestreamQuery,

    #[strum(serialize = "timestream-write")]
    TimestreamWrite,

    #[strum(serialize = "transcribe")]
    Transcribe,

    #[strum(serialize = "transfer")]
    Transfer,

    #[strum(serialize = "translate")]
    Translate,

    #[strum(serialize = "trustedadvisor")]
    TrustedAdvisor,

    #[strum(serialize = "waf")]
    WebApplicationFirewall,

    #[strum(serialize = "waf-regional")]
    WebApplicationFirewallRegional,

    #[strum(serialize = "wafv2")]
    WebApplicationFirewallV2,

    #[strum(serialize = "wellarchitected")]
    WellArchitected,

    #[strum(serialize = "workdocs")]
    WorkDocs,

    #[strum(serialize = "worklink")]
    WorkLink,

    #[strum(serialize = "workmail")]
    WorkMail,

    #[strum(serialize = "workmailmessageflow")]
    WorkMailMessageFlow,

    #[strum(serialize = "workspaces")]
    WorkSpaces,

    #[strum(serialize = "xray")]
    XRay,
}

fn convert_service_parse_err(s: &str) -> ArnError {
    ArnError::InvalidService(s.to_string())
}
