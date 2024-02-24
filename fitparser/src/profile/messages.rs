#![allow(missing_docs)]
#![doc = "Auto generated profile messages from FIT SDK Release: 21.133.00"]
use crate::{
    profile::{
        field_types, FitMessage, FromValue, MesgNum, MessageParseOptions, TryFromRecordError,
    },
    FitDataRecord, ValueWithUnits,
};
use serde::Serialize;
use std::collections::BTreeMap;
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
    #[doc = r" Parse a message from a [`FitDataRecord`][] using the default options."]
    pub fn parse(record: FitDataRecord) -> Result<Self, TryFromRecordError> {
        Self::parse_with_options(record, Default::default())
    }
    #[doc = r" Parse a message from a [`FitDataRecord`][] using the given options."]
    pub fn parse_with_options(
        record: FitDataRecord,
        options: MessageParseOptions,
    ) -> Result<Self, TryFromRecordError> {
        match record.kind() {
            FileId::KIND => FileId::parse_with_options(record, options).map(Self::FileId),
            FileCreator::KIND => {
                FileCreator::parse_with_options(record, options).map(Self::FileCreator)
            }
            TimestampCorrelation::KIND => TimestampCorrelation::parse_with_options(record, options)
                .map(Self::TimestampCorrelation),
            Software::KIND => Software::parse_with_options(record, options).map(Self::Software),
            SlaveDevice::KIND => {
                SlaveDevice::parse_with_options(record, options).map(Self::SlaveDevice)
            }
            Capabilities::KIND => {
                Capabilities::parse_with_options(record, options).map(Self::Capabilities)
            }
            FileCapabilities::KIND => {
                FileCapabilities::parse_with_options(record, options).map(Self::FileCapabilities)
            }
            MesgCapabilities::KIND => {
                MesgCapabilities::parse_with_options(record, options).map(Self::MesgCapabilities)
            }
            FieldCapabilities::KIND => {
                FieldCapabilities::parse_with_options(record, options).map(Self::FieldCapabilities)
            }
            DeviceSettings::KIND => {
                DeviceSettings::parse_with_options(record, options).map(Self::DeviceSettings)
            }
            UserProfile::KIND => {
                UserProfile::parse_with_options(record, options).map(Self::UserProfile)
            }
            HrmProfile::KIND => {
                HrmProfile::parse_with_options(record, options).map(Self::HrmProfile)
            }
            SdmProfile::KIND => {
                SdmProfile::parse_with_options(record, options).map(Self::SdmProfile)
            }
            BikeProfile::KIND => {
                BikeProfile::parse_with_options(record, options).map(Self::BikeProfile)
            }
            Connectivity::KIND => {
                Connectivity::parse_with_options(record, options).map(Self::Connectivity)
            }
            WatchfaceSettings::KIND => {
                WatchfaceSettings::parse_with_options(record, options).map(Self::WatchfaceSettings)
            }
            OhrSettings::KIND => {
                OhrSettings::parse_with_options(record, options).map(Self::OhrSettings)
            }
            TimeInZone::KIND => {
                TimeInZone::parse_with_options(record, options).map(Self::TimeInZone)
            }
            ZonesTarget::KIND => {
                ZonesTarget::parse_with_options(record, options).map(Self::ZonesTarget)
            }
            Sport::KIND => Sport::parse_with_options(record, options).map(Self::Sport),
            HrZone::KIND => HrZone::parse_with_options(record, options).map(Self::HrZone),
            SpeedZone::KIND => SpeedZone::parse_with_options(record, options).map(Self::SpeedZone),
            CadenceZone::KIND => {
                CadenceZone::parse_with_options(record, options).map(Self::CadenceZone)
            }
            PowerZone::KIND => PowerZone::parse_with_options(record, options).map(Self::PowerZone),
            MetZone::KIND => MetZone::parse_with_options(record, options).map(Self::MetZone),
            DiveSettings::KIND => {
                DiveSettings::parse_with_options(record, options).map(Self::DiveSettings)
            }
            DiveAlarm::KIND => DiveAlarm::parse_with_options(record, options).map(Self::DiveAlarm),
            DiveApneaAlarm::KIND => {
                DiveApneaAlarm::parse_with_options(record, options).map(Self::DiveApneaAlarm)
            }
            DiveGas::KIND => DiveGas::parse_with_options(record, options).map(Self::DiveGas),
            Goal::KIND => Goal::parse_with_options(record, options).map(Self::Goal),
            Activity::KIND => Activity::parse_with_options(record, options).map(Self::Activity),
            Session::KIND => Session::parse_with_options(record, options).map(Self::Session),
            Lap::KIND => Lap::parse_with_options(record, options).map(Self::Lap),
            Length::KIND => Length::parse_with_options(record, options).map(Self::Length),
            Record::KIND => Record::parse_with_options(record, options).map(Self::Record),
            Event::KIND => Event::parse_with_options(record, options).map(Self::Event),
            DeviceInfo::KIND => {
                DeviceInfo::parse_with_options(record, options).map(Self::DeviceInfo)
            }
            DeviceAuxBatteryInfo::KIND => DeviceAuxBatteryInfo::parse_with_options(record, options)
                .map(Self::DeviceAuxBatteryInfo),
            TrainingFile::KIND => {
                TrainingFile::parse_with_options(record, options).map(Self::TrainingFile)
            }
            WeatherConditions::KIND => {
                WeatherConditions::parse_with_options(record, options).map(Self::WeatherConditions)
            }
            WeatherAlert::KIND => {
                WeatherAlert::parse_with_options(record, options).map(Self::WeatherAlert)
            }
            GpsMetadata::KIND => {
                GpsMetadata::parse_with_options(record, options).map(Self::GpsMetadata)
            }
            CameraEvent::KIND => {
                CameraEvent::parse_with_options(record, options).map(Self::CameraEvent)
            }
            GyroscopeData::KIND => {
                GyroscopeData::parse_with_options(record, options).map(Self::GyroscopeData)
            }
            AccelerometerData::KIND => {
                AccelerometerData::parse_with_options(record, options).map(Self::AccelerometerData)
            }
            MagnetometerData::KIND => {
                MagnetometerData::parse_with_options(record, options).map(Self::MagnetometerData)
            }
            BarometerData::KIND => {
                BarometerData::parse_with_options(record, options).map(Self::BarometerData)
            }
            ThreeDSensorCalibration::KIND => {
                ThreeDSensorCalibration::parse_with_options(record, options)
                    .map(Self::ThreeDSensorCalibration)
            }
            OneDSensorCalibration::KIND => {
                OneDSensorCalibration::parse_with_options(record, options)
                    .map(Self::OneDSensorCalibration)
            }
            VideoFrame::KIND => {
                VideoFrame::parse_with_options(record, options).map(Self::VideoFrame)
            }
            ObdiiData::KIND => ObdiiData::parse_with_options(record, options).map(Self::ObdiiData),
            NmeaSentence::KIND => {
                NmeaSentence::parse_with_options(record, options).map(Self::NmeaSentence)
            }
            AviationAttitude::KIND => {
                AviationAttitude::parse_with_options(record, options).map(Self::AviationAttitude)
            }
            Video::KIND => Video::parse_with_options(record, options).map(Self::Video),
            VideoTitle::KIND => {
                VideoTitle::parse_with_options(record, options).map(Self::VideoTitle)
            }
            VideoDescription::KIND => {
                VideoDescription::parse_with_options(record, options).map(Self::VideoDescription)
            }
            VideoClip::KIND => VideoClip::parse_with_options(record, options).map(Self::VideoClip),
            Set::KIND => Set::parse_with_options(record, options).map(Self::Set),
            Jump::KIND => Jump::parse_with_options(record, options).map(Self::Jump),
            Split::KIND => Split::parse_with_options(record, options).map(Self::Split),
            SplitSummary::KIND => {
                SplitSummary::parse_with_options(record, options).map(Self::SplitSummary)
            }
            ClimbPro::KIND => ClimbPro::parse_with_options(record, options).map(Self::ClimbPro),
            FieldDescription::KIND => {
                FieldDescription::parse_with_options(record, options).map(Self::FieldDescription)
            }
            DeveloperDataId::KIND => {
                DeveloperDataId::parse_with_options(record, options).map(Self::DeveloperDataId)
            }
            Course::KIND => Course::parse_with_options(record, options).map(Self::Course),
            CoursePoint::KIND => {
                CoursePoint::parse_with_options(record, options).map(Self::CoursePoint)
            }
            SegmentId::KIND => SegmentId::parse_with_options(record, options).map(Self::SegmentId),
            SegmentLeaderboardEntry::KIND => {
                SegmentLeaderboardEntry::parse_with_options(record, options)
                    .map(Self::SegmentLeaderboardEntry)
            }
            SegmentPoint::KIND => {
                SegmentPoint::parse_with_options(record, options).map(Self::SegmentPoint)
            }
            SegmentLap::KIND => {
                SegmentLap::parse_with_options(record, options).map(Self::SegmentLap)
            }
            SegmentFile::KIND => {
                SegmentFile::parse_with_options(record, options).map(Self::SegmentFile)
            }
            Workout::KIND => Workout::parse_with_options(record, options).map(Self::Workout),
            WorkoutSession::KIND => {
                WorkoutSession::parse_with_options(record, options).map(Self::WorkoutSession)
            }
            WorkoutStep::KIND => {
                WorkoutStep::parse_with_options(record, options).map(Self::WorkoutStep)
            }
            ExerciseTitle::KIND => {
                ExerciseTitle::parse_with_options(record, options).map(Self::ExerciseTitle)
            }
            Schedule::KIND => Schedule::parse_with_options(record, options).map(Self::Schedule),
            Totals::KIND => Totals::parse_with_options(record, options).map(Self::Totals),
            WeightScale::KIND => {
                WeightScale::parse_with_options(record, options).map(Self::WeightScale)
            }
            BloodPressure::KIND => {
                BloodPressure::parse_with_options(record, options).map(Self::BloodPressure)
            }
            MonitoringInfo::KIND => {
                MonitoringInfo::parse_with_options(record, options).map(Self::MonitoringInfo)
            }
            Monitoring::KIND => {
                Monitoring::parse_with_options(record, options).map(Self::Monitoring)
            }
            MonitoringHrData::KIND => {
                MonitoringHrData::parse_with_options(record, options).map(Self::MonitoringHrData)
            }
            Spo2Data::KIND => Spo2Data::parse_with_options(record, options).map(Self::Spo2Data),
            Hr::KIND => Hr::parse_with_options(record, options).map(Self::Hr),
            StressLevel::KIND => {
                StressLevel::parse_with_options(record, options).map(Self::StressLevel)
            }
            MaxMetData::KIND => {
                MaxMetData::parse_with_options(record, options).map(Self::MaxMetData)
            }
            HsaBodyBatteryData::KIND => HsaBodyBatteryData::parse_with_options(record, options)
                .map(Self::HsaBodyBatteryData),
            HsaEvent::KIND => HsaEvent::parse_with_options(record, options).map(Self::HsaEvent),
            HsaAccelerometerData::KIND => HsaAccelerometerData::parse_with_options(record, options)
                .map(Self::HsaAccelerometerData),
            HsaGyroscopeData::KIND => {
                HsaGyroscopeData::parse_with_options(record, options).map(Self::HsaGyroscopeData)
            }
            HsaStepData::KIND => {
                HsaStepData::parse_with_options(record, options).map(Self::HsaStepData)
            }
            HsaSpo2Data::KIND => {
                HsaSpo2Data::parse_with_options(record, options).map(Self::HsaSpo2Data)
            }
            HsaStressData::KIND => {
                HsaStressData::parse_with_options(record, options).map(Self::HsaStressData)
            }
            HsaRespirationData::KIND => HsaRespirationData::parse_with_options(record, options)
                .map(Self::HsaRespirationData),
            HsaHeartRateData::KIND => {
                HsaHeartRateData::parse_with_options(record, options).map(Self::HsaHeartRateData)
            }
            HsaConfigurationData::KIND => HsaConfigurationData::parse_with_options(record, options)
                .map(Self::HsaConfigurationData),
            HsaWristTemperatureData::KIND => {
                HsaWristTemperatureData::parse_with_options(record, options)
                    .map(Self::HsaWristTemperatureData)
            }
            MemoGlob::KIND => MemoGlob::parse_with_options(record, options).map(Self::MemoGlob),
            SleepLevel::KIND => {
                SleepLevel::parse_with_options(record, options).map(Self::SleepLevel)
            }
            AntChannelId::KIND => {
                AntChannelId::parse_with_options(record, options).map(Self::AntChannelId)
            }
            AntRx::KIND => AntRx::parse_with_options(record, options).map(Self::AntRx),
            AntTx::KIND => AntTx::parse_with_options(record, options).map(Self::AntTx),
            ExdScreenConfiguration::KIND => {
                ExdScreenConfiguration::parse_with_options(record, options)
                    .map(Self::ExdScreenConfiguration)
            }
            ExdDataFieldConfiguration::KIND => {
                ExdDataFieldConfiguration::parse_with_options(record, options)
                    .map(Self::ExdDataFieldConfiguration)
            }
            ExdDataConceptConfiguration::KIND => {
                ExdDataConceptConfiguration::parse_with_options(record, options)
                    .map(Self::ExdDataConceptConfiguration)
            }
            DiveSummary::KIND => {
                DiveSummary::parse_with_options(record, options).map(Self::DiveSummary)
            }
            AadAccelFeatures::KIND => {
                AadAccelFeatures::parse_with_options(record, options).map(Self::AadAccelFeatures)
            }
            Hrv::KIND => Hrv::parse_with_options(record, options).map(Self::Hrv),
            BeatIntervals::KIND => {
                BeatIntervals::parse_with_options(record, options).map(Self::BeatIntervals)
            }
            HrvStatusSummary::KIND => {
                HrvStatusSummary::parse_with_options(record, options).map(Self::HrvStatusSummary)
            }
            HrvValue::KIND => HrvValue::parse_with_options(record, options).map(Self::HrvValue),
            RawBbi::KIND => RawBbi::parse_with_options(record, options).map(Self::RawBbi),
            RespirationRate::KIND => {
                RespirationRate::parse_with_options(record, options).map(Self::RespirationRate)
            }
            ChronoShotSession::KIND => {
                ChronoShotSession::parse_with_options(record, options).map(Self::ChronoShotSession)
            }
            ChronoShotData::KIND => {
                ChronoShotData::parse_with_options(record, options).map(Self::ChronoShotData)
            }
            TankUpdate::KIND => {
                TankUpdate::parse_with_options(record, options).map(Self::TankUpdate)
            }
            TankSummary::KIND => {
                TankSummary::parse_with_options(record, options).map(Self::TankSummary)
            }
            SleepAssessment::KIND => {
                SleepAssessment::parse_with_options(record, options).map(Self::SleepAssessment)
            }
            kind => Err(TryFromRecordError::UnsupportedMessageKind(kind)),
        }
    }
    #[doc = r" Return all invalid fields in this message."]
    pub fn invalid_fields(&self) -> &BTreeMap<&'static str, ValueWithUnits> {
        match self {
            Self::FileId(message) => message.invalid_fields(),
            Self::FileCreator(message) => message.invalid_fields(),
            Self::TimestampCorrelation(message) => message.invalid_fields(),
            Self::Software(message) => message.invalid_fields(),
            Self::SlaveDevice(message) => message.invalid_fields(),
            Self::Capabilities(message) => message.invalid_fields(),
            Self::FileCapabilities(message) => message.invalid_fields(),
            Self::MesgCapabilities(message) => message.invalid_fields(),
            Self::FieldCapabilities(message) => message.invalid_fields(),
            Self::DeviceSettings(message) => message.invalid_fields(),
            Self::UserProfile(message) => message.invalid_fields(),
            Self::HrmProfile(message) => message.invalid_fields(),
            Self::SdmProfile(message) => message.invalid_fields(),
            Self::BikeProfile(message) => message.invalid_fields(),
            Self::Connectivity(message) => message.invalid_fields(),
            Self::WatchfaceSettings(message) => message.invalid_fields(),
            Self::OhrSettings(message) => message.invalid_fields(),
            Self::TimeInZone(message) => message.invalid_fields(),
            Self::ZonesTarget(message) => message.invalid_fields(),
            Self::Sport(message) => message.invalid_fields(),
            Self::HrZone(message) => message.invalid_fields(),
            Self::SpeedZone(message) => message.invalid_fields(),
            Self::CadenceZone(message) => message.invalid_fields(),
            Self::PowerZone(message) => message.invalid_fields(),
            Self::MetZone(message) => message.invalid_fields(),
            Self::DiveSettings(message) => message.invalid_fields(),
            Self::DiveAlarm(message) => message.invalid_fields(),
            Self::DiveApneaAlarm(message) => message.invalid_fields(),
            Self::DiveGas(message) => message.invalid_fields(),
            Self::Goal(message) => message.invalid_fields(),
            Self::Activity(message) => message.invalid_fields(),
            Self::Session(message) => message.invalid_fields(),
            Self::Lap(message) => message.invalid_fields(),
            Self::Length(message) => message.invalid_fields(),
            Self::Record(message) => message.invalid_fields(),
            Self::Event(message) => message.invalid_fields(),
            Self::DeviceInfo(message) => message.invalid_fields(),
            Self::DeviceAuxBatteryInfo(message) => message.invalid_fields(),
            Self::TrainingFile(message) => message.invalid_fields(),
            Self::WeatherConditions(message) => message.invalid_fields(),
            Self::WeatherAlert(message) => message.invalid_fields(),
            Self::GpsMetadata(message) => message.invalid_fields(),
            Self::CameraEvent(message) => message.invalid_fields(),
            Self::GyroscopeData(message) => message.invalid_fields(),
            Self::AccelerometerData(message) => message.invalid_fields(),
            Self::MagnetometerData(message) => message.invalid_fields(),
            Self::BarometerData(message) => message.invalid_fields(),
            Self::ThreeDSensorCalibration(message) => message.invalid_fields(),
            Self::OneDSensorCalibration(message) => message.invalid_fields(),
            Self::VideoFrame(message) => message.invalid_fields(),
            Self::ObdiiData(message) => message.invalid_fields(),
            Self::NmeaSentence(message) => message.invalid_fields(),
            Self::AviationAttitude(message) => message.invalid_fields(),
            Self::Video(message) => message.invalid_fields(),
            Self::VideoTitle(message) => message.invalid_fields(),
            Self::VideoDescription(message) => message.invalid_fields(),
            Self::VideoClip(message) => message.invalid_fields(),
            Self::Set(message) => message.invalid_fields(),
            Self::Jump(message) => message.invalid_fields(),
            Self::Split(message) => message.invalid_fields(),
            Self::SplitSummary(message) => message.invalid_fields(),
            Self::ClimbPro(message) => message.invalid_fields(),
            Self::FieldDescription(message) => message.invalid_fields(),
            Self::DeveloperDataId(message) => message.invalid_fields(),
            Self::Course(message) => message.invalid_fields(),
            Self::CoursePoint(message) => message.invalid_fields(),
            Self::SegmentId(message) => message.invalid_fields(),
            Self::SegmentLeaderboardEntry(message) => message.invalid_fields(),
            Self::SegmentPoint(message) => message.invalid_fields(),
            Self::SegmentLap(message) => message.invalid_fields(),
            Self::SegmentFile(message) => message.invalid_fields(),
            Self::Workout(message) => message.invalid_fields(),
            Self::WorkoutSession(message) => message.invalid_fields(),
            Self::WorkoutStep(message) => message.invalid_fields(),
            Self::ExerciseTitle(message) => message.invalid_fields(),
            Self::Schedule(message) => message.invalid_fields(),
            Self::Totals(message) => message.invalid_fields(),
            Self::WeightScale(message) => message.invalid_fields(),
            Self::BloodPressure(message) => message.invalid_fields(),
            Self::MonitoringInfo(message) => message.invalid_fields(),
            Self::Monitoring(message) => message.invalid_fields(),
            Self::MonitoringHrData(message) => message.invalid_fields(),
            Self::Spo2Data(message) => message.invalid_fields(),
            Self::Hr(message) => message.invalid_fields(),
            Self::StressLevel(message) => message.invalid_fields(),
            Self::MaxMetData(message) => message.invalid_fields(),
            Self::HsaBodyBatteryData(message) => message.invalid_fields(),
            Self::HsaEvent(message) => message.invalid_fields(),
            Self::HsaAccelerometerData(message) => message.invalid_fields(),
            Self::HsaGyroscopeData(message) => message.invalid_fields(),
            Self::HsaStepData(message) => message.invalid_fields(),
            Self::HsaSpo2Data(message) => message.invalid_fields(),
            Self::HsaStressData(message) => message.invalid_fields(),
            Self::HsaRespirationData(message) => message.invalid_fields(),
            Self::HsaHeartRateData(message) => message.invalid_fields(),
            Self::HsaConfigurationData(message) => message.invalid_fields(),
            Self::HsaWristTemperatureData(message) => message.invalid_fields(),
            Self::MemoGlob(message) => message.invalid_fields(),
            Self::SleepLevel(message) => message.invalid_fields(),
            Self::AntChannelId(message) => message.invalid_fields(),
            Self::AntRx(message) => message.invalid_fields(),
            Self::AntTx(message) => message.invalid_fields(),
            Self::ExdScreenConfiguration(message) => message.invalid_fields(),
            Self::ExdDataFieldConfiguration(message) => message.invalid_fields(),
            Self::ExdDataConceptConfiguration(message) => message.invalid_fields(),
            Self::DiveSummary(message) => message.invalid_fields(),
            Self::AadAccelFeatures(message) => message.invalid_fields(),
            Self::Hrv(message) => message.invalid_fields(),
            Self::BeatIntervals(message) => message.invalid_fields(),
            Self::HrvStatusSummary(message) => message.invalid_fields(),
            Self::HrvValue(message) => message.invalid_fields(),
            Self::RawBbi(message) => message.invalid_fields(),
            Self::RespirationRate(message) => message.invalid_fields(),
            Self::ChronoShotSession(message) => message.invalid_fields(),
            Self::ChronoShotData(message) => message.invalid_fields(),
            Self::TankUpdate(message) => message.invalid_fields(),
            Self::TankSummary(message) => message.invalid_fields(),
            Self::SleepAssessment(message) => message.invalid_fields(),
        }
    }
}
#[doc = "Must be first message in file."]
#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize)]
pub struct FileId {
    pub r#type: Option<field_types::File>,
    pub r#manufacturer: Option<field_types::Manufacturer>,
    pub r#product: Option<ValueWithUnits>,
    pub r#serial_number: Option<ValueWithUnits>,
    pub r#time_created: Option<field_types::DateTime>,
    pub r#number: Option<ValueWithUnits>,
    pub r#product_name: Option<ValueWithUnits>,
    pub invalid_fields: BTreeMap<&'static str, ValueWithUnits>,
}
impl FitMessage for FileId {
    const NAME: &'static str = "FileId";
    const KIND: MesgNum = MesgNum::FileId;
    fn parse_with_options(
        record: FitDataRecord,
        options: MessageParseOptions,
    ) -> Result<Self, TryFromRecordError> {
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
        #[allow(unused_mut)]
        let mut invalid_fields = BTreeMap::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#type = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("type", value);
                    }
                },
                1u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#manufacturer = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("manufacturer", value);
                    }
                },
                2u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#product = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("product", value);
                    }
                },
                3u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#serial_number = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("serial_number", value);
                    }
                },
                4u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#time_created = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("time_created", value);
                    }
                },
                5u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#number = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("number", value);
                    }
                },
                8u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#product_name = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("product_name", value);
                    }
                },
                _ => {
                    if !options.ignore_unexpected_fields {
                        return Err(TryFromRecordError::unexpected_field(&field));
                    }
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
            invalid_fields,
        })
    }
    fn invalid_fields(&self) -> &BTreeMap<&'static str, ValueWithUnits> {
        &self.invalid_fields
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
    pub r#software_version: Option<ValueWithUnits>,
    pub r#hardware_version: Option<ValueWithUnits>,
    pub invalid_fields: BTreeMap<&'static str, ValueWithUnits>,
}
impl FitMessage for FileCreator {
    const NAME: &'static str = "FileCreator";
    const KIND: MesgNum = MesgNum::FileCreator;
    fn parse_with_options(
        record: FitDataRecord,
        options: MessageParseOptions,
    ) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#software_version = None;
        let mut r#hardware_version = None;
        #[allow(unused_mut)]
        let mut invalid_fields = BTreeMap::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#software_version = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("software_version", value);
                    }
                },
                1u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#hardware_version = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("hardware_version", value);
                    }
                },
                _ => {
                    if !options.ignore_unexpected_fields {
                        return Err(TryFromRecordError::unexpected_field(&field));
                    }
                }
            }
        }
        Ok(Self {
            r#software_version,
            r#hardware_version,
            invalid_fields,
        })
    }
    fn invalid_fields(&self) -> &BTreeMap<&'static str, ValueWithUnits> {
        &self.invalid_fields
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
    pub r#fractional_timestamp: Option<ValueWithUnits>,
    pub r#system_timestamp: Option<field_types::DateTime>,
    pub r#fractional_system_timestamp: Option<ValueWithUnits>,
    pub r#local_timestamp: Option<field_types::LocalDateTime>,
    pub r#timestamp_ms: Option<ValueWithUnits>,
    pub r#system_timestamp_ms: Option<ValueWithUnits>,
    pub r#timestamp: Option<field_types::DateTime>,
    pub invalid_fields: BTreeMap<&'static str, ValueWithUnits>,
}
impl FitMessage for TimestampCorrelation {
    const NAME: &'static str = "TimestampCorrelation";
    const KIND: MesgNum = MesgNum::TimestampCorrelation;
    fn parse_with_options(
        record: FitDataRecord,
        options: MessageParseOptions,
    ) -> Result<Self, TryFromRecordError> {
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
        #[allow(unused_mut)]
        let mut invalid_fields = BTreeMap::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#fractional_timestamp = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("fractional_timestamp", value);
                    }
                },
                1u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#system_timestamp = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("system_timestamp", value);
                    }
                },
                2u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#fractional_system_timestamp = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("fractional_system_timestamp", value);
                    }
                },
                3u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#local_timestamp = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("local_timestamp", value);
                    }
                },
                4u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#timestamp_ms = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("timestamp_ms", value);
                    }
                },
                5u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#system_timestamp_ms = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("system_timestamp_ms", value);
                    }
                },
                253u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#timestamp = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("timestamp", value);
                    }
                },
                _ => {
                    if !options.ignore_unexpected_fields {
                        return Err(TryFromRecordError::unexpected_field(&field));
                    }
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
            invalid_fields,
        })
    }
    fn invalid_fields(&self) -> &BTreeMap<&'static str, ValueWithUnits> {
        &self.invalid_fields
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
    pub r#version: Option<ValueWithUnits>,
    pub r#part_number: Option<ValueWithUnits>,
    pub r#message_index: Option<field_types::MessageIndex>,
    pub invalid_fields: BTreeMap<&'static str, ValueWithUnits>,
}
impl FitMessage for Software {
    const NAME: &'static str = "Software";
    const KIND: MesgNum = MesgNum::Software;
    fn parse_with_options(
        record: FitDataRecord,
        options: MessageParseOptions,
    ) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#version = None;
        let mut r#part_number = None;
        let mut r#message_index = None;
        #[allow(unused_mut)]
        let mut invalid_fields = BTreeMap::new();
        for field in record.into_vec() {
            match field.number() {
                3u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#version = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("version", value);
                    }
                },
                5u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#part_number = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("part_number", value);
                    }
                },
                254u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#message_index = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("message_index", value);
                    }
                },
                _ => {
                    if !options.ignore_unexpected_fields {
                        return Err(TryFromRecordError::unexpected_field(&field));
                    }
                }
            }
        }
        Ok(Self {
            r#version,
            r#part_number,
            r#message_index,
            invalid_fields,
        })
    }
    fn invalid_fields(&self) -> &BTreeMap<&'static str, ValueWithUnits> {
        &self.invalid_fields
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
    pub r#manufacturer: Option<field_types::Manufacturer>,
    pub r#product: Option<ValueWithUnits>,
    pub invalid_fields: BTreeMap<&'static str, ValueWithUnits>,
}
impl FitMessage for SlaveDevice {
    const NAME: &'static str = "SlaveDevice";
    const KIND: MesgNum = MesgNum::SlaveDevice;
    fn parse_with_options(
        record: FitDataRecord,
        options: MessageParseOptions,
    ) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#manufacturer = None;
        let mut r#product = None;
        #[allow(unused_mut)]
        let mut invalid_fields = BTreeMap::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#manufacturer = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("manufacturer", value);
                    }
                },
                1u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#product = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("product", value);
                    }
                },
                _ => {
                    if !options.ignore_unexpected_fields {
                        return Err(TryFromRecordError::unexpected_field(&field));
                    }
                }
            }
        }
        Ok(Self {
            r#manufacturer,
            r#product,
            invalid_fields,
        })
    }
    fn invalid_fields(&self) -> &BTreeMap<&'static str, ValueWithUnits> {
        &self.invalid_fields
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
    pub r#languages: Option<ValueWithUnits>,
    pub r#sports: Option<ValueWithUnits>,
    pub r#workouts_supported: Option<field_types::WorkoutCapabilities>,
    pub r#connectivity_supported: Option<field_types::ConnectivityCapabilities>,
    pub invalid_fields: BTreeMap<&'static str, ValueWithUnits>,
}
impl FitMessage for Capabilities {
    const NAME: &'static str = "Capabilities";
    const KIND: MesgNum = MesgNum::Capabilities;
    fn parse_with_options(
        record: FitDataRecord,
        options: MessageParseOptions,
    ) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#languages = None;
        let mut r#sports = None;
        let mut r#workouts_supported = None;
        let mut r#connectivity_supported = None;
        #[allow(unused_mut)]
        let mut invalid_fields = BTreeMap::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#languages = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("languages", value);
                    }
                },
                1u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#sports = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("sports", value);
                    }
                },
                21u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#workouts_supported = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("workouts_supported", value);
                    }
                },
                23u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#connectivity_supported = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("connectivity_supported", value);
                    }
                },
                _ => {
                    if !options.ignore_unexpected_fields {
                        return Err(TryFromRecordError::unexpected_field(&field));
                    }
                }
            }
        }
        Ok(Self {
            r#languages,
            r#sports,
            r#workouts_supported,
            r#connectivity_supported,
            invalid_fields,
        })
    }
    fn invalid_fields(&self) -> &BTreeMap<&'static str, ValueWithUnits> {
        &self.invalid_fields
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
    pub r#type: Option<field_types::File>,
    pub r#flags: Option<field_types::FileFlags>,
    pub r#directory: Option<ValueWithUnits>,
    pub r#max_count: Option<ValueWithUnits>,
    pub r#max_size: Option<ValueWithUnits>,
    pub r#message_index: Option<field_types::MessageIndex>,
    pub invalid_fields: BTreeMap<&'static str, ValueWithUnits>,
}
impl FitMessage for FileCapabilities {
    const NAME: &'static str = "FileCapabilities";
    const KIND: MesgNum = MesgNum::FileCapabilities;
    fn parse_with_options(
        record: FitDataRecord,
        options: MessageParseOptions,
    ) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#type = None;
        let mut r#flags = None;
        let mut r#directory = None;
        let mut r#max_count = None;
        let mut r#max_size = None;
        let mut r#message_index = None;
        #[allow(unused_mut)]
        let mut invalid_fields = BTreeMap::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#type = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("type", value);
                    }
                },
                1u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#flags = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("flags", value);
                    }
                },
                2u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#directory = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("directory", value);
                    }
                },
                3u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#max_count = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("max_count", value);
                    }
                },
                4u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#max_size = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("max_size", value);
                    }
                },
                254u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#message_index = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("message_index", value);
                    }
                },
                _ => {
                    if !options.ignore_unexpected_fields {
                        return Err(TryFromRecordError::unexpected_field(&field));
                    }
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
            invalid_fields,
        })
    }
    fn invalid_fields(&self) -> &BTreeMap<&'static str, ValueWithUnits> {
        &self.invalid_fields
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
    pub r#file: Option<field_types::File>,
    pub r#mesg_num: Option<field_types::MesgNum>,
    pub r#count_type: Option<field_types::MesgCount>,
    pub r#count: Option<ValueWithUnits>,
    pub r#message_index: Option<field_types::MessageIndex>,
    pub invalid_fields: BTreeMap<&'static str, ValueWithUnits>,
}
impl FitMessage for MesgCapabilities {
    const NAME: &'static str = "MesgCapabilities";
    const KIND: MesgNum = MesgNum::MesgCapabilities;
    fn parse_with_options(
        record: FitDataRecord,
        options: MessageParseOptions,
    ) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#file = None;
        let mut r#mesg_num = None;
        let mut r#count_type = None;
        let mut r#count = None;
        let mut r#message_index = None;
        #[allow(unused_mut)]
        let mut invalid_fields = BTreeMap::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#file = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("file", value);
                    }
                },
                1u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#mesg_num = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("mesg_num", value);
                    }
                },
                2u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#count_type = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("count_type", value);
                    }
                },
                3u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#count = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("count", value);
                    }
                },
                254u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#message_index = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("message_index", value);
                    }
                },
                _ => {
                    if !options.ignore_unexpected_fields {
                        return Err(TryFromRecordError::unexpected_field(&field));
                    }
                }
            }
        }
        Ok(Self {
            r#file,
            r#mesg_num,
            r#count_type,
            r#count,
            r#message_index,
            invalid_fields,
        })
    }
    fn invalid_fields(&self) -> &BTreeMap<&'static str, ValueWithUnits> {
        &self.invalid_fields
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
    pub r#file: Option<field_types::File>,
    pub r#mesg_num: Option<field_types::MesgNum>,
    pub r#field_num: Option<ValueWithUnits>,
    pub r#count: Option<ValueWithUnits>,
    pub r#message_index: Option<field_types::MessageIndex>,
    pub invalid_fields: BTreeMap<&'static str, ValueWithUnits>,
}
impl FitMessage for FieldCapabilities {
    const NAME: &'static str = "FieldCapabilities";
    const KIND: MesgNum = MesgNum::FieldCapabilities;
    fn parse_with_options(
        record: FitDataRecord,
        options: MessageParseOptions,
    ) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#file = None;
        let mut r#mesg_num = None;
        let mut r#field_num = None;
        let mut r#count = None;
        let mut r#message_index = None;
        #[allow(unused_mut)]
        let mut invalid_fields = BTreeMap::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#file = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("file", value);
                    }
                },
                1u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#mesg_num = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("mesg_num", value);
                    }
                },
                2u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#field_num = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("field_num", value);
                    }
                },
                3u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#count = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("count", value);
                    }
                },
                254u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#message_index = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("message_index", value);
                    }
                },
                _ => {
                    if !options.ignore_unexpected_fields {
                        return Err(TryFromRecordError::unexpected_field(&field));
                    }
                }
            }
        }
        Ok(Self {
            r#file,
            r#mesg_num,
            r#field_num,
            r#count,
            r#message_index,
            invalid_fields,
        })
    }
    fn invalid_fields(&self) -> &BTreeMap<&'static str, ValueWithUnits> {
        &self.invalid_fields
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
    pub r#active_time_zone: Option<ValueWithUnits>,
    pub r#utc_offset: Option<ValueWithUnits>,
    pub r#time_offset: Option<ValueWithUnits>,
    pub r#time_mode: Option<ValueWithUnits>,
    pub r#time_zone_offset: Option<ValueWithUnits>,
    pub r#backlight_mode: Option<field_types::BacklightMode>,
    pub r#activity_tracker_enabled: Option<ValueWithUnits>,
    pub r#clock_time: Option<field_types::DateTime>,
    pub r#pages_enabled: Option<ValueWithUnits>,
    pub r#move_alert_enabled: Option<ValueWithUnits>,
    pub r#date_mode: Option<field_types::DateMode>,
    pub r#display_orientation: Option<field_types::DisplayOrientation>,
    pub r#mounting_side: Option<field_types::Side>,
    pub r#default_page: Option<ValueWithUnits>,
    pub r#autosync_min_steps: Option<ValueWithUnits>,
    pub r#autosync_min_time: Option<ValueWithUnits>,
    pub r#lactate_threshold_autodetect_enabled: Option<ValueWithUnits>,
    pub r#ble_auto_upload_enabled: Option<ValueWithUnits>,
    pub r#auto_sync_frequency: Option<field_types::AutoSyncFrequency>,
    pub r#auto_activity_detect: Option<field_types::AutoActivityDetect>,
    pub r#number_of_screens: Option<ValueWithUnits>,
    pub r#smart_notification_display_orientation: Option<field_types::DisplayOrientation>,
    pub r#tap_interface: Option<field_types::Switch>,
    pub r#tap_sensitivity: Option<field_types::TapSensitivity>,
    pub invalid_fields: BTreeMap<&'static str, ValueWithUnits>,
}
impl FitMessage for DeviceSettings {
    const NAME: &'static str = "DeviceSettings";
    const KIND: MesgNum = MesgNum::DeviceSettings;
    fn parse_with_options(
        record: FitDataRecord,
        options: MessageParseOptions,
    ) -> Result<Self, TryFromRecordError> {
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
        #[allow(unused_mut)]
        let mut invalid_fields = BTreeMap::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#active_time_zone = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("active_time_zone", value);
                    }
                },
                1u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#utc_offset = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("utc_offset", value);
                    }
                },
                2u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#time_offset = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("time_offset", value);
                    }
                },
                4u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#time_mode = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("time_mode", value);
                    }
                },
                5u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#time_zone_offset = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("time_zone_offset", value);
                    }
                },
                12u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#backlight_mode = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("backlight_mode", value);
                    }
                },
                36u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#activity_tracker_enabled = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("activity_tracker_enabled", value);
                    }
                },
                39u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#clock_time = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("clock_time", value);
                    }
                },
                40u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#pages_enabled = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("pages_enabled", value);
                    }
                },
                46u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#move_alert_enabled = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("move_alert_enabled", value);
                    }
                },
                47u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#date_mode = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("date_mode", value);
                    }
                },
                55u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#display_orientation = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("display_orientation", value);
                    }
                },
                56u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#mounting_side = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("mounting_side", value);
                    }
                },
                57u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#default_page = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("default_page", value);
                    }
                },
                58u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#autosync_min_steps = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("autosync_min_steps", value);
                    }
                },
                59u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#autosync_min_time = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("autosync_min_time", value);
                    }
                },
                80u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#lactate_threshold_autodetect_enabled = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("lactate_threshold_autodetect_enabled", value);
                    }
                },
                86u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#ble_auto_upload_enabled = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("ble_auto_upload_enabled", value);
                    }
                },
                89u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#auto_sync_frequency = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("auto_sync_frequency", value);
                    }
                },
                90u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#auto_activity_detect = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("auto_activity_detect", value);
                    }
                },
                94u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#number_of_screens = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("number_of_screens", value);
                    }
                },
                95u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#smart_notification_display_orientation = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("smart_notification_display_orientation", value);
                    }
                },
                134u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#tap_interface = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("tap_interface", value);
                    }
                },
                174u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#tap_sensitivity = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("tap_sensitivity", value);
                    }
                },
                _ => {
                    if !options.ignore_unexpected_fields {
                        return Err(TryFromRecordError::unexpected_field(&field));
                    }
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
            invalid_fields,
        })
    }
    fn invalid_fields(&self) -> &BTreeMap<&'static str, ValueWithUnits> {
        &self.invalid_fields
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
    pub r#friendly_name: Option<ValueWithUnits>,
    pub r#gender: Option<field_types::Gender>,
    pub r#age: Option<ValueWithUnits>,
    pub r#height: Option<ValueWithUnits>,
    pub r#weight: Option<ValueWithUnits>,
    pub r#language: Option<field_types::Language>,
    pub r#elev_setting: Option<field_types::DisplayMeasure>,
    pub r#weight_setting: Option<field_types::DisplayMeasure>,
    pub r#resting_heart_rate: Option<ValueWithUnits>,
    pub r#default_max_running_heart_rate: Option<ValueWithUnits>,
    pub r#default_max_biking_heart_rate: Option<ValueWithUnits>,
    pub r#default_max_heart_rate: Option<ValueWithUnits>,
    pub r#hr_setting: Option<field_types::DisplayHeart>,
    pub r#speed_setting: Option<field_types::DisplayMeasure>,
    pub r#dist_setting: Option<field_types::DisplayMeasure>,
    pub r#power_setting: Option<field_types::DisplayPower>,
    pub r#activity_class: Option<field_types::ActivityClass>,
    pub r#position_setting: Option<field_types::DisplayPosition>,
    pub r#temperature_setting: Option<field_types::DisplayMeasure>,
    pub r#local_id: Option<field_types::UserLocalId>,
    pub r#global_id: Option<ValueWithUnits>,
    pub r#wake_time: Option<ValueWithUnits>,
    pub r#sleep_time: Option<ValueWithUnits>,
    pub r#height_setting: Option<field_types::DisplayMeasure>,
    pub r#user_running_step_length: Option<ValueWithUnits>,
    pub r#user_walking_step_length: Option<ValueWithUnits>,
    pub r#depth_setting: Option<field_types::DisplayMeasure>,
    pub r#dive_count: Option<ValueWithUnits>,
    pub r#message_index: Option<field_types::MessageIndex>,
    pub invalid_fields: BTreeMap<&'static str, ValueWithUnits>,
}
impl FitMessage for UserProfile {
    const NAME: &'static str = "UserProfile";
    const KIND: MesgNum = MesgNum::UserProfile;
    fn parse_with_options(
        record: FitDataRecord,
        options: MessageParseOptions,
    ) -> Result<Self, TryFromRecordError> {
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
        #[allow(unused_mut)]
        let mut invalid_fields = BTreeMap::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#friendly_name = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("friendly_name", value);
                    }
                },
                1u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#gender = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("gender", value);
                    }
                },
                2u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#age = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("age", value);
                    }
                },
                3u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#height = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("height", value);
                    }
                },
                4u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#weight = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("weight", value);
                    }
                },
                5u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#language = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("language", value);
                    }
                },
                6u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#elev_setting = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("elev_setting", value);
                    }
                },
                7u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#weight_setting = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("weight_setting", value);
                    }
                },
                8u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#resting_heart_rate = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("resting_heart_rate", value);
                    }
                },
                9u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#default_max_running_heart_rate = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("default_max_running_heart_rate", value);
                    }
                },
                10u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#default_max_biking_heart_rate = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("default_max_biking_heart_rate", value);
                    }
                },
                11u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#default_max_heart_rate = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("default_max_heart_rate", value);
                    }
                },
                12u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#hr_setting = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("hr_setting", value);
                    }
                },
                13u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#speed_setting = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("speed_setting", value);
                    }
                },
                14u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#dist_setting = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("dist_setting", value);
                    }
                },
                16u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#power_setting = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("power_setting", value);
                    }
                },
                17u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#activity_class = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("activity_class", value);
                    }
                },
                18u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#position_setting = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("position_setting", value);
                    }
                },
                21u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#temperature_setting = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("temperature_setting", value);
                    }
                },
                22u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#local_id = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("local_id", value);
                    }
                },
                23u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#global_id = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("global_id", value);
                    }
                },
                28u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#wake_time = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("wake_time", value);
                    }
                },
                29u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#sleep_time = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("sleep_time", value);
                    }
                },
                30u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#height_setting = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("height_setting", value);
                    }
                },
                31u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#user_running_step_length = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("user_running_step_length", value);
                    }
                },
                32u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#user_walking_step_length = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("user_walking_step_length", value);
                    }
                },
                47u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#depth_setting = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("depth_setting", value);
                    }
                },
                49u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#dive_count = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("dive_count", value);
                    }
                },
                254u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#message_index = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("message_index", value);
                    }
                },
                _ => {
                    if !options.ignore_unexpected_fields {
                        return Err(TryFromRecordError::unexpected_field(&field));
                    }
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
            invalid_fields,
        })
    }
    fn invalid_fields(&self) -> &BTreeMap<&'static str, ValueWithUnits> {
        &self.invalid_fields
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
    pub r#enabled: Option<ValueWithUnits>,
    pub r#hrm_ant_id: Option<ValueWithUnits>,
    pub r#log_hrv: Option<ValueWithUnits>,
    pub r#hrm_ant_id_trans_type: Option<ValueWithUnits>,
    pub r#message_index: Option<field_types::MessageIndex>,
    pub invalid_fields: BTreeMap<&'static str, ValueWithUnits>,
}
impl FitMessage for HrmProfile {
    const NAME: &'static str = "HrmProfile";
    const KIND: MesgNum = MesgNum::HrmProfile;
    fn parse_with_options(
        record: FitDataRecord,
        options: MessageParseOptions,
    ) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#enabled = None;
        let mut r#hrm_ant_id = None;
        let mut r#log_hrv = None;
        let mut r#hrm_ant_id_trans_type = None;
        let mut r#message_index = None;
        #[allow(unused_mut)]
        let mut invalid_fields = BTreeMap::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#enabled = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("enabled", value);
                    }
                },
                1u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#hrm_ant_id = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("hrm_ant_id", value);
                    }
                },
                2u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#log_hrv = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("log_hrv", value);
                    }
                },
                3u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#hrm_ant_id_trans_type = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("hrm_ant_id_trans_type", value);
                    }
                },
                254u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#message_index = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("message_index", value);
                    }
                },
                _ => {
                    if !options.ignore_unexpected_fields {
                        return Err(TryFromRecordError::unexpected_field(&field));
                    }
                }
            }
        }
        Ok(Self {
            r#enabled,
            r#hrm_ant_id,
            r#log_hrv,
            r#hrm_ant_id_trans_type,
            r#message_index,
            invalid_fields,
        })
    }
    fn invalid_fields(&self) -> &BTreeMap<&'static str, ValueWithUnits> {
        &self.invalid_fields
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
    pub r#enabled: Option<ValueWithUnits>,
    pub r#sdm_ant_id: Option<ValueWithUnits>,
    pub r#sdm_cal_factor: Option<ValueWithUnits>,
    pub r#odometer: Option<ValueWithUnits>,
    pub r#speed_source: Option<ValueWithUnits>,
    pub r#sdm_ant_id_trans_type: Option<ValueWithUnits>,
    pub r#odometer_rollover: Option<ValueWithUnits>,
    pub r#message_index: Option<field_types::MessageIndex>,
    pub invalid_fields: BTreeMap<&'static str, ValueWithUnits>,
}
impl FitMessage for SdmProfile {
    const NAME: &'static str = "SdmProfile";
    const KIND: MesgNum = MesgNum::SdmProfile;
    fn parse_with_options(
        record: FitDataRecord,
        options: MessageParseOptions,
    ) -> Result<Self, TryFromRecordError> {
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
        #[allow(unused_mut)]
        let mut invalid_fields = BTreeMap::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#enabled = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("enabled", value);
                    }
                },
                1u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#sdm_ant_id = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("sdm_ant_id", value);
                    }
                },
                2u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#sdm_cal_factor = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("sdm_cal_factor", value);
                    }
                },
                3u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#odometer = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("odometer", value);
                    }
                },
                4u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#speed_source = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("speed_source", value);
                    }
                },
                5u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#sdm_ant_id_trans_type = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("sdm_ant_id_trans_type", value);
                    }
                },
                7u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#odometer_rollover = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("odometer_rollover", value);
                    }
                },
                254u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#message_index = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("message_index", value);
                    }
                },
                _ => {
                    if !options.ignore_unexpected_fields {
                        return Err(TryFromRecordError::unexpected_field(&field));
                    }
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
            invalid_fields,
        })
    }
    fn invalid_fields(&self) -> &BTreeMap<&'static str, ValueWithUnits> {
        &self.invalid_fields
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
    pub r#name: Option<ValueWithUnits>,
    pub r#sport: Option<field_types::Sport>,
    pub r#sub_sport: Option<field_types::SubSport>,
    pub r#odometer: Option<ValueWithUnits>,
    pub r#bike_spd_ant_id: Option<ValueWithUnits>,
    pub r#bike_cad_ant_id: Option<ValueWithUnits>,
    pub r#bike_spdcad_ant_id: Option<ValueWithUnits>,
    pub r#bike_power_ant_id: Option<ValueWithUnits>,
    pub r#custom_wheelsize: Option<ValueWithUnits>,
    pub r#auto_wheelsize: Option<ValueWithUnits>,
    pub r#bike_weight: Option<ValueWithUnits>,
    pub r#power_cal_factor: Option<ValueWithUnits>,
    pub r#auto_wheel_cal: Option<ValueWithUnits>,
    pub r#auto_power_zero: Option<ValueWithUnits>,
    pub r#id: Option<ValueWithUnits>,
    pub r#spd_enabled: Option<ValueWithUnits>,
    pub r#cad_enabled: Option<ValueWithUnits>,
    pub r#spdcad_enabled: Option<ValueWithUnits>,
    pub r#power_enabled: Option<ValueWithUnits>,
    pub r#crank_length: Option<ValueWithUnits>,
    pub r#enabled: Option<ValueWithUnits>,
    pub r#bike_spd_ant_id_trans_type: Option<ValueWithUnits>,
    pub r#bike_cad_ant_id_trans_type: Option<ValueWithUnits>,
    pub r#bike_spdcad_ant_id_trans_type: Option<ValueWithUnits>,
    pub r#bike_power_ant_id_trans_type: Option<ValueWithUnits>,
    pub r#odometer_rollover: Option<ValueWithUnits>,
    pub r#front_gear_num: Option<ValueWithUnits>,
    pub r#front_gear: Option<ValueWithUnits>,
    pub r#rear_gear_num: Option<ValueWithUnits>,
    pub r#rear_gear: Option<ValueWithUnits>,
    pub r#shimano_di2_enabled: Option<ValueWithUnits>,
    pub r#message_index: Option<field_types::MessageIndex>,
    pub invalid_fields: BTreeMap<&'static str, ValueWithUnits>,
}
impl FitMessage for BikeProfile {
    const NAME: &'static str = "BikeProfile";
    const KIND: MesgNum = MesgNum::BikeProfile;
    fn parse_with_options(
        record: FitDataRecord,
        options: MessageParseOptions,
    ) -> Result<Self, TryFromRecordError> {
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
        #[allow(unused_mut)]
        let mut invalid_fields = BTreeMap::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#name = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("name", value);
                    }
                },
                1u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#sport = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("sport", value);
                    }
                },
                2u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#sub_sport = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("sub_sport", value);
                    }
                },
                3u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#odometer = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("odometer", value);
                    }
                },
                4u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#bike_spd_ant_id = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("bike_spd_ant_id", value);
                    }
                },
                5u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#bike_cad_ant_id = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("bike_cad_ant_id", value);
                    }
                },
                6u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#bike_spdcad_ant_id = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("bike_spdcad_ant_id", value);
                    }
                },
                7u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#bike_power_ant_id = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("bike_power_ant_id", value);
                    }
                },
                8u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#custom_wheelsize = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("custom_wheelsize", value);
                    }
                },
                9u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#auto_wheelsize = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("auto_wheelsize", value);
                    }
                },
                10u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#bike_weight = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("bike_weight", value);
                    }
                },
                11u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#power_cal_factor = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("power_cal_factor", value);
                    }
                },
                12u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#auto_wheel_cal = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("auto_wheel_cal", value);
                    }
                },
                13u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#auto_power_zero = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("auto_power_zero", value);
                    }
                },
                14u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#id = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("id", value);
                    }
                },
                15u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#spd_enabled = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("spd_enabled", value);
                    }
                },
                16u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#cad_enabled = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("cad_enabled", value);
                    }
                },
                17u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#spdcad_enabled = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("spdcad_enabled", value);
                    }
                },
                18u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#power_enabled = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("power_enabled", value);
                    }
                },
                19u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#crank_length = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("crank_length", value);
                    }
                },
                20u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#enabled = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("enabled", value);
                    }
                },
                21u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#bike_spd_ant_id_trans_type = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("bike_spd_ant_id_trans_type", value);
                    }
                },
                22u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#bike_cad_ant_id_trans_type = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("bike_cad_ant_id_trans_type", value);
                    }
                },
                23u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#bike_spdcad_ant_id_trans_type = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("bike_spdcad_ant_id_trans_type", value);
                    }
                },
                24u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#bike_power_ant_id_trans_type = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("bike_power_ant_id_trans_type", value);
                    }
                },
                37u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#odometer_rollover = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("odometer_rollover", value);
                    }
                },
                38u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#front_gear_num = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("front_gear_num", value);
                    }
                },
                39u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#front_gear = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("front_gear", value);
                    }
                },
                40u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#rear_gear_num = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("rear_gear_num", value);
                    }
                },
                41u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#rear_gear = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("rear_gear", value);
                    }
                },
                44u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#shimano_di2_enabled = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("shimano_di2_enabled", value);
                    }
                },
                254u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#message_index = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("message_index", value);
                    }
                },
                _ => {
                    if !options.ignore_unexpected_fields {
                        return Err(TryFromRecordError::unexpected_field(&field));
                    }
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
            invalid_fields,
        })
    }
    fn invalid_fields(&self) -> &BTreeMap<&'static str, ValueWithUnits> {
        &self.invalid_fields
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
    pub r#bluetooth_enabled: Option<ValueWithUnits>,
    pub r#bluetooth_le_enabled: Option<ValueWithUnits>,
    pub r#ant_enabled: Option<ValueWithUnits>,
    pub r#name: Option<ValueWithUnits>,
    pub r#live_tracking_enabled: Option<ValueWithUnits>,
    pub r#weather_conditions_enabled: Option<ValueWithUnits>,
    pub r#weather_alerts_enabled: Option<ValueWithUnits>,
    pub r#auto_activity_upload_enabled: Option<ValueWithUnits>,
    pub r#course_download_enabled: Option<ValueWithUnits>,
    pub r#workout_download_enabled: Option<ValueWithUnits>,
    pub r#gps_ephemeris_download_enabled: Option<ValueWithUnits>,
    pub r#incident_detection_enabled: Option<ValueWithUnits>,
    pub r#grouptrack_enabled: Option<ValueWithUnits>,
    pub invalid_fields: BTreeMap<&'static str, ValueWithUnits>,
}
impl FitMessage for Connectivity {
    const NAME: &'static str = "Connectivity";
    const KIND: MesgNum = MesgNum::Connectivity;
    fn parse_with_options(
        record: FitDataRecord,
        options: MessageParseOptions,
    ) -> Result<Self, TryFromRecordError> {
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
        #[allow(unused_mut)]
        let mut invalid_fields = BTreeMap::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#bluetooth_enabled = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("bluetooth_enabled", value);
                    }
                },
                1u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#bluetooth_le_enabled = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("bluetooth_le_enabled", value);
                    }
                },
                2u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#ant_enabled = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("ant_enabled", value);
                    }
                },
                3u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#name = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("name", value);
                    }
                },
                4u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#live_tracking_enabled = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("live_tracking_enabled", value);
                    }
                },
                5u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#weather_conditions_enabled = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("weather_conditions_enabled", value);
                    }
                },
                6u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#weather_alerts_enabled = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("weather_alerts_enabled", value);
                    }
                },
                7u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#auto_activity_upload_enabled = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("auto_activity_upload_enabled", value);
                    }
                },
                8u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#course_download_enabled = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("course_download_enabled", value);
                    }
                },
                9u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#workout_download_enabled = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("workout_download_enabled", value);
                    }
                },
                10u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#gps_ephemeris_download_enabled = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("gps_ephemeris_download_enabled", value);
                    }
                },
                11u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#incident_detection_enabled = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("incident_detection_enabled", value);
                    }
                },
                12u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#grouptrack_enabled = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("grouptrack_enabled", value);
                    }
                },
                _ => {
                    if !options.ignore_unexpected_fields {
                        return Err(TryFromRecordError::unexpected_field(&field));
                    }
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
            invalid_fields,
        })
    }
    fn invalid_fields(&self) -> &BTreeMap<&'static str, ValueWithUnits> {
        &self.invalid_fields
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
    pub r#mode: Option<field_types::WatchfaceMode>,
    pub r#layout: Option<ValueWithUnits>,
    pub r#message_index: Option<field_types::MessageIndex>,
    pub invalid_fields: BTreeMap<&'static str, ValueWithUnits>,
}
impl FitMessage for WatchfaceSettings {
    const NAME: &'static str = "WatchfaceSettings";
    const KIND: MesgNum = MesgNum::WatchfaceSettings;
    fn parse_with_options(
        record: FitDataRecord,
        options: MessageParseOptions,
    ) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#mode = None;
        let mut r#layout = None;
        let mut r#message_index = None;
        #[allow(unused_mut)]
        let mut invalid_fields = BTreeMap::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#mode = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("mode", value);
                    }
                },
                1u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#layout = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("layout", value);
                    }
                },
                254u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#message_index = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("message_index", value);
                    }
                },
                _ => {
                    if !options.ignore_unexpected_fields {
                        return Err(TryFromRecordError::unexpected_field(&field));
                    }
                }
            }
        }
        Ok(Self {
            r#mode,
            r#layout,
            r#message_index,
            invalid_fields,
        })
    }
    fn invalid_fields(&self) -> &BTreeMap<&'static str, ValueWithUnits> {
        &self.invalid_fields
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
    pub r#enabled: Option<field_types::Switch>,
    pub r#timestamp: Option<field_types::DateTime>,
    pub invalid_fields: BTreeMap<&'static str, ValueWithUnits>,
}
impl FitMessage for OhrSettings {
    const NAME: &'static str = "OhrSettings";
    const KIND: MesgNum = MesgNum::OhrSettings;
    fn parse_with_options(
        record: FitDataRecord,
        options: MessageParseOptions,
    ) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#enabled = None;
        let mut r#timestamp = None;
        #[allow(unused_mut)]
        let mut invalid_fields = BTreeMap::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#enabled = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("enabled", value);
                    }
                },
                253u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#timestamp = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("timestamp", value);
                    }
                },
                _ => {
                    if !options.ignore_unexpected_fields {
                        return Err(TryFromRecordError::unexpected_field(&field));
                    }
                }
            }
        }
        Ok(Self {
            r#enabled,
            r#timestamp,
            invalid_fields,
        })
    }
    fn invalid_fields(&self) -> &BTreeMap<&'static str, ValueWithUnits> {
        &self.invalid_fields
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
    pub r#reference_mesg: Option<field_types::MesgNum>,
    pub r#reference_index: Option<field_types::MessageIndex>,
    pub r#time_in_hr_zone: Option<ValueWithUnits>,
    pub r#time_in_speed_zone: Option<ValueWithUnits>,
    pub r#time_in_cadence_zone: Option<ValueWithUnits>,
    pub r#time_in_power_zone: Option<ValueWithUnits>,
    pub r#hr_zone_high_boundary: Option<ValueWithUnits>,
    pub r#speed_zone_high_boundary: Option<ValueWithUnits>,
    pub r#cadence_zone_high_bondary: Option<ValueWithUnits>,
    pub r#power_zone_high_boundary: Option<ValueWithUnits>,
    pub r#hr_calc_type: Option<field_types::HrZoneCalc>,
    pub r#max_heart_rate: Option<ValueWithUnits>,
    pub r#resting_heart_rate: Option<ValueWithUnits>,
    pub r#threshold_heart_rate: Option<ValueWithUnits>,
    pub r#pwr_calc_type: Option<field_types::PwrZoneCalc>,
    pub r#functional_threshold_power: Option<ValueWithUnits>,
    pub r#timestamp: Option<field_types::DateTime>,
    pub invalid_fields: BTreeMap<&'static str, ValueWithUnits>,
}
impl FitMessage for TimeInZone {
    const NAME: &'static str = "TimeInZone";
    const KIND: MesgNum = MesgNum::TimeInZone;
    fn parse_with_options(
        record: FitDataRecord,
        options: MessageParseOptions,
    ) -> Result<Self, TryFromRecordError> {
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
        #[allow(unused_mut)]
        let mut invalid_fields = BTreeMap::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#reference_mesg = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("reference_mesg", value);
                    }
                },
                1u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#reference_index = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("reference_index", value);
                    }
                },
                2u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#time_in_hr_zone = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("time_in_hr_zone", value);
                    }
                },
                3u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#time_in_speed_zone = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("time_in_speed_zone", value);
                    }
                },
                4u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#time_in_cadence_zone = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("time_in_cadence_zone", value);
                    }
                },
                5u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#time_in_power_zone = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("time_in_power_zone", value);
                    }
                },
                6u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#hr_zone_high_boundary = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("hr_zone_high_boundary", value);
                    }
                },
                7u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#speed_zone_high_boundary = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("speed_zone_high_boundary", value);
                    }
                },
                8u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#cadence_zone_high_bondary = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("cadence_zone_high_bondary", value);
                    }
                },
                9u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#power_zone_high_boundary = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("power_zone_high_boundary", value);
                    }
                },
                10u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#hr_calc_type = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("hr_calc_type", value);
                    }
                },
                11u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#max_heart_rate = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("max_heart_rate", value);
                    }
                },
                12u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#resting_heart_rate = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("resting_heart_rate", value);
                    }
                },
                13u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#threshold_heart_rate = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("threshold_heart_rate", value);
                    }
                },
                14u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#pwr_calc_type = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("pwr_calc_type", value);
                    }
                },
                15u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#functional_threshold_power = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("functional_threshold_power", value);
                    }
                },
                253u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#timestamp = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("timestamp", value);
                    }
                },
                _ => {
                    if !options.ignore_unexpected_fields {
                        return Err(TryFromRecordError::unexpected_field(&field));
                    }
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
            invalid_fields,
        })
    }
    fn invalid_fields(&self) -> &BTreeMap<&'static str, ValueWithUnits> {
        &self.invalid_fields
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
    pub r#max_heart_rate: Option<ValueWithUnits>,
    pub r#threshold_heart_rate: Option<ValueWithUnits>,
    pub r#functional_threshold_power: Option<ValueWithUnits>,
    pub r#hr_calc_type: Option<field_types::HrZoneCalc>,
    pub r#pwr_calc_type: Option<field_types::PwrZoneCalc>,
    pub invalid_fields: BTreeMap<&'static str, ValueWithUnits>,
}
impl FitMessage for ZonesTarget {
    const NAME: &'static str = "ZonesTarget";
    const KIND: MesgNum = MesgNum::ZonesTarget;
    fn parse_with_options(
        record: FitDataRecord,
        options: MessageParseOptions,
    ) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#max_heart_rate = None;
        let mut r#threshold_heart_rate = None;
        let mut r#functional_threshold_power = None;
        let mut r#hr_calc_type = None;
        let mut r#pwr_calc_type = None;
        #[allow(unused_mut)]
        let mut invalid_fields = BTreeMap::new();
        for field in record.into_vec() {
            match field.number() {
                1u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#max_heart_rate = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("max_heart_rate", value);
                    }
                },
                2u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#threshold_heart_rate = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("threshold_heart_rate", value);
                    }
                },
                3u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#functional_threshold_power = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("functional_threshold_power", value);
                    }
                },
                5u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#hr_calc_type = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("hr_calc_type", value);
                    }
                },
                7u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#pwr_calc_type = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("pwr_calc_type", value);
                    }
                },
                _ => {
                    if !options.ignore_unexpected_fields {
                        return Err(TryFromRecordError::unexpected_field(&field));
                    }
                }
            }
        }
        Ok(Self {
            r#max_heart_rate,
            r#threshold_heart_rate,
            r#functional_threshold_power,
            r#hr_calc_type,
            r#pwr_calc_type,
            invalid_fields,
        })
    }
    fn invalid_fields(&self) -> &BTreeMap<&'static str, ValueWithUnits> {
        &self.invalid_fields
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
    pub r#sport: Option<field_types::Sport>,
    pub r#sub_sport: Option<field_types::SubSport>,
    pub r#name: Option<ValueWithUnits>,
    pub invalid_fields: BTreeMap<&'static str, ValueWithUnits>,
}
impl FitMessage for Sport {
    const NAME: &'static str = "Sport";
    const KIND: MesgNum = MesgNum::Sport;
    fn parse_with_options(
        record: FitDataRecord,
        options: MessageParseOptions,
    ) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#sport = None;
        let mut r#sub_sport = None;
        let mut r#name = None;
        #[allow(unused_mut)]
        let mut invalid_fields = BTreeMap::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#sport = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("sport", value);
                    }
                },
                1u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#sub_sport = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("sub_sport", value);
                    }
                },
                3u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#name = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("name", value);
                    }
                },
                _ => {
                    if !options.ignore_unexpected_fields {
                        return Err(TryFromRecordError::unexpected_field(&field));
                    }
                }
            }
        }
        Ok(Self {
            r#sport,
            r#sub_sport,
            r#name,
            invalid_fields,
        })
    }
    fn invalid_fields(&self) -> &BTreeMap<&'static str, ValueWithUnits> {
        &self.invalid_fields
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
    pub r#high_bpm: Option<ValueWithUnits>,
    pub r#name: Option<ValueWithUnits>,
    pub r#message_index: Option<field_types::MessageIndex>,
    pub invalid_fields: BTreeMap<&'static str, ValueWithUnits>,
}
impl FitMessage for HrZone {
    const NAME: &'static str = "HrZone";
    const KIND: MesgNum = MesgNum::HrZone;
    fn parse_with_options(
        record: FitDataRecord,
        options: MessageParseOptions,
    ) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#high_bpm = None;
        let mut r#name = None;
        let mut r#message_index = None;
        #[allow(unused_mut)]
        let mut invalid_fields = BTreeMap::new();
        for field in record.into_vec() {
            match field.number() {
                1u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#high_bpm = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("high_bpm", value);
                    }
                },
                2u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#name = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("name", value);
                    }
                },
                254u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#message_index = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("message_index", value);
                    }
                },
                _ => {
                    if !options.ignore_unexpected_fields {
                        return Err(TryFromRecordError::unexpected_field(&field));
                    }
                }
            }
        }
        Ok(Self {
            r#high_bpm,
            r#name,
            r#message_index,
            invalid_fields,
        })
    }
    fn invalid_fields(&self) -> &BTreeMap<&'static str, ValueWithUnits> {
        &self.invalid_fields
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
    pub r#high_value: Option<ValueWithUnits>,
    pub r#name: Option<ValueWithUnits>,
    pub r#message_index: Option<field_types::MessageIndex>,
    pub invalid_fields: BTreeMap<&'static str, ValueWithUnits>,
}
impl FitMessage for SpeedZone {
    const NAME: &'static str = "SpeedZone";
    const KIND: MesgNum = MesgNum::SpeedZone;
    fn parse_with_options(
        record: FitDataRecord,
        options: MessageParseOptions,
    ) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#high_value = None;
        let mut r#name = None;
        let mut r#message_index = None;
        #[allow(unused_mut)]
        let mut invalid_fields = BTreeMap::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#high_value = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("high_value", value);
                    }
                },
                1u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#name = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("name", value);
                    }
                },
                254u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#message_index = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("message_index", value);
                    }
                },
                _ => {
                    if !options.ignore_unexpected_fields {
                        return Err(TryFromRecordError::unexpected_field(&field));
                    }
                }
            }
        }
        Ok(Self {
            r#high_value,
            r#name,
            r#message_index,
            invalid_fields,
        })
    }
    fn invalid_fields(&self) -> &BTreeMap<&'static str, ValueWithUnits> {
        &self.invalid_fields
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
    pub r#high_value: Option<ValueWithUnits>,
    pub r#name: Option<ValueWithUnits>,
    pub r#message_index: Option<field_types::MessageIndex>,
    pub invalid_fields: BTreeMap<&'static str, ValueWithUnits>,
}
impl FitMessage for CadenceZone {
    const NAME: &'static str = "CadenceZone";
    const KIND: MesgNum = MesgNum::CadenceZone;
    fn parse_with_options(
        record: FitDataRecord,
        options: MessageParseOptions,
    ) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#high_value = None;
        let mut r#name = None;
        let mut r#message_index = None;
        #[allow(unused_mut)]
        let mut invalid_fields = BTreeMap::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#high_value = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("high_value", value);
                    }
                },
                1u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#name = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("name", value);
                    }
                },
                254u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#message_index = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("message_index", value);
                    }
                },
                _ => {
                    if !options.ignore_unexpected_fields {
                        return Err(TryFromRecordError::unexpected_field(&field));
                    }
                }
            }
        }
        Ok(Self {
            r#high_value,
            r#name,
            r#message_index,
            invalid_fields,
        })
    }
    fn invalid_fields(&self) -> &BTreeMap<&'static str, ValueWithUnits> {
        &self.invalid_fields
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
    pub r#high_value: Option<ValueWithUnits>,
    pub r#name: Option<ValueWithUnits>,
    pub r#message_index: Option<field_types::MessageIndex>,
    pub invalid_fields: BTreeMap<&'static str, ValueWithUnits>,
}
impl FitMessage for PowerZone {
    const NAME: &'static str = "PowerZone";
    const KIND: MesgNum = MesgNum::PowerZone;
    fn parse_with_options(
        record: FitDataRecord,
        options: MessageParseOptions,
    ) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#high_value = None;
        let mut r#name = None;
        let mut r#message_index = None;
        #[allow(unused_mut)]
        let mut invalid_fields = BTreeMap::new();
        for field in record.into_vec() {
            match field.number() {
                1u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#high_value = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("high_value", value);
                    }
                },
                2u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#name = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("name", value);
                    }
                },
                254u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#message_index = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("message_index", value);
                    }
                },
                _ => {
                    if !options.ignore_unexpected_fields {
                        return Err(TryFromRecordError::unexpected_field(&field));
                    }
                }
            }
        }
        Ok(Self {
            r#high_value,
            r#name,
            r#message_index,
            invalid_fields,
        })
    }
    fn invalid_fields(&self) -> &BTreeMap<&'static str, ValueWithUnits> {
        &self.invalid_fields
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
    pub r#high_bpm: Option<ValueWithUnits>,
    pub r#calories: Option<ValueWithUnits>,
    pub r#fat_calories: Option<ValueWithUnits>,
    pub r#message_index: Option<field_types::MessageIndex>,
    pub invalid_fields: BTreeMap<&'static str, ValueWithUnits>,
}
impl FitMessage for MetZone {
    const NAME: &'static str = "MetZone";
    const KIND: MesgNum = MesgNum::MetZone;
    fn parse_with_options(
        record: FitDataRecord,
        options: MessageParseOptions,
    ) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#high_bpm = None;
        let mut r#calories = None;
        let mut r#fat_calories = None;
        let mut r#message_index = None;
        #[allow(unused_mut)]
        let mut invalid_fields = BTreeMap::new();
        for field in record.into_vec() {
            match field.number() {
                1u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#high_bpm = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("high_bpm", value);
                    }
                },
                2u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#calories = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("calories", value);
                    }
                },
                3u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#fat_calories = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("fat_calories", value);
                    }
                },
                254u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#message_index = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("message_index", value);
                    }
                },
                _ => {
                    if !options.ignore_unexpected_fields {
                        return Err(TryFromRecordError::unexpected_field(&field));
                    }
                }
            }
        }
        Ok(Self {
            r#high_bpm,
            r#calories,
            r#fat_calories,
            r#message_index,
            invalid_fields,
        })
    }
    fn invalid_fields(&self) -> &BTreeMap<&'static str, ValueWithUnits> {
        &self.invalid_fields
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
    pub r#name: Option<ValueWithUnits>,
    pub r#model: Option<field_types::TissueModelType>,
    pub r#gf_low: Option<ValueWithUnits>,
    pub r#gf_high: Option<ValueWithUnits>,
    pub r#water_type: Option<field_types::WaterType>,
    pub r#water_density: Option<ValueWithUnits>,
    pub r#po2_warn: Option<ValueWithUnits>,
    pub r#po2_critical: Option<ValueWithUnits>,
    pub r#po2_deco: Option<ValueWithUnits>,
    pub r#safety_stop_enabled: Option<ValueWithUnits>,
    pub r#bottom_depth: Option<ValueWithUnits>,
    pub r#bottom_time: Option<ValueWithUnits>,
    pub r#apnea_countdown_enabled: Option<ValueWithUnits>,
    pub r#apnea_countdown_time: Option<ValueWithUnits>,
    pub r#backlight_mode: Option<field_types::DiveBacklightMode>,
    pub r#backlight_brightness: Option<ValueWithUnits>,
    pub r#backlight_timeout: Option<field_types::BacklightTimeout>,
    pub r#repeat_dive_interval: Option<ValueWithUnits>,
    pub r#safety_stop_time: Option<ValueWithUnits>,
    pub r#heart_rate_source_type: Option<field_types::SourceType>,
    pub r#heart_rate_source: Option<ValueWithUnits>,
    pub r#travel_gas: Option<field_types::MessageIndex>,
    pub r#ccr_low_setpoint_switch_mode: Option<field_types::CcrSetpointSwitchMode>,
    pub r#ccr_low_setpoint: Option<ValueWithUnits>,
    pub r#ccr_low_setpoint_depth: Option<ValueWithUnits>,
    pub r#ccr_high_setpoint_switch_mode: Option<field_types::CcrSetpointSwitchMode>,
    pub r#ccr_high_setpoint: Option<ValueWithUnits>,
    pub r#ccr_high_setpoint_depth: Option<ValueWithUnits>,
    pub r#gas_consumption_display: Option<field_types::GasConsumptionRateType>,
    pub r#up_key_enabled: Option<ValueWithUnits>,
    pub r#dive_sounds: Option<field_types::Tone>,
    pub r#last_stop_multiple: Option<ValueWithUnits>,
    pub r#no_fly_time_mode: Option<field_types::NoFlyTimeMode>,
    pub r#timestamp: Option<field_types::DateTime>,
    pub r#message_index: Option<field_types::MessageIndex>,
    pub invalid_fields: BTreeMap<&'static str, ValueWithUnits>,
}
impl FitMessage for DiveSettings {
    const NAME: &'static str = "DiveSettings";
    const KIND: MesgNum = MesgNum::DiveSettings;
    fn parse_with_options(
        record: FitDataRecord,
        options: MessageParseOptions,
    ) -> Result<Self, TryFromRecordError> {
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
        #[allow(unused_mut)]
        let mut invalid_fields = BTreeMap::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#name = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("name", value);
                    }
                },
                1u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#model = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("model", value);
                    }
                },
                2u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#gf_low = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("gf_low", value);
                    }
                },
                3u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#gf_high = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("gf_high", value);
                    }
                },
                4u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#water_type = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("water_type", value);
                    }
                },
                5u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#water_density = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("water_density", value);
                    }
                },
                6u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#po2_warn = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("po2_warn", value);
                    }
                },
                7u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#po2_critical = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("po2_critical", value);
                    }
                },
                8u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#po2_deco = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("po2_deco", value);
                    }
                },
                9u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#safety_stop_enabled = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("safety_stop_enabled", value);
                    }
                },
                10u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#bottom_depth = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("bottom_depth", value);
                    }
                },
                11u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#bottom_time = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("bottom_time", value);
                    }
                },
                12u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#apnea_countdown_enabled = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("apnea_countdown_enabled", value);
                    }
                },
                13u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#apnea_countdown_time = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("apnea_countdown_time", value);
                    }
                },
                14u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#backlight_mode = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("backlight_mode", value);
                    }
                },
                15u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#backlight_brightness = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("backlight_brightness", value);
                    }
                },
                16u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#backlight_timeout = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("backlight_timeout", value);
                    }
                },
                17u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#repeat_dive_interval = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("repeat_dive_interval", value);
                    }
                },
                18u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#safety_stop_time = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("safety_stop_time", value);
                    }
                },
                19u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#heart_rate_source_type = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("heart_rate_source_type", value);
                    }
                },
                20u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#heart_rate_source = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("heart_rate_source", value);
                    }
                },
                21u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#travel_gas = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("travel_gas", value);
                    }
                },
                22u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#ccr_low_setpoint_switch_mode = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("ccr_low_setpoint_switch_mode", value);
                    }
                },
                23u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#ccr_low_setpoint = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("ccr_low_setpoint", value);
                    }
                },
                24u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#ccr_low_setpoint_depth = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("ccr_low_setpoint_depth", value);
                    }
                },
                25u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#ccr_high_setpoint_switch_mode = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("ccr_high_setpoint_switch_mode", value);
                    }
                },
                26u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#ccr_high_setpoint = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("ccr_high_setpoint", value);
                    }
                },
                27u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#ccr_high_setpoint_depth = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("ccr_high_setpoint_depth", value);
                    }
                },
                29u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#gas_consumption_display = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("gas_consumption_display", value);
                    }
                },
                30u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#up_key_enabled = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("up_key_enabled", value);
                    }
                },
                35u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#dive_sounds = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("dive_sounds", value);
                    }
                },
                36u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#last_stop_multiple = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("last_stop_multiple", value);
                    }
                },
                37u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#no_fly_time_mode = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("no_fly_time_mode", value);
                    }
                },
                253u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#timestamp = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("timestamp", value);
                    }
                },
                254u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#message_index = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("message_index", value);
                    }
                },
                _ => {
                    if !options.ignore_unexpected_fields {
                        return Err(TryFromRecordError::unexpected_field(&field));
                    }
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
            invalid_fields,
        })
    }
    fn invalid_fields(&self) -> &BTreeMap<&'static str, ValueWithUnits> {
        &self.invalid_fields
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
    pub r#depth: Option<ValueWithUnits>,
    pub r#time: Option<ValueWithUnits>,
    pub r#enabled: Option<ValueWithUnits>,
    pub r#alarm_type: Option<field_types::DiveAlarmType>,
    pub r#sound: Option<field_types::Tone>,
    pub r#dive_types: Option<ValueWithUnits>,
    pub r#id: Option<ValueWithUnits>,
    pub r#popup_enabled: Option<ValueWithUnits>,
    pub r#trigger_on_descent: Option<ValueWithUnits>,
    pub r#trigger_on_ascent: Option<ValueWithUnits>,
    pub r#repeating: Option<ValueWithUnits>,
    pub r#speed: Option<ValueWithUnits>,
    pub r#message_index: Option<field_types::MessageIndex>,
    pub invalid_fields: BTreeMap<&'static str, ValueWithUnits>,
}
impl FitMessage for DiveAlarm {
    const NAME: &'static str = "DiveAlarm";
    const KIND: MesgNum = MesgNum::DiveAlarm;
    fn parse_with_options(
        record: FitDataRecord,
        options: MessageParseOptions,
    ) -> Result<Self, TryFromRecordError> {
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
        #[allow(unused_mut)]
        let mut invalid_fields = BTreeMap::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#depth = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("depth", value);
                    }
                },
                1u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#time = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("time", value);
                    }
                },
                2u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#enabled = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("enabled", value);
                    }
                },
                3u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#alarm_type = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("alarm_type", value);
                    }
                },
                4u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#sound = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("sound", value);
                    }
                },
                5u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#dive_types = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("dive_types", value);
                    }
                },
                6u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#id = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("id", value);
                    }
                },
                7u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#popup_enabled = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("popup_enabled", value);
                    }
                },
                8u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#trigger_on_descent = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("trigger_on_descent", value);
                    }
                },
                9u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#trigger_on_ascent = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("trigger_on_ascent", value);
                    }
                },
                10u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#repeating = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("repeating", value);
                    }
                },
                11u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#speed = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("speed", value);
                    }
                },
                254u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#message_index = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("message_index", value);
                    }
                },
                _ => {
                    if !options.ignore_unexpected_fields {
                        return Err(TryFromRecordError::unexpected_field(&field));
                    }
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
            invalid_fields,
        })
    }
    fn invalid_fields(&self) -> &BTreeMap<&'static str, ValueWithUnits> {
        &self.invalid_fields
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
    pub r#depth: Option<ValueWithUnits>,
    pub r#time: Option<ValueWithUnits>,
    pub r#enabled: Option<ValueWithUnits>,
    pub r#alarm_type: Option<field_types::DiveAlarmType>,
    pub r#sound: Option<field_types::Tone>,
    pub r#dive_types: Option<ValueWithUnits>,
    pub r#id: Option<ValueWithUnits>,
    pub r#popup_enabled: Option<ValueWithUnits>,
    pub r#trigger_on_descent: Option<ValueWithUnits>,
    pub r#trigger_on_ascent: Option<ValueWithUnits>,
    pub r#repeating: Option<ValueWithUnits>,
    pub r#speed: Option<ValueWithUnits>,
    pub r#message_index: Option<field_types::MessageIndex>,
    pub invalid_fields: BTreeMap<&'static str, ValueWithUnits>,
}
impl FitMessage for DiveApneaAlarm {
    const NAME: &'static str = "DiveApneaAlarm";
    const KIND: MesgNum = MesgNum::DiveApneaAlarm;
    fn parse_with_options(
        record: FitDataRecord,
        options: MessageParseOptions,
    ) -> Result<Self, TryFromRecordError> {
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
        #[allow(unused_mut)]
        let mut invalid_fields = BTreeMap::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#depth = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("depth", value);
                    }
                },
                1u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#time = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("time", value);
                    }
                },
                2u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#enabled = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("enabled", value);
                    }
                },
                3u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#alarm_type = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("alarm_type", value);
                    }
                },
                4u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#sound = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("sound", value);
                    }
                },
                5u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#dive_types = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("dive_types", value);
                    }
                },
                6u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#id = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("id", value);
                    }
                },
                7u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#popup_enabled = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("popup_enabled", value);
                    }
                },
                8u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#trigger_on_descent = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("trigger_on_descent", value);
                    }
                },
                9u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#trigger_on_ascent = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("trigger_on_ascent", value);
                    }
                },
                10u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#repeating = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("repeating", value);
                    }
                },
                11u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#speed = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("speed", value);
                    }
                },
                254u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#message_index = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("message_index", value);
                    }
                },
                _ => {
                    if !options.ignore_unexpected_fields {
                        return Err(TryFromRecordError::unexpected_field(&field));
                    }
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
            invalid_fields,
        })
    }
    fn invalid_fields(&self) -> &BTreeMap<&'static str, ValueWithUnits> {
        &self.invalid_fields
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
    pub r#helium_content: Option<ValueWithUnits>,
    pub r#oxygen_content: Option<ValueWithUnits>,
    pub r#status: Option<field_types::DiveGasStatus>,
    pub r#mode: Option<field_types::DiveGasMode>,
    pub r#message_index: Option<field_types::MessageIndex>,
    pub invalid_fields: BTreeMap<&'static str, ValueWithUnits>,
}
impl FitMessage for DiveGas {
    const NAME: &'static str = "DiveGas";
    const KIND: MesgNum = MesgNum::DiveGas;
    fn parse_with_options(
        record: FitDataRecord,
        options: MessageParseOptions,
    ) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#helium_content = None;
        let mut r#oxygen_content = None;
        let mut r#status = None;
        let mut r#mode = None;
        let mut r#message_index = None;
        #[allow(unused_mut)]
        let mut invalid_fields = BTreeMap::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#helium_content = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("helium_content", value);
                    }
                },
                1u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#oxygen_content = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("oxygen_content", value);
                    }
                },
                2u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#status = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("status", value);
                    }
                },
                3u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#mode = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("mode", value);
                    }
                },
                254u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#message_index = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("message_index", value);
                    }
                },
                _ => {
                    if !options.ignore_unexpected_fields {
                        return Err(TryFromRecordError::unexpected_field(&field));
                    }
                }
            }
        }
        Ok(Self {
            r#helium_content,
            r#oxygen_content,
            r#status,
            r#mode,
            r#message_index,
            invalid_fields,
        })
    }
    fn invalid_fields(&self) -> &BTreeMap<&'static str, ValueWithUnits> {
        &self.invalid_fields
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
    pub r#sport: Option<field_types::Sport>,
    pub r#sub_sport: Option<field_types::SubSport>,
    pub r#start_date: Option<field_types::DateTime>,
    pub r#end_date: Option<field_types::DateTime>,
    pub r#type: Option<field_types::Goal>,
    pub r#value: Option<ValueWithUnits>,
    pub r#repeat: Option<ValueWithUnits>,
    pub r#target_value: Option<ValueWithUnits>,
    pub r#recurrence: Option<field_types::GoalRecurrence>,
    pub r#recurrence_value: Option<ValueWithUnits>,
    pub r#enabled: Option<ValueWithUnits>,
    pub r#source: Option<field_types::GoalSource>,
    pub r#message_index: Option<field_types::MessageIndex>,
    pub invalid_fields: BTreeMap<&'static str, ValueWithUnits>,
}
impl FitMessage for Goal {
    const NAME: &'static str = "Goal";
    const KIND: MesgNum = MesgNum::Goal;
    fn parse_with_options(
        record: FitDataRecord,
        options: MessageParseOptions,
    ) -> Result<Self, TryFromRecordError> {
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
        #[allow(unused_mut)]
        let mut invalid_fields = BTreeMap::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#sport = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("sport", value);
                    }
                },
                1u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#sub_sport = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("sub_sport", value);
                    }
                },
                2u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#start_date = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("start_date", value);
                    }
                },
                3u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#end_date = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("end_date", value);
                    }
                },
                4u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#type = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("type", value);
                    }
                },
                5u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#value = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("value", value);
                    }
                },
                6u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#repeat = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("repeat", value);
                    }
                },
                7u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#target_value = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("target_value", value);
                    }
                },
                8u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#recurrence = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("recurrence", value);
                    }
                },
                9u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#recurrence_value = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("recurrence_value", value);
                    }
                },
                10u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#enabled = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("enabled", value);
                    }
                },
                11u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#source = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("source", value);
                    }
                },
                254u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#message_index = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("message_index", value);
                    }
                },
                _ => {
                    if !options.ignore_unexpected_fields {
                        return Err(TryFromRecordError::unexpected_field(&field));
                    }
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
            invalid_fields,
        })
    }
    fn invalid_fields(&self) -> &BTreeMap<&'static str, ValueWithUnits> {
        &self.invalid_fields
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
    pub r#total_timer_time: Option<ValueWithUnits>,
    pub r#num_sessions: Option<ValueWithUnits>,
    pub r#type: Option<field_types::Activity>,
    pub r#event: Option<field_types::Event>,
    pub r#event_type: Option<field_types::EventType>,
    pub r#local_timestamp: Option<field_types::LocalDateTime>,
    pub r#event_group: Option<ValueWithUnits>,
    pub r#timestamp: Option<field_types::DateTime>,
    pub invalid_fields: BTreeMap<&'static str, ValueWithUnits>,
}
impl FitMessage for Activity {
    const NAME: &'static str = "Activity";
    const KIND: MesgNum = MesgNum::Activity;
    fn parse_with_options(
        record: FitDataRecord,
        options: MessageParseOptions,
    ) -> Result<Self, TryFromRecordError> {
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
        #[allow(unused_mut)]
        let mut invalid_fields = BTreeMap::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#total_timer_time = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("total_timer_time", value);
                    }
                },
                1u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#num_sessions = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("num_sessions", value);
                    }
                },
                2u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#type = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("type", value);
                    }
                },
                3u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#event = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("event", value);
                    }
                },
                4u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#event_type = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("event_type", value);
                    }
                },
                5u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#local_timestamp = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("local_timestamp", value);
                    }
                },
                6u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#event_group = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("event_group", value);
                    }
                },
                253u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#timestamp = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("timestamp", value);
                    }
                },
                _ => {
                    if !options.ignore_unexpected_fields {
                        return Err(TryFromRecordError::unexpected_field(&field));
                    }
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
            invalid_fields,
        })
    }
    fn invalid_fields(&self) -> &BTreeMap<&'static str, ValueWithUnits> {
        &self.invalid_fields
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
    pub r#event: Option<field_types::Event>,
    pub r#event_type: Option<field_types::EventType>,
    pub r#start_time: Option<field_types::DateTime>,
    pub r#start_position_lat: Option<ValueWithUnits>,
    pub r#start_position_long: Option<ValueWithUnits>,
    pub r#sport: Option<field_types::Sport>,
    pub r#sub_sport: Option<field_types::SubSport>,
    pub r#total_elapsed_time: Option<ValueWithUnits>,
    pub r#total_timer_time: Option<ValueWithUnits>,
    pub r#total_distance: Option<ValueWithUnits>,
    pub r#total_cycles: Option<ValueWithUnits>,
    pub r#total_calories: Option<ValueWithUnits>,
    pub r#total_fat_calories: Option<ValueWithUnits>,
    pub r#avg_speed: Option<ValueWithUnits>,
    pub r#max_speed: Option<ValueWithUnits>,
    pub r#avg_heart_rate: Option<ValueWithUnits>,
    pub r#max_heart_rate: Option<ValueWithUnits>,
    pub r#avg_cadence: Option<ValueWithUnits>,
    pub r#max_cadence: Option<ValueWithUnits>,
    pub r#avg_power: Option<ValueWithUnits>,
    pub r#max_power: Option<ValueWithUnits>,
    pub r#total_ascent: Option<ValueWithUnits>,
    pub r#total_descent: Option<ValueWithUnits>,
    pub r#total_training_effect: Option<ValueWithUnits>,
    pub r#first_lap_index: Option<ValueWithUnits>,
    pub r#num_laps: Option<ValueWithUnits>,
    pub r#event_group: Option<ValueWithUnits>,
    pub r#trigger: Option<field_types::SessionTrigger>,
    pub r#nec_lat: Option<ValueWithUnits>,
    pub r#nec_long: Option<ValueWithUnits>,
    pub r#swc_lat: Option<ValueWithUnits>,
    pub r#swc_long: Option<ValueWithUnits>,
    pub r#num_lengths: Option<ValueWithUnits>,
    pub r#normalized_power: Option<ValueWithUnits>,
    pub r#training_stress_score: Option<ValueWithUnits>,
    pub r#intensity_factor: Option<ValueWithUnits>,
    pub r#left_right_balance: Option<field_types::LeftRightBalance100>,
    pub r#end_position_lat: Option<ValueWithUnits>,
    pub r#end_position_long: Option<ValueWithUnits>,
    pub r#avg_stroke_count: Option<ValueWithUnits>,
    pub r#avg_stroke_distance: Option<ValueWithUnits>,
    pub r#swim_stroke: Option<field_types::SwimStroke>,
    pub r#pool_length: Option<ValueWithUnits>,
    pub r#threshold_power: Option<ValueWithUnits>,
    pub r#pool_length_unit: Option<field_types::DisplayMeasure>,
    pub r#num_active_lengths: Option<ValueWithUnits>,
    pub r#total_work: Option<ValueWithUnits>,
    pub r#avg_altitude: Option<ValueWithUnits>,
    pub r#max_altitude: Option<ValueWithUnits>,
    pub r#gps_accuracy: Option<ValueWithUnits>,
    pub r#avg_grade: Option<ValueWithUnits>,
    pub r#avg_pos_grade: Option<ValueWithUnits>,
    pub r#avg_neg_grade: Option<ValueWithUnits>,
    pub r#max_pos_grade: Option<ValueWithUnits>,
    pub r#max_neg_grade: Option<ValueWithUnits>,
    pub r#avg_temperature: Option<ValueWithUnits>,
    pub r#max_temperature: Option<ValueWithUnits>,
    pub r#total_moving_time: Option<ValueWithUnits>,
    pub r#avg_pos_vertical_speed: Option<ValueWithUnits>,
    pub r#avg_neg_vertical_speed: Option<ValueWithUnits>,
    pub r#max_pos_vertical_speed: Option<ValueWithUnits>,
    pub r#max_neg_vertical_speed: Option<ValueWithUnits>,
    pub r#min_heart_rate: Option<ValueWithUnits>,
    pub r#time_in_hr_zone: Option<ValueWithUnits>,
    pub r#time_in_speed_zone: Option<ValueWithUnits>,
    pub r#time_in_cadence_zone: Option<ValueWithUnits>,
    pub r#time_in_power_zone: Option<ValueWithUnits>,
    pub r#avg_lap_time: Option<ValueWithUnits>,
    pub r#best_lap_index: Option<ValueWithUnits>,
    pub r#min_altitude: Option<ValueWithUnits>,
    pub r#player_score: Option<ValueWithUnits>,
    pub r#opponent_score: Option<ValueWithUnits>,
    pub r#opponent_name: Option<ValueWithUnits>,
    pub r#stroke_count: Option<ValueWithUnits>,
    pub r#zone_count: Option<ValueWithUnits>,
    pub r#max_ball_speed: Option<ValueWithUnits>,
    pub r#avg_ball_speed: Option<ValueWithUnits>,
    pub r#avg_vertical_oscillation: Option<ValueWithUnits>,
    pub r#avg_stance_time_percent: Option<ValueWithUnits>,
    pub r#avg_stance_time: Option<ValueWithUnits>,
    pub r#avg_fractional_cadence: Option<ValueWithUnits>,
    pub r#max_fractional_cadence: Option<ValueWithUnits>,
    pub r#total_fractional_cycles: Option<ValueWithUnits>,
    pub r#avg_total_hemoglobin_conc: Option<ValueWithUnits>,
    pub r#min_total_hemoglobin_conc: Option<ValueWithUnits>,
    pub r#max_total_hemoglobin_conc: Option<ValueWithUnits>,
    pub r#avg_saturated_hemoglobin_percent: Option<ValueWithUnits>,
    pub r#min_saturated_hemoglobin_percent: Option<ValueWithUnits>,
    pub r#max_saturated_hemoglobin_percent: Option<ValueWithUnits>,
    pub r#avg_left_torque_effectiveness: Option<ValueWithUnits>,
    pub r#avg_right_torque_effectiveness: Option<ValueWithUnits>,
    pub r#avg_left_pedal_smoothness: Option<ValueWithUnits>,
    pub r#avg_right_pedal_smoothness: Option<ValueWithUnits>,
    pub r#avg_combined_pedal_smoothness: Option<ValueWithUnits>,
    pub r#sport_profile_name: Option<ValueWithUnits>,
    pub r#sport_index: Option<ValueWithUnits>,
    pub r#time_standing: Option<ValueWithUnits>,
    pub r#stand_count: Option<ValueWithUnits>,
    pub r#avg_left_pco: Option<ValueWithUnits>,
    pub r#avg_right_pco: Option<ValueWithUnits>,
    pub r#avg_left_power_phase: Option<ValueWithUnits>,
    pub r#avg_left_power_phase_peak: Option<ValueWithUnits>,
    pub r#avg_right_power_phase: Option<ValueWithUnits>,
    pub r#avg_right_power_phase_peak: Option<ValueWithUnits>,
    pub r#avg_power_position: Option<ValueWithUnits>,
    pub r#max_power_position: Option<ValueWithUnits>,
    pub r#avg_cadence_position: Option<ValueWithUnits>,
    pub r#max_cadence_position: Option<ValueWithUnits>,
    pub r#enhanced_avg_speed: Option<ValueWithUnits>,
    pub r#enhanced_max_speed: Option<ValueWithUnits>,
    pub r#enhanced_avg_altitude: Option<ValueWithUnits>,
    pub r#enhanced_min_altitude: Option<ValueWithUnits>,
    pub r#enhanced_max_altitude: Option<ValueWithUnits>,
    pub r#avg_lev_motor_power: Option<ValueWithUnits>,
    pub r#max_lev_motor_power: Option<ValueWithUnits>,
    pub r#lev_battery_consumption: Option<ValueWithUnits>,
    pub r#avg_vertical_ratio: Option<ValueWithUnits>,
    pub r#avg_stance_time_balance: Option<ValueWithUnits>,
    pub r#avg_step_length: Option<ValueWithUnits>,
    pub r#total_anaerobic_training_effect: Option<ValueWithUnits>,
    pub r#avg_vam: Option<ValueWithUnits>,
    pub r#avg_depth: Option<ValueWithUnits>,
    pub r#max_depth: Option<ValueWithUnits>,
    pub r#surface_interval: Option<ValueWithUnits>,
    pub r#start_cns: Option<ValueWithUnits>,
    pub r#end_cns: Option<ValueWithUnits>,
    pub r#start_n2: Option<ValueWithUnits>,
    pub r#end_n2: Option<ValueWithUnits>,
    pub r#avg_respiration_rate: Option<ValueWithUnits>,
    pub r#max_respiration_rate: Option<ValueWithUnits>,
    pub r#min_respiration_rate: Option<ValueWithUnits>,
    pub r#min_temperature: Option<ValueWithUnits>,
    pub r#o2_toxicity: Option<ValueWithUnits>,
    pub r#dive_number: Option<ValueWithUnits>,
    pub r#training_load_peak: Option<ValueWithUnits>,
    pub r#enhanced_avg_respiration_rate: Option<ValueWithUnits>,
    pub r#enhanced_max_respiration_rate: Option<ValueWithUnits>,
    pub r#enhanced_min_respiration_rate: Option<ValueWithUnits>,
    pub r#total_grit: Option<ValueWithUnits>,
    pub r#total_flow: Option<ValueWithUnits>,
    pub r#jump_count: Option<ValueWithUnits>,
    pub r#avg_grit: Option<ValueWithUnits>,
    pub r#avg_flow: Option<ValueWithUnits>,
    pub r#avg_spo2: Option<ValueWithUnits>,
    pub r#avg_stress: Option<ValueWithUnits>,
    pub r#sdrr_hrv: Option<ValueWithUnits>,
    pub r#rmssd_hrv: Option<ValueWithUnits>,
    pub r#total_fractional_ascent: Option<ValueWithUnits>,
    pub r#total_fractional_descent: Option<ValueWithUnits>,
    pub r#avg_core_temperature: Option<ValueWithUnits>,
    pub r#min_core_temperature: Option<ValueWithUnits>,
    pub r#max_core_temperature: Option<ValueWithUnits>,
    pub r#timestamp: Option<field_types::DateTime>,
    pub r#message_index: Option<field_types::MessageIndex>,
    pub invalid_fields: BTreeMap<&'static str, ValueWithUnits>,
}
impl FitMessage for Session {
    const NAME: &'static str = "Session";
    const KIND: MesgNum = MesgNum::Session;
    fn parse_with_options(
        record: FitDataRecord,
        options: MessageParseOptions,
    ) -> Result<Self, TryFromRecordError> {
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
        #[allow(unused_mut)]
        let mut invalid_fields = BTreeMap::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#event = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("event", value);
                    }
                },
                1u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#event_type = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("event_type", value);
                    }
                },
                2u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#start_time = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("start_time", value);
                    }
                },
                3u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#start_position_lat = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("start_position_lat", value);
                    }
                },
                4u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#start_position_long = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("start_position_long", value);
                    }
                },
                5u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#sport = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("sport", value);
                    }
                },
                6u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#sub_sport = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("sub_sport", value);
                    }
                },
                7u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#total_elapsed_time = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("total_elapsed_time", value);
                    }
                },
                8u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#total_timer_time = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("total_timer_time", value);
                    }
                },
                9u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#total_distance = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("total_distance", value);
                    }
                },
                10u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#total_cycles = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("total_cycles", value);
                    }
                },
                11u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#total_calories = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("total_calories", value);
                    }
                },
                13u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#total_fat_calories = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("total_fat_calories", value);
                    }
                },
                14u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#avg_speed = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("avg_speed", value);
                    }
                },
                15u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#max_speed = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("max_speed", value);
                    }
                },
                16u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#avg_heart_rate = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("avg_heart_rate", value);
                    }
                },
                17u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#max_heart_rate = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("max_heart_rate", value);
                    }
                },
                18u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#avg_cadence = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("avg_cadence", value);
                    }
                },
                19u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#max_cadence = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("max_cadence", value);
                    }
                },
                20u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#avg_power = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("avg_power", value);
                    }
                },
                21u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#max_power = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("max_power", value);
                    }
                },
                22u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#total_ascent = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("total_ascent", value);
                    }
                },
                23u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#total_descent = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("total_descent", value);
                    }
                },
                24u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#total_training_effect = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("total_training_effect", value);
                    }
                },
                25u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#first_lap_index = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("first_lap_index", value);
                    }
                },
                26u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#num_laps = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("num_laps", value);
                    }
                },
                27u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#event_group = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("event_group", value);
                    }
                },
                28u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#trigger = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("trigger", value);
                    }
                },
                29u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#nec_lat = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("nec_lat", value);
                    }
                },
                30u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#nec_long = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("nec_long", value);
                    }
                },
                31u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#swc_lat = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("swc_lat", value);
                    }
                },
                32u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#swc_long = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("swc_long", value);
                    }
                },
                33u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#num_lengths = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("num_lengths", value);
                    }
                },
                34u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#normalized_power = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("normalized_power", value);
                    }
                },
                35u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#training_stress_score = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("training_stress_score", value);
                    }
                },
                36u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#intensity_factor = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("intensity_factor", value);
                    }
                },
                37u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#left_right_balance = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("left_right_balance", value);
                    }
                },
                38u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#end_position_lat = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("end_position_lat", value);
                    }
                },
                39u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#end_position_long = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("end_position_long", value);
                    }
                },
                41u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#avg_stroke_count = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("avg_stroke_count", value);
                    }
                },
                42u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#avg_stroke_distance = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("avg_stroke_distance", value);
                    }
                },
                43u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#swim_stroke = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("swim_stroke", value);
                    }
                },
                44u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#pool_length = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("pool_length", value);
                    }
                },
                45u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#threshold_power = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("threshold_power", value);
                    }
                },
                46u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#pool_length_unit = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("pool_length_unit", value);
                    }
                },
                47u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#num_active_lengths = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("num_active_lengths", value);
                    }
                },
                48u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#total_work = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("total_work", value);
                    }
                },
                49u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#avg_altitude = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("avg_altitude", value);
                    }
                },
                50u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#max_altitude = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("max_altitude", value);
                    }
                },
                51u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#gps_accuracy = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("gps_accuracy", value);
                    }
                },
                52u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#avg_grade = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("avg_grade", value);
                    }
                },
                53u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#avg_pos_grade = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("avg_pos_grade", value);
                    }
                },
                54u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#avg_neg_grade = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("avg_neg_grade", value);
                    }
                },
                55u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#max_pos_grade = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("max_pos_grade", value);
                    }
                },
                56u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#max_neg_grade = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("max_neg_grade", value);
                    }
                },
                57u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#avg_temperature = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("avg_temperature", value);
                    }
                },
                58u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#max_temperature = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("max_temperature", value);
                    }
                },
                59u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#total_moving_time = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("total_moving_time", value);
                    }
                },
                60u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#avg_pos_vertical_speed = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("avg_pos_vertical_speed", value);
                    }
                },
                61u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#avg_neg_vertical_speed = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("avg_neg_vertical_speed", value);
                    }
                },
                62u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#max_pos_vertical_speed = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("max_pos_vertical_speed", value);
                    }
                },
                63u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#max_neg_vertical_speed = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("max_neg_vertical_speed", value);
                    }
                },
                64u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#min_heart_rate = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("min_heart_rate", value);
                    }
                },
                65u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#time_in_hr_zone = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("time_in_hr_zone", value);
                    }
                },
                66u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#time_in_speed_zone = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("time_in_speed_zone", value);
                    }
                },
                67u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#time_in_cadence_zone = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("time_in_cadence_zone", value);
                    }
                },
                68u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#time_in_power_zone = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("time_in_power_zone", value);
                    }
                },
                69u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#avg_lap_time = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("avg_lap_time", value);
                    }
                },
                70u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#best_lap_index = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("best_lap_index", value);
                    }
                },
                71u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#min_altitude = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("min_altitude", value);
                    }
                },
                82u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#player_score = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("player_score", value);
                    }
                },
                83u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#opponent_score = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("opponent_score", value);
                    }
                },
                84u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#opponent_name = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("opponent_name", value);
                    }
                },
                85u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#stroke_count = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("stroke_count", value);
                    }
                },
                86u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#zone_count = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("zone_count", value);
                    }
                },
                87u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#max_ball_speed = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("max_ball_speed", value);
                    }
                },
                88u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#avg_ball_speed = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("avg_ball_speed", value);
                    }
                },
                89u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#avg_vertical_oscillation = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("avg_vertical_oscillation", value);
                    }
                },
                90u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#avg_stance_time_percent = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("avg_stance_time_percent", value);
                    }
                },
                91u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#avg_stance_time = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("avg_stance_time", value);
                    }
                },
                92u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#avg_fractional_cadence = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("avg_fractional_cadence", value);
                    }
                },
                93u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#max_fractional_cadence = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("max_fractional_cadence", value);
                    }
                },
                94u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#total_fractional_cycles = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("total_fractional_cycles", value);
                    }
                },
                95u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#avg_total_hemoglobin_conc = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("avg_total_hemoglobin_conc", value);
                    }
                },
                96u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#min_total_hemoglobin_conc = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("min_total_hemoglobin_conc", value);
                    }
                },
                97u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#max_total_hemoglobin_conc = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("max_total_hemoglobin_conc", value);
                    }
                },
                98u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#avg_saturated_hemoglobin_percent = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("avg_saturated_hemoglobin_percent", value);
                    }
                },
                99u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#min_saturated_hemoglobin_percent = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("min_saturated_hemoglobin_percent", value);
                    }
                },
                100u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#max_saturated_hemoglobin_percent = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("max_saturated_hemoglobin_percent", value);
                    }
                },
                101u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#avg_left_torque_effectiveness = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("avg_left_torque_effectiveness", value);
                    }
                },
                102u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#avg_right_torque_effectiveness = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("avg_right_torque_effectiveness", value);
                    }
                },
                103u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#avg_left_pedal_smoothness = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("avg_left_pedal_smoothness", value);
                    }
                },
                104u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#avg_right_pedal_smoothness = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("avg_right_pedal_smoothness", value);
                    }
                },
                105u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#avg_combined_pedal_smoothness = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("avg_combined_pedal_smoothness", value);
                    }
                },
                110u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#sport_profile_name = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("sport_profile_name", value);
                    }
                },
                111u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#sport_index = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("sport_index", value);
                    }
                },
                112u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#time_standing = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("time_standing", value);
                    }
                },
                113u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#stand_count = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("stand_count", value);
                    }
                },
                114u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#avg_left_pco = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("avg_left_pco", value);
                    }
                },
                115u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#avg_right_pco = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("avg_right_pco", value);
                    }
                },
                116u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#avg_left_power_phase = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("avg_left_power_phase", value);
                    }
                },
                117u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#avg_left_power_phase_peak = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("avg_left_power_phase_peak", value);
                    }
                },
                118u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#avg_right_power_phase = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("avg_right_power_phase", value);
                    }
                },
                119u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#avg_right_power_phase_peak = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("avg_right_power_phase_peak", value);
                    }
                },
                120u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#avg_power_position = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("avg_power_position", value);
                    }
                },
                121u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#max_power_position = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("max_power_position", value);
                    }
                },
                122u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#avg_cadence_position = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("avg_cadence_position", value);
                    }
                },
                123u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#max_cadence_position = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("max_cadence_position", value);
                    }
                },
                124u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#enhanced_avg_speed = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("enhanced_avg_speed", value);
                    }
                },
                125u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#enhanced_max_speed = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("enhanced_max_speed", value);
                    }
                },
                126u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#enhanced_avg_altitude = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("enhanced_avg_altitude", value);
                    }
                },
                127u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#enhanced_min_altitude = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("enhanced_min_altitude", value);
                    }
                },
                128u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#enhanced_max_altitude = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("enhanced_max_altitude", value);
                    }
                },
                129u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#avg_lev_motor_power = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("avg_lev_motor_power", value);
                    }
                },
                130u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#max_lev_motor_power = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("max_lev_motor_power", value);
                    }
                },
                131u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#lev_battery_consumption = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("lev_battery_consumption", value);
                    }
                },
                132u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#avg_vertical_ratio = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("avg_vertical_ratio", value);
                    }
                },
                133u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#avg_stance_time_balance = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("avg_stance_time_balance", value);
                    }
                },
                134u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#avg_step_length = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("avg_step_length", value);
                    }
                },
                137u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#total_anaerobic_training_effect = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("total_anaerobic_training_effect", value);
                    }
                },
                139u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#avg_vam = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("avg_vam", value);
                    }
                },
                140u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#avg_depth = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("avg_depth", value);
                    }
                },
                141u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#max_depth = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("max_depth", value);
                    }
                },
                142u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#surface_interval = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("surface_interval", value);
                    }
                },
                143u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#start_cns = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("start_cns", value);
                    }
                },
                144u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#end_cns = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("end_cns", value);
                    }
                },
                145u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#start_n2 = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("start_n2", value);
                    }
                },
                146u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#end_n2 = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("end_n2", value);
                    }
                },
                147u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#avg_respiration_rate = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("avg_respiration_rate", value);
                    }
                },
                148u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#max_respiration_rate = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("max_respiration_rate", value);
                    }
                },
                149u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#min_respiration_rate = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("min_respiration_rate", value);
                    }
                },
                150u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#min_temperature = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("min_temperature", value);
                    }
                },
                155u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#o2_toxicity = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("o2_toxicity", value);
                    }
                },
                156u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#dive_number = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("dive_number", value);
                    }
                },
                168u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#training_load_peak = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("training_load_peak", value);
                    }
                },
                169u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#enhanced_avg_respiration_rate = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("enhanced_avg_respiration_rate", value);
                    }
                },
                170u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#enhanced_max_respiration_rate = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("enhanced_max_respiration_rate", value);
                    }
                },
                180u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#enhanced_min_respiration_rate = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("enhanced_min_respiration_rate", value);
                    }
                },
                181u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#total_grit = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("total_grit", value);
                    }
                },
                182u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#total_flow = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("total_flow", value);
                    }
                },
                183u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#jump_count = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("jump_count", value);
                    }
                },
                186u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#avg_grit = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("avg_grit", value);
                    }
                },
                187u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#avg_flow = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("avg_flow", value);
                    }
                },
                194u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#avg_spo2 = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("avg_spo2", value);
                    }
                },
                195u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#avg_stress = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("avg_stress", value);
                    }
                },
                197u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#sdrr_hrv = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("sdrr_hrv", value);
                    }
                },
                198u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#rmssd_hrv = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("rmssd_hrv", value);
                    }
                },
                199u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#total_fractional_ascent = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("total_fractional_ascent", value);
                    }
                },
                200u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#total_fractional_descent = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("total_fractional_descent", value);
                    }
                },
                208u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#avg_core_temperature = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("avg_core_temperature", value);
                    }
                },
                209u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#min_core_temperature = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("min_core_temperature", value);
                    }
                },
                210u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#max_core_temperature = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("max_core_temperature", value);
                    }
                },
                253u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#timestamp = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("timestamp", value);
                    }
                },
                254u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#message_index = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("message_index", value);
                    }
                },
                _ => {
                    if !options.ignore_unexpected_fields {
                        return Err(TryFromRecordError::unexpected_field(&field));
                    }
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
            invalid_fields,
        })
    }
    fn invalid_fields(&self) -> &BTreeMap<&'static str, ValueWithUnits> {
        &self.invalid_fields
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
    pub r#event: Option<field_types::Event>,
    pub r#event_type: Option<field_types::EventType>,
    pub r#start_time: Option<field_types::DateTime>,
    pub r#start_position_lat: Option<ValueWithUnits>,
    pub r#start_position_long: Option<ValueWithUnits>,
    pub r#end_position_lat: Option<ValueWithUnits>,
    pub r#end_position_long: Option<ValueWithUnits>,
    pub r#total_elapsed_time: Option<ValueWithUnits>,
    pub r#total_timer_time: Option<ValueWithUnits>,
    pub r#total_distance: Option<ValueWithUnits>,
    pub r#total_cycles: Option<ValueWithUnits>,
    pub r#total_calories: Option<ValueWithUnits>,
    pub r#total_fat_calories: Option<ValueWithUnits>,
    pub r#avg_speed: Option<ValueWithUnits>,
    pub r#max_speed: Option<ValueWithUnits>,
    pub r#avg_heart_rate: Option<ValueWithUnits>,
    pub r#max_heart_rate: Option<ValueWithUnits>,
    pub r#avg_cadence: Option<ValueWithUnits>,
    pub r#max_cadence: Option<ValueWithUnits>,
    pub r#avg_power: Option<ValueWithUnits>,
    pub r#max_power: Option<ValueWithUnits>,
    pub r#total_ascent: Option<ValueWithUnits>,
    pub r#total_descent: Option<ValueWithUnits>,
    pub r#intensity: Option<field_types::Intensity>,
    pub r#lap_trigger: Option<field_types::LapTrigger>,
    pub r#sport: Option<field_types::Sport>,
    pub r#event_group: Option<ValueWithUnits>,
    pub r#num_lengths: Option<ValueWithUnits>,
    pub r#normalized_power: Option<ValueWithUnits>,
    pub r#left_right_balance: Option<field_types::LeftRightBalance100>,
    pub r#first_length_index: Option<ValueWithUnits>,
    pub r#avg_stroke_distance: Option<ValueWithUnits>,
    pub r#swim_stroke: Option<field_types::SwimStroke>,
    pub r#sub_sport: Option<field_types::SubSport>,
    pub r#num_active_lengths: Option<ValueWithUnits>,
    pub r#total_work: Option<ValueWithUnits>,
    pub r#avg_altitude: Option<ValueWithUnits>,
    pub r#max_altitude: Option<ValueWithUnits>,
    pub r#gps_accuracy: Option<ValueWithUnits>,
    pub r#avg_grade: Option<ValueWithUnits>,
    pub r#avg_pos_grade: Option<ValueWithUnits>,
    pub r#avg_neg_grade: Option<ValueWithUnits>,
    pub r#max_pos_grade: Option<ValueWithUnits>,
    pub r#max_neg_grade: Option<ValueWithUnits>,
    pub r#avg_temperature: Option<ValueWithUnits>,
    pub r#max_temperature: Option<ValueWithUnits>,
    pub r#total_moving_time: Option<ValueWithUnits>,
    pub r#avg_pos_vertical_speed: Option<ValueWithUnits>,
    pub r#avg_neg_vertical_speed: Option<ValueWithUnits>,
    pub r#max_pos_vertical_speed: Option<ValueWithUnits>,
    pub r#max_neg_vertical_speed: Option<ValueWithUnits>,
    pub r#time_in_hr_zone: Option<ValueWithUnits>,
    pub r#time_in_speed_zone: Option<ValueWithUnits>,
    pub r#time_in_cadence_zone: Option<ValueWithUnits>,
    pub r#time_in_power_zone: Option<ValueWithUnits>,
    pub r#repetition_num: Option<ValueWithUnits>,
    pub r#min_altitude: Option<ValueWithUnits>,
    pub r#min_heart_rate: Option<ValueWithUnits>,
    pub r#wkt_step_index: Option<field_types::MessageIndex>,
    pub r#opponent_score: Option<ValueWithUnits>,
    pub r#stroke_count: Option<ValueWithUnits>,
    pub r#zone_count: Option<ValueWithUnits>,
    pub r#avg_vertical_oscillation: Option<ValueWithUnits>,
    pub r#avg_stance_time_percent: Option<ValueWithUnits>,
    pub r#avg_stance_time: Option<ValueWithUnits>,
    pub r#avg_fractional_cadence: Option<ValueWithUnits>,
    pub r#max_fractional_cadence: Option<ValueWithUnits>,
    pub r#total_fractional_cycles: Option<ValueWithUnits>,
    pub r#player_score: Option<ValueWithUnits>,
    pub r#avg_total_hemoglobin_conc: Option<ValueWithUnits>,
    pub r#min_total_hemoglobin_conc: Option<ValueWithUnits>,
    pub r#max_total_hemoglobin_conc: Option<ValueWithUnits>,
    pub r#avg_saturated_hemoglobin_percent: Option<ValueWithUnits>,
    pub r#min_saturated_hemoglobin_percent: Option<ValueWithUnits>,
    pub r#max_saturated_hemoglobin_percent: Option<ValueWithUnits>,
    pub r#avg_left_torque_effectiveness: Option<ValueWithUnits>,
    pub r#avg_right_torque_effectiveness: Option<ValueWithUnits>,
    pub r#avg_left_pedal_smoothness: Option<ValueWithUnits>,
    pub r#avg_right_pedal_smoothness: Option<ValueWithUnits>,
    pub r#avg_combined_pedal_smoothness: Option<ValueWithUnits>,
    pub r#time_standing: Option<ValueWithUnits>,
    pub r#stand_count: Option<ValueWithUnits>,
    pub r#avg_left_pco: Option<ValueWithUnits>,
    pub r#avg_right_pco: Option<ValueWithUnits>,
    pub r#avg_left_power_phase: Option<ValueWithUnits>,
    pub r#avg_left_power_phase_peak: Option<ValueWithUnits>,
    pub r#avg_right_power_phase: Option<ValueWithUnits>,
    pub r#avg_right_power_phase_peak: Option<ValueWithUnits>,
    pub r#avg_power_position: Option<ValueWithUnits>,
    pub r#max_power_position: Option<ValueWithUnits>,
    pub r#avg_cadence_position: Option<ValueWithUnits>,
    pub r#max_cadence_position: Option<ValueWithUnits>,
    pub r#enhanced_avg_speed: Option<ValueWithUnits>,
    pub r#enhanced_max_speed: Option<ValueWithUnits>,
    pub r#enhanced_avg_altitude: Option<ValueWithUnits>,
    pub r#enhanced_min_altitude: Option<ValueWithUnits>,
    pub r#enhanced_max_altitude: Option<ValueWithUnits>,
    pub r#avg_lev_motor_power: Option<ValueWithUnits>,
    pub r#max_lev_motor_power: Option<ValueWithUnits>,
    pub r#lev_battery_consumption: Option<ValueWithUnits>,
    pub r#avg_vertical_ratio: Option<ValueWithUnits>,
    pub r#avg_stance_time_balance: Option<ValueWithUnits>,
    pub r#avg_step_length: Option<ValueWithUnits>,
    pub r#avg_vam: Option<ValueWithUnits>,
    pub r#avg_depth: Option<ValueWithUnits>,
    pub r#max_depth: Option<ValueWithUnits>,
    pub r#min_temperature: Option<ValueWithUnits>,
    pub r#enhanced_avg_respiration_rate: Option<ValueWithUnits>,
    pub r#enhanced_max_respiration_rate: Option<ValueWithUnits>,
    pub r#avg_respiration_rate: Option<ValueWithUnits>,
    pub r#max_respiration_rate: Option<ValueWithUnits>,
    pub r#total_grit: Option<ValueWithUnits>,
    pub r#total_flow: Option<ValueWithUnits>,
    pub r#jump_count: Option<ValueWithUnits>,
    pub r#avg_grit: Option<ValueWithUnits>,
    pub r#avg_flow: Option<ValueWithUnits>,
    pub r#total_fractional_ascent: Option<ValueWithUnits>,
    pub r#total_fractional_descent: Option<ValueWithUnits>,
    pub r#avg_core_temperature: Option<ValueWithUnits>,
    pub r#min_core_temperature: Option<ValueWithUnits>,
    pub r#max_core_temperature: Option<ValueWithUnits>,
    pub r#timestamp: Option<field_types::DateTime>,
    pub r#message_index: Option<field_types::MessageIndex>,
    pub invalid_fields: BTreeMap<&'static str, ValueWithUnits>,
}
impl FitMessage for Lap {
    const NAME: &'static str = "Lap";
    const KIND: MesgNum = MesgNum::Lap;
    fn parse_with_options(
        record: FitDataRecord,
        options: MessageParseOptions,
    ) -> Result<Self, TryFromRecordError> {
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
        #[allow(unused_mut)]
        let mut invalid_fields = BTreeMap::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#event = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("event", value);
                    }
                },
                1u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#event_type = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("event_type", value);
                    }
                },
                2u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#start_time = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("start_time", value);
                    }
                },
                3u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#start_position_lat = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("start_position_lat", value);
                    }
                },
                4u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#start_position_long = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("start_position_long", value);
                    }
                },
                5u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#end_position_lat = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("end_position_lat", value);
                    }
                },
                6u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#end_position_long = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("end_position_long", value);
                    }
                },
                7u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#total_elapsed_time = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("total_elapsed_time", value);
                    }
                },
                8u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#total_timer_time = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("total_timer_time", value);
                    }
                },
                9u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#total_distance = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("total_distance", value);
                    }
                },
                10u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#total_cycles = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("total_cycles", value);
                    }
                },
                11u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#total_calories = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("total_calories", value);
                    }
                },
                12u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#total_fat_calories = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("total_fat_calories", value);
                    }
                },
                13u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#avg_speed = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("avg_speed", value);
                    }
                },
                14u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#max_speed = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("max_speed", value);
                    }
                },
                15u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#avg_heart_rate = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("avg_heart_rate", value);
                    }
                },
                16u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#max_heart_rate = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("max_heart_rate", value);
                    }
                },
                17u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#avg_cadence = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("avg_cadence", value);
                    }
                },
                18u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#max_cadence = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("max_cadence", value);
                    }
                },
                19u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#avg_power = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("avg_power", value);
                    }
                },
                20u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#max_power = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("max_power", value);
                    }
                },
                21u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#total_ascent = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("total_ascent", value);
                    }
                },
                22u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#total_descent = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("total_descent", value);
                    }
                },
                23u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#intensity = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("intensity", value);
                    }
                },
                24u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#lap_trigger = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("lap_trigger", value);
                    }
                },
                25u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#sport = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("sport", value);
                    }
                },
                26u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#event_group = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("event_group", value);
                    }
                },
                32u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#num_lengths = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("num_lengths", value);
                    }
                },
                33u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#normalized_power = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("normalized_power", value);
                    }
                },
                34u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#left_right_balance = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("left_right_balance", value);
                    }
                },
                35u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#first_length_index = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("first_length_index", value);
                    }
                },
                37u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#avg_stroke_distance = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("avg_stroke_distance", value);
                    }
                },
                38u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#swim_stroke = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("swim_stroke", value);
                    }
                },
                39u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#sub_sport = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("sub_sport", value);
                    }
                },
                40u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#num_active_lengths = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("num_active_lengths", value);
                    }
                },
                41u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#total_work = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("total_work", value);
                    }
                },
                42u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#avg_altitude = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("avg_altitude", value);
                    }
                },
                43u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#max_altitude = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("max_altitude", value);
                    }
                },
                44u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#gps_accuracy = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("gps_accuracy", value);
                    }
                },
                45u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#avg_grade = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("avg_grade", value);
                    }
                },
                46u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#avg_pos_grade = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("avg_pos_grade", value);
                    }
                },
                47u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#avg_neg_grade = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("avg_neg_grade", value);
                    }
                },
                48u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#max_pos_grade = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("max_pos_grade", value);
                    }
                },
                49u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#max_neg_grade = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("max_neg_grade", value);
                    }
                },
                50u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#avg_temperature = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("avg_temperature", value);
                    }
                },
                51u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#max_temperature = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("max_temperature", value);
                    }
                },
                52u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#total_moving_time = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("total_moving_time", value);
                    }
                },
                53u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#avg_pos_vertical_speed = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("avg_pos_vertical_speed", value);
                    }
                },
                54u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#avg_neg_vertical_speed = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("avg_neg_vertical_speed", value);
                    }
                },
                55u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#max_pos_vertical_speed = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("max_pos_vertical_speed", value);
                    }
                },
                56u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#max_neg_vertical_speed = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("max_neg_vertical_speed", value);
                    }
                },
                57u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#time_in_hr_zone = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("time_in_hr_zone", value);
                    }
                },
                58u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#time_in_speed_zone = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("time_in_speed_zone", value);
                    }
                },
                59u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#time_in_cadence_zone = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("time_in_cadence_zone", value);
                    }
                },
                60u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#time_in_power_zone = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("time_in_power_zone", value);
                    }
                },
                61u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#repetition_num = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("repetition_num", value);
                    }
                },
                62u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#min_altitude = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("min_altitude", value);
                    }
                },
                63u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#min_heart_rate = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("min_heart_rate", value);
                    }
                },
                71u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#wkt_step_index = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("wkt_step_index", value);
                    }
                },
                74u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#opponent_score = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("opponent_score", value);
                    }
                },
                75u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#stroke_count = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("stroke_count", value);
                    }
                },
                76u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#zone_count = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("zone_count", value);
                    }
                },
                77u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#avg_vertical_oscillation = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("avg_vertical_oscillation", value);
                    }
                },
                78u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#avg_stance_time_percent = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("avg_stance_time_percent", value);
                    }
                },
                79u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#avg_stance_time = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("avg_stance_time", value);
                    }
                },
                80u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#avg_fractional_cadence = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("avg_fractional_cadence", value);
                    }
                },
                81u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#max_fractional_cadence = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("max_fractional_cadence", value);
                    }
                },
                82u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#total_fractional_cycles = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("total_fractional_cycles", value);
                    }
                },
                83u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#player_score = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("player_score", value);
                    }
                },
                84u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#avg_total_hemoglobin_conc = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("avg_total_hemoglobin_conc", value);
                    }
                },
                85u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#min_total_hemoglobin_conc = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("min_total_hemoglobin_conc", value);
                    }
                },
                86u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#max_total_hemoglobin_conc = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("max_total_hemoglobin_conc", value);
                    }
                },
                87u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#avg_saturated_hemoglobin_percent = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("avg_saturated_hemoglobin_percent", value);
                    }
                },
                88u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#min_saturated_hemoglobin_percent = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("min_saturated_hemoglobin_percent", value);
                    }
                },
                89u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#max_saturated_hemoglobin_percent = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("max_saturated_hemoglobin_percent", value);
                    }
                },
                91u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#avg_left_torque_effectiveness = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("avg_left_torque_effectiveness", value);
                    }
                },
                92u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#avg_right_torque_effectiveness = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("avg_right_torque_effectiveness", value);
                    }
                },
                93u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#avg_left_pedal_smoothness = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("avg_left_pedal_smoothness", value);
                    }
                },
                94u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#avg_right_pedal_smoothness = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("avg_right_pedal_smoothness", value);
                    }
                },
                95u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#avg_combined_pedal_smoothness = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("avg_combined_pedal_smoothness", value);
                    }
                },
                98u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#time_standing = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("time_standing", value);
                    }
                },
                99u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#stand_count = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("stand_count", value);
                    }
                },
                100u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#avg_left_pco = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("avg_left_pco", value);
                    }
                },
                101u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#avg_right_pco = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("avg_right_pco", value);
                    }
                },
                102u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#avg_left_power_phase = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("avg_left_power_phase", value);
                    }
                },
                103u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#avg_left_power_phase_peak = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("avg_left_power_phase_peak", value);
                    }
                },
                104u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#avg_right_power_phase = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("avg_right_power_phase", value);
                    }
                },
                105u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#avg_right_power_phase_peak = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("avg_right_power_phase_peak", value);
                    }
                },
                106u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#avg_power_position = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("avg_power_position", value);
                    }
                },
                107u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#max_power_position = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("max_power_position", value);
                    }
                },
                108u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#avg_cadence_position = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("avg_cadence_position", value);
                    }
                },
                109u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#max_cadence_position = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("max_cadence_position", value);
                    }
                },
                110u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#enhanced_avg_speed = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("enhanced_avg_speed", value);
                    }
                },
                111u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#enhanced_max_speed = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("enhanced_max_speed", value);
                    }
                },
                112u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#enhanced_avg_altitude = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("enhanced_avg_altitude", value);
                    }
                },
                113u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#enhanced_min_altitude = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("enhanced_min_altitude", value);
                    }
                },
                114u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#enhanced_max_altitude = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("enhanced_max_altitude", value);
                    }
                },
                115u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#avg_lev_motor_power = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("avg_lev_motor_power", value);
                    }
                },
                116u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#max_lev_motor_power = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("max_lev_motor_power", value);
                    }
                },
                117u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#lev_battery_consumption = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("lev_battery_consumption", value);
                    }
                },
                118u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#avg_vertical_ratio = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("avg_vertical_ratio", value);
                    }
                },
                119u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#avg_stance_time_balance = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("avg_stance_time_balance", value);
                    }
                },
                120u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#avg_step_length = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("avg_step_length", value);
                    }
                },
                121u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#avg_vam = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("avg_vam", value);
                    }
                },
                122u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#avg_depth = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("avg_depth", value);
                    }
                },
                123u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#max_depth = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("max_depth", value);
                    }
                },
                124u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#min_temperature = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("min_temperature", value);
                    }
                },
                136u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#enhanced_avg_respiration_rate = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("enhanced_avg_respiration_rate", value);
                    }
                },
                137u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#enhanced_max_respiration_rate = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("enhanced_max_respiration_rate", value);
                    }
                },
                147u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#avg_respiration_rate = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("avg_respiration_rate", value);
                    }
                },
                148u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#max_respiration_rate = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("max_respiration_rate", value);
                    }
                },
                149u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#total_grit = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("total_grit", value);
                    }
                },
                150u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#total_flow = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("total_flow", value);
                    }
                },
                151u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#jump_count = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("jump_count", value);
                    }
                },
                153u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#avg_grit = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("avg_grit", value);
                    }
                },
                154u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#avg_flow = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("avg_flow", value);
                    }
                },
                156u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#total_fractional_ascent = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("total_fractional_ascent", value);
                    }
                },
                157u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#total_fractional_descent = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("total_fractional_descent", value);
                    }
                },
                158u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#avg_core_temperature = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("avg_core_temperature", value);
                    }
                },
                159u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#min_core_temperature = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("min_core_temperature", value);
                    }
                },
                160u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#max_core_temperature = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("max_core_temperature", value);
                    }
                },
                253u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#timestamp = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("timestamp", value);
                    }
                },
                254u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#message_index = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("message_index", value);
                    }
                },
                _ => {
                    if !options.ignore_unexpected_fields {
                        return Err(TryFromRecordError::unexpected_field(&field));
                    }
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
            invalid_fields,
        })
    }
    fn invalid_fields(&self) -> &BTreeMap<&'static str, ValueWithUnits> {
        &self.invalid_fields
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
    pub r#event: Option<field_types::Event>,
    pub r#event_type: Option<field_types::EventType>,
    pub r#start_time: Option<field_types::DateTime>,
    pub r#total_elapsed_time: Option<ValueWithUnits>,
    pub r#total_timer_time: Option<ValueWithUnits>,
    pub r#total_strokes: Option<ValueWithUnits>,
    pub r#avg_speed: Option<ValueWithUnits>,
    pub r#swim_stroke: Option<field_types::SwimStroke>,
    pub r#avg_swimming_cadence: Option<ValueWithUnits>,
    pub r#event_group: Option<ValueWithUnits>,
    pub r#total_calories: Option<ValueWithUnits>,
    pub r#length_type: Option<field_types::LengthType>,
    pub r#player_score: Option<ValueWithUnits>,
    pub r#opponent_score: Option<ValueWithUnits>,
    pub r#stroke_count: Option<ValueWithUnits>,
    pub r#zone_count: Option<ValueWithUnits>,
    pub r#enhanced_avg_respiration_rate: Option<ValueWithUnits>,
    pub r#enhanced_max_respiration_rate: Option<ValueWithUnits>,
    pub r#avg_respiration_rate: Option<ValueWithUnits>,
    pub r#max_respiration_rate: Option<ValueWithUnits>,
    pub r#timestamp: Option<field_types::DateTime>,
    pub r#message_index: Option<field_types::MessageIndex>,
    pub invalid_fields: BTreeMap<&'static str, ValueWithUnits>,
}
impl FitMessage for Length {
    const NAME: &'static str = "Length";
    const KIND: MesgNum = MesgNum::Length;
    fn parse_with_options(
        record: FitDataRecord,
        options: MessageParseOptions,
    ) -> Result<Self, TryFromRecordError> {
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
        #[allow(unused_mut)]
        let mut invalid_fields = BTreeMap::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#event = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("event", value);
                    }
                },
                1u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#event_type = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("event_type", value);
                    }
                },
                2u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#start_time = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("start_time", value);
                    }
                },
                3u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#total_elapsed_time = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("total_elapsed_time", value);
                    }
                },
                4u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#total_timer_time = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("total_timer_time", value);
                    }
                },
                5u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#total_strokes = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("total_strokes", value);
                    }
                },
                6u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#avg_speed = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("avg_speed", value);
                    }
                },
                7u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#swim_stroke = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("swim_stroke", value);
                    }
                },
                9u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#avg_swimming_cadence = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("avg_swimming_cadence", value);
                    }
                },
                10u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#event_group = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("event_group", value);
                    }
                },
                11u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#total_calories = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("total_calories", value);
                    }
                },
                12u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#length_type = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("length_type", value);
                    }
                },
                18u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#player_score = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("player_score", value);
                    }
                },
                19u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#opponent_score = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("opponent_score", value);
                    }
                },
                20u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#stroke_count = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("stroke_count", value);
                    }
                },
                21u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#zone_count = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("zone_count", value);
                    }
                },
                22u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#enhanced_avg_respiration_rate = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("enhanced_avg_respiration_rate", value);
                    }
                },
                23u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#enhanced_max_respiration_rate = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("enhanced_max_respiration_rate", value);
                    }
                },
                24u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#avg_respiration_rate = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("avg_respiration_rate", value);
                    }
                },
                25u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#max_respiration_rate = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("max_respiration_rate", value);
                    }
                },
                253u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#timestamp = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("timestamp", value);
                    }
                },
                254u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#message_index = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("message_index", value);
                    }
                },
                _ => {
                    if !options.ignore_unexpected_fields {
                        return Err(TryFromRecordError::unexpected_field(&field));
                    }
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
            invalid_fields,
        })
    }
    fn invalid_fields(&self) -> &BTreeMap<&'static str, ValueWithUnits> {
        &self.invalid_fields
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
    pub r#position_lat: Option<ValueWithUnits>,
    pub r#position_long: Option<ValueWithUnits>,
    pub r#altitude: Option<ValueWithUnits>,
    pub r#heart_rate: Option<ValueWithUnits>,
    pub r#cadence: Option<ValueWithUnits>,
    pub r#distance: Option<ValueWithUnits>,
    pub r#speed: Option<ValueWithUnits>,
    pub r#power: Option<ValueWithUnits>,
    pub r#compressed_speed_distance: Option<ValueWithUnits>,
    pub r#grade: Option<ValueWithUnits>,
    pub r#resistance: Option<ValueWithUnits>,
    pub r#time_from_course: Option<ValueWithUnits>,
    pub r#cycle_length: Option<ValueWithUnits>,
    pub r#temperature: Option<ValueWithUnits>,
    pub r#speed_1s: Option<ValueWithUnits>,
    pub r#cycles: Option<ValueWithUnits>,
    pub r#total_cycles: Option<ValueWithUnits>,
    pub r#compressed_accumulated_power: Option<ValueWithUnits>,
    pub r#accumulated_power: Option<ValueWithUnits>,
    pub r#left_right_balance: Option<field_types::LeftRightBalance>,
    pub r#gps_accuracy: Option<ValueWithUnits>,
    pub r#vertical_speed: Option<ValueWithUnits>,
    pub r#calories: Option<ValueWithUnits>,
    pub r#vertical_oscillation: Option<ValueWithUnits>,
    pub r#stance_time_percent: Option<ValueWithUnits>,
    pub r#stance_time: Option<ValueWithUnits>,
    pub r#activity_type: Option<field_types::ActivityType>,
    pub r#left_torque_effectiveness: Option<ValueWithUnits>,
    pub r#right_torque_effectiveness: Option<ValueWithUnits>,
    pub r#left_pedal_smoothness: Option<ValueWithUnits>,
    pub r#right_pedal_smoothness: Option<ValueWithUnits>,
    pub r#combined_pedal_smoothness: Option<ValueWithUnits>,
    pub r#time128: Option<ValueWithUnits>,
    pub r#stroke_type: Option<field_types::StrokeType>,
    pub r#zone: Option<ValueWithUnits>,
    pub r#ball_speed: Option<ValueWithUnits>,
    pub r#cadence256: Option<ValueWithUnits>,
    pub r#fractional_cadence: Option<ValueWithUnits>,
    pub r#total_hemoglobin_conc: Option<ValueWithUnits>,
    pub r#total_hemoglobin_conc_min: Option<ValueWithUnits>,
    pub r#total_hemoglobin_conc_max: Option<ValueWithUnits>,
    pub r#saturated_hemoglobin_percent: Option<ValueWithUnits>,
    pub r#saturated_hemoglobin_percent_min: Option<ValueWithUnits>,
    pub r#saturated_hemoglobin_percent_max: Option<ValueWithUnits>,
    pub r#device_index: Option<field_types::DeviceIndex>,
    pub r#left_pco: Option<ValueWithUnits>,
    pub r#right_pco: Option<ValueWithUnits>,
    pub r#left_power_phase: Option<ValueWithUnits>,
    pub r#left_power_phase_peak: Option<ValueWithUnits>,
    pub r#right_power_phase: Option<ValueWithUnits>,
    pub r#right_power_phase_peak: Option<ValueWithUnits>,
    pub r#enhanced_speed: Option<ValueWithUnits>,
    pub r#enhanced_altitude: Option<ValueWithUnits>,
    pub r#battery_soc: Option<ValueWithUnits>,
    pub r#motor_power: Option<ValueWithUnits>,
    pub r#vertical_ratio: Option<ValueWithUnits>,
    pub r#stance_time_balance: Option<ValueWithUnits>,
    pub r#step_length: Option<ValueWithUnits>,
    pub r#cycle_length16: Option<ValueWithUnits>,
    pub r#absolute_pressure: Option<ValueWithUnits>,
    pub r#depth: Option<ValueWithUnits>,
    pub r#next_stop_depth: Option<ValueWithUnits>,
    pub r#next_stop_time: Option<ValueWithUnits>,
    pub r#time_to_surface: Option<ValueWithUnits>,
    pub r#ndl_time: Option<ValueWithUnits>,
    pub r#cns_load: Option<ValueWithUnits>,
    pub r#n2_load: Option<ValueWithUnits>,
    pub r#respiration_rate: Option<ValueWithUnits>,
    pub r#enhanced_respiration_rate: Option<ValueWithUnits>,
    pub r#grit: Option<ValueWithUnits>,
    pub r#flow: Option<ValueWithUnits>,
    pub r#current_stress: Option<ValueWithUnits>,
    pub r#ebike_travel_range: Option<ValueWithUnits>,
    pub r#ebike_battery_level: Option<ValueWithUnits>,
    pub r#ebike_assist_mode: Option<ValueWithUnits>,
    pub r#ebike_assist_level_percent: Option<ValueWithUnits>,
    pub r#air_time_remaining: Option<ValueWithUnits>,
    pub r#pressure_sac: Option<ValueWithUnits>,
    pub r#volume_sac: Option<ValueWithUnits>,
    pub r#rmv: Option<ValueWithUnits>,
    pub r#ascent_rate: Option<ValueWithUnits>,
    pub r#po2: Option<ValueWithUnits>,
    pub r#core_temperature: Option<ValueWithUnits>,
    pub r#timestamp: Option<field_types::DateTime>,
    pub invalid_fields: BTreeMap<&'static str, ValueWithUnits>,
}
impl FitMessage for Record {
    const NAME: &'static str = "Record";
    const KIND: MesgNum = MesgNum::Record;
    fn parse_with_options(
        record: FitDataRecord,
        options: MessageParseOptions,
    ) -> Result<Self, TryFromRecordError> {
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
        #[allow(unused_mut)]
        let mut invalid_fields = BTreeMap::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#position_lat = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("position_lat", value);
                    }
                },
                1u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#position_long = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("position_long", value);
                    }
                },
                2u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#altitude = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("altitude", value);
                    }
                },
                3u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#heart_rate = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("heart_rate", value);
                    }
                },
                4u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#cadence = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("cadence", value);
                    }
                },
                5u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#distance = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("distance", value);
                    }
                },
                6u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#speed = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("speed", value);
                    }
                },
                7u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#power = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("power", value);
                    }
                },
                8u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#compressed_speed_distance = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("compressed_speed_distance", value);
                    }
                },
                9u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#grade = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("grade", value);
                    }
                },
                10u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#resistance = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("resistance", value);
                    }
                },
                11u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#time_from_course = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("time_from_course", value);
                    }
                },
                12u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#cycle_length = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("cycle_length", value);
                    }
                },
                13u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#temperature = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("temperature", value);
                    }
                },
                17u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#speed_1s = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("speed_1s", value);
                    }
                },
                18u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#cycles = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("cycles", value);
                    }
                },
                19u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#total_cycles = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("total_cycles", value);
                    }
                },
                28u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#compressed_accumulated_power = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("compressed_accumulated_power", value);
                    }
                },
                29u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#accumulated_power = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("accumulated_power", value);
                    }
                },
                30u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#left_right_balance = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("left_right_balance", value);
                    }
                },
                31u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#gps_accuracy = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("gps_accuracy", value);
                    }
                },
                32u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#vertical_speed = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("vertical_speed", value);
                    }
                },
                33u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#calories = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("calories", value);
                    }
                },
                39u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#vertical_oscillation = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("vertical_oscillation", value);
                    }
                },
                40u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#stance_time_percent = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("stance_time_percent", value);
                    }
                },
                41u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#stance_time = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("stance_time", value);
                    }
                },
                42u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#activity_type = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("activity_type", value);
                    }
                },
                43u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#left_torque_effectiveness = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("left_torque_effectiveness", value);
                    }
                },
                44u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#right_torque_effectiveness = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("right_torque_effectiveness", value);
                    }
                },
                45u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#left_pedal_smoothness = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("left_pedal_smoothness", value);
                    }
                },
                46u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#right_pedal_smoothness = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("right_pedal_smoothness", value);
                    }
                },
                47u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#combined_pedal_smoothness = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("combined_pedal_smoothness", value);
                    }
                },
                48u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#time128 = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("time128", value);
                    }
                },
                49u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#stroke_type = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("stroke_type", value);
                    }
                },
                50u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#zone = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("zone", value);
                    }
                },
                51u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#ball_speed = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("ball_speed", value);
                    }
                },
                52u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#cadence256 = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("cadence256", value);
                    }
                },
                53u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#fractional_cadence = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("fractional_cadence", value);
                    }
                },
                54u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#total_hemoglobin_conc = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("total_hemoglobin_conc", value);
                    }
                },
                55u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#total_hemoglobin_conc_min = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("total_hemoglobin_conc_min", value);
                    }
                },
                56u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#total_hemoglobin_conc_max = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("total_hemoglobin_conc_max", value);
                    }
                },
                57u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#saturated_hemoglobin_percent = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("saturated_hemoglobin_percent", value);
                    }
                },
                58u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#saturated_hemoglobin_percent_min = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("saturated_hemoglobin_percent_min", value);
                    }
                },
                59u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#saturated_hemoglobin_percent_max = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("saturated_hemoglobin_percent_max", value);
                    }
                },
                62u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#device_index = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("device_index", value);
                    }
                },
                67u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#left_pco = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("left_pco", value);
                    }
                },
                68u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#right_pco = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("right_pco", value);
                    }
                },
                69u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#left_power_phase = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("left_power_phase", value);
                    }
                },
                70u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#left_power_phase_peak = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("left_power_phase_peak", value);
                    }
                },
                71u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#right_power_phase = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("right_power_phase", value);
                    }
                },
                72u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#right_power_phase_peak = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("right_power_phase_peak", value);
                    }
                },
                73u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#enhanced_speed = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("enhanced_speed", value);
                    }
                },
                78u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#enhanced_altitude = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("enhanced_altitude", value);
                    }
                },
                81u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#battery_soc = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("battery_soc", value);
                    }
                },
                82u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#motor_power = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("motor_power", value);
                    }
                },
                83u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#vertical_ratio = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("vertical_ratio", value);
                    }
                },
                84u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#stance_time_balance = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("stance_time_balance", value);
                    }
                },
                85u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#step_length = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("step_length", value);
                    }
                },
                87u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#cycle_length16 = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("cycle_length16", value);
                    }
                },
                91u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#absolute_pressure = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("absolute_pressure", value);
                    }
                },
                92u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#depth = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("depth", value);
                    }
                },
                93u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#next_stop_depth = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("next_stop_depth", value);
                    }
                },
                94u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#next_stop_time = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("next_stop_time", value);
                    }
                },
                95u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#time_to_surface = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("time_to_surface", value);
                    }
                },
                96u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#ndl_time = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("ndl_time", value);
                    }
                },
                97u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#cns_load = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("cns_load", value);
                    }
                },
                98u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#n2_load = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("n2_load", value);
                    }
                },
                99u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#respiration_rate = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("respiration_rate", value);
                    }
                },
                108u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#enhanced_respiration_rate = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("enhanced_respiration_rate", value);
                    }
                },
                114u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#grit = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("grit", value);
                    }
                },
                115u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#flow = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("flow", value);
                    }
                },
                116u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#current_stress = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("current_stress", value);
                    }
                },
                117u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#ebike_travel_range = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("ebike_travel_range", value);
                    }
                },
                118u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#ebike_battery_level = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("ebike_battery_level", value);
                    }
                },
                119u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#ebike_assist_mode = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("ebike_assist_mode", value);
                    }
                },
                120u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#ebike_assist_level_percent = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("ebike_assist_level_percent", value);
                    }
                },
                123u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#air_time_remaining = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("air_time_remaining", value);
                    }
                },
                124u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#pressure_sac = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("pressure_sac", value);
                    }
                },
                125u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#volume_sac = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("volume_sac", value);
                    }
                },
                126u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#rmv = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("rmv", value);
                    }
                },
                127u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#ascent_rate = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("ascent_rate", value);
                    }
                },
                129u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#po2 = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("po2", value);
                    }
                },
                139u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#core_temperature = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("core_temperature", value);
                    }
                },
                253u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#timestamp = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("timestamp", value);
                    }
                },
                _ => {
                    if !options.ignore_unexpected_fields {
                        return Err(TryFromRecordError::unexpected_field(&field));
                    }
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
            invalid_fields,
        })
    }
    fn invalid_fields(&self) -> &BTreeMap<&'static str, ValueWithUnits> {
        &self.invalid_fields
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
    pub r#event: Option<field_types::Event>,
    pub r#event_type: Option<field_types::EventType>,
    pub r#data16: Option<ValueWithUnits>,
    pub r#data: Option<ValueWithUnits>,
    pub r#event_group: Option<ValueWithUnits>,
    pub r#score: Option<ValueWithUnits>,
    pub r#opponent_score: Option<ValueWithUnits>,
    pub r#front_gear_num: Option<ValueWithUnits>,
    pub r#front_gear: Option<ValueWithUnits>,
    pub r#rear_gear_num: Option<ValueWithUnits>,
    pub r#rear_gear: Option<ValueWithUnits>,
    pub r#device_index: Option<field_types::DeviceIndex>,
    pub r#activity_type: Option<field_types::ActivityType>,
    pub r#start_timestamp: Option<field_types::DateTime>,
    pub r#radar_threat_level_max: Option<field_types::RadarThreatLevelType>,
    pub r#radar_threat_count: Option<ValueWithUnits>,
    pub r#radar_threat_avg_approach_speed: Option<ValueWithUnits>,
    pub r#radar_threat_max_approach_speed: Option<ValueWithUnits>,
    pub r#timestamp: Option<field_types::DateTime>,
    pub invalid_fields: BTreeMap<&'static str, ValueWithUnits>,
}
impl FitMessage for Event {
    const NAME: &'static str = "Event";
    const KIND: MesgNum = MesgNum::Event;
    fn parse_with_options(
        record: FitDataRecord,
        options: MessageParseOptions,
    ) -> Result<Self, TryFromRecordError> {
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
        #[allow(unused_mut)]
        let mut invalid_fields = BTreeMap::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#event = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("event", value);
                    }
                },
                1u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#event_type = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("event_type", value);
                    }
                },
                2u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#data16 = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("data16", value);
                    }
                },
                3u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#data = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("data", value);
                    }
                },
                4u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#event_group = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("event_group", value);
                    }
                },
                7u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#score = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("score", value);
                    }
                },
                8u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#opponent_score = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("opponent_score", value);
                    }
                },
                9u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#front_gear_num = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("front_gear_num", value);
                    }
                },
                10u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#front_gear = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("front_gear", value);
                    }
                },
                11u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#rear_gear_num = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("rear_gear_num", value);
                    }
                },
                12u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#rear_gear = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("rear_gear", value);
                    }
                },
                13u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#device_index = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("device_index", value);
                    }
                },
                14u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#activity_type = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("activity_type", value);
                    }
                },
                15u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#start_timestamp = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("start_timestamp", value);
                    }
                },
                21u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#radar_threat_level_max = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("radar_threat_level_max", value);
                    }
                },
                22u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#radar_threat_count = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("radar_threat_count", value);
                    }
                },
                23u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#radar_threat_avg_approach_speed = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("radar_threat_avg_approach_speed", value);
                    }
                },
                24u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#radar_threat_max_approach_speed = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("radar_threat_max_approach_speed", value);
                    }
                },
                253u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#timestamp = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("timestamp", value);
                    }
                },
                _ => {
                    if !options.ignore_unexpected_fields {
                        return Err(TryFromRecordError::unexpected_field(&field));
                    }
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
            invalid_fields,
        })
    }
    fn invalid_fields(&self) -> &BTreeMap<&'static str, ValueWithUnits> {
        &self.invalid_fields
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
    pub r#device_index: Option<field_types::DeviceIndex>,
    pub r#device_type: Option<ValueWithUnits>,
    pub r#manufacturer: Option<field_types::Manufacturer>,
    pub r#serial_number: Option<ValueWithUnits>,
    pub r#product: Option<ValueWithUnits>,
    pub r#software_version: Option<ValueWithUnits>,
    pub r#hardware_version: Option<ValueWithUnits>,
    pub r#cum_operating_time: Option<ValueWithUnits>,
    pub r#battery_voltage: Option<ValueWithUnits>,
    pub r#battery_status: Option<field_types::BatteryStatus>,
    pub r#sensor_position: Option<field_types::BodyLocation>,
    pub r#descriptor: Option<ValueWithUnits>,
    pub r#ant_transmission_type: Option<ValueWithUnits>,
    pub r#ant_device_number: Option<ValueWithUnits>,
    pub r#ant_network: Option<field_types::AntNetwork>,
    pub r#source_type: Option<field_types::SourceType>,
    pub r#product_name: Option<ValueWithUnits>,
    pub r#battery_level: Option<ValueWithUnits>,
    pub r#timestamp: Option<field_types::DateTime>,
    pub invalid_fields: BTreeMap<&'static str, ValueWithUnits>,
}
impl FitMessage for DeviceInfo {
    const NAME: &'static str = "DeviceInfo";
    const KIND: MesgNum = MesgNum::DeviceInfo;
    fn parse_with_options(
        record: FitDataRecord,
        options: MessageParseOptions,
    ) -> Result<Self, TryFromRecordError> {
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
        #[allow(unused_mut)]
        let mut invalid_fields = BTreeMap::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#device_index = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("device_index", value);
                    }
                },
                1u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#device_type = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("device_type", value);
                    }
                },
                2u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#manufacturer = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("manufacturer", value);
                    }
                },
                3u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#serial_number = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("serial_number", value);
                    }
                },
                4u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#product = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("product", value);
                    }
                },
                5u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#software_version = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("software_version", value);
                    }
                },
                6u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#hardware_version = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("hardware_version", value);
                    }
                },
                7u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#cum_operating_time = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("cum_operating_time", value);
                    }
                },
                10u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#battery_voltage = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("battery_voltage", value);
                    }
                },
                11u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#battery_status = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("battery_status", value);
                    }
                },
                18u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#sensor_position = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("sensor_position", value);
                    }
                },
                19u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#descriptor = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("descriptor", value);
                    }
                },
                20u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#ant_transmission_type = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("ant_transmission_type", value);
                    }
                },
                21u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#ant_device_number = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("ant_device_number", value);
                    }
                },
                22u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#ant_network = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("ant_network", value);
                    }
                },
                25u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#source_type = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("source_type", value);
                    }
                },
                27u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#product_name = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("product_name", value);
                    }
                },
                32u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#battery_level = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("battery_level", value);
                    }
                },
                253u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#timestamp = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("timestamp", value);
                    }
                },
                _ => {
                    if !options.ignore_unexpected_fields {
                        return Err(TryFromRecordError::unexpected_field(&field));
                    }
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
            invalid_fields,
        })
    }
    fn invalid_fields(&self) -> &BTreeMap<&'static str, ValueWithUnits> {
        &self.invalid_fields
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
    pub r#device_index: Option<field_types::DeviceIndex>,
    pub r#battery_voltage: Option<ValueWithUnits>,
    pub r#battery_status: Option<field_types::BatteryStatus>,
    pub r#battery_identifier: Option<ValueWithUnits>,
    pub r#timestamp: Option<field_types::DateTime>,
    pub invalid_fields: BTreeMap<&'static str, ValueWithUnits>,
}
impl FitMessage for DeviceAuxBatteryInfo {
    const NAME: &'static str = "DeviceAuxBatteryInfo";
    const KIND: MesgNum = MesgNum::DeviceAuxBatteryInfo;
    fn parse_with_options(
        record: FitDataRecord,
        options: MessageParseOptions,
    ) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#device_index = None;
        let mut r#battery_voltage = None;
        let mut r#battery_status = None;
        let mut r#battery_identifier = None;
        let mut r#timestamp = None;
        #[allow(unused_mut)]
        let mut invalid_fields = BTreeMap::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#device_index = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("device_index", value);
                    }
                },
                1u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#battery_voltage = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("battery_voltage", value);
                    }
                },
                2u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#battery_status = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("battery_status", value);
                    }
                },
                3u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#battery_identifier = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("battery_identifier", value);
                    }
                },
                253u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#timestamp = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("timestamp", value);
                    }
                },
                _ => {
                    if !options.ignore_unexpected_fields {
                        return Err(TryFromRecordError::unexpected_field(&field));
                    }
                }
            }
        }
        Ok(Self {
            r#device_index,
            r#battery_voltage,
            r#battery_status,
            r#battery_identifier,
            r#timestamp,
            invalid_fields,
        })
    }
    fn invalid_fields(&self) -> &BTreeMap<&'static str, ValueWithUnits> {
        &self.invalid_fields
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
    pub r#type: Option<field_types::File>,
    pub r#manufacturer: Option<field_types::Manufacturer>,
    pub r#product: Option<ValueWithUnits>,
    pub r#serial_number: Option<ValueWithUnits>,
    pub r#time_created: Option<field_types::DateTime>,
    pub r#timestamp: Option<field_types::DateTime>,
    pub invalid_fields: BTreeMap<&'static str, ValueWithUnits>,
}
impl FitMessage for TrainingFile {
    const NAME: &'static str = "TrainingFile";
    const KIND: MesgNum = MesgNum::TrainingFile;
    fn parse_with_options(
        record: FitDataRecord,
        options: MessageParseOptions,
    ) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#type = None;
        let mut r#manufacturer = None;
        let mut r#product = None;
        let mut r#serial_number = None;
        let mut r#time_created = None;
        let mut r#timestamp = None;
        #[allow(unused_mut)]
        let mut invalid_fields = BTreeMap::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#type = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("type", value);
                    }
                },
                1u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#manufacturer = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("manufacturer", value);
                    }
                },
                2u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#product = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("product", value);
                    }
                },
                3u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#serial_number = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("serial_number", value);
                    }
                },
                4u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#time_created = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("time_created", value);
                    }
                },
                253u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#timestamp = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("timestamp", value);
                    }
                },
                _ => {
                    if !options.ignore_unexpected_fields {
                        return Err(TryFromRecordError::unexpected_field(&field));
                    }
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
            invalid_fields,
        })
    }
    fn invalid_fields(&self) -> &BTreeMap<&'static str, ValueWithUnits> {
        &self.invalid_fields
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
    pub r#weather_report: Option<field_types::WeatherReport>,
    pub r#temperature: Option<ValueWithUnits>,
    pub r#condition: Option<field_types::WeatherStatus>,
    pub r#wind_direction: Option<ValueWithUnits>,
    pub r#wind_speed: Option<ValueWithUnits>,
    pub r#precipitation_probability: Option<ValueWithUnits>,
    pub r#temperature_feels_like: Option<ValueWithUnits>,
    pub r#relative_humidity: Option<ValueWithUnits>,
    pub r#location: Option<ValueWithUnits>,
    pub r#observed_at_time: Option<field_types::DateTime>,
    pub r#observed_location_lat: Option<ValueWithUnits>,
    pub r#observed_location_long: Option<ValueWithUnits>,
    pub r#day_of_week: Option<field_types::DayOfWeek>,
    pub r#high_temperature: Option<ValueWithUnits>,
    pub r#low_temperature: Option<ValueWithUnits>,
    pub r#timestamp: Option<field_types::DateTime>,
    pub invalid_fields: BTreeMap<&'static str, ValueWithUnits>,
}
impl FitMessage for WeatherConditions {
    const NAME: &'static str = "WeatherConditions";
    const KIND: MesgNum = MesgNum::WeatherConditions;
    fn parse_with_options(
        record: FitDataRecord,
        options: MessageParseOptions,
    ) -> Result<Self, TryFromRecordError> {
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
        #[allow(unused_mut)]
        let mut invalid_fields = BTreeMap::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#weather_report = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("weather_report", value);
                    }
                },
                1u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#temperature = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("temperature", value);
                    }
                },
                2u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#condition = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("condition", value);
                    }
                },
                3u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#wind_direction = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("wind_direction", value);
                    }
                },
                4u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#wind_speed = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("wind_speed", value);
                    }
                },
                5u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#precipitation_probability = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("precipitation_probability", value);
                    }
                },
                6u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#temperature_feels_like = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("temperature_feels_like", value);
                    }
                },
                7u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#relative_humidity = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("relative_humidity", value);
                    }
                },
                8u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#location = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("location", value);
                    }
                },
                9u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#observed_at_time = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("observed_at_time", value);
                    }
                },
                10u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#observed_location_lat = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("observed_location_lat", value);
                    }
                },
                11u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#observed_location_long = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("observed_location_long", value);
                    }
                },
                12u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#day_of_week = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("day_of_week", value);
                    }
                },
                13u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#high_temperature = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("high_temperature", value);
                    }
                },
                14u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#low_temperature = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("low_temperature", value);
                    }
                },
                253u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#timestamp = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("timestamp", value);
                    }
                },
                _ => {
                    if !options.ignore_unexpected_fields {
                        return Err(TryFromRecordError::unexpected_field(&field));
                    }
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
            invalid_fields,
        })
    }
    fn invalid_fields(&self) -> &BTreeMap<&'static str, ValueWithUnits> {
        &self.invalid_fields
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
    pub r#report_id: Option<ValueWithUnits>,
    pub r#issue_time: Option<field_types::DateTime>,
    pub r#expire_time: Option<field_types::DateTime>,
    pub r#severity: Option<field_types::WeatherSeverity>,
    pub r#type: Option<field_types::WeatherSevereType>,
    pub r#timestamp: Option<field_types::DateTime>,
    pub invalid_fields: BTreeMap<&'static str, ValueWithUnits>,
}
impl FitMessage for WeatherAlert {
    const NAME: &'static str = "WeatherAlert";
    const KIND: MesgNum = MesgNum::WeatherAlert;
    fn parse_with_options(
        record: FitDataRecord,
        options: MessageParseOptions,
    ) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#report_id = None;
        let mut r#issue_time = None;
        let mut r#expire_time = None;
        let mut r#severity = None;
        let mut r#type = None;
        let mut r#timestamp = None;
        #[allow(unused_mut)]
        let mut invalid_fields = BTreeMap::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#report_id = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("report_id", value);
                    }
                },
                1u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#issue_time = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("issue_time", value);
                    }
                },
                2u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#expire_time = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("expire_time", value);
                    }
                },
                3u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#severity = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("severity", value);
                    }
                },
                4u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#type = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("type", value);
                    }
                },
                253u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#timestamp = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("timestamp", value);
                    }
                },
                _ => {
                    if !options.ignore_unexpected_fields {
                        return Err(TryFromRecordError::unexpected_field(&field));
                    }
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
            invalid_fields,
        })
    }
    fn invalid_fields(&self) -> &BTreeMap<&'static str, ValueWithUnits> {
        &self.invalid_fields
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
    pub r#timestamp_ms: Option<ValueWithUnits>,
    pub r#position_lat: Option<ValueWithUnits>,
    pub r#position_long: Option<ValueWithUnits>,
    pub r#enhanced_altitude: Option<ValueWithUnits>,
    pub r#enhanced_speed: Option<ValueWithUnits>,
    pub r#heading: Option<ValueWithUnits>,
    pub r#utc_timestamp: Option<field_types::DateTime>,
    pub r#velocity: Option<ValueWithUnits>,
    pub r#timestamp: Option<field_types::DateTime>,
    pub invalid_fields: BTreeMap<&'static str, ValueWithUnits>,
}
impl FitMessage for GpsMetadata {
    const NAME: &'static str = "GpsMetadata";
    const KIND: MesgNum = MesgNum::GpsMetadata;
    fn parse_with_options(
        record: FitDataRecord,
        options: MessageParseOptions,
    ) -> Result<Self, TryFromRecordError> {
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
        #[allow(unused_mut)]
        let mut invalid_fields = BTreeMap::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#timestamp_ms = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("timestamp_ms", value);
                    }
                },
                1u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#position_lat = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("position_lat", value);
                    }
                },
                2u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#position_long = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("position_long", value);
                    }
                },
                3u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#enhanced_altitude = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("enhanced_altitude", value);
                    }
                },
                4u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#enhanced_speed = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("enhanced_speed", value);
                    }
                },
                5u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#heading = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("heading", value);
                    }
                },
                6u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#utc_timestamp = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("utc_timestamp", value);
                    }
                },
                7u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#velocity = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("velocity", value);
                    }
                },
                253u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#timestamp = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("timestamp", value);
                    }
                },
                _ => {
                    if !options.ignore_unexpected_fields {
                        return Err(TryFromRecordError::unexpected_field(&field));
                    }
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
            invalid_fields,
        })
    }
    fn invalid_fields(&self) -> &BTreeMap<&'static str, ValueWithUnits> {
        &self.invalid_fields
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
    pub r#timestamp_ms: Option<ValueWithUnits>,
    pub r#camera_event_type: Option<field_types::CameraEventType>,
    pub r#camera_file_uuid: Option<ValueWithUnits>,
    pub r#camera_orientation: Option<field_types::CameraOrientationType>,
    pub r#timestamp: Option<field_types::DateTime>,
    pub invalid_fields: BTreeMap<&'static str, ValueWithUnits>,
}
impl FitMessage for CameraEvent {
    const NAME: &'static str = "CameraEvent";
    const KIND: MesgNum = MesgNum::CameraEvent;
    fn parse_with_options(
        record: FitDataRecord,
        options: MessageParseOptions,
    ) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#timestamp_ms = None;
        let mut r#camera_event_type = None;
        let mut r#camera_file_uuid = None;
        let mut r#camera_orientation = None;
        let mut r#timestamp = None;
        #[allow(unused_mut)]
        let mut invalid_fields = BTreeMap::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#timestamp_ms = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("timestamp_ms", value);
                    }
                },
                1u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#camera_event_type = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("camera_event_type", value);
                    }
                },
                2u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#camera_file_uuid = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("camera_file_uuid", value);
                    }
                },
                3u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#camera_orientation = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("camera_orientation", value);
                    }
                },
                253u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#timestamp = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("timestamp", value);
                    }
                },
                _ => {
                    if !options.ignore_unexpected_fields {
                        return Err(TryFromRecordError::unexpected_field(&field));
                    }
                }
            }
        }
        Ok(Self {
            r#timestamp_ms,
            r#camera_event_type,
            r#camera_file_uuid,
            r#camera_orientation,
            r#timestamp,
            invalid_fields,
        })
    }
    fn invalid_fields(&self) -> &BTreeMap<&'static str, ValueWithUnits> {
        &self.invalid_fields
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
    pub r#timestamp_ms: Option<ValueWithUnits>,
    pub r#sample_time_offset: Option<ValueWithUnits>,
    pub r#gyro_x: Option<ValueWithUnits>,
    pub r#gyro_y: Option<ValueWithUnits>,
    pub r#gyro_z: Option<ValueWithUnits>,
    pub r#calibrated_gyro_x: Option<ValueWithUnits>,
    pub r#calibrated_gyro_y: Option<ValueWithUnits>,
    pub r#calibrated_gyro_z: Option<ValueWithUnits>,
    pub r#timestamp: Option<field_types::DateTime>,
    pub invalid_fields: BTreeMap<&'static str, ValueWithUnits>,
}
impl FitMessage for GyroscopeData {
    const NAME: &'static str = "GyroscopeData";
    const KIND: MesgNum = MesgNum::GyroscopeData;
    fn parse_with_options(
        record: FitDataRecord,
        options: MessageParseOptions,
    ) -> Result<Self, TryFromRecordError> {
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
        #[allow(unused_mut)]
        let mut invalid_fields = BTreeMap::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#timestamp_ms = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("timestamp_ms", value);
                    }
                },
                1u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#sample_time_offset = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("sample_time_offset", value);
                    }
                },
                2u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#gyro_x = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("gyro_x", value);
                    }
                },
                3u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#gyro_y = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("gyro_y", value);
                    }
                },
                4u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#gyro_z = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("gyro_z", value);
                    }
                },
                5u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#calibrated_gyro_x = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("calibrated_gyro_x", value);
                    }
                },
                6u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#calibrated_gyro_y = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("calibrated_gyro_y", value);
                    }
                },
                7u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#calibrated_gyro_z = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("calibrated_gyro_z", value);
                    }
                },
                253u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#timestamp = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("timestamp", value);
                    }
                },
                _ => {
                    if !options.ignore_unexpected_fields {
                        return Err(TryFromRecordError::unexpected_field(&field));
                    }
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
            invalid_fields,
        })
    }
    fn invalid_fields(&self) -> &BTreeMap<&'static str, ValueWithUnits> {
        &self.invalid_fields
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
    pub r#timestamp_ms: Option<ValueWithUnits>,
    pub r#sample_time_offset: Option<ValueWithUnits>,
    pub r#accel_x: Option<ValueWithUnits>,
    pub r#accel_y: Option<ValueWithUnits>,
    pub r#accel_z: Option<ValueWithUnits>,
    pub r#calibrated_accel_x: Option<ValueWithUnits>,
    pub r#calibrated_accel_y: Option<ValueWithUnits>,
    pub r#calibrated_accel_z: Option<ValueWithUnits>,
    pub r#compressed_calibrated_accel_x: Option<ValueWithUnits>,
    pub r#compressed_calibrated_accel_y: Option<ValueWithUnits>,
    pub r#compressed_calibrated_accel_z: Option<ValueWithUnits>,
    pub r#timestamp: Option<field_types::DateTime>,
    pub invalid_fields: BTreeMap<&'static str, ValueWithUnits>,
}
impl FitMessage for AccelerometerData {
    const NAME: &'static str = "AccelerometerData";
    const KIND: MesgNum = MesgNum::AccelerometerData;
    fn parse_with_options(
        record: FitDataRecord,
        options: MessageParseOptions,
    ) -> Result<Self, TryFromRecordError> {
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
        #[allow(unused_mut)]
        let mut invalid_fields = BTreeMap::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#timestamp_ms = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("timestamp_ms", value);
                    }
                },
                1u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#sample_time_offset = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("sample_time_offset", value);
                    }
                },
                2u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#accel_x = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("accel_x", value);
                    }
                },
                3u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#accel_y = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("accel_y", value);
                    }
                },
                4u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#accel_z = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("accel_z", value);
                    }
                },
                5u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#calibrated_accel_x = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("calibrated_accel_x", value);
                    }
                },
                6u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#calibrated_accel_y = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("calibrated_accel_y", value);
                    }
                },
                7u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#calibrated_accel_z = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("calibrated_accel_z", value);
                    }
                },
                8u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#compressed_calibrated_accel_x = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("compressed_calibrated_accel_x", value);
                    }
                },
                9u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#compressed_calibrated_accel_y = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("compressed_calibrated_accel_y", value);
                    }
                },
                10u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#compressed_calibrated_accel_z = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("compressed_calibrated_accel_z", value);
                    }
                },
                253u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#timestamp = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("timestamp", value);
                    }
                },
                _ => {
                    if !options.ignore_unexpected_fields {
                        return Err(TryFromRecordError::unexpected_field(&field));
                    }
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
            invalid_fields,
        })
    }
    fn invalid_fields(&self) -> &BTreeMap<&'static str, ValueWithUnits> {
        &self.invalid_fields
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
    pub r#timestamp_ms: Option<ValueWithUnits>,
    pub r#sample_time_offset: Option<ValueWithUnits>,
    pub r#mag_x: Option<ValueWithUnits>,
    pub r#mag_y: Option<ValueWithUnits>,
    pub r#mag_z: Option<ValueWithUnits>,
    pub r#calibrated_mag_x: Option<ValueWithUnits>,
    pub r#calibrated_mag_y: Option<ValueWithUnits>,
    pub r#calibrated_mag_z: Option<ValueWithUnits>,
    pub r#timestamp: Option<field_types::DateTime>,
    pub invalid_fields: BTreeMap<&'static str, ValueWithUnits>,
}
impl FitMessage for MagnetometerData {
    const NAME: &'static str = "MagnetometerData";
    const KIND: MesgNum = MesgNum::MagnetometerData;
    fn parse_with_options(
        record: FitDataRecord,
        options: MessageParseOptions,
    ) -> Result<Self, TryFromRecordError> {
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
        #[allow(unused_mut)]
        let mut invalid_fields = BTreeMap::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#timestamp_ms = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("timestamp_ms", value);
                    }
                },
                1u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#sample_time_offset = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("sample_time_offset", value);
                    }
                },
                2u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#mag_x = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("mag_x", value);
                    }
                },
                3u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#mag_y = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("mag_y", value);
                    }
                },
                4u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#mag_z = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("mag_z", value);
                    }
                },
                5u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#calibrated_mag_x = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("calibrated_mag_x", value);
                    }
                },
                6u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#calibrated_mag_y = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("calibrated_mag_y", value);
                    }
                },
                7u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#calibrated_mag_z = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("calibrated_mag_z", value);
                    }
                },
                253u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#timestamp = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("timestamp", value);
                    }
                },
                _ => {
                    if !options.ignore_unexpected_fields {
                        return Err(TryFromRecordError::unexpected_field(&field));
                    }
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
            invalid_fields,
        })
    }
    fn invalid_fields(&self) -> &BTreeMap<&'static str, ValueWithUnits> {
        &self.invalid_fields
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
    pub r#timestamp_ms: Option<ValueWithUnits>,
    pub r#sample_time_offset: Option<ValueWithUnits>,
    pub r#baro_pres: Option<ValueWithUnits>,
    pub r#timestamp: Option<field_types::DateTime>,
    pub invalid_fields: BTreeMap<&'static str, ValueWithUnits>,
}
impl FitMessage for BarometerData {
    const NAME: &'static str = "BarometerData";
    const KIND: MesgNum = MesgNum::BarometerData;
    fn parse_with_options(
        record: FitDataRecord,
        options: MessageParseOptions,
    ) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#timestamp_ms = None;
        let mut r#sample_time_offset = None;
        let mut r#baro_pres = None;
        let mut r#timestamp = None;
        #[allow(unused_mut)]
        let mut invalid_fields = BTreeMap::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#timestamp_ms = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("timestamp_ms", value);
                    }
                },
                1u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#sample_time_offset = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("sample_time_offset", value);
                    }
                },
                2u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#baro_pres = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("baro_pres", value);
                    }
                },
                253u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#timestamp = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("timestamp", value);
                    }
                },
                _ => {
                    if !options.ignore_unexpected_fields {
                        return Err(TryFromRecordError::unexpected_field(&field));
                    }
                }
            }
        }
        Ok(Self {
            r#timestamp_ms,
            r#sample_time_offset,
            r#baro_pres,
            r#timestamp,
            invalid_fields,
        })
    }
    fn invalid_fields(&self) -> &BTreeMap<&'static str, ValueWithUnits> {
        &self.invalid_fields
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
    pub r#sensor_type: Option<field_types::SensorType>,
    pub r#calibration_factor: Option<ValueWithUnits>,
    pub r#calibration_divisor: Option<ValueWithUnits>,
    pub r#level_shift: Option<ValueWithUnits>,
    pub r#offset_cal: Option<ValueWithUnits>,
    pub r#orientation_matrix: Option<ValueWithUnits>,
    pub r#timestamp: Option<field_types::DateTime>,
    pub invalid_fields: BTreeMap<&'static str, ValueWithUnits>,
}
impl FitMessage for ThreeDSensorCalibration {
    const NAME: &'static str = "ThreeDSensorCalibration";
    const KIND: MesgNum = MesgNum::ThreeDSensorCalibration;
    fn parse_with_options(
        record: FitDataRecord,
        options: MessageParseOptions,
    ) -> Result<Self, TryFromRecordError> {
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
        #[allow(unused_mut)]
        let mut invalid_fields = BTreeMap::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#sensor_type = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("sensor_type", value);
                    }
                },
                1u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#calibration_factor = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("calibration_factor", value);
                    }
                },
                2u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#calibration_divisor = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("calibration_divisor", value);
                    }
                },
                3u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#level_shift = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("level_shift", value);
                    }
                },
                4u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#offset_cal = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("offset_cal", value);
                    }
                },
                5u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#orientation_matrix = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("orientation_matrix", value);
                    }
                },
                253u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#timestamp = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("timestamp", value);
                    }
                },
                _ => {
                    if !options.ignore_unexpected_fields {
                        return Err(TryFromRecordError::unexpected_field(&field));
                    }
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
            invalid_fields,
        })
    }
    fn invalid_fields(&self) -> &BTreeMap<&'static str, ValueWithUnits> {
        &self.invalid_fields
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
    pub r#sensor_type: Option<field_types::SensorType>,
    pub r#calibration_factor: Option<ValueWithUnits>,
    pub r#calibration_divisor: Option<ValueWithUnits>,
    pub r#level_shift: Option<ValueWithUnits>,
    pub r#offset_cal: Option<ValueWithUnits>,
    pub r#timestamp: Option<field_types::DateTime>,
    pub invalid_fields: BTreeMap<&'static str, ValueWithUnits>,
}
impl FitMessage for OneDSensorCalibration {
    const NAME: &'static str = "OneDSensorCalibration";
    const KIND: MesgNum = MesgNum::OneDSensorCalibration;
    fn parse_with_options(
        record: FitDataRecord,
        options: MessageParseOptions,
    ) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#sensor_type = None;
        let mut r#calibration_factor = None;
        let mut r#calibration_divisor = None;
        let mut r#level_shift = None;
        let mut r#offset_cal = None;
        let mut r#timestamp = None;
        #[allow(unused_mut)]
        let mut invalid_fields = BTreeMap::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#sensor_type = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("sensor_type", value);
                    }
                },
                1u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#calibration_factor = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("calibration_factor", value);
                    }
                },
                2u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#calibration_divisor = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("calibration_divisor", value);
                    }
                },
                3u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#level_shift = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("level_shift", value);
                    }
                },
                4u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#offset_cal = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("offset_cal", value);
                    }
                },
                253u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#timestamp = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("timestamp", value);
                    }
                },
                _ => {
                    if !options.ignore_unexpected_fields {
                        return Err(TryFromRecordError::unexpected_field(&field));
                    }
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
            invalid_fields,
        })
    }
    fn invalid_fields(&self) -> &BTreeMap<&'static str, ValueWithUnits> {
        &self.invalid_fields
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
    pub r#timestamp_ms: Option<ValueWithUnits>,
    pub r#frame_number: Option<ValueWithUnits>,
    pub r#timestamp: Option<field_types::DateTime>,
    pub invalid_fields: BTreeMap<&'static str, ValueWithUnits>,
}
impl FitMessage for VideoFrame {
    const NAME: &'static str = "VideoFrame";
    const KIND: MesgNum = MesgNum::VideoFrame;
    fn parse_with_options(
        record: FitDataRecord,
        options: MessageParseOptions,
    ) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#timestamp_ms = None;
        let mut r#frame_number = None;
        let mut r#timestamp = None;
        #[allow(unused_mut)]
        let mut invalid_fields = BTreeMap::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#timestamp_ms = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("timestamp_ms", value);
                    }
                },
                1u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#frame_number = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("frame_number", value);
                    }
                },
                253u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#timestamp = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("timestamp", value);
                    }
                },
                _ => {
                    if !options.ignore_unexpected_fields {
                        return Err(TryFromRecordError::unexpected_field(&field));
                    }
                }
            }
        }
        Ok(Self {
            r#timestamp_ms,
            r#frame_number,
            r#timestamp,
            invalid_fields,
        })
    }
    fn invalid_fields(&self) -> &BTreeMap<&'static str, ValueWithUnits> {
        &self.invalid_fields
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
    pub r#timestamp_ms: Option<ValueWithUnits>,
    pub r#time_offset: Option<ValueWithUnits>,
    pub r#pid: Option<ValueWithUnits>,
    pub r#raw_data: Option<ValueWithUnits>,
    pub r#pid_data_size: Option<ValueWithUnits>,
    pub r#system_time: Option<ValueWithUnits>,
    pub r#start_timestamp: Option<field_types::DateTime>,
    pub r#start_timestamp_ms: Option<ValueWithUnits>,
    pub r#timestamp: Option<field_types::DateTime>,
    pub invalid_fields: BTreeMap<&'static str, ValueWithUnits>,
}
impl FitMessage for ObdiiData {
    const NAME: &'static str = "ObdiiData";
    const KIND: MesgNum = MesgNum::ObdiiData;
    fn parse_with_options(
        record: FitDataRecord,
        options: MessageParseOptions,
    ) -> Result<Self, TryFromRecordError> {
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
        #[allow(unused_mut)]
        let mut invalid_fields = BTreeMap::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#timestamp_ms = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("timestamp_ms", value);
                    }
                },
                1u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#time_offset = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("time_offset", value);
                    }
                },
                2u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#pid = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("pid", value);
                    }
                },
                3u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#raw_data = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("raw_data", value);
                    }
                },
                4u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#pid_data_size = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("pid_data_size", value);
                    }
                },
                5u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#system_time = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("system_time", value);
                    }
                },
                6u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#start_timestamp = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("start_timestamp", value);
                    }
                },
                7u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#start_timestamp_ms = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("start_timestamp_ms", value);
                    }
                },
                253u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#timestamp = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("timestamp", value);
                    }
                },
                _ => {
                    if !options.ignore_unexpected_fields {
                        return Err(TryFromRecordError::unexpected_field(&field));
                    }
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
            invalid_fields,
        })
    }
    fn invalid_fields(&self) -> &BTreeMap<&'static str, ValueWithUnits> {
        &self.invalid_fields
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
    pub r#timestamp_ms: Option<ValueWithUnits>,
    pub r#sentence: Option<ValueWithUnits>,
    pub r#timestamp: Option<field_types::DateTime>,
    pub invalid_fields: BTreeMap<&'static str, ValueWithUnits>,
}
impl FitMessage for NmeaSentence {
    const NAME: &'static str = "NmeaSentence";
    const KIND: MesgNum = MesgNum::NmeaSentence;
    fn parse_with_options(
        record: FitDataRecord,
        options: MessageParseOptions,
    ) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#timestamp_ms = None;
        let mut r#sentence = None;
        let mut r#timestamp = None;
        #[allow(unused_mut)]
        let mut invalid_fields = BTreeMap::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#timestamp_ms = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("timestamp_ms", value);
                    }
                },
                1u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#sentence = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("sentence", value);
                    }
                },
                253u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#timestamp = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("timestamp", value);
                    }
                },
                _ => {
                    if !options.ignore_unexpected_fields {
                        return Err(TryFromRecordError::unexpected_field(&field));
                    }
                }
            }
        }
        Ok(Self {
            r#timestamp_ms,
            r#sentence,
            r#timestamp,
            invalid_fields,
        })
    }
    fn invalid_fields(&self) -> &BTreeMap<&'static str, ValueWithUnits> {
        &self.invalid_fields
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
    pub r#timestamp_ms: Option<ValueWithUnits>,
    pub r#system_time: Option<ValueWithUnits>,
    pub r#pitch: Option<ValueWithUnits>,
    pub r#roll: Option<ValueWithUnits>,
    pub r#accel_lateral: Option<ValueWithUnits>,
    pub r#accel_normal: Option<ValueWithUnits>,
    pub r#turn_rate: Option<ValueWithUnits>,
    pub r#stage: Option<ValueWithUnits>,
    pub r#attitude_stage_complete: Option<ValueWithUnits>,
    pub r#track: Option<ValueWithUnits>,
    pub r#validity: Option<ValueWithUnits>,
    pub r#timestamp: Option<field_types::DateTime>,
    pub invalid_fields: BTreeMap<&'static str, ValueWithUnits>,
}
impl FitMessage for AviationAttitude {
    const NAME: &'static str = "AviationAttitude";
    const KIND: MesgNum = MesgNum::AviationAttitude;
    fn parse_with_options(
        record: FitDataRecord,
        options: MessageParseOptions,
    ) -> Result<Self, TryFromRecordError> {
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
        #[allow(unused_mut)]
        let mut invalid_fields = BTreeMap::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#timestamp_ms = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("timestamp_ms", value);
                    }
                },
                1u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#system_time = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("system_time", value);
                    }
                },
                2u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#pitch = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("pitch", value);
                    }
                },
                3u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#roll = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("roll", value);
                    }
                },
                4u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#accel_lateral = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("accel_lateral", value);
                    }
                },
                5u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#accel_normal = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("accel_normal", value);
                    }
                },
                6u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#turn_rate = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("turn_rate", value);
                    }
                },
                7u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#stage = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("stage", value);
                    }
                },
                8u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#attitude_stage_complete = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("attitude_stage_complete", value);
                    }
                },
                9u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#track = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("track", value);
                    }
                },
                10u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#validity = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("validity", value);
                    }
                },
                253u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#timestamp = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("timestamp", value);
                    }
                },
                _ => {
                    if !options.ignore_unexpected_fields {
                        return Err(TryFromRecordError::unexpected_field(&field));
                    }
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
            invalid_fields,
        })
    }
    fn invalid_fields(&self) -> &BTreeMap<&'static str, ValueWithUnits> {
        &self.invalid_fields
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
    pub r#url: Option<ValueWithUnits>,
    pub r#hosting_provider: Option<ValueWithUnits>,
    pub r#duration: Option<ValueWithUnits>,
    pub invalid_fields: BTreeMap<&'static str, ValueWithUnits>,
}
impl FitMessage for Video {
    const NAME: &'static str = "Video";
    const KIND: MesgNum = MesgNum::Video;
    fn parse_with_options(
        record: FitDataRecord,
        options: MessageParseOptions,
    ) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#url = None;
        let mut r#hosting_provider = None;
        let mut r#duration = None;
        #[allow(unused_mut)]
        let mut invalid_fields = BTreeMap::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#url = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("url", value);
                    }
                },
                1u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#hosting_provider = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("hosting_provider", value);
                    }
                },
                2u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#duration = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("duration", value);
                    }
                },
                _ => {
                    if !options.ignore_unexpected_fields {
                        return Err(TryFromRecordError::unexpected_field(&field));
                    }
                }
            }
        }
        Ok(Self {
            r#url,
            r#hosting_provider,
            r#duration,
            invalid_fields,
        })
    }
    fn invalid_fields(&self) -> &BTreeMap<&'static str, ValueWithUnits> {
        &self.invalid_fields
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
    pub r#message_count: Option<ValueWithUnits>,
    pub r#text: Option<ValueWithUnits>,
    pub r#message_index: Option<field_types::MessageIndex>,
    pub invalid_fields: BTreeMap<&'static str, ValueWithUnits>,
}
impl FitMessage for VideoTitle {
    const NAME: &'static str = "VideoTitle";
    const KIND: MesgNum = MesgNum::VideoTitle;
    fn parse_with_options(
        record: FitDataRecord,
        options: MessageParseOptions,
    ) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#message_count = None;
        let mut r#text = None;
        let mut r#message_index = None;
        #[allow(unused_mut)]
        let mut invalid_fields = BTreeMap::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#message_count = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("message_count", value);
                    }
                },
                1u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#text = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("text", value);
                    }
                },
                254u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#message_index = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("message_index", value);
                    }
                },
                _ => {
                    if !options.ignore_unexpected_fields {
                        return Err(TryFromRecordError::unexpected_field(&field));
                    }
                }
            }
        }
        Ok(Self {
            r#message_count,
            r#text,
            r#message_index,
            invalid_fields,
        })
    }
    fn invalid_fields(&self) -> &BTreeMap<&'static str, ValueWithUnits> {
        &self.invalid_fields
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
    pub r#message_count: Option<ValueWithUnits>,
    pub r#text: Option<ValueWithUnits>,
    pub r#message_index: Option<field_types::MessageIndex>,
    pub invalid_fields: BTreeMap<&'static str, ValueWithUnits>,
}
impl FitMessage for VideoDescription {
    const NAME: &'static str = "VideoDescription";
    const KIND: MesgNum = MesgNum::VideoDescription;
    fn parse_with_options(
        record: FitDataRecord,
        options: MessageParseOptions,
    ) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#message_count = None;
        let mut r#text = None;
        let mut r#message_index = None;
        #[allow(unused_mut)]
        let mut invalid_fields = BTreeMap::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#message_count = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("message_count", value);
                    }
                },
                1u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#text = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("text", value);
                    }
                },
                254u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#message_index = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("message_index", value);
                    }
                },
                _ => {
                    if !options.ignore_unexpected_fields {
                        return Err(TryFromRecordError::unexpected_field(&field));
                    }
                }
            }
        }
        Ok(Self {
            r#message_count,
            r#text,
            r#message_index,
            invalid_fields,
        })
    }
    fn invalid_fields(&self) -> &BTreeMap<&'static str, ValueWithUnits> {
        &self.invalid_fields
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
    pub r#clip_number: Option<ValueWithUnits>,
    pub r#start_timestamp: Option<field_types::DateTime>,
    pub r#start_timestamp_ms: Option<ValueWithUnits>,
    pub r#end_timestamp: Option<field_types::DateTime>,
    pub r#end_timestamp_ms: Option<ValueWithUnits>,
    pub r#clip_start: Option<ValueWithUnits>,
    pub r#clip_end: Option<ValueWithUnits>,
    pub invalid_fields: BTreeMap<&'static str, ValueWithUnits>,
}
impl FitMessage for VideoClip {
    const NAME: &'static str = "VideoClip";
    const KIND: MesgNum = MesgNum::VideoClip;
    fn parse_with_options(
        record: FitDataRecord,
        options: MessageParseOptions,
    ) -> Result<Self, TryFromRecordError> {
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
        #[allow(unused_mut)]
        let mut invalid_fields = BTreeMap::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#clip_number = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("clip_number", value);
                    }
                },
                1u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#start_timestamp = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("start_timestamp", value);
                    }
                },
                2u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#start_timestamp_ms = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("start_timestamp_ms", value);
                    }
                },
                3u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#end_timestamp = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("end_timestamp", value);
                    }
                },
                4u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#end_timestamp_ms = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("end_timestamp_ms", value);
                    }
                },
                6u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#clip_start = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("clip_start", value);
                    }
                },
                7u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#clip_end = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("clip_end", value);
                    }
                },
                _ => {
                    if !options.ignore_unexpected_fields {
                        return Err(TryFromRecordError::unexpected_field(&field));
                    }
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
            invalid_fields,
        })
    }
    fn invalid_fields(&self) -> &BTreeMap<&'static str, ValueWithUnits> {
        &self.invalid_fields
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
    pub r#duration: Option<ValueWithUnits>,
    pub r#repetitions: Option<ValueWithUnits>,
    pub r#weight: Option<ValueWithUnits>,
    pub r#set_type: Option<field_types::SetType>,
    pub r#start_time: Option<field_types::DateTime>,
    pub r#category: Option<ValueWithUnits>,
    pub r#category_subtype: Option<ValueWithUnits>,
    pub r#weight_display_unit: Option<field_types::FitBaseUnit>,
    pub r#message_index: Option<field_types::MessageIndex>,
    pub r#wkt_step_index: Option<field_types::MessageIndex>,
    pub r#timestamp: Option<field_types::DateTime>,
    pub invalid_fields: BTreeMap<&'static str, ValueWithUnits>,
}
impl FitMessage for Set {
    const NAME: &'static str = "Set";
    const KIND: MesgNum = MesgNum::Set;
    fn parse_with_options(
        record: FitDataRecord,
        options: MessageParseOptions,
    ) -> Result<Self, TryFromRecordError> {
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
        #[allow(unused_mut)]
        let mut invalid_fields = BTreeMap::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#duration = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("duration", value);
                    }
                },
                3u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#repetitions = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("repetitions", value);
                    }
                },
                4u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#weight = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("weight", value);
                    }
                },
                5u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#set_type = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("set_type", value);
                    }
                },
                6u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#start_time = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("start_time", value);
                    }
                },
                7u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#category = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("category", value);
                    }
                },
                8u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#category_subtype = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("category_subtype", value);
                    }
                },
                9u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#weight_display_unit = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("weight_display_unit", value);
                    }
                },
                10u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#message_index = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("message_index", value);
                    }
                },
                11u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#wkt_step_index = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("wkt_step_index", value);
                    }
                },
                254u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#timestamp = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("timestamp", value);
                    }
                },
                _ => {
                    if !options.ignore_unexpected_fields {
                        return Err(TryFromRecordError::unexpected_field(&field));
                    }
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
            invalid_fields,
        })
    }
    fn invalid_fields(&self) -> &BTreeMap<&'static str, ValueWithUnits> {
        &self.invalid_fields
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
    pub r#distance: Option<ValueWithUnits>,
    pub r#height: Option<ValueWithUnits>,
    pub r#rotations: Option<ValueWithUnits>,
    pub r#hang_time: Option<ValueWithUnits>,
    pub r#score: Option<ValueWithUnits>,
    pub r#position_lat: Option<ValueWithUnits>,
    pub r#position_long: Option<ValueWithUnits>,
    pub r#speed: Option<ValueWithUnits>,
    pub r#enhanced_speed: Option<ValueWithUnits>,
    pub r#timestamp: Option<field_types::DateTime>,
    pub invalid_fields: BTreeMap<&'static str, ValueWithUnits>,
}
impl FitMessage for Jump {
    const NAME: &'static str = "Jump";
    const KIND: MesgNum = MesgNum::Jump;
    fn parse_with_options(
        record: FitDataRecord,
        options: MessageParseOptions,
    ) -> Result<Self, TryFromRecordError> {
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
        #[allow(unused_mut)]
        let mut invalid_fields = BTreeMap::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#distance = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("distance", value);
                    }
                },
                1u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#height = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("height", value);
                    }
                },
                2u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#rotations = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("rotations", value);
                    }
                },
                3u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#hang_time = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("hang_time", value);
                    }
                },
                4u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#score = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("score", value);
                    }
                },
                5u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#position_lat = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("position_lat", value);
                    }
                },
                6u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#position_long = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("position_long", value);
                    }
                },
                7u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#speed = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("speed", value);
                    }
                },
                8u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#enhanced_speed = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("enhanced_speed", value);
                    }
                },
                253u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#timestamp = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("timestamp", value);
                    }
                },
                _ => {
                    if !options.ignore_unexpected_fields {
                        return Err(TryFromRecordError::unexpected_field(&field));
                    }
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
            invalid_fields,
        })
    }
    fn invalid_fields(&self) -> &BTreeMap<&'static str, ValueWithUnits> {
        &self.invalid_fields
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
    pub r#split_type: Option<field_types::SplitType>,
    pub r#total_elapsed_time: Option<ValueWithUnits>,
    pub r#total_timer_time: Option<ValueWithUnits>,
    pub r#total_distance: Option<ValueWithUnits>,
    pub r#avg_speed: Option<ValueWithUnits>,
    pub r#start_time: Option<field_types::DateTime>,
    pub r#total_ascent: Option<ValueWithUnits>,
    pub r#total_descent: Option<ValueWithUnits>,
    pub r#start_position_lat: Option<ValueWithUnits>,
    pub r#start_position_long: Option<ValueWithUnits>,
    pub r#end_position_lat: Option<ValueWithUnits>,
    pub r#end_position_long: Option<ValueWithUnits>,
    pub r#max_speed: Option<ValueWithUnits>,
    pub r#avg_vert_speed: Option<ValueWithUnits>,
    pub r#end_time: Option<field_types::DateTime>,
    pub r#total_calories: Option<ValueWithUnits>,
    pub r#start_elevation: Option<ValueWithUnits>,
    pub r#total_moving_time: Option<ValueWithUnits>,
    pub r#message_index: Option<field_types::MessageIndex>,
    pub invalid_fields: BTreeMap<&'static str, ValueWithUnits>,
}
impl FitMessage for Split {
    const NAME: &'static str = "Split";
    const KIND: MesgNum = MesgNum::Split;
    fn parse_with_options(
        record: FitDataRecord,
        options: MessageParseOptions,
    ) -> Result<Self, TryFromRecordError> {
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
        #[allow(unused_mut)]
        let mut invalid_fields = BTreeMap::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#split_type = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("split_type", value);
                    }
                },
                1u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#total_elapsed_time = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("total_elapsed_time", value);
                    }
                },
                2u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#total_timer_time = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("total_timer_time", value);
                    }
                },
                3u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#total_distance = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("total_distance", value);
                    }
                },
                4u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#avg_speed = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("avg_speed", value);
                    }
                },
                9u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#start_time = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("start_time", value);
                    }
                },
                13u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#total_ascent = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("total_ascent", value);
                    }
                },
                14u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#total_descent = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("total_descent", value);
                    }
                },
                21u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#start_position_lat = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("start_position_lat", value);
                    }
                },
                22u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#start_position_long = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("start_position_long", value);
                    }
                },
                23u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#end_position_lat = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("end_position_lat", value);
                    }
                },
                24u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#end_position_long = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("end_position_long", value);
                    }
                },
                25u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#max_speed = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("max_speed", value);
                    }
                },
                26u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#avg_vert_speed = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("avg_vert_speed", value);
                    }
                },
                27u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#end_time = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("end_time", value);
                    }
                },
                28u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#total_calories = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("total_calories", value);
                    }
                },
                74u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#start_elevation = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("start_elevation", value);
                    }
                },
                110u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#total_moving_time = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("total_moving_time", value);
                    }
                },
                254u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#message_index = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("message_index", value);
                    }
                },
                _ => {
                    if !options.ignore_unexpected_fields {
                        return Err(TryFromRecordError::unexpected_field(&field));
                    }
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
            invalid_fields,
        })
    }
    fn invalid_fields(&self) -> &BTreeMap<&'static str, ValueWithUnits> {
        &self.invalid_fields
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
    pub r#split_type: Option<field_types::SplitType>,
    pub r#num_splits: Option<ValueWithUnits>,
    pub r#total_timer_time: Option<ValueWithUnits>,
    pub r#total_distance: Option<ValueWithUnits>,
    pub r#avg_speed: Option<ValueWithUnits>,
    pub r#max_speed: Option<ValueWithUnits>,
    pub r#total_ascent: Option<ValueWithUnits>,
    pub r#total_descent: Option<ValueWithUnits>,
    pub r#avg_heart_rate: Option<ValueWithUnits>,
    pub r#max_heart_rate: Option<ValueWithUnits>,
    pub r#avg_vert_speed: Option<ValueWithUnits>,
    pub r#total_calories: Option<ValueWithUnits>,
    pub r#total_moving_time: Option<ValueWithUnits>,
    pub r#message_index: Option<field_types::MessageIndex>,
    pub invalid_fields: BTreeMap<&'static str, ValueWithUnits>,
}
impl FitMessage for SplitSummary {
    const NAME: &'static str = "SplitSummary";
    const KIND: MesgNum = MesgNum::SplitSummary;
    fn parse_with_options(
        record: FitDataRecord,
        options: MessageParseOptions,
    ) -> Result<Self, TryFromRecordError> {
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
        #[allow(unused_mut)]
        let mut invalid_fields = BTreeMap::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#split_type = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("split_type", value);
                    }
                },
                3u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#num_splits = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("num_splits", value);
                    }
                },
                4u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#total_timer_time = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("total_timer_time", value);
                    }
                },
                5u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#total_distance = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("total_distance", value);
                    }
                },
                6u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#avg_speed = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("avg_speed", value);
                    }
                },
                7u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#max_speed = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("max_speed", value);
                    }
                },
                8u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#total_ascent = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("total_ascent", value);
                    }
                },
                9u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#total_descent = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("total_descent", value);
                    }
                },
                10u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#avg_heart_rate = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("avg_heart_rate", value);
                    }
                },
                11u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#max_heart_rate = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("max_heart_rate", value);
                    }
                },
                12u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#avg_vert_speed = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("avg_vert_speed", value);
                    }
                },
                13u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#total_calories = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("total_calories", value);
                    }
                },
                77u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#total_moving_time = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("total_moving_time", value);
                    }
                },
                254u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#message_index = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("message_index", value);
                    }
                },
                _ => {
                    if !options.ignore_unexpected_fields {
                        return Err(TryFromRecordError::unexpected_field(&field));
                    }
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
            invalid_fields,
        })
    }
    fn invalid_fields(&self) -> &BTreeMap<&'static str, ValueWithUnits> {
        &self.invalid_fields
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
    pub r#position_lat: Option<ValueWithUnits>,
    pub r#position_long: Option<ValueWithUnits>,
    pub r#climb_pro_event: Option<field_types::ClimbProEvent>,
    pub r#climb_number: Option<ValueWithUnits>,
    pub r#climb_category: Option<ValueWithUnits>,
    pub r#current_dist: Option<ValueWithUnits>,
    pub r#timestamp: Option<field_types::DateTime>,
    pub invalid_fields: BTreeMap<&'static str, ValueWithUnits>,
}
impl FitMessage for ClimbPro {
    const NAME: &'static str = "ClimbPro";
    const KIND: MesgNum = MesgNum::ClimbPro;
    fn parse_with_options(
        record: FitDataRecord,
        options: MessageParseOptions,
    ) -> Result<Self, TryFromRecordError> {
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
        #[allow(unused_mut)]
        let mut invalid_fields = BTreeMap::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#position_lat = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("position_lat", value);
                    }
                },
                1u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#position_long = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("position_long", value);
                    }
                },
                2u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#climb_pro_event = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("climb_pro_event", value);
                    }
                },
                3u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#climb_number = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("climb_number", value);
                    }
                },
                4u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#climb_category = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("climb_category", value);
                    }
                },
                5u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#current_dist = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("current_dist", value);
                    }
                },
                253u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#timestamp = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("timestamp", value);
                    }
                },
                _ => {
                    if !options.ignore_unexpected_fields {
                        return Err(TryFromRecordError::unexpected_field(&field));
                    }
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
            invalid_fields,
        })
    }
    fn invalid_fields(&self) -> &BTreeMap<&'static str, ValueWithUnits> {
        &self.invalid_fields
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
    pub r#developer_data_index: Option<ValueWithUnits>,
    pub r#field_definition_number: Option<ValueWithUnits>,
    pub r#fit_base_type_id: Option<field_types::FitBaseType>,
    pub r#field_name: Option<ValueWithUnits>,
    pub r#array: Option<ValueWithUnits>,
    pub r#components: Option<ValueWithUnits>,
    pub r#scale: Option<ValueWithUnits>,
    pub r#offset: Option<ValueWithUnits>,
    pub r#units: Option<ValueWithUnits>,
    pub r#bits: Option<ValueWithUnits>,
    pub r#accumulate: Option<ValueWithUnits>,
    pub r#fit_base_unit_id: Option<field_types::FitBaseUnit>,
    pub r#native_mesg_num: Option<field_types::MesgNum>,
    pub r#native_field_num: Option<ValueWithUnits>,
    pub invalid_fields: BTreeMap<&'static str, ValueWithUnits>,
}
impl FitMessage for FieldDescription {
    const NAME: &'static str = "FieldDescription";
    const KIND: MesgNum = MesgNum::FieldDescription;
    fn parse_with_options(
        record: FitDataRecord,
        options: MessageParseOptions,
    ) -> Result<Self, TryFromRecordError> {
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
        #[allow(unused_mut)]
        let mut invalid_fields = BTreeMap::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#developer_data_index = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("developer_data_index", value);
                    }
                },
                1u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#field_definition_number = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("field_definition_number", value);
                    }
                },
                2u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#fit_base_type_id = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("fit_base_type_id", value);
                    }
                },
                3u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#field_name = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("field_name", value);
                    }
                },
                4u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#array = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("array", value);
                    }
                },
                5u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#components = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("components", value);
                    }
                },
                6u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#scale = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("scale", value);
                    }
                },
                7u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#offset = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("offset", value);
                    }
                },
                8u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#units = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("units", value);
                    }
                },
                9u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#bits = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("bits", value);
                    }
                },
                10u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#accumulate = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("accumulate", value);
                    }
                },
                13u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#fit_base_unit_id = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("fit_base_unit_id", value);
                    }
                },
                14u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#native_mesg_num = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("native_mesg_num", value);
                    }
                },
                15u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#native_field_num = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("native_field_num", value);
                    }
                },
                _ => {
                    if !options.ignore_unexpected_fields {
                        return Err(TryFromRecordError::unexpected_field(&field));
                    }
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
            invalid_fields,
        })
    }
    fn invalid_fields(&self) -> &BTreeMap<&'static str, ValueWithUnits> {
        &self.invalid_fields
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
    pub r#developer_id: Option<ValueWithUnits>,
    pub r#application_id: Option<ValueWithUnits>,
    pub r#manufacturer_id: Option<field_types::Manufacturer>,
    pub r#developer_data_index: Option<ValueWithUnits>,
    pub r#application_version: Option<ValueWithUnits>,
    pub invalid_fields: BTreeMap<&'static str, ValueWithUnits>,
}
impl FitMessage for DeveloperDataId {
    const NAME: &'static str = "DeveloperDataId";
    const KIND: MesgNum = MesgNum::DeveloperDataId;
    fn parse_with_options(
        record: FitDataRecord,
        options: MessageParseOptions,
    ) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#developer_id = None;
        let mut r#application_id = None;
        let mut r#manufacturer_id = None;
        let mut r#developer_data_index = None;
        let mut r#application_version = None;
        #[allow(unused_mut)]
        let mut invalid_fields = BTreeMap::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#developer_id = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("developer_id", value);
                    }
                },
                1u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#application_id = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("application_id", value);
                    }
                },
                2u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#manufacturer_id = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("manufacturer_id", value);
                    }
                },
                3u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#developer_data_index = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("developer_data_index", value);
                    }
                },
                4u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#application_version = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("application_version", value);
                    }
                },
                _ => {
                    if !options.ignore_unexpected_fields {
                        return Err(TryFromRecordError::unexpected_field(&field));
                    }
                }
            }
        }
        Ok(Self {
            r#developer_id,
            r#application_id,
            r#manufacturer_id,
            r#developer_data_index,
            r#application_version,
            invalid_fields,
        })
    }
    fn invalid_fields(&self) -> &BTreeMap<&'static str, ValueWithUnits> {
        &self.invalid_fields
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
    pub r#sport: Option<field_types::Sport>,
    pub r#name: Option<ValueWithUnits>,
    pub r#capabilities: Option<field_types::CourseCapabilities>,
    pub r#sub_sport: Option<field_types::SubSport>,
    pub invalid_fields: BTreeMap<&'static str, ValueWithUnits>,
}
impl FitMessage for Course {
    const NAME: &'static str = "Course";
    const KIND: MesgNum = MesgNum::Course;
    fn parse_with_options(
        record: FitDataRecord,
        options: MessageParseOptions,
    ) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#sport = None;
        let mut r#name = None;
        let mut r#capabilities = None;
        let mut r#sub_sport = None;
        #[allow(unused_mut)]
        let mut invalid_fields = BTreeMap::new();
        for field in record.into_vec() {
            match field.number() {
                4u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#sport = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("sport", value);
                    }
                },
                5u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#name = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("name", value);
                    }
                },
                6u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#capabilities = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("capabilities", value);
                    }
                },
                7u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#sub_sport = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("sub_sport", value);
                    }
                },
                _ => {
                    if !options.ignore_unexpected_fields {
                        return Err(TryFromRecordError::unexpected_field(&field));
                    }
                }
            }
        }
        Ok(Self {
            r#sport,
            r#name,
            r#capabilities,
            r#sub_sport,
            invalid_fields,
        })
    }
    fn invalid_fields(&self) -> &BTreeMap<&'static str, ValueWithUnits> {
        &self.invalid_fields
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
    pub r#timestamp: Option<field_types::DateTime>,
    pub r#position_lat: Option<ValueWithUnits>,
    pub r#position_long: Option<ValueWithUnits>,
    pub r#distance: Option<ValueWithUnits>,
    pub r#type: Option<field_types::CoursePoint>,
    pub r#name: Option<ValueWithUnits>,
    pub r#favorite: Option<ValueWithUnits>,
    pub r#message_index: Option<field_types::MessageIndex>,
    pub invalid_fields: BTreeMap<&'static str, ValueWithUnits>,
}
impl FitMessage for CoursePoint {
    const NAME: &'static str = "CoursePoint";
    const KIND: MesgNum = MesgNum::CoursePoint;
    fn parse_with_options(
        record: FitDataRecord,
        options: MessageParseOptions,
    ) -> Result<Self, TryFromRecordError> {
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
        #[allow(unused_mut)]
        let mut invalid_fields = BTreeMap::new();
        for field in record.into_vec() {
            match field.number() {
                1u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#timestamp = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("timestamp", value);
                    }
                },
                2u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#position_lat = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("position_lat", value);
                    }
                },
                3u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#position_long = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("position_long", value);
                    }
                },
                4u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#distance = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("distance", value);
                    }
                },
                5u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#type = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("type", value);
                    }
                },
                6u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#name = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("name", value);
                    }
                },
                8u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#favorite = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("favorite", value);
                    }
                },
                254u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#message_index = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("message_index", value);
                    }
                },
                _ => {
                    if !options.ignore_unexpected_fields {
                        return Err(TryFromRecordError::unexpected_field(&field));
                    }
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
            invalid_fields,
        })
    }
    fn invalid_fields(&self) -> &BTreeMap<&'static str, ValueWithUnits> {
        &self.invalid_fields
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
    pub r#name: Option<ValueWithUnits>,
    pub r#uuid: Option<ValueWithUnits>,
    pub r#sport: Option<field_types::Sport>,
    pub r#enabled: Option<ValueWithUnits>,
    pub r#user_profile_primary_key: Option<ValueWithUnits>,
    pub r#device_id: Option<ValueWithUnits>,
    pub r#default_race_leader: Option<ValueWithUnits>,
    pub r#delete_status: Option<field_types::SegmentDeleteStatus>,
    pub r#selection_type: Option<field_types::SegmentSelectionType>,
    pub invalid_fields: BTreeMap<&'static str, ValueWithUnits>,
}
impl FitMessage for SegmentId {
    const NAME: &'static str = "SegmentId";
    const KIND: MesgNum = MesgNum::SegmentId;
    fn parse_with_options(
        record: FitDataRecord,
        options: MessageParseOptions,
    ) -> Result<Self, TryFromRecordError> {
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
        #[allow(unused_mut)]
        let mut invalid_fields = BTreeMap::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#name = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("name", value);
                    }
                },
                1u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#uuid = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("uuid", value);
                    }
                },
                2u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#sport = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("sport", value);
                    }
                },
                3u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#enabled = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("enabled", value);
                    }
                },
                4u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#user_profile_primary_key = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("user_profile_primary_key", value);
                    }
                },
                5u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#device_id = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("device_id", value);
                    }
                },
                6u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#default_race_leader = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("default_race_leader", value);
                    }
                },
                7u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#delete_status = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("delete_status", value);
                    }
                },
                8u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#selection_type = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("selection_type", value);
                    }
                },
                _ => {
                    if !options.ignore_unexpected_fields {
                        return Err(TryFromRecordError::unexpected_field(&field));
                    }
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
            invalid_fields,
        })
    }
    fn invalid_fields(&self) -> &BTreeMap<&'static str, ValueWithUnits> {
        &self.invalid_fields
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
    pub r#name: Option<ValueWithUnits>,
    pub r#type: Option<field_types::SegmentLeaderboardType>,
    pub r#group_primary_key: Option<ValueWithUnits>,
    pub r#activity_id: Option<ValueWithUnits>,
    pub r#segment_time: Option<ValueWithUnits>,
    pub r#activity_id_string: Option<ValueWithUnits>,
    pub r#message_index: Option<field_types::MessageIndex>,
    pub invalid_fields: BTreeMap<&'static str, ValueWithUnits>,
}
impl FitMessage for SegmentLeaderboardEntry {
    const NAME: &'static str = "SegmentLeaderboardEntry";
    const KIND: MesgNum = MesgNum::SegmentLeaderboardEntry;
    fn parse_with_options(
        record: FitDataRecord,
        options: MessageParseOptions,
    ) -> Result<Self, TryFromRecordError> {
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
        #[allow(unused_mut)]
        let mut invalid_fields = BTreeMap::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#name = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("name", value);
                    }
                },
                1u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#type = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("type", value);
                    }
                },
                2u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#group_primary_key = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("group_primary_key", value);
                    }
                },
                3u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#activity_id = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("activity_id", value);
                    }
                },
                4u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#segment_time = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("segment_time", value);
                    }
                },
                5u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#activity_id_string = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("activity_id_string", value);
                    }
                },
                254u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#message_index = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("message_index", value);
                    }
                },
                _ => {
                    if !options.ignore_unexpected_fields {
                        return Err(TryFromRecordError::unexpected_field(&field));
                    }
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
            invalid_fields,
        })
    }
    fn invalid_fields(&self) -> &BTreeMap<&'static str, ValueWithUnits> {
        &self.invalid_fields
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
    pub r#position_lat: Option<ValueWithUnits>,
    pub r#position_long: Option<ValueWithUnits>,
    pub r#distance: Option<ValueWithUnits>,
    pub r#altitude: Option<ValueWithUnits>,
    pub r#leader_time: Option<ValueWithUnits>,
    pub r#enhanced_altitude: Option<ValueWithUnits>,
    pub r#message_index: Option<field_types::MessageIndex>,
    pub invalid_fields: BTreeMap<&'static str, ValueWithUnits>,
}
impl FitMessage for SegmentPoint {
    const NAME: &'static str = "SegmentPoint";
    const KIND: MesgNum = MesgNum::SegmentPoint;
    fn parse_with_options(
        record: FitDataRecord,
        options: MessageParseOptions,
    ) -> Result<Self, TryFromRecordError> {
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
        #[allow(unused_mut)]
        let mut invalid_fields = BTreeMap::new();
        for field in record.into_vec() {
            match field.number() {
                1u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#position_lat = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("position_lat", value);
                    }
                },
                2u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#position_long = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("position_long", value);
                    }
                },
                3u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#distance = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("distance", value);
                    }
                },
                4u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#altitude = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("altitude", value);
                    }
                },
                5u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#leader_time = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("leader_time", value);
                    }
                },
                6u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#enhanced_altitude = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("enhanced_altitude", value);
                    }
                },
                254u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#message_index = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("message_index", value);
                    }
                },
                _ => {
                    if !options.ignore_unexpected_fields {
                        return Err(TryFromRecordError::unexpected_field(&field));
                    }
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
            invalid_fields,
        })
    }
    fn invalid_fields(&self) -> &BTreeMap<&'static str, ValueWithUnits> {
        &self.invalid_fields
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
    pub r#event: Option<field_types::Event>,
    pub r#event_type: Option<field_types::EventType>,
    pub r#start_time: Option<field_types::DateTime>,
    pub r#start_position_lat: Option<ValueWithUnits>,
    pub r#start_position_long: Option<ValueWithUnits>,
    pub r#end_position_lat: Option<ValueWithUnits>,
    pub r#end_position_long: Option<ValueWithUnits>,
    pub r#total_elapsed_time: Option<ValueWithUnits>,
    pub r#total_timer_time: Option<ValueWithUnits>,
    pub r#total_distance: Option<ValueWithUnits>,
    pub r#total_cycles: Option<ValueWithUnits>,
    pub r#total_calories: Option<ValueWithUnits>,
    pub r#total_fat_calories: Option<ValueWithUnits>,
    pub r#avg_speed: Option<ValueWithUnits>,
    pub r#max_speed: Option<ValueWithUnits>,
    pub r#avg_heart_rate: Option<ValueWithUnits>,
    pub r#max_heart_rate: Option<ValueWithUnits>,
    pub r#avg_cadence: Option<ValueWithUnits>,
    pub r#max_cadence: Option<ValueWithUnits>,
    pub r#avg_power: Option<ValueWithUnits>,
    pub r#max_power: Option<ValueWithUnits>,
    pub r#total_ascent: Option<ValueWithUnits>,
    pub r#total_descent: Option<ValueWithUnits>,
    pub r#sport: Option<field_types::Sport>,
    pub r#event_group: Option<ValueWithUnits>,
    pub r#nec_lat: Option<ValueWithUnits>,
    pub r#nec_long: Option<ValueWithUnits>,
    pub r#swc_lat: Option<ValueWithUnits>,
    pub r#swc_long: Option<ValueWithUnits>,
    pub r#name: Option<ValueWithUnits>,
    pub r#normalized_power: Option<ValueWithUnits>,
    pub r#left_right_balance: Option<field_types::LeftRightBalance100>,
    pub r#sub_sport: Option<field_types::SubSport>,
    pub r#total_work: Option<ValueWithUnits>,
    pub r#avg_altitude: Option<ValueWithUnits>,
    pub r#max_altitude: Option<ValueWithUnits>,
    pub r#gps_accuracy: Option<ValueWithUnits>,
    pub r#avg_grade: Option<ValueWithUnits>,
    pub r#avg_pos_grade: Option<ValueWithUnits>,
    pub r#avg_neg_grade: Option<ValueWithUnits>,
    pub r#max_pos_grade: Option<ValueWithUnits>,
    pub r#max_neg_grade: Option<ValueWithUnits>,
    pub r#avg_temperature: Option<ValueWithUnits>,
    pub r#max_temperature: Option<ValueWithUnits>,
    pub r#total_moving_time: Option<ValueWithUnits>,
    pub r#avg_pos_vertical_speed: Option<ValueWithUnits>,
    pub r#avg_neg_vertical_speed: Option<ValueWithUnits>,
    pub r#max_pos_vertical_speed: Option<ValueWithUnits>,
    pub r#max_neg_vertical_speed: Option<ValueWithUnits>,
    pub r#time_in_hr_zone: Option<ValueWithUnits>,
    pub r#time_in_speed_zone: Option<ValueWithUnits>,
    pub r#time_in_cadence_zone: Option<ValueWithUnits>,
    pub r#time_in_power_zone: Option<ValueWithUnits>,
    pub r#repetition_num: Option<ValueWithUnits>,
    pub r#min_altitude: Option<ValueWithUnits>,
    pub r#min_heart_rate: Option<ValueWithUnits>,
    pub r#active_time: Option<ValueWithUnits>,
    pub r#wkt_step_index: Option<field_types::MessageIndex>,
    pub r#sport_event: Option<field_types::SportEvent>,
    pub r#avg_left_torque_effectiveness: Option<ValueWithUnits>,
    pub r#avg_right_torque_effectiveness: Option<ValueWithUnits>,
    pub r#avg_left_pedal_smoothness: Option<ValueWithUnits>,
    pub r#avg_right_pedal_smoothness: Option<ValueWithUnits>,
    pub r#avg_combined_pedal_smoothness: Option<ValueWithUnits>,
    pub r#status: Option<field_types::SegmentLapStatus>,
    pub r#uuid: Option<ValueWithUnits>,
    pub r#avg_fractional_cadence: Option<ValueWithUnits>,
    pub r#max_fractional_cadence: Option<ValueWithUnits>,
    pub r#total_fractional_cycles: Option<ValueWithUnits>,
    pub r#front_gear_shift_count: Option<ValueWithUnits>,
    pub r#rear_gear_shift_count: Option<ValueWithUnits>,
    pub r#time_standing: Option<ValueWithUnits>,
    pub r#stand_count: Option<ValueWithUnits>,
    pub r#avg_left_pco: Option<ValueWithUnits>,
    pub r#avg_right_pco: Option<ValueWithUnits>,
    pub r#avg_left_power_phase: Option<ValueWithUnits>,
    pub r#avg_left_power_phase_peak: Option<ValueWithUnits>,
    pub r#avg_right_power_phase: Option<ValueWithUnits>,
    pub r#avg_right_power_phase_peak: Option<ValueWithUnits>,
    pub r#avg_power_position: Option<ValueWithUnits>,
    pub r#max_power_position: Option<ValueWithUnits>,
    pub r#avg_cadence_position: Option<ValueWithUnits>,
    pub r#max_cadence_position: Option<ValueWithUnits>,
    pub r#manufacturer: Option<field_types::Manufacturer>,
    pub r#total_grit: Option<ValueWithUnits>,
    pub r#total_flow: Option<ValueWithUnits>,
    pub r#avg_grit: Option<ValueWithUnits>,
    pub r#avg_flow: Option<ValueWithUnits>,
    pub r#total_fractional_ascent: Option<ValueWithUnits>,
    pub r#total_fractional_descent: Option<ValueWithUnits>,
    pub r#enhanced_avg_altitude: Option<ValueWithUnits>,
    pub r#enhanced_max_altitude: Option<ValueWithUnits>,
    pub r#enhanced_min_altitude: Option<ValueWithUnits>,
    pub r#timestamp: Option<field_types::DateTime>,
    pub r#message_index: Option<field_types::MessageIndex>,
    pub invalid_fields: BTreeMap<&'static str, ValueWithUnits>,
}
impl FitMessage for SegmentLap {
    const NAME: &'static str = "SegmentLap";
    const KIND: MesgNum = MesgNum::SegmentLap;
    fn parse_with_options(
        record: FitDataRecord,
        options: MessageParseOptions,
    ) -> Result<Self, TryFromRecordError> {
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
        #[allow(unused_mut)]
        let mut invalid_fields = BTreeMap::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#event = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("event", value);
                    }
                },
                1u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#event_type = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("event_type", value);
                    }
                },
                2u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#start_time = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("start_time", value);
                    }
                },
                3u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#start_position_lat = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("start_position_lat", value);
                    }
                },
                4u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#start_position_long = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("start_position_long", value);
                    }
                },
                5u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#end_position_lat = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("end_position_lat", value);
                    }
                },
                6u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#end_position_long = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("end_position_long", value);
                    }
                },
                7u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#total_elapsed_time = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("total_elapsed_time", value);
                    }
                },
                8u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#total_timer_time = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("total_timer_time", value);
                    }
                },
                9u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#total_distance = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("total_distance", value);
                    }
                },
                10u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#total_cycles = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("total_cycles", value);
                    }
                },
                11u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#total_calories = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("total_calories", value);
                    }
                },
                12u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#total_fat_calories = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("total_fat_calories", value);
                    }
                },
                13u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#avg_speed = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("avg_speed", value);
                    }
                },
                14u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#max_speed = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("max_speed", value);
                    }
                },
                15u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#avg_heart_rate = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("avg_heart_rate", value);
                    }
                },
                16u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#max_heart_rate = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("max_heart_rate", value);
                    }
                },
                17u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#avg_cadence = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("avg_cadence", value);
                    }
                },
                18u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#max_cadence = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("max_cadence", value);
                    }
                },
                19u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#avg_power = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("avg_power", value);
                    }
                },
                20u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#max_power = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("max_power", value);
                    }
                },
                21u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#total_ascent = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("total_ascent", value);
                    }
                },
                22u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#total_descent = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("total_descent", value);
                    }
                },
                23u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#sport = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("sport", value);
                    }
                },
                24u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#event_group = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("event_group", value);
                    }
                },
                25u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#nec_lat = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("nec_lat", value);
                    }
                },
                26u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#nec_long = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("nec_long", value);
                    }
                },
                27u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#swc_lat = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("swc_lat", value);
                    }
                },
                28u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#swc_long = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("swc_long", value);
                    }
                },
                29u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#name = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("name", value);
                    }
                },
                30u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#normalized_power = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("normalized_power", value);
                    }
                },
                31u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#left_right_balance = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("left_right_balance", value);
                    }
                },
                32u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#sub_sport = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("sub_sport", value);
                    }
                },
                33u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#total_work = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("total_work", value);
                    }
                },
                34u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#avg_altitude = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("avg_altitude", value);
                    }
                },
                35u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#max_altitude = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("max_altitude", value);
                    }
                },
                36u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#gps_accuracy = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("gps_accuracy", value);
                    }
                },
                37u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#avg_grade = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("avg_grade", value);
                    }
                },
                38u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#avg_pos_grade = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("avg_pos_grade", value);
                    }
                },
                39u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#avg_neg_grade = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("avg_neg_grade", value);
                    }
                },
                40u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#max_pos_grade = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("max_pos_grade", value);
                    }
                },
                41u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#max_neg_grade = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("max_neg_grade", value);
                    }
                },
                42u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#avg_temperature = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("avg_temperature", value);
                    }
                },
                43u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#max_temperature = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("max_temperature", value);
                    }
                },
                44u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#total_moving_time = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("total_moving_time", value);
                    }
                },
                45u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#avg_pos_vertical_speed = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("avg_pos_vertical_speed", value);
                    }
                },
                46u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#avg_neg_vertical_speed = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("avg_neg_vertical_speed", value);
                    }
                },
                47u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#max_pos_vertical_speed = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("max_pos_vertical_speed", value);
                    }
                },
                48u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#max_neg_vertical_speed = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("max_neg_vertical_speed", value);
                    }
                },
                49u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#time_in_hr_zone = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("time_in_hr_zone", value);
                    }
                },
                50u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#time_in_speed_zone = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("time_in_speed_zone", value);
                    }
                },
                51u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#time_in_cadence_zone = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("time_in_cadence_zone", value);
                    }
                },
                52u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#time_in_power_zone = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("time_in_power_zone", value);
                    }
                },
                53u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#repetition_num = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("repetition_num", value);
                    }
                },
                54u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#min_altitude = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("min_altitude", value);
                    }
                },
                55u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#min_heart_rate = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("min_heart_rate", value);
                    }
                },
                56u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#active_time = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("active_time", value);
                    }
                },
                57u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#wkt_step_index = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("wkt_step_index", value);
                    }
                },
                58u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#sport_event = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("sport_event", value);
                    }
                },
                59u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#avg_left_torque_effectiveness = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("avg_left_torque_effectiveness", value);
                    }
                },
                60u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#avg_right_torque_effectiveness = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("avg_right_torque_effectiveness", value);
                    }
                },
                61u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#avg_left_pedal_smoothness = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("avg_left_pedal_smoothness", value);
                    }
                },
                62u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#avg_right_pedal_smoothness = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("avg_right_pedal_smoothness", value);
                    }
                },
                63u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#avg_combined_pedal_smoothness = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("avg_combined_pedal_smoothness", value);
                    }
                },
                64u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#status = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("status", value);
                    }
                },
                65u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#uuid = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("uuid", value);
                    }
                },
                66u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#avg_fractional_cadence = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("avg_fractional_cadence", value);
                    }
                },
                67u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#max_fractional_cadence = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("max_fractional_cadence", value);
                    }
                },
                68u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#total_fractional_cycles = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("total_fractional_cycles", value);
                    }
                },
                69u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#front_gear_shift_count = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("front_gear_shift_count", value);
                    }
                },
                70u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#rear_gear_shift_count = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("rear_gear_shift_count", value);
                    }
                },
                71u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#time_standing = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("time_standing", value);
                    }
                },
                72u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#stand_count = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("stand_count", value);
                    }
                },
                73u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#avg_left_pco = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("avg_left_pco", value);
                    }
                },
                74u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#avg_right_pco = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("avg_right_pco", value);
                    }
                },
                75u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#avg_left_power_phase = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("avg_left_power_phase", value);
                    }
                },
                76u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#avg_left_power_phase_peak = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("avg_left_power_phase_peak", value);
                    }
                },
                77u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#avg_right_power_phase = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("avg_right_power_phase", value);
                    }
                },
                78u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#avg_right_power_phase_peak = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("avg_right_power_phase_peak", value);
                    }
                },
                79u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#avg_power_position = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("avg_power_position", value);
                    }
                },
                80u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#max_power_position = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("max_power_position", value);
                    }
                },
                81u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#avg_cadence_position = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("avg_cadence_position", value);
                    }
                },
                82u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#max_cadence_position = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("max_cadence_position", value);
                    }
                },
                83u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#manufacturer = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("manufacturer", value);
                    }
                },
                84u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#total_grit = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("total_grit", value);
                    }
                },
                85u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#total_flow = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("total_flow", value);
                    }
                },
                86u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#avg_grit = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("avg_grit", value);
                    }
                },
                87u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#avg_flow = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("avg_flow", value);
                    }
                },
                89u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#total_fractional_ascent = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("total_fractional_ascent", value);
                    }
                },
                90u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#total_fractional_descent = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("total_fractional_descent", value);
                    }
                },
                91u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#enhanced_avg_altitude = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("enhanced_avg_altitude", value);
                    }
                },
                92u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#enhanced_max_altitude = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("enhanced_max_altitude", value);
                    }
                },
                93u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#enhanced_min_altitude = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("enhanced_min_altitude", value);
                    }
                },
                253u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#timestamp = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("timestamp", value);
                    }
                },
                254u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#message_index = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("message_index", value);
                    }
                },
                _ => {
                    if !options.ignore_unexpected_fields {
                        return Err(TryFromRecordError::unexpected_field(&field));
                    }
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
            invalid_fields,
        })
    }
    fn invalid_fields(&self) -> &BTreeMap<&'static str, ValueWithUnits> {
        &self.invalid_fields
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
    pub r#file_uuid: Option<ValueWithUnits>,
    pub r#enabled: Option<ValueWithUnits>,
    pub r#user_profile_primary_key: Option<ValueWithUnits>,
    pub r#leader_type: Option<ValueWithUnits>,
    pub r#leader_group_primary_key: Option<ValueWithUnits>,
    pub r#leader_activity_id: Option<ValueWithUnits>,
    pub r#leader_activity_id_string: Option<ValueWithUnits>,
    pub r#default_race_leader: Option<ValueWithUnits>,
    pub r#message_index: Option<field_types::MessageIndex>,
    pub invalid_fields: BTreeMap<&'static str, ValueWithUnits>,
}
impl FitMessage for SegmentFile {
    const NAME: &'static str = "SegmentFile";
    const KIND: MesgNum = MesgNum::SegmentFile;
    fn parse_with_options(
        record: FitDataRecord,
        options: MessageParseOptions,
    ) -> Result<Self, TryFromRecordError> {
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
        #[allow(unused_mut)]
        let mut invalid_fields = BTreeMap::new();
        for field in record.into_vec() {
            match field.number() {
                1u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#file_uuid = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("file_uuid", value);
                    }
                },
                3u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#enabled = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("enabled", value);
                    }
                },
                4u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#user_profile_primary_key = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("user_profile_primary_key", value);
                    }
                },
                7u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#leader_type = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("leader_type", value);
                    }
                },
                8u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#leader_group_primary_key = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("leader_group_primary_key", value);
                    }
                },
                9u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#leader_activity_id = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("leader_activity_id", value);
                    }
                },
                10u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#leader_activity_id_string = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("leader_activity_id_string", value);
                    }
                },
                11u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#default_race_leader = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("default_race_leader", value);
                    }
                },
                254u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#message_index = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("message_index", value);
                    }
                },
                _ => {
                    if !options.ignore_unexpected_fields {
                        return Err(TryFromRecordError::unexpected_field(&field));
                    }
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
            invalid_fields,
        })
    }
    fn invalid_fields(&self) -> &BTreeMap<&'static str, ValueWithUnits> {
        &self.invalid_fields
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
    pub r#sport: Option<field_types::Sport>,
    pub r#capabilities: Option<field_types::WorkoutCapabilities>,
    pub r#num_valid_steps: Option<ValueWithUnits>,
    pub r#wkt_name: Option<ValueWithUnits>,
    pub r#sub_sport: Option<field_types::SubSport>,
    pub r#pool_length: Option<ValueWithUnits>,
    pub r#pool_length_unit: Option<field_types::DisplayMeasure>,
    pub r#message_index: Option<field_types::MessageIndex>,
    pub invalid_fields: BTreeMap<&'static str, ValueWithUnits>,
}
impl FitMessage for Workout {
    const NAME: &'static str = "Workout";
    const KIND: MesgNum = MesgNum::Workout;
    fn parse_with_options(
        record: FitDataRecord,
        options: MessageParseOptions,
    ) -> Result<Self, TryFromRecordError> {
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
        #[allow(unused_mut)]
        let mut invalid_fields = BTreeMap::new();
        for field in record.into_vec() {
            match field.number() {
                4u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#sport = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("sport", value);
                    }
                },
                5u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#capabilities = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("capabilities", value);
                    }
                },
                6u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#num_valid_steps = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("num_valid_steps", value);
                    }
                },
                8u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#wkt_name = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("wkt_name", value);
                    }
                },
                11u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#sub_sport = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("sub_sport", value);
                    }
                },
                14u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#pool_length = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("pool_length", value);
                    }
                },
                15u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#pool_length_unit = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("pool_length_unit", value);
                    }
                },
                254u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#message_index = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("message_index", value);
                    }
                },
                _ => {
                    if !options.ignore_unexpected_fields {
                        return Err(TryFromRecordError::unexpected_field(&field));
                    }
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
            invalid_fields,
        })
    }
    fn invalid_fields(&self) -> &BTreeMap<&'static str, ValueWithUnits> {
        &self.invalid_fields
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
    pub r#sport: Option<field_types::Sport>,
    pub r#sub_sport: Option<field_types::SubSport>,
    pub r#num_valid_steps: Option<ValueWithUnits>,
    pub r#first_step_index: Option<ValueWithUnits>,
    pub r#pool_length: Option<ValueWithUnits>,
    pub r#pool_length_unit: Option<field_types::DisplayMeasure>,
    pub r#message_index: Option<field_types::MessageIndex>,
    pub invalid_fields: BTreeMap<&'static str, ValueWithUnits>,
}
impl FitMessage for WorkoutSession {
    const NAME: &'static str = "WorkoutSession";
    const KIND: MesgNum = MesgNum::WorkoutSession;
    fn parse_with_options(
        record: FitDataRecord,
        options: MessageParseOptions,
    ) -> Result<Self, TryFromRecordError> {
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
        #[allow(unused_mut)]
        let mut invalid_fields = BTreeMap::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#sport = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("sport", value);
                    }
                },
                1u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#sub_sport = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("sub_sport", value);
                    }
                },
                2u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#num_valid_steps = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("num_valid_steps", value);
                    }
                },
                3u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#first_step_index = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("first_step_index", value);
                    }
                },
                4u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#pool_length = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("pool_length", value);
                    }
                },
                5u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#pool_length_unit = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("pool_length_unit", value);
                    }
                },
                254u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#message_index = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("message_index", value);
                    }
                },
                _ => {
                    if !options.ignore_unexpected_fields {
                        return Err(TryFromRecordError::unexpected_field(&field));
                    }
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
            invalid_fields,
        })
    }
    fn invalid_fields(&self) -> &BTreeMap<&'static str, ValueWithUnits> {
        &self.invalid_fields
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
    pub r#wkt_step_name: Option<ValueWithUnits>,
    pub r#duration_type: Option<field_types::WktStepDuration>,
    pub r#duration_value: Option<ValueWithUnits>,
    pub r#target_type: Option<field_types::WktStepTarget>,
    pub r#target_value: Option<ValueWithUnits>,
    pub r#custom_target_value_low: Option<ValueWithUnits>,
    pub r#custom_target_value_high: Option<ValueWithUnits>,
    pub r#intensity: Option<field_types::Intensity>,
    pub r#notes: Option<ValueWithUnits>,
    pub r#equipment: Option<field_types::WorkoutEquipment>,
    pub r#exercise_category: Option<field_types::ExerciseCategory>,
    pub r#exercise_name: Option<ValueWithUnits>,
    pub r#exercise_weight: Option<ValueWithUnits>,
    pub r#weight_display_unit: Option<field_types::FitBaseUnit>,
    pub r#secondary_target_type: Option<field_types::WktStepTarget>,
    pub r#secondary_target_value: Option<ValueWithUnits>,
    pub r#secondary_custom_target_value_low: Option<ValueWithUnits>,
    pub r#secondary_custom_target_value_high: Option<ValueWithUnits>,
    pub r#message_index: Option<field_types::MessageIndex>,
    pub invalid_fields: BTreeMap<&'static str, ValueWithUnits>,
}
impl FitMessage for WorkoutStep {
    const NAME: &'static str = "WorkoutStep";
    const KIND: MesgNum = MesgNum::WorkoutStep;
    fn parse_with_options(
        record: FitDataRecord,
        options: MessageParseOptions,
    ) -> Result<Self, TryFromRecordError> {
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
        #[allow(unused_mut)]
        let mut invalid_fields = BTreeMap::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#wkt_step_name = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("wkt_step_name", value);
                    }
                },
                1u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#duration_type = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("duration_type", value);
                    }
                },
                2u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#duration_value = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("duration_value", value);
                    }
                },
                3u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#target_type = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("target_type", value);
                    }
                },
                4u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#target_value = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("target_value", value);
                    }
                },
                5u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#custom_target_value_low = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("custom_target_value_low", value);
                    }
                },
                6u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#custom_target_value_high = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("custom_target_value_high", value);
                    }
                },
                7u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#intensity = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("intensity", value);
                    }
                },
                8u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#notes = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("notes", value);
                    }
                },
                9u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#equipment = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("equipment", value);
                    }
                },
                10u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#exercise_category = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("exercise_category", value);
                    }
                },
                11u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#exercise_name = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("exercise_name", value);
                    }
                },
                12u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#exercise_weight = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("exercise_weight", value);
                    }
                },
                13u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#weight_display_unit = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("weight_display_unit", value);
                    }
                },
                19u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#secondary_target_type = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("secondary_target_type", value);
                    }
                },
                20u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#secondary_target_value = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("secondary_target_value", value);
                    }
                },
                21u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#secondary_custom_target_value_low = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("secondary_custom_target_value_low", value);
                    }
                },
                22u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#secondary_custom_target_value_high = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("secondary_custom_target_value_high", value);
                    }
                },
                254u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#message_index = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("message_index", value);
                    }
                },
                _ => {
                    if !options.ignore_unexpected_fields {
                        return Err(TryFromRecordError::unexpected_field(&field));
                    }
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
            invalid_fields,
        })
    }
    fn invalid_fields(&self) -> &BTreeMap<&'static str, ValueWithUnits> {
        &self.invalid_fields
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
    pub r#exercise_category: Option<field_types::ExerciseCategory>,
    pub r#exercise_name: Option<ValueWithUnits>,
    pub r#wkt_step_name: Option<ValueWithUnits>,
    pub r#message_index: Option<field_types::MessageIndex>,
    pub invalid_fields: BTreeMap<&'static str, ValueWithUnits>,
}
impl FitMessage for ExerciseTitle {
    const NAME: &'static str = "ExerciseTitle";
    const KIND: MesgNum = MesgNum::ExerciseTitle;
    fn parse_with_options(
        record: FitDataRecord,
        options: MessageParseOptions,
    ) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#exercise_category = None;
        let mut r#exercise_name = None;
        let mut r#wkt_step_name = None;
        let mut r#message_index = None;
        #[allow(unused_mut)]
        let mut invalid_fields = BTreeMap::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#exercise_category = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("exercise_category", value);
                    }
                },
                1u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#exercise_name = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("exercise_name", value);
                    }
                },
                2u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#wkt_step_name = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("wkt_step_name", value);
                    }
                },
                254u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#message_index = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("message_index", value);
                    }
                },
                _ => {
                    if !options.ignore_unexpected_fields {
                        return Err(TryFromRecordError::unexpected_field(&field));
                    }
                }
            }
        }
        Ok(Self {
            r#exercise_category,
            r#exercise_name,
            r#wkt_step_name,
            r#message_index,
            invalid_fields,
        })
    }
    fn invalid_fields(&self) -> &BTreeMap<&'static str, ValueWithUnits> {
        &self.invalid_fields
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
    pub r#manufacturer: Option<field_types::Manufacturer>,
    pub r#product: Option<ValueWithUnits>,
    pub r#serial_number: Option<ValueWithUnits>,
    pub r#time_created: Option<field_types::DateTime>,
    pub r#completed: Option<ValueWithUnits>,
    pub r#type: Option<field_types::Schedule>,
    pub r#scheduled_time: Option<field_types::LocalDateTime>,
    pub invalid_fields: BTreeMap<&'static str, ValueWithUnits>,
}
impl FitMessage for Schedule {
    const NAME: &'static str = "Schedule";
    const KIND: MesgNum = MesgNum::Schedule;
    fn parse_with_options(
        record: FitDataRecord,
        options: MessageParseOptions,
    ) -> Result<Self, TryFromRecordError> {
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
        #[allow(unused_mut)]
        let mut invalid_fields = BTreeMap::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#manufacturer = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("manufacturer", value);
                    }
                },
                1u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#product = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("product", value);
                    }
                },
                2u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#serial_number = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("serial_number", value);
                    }
                },
                3u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#time_created = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("time_created", value);
                    }
                },
                4u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#completed = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("completed", value);
                    }
                },
                5u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#type = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("type", value);
                    }
                },
                6u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#scheduled_time = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("scheduled_time", value);
                    }
                },
                _ => {
                    if !options.ignore_unexpected_fields {
                        return Err(TryFromRecordError::unexpected_field(&field));
                    }
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
            invalid_fields,
        })
    }
    fn invalid_fields(&self) -> &BTreeMap<&'static str, ValueWithUnits> {
        &self.invalid_fields
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
    pub r#timer_time: Option<ValueWithUnits>,
    pub r#distance: Option<ValueWithUnits>,
    pub r#calories: Option<ValueWithUnits>,
    pub r#sport: Option<field_types::Sport>,
    pub r#elapsed_time: Option<ValueWithUnits>,
    pub r#sessions: Option<ValueWithUnits>,
    pub r#active_time: Option<ValueWithUnits>,
    pub r#sport_index: Option<ValueWithUnits>,
    pub r#timestamp: Option<field_types::DateTime>,
    pub r#message_index: Option<field_types::MessageIndex>,
    pub invalid_fields: BTreeMap<&'static str, ValueWithUnits>,
}
impl FitMessage for Totals {
    const NAME: &'static str = "Totals";
    const KIND: MesgNum = MesgNum::Totals;
    fn parse_with_options(
        record: FitDataRecord,
        options: MessageParseOptions,
    ) -> Result<Self, TryFromRecordError> {
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
        #[allow(unused_mut)]
        let mut invalid_fields = BTreeMap::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#timer_time = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("timer_time", value);
                    }
                },
                1u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#distance = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("distance", value);
                    }
                },
                2u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#calories = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("calories", value);
                    }
                },
                3u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#sport = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("sport", value);
                    }
                },
                4u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#elapsed_time = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("elapsed_time", value);
                    }
                },
                5u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#sessions = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("sessions", value);
                    }
                },
                6u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#active_time = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("active_time", value);
                    }
                },
                9u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#sport_index = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("sport_index", value);
                    }
                },
                253u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#timestamp = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("timestamp", value);
                    }
                },
                254u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#message_index = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("message_index", value);
                    }
                },
                _ => {
                    if !options.ignore_unexpected_fields {
                        return Err(TryFromRecordError::unexpected_field(&field));
                    }
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
            invalid_fields,
        })
    }
    fn invalid_fields(&self) -> &BTreeMap<&'static str, ValueWithUnits> {
        &self.invalid_fields
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
    pub r#weight: Option<field_types::Weight>,
    pub r#percent_fat: Option<ValueWithUnits>,
    pub r#percent_hydration: Option<ValueWithUnits>,
    pub r#visceral_fat_mass: Option<ValueWithUnits>,
    pub r#bone_mass: Option<ValueWithUnits>,
    pub r#muscle_mass: Option<ValueWithUnits>,
    pub r#basal_met: Option<ValueWithUnits>,
    pub r#physique_rating: Option<ValueWithUnits>,
    pub r#active_met: Option<ValueWithUnits>,
    pub r#metabolic_age: Option<ValueWithUnits>,
    pub r#visceral_fat_rating: Option<ValueWithUnits>,
    pub r#user_profile_index: Option<field_types::MessageIndex>,
    pub r#bmi: Option<ValueWithUnits>,
    pub r#timestamp: Option<field_types::DateTime>,
    pub invalid_fields: BTreeMap<&'static str, ValueWithUnits>,
}
impl FitMessage for WeightScale {
    const NAME: &'static str = "WeightScale";
    const KIND: MesgNum = MesgNum::WeightScale;
    fn parse_with_options(
        record: FitDataRecord,
        options: MessageParseOptions,
    ) -> Result<Self, TryFromRecordError> {
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
        #[allow(unused_mut)]
        let mut invalid_fields = BTreeMap::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#weight = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("weight", value);
                    }
                },
                1u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#percent_fat = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("percent_fat", value);
                    }
                },
                2u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#percent_hydration = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("percent_hydration", value);
                    }
                },
                3u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#visceral_fat_mass = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("visceral_fat_mass", value);
                    }
                },
                4u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#bone_mass = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("bone_mass", value);
                    }
                },
                5u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#muscle_mass = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("muscle_mass", value);
                    }
                },
                7u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#basal_met = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("basal_met", value);
                    }
                },
                8u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#physique_rating = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("physique_rating", value);
                    }
                },
                9u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#active_met = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("active_met", value);
                    }
                },
                10u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#metabolic_age = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("metabolic_age", value);
                    }
                },
                11u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#visceral_fat_rating = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("visceral_fat_rating", value);
                    }
                },
                12u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#user_profile_index = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("user_profile_index", value);
                    }
                },
                13u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#bmi = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("bmi", value);
                    }
                },
                253u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#timestamp = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("timestamp", value);
                    }
                },
                _ => {
                    if !options.ignore_unexpected_fields {
                        return Err(TryFromRecordError::unexpected_field(&field));
                    }
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
            invalid_fields,
        })
    }
    fn invalid_fields(&self) -> &BTreeMap<&'static str, ValueWithUnits> {
        &self.invalid_fields
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
    pub r#systolic_pressure: Option<ValueWithUnits>,
    pub r#diastolic_pressure: Option<ValueWithUnits>,
    pub r#mean_arterial_pressure: Option<ValueWithUnits>,
    pub r#map_3_sample_mean: Option<ValueWithUnits>,
    pub r#map_morning_values: Option<ValueWithUnits>,
    pub r#map_evening_values: Option<ValueWithUnits>,
    pub r#heart_rate: Option<ValueWithUnits>,
    pub r#heart_rate_type: Option<field_types::HrType>,
    pub r#status: Option<field_types::BpStatus>,
    pub r#user_profile_index: Option<field_types::MessageIndex>,
    pub r#timestamp: Option<field_types::DateTime>,
    pub invalid_fields: BTreeMap<&'static str, ValueWithUnits>,
}
impl FitMessage for BloodPressure {
    const NAME: &'static str = "BloodPressure";
    const KIND: MesgNum = MesgNum::BloodPressure;
    fn parse_with_options(
        record: FitDataRecord,
        options: MessageParseOptions,
    ) -> Result<Self, TryFromRecordError> {
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
        #[allow(unused_mut)]
        let mut invalid_fields = BTreeMap::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#systolic_pressure = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("systolic_pressure", value);
                    }
                },
                1u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#diastolic_pressure = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("diastolic_pressure", value);
                    }
                },
                2u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#mean_arterial_pressure = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("mean_arterial_pressure", value);
                    }
                },
                3u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#map_3_sample_mean = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("map_3_sample_mean", value);
                    }
                },
                4u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#map_morning_values = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("map_morning_values", value);
                    }
                },
                5u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#map_evening_values = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("map_evening_values", value);
                    }
                },
                6u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#heart_rate = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("heart_rate", value);
                    }
                },
                7u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#heart_rate_type = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("heart_rate_type", value);
                    }
                },
                8u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#status = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("status", value);
                    }
                },
                9u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#user_profile_index = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("user_profile_index", value);
                    }
                },
                253u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#timestamp = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("timestamp", value);
                    }
                },
                _ => {
                    if !options.ignore_unexpected_fields {
                        return Err(TryFromRecordError::unexpected_field(&field));
                    }
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
            invalid_fields,
        })
    }
    fn invalid_fields(&self) -> &BTreeMap<&'static str, ValueWithUnits> {
        &self.invalid_fields
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
    pub r#local_timestamp: Option<field_types::LocalDateTime>,
    pub r#activity_type: Option<ValueWithUnits>,
    pub r#cycles_to_distance: Option<ValueWithUnits>,
    pub r#cycles_to_calories: Option<ValueWithUnits>,
    pub r#resting_metabolic_rate: Option<ValueWithUnits>,
    pub r#timestamp: Option<field_types::DateTime>,
    pub invalid_fields: BTreeMap<&'static str, ValueWithUnits>,
}
impl FitMessage for MonitoringInfo {
    const NAME: &'static str = "MonitoringInfo";
    const KIND: MesgNum = MesgNum::MonitoringInfo;
    fn parse_with_options(
        record: FitDataRecord,
        options: MessageParseOptions,
    ) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#local_timestamp = None;
        let mut r#activity_type = None;
        let mut r#cycles_to_distance = None;
        let mut r#cycles_to_calories = None;
        let mut r#resting_metabolic_rate = None;
        let mut r#timestamp = None;
        #[allow(unused_mut)]
        let mut invalid_fields = BTreeMap::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#local_timestamp = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("local_timestamp", value);
                    }
                },
                1u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#activity_type = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("activity_type", value);
                    }
                },
                3u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#cycles_to_distance = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("cycles_to_distance", value);
                    }
                },
                4u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#cycles_to_calories = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("cycles_to_calories", value);
                    }
                },
                5u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#resting_metabolic_rate = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("resting_metabolic_rate", value);
                    }
                },
                253u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#timestamp = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("timestamp", value);
                    }
                },
                _ => {
                    if !options.ignore_unexpected_fields {
                        return Err(TryFromRecordError::unexpected_field(&field));
                    }
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
            invalid_fields,
        })
    }
    fn invalid_fields(&self) -> &BTreeMap<&'static str, ValueWithUnits> {
        &self.invalid_fields
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
    pub r#device_index: Option<field_types::DeviceIndex>,
    pub r#calories: Option<ValueWithUnits>,
    pub r#distance: Option<ValueWithUnits>,
    pub r#cycles: Option<ValueWithUnits>,
    pub r#active_time: Option<ValueWithUnits>,
    pub r#activity_type: Option<field_types::ActivityType>,
    pub r#activity_subtype: Option<field_types::ActivitySubtype>,
    pub r#activity_level: Option<field_types::ActivityLevel>,
    pub r#distance_16: Option<ValueWithUnits>,
    pub r#cycles_16: Option<ValueWithUnits>,
    pub r#active_time_16: Option<ValueWithUnits>,
    pub r#local_timestamp: Option<field_types::LocalDateTime>,
    pub r#temperature: Option<ValueWithUnits>,
    pub r#temperature_min: Option<ValueWithUnits>,
    pub r#temperature_max: Option<ValueWithUnits>,
    pub r#activity_time: Option<ValueWithUnits>,
    pub r#active_calories: Option<ValueWithUnits>,
    pub r#current_activity_type_intensity: Option<ValueWithUnits>,
    pub r#timestamp_min_8: Option<ValueWithUnits>,
    pub r#timestamp_16: Option<ValueWithUnits>,
    pub r#heart_rate: Option<ValueWithUnits>,
    pub r#intensity: Option<ValueWithUnits>,
    pub r#duration_min: Option<ValueWithUnits>,
    pub r#duration: Option<ValueWithUnits>,
    pub r#ascent: Option<ValueWithUnits>,
    pub r#descent: Option<ValueWithUnits>,
    pub r#moderate_activity_minutes: Option<ValueWithUnits>,
    pub r#vigorous_activity_minutes: Option<ValueWithUnits>,
    pub r#timestamp: Option<field_types::DateTime>,
    pub invalid_fields: BTreeMap<&'static str, ValueWithUnits>,
}
impl FitMessage for Monitoring {
    const NAME: &'static str = "Monitoring";
    const KIND: MesgNum = MesgNum::Monitoring;
    fn parse_with_options(
        record: FitDataRecord,
        options: MessageParseOptions,
    ) -> Result<Self, TryFromRecordError> {
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
        #[allow(unused_mut)]
        let mut invalid_fields = BTreeMap::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#device_index = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("device_index", value);
                    }
                },
                1u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#calories = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("calories", value);
                    }
                },
                2u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#distance = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("distance", value);
                    }
                },
                3u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#cycles = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("cycles", value);
                    }
                },
                4u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#active_time = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("active_time", value);
                    }
                },
                5u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#activity_type = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("activity_type", value);
                    }
                },
                6u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#activity_subtype = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("activity_subtype", value);
                    }
                },
                7u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#activity_level = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("activity_level", value);
                    }
                },
                8u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#distance_16 = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("distance_16", value);
                    }
                },
                9u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#cycles_16 = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("cycles_16", value);
                    }
                },
                10u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#active_time_16 = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("active_time_16", value);
                    }
                },
                11u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#local_timestamp = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("local_timestamp", value);
                    }
                },
                12u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#temperature = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("temperature", value);
                    }
                },
                14u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#temperature_min = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("temperature_min", value);
                    }
                },
                15u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#temperature_max = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("temperature_max", value);
                    }
                },
                16u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#activity_time = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("activity_time", value);
                    }
                },
                19u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#active_calories = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("active_calories", value);
                    }
                },
                24u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#current_activity_type_intensity = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("current_activity_type_intensity", value);
                    }
                },
                25u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#timestamp_min_8 = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("timestamp_min_8", value);
                    }
                },
                26u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#timestamp_16 = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("timestamp_16", value);
                    }
                },
                27u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#heart_rate = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("heart_rate", value);
                    }
                },
                28u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#intensity = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("intensity", value);
                    }
                },
                29u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#duration_min = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("duration_min", value);
                    }
                },
                30u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#duration = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("duration", value);
                    }
                },
                31u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#ascent = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("ascent", value);
                    }
                },
                32u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#descent = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("descent", value);
                    }
                },
                33u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#moderate_activity_minutes = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("moderate_activity_minutes", value);
                    }
                },
                34u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#vigorous_activity_minutes = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("vigorous_activity_minutes", value);
                    }
                },
                253u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#timestamp = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("timestamp", value);
                    }
                },
                _ => {
                    if !options.ignore_unexpected_fields {
                        return Err(TryFromRecordError::unexpected_field(&field));
                    }
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
            invalid_fields,
        })
    }
    fn invalid_fields(&self) -> &BTreeMap<&'static str, ValueWithUnits> {
        &self.invalid_fields
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
    pub r#resting_heart_rate: Option<ValueWithUnits>,
    pub r#current_day_resting_heart_rate: Option<ValueWithUnits>,
    pub r#timestamp: Option<field_types::DateTime>,
    pub invalid_fields: BTreeMap<&'static str, ValueWithUnits>,
}
impl FitMessage for MonitoringHrData {
    const NAME: &'static str = "MonitoringHrData";
    const KIND: MesgNum = MesgNum::MonitoringHrData;
    fn parse_with_options(
        record: FitDataRecord,
        options: MessageParseOptions,
    ) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#resting_heart_rate = None;
        let mut r#current_day_resting_heart_rate = None;
        let mut r#timestamp = None;
        #[allow(unused_mut)]
        let mut invalid_fields = BTreeMap::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#resting_heart_rate = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("resting_heart_rate", value);
                    }
                },
                1u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#current_day_resting_heart_rate = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("current_day_resting_heart_rate", value);
                    }
                },
                253u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#timestamp = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("timestamp", value);
                    }
                },
                _ => {
                    if !options.ignore_unexpected_fields {
                        return Err(TryFromRecordError::unexpected_field(&field));
                    }
                }
            }
        }
        Ok(Self {
            r#resting_heart_rate,
            r#current_day_resting_heart_rate,
            r#timestamp,
            invalid_fields,
        })
    }
    fn invalid_fields(&self) -> &BTreeMap<&'static str, ValueWithUnits> {
        &self.invalid_fields
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
    pub r#reading_spo2: Option<ValueWithUnits>,
    pub r#reading_confidence: Option<ValueWithUnits>,
    pub r#mode: Option<field_types::Spo2MeasurementType>,
    pub r#timestamp: Option<field_types::DateTime>,
    pub invalid_fields: BTreeMap<&'static str, ValueWithUnits>,
}
impl FitMessage for Spo2Data {
    const NAME: &'static str = "Spo2Data";
    const KIND: MesgNum = MesgNum::Spo2Data;
    fn parse_with_options(
        record: FitDataRecord,
        options: MessageParseOptions,
    ) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#reading_spo2 = None;
        let mut r#reading_confidence = None;
        let mut r#mode = None;
        let mut r#timestamp = None;
        #[allow(unused_mut)]
        let mut invalid_fields = BTreeMap::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#reading_spo2 = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("reading_spo2", value);
                    }
                },
                1u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#reading_confidence = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("reading_confidence", value);
                    }
                },
                2u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#mode = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("mode", value);
                    }
                },
                253u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#timestamp = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("timestamp", value);
                    }
                },
                _ => {
                    if !options.ignore_unexpected_fields {
                        return Err(TryFromRecordError::unexpected_field(&field));
                    }
                }
            }
        }
        Ok(Self {
            r#reading_spo2,
            r#reading_confidence,
            r#mode,
            r#timestamp,
            invalid_fields,
        })
    }
    fn invalid_fields(&self) -> &BTreeMap<&'static str, ValueWithUnits> {
        &self.invalid_fields
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
    pub r#fractional_timestamp: Option<ValueWithUnits>,
    pub r#time256: Option<ValueWithUnits>,
    pub r#filtered_bpm: Option<ValueWithUnits>,
    pub r#event_timestamp: Option<ValueWithUnits>,
    pub r#event_timestamp_12: Option<ValueWithUnits>,
    pub r#timestamp: Option<field_types::DateTime>,
    pub invalid_fields: BTreeMap<&'static str, ValueWithUnits>,
}
impl FitMessage for Hr {
    const NAME: &'static str = "Hr";
    const KIND: MesgNum = MesgNum::Hr;
    fn parse_with_options(
        record: FitDataRecord,
        options: MessageParseOptions,
    ) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#fractional_timestamp = None;
        let mut r#time256 = None;
        let mut r#filtered_bpm = None;
        let mut r#event_timestamp = None;
        let mut r#event_timestamp_12 = None;
        let mut r#timestamp = None;
        #[allow(unused_mut)]
        let mut invalid_fields = BTreeMap::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#fractional_timestamp = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("fractional_timestamp", value);
                    }
                },
                1u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#time256 = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("time256", value);
                    }
                },
                6u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#filtered_bpm = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("filtered_bpm", value);
                    }
                },
                9u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#event_timestamp = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("event_timestamp", value);
                    }
                },
                10u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#event_timestamp_12 = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("event_timestamp_12", value);
                    }
                },
                253u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#timestamp = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("timestamp", value);
                    }
                },
                _ => {
                    if !options.ignore_unexpected_fields {
                        return Err(TryFromRecordError::unexpected_field(&field));
                    }
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
            invalid_fields,
        })
    }
    fn invalid_fields(&self) -> &BTreeMap<&'static str, ValueWithUnits> {
        &self.invalid_fields
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
    pub r#stress_level_value: Option<ValueWithUnits>,
    pub r#stress_level_time: Option<field_types::DateTime>,
    pub invalid_fields: BTreeMap<&'static str, ValueWithUnits>,
}
impl FitMessage for StressLevel {
    const NAME: &'static str = "StressLevel";
    const KIND: MesgNum = MesgNum::StressLevel;
    fn parse_with_options(
        record: FitDataRecord,
        options: MessageParseOptions,
    ) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#stress_level_value = None;
        let mut r#stress_level_time = None;
        #[allow(unused_mut)]
        let mut invalid_fields = BTreeMap::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#stress_level_value = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("stress_level_value", value);
                    }
                },
                1u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#stress_level_time = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("stress_level_time", value);
                    }
                },
                _ => {
                    if !options.ignore_unexpected_fields {
                        return Err(TryFromRecordError::unexpected_field(&field));
                    }
                }
            }
        }
        Ok(Self {
            r#stress_level_value,
            r#stress_level_time,
            invalid_fields,
        })
    }
    fn invalid_fields(&self) -> &BTreeMap<&'static str, ValueWithUnits> {
        &self.invalid_fields
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
    pub r#update_time: Option<field_types::DateTime>,
    pub r#vo2_max: Option<ValueWithUnits>,
    pub r#sport: Option<field_types::Sport>,
    pub r#sub_sport: Option<field_types::SubSport>,
    pub r#max_met_category: Option<field_types::MaxMetCategory>,
    pub r#calibrated_data: Option<ValueWithUnits>,
    pub r#hr_source: Option<field_types::MaxMetHeartRateSource>,
    pub r#speed_source: Option<field_types::MaxMetSpeedSource>,
    pub invalid_fields: BTreeMap<&'static str, ValueWithUnits>,
}
impl FitMessage for MaxMetData {
    const NAME: &'static str = "MaxMetData";
    const KIND: MesgNum = MesgNum::MaxMetData;
    fn parse_with_options(
        record: FitDataRecord,
        options: MessageParseOptions,
    ) -> Result<Self, TryFromRecordError> {
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
        #[allow(unused_mut)]
        let mut invalid_fields = BTreeMap::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#update_time = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("update_time", value);
                    }
                },
                2u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#vo2_max = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("vo2_max", value);
                    }
                },
                5u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#sport = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("sport", value);
                    }
                },
                6u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#sub_sport = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("sub_sport", value);
                    }
                },
                8u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#max_met_category = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("max_met_category", value);
                    }
                },
                9u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#calibrated_data = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("calibrated_data", value);
                    }
                },
                12u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#hr_source = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("hr_source", value);
                    }
                },
                13u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#speed_source = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("speed_source", value);
                    }
                },
                _ => {
                    if !options.ignore_unexpected_fields {
                        return Err(TryFromRecordError::unexpected_field(&field));
                    }
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
            invalid_fields,
        })
    }
    fn invalid_fields(&self) -> &BTreeMap<&'static str, ValueWithUnits> {
        &self.invalid_fields
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
    pub r#processing_interval: Option<ValueWithUnits>,
    pub r#level: Option<ValueWithUnits>,
    pub r#charged: Option<ValueWithUnits>,
    pub r#uncharged: Option<ValueWithUnits>,
    pub r#timestamp: Option<field_types::DateTime>,
    pub invalid_fields: BTreeMap<&'static str, ValueWithUnits>,
}
impl FitMessage for HsaBodyBatteryData {
    const NAME: &'static str = "HsaBodyBatteryData";
    const KIND: MesgNum = MesgNum::HsaBodyBatteryData;
    fn parse_with_options(
        record: FitDataRecord,
        options: MessageParseOptions,
    ) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#processing_interval = None;
        let mut r#level = None;
        let mut r#charged = None;
        let mut r#uncharged = None;
        let mut r#timestamp = None;
        #[allow(unused_mut)]
        let mut invalid_fields = BTreeMap::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#processing_interval = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("processing_interval", value);
                    }
                },
                1u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#level = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("level", value);
                    }
                },
                2u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#charged = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("charged", value);
                    }
                },
                3u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#uncharged = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("uncharged", value);
                    }
                },
                253u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#timestamp = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("timestamp", value);
                    }
                },
                _ => {
                    if !options.ignore_unexpected_fields {
                        return Err(TryFromRecordError::unexpected_field(&field));
                    }
                }
            }
        }
        Ok(Self {
            r#processing_interval,
            r#level,
            r#charged,
            r#uncharged,
            r#timestamp,
            invalid_fields,
        })
    }
    fn invalid_fields(&self) -> &BTreeMap<&'static str, ValueWithUnits> {
        &self.invalid_fields
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
    pub r#event_id: Option<ValueWithUnits>,
    pub r#timestamp: Option<field_types::DateTime>,
    pub invalid_fields: BTreeMap<&'static str, ValueWithUnits>,
}
impl FitMessage for HsaEvent {
    const NAME: &'static str = "HsaEvent";
    const KIND: MesgNum = MesgNum::HsaEvent;
    fn parse_with_options(
        record: FitDataRecord,
        options: MessageParseOptions,
    ) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#event_id = None;
        let mut r#timestamp = None;
        #[allow(unused_mut)]
        let mut invalid_fields = BTreeMap::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#event_id = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("event_id", value);
                    }
                },
                253u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#timestamp = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("timestamp", value);
                    }
                },
                _ => {
                    if !options.ignore_unexpected_fields {
                        return Err(TryFromRecordError::unexpected_field(&field));
                    }
                }
            }
        }
        Ok(Self {
            r#event_id,
            r#timestamp,
            invalid_fields,
        })
    }
    fn invalid_fields(&self) -> &BTreeMap<&'static str, ValueWithUnits> {
        &self.invalid_fields
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
    pub r#timestamp_ms: Option<ValueWithUnits>,
    pub r#sampling_interval: Option<ValueWithUnits>,
    pub r#accel_x: Option<ValueWithUnits>,
    pub r#accel_y: Option<ValueWithUnits>,
    pub r#accel_z: Option<ValueWithUnits>,
    pub r#timestamp_32k: Option<ValueWithUnits>,
    pub r#timestamp: Option<field_types::DateTime>,
    pub invalid_fields: BTreeMap<&'static str, ValueWithUnits>,
}
impl FitMessage for HsaAccelerometerData {
    const NAME: &'static str = "HsaAccelerometerData";
    const KIND: MesgNum = MesgNum::HsaAccelerometerData;
    fn parse_with_options(
        record: FitDataRecord,
        options: MessageParseOptions,
    ) -> Result<Self, TryFromRecordError> {
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
        #[allow(unused_mut)]
        let mut invalid_fields = BTreeMap::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#timestamp_ms = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("timestamp_ms", value);
                    }
                },
                1u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#sampling_interval = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("sampling_interval", value);
                    }
                },
                2u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#accel_x = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("accel_x", value);
                    }
                },
                3u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#accel_y = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("accel_y", value);
                    }
                },
                4u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#accel_z = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("accel_z", value);
                    }
                },
                5u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#timestamp_32k = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("timestamp_32k", value);
                    }
                },
                253u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#timestamp = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("timestamp", value);
                    }
                },
                _ => {
                    if !options.ignore_unexpected_fields {
                        return Err(TryFromRecordError::unexpected_field(&field));
                    }
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
            invalid_fields,
        })
    }
    fn invalid_fields(&self) -> &BTreeMap<&'static str, ValueWithUnits> {
        &self.invalid_fields
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
    pub r#timestamp_ms: Option<ValueWithUnits>,
    pub r#sampling_interval: Option<ValueWithUnits>,
    pub r#gyro_x: Option<ValueWithUnits>,
    pub r#gyro_y: Option<ValueWithUnits>,
    pub r#gyro_z: Option<ValueWithUnits>,
    pub r#timestamp_32k: Option<ValueWithUnits>,
    pub r#timestamp: Option<field_types::DateTime>,
    pub invalid_fields: BTreeMap<&'static str, ValueWithUnits>,
}
impl FitMessage for HsaGyroscopeData {
    const NAME: &'static str = "HsaGyroscopeData";
    const KIND: MesgNum = MesgNum::HsaGyroscopeData;
    fn parse_with_options(
        record: FitDataRecord,
        options: MessageParseOptions,
    ) -> Result<Self, TryFromRecordError> {
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
        #[allow(unused_mut)]
        let mut invalid_fields = BTreeMap::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#timestamp_ms = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("timestamp_ms", value);
                    }
                },
                1u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#sampling_interval = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("sampling_interval", value);
                    }
                },
                2u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#gyro_x = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("gyro_x", value);
                    }
                },
                3u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#gyro_y = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("gyro_y", value);
                    }
                },
                4u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#gyro_z = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("gyro_z", value);
                    }
                },
                5u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#timestamp_32k = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("timestamp_32k", value);
                    }
                },
                253u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#timestamp = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("timestamp", value);
                    }
                },
                _ => {
                    if !options.ignore_unexpected_fields {
                        return Err(TryFromRecordError::unexpected_field(&field));
                    }
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
            invalid_fields,
        })
    }
    fn invalid_fields(&self) -> &BTreeMap<&'static str, ValueWithUnits> {
        &self.invalid_fields
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
    pub r#processing_interval: Option<ValueWithUnits>,
    pub r#steps: Option<ValueWithUnits>,
    pub r#timestamp: Option<field_types::DateTime>,
    pub invalid_fields: BTreeMap<&'static str, ValueWithUnits>,
}
impl FitMessage for HsaStepData {
    const NAME: &'static str = "HsaStepData";
    const KIND: MesgNum = MesgNum::HsaStepData;
    fn parse_with_options(
        record: FitDataRecord,
        options: MessageParseOptions,
    ) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#processing_interval = None;
        let mut r#steps = None;
        let mut r#timestamp = None;
        #[allow(unused_mut)]
        let mut invalid_fields = BTreeMap::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#processing_interval = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("processing_interval", value);
                    }
                },
                1u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#steps = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("steps", value);
                    }
                },
                253u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#timestamp = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("timestamp", value);
                    }
                },
                _ => {
                    if !options.ignore_unexpected_fields {
                        return Err(TryFromRecordError::unexpected_field(&field));
                    }
                }
            }
        }
        Ok(Self {
            r#processing_interval,
            r#steps,
            r#timestamp,
            invalid_fields,
        })
    }
    fn invalid_fields(&self) -> &BTreeMap<&'static str, ValueWithUnits> {
        &self.invalid_fields
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
    pub r#processing_interval: Option<ValueWithUnits>,
    pub r#reading_spo2: Option<ValueWithUnits>,
    pub r#confidence: Option<ValueWithUnits>,
    pub r#timestamp: Option<field_types::DateTime>,
    pub invalid_fields: BTreeMap<&'static str, ValueWithUnits>,
}
impl FitMessage for HsaSpo2Data {
    const NAME: &'static str = "HsaSpo2Data";
    const KIND: MesgNum = MesgNum::HsaSpo2Data;
    fn parse_with_options(
        record: FitDataRecord,
        options: MessageParseOptions,
    ) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#processing_interval = None;
        let mut r#reading_spo2 = None;
        let mut r#confidence = None;
        let mut r#timestamp = None;
        #[allow(unused_mut)]
        let mut invalid_fields = BTreeMap::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#processing_interval = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("processing_interval", value);
                    }
                },
                1u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#reading_spo2 = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("reading_spo2", value);
                    }
                },
                2u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#confidence = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("confidence", value);
                    }
                },
                253u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#timestamp = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("timestamp", value);
                    }
                },
                _ => {
                    if !options.ignore_unexpected_fields {
                        return Err(TryFromRecordError::unexpected_field(&field));
                    }
                }
            }
        }
        Ok(Self {
            r#processing_interval,
            r#reading_spo2,
            r#confidence,
            r#timestamp,
            invalid_fields,
        })
    }
    fn invalid_fields(&self) -> &BTreeMap<&'static str, ValueWithUnits> {
        &self.invalid_fields
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
    pub r#processing_interval: Option<ValueWithUnits>,
    pub r#stress_level: Option<ValueWithUnits>,
    pub r#timestamp: Option<field_types::DateTime>,
    pub invalid_fields: BTreeMap<&'static str, ValueWithUnits>,
}
impl FitMessage for HsaStressData {
    const NAME: &'static str = "HsaStressData";
    const KIND: MesgNum = MesgNum::HsaStressData;
    fn parse_with_options(
        record: FitDataRecord,
        options: MessageParseOptions,
    ) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#processing_interval = None;
        let mut r#stress_level = None;
        let mut r#timestamp = None;
        #[allow(unused_mut)]
        let mut invalid_fields = BTreeMap::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#processing_interval = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("processing_interval", value);
                    }
                },
                1u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#stress_level = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("stress_level", value);
                    }
                },
                253u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#timestamp = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("timestamp", value);
                    }
                },
                _ => {
                    if !options.ignore_unexpected_fields {
                        return Err(TryFromRecordError::unexpected_field(&field));
                    }
                }
            }
        }
        Ok(Self {
            r#processing_interval,
            r#stress_level,
            r#timestamp,
            invalid_fields,
        })
    }
    fn invalid_fields(&self) -> &BTreeMap<&'static str, ValueWithUnits> {
        &self.invalid_fields
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
    pub r#processing_interval: Option<ValueWithUnits>,
    pub r#respiration_rate: Option<ValueWithUnits>,
    pub r#timestamp: Option<field_types::DateTime>,
    pub invalid_fields: BTreeMap<&'static str, ValueWithUnits>,
}
impl FitMessage for HsaRespirationData {
    const NAME: &'static str = "HsaRespirationData";
    const KIND: MesgNum = MesgNum::HsaRespirationData;
    fn parse_with_options(
        record: FitDataRecord,
        options: MessageParseOptions,
    ) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#processing_interval = None;
        let mut r#respiration_rate = None;
        let mut r#timestamp = None;
        #[allow(unused_mut)]
        let mut invalid_fields = BTreeMap::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#processing_interval = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("processing_interval", value);
                    }
                },
                1u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#respiration_rate = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("respiration_rate", value);
                    }
                },
                253u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#timestamp = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("timestamp", value);
                    }
                },
                _ => {
                    if !options.ignore_unexpected_fields {
                        return Err(TryFromRecordError::unexpected_field(&field));
                    }
                }
            }
        }
        Ok(Self {
            r#processing_interval,
            r#respiration_rate,
            r#timestamp,
            invalid_fields,
        })
    }
    fn invalid_fields(&self) -> &BTreeMap<&'static str, ValueWithUnits> {
        &self.invalid_fields
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
    pub r#processing_interval: Option<ValueWithUnits>,
    pub r#status: Option<ValueWithUnits>,
    pub r#heart_rate: Option<ValueWithUnits>,
    pub r#timestamp: Option<field_types::DateTime>,
    pub invalid_fields: BTreeMap<&'static str, ValueWithUnits>,
}
impl FitMessage for HsaHeartRateData {
    const NAME: &'static str = "HsaHeartRateData";
    const KIND: MesgNum = MesgNum::HsaHeartRateData;
    fn parse_with_options(
        record: FitDataRecord,
        options: MessageParseOptions,
    ) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#processing_interval = None;
        let mut r#status = None;
        let mut r#heart_rate = None;
        let mut r#timestamp = None;
        #[allow(unused_mut)]
        let mut invalid_fields = BTreeMap::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#processing_interval = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("processing_interval", value);
                    }
                },
                1u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#status = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("status", value);
                    }
                },
                2u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#heart_rate = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("heart_rate", value);
                    }
                },
                253u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#timestamp = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("timestamp", value);
                    }
                },
                _ => {
                    if !options.ignore_unexpected_fields {
                        return Err(TryFromRecordError::unexpected_field(&field));
                    }
                }
            }
        }
        Ok(Self {
            r#processing_interval,
            r#status,
            r#heart_rate,
            r#timestamp,
            invalid_fields,
        })
    }
    fn invalid_fields(&self) -> &BTreeMap<&'static str, ValueWithUnits> {
        &self.invalid_fields
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
    pub r#data: Option<ValueWithUnits>,
    pub r#data_size: Option<ValueWithUnits>,
    pub r#timestamp: Option<field_types::DateTime>,
    pub invalid_fields: BTreeMap<&'static str, ValueWithUnits>,
}
impl FitMessage for HsaConfigurationData {
    const NAME: &'static str = "HsaConfigurationData";
    const KIND: MesgNum = MesgNum::HsaConfigurationData;
    fn parse_with_options(
        record: FitDataRecord,
        options: MessageParseOptions,
    ) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#data = None;
        let mut r#data_size = None;
        let mut r#timestamp = None;
        #[allow(unused_mut)]
        let mut invalid_fields = BTreeMap::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#data = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("data", value);
                    }
                },
                1u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#data_size = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("data_size", value);
                    }
                },
                253u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#timestamp = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("timestamp", value);
                    }
                },
                _ => {
                    if !options.ignore_unexpected_fields {
                        return Err(TryFromRecordError::unexpected_field(&field));
                    }
                }
            }
        }
        Ok(Self {
            r#data,
            r#data_size,
            r#timestamp,
            invalid_fields,
        })
    }
    fn invalid_fields(&self) -> &BTreeMap<&'static str, ValueWithUnits> {
        &self.invalid_fields
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
    pub r#processing_interval: Option<ValueWithUnits>,
    pub r#value: Option<ValueWithUnits>,
    pub r#timestamp: Option<field_types::DateTime>,
    pub invalid_fields: BTreeMap<&'static str, ValueWithUnits>,
}
impl FitMessage for HsaWristTemperatureData {
    const NAME: &'static str = "HsaWristTemperatureData";
    const KIND: MesgNum = MesgNum::HsaWristTemperatureData;
    fn parse_with_options(
        record: FitDataRecord,
        options: MessageParseOptions,
    ) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#processing_interval = None;
        let mut r#value = None;
        let mut r#timestamp = None;
        #[allow(unused_mut)]
        let mut invalid_fields = BTreeMap::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#processing_interval = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("processing_interval", value);
                    }
                },
                1u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#value = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("value", value);
                    }
                },
                253u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#timestamp = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("timestamp", value);
                    }
                },
                _ => {
                    if !options.ignore_unexpected_fields {
                        return Err(TryFromRecordError::unexpected_field(&field));
                    }
                }
            }
        }
        Ok(Self {
            r#processing_interval,
            r#value,
            r#timestamp,
            invalid_fields,
        })
    }
    fn invalid_fields(&self) -> &BTreeMap<&'static str, ValueWithUnits> {
        &self.invalid_fields
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
    pub r#memo: Option<ValueWithUnits>,
    pub r#mesg_num: Option<field_types::MesgNum>,
    pub r#parent_index: Option<field_types::MessageIndex>,
    pub r#field_num: Option<ValueWithUnits>,
    pub r#data: Option<ValueWithUnits>,
    pub r#part_index: Option<ValueWithUnits>,
    pub invalid_fields: BTreeMap<&'static str, ValueWithUnits>,
}
impl FitMessage for MemoGlob {
    const NAME: &'static str = "MemoGlob";
    const KIND: MesgNum = MesgNum::MemoGlob;
    fn parse_with_options(
        record: FitDataRecord,
        options: MessageParseOptions,
    ) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#memo = None;
        let mut r#mesg_num = None;
        let mut r#parent_index = None;
        let mut r#field_num = None;
        let mut r#data = None;
        let mut r#part_index = None;
        #[allow(unused_mut)]
        let mut invalid_fields = BTreeMap::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#memo = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("memo", value);
                    }
                },
                1u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#mesg_num = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("mesg_num", value);
                    }
                },
                2u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#parent_index = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("parent_index", value);
                    }
                },
                3u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#field_num = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("field_num", value);
                    }
                },
                4u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#data = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("data", value);
                    }
                },
                250u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#part_index = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("part_index", value);
                    }
                },
                _ => {
                    if !options.ignore_unexpected_fields {
                        return Err(TryFromRecordError::unexpected_field(&field));
                    }
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
            invalid_fields,
        })
    }
    fn invalid_fields(&self) -> &BTreeMap<&'static str, ValueWithUnits> {
        &self.invalid_fields
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
    pub r#sleep_level: Option<field_types::SleepLevel>,
    pub r#timestamp: Option<field_types::DateTime>,
    pub invalid_fields: BTreeMap<&'static str, ValueWithUnits>,
}
impl FitMessage for SleepLevel {
    const NAME: &'static str = "SleepLevel";
    const KIND: MesgNum = MesgNum::SleepLevel;
    fn parse_with_options(
        record: FitDataRecord,
        options: MessageParseOptions,
    ) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#sleep_level = None;
        let mut r#timestamp = None;
        #[allow(unused_mut)]
        let mut invalid_fields = BTreeMap::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#sleep_level = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("sleep_level", value);
                    }
                },
                253u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#timestamp = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("timestamp", value);
                    }
                },
                _ => {
                    if !options.ignore_unexpected_fields {
                        return Err(TryFromRecordError::unexpected_field(&field));
                    }
                }
            }
        }
        Ok(Self {
            r#sleep_level,
            r#timestamp,
            invalid_fields,
        })
    }
    fn invalid_fields(&self) -> &BTreeMap<&'static str, ValueWithUnits> {
        &self.invalid_fields
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
    pub r#channel_number: Option<ValueWithUnits>,
    pub r#device_type: Option<ValueWithUnits>,
    pub r#device_number: Option<ValueWithUnits>,
    pub r#transmission_type: Option<ValueWithUnits>,
    pub r#device_index: Option<field_types::DeviceIndex>,
    pub invalid_fields: BTreeMap<&'static str, ValueWithUnits>,
}
impl FitMessage for AntChannelId {
    const NAME: &'static str = "AntChannelId";
    const KIND: MesgNum = MesgNum::AntChannelId;
    fn parse_with_options(
        record: FitDataRecord,
        options: MessageParseOptions,
    ) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#channel_number = None;
        let mut r#device_type = None;
        let mut r#device_number = None;
        let mut r#transmission_type = None;
        let mut r#device_index = None;
        #[allow(unused_mut)]
        let mut invalid_fields = BTreeMap::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#channel_number = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("channel_number", value);
                    }
                },
                1u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#device_type = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("device_type", value);
                    }
                },
                2u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#device_number = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("device_number", value);
                    }
                },
                3u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#transmission_type = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("transmission_type", value);
                    }
                },
                4u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#device_index = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("device_index", value);
                    }
                },
                _ => {
                    if !options.ignore_unexpected_fields {
                        return Err(TryFromRecordError::unexpected_field(&field));
                    }
                }
            }
        }
        Ok(Self {
            r#channel_number,
            r#device_type,
            r#device_number,
            r#transmission_type,
            r#device_index,
            invalid_fields,
        })
    }
    fn invalid_fields(&self) -> &BTreeMap<&'static str, ValueWithUnits> {
        &self.invalid_fields
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
    pub r#fractional_timestamp: Option<ValueWithUnits>,
    pub r#mesg_id: Option<ValueWithUnits>,
    pub r#mesg_data: Option<ValueWithUnits>,
    pub r#channel_number: Option<ValueWithUnits>,
    pub r#data: Option<ValueWithUnits>,
    pub r#timestamp: Option<field_types::DateTime>,
    pub invalid_fields: BTreeMap<&'static str, ValueWithUnits>,
}
impl FitMessage for AntRx {
    const NAME: &'static str = "AntRx";
    const KIND: MesgNum = MesgNum::AntRx;
    fn parse_with_options(
        record: FitDataRecord,
        options: MessageParseOptions,
    ) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#fractional_timestamp = None;
        let mut r#mesg_id = None;
        let mut r#mesg_data = None;
        let mut r#channel_number = None;
        let mut r#data = None;
        let mut r#timestamp = None;
        #[allow(unused_mut)]
        let mut invalid_fields = BTreeMap::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#fractional_timestamp = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("fractional_timestamp", value);
                    }
                },
                1u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#mesg_id = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("mesg_id", value);
                    }
                },
                2u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#mesg_data = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("mesg_data", value);
                    }
                },
                3u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#channel_number = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("channel_number", value);
                    }
                },
                4u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#data = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("data", value);
                    }
                },
                253u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#timestamp = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("timestamp", value);
                    }
                },
                _ => {
                    if !options.ignore_unexpected_fields {
                        return Err(TryFromRecordError::unexpected_field(&field));
                    }
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
            invalid_fields,
        })
    }
    fn invalid_fields(&self) -> &BTreeMap<&'static str, ValueWithUnits> {
        &self.invalid_fields
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
    pub r#fractional_timestamp: Option<ValueWithUnits>,
    pub r#mesg_id: Option<ValueWithUnits>,
    pub r#mesg_data: Option<ValueWithUnits>,
    pub r#channel_number: Option<ValueWithUnits>,
    pub r#data: Option<ValueWithUnits>,
    pub r#timestamp: Option<field_types::DateTime>,
    pub invalid_fields: BTreeMap<&'static str, ValueWithUnits>,
}
impl FitMessage for AntTx {
    const NAME: &'static str = "AntTx";
    const KIND: MesgNum = MesgNum::AntTx;
    fn parse_with_options(
        record: FitDataRecord,
        options: MessageParseOptions,
    ) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#fractional_timestamp = None;
        let mut r#mesg_id = None;
        let mut r#mesg_data = None;
        let mut r#channel_number = None;
        let mut r#data = None;
        let mut r#timestamp = None;
        #[allow(unused_mut)]
        let mut invalid_fields = BTreeMap::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#fractional_timestamp = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("fractional_timestamp", value);
                    }
                },
                1u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#mesg_id = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("mesg_id", value);
                    }
                },
                2u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#mesg_data = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("mesg_data", value);
                    }
                },
                3u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#channel_number = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("channel_number", value);
                    }
                },
                4u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#data = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("data", value);
                    }
                },
                253u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#timestamp = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("timestamp", value);
                    }
                },
                _ => {
                    if !options.ignore_unexpected_fields {
                        return Err(TryFromRecordError::unexpected_field(&field));
                    }
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
            invalid_fields,
        })
    }
    fn invalid_fields(&self) -> &BTreeMap<&'static str, ValueWithUnits> {
        &self.invalid_fields
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
    pub r#screen_index: Option<ValueWithUnits>,
    pub r#field_count: Option<ValueWithUnits>,
    pub r#layout: Option<field_types::ExdLayout>,
    pub r#screen_enabled: Option<ValueWithUnits>,
    pub invalid_fields: BTreeMap<&'static str, ValueWithUnits>,
}
impl FitMessage for ExdScreenConfiguration {
    const NAME: &'static str = "ExdScreenConfiguration";
    const KIND: MesgNum = MesgNum::ExdScreenConfiguration;
    fn parse_with_options(
        record: FitDataRecord,
        options: MessageParseOptions,
    ) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#screen_index = None;
        let mut r#field_count = None;
        let mut r#layout = None;
        let mut r#screen_enabled = None;
        #[allow(unused_mut)]
        let mut invalid_fields = BTreeMap::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#screen_index = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("screen_index", value);
                    }
                },
                1u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#field_count = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("field_count", value);
                    }
                },
                2u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#layout = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("layout", value);
                    }
                },
                3u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#screen_enabled = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("screen_enabled", value);
                    }
                },
                _ => {
                    if !options.ignore_unexpected_fields {
                        return Err(TryFromRecordError::unexpected_field(&field));
                    }
                }
            }
        }
        Ok(Self {
            r#screen_index,
            r#field_count,
            r#layout,
            r#screen_enabled,
            invalid_fields,
        })
    }
    fn invalid_fields(&self) -> &BTreeMap<&'static str, ValueWithUnits> {
        &self.invalid_fields
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
    pub r#screen_index: Option<ValueWithUnits>,
    pub r#concept_field: Option<ValueWithUnits>,
    pub r#field_id: Option<ValueWithUnits>,
    pub r#concept_count: Option<ValueWithUnits>,
    pub r#display_type: Option<field_types::ExdDisplayType>,
    pub r#title: Option<ValueWithUnits>,
    pub invalid_fields: BTreeMap<&'static str, ValueWithUnits>,
}
impl FitMessage for ExdDataFieldConfiguration {
    const NAME: &'static str = "ExdDataFieldConfiguration";
    const KIND: MesgNum = MesgNum::ExdDataFieldConfiguration;
    fn parse_with_options(
        record: FitDataRecord,
        options: MessageParseOptions,
    ) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#screen_index = None;
        let mut r#concept_field = None;
        let mut r#field_id = None;
        let mut r#concept_count = None;
        let mut r#display_type = None;
        let mut r#title = None;
        #[allow(unused_mut)]
        let mut invalid_fields = BTreeMap::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#screen_index = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("screen_index", value);
                    }
                },
                1u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#concept_field = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("concept_field", value);
                    }
                },
                2u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#field_id = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("field_id", value);
                    }
                },
                3u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#concept_count = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("concept_count", value);
                    }
                },
                4u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#display_type = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("display_type", value);
                    }
                },
                5u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#title = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("title", value);
                    }
                },
                _ => {
                    if !options.ignore_unexpected_fields {
                        return Err(TryFromRecordError::unexpected_field(&field));
                    }
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
            invalid_fields,
        })
    }
    fn invalid_fields(&self) -> &BTreeMap<&'static str, ValueWithUnits> {
        &self.invalid_fields
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
    pub r#screen_index: Option<ValueWithUnits>,
    pub r#concept_field: Option<ValueWithUnits>,
    pub r#field_id: Option<ValueWithUnits>,
    pub r#concept_index: Option<ValueWithUnits>,
    pub r#data_page: Option<ValueWithUnits>,
    pub r#concept_key: Option<ValueWithUnits>,
    pub r#scaling: Option<ValueWithUnits>,
    pub r#data_units: Option<field_types::ExdDataUnits>,
    pub r#qualifier: Option<field_types::ExdQualifiers>,
    pub r#descriptor: Option<field_types::ExdDescriptors>,
    pub r#is_signed: Option<ValueWithUnits>,
    pub invalid_fields: BTreeMap<&'static str, ValueWithUnits>,
}
impl FitMessage for ExdDataConceptConfiguration {
    const NAME: &'static str = "ExdDataConceptConfiguration";
    const KIND: MesgNum = MesgNum::ExdDataConceptConfiguration;
    fn parse_with_options(
        record: FitDataRecord,
        options: MessageParseOptions,
    ) -> Result<Self, TryFromRecordError> {
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
        #[allow(unused_mut)]
        let mut invalid_fields = BTreeMap::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#screen_index = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("screen_index", value);
                    }
                },
                1u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#concept_field = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("concept_field", value);
                    }
                },
                2u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#field_id = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("field_id", value);
                    }
                },
                3u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#concept_index = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("concept_index", value);
                    }
                },
                4u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#data_page = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("data_page", value);
                    }
                },
                5u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#concept_key = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("concept_key", value);
                    }
                },
                6u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#scaling = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("scaling", value);
                    }
                },
                8u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#data_units = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("data_units", value);
                    }
                },
                9u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#qualifier = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("qualifier", value);
                    }
                },
                10u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#descriptor = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("descriptor", value);
                    }
                },
                11u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#is_signed = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("is_signed", value);
                    }
                },
                _ => {
                    if !options.ignore_unexpected_fields {
                        return Err(TryFromRecordError::unexpected_field(&field));
                    }
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
            invalid_fields,
        })
    }
    fn invalid_fields(&self) -> &BTreeMap<&'static str, ValueWithUnits> {
        &self.invalid_fields
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
    pub r#reference_mesg: Option<field_types::MesgNum>,
    pub r#reference_index: Option<field_types::MessageIndex>,
    pub r#avg_depth: Option<ValueWithUnits>,
    pub r#max_depth: Option<ValueWithUnits>,
    pub r#surface_interval: Option<ValueWithUnits>,
    pub r#start_cns: Option<ValueWithUnits>,
    pub r#end_cns: Option<ValueWithUnits>,
    pub r#start_n2: Option<ValueWithUnits>,
    pub r#end_n2: Option<ValueWithUnits>,
    pub r#o2_toxicity: Option<ValueWithUnits>,
    pub r#dive_number: Option<ValueWithUnits>,
    pub r#bottom_time: Option<ValueWithUnits>,
    pub r#avg_pressure_sac: Option<ValueWithUnits>,
    pub r#avg_volume_sac: Option<ValueWithUnits>,
    pub r#avg_rmv: Option<ValueWithUnits>,
    pub r#descent_time: Option<ValueWithUnits>,
    pub r#ascent_time: Option<ValueWithUnits>,
    pub r#avg_ascent_rate: Option<ValueWithUnits>,
    pub r#avg_descent_rate: Option<ValueWithUnits>,
    pub r#max_ascent_rate: Option<ValueWithUnits>,
    pub r#max_descent_rate: Option<ValueWithUnits>,
    pub r#hang_time: Option<ValueWithUnits>,
    pub r#timestamp: Option<field_types::DateTime>,
    pub invalid_fields: BTreeMap<&'static str, ValueWithUnits>,
}
impl FitMessage for DiveSummary {
    const NAME: &'static str = "DiveSummary";
    const KIND: MesgNum = MesgNum::DiveSummary;
    fn parse_with_options(
        record: FitDataRecord,
        options: MessageParseOptions,
    ) -> Result<Self, TryFromRecordError> {
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
        #[allow(unused_mut)]
        let mut invalid_fields = BTreeMap::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#reference_mesg = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("reference_mesg", value);
                    }
                },
                1u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#reference_index = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("reference_index", value);
                    }
                },
                2u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#avg_depth = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("avg_depth", value);
                    }
                },
                3u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#max_depth = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("max_depth", value);
                    }
                },
                4u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#surface_interval = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("surface_interval", value);
                    }
                },
                5u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#start_cns = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("start_cns", value);
                    }
                },
                6u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#end_cns = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("end_cns", value);
                    }
                },
                7u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#start_n2 = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("start_n2", value);
                    }
                },
                8u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#end_n2 = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("end_n2", value);
                    }
                },
                9u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#o2_toxicity = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("o2_toxicity", value);
                    }
                },
                10u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#dive_number = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("dive_number", value);
                    }
                },
                11u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#bottom_time = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("bottom_time", value);
                    }
                },
                12u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#avg_pressure_sac = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("avg_pressure_sac", value);
                    }
                },
                13u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#avg_volume_sac = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("avg_volume_sac", value);
                    }
                },
                14u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#avg_rmv = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("avg_rmv", value);
                    }
                },
                15u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#descent_time = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("descent_time", value);
                    }
                },
                16u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#ascent_time = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("ascent_time", value);
                    }
                },
                17u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#avg_ascent_rate = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("avg_ascent_rate", value);
                    }
                },
                22u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#avg_descent_rate = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("avg_descent_rate", value);
                    }
                },
                23u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#max_ascent_rate = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("max_ascent_rate", value);
                    }
                },
                24u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#max_descent_rate = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("max_descent_rate", value);
                    }
                },
                25u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#hang_time = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("hang_time", value);
                    }
                },
                253u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#timestamp = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("timestamp", value);
                    }
                },
                _ => {
                    if !options.ignore_unexpected_fields {
                        return Err(TryFromRecordError::unexpected_field(&field));
                    }
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
            invalid_fields,
        })
    }
    fn invalid_fields(&self) -> &BTreeMap<&'static str, ValueWithUnits> {
        &self.invalid_fields
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
    pub r#time: Option<ValueWithUnits>,
    pub r#energy_total: Option<ValueWithUnits>,
    pub r#zero_cross_cnt: Option<ValueWithUnits>,
    pub r#instance: Option<ValueWithUnits>,
    pub r#time_above_threshold: Option<ValueWithUnits>,
    pub r#timestamp: Option<field_types::DateTime>,
    pub invalid_fields: BTreeMap<&'static str, ValueWithUnits>,
}
impl FitMessage for AadAccelFeatures {
    const NAME: &'static str = "AadAccelFeatures";
    const KIND: MesgNum = MesgNum::AadAccelFeatures;
    fn parse_with_options(
        record: FitDataRecord,
        options: MessageParseOptions,
    ) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#time = None;
        let mut r#energy_total = None;
        let mut r#zero_cross_cnt = None;
        let mut r#instance = None;
        let mut r#time_above_threshold = None;
        let mut r#timestamp = None;
        #[allow(unused_mut)]
        let mut invalid_fields = BTreeMap::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#time = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("time", value);
                    }
                },
                1u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#energy_total = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("energy_total", value);
                    }
                },
                2u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#zero_cross_cnt = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("zero_cross_cnt", value);
                    }
                },
                3u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#instance = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("instance", value);
                    }
                },
                4u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#time_above_threshold = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("time_above_threshold", value);
                    }
                },
                253u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#timestamp = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("timestamp", value);
                    }
                },
                _ => {
                    if !options.ignore_unexpected_fields {
                        return Err(TryFromRecordError::unexpected_field(&field));
                    }
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
            invalid_fields,
        })
    }
    fn invalid_fields(&self) -> &BTreeMap<&'static str, ValueWithUnits> {
        &self.invalid_fields
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
    pub r#time: Option<ValueWithUnits>,
    pub invalid_fields: BTreeMap<&'static str, ValueWithUnits>,
}
impl FitMessage for Hrv {
    const NAME: &'static str = "Hrv";
    const KIND: MesgNum = MesgNum::Hrv;
    fn parse_with_options(
        record: FitDataRecord,
        options: MessageParseOptions,
    ) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#time = None;
        #[allow(unused_mut)]
        let mut invalid_fields = BTreeMap::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#time = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("time", value);
                    }
                },
                _ => {
                    if !options.ignore_unexpected_fields {
                        return Err(TryFromRecordError::unexpected_field(&field));
                    }
                }
            }
        }
        Ok(Self {
            r#time,
            invalid_fields,
        })
    }
    fn invalid_fields(&self) -> &BTreeMap<&'static str, ValueWithUnits> {
        &self.invalid_fields
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
    pub r#timestamp_ms: Option<ValueWithUnits>,
    pub r#time: Option<ValueWithUnits>,
    pub r#timestamp: Option<field_types::DateTime>,
    pub invalid_fields: BTreeMap<&'static str, ValueWithUnits>,
}
impl FitMessage for BeatIntervals {
    const NAME: &'static str = "BeatIntervals";
    const KIND: MesgNum = MesgNum::BeatIntervals;
    fn parse_with_options(
        record: FitDataRecord,
        options: MessageParseOptions,
    ) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#timestamp_ms = None;
        let mut r#time = None;
        let mut r#timestamp = None;
        #[allow(unused_mut)]
        let mut invalid_fields = BTreeMap::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#timestamp_ms = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("timestamp_ms", value);
                    }
                },
                1u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#time = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("time", value);
                    }
                },
                253u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#timestamp = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("timestamp", value);
                    }
                },
                _ => {
                    if !options.ignore_unexpected_fields {
                        return Err(TryFromRecordError::unexpected_field(&field));
                    }
                }
            }
        }
        Ok(Self {
            r#timestamp_ms,
            r#time,
            r#timestamp,
            invalid_fields,
        })
    }
    fn invalid_fields(&self) -> &BTreeMap<&'static str, ValueWithUnits> {
        &self.invalid_fields
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
    pub r#weekly_average: Option<ValueWithUnits>,
    pub r#last_night_average: Option<ValueWithUnits>,
    pub r#last_night_5_min_high: Option<ValueWithUnits>,
    pub r#baseline_low_upper: Option<ValueWithUnits>,
    pub r#baseline_balanced_lower: Option<ValueWithUnits>,
    pub r#baseline_balanced_upper: Option<ValueWithUnits>,
    pub r#status: Option<field_types::HrvStatus>,
    pub r#timestamp: Option<field_types::DateTime>,
    pub invalid_fields: BTreeMap<&'static str, ValueWithUnits>,
}
impl FitMessage for HrvStatusSummary {
    const NAME: &'static str = "HrvStatusSummary";
    const KIND: MesgNum = MesgNum::HrvStatusSummary;
    fn parse_with_options(
        record: FitDataRecord,
        options: MessageParseOptions,
    ) -> Result<Self, TryFromRecordError> {
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
        #[allow(unused_mut)]
        let mut invalid_fields = BTreeMap::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#weekly_average = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("weekly_average", value);
                    }
                },
                1u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#last_night_average = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("last_night_average", value);
                    }
                },
                2u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#last_night_5_min_high = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("last_night_5_min_high", value);
                    }
                },
                3u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#baseline_low_upper = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("baseline_low_upper", value);
                    }
                },
                4u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#baseline_balanced_lower = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("baseline_balanced_lower", value);
                    }
                },
                5u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#baseline_balanced_upper = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("baseline_balanced_upper", value);
                    }
                },
                6u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#status = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("status", value);
                    }
                },
                253u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#timestamp = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("timestamp", value);
                    }
                },
                _ => {
                    if !options.ignore_unexpected_fields {
                        return Err(TryFromRecordError::unexpected_field(&field));
                    }
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
            invalid_fields,
        })
    }
    fn invalid_fields(&self) -> &BTreeMap<&'static str, ValueWithUnits> {
        &self.invalid_fields
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
    pub r#value: Option<ValueWithUnits>,
    pub r#timestamp: Option<field_types::DateTime>,
    pub invalid_fields: BTreeMap<&'static str, ValueWithUnits>,
}
impl FitMessage for HrvValue {
    const NAME: &'static str = "HrvValue";
    const KIND: MesgNum = MesgNum::HrvValue;
    fn parse_with_options(
        record: FitDataRecord,
        options: MessageParseOptions,
    ) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#value = None;
        let mut r#timestamp = None;
        #[allow(unused_mut)]
        let mut invalid_fields = BTreeMap::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#value = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("value", value);
                    }
                },
                253u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#timestamp = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("timestamp", value);
                    }
                },
                _ => {
                    if !options.ignore_unexpected_fields {
                        return Err(TryFromRecordError::unexpected_field(&field));
                    }
                }
            }
        }
        Ok(Self {
            r#value,
            r#timestamp,
            invalid_fields,
        })
    }
    fn invalid_fields(&self) -> &BTreeMap<&'static str, ValueWithUnits> {
        &self.invalid_fields
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
    pub r#timestamp_ms: Option<ValueWithUnits>,
    pub r#data: Option<ValueWithUnits>,
    pub r#time: Option<ValueWithUnits>,
    pub r#quality: Option<ValueWithUnits>,
    pub r#gap: Option<ValueWithUnits>,
    pub r#timestamp: Option<field_types::DateTime>,
    pub invalid_fields: BTreeMap<&'static str, ValueWithUnits>,
}
impl FitMessage for RawBbi {
    const NAME: &'static str = "RawBbi";
    const KIND: MesgNum = MesgNum::RawBbi;
    fn parse_with_options(
        record: FitDataRecord,
        options: MessageParseOptions,
    ) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#timestamp_ms = None;
        let mut r#data = None;
        let mut r#time = None;
        let mut r#quality = None;
        let mut r#gap = None;
        let mut r#timestamp = None;
        #[allow(unused_mut)]
        let mut invalid_fields = BTreeMap::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#timestamp_ms = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("timestamp_ms", value);
                    }
                },
                1u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#data = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("data", value);
                    }
                },
                2u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#time = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("time", value);
                    }
                },
                3u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#quality = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("quality", value);
                    }
                },
                4u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#gap = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("gap", value);
                    }
                },
                253u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#timestamp = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("timestamp", value);
                    }
                },
                _ => {
                    if !options.ignore_unexpected_fields {
                        return Err(TryFromRecordError::unexpected_field(&field));
                    }
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
            invalid_fields,
        })
    }
    fn invalid_fields(&self) -> &BTreeMap<&'static str, ValueWithUnits> {
        &self.invalid_fields
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
    pub r#respiration_rate: Option<ValueWithUnits>,
    pub r#timestamp: Option<field_types::DateTime>,
    pub invalid_fields: BTreeMap<&'static str, ValueWithUnits>,
}
impl FitMessage for RespirationRate {
    const NAME: &'static str = "RespirationRate";
    const KIND: MesgNum = MesgNum::RespirationRate;
    fn parse_with_options(
        record: FitDataRecord,
        options: MessageParseOptions,
    ) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#respiration_rate = None;
        let mut r#timestamp = None;
        #[allow(unused_mut)]
        let mut invalid_fields = BTreeMap::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#respiration_rate = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("respiration_rate", value);
                    }
                },
                253u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#timestamp = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("timestamp", value);
                    }
                },
                _ => {
                    if !options.ignore_unexpected_fields {
                        return Err(TryFromRecordError::unexpected_field(&field));
                    }
                }
            }
        }
        Ok(Self {
            r#respiration_rate,
            r#timestamp,
            invalid_fields,
        })
    }
    fn invalid_fields(&self) -> &BTreeMap<&'static str, ValueWithUnits> {
        &self.invalid_fields
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
    pub r#min_speed: Option<ValueWithUnits>,
    pub r#max_speed: Option<ValueWithUnits>,
    pub r#avg_speed: Option<ValueWithUnits>,
    pub r#shot_count: Option<ValueWithUnits>,
    pub r#projectile_type: Option<field_types::ProjectileType>,
    pub r#grain_weight: Option<ValueWithUnits>,
    pub r#timestamp: Option<field_types::DateTime>,
    pub invalid_fields: BTreeMap<&'static str, ValueWithUnits>,
}
impl FitMessage for ChronoShotSession {
    const NAME: &'static str = "ChronoShotSession";
    const KIND: MesgNum = MesgNum::ChronoShotSession;
    fn parse_with_options(
        record: FitDataRecord,
        options: MessageParseOptions,
    ) -> Result<Self, TryFromRecordError> {
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
        #[allow(unused_mut)]
        let mut invalid_fields = BTreeMap::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#min_speed = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("min_speed", value);
                    }
                },
                1u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#max_speed = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("max_speed", value);
                    }
                },
                2u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#avg_speed = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("avg_speed", value);
                    }
                },
                3u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#shot_count = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("shot_count", value);
                    }
                },
                4u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#projectile_type = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("projectile_type", value);
                    }
                },
                5u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#grain_weight = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("grain_weight", value);
                    }
                },
                253u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#timestamp = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("timestamp", value);
                    }
                },
                _ => {
                    if !options.ignore_unexpected_fields {
                        return Err(TryFromRecordError::unexpected_field(&field));
                    }
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
            invalid_fields,
        })
    }
    fn invalid_fields(&self) -> &BTreeMap<&'static str, ValueWithUnits> {
        &self.invalid_fields
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
    pub r#shot_speed: Option<ValueWithUnits>,
    pub r#shot_num: Option<ValueWithUnits>,
    pub r#timestamp: Option<field_types::DateTime>,
    pub invalid_fields: BTreeMap<&'static str, ValueWithUnits>,
}
impl FitMessage for ChronoShotData {
    const NAME: &'static str = "ChronoShotData";
    const KIND: MesgNum = MesgNum::ChronoShotData;
    fn parse_with_options(
        record: FitDataRecord,
        options: MessageParseOptions,
    ) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#shot_speed = None;
        let mut r#shot_num = None;
        let mut r#timestamp = None;
        #[allow(unused_mut)]
        let mut invalid_fields = BTreeMap::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#shot_speed = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("shot_speed", value);
                    }
                },
                1u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#shot_num = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("shot_num", value);
                    }
                },
                253u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#timestamp = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("timestamp", value);
                    }
                },
                _ => {
                    if !options.ignore_unexpected_fields {
                        return Err(TryFromRecordError::unexpected_field(&field));
                    }
                }
            }
        }
        Ok(Self {
            r#shot_speed,
            r#shot_num,
            r#timestamp,
            invalid_fields,
        })
    }
    fn invalid_fields(&self) -> &BTreeMap<&'static str, ValueWithUnits> {
        &self.invalid_fields
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
    pub r#sensor: Option<field_types::AntChannelId>,
    pub r#pressure: Option<ValueWithUnits>,
    pub r#timestamp: Option<field_types::DateTime>,
    pub invalid_fields: BTreeMap<&'static str, ValueWithUnits>,
}
impl FitMessage for TankUpdate {
    const NAME: &'static str = "TankUpdate";
    const KIND: MesgNum = MesgNum::TankUpdate;
    fn parse_with_options(
        record: FitDataRecord,
        options: MessageParseOptions,
    ) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#sensor = None;
        let mut r#pressure = None;
        let mut r#timestamp = None;
        #[allow(unused_mut)]
        let mut invalid_fields = BTreeMap::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#sensor = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("sensor", value);
                    }
                },
                1u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#pressure = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("pressure", value);
                    }
                },
                253u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#timestamp = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("timestamp", value);
                    }
                },
                _ => {
                    if !options.ignore_unexpected_fields {
                        return Err(TryFromRecordError::unexpected_field(&field));
                    }
                }
            }
        }
        Ok(Self {
            r#sensor,
            r#pressure,
            r#timestamp,
            invalid_fields,
        })
    }
    fn invalid_fields(&self) -> &BTreeMap<&'static str, ValueWithUnits> {
        &self.invalid_fields
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
    pub r#sensor: Option<field_types::AntChannelId>,
    pub r#start_pressure: Option<ValueWithUnits>,
    pub r#end_pressure: Option<ValueWithUnits>,
    pub r#volume_used: Option<ValueWithUnits>,
    pub r#timestamp: Option<field_types::DateTime>,
    pub invalid_fields: BTreeMap<&'static str, ValueWithUnits>,
}
impl FitMessage for TankSummary {
    const NAME: &'static str = "TankSummary";
    const KIND: MesgNum = MesgNum::TankSummary;
    fn parse_with_options(
        record: FitDataRecord,
        options: MessageParseOptions,
    ) -> Result<Self, TryFromRecordError> {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        let mut r#sensor = None;
        let mut r#start_pressure = None;
        let mut r#end_pressure = None;
        let mut r#volume_used = None;
        let mut r#timestamp = None;
        #[allow(unused_mut)]
        let mut invalid_fields = BTreeMap::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#sensor = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("sensor", value);
                    }
                },
                1u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#start_pressure = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("start_pressure", value);
                    }
                },
                2u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#end_pressure = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("end_pressure", value);
                    }
                },
                3u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#volume_used = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("volume_used", value);
                    }
                },
                253u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#timestamp = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("timestamp", value);
                    }
                },
                _ => {
                    if !options.ignore_unexpected_fields {
                        return Err(TryFromRecordError::unexpected_field(&field));
                    }
                }
            }
        }
        Ok(Self {
            r#sensor,
            r#start_pressure,
            r#end_pressure,
            r#volume_used,
            r#timestamp,
            invalid_fields,
        })
    }
    fn invalid_fields(&self) -> &BTreeMap<&'static str, ValueWithUnits> {
        &self.invalid_fields
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
    pub r#combined_awake_score: Option<ValueWithUnits>,
    pub r#awake_time_score: Option<ValueWithUnits>,
    pub r#awakenings_count_score: Option<ValueWithUnits>,
    pub r#deep_sleep_score: Option<ValueWithUnits>,
    pub r#sleep_duration_score: Option<ValueWithUnits>,
    pub r#light_sleep_score: Option<ValueWithUnits>,
    pub r#overall_sleep_score: Option<ValueWithUnits>,
    pub r#sleep_quality_score: Option<ValueWithUnits>,
    pub r#sleep_recovery_score: Option<ValueWithUnits>,
    pub r#rem_sleep_score: Option<ValueWithUnits>,
    pub r#sleep_restlessness_score: Option<ValueWithUnits>,
    pub r#awakenings_count: Option<ValueWithUnits>,
    pub r#interruptions_score: Option<ValueWithUnits>,
    pub r#average_stress_during_sleep: Option<ValueWithUnits>,
    pub invalid_fields: BTreeMap<&'static str, ValueWithUnits>,
}
impl FitMessage for SleepAssessment {
    const NAME: &'static str = "SleepAssessment";
    const KIND: MesgNum = MesgNum::SleepAssessment;
    fn parse_with_options(
        record: FitDataRecord,
        options: MessageParseOptions,
    ) -> Result<Self, TryFromRecordError> {
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
        #[allow(unused_mut)]
        let mut invalid_fields = BTreeMap::new();
        for field in record.into_vec() {
            match field.number() {
                0u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#combined_awake_score = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("combined_awake_score", value);
                    }
                },
                1u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#awake_time_score = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("awake_time_score", value);
                    }
                },
                2u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#awakenings_count_score = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("awakenings_count_score", value);
                    }
                },
                3u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#deep_sleep_score = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("deep_sleep_score", value);
                    }
                },
                4u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#sleep_duration_score = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("sleep_duration_score", value);
                    }
                },
                5u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#light_sleep_score = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("light_sleep_score", value);
                    }
                },
                6u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#overall_sleep_score = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("overall_sleep_score", value);
                    }
                },
                7u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#sleep_quality_score = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("sleep_quality_score", value);
                    }
                },
                8u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#sleep_recovery_score = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("sleep_recovery_score", value);
                    }
                },
                9u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#rem_sleep_score = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("rem_sleep_score", value);
                    }
                },
                10u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#sleep_restlessness_score = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("sleep_restlessness_score", value);
                    }
                },
                11u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#awakenings_count = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("awakenings_count", value);
                    }
                },
                14u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#interruptions_score = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("interruptions_score", value);
                    }
                },
                15u8 => match FromValue::from_value_with_units(field.into()) {
                    Ok(_value) => {
                        r#average_stress_during_sleep = Some(_value);
                    }
                    Err(value) => {
                        invalid_fields.insert("average_stress_during_sleep", value);
                    }
                },
                _ => {
                    if !options.ignore_unexpected_fields {
                        return Err(TryFromRecordError::unexpected_field(&field));
                    }
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
            invalid_fields,
        })
    }
    fn invalid_fields(&self) -> &BTreeMap<&'static str, ValueWithUnits> {
        &self.invalid_fields
    }
}
impl TryFrom<FitDataRecord> for SleepAssessment {
    type Error = TryFromRecordError;
    fn try_from(record: FitDataRecord) -> Result<Self, Self::Error> {
        Self::parse(record)
    }
}
