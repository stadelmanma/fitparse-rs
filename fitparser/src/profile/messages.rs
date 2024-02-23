#![allow(missing_docs)]
#![doc = "Auto generated profile messages from FIT SDK Release: 21.133.00"]
use crate::{
    profile::{FitMessage, MesgNum, TryFromRecordError},
    FitDataField, FitDataRecord, Value,
};
use serde::Serialize;
#[doc = r" All supported message types."]
#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize)]
pub enum Message {
    FileId(FileId),
    FileCreator(FileCreator),
    TimestampCorrelation(TimestampCorrelation),
    Software(Software),
    SlaveDevice(SlaveDevice),
    Capabilities(Capabilities),
    FileCapabilities(FileCapabilities),
    MesgCapabilities(MesgCapabilities),
    FieldCapabilities(FieldCapabilities),
    DeviceSettings(DeviceSettings),
    UserProfile(UserProfile),
    HrmProfile(HrmProfile),
    SdmProfile(SdmProfile),
    BikeProfile(BikeProfile),
    Connectivity(Connectivity),
    WatchfaceSettings(WatchfaceSettings),
    OhrSettings(OhrSettings),
    TimeInZone(TimeInZone),
    ZonesTarget(ZonesTarget),
    Sport(Sport),
    HrZone(HrZone),
    SpeedZone(SpeedZone),
    CadenceZone(CadenceZone),
    PowerZone(PowerZone),
    MetZone(MetZone),
    DiveSettings(DiveSettings),
    DiveAlarm(DiveAlarm),
    DiveApneaAlarm(DiveApneaAlarm),
    DiveGas(DiveGas),
    Goal(Goal),
    Activity(Activity),
    Session(Session),
    Lap(Lap),
    Length(Length),
    Record(Record),
    Event(Event),
    DeviceInfo(DeviceInfo),
    DeviceAuxBatteryInfo(DeviceAuxBatteryInfo),
    TrainingFile(TrainingFile),
    WeatherConditions(WeatherConditions),
    WeatherAlert(WeatherAlert),
    GpsMetadata(GpsMetadata),
    CameraEvent(CameraEvent),
    GyroscopeData(GyroscopeData),
    AccelerometerData(AccelerometerData),
    MagnetometerData(MagnetometerData),
    BarometerData(BarometerData),
    ThreeDSensorCalibration(ThreeDSensorCalibration),
    OneDSensorCalibration(OneDSensorCalibration),
    VideoFrame(VideoFrame),
    ObdiiData(ObdiiData),
    NmeaSentence(NmeaSentence),
    AviationAttitude(AviationAttitude),
    Video(Video),
    VideoTitle(VideoTitle),
    VideoDescription(VideoDescription),
    VideoClip(VideoClip),
    Set(Set),
    Jump(Jump),
    Split(Split),
    SplitSummary(SplitSummary),
    ClimbPro(ClimbPro),
    FieldDescription(FieldDescription),
    DeveloperDataId(DeveloperDataId),
    Course(Course),
    CoursePoint(CoursePoint),
    SegmentId(SegmentId),
    SegmentLeaderboardEntry(SegmentLeaderboardEntry),
    SegmentPoint(SegmentPoint),
    SegmentLap(SegmentLap),
    SegmentFile(SegmentFile),
    Workout(Workout),
    WorkoutSession(WorkoutSession),
    WorkoutStep(WorkoutStep),
    ExerciseTitle(ExerciseTitle),
    Schedule(Schedule),
    Totals(Totals),
    WeightScale(WeightScale),
    BloodPressure(BloodPressure),
    MonitoringInfo(MonitoringInfo),
    Monitoring(Monitoring),
    MonitoringHrData(MonitoringHrData),
    Spo2Data(Spo2Data),
    Hr(Hr),
    StressLevel(StressLevel),
    MaxMetData(MaxMetData),
    HsaBodyBatteryData(HsaBodyBatteryData),
    HsaEvent(HsaEvent),
    HsaAccelerometerData(HsaAccelerometerData),
    HsaGyroscopeData(HsaGyroscopeData),
    HsaStepData(HsaStepData),
    HsaSpo2Data(HsaSpo2Data),
    HsaStressData(HsaStressData),
    HsaRespirationData(HsaRespirationData),
    HsaHeartRateData(HsaHeartRateData),
    HsaConfigurationData(HsaConfigurationData),
    HsaWristTemperatureData(HsaWristTemperatureData),
    MemoGlob(MemoGlob),
    SleepLevel(SleepLevel),
    AntChannelId(AntChannelId),
    AntRx(AntRx),
    AntTx(AntTx),
    ExdScreenConfiguration(ExdScreenConfiguration),
    ExdDataFieldConfiguration(ExdDataFieldConfiguration),
    ExdDataConceptConfiguration(ExdDataConceptConfiguration),
    DiveSummary(DiveSummary),
    AadAccelFeatures(AadAccelFeatures),
    Hrv(Hrv),
    BeatIntervals(BeatIntervals),
    HrvStatusSummary(HrvStatusSummary),
    HrvValue(HrvValue),
    RawBbi(RawBbi),
    RespirationRate(RespirationRate),
    ChronoShotSession(ChronoShotSession),
    ChronoShotData(ChronoShotData),
    TankUpdate(TankUpdate),
    TankSummary(TankSummary),
    SleepAssessment(SleepAssessment),
}
impl Message {
    #[doc = r" Parse a message from a [`FitDataRecord`][]."]
    pub fn parse(record: FitDataRecord) -> Result<Self, TryFromRecordError> {
        match record.kind() {
            FileId::KIND => FileId::parse(record).map(Self::FileId),
            FileCreator::KIND => FileCreator::parse(record).map(Self::FileCreator),
            TimestampCorrelation::KIND => {
                TimestampCorrelation::parse(record).map(Self::TimestampCorrelation)
            }
            Software::KIND => Software::parse(record).map(Self::Software),
            SlaveDevice::KIND => SlaveDevice::parse(record).map(Self::SlaveDevice),
            Capabilities::KIND => Capabilities::parse(record).map(Self::Capabilities),
            FileCapabilities::KIND => FileCapabilities::parse(record).map(Self::FileCapabilities),
            MesgCapabilities::KIND => MesgCapabilities::parse(record).map(Self::MesgCapabilities),
            FieldCapabilities::KIND => {
                FieldCapabilities::parse(record).map(Self::FieldCapabilities)
            }
            DeviceSettings::KIND => DeviceSettings::parse(record).map(Self::DeviceSettings),
            UserProfile::KIND => UserProfile::parse(record).map(Self::UserProfile),
            HrmProfile::KIND => HrmProfile::parse(record).map(Self::HrmProfile),
            SdmProfile::KIND => SdmProfile::parse(record).map(Self::SdmProfile),
            BikeProfile::KIND => BikeProfile::parse(record).map(Self::BikeProfile),
            Connectivity::KIND => Connectivity::parse(record).map(Self::Connectivity),
            WatchfaceSettings::KIND => {
                WatchfaceSettings::parse(record).map(Self::WatchfaceSettings)
            }
            OhrSettings::KIND => OhrSettings::parse(record).map(Self::OhrSettings),
            TimeInZone::KIND => TimeInZone::parse(record).map(Self::TimeInZone),
            ZonesTarget::KIND => ZonesTarget::parse(record).map(Self::ZonesTarget),
            Sport::KIND => Sport::parse(record).map(Self::Sport),
            HrZone::KIND => HrZone::parse(record).map(Self::HrZone),
            SpeedZone::KIND => SpeedZone::parse(record).map(Self::SpeedZone),
            CadenceZone::KIND => CadenceZone::parse(record).map(Self::CadenceZone),
            PowerZone::KIND => PowerZone::parse(record).map(Self::PowerZone),
            MetZone::KIND => MetZone::parse(record).map(Self::MetZone),
            DiveSettings::KIND => DiveSettings::parse(record).map(Self::DiveSettings),
            DiveAlarm::KIND => DiveAlarm::parse(record).map(Self::DiveAlarm),
            DiveApneaAlarm::KIND => DiveApneaAlarm::parse(record).map(Self::DiveApneaAlarm),
            DiveGas::KIND => DiveGas::parse(record).map(Self::DiveGas),
            Goal::KIND => Goal::parse(record).map(Self::Goal),
            Activity::KIND => Activity::parse(record).map(Self::Activity),
            Session::KIND => Session::parse(record).map(Self::Session),
            Lap::KIND => Lap::parse(record).map(Self::Lap),
            Length::KIND => Length::parse(record).map(Self::Length),
            Record::KIND => Record::parse(record).map(Self::Record),
            Event::KIND => Event::parse(record).map(Self::Event),
            DeviceInfo::KIND => DeviceInfo::parse(record).map(Self::DeviceInfo),
            DeviceAuxBatteryInfo::KIND => {
                DeviceAuxBatteryInfo::parse(record).map(Self::DeviceAuxBatteryInfo)
            }
            TrainingFile::KIND => TrainingFile::parse(record).map(Self::TrainingFile),
            WeatherConditions::KIND => {
                WeatherConditions::parse(record).map(Self::WeatherConditions)
            }
            WeatherAlert::KIND => WeatherAlert::parse(record).map(Self::WeatherAlert),
            GpsMetadata::KIND => GpsMetadata::parse(record).map(Self::GpsMetadata),
            CameraEvent::KIND => CameraEvent::parse(record).map(Self::CameraEvent),
            GyroscopeData::KIND => GyroscopeData::parse(record).map(Self::GyroscopeData),
            AccelerometerData::KIND => {
                AccelerometerData::parse(record).map(Self::AccelerometerData)
            }
            MagnetometerData::KIND => MagnetometerData::parse(record).map(Self::MagnetometerData),
            BarometerData::KIND => BarometerData::parse(record).map(Self::BarometerData),
            ThreeDSensorCalibration::KIND => {
                ThreeDSensorCalibration::parse(record).map(Self::ThreeDSensorCalibration)
            }
            OneDSensorCalibration::KIND => {
                OneDSensorCalibration::parse(record).map(Self::OneDSensorCalibration)
            }
            VideoFrame::KIND => VideoFrame::parse(record).map(Self::VideoFrame),
            ObdiiData::KIND => ObdiiData::parse(record).map(Self::ObdiiData),
            NmeaSentence::KIND => NmeaSentence::parse(record).map(Self::NmeaSentence),
            AviationAttitude::KIND => AviationAttitude::parse(record).map(Self::AviationAttitude),
            Video::KIND => Video::parse(record).map(Self::Video),
            VideoTitle::KIND => VideoTitle::parse(record).map(Self::VideoTitle),
            VideoDescription::KIND => VideoDescription::parse(record).map(Self::VideoDescription),
            VideoClip::KIND => VideoClip::parse(record).map(Self::VideoClip),
            Set::KIND => Set::parse(record).map(Self::Set),
            Jump::KIND => Jump::parse(record).map(Self::Jump),
            Split::KIND => Split::parse(record).map(Self::Split),
            SplitSummary::KIND => SplitSummary::parse(record).map(Self::SplitSummary),
            ClimbPro::KIND => ClimbPro::parse(record).map(Self::ClimbPro),
            FieldDescription::KIND => FieldDescription::parse(record).map(Self::FieldDescription),
            DeveloperDataId::KIND => DeveloperDataId::parse(record).map(Self::DeveloperDataId),
            Course::KIND => Course::parse(record).map(Self::Course),
            CoursePoint::KIND => CoursePoint::parse(record).map(Self::CoursePoint),
            SegmentId::KIND => SegmentId::parse(record).map(Self::SegmentId),
            SegmentLeaderboardEntry::KIND => {
                SegmentLeaderboardEntry::parse(record).map(Self::SegmentLeaderboardEntry)
            }
            SegmentPoint::KIND => SegmentPoint::parse(record).map(Self::SegmentPoint),
            SegmentLap::KIND => SegmentLap::parse(record).map(Self::SegmentLap),
            SegmentFile::KIND => SegmentFile::parse(record).map(Self::SegmentFile),
            Workout::KIND => Workout::parse(record).map(Self::Workout),
            WorkoutSession::KIND => WorkoutSession::parse(record).map(Self::WorkoutSession),
            WorkoutStep::KIND => WorkoutStep::parse(record).map(Self::WorkoutStep),
            ExerciseTitle::KIND => ExerciseTitle::parse(record).map(Self::ExerciseTitle),
            Schedule::KIND => Schedule::parse(record).map(Self::Schedule),
            Totals::KIND => Totals::parse(record).map(Self::Totals),
            WeightScale::KIND => WeightScale::parse(record).map(Self::WeightScale),
            BloodPressure::KIND => BloodPressure::parse(record).map(Self::BloodPressure),
            MonitoringInfo::KIND => MonitoringInfo::parse(record).map(Self::MonitoringInfo),
            Monitoring::KIND => Monitoring::parse(record).map(Self::Monitoring),
            MonitoringHrData::KIND => MonitoringHrData::parse(record).map(Self::MonitoringHrData),
            Spo2Data::KIND => Spo2Data::parse(record).map(Self::Spo2Data),
            Hr::KIND => Hr::parse(record).map(Self::Hr),
            StressLevel::KIND => StressLevel::parse(record).map(Self::StressLevel),
            MaxMetData::KIND => MaxMetData::parse(record).map(Self::MaxMetData),
            HsaBodyBatteryData::KIND => {
                HsaBodyBatteryData::parse(record).map(Self::HsaBodyBatteryData)
            }
            HsaEvent::KIND => HsaEvent::parse(record).map(Self::HsaEvent),
            HsaAccelerometerData::KIND => {
                HsaAccelerometerData::parse(record).map(Self::HsaAccelerometerData)
            }
            HsaGyroscopeData::KIND => HsaGyroscopeData::parse(record).map(Self::HsaGyroscopeData),
            HsaStepData::KIND => HsaStepData::parse(record).map(Self::HsaStepData),
            HsaSpo2Data::KIND => HsaSpo2Data::parse(record).map(Self::HsaSpo2Data),
            HsaStressData::KIND => HsaStressData::parse(record).map(Self::HsaStressData),
            HsaRespirationData::KIND => {
                HsaRespirationData::parse(record).map(Self::HsaRespirationData)
            }
            HsaHeartRateData::KIND => HsaHeartRateData::parse(record).map(Self::HsaHeartRateData),
            HsaConfigurationData::KIND => {
                HsaConfigurationData::parse(record).map(Self::HsaConfigurationData)
            }
            HsaWristTemperatureData::KIND => {
                HsaWristTemperatureData::parse(record).map(Self::HsaWristTemperatureData)
            }
            MemoGlob::KIND => MemoGlob::parse(record).map(Self::MemoGlob),
            SleepLevel::KIND => SleepLevel::parse(record).map(Self::SleepLevel),
            AntChannelId::KIND => AntChannelId::parse(record).map(Self::AntChannelId),
            AntRx::KIND => AntRx::parse(record).map(Self::AntRx),
            AntTx::KIND => AntTx::parse(record).map(Self::AntTx),
            ExdScreenConfiguration::KIND => {
                ExdScreenConfiguration::parse(record).map(Self::ExdScreenConfiguration)
            }
            ExdDataFieldConfiguration::KIND => {
                ExdDataFieldConfiguration::parse(record).map(Self::ExdDataFieldConfiguration)
            }
            ExdDataConceptConfiguration::KIND => {
                ExdDataConceptConfiguration::parse(record).map(Self::ExdDataConceptConfiguration)
            }
            DiveSummary::KIND => DiveSummary::parse(record).map(Self::DiveSummary),
            AadAccelFeatures::KIND => AadAccelFeatures::parse(record).map(Self::AadAccelFeatures),
            Hrv::KIND => Hrv::parse(record).map(Self::Hrv),
            BeatIntervals::KIND => BeatIntervals::parse(record).map(Self::BeatIntervals),
            HrvStatusSummary::KIND => HrvStatusSummary::parse(record).map(Self::HrvStatusSummary),
            HrvValue::KIND => HrvValue::parse(record).map(Self::HrvValue),
            RawBbi::KIND => RawBbi::parse(record).map(Self::RawBbi),
            RespirationRate::KIND => RespirationRate::parse(record).map(Self::RespirationRate),
            ChronoShotSession::KIND => {
                ChronoShotSession::parse(record).map(Self::ChronoShotSession)
            }
            ChronoShotData::KIND => ChronoShotData::parse(record).map(Self::ChronoShotData),
            TankUpdate::KIND => TankUpdate::parse(record).map(Self::TankUpdate),
            TankSummary::KIND => TankSummary::parse(record).map(Self::TankSummary),
            SleepAssessment::KIND => SleepAssessment::parse(record).map(Self::SleepAssessment),
            kind => Err(TryFromRecordError::UnsupportedMessageKind(kind)),
        }
    }
    #[doc = r" Return all fields of the message that are not defined by the profile."]
    pub fn unknown_fields(&self) -> &[FitDataField] {
        match self {
            Self::FileId(message) => &message.unknown_fields,
            Self::FileCreator(message) => &message.unknown_fields,
            Self::TimestampCorrelation(message) => &message.unknown_fields,
            Self::Software(message) => &message.unknown_fields,
            Self::SlaveDevice(message) => &message.unknown_fields,
            Self::Capabilities(message) => &message.unknown_fields,
            Self::FileCapabilities(message) => &message.unknown_fields,
            Self::MesgCapabilities(message) => &message.unknown_fields,
            Self::FieldCapabilities(message) => &message.unknown_fields,
            Self::DeviceSettings(message) => &message.unknown_fields,
            Self::UserProfile(message) => &message.unknown_fields,
            Self::HrmProfile(message) => &message.unknown_fields,
            Self::SdmProfile(message) => &message.unknown_fields,
            Self::BikeProfile(message) => &message.unknown_fields,
            Self::Connectivity(message) => &message.unknown_fields,
            Self::WatchfaceSettings(message) => &message.unknown_fields,
            Self::OhrSettings(message) => &message.unknown_fields,
            Self::TimeInZone(message) => &message.unknown_fields,
            Self::ZonesTarget(message) => &message.unknown_fields,
            Self::Sport(message) => &message.unknown_fields,
            Self::HrZone(message) => &message.unknown_fields,
            Self::SpeedZone(message) => &message.unknown_fields,
            Self::CadenceZone(message) => &message.unknown_fields,
            Self::PowerZone(message) => &message.unknown_fields,
            Self::MetZone(message) => &message.unknown_fields,
            Self::DiveSettings(message) => &message.unknown_fields,
            Self::DiveAlarm(message) => &message.unknown_fields,
            Self::DiveApneaAlarm(message) => &message.unknown_fields,
            Self::DiveGas(message) => &message.unknown_fields,
            Self::Goal(message) => &message.unknown_fields,
            Self::Activity(message) => &message.unknown_fields,
            Self::Session(message) => &message.unknown_fields,
            Self::Lap(message) => &message.unknown_fields,
            Self::Length(message) => &message.unknown_fields,
            Self::Record(message) => &message.unknown_fields,
            Self::Event(message) => &message.unknown_fields,
            Self::DeviceInfo(message) => &message.unknown_fields,
            Self::DeviceAuxBatteryInfo(message) => &message.unknown_fields,
            Self::TrainingFile(message) => &message.unknown_fields,
            Self::WeatherConditions(message) => &message.unknown_fields,
            Self::WeatherAlert(message) => &message.unknown_fields,
            Self::GpsMetadata(message) => &message.unknown_fields,
            Self::CameraEvent(message) => &message.unknown_fields,
            Self::GyroscopeData(message) => &message.unknown_fields,
            Self::AccelerometerData(message) => &message.unknown_fields,
            Self::MagnetometerData(message) => &message.unknown_fields,
            Self::BarometerData(message) => &message.unknown_fields,
            Self::ThreeDSensorCalibration(message) => &message.unknown_fields,
            Self::OneDSensorCalibration(message) => &message.unknown_fields,
            Self::VideoFrame(message) => &message.unknown_fields,
            Self::ObdiiData(message) => &message.unknown_fields,
            Self::NmeaSentence(message) => &message.unknown_fields,
            Self::AviationAttitude(message) => &message.unknown_fields,
            Self::Video(message) => &message.unknown_fields,
            Self::VideoTitle(message) => &message.unknown_fields,
            Self::VideoDescription(message) => &message.unknown_fields,
            Self::VideoClip(message) => &message.unknown_fields,
            Self::Set(message) => &message.unknown_fields,
            Self::Jump(message) => &message.unknown_fields,
            Self::Split(message) => &message.unknown_fields,
            Self::SplitSummary(message) => &message.unknown_fields,
            Self::ClimbPro(message) => &message.unknown_fields,
            Self::FieldDescription(message) => &message.unknown_fields,
            Self::DeveloperDataId(message) => &message.unknown_fields,
            Self::Course(message) => &message.unknown_fields,
            Self::CoursePoint(message) => &message.unknown_fields,
            Self::SegmentId(message) => &message.unknown_fields,
            Self::SegmentLeaderboardEntry(message) => &message.unknown_fields,
            Self::SegmentPoint(message) => &message.unknown_fields,
            Self::SegmentLap(message) => &message.unknown_fields,
            Self::SegmentFile(message) => &message.unknown_fields,
            Self::Workout(message) => &message.unknown_fields,
            Self::WorkoutSession(message) => &message.unknown_fields,
            Self::WorkoutStep(message) => &message.unknown_fields,
            Self::ExerciseTitle(message) => &message.unknown_fields,
            Self::Schedule(message) => &message.unknown_fields,
            Self::Totals(message) => &message.unknown_fields,
            Self::WeightScale(message) => &message.unknown_fields,
            Self::BloodPressure(message) => &message.unknown_fields,
            Self::MonitoringInfo(message) => &message.unknown_fields,
            Self::Monitoring(message) => &message.unknown_fields,
            Self::MonitoringHrData(message) => &message.unknown_fields,
            Self::Spo2Data(message) => &message.unknown_fields,
            Self::Hr(message) => &message.unknown_fields,
            Self::StressLevel(message) => &message.unknown_fields,
            Self::MaxMetData(message) => &message.unknown_fields,
            Self::HsaBodyBatteryData(message) => &message.unknown_fields,
            Self::HsaEvent(message) => &message.unknown_fields,
            Self::HsaAccelerometerData(message) => &message.unknown_fields,
            Self::HsaGyroscopeData(message) => &message.unknown_fields,
            Self::HsaStepData(message) => &message.unknown_fields,
            Self::HsaSpo2Data(message) => &message.unknown_fields,
            Self::HsaStressData(message) => &message.unknown_fields,
            Self::HsaRespirationData(message) => &message.unknown_fields,
            Self::HsaHeartRateData(message) => &message.unknown_fields,
            Self::HsaConfigurationData(message) => &message.unknown_fields,
            Self::HsaWristTemperatureData(message) => &message.unknown_fields,
            Self::MemoGlob(message) => &message.unknown_fields,
            Self::SleepLevel(message) => &message.unknown_fields,
            Self::AntChannelId(message) => &message.unknown_fields,
            Self::AntRx(message) => &message.unknown_fields,
            Self::AntTx(message) => &message.unknown_fields,
            Self::ExdScreenConfiguration(message) => &message.unknown_fields,
            Self::ExdDataFieldConfiguration(message) => &message.unknown_fields,
            Self::ExdDataConceptConfiguration(message) => &message.unknown_fields,
            Self::DiveSummary(message) => &message.unknown_fields,
            Self::AadAccelFeatures(message) => &message.unknown_fields,
            Self::Hrv(message) => &message.unknown_fields,
            Self::BeatIntervals(message) => &message.unknown_fields,
            Self::HrvStatusSummary(message) => &message.unknown_fields,
            Self::HrvValue(message) => &message.unknown_fields,
            Self::RawBbi(message) => &message.unknown_fields,
            Self::RespirationRate(message) => &message.unknown_fields,
            Self::ChronoShotSession(message) => &message.unknown_fields,
            Self::ChronoShotData(message) => &message.unknown_fields,
            Self::TankUpdate(message) => &message.unknown_fields,
            Self::TankSummary(message) => &message.unknown_fields,
            Self::SleepAssessment(message) => &message.unknown_fields,
        }
    }
}
impl TryFrom<FitDataRecord> for Message {
    type Error = TryFromRecordError;
    fn try_from(record: FitDataRecord) -> Result<Self, Self::Error> {
        Self::parse(record)
    }
}
#[doc = "Must be first message in file."]
#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize)]
pub struct FileId {
    pub r#type: Option<Value>,
    pub r#manufacturer: Option<Value>,
    pub r#product: Option<Value>,
    pub r#serial_number: Option<Value>,
    pub r#time_created: Option<Value>,
    pub r#number: Option<Value>,
    pub r#product_name: Option<Value>,
    #[doc = r" All fields that are not defined in the profile."]
    pub unknown_fields: Vec<FitDataField>,
}
impl FitMessage for FileId {
    const NAME: &'static str = "FileId";
    const KIND: MesgNum = MesgNum::FileId;
    fn parse(record: FitDataRecord) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#type = None;
        let mut r#manufacturer = None;
        let mut r#product = None;
        let mut r#serial_number = None;
        let mut r#time_created = None;
        let mut r#number = None;
        let mut r#product_name = None;
        let mut unknown_fields = Vec::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => {
                    r#type = Some(field.into_value());
                }
                1u8 => {
                    r#manufacturer = Some(field.into_value());
                }
                2u8 => {
                    r#product = Some(field.into_value());
                }
                3u8 => {
                    r#serial_number = Some(field.into_value());
                }
                4u8 => {
                    r#time_created = Some(field.into_value());
                }
                5u8 => {
                    r#number = Some(field.into_value());
                }
                8u8 => {
                    r#product_name = Some(field.into_value());
                }
                _ => {
                    unknown_fields.push(field);
                }
            }
        }
        Ok(Self {
            r#type,
            r#manufacturer,
            r#product,
            r#serial_number,
            r#time_created,
            r#number,
            r#product_name,
            unknown_fields,
        })
    }
}
impl TryFrom<FitDataRecord> for FileId {
    type Error = TryFromRecordError;
    fn try_from(record: FitDataRecord) -> Result<Self, Self::Error> {
        Self::parse(record)
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize)]
pub struct FileCreator {
    pub r#software_version: Option<Value>,
    pub r#hardware_version: Option<Value>,
    #[doc = r" All fields that are not defined in the profile."]
    pub unknown_fields: Vec<FitDataField>,
}
impl FitMessage for FileCreator {
    const NAME: &'static str = "FileCreator";
    const KIND: MesgNum = MesgNum::FileCreator;
    fn parse(record: FitDataRecord) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#software_version = None;
        let mut r#hardware_version = None;
        let mut unknown_fields = Vec::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => {
                    r#software_version = Some(field.into_value());
                }
                1u8 => {
                    r#hardware_version = Some(field.into_value());
                }
                _ => {
                    unknown_fields.push(field);
                }
            }
        }
        Ok(Self {
            r#software_version,
            r#hardware_version,
            unknown_fields,
        })
    }
}
impl TryFrom<FitDataRecord> for FileCreator {
    type Error = TryFromRecordError;
    fn try_from(record: FitDataRecord) -> Result<Self, Self::Error> {
        Self::parse(record)
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize)]
pub struct TimestampCorrelation {
    pub r#fractional_timestamp: Option<Value>,
    pub r#system_timestamp: Option<Value>,
    pub r#fractional_system_timestamp: Option<Value>,
    pub r#local_timestamp: Option<Value>,
    pub r#timestamp_ms: Option<Value>,
    pub r#system_timestamp_ms: Option<Value>,
    pub r#timestamp: Option<Value>,
    #[doc = r" All fields that are not defined in the profile."]
    pub unknown_fields: Vec<FitDataField>,
}
impl FitMessage for TimestampCorrelation {
    const NAME: &'static str = "TimestampCorrelation";
    const KIND: MesgNum = MesgNum::TimestampCorrelation;
    fn parse(record: FitDataRecord) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#fractional_timestamp = None;
        let mut r#system_timestamp = None;
        let mut r#fractional_system_timestamp = None;
        let mut r#local_timestamp = None;
        let mut r#timestamp_ms = None;
        let mut r#system_timestamp_ms = None;
        let mut r#timestamp = None;
        let mut unknown_fields = Vec::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => {
                    r#fractional_timestamp = Some(field.into_value());
                }
                1u8 => {
                    r#system_timestamp = Some(field.into_value());
                }
                2u8 => {
                    r#fractional_system_timestamp = Some(field.into_value());
                }
                3u8 => {
                    r#local_timestamp = Some(field.into_value());
                }
                4u8 => {
                    r#timestamp_ms = Some(field.into_value());
                }
                5u8 => {
                    r#system_timestamp_ms = Some(field.into_value());
                }
                253u8 => {
                    r#timestamp = Some(field.into_value());
                }
                _ => {
                    unknown_fields.push(field);
                }
            }
        }
        Ok(Self {
            r#fractional_timestamp,
            r#system_timestamp,
            r#fractional_system_timestamp,
            r#local_timestamp,
            r#timestamp_ms,
            r#system_timestamp_ms,
            r#timestamp,
            unknown_fields,
        })
    }
}
impl TryFrom<FitDataRecord> for TimestampCorrelation {
    type Error = TryFromRecordError;
    fn try_from(record: FitDataRecord) -> Result<Self, Self::Error> {
        Self::parse(record)
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize)]
pub struct Software {
    pub r#version: Option<Value>,
    pub r#part_number: Option<Value>,
    pub r#message_index: Option<Value>,
    #[doc = r" All fields that are not defined in the profile."]
    pub unknown_fields: Vec<FitDataField>,
}
impl FitMessage for Software {
    const NAME: &'static str = "Software";
    const KIND: MesgNum = MesgNum::Software;
    fn parse(record: FitDataRecord) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#version = None;
        let mut r#part_number = None;
        let mut r#message_index = None;
        let mut unknown_fields = Vec::new();
        for field in record.into_vec() {
            match field.number() {
                3u8 => {
                    r#version = Some(field.into_value());
                }
                5u8 => {
                    r#part_number = Some(field.into_value());
                }
                254u8 => {
                    r#message_index = Some(field.into_value());
                }
                _ => {
                    unknown_fields.push(field);
                }
            }
        }
        Ok(Self {
            r#version,
            r#part_number,
            r#message_index,
            unknown_fields,
        })
    }
}
impl TryFrom<FitDataRecord> for Software {
    type Error = TryFromRecordError;
    fn try_from(record: FitDataRecord) -> Result<Self, Self::Error> {
        Self::parse(record)
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize)]
pub struct SlaveDevice {
    pub r#manufacturer: Option<Value>,
    pub r#product: Option<Value>,
    #[doc = r" All fields that are not defined in the profile."]
    pub unknown_fields: Vec<FitDataField>,
}
impl FitMessage for SlaveDevice {
    const NAME: &'static str = "SlaveDevice";
    const KIND: MesgNum = MesgNum::SlaveDevice;
    fn parse(record: FitDataRecord) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#manufacturer = None;
        let mut r#product = None;
        let mut unknown_fields = Vec::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => {
                    r#manufacturer = Some(field.into_value());
                }
                1u8 => {
                    r#product = Some(field.into_value());
                }
                _ => {
                    unknown_fields.push(field);
                }
            }
        }
        Ok(Self {
            r#manufacturer,
            r#product,
            unknown_fields,
        })
    }
}
impl TryFrom<FitDataRecord> for SlaveDevice {
    type Error = TryFromRecordError;
    fn try_from(record: FitDataRecord) -> Result<Self, Self::Error> {
        Self::parse(record)
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize)]
pub struct Capabilities {
    pub r#languages: Option<Value>,
    pub r#sports: Option<Value>,
    pub r#workouts_supported: Option<Value>,
    pub r#connectivity_supported: Option<Value>,
    #[doc = r" All fields that are not defined in the profile."]
    pub unknown_fields: Vec<FitDataField>,
}
impl FitMessage for Capabilities {
    const NAME: &'static str = "Capabilities";
    const KIND: MesgNum = MesgNum::Capabilities;
    fn parse(record: FitDataRecord) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#languages = None;
        let mut r#sports = None;
        let mut r#workouts_supported = None;
        let mut r#connectivity_supported = None;
        let mut unknown_fields = Vec::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => {
                    r#languages = Some(field.into_value());
                }
                1u8 => {
                    r#sports = Some(field.into_value());
                }
                21u8 => {
                    r#workouts_supported = Some(field.into_value());
                }
                23u8 => {
                    r#connectivity_supported = Some(field.into_value());
                }
                _ => {
                    unknown_fields.push(field);
                }
            }
        }
        Ok(Self {
            r#languages,
            r#sports,
            r#workouts_supported,
            r#connectivity_supported,
            unknown_fields,
        })
    }
}
impl TryFrom<FitDataRecord> for Capabilities {
    type Error = TryFromRecordError;
    fn try_from(record: FitDataRecord) -> Result<Self, Self::Error> {
        Self::parse(record)
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize)]
pub struct FileCapabilities {
    pub r#type: Option<Value>,
    pub r#flags: Option<Value>,
    pub r#directory: Option<Value>,
    pub r#max_count: Option<Value>,
    pub r#max_size: Option<Value>,
    pub r#message_index: Option<Value>,
    #[doc = r" All fields that are not defined in the profile."]
    pub unknown_fields: Vec<FitDataField>,
}
impl FitMessage for FileCapabilities {
    const NAME: &'static str = "FileCapabilities";
    const KIND: MesgNum = MesgNum::FileCapabilities;
    fn parse(record: FitDataRecord) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#type = None;
        let mut r#flags = None;
        let mut r#directory = None;
        let mut r#max_count = None;
        let mut r#max_size = None;
        let mut r#message_index = None;
        let mut unknown_fields = Vec::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => {
                    r#type = Some(field.into_value());
                }
                1u8 => {
                    r#flags = Some(field.into_value());
                }
                2u8 => {
                    r#directory = Some(field.into_value());
                }
                3u8 => {
                    r#max_count = Some(field.into_value());
                }
                4u8 => {
                    r#max_size = Some(field.into_value());
                }
                254u8 => {
                    r#message_index = Some(field.into_value());
                }
                _ => {
                    unknown_fields.push(field);
                }
            }
        }
        Ok(Self {
            r#type,
            r#flags,
            r#directory,
            r#max_count,
            r#max_size,
            r#message_index,
            unknown_fields,
        })
    }
}
impl TryFrom<FitDataRecord> for FileCapabilities {
    type Error = TryFromRecordError;
    fn try_from(record: FitDataRecord) -> Result<Self, Self::Error> {
        Self::parse(record)
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize)]
pub struct MesgCapabilities {
    pub r#file: Option<Value>,
    pub r#mesg_num: Option<Value>,
    pub r#count_type: Option<Value>,
    pub r#count: Option<Value>,
    pub r#message_index: Option<Value>,
    #[doc = r" All fields that are not defined in the profile."]
    pub unknown_fields: Vec<FitDataField>,
}
impl FitMessage for MesgCapabilities {
    const NAME: &'static str = "MesgCapabilities";
    const KIND: MesgNum = MesgNum::MesgCapabilities;
    fn parse(record: FitDataRecord) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#file = None;
        let mut r#mesg_num = None;
        let mut r#count_type = None;
        let mut r#count = None;
        let mut r#message_index = None;
        let mut unknown_fields = Vec::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => {
                    r#file = Some(field.into_value());
                }
                1u8 => {
                    r#mesg_num = Some(field.into_value());
                }
                2u8 => {
                    r#count_type = Some(field.into_value());
                }
                3u8 => {
                    r#count = Some(field.into_value());
                }
                254u8 => {
                    r#message_index = Some(field.into_value());
                }
                _ => {
                    unknown_fields.push(field);
                }
            }
        }
        Ok(Self {
            r#file,
            r#mesg_num,
            r#count_type,
            r#count,
            r#message_index,
            unknown_fields,
        })
    }
}
impl TryFrom<FitDataRecord> for MesgCapabilities {
    type Error = TryFromRecordError;
    fn try_from(record: FitDataRecord) -> Result<Self, Self::Error> {
        Self::parse(record)
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize)]
pub struct FieldCapabilities {
    pub r#file: Option<Value>,
    pub r#mesg_num: Option<Value>,
    pub r#field_num: Option<Value>,
    pub r#count: Option<Value>,
    pub r#message_index: Option<Value>,
    #[doc = r" All fields that are not defined in the profile."]
    pub unknown_fields: Vec<FitDataField>,
}
impl FitMessage for FieldCapabilities {
    const NAME: &'static str = "FieldCapabilities";
    const KIND: MesgNum = MesgNum::FieldCapabilities;
    fn parse(record: FitDataRecord) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#file = None;
        let mut r#mesg_num = None;
        let mut r#field_num = None;
        let mut r#count = None;
        let mut r#message_index = None;
        let mut unknown_fields = Vec::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => {
                    r#file = Some(field.into_value());
                }
                1u8 => {
                    r#mesg_num = Some(field.into_value());
                }
                2u8 => {
                    r#field_num = Some(field.into_value());
                }
                3u8 => {
                    r#count = Some(field.into_value());
                }
                254u8 => {
                    r#message_index = Some(field.into_value());
                }
                _ => {
                    unknown_fields.push(field);
                }
            }
        }
        Ok(Self {
            r#file,
            r#mesg_num,
            r#field_num,
            r#count,
            r#message_index,
            unknown_fields,
        })
    }
}
impl TryFrom<FitDataRecord> for FieldCapabilities {
    type Error = TryFromRecordError;
    fn try_from(record: FitDataRecord) -> Result<Self, Self::Error> {
        Self::parse(record)
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize)]
pub struct DeviceSettings {
    pub r#active_time_zone: Option<Value>,
    pub r#utc_offset: Option<Value>,
    pub r#time_offset: Option<Value>,
    pub r#time_mode: Option<Value>,
    pub r#time_zone_offset: Option<Value>,
    pub r#backlight_mode: Option<Value>,
    pub r#activity_tracker_enabled: Option<Value>,
    pub r#clock_time: Option<Value>,
    pub r#pages_enabled: Option<Value>,
    pub r#move_alert_enabled: Option<Value>,
    pub r#date_mode: Option<Value>,
    pub r#display_orientation: Option<Value>,
    pub r#mounting_side: Option<Value>,
    pub r#default_page: Option<Value>,
    pub r#autosync_min_steps: Option<Value>,
    pub r#autosync_min_time: Option<Value>,
    pub r#lactate_threshold_autodetect_enabled: Option<Value>,
    pub r#ble_auto_upload_enabled: Option<Value>,
    pub r#auto_sync_frequency: Option<Value>,
    pub r#auto_activity_detect: Option<Value>,
    pub r#number_of_screens: Option<Value>,
    pub r#smart_notification_display_orientation: Option<Value>,
    pub r#tap_interface: Option<Value>,
    pub r#tap_sensitivity: Option<Value>,
    #[doc = r" All fields that are not defined in the profile."]
    pub unknown_fields: Vec<FitDataField>,
}
impl FitMessage for DeviceSettings {
    const NAME: &'static str = "DeviceSettings";
    const KIND: MesgNum = MesgNum::DeviceSettings;
    fn parse(record: FitDataRecord) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#active_time_zone = None;
        let mut r#utc_offset = None;
        let mut r#time_offset = None;
        let mut r#time_mode = None;
        let mut r#time_zone_offset = None;
        let mut r#backlight_mode = None;
        let mut r#activity_tracker_enabled = None;
        let mut r#clock_time = None;
        let mut r#pages_enabled = None;
        let mut r#move_alert_enabled = None;
        let mut r#date_mode = None;
        let mut r#display_orientation = None;
        let mut r#mounting_side = None;
        let mut r#default_page = None;
        let mut r#autosync_min_steps = None;
        let mut r#autosync_min_time = None;
        let mut r#lactate_threshold_autodetect_enabled = None;
        let mut r#ble_auto_upload_enabled = None;
        let mut r#auto_sync_frequency = None;
        let mut r#auto_activity_detect = None;
        let mut r#number_of_screens = None;
        let mut r#smart_notification_display_orientation = None;
        let mut r#tap_interface = None;
        let mut r#tap_sensitivity = None;
        let mut unknown_fields = Vec::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => {
                    r#active_time_zone = Some(field.into_value());
                }
                1u8 => {
                    r#utc_offset = Some(field.into_value());
                }
                2u8 => {
                    r#time_offset = Some(field.into_value());
                }
                4u8 => {
                    r#time_mode = Some(field.into_value());
                }
                5u8 => {
                    r#time_zone_offset = Some(field.into_value());
                }
                12u8 => {
                    r#backlight_mode = Some(field.into_value());
                }
                36u8 => {
                    r#activity_tracker_enabled = Some(field.into_value());
                }
                39u8 => {
                    r#clock_time = Some(field.into_value());
                }
                40u8 => {
                    r#pages_enabled = Some(field.into_value());
                }
                46u8 => {
                    r#move_alert_enabled = Some(field.into_value());
                }
                47u8 => {
                    r#date_mode = Some(field.into_value());
                }
                55u8 => {
                    r#display_orientation = Some(field.into_value());
                }
                56u8 => {
                    r#mounting_side = Some(field.into_value());
                }
                57u8 => {
                    r#default_page = Some(field.into_value());
                }
                58u8 => {
                    r#autosync_min_steps = Some(field.into_value());
                }
                59u8 => {
                    r#autosync_min_time = Some(field.into_value());
                }
                80u8 => {
                    r#lactate_threshold_autodetect_enabled = Some(field.into_value());
                }
                86u8 => {
                    r#ble_auto_upload_enabled = Some(field.into_value());
                }
                89u8 => {
                    r#auto_sync_frequency = Some(field.into_value());
                }
                90u8 => {
                    r#auto_activity_detect = Some(field.into_value());
                }
                94u8 => {
                    r#number_of_screens = Some(field.into_value());
                }
                95u8 => {
                    r#smart_notification_display_orientation = Some(field.into_value());
                }
                134u8 => {
                    r#tap_interface = Some(field.into_value());
                }
                174u8 => {
                    r#tap_sensitivity = Some(field.into_value());
                }
                _ => {
                    unknown_fields.push(field);
                }
            }
        }
        Ok(Self {
            r#active_time_zone,
            r#utc_offset,
            r#time_offset,
            r#time_mode,
            r#time_zone_offset,
            r#backlight_mode,
            r#activity_tracker_enabled,
            r#clock_time,
            r#pages_enabled,
            r#move_alert_enabled,
            r#date_mode,
            r#display_orientation,
            r#mounting_side,
            r#default_page,
            r#autosync_min_steps,
            r#autosync_min_time,
            r#lactate_threshold_autodetect_enabled,
            r#ble_auto_upload_enabled,
            r#auto_sync_frequency,
            r#auto_activity_detect,
            r#number_of_screens,
            r#smart_notification_display_orientation,
            r#tap_interface,
            r#tap_sensitivity,
            unknown_fields,
        })
    }
}
impl TryFrom<FitDataRecord> for DeviceSettings {
    type Error = TryFromRecordError;
    fn try_from(record: FitDataRecord) -> Result<Self, Self::Error> {
        Self::parse(record)
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize)]
pub struct UserProfile {
    pub r#friendly_name: Option<Value>,
    pub r#gender: Option<Value>,
    pub r#age: Option<Value>,
    pub r#height: Option<Value>,
    pub r#weight: Option<Value>,
    pub r#language: Option<Value>,
    pub r#elev_setting: Option<Value>,
    pub r#weight_setting: Option<Value>,
    pub r#resting_heart_rate: Option<Value>,
    pub r#default_max_running_heart_rate: Option<Value>,
    pub r#default_max_biking_heart_rate: Option<Value>,
    pub r#default_max_heart_rate: Option<Value>,
    pub r#hr_setting: Option<Value>,
    pub r#speed_setting: Option<Value>,
    pub r#dist_setting: Option<Value>,
    pub r#power_setting: Option<Value>,
    pub r#activity_class: Option<Value>,
    pub r#position_setting: Option<Value>,
    pub r#temperature_setting: Option<Value>,
    pub r#local_id: Option<Value>,
    pub r#global_id: Option<Value>,
    pub r#wake_time: Option<Value>,
    pub r#sleep_time: Option<Value>,
    pub r#height_setting: Option<Value>,
    pub r#user_running_step_length: Option<Value>,
    pub r#user_walking_step_length: Option<Value>,
    pub r#depth_setting: Option<Value>,
    pub r#dive_count: Option<Value>,
    pub r#message_index: Option<Value>,
    #[doc = r" All fields that are not defined in the profile."]
    pub unknown_fields: Vec<FitDataField>,
}
impl FitMessage for UserProfile {
    const NAME: &'static str = "UserProfile";
    const KIND: MesgNum = MesgNum::UserProfile;
    fn parse(record: FitDataRecord) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#friendly_name = None;
        let mut r#gender = None;
        let mut r#age = None;
        let mut r#height = None;
        let mut r#weight = None;
        let mut r#language = None;
        let mut r#elev_setting = None;
        let mut r#weight_setting = None;
        let mut r#resting_heart_rate = None;
        let mut r#default_max_running_heart_rate = None;
        let mut r#default_max_biking_heart_rate = None;
        let mut r#default_max_heart_rate = None;
        let mut r#hr_setting = None;
        let mut r#speed_setting = None;
        let mut r#dist_setting = None;
        let mut r#power_setting = None;
        let mut r#activity_class = None;
        let mut r#position_setting = None;
        let mut r#temperature_setting = None;
        let mut r#local_id = None;
        let mut r#global_id = None;
        let mut r#wake_time = None;
        let mut r#sleep_time = None;
        let mut r#height_setting = None;
        let mut r#user_running_step_length = None;
        let mut r#user_walking_step_length = None;
        let mut r#depth_setting = None;
        let mut r#dive_count = None;
        let mut r#message_index = None;
        let mut unknown_fields = Vec::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => {
                    r#friendly_name = Some(field.into_value());
                }
                1u8 => {
                    r#gender = Some(field.into_value());
                }
                2u8 => {
                    r#age = Some(field.into_value());
                }
                3u8 => {
                    r#height = Some(field.into_value());
                }
                4u8 => {
                    r#weight = Some(field.into_value());
                }
                5u8 => {
                    r#language = Some(field.into_value());
                }
                6u8 => {
                    r#elev_setting = Some(field.into_value());
                }
                7u8 => {
                    r#weight_setting = Some(field.into_value());
                }
                8u8 => {
                    r#resting_heart_rate = Some(field.into_value());
                }
                9u8 => {
                    r#default_max_running_heart_rate = Some(field.into_value());
                }
                10u8 => {
                    r#default_max_biking_heart_rate = Some(field.into_value());
                }
                11u8 => {
                    r#default_max_heart_rate = Some(field.into_value());
                }
                12u8 => {
                    r#hr_setting = Some(field.into_value());
                }
                13u8 => {
                    r#speed_setting = Some(field.into_value());
                }
                14u8 => {
                    r#dist_setting = Some(field.into_value());
                }
                16u8 => {
                    r#power_setting = Some(field.into_value());
                }
                17u8 => {
                    r#activity_class = Some(field.into_value());
                }
                18u8 => {
                    r#position_setting = Some(field.into_value());
                }
                21u8 => {
                    r#temperature_setting = Some(field.into_value());
                }
                22u8 => {
                    r#local_id = Some(field.into_value());
                }
                23u8 => {
                    r#global_id = Some(field.into_value());
                }
                28u8 => {
                    r#wake_time = Some(field.into_value());
                }
                29u8 => {
                    r#sleep_time = Some(field.into_value());
                }
                30u8 => {
                    r#height_setting = Some(field.into_value());
                }
                31u8 => {
                    r#user_running_step_length = Some(field.into_value());
                }
                32u8 => {
                    r#user_walking_step_length = Some(field.into_value());
                }
                47u8 => {
                    r#depth_setting = Some(field.into_value());
                }
                49u8 => {
                    r#dive_count = Some(field.into_value());
                }
                254u8 => {
                    r#message_index = Some(field.into_value());
                }
                _ => {
                    unknown_fields.push(field);
                }
            }
        }
        Ok(Self {
            r#friendly_name,
            r#gender,
            r#age,
            r#height,
            r#weight,
            r#language,
            r#elev_setting,
            r#weight_setting,
            r#resting_heart_rate,
            r#default_max_running_heart_rate,
            r#default_max_biking_heart_rate,
            r#default_max_heart_rate,
            r#hr_setting,
            r#speed_setting,
            r#dist_setting,
            r#power_setting,
            r#activity_class,
            r#position_setting,
            r#temperature_setting,
            r#local_id,
            r#global_id,
            r#wake_time,
            r#sleep_time,
            r#height_setting,
            r#user_running_step_length,
            r#user_walking_step_length,
            r#depth_setting,
            r#dive_count,
            r#message_index,
            unknown_fields,
        })
    }
}
impl TryFrom<FitDataRecord> for UserProfile {
    type Error = TryFromRecordError;
    fn try_from(record: FitDataRecord) -> Result<Self, Self::Error> {
        Self::parse(record)
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize)]
pub struct HrmProfile {
    pub r#enabled: Option<Value>,
    pub r#hrm_ant_id: Option<Value>,
    pub r#log_hrv: Option<Value>,
    pub r#hrm_ant_id_trans_type: Option<Value>,
    pub r#message_index: Option<Value>,
    #[doc = r" All fields that are not defined in the profile."]
    pub unknown_fields: Vec<FitDataField>,
}
impl FitMessage for HrmProfile {
    const NAME: &'static str = "HrmProfile";
    const KIND: MesgNum = MesgNum::HrmProfile;
    fn parse(record: FitDataRecord) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#enabled = None;
        let mut r#hrm_ant_id = None;
        let mut r#log_hrv = None;
        let mut r#hrm_ant_id_trans_type = None;
        let mut r#message_index = None;
        let mut unknown_fields = Vec::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => {
                    r#enabled = Some(field.into_value());
                }
                1u8 => {
                    r#hrm_ant_id = Some(field.into_value());
                }
                2u8 => {
                    r#log_hrv = Some(field.into_value());
                }
                3u8 => {
                    r#hrm_ant_id_trans_type = Some(field.into_value());
                }
                254u8 => {
                    r#message_index = Some(field.into_value());
                }
                _ => {
                    unknown_fields.push(field);
                }
            }
        }
        Ok(Self {
            r#enabled,
            r#hrm_ant_id,
            r#log_hrv,
            r#hrm_ant_id_trans_type,
            r#message_index,
            unknown_fields,
        })
    }
}
impl TryFrom<FitDataRecord> for HrmProfile {
    type Error = TryFromRecordError;
    fn try_from(record: FitDataRecord) -> Result<Self, Self::Error> {
        Self::parse(record)
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize)]
pub struct SdmProfile {
    pub r#enabled: Option<Value>,
    pub r#sdm_ant_id: Option<Value>,
    pub r#sdm_cal_factor: Option<Value>,
    pub r#odometer: Option<Value>,
    pub r#speed_source: Option<Value>,
    pub r#sdm_ant_id_trans_type: Option<Value>,
    pub r#odometer_rollover: Option<Value>,
    pub r#message_index: Option<Value>,
    #[doc = r" All fields that are not defined in the profile."]
    pub unknown_fields: Vec<FitDataField>,
}
impl FitMessage for SdmProfile {
    const NAME: &'static str = "SdmProfile";
    const KIND: MesgNum = MesgNum::SdmProfile;
    fn parse(record: FitDataRecord) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#enabled = None;
        let mut r#sdm_ant_id = None;
        let mut r#sdm_cal_factor = None;
        let mut r#odometer = None;
        let mut r#speed_source = None;
        let mut r#sdm_ant_id_trans_type = None;
        let mut r#odometer_rollover = None;
        let mut r#message_index = None;
        let mut unknown_fields = Vec::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => {
                    r#enabled = Some(field.into_value());
                }
                1u8 => {
                    r#sdm_ant_id = Some(field.into_value());
                }
                2u8 => {
                    r#sdm_cal_factor = Some(field.into_value());
                }
                3u8 => {
                    r#odometer = Some(field.into_value());
                }
                4u8 => {
                    r#speed_source = Some(field.into_value());
                }
                5u8 => {
                    r#sdm_ant_id_trans_type = Some(field.into_value());
                }
                7u8 => {
                    r#odometer_rollover = Some(field.into_value());
                }
                254u8 => {
                    r#message_index = Some(field.into_value());
                }
                _ => {
                    unknown_fields.push(field);
                }
            }
        }
        Ok(Self {
            r#enabled,
            r#sdm_ant_id,
            r#sdm_cal_factor,
            r#odometer,
            r#speed_source,
            r#sdm_ant_id_trans_type,
            r#odometer_rollover,
            r#message_index,
            unknown_fields,
        })
    }
}
impl TryFrom<FitDataRecord> for SdmProfile {
    type Error = TryFromRecordError;
    fn try_from(record: FitDataRecord) -> Result<Self, Self::Error> {
        Self::parse(record)
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize)]
pub struct BikeProfile {
    pub r#name: Option<Value>,
    pub r#sport: Option<Value>,
    pub r#sub_sport: Option<Value>,
    pub r#odometer: Option<Value>,
    pub r#bike_spd_ant_id: Option<Value>,
    pub r#bike_cad_ant_id: Option<Value>,
    pub r#bike_spdcad_ant_id: Option<Value>,
    pub r#bike_power_ant_id: Option<Value>,
    pub r#custom_wheelsize: Option<Value>,
    pub r#auto_wheelsize: Option<Value>,
    pub r#bike_weight: Option<Value>,
    pub r#power_cal_factor: Option<Value>,
    pub r#auto_wheel_cal: Option<Value>,
    pub r#auto_power_zero: Option<Value>,
    pub r#id: Option<Value>,
    pub r#spd_enabled: Option<Value>,
    pub r#cad_enabled: Option<Value>,
    pub r#spdcad_enabled: Option<Value>,
    pub r#power_enabled: Option<Value>,
    pub r#crank_length: Option<Value>,
    pub r#enabled: Option<Value>,
    pub r#bike_spd_ant_id_trans_type: Option<Value>,
    pub r#bike_cad_ant_id_trans_type: Option<Value>,
    pub r#bike_spdcad_ant_id_trans_type: Option<Value>,
    pub r#bike_power_ant_id_trans_type: Option<Value>,
    pub r#odometer_rollover: Option<Value>,
    pub r#front_gear_num: Option<Value>,
    pub r#front_gear: Option<Value>,
    pub r#rear_gear_num: Option<Value>,
    pub r#rear_gear: Option<Value>,
    pub r#shimano_di2_enabled: Option<Value>,
    pub r#message_index: Option<Value>,
    #[doc = r" All fields that are not defined in the profile."]
    pub unknown_fields: Vec<FitDataField>,
}
impl FitMessage for BikeProfile {
    const NAME: &'static str = "BikeProfile";
    const KIND: MesgNum = MesgNum::BikeProfile;
    fn parse(record: FitDataRecord) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#name = None;
        let mut r#sport = None;
        let mut r#sub_sport = None;
        let mut r#odometer = None;
        let mut r#bike_spd_ant_id = None;
        let mut r#bike_cad_ant_id = None;
        let mut r#bike_spdcad_ant_id = None;
        let mut r#bike_power_ant_id = None;
        let mut r#custom_wheelsize = None;
        let mut r#auto_wheelsize = None;
        let mut r#bike_weight = None;
        let mut r#power_cal_factor = None;
        let mut r#auto_wheel_cal = None;
        let mut r#auto_power_zero = None;
        let mut r#id = None;
        let mut r#spd_enabled = None;
        let mut r#cad_enabled = None;
        let mut r#spdcad_enabled = None;
        let mut r#power_enabled = None;
        let mut r#crank_length = None;
        let mut r#enabled = None;
        let mut r#bike_spd_ant_id_trans_type = None;
        let mut r#bike_cad_ant_id_trans_type = None;
        let mut r#bike_spdcad_ant_id_trans_type = None;
        let mut r#bike_power_ant_id_trans_type = None;
        let mut r#odometer_rollover = None;
        let mut r#front_gear_num = None;
        let mut r#front_gear = None;
        let mut r#rear_gear_num = None;
        let mut r#rear_gear = None;
        let mut r#shimano_di2_enabled = None;
        let mut r#message_index = None;
        let mut unknown_fields = Vec::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => {
                    r#name = Some(field.into_value());
                }
                1u8 => {
                    r#sport = Some(field.into_value());
                }
                2u8 => {
                    r#sub_sport = Some(field.into_value());
                }
                3u8 => {
                    r#odometer = Some(field.into_value());
                }
                4u8 => {
                    r#bike_spd_ant_id = Some(field.into_value());
                }
                5u8 => {
                    r#bike_cad_ant_id = Some(field.into_value());
                }
                6u8 => {
                    r#bike_spdcad_ant_id = Some(field.into_value());
                }
                7u8 => {
                    r#bike_power_ant_id = Some(field.into_value());
                }
                8u8 => {
                    r#custom_wheelsize = Some(field.into_value());
                }
                9u8 => {
                    r#auto_wheelsize = Some(field.into_value());
                }
                10u8 => {
                    r#bike_weight = Some(field.into_value());
                }
                11u8 => {
                    r#power_cal_factor = Some(field.into_value());
                }
                12u8 => {
                    r#auto_wheel_cal = Some(field.into_value());
                }
                13u8 => {
                    r#auto_power_zero = Some(field.into_value());
                }
                14u8 => {
                    r#id = Some(field.into_value());
                }
                15u8 => {
                    r#spd_enabled = Some(field.into_value());
                }
                16u8 => {
                    r#cad_enabled = Some(field.into_value());
                }
                17u8 => {
                    r#spdcad_enabled = Some(field.into_value());
                }
                18u8 => {
                    r#power_enabled = Some(field.into_value());
                }
                19u8 => {
                    r#crank_length = Some(field.into_value());
                }
                20u8 => {
                    r#enabled = Some(field.into_value());
                }
                21u8 => {
                    r#bike_spd_ant_id_trans_type = Some(field.into_value());
                }
                22u8 => {
                    r#bike_cad_ant_id_trans_type = Some(field.into_value());
                }
                23u8 => {
                    r#bike_spdcad_ant_id_trans_type = Some(field.into_value());
                }
                24u8 => {
                    r#bike_power_ant_id_trans_type = Some(field.into_value());
                }
                37u8 => {
                    r#odometer_rollover = Some(field.into_value());
                }
                38u8 => {
                    r#front_gear_num = Some(field.into_value());
                }
                39u8 => {
                    r#front_gear = Some(field.into_value());
                }
                40u8 => {
                    r#rear_gear_num = Some(field.into_value());
                }
                41u8 => {
                    r#rear_gear = Some(field.into_value());
                }
                44u8 => {
                    r#shimano_di2_enabled = Some(field.into_value());
                }
                254u8 => {
                    r#message_index = Some(field.into_value());
                }
                _ => {
                    unknown_fields.push(field);
                }
            }
        }
        Ok(Self {
            r#name,
            r#sport,
            r#sub_sport,
            r#odometer,
            r#bike_spd_ant_id,
            r#bike_cad_ant_id,
            r#bike_spdcad_ant_id,
            r#bike_power_ant_id,
            r#custom_wheelsize,
            r#auto_wheelsize,
            r#bike_weight,
            r#power_cal_factor,
            r#auto_wheel_cal,
            r#auto_power_zero,
            r#id,
            r#spd_enabled,
            r#cad_enabled,
            r#spdcad_enabled,
            r#power_enabled,
            r#crank_length,
            r#enabled,
            r#bike_spd_ant_id_trans_type,
            r#bike_cad_ant_id_trans_type,
            r#bike_spdcad_ant_id_trans_type,
            r#bike_power_ant_id_trans_type,
            r#odometer_rollover,
            r#front_gear_num,
            r#front_gear,
            r#rear_gear_num,
            r#rear_gear,
            r#shimano_di2_enabled,
            r#message_index,
            unknown_fields,
        })
    }
}
impl TryFrom<FitDataRecord> for BikeProfile {
    type Error = TryFromRecordError;
    fn try_from(record: FitDataRecord) -> Result<Self, Self::Error> {
        Self::parse(record)
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize)]
pub struct Connectivity {
    pub r#bluetooth_enabled: Option<Value>,
    pub r#bluetooth_le_enabled: Option<Value>,
    pub r#ant_enabled: Option<Value>,
    pub r#name: Option<Value>,
    pub r#live_tracking_enabled: Option<Value>,
    pub r#weather_conditions_enabled: Option<Value>,
    pub r#weather_alerts_enabled: Option<Value>,
    pub r#auto_activity_upload_enabled: Option<Value>,
    pub r#course_download_enabled: Option<Value>,
    pub r#workout_download_enabled: Option<Value>,
    pub r#gps_ephemeris_download_enabled: Option<Value>,
    pub r#incident_detection_enabled: Option<Value>,
    pub r#grouptrack_enabled: Option<Value>,
    #[doc = r" All fields that are not defined in the profile."]
    pub unknown_fields: Vec<FitDataField>,
}
impl FitMessage for Connectivity {
    const NAME: &'static str = "Connectivity";
    const KIND: MesgNum = MesgNum::Connectivity;
    fn parse(record: FitDataRecord) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#bluetooth_enabled = None;
        let mut r#bluetooth_le_enabled = None;
        let mut r#ant_enabled = None;
        let mut r#name = None;
        let mut r#live_tracking_enabled = None;
        let mut r#weather_conditions_enabled = None;
        let mut r#weather_alerts_enabled = None;
        let mut r#auto_activity_upload_enabled = None;
        let mut r#course_download_enabled = None;
        let mut r#workout_download_enabled = None;
        let mut r#gps_ephemeris_download_enabled = None;
        let mut r#incident_detection_enabled = None;
        let mut r#grouptrack_enabled = None;
        let mut unknown_fields = Vec::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => {
                    r#bluetooth_enabled = Some(field.into_value());
                }
                1u8 => {
                    r#bluetooth_le_enabled = Some(field.into_value());
                }
                2u8 => {
                    r#ant_enabled = Some(field.into_value());
                }
                3u8 => {
                    r#name = Some(field.into_value());
                }
                4u8 => {
                    r#live_tracking_enabled = Some(field.into_value());
                }
                5u8 => {
                    r#weather_conditions_enabled = Some(field.into_value());
                }
                6u8 => {
                    r#weather_alerts_enabled = Some(field.into_value());
                }
                7u8 => {
                    r#auto_activity_upload_enabled = Some(field.into_value());
                }
                8u8 => {
                    r#course_download_enabled = Some(field.into_value());
                }
                9u8 => {
                    r#workout_download_enabled = Some(field.into_value());
                }
                10u8 => {
                    r#gps_ephemeris_download_enabled = Some(field.into_value());
                }
                11u8 => {
                    r#incident_detection_enabled = Some(field.into_value());
                }
                12u8 => {
                    r#grouptrack_enabled = Some(field.into_value());
                }
                _ => {
                    unknown_fields.push(field);
                }
            }
        }
        Ok(Self {
            r#bluetooth_enabled,
            r#bluetooth_le_enabled,
            r#ant_enabled,
            r#name,
            r#live_tracking_enabled,
            r#weather_conditions_enabled,
            r#weather_alerts_enabled,
            r#auto_activity_upload_enabled,
            r#course_download_enabled,
            r#workout_download_enabled,
            r#gps_ephemeris_download_enabled,
            r#incident_detection_enabled,
            r#grouptrack_enabled,
            unknown_fields,
        })
    }
}
impl TryFrom<FitDataRecord> for Connectivity {
    type Error = TryFromRecordError;
    fn try_from(record: FitDataRecord) -> Result<Self, Self::Error> {
        Self::parse(record)
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize)]
pub struct WatchfaceSettings {
    pub r#mode: Option<Value>,
    pub r#layout: Option<Value>,
    pub r#message_index: Option<Value>,
    #[doc = r" All fields that are not defined in the profile."]
    pub unknown_fields: Vec<FitDataField>,
}
impl FitMessage for WatchfaceSettings {
    const NAME: &'static str = "WatchfaceSettings";
    const KIND: MesgNum = MesgNum::WatchfaceSettings;
    fn parse(record: FitDataRecord) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#mode = None;
        let mut r#layout = None;
        let mut r#message_index = None;
        let mut unknown_fields = Vec::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => {
                    r#mode = Some(field.into_value());
                }
                1u8 => {
                    r#layout = Some(field.into_value());
                }
                254u8 => {
                    r#message_index = Some(field.into_value());
                }
                _ => {
                    unknown_fields.push(field);
                }
            }
        }
        Ok(Self {
            r#mode,
            r#layout,
            r#message_index,
            unknown_fields,
        })
    }
}
impl TryFrom<FitDataRecord> for WatchfaceSettings {
    type Error = TryFromRecordError;
    fn try_from(record: FitDataRecord) -> Result<Self, Self::Error> {
        Self::parse(record)
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize)]
pub struct OhrSettings {
    pub r#enabled: Option<Value>,
    pub r#timestamp: Option<Value>,
    #[doc = r" All fields that are not defined in the profile."]
    pub unknown_fields: Vec<FitDataField>,
}
impl FitMessage for OhrSettings {
    const NAME: &'static str = "OhrSettings";
    const KIND: MesgNum = MesgNum::OhrSettings;
    fn parse(record: FitDataRecord) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#enabled = None;
        let mut r#timestamp = None;
        let mut unknown_fields = Vec::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => {
                    r#enabled = Some(field.into_value());
                }
                253u8 => {
                    r#timestamp = Some(field.into_value());
                }
                _ => {
                    unknown_fields.push(field);
                }
            }
        }
        Ok(Self {
            r#enabled,
            r#timestamp,
            unknown_fields,
        })
    }
}
impl TryFrom<FitDataRecord> for OhrSettings {
    type Error = TryFromRecordError;
    fn try_from(record: FitDataRecord) -> Result<Self, Self::Error> {
        Self::parse(record)
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize)]
pub struct TimeInZone {
    pub r#reference_mesg: Option<Value>,
    pub r#reference_index: Option<Value>,
    pub r#time_in_hr_zone: Option<Value>,
    pub r#time_in_speed_zone: Option<Value>,
    pub r#time_in_cadence_zone: Option<Value>,
    pub r#time_in_power_zone: Option<Value>,
    pub r#hr_zone_high_boundary: Option<Value>,
    pub r#speed_zone_high_boundary: Option<Value>,
    pub r#cadence_zone_high_bondary: Option<Value>,
    pub r#power_zone_high_boundary: Option<Value>,
    pub r#hr_calc_type: Option<Value>,
    pub r#max_heart_rate: Option<Value>,
    pub r#resting_heart_rate: Option<Value>,
    pub r#threshold_heart_rate: Option<Value>,
    pub r#pwr_calc_type: Option<Value>,
    pub r#functional_threshold_power: Option<Value>,
    pub r#timestamp: Option<Value>,
    #[doc = r" All fields that are not defined in the profile."]
    pub unknown_fields: Vec<FitDataField>,
}
impl FitMessage for TimeInZone {
    const NAME: &'static str = "TimeInZone";
    const KIND: MesgNum = MesgNum::TimeInZone;
    fn parse(record: FitDataRecord) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#reference_mesg = None;
        let mut r#reference_index = None;
        let mut r#time_in_hr_zone = None;
        let mut r#time_in_speed_zone = None;
        let mut r#time_in_cadence_zone = None;
        let mut r#time_in_power_zone = None;
        let mut r#hr_zone_high_boundary = None;
        let mut r#speed_zone_high_boundary = None;
        let mut r#cadence_zone_high_bondary = None;
        let mut r#power_zone_high_boundary = None;
        let mut r#hr_calc_type = None;
        let mut r#max_heart_rate = None;
        let mut r#resting_heart_rate = None;
        let mut r#threshold_heart_rate = None;
        let mut r#pwr_calc_type = None;
        let mut r#functional_threshold_power = None;
        let mut r#timestamp = None;
        let mut unknown_fields = Vec::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => {
                    r#reference_mesg = Some(field.into_value());
                }
                1u8 => {
                    r#reference_index = Some(field.into_value());
                }
                2u8 => {
                    r#time_in_hr_zone = Some(field.into_value());
                }
                3u8 => {
                    r#time_in_speed_zone = Some(field.into_value());
                }
                4u8 => {
                    r#time_in_cadence_zone = Some(field.into_value());
                }
                5u8 => {
                    r#time_in_power_zone = Some(field.into_value());
                }
                6u8 => {
                    r#hr_zone_high_boundary = Some(field.into_value());
                }
                7u8 => {
                    r#speed_zone_high_boundary = Some(field.into_value());
                }
                8u8 => {
                    r#cadence_zone_high_bondary = Some(field.into_value());
                }
                9u8 => {
                    r#power_zone_high_boundary = Some(field.into_value());
                }
                10u8 => {
                    r#hr_calc_type = Some(field.into_value());
                }
                11u8 => {
                    r#max_heart_rate = Some(field.into_value());
                }
                12u8 => {
                    r#resting_heart_rate = Some(field.into_value());
                }
                13u8 => {
                    r#threshold_heart_rate = Some(field.into_value());
                }
                14u8 => {
                    r#pwr_calc_type = Some(field.into_value());
                }
                15u8 => {
                    r#functional_threshold_power = Some(field.into_value());
                }
                253u8 => {
                    r#timestamp = Some(field.into_value());
                }
                _ => {
                    unknown_fields.push(field);
                }
            }
        }
        Ok(Self {
            r#reference_mesg,
            r#reference_index,
            r#time_in_hr_zone,
            r#time_in_speed_zone,
            r#time_in_cadence_zone,
            r#time_in_power_zone,
            r#hr_zone_high_boundary,
            r#speed_zone_high_boundary,
            r#cadence_zone_high_bondary,
            r#power_zone_high_boundary,
            r#hr_calc_type,
            r#max_heart_rate,
            r#resting_heart_rate,
            r#threshold_heart_rate,
            r#pwr_calc_type,
            r#functional_threshold_power,
            r#timestamp,
            unknown_fields,
        })
    }
}
impl TryFrom<FitDataRecord> for TimeInZone {
    type Error = TryFromRecordError;
    fn try_from(record: FitDataRecord) -> Result<Self, Self::Error> {
        Self::parse(record)
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize)]
pub struct ZonesTarget {
    pub r#max_heart_rate: Option<Value>,
    pub r#threshold_heart_rate: Option<Value>,
    pub r#functional_threshold_power: Option<Value>,
    pub r#hr_calc_type: Option<Value>,
    pub r#pwr_calc_type: Option<Value>,
    #[doc = r" All fields that are not defined in the profile."]
    pub unknown_fields: Vec<FitDataField>,
}
impl FitMessage for ZonesTarget {
    const NAME: &'static str = "ZonesTarget";
    const KIND: MesgNum = MesgNum::ZonesTarget;
    fn parse(record: FitDataRecord) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#max_heart_rate = None;
        let mut r#threshold_heart_rate = None;
        let mut r#functional_threshold_power = None;
        let mut r#hr_calc_type = None;
        let mut r#pwr_calc_type = None;
        let mut unknown_fields = Vec::new();
        for field in record.into_vec() {
            match field.number() {
                1u8 => {
                    r#max_heart_rate = Some(field.into_value());
                }
                2u8 => {
                    r#threshold_heart_rate = Some(field.into_value());
                }
                3u8 => {
                    r#functional_threshold_power = Some(field.into_value());
                }
                5u8 => {
                    r#hr_calc_type = Some(field.into_value());
                }
                7u8 => {
                    r#pwr_calc_type = Some(field.into_value());
                }
                _ => {
                    unknown_fields.push(field);
                }
            }
        }
        Ok(Self {
            r#max_heart_rate,
            r#threshold_heart_rate,
            r#functional_threshold_power,
            r#hr_calc_type,
            r#pwr_calc_type,
            unknown_fields,
        })
    }
}
impl TryFrom<FitDataRecord> for ZonesTarget {
    type Error = TryFromRecordError;
    fn try_from(record: FitDataRecord) -> Result<Self, Self::Error> {
        Self::parse(record)
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize)]
pub struct Sport {
    pub r#sport: Option<Value>,
    pub r#sub_sport: Option<Value>,
    pub r#name: Option<Value>,
    #[doc = r" All fields that are not defined in the profile."]
    pub unknown_fields: Vec<FitDataField>,
}
impl FitMessage for Sport {
    const NAME: &'static str = "Sport";
    const KIND: MesgNum = MesgNum::Sport;
    fn parse(record: FitDataRecord) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#sport = None;
        let mut r#sub_sport = None;
        let mut r#name = None;
        let mut unknown_fields = Vec::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => {
                    r#sport = Some(field.into_value());
                }
                1u8 => {
                    r#sub_sport = Some(field.into_value());
                }
                3u8 => {
                    r#name = Some(field.into_value());
                }
                _ => {
                    unknown_fields.push(field);
                }
            }
        }
        Ok(Self {
            r#sport,
            r#sub_sport,
            r#name,
            unknown_fields,
        })
    }
}
impl TryFrom<FitDataRecord> for Sport {
    type Error = TryFromRecordError;
    fn try_from(record: FitDataRecord) -> Result<Self, Self::Error> {
        Self::parse(record)
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize)]
pub struct HrZone {
    pub r#high_bpm: Option<Value>,
    pub r#name: Option<Value>,
    pub r#message_index: Option<Value>,
    #[doc = r" All fields that are not defined in the profile."]
    pub unknown_fields: Vec<FitDataField>,
}
impl FitMessage for HrZone {
    const NAME: &'static str = "HrZone";
    const KIND: MesgNum = MesgNum::HrZone;
    fn parse(record: FitDataRecord) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#high_bpm = None;
        let mut r#name = None;
        let mut r#message_index = None;
        let mut unknown_fields = Vec::new();
        for field in record.into_vec() {
            match field.number() {
                1u8 => {
                    r#high_bpm = Some(field.into_value());
                }
                2u8 => {
                    r#name = Some(field.into_value());
                }
                254u8 => {
                    r#message_index = Some(field.into_value());
                }
                _ => {
                    unknown_fields.push(field);
                }
            }
        }
        Ok(Self {
            r#high_bpm,
            r#name,
            r#message_index,
            unknown_fields,
        })
    }
}
impl TryFrom<FitDataRecord> for HrZone {
    type Error = TryFromRecordError;
    fn try_from(record: FitDataRecord) -> Result<Self, Self::Error> {
        Self::parse(record)
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize)]
pub struct SpeedZone {
    pub r#high_value: Option<Value>,
    pub r#name: Option<Value>,
    pub r#message_index: Option<Value>,
    #[doc = r" All fields that are not defined in the profile."]
    pub unknown_fields: Vec<FitDataField>,
}
impl FitMessage for SpeedZone {
    const NAME: &'static str = "SpeedZone";
    const KIND: MesgNum = MesgNum::SpeedZone;
    fn parse(record: FitDataRecord) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#high_value = None;
        let mut r#name = None;
        let mut r#message_index = None;
        let mut unknown_fields = Vec::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => {
                    r#high_value = Some(field.into_value());
                }
                1u8 => {
                    r#name = Some(field.into_value());
                }
                254u8 => {
                    r#message_index = Some(field.into_value());
                }
                _ => {
                    unknown_fields.push(field);
                }
            }
        }
        Ok(Self {
            r#high_value,
            r#name,
            r#message_index,
            unknown_fields,
        })
    }
}
impl TryFrom<FitDataRecord> for SpeedZone {
    type Error = TryFromRecordError;
    fn try_from(record: FitDataRecord) -> Result<Self, Self::Error> {
        Self::parse(record)
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize)]
pub struct CadenceZone {
    pub r#high_value: Option<Value>,
    pub r#name: Option<Value>,
    pub r#message_index: Option<Value>,
    #[doc = r" All fields that are not defined in the profile."]
    pub unknown_fields: Vec<FitDataField>,
}
impl FitMessage for CadenceZone {
    const NAME: &'static str = "CadenceZone";
    const KIND: MesgNum = MesgNum::CadenceZone;
    fn parse(record: FitDataRecord) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#high_value = None;
        let mut r#name = None;
        let mut r#message_index = None;
        let mut unknown_fields = Vec::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => {
                    r#high_value = Some(field.into_value());
                }
                1u8 => {
                    r#name = Some(field.into_value());
                }
                254u8 => {
                    r#message_index = Some(field.into_value());
                }
                _ => {
                    unknown_fields.push(field);
                }
            }
        }
        Ok(Self {
            r#high_value,
            r#name,
            r#message_index,
            unknown_fields,
        })
    }
}
impl TryFrom<FitDataRecord> for CadenceZone {
    type Error = TryFromRecordError;
    fn try_from(record: FitDataRecord) -> Result<Self, Self::Error> {
        Self::parse(record)
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize)]
pub struct PowerZone {
    pub r#high_value: Option<Value>,
    pub r#name: Option<Value>,
    pub r#message_index: Option<Value>,
    #[doc = r" All fields that are not defined in the profile."]
    pub unknown_fields: Vec<FitDataField>,
}
impl FitMessage for PowerZone {
    const NAME: &'static str = "PowerZone";
    const KIND: MesgNum = MesgNum::PowerZone;
    fn parse(record: FitDataRecord) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#high_value = None;
        let mut r#name = None;
        let mut r#message_index = None;
        let mut unknown_fields = Vec::new();
        for field in record.into_vec() {
            match field.number() {
                1u8 => {
                    r#high_value = Some(field.into_value());
                }
                2u8 => {
                    r#name = Some(field.into_value());
                }
                254u8 => {
                    r#message_index = Some(field.into_value());
                }
                _ => {
                    unknown_fields.push(field);
                }
            }
        }
        Ok(Self {
            r#high_value,
            r#name,
            r#message_index,
            unknown_fields,
        })
    }
}
impl TryFrom<FitDataRecord> for PowerZone {
    type Error = TryFromRecordError;
    fn try_from(record: FitDataRecord) -> Result<Self, Self::Error> {
        Self::parse(record)
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize)]
pub struct MetZone {
    pub r#high_bpm: Option<Value>,
    pub r#calories: Option<Value>,
    pub r#fat_calories: Option<Value>,
    pub r#message_index: Option<Value>,
    #[doc = r" All fields that are not defined in the profile."]
    pub unknown_fields: Vec<FitDataField>,
}
impl FitMessage for MetZone {
    const NAME: &'static str = "MetZone";
    const KIND: MesgNum = MesgNum::MetZone;
    fn parse(record: FitDataRecord) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#high_bpm = None;
        let mut r#calories = None;
        let mut r#fat_calories = None;
        let mut r#message_index = None;
        let mut unknown_fields = Vec::new();
        for field in record.into_vec() {
            match field.number() {
                1u8 => {
                    r#high_bpm = Some(field.into_value());
                }
                2u8 => {
                    r#calories = Some(field.into_value());
                }
                3u8 => {
                    r#fat_calories = Some(field.into_value());
                }
                254u8 => {
                    r#message_index = Some(field.into_value());
                }
                _ => {
                    unknown_fields.push(field);
                }
            }
        }
        Ok(Self {
            r#high_bpm,
            r#calories,
            r#fat_calories,
            r#message_index,
            unknown_fields,
        })
    }
}
impl TryFrom<FitDataRecord> for MetZone {
    type Error = TryFromRecordError;
    fn try_from(record: FitDataRecord) -> Result<Self, Self::Error> {
        Self::parse(record)
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize)]
pub struct DiveSettings {
    pub r#name: Option<Value>,
    pub r#model: Option<Value>,
    pub r#gf_low: Option<Value>,
    pub r#gf_high: Option<Value>,
    pub r#water_type: Option<Value>,
    pub r#water_density: Option<Value>,
    pub r#po2_warn: Option<Value>,
    pub r#po2_critical: Option<Value>,
    pub r#po2_deco: Option<Value>,
    pub r#safety_stop_enabled: Option<Value>,
    pub r#bottom_depth: Option<Value>,
    pub r#bottom_time: Option<Value>,
    pub r#apnea_countdown_enabled: Option<Value>,
    pub r#apnea_countdown_time: Option<Value>,
    pub r#backlight_mode: Option<Value>,
    pub r#backlight_brightness: Option<Value>,
    pub r#backlight_timeout: Option<Value>,
    pub r#repeat_dive_interval: Option<Value>,
    pub r#safety_stop_time: Option<Value>,
    pub r#heart_rate_source_type: Option<Value>,
    pub r#heart_rate_source: Option<Value>,
    pub r#travel_gas: Option<Value>,
    pub r#ccr_low_setpoint_switch_mode: Option<Value>,
    pub r#ccr_low_setpoint: Option<Value>,
    pub r#ccr_low_setpoint_depth: Option<Value>,
    pub r#ccr_high_setpoint_switch_mode: Option<Value>,
    pub r#ccr_high_setpoint: Option<Value>,
    pub r#ccr_high_setpoint_depth: Option<Value>,
    pub r#gas_consumption_display: Option<Value>,
    pub r#up_key_enabled: Option<Value>,
    pub r#dive_sounds: Option<Value>,
    pub r#last_stop_multiple: Option<Value>,
    pub r#no_fly_time_mode: Option<Value>,
    pub r#timestamp: Option<Value>,
    pub r#message_index: Option<Value>,
    #[doc = r" All fields that are not defined in the profile."]
    pub unknown_fields: Vec<FitDataField>,
}
impl FitMessage for DiveSettings {
    const NAME: &'static str = "DiveSettings";
    const KIND: MesgNum = MesgNum::DiveSettings;
    fn parse(record: FitDataRecord) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#name = None;
        let mut r#model = None;
        let mut r#gf_low = None;
        let mut r#gf_high = None;
        let mut r#water_type = None;
        let mut r#water_density = None;
        let mut r#po2_warn = None;
        let mut r#po2_critical = None;
        let mut r#po2_deco = None;
        let mut r#safety_stop_enabled = None;
        let mut r#bottom_depth = None;
        let mut r#bottom_time = None;
        let mut r#apnea_countdown_enabled = None;
        let mut r#apnea_countdown_time = None;
        let mut r#backlight_mode = None;
        let mut r#backlight_brightness = None;
        let mut r#backlight_timeout = None;
        let mut r#repeat_dive_interval = None;
        let mut r#safety_stop_time = None;
        let mut r#heart_rate_source_type = None;
        let mut r#heart_rate_source = None;
        let mut r#travel_gas = None;
        let mut r#ccr_low_setpoint_switch_mode = None;
        let mut r#ccr_low_setpoint = None;
        let mut r#ccr_low_setpoint_depth = None;
        let mut r#ccr_high_setpoint_switch_mode = None;
        let mut r#ccr_high_setpoint = None;
        let mut r#ccr_high_setpoint_depth = None;
        let mut r#gas_consumption_display = None;
        let mut r#up_key_enabled = None;
        let mut r#dive_sounds = None;
        let mut r#last_stop_multiple = None;
        let mut r#no_fly_time_mode = None;
        let mut r#timestamp = None;
        let mut r#message_index = None;
        let mut unknown_fields = Vec::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => {
                    r#name = Some(field.into_value());
                }
                1u8 => {
                    r#model = Some(field.into_value());
                }
                2u8 => {
                    r#gf_low = Some(field.into_value());
                }
                3u8 => {
                    r#gf_high = Some(field.into_value());
                }
                4u8 => {
                    r#water_type = Some(field.into_value());
                }
                5u8 => {
                    r#water_density = Some(field.into_value());
                }
                6u8 => {
                    r#po2_warn = Some(field.into_value());
                }
                7u8 => {
                    r#po2_critical = Some(field.into_value());
                }
                8u8 => {
                    r#po2_deco = Some(field.into_value());
                }
                9u8 => {
                    r#safety_stop_enabled = Some(field.into_value());
                }
                10u8 => {
                    r#bottom_depth = Some(field.into_value());
                }
                11u8 => {
                    r#bottom_time = Some(field.into_value());
                }
                12u8 => {
                    r#apnea_countdown_enabled = Some(field.into_value());
                }
                13u8 => {
                    r#apnea_countdown_time = Some(field.into_value());
                }
                14u8 => {
                    r#backlight_mode = Some(field.into_value());
                }
                15u8 => {
                    r#backlight_brightness = Some(field.into_value());
                }
                16u8 => {
                    r#backlight_timeout = Some(field.into_value());
                }
                17u8 => {
                    r#repeat_dive_interval = Some(field.into_value());
                }
                18u8 => {
                    r#safety_stop_time = Some(field.into_value());
                }
                19u8 => {
                    r#heart_rate_source_type = Some(field.into_value());
                }
                20u8 => {
                    r#heart_rate_source = Some(field.into_value());
                }
                21u8 => {
                    r#travel_gas = Some(field.into_value());
                }
                22u8 => {
                    r#ccr_low_setpoint_switch_mode = Some(field.into_value());
                }
                23u8 => {
                    r#ccr_low_setpoint = Some(field.into_value());
                }
                24u8 => {
                    r#ccr_low_setpoint_depth = Some(field.into_value());
                }
                25u8 => {
                    r#ccr_high_setpoint_switch_mode = Some(field.into_value());
                }
                26u8 => {
                    r#ccr_high_setpoint = Some(field.into_value());
                }
                27u8 => {
                    r#ccr_high_setpoint_depth = Some(field.into_value());
                }
                29u8 => {
                    r#gas_consumption_display = Some(field.into_value());
                }
                30u8 => {
                    r#up_key_enabled = Some(field.into_value());
                }
                35u8 => {
                    r#dive_sounds = Some(field.into_value());
                }
                36u8 => {
                    r#last_stop_multiple = Some(field.into_value());
                }
                37u8 => {
                    r#no_fly_time_mode = Some(field.into_value());
                }
                253u8 => {
                    r#timestamp = Some(field.into_value());
                }
                254u8 => {
                    r#message_index = Some(field.into_value());
                }
                _ => {
                    unknown_fields.push(field);
                }
            }
        }
        Ok(Self {
            r#name,
            r#model,
            r#gf_low,
            r#gf_high,
            r#water_type,
            r#water_density,
            r#po2_warn,
            r#po2_critical,
            r#po2_deco,
            r#safety_stop_enabled,
            r#bottom_depth,
            r#bottom_time,
            r#apnea_countdown_enabled,
            r#apnea_countdown_time,
            r#backlight_mode,
            r#backlight_brightness,
            r#backlight_timeout,
            r#repeat_dive_interval,
            r#safety_stop_time,
            r#heart_rate_source_type,
            r#heart_rate_source,
            r#travel_gas,
            r#ccr_low_setpoint_switch_mode,
            r#ccr_low_setpoint,
            r#ccr_low_setpoint_depth,
            r#ccr_high_setpoint_switch_mode,
            r#ccr_high_setpoint,
            r#ccr_high_setpoint_depth,
            r#gas_consumption_display,
            r#up_key_enabled,
            r#dive_sounds,
            r#last_stop_multiple,
            r#no_fly_time_mode,
            r#timestamp,
            r#message_index,
            unknown_fields,
        })
    }
}
impl TryFrom<FitDataRecord> for DiveSettings {
    type Error = TryFromRecordError;
    fn try_from(record: FitDataRecord) -> Result<Self, Self::Error> {
        Self::parse(record)
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize)]
pub struct DiveAlarm {
    pub r#depth: Option<Value>,
    pub r#time: Option<Value>,
    pub r#enabled: Option<Value>,
    pub r#alarm_type: Option<Value>,
    pub r#sound: Option<Value>,
    pub r#dive_types: Option<Value>,
    pub r#id: Option<Value>,
    pub r#popup_enabled: Option<Value>,
    pub r#trigger_on_descent: Option<Value>,
    pub r#trigger_on_ascent: Option<Value>,
    pub r#repeating: Option<Value>,
    pub r#speed: Option<Value>,
    pub r#message_index: Option<Value>,
    #[doc = r" All fields that are not defined in the profile."]
    pub unknown_fields: Vec<FitDataField>,
}
impl FitMessage for DiveAlarm {
    const NAME: &'static str = "DiveAlarm";
    const KIND: MesgNum = MesgNum::DiveAlarm;
    fn parse(record: FitDataRecord) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#depth = None;
        let mut r#time = None;
        let mut r#enabled = None;
        let mut r#alarm_type = None;
        let mut r#sound = None;
        let mut r#dive_types = None;
        let mut r#id = None;
        let mut r#popup_enabled = None;
        let mut r#trigger_on_descent = None;
        let mut r#trigger_on_ascent = None;
        let mut r#repeating = None;
        let mut r#speed = None;
        let mut r#message_index = None;
        let mut unknown_fields = Vec::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => {
                    r#depth = Some(field.into_value());
                }
                1u8 => {
                    r#time = Some(field.into_value());
                }
                2u8 => {
                    r#enabled = Some(field.into_value());
                }
                3u8 => {
                    r#alarm_type = Some(field.into_value());
                }
                4u8 => {
                    r#sound = Some(field.into_value());
                }
                5u8 => {
                    r#dive_types = Some(field.into_value());
                }
                6u8 => {
                    r#id = Some(field.into_value());
                }
                7u8 => {
                    r#popup_enabled = Some(field.into_value());
                }
                8u8 => {
                    r#trigger_on_descent = Some(field.into_value());
                }
                9u8 => {
                    r#trigger_on_ascent = Some(field.into_value());
                }
                10u8 => {
                    r#repeating = Some(field.into_value());
                }
                11u8 => {
                    r#speed = Some(field.into_value());
                }
                254u8 => {
                    r#message_index = Some(field.into_value());
                }
                _ => {
                    unknown_fields.push(field);
                }
            }
        }
        Ok(Self {
            r#depth,
            r#time,
            r#enabled,
            r#alarm_type,
            r#sound,
            r#dive_types,
            r#id,
            r#popup_enabled,
            r#trigger_on_descent,
            r#trigger_on_ascent,
            r#repeating,
            r#speed,
            r#message_index,
            unknown_fields,
        })
    }
}
impl TryFrom<FitDataRecord> for DiveAlarm {
    type Error = TryFromRecordError;
    fn try_from(record: FitDataRecord) -> Result<Self, Self::Error> {
        Self::parse(record)
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize)]
pub struct DiveApneaAlarm {
    pub r#depth: Option<Value>,
    pub r#time: Option<Value>,
    pub r#enabled: Option<Value>,
    pub r#alarm_type: Option<Value>,
    pub r#sound: Option<Value>,
    pub r#dive_types: Option<Value>,
    pub r#id: Option<Value>,
    pub r#popup_enabled: Option<Value>,
    pub r#trigger_on_descent: Option<Value>,
    pub r#trigger_on_ascent: Option<Value>,
    pub r#repeating: Option<Value>,
    pub r#speed: Option<Value>,
    pub r#message_index: Option<Value>,
    #[doc = r" All fields that are not defined in the profile."]
    pub unknown_fields: Vec<FitDataField>,
}
impl FitMessage for DiveApneaAlarm {
    const NAME: &'static str = "DiveApneaAlarm";
    const KIND: MesgNum = MesgNum::DiveApneaAlarm;
    fn parse(record: FitDataRecord) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#depth = None;
        let mut r#time = None;
        let mut r#enabled = None;
        let mut r#alarm_type = None;
        let mut r#sound = None;
        let mut r#dive_types = None;
        let mut r#id = None;
        let mut r#popup_enabled = None;
        let mut r#trigger_on_descent = None;
        let mut r#trigger_on_ascent = None;
        let mut r#repeating = None;
        let mut r#speed = None;
        let mut r#message_index = None;
        let mut unknown_fields = Vec::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => {
                    r#depth = Some(field.into_value());
                }
                1u8 => {
                    r#time = Some(field.into_value());
                }
                2u8 => {
                    r#enabled = Some(field.into_value());
                }
                3u8 => {
                    r#alarm_type = Some(field.into_value());
                }
                4u8 => {
                    r#sound = Some(field.into_value());
                }
                5u8 => {
                    r#dive_types = Some(field.into_value());
                }
                6u8 => {
                    r#id = Some(field.into_value());
                }
                7u8 => {
                    r#popup_enabled = Some(field.into_value());
                }
                8u8 => {
                    r#trigger_on_descent = Some(field.into_value());
                }
                9u8 => {
                    r#trigger_on_ascent = Some(field.into_value());
                }
                10u8 => {
                    r#repeating = Some(field.into_value());
                }
                11u8 => {
                    r#speed = Some(field.into_value());
                }
                254u8 => {
                    r#message_index = Some(field.into_value());
                }
                _ => {
                    unknown_fields.push(field);
                }
            }
        }
        Ok(Self {
            r#depth,
            r#time,
            r#enabled,
            r#alarm_type,
            r#sound,
            r#dive_types,
            r#id,
            r#popup_enabled,
            r#trigger_on_descent,
            r#trigger_on_ascent,
            r#repeating,
            r#speed,
            r#message_index,
            unknown_fields,
        })
    }
}
impl TryFrom<FitDataRecord> for DiveApneaAlarm {
    type Error = TryFromRecordError;
    fn try_from(record: FitDataRecord) -> Result<Self, Self::Error> {
        Self::parse(record)
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize)]
pub struct DiveGas {
    pub r#helium_content: Option<Value>,
    pub r#oxygen_content: Option<Value>,
    pub r#status: Option<Value>,
    pub r#mode: Option<Value>,
    pub r#message_index: Option<Value>,
    #[doc = r" All fields that are not defined in the profile."]
    pub unknown_fields: Vec<FitDataField>,
}
impl FitMessage for DiveGas {
    const NAME: &'static str = "DiveGas";
    const KIND: MesgNum = MesgNum::DiveGas;
    fn parse(record: FitDataRecord) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#helium_content = None;
        let mut r#oxygen_content = None;
        let mut r#status = None;
        let mut r#mode = None;
        let mut r#message_index = None;
        let mut unknown_fields = Vec::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => {
                    r#helium_content = Some(field.into_value());
                }
                1u8 => {
                    r#oxygen_content = Some(field.into_value());
                }
                2u8 => {
                    r#status = Some(field.into_value());
                }
                3u8 => {
                    r#mode = Some(field.into_value());
                }
                254u8 => {
                    r#message_index = Some(field.into_value());
                }
                _ => {
                    unknown_fields.push(field);
                }
            }
        }
        Ok(Self {
            r#helium_content,
            r#oxygen_content,
            r#status,
            r#mode,
            r#message_index,
            unknown_fields,
        })
    }
}
impl TryFrom<FitDataRecord> for DiveGas {
    type Error = TryFromRecordError;
    fn try_from(record: FitDataRecord) -> Result<Self, Self::Error> {
        Self::parse(record)
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize)]
pub struct Goal {
    pub r#sport: Option<Value>,
    pub r#sub_sport: Option<Value>,
    pub r#start_date: Option<Value>,
    pub r#end_date: Option<Value>,
    pub r#type: Option<Value>,
    pub r#value: Option<Value>,
    pub r#repeat: Option<Value>,
    pub r#target_value: Option<Value>,
    pub r#recurrence: Option<Value>,
    pub r#recurrence_value: Option<Value>,
    pub r#enabled: Option<Value>,
    pub r#source: Option<Value>,
    pub r#message_index: Option<Value>,
    #[doc = r" All fields that are not defined in the profile."]
    pub unknown_fields: Vec<FitDataField>,
}
impl FitMessage for Goal {
    const NAME: &'static str = "Goal";
    const KIND: MesgNum = MesgNum::Goal;
    fn parse(record: FitDataRecord) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#sport = None;
        let mut r#sub_sport = None;
        let mut r#start_date = None;
        let mut r#end_date = None;
        let mut r#type = None;
        let mut r#value = None;
        let mut r#repeat = None;
        let mut r#target_value = None;
        let mut r#recurrence = None;
        let mut r#recurrence_value = None;
        let mut r#enabled = None;
        let mut r#source = None;
        let mut r#message_index = None;
        let mut unknown_fields = Vec::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => {
                    r#sport = Some(field.into_value());
                }
                1u8 => {
                    r#sub_sport = Some(field.into_value());
                }
                2u8 => {
                    r#start_date = Some(field.into_value());
                }
                3u8 => {
                    r#end_date = Some(field.into_value());
                }
                4u8 => {
                    r#type = Some(field.into_value());
                }
                5u8 => {
                    r#value = Some(field.into_value());
                }
                6u8 => {
                    r#repeat = Some(field.into_value());
                }
                7u8 => {
                    r#target_value = Some(field.into_value());
                }
                8u8 => {
                    r#recurrence = Some(field.into_value());
                }
                9u8 => {
                    r#recurrence_value = Some(field.into_value());
                }
                10u8 => {
                    r#enabled = Some(field.into_value());
                }
                11u8 => {
                    r#source = Some(field.into_value());
                }
                254u8 => {
                    r#message_index = Some(field.into_value());
                }
                _ => {
                    unknown_fields.push(field);
                }
            }
        }
        Ok(Self {
            r#sport,
            r#sub_sport,
            r#start_date,
            r#end_date,
            r#type,
            r#value,
            r#repeat,
            r#target_value,
            r#recurrence,
            r#recurrence_value,
            r#enabled,
            r#source,
            r#message_index,
            unknown_fields,
        })
    }
}
impl TryFrom<FitDataRecord> for Goal {
    type Error = TryFromRecordError;
    fn try_from(record: FitDataRecord) -> Result<Self, Self::Error> {
        Self::parse(record)
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize)]
pub struct Activity {
    pub r#total_timer_time: Option<Value>,
    pub r#num_sessions: Option<Value>,
    pub r#type: Option<Value>,
    pub r#event: Option<Value>,
    pub r#event_type: Option<Value>,
    pub r#local_timestamp: Option<Value>,
    pub r#event_group: Option<Value>,
    pub r#timestamp: Option<Value>,
    #[doc = r" All fields that are not defined in the profile."]
    pub unknown_fields: Vec<FitDataField>,
}
impl FitMessage for Activity {
    const NAME: &'static str = "Activity";
    const KIND: MesgNum = MesgNum::Activity;
    fn parse(record: FitDataRecord) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#total_timer_time = None;
        let mut r#num_sessions = None;
        let mut r#type = None;
        let mut r#event = None;
        let mut r#event_type = None;
        let mut r#local_timestamp = None;
        let mut r#event_group = None;
        let mut r#timestamp = None;
        let mut unknown_fields = Vec::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => {
                    r#total_timer_time = Some(field.into_value());
                }
                1u8 => {
                    r#num_sessions = Some(field.into_value());
                }
                2u8 => {
                    r#type = Some(field.into_value());
                }
                3u8 => {
                    r#event = Some(field.into_value());
                }
                4u8 => {
                    r#event_type = Some(field.into_value());
                }
                5u8 => {
                    r#local_timestamp = Some(field.into_value());
                }
                6u8 => {
                    r#event_group = Some(field.into_value());
                }
                253u8 => {
                    r#timestamp = Some(field.into_value());
                }
                _ => {
                    unknown_fields.push(field);
                }
            }
        }
        Ok(Self {
            r#total_timer_time,
            r#num_sessions,
            r#type,
            r#event,
            r#event_type,
            r#local_timestamp,
            r#event_group,
            r#timestamp,
            unknown_fields,
        })
    }
}
impl TryFrom<FitDataRecord> for Activity {
    type Error = TryFromRecordError;
    fn try_from(record: FitDataRecord) -> Result<Self, Self::Error> {
        Self::parse(record)
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize)]
pub struct Session {
    pub r#event: Option<Value>,
    pub r#event_type: Option<Value>,
    pub r#start_time: Option<Value>,
    pub r#start_position_lat: Option<Value>,
    pub r#start_position_long: Option<Value>,
    pub r#sport: Option<Value>,
    pub r#sub_sport: Option<Value>,
    pub r#total_elapsed_time: Option<Value>,
    pub r#total_timer_time: Option<Value>,
    pub r#total_distance: Option<Value>,
    pub r#total_cycles: Option<Value>,
    pub r#total_calories: Option<Value>,
    pub r#total_fat_calories: Option<Value>,
    pub r#avg_speed: Option<Value>,
    pub r#max_speed: Option<Value>,
    pub r#avg_heart_rate: Option<Value>,
    pub r#max_heart_rate: Option<Value>,
    pub r#avg_cadence: Option<Value>,
    pub r#max_cadence: Option<Value>,
    pub r#avg_power: Option<Value>,
    pub r#max_power: Option<Value>,
    pub r#total_ascent: Option<Value>,
    pub r#total_descent: Option<Value>,
    pub r#total_training_effect: Option<Value>,
    pub r#first_lap_index: Option<Value>,
    pub r#num_laps: Option<Value>,
    pub r#event_group: Option<Value>,
    pub r#trigger: Option<Value>,
    pub r#nec_lat: Option<Value>,
    pub r#nec_long: Option<Value>,
    pub r#swc_lat: Option<Value>,
    pub r#swc_long: Option<Value>,
    pub r#num_lengths: Option<Value>,
    pub r#normalized_power: Option<Value>,
    pub r#training_stress_score: Option<Value>,
    pub r#intensity_factor: Option<Value>,
    pub r#left_right_balance: Option<Value>,
    pub r#end_position_lat: Option<Value>,
    pub r#end_position_long: Option<Value>,
    pub r#avg_stroke_count: Option<Value>,
    pub r#avg_stroke_distance: Option<Value>,
    pub r#swim_stroke: Option<Value>,
    pub r#pool_length: Option<Value>,
    pub r#threshold_power: Option<Value>,
    pub r#pool_length_unit: Option<Value>,
    pub r#num_active_lengths: Option<Value>,
    pub r#total_work: Option<Value>,
    pub r#avg_altitude: Option<Value>,
    pub r#max_altitude: Option<Value>,
    pub r#gps_accuracy: Option<Value>,
    pub r#avg_grade: Option<Value>,
    pub r#avg_pos_grade: Option<Value>,
    pub r#avg_neg_grade: Option<Value>,
    pub r#max_pos_grade: Option<Value>,
    pub r#max_neg_grade: Option<Value>,
    pub r#avg_temperature: Option<Value>,
    pub r#max_temperature: Option<Value>,
    pub r#total_moving_time: Option<Value>,
    pub r#avg_pos_vertical_speed: Option<Value>,
    pub r#avg_neg_vertical_speed: Option<Value>,
    pub r#max_pos_vertical_speed: Option<Value>,
    pub r#max_neg_vertical_speed: Option<Value>,
    pub r#min_heart_rate: Option<Value>,
    pub r#time_in_hr_zone: Option<Value>,
    pub r#time_in_speed_zone: Option<Value>,
    pub r#time_in_cadence_zone: Option<Value>,
    pub r#time_in_power_zone: Option<Value>,
    pub r#avg_lap_time: Option<Value>,
    pub r#best_lap_index: Option<Value>,
    pub r#min_altitude: Option<Value>,
    pub r#player_score: Option<Value>,
    pub r#opponent_score: Option<Value>,
    pub r#opponent_name: Option<Value>,
    pub r#stroke_count: Option<Value>,
    pub r#zone_count: Option<Value>,
    pub r#max_ball_speed: Option<Value>,
    pub r#avg_ball_speed: Option<Value>,
    pub r#avg_vertical_oscillation: Option<Value>,
    pub r#avg_stance_time_percent: Option<Value>,
    pub r#avg_stance_time: Option<Value>,
    pub r#avg_fractional_cadence: Option<Value>,
    pub r#max_fractional_cadence: Option<Value>,
    pub r#total_fractional_cycles: Option<Value>,
    pub r#avg_total_hemoglobin_conc: Option<Value>,
    pub r#min_total_hemoglobin_conc: Option<Value>,
    pub r#max_total_hemoglobin_conc: Option<Value>,
    pub r#avg_saturated_hemoglobin_percent: Option<Value>,
    pub r#min_saturated_hemoglobin_percent: Option<Value>,
    pub r#max_saturated_hemoglobin_percent: Option<Value>,
    pub r#avg_left_torque_effectiveness: Option<Value>,
    pub r#avg_right_torque_effectiveness: Option<Value>,
    pub r#avg_left_pedal_smoothness: Option<Value>,
    pub r#avg_right_pedal_smoothness: Option<Value>,
    pub r#avg_combined_pedal_smoothness: Option<Value>,
    pub r#sport_profile_name: Option<Value>,
    pub r#sport_index: Option<Value>,
    pub r#time_standing: Option<Value>,
    pub r#stand_count: Option<Value>,
    pub r#avg_left_pco: Option<Value>,
    pub r#avg_right_pco: Option<Value>,
    pub r#avg_left_power_phase: Option<Value>,
    pub r#avg_left_power_phase_peak: Option<Value>,
    pub r#avg_right_power_phase: Option<Value>,
    pub r#avg_right_power_phase_peak: Option<Value>,
    pub r#avg_power_position: Option<Value>,
    pub r#max_power_position: Option<Value>,
    pub r#avg_cadence_position: Option<Value>,
    pub r#max_cadence_position: Option<Value>,
    pub r#enhanced_avg_speed: Option<Value>,
    pub r#enhanced_max_speed: Option<Value>,
    pub r#enhanced_avg_altitude: Option<Value>,
    pub r#enhanced_min_altitude: Option<Value>,
    pub r#enhanced_max_altitude: Option<Value>,
    pub r#avg_lev_motor_power: Option<Value>,
    pub r#max_lev_motor_power: Option<Value>,
    pub r#lev_battery_consumption: Option<Value>,
    pub r#avg_vertical_ratio: Option<Value>,
    pub r#avg_stance_time_balance: Option<Value>,
    pub r#avg_step_length: Option<Value>,
    pub r#total_anaerobic_training_effect: Option<Value>,
    pub r#avg_vam: Option<Value>,
    pub r#avg_depth: Option<Value>,
    pub r#max_depth: Option<Value>,
    pub r#surface_interval: Option<Value>,
    pub r#start_cns: Option<Value>,
    pub r#end_cns: Option<Value>,
    pub r#start_n2: Option<Value>,
    pub r#end_n2: Option<Value>,
    pub r#avg_respiration_rate: Option<Value>,
    pub r#max_respiration_rate: Option<Value>,
    pub r#min_respiration_rate: Option<Value>,
    pub r#min_temperature: Option<Value>,
    pub r#o2_toxicity: Option<Value>,
    pub r#dive_number: Option<Value>,
    pub r#training_load_peak: Option<Value>,
    pub r#enhanced_avg_respiration_rate: Option<Value>,
    pub r#enhanced_max_respiration_rate: Option<Value>,
    pub r#enhanced_min_respiration_rate: Option<Value>,
    pub r#total_grit: Option<Value>,
    pub r#total_flow: Option<Value>,
    pub r#jump_count: Option<Value>,
    pub r#avg_grit: Option<Value>,
    pub r#avg_flow: Option<Value>,
    pub r#avg_spo2: Option<Value>,
    pub r#avg_stress: Option<Value>,
    pub r#sdrr_hrv: Option<Value>,
    pub r#rmssd_hrv: Option<Value>,
    pub r#total_fractional_ascent: Option<Value>,
    pub r#total_fractional_descent: Option<Value>,
    pub r#avg_core_temperature: Option<Value>,
    pub r#min_core_temperature: Option<Value>,
    pub r#max_core_temperature: Option<Value>,
    pub r#timestamp: Option<Value>,
    pub r#message_index: Option<Value>,
    #[doc = r" All fields that are not defined in the profile."]
    pub unknown_fields: Vec<FitDataField>,
}
impl FitMessage for Session {
    const NAME: &'static str = "Session";
    const KIND: MesgNum = MesgNum::Session;
    fn parse(record: FitDataRecord) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#event = None;
        let mut r#event_type = None;
        let mut r#start_time = None;
        let mut r#start_position_lat = None;
        let mut r#start_position_long = None;
        let mut r#sport = None;
        let mut r#sub_sport = None;
        let mut r#total_elapsed_time = None;
        let mut r#total_timer_time = None;
        let mut r#total_distance = None;
        let mut r#total_cycles = None;
        let mut r#total_calories = None;
        let mut r#total_fat_calories = None;
        let mut r#avg_speed = None;
        let mut r#max_speed = None;
        let mut r#avg_heart_rate = None;
        let mut r#max_heart_rate = None;
        let mut r#avg_cadence = None;
        let mut r#max_cadence = None;
        let mut r#avg_power = None;
        let mut r#max_power = None;
        let mut r#total_ascent = None;
        let mut r#total_descent = None;
        let mut r#total_training_effect = None;
        let mut r#first_lap_index = None;
        let mut r#num_laps = None;
        let mut r#event_group = None;
        let mut r#trigger = None;
        let mut r#nec_lat = None;
        let mut r#nec_long = None;
        let mut r#swc_lat = None;
        let mut r#swc_long = None;
        let mut r#num_lengths = None;
        let mut r#normalized_power = None;
        let mut r#training_stress_score = None;
        let mut r#intensity_factor = None;
        let mut r#left_right_balance = None;
        let mut r#end_position_lat = None;
        let mut r#end_position_long = None;
        let mut r#avg_stroke_count = None;
        let mut r#avg_stroke_distance = None;
        let mut r#swim_stroke = None;
        let mut r#pool_length = None;
        let mut r#threshold_power = None;
        let mut r#pool_length_unit = None;
        let mut r#num_active_lengths = None;
        let mut r#total_work = None;
        let mut r#avg_altitude = None;
        let mut r#max_altitude = None;
        let mut r#gps_accuracy = None;
        let mut r#avg_grade = None;
        let mut r#avg_pos_grade = None;
        let mut r#avg_neg_grade = None;
        let mut r#max_pos_grade = None;
        let mut r#max_neg_grade = None;
        let mut r#avg_temperature = None;
        let mut r#max_temperature = None;
        let mut r#total_moving_time = None;
        let mut r#avg_pos_vertical_speed = None;
        let mut r#avg_neg_vertical_speed = None;
        let mut r#max_pos_vertical_speed = None;
        let mut r#max_neg_vertical_speed = None;
        let mut r#min_heart_rate = None;
        let mut r#time_in_hr_zone = None;
        let mut r#time_in_speed_zone = None;
        let mut r#time_in_cadence_zone = None;
        let mut r#time_in_power_zone = None;
        let mut r#avg_lap_time = None;
        let mut r#best_lap_index = None;
        let mut r#min_altitude = None;
        let mut r#player_score = None;
        let mut r#opponent_score = None;
        let mut r#opponent_name = None;
        let mut r#stroke_count = None;
        let mut r#zone_count = None;
        let mut r#max_ball_speed = None;
        let mut r#avg_ball_speed = None;
        let mut r#avg_vertical_oscillation = None;
        let mut r#avg_stance_time_percent = None;
        let mut r#avg_stance_time = None;
        let mut r#avg_fractional_cadence = None;
        let mut r#max_fractional_cadence = None;
        let mut r#total_fractional_cycles = None;
        let mut r#avg_total_hemoglobin_conc = None;
        let mut r#min_total_hemoglobin_conc = None;
        let mut r#max_total_hemoglobin_conc = None;
        let mut r#avg_saturated_hemoglobin_percent = None;
        let mut r#min_saturated_hemoglobin_percent = None;
        let mut r#max_saturated_hemoglobin_percent = None;
        let mut r#avg_left_torque_effectiveness = None;
        let mut r#avg_right_torque_effectiveness = None;
        let mut r#avg_left_pedal_smoothness = None;
        let mut r#avg_right_pedal_smoothness = None;
        let mut r#avg_combined_pedal_smoothness = None;
        let mut r#sport_profile_name = None;
        let mut r#sport_index = None;
        let mut r#time_standing = None;
        let mut r#stand_count = None;
        let mut r#avg_left_pco = None;
        let mut r#avg_right_pco = None;
        let mut r#avg_left_power_phase = None;
        let mut r#avg_left_power_phase_peak = None;
        let mut r#avg_right_power_phase = None;
        let mut r#avg_right_power_phase_peak = None;
        let mut r#avg_power_position = None;
        let mut r#max_power_position = None;
        let mut r#avg_cadence_position = None;
        let mut r#max_cadence_position = None;
        let mut r#enhanced_avg_speed = None;
        let mut r#enhanced_max_speed = None;
        let mut r#enhanced_avg_altitude = None;
        let mut r#enhanced_min_altitude = None;
        let mut r#enhanced_max_altitude = None;
        let mut r#avg_lev_motor_power = None;
        let mut r#max_lev_motor_power = None;
        let mut r#lev_battery_consumption = None;
        let mut r#avg_vertical_ratio = None;
        let mut r#avg_stance_time_balance = None;
        let mut r#avg_step_length = None;
        let mut r#total_anaerobic_training_effect = None;
        let mut r#avg_vam = None;
        let mut r#avg_depth = None;
        let mut r#max_depth = None;
        let mut r#surface_interval = None;
        let mut r#start_cns = None;
        let mut r#end_cns = None;
        let mut r#start_n2 = None;
        let mut r#end_n2 = None;
        let mut r#avg_respiration_rate = None;
        let mut r#max_respiration_rate = None;
        let mut r#min_respiration_rate = None;
        let mut r#min_temperature = None;
        let mut r#o2_toxicity = None;
        let mut r#dive_number = None;
        let mut r#training_load_peak = None;
        let mut r#enhanced_avg_respiration_rate = None;
        let mut r#enhanced_max_respiration_rate = None;
        let mut r#enhanced_min_respiration_rate = None;
        let mut r#total_grit = None;
        let mut r#total_flow = None;
        let mut r#jump_count = None;
        let mut r#avg_grit = None;
        let mut r#avg_flow = None;
        let mut r#avg_spo2 = None;
        let mut r#avg_stress = None;
        let mut r#sdrr_hrv = None;
        let mut r#rmssd_hrv = None;
        let mut r#total_fractional_ascent = None;
        let mut r#total_fractional_descent = None;
        let mut r#avg_core_temperature = None;
        let mut r#min_core_temperature = None;
        let mut r#max_core_temperature = None;
        let mut r#timestamp = None;
        let mut r#message_index = None;
        let mut unknown_fields = Vec::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => {
                    r#event = Some(field.into_value());
                }
                1u8 => {
                    r#event_type = Some(field.into_value());
                }
                2u8 => {
                    r#start_time = Some(field.into_value());
                }
                3u8 => {
                    r#start_position_lat = Some(field.into_value());
                }
                4u8 => {
                    r#start_position_long = Some(field.into_value());
                }
                5u8 => {
                    r#sport = Some(field.into_value());
                }
                6u8 => {
                    r#sub_sport = Some(field.into_value());
                }
                7u8 => {
                    r#total_elapsed_time = Some(field.into_value());
                }
                8u8 => {
                    r#total_timer_time = Some(field.into_value());
                }
                9u8 => {
                    r#total_distance = Some(field.into_value());
                }
                10u8 => {
                    r#total_cycles = Some(field.into_value());
                }
                11u8 => {
                    r#total_calories = Some(field.into_value());
                }
                13u8 => {
                    r#total_fat_calories = Some(field.into_value());
                }
                14u8 => {
                    r#avg_speed = Some(field.into_value());
                }
                15u8 => {
                    r#max_speed = Some(field.into_value());
                }
                16u8 => {
                    r#avg_heart_rate = Some(field.into_value());
                }
                17u8 => {
                    r#max_heart_rate = Some(field.into_value());
                }
                18u8 => {
                    r#avg_cadence = Some(field.into_value());
                }
                19u8 => {
                    r#max_cadence = Some(field.into_value());
                }
                20u8 => {
                    r#avg_power = Some(field.into_value());
                }
                21u8 => {
                    r#max_power = Some(field.into_value());
                }
                22u8 => {
                    r#total_ascent = Some(field.into_value());
                }
                23u8 => {
                    r#total_descent = Some(field.into_value());
                }
                24u8 => {
                    r#total_training_effect = Some(field.into_value());
                }
                25u8 => {
                    r#first_lap_index = Some(field.into_value());
                }
                26u8 => {
                    r#num_laps = Some(field.into_value());
                }
                27u8 => {
                    r#event_group = Some(field.into_value());
                }
                28u8 => {
                    r#trigger = Some(field.into_value());
                }
                29u8 => {
                    r#nec_lat = Some(field.into_value());
                }
                30u8 => {
                    r#nec_long = Some(field.into_value());
                }
                31u8 => {
                    r#swc_lat = Some(field.into_value());
                }
                32u8 => {
                    r#swc_long = Some(field.into_value());
                }
                33u8 => {
                    r#num_lengths = Some(field.into_value());
                }
                34u8 => {
                    r#normalized_power = Some(field.into_value());
                }
                35u8 => {
                    r#training_stress_score = Some(field.into_value());
                }
                36u8 => {
                    r#intensity_factor = Some(field.into_value());
                }
                37u8 => {
                    r#left_right_balance = Some(field.into_value());
                }
                38u8 => {
                    r#end_position_lat = Some(field.into_value());
                }
                39u8 => {
                    r#end_position_long = Some(field.into_value());
                }
                41u8 => {
                    r#avg_stroke_count = Some(field.into_value());
                }
                42u8 => {
                    r#avg_stroke_distance = Some(field.into_value());
                }
                43u8 => {
                    r#swim_stroke = Some(field.into_value());
                }
                44u8 => {
                    r#pool_length = Some(field.into_value());
                }
                45u8 => {
                    r#threshold_power = Some(field.into_value());
                }
                46u8 => {
                    r#pool_length_unit = Some(field.into_value());
                }
                47u8 => {
                    r#num_active_lengths = Some(field.into_value());
                }
                48u8 => {
                    r#total_work = Some(field.into_value());
                }
                49u8 => {
                    r#avg_altitude = Some(field.into_value());
                }
                50u8 => {
                    r#max_altitude = Some(field.into_value());
                }
                51u8 => {
                    r#gps_accuracy = Some(field.into_value());
                }
                52u8 => {
                    r#avg_grade = Some(field.into_value());
                }
                53u8 => {
                    r#avg_pos_grade = Some(field.into_value());
                }
                54u8 => {
                    r#avg_neg_grade = Some(field.into_value());
                }
                55u8 => {
                    r#max_pos_grade = Some(field.into_value());
                }
                56u8 => {
                    r#max_neg_grade = Some(field.into_value());
                }
                57u8 => {
                    r#avg_temperature = Some(field.into_value());
                }
                58u8 => {
                    r#max_temperature = Some(field.into_value());
                }
                59u8 => {
                    r#total_moving_time = Some(field.into_value());
                }
                60u8 => {
                    r#avg_pos_vertical_speed = Some(field.into_value());
                }
                61u8 => {
                    r#avg_neg_vertical_speed = Some(field.into_value());
                }
                62u8 => {
                    r#max_pos_vertical_speed = Some(field.into_value());
                }
                63u8 => {
                    r#max_neg_vertical_speed = Some(field.into_value());
                }
                64u8 => {
                    r#min_heart_rate = Some(field.into_value());
                }
                65u8 => {
                    r#time_in_hr_zone = Some(field.into_value());
                }
                66u8 => {
                    r#time_in_speed_zone = Some(field.into_value());
                }
                67u8 => {
                    r#time_in_cadence_zone = Some(field.into_value());
                }
                68u8 => {
                    r#time_in_power_zone = Some(field.into_value());
                }
                69u8 => {
                    r#avg_lap_time = Some(field.into_value());
                }
                70u8 => {
                    r#best_lap_index = Some(field.into_value());
                }
                71u8 => {
                    r#min_altitude = Some(field.into_value());
                }
                82u8 => {
                    r#player_score = Some(field.into_value());
                }
                83u8 => {
                    r#opponent_score = Some(field.into_value());
                }
                84u8 => {
                    r#opponent_name = Some(field.into_value());
                }
                85u8 => {
                    r#stroke_count = Some(field.into_value());
                }
                86u8 => {
                    r#zone_count = Some(field.into_value());
                }
                87u8 => {
                    r#max_ball_speed = Some(field.into_value());
                }
                88u8 => {
                    r#avg_ball_speed = Some(field.into_value());
                }
                89u8 => {
                    r#avg_vertical_oscillation = Some(field.into_value());
                }
                90u8 => {
                    r#avg_stance_time_percent = Some(field.into_value());
                }
                91u8 => {
                    r#avg_stance_time = Some(field.into_value());
                }
                92u8 => {
                    r#avg_fractional_cadence = Some(field.into_value());
                }
                93u8 => {
                    r#max_fractional_cadence = Some(field.into_value());
                }
                94u8 => {
                    r#total_fractional_cycles = Some(field.into_value());
                }
                95u8 => {
                    r#avg_total_hemoglobin_conc = Some(field.into_value());
                }
                96u8 => {
                    r#min_total_hemoglobin_conc = Some(field.into_value());
                }
                97u8 => {
                    r#max_total_hemoglobin_conc = Some(field.into_value());
                }
                98u8 => {
                    r#avg_saturated_hemoglobin_percent = Some(field.into_value());
                }
                99u8 => {
                    r#min_saturated_hemoglobin_percent = Some(field.into_value());
                }
                100u8 => {
                    r#max_saturated_hemoglobin_percent = Some(field.into_value());
                }
                101u8 => {
                    r#avg_left_torque_effectiveness = Some(field.into_value());
                }
                102u8 => {
                    r#avg_right_torque_effectiveness = Some(field.into_value());
                }
                103u8 => {
                    r#avg_left_pedal_smoothness = Some(field.into_value());
                }
                104u8 => {
                    r#avg_right_pedal_smoothness = Some(field.into_value());
                }
                105u8 => {
                    r#avg_combined_pedal_smoothness = Some(field.into_value());
                }
                110u8 => {
                    r#sport_profile_name = Some(field.into_value());
                }
                111u8 => {
                    r#sport_index = Some(field.into_value());
                }
                112u8 => {
                    r#time_standing = Some(field.into_value());
                }
                113u8 => {
                    r#stand_count = Some(field.into_value());
                }
                114u8 => {
                    r#avg_left_pco = Some(field.into_value());
                }
                115u8 => {
                    r#avg_right_pco = Some(field.into_value());
                }
                116u8 => {
                    r#avg_left_power_phase = Some(field.into_value());
                }
                117u8 => {
                    r#avg_left_power_phase_peak = Some(field.into_value());
                }
                118u8 => {
                    r#avg_right_power_phase = Some(field.into_value());
                }
                119u8 => {
                    r#avg_right_power_phase_peak = Some(field.into_value());
                }
                120u8 => {
                    r#avg_power_position = Some(field.into_value());
                }
                121u8 => {
                    r#max_power_position = Some(field.into_value());
                }
                122u8 => {
                    r#avg_cadence_position = Some(field.into_value());
                }
                123u8 => {
                    r#max_cadence_position = Some(field.into_value());
                }
                124u8 => {
                    r#enhanced_avg_speed = Some(field.into_value());
                }
                125u8 => {
                    r#enhanced_max_speed = Some(field.into_value());
                }
                126u8 => {
                    r#enhanced_avg_altitude = Some(field.into_value());
                }
                127u8 => {
                    r#enhanced_min_altitude = Some(field.into_value());
                }
                128u8 => {
                    r#enhanced_max_altitude = Some(field.into_value());
                }
                129u8 => {
                    r#avg_lev_motor_power = Some(field.into_value());
                }
                130u8 => {
                    r#max_lev_motor_power = Some(field.into_value());
                }
                131u8 => {
                    r#lev_battery_consumption = Some(field.into_value());
                }
                132u8 => {
                    r#avg_vertical_ratio = Some(field.into_value());
                }
                133u8 => {
                    r#avg_stance_time_balance = Some(field.into_value());
                }
                134u8 => {
                    r#avg_step_length = Some(field.into_value());
                }
                137u8 => {
                    r#total_anaerobic_training_effect = Some(field.into_value());
                }
                139u8 => {
                    r#avg_vam = Some(field.into_value());
                }
                140u8 => {
                    r#avg_depth = Some(field.into_value());
                }
                141u8 => {
                    r#max_depth = Some(field.into_value());
                }
                142u8 => {
                    r#surface_interval = Some(field.into_value());
                }
                143u8 => {
                    r#start_cns = Some(field.into_value());
                }
                144u8 => {
                    r#end_cns = Some(field.into_value());
                }
                145u8 => {
                    r#start_n2 = Some(field.into_value());
                }
                146u8 => {
                    r#end_n2 = Some(field.into_value());
                }
                147u8 => {
                    r#avg_respiration_rate = Some(field.into_value());
                }
                148u8 => {
                    r#max_respiration_rate = Some(field.into_value());
                }
                149u8 => {
                    r#min_respiration_rate = Some(field.into_value());
                }
                150u8 => {
                    r#min_temperature = Some(field.into_value());
                }
                155u8 => {
                    r#o2_toxicity = Some(field.into_value());
                }
                156u8 => {
                    r#dive_number = Some(field.into_value());
                }
                168u8 => {
                    r#training_load_peak = Some(field.into_value());
                }
                169u8 => {
                    r#enhanced_avg_respiration_rate = Some(field.into_value());
                }
                170u8 => {
                    r#enhanced_max_respiration_rate = Some(field.into_value());
                }
                180u8 => {
                    r#enhanced_min_respiration_rate = Some(field.into_value());
                }
                181u8 => {
                    r#total_grit = Some(field.into_value());
                }
                182u8 => {
                    r#total_flow = Some(field.into_value());
                }
                183u8 => {
                    r#jump_count = Some(field.into_value());
                }
                186u8 => {
                    r#avg_grit = Some(field.into_value());
                }
                187u8 => {
                    r#avg_flow = Some(field.into_value());
                }
                194u8 => {
                    r#avg_spo2 = Some(field.into_value());
                }
                195u8 => {
                    r#avg_stress = Some(field.into_value());
                }
                197u8 => {
                    r#sdrr_hrv = Some(field.into_value());
                }
                198u8 => {
                    r#rmssd_hrv = Some(field.into_value());
                }
                199u8 => {
                    r#total_fractional_ascent = Some(field.into_value());
                }
                200u8 => {
                    r#total_fractional_descent = Some(field.into_value());
                }
                208u8 => {
                    r#avg_core_temperature = Some(field.into_value());
                }
                209u8 => {
                    r#min_core_temperature = Some(field.into_value());
                }
                210u8 => {
                    r#max_core_temperature = Some(field.into_value());
                }
                253u8 => {
                    r#timestamp = Some(field.into_value());
                }
                254u8 => {
                    r#message_index = Some(field.into_value());
                }
                _ => {
                    unknown_fields.push(field);
                }
            }
        }
        Ok(Self {
            r#event,
            r#event_type,
            r#start_time,
            r#start_position_lat,
            r#start_position_long,
            r#sport,
            r#sub_sport,
            r#total_elapsed_time,
            r#total_timer_time,
            r#total_distance,
            r#total_cycles,
            r#total_calories,
            r#total_fat_calories,
            r#avg_speed,
            r#max_speed,
            r#avg_heart_rate,
            r#max_heart_rate,
            r#avg_cadence,
            r#max_cadence,
            r#avg_power,
            r#max_power,
            r#total_ascent,
            r#total_descent,
            r#total_training_effect,
            r#first_lap_index,
            r#num_laps,
            r#event_group,
            r#trigger,
            r#nec_lat,
            r#nec_long,
            r#swc_lat,
            r#swc_long,
            r#num_lengths,
            r#normalized_power,
            r#training_stress_score,
            r#intensity_factor,
            r#left_right_balance,
            r#end_position_lat,
            r#end_position_long,
            r#avg_stroke_count,
            r#avg_stroke_distance,
            r#swim_stroke,
            r#pool_length,
            r#threshold_power,
            r#pool_length_unit,
            r#num_active_lengths,
            r#total_work,
            r#avg_altitude,
            r#max_altitude,
            r#gps_accuracy,
            r#avg_grade,
            r#avg_pos_grade,
            r#avg_neg_grade,
            r#max_pos_grade,
            r#max_neg_grade,
            r#avg_temperature,
            r#max_temperature,
            r#total_moving_time,
            r#avg_pos_vertical_speed,
            r#avg_neg_vertical_speed,
            r#max_pos_vertical_speed,
            r#max_neg_vertical_speed,
            r#min_heart_rate,
            r#time_in_hr_zone,
            r#time_in_speed_zone,
            r#time_in_cadence_zone,
            r#time_in_power_zone,
            r#avg_lap_time,
            r#best_lap_index,
            r#min_altitude,
            r#player_score,
            r#opponent_score,
            r#opponent_name,
            r#stroke_count,
            r#zone_count,
            r#max_ball_speed,
            r#avg_ball_speed,
            r#avg_vertical_oscillation,
            r#avg_stance_time_percent,
            r#avg_stance_time,
            r#avg_fractional_cadence,
            r#max_fractional_cadence,
            r#total_fractional_cycles,
            r#avg_total_hemoglobin_conc,
            r#min_total_hemoglobin_conc,
            r#max_total_hemoglobin_conc,
            r#avg_saturated_hemoglobin_percent,
            r#min_saturated_hemoglobin_percent,
            r#max_saturated_hemoglobin_percent,
            r#avg_left_torque_effectiveness,
            r#avg_right_torque_effectiveness,
            r#avg_left_pedal_smoothness,
            r#avg_right_pedal_smoothness,
            r#avg_combined_pedal_smoothness,
            r#sport_profile_name,
            r#sport_index,
            r#time_standing,
            r#stand_count,
            r#avg_left_pco,
            r#avg_right_pco,
            r#avg_left_power_phase,
            r#avg_left_power_phase_peak,
            r#avg_right_power_phase,
            r#avg_right_power_phase_peak,
            r#avg_power_position,
            r#max_power_position,
            r#avg_cadence_position,
            r#max_cadence_position,
            r#enhanced_avg_speed,
            r#enhanced_max_speed,
            r#enhanced_avg_altitude,
            r#enhanced_min_altitude,
            r#enhanced_max_altitude,
            r#avg_lev_motor_power,
            r#max_lev_motor_power,
            r#lev_battery_consumption,
            r#avg_vertical_ratio,
            r#avg_stance_time_balance,
            r#avg_step_length,
            r#total_anaerobic_training_effect,
            r#avg_vam,
            r#avg_depth,
            r#max_depth,
            r#surface_interval,
            r#start_cns,
            r#end_cns,
            r#start_n2,
            r#end_n2,
            r#avg_respiration_rate,
            r#max_respiration_rate,
            r#min_respiration_rate,
            r#min_temperature,
            r#o2_toxicity,
            r#dive_number,
            r#training_load_peak,
            r#enhanced_avg_respiration_rate,
            r#enhanced_max_respiration_rate,
            r#enhanced_min_respiration_rate,
            r#total_grit,
            r#total_flow,
            r#jump_count,
            r#avg_grit,
            r#avg_flow,
            r#avg_spo2,
            r#avg_stress,
            r#sdrr_hrv,
            r#rmssd_hrv,
            r#total_fractional_ascent,
            r#total_fractional_descent,
            r#avg_core_temperature,
            r#min_core_temperature,
            r#max_core_temperature,
            r#timestamp,
            r#message_index,
            unknown_fields,
        })
    }
}
impl TryFrom<FitDataRecord> for Session {
    type Error = TryFromRecordError;
    fn try_from(record: FitDataRecord) -> Result<Self, Self::Error> {
        Self::parse(record)
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize)]
pub struct Lap {
    pub r#event: Option<Value>,
    pub r#event_type: Option<Value>,
    pub r#start_time: Option<Value>,
    pub r#start_position_lat: Option<Value>,
    pub r#start_position_long: Option<Value>,
    pub r#end_position_lat: Option<Value>,
    pub r#end_position_long: Option<Value>,
    pub r#total_elapsed_time: Option<Value>,
    pub r#total_timer_time: Option<Value>,
    pub r#total_distance: Option<Value>,
    pub r#total_cycles: Option<Value>,
    pub r#total_calories: Option<Value>,
    pub r#total_fat_calories: Option<Value>,
    pub r#avg_speed: Option<Value>,
    pub r#max_speed: Option<Value>,
    pub r#avg_heart_rate: Option<Value>,
    pub r#max_heart_rate: Option<Value>,
    pub r#avg_cadence: Option<Value>,
    pub r#max_cadence: Option<Value>,
    pub r#avg_power: Option<Value>,
    pub r#max_power: Option<Value>,
    pub r#total_ascent: Option<Value>,
    pub r#total_descent: Option<Value>,
    pub r#intensity: Option<Value>,
    pub r#lap_trigger: Option<Value>,
    pub r#sport: Option<Value>,
    pub r#event_group: Option<Value>,
    pub r#num_lengths: Option<Value>,
    pub r#normalized_power: Option<Value>,
    pub r#left_right_balance: Option<Value>,
    pub r#first_length_index: Option<Value>,
    pub r#avg_stroke_distance: Option<Value>,
    pub r#swim_stroke: Option<Value>,
    pub r#sub_sport: Option<Value>,
    pub r#num_active_lengths: Option<Value>,
    pub r#total_work: Option<Value>,
    pub r#avg_altitude: Option<Value>,
    pub r#max_altitude: Option<Value>,
    pub r#gps_accuracy: Option<Value>,
    pub r#avg_grade: Option<Value>,
    pub r#avg_pos_grade: Option<Value>,
    pub r#avg_neg_grade: Option<Value>,
    pub r#max_pos_grade: Option<Value>,
    pub r#max_neg_grade: Option<Value>,
    pub r#avg_temperature: Option<Value>,
    pub r#max_temperature: Option<Value>,
    pub r#total_moving_time: Option<Value>,
    pub r#avg_pos_vertical_speed: Option<Value>,
    pub r#avg_neg_vertical_speed: Option<Value>,
    pub r#max_pos_vertical_speed: Option<Value>,
    pub r#max_neg_vertical_speed: Option<Value>,
    pub r#time_in_hr_zone: Option<Value>,
    pub r#time_in_speed_zone: Option<Value>,
    pub r#time_in_cadence_zone: Option<Value>,
    pub r#time_in_power_zone: Option<Value>,
    pub r#repetition_num: Option<Value>,
    pub r#min_altitude: Option<Value>,
    pub r#min_heart_rate: Option<Value>,
    pub r#wkt_step_index: Option<Value>,
    pub r#opponent_score: Option<Value>,
    pub r#stroke_count: Option<Value>,
    pub r#zone_count: Option<Value>,
    pub r#avg_vertical_oscillation: Option<Value>,
    pub r#avg_stance_time_percent: Option<Value>,
    pub r#avg_stance_time: Option<Value>,
    pub r#avg_fractional_cadence: Option<Value>,
    pub r#max_fractional_cadence: Option<Value>,
    pub r#total_fractional_cycles: Option<Value>,
    pub r#player_score: Option<Value>,
    pub r#avg_total_hemoglobin_conc: Option<Value>,
    pub r#min_total_hemoglobin_conc: Option<Value>,
    pub r#max_total_hemoglobin_conc: Option<Value>,
    pub r#avg_saturated_hemoglobin_percent: Option<Value>,
    pub r#min_saturated_hemoglobin_percent: Option<Value>,
    pub r#max_saturated_hemoglobin_percent: Option<Value>,
    pub r#avg_left_torque_effectiveness: Option<Value>,
    pub r#avg_right_torque_effectiveness: Option<Value>,
    pub r#avg_left_pedal_smoothness: Option<Value>,
    pub r#avg_right_pedal_smoothness: Option<Value>,
    pub r#avg_combined_pedal_smoothness: Option<Value>,
    pub r#time_standing: Option<Value>,
    pub r#stand_count: Option<Value>,
    pub r#avg_left_pco: Option<Value>,
    pub r#avg_right_pco: Option<Value>,
    pub r#avg_left_power_phase: Option<Value>,
    pub r#avg_left_power_phase_peak: Option<Value>,
    pub r#avg_right_power_phase: Option<Value>,
    pub r#avg_right_power_phase_peak: Option<Value>,
    pub r#avg_power_position: Option<Value>,
    pub r#max_power_position: Option<Value>,
    pub r#avg_cadence_position: Option<Value>,
    pub r#max_cadence_position: Option<Value>,
    pub r#enhanced_avg_speed: Option<Value>,
    pub r#enhanced_max_speed: Option<Value>,
    pub r#enhanced_avg_altitude: Option<Value>,
    pub r#enhanced_min_altitude: Option<Value>,
    pub r#enhanced_max_altitude: Option<Value>,
    pub r#avg_lev_motor_power: Option<Value>,
    pub r#max_lev_motor_power: Option<Value>,
    pub r#lev_battery_consumption: Option<Value>,
    pub r#avg_vertical_ratio: Option<Value>,
    pub r#avg_stance_time_balance: Option<Value>,
    pub r#avg_step_length: Option<Value>,
    pub r#avg_vam: Option<Value>,
    pub r#avg_depth: Option<Value>,
    pub r#max_depth: Option<Value>,
    pub r#min_temperature: Option<Value>,
    pub r#enhanced_avg_respiration_rate: Option<Value>,
    pub r#enhanced_max_respiration_rate: Option<Value>,
    pub r#avg_respiration_rate: Option<Value>,
    pub r#max_respiration_rate: Option<Value>,
    pub r#total_grit: Option<Value>,
    pub r#total_flow: Option<Value>,
    pub r#jump_count: Option<Value>,
    pub r#avg_grit: Option<Value>,
    pub r#avg_flow: Option<Value>,
    pub r#total_fractional_ascent: Option<Value>,
    pub r#total_fractional_descent: Option<Value>,
    pub r#avg_core_temperature: Option<Value>,
    pub r#min_core_temperature: Option<Value>,
    pub r#max_core_temperature: Option<Value>,
    pub r#timestamp: Option<Value>,
    pub r#message_index: Option<Value>,
    #[doc = r" All fields that are not defined in the profile."]
    pub unknown_fields: Vec<FitDataField>,
}
impl FitMessage for Lap {
    const NAME: &'static str = "Lap";
    const KIND: MesgNum = MesgNum::Lap;
    fn parse(record: FitDataRecord) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#event = None;
        let mut r#event_type = None;
        let mut r#start_time = None;
        let mut r#start_position_lat = None;
        let mut r#start_position_long = None;
        let mut r#end_position_lat = None;
        let mut r#end_position_long = None;
        let mut r#total_elapsed_time = None;
        let mut r#total_timer_time = None;
        let mut r#total_distance = None;
        let mut r#total_cycles = None;
        let mut r#total_calories = None;
        let mut r#total_fat_calories = None;
        let mut r#avg_speed = None;
        let mut r#max_speed = None;
        let mut r#avg_heart_rate = None;
        let mut r#max_heart_rate = None;
        let mut r#avg_cadence = None;
        let mut r#max_cadence = None;
        let mut r#avg_power = None;
        let mut r#max_power = None;
        let mut r#total_ascent = None;
        let mut r#total_descent = None;
        let mut r#intensity = None;
        let mut r#lap_trigger = None;
        let mut r#sport = None;
        let mut r#event_group = None;
        let mut r#num_lengths = None;
        let mut r#normalized_power = None;
        let mut r#left_right_balance = None;
        let mut r#first_length_index = None;
        let mut r#avg_stroke_distance = None;
        let mut r#swim_stroke = None;
        let mut r#sub_sport = None;
        let mut r#num_active_lengths = None;
        let mut r#total_work = None;
        let mut r#avg_altitude = None;
        let mut r#max_altitude = None;
        let mut r#gps_accuracy = None;
        let mut r#avg_grade = None;
        let mut r#avg_pos_grade = None;
        let mut r#avg_neg_grade = None;
        let mut r#max_pos_grade = None;
        let mut r#max_neg_grade = None;
        let mut r#avg_temperature = None;
        let mut r#max_temperature = None;
        let mut r#total_moving_time = None;
        let mut r#avg_pos_vertical_speed = None;
        let mut r#avg_neg_vertical_speed = None;
        let mut r#max_pos_vertical_speed = None;
        let mut r#max_neg_vertical_speed = None;
        let mut r#time_in_hr_zone = None;
        let mut r#time_in_speed_zone = None;
        let mut r#time_in_cadence_zone = None;
        let mut r#time_in_power_zone = None;
        let mut r#repetition_num = None;
        let mut r#min_altitude = None;
        let mut r#min_heart_rate = None;
        let mut r#wkt_step_index = None;
        let mut r#opponent_score = None;
        let mut r#stroke_count = None;
        let mut r#zone_count = None;
        let mut r#avg_vertical_oscillation = None;
        let mut r#avg_stance_time_percent = None;
        let mut r#avg_stance_time = None;
        let mut r#avg_fractional_cadence = None;
        let mut r#max_fractional_cadence = None;
        let mut r#total_fractional_cycles = None;
        let mut r#player_score = None;
        let mut r#avg_total_hemoglobin_conc = None;
        let mut r#min_total_hemoglobin_conc = None;
        let mut r#max_total_hemoglobin_conc = None;
        let mut r#avg_saturated_hemoglobin_percent = None;
        let mut r#min_saturated_hemoglobin_percent = None;
        let mut r#max_saturated_hemoglobin_percent = None;
        let mut r#avg_left_torque_effectiveness = None;
        let mut r#avg_right_torque_effectiveness = None;
        let mut r#avg_left_pedal_smoothness = None;
        let mut r#avg_right_pedal_smoothness = None;
        let mut r#avg_combined_pedal_smoothness = None;
        let mut r#time_standing = None;
        let mut r#stand_count = None;
        let mut r#avg_left_pco = None;
        let mut r#avg_right_pco = None;
        let mut r#avg_left_power_phase = None;
        let mut r#avg_left_power_phase_peak = None;
        let mut r#avg_right_power_phase = None;
        let mut r#avg_right_power_phase_peak = None;
        let mut r#avg_power_position = None;
        let mut r#max_power_position = None;
        let mut r#avg_cadence_position = None;
        let mut r#max_cadence_position = None;
        let mut r#enhanced_avg_speed = None;
        let mut r#enhanced_max_speed = None;
        let mut r#enhanced_avg_altitude = None;
        let mut r#enhanced_min_altitude = None;
        let mut r#enhanced_max_altitude = None;
        let mut r#avg_lev_motor_power = None;
        let mut r#max_lev_motor_power = None;
        let mut r#lev_battery_consumption = None;
        let mut r#avg_vertical_ratio = None;
        let mut r#avg_stance_time_balance = None;
        let mut r#avg_step_length = None;
        let mut r#avg_vam = None;
        let mut r#avg_depth = None;
        let mut r#max_depth = None;
        let mut r#min_temperature = None;
        let mut r#enhanced_avg_respiration_rate = None;
        let mut r#enhanced_max_respiration_rate = None;
        let mut r#avg_respiration_rate = None;
        let mut r#max_respiration_rate = None;
        let mut r#total_grit = None;
        let mut r#total_flow = None;
        let mut r#jump_count = None;
        let mut r#avg_grit = None;
        let mut r#avg_flow = None;
        let mut r#total_fractional_ascent = None;
        let mut r#total_fractional_descent = None;
        let mut r#avg_core_temperature = None;
        let mut r#min_core_temperature = None;
        let mut r#max_core_temperature = None;
        let mut r#timestamp = None;
        let mut r#message_index = None;
        let mut unknown_fields = Vec::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => {
                    r#event = Some(field.into_value());
                }
                1u8 => {
                    r#event_type = Some(field.into_value());
                }
                2u8 => {
                    r#start_time = Some(field.into_value());
                }
                3u8 => {
                    r#start_position_lat = Some(field.into_value());
                }
                4u8 => {
                    r#start_position_long = Some(field.into_value());
                }
                5u8 => {
                    r#end_position_lat = Some(field.into_value());
                }
                6u8 => {
                    r#end_position_long = Some(field.into_value());
                }
                7u8 => {
                    r#total_elapsed_time = Some(field.into_value());
                }
                8u8 => {
                    r#total_timer_time = Some(field.into_value());
                }
                9u8 => {
                    r#total_distance = Some(field.into_value());
                }
                10u8 => {
                    r#total_cycles = Some(field.into_value());
                }
                11u8 => {
                    r#total_calories = Some(field.into_value());
                }
                12u8 => {
                    r#total_fat_calories = Some(field.into_value());
                }
                13u8 => {
                    r#avg_speed = Some(field.into_value());
                }
                14u8 => {
                    r#max_speed = Some(field.into_value());
                }
                15u8 => {
                    r#avg_heart_rate = Some(field.into_value());
                }
                16u8 => {
                    r#max_heart_rate = Some(field.into_value());
                }
                17u8 => {
                    r#avg_cadence = Some(field.into_value());
                }
                18u8 => {
                    r#max_cadence = Some(field.into_value());
                }
                19u8 => {
                    r#avg_power = Some(field.into_value());
                }
                20u8 => {
                    r#max_power = Some(field.into_value());
                }
                21u8 => {
                    r#total_ascent = Some(field.into_value());
                }
                22u8 => {
                    r#total_descent = Some(field.into_value());
                }
                23u8 => {
                    r#intensity = Some(field.into_value());
                }
                24u8 => {
                    r#lap_trigger = Some(field.into_value());
                }
                25u8 => {
                    r#sport = Some(field.into_value());
                }
                26u8 => {
                    r#event_group = Some(field.into_value());
                }
                32u8 => {
                    r#num_lengths = Some(field.into_value());
                }
                33u8 => {
                    r#normalized_power = Some(field.into_value());
                }
                34u8 => {
                    r#left_right_balance = Some(field.into_value());
                }
                35u8 => {
                    r#first_length_index = Some(field.into_value());
                }
                37u8 => {
                    r#avg_stroke_distance = Some(field.into_value());
                }
                38u8 => {
                    r#swim_stroke = Some(field.into_value());
                }
                39u8 => {
                    r#sub_sport = Some(field.into_value());
                }
                40u8 => {
                    r#num_active_lengths = Some(field.into_value());
                }
                41u8 => {
                    r#total_work = Some(field.into_value());
                }
                42u8 => {
                    r#avg_altitude = Some(field.into_value());
                }
                43u8 => {
                    r#max_altitude = Some(field.into_value());
                }
                44u8 => {
                    r#gps_accuracy = Some(field.into_value());
                }
                45u8 => {
                    r#avg_grade = Some(field.into_value());
                }
                46u8 => {
                    r#avg_pos_grade = Some(field.into_value());
                }
                47u8 => {
                    r#avg_neg_grade = Some(field.into_value());
                }
                48u8 => {
                    r#max_pos_grade = Some(field.into_value());
                }
                49u8 => {
                    r#max_neg_grade = Some(field.into_value());
                }
                50u8 => {
                    r#avg_temperature = Some(field.into_value());
                }
                51u8 => {
                    r#max_temperature = Some(field.into_value());
                }
                52u8 => {
                    r#total_moving_time = Some(field.into_value());
                }
                53u8 => {
                    r#avg_pos_vertical_speed = Some(field.into_value());
                }
                54u8 => {
                    r#avg_neg_vertical_speed = Some(field.into_value());
                }
                55u8 => {
                    r#max_pos_vertical_speed = Some(field.into_value());
                }
                56u8 => {
                    r#max_neg_vertical_speed = Some(field.into_value());
                }
                57u8 => {
                    r#time_in_hr_zone = Some(field.into_value());
                }
                58u8 => {
                    r#time_in_speed_zone = Some(field.into_value());
                }
                59u8 => {
                    r#time_in_cadence_zone = Some(field.into_value());
                }
                60u8 => {
                    r#time_in_power_zone = Some(field.into_value());
                }
                61u8 => {
                    r#repetition_num = Some(field.into_value());
                }
                62u8 => {
                    r#min_altitude = Some(field.into_value());
                }
                63u8 => {
                    r#min_heart_rate = Some(field.into_value());
                }
                71u8 => {
                    r#wkt_step_index = Some(field.into_value());
                }
                74u8 => {
                    r#opponent_score = Some(field.into_value());
                }
                75u8 => {
                    r#stroke_count = Some(field.into_value());
                }
                76u8 => {
                    r#zone_count = Some(field.into_value());
                }
                77u8 => {
                    r#avg_vertical_oscillation = Some(field.into_value());
                }
                78u8 => {
                    r#avg_stance_time_percent = Some(field.into_value());
                }
                79u8 => {
                    r#avg_stance_time = Some(field.into_value());
                }
                80u8 => {
                    r#avg_fractional_cadence = Some(field.into_value());
                }
                81u8 => {
                    r#max_fractional_cadence = Some(field.into_value());
                }
                82u8 => {
                    r#total_fractional_cycles = Some(field.into_value());
                }
                83u8 => {
                    r#player_score = Some(field.into_value());
                }
                84u8 => {
                    r#avg_total_hemoglobin_conc = Some(field.into_value());
                }
                85u8 => {
                    r#min_total_hemoglobin_conc = Some(field.into_value());
                }
                86u8 => {
                    r#max_total_hemoglobin_conc = Some(field.into_value());
                }
                87u8 => {
                    r#avg_saturated_hemoglobin_percent = Some(field.into_value());
                }
                88u8 => {
                    r#min_saturated_hemoglobin_percent = Some(field.into_value());
                }
                89u8 => {
                    r#max_saturated_hemoglobin_percent = Some(field.into_value());
                }
                91u8 => {
                    r#avg_left_torque_effectiveness = Some(field.into_value());
                }
                92u8 => {
                    r#avg_right_torque_effectiveness = Some(field.into_value());
                }
                93u8 => {
                    r#avg_left_pedal_smoothness = Some(field.into_value());
                }
                94u8 => {
                    r#avg_right_pedal_smoothness = Some(field.into_value());
                }
                95u8 => {
                    r#avg_combined_pedal_smoothness = Some(field.into_value());
                }
                98u8 => {
                    r#time_standing = Some(field.into_value());
                }
                99u8 => {
                    r#stand_count = Some(field.into_value());
                }
                100u8 => {
                    r#avg_left_pco = Some(field.into_value());
                }
                101u8 => {
                    r#avg_right_pco = Some(field.into_value());
                }
                102u8 => {
                    r#avg_left_power_phase = Some(field.into_value());
                }
                103u8 => {
                    r#avg_left_power_phase_peak = Some(field.into_value());
                }
                104u8 => {
                    r#avg_right_power_phase = Some(field.into_value());
                }
                105u8 => {
                    r#avg_right_power_phase_peak = Some(field.into_value());
                }
                106u8 => {
                    r#avg_power_position = Some(field.into_value());
                }
                107u8 => {
                    r#max_power_position = Some(field.into_value());
                }
                108u8 => {
                    r#avg_cadence_position = Some(field.into_value());
                }
                109u8 => {
                    r#max_cadence_position = Some(field.into_value());
                }
                110u8 => {
                    r#enhanced_avg_speed = Some(field.into_value());
                }
                111u8 => {
                    r#enhanced_max_speed = Some(field.into_value());
                }
                112u8 => {
                    r#enhanced_avg_altitude = Some(field.into_value());
                }
                113u8 => {
                    r#enhanced_min_altitude = Some(field.into_value());
                }
                114u8 => {
                    r#enhanced_max_altitude = Some(field.into_value());
                }
                115u8 => {
                    r#avg_lev_motor_power = Some(field.into_value());
                }
                116u8 => {
                    r#max_lev_motor_power = Some(field.into_value());
                }
                117u8 => {
                    r#lev_battery_consumption = Some(field.into_value());
                }
                118u8 => {
                    r#avg_vertical_ratio = Some(field.into_value());
                }
                119u8 => {
                    r#avg_stance_time_balance = Some(field.into_value());
                }
                120u8 => {
                    r#avg_step_length = Some(field.into_value());
                }
                121u8 => {
                    r#avg_vam = Some(field.into_value());
                }
                122u8 => {
                    r#avg_depth = Some(field.into_value());
                }
                123u8 => {
                    r#max_depth = Some(field.into_value());
                }
                124u8 => {
                    r#min_temperature = Some(field.into_value());
                }
                136u8 => {
                    r#enhanced_avg_respiration_rate = Some(field.into_value());
                }
                137u8 => {
                    r#enhanced_max_respiration_rate = Some(field.into_value());
                }
                147u8 => {
                    r#avg_respiration_rate = Some(field.into_value());
                }
                148u8 => {
                    r#max_respiration_rate = Some(field.into_value());
                }
                149u8 => {
                    r#total_grit = Some(field.into_value());
                }
                150u8 => {
                    r#total_flow = Some(field.into_value());
                }
                151u8 => {
                    r#jump_count = Some(field.into_value());
                }
                153u8 => {
                    r#avg_grit = Some(field.into_value());
                }
                154u8 => {
                    r#avg_flow = Some(field.into_value());
                }
                156u8 => {
                    r#total_fractional_ascent = Some(field.into_value());
                }
                157u8 => {
                    r#total_fractional_descent = Some(field.into_value());
                }
                158u8 => {
                    r#avg_core_temperature = Some(field.into_value());
                }
                159u8 => {
                    r#min_core_temperature = Some(field.into_value());
                }
                160u8 => {
                    r#max_core_temperature = Some(field.into_value());
                }
                253u8 => {
                    r#timestamp = Some(field.into_value());
                }
                254u8 => {
                    r#message_index = Some(field.into_value());
                }
                _ => {
                    unknown_fields.push(field);
                }
            }
        }
        Ok(Self {
            r#event,
            r#event_type,
            r#start_time,
            r#start_position_lat,
            r#start_position_long,
            r#end_position_lat,
            r#end_position_long,
            r#total_elapsed_time,
            r#total_timer_time,
            r#total_distance,
            r#total_cycles,
            r#total_calories,
            r#total_fat_calories,
            r#avg_speed,
            r#max_speed,
            r#avg_heart_rate,
            r#max_heart_rate,
            r#avg_cadence,
            r#max_cadence,
            r#avg_power,
            r#max_power,
            r#total_ascent,
            r#total_descent,
            r#intensity,
            r#lap_trigger,
            r#sport,
            r#event_group,
            r#num_lengths,
            r#normalized_power,
            r#left_right_balance,
            r#first_length_index,
            r#avg_stroke_distance,
            r#swim_stroke,
            r#sub_sport,
            r#num_active_lengths,
            r#total_work,
            r#avg_altitude,
            r#max_altitude,
            r#gps_accuracy,
            r#avg_grade,
            r#avg_pos_grade,
            r#avg_neg_grade,
            r#max_pos_grade,
            r#max_neg_grade,
            r#avg_temperature,
            r#max_temperature,
            r#total_moving_time,
            r#avg_pos_vertical_speed,
            r#avg_neg_vertical_speed,
            r#max_pos_vertical_speed,
            r#max_neg_vertical_speed,
            r#time_in_hr_zone,
            r#time_in_speed_zone,
            r#time_in_cadence_zone,
            r#time_in_power_zone,
            r#repetition_num,
            r#min_altitude,
            r#min_heart_rate,
            r#wkt_step_index,
            r#opponent_score,
            r#stroke_count,
            r#zone_count,
            r#avg_vertical_oscillation,
            r#avg_stance_time_percent,
            r#avg_stance_time,
            r#avg_fractional_cadence,
            r#max_fractional_cadence,
            r#total_fractional_cycles,
            r#player_score,
            r#avg_total_hemoglobin_conc,
            r#min_total_hemoglobin_conc,
            r#max_total_hemoglobin_conc,
            r#avg_saturated_hemoglobin_percent,
            r#min_saturated_hemoglobin_percent,
            r#max_saturated_hemoglobin_percent,
            r#avg_left_torque_effectiveness,
            r#avg_right_torque_effectiveness,
            r#avg_left_pedal_smoothness,
            r#avg_right_pedal_smoothness,
            r#avg_combined_pedal_smoothness,
            r#time_standing,
            r#stand_count,
            r#avg_left_pco,
            r#avg_right_pco,
            r#avg_left_power_phase,
            r#avg_left_power_phase_peak,
            r#avg_right_power_phase,
            r#avg_right_power_phase_peak,
            r#avg_power_position,
            r#max_power_position,
            r#avg_cadence_position,
            r#max_cadence_position,
            r#enhanced_avg_speed,
            r#enhanced_max_speed,
            r#enhanced_avg_altitude,
            r#enhanced_min_altitude,
            r#enhanced_max_altitude,
            r#avg_lev_motor_power,
            r#max_lev_motor_power,
            r#lev_battery_consumption,
            r#avg_vertical_ratio,
            r#avg_stance_time_balance,
            r#avg_step_length,
            r#avg_vam,
            r#avg_depth,
            r#max_depth,
            r#min_temperature,
            r#enhanced_avg_respiration_rate,
            r#enhanced_max_respiration_rate,
            r#avg_respiration_rate,
            r#max_respiration_rate,
            r#total_grit,
            r#total_flow,
            r#jump_count,
            r#avg_grit,
            r#avg_flow,
            r#total_fractional_ascent,
            r#total_fractional_descent,
            r#avg_core_temperature,
            r#min_core_temperature,
            r#max_core_temperature,
            r#timestamp,
            r#message_index,
            unknown_fields,
        })
    }
}
impl TryFrom<FitDataRecord> for Lap {
    type Error = TryFromRecordError;
    fn try_from(record: FitDataRecord) -> Result<Self, Self::Error> {
        Self::parse(record)
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize)]
pub struct Length {
    pub r#event: Option<Value>,
    pub r#event_type: Option<Value>,
    pub r#start_time: Option<Value>,
    pub r#total_elapsed_time: Option<Value>,
    pub r#total_timer_time: Option<Value>,
    pub r#total_strokes: Option<Value>,
    pub r#avg_speed: Option<Value>,
    pub r#swim_stroke: Option<Value>,
    pub r#avg_swimming_cadence: Option<Value>,
    pub r#event_group: Option<Value>,
    pub r#total_calories: Option<Value>,
    pub r#length_type: Option<Value>,
    pub r#player_score: Option<Value>,
    pub r#opponent_score: Option<Value>,
    pub r#stroke_count: Option<Value>,
    pub r#zone_count: Option<Value>,
    pub r#enhanced_avg_respiration_rate: Option<Value>,
    pub r#enhanced_max_respiration_rate: Option<Value>,
    pub r#avg_respiration_rate: Option<Value>,
    pub r#max_respiration_rate: Option<Value>,
    pub r#timestamp: Option<Value>,
    pub r#message_index: Option<Value>,
    #[doc = r" All fields that are not defined in the profile."]
    pub unknown_fields: Vec<FitDataField>,
}
impl FitMessage for Length {
    const NAME: &'static str = "Length";
    const KIND: MesgNum = MesgNum::Length;
    fn parse(record: FitDataRecord) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#event = None;
        let mut r#event_type = None;
        let mut r#start_time = None;
        let mut r#total_elapsed_time = None;
        let mut r#total_timer_time = None;
        let mut r#total_strokes = None;
        let mut r#avg_speed = None;
        let mut r#swim_stroke = None;
        let mut r#avg_swimming_cadence = None;
        let mut r#event_group = None;
        let mut r#total_calories = None;
        let mut r#length_type = None;
        let mut r#player_score = None;
        let mut r#opponent_score = None;
        let mut r#stroke_count = None;
        let mut r#zone_count = None;
        let mut r#enhanced_avg_respiration_rate = None;
        let mut r#enhanced_max_respiration_rate = None;
        let mut r#avg_respiration_rate = None;
        let mut r#max_respiration_rate = None;
        let mut r#timestamp = None;
        let mut r#message_index = None;
        let mut unknown_fields = Vec::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => {
                    r#event = Some(field.into_value());
                }
                1u8 => {
                    r#event_type = Some(field.into_value());
                }
                2u8 => {
                    r#start_time = Some(field.into_value());
                }
                3u8 => {
                    r#total_elapsed_time = Some(field.into_value());
                }
                4u8 => {
                    r#total_timer_time = Some(field.into_value());
                }
                5u8 => {
                    r#total_strokes = Some(field.into_value());
                }
                6u8 => {
                    r#avg_speed = Some(field.into_value());
                }
                7u8 => {
                    r#swim_stroke = Some(field.into_value());
                }
                9u8 => {
                    r#avg_swimming_cadence = Some(field.into_value());
                }
                10u8 => {
                    r#event_group = Some(field.into_value());
                }
                11u8 => {
                    r#total_calories = Some(field.into_value());
                }
                12u8 => {
                    r#length_type = Some(field.into_value());
                }
                18u8 => {
                    r#player_score = Some(field.into_value());
                }
                19u8 => {
                    r#opponent_score = Some(field.into_value());
                }
                20u8 => {
                    r#stroke_count = Some(field.into_value());
                }
                21u8 => {
                    r#zone_count = Some(field.into_value());
                }
                22u8 => {
                    r#enhanced_avg_respiration_rate = Some(field.into_value());
                }
                23u8 => {
                    r#enhanced_max_respiration_rate = Some(field.into_value());
                }
                24u8 => {
                    r#avg_respiration_rate = Some(field.into_value());
                }
                25u8 => {
                    r#max_respiration_rate = Some(field.into_value());
                }
                253u8 => {
                    r#timestamp = Some(field.into_value());
                }
                254u8 => {
                    r#message_index = Some(field.into_value());
                }
                _ => {
                    unknown_fields.push(field);
                }
            }
        }
        Ok(Self {
            r#event,
            r#event_type,
            r#start_time,
            r#total_elapsed_time,
            r#total_timer_time,
            r#total_strokes,
            r#avg_speed,
            r#swim_stroke,
            r#avg_swimming_cadence,
            r#event_group,
            r#total_calories,
            r#length_type,
            r#player_score,
            r#opponent_score,
            r#stroke_count,
            r#zone_count,
            r#enhanced_avg_respiration_rate,
            r#enhanced_max_respiration_rate,
            r#avg_respiration_rate,
            r#max_respiration_rate,
            r#timestamp,
            r#message_index,
            unknown_fields,
        })
    }
}
impl TryFrom<FitDataRecord> for Length {
    type Error = TryFromRecordError;
    fn try_from(record: FitDataRecord) -> Result<Self, Self::Error> {
        Self::parse(record)
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize)]
pub struct Record {
    pub r#position_lat: Option<Value>,
    pub r#position_long: Option<Value>,
    pub r#altitude: Option<Value>,
    pub r#heart_rate: Option<Value>,
    pub r#cadence: Option<Value>,
    pub r#distance: Option<Value>,
    pub r#speed: Option<Value>,
    pub r#power: Option<Value>,
    pub r#compressed_speed_distance: Option<Value>,
    pub r#grade: Option<Value>,
    pub r#resistance: Option<Value>,
    pub r#time_from_course: Option<Value>,
    pub r#cycle_length: Option<Value>,
    pub r#temperature: Option<Value>,
    pub r#speed_1s: Option<Value>,
    pub r#cycles: Option<Value>,
    pub r#total_cycles: Option<Value>,
    pub r#compressed_accumulated_power: Option<Value>,
    pub r#accumulated_power: Option<Value>,
    pub r#left_right_balance: Option<Value>,
    pub r#gps_accuracy: Option<Value>,
    pub r#vertical_speed: Option<Value>,
    pub r#calories: Option<Value>,
    pub r#vertical_oscillation: Option<Value>,
    pub r#stance_time_percent: Option<Value>,
    pub r#stance_time: Option<Value>,
    pub r#activity_type: Option<Value>,
    pub r#left_torque_effectiveness: Option<Value>,
    pub r#right_torque_effectiveness: Option<Value>,
    pub r#left_pedal_smoothness: Option<Value>,
    pub r#right_pedal_smoothness: Option<Value>,
    pub r#combined_pedal_smoothness: Option<Value>,
    pub r#time128: Option<Value>,
    pub r#stroke_type: Option<Value>,
    pub r#zone: Option<Value>,
    pub r#ball_speed: Option<Value>,
    pub r#cadence256: Option<Value>,
    pub r#fractional_cadence: Option<Value>,
    pub r#total_hemoglobin_conc: Option<Value>,
    pub r#total_hemoglobin_conc_min: Option<Value>,
    pub r#total_hemoglobin_conc_max: Option<Value>,
    pub r#saturated_hemoglobin_percent: Option<Value>,
    pub r#saturated_hemoglobin_percent_min: Option<Value>,
    pub r#saturated_hemoglobin_percent_max: Option<Value>,
    pub r#device_index: Option<Value>,
    pub r#left_pco: Option<Value>,
    pub r#right_pco: Option<Value>,
    pub r#left_power_phase: Option<Value>,
    pub r#left_power_phase_peak: Option<Value>,
    pub r#right_power_phase: Option<Value>,
    pub r#right_power_phase_peak: Option<Value>,
    pub r#enhanced_speed: Option<Value>,
    pub r#enhanced_altitude: Option<Value>,
    pub r#battery_soc: Option<Value>,
    pub r#motor_power: Option<Value>,
    pub r#vertical_ratio: Option<Value>,
    pub r#stance_time_balance: Option<Value>,
    pub r#step_length: Option<Value>,
    pub r#cycle_length16: Option<Value>,
    pub r#absolute_pressure: Option<Value>,
    pub r#depth: Option<Value>,
    pub r#next_stop_depth: Option<Value>,
    pub r#next_stop_time: Option<Value>,
    pub r#time_to_surface: Option<Value>,
    pub r#ndl_time: Option<Value>,
    pub r#cns_load: Option<Value>,
    pub r#n2_load: Option<Value>,
    pub r#respiration_rate: Option<Value>,
    pub r#enhanced_respiration_rate: Option<Value>,
    pub r#grit: Option<Value>,
    pub r#flow: Option<Value>,
    pub r#current_stress: Option<Value>,
    pub r#ebike_travel_range: Option<Value>,
    pub r#ebike_battery_level: Option<Value>,
    pub r#ebike_assist_mode: Option<Value>,
    pub r#ebike_assist_level_percent: Option<Value>,
    pub r#air_time_remaining: Option<Value>,
    pub r#pressure_sac: Option<Value>,
    pub r#volume_sac: Option<Value>,
    pub r#rmv: Option<Value>,
    pub r#ascent_rate: Option<Value>,
    pub r#po2: Option<Value>,
    pub r#core_temperature: Option<Value>,
    pub r#timestamp: Option<Value>,
    #[doc = r" All fields that are not defined in the profile."]
    pub unknown_fields: Vec<FitDataField>,
}
impl FitMessage for Record {
    const NAME: &'static str = "Record";
    const KIND: MesgNum = MesgNum::Record;
    fn parse(record: FitDataRecord) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#position_lat = None;
        let mut r#position_long = None;
        let mut r#altitude = None;
        let mut r#heart_rate = None;
        let mut r#cadence = None;
        let mut r#distance = None;
        let mut r#speed = None;
        let mut r#power = None;
        let mut r#compressed_speed_distance = None;
        let mut r#grade = None;
        let mut r#resistance = None;
        let mut r#time_from_course = None;
        let mut r#cycle_length = None;
        let mut r#temperature = None;
        let mut r#speed_1s = None;
        let mut r#cycles = None;
        let mut r#total_cycles = None;
        let mut r#compressed_accumulated_power = None;
        let mut r#accumulated_power = None;
        let mut r#left_right_balance = None;
        let mut r#gps_accuracy = None;
        let mut r#vertical_speed = None;
        let mut r#calories = None;
        let mut r#vertical_oscillation = None;
        let mut r#stance_time_percent = None;
        let mut r#stance_time = None;
        let mut r#activity_type = None;
        let mut r#left_torque_effectiveness = None;
        let mut r#right_torque_effectiveness = None;
        let mut r#left_pedal_smoothness = None;
        let mut r#right_pedal_smoothness = None;
        let mut r#combined_pedal_smoothness = None;
        let mut r#time128 = None;
        let mut r#stroke_type = None;
        let mut r#zone = None;
        let mut r#ball_speed = None;
        let mut r#cadence256 = None;
        let mut r#fractional_cadence = None;
        let mut r#total_hemoglobin_conc = None;
        let mut r#total_hemoglobin_conc_min = None;
        let mut r#total_hemoglobin_conc_max = None;
        let mut r#saturated_hemoglobin_percent = None;
        let mut r#saturated_hemoglobin_percent_min = None;
        let mut r#saturated_hemoglobin_percent_max = None;
        let mut r#device_index = None;
        let mut r#left_pco = None;
        let mut r#right_pco = None;
        let mut r#left_power_phase = None;
        let mut r#left_power_phase_peak = None;
        let mut r#right_power_phase = None;
        let mut r#right_power_phase_peak = None;
        let mut r#enhanced_speed = None;
        let mut r#enhanced_altitude = None;
        let mut r#battery_soc = None;
        let mut r#motor_power = None;
        let mut r#vertical_ratio = None;
        let mut r#stance_time_balance = None;
        let mut r#step_length = None;
        let mut r#cycle_length16 = None;
        let mut r#absolute_pressure = None;
        let mut r#depth = None;
        let mut r#next_stop_depth = None;
        let mut r#next_stop_time = None;
        let mut r#time_to_surface = None;
        let mut r#ndl_time = None;
        let mut r#cns_load = None;
        let mut r#n2_load = None;
        let mut r#respiration_rate = None;
        let mut r#enhanced_respiration_rate = None;
        let mut r#grit = None;
        let mut r#flow = None;
        let mut r#current_stress = None;
        let mut r#ebike_travel_range = None;
        let mut r#ebike_battery_level = None;
        let mut r#ebike_assist_mode = None;
        let mut r#ebike_assist_level_percent = None;
        let mut r#air_time_remaining = None;
        let mut r#pressure_sac = None;
        let mut r#volume_sac = None;
        let mut r#rmv = None;
        let mut r#ascent_rate = None;
        let mut r#po2 = None;
        let mut r#core_temperature = None;
        let mut r#timestamp = None;
        let mut unknown_fields = Vec::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => {
                    r#position_lat = Some(field.into_value());
                }
                1u8 => {
                    r#position_long = Some(field.into_value());
                }
                2u8 => {
                    r#altitude = Some(field.into_value());
                }
                3u8 => {
                    r#heart_rate = Some(field.into_value());
                }
                4u8 => {
                    r#cadence = Some(field.into_value());
                }
                5u8 => {
                    r#distance = Some(field.into_value());
                }
                6u8 => {
                    r#speed = Some(field.into_value());
                }
                7u8 => {
                    r#power = Some(field.into_value());
                }
                8u8 => {
                    r#compressed_speed_distance = Some(field.into_value());
                }
                9u8 => {
                    r#grade = Some(field.into_value());
                }
                10u8 => {
                    r#resistance = Some(field.into_value());
                }
                11u8 => {
                    r#time_from_course = Some(field.into_value());
                }
                12u8 => {
                    r#cycle_length = Some(field.into_value());
                }
                13u8 => {
                    r#temperature = Some(field.into_value());
                }
                17u8 => {
                    r#speed_1s = Some(field.into_value());
                }
                18u8 => {
                    r#cycles = Some(field.into_value());
                }
                19u8 => {
                    r#total_cycles = Some(field.into_value());
                }
                28u8 => {
                    r#compressed_accumulated_power = Some(field.into_value());
                }
                29u8 => {
                    r#accumulated_power = Some(field.into_value());
                }
                30u8 => {
                    r#left_right_balance = Some(field.into_value());
                }
                31u8 => {
                    r#gps_accuracy = Some(field.into_value());
                }
                32u8 => {
                    r#vertical_speed = Some(field.into_value());
                }
                33u8 => {
                    r#calories = Some(field.into_value());
                }
                39u8 => {
                    r#vertical_oscillation = Some(field.into_value());
                }
                40u8 => {
                    r#stance_time_percent = Some(field.into_value());
                }
                41u8 => {
                    r#stance_time = Some(field.into_value());
                }
                42u8 => {
                    r#activity_type = Some(field.into_value());
                }
                43u8 => {
                    r#left_torque_effectiveness = Some(field.into_value());
                }
                44u8 => {
                    r#right_torque_effectiveness = Some(field.into_value());
                }
                45u8 => {
                    r#left_pedal_smoothness = Some(field.into_value());
                }
                46u8 => {
                    r#right_pedal_smoothness = Some(field.into_value());
                }
                47u8 => {
                    r#combined_pedal_smoothness = Some(field.into_value());
                }
                48u8 => {
                    r#time128 = Some(field.into_value());
                }
                49u8 => {
                    r#stroke_type = Some(field.into_value());
                }
                50u8 => {
                    r#zone = Some(field.into_value());
                }
                51u8 => {
                    r#ball_speed = Some(field.into_value());
                }
                52u8 => {
                    r#cadence256 = Some(field.into_value());
                }
                53u8 => {
                    r#fractional_cadence = Some(field.into_value());
                }
                54u8 => {
                    r#total_hemoglobin_conc = Some(field.into_value());
                }
                55u8 => {
                    r#total_hemoglobin_conc_min = Some(field.into_value());
                }
                56u8 => {
                    r#total_hemoglobin_conc_max = Some(field.into_value());
                }
                57u8 => {
                    r#saturated_hemoglobin_percent = Some(field.into_value());
                }
                58u8 => {
                    r#saturated_hemoglobin_percent_min = Some(field.into_value());
                }
                59u8 => {
                    r#saturated_hemoglobin_percent_max = Some(field.into_value());
                }
                62u8 => {
                    r#device_index = Some(field.into_value());
                }
                67u8 => {
                    r#left_pco = Some(field.into_value());
                }
                68u8 => {
                    r#right_pco = Some(field.into_value());
                }
                69u8 => {
                    r#left_power_phase = Some(field.into_value());
                }
                70u8 => {
                    r#left_power_phase_peak = Some(field.into_value());
                }
                71u8 => {
                    r#right_power_phase = Some(field.into_value());
                }
                72u8 => {
                    r#right_power_phase_peak = Some(field.into_value());
                }
                73u8 => {
                    r#enhanced_speed = Some(field.into_value());
                }
                78u8 => {
                    r#enhanced_altitude = Some(field.into_value());
                }
                81u8 => {
                    r#battery_soc = Some(field.into_value());
                }
                82u8 => {
                    r#motor_power = Some(field.into_value());
                }
                83u8 => {
                    r#vertical_ratio = Some(field.into_value());
                }
                84u8 => {
                    r#stance_time_balance = Some(field.into_value());
                }
                85u8 => {
                    r#step_length = Some(field.into_value());
                }
                87u8 => {
                    r#cycle_length16 = Some(field.into_value());
                }
                91u8 => {
                    r#absolute_pressure = Some(field.into_value());
                }
                92u8 => {
                    r#depth = Some(field.into_value());
                }
                93u8 => {
                    r#next_stop_depth = Some(field.into_value());
                }
                94u8 => {
                    r#next_stop_time = Some(field.into_value());
                }
                95u8 => {
                    r#time_to_surface = Some(field.into_value());
                }
                96u8 => {
                    r#ndl_time = Some(field.into_value());
                }
                97u8 => {
                    r#cns_load = Some(field.into_value());
                }
                98u8 => {
                    r#n2_load = Some(field.into_value());
                }
                99u8 => {
                    r#respiration_rate = Some(field.into_value());
                }
                108u8 => {
                    r#enhanced_respiration_rate = Some(field.into_value());
                }
                114u8 => {
                    r#grit = Some(field.into_value());
                }
                115u8 => {
                    r#flow = Some(field.into_value());
                }
                116u8 => {
                    r#current_stress = Some(field.into_value());
                }
                117u8 => {
                    r#ebike_travel_range = Some(field.into_value());
                }
                118u8 => {
                    r#ebike_battery_level = Some(field.into_value());
                }
                119u8 => {
                    r#ebike_assist_mode = Some(field.into_value());
                }
                120u8 => {
                    r#ebike_assist_level_percent = Some(field.into_value());
                }
                123u8 => {
                    r#air_time_remaining = Some(field.into_value());
                }
                124u8 => {
                    r#pressure_sac = Some(field.into_value());
                }
                125u8 => {
                    r#volume_sac = Some(field.into_value());
                }
                126u8 => {
                    r#rmv = Some(field.into_value());
                }
                127u8 => {
                    r#ascent_rate = Some(field.into_value());
                }
                129u8 => {
                    r#po2 = Some(field.into_value());
                }
                139u8 => {
                    r#core_temperature = Some(field.into_value());
                }
                253u8 => {
                    r#timestamp = Some(field.into_value());
                }
                _ => {
                    unknown_fields.push(field);
                }
            }
        }
        Ok(Self {
            r#position_lat,
            r#position_long,
            r#altitude,
            r#heart_rate,
            r#cadence,
            r#distance,
            r#speed,
            r#power,
            r#compressed_speed_distance,
            r#grade,
            r#resistance,
            r#time_from_course,
            r#cycle_length,
            r#temperature,
            r#speed_1s,
            r#cycles,
            r#total_cycles,
            r#compressed_accumulated_power,
            r#accumulated_power,
            r#left_right_balance,
            r#gps_accuracy,
            r#vertical_speed,
            r#calories,
            r#vertical_oscillation,
            r#stance_time_percent,
            r#stance_time,
            r#activity_type,
            r#left_torque_effectiveness,
            r#right_torque_effectiveness,
            r#left_pedal_smoothness,
            r#right_pedal_smoothness,
            r#combined_pedal_smoothness,
            r#time128,
            r#stroke_type,
            r#zone,
            r#ball_speed,
            r#cadence256,
            r#fractional_cadence,
            r#total_hemoglobin_conc,
            r#total_hemoglobin_conc_min,
            r#total_hemoglobin_conc_max,
            r#saturated_hemoglobin_percent,
            r#saturated_hemoglobin_percent_min,
            r#saturated_hemoglobin_percent_max,
            r#device_index,
            r#left_pco,
            r#right_pco,
            r#left_power_phase,
            r#left_power_phase_peak,
            r#right_power_phase,
            r#right_power_phase_peak,
            r#enhanced_speed,
            r#enhanced_altitude,
            r#battery_soc,
            r#motor_power,
            r#vertical_ratio,
            r#stance_time_balance,
            r#step_length,
            r#cycle_length16,
            r#absolute_pressure,
            r#depth,
            r#next_stop_depth,
            r#next_stop_time,
            r#time_to_surface,
            r#ndl_time,
            r#cns_load,
            r#n2_load,
            r#respiration_rate,
            r#enhanced_respiration_rate,
            r#grit,
            r#flow,
            r#current_stress,
            r#ebike_travel_range,
            r#ebike_battery_level,
            r#ebike_assist_mode,
            r#ebike_assist_level_percent,
            r#air_time_remaining,
            r#pressure_sac,
            r#volume_sac,
            r#rmv,
            r#ascent_rate,
            r#po2,
            r#core_temperature,
            r#timestamp,
            unknown_fields,
        })
    }
}
impl TryFrom<FitDataRecord> for Record {
    type Error = TryFromRecordError;
    fn try_from(record: FitDataRecord) -> Result<Self, Self::Error> {
        Self::parse(record)
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize)]
pub struct Event {
    pub r#event: Option<Value>,
    pub r#event_type: Option<Value>,
    pub r#data16: Option<Value>,
    pub r#data: Option<Value>,
    pub r#event_group: Option<Value>,
    pub r#score: Option<Value>,
    pub r#opponent_score: Option<Value>,
    pub r#front_gear_num: Option<Value>,
    pub r#front_gear: Option<Value>,
    pub r#rear_gear_num: Option<Value>,
    pub r#rear_gear: Option<Value>,
    pub r#device_index: Option<Value>,
    pub r#activity_type: Option<Value>,
    pub r#start_timestamp: Option<Value>,
    pub r#radar_threat_level_max: Option<Value>,
    pub r#radar_threat_count: Option<Value>,
    pub r#radar_threat_avg_approach_speed: Option<Value>,
    pub r#radar_threat_max_approach_speed: Option<Value>,
    pub r#timestamp: Option<Value>,
    #[doc = r" All fields that are not defined in the profile."]
    pub unknown_fields: Vec<FitDataField>,
}
impl FitMessage for Event {
    const NAME: &'static str = "Event";
    const KIND: MesgNum = MesgNum::Event;
    fn parse(record: FitDataRecord) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#event = None;
        let mut r#event_type = None;
        let mut r#data16 = None;
        let mut r#data = None;
        let mut r#event_group = None;
        let mut r#score = None;
        let mut r#opponent_score = None;
        let mut r#front_gear_num = None;
        let mut r#front_gear = None;
        let mut r#rear_gear_num = None;
        let mut r#rear_gear = None;
        let mut r#device_index = None;
        let mut r#activity_type = None;
        let mut r#start_timestamp = None;
        let mut r#radar_threat_level_max = None;
        let mut r#radar_threat_count = None;
        let mut r#radar_threat_avg_approach_speed = None;
        let mut r#radar_threat_max_approach_speed = None;
        let mut r#timestamp = None;
        let mut unknown_fields = Vec::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => {
                    r#event = Some(field.into_value());
                }
                1u8 => {
                    r#event_type = Some(field.into_value());
                }
                2u8 => {
                    r#data16 = Some(field.into_value());
                }
                3u8 => {
                    r#data = Some(field.into_value());
                }
                4u8 => {
                    r#event_group = Some(field.into_value());
                }
                7u8 => {
                    r#score = Some(field.into_value());
                }
                8u8 => {
                    r#opponent_score = Some(field.into_value());
                }
                9u8 => {
                    r#front_gear_num = Some(field.into_value());
                }
                10u8 => {
                    r#front_gear = Some(field.into_value());
                }
                11u8 => {
                    r#rear_gear_num = Some(field.into_value());
                }
                12u8 => {
                    r#rear_gear = Some(field.into_value());
                }
                13u8 => {
                    r#device_index = Some(field.into_value());
                }
                14u8 => {
                    r#activity_type = Some(field.into_value());
                }
                15u8 => {
                    r#start_timestamp = Some(field.into_value());
                }
                21u8 => {
                    r#radar_threat_level_max = Some(field.into_value());
                }
                22u8 => {
                    r#radar_threat_count = Some(field.into_value());
                }
                23u8 => {
                    r#radar_threat_avg_approach_speed = Some(field.into_value());
                }
                24u8 => {
                    r#radar_threat_max_approach_speed = Some(field.into_value());
                }
                253u8 => {
                    r#timestamp = Some(field.into_value());
                }
                _ => {
                    unknown_fields.push(field);
                }
            }
        }
        Ok(Self {
            r#event,
            r#event_type,
            r#data16,
            r#data,
            r#event_group,
            r#score,
            r#opponent_score,
            r#front_gear_num,
            r#front_gear,
            r#rear_gear_num,
            r#rear_gear,
            r#device_index,
            r#activity_type,
            r#start_timestamp,
            r#radar_threat_level_max,
            r#radar_threat_count,
            r#radar_threat_avg_approach_speed,
            r#radar_threat_max_approach_speed,
            r#timestamp,
            unknown_fields,
        })
    }
}
impl TryFrom<FitDataRecord> for Event {
    type Error = TryFromRecordError;
    fn try_from(record: FitDataRecord) -> Result<Self, Self::Error> {
        Self::parse(record)
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize)]
pub struct DeviceInfo {
    pub r#device_index: Option<Value>,
    pub r#device_type: Option<Value>,
    pub r#manufacturer: Option<Value>,
    pub r#serial_number: Option<Value>,
    pub r#product: Option<Value>,
    pub r#software_version: Option<Value>,
    pub r#hardware_version: Option<Value>,
    pub r#cum_operating_time: Option<Value>,
    pub r#battery_voltage: Option<Value>,
    pub r#battery_status: Option<Value>,
    pub r#sensor_position: Option<Value>,
    pub r#descriptor: Option<Value>,
    pub r#ant_transmission_type: Option<Value>,
    pub r#ant_device_number: Option<Value>,
    pub r#ant_network: Option<Value>,
    pub r#source_type: Option<Value>,
    pub r#product_name: Option<Value>,
    pub r#battery_level: Option<Value>,
    pub r#timestamp: Option<Value>,
    #[doc = r" All fields that are not defined in the profile."]
    pub unknown_fields: Vec<FitDataField>,
}
impl FitMessage for DeviceInfo {
    const NAME: &'static str = "DeviceInfo";
    const KIND: MesgNum = MesgNum::DeviceInfo;
    fn parse(record: FitDataRecord) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#device_index = None;
        let mut r#device_type = None;
        let mut r#manufacturer = None;
        let mut r#serial_number = None;
        let mut r#product = None;
        let mut r#software_version = None;
        let mut r#hardware_version = None;
        let mut r#cum_operating_time = None;
        let mut r#battery_voltage = None;
        let mut r#battery_status = None;
        let mut r#sensor_position = None;
        let mut r#descriptor = None;
        let mut r#ant_transmission_type = None;
        let mut r#ant_device_number = None;
        let mut r#ant_network = None;
        let mut r#source_type = None;
        let mut r#product_name = None;
        let mut r#battery_level = None;
        let mut r#timestamp = None;
        let mut unknown_fields = Vec::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => {
                    r#device_index = Some(field.into_value());
                }
                1u8 => {
                    r#device_type = Some(field.into_value());
                }
                2u8 => {
                    r#manufacturer = Some(field.into_value());
                }
                3u8 => {
                    r#serial_number = Some(field.into_value());
                }
                4u8 => {
                    r#product = Some(field.into_value());
                }
                5u8 => {
                    r#software_version = Some(field.into_value());
                }
                6u8 => {
                    r#hardware_version = Some(field.into_value());
                }
                7u8 => {
                    r#cum_operating_time = Some(field.into_value());
                }
                10u8 => {
                    r#battery_voltage = Some(field.into_value());
                }
                11u8 => {
                    r#battery_status = Some(field.into_value());
                }
                18u8 => {
                    r#sensor_position = Some(field.into_value());
                }
                19u8 => {
                    r#descriptor = Some(field.into_value());
                }
                20u8 => {
                    r#ant_transmission_type = Some(field.into_value());
                }
                21u8 => {
                    r#ant_device_number = Some(field.into_value());
                }
                22u8 => {
                    r#ant_network = Some(field.into_value());
                }
                25u8 => {
                    r#source_type = Some(field.into_value());
                }
                27u8 => {
                    r#product_name = Some(field.into_value());
                }
                32u8 => {
                    r#battery_level = Some(field.into_value());
                }
                253u8 => {
                    r#timestamp = Some(field.into_value());
                }
                _ => {
                    unknown_fields.push(field);
                }
            }
        }
        Ok(Self {
            r#device_index,
            r#device_type,
            r#manufacturer,
            r#serial_number,
            r#product,
            r#software_version,
            r#hardware_version,
            r#cum_operating_time,
            r#battery_voltage,
            r#battery_status,
            r#sensor_position,
            r#descriptor,
            r#ant_transmission_type,
            r#ant_device_number,
            r#ant_network,
            r#source_type,
            r#product_name,
            r#battery_level,
            r#timestamp,
            unknown_fields,
        })
    }
}
impl TryFrom<FitDataRecord> for DeviceInfo {
    type Error = TryFromRecordError;
    fn try_from(record: FitDataRecord) -> Result<Self, Self::Error> {
        Self::parse(record)
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize)]
pub struct DeviceAuxBatteryInfo {
    pub r#device_index: Option<Value>,
    pub r#battery_voltage: Option<Value>,
    pub r#battery_status: Option<Value>,
    pub r#battery_identifier: Option<Value>,
    pub r#timestamp: Option<Value>,
    #[doc = r" All fields that are not defined in the profile."]
    pub unknown_fields: Vec<FitDataField>,
}
impl FitMessage for DeviceAuxBatteryInfo {
    const NAME: &'static str = "DeviceAuxBatteryInfo";
    const KIND: MesgNum = MesgNum::DeviceAuxBatteryInfo;
    fn parse(record: FitDataRecord) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#device_index = None;
        let mut r#battery_voltage = None;
        let mut r#battery_status = None;
        let mut r#battery_identifier = None;
        let mut r#timestamp = None;
        let mut unknown_fields = Vec::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => {
                    r#device_index = Some(field.into_value());
                }
                1u8 => {
                    r#battery_voltage = Some(field.into_value());
                }
                2u8 => {
                    r#battery_status = Some(field.into_value());
                }
                3u8 => {
                    r#battery_identifier = Some(field.into_value());
                }
                253u8 => {
                    r#timestamp = Some(field.into_value());
                }
                _ => {
                    unknown_fields.push(field);
                }
            }
        }
        Ok(Self {
            r#device_index,
            r#battery_voltage,
            r#battery_status,
            r#battery_identifier,
            r#timestamp,
            unknown_fields,
        })
    }
}
impl TryFrom<FitDataRecord> for DeviceAuxBatteryInfo {
    type Error = TryFromRecordError;
    fn try_from(record: FitDataRecord) -> Result<Self, Self::Error> {
        Self::parse(record)
    }
}
#[doc = "Corresponds to file_id of workout or course."]
#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize)]
pub struct TrainingFile {
    pub r#type: Option<Value>,
    pub r#manufacturer: Option<Value>,
    pub r#product: Option<Value>,
    pub r#serial_number: Option<Value>,
    pub r#time_created: Option<Value>,
    pub r#timestamp: Option<Value>,
    #[doc = r" All fields that are not defined in the profile."]
    pub unknown_fields: Vec<FitDataField>,
}
impl FitMessage for TrainingFile {
    const NAME: &'static str = "TrainingFile";
    const KIND: MesgNum = MesgNum::TrainingFile;
    fn parse(record: FitDataRecord) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#type = None;
        let mut r#manufacturer = None;
        let mut r#product = None;
        let mut r#serial_number = None;
        let mut r#time_created = None;
        let mut r#timestamp = None;
        let mut unknown_fields = Vec::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => {
                    r#type = Some(field.into_value());
                }
                1u8 => {
                    r#manufacturer = Some(field.into_value());
                }
                2u8 => {
                    r#product = Some(field.into_value());
                }
                3u8 => {
                    r#serial_number = Some(field.into_value());
                }
                4u8 => {
                    r#time_created = Some(field.into_value());
                }
                253u8 => {
                    r#timestamp = Some(field.into_value());
                }
                _ => {
                    unknown_fields.push(field);
                }
            }
        }
        Ok(Self {
            r#type,
            r#manufacturer,
            r#product,
            r#serial_number,
            r#time_created,
            r#timestamp,
            unknown_fields,
        })
    }
}
impl TryFrom<FitDataRecord> for TrainingFile {
    type Error = TryFromRecordError;
    fn try_from(record: FitDataRecord) -> Result<Self, Self::Error> {
        Self::parse(record)
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize)]
pub struct WeatherConditions {
    pub r#weather_report: Option<Value>,
    pub r#temperature: Option<Value>,
    pub r#condition: Option<Value>,
    pub r#wind_direction: Option<Value>,
    pub r#wind_speed: Option<Value>,
    pub r#precipitation_probability: Option<Value>,
    pub r#temperature_feels_like: Option<Value>,
    pub r#relative_humidity: Option<Value>,
    pub r#location: Option<Value>,
    pub r#observed_at_time: Option<Value>,
    pub r#observed_location_lat: Option<Value>,
    pub r#observed_location_long: Option<Value>,
    pub r#day_of_week: Option<Value>,
    pub r#high_temperature: Option<Value>,
    pub r#low_temperature: Option<Value>,
    pub r#timestamp: Option<Value>,
    #[doc = r" All fields that are not defined in the profile."]
    pub unknown_fields: Vec<FitDataField>,
}
impl FitMessage for WeatherConditions {
    const NAME: &'static str = "WeatherConditions";
    const KIND: MesgNum = MesgNum::WeatherConditions;
    fn parse(record: FitDataRecord) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#weather_report = None;
        let mut r#temperature = None;
        let mut r#condition = None;
        let mut r#wind_direction = None;
        let mut r#wind_speed = None;
        let mut r#precipitation_probability = None;
        let mut r#temperature_feels_like = None;
        let mut r#relative_humidity = None;
        let mut r#location = None;
        let mut r#observed_at_time = None;
        let mut r#observed_location_lat = None;
        let mut r#observed_location_long = None;
        let mut r#day_of_week = None;
        let mut r#high_temperature = None;
        let mut r#low_temperature = None;
        let mut r#timestamp = None;
        let mut unknown_fields = Vec::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => {
                    r#weather_report = Some(field.into_value());
                }
                1u8 => {
                    r#temperature = Some(field.into_value());
                }
                2u8 => {
                    r#condition = Some(field.into_value());
                }
                3u8 => {
                    r#wind_direction = Some(field.into_value());
                }
                4u8 => {
                    r#wind_speed = Some(field.into_value());
                }
                5u8 => {
                    r#precipitation_probability = Some(field.into_value());
                }
                6u8 => {
                    r#temperature_feels_like = Some(field.into_value());
                }
                7u8 => {
                    r#relative_humidity = Some(field.into_value());
                }
                8u8 => {
                    r#location = Some(field.into_value());
                }
                9u8 => {
                    r#observed_at_time = Some(field.into_value());
                }
                10u8 => {
                    r#observed_location_lat = Some(field.into_value());
                }
                11u8 => {
                    r#observed_location_long = Some(field.into_value());
                }
                12u8 => {
                    r#day_of_week = Some(field.into_value());
                }
                13u8 => {
                    r#high_temperature = Some(field.into_value());
                }
                14u8 => {
                    r#low_temperature = Some(field.into_value());
                }
                253u8 => {
                    r#timestamp = Some(field.into_value());
                }
                _ => {
                    unknown_fields.push(field);
                }
            }
        }
        Ok(Self {
            r#weather_report,
            r#temperature,
            r#condition,
            r#wind_direction,
            r#wind_speed,
            r#precipitation_probability,
            r#temperature_feels_like,
            r#relative_humidity,
            r#location,
            r#observed_at_time,
            r#observed_location_lat,
            r#observed_location_long,
            r#day_of_week,
            r#high_temperature,
            r#low_temperature,
            r#timestamp,
            unknown_fields,
        })
    }
}
impl TryFrom<FitDataRecord> for WeatherConditions {
    type Error = TryFromRecordError;
    fn try_from(record: FitDataRecord) -> Result<Self, Self::Error> {
        Self::parse(record)
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize)]
pub struct WeatherAlert {
    pub r#report_id: Option<Value>,
    pub r#issue_time: Option<Value>,
    pub r#expire_time: Option<Value>,
    pub r#severity: Option<Value>,
    pub r#type: Option<Value>,
    pub r#timestamp: Option<Value>,
    #[doc = r" All fields that are not defined in the profile."]
    pub unknown_fields: Vec<FitDataField>,
}
impl FitMessage for WeatherAlert {
    const NAME: &'static str = "WeatherAlert";
    const KIND: MesgNum = MesgNum::WeatherAlert;
    fn parse(record: FitDataRecord) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#report_id = None;
        let mut r#issue_time = None;
        let mut r#expire_time = None;
        let mut r#severity = None;
        let mut r#type = None;
        let mut r#timestamp = None;
        let mut unknown_fields = Vec::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => {
                    r#report_id = Some(field.into_value());
                }
                1u8 => {
                    r#issue_time = Some(field.into_value());
                }
                2u8 => {
                    r#expire_time = Some(field.into_value());
                }
                3u8 => {
                    r#severity = Some(field.into_value());
                }
                4u8 => {
                    r#type = Some(field.into_value());
                }
                253u8 => {
                    r#timestamp = Some(field.into_value());
                }
                _ => {
                    unknown_fields.push(field);
                }
            }
        }
        Ok(Self {
            r#report_id,
            r#issue_time,
            r#expire_time,
            r#severity,
            r#type,
            r#timestamp,
            unknown_fields,
        })
    }
}
impl TryFrom<FitDataRecord> for WeatherAlert {
    type Error = TryFromRecordError;
    fn try_from(record: FitDataRecord) -> Result<Self, Self::Error> {
        Self::parse(record)
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize)]
pub struct GpsMetadata {
    pub r#timestamp_ms: Option<Value>,
    pub r#position_lat: Option<Value>,
    pub r#position_long: Option<Value>,
    pub r#enhanced_altitude: Option<Value>,
    pub r#enhanced_speed: Option<Value>,
    pub r#heading: Option<Value>,
    pub r#utc_timestamp: Option<Value>,
    pub r#velocity: Option<Value>,
    pub r#timestamp: Option<Value>,
    #[doc = r" All fields that are not defined in the profile."]
    pub unknown_fields: Vec<FitDataField>,
}
impl FitMessage for GpsMetadata {
    const NAME: &'static str = "GpsMetadata";
    const KIND: MesgNum = MesgNum::GpsMetadata;
    fn parse(record: FitDataRecord) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#timestamp_ms = None;
        let mut r#position_lat = None;
        let mut r#position_long = None;
        let mut r#enhanced_altitude = None;
        let mut r#enhanced_speed = None;
        let mut r#heading = None;
        let mut r#utc_timestamp = None;
        let mut r#velocity = None;
        let mut r#timestamp = None;
        let mut unknown_fields = Vec::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => {
                    r#timestamp_ms = Some(field.into_value());
                }
                1u8 => {
                    r#position_lat = Some(field.into_value());
                }
                2u8 => {
                    r#position_long = Some(field.into_value());
                }
                3u8 => {
                    r#enhanced_altitude = Some(field.into_value());
                }
                4u8 => {
                    r#enhanced_speed = Some(field.into_value());
                }
                5u8 => {
                    r#heading = Some(field.into_value());
                }
                6u8 => {
                    r#utc_timestamp = Some(field.into_value());
                }
                7u8 => {
                    r#velocity = Some(field.into_value());
                }
                253u8 => {
                    r#timestamp = Some(field.into_value());
                }
                _ => {
                    unknown_fields.push(field);
                }
            }
        }
        Ok(Self {
            r#timestamp_ms,
            r#position_lat,
            r#position_long,
            r#enhanced_altitude,
            r#enhanced_speed,
            r#heading,
            r#utc_timestamp,
            r#velocity,
            r#timestamp,
            unknown_fields,
        })
    }
}
impl TryFrom<FitDataRecord> for GpsMetadata {
    type Error = TryFromRecordError;
    fn try_from(record: FitDataRecord) -> Result<Self, Self::Error> {
        Self::parse(record)
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize)]
pub struct CameraEvent {
    pub r#timestamp_ms: Option<Value>,
    pub r#camera_event_type: Option<Value>,
    pub r#camera_file_uuid: Option<Value>,
    pub r#camera_orientation: Option<Value>,
    pub r#timestamp: Option<Value>,
    #[doc = r" All fields that are not defined in the profile."]
    pub unknown_fields: Vec<FitDataField>,
}
impl FitMessage for CameraEvent {
    const NAME: &'static str = "CameraEvent";
    const KIND: MesgNum = MesgNum::CameraEvent;
    fn parse(record: FitDataRecord) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#timestamp_ms = None;
        let mut r#camera_event_type = None;
        let mut r#camera_file_uuid = None;
        let mut r#camera_orientation = None;
        let mut r#timestamp = None;
        let mut unknown_fields = Vec::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => {
                    r#timestamp_ms = Some(field.into_value());
                }
                1u8 => {
                    r#camera_event_type = Some(field.into_value());
                }
                2u8 => {
                    r#camera_file_uuid = Some(field.into_value());
                }
                3u8 => {
                    r#camera_orientation = Some(field.into_value());
                }
                253u8 => {
                    r#timestamp = Some(field.into_value());
                }
                _ => {
                    unknown_fields.push(field);
                }
            }
        }
        Ok(Self {
            r#timestamp_ms,
            r#camera_event_type,
            r#camera_file_uuid,
            r#camera_orientation,
            r#timestamp,
            unknown_fields,
        })
    }
}
impl TryFrom<FitDataRecord> for CameraEvent {
    type Error = TryFromRecordError;
    fn try_from(record: FitDataRecord) -> Result<Self, Self::Error> {
        Self::parse(record)
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize)]
pub struct GyroscopeData {
    pub r#timestamp_ms: Option<Value>,
    pub r#sample_time_offset: Option<Value>,
    pub r#gyro_x: Option<Value>,
    pub r#gyro_y: Option<Value>,
    pub r#gyro_z: Option<Value>,
    pub r#calibrated_gyro_x: Option<Value>,
    pub r#calibrated_gyro_y: Option<Value>,
    pub r#calibrated_gyro_z: Option<Value>,
    pub r#timestamp: Option<Value>,
    #[doc = r" All fields that are not defined in the profile."]
    pub unknown_fields: Vec<FitDataField>,
}
impl FitMessage for GyroscopeData {
    const NAME: &'static str = "GyroscopeData";
    const KIND: MesgNum = MesgNum::GyroscopeData;
    fn parse(record: FitDataRecord) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#timestamp_ms = None;
        let mut r#sample_time_offset = None;
        let mut r#gyro_x = None;
        let mut r#gyro_y = None;
        let mut r#gyro_z = None;
        let mut r#calibrated_gyro_x = None;
        let mut r#calibrated_gyro_y = None;
        let mut r#calibrated_gyro_z = None;
        let mut r#timestamp = None;
        let mut unknown_fields = Vec::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => {
                    r#timestamp_ms = Some(field.into_value());
                }
                1u8 => {
                    r#sample_time_offset = Some(field.into_value());
                }
                2u8 => {
                    r#gyro_x = Some(field.into_value());
                }
                3u8 => {
                    r#gyro_y = Some(field.into_value());
                }
                4u8 => {
                    r#gyro_z = Some(field.into_value());
                }
                5u8 => {
                    r#calibrated_gyro_x = Some(field.into_value());
                }
                6u8 => {
                    r#calibrated_gyro_y = Some(field.into_value());
                }
                7u8 => {
                    r#calibrated_gyro_z = Some(field.into_value());
                }
                253u8 => {
                    r#timestamp = Some(field.into_value());
                }
                _ => {
                    unknown_fields.push(field);
                }
            }
        }
        Ok(Self {
            r#timestamp_ms,
            r#sample_time_offset,
            r#gyro_x,
            r#gyro_y,
            r#gyro_z,
            r#calibrated_gyro_x,
            r#calibrated_gyro_y,
            r#calibrated_gyro_z,
            r#timestamp,
            unknown_fields,
        })
    }
}
impl TryFrom<FitDataRecord> for GyroscopeData {
    type Error = TryFromRecordError;
    fn try_from(record: FitDataRecord) -> Result<Self, Self::Error> {
        Self::parse(record)
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize)]
pub struct AccelerometerData {
    pub r#timestamp_ms: Option<Value>,
    pub r#sample_time_offset: Option<Value>,
    pub r#accel_x: Option<Value>,
    pub r#accel_y: Option<Value>,
    pub r#accel_z: Option<Value>,
    pub r#calibrated_accel_x: Option<Value>,
    pub r#calibrated_accel_y: Option<Value>,
    pub r#calibrated_accel_z: Option<Value>,
    pub r#compressed_calibrated_accel_x: Option<Value>,
    pub r#compressed_calibrated_accel_y: Option<Value>,
    pub r#compressed_calibrated_accel_z: Option<Value>,
    pub r#timestamp: Option<Value>,
    #[doc = r" All fields that are not defined in the profile."]
    pub unknown_fields: Vec<FitDataField>,
}
impl FitMessage for AccelerometerData {
    const NAME: &'static str = "AccelerometerData";
    const KIND: MesgNum = MesgNum::AccelerometerData;
    fn parse(record: FitDataRecord) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#timestamp_ms = None;
        let mut r#sample_time_offset = None;
        let mut r#accel_x = None;
        let mut r#accel_y = None;
        let mut r#accel_z = None;
        let mut r#calibrated_accel_x = None;
        let mut r#calibrated_accel_y = None;
        let mut r#calibrated_accel_z = None;
        let mut r#compressed_calibrated_accel_x = None;
        let mut r#compressed_calibrated_accel_y = None;
        let mut r#compressed_calibrated_accel_z = None;
        let mut r#timestamp = None;
        let mut unknown_fields = Vec::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => {
                    r#timestamp_ms = Some(field.into_value());
                }
                1u8 => {
                    r#sample_time_offset = Some(field.into_value());
                }
                2u8 => {
                    r#accel_x = Some(field.into_value());
                }
                3u8 => {
                    r#accel_y = Some(field.into_value());
                }
                4u8 => {
                    r#accel_z = Some(field.into_value());
                }
                5u8 => {
                    r#calibrated_accel_x = Some(field.into_value());
                }
                6u8 => {
                    r#calibrated_accel_y = Some(field.into_value());
                }
                7u8 => {
                    r#calibrated_accel_z = Some(field.into_value());
                }
                8u8 => {
                    r#compressed_calibrated_accel_x = Some(field.into_value());
                }
                9u8 => {
                    r#compressed_calibrated_accel_y = Some(field.into_value());
                }
                10u8 => {
                    r#compressed_calibrated_accel_z = Some(field.into_value());
                }
                253u8 => {
                    r#timestamp = Some(field.into_value());
                }
                _ => {
                    unknown_fields.push(field);
                }
            }
        }
        Ok(Self {
            r#timestamp_ms,
            r#sample_time_offset,
            r#accel_x,
            r#accel_y,
            r#accel_z,
            r#calibrated_accel_x,
            r#calibrated_accel_y,
            r#calibrated_accel_z,
            r#compressed_calibrated_accel_x,
            r#compressed_calibrated_accel_y,
            r#compressed_calibrated_accel_z,
            r#timestamp,
            unknown_fields,
        })
    }
}
impl TryFrom<FitDataRecord> for AccelerometerData {
    type Error = TryFromRecordError;
    fn try_from(record: FitDataRecord) -> Result<Self, Self::Error> {
        Self::parse(record)
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize)]
pub struct MagnetometerData {
    pub r#timestamp_ms: Option<Value>,
    pub r#sample_time_offset: Option<Value>,
    pub r#mag_x: Option<Value>,
    pub r#mag_y: Option<Value>,
    pub r#mag_z: Option<Value>,
    pub r#calibrated_mag_x: Option<Value>,
    pub r#calibrated_mag_y: Option<Value>,
    pub r#calibrated_mag_z: Option<Value>,
    pub r#timestamp: Option<Value>,
    #[doc = r" All fields that are not defined in the profile."]
    pub unknown_fields: Vec<FitDataField>,
}
impl FitMessage for MagnetometerData {
    const NAME: &'static str = "MagnetometerData";
    const KIND: MesgNum = MesgNum::MagnetometerData;
    fn parse(record: FitDataRecord) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#timestamp_ms = None;
        let mut r#sample_time_offset = None;
        let mut r#mag_x = None;
        let mut r#mag_y = None;
        let mut r#mag_z = None;
        let mut r#calibrated_mag_x = None;
        let mut r#calibrated_mag_y = None;
        let mut r#calibrated_mag_z = None;
        let mut r#timestamp = None;
        let mut unknown_fields = Vec::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => {
                    r#timestamp_ms = Some(field.into_value());
                }
                1u8 => {
                    r#sample_time_offset = Some(field.into_value());
                }
                2u8 => {
                    r#mag_x = Some(field.into_value());
                }
                3u8 => {
                    r#mag_y = Some(field.into_value());
                }
                4u8 => {
                    r#mag_z = Some(field.into_value());
                }
                5u8 => {
                    r#calibrated_mag_x = Some(field.into_value());
                }
                6u8 => {
                    r#calibrated_mag_y = Some(field.into_value());
                }
                7u8 => {
                    r#calibrated_mag_z = Some(field.into_value());
                }
                253u8 => {
                    r#timestamp = Some(field.into_value());
                }
                _ => {
                    unknown_fields.push(field);
                }
            }
        }
        Ok(Self {
            r#timestamp_ms,
            r#sample_time_offset,
            r#mag_x,
            r#mag_y,
            r#mag_z,
            r#calibrated_mag_x,
            r#calibrated_mag_y,
            r#calibrated_mag_z,
            r#timestamp,
            unknown_fields,
        })
    }
}
impl TryFrom<FitDataRecord> for MagnetometerData {
    type Error = TryFromRecordError;
    fn try_from(record: FitDataRecord) -> Result<Self, Self::Error> {
        Self::parse(record)
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize)]
pub struct BarometerData {
    pub r#timestamp_ms: Option<Value>,
    pub r#sample_time_offset: Option<Value>,
    pub r#baro_pres: Option<Value>,
    pub r#timestamp: Option<Value>,
    #[doc = r" All fields that are not defined in the profile."]
    pub unknown_fields: Vec<FitDataField>,
}
impl FitMessage for BarometerData {
    const NAME: &'static str = "BarometerData";
    const KIND: MesgNum = MesgNum::BarometerData;
    fn parse(record: FitDataRecord) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#timestamp_ms = None;
        let mut r#sample_time_offset = None;
        let mut r#baro_pres = None;
        let mut r#timestamp = None;
        let mut unknown_fields = Vec::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => {
                    r#timestamp_ms = Some(field.into_value());
                }
                1u8 => {
                    r#sample_time_offset = Some(field.into_value());
                }
                2u8 => {
                    r#baro_pres = Some(field.into_value());
                }
                253u8 => {
                    r#timestamp = Some(field.into_value());
                }
                _ => {
                    unknown_fields.push(field);
                }
            }
        }
        Ok(Self {
            r#timestamp_ms,
            r#sample_time_offset,
            r#baro_pres,
            r#timestamp,
            unknown_fields,
        })
    }
}
impl TryFrom<FitDataRecord> for BarometerData {
    type Error = TryFromRecordError;
    fn try_from(record: FitDataRecord) -> Result<Self, Self::Error> {
        Self::parse(record)
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize)]
pub struct ThreeDSensorCalibration {
    pub r#sensor_type: Option<Value>,
    pub r#calibration_factor: Option<Value>,
    pub r#calibration_divisor: Option<Value>,
    pub r#level_shift: Option<Value>,
    pub r#offset_cal: Option<Value>,
    pub r#orientation_matrix: Option<Value>,
    pub r#timestamp: Option<Value>,
    #[doc = r" All fields that are not defined in the profile."]
    pub unknown_fields: Vec<FitDataField>,
}
impl FitMessage for ThreeDSensorCalibration {
    const NAME: &'static str = "ThreeDSensorCalibration";
    const KIND: MesgNum = MesgNum::ThreeDSensorCalibration;
    fn parse(record: FitDataRecord) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#sensor_type = None;
        let mut r#calibration_factor = None;
        let mut r#calibration_divisor = None;
        let mut r#level_shift = None;
        let mut r#offset_cal = None;
        let mut r#orientation_matrix = None;
        let mut r#timestamp = None;
        let mut unknown_fields = Vec::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => {
                    r#sensor_type = Some(field.into_value());
                }
                1u8 => {
                    r#calibration_factor = Some(field.into_value());
                }
                2u8 => {
                    r#calibration_divisor = Some(field.into_value());
                }
                3u8 => {
                    r#level_shift = Some(field.into_value());
                }
                4u8 => {
                    r#offset_cal = Some(field.into_value());
                }
                5u8 => {
                    r#orientation_matrix = Some(field.into_value());
                }
                253u8 => {
                    r#timestamp = Some(field.into_value());
                }
                _ => {
                    unknown_fields.push(field);
                }
            }
        }
        Ok(Self {
            r#sensor_type,
            r#calibration_factor,
            r#calibration_divisor,
            r#level_shift,
            r#offset_cal,
            r#orientation_matrix,
            r#timestamp,
            unknown_fields,
        })
    }
}
impl TryFrom<FitDataRecord> for ThreeDSensorCalibration {
    type Error = TryFromRecordError;
    fn try_from(record: FitDataRecord) -> Result<Self, Self::Error> {
        Self::parse(record)
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize)]
pub struct OneDSensorCalibration {
    pub r#sensor_type: Option<Value>,
    pub r#calibration_factor: Option<Value>,
    pub r#calibration_divisor: Option<Value>,
    pub r#level_shift: Option<Value>,
    pub r#offset_cal: Option<Value>,
    pub r#timestamp: Option<Value>,
    #[doc = r" All fields that are not defined in the profile."]
    pub unknown_fields: Vec<FitDataField>,
}
impl FitMessage for OneDSensorCalibration {
    const NAME: &'static str = "OneDSensorCalibration";
    const KIND: MesgNum = MesgNum::OneDSensorCalibration;
    fn parse(record: FitDataRecord) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#sensor_type = None;
        let mut r#calibration_factor = None;
        let mut r#calibration_divisor = None;
        let mut r#level_shift = None;
        let mut r#offset_cal = None;
        let mut r#timestamp = None;
        let mut unknown_fields = Vec::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => {
                    r#sensor_type = Some(field.into_value());
                }
                1u8 => {
                    r#calibration_factor = Some(field.into_value());
                }
                2u8 => {
                    r#calibration_divisor = Some(field.into_value());
                }
                3u8 => {
                    r#level_shift = Some(field.into_value());
                }
                4u8 => {
                    r#offset_cal = Some(field.into_value());
                }
                253u8 => {
                    r#timestamp = Some(field.into_value());
                }
                _ => {
                    unknown_fields.push(field);
                }
            }
        }
        Ok(Self {
            r#sensor_type,
            r#calibration_factor,
            r#calibration_divisor,
            r#level_shift,
            r#offset_cal,
            r#timestamp,
            unknown_fields,
        })
    }
}
impl TryFrom<FitDataRecord> for OneDSensorCalibration {
    type Error = TryFromRecordError;
    fn try_from(record: FitDataRecord) -> Result<Self, Self::Error> {
        Self::parse(record)
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize)]
pub struct VideoFrame {
    pub r#timestamp_ms: Option<Value>,
    pub r#frame_number: Option<Value>,
    pub r#timestamp: Option<Value>,
    #[doc = r" All fields that are not defined in the profile."]
    pub unknown_fields: Vec<FitDataField>,
}
impl FitMessage for VideoFrame {
    const NAME: &'static str = "VideoFrame";
    const KIND: MesgNum = MesgNum::VideoFrame;
    fn parse(record: FitDataRecord) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#timestamp_ms = None;
        let mut r#frame_number = None;
        let mut r#timestamp = None;
        let mut unknown_fields = Vec::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => {
                    r#timestamp_ms = Some(field.into_value());
                }
                1u8 => {
                    r#frame_number = Some(field.into_value());
                }
                253u8 => {
                    r#timestamp = Some(field.into_value());
                }
                _ => {
                    unknown_fields.push(field);
                }
            }
        }
        Ok(Self {
            r#timestamp_ms,
            r#frame_number,
            r#timestamp,
            unknown_fields,
        })
    }
}
impl TryFrom<FitDataRecord> for VideoFrame {
    type Error = TryFromRecordError;
    fn try_from(record: FitDataRecord) -> Result<Self, Self::Error> {
        Self::parse(record)
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize)]
pub struct ObdiiData {
    pub r#timestamp_ms: Option<Value>,
    pub r#time_offset: Option<Value>,
    pub r#pid: Option<Value>,
    pub r#raw_data: Option<Value>,
    pub r#pid_data_size: Option<Value>,
    pub r#system_time: Option<Value>,
    pub r#start_timestamp: Option<Value>,
    pub r#start_timestamp_ms: Option<Value>,
    pub r#timestamp: Option<Value>,
    #[doc = r" All fields that are not defined in the profile."]
    pub unknown_fields: Vec<FitDataField>,
}
impl FitMessage for ObdiiData {
    const NAME: &'static str = "ObdiiData";
    const KIND: MesgNum = MesgNum::ObdiiData;
    fn parse(record: FitDataRecord) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#timestamp_ms = None;
        let mut r#time_offset = None;
        let mut r#pid = None;
        let mut r#raw_data = None;
        let mut r#pid_data_size = None;
        let mut r#system_time = None;
        let mut r#start_timestamp = None;
        let mut r#start_timestamp_ms = None;
        let mut r#timestamp = None;
        let mut unknown_fields = Vec::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => {
                    r#timestamp_ms = Some(field.into_value());
                }
                1u8 => {
                    r#time_offset = Some(field.into_value());
                }
                2u8 => {
                    r#pid = Some(field.into_value());
                }
                3u8 => {
                    r#raw_data = Some(field.into_value());
                }
                4u8 => {
                    r#pid_data_size = Some(field.into_value());
                }
                5u8 => {
                    r#system_time = Some(field.into_value());
                }
                6u8 => {
                    r#start_timestamp = Some(field.into_value());
                }
                7u8 => {
                    r#start_timestamp_ms = Some(field.into_value());
                }
                253u8 => {
                    r#timestamp = Some(field.into_value());
                }
                _ => {
                    unknown_fields.push(field);
                }
            }
        }
        Ok(Self {
            r#timestamp_ms,
            r#time_offset,
            r#pid,
            r#raw_data,
            r#pid_data_size,
            r#system_time,
            r#start_timestamp,
            r#start_timestamp_ms,
            r#timestamp,
            unknown_fields,
        })
    }
}
impl TryFrom<FitDataRecord> for ObdiiData {
    type Error = TryFromRecordError;
    fn try_from(record: FitDataRecord) -> Result<Self, Self::Error> {
        Self::parse(record)
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize)]
pub struct NmeaSentence {
    pub r#timestamp_ms: Option<Value>,
    pub r#sentence: Option<Value>,
    pub r#timestamp: Option<Value>,
    #[doc = r" All fields that are not defined in the profile."]
    pub unknown_fields: Vec<FitDataField>,
}
impl FitMessage for NmeaSentence {
    const NAME: &'static str = "NmeaSentence";
    const KIND: MesgNum = MesgNum::NmeaSentence;
    fn parse(record: FitDataRecord) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#timestamp_ms = None;
        let mut r#sentence = None;
        let mut r#timestamp = None;
        let mut unknown_fields = Vec::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => {
                    r#timestamp_ms = Some(field.into_value());
                }
                1u8 => {
                    r#sentence = Some(field.into_value());
                }
                253u8 => {
                    r#timestamp = Some(field.into_value());
                }
                _ => {
                    unknown_fields.push(field);
                }
            }
        }
        Ok(Self {
            r#timestamp_ms,
            r#sentence,
            r#timestamp,
            unknown_fields,
        })
    }
}
impl TryFrom<FitDataRecord> for NmeaSentence {
    type Error = TryFromRecordError;
    fn try_from(record: FitDataRecord) -> Result<Self, Self::Error> {
        Self::parse(record)
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize)]
pub struct AviationAttitude {
    pub r#timestamp_ms: Option<Value>,
    pub r#system_time: Option<Value>,
    pub r#pitch: Option<Value>,
    pub r#roll: Option<Value>,
    pub r#accel_lateral: Option<Value>,
    pub r#accel_normal: Option<Value>,
    pub r#turn_rate: Option<Value>,
    pub r#stage: Option<Value>,
    pub r#attitude_stage_complete: Option<Value>,
    pub r#track: Option<Value>,
    pub r#validity: Option<Value>,
    pub r#timestamp: Option<Value>,
    #[doc = r" All fields that are not defined in the profile."]
    pub unknown_fields: Vec<FitDataField>,
}
impl FitMessage for AviationAttitude {
    const NAME: &'static str = "AviationAttitude";
    const KIND: MesgNum = MesgNum::AviationAttitude;
    fn parse(record: FitDataRecord) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#timestamp_ms = None;
        let mut r#system_time = None;
        let mut r#pitch = None;
        let mut r#roll = None;
        let mut r#accel_lateral = None;
        let mut r#accel_normal = None;
        let mut r#turn_rate = None;
        let mut r#stage = None;
        let mut r#attitude_stage_complete = None;
        let mut r#track = None;
        let mut r#validity = None;
        let mut r#timestamp = None;
        let mut unknown_fields = Vec::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => {
                    r#timestamp_ms = Some(field.into_value());
                }
                1u8 => {
                    r#system_time = Some(field.into_value());
                }
                2u8 => {
                    r#pitch = Some(field.into_value());
                }
                3u8 => {
                    r#roll = Some(field.into_value());
                }
                4u8 => {
                    r#accel_lateral = Some(field.into_value());
                }
                5u8 => {
                    r#accel_normal = Some(field.into_value());
                }
                6u8 => {
                    r#turn_rate = Some(field.into_value());
                }
                7u8 => {
                    r#stage = Some(field.into_value());
                }
                8u8 => {
                    r#attitude_stage_complete = Some(field.into_value());
                }
                9u8 => {
                    r#track = Some(field.into_value());
                }
                10u8 => {
                    r#validity = Some(field.into_value());
                }
                253u8 => {
                    r#timestamp = Some(field.into_value());
                }
                _ => {
                    unknown_fields.push(field);
                }
            }
        }
        Ok(Self {
            r#timestamp_ms,
            r#system_time,
            r#pitch,
            r#roll,
            r#accel_lateral,
            r#accel_normal,
            r#turn_rate,
            r#stage,
            r#attitude_stage_complete,
            r#track,
            r#validity,
            r#timestamp,
            unknown_fields,
        })
    }
}
impl TryFrom<FitDataRecord> for AviationAttitude {
    type Error = TryFromRecordError;
    fn try_from(record: FitDataRecord) -> Result<Self, Self::Error> {
        Self::parse(record)
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize)]
pub struct Video {
    pub r#url: Option<Value>,
    pub r#hosting_provider: Option<Value>,
    pub r#duration: Option<Value>,
    #[doc = r" All fields that are not defined in the profile."]
    pub unknown_fields: Vec<FitDataField>,
}
impl FitMessage for Video {
    const NAME: &'static str = "Video";
    const KIND: MesgNum = MesgNum::Video;
    fn parse(record: FitDataRecord) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#url = None;
        let mut r#hosting_provider = None;
        let mut r#duration = None;
        let mut unknown_fields = Vec::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => {
                    r#url = Some(field.into_value());
                }
                1u8 => {
                    r#hosting_provider = Some(field.into_value());
                }
                2u8 => {
                    r#duration = Some(field.into_value());
                }
                _ => {
                    unknown_fields.push(field);
                }
            }
        }
        Ok(Self {
            r#url,
            r#hosting_provider,
            r#duration,
            unknown_fields,
        })
    }
}
impl TryFrom<FitDataRecord> for Video {
    type Error = TryFromRecordError;
    fn try_from(record: FitDataRecord) -> Result<Self, Self::Error> {
        Self::parse(record)
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize)]
pub struct VideoTitle {
    pub r#message_count: Option<Value>,
    pub r#text: Option<Value>,
    pub r#message_index: Option<Value>,
    #[doc = r" All fields that are not defined in the profile."]
    pub unknown_fields: Vec<FitDataField>,
}
impl FitMessage for VideoTitle {
    const NAME: &'static str = "VideoTitle";
    const KIND: MesgNum = MesgNum::VideoTitle;
    fn parse(record: FitDataRecord) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#message_count = None;
        let mut r#text = None;
        let mut r#message_index = None;
        let mut unknown_fields = Vec::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => {
                    r#message_count = Some(field.into_value());
                }
                1u8 => {
                    r#text = Some(field.into_value());
                }
                254u8 => {
                    r#message_index = Some(field.into_value());
                }
                _ => {
                    unknown_fields.push(field);
                }
            }
        }
        Ok(Self {
            r#message_count,
            r#text,
            r#message_index,
            unknown_fields,
        })
    }
}
impl TryFrom<FitDataRecord> for VideoTitle {
    type Error = TryFromRecordError;
    fn try_from(record: FitDataRecord) -> Result<Self, Self::Error> {
        Self::parse(record)
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize)]
pub struct VideoDescription {
    pub r#message_count: Option<Value>,
    pub r#text: Option<Value>,
    pub r#message_index: Option<Value>,
    #[doc = r" All fields that are not defined in the profile."]
    pub unknown_fields: Vec<FitDataField>,
}
impl FitMessage for VideoDescription {
    const NAME: &'static str = "VideoDescription";
    const KIND: MesgNum = MesgNum::VideoDescription;
    fn parse(record: FitDataRecord) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#message_count = None;
        let mut r#text = None;
        let mut r#message_index = None;
        let mut unknown_fields = Vec::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => {
                    r#message_count = Some(field.into_value());
                }
                1u8 => {
                    r#text = Some(field.into_value());
                }
                254u8 => {
                    r#message_index = Some(field.into_value());
                }
                _ => {
                    unknown_fields.push(field);
                }
            }
        }
        Ok(Self {
            r#message_count,
            r#text,
            r#message_index,
            unknown_fields,
        })
    }
}
impl TryFrom<FitDataRecord> for VideoDescription {
    type Error = TryFromRecordError;
    fn try_from(record: FitDataRecord) -> Result<Self, Self::Error> {
        Self::parse(record)
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize)]
pub struct VideoClip {
    pub r#clip_number: Option<Value>,
    pub r#start_timestamp: Option<Value>,
    pub r#start_timestamp_ms: Option<Value>,
    pub r#end_timestamp: Option<Value>,
    pub r#end_timestamp_ms: Option<Value>,
    pub r#clip_start: Option<Value>,
    pub r#clip_end: Option<Value>,
    #[doc = r" All fields that are not defined in the profile."]
    pub unknown_fields: Vec<FitDataField>,
}
impl FitMessage for VideoClip {
    const NAME: &'static str = "VideoClip";
    const KIND: MesgNum = MesgNum::VideoClip;
    fn parse(record: FitDataRecord) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#clip_number = None;
        let mut r#start_timestamp = None;
        let mut r#start_timestamp_ms = None;
        let mut r#end_timestamp = None;
        let mut r#end_timestamp_ms = None;
        let mut r#clip_start = None;
        let mut r#clip_end = None;
        let mut unknown_fields = Vec::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => {
                    r#clip_number = Some(field.into_value());
                }
                1u8 => {
                    r#start_timestamp = Some(field.into_value());
                }
                2u8 => {
                    r#start_timestamp_ms = Some(field.into_value());
                }
                3u8 => {
                    r#end_timestamp = Some(field.into_value());
                }
                4u8 => {
                    r#end_timestamp_ms = Some(field.into_value());
                }
                6u8 => {
                    r#clip_start = Some(field.into_value());
                }
                7u8 => {
                    r#clip_end = Some(field.into_value());
                }
                _ => {
                    unknown_fields.push(field);
                }
            }
        }
        Ok(Self {
            r#clip_number,
            r#start_timestamp,
            r#start_timestamp_ms,
            r#end_timestamp,
            r#end_timestamp_ms,
            r#clip_start,
            r#clip_end,
            unknown_fields,
        })
    }
}
impl TryFrom<FitDataRecord> for VideoClip {
    type Error = TryFromRecordError;
    fn try_from(record: FitDataRecord) -> Result<Self, Self::Error> {
        Self::parse(record)
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize)]
pub struct Set {
    pub r#duration: Option<Value>,
    pub r#repetitions: Option<Value>,
    pub r#weight: Option<Value>,
    pub r#set_type: Option<Value>,
    pub r#start_time: Option<Value>,
    pub r#category: Option<Value>,
    pub r#category_subtype: Option<Value>,
    pub r#weight_display_unit: Option<Value>,
    pub r#message_index: Option<Value>,
    pub r#wkt_step_index: Option<Value>,
    pub r#timestamp: Option<Value>,
    #[doc = r" All fields that are not defined in the profile."]
    pub unknown_fields: Vec<FitDataField>,
}
impl FitMessage for Set {
    const NAME: &'static str = "Set";
    const KIND: MesgNum = MesgNum::Set;
    fn parse(record: FitDataRecord) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#duration = None;
        let mut r#repetitions = None;
        let mut r#weight = None;
        let mut r#set_type = None;
        let mut r#start_time = None;
        let mut r#category = None;
        let mut r#category_subtype = None;
        let mut r#weight_display_unit = None;
        let mut r#message_index = None;
        let mut r#wkt_step_index = None;
        let mut r#timestamp = None;
        let mut unknown_fields = Vec::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => {
                    r#duration = Some(field.into_value());
                }
                3u8 => {
                    r#repetitions = Some(field.into_value());
                }
                4u8 => {
                    r#weight = Some(field.into_value());
                }
                5u8 => {
                    r#set_type = Some(field.into_value());
                }
                6u8 => {
                    r#start_time = Some(field.into_value());
                }
                7u8 => {
                    r#category = Some(field.into_value());
                }
                8u8 => {
                    r#category_subtype = Some(field.into_value());
                }
                9u8 => {
                    r#weight_display_unit = Some(field.into_value());
                }
                10u8 => {
                    r#message_index = Some(field.into_value());
                }
                11u8 => {
                    r#wkt_step_index = Some(field.into_value());
                }
                254u8 => {
                    r#timestamp = Some(field.into_value());
                }
                _ => {
                    unknown_fields.push(field);
                }
            }
        }
        Ok(Self {
            r#duration,
            r#repetitions,
            r#weight,
            r#set_type,
            r#start_time,
            r#category,
            r#category_subtype,
            r#weight_display_unit,
            r#message_index,
            r#wkt_step_index,
            r#timestamp,
            unknown_fields,
        })
    }
}
impl TryFrom<FitDataRecord> for Set {
    type Error = TryFromRecordError;
    fn try_from(record: FitDataRecord) -> Result<Self, Self::Error> {
        Self::parse(record)
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize)]
pub struct Jump {
    pub r#distance: Option<Value>,
    pub r#height: Option<Value>,
    pub r#rotations: Option<Value>,
    pub r#hang_time: Option<Value>,
    pub r#score: Option<Value>,
    pub r#position_lat: Option<Value>,
    pub r#position_long: Option<Value>,
    pub r#speed: Option<Value>,
    pub r#enhanced_speed: Option<Value>,
    pub r#timestamp: Option<Value>,
    #[doc = r" All fields that are not defined in the profile."]
    pub unknown_fields: Vec<FitDataField>,
}
impl FitMessage for Jump {
    const NAME: &'static str = "Jump";
    const KIND: MesgNum = MesgNum::Jump;
    fn parse(record: FitDataRecord) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#distance = None;
        let mut r#height = None;
        let mut r#rotations = None;
        let mut r#hang_time = None;
        let mut r#score = None;
        let mut r#position_lat = None;
        let mut r#position_long = None;
        let mut r#speed = None;
        let mut r#enhanced_speed = None;
        let mut r#timestamp = None;
        let mut unknown_fields = Vec::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => {
                    r#distance = Some(field.into_value());
                }
                1u8 => {
                    r#height = Some(field.into_value());
                }
                2u8 => {
                    r#rotations = Some(field.into_value());
                }
                3u8 => {
                    r#hang_time = Some(field.into_value());
                }
                4u8 => {
                    r#score = Some(field.into_value());
                }
                5u8 => {
                    r#position_lat = Some(field.into_value());
                }
                6u8 => {
                    r#position_long = Some(field.into_value());
                }
                7u8 => {
                    r#speed = Some(field.into_value());
                }
                8u8 => {
                    r#enhanced_speed = Some(field.into_value());
                }
                253u8 => {
                    r#timestamp = Some(field.into_value());
                }
                _ => {
                    unknown_fields.push(field);
                }
            }
        }
        Ok(Self {
            r#distance,
            r#height,
            r#rotations,
            r#hang_time,
            r#score,
            r#position_lat,
            r#position_long,
            r#speed,
            r#enhanced_speed,
            r#timestamp,
            unknown_fields,
        })
    }
}
impl TryFrom<FitDataRecord> for Jump {
    type Error = TryFromRecordError;
    fn try_from(record: FitDataRecord) -> Result<Self, Self::Error> {
        Self::parse(record)
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize)]
pub struct Split {
    pub r#split_type: Option<Value>,
    pub r#total_elapsed_time: Option<Value>,
    pub r#total_timer_time: Option<Value>,
    pub r#total_distance: Option<Value>,
    pub r#avg_speed: Option<Value>,
    pub r#start_time: Option<Value>,
    pub r#total_ascent: Option<Value>,
    pub r#total_descent: Option<Value>,
    pub r#start_position_lat: Option<Value>,
    pub r#start_position_long: Option<Value>,
    pub r#end_position_lat: Option<Value>,
    pub r#end_position_long: Option<Value>,
    pub r#max_speed: Option<Value>,
    pub r#avg_vert_speed: Option<Value>,
    pub r#end_time: Option<Value>,
    pub r#total_calories: Option<Value>,
    pub r#start_elevation: Option<Value>,
    pub r#total_moving_time: Option<Value>,
    pub r#message_index: Option<Value>,
    #[doc = r" All fields that are not defined in the profile."]
    pub unknown_fields: Vec<FitDataField>,
}
impl FitMessage for Split {
    const NAME: &'static str = "Split";
    const KIND: MesgNum = MesgNum::Split;
    fn parse(record: FitDataRecord) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#split_type = None;
        let mut r#total_elapsed_time = None;
        let mut r#total_timer_time = None;
        let mut r#total_distance = None;
        let mut r#avg_speed = None;
        let mut r#start_time = None;
        let mut r#total_ascent = None;
        let mut r#total_descent = None;
        let mut r#start_position_lat = None;
        let mut r#start_position_long = None;
        let mut r#end_position_lat = None;
        let mut r#end_position_long = None;
        let mut r#max_speed = None;
        let mut r#avg_vert_speed = None;
        let mut r#end_time = None;
        let mut r#total_calories = None;
        let mut r#start_elevation = None;
        let mut r#total_moving_time = None;
        let mut r#message_index = None;
        let mut unknown_fields = Vec::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => {
                    r#split_type = Some(field.into_value());
                }
                1u8 => {
                    r#total_elapsed_time = Some(field.into_value());
                }
                2u8 => {
                    r#total_timer_time = Some(field.into_value());
                }
                3u8 => {
                    r#total_distance = Some(field.into_value());
                }
                4u8 => {
                    r#avg_speed = Some(field.into_value());
                }
                9u8 => {
                    r#start_time = Some(field.into_value());
                }
                13u8 => {
                    r#total_ascent = Some(field.into_value());
                }
                14u8 => {
                    r#total_descent = Some(field.into_value());
                }
                21u8 => {
                    r#start_position_lat = Some(field.into_value());
                }
                22u8 => {
                    r#start_position_long = Some(field.into_value());
                }
                23u8 => {
                    r#end_position_lat = Some(field.into_value());
                }
                24u8 => {
                    r#end_position_long = Some(field.into_value());
                }
                25u8 => {
                    r#max_speed = Some(field.into_value());
                }
                26u8 => {
                    r#avg_vert_speed = Some(field.into_value());
                }
                27u8 => {
                    r#end_time = Some(field.into_value());
                }
                28u8 => {
                    r#total_calories = Some(field.into_value());
                }
                74u8 => {
                    r#start_elevation = Some(field.into_value());
                }
                110u8 => {
                    r#total_moving_time = Some(field.into_value());
                }
                254u8 => {
                    r#message_index = Some(field.into_value());
                }
                _ => {
                    unknown_fields.push(field);
                }
            }
        }
        Ok(Self {
            r#split_type,
            r#total_elapsed_time,
            r#total_timer_time,
            r#total_distance,
            r#avg_speed,
            r#start_time,
            r#total_ascent,
            r#total_descent,
            r#start_position_lat,
            r#start_position_long,
            r#end_position_lat,
            r#end_position_long,
            r#max_speed,
            r#avg_vert_speed,
            r#end_time,
            r#total_calories,
            r#start_elevation,
            r#total_moving_time,
            r#message_index,
            unknown_fields,
        })
    }
}
impl TryFrom<FitDataRecord> for Split {
    type Error = TryFromRecordError;
    fn try_from(record: FitDataRecord) -> Result<Self, Self::Error> {
        Self::parse(record)
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize)]
pub struct SplitSummary {
    pub r#split_type: Option<Value>,
    pub r#num_splits: Option<Value>,
    pub r#total_timer_time: Option<Value>,
    pub r#total_distance: Option<Value>,
    pub r#avg_speed: Option<Value>,
    pub r#max_speed: Option<Value>,
    pub r#total_ascent: Option<Value>,
    pub r#total_descent: Option<Value>,
    pub r#avg_heart_rate: Option<Value>,
    pub r#max_heart_rate: Option<Value>,
    pub r#avg_vert_speed: Option<Value>,
    pub r#total_calories: Option<Value>,
    pub r#total_moving_time: Option<Value>,
    pub r#message_index: Option<Value>,
    #[doc = r" All fields that are not defined in the profile."]
    pub unknown_fields: Vec<FitDataField>,
}
impl FitMessage for SplitSummary {
    const NAME: &'static str = "SplitSummary";
    const KIND: MesgNum = MesgNum::SplitSummary;
    fn parse(record: FitDataRecord) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#split_type = None;
        let mut r#num_splits = None;
        let mut r#total_timer_time = None;
        let mut r#total_distance = None;
        let mut r#avg_speed = None;
        let mut r#max_speed = None;
        let mut r#total_ascent = None;
        let mut r#total_descent = None;
        let mut r#avg_heart_rate = None;
        let mut r#max_heart_rate = None;
        let mut r#avg_vert_speed = None;
        let mut r#total_calories = None;
        let mut r#total_moving_time = None;
        let mut r#message_index = None;
        let mut unknown_fields = Vec::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => {
                    r#split_type = Some(field.into_value());
                }
                3u8 => {
                    r#num_splits = Some(field.into_value());
                }
                4u8 => {
                    r#total_timer_time = Some(field.into_value());
                }
                5u8 => {
                    r#total_distance = Some(field.into_value());
                }
                6u8 => {
                    r#avg_speed = Some(field.into_value());
                }
                7u8 => {
                    r#max_speed = Some(field.into_value());
                }
                8u8 => {
                    r#total_ascent = Some(field.into_value());
                }
                9u8 => {
                    r#total_descent = Some(field.into_value());
                }
                10u8 => {
                    r#avg_heart_rate = Some(field.into_value());
                }
                11u8 => {
                    r#max_heart_rate = Some(field.into_value());
                }
                12u8 => {
                    r#avg_vert_speed = Some(field.into_value());
                }
                13u8 => {
                    r#total_calories = Some(field.into_value());
                }
                77u8 => {
                    r#total_moving_time = Some(field.into_value());
                }
                254u8 => {
                    r#message_index = Some(field.into_value());
                }
                _ => {
                    unknown_fields.push(field);
                }
            }
        }
        Ok(Self {
            r#split_type,
            r#num_splits,
            r#total_timer_time,
            r#total_distance,
            r#avg_speed,
            r#max_speed,
            r#total_ascent,
            r#total_descent,
            r#avg_heart_rate,
            r#max_heart_rate,
            r#avg_vert_speed,
            r#total_calories,
            r#total_moving_time,
            r#message_index,
            unknown_fields,
        })
    }
}
impl TryFrom<FitDataRecord> for SplitSummary {
    type Error = TryFromRecordError;
    fn try_from(record: FitDataRecord) -> Result<Self, Self::Error> {
        Self::parse(record)
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize)]
pub struct ClimbPro {
    pub r#position_lat: Option<Value>,
    pub r#position_long: Option<Value>,
    pub r#climb_pro_event: Option<Value>,
    pub r#climb_number: Option<Value>,
    pub r#climb_category: Option<Value>,
    pub r#current_dist: Option<Value>,
    pub r#timestamp: Option<Value>,
    #[doc = r" All fields that are not defined in the profile."]
    pub unknown_fields: Vec<FitDataField>,
}
impl FitMessage for ClimbPro {
    const NAME: &'static str = "ClimbPro";
    const KIND: MesgNum = MesgNum::ClimbPro;
    fn parse(record: FitDataRecord) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#position_lat = None;
        let mut r#position_long = None;
        let mut r#climb_pro_event = None;
        let mut r#climb_number = None;
        let mut r#climb_category = None;
        let mut r#current_dist = None;
        let mut r#timestamp = None;
        let mut unknown_fields = Vec::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => {
                    r#position_lat = Some(field.into_value());
                }
                1u8 => {
                    r#position_long = Some(field.into_value());
                }
                2u8 => {
                    r#climb_pro_event = Some(field.into_value());
                }
                3u8 => {
                    r#climb_number = Some(field.into_value());
                }
                4u8 => {
                    r#climb_category = Some(field.into_value());
                }
                5u8 => {
                    r#current_dist = Some(field.into_value());
                }
                253u8 => {
                    r#timestamp = Some(field.into_value());
                }
                _ => {
                    unknown_fields.push(field);
                }
            }
        }
        Ok(Self {
            r#position_lat,
            r#position_long,
            r#climb_pro_event,
            r#climb_number,
            r#climb_category,
            r#current_dist,
            r#timestamp,
            unknown_fields,
        })
    }
}
impl TryFrom<FitDataRecord> for ClimbPro {
    type Error = TryFromRecordError;
    fn try_from(record: FitDataRecord) -> Result<Self, Self::Error> {
        Self::parse(record)
    }
}
#[doc = "Must be logged before developer field is used"]
#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize)]
pub struct FieldDescription {
    pub r#developer_data_index: Option<Value>,
    pub r#field_definition_number: Option<Value>,
    pub r#fit_base_type_id: Option<Value>,
    pub r#field_name: Option<Value>,
    pub r#array: Option<Value>,
    pub r#components: Option<Value>,
    pub r#scale: Option<Value>,
    pub r#offset: Option<Value>,
    pub r#units: Option<Value>,
    pub r#bits: Option<Value>,
    pub r#accumulate: Option<Value>,
    pub r#fit_base_unit_id: Option<Value>,
    pub r#native_mesg_num: Option<Value>,
    pub r#native_field_num: Option<Value>,
    #[doc = r" All fields that are not defined in the profile."]
    pub unknown_fields: Vec<FitDataField>,
}
impl FitMessage for FieldDescription {
    const NAME: &'static str = "FieldDescription";
    const KIND: MesgNum = MesgNum::FieldDescription;
    fn parse(record: FitDataRecord) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#developer_data_index = None;
        let mut r#field_definition_number = None;
        let mut r#fit_base_type_id = None;
        let mut r#field_name = None;
        let mut r#array = None;
        let mut r#components = None;
        let mut r#scale = None;
        let mut r#offset = None;
        let mut r#units = None;
        let mut r#bits = None;
        let mut r#accumulate = None;
        let mut r#fit_base_unit_id = None;
        let mut r#native_mesg_num = None;
        let mut r#native_field_num = None;
        let mut unknown_fields = Vec::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => {
                    r#developer_data_index = Some(field.into_value());
                }
                1u8 => {
                    r#field_definition_number = Some(field.into_value());
                }
                2u8 => {
                    r#fit_base_type_id = Some(field.into_value());
                }
                3u8 => {
                    r#field_name = Some(field.into_value());
                }
                4u8 => {
                    r#array = Some(field.into_value());
                }
                5u8 => {
                    r#components = Some(field.into_value());
                }
                6u8 => {
                    r#scale = Some(field.into_value());
                }
                7u8 => {
                    r#offset = Some(field.into_value());
                }
                8u8 => {
                    r#units = Some(field.into_value());
                }
                9u8 => {
                    r#bits = Some(field.into_value());
                }
                10u8 => {
                    r#accumulate = Some(field.into_value());
                }
                13u8 => {
                    r#fit_base_unit_id = Some(field.into_value());
                }
                14u8 => {
                    r#native_mesg_num = Some(field.into_value());
                }
                15u8 => {
                    r#native_field_num = Some(field.into_value());
                }
                _ => {
                    unknown_fields.push(field);
                }
            }
        }
        Ok(Self {
            r#developer_data_index,
            r#field_definition_number,
            r#fit_base_type_id,
            r#field_name,
            r#array,
            r#components,
            r#scale,
            r#offset,
            r#units,
            r#bits,
            r#accumulate,
            r#fit_base_unit_id,
            r#native_mesg_num,
            r#native_field_num,
            unknown_fields,
        })
    }
}
impl TryFrom<FitDataRecord> for FieldDescription {
    type Error = TryFromRecordError;
    fn try_from(record: FitDataRecord) -> Result<Self, Self::Error> {
        Self::parse(record)
    }
}
#[doc = "Must be logged before field description"]
#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize)]
pub struct DeveloperDataId {
    pub r#developer_id: Option<Value>,
    pub r#application_id: Option<Value>,
    pub r#manufacturer_id: Option<Value>,
    pub r#developer_data_index: Option<Value>,
    pub r#application_version: Option<Value>,
    #[doc = r" All fields that are not defined in the profile."]
    pub unknown_fields: Vec<FitDataField>,
}
impl FitMessage for DeveloperDataId {
    const NAME: &'static str = "DeveloperDataId";
    const KIND: MesgNum = MesgNum::DeveloperDataId;
    fn parse(record: FitDataRecord) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#developer_id = None;
        let mut r#application_id = None;
        let mut r#manufacturer_id = None;
        let mut r#developer_data_index = None;
        let mut r#application_version = None;
        let mut unknown_fields = Vec::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => {
                    r#developer_id = Some(field.into_value());
                }
                1u8 => {
                    r#application_id = Some(field.into_value());
                }
                2u8 => {
                    r#manufacturer_id = Some(field.into_value());
                }
                3u8 => {
                    r#developer_data_index = Some(field.into_value());
                }
                4u8 => {
                    r#application_version = Some(field.into_value());
                }
                _ => {
                    unknown_fields.push(field);
                }
            }
        }
        Ok(Self {
            r#developer_id,
            r#application_id,
            r#manufacturer_id,
            r#developer_data_index,
            r#application_version,
            unknown_fields,
        })
    }
}
impl TryFrom<FitDataRecord> for DeveloperDataId {
    type Error = TryFromRecordError;
    fn try_from(record: FitDataRecord) -> Result<Self, Self::Error> {
        Self::parse(record)
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize)]
pub struct Course {
    pub r#sport: Option<Value>,
    pub r#name: Option<Value>,
    pub r#capabilities: Option<Value>,
    pub r#sub_sport: Option<Value>,
    #[doc = r" All fields that are not defined in the profile."]
    pub unknown_fields: Vec<FitDataField>,
}
impl FitMessage for Course {
    const NAME: &'static str = "Course";
    const KIND: MesgNum = MesgNum::Course;
    fn parse(record: FitDataRecord) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#sport = None;
        let mut r#name = None;
        let mut r#capabilities = None;
        let mut r#sub_sport = None;
        let mut unknown_fields = Vec::new();
        for field in record.into_vec() {
            match field.number() {
                4u8 => {
                    r#sport = Some(field.into_value());
                }
                5u8 => {
                    r#name = Some(field.into_value());
                }
                6u8 => {
                    r#capabilities = Some(field.into_value());
                }
                7u8 => {
                    r#sub_sport = Some(field.into_value());
                }
                _ => {
                    unknown_fields.push(field);
                }
            }
        }
        Ok(Self {
            r#sport,
            r#name,
            r#capabilities,
            r#sub_sport,
            unknown_fields,
        })
    }
}
impl TryFrom<FitDataRecord> for Course {
    type Error = TryFromRecordError;
    fn try_from(record: FitDataRecord) -> Result<Self, Self::Error> {
        Self::parse(record)
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize)]
pub struct CoursePoint {
    pub r#timestamp: Option<Value>,
    pub r#position_lat: Option<Value>,
    pub r#position_long: Option<Value>,
    pub r#distance: Option<Value>,
    pub r#type: Option<Value>,
    pub r#name: Option<Value>,
    pub r#favorite: Option<Value>,
    pub r#message_index: Option<Value>,
    #[doc = r" All fields that are not defined in the profile."]
    pub unknown_fields: Vec<FitDataField>,
}
impl FitMessage for CoursePoint {
    const NAME: &'static str = "CoursePoint";
    const KIND: MesgNum = MesgNum::CoursePoint;
    fn parse(record: FitDataRecord) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#timestamp = None;
        let mut r#position_lat = None;
        let mut r#position_long = None;
        let mut r#distance = None;
        let mut r#type = None;
        let mut r#name = None;
        let mut r#favorite = None;
        let mut r#message_index = None;
        let mut unknown_fields = Vec::new();
        for field in record.into_vec() {
            match field.number() {
                1u8 => {
                    r#timestamp = Some(field.into_value());
                }
                2u8 => {
                    r#position_lat = Some(field.into_value());
                }
                3u8 => {
                    r#position_long = Some(field.into_value());
                }
                4u8 => {
                    r#distance = Some(field.into_value());
                }
                5u8 => {
                    r#type = Some(field.into_value());
                }
                6u8 => {
                    r#name = Some(field.into_value());
                }
                8u8 => {
                    r#favorite = Some(field.into_value());
                }
                254u8 => {
                    r#message_index = Some(field.into_value());
                }
                _ => {
                    unknown_fields.push(field);
                }
            }
        }
        Ok(Self {
            r#timestamp,
            r#position_lat,
            r#position_long,
            r#distance,
            r#type,
            r#name,
            r#favorite,
            r#message_index,
            unknown_fields,
        })
    }
}
impl TryFrom<FitDataRecord> for CoursePoint {
    type Error = TryFromRecordError;
    fn try_from(record: FitDataRecord) -> Result<Self, Self::Error> {
        Self::parse(record)
    }
}
#[doc = "Unique Identification data for a segment file"]
#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize)]
pub struct SegmentId {
    pub r#name: Option<Value>,
    pub r#uuid: Option<Value>,
    pub r#sport: Option<Value>,
    pub r#enabled: Option<Value>,
    pub r#user_profile_primary_key: Option<Value>,
    pub r#device_id: Option<Value>,
    pub r#default_race_leader: Option<Value>,
    pub r#delete_status: Option<Value>,
    pub r#selection_type: Option<Value>,
    #[doc = r" All fields that are not defined in the profile."]
    pub unknown_fields: Vec<FitDataField>,
}
impl FitMessage for SegmentId {
    const NAME: &'static str = "SegmentId";
    const KIND: MesgNum = MesgNum::SegmentId;
    fn parse(record: FitDataRecord) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#name = None;
        let mut r#uuid = None;
        let mut r#sport = None;
        let mut r#enabled = None;
        let mut r#user_profile_primary_key = None;
        let mut r#device_id = None;
        let mut r#default_race_leader = None;
        let mut r#delete_status = None;
        let mut r#selection_type = None;
        let mut unknown_fields = Vec::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => {
                    r#name = Some(field.into_value());
                }
                1u8 => {
                    r#uuid = Some(field.into_value());
                }
                2u8 => {
                    r#sport = Some(field.into_value());
                }
                3u8 => {
                    r#enabled = Some(field.into_value());
                }
                4u8 => {
                    r#user_profile_primary_key = Some(field.into_value());
                }
                5u8 => {
                    r#device_id = Some(field.into_value());
                }
                6u8 => {
                    r#default_race_leader = Some(field.into_value());
                }
                7u8 => {
                    r#delete_status = Some(field.into_value());
                }
                8u8 => {
                    r#selection_type = Some(field.into_value());
                }
                _ => {
                    unknown_fields.push(field);
                }
            }
        }
        Ok(Self {
            r#name,
            r#uuid,
            r#sport,
            r#enabled,
            r#user_profile_primary_key,
            r#device_id,
            r#default_race_leader,
            r#delete_status,
            r#selection_type,
            unknown_fields,
        })
    }
}
impl TryFrom<FitDataRecord> for SegmentId {
    type Error = TryFromRecordError;
    fn try_from(record: FitDataRecord) -> Result<Self, Self::Error> {
        Self::parse(record)
    }
}
#[doc = "Unique Identification data for an individual segment leader within a segment file"]
#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize)]
pub struct SegmentLeaderboardEntry {
    pub r#name: Option<Value>,
    pub r#type: Option<Value>,
    pub r#group_primary_key: Option<Value>,
    pub r#activity_id: Option<Value>,
    pub r#segment_time: Option<Value>,
    pub r#activity_id_string: Option<Value>,
    pub r#message_index: Option<Value>,
    #[doc = r" All fields that are not defined in the profile."]
    pub unknown_fields: Vec<FitDataField>,
}
impl FitMessage for SegmentLeaderboardEntry {
    const NAME: &'static str = "SegmentLeaderboardEntry";
    const KIND: MesgNum = MesgNum::SegmentLeaderboardEntry;
    fn parse(record: FitDataRecord) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#name = None;
        let mut r#type = None;
        let mut r#group_primary_key = None;
        let mut r#activity_id = None;
        let mut r#segment_time = None;
        let mut r#activity_id_string = None;
        let mut r#message_index = None;
        let mut unknown_fields = Vec::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => {
                    r#name = Some(field.into_value());
                }
                1u8 => {
                    r#type = Some(field.into_value());
                }
                2u8 => {
                    r#group_primary_key = Some(field.into_value());
                }
                3u8 => {
                    r#activity_id = Some(field.into_value());
                }
                4u8 => {
                    r#segment_time = Some(field.into_value());
                }
                5u8 => {
                    r#activity_id_string = Some(field.into_value());
                }
                254u8 => {
                    r#message_index = Some(field.into_value());
                }
                _ => {
                    unknown_fields.push(field);
                }
            }
        }
        Ok(Self {
            r#name,
            r#type,
            r#group_primary_key,
            r#activity_id,
            r#segment_time,
            r#activity_id_string,
            r#message_index,
            unknown_fields,
        })
    }
}
impl TryFrom<FitDataRecord> for SegmentLeaderboardEntry {
    type Error = TryFromRecordError;
    fn try_from(record: FitDataRecord) -> Result<Self, Self::Error> {
        Self::parse(record)
    }
}
#[doc = "Navigation and race evaluation point for a segment decribing a point along the segment path and time it took each segment leader to reach that point"]
#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize)]
pub struct SegmentPoint {
    pub r#position_lat: Option<Value>,
    pub r#position_long: Option<Value>,
    pub r#distance: Option<Value>,
    pub r#altitude: Option<Value>,
    pub r#leader_time: Option<Value>,
    pub r#enhanced_altitude: Option<Value>,
    pub r#message_index: Option<Value>,
    #[doc = r" All fields that are not defined in the profile."]
    pub unknown_fields: Vec<FitDataField>,
}
impl FitMessage for SegmentPoint {
    const NAME: &'static str = "SegmentPoint";
    const KIND: MesgNum = MesgNum::SegmentPoint;
    fn parse(record: FitDataRecord) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#position_lat = None;
        let mut r#position_long = None;
        let mut r#distance = None;
        let mut r#altitude = None;
        let mut r#leader_time = None;
        let mut r#enhanced_altitude = None;
        let mut r#message_index = None;
        let mut unknown_fields = Vec::new();
        for field in record.into_vec() {
            match field.number() {
                1u8 => {
                    r#position_lat = Some(field.into_value());
                }
                2u8 => {
                    r#position_long = Some(field.into_value());
                }
                3u8 => {
                    r#distance = Some(field.into_value());
                }
                4u8 => {
                    r#altitude = Some(field.into_value());
                }
                5u8 => {
                    r#leader_time = Some(field.into_value());
                }
                6u8 => {
                    r#enhanced_altitude = Some(field.into_value());
                }
                254u8 => {
                    r#message_index = Some(field.into_value());
                }
                _ => {
                    unknown_fields.push(field);
                }
            }
        }
        Ok(Self {
            r#position_lat,
            r#position_long,
            r#distance,
            r#altitude,
            r#leader_time,
            r#enhanced_altitude,
            r#message_index,
            unknown_fields,
        })
    }
}
impl TryFrom<FitDataRecord> for SegmentPoint {
    type Error = TryFromRecordError;
    fn try_from(record: FitDataRecord) -> Result<Self, Self::Error> {
        Self::parse(record)
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize)]
pub struct SegmentLap {
    pub r#event: Option<Value>,
    pub r#event_type: Option<Value>,
    pub r#start_time: Option<Value>,
    pub r#start_position_lat: Option<Value>,
    pub r#start_position_long: Option<Value>,
    pub r#end_position_lat: Option<Value>,
    pub r#end_position_long: Option<Value>,
    pub r#total_elapsed_time: Option<Value>,
    pub r#total_timer_time: Option<Value>,
    pub r#total_distance: Option<Value>,
    pub r#total_cycles: Option<Value>,
    pub r#total_calories: Option<Value>,
    pub r#total_fat_calories: Option<Value>,
    pub r#avg_speed: Option<Value>,
    pub r#max_speed: Option<Value>,
    pub r#avg_heart_rate: Option<Value>,
    pub r#max_heart_rate: Option<Value>,
    pub r#avg_cadence: Option<Value>,
    pub r#max_cadence: Option<Value>,
    pub r#avg_power: Option<Value>,
    pub r#max_power: Option<Value>,
    pub r#total_ascent: Option<Value>,
    pub r#total_descent: Option<Value>,
    pub r#sport: Option<Value>,
    pub r#event_group: Option<Value>,
    pub r#nec_lat: Option<Value>,
    pub r#nec_long: Option<Value>,
    pub r#swc_lat: Option<Value>,
    pub r#swc_long: Option<Value>,
    pub r#name: Option<Value>,
    pub r#normalized_power: Option<Value>,
    pub r#left_right_balance: Option<Value>,
    pub r#sub_sport: Option<Value>,
    pub r#total_work: Option<Value>,
    pub r#avg_altitude: Option<Value>,
    pub r#max_altitude: Option<Value>,
    pub r#gps_accuracy: Option<Value>,
    pub r#avg_grade: Option<Value>,
    pub r#avg_pos_grade: Option<Value>,
    pub r#avg_neg_grade: Option<Value>,
    pub r#max_pos_grade: Option<Value>,
    pub r#max_neg_grade: Option<Value>,
    pub r#avg_temperature: Option<Value>,
    pub r#max_temperature: Option<Value>,
    pub r#total_moving_time: Option<Value>,
    pub r#avg_pos_vertical_speed: Option<Value>,
    pub r#avg_neg_vertical_speed: Option<Value>,
    pub r#max_pos_vertical_speed: Option<Value>,
    pub r#max_neg_vertical_speed: Option<Value>,
    pub r#time_in_hr_zone: Option<Value>,
    pub r#time_in_speed_zone: Option<Value>,
    pub r#time_in_cadence_zone: Option<Value>,
    pub r#time_in_power_zone: Option<Value>,
    pub r#repetition_num: Option<Value>,
    pub r#min_altitude: Option<Value>,
    pub r#min_heart_rate: Option<Value>,
    pub r#active_time: Option<Value>,
    pub r#wkt_step_index: Option<Value>,
    pub r#sport_event: Option<Value>,
    pub r#avg_left_torque_effectiveness: Option<Value>,
    pub r#avg_right_torque_effectiveness: Option<Value>,
    pub r#avg_left_pedal_smoothness: Option<Value>,
    pub r#avg_right_pedal_smoothness: Option<Value>,
    pub r#avg_combined_pedal_smoothness: Option<Value>,
    pub r#status: Option<Value>,
    pub r#uuid: Option<Value>,
    pub r#avg_fractional_cadence: Option<Value>,
    pub r#max_fractional_cadence: Option<Value>,
    pub r#total_fractional_cycles: Option<Value>,
    pub r#front_gear_shift_count: Option<Value>,
    pub r#rear_gear_shift_count: Option<Value>,
    pub r#time_standing: Option<Value>,
    pub r#stand_count: Option<Value>,
    pub r#avg_left_pco: Option<Value>,
    pub r#avg_right_pco: Option<Value>,
    pub r#avg_left_power_phase: Option<Value>,
    pub r#avg_left_power_phase_peak: Option<Value>,
    pub r#avg_right_power_phase: Option<Value>,
    pub r#avg_right_power_phase_peak: Option<Value>,
    pub r#avg_power_position: Option<Value>,
    pub r#max_power_position: Option<Value>,
    pub r#avg_cadence_position: Option<Value>,
    pub r#max_cadence_position: Option<Value>,
    pub r#manufacturer: Option<Value>,
    pub r#total_grit: Option<Value>,
    pub r#total_flow: Option<Value>,
    pub r#avg_grit: Option<Value>,
    pub r#avg_flow: Option<Value>,
    pub r#total_fractional_ascent: Option<Value>,
    pub r#total_fractional_descent: Option<Value>,
    pub r#enhanced_avg_altitude: Option<Value>,
    pub r#enhanced_max_altitude: Option<Value>,
    pub r#enhanced_min_altitude: Option<Value>,
    pub r#timestamp: Option<Value>,
    pub r#message_index: Option<Value>,
    #[doc = r" All fields that are not defined in the profile."]
    pub unknown_fields: Vec<FitDataField>,
}
impl FitMessage for SegmentLap {
    const NAME: &'static str = "SegmentLap";
    const KIND: MesgNum = MesgNum::SegmentLap;
    fn parse(record: FitDataRecord) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#event = None;
        let mut r#event_type = None;
        let mut r#start_time = None;
        let mut r#start_position_lat = None;
        let mut r#start_position_long = None;
        let mut r#end_position_lat = None;
        let mut r#end_position_long = None;
        let mut r#total_elapsed_time = None;
        let mut r#total_timer_time = None;
        let mut r#total_distance = None;
        let mut r#total_cycles = None;
        let mut r#total_calories = None;
        let mut r#total_fat_calories = None;
        let mut r#avg_speed = None;
        let mut r#max_speed = None;
        let mut r#avg_heart_rate = None;
        let mut r#max_heart_rate = None;
        let mut r#avg_cadence = None;
        let mut r#max_cadence = None;
        let mut r#avg_power = None;
        let mut r#max_power = None;
        let mut r#total_ascent = None;
        let mut r#total_descent = None;
        let mut r#sport = None;
        let mut r#event_group = None;
        let mut r#nec_lat = None;
        let mut r#nec_long = None;
        let mut r#swc_lat = None;
        let mut r#swc_long = None;
        let mut r#name = None;
        let mut r#normalized_power = None;
        let mut r#left_right_balance = None;
        let mut r#sub_sport = None;
        let mut r#total_work = None;
        let mut r#avg_altitude = None;
        let mut r#max_altitude = None;
        let mut r#gps_accuracy = None;
        let mut r#avg_grade = None;
        let mut r#avg_pos_grade = None;
        let mut r#avg_neg_grade = None;
        let mut r#max_pos_grade = None;
        let mut r#max_neg_grade = None;
        let mut r#avg_temperature = None;
        let mut r#max_temperature = None;
        let mut r#total_moving_time = None;
        let mut r#avg_pos_vertical_speed = None;
        let mut r#avg_neg_vertical_speed = None;
        let mut r#max_pos_vertical_speed = None;
        let mut r#max_neg_vertical_speed = None;
        let mut r#time_in_hr_zone = None;
        let mut r#time_in_speed_zone = None;
        let mut r#time_in_cadence_zone = None;
        let mut r#time_in_power_zone = None;
        let mut r#repetition_num = None;
        let mut r#min_altitude = None;
        let mut r#min_heart_rate = None;
        let mut r#active_time = None;
        let mut r#wkt_step_index = None;
        let mut r#sport_event = None;
        let mut r#avg_left_torque_effectiveness = None;
        let mut r#avg_right_torque_effectiveness = None;
        let mut r#avg_left_pedal_smoothness = None;
        let mut r#avg_right_pedal_smoothness = None;
        let mut r#avg_combined_pedal_smoothness = None;
        let mut r#status = None;
        let mut r#uuid = None;
        let mut r#avg_fractional_cadence = None;
        let mut r#max_fractional_cadence = None;
        let mut r#total_fractional_cycles = None;
        let mut r#front_gear_shift_count = None;
        let mut r#rear_gear_shift_count = None;
        let mut r#time_standing = None;
        let mut r#stand_count = None;
        let mut r#avg_left_pco = None;
        let mut r#avg_right_pco = None;
        let mut r#avg_left_power_phase = None;
        let mut r#avg_left_power_phase_peak = None;
        let mut r#avg_right_power_phase = None;
        let mut r#avg_right_power_phase_peak = None;
        let mut r#avg_power_position = None;
        let mut r#max_power_position = None;
        let mut r#avg_cadence_position = None;
        let mut r#max_cadence_position = None;
        let mut r#manufacturer = None;
        let mut r#total_grit = None;
        let mut r#total_flow = None;
        let mut r#avg_grit = None;
        let mut r#avg_flow = None;
        let mut r#total_fractional_ascent = None;
        let mut r#total_fractional_descent = None;
        let mut r#enhanced_avg_altitude = None;
        let mut r#enhanced_max_altitude = None;
        let mut r#enhanced_min_altitude = None;
        let mut r#timestamp = None;
        let mut r#message_index = None;
        let mut unknown_fields = Vec::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => {
                    r#event = Some(field.into_value());
                }
                1u8 => {
                    r#event_type = Some(field.into_value());
                }
                2u8 => {
                    r#start_time = Some(field.into_value());
                }
                3u8 => {
                    r#start_position_lat = Some(field.into_value());
                }
                4u8 => {
                    r#start_position_long = Some(field.into_value());
                }
                5u8 => {
                    r#end_position_lat = Some(field.into_value());
                }
                6u8 => {
                    r#end_position_long = Some(field.into_value());
                }
                7u8 => {
                    r#total_elapsed_time = Some(field.into_value());
                }
                8u8 => {
                    r#total_timer_time = Some(field.into_value());
                }
                9u8 => {
                    r#total_distance = Some(field.into_value());
                }
                10u8 => {
                    r#total_cycles = Some(field.into_value());
                }
                11u8 => {
                    r#total_calories = Some(field.into_value());
                }
                12u8 => {
                    r#total_fat_calories = Some(field.into_value());
                }
                13u8 => {
                    r#avg_speed = Some(field.into_value());
                }
                14u8 => {
                    r#max_speed = Some(field.into_value());
                }
                15u8 => {
                    r#avg_heart_rate = Some(field.into_value());
                }
                16u8 => {
                    r#max_heart_rate = Some(field.into_value());
                }
                17u8 => {
                    r#avg_cadence = Some(field.into_value());
                }
                18u8 => {
                    r#max_cadence = Some(field.into_value());
                }
                19u8 => {
                    r#avg_power = Some(field.into_value());
                }
                20u8 => {
                    r#max_power = Some(field.into_value());
                }
                21u8 => {
                    r#total_ascent = Some(field.into_value());
                }
                22u8 => {
                    r#total_descent = Some(field.into_value());
                }
                23u8 => {
                    r#sport = Some(field.into_value());
                }
                24u8 => {
                    r#event_group = Some(field.into_value());
                }
                25u8 => {
                    r#nec_lat = Some(field.into_value());
                }
                26u8 => {
                    r#nec_long = Some(field.into_value());
                }
                27u8 => {
                    r#swc_lat = Some(field.into_value());
                }
                28u8 => {
                    r#swc_long = Some(field.into_value());
                }
                29u8 => {
                    r#name = Some(field.into_value());
                }
                30u8 => {
                    r#normalized_power = Some(field.into_value());
                }
                31u8 => {
                    r#left_right_balance = Some(field.into_value());
                }
                32u8 => {
                    r#sub_sport = Some(field.into_value());
                }
                33u8 => {
                    r#total_work = Some(field.into_value());
                }
                34u8 => {
                    r#avg_altitude = Some(field.into_value());
                }
                35u8 => {
                    r#max_altitude = Some(field.into_value());
                }
                36u8 => {
                    r#gps_accuracy = Some(field.into_value());
                }
                37u8 => {
                    r#avg_grade = Some(field.into_value());
                }
                38u8 => {
                    r#avg_pos_grade = Some(field.into_value());
                }
                39u8 => {
                    r#avg_neg_grade = Some(field.into_value());
                }
                40u8 => {
                    r#max_pos_grade = Some(field.into_value());
                }
                41u8 => {
                    r#max_neg_grade = Some(field.into_value());
                }
                42u8 => {
                    r#avg_temperature = Some(field.into_value());
                }
                43u8 => {
                    r#max_temperature = Some(field.into_value());
                }
                44u8 => {
                    r#total_moving_time = Some(field.into_value());
                }
                45u8 => {
                    r#avg_pos_vertical_speed = Some(field.into_value());
                }
                46u8 => {
                    r#avg_neg_vertical_speed = Some(field.into_value());
                }
                47u8 => {
                    r#max_pos_vertical_speed = Some(field.into_value());
                }
                48u8 => {
                    r#max_neg_vertical_speed = Some(field.into_value());
                }
                49u8 => {
                    r#time_in_hr_zone = Some(field.into_value());
                }
                50u8 => {
                    r#time_in_speed_zone = Some(field.into_value());
                }
                51u8 => {
                    r#time_in_cadence_zone = Some(field.into_value());
                }
                52u8 => {
                    r#time_in_power_zone = Some(field.into_value());
                }
                53u8 => {
                    r#repetition_num = Some(field.into_value());
                }
                54u8 => {
                    r#min_altitude = Some(field.into_value());
                }
                55u8 => {
                    r#min_heart_rate = Some(field.into_value());
                }
                56u8 => {
                    r#active_time = Some(field.into_value());
                }
                57u8 => {
                    r#wkt_step_index = Some(field.into_value());
                }
                58u8 => {
                    r#sport_event = Some(field.into_value());
                }
                59u8 => {
                    r#avg_left_torque_effectiveness = Some(field.into_value());
                }
                60u8 => {
                    r#avg_right_torque_effectiveness = Some(field.into_value());
                }
                61u8 => {
                    r#avg_left_pedal_smoothness = Some(field.into_value());
                }
                62u8 => {
                    r#avg_right_pedal_smoothness = Some(field.into_value());
                }
                63u8 => {
                    r#avg_combined_pedal_smoothness = Some(field.into_value());
                }
                64u8 => {
                    r#status = Some(field.into_value());
                }
                65u8 => {
                    r#uuid = Some(field.into_value());
                }
                66u8 => {
                    r#avg_fractional_cadence = Some(field.into_value());
                }
                67u8 => {
                    r#max_fractional_cadence = Some(field.into_value());
                }
                68u8 => {
                    r#total_fractional_cycles = Some(field.into_value());
                }
                69u8 => {
                    r#front_gear_shift_count = Some(field.into_value());
                }
                70u8 => {
                    r#rear_gear_shift_count = Some(field.into_value());
                }
                71u8 => {
                    r#time_standing = Some(field.into_value());
                }
                72u8 => {
                    r#stand_count = Some(field.into_value());
                }
                73u8 => {
                    r#avg_left_pco = Some(field.into_value());
                }
                74u8 => {
                    r#avg_right_pco = Some(field.into_value());
                }
                75u8 => {
                    r#avg_left_power_phase = Some(field.into_value());
                }
                76u8 => {
                    r#avg_left_power_phase_peak = Some(field.into_value());
                }
                77u8 => {
                    r#avg_right_power_phase = Some(field.into_value());
                }
                78u8 => {
                    r#avg_right_power_phase_peak = Some(field.into_value());
                }
                79u8 => {
                    r#avg_power_position = Some(field.into_value());
                }
                80u8 => {
                    r#max_power_position = Some(field.into_value());
                }
                81u8 => {
                    r#avg_cadence_position = Some(field.into_value());
                }
                82u8 => {
                    r#max_cadence_position = Some(field.into_value());
                }
                83u8 => {
                    r#manufacturer = Some(field.into_value());
                }
                84u8 => {
                    r#total_grit = Some(field.into_value());
                }
                85u8 => {
                    r#total_flow = Some(field.into_value());
                }
                86u8 => {
                    r#avg_grit = Some(field.into_value());
                }
                87u8 => {
                    r#avg_flow = Some(field.into_value());
                }
                89u8 => {
                    r#total_fractional_ascent = Some(field.into_value());
                }
                90u8 => {
                    r#total_fractional_descent = Some(field.into_value());
                }
                91u8 => {
                    r#enhanced_avg_altitude = Some(field.into_value());
                }
                92u8 => {
                    r#enhanced_max_altitude = Some(field.into_value());
                }
                93u8 => {
                    r#enhanced_min_altitude = Some(field.into_value());
                }
                253u8 => {
                    r#timestamp = Some(field.into_value());
                }
                254u8 => {
                    r#message_index = Some(field.into_value());
                }
                _ => {
                    unknown_fields.push(field);
                }
            }
        }
        Ok(Self {
            r#event,
            r#event_type,
            r#start_time,
            r#start_position_lat,
            r#start_position_long,
            r#end_position_lat,
            r#end_position_long,
            r#total_elapsed_time,
            r#total_timer_time,
            r#total_distance,
            r#total_cycles,
            r#total_calories,
            r#total_fat_calories,
            r#avg_speed,
            r#max_speed,
            r#avg_heart_rate,
            r#max_heart_rate,
            r#avg_cadence,
            r#max_cadence,
            r#avg_power,
            r#max_power,
            r#total_ascent,
            r#total_descent,
            r#sport,
            r#event_group,
            r#nec_lat,
            r#nec_long,
            r#swc_lat,
            r#swc_long,
            r#name,
            r#normalized_power,
            r#left_right_balance,
            r#sub_sport,
            r#total_work,
            r#avg_altitude,
            r#max_altitude,
            r#gps_accuracy,
            r#avg_grade,
            r#avg_pos_grade,
            r#avg_neg_grade,
            r#max_pos_grade,
            r#max_neg_grade,
            r#avg_temperature,
            r#max_temperature,
            r#total_moving_time,
            r#avg_pos_vertical_speed,
            r#avg_neg_vertical_speed,
            r#max_pos_vertical_speed,
            r#max_neg_vertical_speed,
            r#time_in_hr_zone,
            r#time_in_speed_zone,
            r#time_in_cadence_zone,
            r#time_in_power_zone,
            r#repetition_num,
            r#min_altitude,
            r#min_heart_rate,
            r#active_time,
            r#wkt_step_index,
            r#sport_event,
            r#avg_left_torque_effectiveness,
            r#avg_right_torque_effectiveness,
            r#avg_left_pedal_smoothness,
            r#avg_right_pedal_smoothness,
            r#avg_combined_pedal_smoothness,
            r#status,
            r#uuid,
            r#avg_fractional_cadence,
            r#max_fractional_cadence,
            r#total_fractional_cycles,
            r#front_gear_shift_count,
            r#rear_gear_shift_count,
            r#time_standing,
            r#stand_count,
            r#avg_left_pco,
            r#avg_right_pco,
            r#avg_left_power_phase,
            r#avg_left_power_phase_peak,
            r#avg_right_power_phase,
            r#avg_right_power_phase_peak,
            r#avg_power_position,
            r#max_power_position,
            r#avg_cadence_position,
            r#max_cadence_position,
            r#manufacturer,
            r#total_grit,
            r#total_flow,
            r#avg_grit,
            r#avg_flow,
            r#total_fractional_ascent,
            r#total_fractional_descent,
            r#enhanced_avg_altitude,
            r#enhanced_max_altitude,
            r#enhanced_min_altitude,
            r#timestamp,
            r#message_index,
            unknown_fields,
        })
    }
}
impl TryFrom<FitDataRecord> for SegmentLap {
    type Error = TryFromRecordError;
    fn try_from(record: FitDataRecord) -> Result<Self, Self::Error> {
        Self::parse(record)
    }
}
#[doc = "Summary of the unique segment and leaderboard information associated with a segment file. This message is used to compile a segment list file describing all segment files on a device. The segment list file is used when refreshing the contents of a segment file with the latest available leaderboard information."]
#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize)]
pub struct SegmentFile {
    pub r#file_uuid: Option<Value>,
    pub r#enabled: Option<Value>,
    pub r#user_profile_primary_key: Option<Value>,
    pub r#leader_type: Option<Value>,
    pub r#leader_group_primary_key: Option<Value>,
    pub r#leader_activity_id: Option<Value>,
    pub r#leader_activity_id_string: Option<Value>,
    pub r#default_race_leader: Option<Value>,
    pub r#message_index: Option<Value>,
    #[doc = r" All fields that are not defined in the profile."]
    pub unknown_fields: Vec<FitDataField>,
}
impl FitMessage for SegmentFile {
    const NAME: &'static str = "SegmentFile";
    const KIND: MesgNum = MesgNum::SegmentFile;
    fn parse(record: FitDataRecord) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#file_uuid = None;
        let mut r#enabled = None;
        let mut r#user_profile_primary_key = None;
        let mut r#leader_type = None;
        let mut r#leader_group_primary_key = None;
        let mut r#leader_activity_id = None;
        let mut r#leader_activity_id_string = None;
        let mut r#default_race_leader = None;
        let mut r#message_index = None;
        let mut unknown_fields = Vec::new();
        for field in record.into_vec() {
            match field.number() {
                1u8 => {
                    r#file_uuid = Some(field.into_value());
                }
                3u8 => {
                    r#enabled = Some(field.into_value());
                }
                4u8 => {
                    r#user_profile_primary_key = Some(field.into_value());
                }
                7u8 => {
                    r#leader_type = Some(field.into_value());
                }
                8u8 => {
                    r#leader_group_primary_key = Some(field.into_value());
                }
                9u8 => {
                    r#leader_activity_id = Some(field.into_value());
                }
                10u8 => {
                    r#leader_activity_id_string = Some(field.into_value());
                }
                11u8 => {
                    r#default_race_leader = Some(field.into_value());
                }
                254u8 => {
                    r#message_index = Some(field.into_value());
                }
                _ => {
                    unknown_fields.push(field);
                }
            }
        }
        Ok(Self {
            r#file_uuid,
            r#enabled,
            r#user_profile_primary_key,
            r#leader_type,
            r#leader_group_primary_key,
            r#leader_activity_id,
            r#leader_activity_id_string,
            r#default_race_leader,
            r#message_index,
            unknown_fields,
        })
    }
}
impl TryFrom<FitDataRecord> for SegmentFile {
    type Error = TryFromRecordError;
    fn try_from(record: FitDataRecord) -> Result<Self, Self::Error> {
        Self::parse(record)
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize)]
pub struct Workout {
    pub r#sport: Option<Value>,
    pub r#capabilities: Option<Value>,
    pub r#num_valid_steps: Option<Value>,
    pub r#wkt_name: Option<Value>,
    pub r#sub_sport: Option<Value>,
    pub r#pool_length: Option<Value>,
    pub r#pool_length_unit: Option<Value>,
    pub r#message_index: Option<Value>,
    #[doc = r" All fields that are not defined in the profile."]
    pub unknown_fields: Vec<FitDataField>,
}
impl FitMessage for Workout {
    const NAME: &'static str = "Workout";
    const KIND: MesgNum = MesgNum::Workout;
    fn parse(record: FitDataRecord) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#sport = None;
        let mut r#capabilities = None;
        let mut r#num_valid_steps = None;
        let mut r#wkt_name = None;
        let mut r#sub_sport = None;
        let mut r#pool_length = None;
        let mut r#pool_length_unit = None;
        let mut r#message_index = None;
        let mut unknown_fields = Vec::new();
        for field in record.into_vec() {
            match field.number() {
                4u8 => {
                    r#sport = Some(field.into_value());
                }
                5u8 => {
                    r#capabilities = Some(field.into_value());
                }
                6u8 => {
                    r#num_valid_steps = Some(field.into_value());
                }
                8u8 => {
                    r#wkt_name = Some(field.into_value());
                }
                11u8 => {
                    r#sub_sport = Some(field.into_value());
                }
                14u8 => {
                    r#pool_length = Some(field.into_value());
                }
                15u8 => {
                    r#pool_length_unit = Some(field.into_value());
                }
                254u8 => {
                    r#message_index = Some(field.into_value());
                }
                _ => {
                    unknown_fields.push(field);
                }
            }
        }
        Ok(Self {
            r#sport,
            r#capabilities,
            r#num_valid_steps,
            r#wkt_name,
            r#sub_sport,
            r#pool_length,
            r#pool_length_unit,
            r#message_index,
            unknown_fields,
        })
    }
}
impl TryFrom<FitDataRecord> for Workout {
    type Error = TryFromRecordError;
    fn try_from(record: FitDataRecord) -> Result<Self, Self::Error> {
        Self::parse(record)
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize)]
pub struct WorkoutSession {
    pub r#sport: Option<Value>,
    pub r#sub_sport: Option<Value>,
    pub r#num_valid_steps: Option<Value>,
    pub r#first_step_index: Option<Value>,
    pub r#pool_length: Option<Value>,
    pub r#pool_length_unit: Option<Value>,
    pub r#message_index: Option<Value>,
    #[doc = r" All fields that are not defined in the profile."]
    pub unknown_fields: Vec<FitDataField>,
}
impl FitMessage for WorkoutSession {
    const NAME: &'static str = "WorkoutSession";
    const KIND: MesgNum = MesgNum::WorkoutSession;
    fn parse(record: FitDataRecord) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#sport = None;
        let mut r#sub_sport = None;
        let mut r#num_valid_steps = None;
        let mut r#first_step_index = None;
        let mut r#pool_length = None;
        let mut r#pool_length_unit = None;
        let mut r#message_index = None;
        let mut unknown_fields = Vec::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => {
                    r#sport = Some(field.into_value());
                }
                1u8 => {
                    r#sub_sport = Some(field.into_value());
                }
                2u8 => {
                    r#num_valid_steps = Some(field.into_value());
                }
                3u8 => {
                    r#first_step_index = Some(field.into_value());
                }
                4u8 => {
                    r#pool_length = Some(field.into_value());
                }
                5u8 => {
                    r#pool_length_unit = Some(field.into_value());
                }
                254u8 => {
                    r#message_index = Some(field.into_value());
                }
                _ => {
                    unknown_fields.push(field);
                }
            }
        }
        Ok(Self {
            r#sport,
            r#sub_sport,
            r#num_valid_steps,
            r#first_step_index,
            r#pool_length,
            r#pool_length_unit,
            r#message_index,
            unknown_fields,
        })
    }
}
impl TryFrom<FitDataRecord> for WorkoutSession {
    type Error = TryFromRecordError;
    fn try_from(record: FitDataRecord) -> Result<Self, Self::Error> {
        Self::parse(record)
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize)]
pub struct WorkoutStep {
    pub r#wkt_step_name: Option<Value>,
    pub r#duration_type: Option<Value>,
    pub r#duration_value: Option<Value>,
    pub r#target_type: Option<Value>,
    pub r#target_value: Option<Value>,
    pub r#custom_target_value_low: Option<Value>,
    pub r#custom_target_value_high: Option<Value>,
    pub r#intensity: Option<Value>,
    pub r#notes: Option<Value>,
    pub r#equipment: Option<Value>,
    pub r#exercise_category: Option<Value>,
    pub r#exercise_name: Option<Value>,
    pub r#exercise_weight: Option<Value>,
    pub r#weight_display_unit: Option<Value>,
    pub r#secondary_target_type: Option<Value>,
    pub r#secondary_target_value: Option<Value>,
    pub r#secondary_custom_target_value_low: Option<Value>,
    pub r#secondary_custom_target_value_high: Option<Value>,
    pub r#message_index: Option<Value>,
    #[doc = r" All fields that are not defined in the profile."]
    pub unknown_fields: Vec<FitDataField>,
}
impl FitMessage for WorkoutStep {
    const NAME: &'static str = "WorkoutStep";
    const KIND: MesgNum = MesgNum::WorkoutStep;
    fn parse(record: FitDataRecord) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#wkt_step_name = None;
        let mut r#duration_type = None;
        let mut r#duration_value = None;
        let mut r#target_type = None;
        let mut r#target_value = None;
        let mut r#custom_target_value_low = None;
        let mut r#custom_target_value_high = None;
        let mut r#intensity = None;
        let mut r#notes = None;
        let mut r#equipment = None;
        let mut r#exercise_category = None;
        let mut r#exercise_name = None;
        let mut r#exercise_weight = None;
        let mut r#weight_display_unit = None;
        let mut r#secondary_target_type = None;
        let mut r#secondary_target_value = None;
        let mut r#secondary_custom_target_value_low = None;
        let mut r#secondary_custom_target_value_high = None;
        let mut r#message_index = None;
        let mut unknown_fields = Vec::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => {
                    r#wkt_step_name = Some(field.into_value());
                }
                1u8 => {
                    r#duration_type = Some(field.into_value());
                }
                2u8 => {
                    r#duration_value = Some(field.into_value());
                }
                3u8 => {
                    r#target_type = Some(field.into_value());
                }
                4u8 => {
                    r#target_value = Some(field.into_value());
                }
                5u8 => {
                    r#custom_target_value_low = Some(field.into_value());
                }
                6u8 => {
                    r#custom_target_value_high = Some(field.into_value());
                }
                7u8 => {
                    r#intensity = Some(field.into_value());
                }
                8u8 => {
                    r#notes = Some(field.into_value());
                }
                9u8 => {
                    r#equipment = Some(field.into_value());
                }
                10u8 => {
                    r#exercise_category = Some(field.into_value());
                }
                11u8 => {
                    r#exercise_name = Some(field.into_value());
                }
                12u8 => {
                    r#exercise_weight = Some(field.into_value());
                }
                13u8 => {
                    r#weight_display_unit = Some(field.into_value());
                }
                19u8 => {
                    r#secondary_target_type = Some(field.into_value());
                }
                20u8 => {
                    r#secondary_target_value = Some(field.into_value());
                }
                21u8 => {
                    r#secondary_custom_target_value_low = Some(field.into_value());
                }
                22u8 => {
                    r#secondary_custom_target_value_high = Some(field.into_value());
                }
                254u8 => {
                    r#message_index = Some(field.into_value());
                }
                _ => {
                    unknown_fields.push(field);
                }
            }
        }
        Ok(Self {
            r#wkt_step_name,
            r#duration_type,
            r#duration_value,
            r#target_type,
            r#target_value,
            r#custom_target_value_low,
            r#custom_target_value_high,
            r#intensity,
            r#notes,
            r#equipment,
            r#exercise_category,
            r#exercise_name,
            r#exercise_weight,
            r#weight_display_unit,
            r#secondary_target_type,
            r#secondary_target_value,
            r#secondary_custom_target_value_low,
            r#secondary_custom_target_value_high,
            r#message_index,
            unknown_fields,
        })
    }
}
impl TryFrom<FitDataRecord> for WorkoutStep {
    type Error = TryFromRecordError;
    fn try_from(record: FitDataRecord) -> Result<Self, Self::Error> {
        Self::parse(record)
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize)]
pub struct ExerciseTitle {
    pub r#exercise_category: Option<Value>,
    pub r#exercise_name: Option<Value>,
    pub r#wkt_step_name: Option<Value>,
    pub r#message_index: Option<Value>,
    #[doc = r" All fields that are not defined in the profile."]
    pub unknown_fields: Vec<FitDataField>,
}
impl FitMessage for ExerciseTitle {
    const NAME: &'static str = "ExerciseTitle";
    const KIND: MesgNum = MesgNum::ExerciseTitle;
    fn parse(record: FitDataRecord) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#exercise_category = None;
        let mut r#exercise_name = None;
        let mut r#wkt_step_name = None;
        let mut r#message_index = None;
        let mut unknown_fields = Vec::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => {
                    r#exercise_category = Some(field.into_value());
                }
                1u8 => {
                    r#exercise_name = Some(field.into_value());
                }
                2u8 => {
                    r#wkt_step_name = Some(field.into_value());
                }
                254u8 => {
                    r#message_index = Some(field.into_value());
                }
                _ => {
                    unknown_fields.push(field);
                }
            }
        }
        Ok(Self {
            r#exercise_category,
            r#exercise_name,
            r#wkt_step_name,
            r#message_index,
            unknown_fields,
        })
    }
}
impl TryFrom<FitDataRecord> for ExerciseTitle {
    type Error = TryFromRecordError;
    fn try_from(record: FitDataRecord) -> Result<Self, Self::Error> {
        Self::parse(record)
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize)]
pub struct Schedule {
    pub r#manufacturer: Option<Value>,
    pub r#product: Option<Value>,
    pub r#serial_number: Option<Value>,
    pub r#time_created: Option<Value>,
    pub r#completed: Option<Value>,
    pub r#type: Option<Value>,
    pub r#scheduled_time: Option<Value>,
    #[doc = r" All fields that are not defined in the profile."]
    pub unknown_fields: Vec<FitDataField>,
}
impl FitMessage for Schedule {
    const NAME: &'static str = "Schedule";
    const KIND: MesgNum = MesgNum::Schedule;
    fn parse(record: FitDataRecord) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#manufacturer = None;
        let mut r#product = None;
        let mut r#serial_number = None;
        let mut r#time_created = None;
        let mut r#completed = None;
        let mut r#type = None;
        let mut r#scheduled_time = None;
        let mut unknown_fields = Vec::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => {
                    r#manufacturer = Some(field.into_value());
                }
                1u8 => {
                    r#product = Some(field.into_value());
                }
                2u8 => {
                    r#serial_number = Some(field.into_value());
                }
                3u8 => {
                    r#time_created = Some(field.into_value());
                }
                4u8 => {
                    r#completed = Some(field.into_value());
                }
                5u8 => {
                    r#type = Some(field.into_value());
                }
                6u8 => {
                    r#scheduled_time = Some(field.into_value());
                }
                _ => {
                    unknown_fields.push(field);
                }
            }
        }
        Ok(Self {
            r#manufacturer,
            r#product,
            r#serial_number,
            r#time_created,
            r#completed,
            r#type,
            r#scheduled_time,
            unknown_fields,
        })
    }
}
impl TryFrom<FitDataRecord> for Schedule {
    type Error = TryFromRecordError;
    fn try_from(record: FitDataRecord) -> Result<Self, Self::Error> {
        Self::parse(record)
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize)]
pub struct Totals {
    pub r#timer_time: Option<Value>,
    pub r#distance: Option<Value>,
    pub r#calories: Option<Value>,
    pub r#sport: Option<Value>,
    pub r#elapsed_time: Option<Value>,
    pub r#sessions: Option<Value>,
    pub r#active_time: Option<Value>,
    pub r#sport_index: Option<Value>,
    pub r#timestamp: Option<Value>,
    pub r#message_index: Option<Value>,
    #[doc = r" All fields that are not defined in the profile."]
    pub unknown_fields: Vec<FitDataField>,
}
impl FitMessage for Totals {
    const NAME: &'static str = "Totals";
    const KIND: MesgNum = MesgNum::Totals;
    fn parse(record: FitDataRecord) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#timer_time = None;
        let mut r#distance = None;
        let mut r#calories = None;
        let mut r#sport = None;
        let mut r#elapsed_time = None;
        let mut r#sessions = None;
        let mut r#active_time = None;
        let mut r#sport_index = None;
        let mut r#timestamp = None;
        let mut r#message_index = None;
        let mut unknown_fields = Vec::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => {
                    r#timer_time = Some(field.into_value());
                }
                1u8 => {
                    r#distance = Some(field.into_value());
                }
                2u8 => {
                    r#calories = Some(field.into_value());
                }
                3u8 => {
                    r#sport = Some(field.into_value());
                }
                4u8 => {
                    r#elapsed_time = Some(field.into_value());
                }
                5u8 => {
                    r#sessions = Some(field.into_value());
                }
                6u8 => {
                    r#active_time = Some(field.into_value());
                }
                9u8 => {
                    r#sport_index = Some(field.into_value());
                }
                253u8 => {
                    r#timestamp = Some(field.into_value());
                }
                254u8 => {
                    r#message_index = Some(field.into_value());
                }
                _ => {
                    unknown_fields.push(field);
                }
            }
        }
        Ok(Self {
            r#timer_time,
            r#distance,
            r#calories,
            r#sport,
            r#elapsed_time,
            r#sessions,
            r#active_time,
            r#sport_index,
            r#timestamp,
            r#message_index,
            unknown_fields,
        })
    }
}
impl TryFrom<FitDataRecord> for Totals {
    type Error = TryFromRecordError;
    fn try_from(record: FitDataRecord) -> Result<Self, Self::Error> {
        Self::parse(record)
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize)]
pub struct WeightScale {
    pub r#weight: Option<Value>,
    pub r#percent_fat: Option<Value>,
    pub r#percent_hydration: Option<Value>,
    pub r#visceral_fat_mass: Option<Value>,
    pub r#bone_mass: Option<Value>,
    pub r#muscle_mass: Option<Value>,
    pub r#basal_met: Option<Value>,
    pub r#physique_rating: Option<Value>,
    pub r#active_met: Option<Value>,
    pub r#metabolic_age: Option<Value>,
    pub r#visceral_fat_rating: Option<Value>,
    pub r#user_profile_index: Option<Value>,
    pub r#bmi: Option<Value>,
    pub r#timestamp: Option<Value>,
    #[doc = r" All fields that are not defined in the profile."]
    pub unknown_fields: Vec<FitDataField>,
}
impl FitMessage for WeightScale {
    const NAME: &'static str = "WeightScale";
    const KIND: MesgNum = MesgNum::WeightScale;
    fn parse(record: FitDataRecord) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#weight = None;
        let mut r#percent_fat = None;
        let mut r#percent_hydration = None;
        let mut r#visceral_fat_mass = None;
        let mut r#bone_mass = None;
        let mut r#muscle_mass = None;
        let mut r#basal_met = None;
        let mut r#physique_rating = None;
        let mut r#active_met = None;
        let mut r#metabolic_age = None;
        let mut r#visceral_fat_rating = None;
        let mut r#user_profile_index = None;
        let mut r#bmi = None;
        let mut r#timestamp = None;
        let mut unknown_fields = Vec::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => {
                    r#weight = Some(field.into_value());
                }
                1u8 => {
                    r#percent_fat = Some(field.into_value());
                }
                2u8 => {
                    r#percent_hydration = Some(field.into_value());
                }
                3u8 => {
                    r#visceral_fat_mass = Some(field.into_value());
                }
                4u8 => {
                    r#bone_mass = Some(field.into_value());
                }
                5u8 => {
                    r#muscle_mass = Some(field.into_value());
                }
                7u8 => {
                    r#basal_met = Some(field.into_value());
                }
                8u8 => {
                    r#physique_rating = Some(field.into_value());
                }
                9u8 => {
                    r#active_met = Some(field.into_value());
                }
                10u8 => {
                    r#metabolic_age = Some(field.into_value());
                }
                11u8 => {
                    r#visceral_fat_rating = Some(field.into_value());
                }
                12u8 => {
                    r#user_profile_index = Some(field.into_value());
                }
                13u8 => {
                    r#bmi = Some(field.into_value());
                }
                253u8 => {
                    r#timestamp = Some(field.into_value());
                }
                _ => {
                    unknown_fields.push(field);
                }
            }
        }
        Ok(Self {
            r#weight,
            r#percent_fat,
            r#percent_hydration,
            r#visceral_fat_mass,
            r#bone_mass,
            r#muscle_mass,
            r#basal_met,
            r#physique_rating,
            r#active_met,
            r#metabolic_age,
            r#visceral_fat_rating,
            r#user_profile_index,
            r#bmi,
            r#timestamp,
            unknown_fields,
        })
    }
}
impl TryFrom<FitDataRecord> for WeightScale {
    type Error = TryFromRecordError;
    fn try_from(record: FitDataRecord) -> Result<Self, Self::Error> {
        Self::parse(record)
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize)]
pub struct BloodPressure {
    pub r#systolic_pressure: Option<Value>,
    pub r#diastolic_pressure: Option<Value>,
    pub r#mean_arterial_pressure: Option<Value>,
    pub r#map_3_sample_mean: Option<Value>,
    pub r#map_morning_values: Option<Value>,
    pub r#map_evening_values: Option<Value>,
    pub r#heart_rate: Option<Value>,
    pub r#heart_rate_type: Option<Value>,
    pub r#status: Option<Value>,
    pub r#user_profile_index: Option<Value>,
    pub r#timestamp: Option<Value>,
    #[doc = r" All fields that are not defined in the profile."]
    pub unknown_fields: Vec<FitDataField>,
}
impl FitMessage for BloodPressure {
    const NAME: &'static str = "BloodPressure";
    const KIND: MesgNum = MesgNum::BloodPressure;
    fn parse(record: FitDataRecord) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#systolic_pressure = None;
        let mut r#diastolic_pressure = None;
        let mut r#mean_arterial_pressure = None;
        let mut r#map_3_sample_mean = None;
        let mut r#map_morning_values = None;
        let mut r#map_evening_values = None;
        let mut r#heart_rate = None;
        let mut r#heart_rate_type = None;
        let mut r#status = None;
        let mut r#user_profile_index = None;
        let mut r#timestamp = None;
        let mut unknown_fields = Vec::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => {
                    r#systolic_pressure = Some(field.into_value());
                }
                1u8 => {
                    r#diastolic_pressure = Some(field.into_value());
                }
                2u8 => {
                    r#mean_arterial_pressure = Some(field.into_value());
                }
                3u8 => {
                    r#map_3_sample_mean = Some(field.into_value());
                }
                4u8 => {
                    r#map_morning_values = Some(field.into_value());
                }
                5u8 => {
                    r#map_evening_values = Some(field.into_value());
                }
                6u8 => {
                    r#heart_rate = Some(field.into_value());
                }
                7u8 => {
                    r#heart_rate_type = Some(field.into_value());
                }
                8u8 => {
                    r#status = Some(field.into_value());
                }
                9u8 => {
                    r#user_profile_index = Some(field.into_value());
                }
                253u8 => {
                    r#timestamp = Some(field.into_value());
                }
                _ => {
                    unknown_fields.push(field);
                }
            }
        }
        Ok(Self {
            r#systolic_pressure,
            r#diastolic_pressure,
            r#mean_arterial_pressure,
            r#map_3_sample_mean,
            r#map_morning_values,
            r#map_evening_values,
            r#heart_rate,
            r#heart_rate_type,
            r#status,
            r#user_profile_index,
            r#timestamp,
            unknown_fields,
        })
    }
}
impl TryFrom<FitDataRecord> for BloodPressure {
    type Error = TryFromRecordError;
    fn try_from(record: FitDataRecord) -> Result<Self, Self::Error> {
        Self::parse(record)
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize)]
pub struct MonitoringInfo {
    pub r#local_timestamp: Option<Value>,
    pub r#activity_type: Option<Value>,
    pub r#cycles_to_distance: Option<Value>,
    pub r#cycles_to_calories: Option<Value>,
    pub r#resting_metabolic_rate: Option<Value>,
    pub r#timestamp: Option<Value>,
    #[doc = r" All fields that are not defined in the profile."]
    pub unknown_fields: Vec<FitDataField>,
}
impl FitMessage for MonitoringInfo {
    const NAME: &'static str = "MonitoringInfo";
    const KIND: MesgNum = MesgNum::MonitoringInfo;
    fn parse(record: FitDataRecord) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#local_timestamp = None;
        let mut r#activity_type = None;
        let mut r#cycles_to_distance = None;
        let mut r#cycles_to_calories = None;
        let mut r#resting_metabolic_rate = None;
        let mut r#timestamp = None;
        let mut unknown_fields = Vec::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => {
                    r#local_timestamp = Some(field.into_value());
                }
                1u8 => {
                    r#activity_type = Some(field.into_value());
                }
                3u8 => {
                    r#cycles_to_distance = Some(field.into_value());
                }
                4u8 => {
                    r#cycles_to_calories = Some(field.into_value());
                }
                5u8 => {
                    r#resting_metabolic_rate = Some(field.into_value());
                }
                253u8 => {
                    r#timestamp = Some(field.into_value());
                }
                _ => {
                    unknown_fields.push(field);
                }
            }
        }
        Ok(Self {
            r#local_timestamp,
            r#activity_type,
            r#cycles_to_distance,
            r#cycles_to_calories,
            r#resting_metabolic_rate,
            r#timestamp,
            unknown_fields,
        })
    }
}
impl TryFrom<FitDataRecord> for MonitoringInfo {
    type Error = TryFromRecordError;
    fn try_from(record: FitDataRecord) -> Result<Self, Self::Error> {
        Self::parse(record)
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize)]
pub struct Monitoring {
    pub r#device_index: Option<Value>,
    pub r#calories: Option<Value>,
    pub r#distance: Option<Value>,
    pub r#cycles: Option<Value>,
    pub r#active_time: Option<Value>,
    pub r#activity_type: Option<Value>,
    pub r#activity_subtype: Option<Value>,
    pub r#activity_level: Option<Value>,
    pub r#distance_16: Option<Value>,
    pub r#cycles_16: Option<Value>,
    pub r#active_time_16: Option<Value>,
    pub r#local_timestamp: Option<Value>,
    pub r#temperature: Option<Value>,
    pub r#temperature_min: Option<Value>,
    pub r#temperature_max: Option<Value>,
    pub r#activity_time: Option<Value>,
    pub r#active_calories: Option<Value>,
    pub r#current_activity_type_intensity: Option<Value>,
    pub r#timestamp_min_8: Option<Value>,
    pub r#timestamp_16: Option<Value>,
    pub r#heart_rate: Option<Value>,
    pub r#intensity: Option<Value>,
    pub r#duration_min: Option<Value>,
    pub r#duration: Option<Value>,
    pub r#ascent: Option<Value>,
    pub r#descent: Option<Value>,
    pub r#moderate_activity_minutes: Option<Value>,
    pub r#vigorous_activity_minutes: Option<Value>,
    pub r#timestamp: Option<Value>,
    #[doc = r" All fields that are not defined in the profile."]
    pub unknown_fields: Vec<FitDataField>,
}
impl FitMessage for Monitoring {
    const NAME: &'static str = "Monitoring";
    const KIND: MesgNum = MesgNum::Monitoring;
    fn parse(record: FitDataRecord) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#device_index = None;
        let mut r#calories = None;
        let mut r#distance = None;
        let mut r#cycles = None;
        let mut r#active_time = None;
        let mut r#activity_type = None;
        let mut r#activity_subtype = None;
        let mut r#activity_level = None;
        let mut r#distance_16 = None;
        let mut r#cycles_16 = None;
        let mut r#active_time_16 = None;
        let mut r#local_timestamp = None;
        let mut r#temperature = None;
        let mut r#temperature_min = None;
        let mut r#temperature_max = None;
        let mut r#activity_time = None;
        let mut r#active_calories = None;
        let mut r#current_activity_type_intensity = None;
        let mut r#timestamp_min_8 = None;
        let mut r#timestamp_16 = None;
        let mut r#heart_rate = None;
        let mut r#intensity = None;
        let mut r#duration_min = None;
        let mut r#duration = None;
        let mut r#ascent = None;
        let mut r#descent = None;
        let mut r#moderate_activity_minutes = None;
        let mut r#vigorous_activity_minutes = None;
        let mut r#timestamp = None;
        let mut unknown_fields = Vec::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => {
                    r#device_index = Some(field.into_value());
                }
                1u8 => {
                    r#calories = Some(field.into_value());
                }
                2u8 => {
                    r#distance = Some(field.into_value());
                }
                3u8 => {
                    r#cycles = Some(field.into_value());
                }
                4u8 => {
                    r#active_time = Some(field.into_value());
                }
                5u8 => {
                    r#activity_type = Some(field.into_value());
                }
                6u8 => {
                    r#activity_subtype = Some(field.into_value());
                }
                7u8 => {
                    r#activity_level = Some(field.into_value());
                }
                8u8 => {
                    r#distance_16 = Some(field.into_value());
                }
                9u8 => {
                    r#cycles_16 = Some(field.into_value());
                }
                10u8 => {
                    r#active_time_16 = Some(field.into_value());
                }
                11u8 => {
                    r#local_timestamp = Some(field.into_value());
                }
                12u8 => {
                    r#temperature = Some(field.into_value());
                }
                14u8 => {
                    r#temperature_min = Some(field.into_value());
                }
                15u8 => {
                    r#temperature_max = Some(field.into_value());
                }
                16u8 => {
                    r#activity_time = Some(field.into_value());
                }
                19u8 => {
                    r#active_calories = Some(field.into_value());
                }
                24u8 => {
                    r#current_activity_type_intensity = Some(field.into_value());
                }
                25u8 => {
                    r#timestamp_min_8 = Some(field.into_value());
                }
                26u8 => {
                    r#timestamp_16 = Some(field.into_value());
                }
                27u8 => {
                    r#heart_rate = Some(field.into_value());
                }
                28u8 => {
                    r#intensity = Some(field.into_value());
                }
                29u8 => {
                    r#duration_min = Some(field.into_value());
                }
                30u8 => {
                    r#duration = Some(field.into_value());
                }
                31u8 => {
                    r#ascent = Some(field.into_value());
                }
                32u8 => {
                    r#descent = Some(field.into_value());
                }
                33u8 => {
                    r#moderate_activity_minutes = Some(field.into_value());
                }
                34u8 => {
                    r#vigorous_activity_minutes = Some(field.into_value());
                }
                253u8 => {
                    r#timestamp = Some(field.into_value());
                }
                _ => {
                    unknown_fields.push(field);
                }
            }
        }
        Ok(Self {
            r#device_index,
            r#calories,
            r#distance,
            r#cycles,
            r#active_time,
            r#activity_type,
            r#activity_subtype,
            r#activity_level,
            r#distance_16,
            r#cycles_16,
            r#active_time_16,
            r#local_timestamp,
            r#temperature,
            r#temperature_min,
            r#temperature_max,
            r#activity_time,
            r#active_calories,
            r#current_activity_type_intensity,
            r#timestamp_min_8,
            r#timestamp_16,
            r#heart_rate,
            r#intensity,
            r#duration_min,
            r#duration,
            r#ascent,
            r#descent,
            r#moderate_activity_minutes,
            r#vigorous_activity_minutes,
            r#timestamp,
            unknown_fields,
        })
    }
}
impl TryFrom<FitDataRecord> for Monitoring {
    type Error = TryFromRecordError;
    fn try_from(record: FitDataRecord) -> Result<Self, Self::Error> {
        Self::parse(record)
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize)]
pub struct MonitoringHrData {
    pub r#resting_heart_rate: Option<Value>,
    pub r#current_day_resting_heart_rate: Option<Value>,
    pub r#timestamp: Option<Value>,
    #[doc = r" All fields that are not defined in the profile."]
    pub unknown_fields: Vec<FitDataField>,
}
impl FitMessage for MonitoringHrData {
    const NAME: &'static str = "MonitoringHrData";
    const KIND: MesgNum = MesgNum::MonitoringHrData;
    fn parse(record: FitDataRecord) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#resting_heart_rate = None;
        let mut r#current_day_resting_heart_rate = None;
        let mut r#timestamp = None;
        let mut unknown_fields = Vec::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => {
                    r#resting_heart_rate = Some(field.into_value());
                }
                1u8 => {
                    r#current_day_resting_heart_rate = Some(field.into_value());
                }
                253u8 => {
                    r#timestamp = Some(field.into_value());
                }
                _ => {
                    unknown_fields.push(field);
                }
            }
        }
        Ok(Self {
            r#resting_heart_rate,
            r#current_day_resting_heart_rate,
            r#timestamp,
            unknown_fields,
        })
    }
}
impl TryFrom<FitDataRecord> for MonitoringHrData {
    type Error = TryFromRecordError;
    fn try_from(record: FitDataRecord) -> Result<Self, Self::Error> {
        Self::parse(record)
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize)]
pub struct Spo2Data {
    pub r#reading_spo2: Option<Value>,
    pub r#reading_confidence: Option<Value>,
    pub r#mode: Option<Value>,
    pub r#timestamp: Option<Value>,
    #[doc = r" All fields that are not defined in the profile."]
    pub unknown_fields: Vec<FitDataField>,
}
impl FitMessage for Spo2Data {
    const NAME: &'static str = "Spo2Data";
    const KIND: MesgNum = MesgNum::Spo2Data;
    fn parse(record: FitDataRecord) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#reading_spo2 = None;
        let mut r#reading_confidence = None;
        let mut r#mode = None;
        let mut r#timestamp = None;
        let mut unknown_fields = Vec::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => {
                    r#reading_spo2 = Some(field.into_value());
                }
                1u8 => {
                    r#reading_confidence = Some(field.into_value());
                }
                2u8 => {
                    r#mode = Some(field.into_value());
                }
                253u8 => {
                    r#timestamp = Some(field.into_value());
                }
                _ => {
                    unknown_fields.push(field);
                }
            }
        }
        Ok(Self {
            r#reading_spo2,
            r#reading_confidence,
            r#mode,
            r#timestamp,
            unknown_fields,
        })
    }
}
impl TryFrom<FitDataRecord> for Spo2Data {
    type Error = TryFromRecordError;
    fn try_from(record: FitDataRecord) -> Result<Self, Self::Error> {
        Self::parse(record)
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize)]
pub struct Hr {
    pub r#fractional_timestamp: Option<Value>,
    pub r#time256: Option<Value>,
    pub r#filtered_bpm: Option<Value>,
    pub r#event_timestamp: Option<Value>,
    pub r#event_timestamp_12: Option<Value>,
    pub r#timestamp: Option<Value>,
    #[doc = r" All fields that are not defined in the profile."]
    pub unknown_fields: Vec<FitDataField>,
}
impl FitMessage for Hr {
    const NAME: &'static str = "Hr";
    const KIND: MesgNum = MesgNum::Hr;
    fn parse(record: FitDataRecord) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#fractional_timestamp = None;
        let mut r#time256 = None;
        let mut r#filtered_bpm = None;
        let mut r#event_timestamp = None;
        let mut r#event_timestamp_12 = None;
        let mut r#timestamp = None;
        let mut unknown_fields = Vec::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => {
                    r#fractional_timestamp = Some(field.into_value());
                }
                1u8 => {
                    r#time256 = Some(field.into_value());
                }
                6u8 => {
                    r#filtered_bpm = Some(field.into_value());
                }
                9u8 => {
                    r#event_timestamp = Some(field.into_value());
                }
                10u8 => {
                    r#event_timestamp_12 = Some(field.into_value());
                }
                253u8 => {
                    r#timestamp = Some(field.into_value());
                }
                _ => {
                    unknown_fields.push(field);
                }
            }
        }
        Ok(Self {
            r#fractional_timestamp,
            r#time256,
            r#filtered_bpm,
            r#event_timestamp,
            r#event_timestamp_12,
            r#timestamp,
            unknown_fields,
        })
    }
}
impl TryFrom<FitDataRecord> for Hr {
    type Error = TryFromRecordError;
    fn try_from(record: FitDataRecord) -> Result<Self, Self::Error> {
        Self::parse(record)
    }
}
#[doc = "Value from 1 to 100 calculated by FirstBeat"]
#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize)]
pub struct StressLevel {
    pub r#stress_level_value: Option<Value>,
    pub r#stress_level_time: Option<Value>,
    #[doc = r" All fields that are not defined in the profile."]
    pub unknown_fields: Vec<FitDataField>,
}
impl FitMessage for StressLevel {
    const NAME: &'static str = "StressLevel";
    const KIND: MesgNum = MesgNum::StressLevel;
    fn parse(record: FitDataRecord) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#stress_level_value = None;
        let mut r#stress_level_time = None;
        let mut unknown_fields = Vec::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => {
                    r#stress_level_value = Some(field.into_value());
                }
                1u8 => {
                    r#stress_level_time = Some(field.into_value());
                }
                _ => {
                    unknown_fields.push(field);
                }
            }
        }
        Ok(Self {
            r#stress_level_value,
            r#stress_level_time,
            unknown_fields,
        })
    }
}
impl TryFrom<FitDataRecord> for StressLevel {
    type Error = TryFromRecordError;
    fn try_from(record: FitDataRecord) -> Result<Self, Self::Error> {
        Self::parse(record)
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize)]
pub struct MaxMetData {
    pub r#update_time: Option<Value>,
    pub r#vo2_max: Option<Value>,
    pub r#sport: Option<Value>,
    pub r#sub_sport: Option<Value>,
    pub r#max_met_category: Option<Value>,
    pub r#calibrated_data: Option<Value>,
    pub r#hr_source: Option<Value>,
    pub r#speed_source: Option<Value>,
    #[doc = r" All fields that are not defined in the profile."]
    pub unknown_fields: Vec<FitDataField>,
}
impl FitMessage for MaxMetData {
    const NAME: &'static str = "MaxMetData";
    const KIND: MesgNum = MesgNum::MaxMetData;
    fn parse(record: FitDataRecord) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#update_time = None;
        let mut r#vo2_max = None;
        let mut r#sport = None;
        let mut r#sub_sport = None;
        let mut r#max_met_category = None;
        let mut r#calibrated_data = None;
        let mut r#hr_source = None;
        let mut r#speed_source = None;
        let mut unknown_fields = Vec::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => {
                    r#update_time = Some(field.into_value());
                }
                2u8 => {
                    r#vo2_max = Some(field.into_value());
                }
                5u8 => {
                    r#sport = Some(field.into_value());
                }
                6u8 => {
                    r#sub_sport = Some(field.into_value());
                }
                8u8 => {
                    r#max_met_category = Some(field.into_value());
                }
                9u8 => {
                    r#calibrated_data = Some(field.into_value());
                }
                12u8 => {
                    r#hr_source = Some(field.into_value());
                }
                13u8 => {
                    r#speed_source = Some(field.into_value());
                }
                _ => {
                    unknown_fields.push(field);
                }
            }
        }
        Ok(Self {
            r#update_time,
            r#vo2_max,
            r#sport,
            r#sub_sport,
            r#max_met_category,
            r#calibrated_data,
            r#hr_source,
            r#speed_source,
            unknown_fields,
        })
    }
}
impl TryFrom<FitDataRecord> for MaxMetData {
    type Error = TryFromRecordError;
    fn try_from(record: FitDataRecord) -> Result<Self, Self::Error> {
        Self::parse(record)
    }
}
#[doc = "Body battery data used for HSA custom data logging"]
#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize)]
pub struct HsaBodyBatteryData {
    pub r#processing_interval: Option<Value>,
    pub r#level: Option<Value>,
    pub r#charged: Option<Value>,
    pub r#uncharged: Option<Value>,
    pub r#timestamp: Option<Value>,
    #[doc = r" All fields that are not defined in the profile."]
    pub unknown_fields: Vec<FitDataField>,
}
impl FitMessage for HsaBodyBatteryData {
    const NAME: &'static str = "HsaBodyBatteryData";
    const KIND: MesgNum = MesgNum::HsaBodyBatteryData;
    fn parse(record: FitDataRecord) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#processing_interval = None;
        let mut r#level = None;
        let mut r#charged = None;
        let mut r#uncharged = None;
        let mut r#timestamp = None;
        let mut unknown_fields = Vec::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => {
                    r#processing_interval = Some(field.into_value());
                }
                1u8 => {
                    r#level = Some(field.into_value());
                }
                2u8 => {
                    r#charged = Some(field.into_value());
                }
                3u8 => {
                    r#uncharged = Some(field.into_value());
                }
                253u8 => {
                    r#timestamp = Some(field.into_value());
                }
                _ => {
                    unknown_fields.push(field);
                }
            }
        }
        Ok(Self {
            r#processing_interval,
            r#level,
            r#charged,
            r#uncharged,
            r#timestamp,
            unknown_fields,
        })
    }
}
impl TryFrom<FitDataRecord> for HsaBodyBatteryData {
    type Error = TryFromRecordError;
    fn try_from(record: FitDataRecord) -> Result<Self, Self::Error> {
        Self::parse(record)
    }
}
#[doc = "HSA events"]
#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize)]
pub struct HsaEvent {
    pub r#event_id: Option<Value>,
    pub r#timestamp: Option<Value>,
    #[doc = r" All fields that are not defined in the profile."]
    pub unknown_fields: Vec<FitDataField>,
}
impl FitMessage for HsaEvent {
    const NAME: &'static str = "HsaEvent";
    const KIND: MesgNum = MesgNum::HsaEvent;
    fn parse(record: FitDataRecord) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#event_id = None;
        let mut r#timestamp = None;
        let mut unknown_fields = Vec::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => {
                    r#event_id = Some(field.into_value());
                }
                253u8 => {
                    r#timestamp = Some(field.into_value());
                }
                _ => {
                    unknown_fields.push(field);
                }
            }
        }
        Ok(Self {
            r#event_id,
            r#timestamp,
            unknown_fields,
        })
    }
}
impl TryFrom<FitDataRecord> for HsaEvent {
    type Error = TryFromRecordError;
    fn try_from(record: FitDataRecord) -> Result<Self, Self::Error> {
        Self::parse(record)
    }
}
#[doc = "Raw accelerometer data used for HSA custom data logging"]
#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize)]
pub struct HsaAccelerometerData {
    pub r#timestamp_ms: Option<Value>,
    pub r#sampling_interval: Option<Value>,
    pub r#accel_x: Option<Value>,
    pub r#accel_y: Option<Value>,
    pub r#accel_z: Option<Value>,
    pub r#timestamp_32k: Option<Value>,
    pub r#timestamp: Option<Value>,
    #[doc = r" All fields that are not defined in the profile."]
    pub unknown_fields: Vec<FitDataField>,
}
impl FitMessage for HsaAccelerometerData {
    const NAME: &'static str = "HsaAccelerometerData";
    const KIND: MesgNum = MesgNum::HsaAccelerometerData;
    fn parse(record: FitDataRecord) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#timestamp_ms = None;
        let mut r#sampling_interval = None;
        let mut r#accel_x = None;
        let mut r#accel_y = None;
        let mut r#accel_z = None;
        let mut r#timestamp_32k = None;
        let mut r#timestamp = None;
        let mut unknown_fields = Vec::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => {
                    r#timestamp_ms = Some(field.into_value());
                }
                1u8 => {
                    r#sampling_interval = Some(field.into_value());
                }
                2u8 => {
                    r#accel_x = Some(field.into_value());
                }
                3u8 => {
                    r#accel_y = Some(field.into_value());
                }
                4u8 => {
                    r#accel_z = Some(field.into_value());
                }
                5u8 => {
                    r#timestamp_32k = Some(field.into_value());
                }
                253u8 => {
                    r#timestamp = Some(field.into_value());
                }
                _ => {
                    unknown_fields.push(field);
                }
            }
        }
        Ok(Self {
            r#timestamp_ms,
            r#sampling_interval,
            r#accel_x,
            r#accel_y,
            r#accel_z,
            r#timestamp_32k,
            r#timestamp,
            unknown_fields,
        })
    }
}
impl TryFrom<FitDataRecord> for HsaAccelerometerData {
    type Error = TryFromRecordError;
    fn try_from(record: FitDataRecord) -> Result<Self, Self::Error> {
        Self::parse(record)
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize)]
pub struct HsaGyroscopeData {
    pub r#timestamp_ms: Option<Value>,
    pub r#sampling_interval: Option<Value>,
    pub r#gyro_x: Option<Value>,
    pub r#gyro_y: Option<Value>,
    pub r#gyro_z: Option<Value>,
    pub r#timestamp_32k: Option<Value>,
    pub r#timestamp: Option<Value>,
    #[doc = r" All fields that are not defined in the profile."]
    pub unknown_fields: Vec<FitDataField>,
}
impl FitMessage for HsaGyroscopeData {
    const NAME: &'static str = "HsaGyroscopeData";
    const KIND: MesgNum = MesgNum::HsaGyroscopeData;
    fn parse(record: FitDataRecord) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#timestamp_ms = None;
        let mut r#sampling_interval = None;
        let mut r#gyro_x = None;
        let mut r#gyro_y = None;
        let mut r#gyro_z = None;
        let mut r#timestamp_32k = None;
        let mut r#timestamp = None;
        let mut unknown_fields = Vec::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => {
                    r#timestamp_ms = Some(field.into_value());
                }
                1u8 => {
                    r#sampling_interval = Some(field.into_value());
                }
                2u8 => {
                    r#gyro_x = Some(field.into_value());
                }
                3u8 => {
                    r#gyro_y = Some(field.into_value());
                }
                4u8 => {
                    r#gyro_z = Some(field.into_value());
                }
                5u8 => {
                    r#timestamp_32k = Some(field.into_value());
                }
                253u8 => {
                    r#timestamp = Some(field.into_value());
                }
                _ => {
                    unknown_fields.push(field);
                }
            }
        }
        Ok(Self {
            r#timestamp_ms,
            r#sampling_interval,
            r#gyro_x,
            r#gyro_y,
            r#gyro_z,
            r#timestamp_32k,
            r#timestamp,
            unknown_fields,
        })
    }
}
impl TryFrom<FitDataRecord> for HsaGyroscopeData {
    type Error = TryFromRecordError;
    fn try_from(record: FitDataRecord) -> Result<Self, Self::Error> {
        Self::parse(record)
    }
}
#[doc = "User's current daily step data used for HSA custom data logging"]
#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize)]
pub struct HsaStepData {
    pub r#processing_interval: Option<Value>,
    pub r#steps: Option<Value>,
    pub r#timestamp: Option<Value>,
    #[doc = r" All fields that are not defined in the profile."]
    pub unknown_fields: Vec<FitDataField>,
}
impl FitMessage for HsaStepData {
    const NAME: &'static str = "HsaStepData";
    const KIND: MesgNum = MesgNum::HsaStepData;
    fn parse(record: FitDataRecord) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#processing_interval = None;
        let mut r#steps = None;
        let mut r#timestamp = None;
        let mut unknown_fields = Vec::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => {
                    r#processing_interval = Some(field.into_value());
                }
                1u8 => {
                    r#steps = Some(field.into_value());
                }
                253u8 => {
                    r#timestamp = Some(field.into_value());
                }
                _ => {
                    unknown_fields.push(field);
                }
            }
        }
        Ok(Self {
            r#processing_interval,
            r#steps,
            r#timestamp,
            unknown_fields,
        })
    }
}
impl TryFrom<FitDataRecord> for HsaStepData {
    type Error = TryFromRecordError;
    fn try_from(record: FitDataRecord) -> Result<Self, Self::Error> {
        Self::parse(record)
    }
}
#[doc = "User's current SpO2 data used for HSA custom data logging"]
#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize)]
pub struct HsaSpo2Data {
    pub r#processing_interval: Option<Value>,
    pub r#reading_spo2: Option<Value>,
    pub r#confidence: Option<Value>,
    pub r#timestamp: Option<Value>,
    #[doc = r" All fields that are not defined in the profile."]
    pub unknown_fields: Vec<FitDataField>,
}
impl FitMessage for HsaSpo2Data {
    const NAME: &'static str = "HsaSpo2Data";
    const KIND: MesgNum = MesgNum::HsaSpo2Data;
    fn parse(record: FitDataRecord) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#processing_interval = None;
        let mut r#reading_spo2 = None;
        let mut r#confidence = None;
        let mut r#timestamp = None;
        let mut unknown_fields = Vec::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => {
                    r#processing_interval = Some(field.into_value());
                }
                1u8 => {
                    r#reading_spo2 = Some(field.into_value());
                }
                2u8 => {
                    r#confidence = Some(field.into_value());
                }
                253u8 => {
                    r#timestamp = Some(field.into_value());
                }
                _ => {
                    unknown_fields.push(field);
                }
            }
        }
        Ok(Self {
            r#processing_interval,
            r#reading_spo2,
            r#confidence,
            r#timestamp,
            unknown_fields,
        })
    }
}
impl TryFrom<FitDataRecord> for HsaSpo2Data {
    type Error = TryFromRecordError;
    fn try_from(record: FitDataRecord) -> Result<Self, Self::Error> {
        Self::parse(record)
    }
}
#[doc = "User's current stress data used for HSA custom data logging"]
#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize)]
pub struct HsaStressData {
    pub r#processing_interval: Option<Value>,
    pub r#stress_level: Option<Value>,
    pub r#timestamp: Option<Value>,
    #[doc = r" All fields that are not defined in the profile."]
    pub unknown_fields: Vec<FitDataField>,
}
impl FitMessage for HsaStressData {
    const NAME: &'static str = "HsaStressData";
    const KIND: MesgNum = MesgNum::HsaStressData;
    fn parse(record: FitDataRecord) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#processing_interval = None;
        let mut r#stress_level = None;
        let mut r#timestamp = None;
        let mut unknown_fields = Vec::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => {
                    r#processing_interval = Some(field.into_value());
                }
                1u8 => {
                    r#stress_level = Some(field.into_value());
                }
                253u8 => {
                    r#timestamp = Some(field.into_value());
                }
                _ => {
                    unknown_fields.push(field);
                }
            }
        }
        Ok(Self {
            r#processing_interval,
            r#stress_level,
            r#timestamp,
            unknown_fields,
        })
    }
}
impl TryFrom<FitDataRecord> for HsaStressData {
    type Error = TryFromRecordError;
    fn try_from(record: FitDataRecord) -> Result<Self, Self::Error> {
        Self::parse(record)
    }
}
#[doc = "User's current respiration data used for HSA custom data logging"]
#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize)]
pub struct HsaRespirationData {
    pub r#processing_interval: Option<Value>,
    pub r#respiration_rate: Option<Value>,
    pub r#timestamp: Option<Value>,
    #[doc = r" All fields that are not defined in the profile."]
    pub unknown_fields: Vec<FitDataField>,
}
impl FitMessage for HsaRespirationData {
    const NAME: &'static str = "HsaRespirationData";
    const KIND: MesgNum = MesgNum::HsaRespirationData;
    fn parse(record: FitDataRecord) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#processing_interval = None;
        let mut r#respiration_rate = None;
        let mut r#timestamp = None;
        let mut unknown_fields = Vec::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => {
                    r#processing_interval = Some(field.into_value());
                }
                1u8 => {
                    r#respiration_rate = Some(field.into_value());
                }
                253u8 => {
                    r#timestamp = Some(field.into_value());
                }
                _ => {
                    unknown_fields.push(field);
                }
            }
        }
        Ok(Self {
            r#processing_interval,
            r#respiration_rate,
            r#timestamp,
            unknown_fields,
        })
    }
}
impl TryFrom<FitDataRecord> for HsaRespirationData {
    type Error = TryFromRecordError;
    fn try_from(record: FitDataRecord) -> Result<Self, Self::Error> {
        Self::parse(record)
    }
}
#[doc = "User's current heart rate data used for HSA custom data logging"]
#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize)]
pub struct HsaHeartRateData {
    pub r#processing_interval: Option<Value>,
    pub r#status: Option<Value>,
    pub r#heart_rate: Option<Value>,
    pub r#timestamp: Option<Value>,
    #[doc = r" All fields that are not defined in the profile."]
    pub unknown_fields: Vec<FitDataField>,
}
impl FitMessage for HsaHeartRateData {
    const NAME: &'static str = "HsaHeartRateData";
    const KIND: MesgNum = MesgNum::HsaHeartRateData;
    fn parse(record: FitDataRecord) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#processing_interval = None;
        let mut r#status = None;
        let mut r#heart_rate = None;
        let mut r#timestamp = None;
        let mut unknown_fields = Vec::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => {
                    r#processing_interval = Some(field.into_value());
                }
                1u8 => {
                    r#status = Some(field.into_value());
                }
                2u8 => {
                    r#heart_rate = Some(field.into_value());
                }
                253u8 => {
                    r#timestamp = Some(field.into_value());
                }
                _ => {
                    unknown_fields.push(field);
                }
            }
        }
        Ok(Self {
            r#processing_interval,
            r#status,
            r#heart_rate,
            r#timestamp,
            unknown_fields,
        })
    }
}
impl TryFrom<FitDataRecord> for HsaHeartRateData {
    type Error = TryFromRecordError;
    fn try_from(record: FitDataRecord) -> Result<Self, Self::Error> {
        Self::parse(record)
    }
}
#[doc = "Configuration data for HSA custom data logging"]
#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize)]
pub struct HsaConfigurationData {
    pub r#data: Option<Value>,
    pub r#data_size: Option<Value>,
    pub r#timestamp: Option<Value>,
    #[doc = r" All fields that are not defined in the profile."]
    pub unknown_fields: Vec<FitDataField>,
}
impl FitMessage for HsaConfigurationData {
    const NAME: &'static str = "HsaConfigurationData";
    const KIND: MesgNum = MesgNum::HsaConfigurationData;
    fn parse(record: FitDataRecord) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#data = None;
        let mut r#data_size = None;
        let mut r#timestamp = None;
        let mut unknown_fields = Vec::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => {
                    r#data = Some(field.into_value());
                }
                1u8 => {
                    r#data_size = Some(field.into_value());
                }
                253u8 => {
                    r#timestamp = Some(field.into_value());
                }
                _ => {
                    unknown_fields.push(field);
                }
            }
        }
        Ok(Self {
            r#data,
            r#data_size,
            r#timestamp,
            unknown_fields,
        })
    }
}
impl TryFrom<FitDataRecord> for HsaConfigurationData {
    type Error = TryFromRecordError;
    fn try_from(record: FitDataRecord) -> Result<Self, Self::Error> {
        Self::parse(record)
    }
}
#[doc = "Wrist temperature data used for HSA custom data logging"]
#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize)]
pub struct HsaWristTemperatureData {
    pub r#processing_interval: Option<Value>,
    pub r#value: Option<Value>,
    pub r#timestamp: Option<Value>,
    #[doc = r" All fields that are not defined in the profile."]
    pub unknown_fields: Vec<FitDataField>,
}
impl FitMessage for HsaWristTemperatureData {
    const NAME: &'static str = "HsaWristTemperatureData";
    const KIND: MesgNum = MesgNum::HsaWristTemperatureData;
    fn parse(record: FitDataRecord) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#processing_interval = None;
        let mut r#value = None;
        let mut r#timestamp = None;
        let mut unknown_fields = Vec::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => {
                    r#processing_interval = Some(field.into_value());
                }
                1u8 => {
                    r#value = Some(field.into_value());
                }
                253u8 => {
                    r#timestamp = Some(field.into_value());
                }
                _ => {
                    unknown_fields.push(field);
                }
            }
        }
        Ok(Self {
            r#processing_interval,
            r#value,
            r#timestamp,
            unknown_fields,
        })
    }
}
impl TryFrom<FitDataRecord> for HsaWristTemperatureData {
    type Error = TryFromRecordError;
    fn try_from(record: FitDataRecord) -> Result<Self, Self::Error> {
        Self::parse(record)
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize)]
pub struct MemoGlob {
    pub r#memo: Option<Value>,
    pub r#mesg_num: Option<Value>,
    pub r#parent_index: Option<Value>,
    pub r#field_num: Option<Value>,
    pub r#data: Option<Value>,
    pub r#part_index: Option<Value>,
    #[doc = r" All fields that are not defined in the profile."]
    pub unknown_fields: Vec<FitDataField>,
}
impl FitMessage for MemoGlob {
    const NAME: &'static str = "MemoGlob";
    const KIND: MesgNum = MesgNum::MemoGlob;
    fn parse(record: FitDataRecord) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#memo = None;
        let mut r#mesg_num = None;
        let mut r#parent_index = None;
        let mut r#field_num = None;
        let mut r#data = None;
        let mut r#part_index = None;
        let mut unknown_fields = Vec::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => {
                    r#memo = Some(field.into_value());
                }
                1u8 => {
                    r#mesg_num = Some(field.into_value());
                }
                2u8 => {
                    r#parent_index = Some(field.into_value());
                }
                3u8 => {
                    r#field_num = Some(field.into_value());
                }
                4u8 => {
                    r#data = Some(field.into_value());
                }
                250u8 => {
                    r#part_index = Some(field.into_value());
                }
                _ => {
                    unknown_fields.push(field);
                }
            }
        }
        Ok(Self {
            r#memo,
            r#mesg_num,
            r#parent_index,
            r#field_num,
            r#data,
            r#part_index,
            unknown_fields,
        })
    }
}
impl TryFrom<FitDataRecord> for MemoGlob {
    type Error = TryFromRecordError;
    fn try_from(record: FitDataRecord) -> Result<Self, Self::Error> {
        Self::parse(record)
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize)]
pub struct SleepLevel {
    pub r#sleep_level: Option<Value>,
    pub r#timestamp: Option<Value>,
    #[doc = r" All fields that are not defined in the profile."]
    pub unknown_fields: Vec<FitDataField>,
}
impl FitMessage for SleepLevel {
    const NAME: &'static str = "SleepLevel";
    const KIND: MesgNum = MesgNum::SleepLevel;
    fn parse(record: FitDataRecord) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#sleep_level = None;
        let mut r#timestamp = None;
        let mut unknown_fields = Vec::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => {
                    r#sleep_level = Some(field.into_value());
                }
                253u8 => {
                    r#timestamp = Some(field.into_value());
                }
                _ => {
                    unknown_fields.push(field);
                }
            }
        }
        Ok(Self {
            r#sleep_level,
            r#timestamp,
            unknown_fields,
        })
    }
}
impl TryFrom<FitDataRecord> for SleepLevel {
    type Error = TryFromRecordError;
    fn try_from(record: FitDataRecord) -> Result<Self, Self::Error> {
        Self::parse(record)
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize)]
pub struct AntChannelId {
    pub r#channel_number: Option<Value>,
    pub r#device_type: Option<Value>,
    pub r#device_number: Option<Value>,
    pub r#transmission_type: Option<Value>,
    pub r#device_index: Option<Value>,
    #[doc = r" All fields that are not defined in the profile."]
    pub unknown_fields: Vec<FitDataField>,
}
impl FitMessage for AntChannelId {
    const NAME: &'static str = "AntChannelId";
    const KIND: MesgNum = MesgNum::AntChannelId;
    fn parse(record: FitDataRecord) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#channel_number = None;
        let mut r#device_type = None;
        let mut r#device_number = None;
        let mut r#transmission_type = None;
        let mut r#device_index = None;
        let mut unknown_fields = Vec::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => {
                    r#channel_number = Some(field.into_value());
                }
                1u8 => {
                    r#device_type = Some(field.into_value());
                }
                2u8 => {
                    r#device_number = Some(field.into_value());
                }
                3u8 => {
                    r#transmission_type = Some(field.into_value());
                }
                4u8 => {
                    r#device_index = Some(field.into_value());
                }
                _ => {
                    unknown_fields.push(field);
                }
            }
        }
        Ok(Self {
            r#channel_number,
            r#device_type,
            r#device_number,
            r#transmission_type,
            r#device_index,
            unknown_fields,
        })
    }
}
impl TryFrom<FitDataRecord> for AntChannelId {
    type Error = TryFromRecordError;
    fn try_from(record: FitDataRecord) -> Result<Self, Self::Error> {
        Self::parse(record)
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize)]
pub struct AntRx {
    pub r#fractional_timestamp: Option<Value>,
    pub r#mesg_id: Option<Value>,
    pub r#mesg_data: Option<Value>,
    pub r#channel_number: Option<Value>,
    pub r#data: Option<Value>,
    pub r#timestamp: Option<Value>,
    #[doc = r" All fields that are not defined in the profile."]
    pub unknown_fields: Vec<FitDataField>,
}
impl FitMessage for AntRx {
    const NAME: &'static str = "AntRx";
    const KIND: MesgNum = MesgNum::AntRx;
    fn parse(record: FitDataRecord) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#fractional_timestamp = None;
        let mut r#mesg_id = None;
        let mut r#mesg_data = None;
        let mut r#channel_number = None;
        let mut r#data = None;
        let mut r#timestamp = None;
        let mut unknown_fields = Vec::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => {
                    r#fractional_timestamp = Some(field.into_value());
                }
                1u8 => {
                    r#mesg_id = Some(field.into_value());
                }
                2u8 => {
                    r#mesg_data = Some(field.into_value());
                }
                3u8 => {
                    r#channel_number = Some(field.into_value());
                }
                4u8 => {
                    r#data = Some(field.into_value());
                }
                253u8 => {
                    r#timestamp = Some(field.into_value());
                }
                _ => {
                    unknown_fields.push(field);
                }
            }
        }
        Ok(Self {
            r#fractional_timestamp,
            r#mesg_id,
            r#mesg_data,
            r#channel_number,
            r#data,
            r#timestamp,
            unknown_fields,
        })
    }
}
impl TryFrom<FitDataRecord> for AntRx {
    type Error = TryFromRecordError;
    fn try_from(record: FitDataRecord) -> Result<Self, Self::Error> {
        Self::parse(record)
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize)]
pub struct AntTx {
    pub r#fractional_timestamp: Option<Value>,
    pub r#mesg_id: Option<Value>,
    pub r#mesg_data: Option<Value>,
    pub r#channel_number: Option<Value>,
    pub r#data: Option<Value>,
    pub r#timestamp: Option<Value>,
    #[doc = r" All fields that are not defined in the profile."]
    pub unknown_fields: Vec<FitDataField>,
}
impl FitMessage for AntTx {
    const NAME: &'static str = "AntTx";
    const KIND: MesgNum = MesgNum::AntTx;
    fn parse(record: FitDataRecord) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#fractional_timestamp = None;
        let mut r#mesg_id = None;
        let mut r#mesg_data = None;
        let mut r#channel_number = None;
        let mut r#data = None;
        let mut r#timestamp = None;
        let mut unknown_fields = Vec::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => {
                    r#fractional_timestamp = Some(field.into_value());
                }
                1u8 => {
                    r#mesg_id = Some(field.into_value());
                }
                2u8 => {
                    r#mesg_data = Some(field.into_value());
                }
                3u8 => {
                    r#channel_number = Some(field.into_value());
                }
                4u8 => {
                    r#data = Some(field.into_value());
                }
                253u8 => {
                    r#timestamp = Some(field.into_value());
                }
                _ => {
                    unknown_fields.push(field);
                }
            }
        }
        Ok(Self {
            r#fractional_timestamp,
            r#mesg_id,
            r#mesg_data,
            r#channel_number,
            r#data,
            r#timestamp,
            unknown_fields,
        })
    }
}
impl TryFrom<FitDataRecord> for AntTx {
    type Error = TryFromRecordError;
    fn try_from(record: FitDataRecord) -> Result<Self, Self::Error> {
        Self::parse(record)
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize)]
pub struct ExdScreenConfiguration {
    pub r#screen_index: Option<Value>,
    pub r#field_count: Option<Value>,
    pub r#layout: Option<Value>,
    pub r#screen_enabled: Option<Value>,
    #[doc = r" All fields that are not defined in the profile."]
    pub unknown_fields: Vec<FitDataField>,
}
impl FitMessage for ExdScreenConfiguration {
    const NAME: &'static str = "ExdScreenConfiguration";
    const KIND: MesgNum = MesgNum::ExdScreenConfiguration;
    fn parse(record: FitDataRecord) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#screen_index = None;
        let mut r#field_count = None;
        let mut r#layout = None;
        let mut r#screen_enabled = None;
        let mut unknown_fields = Vec::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => {
                    r#screen_index = Some(field.into_value());
                }
                1u8 => {
                    r#field_count = Some(field.into_value());
                }
                2u8 => {
                    r#layout = Some(field.into_value());
                }
                3u8 => {
                    r#screen_enabled = Some(field.into_value());
                }
                _ => {
                    unknown_fields.push(field);
                }
            }
        }
        Ok(Self {
            r#screen_index,
            r#field_count,
            r#layout,
            r#screen_enabled,
            unknown_fields,
        })
    }
}
impl TryFrom<FitDataRecord> for ExdScreenConfiguration {
    type Error = TryFromRecordError;
    fn try_from(record: FitDataRecord) -> Result<Self, Self::Error> {
        Self::parse(record)
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize)]
pub struct ExdDataFieldConfiguration {
    pub r#screen_index: Option<Value>,
    pub r#concept_field: Option<Value>,
    pub r#field_id: Option<Value>,
    pub r#concept_count: Option<Value>,
    pub r#display_type: Option<Value>,
    pub r#title: Option<Value>,
    #[doc = r" All fields that are not defined in the profile."]
    pub unknown_fields: Vec<FitDataField>,
}
impl FitMessage for ExdDataFieldConfiguration {
    const NAME: &'static str = "ExdDataFieldConfiguration";
    const KIND: MesgNum = MesgNum::ExdDataFieldConfiguration;
    fn parse(record: FitDataRecord) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#screen_index = None;
        let mut r#concept_field = None;
        let mut r#field_id = None;
        let mut r#concept_count = None;
        let mut r#display_type = None;
        let mut r#title = None;
        let mut unknown_fields = Vec::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => {
                    r#screen_index = Some(field.into_value());
                }
                1u8 => {
                    r#concept_field = Some(field.into_value());
                }
                2u8 => {
                    r#field_id = Some(field.into_value());
                }
                3u8 => {
                    r#concept_count = Some(field.into_value());
                }
                4u8 => {
                    r#display_type = Some(field.into_value());
                }
                5u8 => {
                    r#title = Some(field.into_value());
                }
                _ => {
                    unknown_fields.push(field);
                }
            }
        }
        Ok(Self {
            r#screen_index,
            r#concept_field,
            r#field_id,
            r#concept_count,
            r#display_type,
            r#title,
            unknown_fields,
        })
    }
}
impl TryFrom<FitDataRecord> for ExdDataFieldConfiguration {
    type Error = TryFromRecordError;
    fn try_from(record: FitDataRecord) -> Result<Self, Self::Error> {
        Self::parse(record)
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize)]
pub struct ExdDataConceptConfiguration {
    pub r#screen_index: Option<Value>,
    pub r#concept_field: Option<Value>,
    pub r#field_id: Option<Value>,
    pub r#concept_index: Option<Value>,
    pub r#data_page: Option<Value>,
    pub r#concept_key: Option<Value>,
    pub r#scaling: Option<Value>,
    pub r#data_units: Option<Value>,
    pub r#qualifier: Option<Value>,
    pub r#descriptor: Option<Value>,
    pub r#is_signed: Option<Value>,
    #[doc = r" All fields that are not defined in the profile."]
    pub unknown_fields: Vec<FitDataField>,
}
impl FitMessage for ExdDataConceptConfiguration {
    const NAME: &'static str = "ExdDataConceptConfiguration";
    const KIND: MesgNum = MesgNum::ExdDataConceptConfiguration;
    fn parse(record: FitDataRecord) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#screen_index = None;
        let mut r#concept_field = None;
        let mut r#field_id = None;
        let mut r#concept_index = None;
        let mut r#data_page = None;
        let mut r#concept_key = None;
        let mut r#scaling = None;
        let mut r#data_units = None;
        let mut r#qualifier = None;
        let mut r#descriptor = None;
        let mut r#is_signed = None;
        let mut unknown_fields = Vec::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => {
                    r#screen_index = Some(field.into_value());
                }
                1u8 => {
                    r#concept_field = Some(field.into_value());
                }
                2u8 => {
                    r#field_id = Some(field.into_value());
                }
                3u8 => {
                    r#concept_index = Some(field.into_value());
                }
                4u8 => {
                    r#data_page = Some(field.into_value());
                }
                5u8 => {
                    r#concept_key = Some(field.into_value());
                }
                6u8 => {
                    r#scaling = Some(field.into_value());
                }
                8u8 => {
                    r#data_units = Some(field.into_value());
                }
                9u8 => {
                    r#qualifier = Some(field.into_value());
                }
                10u8 => {
                    r#descriptor = Some(field.into_value());
                }
                11u8 => {
                    r#is_signed = Some(field.into_value());
                }
                _ => {
                    unknown_fields.push(field);
                }
            }
        }
        Ok(Self {
            r#screen_index,
            r#concept_field,
            r#field_id,
            r#concept_index,
            r#data_page,
            r#concept_key,
            r#scaling,
            r#data_units,
            r#qualifier,
            r#descriptor,
            r#is_signed,
            unknown_fields,
        })
    }
}
impl TryFrom<FitDataRecord> for ExdDataConceptConfiguration {
    type Error = TryFromRecordError;
    fn try_from(record: FitDataRecord) -> Result<Self, Self::Error> {
        Self::parse(record)
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize)]
pub struct DiveSummary {
    pub r#reference_mesg: Option<Value>,
    pub r#reference_index: Option<Value>,
    pub r#avg_depth: Option<Value>,
    pub r#max_depth: Option<Value>,
    pub r#surface_interval: Option<Value>,
    pub r#start_cns: Option<Value>,
    pub r#end_cns: Option<Value>,
    pub r#start_n2: Option<Value>,
    pub r#end_n2: Option<Value>,
    pub r#o2_toxicity: Option<Value>,
    pub r#dive_number: Option<Value>,
    pub r#bottom_time: Option<Value>,
    pub r#avg_pressure_sac: Option<Value>,
    pub r#avg_volume_sac: Option<Value>,
    pub r#avg_rmv: Option<Value>,
    pub r#descent_time: Option<Value>,
    pub r#ascent_time: Option<Value>,
    pub r#avg_ascent_rate: Option<Value>,
    pub r#avg_descent_rate: Option<Value>,
    pub r#max_ascent_rate: Option<Value>,
    pub r#max_descent_rate: Option<Value>,
    pub r#hang_time: Option<Value>,
    pub r#timestamp: Option<Value>,
    #[doc = r" All fields that are not defined in the profile."]
    pub unknown_fields: Vec<FitDataField>,
}
impl FitMessage for DiveSummary {
    const NAME: &'static str = "DiveSummary";
    const KIND: MesgNum = MesgNum::DiveSummary;
    fn parse(record: FitDataRecord) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#reference_mesg = None;
        let mut r#reference_index = None;
        let mut r#avg_depth = None;
        let mut r#max_depth = None;
        let mut r#surface_interval = None;
        let mut r#start_cns = None;
        let mut r#end_cns = None;
        let mut r#start_n2 = None;
        let mut r#end_n2 = None;
        let mut r#o2_toxicity = None;
        let mut r#dive_number = None;
        let mut r#bottom_time = None;
        let mut r#avg_pressure_sac = None;
        let mut r#avg_volume_sac = None;
        let mut r#avg_rmv = None;
        let mut r#descent_time = None;
        let mut r#ascent_time = None;
        let mut r#avg_ascent_rate = None;
        let mut r#avg_descent_rate = None;
        let mut r#max_ascent_rate = None;
        let mut r#max_descent_rate = None;
        let mut r#hang_time = None;
        let mut r#timestamp = None;
        let mut unknown_fields = Vec::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => {
                    r#reference_mesg = Some(field.into_value());
                }
                1u8 => {
                    r#reference_index = Some(field.into_value());
                }
                2u8 => {
                    r#avg_depth = Some(field.into_value());
                }
                3u8 => {
                    r#max_depth = Some(field.into_value());
                }
                4u8 => {
                    r#surface_interval = Some(field.into_value());
                }
                5u8 => {
                    r#start_cns = Some(field.into_value());
                }
                6u8 => {
                    r#end_cns = Some(field.into_value());
                }
                7u8 => {
                    r#start_n2 = Some(field.into_value());
                }
                8u8 => {
                    r#end_n2 = Some(field.into_value());
                }
                9u8 => {
                    r#o2_toxicity = Some(field.into_value());
                }
                10u8 => {
                    r#dive_number = Some(field.into_value());
                }
                11u8 => {
                    r#bottom_time = Some(field.into_value());
                }
                12u8 => {
                    r#avg_pressure_sac = Some(field.into_value());
                }
                13u8 => {
                    r#avg_volume_sac = Some(field.into_value());
                }
                14u8 => {
                    r#avg_rmv = Some(field.into_value());
                }
                15u8 => {
                    r#descent_time = Some(field.into_value());
                }
                16u8 => {
                    r#ascent_time = Some(field.into_value());
                }
                17u8 => {
                    r#avg_ascent_rate = Some(field.into_value());
                }
                22u8 => {
                    r#avg_descent_rate = Some(field.into_value());
                }
                23u8 => {
                    r#max_ascent_rate = Some(field.into_value());
                }
                24u8 => {
                    r#max_descent_rate = Some(field.into_value());
                }
                25u8 => {
                    r#hang_time = Some(field.into_value());
                }
                253u8 => {
                    r#timestamp = Some(field.into_value());
                }
                _ => {
                    unknown_fields.push(field);
                }
            }
        }
        Ok(Self {
            r#reference_mesg,
            r#reference_index,
            r#avg_depth,
            r#max_depth,
            r#surface_interval,
            r#start_cns,
            r#end_cns,
            r#start_n2,
            r#end_n2,
            r#o2_toxicity,
            r#dive_number,
            r#bottom_time,
            r#avg_pressure_sac,
            r#avg_volume_sac,
            r#avg_rmv,
            r#descent_time,
            r#ascent_time,
            r#avg_ascent_rate,
            r#avg_descent_rate,
            r#max_ascent_rate,
            r#max_descent_rate,
            r#hang_time,
            r#timestamp,
            unknown_fields,
        })
    }
}
impl TryFrom<FitDataRecord> for DiveSummary {
    type Error = TryFromRecordError;
    fn try_from(record: FitDataRecord) -> Result<Self, Self::Error> {
        Self::parse(record)
    }
}
#[doc = "Number of acclerometer zero crossings summed over the specified time interval"]
#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize)]
pub struct AadAccelFeatures {
    pub r#time: Option<Value>,
    pub r#energy_total: Option<Value>,
    pub r#zero_cross_cnt: Option<Value>,
    pub r#instance: Option<Value>,
    pub r#time_above_threshold: Option<Value>,
    pub r#timestamp: Option<Value>,
    #[doc = r" All fields that are not defined in the profile."]
    pub unknown_fields: Vec<FitDataField>,
}
impl FitMessage for AadAccelFeatures {
    const NAME: &'static str = "AadAccelFeatures";
    const KIND: MesgNum = MesgNum::AadAccelFeatures;
    fn parse(record: FitDataRecord) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#time = None;
        let mut r#energy_total = None;
        let mut r#zero_cross_cnt = None;
        let mut r#instance = None;
        let mut r#time_above_threshold = None;
        let mut r#timestamp = None;
        let mut unknown_fields = Vec::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => {
                    r#time = Some(field.into_value());
                }
                1u8 => {
                    r#energy_total = Some(field.into_value());
                }
                2u8 => {
                    r#zero_cross_cnt = Some(field.into_value());
                }
                3u8 => {
                    r#instance = Some(field.into_value());
                }
                4u8 => {
                    r#time_above_threshold = Some(field.into_value());
                }
                253u8 => {
                    r#timestamp = Some(field.into_value());
                }
                _ => {
                    unknown_fields.push(field);
                }
            }
        }
        Ok(Self {
            r#time,
            r#energy_total,
            r#zero_cross_cnt,
            r#instance,
            r#time_above_threshold,
            r#timestamp,
            unknown_fields,
        })
    }
}
impl TryFrom<FitDataRecord> for AadAccelFeatures {
    type Error = TryFromRecordError;
    fn try_from(record: FitDataRecord) -> Result<Self, Self::Error> {
        Self::parse(record)
    }
}
#[doc = "Heart rate variability"]
#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize)]
pub struct Hrv {
    pub r#time: Option<Value>,
    #[doc = r" All fields that are not defined in the profile."]
    pub unknown_fields: Vec<FitDataField>,
}
impl FitMessage for Hrv {
    const NAME: &'static str = "Hrv";
    const KIND: MesgNum = MesgNum::Hrv;
    fn parse(record: FitDataRecord) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#time = None;
        let mut unknown_fields = Vec::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => {
                    r#time = Some(field.into_value());
                }
                _ => {
                    unknown_fields.push(field);
                }
            }
        }
        Ok(Self {
            r#time,
            unknown_fields,
        })
    }
}
impl TryFrom<FitDataRecord> for Hrv {
    type Error = TryFromRecordError;
    fn try_from(record: FitDataRecord) -> Result<Self, Self::Error> {
        Self::parse(record)
    }
}
#[doc = "Array of heart beat intervals"]
#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize)]
pub struct BeatIntervals {
    pub r#timestamp_ms: Option<Value>,
    pub r#time: Option<Value>,
    pub r#timestamp: Option<Value>,
    #[doc = r" All fields that are not defined in the profile."]
    pub unknown_fields: Vec<FitDataField>,
}
impl FitMessage for BeatIntervals {
    const NAME: &'static str = "BeatIntervals";
    const KIND: MesgNum = MesgNum::BeatIntervals;
    fn parse(record: FitDataRecord) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#timestamp_ms = None;
        let mut r#time = None;
        let mut r#timestamp = None;
        let mut unknown_fields = Vec::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => {
                    r#timestamp_ms = Some(field.into_value());
                }
                1u8 => {
                    r#time = Some(field.into_value());
                }
                253u8 => {
                    r#timestamp = Some(field.into_value());
                }
                _ => {
                    unknown_fields.push(field);
                }
            }
        }
        Ok(Self {
            r#timestamp_ms,
            r#time,
            r#timestamp,
            unknown_fields,
        })
    }
}
impl TryFrom<FitDataRecord> for BeatIntervals {
    type Error = TryFromRecordError;
    fn try_from(record: FitDataRecord) -> Result<Self, Self::Error> {
        Self::parse(record)
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize)]
pub struct HrvStatusSummary {
    pub r#weekly_average: Option<Value>,
    pub r#last_night_average: Option<Value>,
    pub r#last_night_5_min_high: Option<Value>,
    pub r#baseline_low_upper: Option<Value>,
    pub r#baseline_balanced_lower: Option<Value>,
    pub r#baseline_balanced_upper: Option<Value>,
    pub r#status: Option<Value>,
    pub r#timestamp: Option<Value>,
    #[doc = r" All fields that are not defined in the profile."]
    pub unknown_fields: Vec<FitDataField>,
}
impl FitMessage for HrvStatusSummary {
    const NAME: &'static str = "HrvStatusSummary";
    const KIND: MesgNum = MesgNum::HrvStatusSummary;
    fn parse(record: FitDataRecord) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#weekly_average = None;
        let mut r#last_night_average = None;
        let mut r#last_night_5_min_high = None;
        let mut r#baseline_low_upper = None;
        let mut r#baseline_balanced_lower = None;
        let mut r#baseline_balanced_upper = None;
        let mut r#status = None;
        let mut r#timestamp = None;
        let mut unknown_fields = Vec::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => {
                    r#weekly_average = Some(field.into_value());
                }
                1u8 => {
                    r#last_night_average = Some(field.into_value());
                }
                2u8 => {
                    r#last_night_5_min_high = Some(field.into_value());
                }
                3u8 => {
                    r#baseline_low_upper = Some(field.into_value());
                }
                4u8 => {
                    r#baseline_balanced_lower = Some(field.into_value());
                }
                5u8 => {
                    r#baseline_balanced_upper = Some(field.into_value());
                }
                6u8 => {
                    r#status = Some(field.into_value());
                }
                253u8 => {
                    r#timestamp = Some(field.into_value());
                }
                _ => {
                    unknown_fields.push(field);
                }
            }
        }
        Ok(Self {
            r#weekly_average,
            r#last_night_average,
            r#last_night_5_min_high,
            r#baseline_low_upper,
            r#baseline_balanced_lower,
            r#baseline_balanced_upper,
            r#status,
            r#timestamp,
            unknown_fields,
        })
    }
}
impl TryFrom<FitDataRecord> for HrvStatusSummary {
    type Error = TryFromRecordError;
    fn try_from(record: FitDataRecord) -> Result<Self, Self::Error> {
        Self::parse(record)
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize)]
pub struct HrvValue {
    pub r#value: Option<Value>,
    pub r#timestamp: Option<Value>,
    #[doc = r" All fields that are not defined in the profile."]
    pub unknown_fields: Vec<FitDataField>,
}
impl FitMessage for HrvValue {
    const NAME: &'static str = "HrvValue";
    const KIND: MesgNum = MesgNum::HrvValue;
    fn parse(record: FitDataRecord) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#value = None;
        let mut r#timestamp = None;
        let mut unknown_fields = Vec::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => {
                    r#value = Some(field.into_value());
                }
                253u8 => {
                    r#timestamp = Some(field.into_value());
                }
                _ => {
                    unknown_fields.push(field);
                }
            }
        }
        Ok(Self {
            r#value,
            r#timestamp,
            unknown_fields,
        })
    }
}
impl TryFrom<FitDataRecord> for HrvValue {
    type Error = TryFromRecordError;
    fn try_from(record: FitDataRecord) -> Result<Self, Self::Error> {
        Self::parse(record)
    }
}
#[doc = "Raw Beat-to-Beat Interval values"]
#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize)]
pub struct RawBbi {
    pub r#timestamp_ms: Option<Value>,
    pub r#data: Option<Value>,
    pub r#time: Option<Value>,
    pub r#quality: Option<Value>,
    pub r#gap: Option<Value>,
    pub r#timestamp: Option<Value>,
    #[doc = r" All fields that are not defined in the profile."]
    pub unknown_fields: Vec<FitDataField>,
}
impl FitMessage for RawBbi {
    const NAME: &'static str = "RawBbi";
    const KIND: MesgNum = MesgNum::RawBbi;
    fn parse(record: FitDataRecord) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#timestamp_ms = None;
        let mut r#data = None;
        let mut r#time = None;
        let mut r#quality = None;
        let mut r#gap = None;
        let mut r#timestamp = None;
        let mut unknown_fields = Vec::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => {
                    r#timestamp_ms = Some(field.into_value());
                }
                1u8 => {
                    r#data = Some(field.into_value());
                }
                2u8 => {
                    r#time = Some(field.into_value());
                }
                3u8 => {
                    r#quality = Some(field.into_value());
                }
                4u8 => {
                    r#gap = Some(field.into_value());
                }
                253u8 => {
                    r#timestamp = Some(field.into_value());
                }
                _ => {
                    unknown_fields.push(field);
                }
            }
        }
        Ok(Self {
            r#timestamp_ms,
            r#data,
            r#time,
            r#quality,
            r#gap,
            r#timestamp,
            unknown_fields,
        })
    }
}
impl TryFrom<FitDataRecord> for RawBbi {
    type Error = TryFromRecordError;
    fn try_from(record: FitDataRecord) -> Result<Self, Self::Error> {
        Self::parse(record)
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize)]
pub struct RespirationRate {
    pub r#respiration_rate: Option<Value>,
    pub r#timestamp: Option<Value>,
    #[doc = r" All fields that are not defined in the profile."]
    pub unknown_fields: Vec<FitDataField>,
}
impl FitMessage for RespirationRate {
    const NAME: &'static str = "RespirationRate";
    const KIND: MesgNum = MesgNum::RespirationRate;
    fn parse(record: FitDataRecord) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#respiration_rate = None;
        let mut r#timestamp = None;
        let mut unknown_fields = Vec::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => {
                    r#respiration_rate = Some(field.into_value());
                }
                253u8 => {
                    r#timestamp = Some(field.into_value());
                }
                _ => {
                    unknown_fields.push(field);
                }
            }
        }
        Ok(Self {
            r#respiration_rate,
            r#timestamp,
            unknown_fields,
        })
    }
}
impl TryFrom<FitDataRecord> for RespirationRate {
    type Error = TryFromRecordError;
    fn try_from(record: FitDataRecord) -> Result<Self, Self::Error> {
        Self::parse(record)
    }
}
#[doc = "Specifically used for XERO products."]
#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize)]
pub struct ChronoShotSession {
    pub r#min_speed: Option<Value>,
    pub r#max_speed: Option<Value>,
    pub r#avg_speed: Option<Value>,
    pub r#shot_count: Option<Value>,
    pub r#projectile_type: Option<Value>,
    pub r#grain_weight: Option<Value>,
    pub r#timestamp: Option<Value>,
    #[doc = r" All fields that are not defined in the profile."]
    pub unknown_fields: Vec<FitDataField>,
}
impl FitMessage for ChronoShotSession {
    const NAME: &'static str = "ChronoShotSession";
    const KIND: MesgNum = MesgNum::ChronoShotSession;
    fn parse(record: FitDataRecord) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#min_speed = None;
        let mut r#max_speed = None;
        let mut r#avg_speed = None;
        let mut r#shot_count = None;
        let mut r#projectile_type = None;
        let mut r#grain_weight = None;
        let mut r#timestamp = None;
        let mut unknown_fields = Vec::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => {
                    r#min_speed = Some(field.into_value());
                }
                1u8 => {
                    r#max_speed = Some(field.into_value());
                }
                2u8 => {
                    r#avg_speed = Some(field.into_value());
                }
                3u8 => {
                    r#shot_count = Some(field.into_value());
                }
                4u8 => {
                    r#projectile_type = Some(field.into_value());
                }
                5u8 => {
                    r#grain_weight = Some(field.into_value());
                }
                253u8 => {
                    r#timestamp = Some(field.into_value());
                }
                _ => {
                    unknown_fields.push(field);
                }
            }
        }
        Ok(Self {
            r#min_speed,
            r#max_speed,
            r#avg_speed,
            r#shot_count,
            r#projectile_type,
            r#grain_weight,
            r#timestamp,
            unknown_fields,
        })
    }
}
impl TryFrom<FitDataRecord> for ChronoShotSession {
    type Error = TryFromRecordError;
    fn try_from(record: FitDataRecord) -> Result<Self, Self::Error> {
        Self::parse(record)
    }
}
#[doc = "Specifically used for XERO products."]
#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize)]
pub struct ChronoShotData {
    pub r#shot_speed: Option<Value>,
    pub r#shot_num: Option<Value>,
    pub r#timestamp: Option<Value>,
    #[doc = r" All fields that are not defined in the profile."]
    pub unknown_fields: Vec<FitDataField>,
}
impl FitMessage for ChronoShotData {
    const NAME: &'static str = "ChronoShotData";
    const KIND: MesgNum = MesgNum::ChronoShotData;
    fn parse(record: FitDataRecord) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#shot_speed = None;
        let mut r#shot_num = None;
        let mut r#timestamp = None;
        let mut unknown_fields = Vec::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => {
                    r#shot_speed = Some(field.into_value());
                }
                1u8 => {
                    r#shot_num = Some(field.into_value());
                }
                253u8 => {
                    r#timestamp = Some(field.into_value());
                }
                _ => {
                    unknown_fields.push(field);
                }
            }
        }
        Ok(Self {
            r#shot_speed,
            r#shot_num,
            r#timestamp,
            unknown_fields,
        })
    }
}
impl TryFrom<FitDataRecord> for ChronoShotData {
    type Error = TryFromRecordError;
    fn try_from(record: FitDataRecord) -> Result<Self, Self::Error> {
        Self::parse(record)
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize)]
pub struct TankUpdate {
    pub r#sensor: Option<Value>,
    pub r#pressure: Option<Value>,
    pub r#timestamp: Option<Value>,
    #[doc = r" All fields that are not defined in the profile."]
    pub unknown_fields: Vec<FitDataField>,
}
impl FitMessage for TankUpdate {
    const NAME: &'static str = "TankUpdate";
    const KIND: MesgNum = MesgNum::TankUpdate;
    fn parse(record: FitDataRecord) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#sensor = None;
        let mut r#pressure = None;
        let mut r#timestamp = None;
        let mut unknown_fields = Vec::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => {
                    r#sensor = Some(field.into_value());
                }
                1u8 => {
                    r#pressure = Some(field.into_value());
                }
                253u8 => {
                    r#timestamp = Some(field.into_value());
                }
                _ => {
                    unknown_fields.push(field);
                }
            }
        }
        Ok(Self {
            r#sensor,
            r#pressure,
            r#timestamp,
            unknown_fields,
        })
    }
}
impl TryFrom<FitDataRecord> for TankUpdate {
    type Error = TryFromRecordError;
    fn try_from(record: FitDataRecord) -> Result<Self, Self::Error> {
        Self::parse(record)
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize)]
pub struct TankSummary {
    pub r#sensor: Option<Value>,
    pub r#start_pressure: Option<Value>,
    pub r#end_pressure: Option<Value>,
    pub r#volume_used: Option<Value>,
    pub r#timestamp: Option<Value>,
    #[doc = r" All fields that are not defined in the profile."]
    pub unknown_fields: Vec<FitDataField>,
}
impl FitMessage for TankSummary {
    const NAME: &'static str = "TankSummary";
    const KIND: MesgNum = MesgNum::TankSummary;
    fn parse(record: FitDataRecord) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#sensor = None;
        let mut r#start_pressure = None;
        let mut r#end_pressure = None;
        let mut r#volume_used = None;
        let mut r#timestamp = None;
        let mut unknown_fields = Vec::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => {
                    r#sensor = Some(field.into_value());
                }
                1u8 => {
                    r#start_pressure = Some(field.into_value());
                }
                2u8 => {
                    r#end_pressure = Some(field.into_value());
                }
                3u8 => {
                    r#volume_used = Some(field.into_value());
                }
                253u8 => {
                    r#timestamp = Some(field.into_value());
                }
                _ => {
                    unknown_fields.push(field);
                }
            }
        }
        Ok(Self {
            r#sensor,
            r#start_pressure,
            r#end_pressure,
            r#volume_used,
            r#timestamp,
            unknown_fields,
        })
    }
}
impl TryFrom<FitDataRecord> for TankSummary {
    type Error = TryFromRecordError;
    fn try_from(record: FitDataRecord) -> Result<Self, Self::Error> {
        Self::parse(record)
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize)]
pub struct SleepAssessment {
    pub r#combined_awake_score: Option<Value>,
    pub r#awake_time_score: Option<Value>,
    pub r#awakenings_count_score: Option<Value>,
    pub r#deep_sleep_score: Option<Value>,
    pub r#sleep_duration_score: Option<Value>,
    pub r#light_sleep_score: Option<Value>,
    pub r#overall_sleep_score: Option<Value>,
    pub r#sleep_quality_score: Option<Value>,
    pub r#sleep_recovery_score: Option<Value>,
    pub r#rem_sleep_score: Option<Value>,
    pub r#sleep_restlessness_score: Option<Value>,
    pub r#awakenings_count: Option<Value>,
    pub r#interruptions_score: Option<Value>,
    pub r#average_stress_during_sleep: Option<Value>,
    #[doc = r" All fields that are not defined in the profile."]
    pub unknown_fields: Vec<FitDataField>,
}
impl FitMessage for SleepAssessment {
    const NAME: &'static str = "SleepAssessment";
    const KIND: MesgNum = MesgNum::SleepAssessment;
    fn parse(record: FitDataRecord) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#combined_awake_score = None;
        let mut r#awake_time_score = None;
        let mut r#awakenings_count_score = None;
        let mut r#deep_sleep_score = None;
        let mut r#sleep_duration_score = None;
        let mut r#light_sleep_score = None;
        let mut r#overall_sleep_score = None;
        let mut r#sleep_quality_score = None;
        let mut r#sleep_recovery_score = None;
        let mut r#rem_sleep_score = None;
        let mut r#sleep_restlessness_score = None;
        let mut r#awakenings_count = None;
        let mut r#interruptions_score = None;
        let mut r#average_stress_during_sleep = None;
        let mut unknown_fields = Vec::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => {
                    r#combined_awake_score = Some(field.into_value());
                }
                1u8 => {
                    r#awake_time_score = Some(field.into_value());
                }
                2u8 => {
                    r#awakenings_count_score = Some(field.into_value());
                }
                3u8 => {
                    r#deep_sleep_score = Some(field.into_value());
                }
                4u8 => {
                    r#sleep_duration_score = Some(field.into_value());
                }
                5u8 => {
                    r#light_sleep_score = Some(field.into_value());
                }
                6u8 => {
                    r#overall_sleep_score = Some(field.into_value());
                }
                7u8 => {
                    r#sleep_quality_score = Some(field.into_value());
                }
                8u8 => {
                    r#sleep_recovery_score = Some(field.into_value());
                }
                9u8 => {
                    r#rem_sleep_score = Some(field.into_value());
                }
                10u8 => {
                    r#sleep_restlessness_score = Some(field.into_value());
                }
                11u8 => {
                    r#awakenings_count = Some(field.into_value());
                }
                14u8 => {
                    r#interruptions_score = Some(field.into_value());
                }
                15u8 => {
                    r#average_stress_during_sleep = Some(field.into_value());
                }
                _ => {
                    unknown_fields.push(field);
                }
            }
        }
        Ok(Self {
            r#combined_awake_score,
            r#awake_time_score,
            r#awakenings_count_score,
            r#deep_sleep_score,
            r#sleep_duration_score,
            r#light_sleep_score,
            r#overall_sleep_score,
            r#sleep_quality_score,
            r#sleep_recovery_score,
            r#rem_sleep_score,
            r#sleep_restlessness_score,
            r#awakenings_count,
            r#interruptions_score,
            r#average_stress_during_sleep,
            unknown_fields,
        })
    }
}
impl TryFrom<FitDataRecord> for SleepAssessment {
    type Error = TryFromRecordError;
    fn try_from(record: FitDataRecord) -> Result<Self, Self::Error> {
        Self::parse(record)
    }
}
