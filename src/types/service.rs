//! AWS services

use std::{fmt::Display, str::FromStr};

use crate::ArnError;

/// A list of known service identifiers.
#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, strum::AsRefStr, strum::IntoStaticStr,
)]
#[strum(
    parse_err_fn = convert_service_parse_err,
    parse_err_ty = Error,
    use_phf
)]
#[cfg_attr(
    feature = "serde",
    derive(serde_with::DeserializeFromStr, serde_with::SerializeDisplay)
)]
pub enum Service {
    /// Corresponds to the service "accessanalyzer"
    AccessAnalyzer,

    /// Corresponds to the service "acm"
    CertificateManager,

    /// Corresponds to the service "acm-pca"
    CertificateManagerPrivateCa,

    /// Corresponds to the service "alexaforbusiness"
    AlexaForBusiness,

    /// Corresponds to the service "amp"
    Prometheus,

    /// Corresponds to the service "amplify"
    Amplify,

    /// Corresponds to the service "amplifybackend"
    AmplifyBackend,

    /// Corresponds to the service "apigateway"
    ApiGateway,

    /// Corresponds to the service "apigatewaymanagementapi"
    ApiGatewayManagementApi,

    /// Corresponds to the service "apigatewayv2"
    ApiGatewayV2,

    /// Corresponds to the service "appconfig"
    AppConfig,

    /// Corresponds to the service "appflow"
    AppFlow,

    /// Corresponds to the service "appintegrations"
    AppIntegrations,

    /// Corresponds to the service "application-autoscaling"
    ApplicationAutoscaling,

    /// Corresponds to the service "application-insights"
    ApplicationInsights,

    /// Corresponds to the service "appmesh"
    AppMesh,

    /// Corresponds to the service "appstream"
    AppStream,

    /// Corresponds to the service "appsync"
    AppSync,

    /// Corresponds to the service "artifact"
    Artifact,

    /// Corresponds to the service "athena"
    Athena,

    /// Corresponds to the service "auditmanager"
    AuditManager,

    /// Corresponds to the service "autoscaling"
    AutoScaling,

    /// Corresponds to the service "autoscaling-plans"
    AutoScalingPlans,

    /// Corresponds to the service "backup"
    Backup,

    /// Corresponds to the service "batch"
    Batch,

    /// Corresponds to the service "braket"
    Braket,

    /// Corresponds to the service "budgets"
    Budgets,

    /// Corresponds to the service "ce"
    CostExplorer,

    /// Corresponds to the service "chime"
    Chime,

    /// Corresponds to the service "cloud9"
    Cloud9,

    /// Corresponds to the service "clouddirectory"
    CloudDirectory,

    /// Corresponds to the service "cloudformation"
    CloudFormation,

    /// Corresponds to the service "cloudhsm"
    CloudHsm,

    /// Corresponds to the service "cloudhsmv2"
    CloudHsmV2,

    /// Corresponds to the service "cloudsearch"
    CloudSearch,

    /// Corresponds to the service "cloudsearchdomain"
    CloudSearchDomain,

    /// Corresponds to the service "cloudtrail"
    CloudTrail,

    /// Corresponds to the service "cloudwatch"
    CloudWatch,

    /// Corresponds to the service "codeartifact"
    CodeArtifact,

    /// Corresponds to the service "codebuild"
    CodeBuild,

    /// Corresponds to the service "codecommit"
    CodeCommit,

    /// Corresponds to the service "codedeploy"
    CodeDeploy,

    /// Corresponds to the service "codeguru-reviewer"
    CodeGuruReviewer,

    /// Corresponds to the service "codeguruprofiler"
    CodeGuruProfiler,

    /// Corresponds to the service "codepipeline"
    CodePipeline,

    /// Corresponds to the service "codestar"
    CodeStar,

    /// Corresponds to the service "codestar-connections"
    CodeStarConnections,

    /// Corresponds to the service "codestar-notifications"
    CodeStarNotifications,

    /// Corresponds to the service "cognito-identity"
    CognitoIdentity,

    /// Corresponds to the service "cognito-idp"
    CognitoIdentityProvider,

    /// Corresponds to the service "cognito-sync"
    CognitoSync,

    /// Corresponds to the service "comprehend"
    Comprehend,

    /// Corresponds to the service "comprehendmedical"
    ComprehendMedical,

    /// Corresponds to the service "compute-optimizer"
    ComputeOptimizer,

    /// Corresponds to the service "config"
    Config,

    /// Corresponds to the service "connect"
    Connect,

    /// Corresponds to the service "connect-contact-lens"
    ConnectContactLens,

    /// Corresponds to the service "connectparticipant"
    ConnectParticipant,

    /// Corresponds to the service "cur"
    CostUsageReport,

    /// Corresponds to the service "customer-profiles"
    CustomerProfiles,

    /// Corresponds to the service "databrew"
    GlueDataBrew,

    /// Corresponds to the service "dataexchange"
    DataExchange,

    /// Corresponds to the service "datapipeline"
    DataPipeline,

    /// Corresponds to the service "datasync"
    DataSync,

    /// Corresponds to the service "dax"
    DynamoDbAccelerator,

    /// Corresponds to the service "detective"
    Detective,

    /// Corresponds to the service "devicefarm"
    DeviceFarm,

    /// Corresponds to the service "devops-guru"
    DevOpsGuru,

    /// Corresponds to the service "directconnect"
    DirectConnect,

    /// Corresponds to the service "discovery"
    Discovery,

    /// Corresponds to the service "dlm"
    DataLifecycleManager,

    /// Corresponds to the service "dms"
    DatabaseMigration,

    /// Corresponds to the service "docdb"
    DocumentDb,

    /// Corresponds to the service "dynamodb"
    DynamoDb,

    /// Corresponds to the service "dynamodbstreams"
    DynamoDbStreams,

    /// Corresponds to the service "ebs"
    ElasticBlockStore,

    /// Corresponds to the service "ec2"
    Ec2,

    /// Corresponds to the service "ec2-instance-connect"
    Ec2InstanceConnect,

    /// Corresponds to the service "ecr"
    Ec2ContainerRegistry,

    /// Corresponds to the service "ecr-public"
    Ec2containerRegistryPublic,

    /// Corresponds to the service "ecs"
    Ec2ContainerService,

    /// Corresponds to the service "efs"
    ElasticFileSystem,

    /// Corresponds to the service "eks"
    ElasticKubernetes,

    /// Corresponds to the service "elastic-inference"
    ElasticInference,

    /// Corresponds to the service "elasticache"
    Elasticache,

    /// Corresponds to the service "elasticbeanstalk"
    ElasticBeanstalk,

    /// Corresponds to the service "elastictranscoder"
    ElasticTranscoder,

    /// Corresponds to the service "elb"
    ElasticLoadBalancing,

    /// Corresponds to the service "elbv2"
    ElasticLoadBalancingV2,

    /// Corresponds to the service "emr"
    ElasticMapReduce,

    /// Corresponds to the service "emr-containers"
    ElasticMapReduceContainers,

    /// Corresponds to the service "es"
    ElasticsearchService,

    /// Corresponds to the service "events"
    EventBridge,

    /// Corresponds to the service "firehose"
    Firehose,

    /// Corresponds to the service "fis"
    FaultInjectionSimulator,

    /// Corresponds to the service "fms"
    FirewallManagementService,

    /// Corresponds to the service "forecast"
    ForecastService,

    /// Corresponds to the service "forecastquery"
    ForecastQueryService,

    /// Corresponds to the service "frauddetector"
    FraudDetector,

    /// Corresponds to the service "fsx"
    Fsx,

    /// Corresponds to the service "gamelift"
    GameLift,

    /// Corresponds to the service "glacier"
    Glacier,

    /// Corresponds to the service "globalaccelerator"
    GlobalAccelerator,

    /// Corresponds to the service "glue"
    Glue,

    /// Corresponds to the service "greengrass"
    Greengrass,

    /// Corresponds to the service "greengrassv2"
    GreengrassV2,

    /// Corresponds to the service "groundstation"
    GroundStation,

    /// Corresponds to the service "guardduty"
    GuardDuty,

    /// Corresponds to the service "health"
    Health,

    /// Corresponds to the service "healthlake"
    HealthLake,

    /// Corresponds to the service "honeycode"
    Honeycode,

    /// Corresponds to the service "iam"
    IdentityAccessManagement,

    /// Corresponds to the service "identitystore"
    IdentityStore,

    /// Corresponds to the service "imagebuilder"
    ImageBuilder,

    /// Corresponds to the service "importexport"
    ImportExport,

    /// Corresponds to the service "inspector"
    Inspector,

    /// Corresponds to the service "iot"
    IoT,

    /// Corresponds to the service "iot-data"
    IoTData,

    /// Corresponds to the service "iot-jobs-data"
    IoTJobsData,

    /// Corresponds to the service "iot1click-devices"
    IoT1clickDevices,

    /// Corresponds to the service "iot1click-projects"
    IoT1clickProjects,

    /// Corresponds to the service "iotanalytics"
    IoTAnalytics,

    /// Corresponds to the service "iotdeviceadvisor"
    IoTDeviceAdvisor,

    /// Corresponds to the service "iotevents"
    IoTEvents,

    /// Corresponds to the service "iotevents-data"
    IoTEventsData,

    /// Corresponds to the service "iotfleethub"
    IoTFleetHub,

    /// Corresponds to the service "iotsecuretunneling"
    IoTSecureTunneling,

    /// Corresponds to the service "iotsitewise"
    IoTSitewise,

    /// Corresponds to the service "iotthingsgraph"
    IoTThingsGraph,

    /// Corresponds to the service "iotwireless"
    IoTWireless,

    /// Corresponds to the service "ivs"
    InteractiveVideo,

    /// Corresponds to the service "kafka"
    Kafka,

    /// Corresponds to the service "kendra"
    Kendra,

    /// Corresponds to the service "kinesis"
    Kinesis,

    /// Corresponds to the service "kinesis-video-archived-media"
    KinesisVideoArchivedMedia,

    /// Corresponds to the service "kinesis-video-media"
    KinesisVideoMedia,

    /// Corresponds to the service "kinesis-video-signaling"
    KinesisVideoSignaling,

    /// Corresponds to the service "kinesisanalytics"
    KinesisAnalytics,

    /// Corresponds to the service "kinesisanalyticsv2"
    KinesisAnalyticsV2,

    /// Corresponds to the service "kinesisvideo"
    KinesisVideo,

    /// Corresponds to the service "kms"
    KeyManagement,

    /// Corresponds to the service "lakeformation"
    LakeFormation,

    /// Corresponds to the service "lambda"
    Lambda,

    /// Corresponds to the service "lex-models"
    LexModels,

    /// Corresponds to the service "lex-runtime"
    LexRuntime,

    /// Corresponds to the service "lexv2-models"
    LexV2Models,

    /// Corresponds to the service "lexv2-runtime"
    LexV2Runtime,

    /// Corresponds to the service "license-manager"
    LicenseManager,

    /// Corresponds to the service "lightsail"
    Lightsail,

    /// Corresponds to the service "location"
    Location,

    /// Corresponds to the service "logs"
    CloudWatchLogs,

    /// Corresponds to the service "lookoutequipment"
    LookoutEquipment,

    /// Corresponds to the service "lookoutmetrics"
    LookoutMetrics,

    /// Corresponds to the service "lookoutvision"
    LookoutVision,

    /// Corresponds to the service "machinelearning"
    MachineLearning,

    /// Corresponds to the service "macie"
    Macie,

    /// Corresponds to the service "macie2"
    Macie2,

    /// Corresponds to the service "managedblockchain"
    ManagedBlockchain,

    /// Corresponds to the service "marketplace-catalog"
    MarketplaceCatalog,

    /// Corresponds to the service "marketplace-entitlement"
    MarketplaceEntitlement,

    /// Corresponds to the service "marketplacecommerceanalytics"
    MarketplaceCommerceAnalytics,

    /// Corresponds to the service "mediaconnect"
    MediaConnect,

    /// Corresponds to the service "mediaconvert"
    MediaConvert,

    /// Corresponds to the service "medialive"
    MediaLive,

    /// Corresponds to the service "mediapackage"
    MediaPackage,

    /// Corresponds to the service "mediapackage-vod"
    MediaPackageVod,

    /// Corresponds to the service "mediastore"
    MediaStore,

    /// Corresponds to the service "mediastore-data"
    MediaStoreData,

    /// Corresponds to the service "mediatailor"
    MediaTailor,

    /// Corresponds to the service "meteringmarketplace"
    MarketplaceMetering,

    /// Corresponds to the service "mgh"
    MigrationHub,

    /// Corresponds to the service "mgn"
    ApplicationMigration,

    /// Corresponds to the service "migrationhub-config"
    MigrationHubConfig,

    /// Corresponds to the service "mobile"
    Mobile,

    /// Corresponds to the service "mq"
    Mq,

    /// Corresponds to the service "mturk"
    MechanicalTurk,

    /// Corresponds to the service "mwaa"
    ManagedWorkflowsForApacheAirflow,

    /// Corresponds to the service "neptune"
    Neptune,

    /// Corresponds to the service "network-firewall"
    NetworkFirewall,

    /// Corresponds to the service "networkmanager"
    NetworkManager,

    /// Corresponds to the service "opsworks"
    OpsWorks,

    /// Corresponds to the service "opsworkscm"
    OpsWorksCm,

    /// Corresponds to the service "organizations"
    Organizations,

    /// Corresponds to the service "outposts"
    Outposts,

    /// Corresponds to the service "personalize"
    Personalize,

    /// Corresponds to the service "personalize-events"
    PersonalizeEvents,

    /// Corresponds to the service "personalize-runtime"
    PersonalizeRuntime,

    /// Corresponds to the service "pi"
    PerformanceInsights,

    /// Corresponds to the service "pinpoint"
    Pinpoint,

    /// Corresponds to the service "pinpoint-email"
    PinpointEmail,

    /// Corresponds to the service "pinpoint-sms-voice"
    PinpointSmsVoice,

    /// Corresponds to the service "polly"
    Polly,

    /// Corresponds to the service "pricing"
    Pricing,

    /// Corresponds to the service "qldb"
    Qldb,

    /// Corresponds to the service "qldb-session"
    QldbSession,

    /// Corresponds to the service "quicksight"
    QuickSight,

    /// Corresponds to the service "ram"
    ResourceAccessManager,

    /// Corresponds to the service "rds"
    RelationalDatabaseService,

    /// Corresponds to the service "rds-data"
    RdsDataService,

    /// Corresponds to the service "redshift"
    Redshift,

    /// Corresponds to the service "redshift-data"
    RedshiftDataApiService,

    /// Corresponds to the service "rekognition"
    Rekognition,

    /// Corresponds to the service "resource-groups"
    ResourceGroups,

    /// Corresponds to the service "resourcegroupstaggingapi"
    ResourceGroupsTaggingApi,

    /// Corresponds to the service "robomaker"
    RoboMaker,

    /// Corresponds to the service "route53"
    Route53,

    /// Corresponds to the service "route53domains"
    Route53Domains,

    /// Corresponds to the service "route53resolver"
    Route53Resolver,

    /// Corresponds to the service "s3"
    S3,

    /// Corresponds to the service "s3control"
    S3Control,

    /// Corresponds to the service "s3outposts"
    S3Outposts,

    /// Corresponds to the service "sagemaker"
    SageMaker,

    /// Corresponds to the service "sagemaker-a2i-runtime"
    AugmentedAiRuntime,

    /// Corresponds to the service "sagemaker-edge"
    SagemakerEdgeManager,

    /// Corresponds to the service "sagemaker-featurestore-runtime"
    SageMakerFeatureStoreRuntime,

    /// Corresponds to the service "sagemaker-runtime"
    SageMakerRuntime,

    /// Corresponds to the service "savingsplans"
    SavingsPlans,

    /// Corresponds to the service "schemas"
    EventBridgeSchemaRegistry,

    /// Corresponds to the service "sdb"
    SimpleDb,

    /// Corresponds to the service "secretsmanager"
    SecretsManager,

    /// Corresponds to the service "securityhub"
    SecurityHub,

    /// Corresponds to the service "serverlessrepo"
    ServerlessApplicationRepository,

    /// Corresponds to the service "service-quotas"
    ServiceQuotas,

    /// Corresponds to the service "servicecatalog"
    ServiceCatalog,

    /// Corresponds to the service "servicecatalog-appregistry"
    ServiceCatalogAppRegistry,

    /// Corresponds to the service "servicediscovery"
    ServiceDiscovery,

    /// Corresponds to the service "ses"
    SimpleEmail,

    /// Corresponds to the service "sesv2"
    SimpleEmailV2,

    /// Corresponds to the service "shield"
    Shield,

    /// Corresponds to the service "signer"
    Signer,

    /// Corresponds to the service "sms"
    ServerMigration,

    /// Corresponds to the service "snowball"
    Snowball,

    /// Corresponds to the service "sns"
    SimpleNotification,

    /// Corresponds to the service "sqs"
    SimpleQueue,

    /// Corresponds to the service "ssm"
    SimpleSystemsManager,

    /// Corresponds to the service "sso"
    SingleSignOn,

    /// Corresponds to the service "sso-admin"
    SingleSignOnAdmin,

    /// Corresponds to the service "sso-oidc"
    SingleSignOnOpenIdConnect,

    /// Corresponds to the service "storagegateway"
    StorageGateway,

    /// Corresponds to the service "sts"
    SecurityToken,

    /// Corresponds to the service "states"
    StepFunctions,

    /// Corresponds to the service "support"
    Support,

    /// Corresponds to the service "swf"
    SimpleWorkflow,

    /// Corresponds to the service "synthetics"
    CloudWatchSynthetics,

    /// Corresponds to the service "textract"
    Textract,

    /// Corresponds to the service "timestream-query"
    TimestreamQuery,

    /// Corresponds to the service "timestream-write"
    TimestreamWrite,

    /// Corresponds to the service "transcribe"
    Transcribe,

    /// Corresponds to the service "transfer"
    Transfer,

    /// Corresponds to the service "translate"
    Translate,

    /// Corresponds to the service "trustedadvisor"
    TrustedAdvisor,

    /// Corresponds to the service "waf"
    WebApplicationFirewall,

    /// Corresponds to the service "waf-regional"
    WebApplicationFirewallRegional,

    /// Corresponds to the service "wafv2"
    WebApplicationFirewallV2,

    /// Corresponds to the service "wellarchitected"
    WellArchitected,

    /// Corresponds to the service "workdocs"
    WorkDocs,

    /// Corresponds to the service "worklink"
    WorkLink,

    /// Corresponds to the service "workmail"
    WorkMail,

    /// Corresponds to the service "workmailmessageflow"
    WorkMailMessageFlow,

    /// Corresponds to the service "workspaces"
    WorkSpaces,

    /// Corresponds to the service "xray"
    XRay,
}

impl Display for Service {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Self::AccessAnalyzer => "accessanalyzer",
            Self::CertificateManager => "acm",
            Self::CertificateManagerPrivateCa => "acm-pca",
            Self::AlexaForBusiness => "alexaforbusiness",
            Self::Prometheus => "amp",
            Self::Amplify => "amplify",
            Self::AmplifyBackend => "amplifybackend",
            Self::ApiGateway => "apigateway",
            Self::ApiGatewayManagementApi => "apigatewaymanagementapi",
            Self::ApiGatewayV2 => "apigatewayv2",
            Self::AppConfig => "appconfig",
            Self::AppFlow => "appflow",
            Self::AppIntegrations => "appintegrations",
            Self::ApplicationAutoscaling => "application-autoscaling",
            Self::ApplicationInsights => "application-insights",
            Self::AppMesh => "appmesh",
            Self::AppStream => "appstream",
            Self::AppSync => "appsync",
            Self::Artifact => "artifact",
            Self::Athena => "athena",
            Self::AuditManager => "auditmanager",
            Self::AutoScaling => "autoscaling",
            Self::AutoScalingPlans => "autoscaling-plans",
            Self::Backup => "backup",
            Self::Batch => "batch",
            Self::Braket => "braket",
            Self::Budgets => "budgets",
            Self::CostExplorer => "ce",
            Self::Chime => "chime",
            Self::Cloud9 => "cloud9",
            Self::CloudDirectory => "clouddirectory",
            Self::CloudFormation => "cloudformation",
            Self::CloudHsm => "cloudhsm",
            Self::CloudHsmV2 => "cloudhsmv2",
            Self::CloudSearch => "cloudsearch",
            Self::CloudSearchDomain => "cloudsearchdomain",
            Self::CloudTrail => "cloudtrail",
            Self::CloudWatch => "cloudwatch",
            Self::CodeArtifact => "codeartifact",
            Self::CodeBuild => "codebuild",
            Self::CodeCommit => "codecommit",
            Self::CodeDeploy => "codedeploy",
            Self::CodeGuruReviewer => "codeguru-reviewer",
            Self::CodeGuruProfiler => "codeguruprofiler",
            Self::CodePipeline => "codepipeline",
            Self::CodeStar => "codestar",
            Self::CodeStarConnections => "codestar-connections",
            Self::CodeStarNotifications => "codestar-notifications",
            Self::CognitoIdentity => "cognito-identity",
            Self::CognitoIdentityProvider => "cognito-idp",
            Self::CognitoSync => "cognito-sync",
            Self::Comprehend => "comprehend",
            Self::ComprehendMedical => "comprehendmedical",
            Self::ComputeOptimizer => "compute-optimizer",
            Self::Config => "config",
            Self::Connect => "connect",
            Self::ConnectContactLens => "connect-contact-lens",
            Self::ConnectParticipant => "connectparticipant",
            Self::CostUsageReport => "cur",
            Self::CustomerProfiles => "customer-profiles",
            Self::GlueDataBrew => "databrew",
            Self::DataExchange => "dataexchange",
            Self::DataPipeline => "datapipeline",
            Self::DataSync => "datasync",
            Self::DynamoDbAccelerator => "dax",
            Self::Detective => "detective",
            Self::DeviceFarm => "devicefarm",
            Self::DevOpsGuru => "devops-guru",
            Self::DirectConnect => "directconnect",
            Self::Discovery => "discovery",
            Self::DataLifecycleManager => "dlm",
            Self::DatabaseMigration => "dms",
            Self::DocumentDb => "docdb",
            Self::DynamoDb => "dynamodb",
            Self::DynamoDbStreams => "dynamodbstreams",
            Self::ElasticBlockStore => "ebs",
            Self::Ec2 => "ec2",
            Self::Ec2InstanceConnect => "ec2-instance-connect",
            Self::Ec2ContainerRegistry => "ecr",
            Self::Ec2containerRegistryPublic => "ecr-public",
            Self::Ec2ContainerService => "ecs",
            Self::ElasticFileSystem => "efs",
            Self::ElasticKubernetes => "eks",
            Self::ElasticInference => "elastic-inference",
            Self::Elasticache => "elasticache",
            Self::ElasticBeanstalk => "elasticbeanstalk",
            Self::ElasticTranscoder => "elastictranscoder",
            Self::ElasticLoadBalancing => "elb",
            Self::ElasticLoadBalancingV2 => "elbv2",
            Self::ElasticMapReduce => "emr",
            Self::ElasticMapReduceContainers => "emr-containers",
            Self::ElasticsearchService => "es",
            Self::EventBridge => "events",
            Self::Firehose => "firehose",
            Self::FaultInjectionSimulator => "fis",
            Self::FirewallManagementService => "fms",
            Self::ForecastService => "forecast",
            Self::ForecastQueryService => "forecastquery",
            Self::FraudDetector => "frauddetector",
            Self::Fsx => "fsx",
            Self::GameLift => "gamelift",
            Self::Glacier => "glacier",
            Self::GlobalAccelerator => "globalaccelerator",
            Self::Glue => "glue",
            Self::Greengrass => "greengrass",
            Self::GreengrassV2 => "greengrassv2",
            Self::GroundStation => "groundstation",
            Self::GuardDuty => "guardduty",
            Self::Health => "health",
            Self::HealthLake => "healthlake",
            Self::Honeycode => "honeycode",
            Self::IdentityAccessManagement => "iam",
            Self::IdentityStore => "identitystore",
            Self::ImageBuilder => "imagebuilder",
            Self::ImportExport => "importexport",
            Self::Inspector => "inspector",
            Self::IoT => "iot",
            Self::IoTData => "iot-data",
            Self::IoTJobsData => "iot-jobs-data",
            Self::IoT1clickDevices => "iot1click-devices",
            Self::IoT1clickProjects => "iot1click-projects",
            Self::IoTAnalytics => "iotanalytics",
            Self::IoTDeviceAdvisor => "iotdeviceadvisor",
            Self::IoTEvents => "iotevents",
            Self::IoTEventsData => "iotevents-data",
            Self::IoTFleetHub => "iotfleethub",
            Self::IoTSecureTunneling => "iotsecuretunneling",
            Self::IoTSitewise => "iotsitewise",
            Self::IoTThingsGraph => "iotthingsgraph",
            Self::IoTWireless => "iotwireless",
            Self::InteractiveVideo => "ivs",
            Self::Kafka => "kafka",
            Self::Kendra => "kendra",
            Self::Kinesis => "kinesis",
            Self::KinesisVideoArchivedMedia => "kinesis-video-archived-media",
            Self::KinesisVideoMedia => "kinesis-video-media",
            Self::KinesisVideoSignaling => "kinesis-video-signaling",
            Self::KinesisAnalytics => "kinesisanalytics",
            Self::KinesisAnalyticsV2 => "kinesisanalyticsv2",
            Self::KinesisVideo => "kinesisvideo",
            Self::KeyManagement => "kms",
            Self::LakeFormation => "lakeformation",
            Self::Lambda => "lambda",
            Self::LexModels => "lex-models",
            Self::LexRuntime => "lex-runtime",
            Self::LexV2Models => "lexv2-models",
            Self::LexV2Runtime => "lexv2-runtime",
            Self::LicenseManager => "license-manager",
            Self::Lightsail => "lightsail",
            Self::Location => "location",
            Self::CloudWatchLogs => "logs",
            Self::LookoutEquipment => "lookoutequipment",
            Self::LookoutMetrics => "lookoutmetrics",
            Self::LookoutVision => "lookoutvision",
            Self::MachineLearning => "machinelearning",
            Self::Macie => "macie",
            Self::Macie2 => "macie2",
            Self::ManagedBlockchain => "managedblockchain",
            Self::MarketplaceCatalog => "marketplace-catalog",
            Self::MarketplaceEntitlement => "marketplace-entitlement",
            Self::MarketplaceCommerceAnalytics => "marketplacecommerceanalytics",
            Self::MediaConnect => "mediaconnect",
            Self::MediaConvert => "mediaconvert",
            Self::MediaLive => "medialive",
            Self::MediaPackage => "mediapackage",
            Self::MediaPackageVod => "mediapackage-vod",
            Self::MediaStore => "mediastore",
            Self::MediaStoreData => "mediastore-data",
            Self::MediaTailor => "mediatailor",
            Self::MarketplaceMetering => "meteringmarketplace",
            Self::MigrationHub => "mgh",
            Self::ApplicationMigration => "mgn",
            Self::MigrationHubConfig => "migrationhub-config",
            Self::Mobile => "mobile",
            Self::Mq => "mq",
            Self::MechanicalTurk => "mturk",
            Self::ManagedWorkflowsForApacheAirflow => "mwaa",
            Self::Neptune => "neptune",
            Self::NetworkFirewall => "network-firewall",
            Self::NetworkManager => "networkmanager",
            Self::OpsWorks => "opsworks",
            Self::OpsWorksCm => "opsworkscm",
            Self::Organizations => "organizations",
            Self::Outposts => "outposts",
            Self::Personalize => "personalize",
            Self::PersonalizeEvents => "personalize-events",
            Self::PersonalizeRuntime => "personalize-runtime",
            Self::PerformanceInsights => "pi",
            Self::Pinpoint => "pinpoint",
            Self::PinpointEmail => "pinpoint-email",
            Self::PinpointSmsVoice => "pinpoint-sms-voice",
            Self::Polly => "polly",
            Self::Pricing => "pricing",
            Self::Qldb => "qldb",
            Self::QldbSession => "qldb-session",
            Self::QuickSight => "quicksight",
            Self::ResourceAccessManager => "ram",
            Self::RelationalDatabaseService => "rds",
            Self::RdsDataService => "rds-data",
            Self::Redshift => "redshift",
            Self::RedshiftDataApiService => "redshift-data",
            Self::Rekognition => "rekognition",
            Self::ResourceGroups => "resource-groups",
            Self::ResourceGroupsTaggingApi => "resourcegroupstaggingapi",
            Self::RoboMaker => "robomaker",
            Self::Route53 => "route53",
            Self::Route53Domains => "route53domains",
            Self::Route53Resolver => "route53resolver",
            Self::S3 => "s3",
            Self::S3Control => "s3control",
            Self::S3Outposts => "s3outposts",
            Self::SageMaker => "sagemaker",
            Self::AugmentedAiRuntime => "sagemaker-a2i-runtime",
            Self::SagemakerEdgeManager => "sagemaker-edge",
            Self::SageMakerFeatureStoreRuntime => "sagemaker-featurestore-runtime",
            Self::SageMakerRuntime => "sagemaker-runtime",
            Self::SavingsPlans => "savingsplans",
            Self::EventBridgeSchemaRegistry => "schemas",
            Self::SimpleDb => "sdb",
            Self::SecretsManager => "secretsmanager",
            Self::SecurityHub => "securityhub",
            Self::ServerlessApplicationRepository => "serverlessrepo",
            Self::ServiceQuotas => "service-quotas",
            Self::ServiceCatalog => "servicecatalog",
            Self::ServiceCatalogAppRegistry => "servicecatalog-appregistry",
            Self::ServiceDiscovery => "servicediscovery",
            Self::SimpleEmail => "ses",
            Self::SimpleEmailV2 => "sesv2",
            Self::Shield => "shield",
            Self::Signer => "signer",
            Self::ServerMigration => "sms",
            Self::Snowball => "snowball",
            Self::SimpleNotification => "sns",
            Self::SimpleQueue => "sqs",
            Self::SimpleSystemsManager => "ssm",
            Self::SingleSignOn => "sso",
            Self::SingleSignOnAdmin => "sso-admin",
            Self::SingleSignOnOpenIdConnect => "sso-oidc",
            Self::StepFunctions => "stepfunctions",
            Self::StorageGateway => "storagegateway",
            Self::SecurityToken => "sts",
            Self::Support => "support",
            Self::SimpleWorkflow => "swf",
            Self::CloudWatchSynthetics => "synthetics",
            Self::Textract => "textract",
            Self::TimestreamQuery => "timestream-query",
            Self::TimestreamWrite => "timestream-write",
            Self::Transcribe => "transcribe",
            Self::Transfer => "transfer",
            Self::Translate => "translate",
            Self::TrustedAdvisor => "trustedadvisor",
            Self::WebApplicationFirewall => "waf",
            Self::WebApplicationFirewallRegional => "waf-regional",
            Self::WebApplicationFirewallV2 => "wafv2",
            Self::WellArchitected => "wellarchitected",
            Self::WorkDocs => "workdocs",
            Self::WorkLink => "worklink",
            Self::WorkMail => "workmail",
            Self::WorkMailMessageFlow => "workmailmessageflow",
            Self::WorkSpaces => "workspaces",
            Self::XRay => "xray",
        };

        write!(f, "{str}")
    }
}

impl FromStr for Service {
    type Err = ArnError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "accessanalyzer" => Ok(Self::AccessAnalyzer),
            "acm" => Ok(Self::CertificateManager),
            "acm-pca" => Ok(Self::CertificateManagerPrivateCa),
            "alexaforbusiness" => Ok(Self::AlexaForBusiness),
            "amp" => Ok(Self::Prometheus),
            "amplify" => Ok(Self::Amplify),
            "amplifybackend" => Ok(Self::AmplifyBackend),
            "apigateway" => Ok(Self::ApiGateway),
            "apigatewaymanagementapi" => Ok(Self::ApiGatewayManagementApi),
            "apigatewayv2" => Ok(Self::ApiGatewayV2),
            "appconfig" => Ok(Self::AppConfig),
            "appflow" => Ok(Self::AppFlow),
            "appintegrations" => Ok(Self::AppIntegrations),
            "application-autoscaling" => Ok(Self::ApplicationAutoscaling),
            "application-insights" => Ok(Self::ApplicationInsights),
            "appmesh" => Ok(Self::AppMesh),
            "appstream" => Ok(Self::AppStream),
            "appsync" => Ok(Self::AppSync),
            "artifact" => Ok(Self::Artifact),
            "athena" => Ok(Self::Athena),
            "auditmanager" => Ok(Self::AuditManager),
            "autoscaling" => Ok(Self::AutoScaling),
            "autoscaling-plans" => Ok(Self::AutoScalingPlans),
            "backup" => Ok(Self::Backup),
            "batch" => Ok(Self::Batch),
            "braket" => Ok(Self::Braket),
            "budgets" => Ok(Self::Budgets),
            "ce" => Ok(Self::CostExplorer),
            "chime" => Ok(Self::Chime),
            "cloud9" => Ok(Self::Cloud9),
            "clouddirectory" => Ok(Self::CloudDirectory),
            "cloudformation" => Ok(Self::CloudFormation),
            "cloudhsm" => Ok(Self::CloudHsm),
            "cloudhsmv2" => Ok(Self::CloudHsmV2),
            "cloudsearch" => Ok(Self::CloudSearch),
            "cloudsearchdomain" => Ok(Self::CloudSearchDomain),
            "cloudtrail" => Ok(Self::CloudTrail),
            "cloudwatch" => Ok(Self::CloudWatch),
            "codeartifact" => Ok(Self::CodeArtifact),
            "codebuild" => Ok(Self::CodeBuild),
            "codecommit" => Ok(Self::CodeCommit),
            "codedeploy" => Ok(Self::CodeDeploy),
            "codeguru-reviewer" => Ok(Self::CodeGuruReviewer),
            "codeguruprofiler" => Ok(Self::CodeGuruProfiler),
            "codepipeline" => Ok(Self::CodePipeline),
            "codestar" => Ok(Self::CodeStar),
            "codestar-connections" => Ok(Self::CodeStarConnections),
            "codestar-notifications" => Ok(Self::CodeStarNotifications),
            "cognito-identity" => Ok(Self::CognitoIdentity),
            "cognito-idp" => Ok(Self::CognitoIdentityProvider),
            "cognito-sync" => Ok(Self::CognitoSync),
            "comprehend" => Ok(Self::Comprehend),
            "comprehendmedical" => Ok(Self::ComprehendMedical),
            "compute-optimizer" => Ok(Self::ComputeOptimizer),
            "config" => Ok(Self::Config),
            "connect" => Ok(Self::Connect),
            "connect-contact-lens" => Ok(Self::ConnectContactLens),
            "connectparticipant" => Ok(Self::ConnectParticipant),
            "cur" => Ok(Self::CostUsageReport),
            "customer-profiles" => Ok(Self::CustomerProfiles),
            "databrew" => Ok(Self::GlueDataBrew),
            "dataexchange" => Ok(Self::DataExchange),
            "datapipeline" => Ok(Self::DataPipeline),
            "datasync" => Ok(Self::DataSync),
            "dax" => Ok(Self::DynamoDbAccelerator),
            "detective" => Ok(Self::Detective),
            "devicefarm" => Ok(Self::DeviceFarm),
            "devops-guru" => Ok(Self::DevOpsGuru),
            "directconnect" => Ok(Self::DirectConnect),
            "discovery" => Ok(Self::Discovery),
            "dlm" => Ok(Self::DataLifecycleManager),
            "dms" => Ok(Self::DatabaseMigration),
            "docdb" => Ok(Self::DocumentDb),
            "dynamodb" => Ok(Self::DynamoDb),
            "dynamodbstreams" => Ok(Self::DynamoDbStreams),
            "ebs" => Ok(Self::ElasticBlockStore),
            "ec2" => Ok(Self::Ec2),
            "ec2-instance-connect" => Ok(Self::Ec2InstanceConnect),
            "ecr" => Ok(Self::Ec2ContainerRegistry),
            "ecr-public" => Ok(Self::Ec2containerRegistryPublic),
            "ecs" => Ok(Self::Ec2ContainerService),
            "efs" => Ok(Self::ElasticFileSystem),
            "elasticfilesystem" => Ok(Self::ElasticFileSystem),
            "eks" => Ok(Self::ElasticKubernetes),
            "elastic-inference" => Ok(Self::ElasticInference),
            "elasticache" => Ok(Self::Elasticache),
            "elasticbeanstalk" => Ok(Self::ElasticBeanstalk),
            "elastictranscoder" => Ok(Self::ElasticTranscoder),
            "elb" => Ok(Self::ElasticLoadBalancing),
            "elasticloadbalancing" => Ok(Self::ElasticLoadBalancing),
            "elbv2" => Ok(Self::ElasticLoadBalancingV2),
            "emr" => Ok(Self::ElasticMapReduce),
            "emr-containers" => Ok(Self::ElasticMapReduceContainers),
            "es" => Ok(Self::ElasticsearchService),
            "events" => Ok(Self::EventBridge),
            "execute-api" => Ok(Self::ApiGateway),
            "firehose" => Ok(Self::Firehose),
            "fis" => Ok(Self::FaultInjectionSimulator),
            "fms" => Ok(Self::FirewallManagementService),
            "forecast" => Ok(Self::ForecastService),
            "forecastquery" => Ok(Self::ForecastQueryService),
            "frauddetector" => Ok(Self::FraudDetector),
            "fsx" => Ok(Self::Fsx),
            "gamelift" => Ok(Self::GameLift),
            "glacier" => Ok(Self::Glacier),
            "globalaccelerator" => Ok(Self::GlobalAccelerator),
            "glue" => Ok(Self::Glue),
            "greengrass" => Ok(Self::Greengrass),
            "greengrassv2" => Ok(Self::GreengrassV2),
            "groundstation" => Ok(Self::GroundStation),
            "guardduty" => Ok(Self::GuardDuty),
            "health" => Ok(Self::Health),
            "healthlake" => Ok(Self::HealthLake),
            "honeycode" => Ok(Self::Honeycode),
            "iam" => Ok(Self::IdentityAccessManagement),
            "identitystore" => Ok(Self::IdentityStore),
            "imagebuilder" => Ok(Self::ImageBuilder),
            "importexport" => Ok(Self::ImportExport),
            "inspector" => Ok(Self::Inspector),
            "iot" => Ok(Self::IoT),
            "iot-data" => Ok(Self::IoTData),
            "iot-jobs-data" => Ok(Self::IoTJobsData),
            "iot1click-devices" => Ok(Self::IoT1clickDevices),
            "iot1click-projects" => Ok(Self::IoT1clickProjects),
            "iotanalytics" => Ok(Self::IoTAnalytics),
            "iotdeviceadvisor" => Ok(Self::IoTDeviceAdvisor),
            "iotevents" => Ok(Self::IoTEvents),
            "iotevents-data" => Ok(Self::IoTEventsData),
            "iotfleethub" => Ok(Self::IoTFleetHub),
            "iotsecuretunneling" => Ok(Self::IoTSecureTunneling),
            "iotsitewise" => Ok(Self::IoTSitewise),
            "iotthingsgraph" => Ok(Self::IoTThingsGraph),
            "iotwireless" => Ok(Self::IoTWireless),
            "ivs" => Ok(Self::InteractiveVideo),
            "kafka" => Ok(Self::Kafka),
            "kendra" => Ok(Self::Kendra),
            "kinesis" => Ok(Self::Kinesis),
            "kinesis-video-archived-media" => Ok(Self::KinesisVideoArchivedMedia),
            "kinesis-video-media" => Ok(Self::KinesisVideoMedia),
            "kinesis-video-signaling" => Ok(Self::KinesisVideoSignaling),
            "kinesisanalytics" => Ok(Self::KinesisAnalytics),
            "kinesisanalyticsv2" => Ok(Self::KinesisAnalyticsV2),
            "kinesisvideo" => Ok(Self::KinesisVideo),
            "kms" => Ok(Self::KeyManagement),
            "lakeformation" => Ok(Self::LakeFormation),
            "lambda" => Ok(Self::Lambda),
            "lex-models" => Ok(Self::LexModels),
            "lex-runtime" => Ok(Self::LexRuntime),
            "lexv2-models" => Ok(Self::LexV2Models),
            "lexv2-runtime" => Ok(Self::LexV2Runtime),
            "license-manager" => Ok(Self::LicenseManager),
            "lightsail" => Ok(Self::Lightsail),
            "location" => Ok(Self::Location),
            "logs" => Ok(Self::CloudWatchLogs),
            "lookoutequipment" => Ok(Self::LookoutEquipment),
            "lookoutmetrics" => Ok(Self::LookoutMetrics),
            "lookoutvision" => Ok(Self::LookoutVision),
            "machinelearning" => Ok(Self::MachineLearning),
            "macie" => Ok(Self::Macie),
            "macie2" => Ok(Self::Macie2),
            "managedblockchain" => Ok(Self::ManagedBlockchain),
            "marketplace-catalog" => Ok(Self::MarketplaceCatalog),
            "marketplace-entitlement" => Ok(Self::MarketplaceEntitlement),
            "marketplacecommerceanalytics" => Ok(Self::MarketplaceCommerceAnalytics),
            "mediaconnect" => Ok(Self::MediaConnect),
            "mediaconvert" => Ok(Self::MediaConvert),
            "medialive" => Ok(Self::MediaLive),
            "mediapackage" => Ok(Self::MediaPackage),
            "mediapackage-vod" => Ok(Self::MediaPackageVod),
            "mediastore" => Ok(Self::MediaStore),
            "mediastore-data" => Ok(Self::MediaStoreData),
            "mediatailor" => Ok(Self::MediaTailor),
            "meteringmarketplace" => Ok(Self::MarketplaceMetering),
            "mgh" => Ok(Self::MigrationHub),
            "mgn" => Ok(Self::ApplicationMigration),
            "migrationhub-config" => Ok(Self::MigrationHubConfig),
            "mobile" => Ok(Self::Mobile),
            "mq" => Ok(Self::Mq),
            "mturk" => Ok(Self::MechanicalTurk),
            "mwaa" => Ok(Self::ManagedWorkflowsForApacheAirflow),
            "neptune" => Ok(Self::Neptune),
            "network-firewall" => Ok(Self::NetworkFirewall),
            "networkmanager" => Ok(Self::NetworkManager),
            "opsworks" => Ok(Self::OpsWorks),
            "opsworkscm" => Ok(Self::OpsWorksCm),
            "organizations" => Ok(Self::Organizations),
            "outposts" => Ok(Self::Outposts),
            "personalize" => Ok(Self::Personalize),
            "personalize-events" => Ok(Self::PersonalizeEvents),
            "personalize-runtime" => Ok(Self::PersonalizeRuntime),
            "pi" => Ok(Self::PerformanceInsights),
            "pinpoint" => Ok(Self::Pinpoint),
            "pinpoint-email" => Ok(Self::PinpointEmail),
            "pinpoint-sms-voice" => Ok(Self::PinpointSmsVoice),
            "polly" => Ok(Self::Polly),
            "pricing" => Ok(Self::Pricing),
            "qldb" => Ok(Self::Qldb),
            "qldb-session" => Ok(Self::QldbSession),
            "quicksight" => Ok(Self::QuickSight),
            "ram" => Ok(Self::ResourceAccessManager),
            "rds" => Ok(Self::RelationalDatabaseService),
            "rds-data" => Ok(Self::RdsDataService),
            "redshift" => Ok(Self::Redshift),
            "redshift-data" => Ok(Self::RedshiftDataApiService),
            "rekognition" => Ok(Self::Rekognition),
            "resource-groups" => Ok(Self::ResourceGroups),
            "resourcegroupstaggingapi" => Ok(Self::ResourceGroupsTaggingApi),
            "robomaker" => Ok(Self::RoboMaker),
            "route53" => Ok(Self::Route53),
            "route53domains" => Ok(Self::Route53Domains),
            "route53resolver" => Ok(Self::Route53Resolver),
            "s3" => Ok(Self::S3),
            "s3control" => Ok(Self::S3Control),
            "s3outposts" => Ok(Self::S3Outposts),
            "sagemaker" => Ok(Self::SageMaker),
            "sagemaker-a2i-runtime" => Ok(Self::AugmentedAiRuntime),
            "sagemaker-edge" => Ok(Self::SagemakerEdgeManager),
            "sagemaker-featurestore-runtime" => Ok(Self::SageMakerFeatureStoreRuntime),
            "sagemaker-runtime" => Ok(Self::SageMakerRuntime),
            "savingsplans" => Ok(Self::SavingsPlans),
            "schemas" => Ok(Self::EventBridgeSchemaRegistry),
            "sdb" => Ok(Self::SimpleDb),
            "secretsmanager" => Ok(Self::SecretsManager),
            "securityhub" => Ok(Self::SecurityHub),
            "serverlessrepo" => Ok(Self::ServerlessApplicationRepository),
            "service-quotas" => Ok(Self::ServiceQuotas),
            "servicecatalog" => Ok(Self::ServiceCatalog),
            "servicecatalog-appregistry" => Ok(Self::ServiceCatalogAppRegistry),
            "servicediscovery" => Ok(Self::ServiceDiscovery),
            "ses" => Ok(Self::SimpleEmail),
            "sesv2" => Ok(Self::SimpleEmailV2),
            "shield" => Ok(Self::Shield),
            "signer" => Ok(Self::Signer),
            "sms" => Ok(Self::ServerMigration),
            "snowball" => Ok(Self::Snowball),
            "sns" => Ok(Self::SimpleNotification),
            "sqs" => Ok(Self::SimpleQueue),
            "ssm" => Ok(Self::SimpleSystemsManager),
            "sso" => Ok(Self::SingleSignOn),
            "sso-admin" => Ok(Self::SingleSignOnAdmin),
            "sso-oidc" => Ok(Self::SingleSignOnOpenIdConnect),
            "states" => Ok(Self::StepFunctions),
            "stepfunctions" => Ok(Self::StepFunctions),
            "storagegateway" => Ok(Self::StorageGateway),
            "sts" => Ok(Self::SecurityToken),
            "support" => Ok(Self::Support),
            "swf" => Ok(Self::SimpleWorkflow),
            "synthetics" => Ok(Self::CloudWatchSynthetics),
            "textract" => Ok(Self::Textract),
            "timestream-query" => Ok(Self::TimestreamQuery),
            "timestream-write" => Ok(Self::TimestreamWrite),
            "transcribe" => Ok(Self::Transcribe),
            "transfer" => Ok(Self::Transfer),
            "translate" => Ok(Self::Translate),
            "trustedadvisor" => Ok(Self::TrustedAdvisor),
            "waf" => Ok(Self::WebApplicationFirewall),
            "waf-regional" => Ok(Self::WebApplicationFirewallRegional),
            "wafv2" => Ok(Self::WebApplicationFirewallV2),
            "wellarchitected" => Ok(Self::WellArchitected),
            "workdocs" => Ok(Self::WorkDocs),
            "worklink" => Ok(Self::WorkLink),
            "workmail" => Ok(Self::WorkMail),
            "workmailmessageflow" => Ok(Self::WorkMailMessageFlow),
            "workspaces" => Ok(Self::WorkSpaces),
            "xray" => Ok(Self::XRay),
            s => Err(ArnError::InvalidService(s.to_string())),
        }
    }
}

fn convert_service_parse_err(s: &str) -> ArnError {
    ArnError::InvalidService(s.to_string())
}
