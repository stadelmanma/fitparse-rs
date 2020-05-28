//! Auto generated profile field types from FIT SDK Release: XXX
//! Not all of these may be used by the defined set of FIT messages
#![allow(missing_docs)]
#![allow(dead_code)]
#![allow(clippy::unreadable_literal)]
use std::convert;
use std::fmt;
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum File {
    /// Read only, single file. Must be in root directory.
    Device,
    /// Read/write, single file. Directory=Settings
    Settings,
    /// Read/write, multiple files, file number = sport type. Directory=Sports
    Sport,
    /// Read/erase, multiple files. Directory=Activities
    Activity,
    /// Read/write/erase, multiple files. Directory=Workouts
    Workout,
    /// Read/write/erase, multiple files. Directory=Courses
    Course,
    /// Read/write, single file. Directory=Schedules
    Schedules,
    /// Read only, single file. Circular buffer. All message definitions at start of file. Directory=Weight
    Weight,
    /// Read only, single file. Directory=Totals
    Totals,
    /// Read/write, single file. Directory=Goals
    Goals,
    /// Read only. Directory=Blood Pressure
    BloodPressure,
    /// Read only. Directory=Monitoring. File number=sub type.
    MonitoringA,
    /// Read/erase, multiple files. Directory=Activities
    ActivitySummary,
    MonitoringDaily,
    /// Read only. Directory=Monitoring. File number=identifier
    MonitoringB,
    /// Read/write/erase. Multiple Files.  Directory=Segments
    Segment,
    /// Read/write/erase. Single File.  Directory=Segments
    SegmentList,
    /// Read/write/erase. Single File. Directory=Settings
    ExdConfiguration,
    /// 0xF7 - 0xFE reserved for manufacturer specific file types
    MfgRangeMin,
    /// 0xF7 - 0xFE reserved for manufacturer specific file types
    MfgRangeMax,
    UnknownVariant(u8),
}
impl File {
    pub fn as_u8(self) -> u8 {
        match self {
            File::Device => 1,
            File::Settings => 2,
            File::Sport => 3,
            File::Activity => 4,
            File::Workout => 5,
            File::Course => 6,
            File::Schedules => 7,
            File::Weight => 9,
            File::Totals => 10,
            File::Goals => 11,
            File::BloodPressure => 14,
            File::MonitoringA => 15,
            File::ActivitySummary => 20,
            File::MonitoringDaily => 28,
            File::MonitoringB => 32,
            File::Segment => 34,
            File::SegmentList => 35,
            File::ExdConfiguration => 40,
            File::MfgRangeMin => 247,
            File::MfgRangeMax => 254,
            File::UnknownVariant(value) => value,
        }
    }
    pub fn as_i64(self) -> i64 {
        self.as_u8() as i64
    }
}
impl fmt::Display for File {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            File::Device => writeln!(f, "device"),
            File::Settings => writeln!(f, "settings"),
            File::Sport => writeln!(f, "sport"),
            File::Activity => writeln!(f, "activity"),
            File::Workout => writeln!(f, "workout"),
            File::Course => writeln!(f, "course"),
            File::Schedules => writeln!(f, "schedules"),
            File::Weight => writeln!(f, "weight"),
            File::Totals => writeln!(f, "totals"),
            File::Goals => writeln!(f, "goals"),
            File::BloodPressure => writeln!(f, "blood_pressure"),
            File::MonitoringA => writeln!(f, "monitoring_a"),
            File::ActivitySummary => writeln!(f, "activity_summary"),
            File::MonitoringDaily => writeln!(f, "monitoring_daily"),
            File::MonitoringB => writeln!(f, "monitoring_b"),
            File::Segment => writeln!(f, "segment"),
            File::SegmentList => writeln!(f, "segment_list"),
            File::ExdConfiguration => writeln!(f, "exd_configuration"),
            File::MfgRangeMin => writeln!(f, "mfg_range_min"),
            File::MfgRangeMax => writeln!(f, "mfg_range_max"),
            File::UnknownVariant(value) => writeln!(f, "unknown_variant_{}", *value),
        }
    }
}
impl convert::From<u8> for File {
    fn from(value: u8) -> Self {
        match value {
            1 => File::Device,
            2 => File::Settings,
            3 => File::Sport,
            4 => File::Activity,
            5 => File::Workout,
            6 => File::Course,
            7 => File::Schedules,
            9 => File::Weight,
            10 => File::Totals,
            11 => File::Goals,
            14 => File::BloodPressure,
            15 => File::MonitoringA,
            20 => File::ActivitySummary,
            28 => File::MonitoringDaily,
            32 => File::MonitoringB,
            34 => File::Segment,
            35 => File::SegmentList,
            40 => File::ExdConfiguration,
            247 => File::MfgRangeMin,
            254 => File::MfgRangeMax,
            _ => File::UnknownVariant(value),
        }
    }
}
impl convert::From<i64> for File {
    fn from(value: i64) -> Self {
        File::from(value as u8)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum MesgNum {
    FileId,
    Capabilities,
    DeviceSettings,
    UserProfile,
    HrmProfile,
    SdmProfile,
    BikeProfile,
    ZonesTarget,
    HrZone,
    PowerZone,
    MetZone,
    Sport,
    Goal,
    Session,
    Lap,
    Record,
    Event,
    DeviceInfo,
    Workout,
    WorkoutStep,
    Schedule,
    WeightScale,
    Course,
    CoursePoint,
    Totals,
    Activity,
    Software,
    FileCapabilities,
    MesgCapabilities,
    FieldCapabilities,
    FileCreator,
    BloodPressure,
    SpeedZone,
    Monitoring,
    TrainingFile,
    Hrv,
    AntRx,
    AntTx,
    AntChannelId,
    Length,
    MonitoringInfo,
    Pad,
    SlaveDevice,
    Connectivity,
    WeatherConditions,
    WeatherAlert,
    CadenceZone,
    Hr,
    SegmentLap,
    MemoGlob,
    SegmentId,
    SegmentLeaderboardEntry,
    SegmentPoint,
    SegmentFile,
    WorkoutSession,
    WatchfaceSettings,
    GpsMetadata,
    CameraEvent,
    TimestampCorrelation,
    GyroscopeData,
    AccelerometerData,
    ThreeDSensorCalibration,
    VideoFrame,
    ObdiiData,
    NmeaSentence,
    AviationAttitude,
    Video,
    VideoTitle,
    VideoDescription,
    VideoClip,
    OhrSettings,
    ExdScreenConfiguration,
    ExdDataFieldConfiguration,
    ExdDataConceptConfiguration,
    FieldDescription,
    DeveloperDataId,
    MagnetometerData,
    BarometerData,
    OneDSensorCalibration,
    Set,
    StressLevel,
    DiveSettings,
    DiveGas,
    DiveAlarm,
    ExerciseTitle,
    DiveSummary,
    Jump,
    ClimbPro,
    /// 0xFF00 - 0xFFFE reserved for manufacturer specific messages
    MfgRangeMin,
    /// 0xFF00 - 0xFFFE reserved for manufacturer specific messages
    MfgRangeMax,
    UnknownVariant(u16),
}
impl MesgNum {
    pub fn as_u16(self) -> u16 {
        match self {
            MesgNum::FileId => 0,
            MesgNum::Capabilities => 1,
            MesgNum::DeviceSettings => 2,
            MesgNum::UserProfile => 3,
            MesgNum::HrmProfile => 4,
            MesgNum::SdmProfile => 5,
            MesgNum::BikeProfile => 6,
            MesgNum::ZonesTarget => 7,
            MesgNum::HrZone => 8,
            MesgNum::PowerZone => 9,
            MesgNum::MetZone => 10,
            MesgNum::Sport => 12,
            MesgNum::Goal => 15,
            MesgNum::Session => 18,
            MesgNum::Lap => 19,
            MesgNum::Record => 20,
            MesgNum::Event => 21,
            MesgNum::DeviceInfo => 23,
            MesgNum::Workout => 26,
            MesgNum::WorkoutStep => 27,
            MesgNum::Schedule => 28,
            MesgNum::WeightScale => 30,
            MesgNum::Course => 31,
            MesgNum::CoursePoint => 32,
            MesgNum::Totals => 33,
            MesgNum::Activity => 34,
            MesgNum::Software => 35,
            MesgNum::FileCapabilities => 37,
            MesgNum::MesgCapabilities => 38,
            MesgNum::FieldCapabilities => 39,
            MesgNum::FileCreator => 49,
            MesgNum::BloodPressure => 51,
            MesgNum::SpeedZone => 53,
            MesgNum::Monitoring => 55,
            MesgNum::TrainingFile => 72,
            MesgNum::Hrv => 78,
            MesgNum::AntRx => 80,
            MesgNum::AntTx => 81,
            MesgNum::AntChannelId => 82,
            MesgNum::Length => 101,
            MesgNum::MonitoringInfo => 103,
            MesgNum::Pad => 105,
            MesgNum::SlaveDevice => 106,
            MesgNum::Connectivity => 127,
            MesgNum::WeatherConditions => 128,
            MesgNum::WeatherAlert => 129,
            MesgNum::CadenceZone => 131,
            MesgNum::Hr => 132,
            MesgNum::SegmentLap => 142,
            MesgNum::MemoGlob => 145,
            MesgNum::SegmentId => 148,
            MesgNum::SegmentLeaderboardEntry => 149,
            MesgNum::SegmentPoint => 150,
            MesgNum::SegmentFile => 151,
            MesgNum::WorkoutSession => 158,
            MesgNum::WatchfaceSettings => 159,
            MesgNum::GpsMetadata => 160,
            MesgNum::CameraEvent => 161,
            MesgNum::TimestampCorrelation => 162,
            MesgNum::GyroscopeData => 164,
            MesgNum::AccelerometerData => 165,
            MesgNum::ThreeDSensorCalibration => 167,
            MesgNum::VideoFrame => 169,
            MesgNum::ObdiiData => 174,
            MesgNum::NmeaSentence => 177,
            MesgNum::AviationAttitude => 178,
            MesgNum::Video => 184,
            MesgNum::VideoTitle => 185,
            MesgNum::VideoDescription => 186,
            MesgNum::VideoClip => 187,
            MesgNum::OhrSettings => 188,
            MesgNum::ExdScreenConfiguration => 200,
            MesgNum::ExdDataFieldConfiguration => 201,
            MesgNum::ExdDataConceptConfiguration => 202,
            MesgNum::FieldDescription => 206,
            MesgNum::DeveloperDataId => 207,
            MesgNum::MagnetometerData => 208,
            MesgNum::BarometerData => 209,
            MesgNum::OneDSensorCalibration => 210,
            MesgNum::Set => 225,
            MesgNum::StressLevel => 227,
            MesgNum::DiveSettings => 258,
            MesgNum::DiveGas => 259,
            MesgNum::DiveAlarm => 262,
            MesgNum::ExerciseTitle => 264,
            MesgNum::DiveSummary => 268,
            MesgNum::Jump => 285,
            MesgNum::ClimbPro => 317,
            MesgNum::MfgRangeMin => 65280,
            MesgNum::MfgRangeMax => 65534,
            MesgNum::UnknownVariant(value) => value,
        }
    }
    pub fn as_i64(self) -> i64 {
        self.as_u16() as i64
    }
}
impl fmt::Display for MesgNum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            MesgNum::FileId => writeln!(f, "file_id"),
            MesgNum::Capabilities => writeln!(f, "capabilities"),
            MesgNum::DeviceSettings => writeln!(f, "device_settings"),
            MesgNum::UserProfile => writeln!(f, "user_profile"),
            MesgNum::HrmProfile => writeln!(f, "hrm_profile"),
            MesgNum::SdmProfile => writeln!(f, "sdm_profile"),
            MesgNum::BikeProfile => writeln!(f, "bike_profile"),
            MesgNum::ZonesTarget => writeln!(f, "zones_target"),
            MesgNum::HrZone => writeln!(f, "hr_zone"),
            MesgNum::PowerZone => writeln!(f, "power_zone"),
            MesgNum::MetZone => writeln!(f, "met_zone"),
            MesgNum::Sport => writeln!(f, "sport"),
            MesgNum::Goal => writeln!(f, "goal"),
            MesgNum::Session => writeln!(f, "session"),
            MesgNum::Lap => writeln!(f, "lap"),
            MesgNum::Record => writeln!(f, "record"),
            MesgNum::Event => writeln!(f, "event"),
            MesgNum::DeviceInfo => writeln!(f, "device_info"),
            MesgNum::Workout => writeln!(f, "workout"),
            MesgNum::WorkoutStep => writeln!(f, "workout_step"),
            MesgNum::Schedule => writeln!(f, "schedule"),
            MesgNum::WeightScale => writeln!(f, "weight_scale"),
            MesgNum::Course => writeln!(f, "course"),
            MesgNum::CoursePoint => writeln!(f, "course_point"),
            MesgNum::Totals => writeln!(f, "totals"),
            MesgNum::Activity => writeln!(f, "activity"),
            MesgNum::Software => writeln!(f, "software"),
            MesgNum::FileCapabilities => writeln!(f, "file_capabilities"),
            MesgNum::MesgCapabilities => writeln!(f, "mesg_capabilities"),
            MesgNum::FieldCapabilities => writeln!(f, "field_capabilities"),
            MesgNum::FileCreator => writeln!(f, "file_creator"),
            MesgNum::BloodPressure => writeln!(f, "blood_pressure"),
            MesgNum::SpeedZone => writeln!(f, "speed_zone"),
            MesgNum::Monitoring => writeln!(f, "monitoring"),
            MesgNum::TrainingFile => writeln!(f, "training_file"),
            MesgNum::Hrv => writeln!(f, "hrv"),
            MesgNum::AntRx => writeln!(f, "ant_rx"),
            MesgNum::AntTx => writeln!(f, "ant_tx"),
            MesgNum::AntChannelId => writeln!(f, "ant_channel_id"),
            MesgNum::Length => writeln!(f, "length"),
            MesgNum::MonitoringInfo => writeln!(f, "monitoring_info"),
            MesgNum::Pad => writeln!(f, "pad"),
            MesgNum::SlaveDevice => writeln!(f, "slave_device"),
            MesgNum::Connectivity => writeln!(f, "connectivity"),
            MesgNum::WeatherConditions => writeln!(f, "weather_conditions"),
            MesgNum::WeatherAlert => writeln!(f, "weather_alert"),
            MesgNum::CadenceZone => writeln!(f, "cadence_zone"),
            MesgNum::Hr => writeln!(f, "hr"),
            MesgNum::SegmentLap => writeln!(f, "segment_lap"),
            MesgNum::MemoGlob => writeln!(f, "memo_glob"),
            MesgNum::SegmentId => writeln!(f, "segment_id"),
            MesgNum::SegmentLeaderboardEntry => writeln!(f, "segment_leaderboard_entry"),
            MesgNum::SegmentPoint => writeln!(f, "segment_point"),
            MesgNum::SegmentFile => writeln!(f, "segment_file"),
            MesgNum::WorkoutSession => writeln!(f, "workout_session"),
            MesgNum::WatchfaceSettings => writeln!(f, "watchface_settings"),
            MesgNum::GpsMetadata => writeln!(f, "gps_metadata"),
            MesgNum::CameraEvent => writeln!(f, "camera_event"),
            MesgNum::TimestampCorrelation => writeln!(f, "timestamp_correlation"),
            MesgNum::GyroscopeData => writeln!(f, "gyroscope_data"),
            MesgNum::AccelerometerData => writeln!(f, "accelerometer_data"),
            MesgNum::ThreeDSensorCalibration => writeln!(f, "three_d_sensor_calibration"),
            MesgNum::VideoFrame => writeln!(f, "video_frame"),
            MesgNum::ObdiiData => writeln!(f, "obdii_data"),
            MesgNum::NmeaSentence => writeln!(f, "nmea_sentence"),
            MesgNum::AviationAttitude => writeln!(f, "aviation_attitude"),
            MesgNum::Video => writeln!(f, "video"),
            MesgNum::VideoTitle => writeln!(f, "video_title"),
            MesgNum::VideoDescription => writeln!(f, "video_description"),
            MesgNum::VideoClip => writeln!(f, "video_clip"),
            MesgNum::OhrSettings => writeln!(f, "ohr_settings"),
            MesgNum::ExdScreenConfiguration => writeln!(f, "exd_screen_configuration"),
            MesgNum::ExdDataFieldConfiguration => writeln!(f, "exd_data_field_configuration"),
            MesgNum::ExdDataConceptConfiguration => writeln!(f, "exd_data_concept_configuration"),
            MesgNum::FieldDescription => writeln!(f, "field_description"),
            MesgNum::DeveloperDataId => writeln!(f, "developer_data_id"),
            MesgNum::MagnetometerData => writeln!(f, "magnetometer_data"),
            MesgNum::BarometerData => writeln!(f, "barometer_data"),
            MesgNum::OneDSensorCalibration => writeln!(f, "one_d_sensor_calibration"),
            MesgNum::Set => writeln!(f, "set"),
            MesgNum::StressLevel => writeln!(f, "stress_level"),
            MesgNum::DiveSettings => writeln!(f, "dive_settings"),
            MesgNum::DiveGas => writeln!(f, "dive_gas"),
            MesgNum::DiveAlarm => writeln!(f, "dive_alarm"),
            MesgNum::ExerciseTitle => writeln!(f, "exercise_title"),
            MesgNum::DiveSummary => writeln!(f, "dive_summary"),
            MesgNum::Jump => writeln!(f, "jump"),
            MesgNum::ClimbPro => writeln!(f, "climb_pro"),
            MesgNum::MfgRangeMin => writeln!(f, "mfg_range_min"),
            MesgNum::MfgRangeMax => writeln!(f, "mfg_range_max"),
            MesgNum::UnknownVariant(value) => writeln!(f, "unknown_variant_{}", *value),
        }
    }
}
impl convert::From<u16> for MesgNum {
    fn from(value: u16) -> Self {
        match value {
            0 => MesgNum::FileId,
            1 => MesgNum::Capabilities,
            2 => MesgNum::DeviceSettings,
            3 => MesgNum::UserProfile,
            4 => MesgNum::HrmProfile,
            5 => MesgNum::SdmProfile,
            6 => MesgNum::BikeProfile,
            7 => MesgNum::ZonesTarget,
            8 => MesgNum::HrZone,
            9 => MesgNum::PowerZone,
            10 => MesgNum::MetZone,
            12 => MesgNum::Sport,
            15 => MesgNum::Goal,
            18 => MesgNum::Session,
            19 => MesgNum::Lap,
            20 => MesgNum::Record,
            21 => MesgNum::Event,
            23 => MesgNum::DeviceInfo,
            26 => MesgNum::Workout,
            27 => MesgNum::WorkoutStep,
            28 => MesgNum::Schedule,
            30 => MesgNum::WeightScale,
            31 => MesgNum::Course,
            32 => MesgNum::CoursePoint,
            33 => MesgNum::Totals,
            34 => MesgNum::Activity,
            35 => MesgNum::Software,
            37 => MesgNum::FileCapabilities,
            38 => MesgNum::MesgCapabilities,
            39 => MesgNum::FieldCapabilities,
            49 => MesgNum::FileCreator,
            51 => MesgNum::BloodPressure,
            53 => MesgNum::SpeedZone,
            55 => MesgNum::Monitoring,
            72 => MesgNum::TrainingFile,
            78 => MesgNum::Hrv,
            80 => MesgNum::AntRx,
            81 => MesgNum::AntTx,
            82 => MesgNum::AntChannelId,
            101 => MesgNum::Length,
            103 => MesgNum::MonitoringInfo,
            105 => MesgNum::Pad,
            106 => MesgNum::SlaveDevice,
            127 => MesgNum::Connectivity,
            128 => MesgNum::WeatherConditions,
            129 => MesgNum::WeatherAlert,
            131 => MesgNum::CadenceZone,
            132 => MesgNum::Hr,
            142 => MesgNum::SegmentLap,
            145 => MesgNum::MemoGlob,
            148 => MesgNum::SegmentId,
            149 => MesgNum::SegmentLeaderboardEntry,
            150 => MesgNum::SegmentPoint,
            151 => MesgNum::SegmentFile,
            158 => MesgNum::WorkoutSession,
            159 => MesgNum::WatchfaceSettings,
            160 => MesgNum::GpsMetadata,
            161 => MesgNum::CameraEvent,
            162 => MesgNum::TimestampCorrelation,
            164 => MesgNum::GyroscopeData,
            165 => MesgNum::AccelerometerData,
            167 => MesgNum::ThreeDSensorCalibration,
            169 => MesgNum::VideoFrame,
            174 => MesgNum::ObdiiData,
            177 => MesgNum::NmeaSentence,
            178 => MesgNum::AviationAttitude,
            184 => MesgNum::Video,
            185 => MesgNum::VideoTitle,
            186 => MesgNum::VideoDescription,
            187 => MesgNum::VideoClip,
            188 => MesgNum::OhrSettings,
            200 => MesgNum::ExdScreenConfiguration,
            201 => MesgNum::ExdDataFieldConfiguration,
            202 => MesgNum::ExdDataConceptConfiguration,
            206 => MesgNum::FieldDescription,
            207 => MesgNum::DeveloperDataId,
            208 => MesgNum::MagnetometerData,
            209 => MesgNum::BarometerData,
            210 => MesgNum::OneDSensorCalibration,
            225 => MesgNum::Set,
            227 => MesgNum::StressLevel,
            258 => MesgNum::DiveSettings,
            259 => MesgNum::DiveGas,
            262 => MesgNum::DiveAlarm,
            264 => MesgNum::ExerciseTitle,
            268 => MesgNum::DiveSummary,
            285 => MesgNum::Jump,
            317 => MesgNum::ClimbPro,
            65280 => MesgNum::MfgRangeMin,
            65534 => MesgNum::MfgRangeMax,
            _ => MesgNum::UnknownVariant(value),
        }
    }
}
impl convert::From<i64> for MesgNum {
    fn from(value: i64) -> Self {
        MesgNum::from(value as u16)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum Checksum {
    /// Allows clear of checksum for flash memory where can only write 1 to 0 without erasing sector.
    Clear,
    /// Set to mark checksum as valid if computes to invalid values 0 or 0xFF.  Checksum can also be set to ok to save encoding computation time.
    Ok,
    UnknownVariant(u8),
}
impl Checksum {
    pub fn as_u8(self) -> u8 {
        match self {
            Checksum::Clear => 0,
            Checksum::Ok => 1,
            Checksum::UnknownVariant(value) => value,
        }
    }
    pub fn as_i64(self) -> i64 {
        self.as_u8() as i64
    }
}
impl fmt::Display for Checksum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Checksum::Clear => writeln!(f, "clear"),
            Checksum::Ok => writeln!(f, "ok"),
            Checksum::UnknownVariant(value) => writeln!(f, "unknown_variant_{}", *value),
        }
    }
}
impl convert::From<u8> for Checksum {
    fn from(value: u8) -> Self {
        match value {
            0 => Checksum::Clear,
            1 => Checksum::Ok,
            _ => Checksum::UnknownVariant(value),
        }
    }
}
impl convert::From<i64> for Checksum {
    fn from(value: i64) -> Self {
        Checksum::from(value as u8)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum FileFlags {
    Read,
    Write,
    Erase,
    UnknownVariant(u8),
}
impl FileFlags {
    pub fn as_u8(self) -> u8 {
        match self {
            FileFlags::Read => 2,
            FileFlags::Write => 4,
            FileFlags::Erase => 8,
            FileFlags::UnknownVariant(value) => value,
        }
    }
    pub fn as_i64(self) -> i64 {
        self.as_u8() as i64
    }
}
impl fmt::Display for FileFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            FileFlags::Read => writeln!(f, "read"),
            FileFlags::Write => writeln!(f, "write"),
            FileFlags::Erase => writeln!(f, "erase"),
            FileFlags::UnknownVariant(value) => writeln!(f, "unknown_variant_{}", *value),
        }
    }
}
impl convert::From<u8> for FileFlags {
    fn from(value: u8) -> Self {
        match value {
            2 => FileFlags::Read,
            4 => FileFlags::Write,
            8 => FileFlags::Erase,
            _ => FileFlags::UnknownVariant(value),
        }
    }
}
impl convert::From<i64> for FileFlags {
    fn from(value: i64) -> Self {
        FileFlags::from(value as u8)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum MesgCount {
    NumPerFile,
    MaxPerFile,
    MaxPerFileType,
    UnknownVariant(u8),
}
impl MesgCount {
    pub fn as_u8(self) -> u8 {
        match self {
            MesgCount::NumPerFile => 0,
            MesgCount::MaxPerFile => 1,
            MesgCount::MaxPerFileType => 2,
            MesgCount::UnknownVariant(value) => value,
        }
    }
    pub fn as_i64(self) -> i64 {
        self.as_u8() as i64
    }
}
impl fmt::Display for MesgCount {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            MesgCount::NumPerFile => writeln!(f, "num_per_file"),
            MesgCount::MaxPerFile => writeln!(f, "max_per_file"),
            MesgCount::MaxPerFileType => writeln!(f, "max_per_file_type"),
            MesgCount::UnknownVariant(value) => writeln!(f, "unknown_variant_{}", *value),
        }
    }
}
impl convert::From<u8> for MesgCount {
    fn from(value: u8) -> Self {
        match value {
            0 => MesgCount::NumPerFile,
            1 => MesgCount::MaxPerFile,
            2 => MesgCount::MaxPerFileType,
            _ => MesgCount::UnknownVariant(value),
        }
    }
}
impl convert::From<i64> for MesgCount {
    fn from(value: i64) -> Self {
        MesgCount::from(value as u8)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum DateTime {
    /// if date_time is < 0x10000000 then it is system time (seconds from device power on)
    Min,
    UnknownVariant(u32),
}
impl DateTime {
    pub fn as_u32(self) -> u32 {
        match self {
            DateTime::Min => 268435456,
            DateTime::UnknownVariant(value) => value,
        }
    }
    pub fn as_i64(self) -> i64 {
        self.as_u32() as i64
    }
}
impl fmt::Display for DateTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            DateTime::Min => writeln!(f, "min"),
            DateTime::UnknownVariant(value) => writeln!(f, "unknown_variant_{}", *value),
        }
    }
}
impl convert::From<u32> for DateTime {
    fn from(value: u32) -> Self {
        match value {
            268435456 => DateTime::Min,
            _ => DateTime::UnknownVariant(value),
        }
    }
}
impl convert::From<i64> for DateTime {
    fn from(value: i64) -> Self {
        DateTime::from(value as u32)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum LocalDateTime {
    /// if date_time is < 0x10000000 then it is system time (seconds from device power on)
    Min,
    UnknownVariant(u32),
}
impl LocalDateTime {
    pub fn as_u32(self) -> u32 {
        match self {
            LocalDateTime::Min => 268435456,
            LocalDateTime::UnknownVariant(value) => value,
        }
    }
    pub fn as_i64(self) -> i64 {
        self.as_u32() as i64
    }
}
impl fmt::Display for LocalDateTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            LocalDateTime::Min => writeln!(f, "min"),
            LocalDateTime::UnknownVariant(value) => writeln!(f, "unknown_variant_{}", *value),
        }
    }
}
impl convert::From<u32> for LocalDateTime {
    fn from(value: u32) -> Self {
        match value {
            268435456 => LocalDateTime::Min,
            _ => LocalDateTime::UnknownVariant(value),
        }
    }
}
impl convert::From<i64> for LocalDateTime {
    fn from(value: i64) -> Self {
        LocalDateTime::from(value as u32)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum MessageIndex {
    /// index
    Mask,
    /// reserved (default 0)
    Reserved,
    /// message is selected if set
    Selected,
    UnknownVariant(u16),
}
impl MessageIndex {
    pub fn as_u16(self) -> u16 {
        match self {
            MessageIndex::Mask => 4095,
            MessageIndex::Reserved => 28672,
            MessageIndex::Selected => 32768,
            MessageIndex::UnknownVariant(value) => value,
        }
    }
    pub fn as_i64(self) -> i64 {
        self.as_u16() as i64
    }
}
impl fmt::Display for MessageIndex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            MessageIndex::Mask => writeln!(f, "mask"),
            MessageIndex::Reserved => writeln!(f, "reserved"),
            MessageIndex::Selected => writeln!(f, "selected"),
            MessageIndex::UnknownVariant(value) => writeln!(f, "unknown_variant_{}", *value),
        }
    }
}
impl convert::From<u16> for MessageIndex {
    fn from(value: u16) -> Self {
        match value {
            4095 => MessageIndex::Mask,
            28672 => MessageIndex::Reserved,
            32768 => MessageIndex::Selected,
            _ => MessageIndex::UnknownVariant(value),
        }
    }
}
impl convert::From<i64> for MessageIndex {
    fn from(value: i64) -> Self {
        MessageIndex::from(value as u16)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum DeviceIndex {
    /// Creator of the file is always device index 0.
    Creator,
    UnknownVariant(u8),
}
impl DeviceIndex {
    pub fn as_u8(self) -> u8 {
        match self {
            DeviceIndex::Creator => 0,
            DeviceIndex::UnknownVariant(value) => value,
        }
    }
    pub fn as_i64(self) -> i64 {
        self.as_u8() as i64
    }
}
impl fmt::Display for DeviceIndex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            DeviceIndex::Creator => writeln!(f, "creator"),
            DeviceIndex::UnknownVariant(value) => writeln!(f, "unknown_variant_{}", *value),
        }
    }
}
impl convert::From<u8> for DeviceIndex {
    fn from(value: u8) -> Self {
        match value {
            0 => DeviceIndex::Creator,
            _ => DeviceIndex::UnknownVariant(value),
        }
    }
}
impl convert::From<i64> for DeviceIndex {
    fn from(value: i64) -> Self {
        DeviceIndex::from(value as u8)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum Gender {
    Female,
    Male,
    UnknownVariant(u8),
}
impl Gender {
    pub fn as_u8(self) -> u8 {
        match self {
            Gender::Female => 0,
            Gender::Male => 1,
            Gender::UnknownVariant(value) => value,
        }
    }
    pub fn as_i64(self) -> i64 {
        self.as_u8() as i64
    }
}
impl fmt::Display for Gender {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Gender::Female => writeln!(f, "female"),
            Gender::Male => writeln!(f, "male"),
            Gender::UnknownVariant(value) => writeln!(f, "unknown_variant_{}", *value),
        }
    }
}
impl convert::From<u8> for Gender {
    fn from(value: u8) -> Self {
        match value {
            0 => Gender::Female,
            1 => Gender::Male,
            _ => Gender::UnknownVariant(value),
        }
    }
}
impl convert::From<i64> for Gender {
    fn from(value: i64) -> Self {
        Gender::from(value as u8)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum Language {
    English,
    French,
    Italian,
    German,
    Spanish,
    Croatian,
    Czech,
    Danish,
    Dutch,
    Finnish,
    Greek,
    Hungarian,
    Norwegian,
    Polish,
    Portuguese,
    Slovakian,
    Slovenian,
    Swedish,
    Russian,
    Turkish,
    Latvian,
    Ukrainian,
    Arabic,
    Farsi,
    Bulgarian,
    Romanian,
    Chinese,
    Japanese,
    Korean,
    Taiwanese,
    Thai,
    Hebrew,
    BrazilianPortuguese,
    Indonesian,
    Malaysian,
    Vietnamese,
    Burmese,
    Mongolian,
    Custom,
    UnknownVariant(u8),
}
impl Language {
    pub fn as_u8(self) -> u8 {
        match self {
            Language::English => 0,
            Language::French => 1,
            Language::Italian => 2,
            Language::German => 3,
            Language::Spanish => 4,
            Language::Croatian => 5,
            Language::Czech => 6,
            Language::Danish => 7,
            Language::Dutch => 8,
            Language::Finnish => 9,
            Language::Greek => 10,
            Language::Hungarian => 11,
            Language::Norwegian => 12,
            Language::Polish => 13,
            Language::Portuguese => 14,
            Language::Slovakian => 15,
            Language::Slovenian => 16,
            Language::Swedish => 17,
            Language::Russian => 18,
            Language::Turkish => 19,
            Language::Latvian => 20,
            Language::Ukrainian => 21,
            Language::Arabic => 22,
            Language::Farsi => 23,
            Language::Bulgarian => 24,
            Language::Romanian => 25,
            Language::Chinese => 26,
            Language::Japanese => 27,
            Language::Korean => 28,
            Language::Taiwanese => 29,
            Language::Thai => 30,
            Language::Hebrew => 31,
            Language::BrazilianPortuguese => 32,
            Language::Indonesian => 33,
            Language::Malaysian => 34,
            Language::Vietnamese => 35,
            Language::Burmese => 36,
            Language::Mongolian => 37,
            Language::Custom => 254,
            Language::UnknownVariant(value) => value,
        }
    }
    pub fn as_i64(self) -> i64 {
        self.as_u8() as i64
    }
}
impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Language::English => writeln!(f, "english"),
            Language::French => writeln!(f, "french"),
            Language::Italian => writeln!(f, "italian"),
            Language::German => writeln!(f, "german"),
            Language::Spanish => writeln!(f, "spanish"),
            Language::Croatian => writeln!(f, "croatian"),
            Language::Czech => writeln!(f, "czech"),
            Language::Danish => writeln!(f, "danish"),
            Language::Dutch => writeln!(f, "dutch"),
            Language::Finnish => writeln!(f, "finnish"),
            Language::Greek => writeln!(f, "greek"),
            Language::Hungarian => writeln!(f, "hungarian"),
            Language::Norwegian => writeln!(f, "norwegian"),
            Language::Polish => writeln!(f, "polish"),
            Language::Portuguese => writeln!(f, "portuguese"),
            Language::Slovakian => writeln!(f, "slovakian"),
            Language::Slovenian => writeln!(f, "slovenian"),
            Language::Swedish => writeln!(f, "swedish"),
            Language::Russian => writeln!(f, "russian"),
            Language::Turkish => writeln!(f, "turkish"),
            Language::Latvian => writeln!(f, "latvian"),
            Language::Ukrainian => writeln!(f, "ukrainian"),
            Language::Arabic => writeln!(f, "arabic"),
            Language::Farsi => writeln!(f, "farsi"),
            Language::Bulgarian => writeln!(f, "bulgarian"),
            Language::Romanian => writeln!(f, "romanian"),
            Language::Chinese => writeln!(f, "chinese"),
            Language::Japanese => writeln!(f, "japanese"),
            Language::Korean => writeln!(f, "korean"),
            Language::Taiwanese => writeln!(f, "taiwanese"),
            Language::Thai => writeln!(f, "thai"),
            Language::Hebrew => writeln!(f, "hebrew"),
            Language::BrazilianPortuguese => writeln!(f, "brazilian_portuguese"),
            Language::Indonesian => writeln!(f, "indonesian"),
            Language::Malaysian => writeln!(f, "malaysian"),
            Language::Vietnamese => writeln!(f, "vietnamese"),
            Language::Burmese => writeln!(f, "burmese"),
            Language::Mongolian => writeln!(f, "mongolian"),
            Language::Custom => writeln!(f, "custom"),
            Language::UnknownVariant(value) => writeln!(f, "unknown_variant_{}", *value),
        }
    }
}
impl convert::From<u8> for Language {
    fn from(value: u8) -> Self {
        match value {
            0 => Language::English,
            1 => Language::French,
            2 => Language::Italian,
            3 => Language::German,
            4 => Language::Spanish,
            5 => Language::Croatian,
            6 => Language::Czech,
            7 => Language::Danish,
            8 => Language::Dutch,
            9 => Language::Finnish,
            10 => Language::Greek,
            11 => Language::Hungarian,
            12 => Language::Norwegian,
            13 => Language::Polish,
            14 => Language::Portuguese,
            15 => Language::Slovakian,
            16 => Language::Slovenian,
            17 => Language::Swedish,
            18 => Language::Russian,
            19 => Language::Turkish,
            20 => Language::Latvian,
            21 => Language::Ukrainian,
            22 => Language::Arabic,
            23 => Language::Farsi,
            24 => Language::Bulgarian,
            25 => Language::Romanian,
            26 => Language::Chinese,
            27 => Language::Japanese,
            28 => Language::Korean,
            29 => Language::Taiwanese,
            30 => Language::Thai,
            31 => Language::Hebrew,
            32 => Language::BrazilianPortuguese,
            33 => Language::Indonesian,
            34 => Language::Malaysian,
            35 => Language::Vietnamese,
            36 => Language::Burmese,
            37 => Language::Mongolian,
            254 => Language::Custom,
            _ => Language::UnknownVariant(value),
        }
    }
}
impl convert::From<i64> for Language {
    fn from(value: i64) -> Self {
        Language::from(value as u8)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum LanguageBits0 {
    English,
    French,
    Italian,
    German,
    Spanish,
    Croatian,
    Czech,
    Danish,
    UnknownVariant(u8),
}
impl LanguageBits0 {
    pub fn as_u8(self) -> u8 {
        match self {
            LanguageBits0::English => 1,
            LanguageBits0::French => 2,
            LanguageBits0::Italian => 4,
            LanguageBits0::German => 8,
            LanguageBits0::Spanish => 16,
            LanguageBits0::Croatian => 32,
            LanguageBits0::Czech => 64,
            LanguageBits0::Danish => 128,
            LanguageBits0::UnknownVariant(value) => value,
        }
    }
    pub fn as_i64(self) -> i64 {
        self.as_u8() as i64
    }
}
impl fmt::Display for LanguageBits0 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            LanguageBits0::English => writeln!(f, "english"),
            LanguageBits0::French => writeln!(f, "french"),
            LanguageBits0::Italian => writeln!(f, "italian"),
            LanguageBits0::German => writeln!(f, "german"),
            LanguageBits0::Spanish => writeln!(f, "spanish"),
            LanguageBits0::Croatian => writeln!(f, "croatian"),
            LanguageBits0::Czech => writeln!(f, "czech"),
            LanguageBits0::Danish => writeln!(f, "danish"),
            LanguageBits0::UnknownVariant(value) => writeln!(f, "unknown_variant_{}", *value),
        }
    }
}
impl convert::From<u8> for LanguageBits0 {
    fn from(value: u8) -> Self {
        match value {
            1 => LanguageBits0::English,
            2 => LanguageBits0::French,
            4 => LanguageBits0::Italian,
            8 => LanguageBits0::German,
            16 => LanguageBits0::Spanish,
            32 => LanguageBits0::Croatian,
            64 => LanguageBits0::Czech,
            128 => LanguageBits0::Danish,
            _ => LanguageBits0::UnknownVariant(value),
        }
    }
}
impl convert::From<i64> for LanguageBits0 {
    fn from(value: i64) -> Self {
        LanguageBits0::from(value as u8)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum LanguageBits1 {
    Dutch,
    Finnish,
    Greek,
    Hungarian,
    Norwegian,
    Polish,
    Portuguese,
    Slovakian,
    UnknownVariant(u8),
}
impl LanguageBits1 {
    pub fn as_u8(self) -> u8 {
        match self {
            LanguageBits1::Dutch => 1,
            LanguageBits1::Finnish => 2,
            LanguageBits1::Greek => 4,
            LanguageBits1::Hungarian => 8,
            LanguageBits1::Norwegian => 16,
            LanguageBits1::Polish => 32,
            LanguageBits1::Portuguese => 64,
            LanguageBits1::Slovakian => 128,
            LanguageBits1::UnknownVariant(value) => value,
        }
    }
    pub fn as_i64(self) -> i64 {
        self.as_u8() as i64
    }
}
impl fmt::Display for LanguageBits1 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            LanguageBits1::Dutch => writeln!(f, "dutch"),
            LanguageBits1::Finnish => writeln!(f, "finnish"),
            LanguageBits1::Greek => writeln!(f, "greek"),
            LanguageBits1::Hungarian => writeln!(f, "hungarian"),
            LanguageBits1::Norwegian => writeln!(f, "norwegian"),
            LanguageBits1::Polish => writeln!(f, "polish"),
            LanguageBits1::Portuguese => writeln!(f, "portuguese"),
            LanguageBits1::Slovakian => writeln!(f, "slovakian"),
            LanguageBits1::UnknownVariant(value) => writeln!(f, "unknown_variant_{}", *value),
        }
    }
}
impl convert::From<u8> for LanguageBits1 {
    fn from(value: u8) -> Self {
        match value {
            1 => LanguageBits1::Dutch,
            2 => LanguageBits1::Finnish,
            4 => LanguageBits1::Greek,
            8 => LanguageBits1::Hungarian,
            16 => LanguageBits1::Norwegian,
            32 => LanguageBits1::Polish,
            64 => LanguageBits1::Portuguese,
            128 => LanguageBits1::Slovakian,
            _ => LanguageBits1::UnknownVariant(value),
        }
    }
}
impl convert::From<i64> for LanguageBits1 {
    fn from(value: i64) -> Self {
        LanguageBits1::from(value as u8)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum LanguageBits2 {
    Slovenian,
    Swedish,
    Russian,
    Turkish,
    Latvian,
    Ukrainian,
    Arabic,
    Farsi,
    UnknownVariant(u8),
}
impl LanguageBits2 {
    pub fn as_u8(self) -> u8 {
        match self {
            LanguageBits2::Slovenian => 1,
            LanguageBits2::Swedish => 2,
            LanguageBits2::Russian => 4,
            LanguageBits2::Turkish => 8,
            LanguageBits2::Latvian => 16,
            LanguageBits2::Ukrainian => 32,
            LanguageBits2::Arabic => 64,
            LanguageBits2::Farsi => 128,
            LanguageBits2::UnknownVariant(value) => value,
        }
    }
    pub fn as_i64(self) -> i64 {
        self.as_u8() as i64
    }
}
impl fmt::Display for LanguageBits2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            LanguageBits2::Slovenian => writeln!(f, "slovenian"),
            LanguageBits2::Swedish => writeln!(f, "swedish"),
            LanguageBits2::Russian => writeln!(f, "russian"),
            LanguageBits2::Turkish => writeln!(f, "turkish"),
            LanguageBits2::Latvian => writeln!(f, "latvian"),
            LanguageBits2::Ukrainian => writeln!(f, "ukrainian"),
            LanguageBits2::Arabic => writeln!(f, "arabic"),
            LanguageBits2::Farsi => writeln!(f, "farsi"),
            LanguageBits2::UnknownVariant(value) => writeln!(f, "unknown_variant_{}", *value),
        }
    }
}
impl convert::From<u8> for LanguageBits2 {
    fn from(value: u8) -> Self {
        match value {
            1 => LanguageBits2::Slovenian,
            2 => LanguageBits2::Swedish,
            4 => LanguageBits2::Russian,
            8 => LanguageBits2::Turkish,
            16 => LanguageBits2::Latvian,
            32 => LanguageBits2::Ukrainian,
            64 => LanguageBits2::Arabic,
            128 => LanguageBits2::Farsi,
            _ => LanguageBits2::UnknownVariant(value),
        }
    }
}
impl convert::From<i64> for LanguageBits2 {
    fn from(value: i64) -> Self {
        LanguageBits2::from(value as u8)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum LanguageBits3 {
    Bulgarian,
    Romanian,
    Chinese,
    Japanese,
    Korean,
    Taiwanese,
    Thai,
    Hebrew,
    UnknownVariant(u8),
}
impl LanguageBits3 {
    pub fn as_u8(self) -> u8 {
        match self {
            LanguageBits3::Bulgarian => 1,
            LanguageBits3::Romanian => 2,
            LanguageBits3::Chinese => 4,
            LanguageBits3::Japanese => 8,
            LanguageBits3::Korean => 16,
            LanguageBits3::Taiwanese => 32,
            LanguageBits3::Thai => 64,
            LanguageBits3::Hebrew => 128,
            LanguageBits3::UnknownVariant(value) => value,
        }
    }
    pub fn as_i64(self) -> i64 {
        self.as_u8() as i64
    }
}
impl fmt::Display for LanguageBits3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            LanguageBits3::Bulgarian => writeln!(f, "bulgarian"),
            LanguageBits3::Romanian => writeln!(f, "romanian"),
            LanguageBits3::Chinese => writeln!(f, "chinese"),
            LanguageBits3::Japanese => writeln!(f, "japanese"),
            LanguageBits3::Korean => writeln!(f, "korean"),
            LanguageBits3::Taiwanese => writeln!(f, "taiwanese"),
            LanguageBits3::Thai => writeln!(f, "thai"),
            LanguageBits3::Hebrew => writeln!(f, "hebrew"),
            LanguageBits3::UnknownVariant(value) => writeln!(f, "unknown_variant_{}", *value),
        }
    }
}
impl convert::From<u8> for LanguageBits3 {
    fn from(value: u8) -> Self {
        match value {
            1 => LanguageBits3::Bulgarian,
            2 => LanguageBits3::Romanian,
            4 => LanguageBits3::Chinese,
            8 => LanguageBits3::Japanese,
            16 => LanguageBits3::Korean,
            32 => LanguageBits3::Taiwanese,
            64 => LanguageBits3::Thai,
            128 => LanguageBits3::Hebrew,
            _ => LanguageBits3::UnknownVariant(value),
        }
    }
}
impl convert::From<i64> for LanguageBits3 {
    fn from(value: i64) -> Self {
        LanguageBits3::from(value as u8)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum LanguageBits4 {
    BrazilianPortuguese,
    Indonesian,
    Malaysian,
    Vietnamese,
    Burmese,
    Mongolian,
    UnknownVariant(u8),
}
impl LanguageBits4 {
    pub fn as_u8(self) -> u8 {
        match self {
            LanguageBits4::BrazilianPortuguese => 1,
            LanguageBits4::Indonesian => 2,
            LanguageBits4::Malaysian => 4,
            LanguageBits4::Vietnamese => 8,
            LanguageBits4::Burmese => 16,
            LanguageBits4::Mongolian => 32,
            LanguageBits4::UnknownVariant(value) => value,
        }
    }
    pub fn as_i64(self) -> i64 {
        self.as_u8() as i64
    }
}
impl fmt::Display for LanguageBits4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            LanguageBits4::BrazilianPortuguese => writeln!(f, "brazilian_portuguese"),
            LanguageBits4::Indonesian => writeln!(f, "indonesian"),
            LanguageBits4::Malaysian => writeln!(f, "malaysian"),
            LanguageBits4::Vietnamese => writeln!(f, "vietnamese"),
            LanguageBits4::Burmese => writeln!(f, "burmese"),
            LanguageBits4::Mongolian => writeln!(f, "mongolian"),
            LanguageBits4::UnknownVariant(value) => writeln!(f, "unknown_variant_{}", *value),
        }
    }
}
impl convert::From<u8> for LanguageBits4 {
    fn from(value: u8) -> Self {
        match value {
            1 => LanguageBits4::BrazilianPortuguese,
            2 => LanguageBits4::Indonesian,
            4 => LanguageBits4::Malaysian,
            8 => LanguageBits4::Vietnamese,
            16 => LanguageBits4::Burmese,
            32 => LanguageBits4::Mongolian,
            _ => LanguageBits4::UnknownVariant(value),
        }
    }
}
impl convert::From<i64> for LanguageBits4 {
    fn from(value: i64) -> Self {
        LanguageBits4::from(value as u8)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum TimeZone {
    Almaty,
    Bangkok,
    Bombay,
    Brasilia,
    Cairo,
    CapeVerdeIs,
    Darwin,
    Eniwetok,
    Fiji,
    HongKong,
    Islamabad,
    Kabul,
    Magadan,
    MidAtlantic,
    Moscow,
    Muscat,
    Newfoundland,
    Samoa,
    Sydney,
    Tehran,
    Tokyo,
    UsAlaska,
    UsAtlantic,
    UsCentral,
    UsEastern,
    UsHawaii,
    UsMountain,
    UsPacific,
    Other,
    Auckland,
    Kathmandu,
    EuropeWesternWet,
    EuropeCentralCet,
    EuropeEasternEet,
    Jakarta,
    Perth,
    Adelaide,
    Brisbane,
    Tasmania,
    Iceland,
    Amsterdam,
    Athens,
    Barcelona,
    Berlin,
    Brussels,
    Budapest,
    Copenhagen,
    Dublin,
    Helsinki,
    Lisbon,
    London,
    Madrid,
    Munich,
    Oslo,
    Paris,
    Prague,
    Reykjavik,
    Rome,
    Stockholm,
    Vienna,
    Warsaw,
    Zurich,
    Quebec,
    Ontario,
    Manitoba,
    Saskatchewan,
    Alberta,
    BritishColumbia,
    Boise,
    Boston,
    Chicago,
    Dallas,
    Denver,
    KansasCity,
    LasVegas,
    LosAngeles,
    Miami,
    Minneapolis,
    NewYork,
    NewOrleans,
    Phoenix,
    SantaFe,
    Seattle,
    WashingtonDc,
    UsArizona,
    Chita,
    Ekaterinburg,
    Irkutsk,
    Kaliningrad,
    Krasnoyarsk,
    Novosibirsk,
    PetropavlovskKamchatskiy,
    Samara,
    Vladivostok,
    MexicoCentral,
    MexicoMountain,
    MexicoPacific,
    CapeTown,
    Winkhoek,
    Lagos,
    Riyahd,
    Venezuela,
    AustraliaLh,
    Santiago,
    Manual,
    Automatic,
    UnknownVariant(u8),
}
impl TimeZone {
    pub fn as_u8(self) -> u8 {
        match self {
            TimeZone::Almaty => 0,
            TimeZone::Bangkok => 1,
            TimeZone::Bombay => 2,
            TimeZone::Brasilia => 3,
            TimeZone::Cairo => 4,
            TimeZone::CapeVerdeIs => 5,
            TimeZone::Darwin => 6,
            TimeZone::Eniwetok => 7,
            TimeZone::Fiji => 8,
            TimeZone::HongKong => 9,
            TimeZone::Islamabad => 10,
            TimeZone::Kabul => 11,
            TimeZone::Magadan => 12,
            TimeZone::MidAtlantic => 13,
            TimeZone::Moscow => 14,
            TimeZone::Muscat => 15,
            TimeZone::Newfoundland => 16,
            TimeZone::Samoa => 17,
            TimeZone::Sydney => 18,
            TimeZone::Tehran => 19,
            TimeZone::Tokyo => 20,
            TimeZone::UsAlaska => 21,
            TimeZone::UsAtlantic => 22,
            TimeZone::UsCentral => 23,
            TimeZone::UsEastern => 24,
            TimeZone::UsHawaii => 25,
            TimeZone::UsMountain => 26,
            TimeZone::UsPacific => 27,
            TimeZone::Other => 28,
            TimeZone::Auckland => 29,
            TimeZone::Kathmandu => 30,
            TimeZone::EuropeWesternWet => 31,
            TimeZone::EuropeCentralCet => 32,
            TimeZone::EuropeEasternEet => 33,
            TimeZone::Jakarta => 34,
            TimeZone::Perth => 35,
            TimeZone::Adelaide => 36,
            TimeZone::Brisbane => 37,
            TimeZone::Tasmania => 38,
            TimeZone::Iceland => 39,
            TimeZone::Amsterdam => 40,
            TimeZone::Athens => 41,
            TimeZone::Barcelona => 42,
            TimeZone::Berlin => 43,
            TimeZone::Brussels => 44,
            TimeZone::Budapest => 45,
            TimeZone::Copenhagen => 46,
            TimeZone::Dublin => 47,
            TimeZone::Helsinki => 48,
            TimeZone::Lisbon => 49,
            TimeZone::London => 50,
            TimeZone::Madrid => 51,
            TimeZone::Munich => 52,
            TimeZone::Oslo => 53,
            TimeZone::Paris => 54,
            TimeZone::Prague => 55,
            TimeZone::Reykjavik => 56,
            TimeZone::Rome => 57,
            TimeZone::Stockholm => 58,
            TimeZone::Vienna => 59,
            TimeZone::Warsaw => 60,
            TimeZone::Zurich => 61,
            TimeZone::Quebec => 62,
            TimeZone::Ontario => 63,
            TimeZone::Manitoba => 64,
            TimeZone::Saskatchewan => 65,
            TimeZone::Alberta => 66,
            TimeZone::BritishColumbia => 67,
            TimeZone::Boise => 68,
            TimeZone::Boston => 69,
            TimeZone::Chicago => 70,
            TimeZone::Dallas => 71,
            TimeZone::Denver => 72,
            TimeZone::KansasCity => 73,
            TimeZone::LasVegas => 74,
            TimeZone::LosAngeles => 75,
            TimeZone::Miami => 76,
            TimeZone::Minneapolis => 77,
            TimeZone::NewYork => 78,
            TimeZone::NewOrleans => 79,
            TimeZone::Phoenix => 80,
            TimeZone::SantaFe => 81,
            TimeZone::Seattle => 82,
            TimeZone::WashingtonDc => 83,
            TimeZone::UsArizona => 84,
            TimeZone::Chita => 85,
            TimeZone::Ekaterinburg => 86,
            TimeZone::Irkutsk => 87,
            TimeZone::Kaliningrad => 88,
            TimeZone::Krasnoyarsk => 89,
            TimeZone::Novosibirsk => 90,
            TimeZone::PetropavlovskKamchatskiy => 91,
            TimeZone::Samara => 92,
            TimeZone::Vladivostok => 93,
            TimeZone::MexicoCentral => 94,
            TimeZone::MexicoMountain => 95,
            TimeZone::MexicoPacific => 96,
            TimeZone::CapeTown => 97,
            TimeZone::Winkhoek => 98,
            TimeZone::Lagos => 99,
            TimeZone::Riyahd => 100,
            TimeZone::Venezuela => 101,
            TimeZone::AustraliaLh => 102,
            TimeZone::Santiago => 103,
            TimeZone::Manual => 253,
            TimeZone::Automatic => 254,
            TimeZone::UnknownVariant(value) => value,
        }
    }
    pub fn as_i64(self) -> i64 {
        self.as_u8() as i64
    }
}
impl fmt::Display for TimeZone {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            TimeZone::Almaty => writeln!(f, "almaty"),
            TimeZone::Bangkok => writeln!(f, "bangkok"),
            TimeZone::Bombay => writeln!(f, "bombay"),
            TimeZone::Brasilia => writeln!(f, "brasilia"),
            TimeZone::Cairo => writeln!(f, "cairo"),
            TimeZone::CapeVerdeIs => writeln!(f, "cape_verde_is"),
            TimeZone::Darwin => writeln!(f, "darwin"),
            TimeZone::Eniwetok => writeln!(f, "eniwetok"),
            TimeZone::Fiji => writeln!(f, "fiji"),
            TimeZone::HongKong => writeln!(f, "hong_kong"),
            TimeZone::Islamabad => writeln!(f, "islamabad"),
            TimeZone::Kabul => writeln!(f, "kabul"),
            TimeZone::Magadan => writeln!(f, "magadan"),
            TimeZone::MidAtlantic => writeln!(f, "mid_atlantic"),
            TimeZone::Moscow => writeln!(f, "moscow"),
            TimeZone::Muscat => writeln!(f, "muscat"),
            TimeZone::Newfoundland => writeln!(f, "newfoundland"),
            TimeZone::Samoa => writeln!(f, "samoa"),
            TimeZone::Sydney => writeln!(f, "sydney"),
            TimeZone::Tehran => writeln!(f, "tehran"),
            TimeZone::Tokyo => writeln!(f, "tokyo"),
            TimeZone::UsAlaska => writeln!(f, "us_alaska"),
            TimeZone::UsAtlantic => writeln!(f, "us_atlantic"),
            TimeZone::UsCentral => writeln!(f, "us_central"),
            TimeZone::UsEastern => writeln!(f, "us_eastern"),
            TimeZone::UsHawaii => writeln!(f, "us_hawaii"),
            TimeZone::UsMountain => writeln!(f, "us_mountain"),
            TimeZone::UsPacific => writeln!(f, "us_pacific"),
            TimeZone::Other => writeln!(f, "other"),
            TimeZone::Auckland => writeln!(f, "auckland"),
            TimeZone::Kathmandu => writeln!(f, "kathmandu"),
            TimeZone::EuropeWesternWet => writeln!(f, "europe_western_wet"),
            TimeZone::EuropeCentralCet => writeln!(f, "europe_central_cet"),
            TimeZone::EuropeEasternEet => writeln!(f, "europe_eastern_eet"),
            TimeZone::Jakarta => writeln!(f, "jakarta"),
            TimeZone::Perth => writeln!(f, "perth"),
            TimeZone::Adelaide => writeln!(f, "adelaide"),
            TimeZone::Brisbane => writeln!(f, "brisbane"),
            TimeZone::Tasmania => writeln!(f, "tasmania"),
            TimeZone::Iceland => writeln!(f, "iceland"),
            TimeZone::Amsterdam => writeln!(f, "amsterdam"),
            TimeZone::Athens => writeln!(f, "athens"),
            TimeZone::Barcelona => writeln!(f, "barcelona"),
            TimeZone::Berlin => writeln!(f, "berlin"),
            TimeZone::Brussels => writeln!(f, "brussels"),
            TimeZone::Budapest => writeln!(f, "budapest"),
            TimeZone::Copenhagen => writeln!(f, "copenhagen"),
            TimeZone::Dublin => writeln!(f, "dublin"),
            TimeZone::Helsinki => writeln!(f, "helsinki"),
            TimeZone::Lisbon => writeln!(f, "lisbon"),
            TimeZone::London => writeln!(f, "london"),
            TimeZone::Madrid => writeln!(f, "madrid"),
            TimeZone::Munich => writeln!(f, "munich"),
            TimeZone::Oslo => writeln!(f, "oslo"),
            TimeZone::Paris => writeln!(f, "paris"),
            TimeZone::Prague => writeln!(f, "prague"),
            TimeZone::Reykjavik => writeln!(f, "reykjavik"),
            TimeZone::Rome => writeln!(f, "rome"),
            TimeZone::Stockholm => writeln!(f, "stockholm"),
            TimeZone::Vienna => writeln!(f, "vienna"),
            TimeZone::Warsaw => writeln!(f, "warsaw"),
            TimeZone::Zurich => writeln!(f, "zurich"),
            TimeZone::Quebec => writeln!(f, "quebec"),
            TimeZone::Ontario => writeln!(f, "ontario"),
            TimeZone::Manitoba => writeln!(f, "manitoba"),
            TimeZone::Saskatchewan => writeln!(f, "saskatchewan"),
            TimeZone::Alberta => writeln!(f, "alberta"),
            TimeZone::BritishColumbia => writeln!(f, "british_columbia"),
            TimeZone::Boise => writeln!(f, "boise"),
            TimeZone::Boston => writeln!(f, "boston"),
            TimeZone::Chicago => writeln!(f, "chicago"),
            TimeZone::Dallas => writeln!(f, "dallas"),
            TimeZone::Denver => writeln!(f, "denver"),
            TimeZone::KansasCity => writeln!(f, "kansas_city"),
            TimeZone::LasVegas => writeln!(f, "las_vegas"),
            TimeZone::LosAngeles => writeln!(f, "los_angeles"),
            TimeZone::Miami => writeln!(f, "miami"),
            TimeZone::Minneapolis => writeln!(f, "minneapolis"),
            TimeZone::NewYork => writeln!(f, "new_york"),
            TimeZone::NewOrleans => writeln!(f, "new_orleans"),
            TimeZone::Phoenix => writeln!(f, "phoenix"),
            TimeZone::SantaFe => writeln!(f, "santa_fe"),
            TimeZone::Seattle => writeln!(f, "seattle"),
            TimeZone::WashingtonDc => writeln!(f, "washington_dc"),
            TimeZone::UsArizona => writeln!(f, "us_arizona"),
            TimeZone::Chita => writeln!(f, "chita"),
            TimeZone::Ekaterinburg => writeln!(f, "ekaterinburg"),
            TimeZone::Irkutsk => writeln!(f, "irkutsk"),
            TimeZone::Kaliningrad => writeln!(f, "kaliningrad"),
            TimeZone::Krasnoyarsk => writeln!(f, "krasnoyarsk"),
            TimeZone::Novosibirsk => writeln!(f, "novosibirsk"),
            TimeZone::PetropavlovskKamchatskiy => writeln!(f, "petropavlovsk_kamchatskiy"),
            TimeZone::Samara => writeln!(f, "samara"),
            TimeZone::Vladivostok => writeln!(f, "vladivostok"),
            TimeZone::MexicoCentral => writeln!(f, "mexico_central"),
            TimeZone::MexicoMountain => writeln!(f, "mexico_mountain"),
            TimeZone::MexicoPacific => writeln!(f, "mexico_pacific"),
            TimeZone::CapeTown => writeln!(f, "cape_town"),
            TimeZone::Winkhoek => writeln!(f, "winkhoek"),
            TimeZone::Lagos => writeln!(f, "lagos"),
            TimeZone::Riyahd => writeln!(f, "riyahd"),
            TimeZone::Venezuela => writeln!(f, "venezuela"),
            TimeZone::AustraliaLh => writeln!(f, "australia_lh"),
            TimeZone::Santiago => writeln!(f, "santiago"),
            TimeZone::Manual => writeln!(f, "manual"),
            TimeZone::Automatic => writeln!(f, "automatic"),
            TimeZone::UnknownVariant(value) => writeln!(f, "unknown_variant_{}", *value),
        }
    }
}
impl convert::From<u8> for TimeZone {
    fn from(value: u8) -> Self {
        match value {
            0 => TimeZone::Almaty,
            1 => TimeZone::Bangkok,
            2 => TimeZone::Bombay,
            3 => TimeZone::Brasilia,
            4 => TimeZone::Cairo,
            5 => TimeZone::CapeVerdeIs,
            6 => TimeZone::Darwin,
            7 => TimeZone::Eniwetok,
            8 => TimeZone::Fiji,
            9 => TimeZone::HongKong,
            10 => TimeZone::Islamabad,
            11 => TimeZone::Kabul,
            12 => TimeZone::Magadan,
            13 => TimeZone::MidAtlantic,
            14 => TimeZone::Moscow,
            15 => TimeZone::Muscat,
            16 => TimeZone::Newfoundland,
            17 => TimeZone::Samoa,
            18 => TimeZone::Sydney,
            19 => TimeZone::Tehran,
            20 => TimeZone::Tokyo,
            21 => TimeZone::UsAlaska,
            22 => TimeZone::UsAtlantic,
            23 => TimeZone::UsCentral,
            24 => TimeZone::UsEastern,
            25 => TimeZone::UsHawaii,
            26 => TimeZone::UsMountain,
            27 => TimeZone::UsPacific,
            28 => TimeZone::Other,
            29 => TimeZone::Auckland,
            30 => TimeZone::Kathmandu,
            31 => TimeZone::EuropeWesternWet,
            32 => TimeZone::EuropeCentralCet,
            33 => TimeZone::EuropeEasternEet,
            34 => TimeZone::Jakarta,
            35 => TimeZone::Perth,
            36 => TimeZone::Adelaide,
            37 => TimeZone::Brisbane,
            38 => TimeZone::Tasmania,
            39 => TimeZone::Iceland,
            40 => TimeZone::Amsterdam,
            41 => TimeZone::Athens,
            42 => TimeZone::Barcelona,
            43 => TimeZone::Berlin,
            44 => TimeZone::Brussels,
            45 => TimeZone::Budapest,
            46 => TimeZone::Copenhagen,
            47 => TimeZone::Dublin,
            48 => TimeZone::Helsinki,
            49 => TimeZone::Lisbon,
            50 => TimeZone::London,
            51 => TimeZone::Madrid,
            52 => TimeZone::Munich,
            53 => TimeZone::Oslo,
            54 => TimeZone::Paris,
            55 => TimeZone::Prague,
            56 => TimeZone::Reykjavik,
            57 => TimeZone::Rome,
            58 => TimeZone::Stockholm,
            59 => TimeZone::Vienna,
            60 => TimeZone::Warsaw,
            61 => TimeZone::Zurich,
            62 => TimeZone::Quebec,
            63 => TimeZone::Ontario,
            64 => TimeZone::Manitoba,
            65 => TimeZone::Saskatchewan,
            66 => TimeZone::Alberta,
            67 => TimeZone::BritishColumbia,
            68 => TimeZone::Boise,
            69 => TimeZone::Boston,
            70 => TimeZone::Chicago,
            71 => TimeZone::Dallas,
            72 => TimeZone::Denver,
            73 => TimeZone::KansasCity,
            74 => TimeZone::LasVegas,
            75 => TimeZone::LosAngeles,
            76 => TimeZone::Miami,
            77 => TimeZone::Minneapolis,
            78 => TimeZone::NewYork,
            79 => TimeZone::NewOrleans,
            80 => TimeZone::Phoenix,
            81 => TimeZone::SantaFe,
            82 => TimeZone::Seattle,
            83 => TimeZone::WashingtonDc,
            84 => TimeZone::UsArizona,
            85 => TimeZone::Chita,
            86 => TimeZone::Ekaterinburg,
            87 => TimeZone::Irkutsk,
            88 => TimeZone::Kaliningrad,
            89 => TimeZone::Krasnoyarsk,
            90 => TimeZone::Novosibirsk,
            91 => TimeZone::PetropavlovskKamchatskiy,
            92 => TimeZone::Samara,
            93 => TimeZone::Vladivostok,
            94 => TimeZone::MexicoCentral,
            95 => TimeZone::MexicoMountain,
            96 => TimeZone::MexicoPacific,
            97 => TimeZone::CapeTown,
            98 => TimeZone::Winkhoek,
            99 => TimeZone::Lagos,
            100 => TimeZone::Riyahd,
            101 => TimeZone::Venezuela,
            102 => TimeZone::AustraliaLh,
            103 => TimeZone::Santiago,
            253 => TimeZone::Manual,
            254 => TimeZone::Automatic,
            _ => TimeZone::UnknownVariant(value),
        }
    }
}
impl convert::From<i64> for TimeZone {
    fn from(value: i64) -> Self {
        TimeZone::from(value as u8)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum DisplayMeasure {
    Metric,
    Statute,
    Nautical,
    UnknownVariant(u8),
}
impl DisplayMeasure {
    pub fn as_u8(self) -> u8 {
        match self {
            DisplayMeasure::Metric => 0,
            DisplayMeasure::Statute => 1,
            DisplayMeasure::Nautical => 2,
            DisplayMeasure::UnknownVariant(value) => value,
        }
    }
    pub fn as_i64(self) -> i64 {
        self.as_u8() as i64
    }
}
impl fmt::Display for DisplayMeasure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            DisplayMeasure::Metric => writeln!(f, "metric"),
            DisplayMeasure::Statute => writeln!(f, "statute"),
            DisplayMeasure::Nautical => writeln!(f, "nautical"),
            DisplayMeasure::UnknownVariant(value) => writeln!(f, "unknown_variant_{}", *value),
        }
    }
}
impl convert::From<u8> for DisplayMeasure {
    fn from(value: u8) -> Self {
        match value {
            0 => DisplayMeasure::Metric,
            1 => DisplayMeasure::Statute,
            2 => DisplayMeasure::Nautical,
            _ => DisplayMeasure::UnknownVariant(value),
        }
    }
}
impl convert::From<i64> for DisplayMeasure {
    fn from(value: i64) -> Self {
        DisplayMeasure::from(value as u8)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum DisplayHeart {
    Bpm,
    Max,
    Reserve,
    UnknownVariant(u8),
}
impl DisplayHeart {
    pub fn as_u8(self) -> u8 {
        match self {
            DisplayHeart::Bpm => 0,
            DisplayHeart::Max => 1,
            DisplayHeart::Reserve => 2,
            DisplayHeart::UnknownVariant(value) => value,
        }
    }
    pub fn as_i64(self) -> i64 {
        self.as_u8() as i64
    }
}
impl fmt::Display for DisplayHeart {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            DisplayHeart::Bpm => writeln!(f, "bpm"),
            DisplayHeart::Max => writeln!(f, "max"),
            DisplayHeart::Reserve => writeln!(f, "reserve"),
            DisplayHeart::UnknownVariant(value) => writeln!(f, "unknown_variant_{}", *value),
        }
    }
}
impl convert::From<u8> for DisplayHeart {
    fn from(value: u8) -> Self {
        match value {
            0 => DisplayHeart::Bpm,
            1 => DisplayHeart::Max,
            2 => DisplayHeart::Reserve,
            _ => DisplayHeart::UnknownVariant(value),
        }
    }
}
impl convert::From<i64> for DisplayHeart {
    fn from(value: i64) -> Self {
        DisplayHeart::from(value as u8)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum DisplayPower {
    Watts,
    PercentFtp,
    UnknownVariant(u8),
}
impl DisplayPower {
    pub fn as_u8(self) -> u8 {
        match self {
            DisplayPower::Watts => 0,
            DisplayPower::PercentFtp => 1,
            DisplayPower::UnknownVariant(value) => value,
        }
    }
    pub fn as_i64(self) -> i64 {
        self.as_u8() as i64
    }
}
impl fmt::Display for DisplayPower {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            DisplayPower::Watts => writeln!(f, "watts"),
            DisplayPower::PercentFtp => writeln!(f, "percent_ftp"),
            DisplayPower::UnknownVariant(value) => writeln!(f, "unknown_variant_{}", *value),
        }
    }
}
impl convert::From<u8> for DisplayPower {
    fn from(value: u8) -> Self {
        match value {
            0 => DisplayPower::Watts,
            1 => DisplayPower::PercentFtp,
            _ => DisplayPower::UnknownVariant(value),
        }
    }
}
impl convert::From<i64> for DisplayPower {
    fn from(value: i64) -> Self {
        DisplayPower::from(value as u8)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum DisplayPosition {
    /// dd.dddddd
    Degree,
    /// dddmm.mmm
    DegreeMinute,
    /// dddmmss
    DegreeMinuteSecond,
    /// Austrian Grid (BMN)
    AustrianGrid,
    /// British National Grid
    BritishGrid,
    /// Dutch grid system
    DutchGrid,
    /// Hungarian grid system
    HungarianGrid,
    /// Finnish grid system Zone3 KKJ27
    FinnishGrid,
    /// Gausss Krueger (German)
    GermanGrid,
    /// Icelandic Grid
    IcelandicGrid,
    /// Indonesian Equatorial LCO
    IndonesianEquatorial,
    /// Indonesian Irian LCO
    IndonesianIrian,
    /// Indonesian Southern LCO
    IndonesianSouthern,
    /// India zone 0
    IndiaZone0,
    /// India zone IA
    IndiaZoneIA,
    /// India zone IB
    IndiaZoneIB,
    /// India zone IIA
    IndiaZoneIIA,
    /// India zone IIB
    IndiaZoneIIB,
    /// India zone IIIA
    IndiaZoneIIIA,
    /// India zone IIIB
    IndiaZoneIIIB,
    /// India zone IVA
    IndiaZoneIVA,
    /// India zone IVB
    IndiaZoneIVB,
    /// Irish Transverse Mercator
    IrishTransverse,
    /// Irish Grid
    IrishGrid,
    /// Loran TD
    Loran,
    /// Maidenhead grid system
    MaidenheadGrid,
    /// MGRS grid system
    MgrsGrid,
    /// New Zealand grid system
    NewZealandGrid,
    /// New Zealand Transverse Mercator
    NewZealandTransverse,
    /// Qatar National Grid
    QatarGrid,
    /// Modified RT-90 (Sweden)
    ModifiedSwedishGrid,
    /// RT-90 (Sweden)
    SwedishGrid,
    /// South African Grid
    SouthAfricanGrid,
    /// Swiss CH-1903 grid
    SwissGrid,
    /// Taiwan Grid
    TaiwanGrid,
    /// United States National Grid
    UnitedStatesGrid,
    /// UTM/UPS grid system
    UtmUpsGrid,
    /// West Malayan RSO
    WestMalayan,
    /// Borneo RSO
    BorneoRso,
    /// Estonian grid system
    EstonianGrid,
    /// Latvian Transverse Mercator
    LatvianGrid,
    /// Reference Grid 99 TM (Swedish)
    SwedishRef99Grid,
    UnknownVariant(u8),
}
impl DisplayPosition {
    pub fn as_u8(self) -> u8 {
        match self {
            DisplayPosition::Degree => 0,
            DisplayPosition::DegreeMinute => 1,
            DisplayPosition::DegreeMinuteSecond => 2,
            DisplayPosition::AustrianGrid => 3,
            DisplayPosition::BritishGrid => 4,
            DisplayPosition::DutchGrid => 5,
            DisplayPosition::HungarianGrid => 6,
            DisplayPosition::FinnishGrid => 7,
            DisplayPosition::GermanGrid => 8,
            DisplayPosition::IcelandicGrid => 9,
            DisplayPosition::IndonesianEquatorial => 10,
            DisplayPosition::IndonesianIrian => 11,
            DisplayPosition::IndonesianSouthern => 12,
            DisplayPosition::IndiaZone0 => 13,
            DisplayPosition::IndiaZoneIA => 14,
            DisplayPosition::IndiaZoneIB => 15,
            DisplayPosition::IndiaZoneIIA => 16,
            DisplayPosition::IndiaZoneIIB => 17,
            DisplayPosition::IndiaZoneIIIA => 18,
            DisplayPosition::IndiaZoneIIIB => 19,
            DisplayPosition::IndiaZoneIVA => 20,
            DisplayPosition::IndiaZoneIVB => 21,
            DisplayPosition::IrishTransverse => 22,
            DisplayPosition::IrishGrid => 23,
            DisplayPosition::Loran => 24,
            DisplayPosition::MaidenheadGrid => 25,
            DisplayPosition::MgrsGrid => 26,
            DisplayPosition::NewZealandGrid => 27,
            DisplayPosition::NewZealandTransverse => 28,
            DisplayPosition::QatarGrid => 29,
            DisplayPosition::ModifiedSwedishGrid => 30,
            DisplayPosition::SwedishGrid => 31,
            DisplayPosition::SouthAfricanGrid => 32,
            DisplayPosition::SwissGrid => 33,
            DisplayPosition::TaiwanGrid => 34,
            DisplayPosition::UnitedStatesGrid => 35,
            DisplayPosition::UtmUpsGrid => 36,
            DisplayPosition::WestMalayan => 37,
            DisplayPosition::BorneoRso => 38,
            DisplayPosition::EstonianGrid => 39,
            DisplayPosition::LatvianGrid => 40,
            DisplayPosition::SwedishRef99Grid => 41,
            DisplayPosition::UnknownVariant(value) => value,
        }
    }
    pub fn as_i64(self) -> i64 {
        self.as_u8() as i64
    }
}
impl fmt::Display for DisplayPosition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            DisplayPosition::Degree => writeln!(f, "degree"),
            DisplayPosition::DegreeMinute => writeln!(f, "degree_minute"),
            DisplayPosition::DegreeMinuteSecond => writeln!(f, "degree_minute_second"),
            DisplayPosition::AustrianGrid => writeln!(f, "austrian_grid"),
            DisplayPosition::BritishGrid => writeln!(f, "british_grid"),
            DisplayPosition::DutchGrid => writeln!(f, "dutch_grid"),
            DisplayPosition::HungarianGrid => writeln!(f, "hungarian_grid"),
            DisplayPosition::FinnishGrid => writeln!(f, "finnish_grid"),
            DisplayPosition::GermanGrid => writeln!(f, "german_grid"),
            DisplayPosition::IcelandicGrid => writeln!(f, "icelandic_grid"),
            DisplayPosition::IndonesianEquatorial => writeln!(f, "indonesian_equatorial"),
            DisplayPosition::IndonesianIrian => writeln!(f, "indonesian_irian"),
            DisplayPosition::IndonesianSouthern => writeln!(f, "indonesian_southern"),
            DisplayPosition::IndiaZone0 => writeln!(f, "india_zone_0"),
            DisplayPosition::IndiaZoneIA => writeln!(f, "india_zone_IA"),
            DisplayPosition::IndiaZoneIB => writeln!(f, "india_zone_IB"),
            DisplayPosition::IndiaZoneIIA => writeln!(f, "india_zone_IIA"),
            DisplayPosition::IndiaZoneIIB => writeln!(f, "india_zone_IIB"),
            DisplayPosition::IndiaZoneIIIA => writeln!(f, "india_zone_IIIA"),
            DisplayPosition::IndiaZoneIIIB => writeln!(f, "india_zone_IIIB"),
            DisplayPosition::IndiaZoneIVA => writeln!(f, "india_zone_IVA"),
            DisplayPosition::IndiaZoneIVB => writeln!(f, "india_zone_IVB"),
            DisplayPosition::IrishTransverse => writeln!(f, "irish_transverse"),
            DisplayPosition::IrishGrid => writeln!(f, "irish_grid"),
            DisplayPosition::Loran => writeln!(f, "loran"),
            DisplayPosition::MaidenheadGrid => writeln!(f, "maidenhead_grid"),
            DisplayPosition::MgrsGrid => writeln!(f, "mgrs_grid"),
            DisplayPosition::NewZealandGrid => writeln!(f, "new_zealand_grid"),
            DisplayPosition::NewZealandTransverse => writeln!(f, "new_zealand_transverse"),
            DisplayPosition::QatarGrid => writeln!(f, "qatar_grid"),
            DisplayPosition::ModifiedSwedishGrid => writeln!(f, "modified_swedish_grid"),
            DisplayPosition::SwedishGrid => writeln!(f, "swedish_grid"),
            DisplayPosition::SouthAfricanGrid => writeln!(f, "south_african_grid"),
            DisplayPosition::SwissGrid => writeln!(f, "swiss_grid"),
            DisplayPosition::TaiwanGrid => writeln!(f, "taiwan_grid"),
            DisplayPosition::UnitedStatesGrid => writeln!(f, "united_states_grid"),
            DisplayPosition::UtmUpsGrid => writeln!(f, "utm_ups_grid"),
            DisplayPosition::WestMalayan => writeln!(f, "west_malayan"),
            DisplayPosition::BorneoRso => writeln!(f, "borneo_rso"),
            DisplayPosition::EstonianGrid => writeln!(f, "estonian_grid"),
            DisplayPosition::LatvianGrid => writeln!(f, "latvian_grid"),
            DisplayPosition::SwedishRef99Grid => writeln!(f, "swedish_ref_99_grid"),
            DisplayPosition::UnknownVariant(value) => writeln!(f, "unknown_variant_{}", *value),
        }
    }
}
impl convert::From<u8> for DisplayPosition {
    fn from(value: u8) -> Self {
        match value {
            0 => DisplayPosition::Degree,
            1 => DisplayPosition::DegreeMinute,
            2 => DisplayPosition::DegreeMinuteSecond,
            3 => DisplayPosition::AustrianGrid,
            4 => DisplayPosition::BritishGrid,
            5 => DisplayPosition::DutchGrid,
            6 => DisplayPosition::HungarianGrid,
            7 => DisplayPosition::FinnishGrid,
            8 => DisplayPosition::GermanGrid,
            9 => DisplayPosition::IcelandicGrid,
            10 => DisplayPosition::IndonesianEquatorial,
            11 => DisplayPosition::IndonesianIrian,
            12 => DisplayPosition::IndonesianSouthern,
            13 => DisplayPosition::IndiaZone0,
            14 => DisplayPosition::IndiaZoneIA,
            15 => DisplayPosition::IndiaZoneIB,
            16 => DisplayPosition::IndiaZoneIIA,
            17 => DisplayPosition::IndiaZoneIIB,
            18 => DisplayPosition::IndiaZoneIIIA,
            19 => DisplayPosition::IndiaZoneIIIB,
            20 => DisplayPosition::IndiaZoneIVA,
            21 => DisplayPosition::IndiaZoneIVB,
            22 => DisplayPosition::IrishTransverse,
            23 => DisplayPosition::IrishGrid,
            24 => DisplayPosition::Loran,
            25 => DisplayPosition::MaidenheadGrid,
            26 => DisplayPosition::MgrsGrid,
            27 => DisplayPosition::NewZealandGrid,
            28 => DisplayPosition::NewZealandTransverse,
            29 => DisplayPosition::QatarGrid,
            30 => DisplayPosition::ModifiedSwedishGrid,
            31 => DisplayPosition::SwedishGrid,
            32 => DisplayPosition::SouthAfricanGrid,
            33 => DisplayPosition::SwissGrid,
            34 => DisplayPosition::TaiwanGrid,
            35 => DisplayPosition::UnitedStatesGrid,
            36 => DisplayPosition::UtmUpsGrid,
            37 => DisplayPosition::WestMalayan,
            38 => DisplayPosition::BorneoRso,
            39 => DisplayPosition::EstonianGrid,
            40 => DisplayPosition::LatvianGrid,
            41 => DisplayPosition::SwedishRef99Grid,
            _ => DisplayPosition::UnknownVariant(value),
        }
    }
}
impl convert::From<i64> for DisplayPosition {
    fn from(value: i64) -> Self {
        DisplayPosition::from(value as u8)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum Switch {
    Off,
    On,
    Auto,
    UnknownVariant(u8),
}
impl Switch {
    pub fn as_u8(self) -> u8 {
        match self {
            Switch::Off => 0,
            Switch::On => 1,
            Switch::Auto => 2,
            Switch::UnknownVariant(value) => value,
        }
    }
    pub fn as_i64(self) -> i64 {
        self.as_u8() as i64
    }
}
impl fmt::Display for Switch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Switch::Off => writeln!(f, "off"),
            Switch::On => writeln!(f, "on"),
            Switch::Auto => writeln!(f, "auto"),
            Switch::UnknownVariant(value) => writeln!(f, "unknown_variant_{}", *value),
        }
    }
}
impl convert::From<u8> for Switch {
    fn from(value: u8) -> Self {
        match value {
            0 => Switch::Off,
            1 => Switch::On,
            2 => Switch::Auto,
            _ => Switch::UnknownVariant(value),
        }
    }
}
impl convert::From<i64> for Switch {
    fn from(value: i64) -> Self {
        Switch::from(value as u8)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum Sport {
    Generic,
    Running,
    Cycling,
    /// Mulitsport transition
    Transition,
    FitnessEquipment,
    Swimming,
    Basketball,
    Soccer,
    Tennis,
    AmericanFootball,
    Training,
    Walking,
    CrossCountrySkiing,
    AlpineSkiing,
    Snowboarding,
    Rowing,
    Mountaineering,
    Hiking,
    Multisport,
    Paddling,
    Flying,
    EBiking,
    Motorcycling,
    Boating,
    Driving,
    Golf,
    HangGliding,
    HorsebackRiding,
    Hunting,
    Fishing,
    InlineSkating,
    RockClimbing,
    Sailing,
    IceSkating,
    SkyDiving,
    Snowshoeing,
    Snowmobiling,
    StandUpPaddleboarding,
    Surfing,
    Wakeboarding,
    WaterSkiing,
    Kayaking,
    Rafting,
    Windsurfing,
    Kitesurfing,
    Tactical,
    Jumpmaster,
    Boxing,
    FloorClimbing,
    /// All is for goals only to include all sports.
    All,
    UnknownVariant(u8),
}
impl Sport {
    pub fn as_u8(self) -> u8 {
        match self {
            Sport::Generic => 0,
            Sport::Running => 1,
            Sport::Cycling => 2,
            Sport::Transition => 3,
            Sport::FitnessEquipment => 4,
            Sport::Swimming => 5,
            Sport::Basketball => 6,
            Sport::Soccer => 7,
            Sport::Tennis => 8,
            Sport::AmericanFootball => 9,
            Sport::Training => 10,
            Sport::Walking => 11,
            Sport::CrossCountrySkiing => 12,
            Sport::AlpineSkiing => 13,
            Sport::Snowboarding => 14,
            Sport::Rowing => 15,
            Sport::Mountaineering => 16,
            Sport::Hiking => 17,
            Sport::Multisport => 18,
            Sport::Paddling => 19,
            Sport::Flying => 20,
            Sport::EBiking => 21,
            Sport::Motorcycling => 22,
            Sport::Boating => 23,
            Sport::Driving => 24,
            Sport::Golf => 25,
            Sport::HangGliding => 26,
            Sport::HorsebackRiding => 27,
            Sport::Hunting => 28,
            Sport::Fishing => 29,
            Sport::InlineSkating => 30,
            Sport::RockClimbing => 31,
            Sport::Sailing => 32,
            Sport::IceSkating => 33,
            Sport::SkyDiving => 34,
            Sport::Snowshoeing => 35,
            Sport::Snowmobiling => 36,
            Sport::StandUpPaddleboarding => 37,
            Sport::Surfing => 38,
            Sport::Wakeboarding => 39,
            Sport::WaterSkiing => 40,
            Sport::Kayaking => 41,
            Sport::Rafting => 42,
            Sport::Windsurfing => 43,
            Sport::Kitesurfing => 44,
            Sport::Tactical => 45,
            Sport::Jumpmaster => 46,
            Sport::Boxing => 47,
            Sport::FloorClimbing => 48,
            Sport::All => 254,
            Sport::UnknownVariant(value) => value,
        }
    }
    pub fn as_i64(self) -> i64 {
        self.as_u8() as i64
    }
}
impl fmt::Display for Sport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Sport::Generic => writeln!(f, "generic"),
            Sport::Running => writeln!(f, "running"),
            Sport::Cycling => writeln!(f, "cycling"),
            Sport::Transition => writeln!(f, "transition"),
            Sport::FitnessEquipment => writeln!(f, "fitness_equipment"),
            Sport::Swimming => writeln!(f, "swimming"),
            Sport::Basketball => writeln!(f, "basketball"),
            Sport::Soccer => writeln!(f, "soccer"),
            Sport::Tennis => writeln!(f, "tennis"),
            Sport::AmericanFootball => writeln!(f, "american_football"),
            Sport::Training => writeln!(f, "training"),
            Sport::Walking => writeln!(f, "walking"),
            Sport::CrossCountrySkiing => writeln!(f, "cross_country_skiing"),
            Sport::AlpineSkiing => writeln!(f, "alpine_skiing"),
            Sport::Snowboarding => writeln!(f, "snowboarding"),
            Sport::Rowing => writeln!(f, "rowing"),
            Sport::Mountaineering => writeln!(f, "mountaineering"),
            Sport::Hiking => writeln!(f, "hiking"),
            Sport::Multisport => writeln!(f, "multisport"),
            Sport::Paddling => writeln!(f, "paddling"),
            Sport::Flying => writeln!(f, "flying"),
            Sport::EBiking => writeln!(f, "e_biking"),
            Sport::Motorcycling => writeln!(f, "motorcycling"),
            Sport::Boating => writeln!(f, "boating"),
            Sport::Driving => writeln!(f, "driving"),
            Sport::Golf => writeln!(f, "golf"),
            Sport::HangGliding => writeln!(f, "hang_gliding"),
            Sport::HorsebackRiding => writeln!(f, "horseback_riding"),
            Sport::Hunting => writeln!(f, "hunting"),
            Sport::Fishing => writeln!(f, "fishing"),
            Sport::InlineSkating => writeln!(f, "inline_skating"),
            Sport::RockClimbing => writeln!(f, "rock_climbing"),
            Sport::Sailing => writeln!(f, "sailing"),
            Sport::IceSkating => writeln!(f, "ice_skating"),
            Sport::SkyDiving => writeln!(f, "sky_diving"),
            Sport::Snowshoeing => writeln!(f, "snowshoeing"),
            Sport::Snowmobiling => writeln!(f, "snowmobiling"),
            Sport::StandUpPaddleboarding => writeln!(f, "stand_up_paddleboarding"),
            Sport::Surfing => writeln!(f, "surfing"),
            Sport::Wakeboarding => writeln!(f, "wakeboarding"),
            Sport::WaterSkiing => writeln!(f, "water_skiing"),
            Sport::Kayaking => writeln!(f, "kayaking"),
            Sport::Rafting => writeln!(f, "rafting"),
            Sport::Windsurfing => writeln!(f, "windsurfing"),
            Sport::Kitesurfing => writeln!(f, "kitesurfing"),
            Sport::Tactical => writeln!(f, "tactical"),
            Sport::Jumpmaster => writeln!(f, "jumpmaster"),
            Sport::Boxing => writeln!(f, "boxing"),
            Sport::FloorClimbing => writeln!(f, "floor_climbing"),
            Sport::All => writeln!(f, "all"),
            Sport::UnknownVariant(value) => writeln!(f, "unknown_variant_{}", *value),
        }
    }
}
impl convert::From<u8> for Sport {
    fn from(value: u8) -> Self {
        match value {
            0 => Sport::Generic,
            1 => Sport::Running,
            2 => Sport::Cycling,
            3 => Sport::Transition,
            4 => Sport::FitnessEquipment,
            5 => Sport::Swimming,
            6 => Sport::Basketball,
            7 => Sport::Soccer,
            8 => Sport::Tennis,
            9 => Sport::AmericanFootball,
            10 => Sport::Training,
            11 => Sport::Walking,
            12 => Sport::CrossCountrySkiing,
            13 => Sport::AlpineSkiing,
            14 => Sport::Snowboarding,
            15 => Sport::Rowing,
            16 => Sport::Mountaineering,
            17 => Sport::Hiking,
            18 => Sport::Multisport,
            19 => Sport::Paddling,
            20 => Sport::Flying,
            21 => Sport::EBiking,
            22 => Sport::Motorcycling,
            23 => Sport::Boating,
            24 => Sport::Driving,
            25 => Sport::Golf,
            26 => Sport::HangGliding,
            27 => Sport::HorsebackRiding,
            28 => Sport::Hunting,
            29 => Sport::Fishing,
            30 => Sport::InlineSkating,
            31 => Sport::RockClimbing,
            32 => Sport::Sailing,
            33 => Sport::IceSkating,
            34 => Sport::SkyDiving,
            35 => Sport::Snowshoeing,
            36 => Sport::Snowmobiling,
            37 => Sport::StandUpPaddleboarding,
            38 => Sport::Surfing,
            39 => Sport::Wakeboarding,
            40 => Sport::WaterSkiing,
            41 => Sport::Kayaking,
            42 => Sport::Rafting,
            43 => Sport::Windsurfing,
            44 => Sport::Kitesurfing,
            45 => Sport::Tactical,
            46 => Sport::Jumpmaster,
            47 => Sport::Boxing,
            48 => Sport::FloorClimbing,
            254 => Sport::All,
            _ => Sport::UnknownVariant(value),
        }
    }
}
impl convert::From<i64> for Sport {
    fn from(value: i64) -> Self {
        Sport::from(value as u8)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum SportBits0 {
    Generic,
    Running,
    Cycling,
    /// Mulitsport transition
    Transition,
    FitnessEquipment,
    Swimming,
    Basketball,
    Soccer,
    UnknownVariant(u8),
}
impl SportBits0 {
    pub fn as_u8(self) -> u8 {
        match self {
            SportBits0::Generic => 1,
            SportBits0::Running => 2,
            SportBits0::Cycling => 4,
            SportBits0::Transition => 8,
            SportBits0::FitnessEquipment => 16,
            SportBits0::Swimming => 32,
            SportBits0::Basketball => 64,
            SportBits0::Soccer => 128,
            SportBits0::UnknownVariant(value) => value,
        }
    }
    pub fn as_i64(self) -> i64 {
        self.as_u8() as i64
    }
}
impl fmt::Display for SportBits0 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            SportBits0::Generic => writeln!(f, "generic"),
            SportBits0::Running => writeln!(f, "running"),
            SportBits0::Cycling => writeln!(f, "cycling"),
            SportBits0::Transition => writeln!(f, "transition"),
            SportBits0::FitnessEquipment => writeln!(f, "fitness_equipment"),
            SportBits0::Swimming => writeln!(f, "swimming"),
            SportBits0::Basketball => writeln!(f, "basketball"),
            SportBits0::Soccer => writeln!(f, "soccer"),
            SportBits0::UnknownVariant(value) => writeln!(f, "unknown_variant_{}", *value),
        }
    }
}
impl convert::From<u8> for SportBits0 {
    fn from(value: u8) -> Self {
        match value {
            1 => SportBits0::Generic,
            2 => SportBits0::Running,
            4 => SportBits0::Cycling,
            8 => SportBits0::Transition,
            16 => SportBits0::FitnessEquipment,
            32 => SportBits0::Swimming,
            64 => SportBits0::Basketball,
            128 => SportBits0::Soccer,
            _ => SportBits0::UnknownVariant(value),
        }
    }
}
impl convert::From<i64> for SportBits0 {
    fn from(value: i64) -> Self {
        SportBits0::from(value as u8)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum SportBits1 {
    Tennis,
    AmericanFootball,
    Training,
    Walking,
    CrossCountrySkiing,
    AlpineSkiing,
    Snowboarding,
    Rowing,
    UnknownVariant(u8),
}
impl SportBits1 {
    pub fn as_u8(self) -> u8 {
        match self {
            SportBits1::Tennis => 1,
            SportBits1::AmericanFootball => 2,
            SportBits1::Training => 4,
            SportBits1::Walking => 8,
            SportBits1::CrossCountrySkiing => 16,
            SportBits1::AlpineSkiing => 32,
            SportBits1::Snowboarding => 64,
            SportBits1::Rowing => 128,
            SportBits1::UnknownVariant(value) => value,
        }
    }
    pub fn as_i64(self) -> i64 {
        self.as_u8() as i64
    }
}
impl fmt::Display for SportBits1 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            SportBits1::Tennis => writeln!(f, "tennis"),
            SportBits1::AmericanFootball => writeln!(f, "american_football"),
            SportBits1::Training => writeln!(f, "training"),
            SportBits1::Walking => writeln!(f, "walking"),
            SportBits1::CrossCountrySkiing => writeln!(f, "cross_country_skiing"),
            SportBits1::AlpineSkiing => writeln!(f, "alpine_skiing"),
            SportBits1::Snowboarding => writeln!(f, "snowboarding"),
            SportBits1::Rowing => writeln!(f, "rowing"),
            SportBits1::UnknownVariant(value) => writeln!(f, "unknown_variant_{}", *value),
        }
    }
}
impl convert::From<u8> for SportBits1 {
    fn from(value: u8) -> Self {
        match value {
            1 => SportBits1::Tennis,
            2 => SportBits1::AmericanFootball,
            4 => SportBits1::Training,
            8 => SportBits1::Walking,
            16 => SportBits1::CrossCountrySkiing,
            32 => SportBits1::AlpineSkiing,
            64 => SportBits1::Snowboarding,
            128 => SportBits1::Rowing,
            _ => SportBits1::UnknownVariant(value),
        }
    }
}
impl convert::From<i64> for SportBits1 {
    fn from(value: i64) -> Self {
        SportBits1::from(value as u8)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum SportBits2 {
    Mountaineering,
    Hiking,
    Multisport,
    Paddling,
    Flying,
    EBiking,
    Motorcycling,
    Boating,
    UnknownVariant(u8),
}
impl SportBits2 {
    pub fn as_u8(self) -> u8 {
        match self {
            SportBits2::Mountaineering => 1,
            SportBits2::Hiking => 2,
            SportBits2::Multisport => 4,
            SportBits2::Paddling => 8,
            SportBits2::Flying => 16,
            SportBits2::EBiking => 32,
            SportBits2::Motorcycling => 64,
            SportBits2::Boating => 128,
            SportBits2::UnknownVariant(value) => value,
        }
    }
    pub fn as_i64(self) -> i64 {
        self.as_u8() as i64
    }
}
impl fmt::Display for SportBits2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            SportBits2::Mountaineering => writeln!(f, "mountaineering"),
            SportBits2::Hiking => writeln!(f, "hiking"),
            SportBits2::Multisport => writeln!(f, "multisport"),
            SportBits2::Paddling => writeln!(f, "paddling"),
            SportBits2::Flying => writeln!(f, "flying"),
            SportBits2::EBiking => writeln!(f, "e_biking"),
            SportBits2::Motorcycling => writeln!(f, "motorcycling"),
            SportBits2::Boating => writeln!(f, "boating"),
            SportBits2::UnknownVariant(value) => writeln!(f, "unknown_variant_{}", *value),
        }
    }
}
impl convert::From<u8> for SportBits2 {
    fn from(value: u8) -> Self {
        match value {
            1 => SportBits2::Mountaineering,
            2 => SportBits2::Hiking,
            4 => SportBits2::Multisport,
            8 => SportBits2::Paddling,
            16 => SportBits2::Flying,
            32 => SportBits2::EBiking,
            64 => SportBits2::Motorcycling,
            128 => SportBits2::Boating,
            _ => SportBits2::UnknownVariant(value),
        }
    }
}
impl convert::From<i64> for SportBits2 {
    fn from(value: i64) -> Self {
        SportBits2::from(value as u8)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum SportBits3 {
    Driving,
    Golf,
    HangGliding,
    HorsebackRiding,
    Hunting,
    Fishing,
    InlineSkating,
    RockClimbing,
    UnknownVariant(u8),
}
impl SportBits3 {
    pub fn as_u8(self) -> u8 {
        match self {
            SportBits3::Driving => 1,
            SportBits3::Golf => 2,
            SportBits3::HangGliding => 4,
            SportBits3::HorsebackRiding => 8,
            SportBits3::Hunting => 16,
            SportBits3::Fishing => 32,
            SportBits3::InlineSkating => 64,
            SportBits3::RockClimbing => 128,
            SportBits3::UnknownVariant(value) => value,
        }
    }
    pub fn as_i64(self) -> i64 {
        self.as_u8() as i64
    }
}
impl fmt::Display for SportBits3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            SportBits3::Driving => writeln!(f, "driving"),
            SportBits3::Golf => writeln!(f, "golf"),
            SportBits3::HangGliding => writeln!(f, "hang_gliding"),
            SportBits3::HorsebackRiding => writeln!(f, "horseback_riding"),
            SportBits3::Hunting => writeln!(f, "hunting"),
            SportBits3::Fishing => writeln!(f, "fishing"),
            SportBits3::InlineSkating => writeln!(f, "inline_skating"),
            SportBits3::RockClimbing => writeln!(f, "rock_climbing"),
            SportBits3::UnknownVariant(value) => writeln!(f, "unknown_variant_{}", *value),
        }
    }
}
impl convert::From<u8> for SportBits3 {
    fn from(value: u8) -> Self {
        match value {
            1 => SportBits3::Driving,
            2 => SportBits3::Golf,
            4 => SportBits3::HangGliding,
            8 => SportBits3::HorsebackRiding,
            16 => SportBits3::Hunting,
            32 => SportBits3::Fishing,
            64 => SportBits3::InlineSkating,
            128 => SportBits3::RockClimbing,
            _ => SportBits3::UnknownVariant(value),
        }
    }
}
impl convert::From<i64> for SportBits3 {
    fn from(value: i64) -> Self {
        SportBits3::from(value as u8)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum SportBits4 {
    Sailing,
    IceSkating,
    SkyDiving,
    Snowshoeing,
    Snowmobiling,
    StandUpPaddleboarding,
    Surfing,
    Wakeboarding,
    UnknownVariant(u8),
}
impl SportBits4 {
    pub fn as_u8(self) -> u8 {
        match self {
            SportBits4::Sailing => 1,
            SportBits4::IceSkating => 2,
            SportBits4::SkyDiving => 4,
            SportBits4::Snowshoeing => 8,
            SportBits4::Snowmobiling => 16,
            SportBits4::StandUpPaddleboarding => 32,
            SportBits4::Surfing => 64,
            SportBits4::Wakeboarding => 128,
            SportBits4::UnknownVariant(value) => value,
        }
    }
    pub fn as_i64(self) -> i64 {
        self.as_u8() as i64
    }
}
impl fmt::Display for SportBits4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            SportBits4::Sailing => writeln!(f, "sailing"),
            SportBits4::IceSkating => writeln!(f, "ice_skating"),
            SportBits4::SkyDiving => writeln!(f, "sky_diving"),
            SportBits4::Snowshoeing => writeln!(f, "snowshoeing"),
            SportBits4::Snowmobiling => writeln!(f, "snowmobiling"),
            SportBits4::StandUpPaddleboarding => writeln!(f, "stand_up_paddleboarding"),
            SportBits4::Surfing => writeln!(f, "surfing"),
            SportBits4::Wakeboarding => writeln!(f, "wakeboarding"),
            SportBits4::UnknownVariant(value) => writeln!(f, "unknown_variant_{}", *value),
        }
    }
}
impl convert::From<u8> for SportBits4 {
    fn from(value: u8) -> Self {
        match value {
            1 => SportBits4::Sailing,
            2 => SportBits4::IceSkating,
            4 => SportBits4::SkyDiving,
            8 => SportBits4::Snowshoeing,
            16 => SportBits4::Snowmobiling,
            32 => SportBits4::StandUpPaddleboarding,
            64 => SportBits4::Surfing,
            128 => SportBits4::Wakeboarding,
            _ => SportBits4::UnknownVariant(value),
        }
    }
}
impl convert::From<i64> for SportBits4 {
    fn from(value: i64) -> Self {
        SportBits4::from(value as u8)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum SportBits5 {
    WaterSkiing,
    Kayaking,
    Rafting,
    Windsurfing,
    Kitesurfing,
    Tactical,
    Jumpmaster,
    Boxing,
    UnknownVariant(u8),
}
impl SportBits5 {
    pub fn as_u8(self) -> u8 {
        match self {
            SportBits5::WaterSkiing => 1,
            SportBits5::Kayaking => 2,
            SportBits5::Rafting => 4,
            SportBits5::Windsurfing => 8,
            SportBits5::Kitesurfing => 16,
            SportBits5::Tactical => 32,
            SportBits5::Jumpmaster => 64,
            SportBits5::Boxing => 128,
            SportBits5::UnknownVariant(value) => value,
        }
    }
    pub fn as_i64(self) -> i64 {
        self.as_u8() as i64
    }
}
impl fmt::Display for SportBits5 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            SportBits5::WaterSkiing => writeln!(f, "water_skiing"),
            SportBits5::Kayaking => writeln!(f, "kayaking"),
            SportBits5::Rafting => writeln!(f, "rafting"),
            SportBits5::Windsurfing => writeln!(f, "windsurfing"),
            SportBits5::Kitesurfing => writeln!(f, "kitesurfing"),
            SportBits5::Tactical => writeln!(f, "tactical"),
            SportBits5::Jumpmaster => writeln!(f, "jumpmaster"),
            SportBits5::Boxing => writeln!(f, "boxing"),
            SportBits5::UnknownVariant(value) => writeln!(f, "unknown_variant_{}", *value),
        }
    }
}
impl convert::From<u8> for SportBits5 {
    fn from(value: u8) -> Self {
        match value {
            1 => SportBits5::WaterSkiing,
            2 => SportBits5::Kayaking,
            4 => SportBits5::Rafting,
            8 => SportBits5::Windsurfing,
            16 => SportBits5::Kitesurfing,
            32 => SportBits5::Tactical,
            64 => SportBits5::Jumpmaster,
            128 => SportBits5::Boxing,
            _ => SportBits5::UnknownVariant(value),
        }
    }
}
impl convert::From<i64> for SportBits5 {
    fn from(value: i64) -> Self {
        SportBits5::from(value as u8)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum SportBits6 {
    FloorClimbing,
    UnknownVariant(u8),
}
impl SportBits6 {
    pub fn as_u8(self) -> u8 {
        match self {
            SportBits6::FloorClimbing => 1,
            SportBits6::UnknownVariant(value) => value,
        }
    }
    pub fn as_i64(self) -> i64 {
        self.as_u8() as i64
    }
}
impl fmt::Display for SportBits6 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            SportBits6::FloorClimbing => writeln!(f, "floor_climbing"),
            SportBits6::UnknownVariant(value) => writeln!(f, "unknown_variant_{}", *value),
        }
    }
}
impl convert::From<u8> for SportBits6 {
    fn from(value: u8) -> Self {
        match value {
            1 => SportBits6::FloorClimbing,
            _ => SportBits6::UnknownVariant(value),
        }
    }
}
impl convert::From<i64> for SportBits6 {
    fn from(value: i64) -> Self {
        SportBits6::from(value as u8)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum SubSport {
    Generic,
    /// Run/Fitness Equipment
    Treadmill,
    /// Run
    Street,
    /// Run
    Trail,
    /// Run
    Track,
    /// Cycling
    Spin,
    /// Cycling/Fitness Equipment
    IndoorCycling,
    /// Cycling
    Road,
    /// Cycling
    Mountain,
    /// Cycling
    Downhill,
    /// Cycling
    Recumbent,
    /// Cycling
    Cyclocross,
    /// Cycling
    HandCycling,
    /// Cycling
    TrackCycling,
    /// Fitness Equipment
    IndoorRowing,
    /// Fitness Equipment
    Elliptical,
    /// Fitness Equipment
    StairClimbing,
    /// Swimming
    LapSwimming,
    /// Swimming
    OpenWater,
    /// Training
    FlexibilityTraining,
    /// Training
    StrengthTraining,
    /// Tennis
    WarmUp,
    /// Tennis
    Match,
    /// Tennis
    Exercise,
    Challenge,
    /// Fitness Equipment
    IndoorSkiing,
    /// Training
    CardioTraining,
    /// Walking/Fitness Equipment
    IndoorWalking,
    /// E-Biking
    EBikeFitness,
    /// Cycling
    Bmx,
    /// Walking
    CasualWalking,
    /// Walking
    SpeedWalking,
    /// Transition
    BikeToRunTransition,
    /// Transition
    RunToBikeTransition,
    /// Transition
    SwimToBikeTransition,
    /// Motorcycling
    Atv,
    /// Motorcycling
    Motocross,
    /// Alpine Skiing/Snowboarding
    Backcountry,
    /// Alpine Skiing/Snowboarding
    Resort,
    /// Flying
    RcDrone,
    /// Flying
    Wingsuit,
    /// Kayaking/Rafting
    Whitewater,
    /// Cross Country Skiing
    SkateSkiing,
    /// Training
    Yoga,
    /// Fitness Equipment
    Pilates,
    /// Run
    IndoorRunning,
    /// Cycling
    GravelCycling,
    /// Cycling
    EBikeMountain,
    /// Cycling
    Commuting,
    /// Cycling
    MixedSurface,
    Navigate,
    TrackMe,
    Map,
    /// Diving
    SingleGasDiving,
    /// Diving
    MultiGasDiving,
    /// Diving
    GaugeDiving,
    /// Diving
    ApneaDiving,
    /// Diving
    ApneaHunting,
    VirtualActivity,
    /// Used for events where participants run, crawl through mud, climb over walls, etc.
    Obstacle,
    All,
    UnknownVariant(u8),
}
impl SubSport {
    pub fn as_u8(self) -> u8 {
        match self {
            SubSport::Generic => 0,
            SubSport::Treadmill => 1,
            SubSport::Street => 2,
            SubSport::Trail => 3,
            SubSport::Track => 4,
            SubSport::Spin => 5,
            SubSport::IndoorCycling => 6,
            SubSport::Road => 7,
            SubSport::Mountain => 8,
            SubSport::Downhill => 9,
            SubSport::Recumbent => 10,
            SubSport::Cyclocross => 11,
            SubSport::HandCycling => 12,
            SubSport::TrackCycling => 13,
            SubSport::IndoorRowing => 14,
            SubSport::Elliptical => 15,
            SubSport::StairClimbing => 16,
            SubSport::LapSwimming => 17,
            SubSport::OpenWater => 18,
            SubSport::FlexibilityTraining => 19,
            SubSport::StrengthTraining => 20,
            SubSport::WarmUp => 21,
            SubSport::Match => 22,
            SubSport::Exercise => 23,
            SubSport::Challenge => 24,
            SubSport::IndoorSkiing => 25,
            SubSport::CardioTraining => 26,
            SubSport::IndoorWalking => 27,
            SubSport::EBikeFitness => 28,
            SubSport::Bmx => 29,
            SubSport::CasualWalking => 30,
            SubSport::SpeedWalking => 31,
            SubSport::BikeToRunTransition => 32,
            SubSport::RunToBikeTransition => 33,
            SubSport::SwimToBikeTransition => 34,
            SubSport::Atv => 35,
            SubSport::Motocross => 36,
            SubSport::Backcountry => 37,
            SubSport::Resort => 38,
            SubSport::RcDrone => 39,
            SubSport::Wingsuit => 40,
            SubSport::Whitewater => 41,
            SubSport::SkateSkiing => 42,
            SubSport::Yoga => 43,
            SubSport::Pilates => 44,
            SubSport::IndoorRunning => 45,
            SubSport::GravelCycling => 46,
            SubSport::EBikeMountain => 47,
            SubSport::Commuting => 48,
            SubSport::MixedSurface => 49,
            SubSport::Navigate => 50,
            SubSport::TrackMe => 51,
            SubSport::Map => 52,
            SubSport::SingleGasDiving => 53,
            SubSport::MultiGasDiving => 54,
            SubSport::GaugeDiving => 55,
            SubSport::ApneaDiving => 56,
            SubSport::ApneaHunting => 57,
            SubSport::VirtualActivity => 58,
            SubSport::Obstacle => 59,
            SubSport::All => 254,
            SubSport::UnknownVariant(value) => value,
        }
    }
    pub fn as_i64(self) -> i64 {
        self.as_u8() as i64
    }
}
impl fmt::Display for SubSport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            SubSport::Generic => writeln!(f, "generic"),
            SubSport::Treadmill => writeln!(f, "treadmill"),
            SubSport::Street => writeln!(f, "street"),
            SubSport::Trail => writeln!(f, "trail"),
            SubSport::Track => writeln!(f, "track"),
            SubSport::Spin => writeln!(f, "spin"),
            SubSport::IndoorCycling => writeln!(f, "indoor_cycling"),
            SubSport::Road => writeln!(f, "road"),
            SubSport::Mountain => writeln!(f, "mountain"),
            SubSport::Downhill => writeln!(f, "downhill"),
            SubSport::Recumbent => writeln!(f, "recumbent"),
            SubSport::Cyclocross => writeln!(f, "cyclocross"),
            SubSport::HandCycling => writeln!(f, "hand_cycling"),
            SubSport::TrackCycling => writeln!(f, "track_cycling"),
            SubSport::IndoorRowing => writeln!(f, "indoor_rowing"),
            SubSport::Elliptical => writeln!(f, "elliptical"),
            SubSport::StairClimbing => writeln!(f, "stair_climbing"),
            SubSport::LapSwimming => writeln!(f, "lap_swimming"),
            SubSport::OpenWater => writeln!(f, "open_water"),
            SubSport::FlexibilityTraining => writeln!(f, "flexibility_training"),
            SubSport::StrengthTraining => writeln!(f, "strength_training"),
            SubSport::WarmUp => writeln!(f, "warm_up"),
            SubSport::Match => writeln!(f, "match"),
            SubSport::Exercise => writeln!(f, "exercise"),
            SubSport::Challenge => writeln!(f, "challenge"),
            SubSport::IndoorSkiing => writeln!(f, "indoor_skiing"),
            SubSport::CardioTraining => writeln!(f, "cardio_training"),
            SubSport::IndoorWalking => writeln!(f, "indoor_walking"),
            SubSport::EBikeFitness => writeln!(f, "e_bike_fitness"),
            SubSport::Bmx => writeln!(f, "bmx"),
            SubSport::CasualWalking => writeln!(f, "casual_walking"),
            SubSport::SpeedWalking => writeln!(f, "speed_walking"),
            SubSport::BikeToRunTransition => writeln!(f, "bike_to_run_transition"),
            SubSport::RunToBikeTransition => writeln!(f, "run_to_bike_transition"),
            SubSport::SwimToBikeTransition => writeln!(f, "swim_to_bike_transition"),
            SubSport::Atv => writeln!(f, "atv"),
            SubSport::Motocross => writeln!(f, "motocross"),
            SubSport::Backcountry => writeln!(f, "backcountry"),
            SubSport::Resort => writeln!(f, "resort"),
            SubSport::RcDrone => writeln!(f, "rc_drone"),
            SubSport::Wingsuit => writeln!(f, "wingsuit"),
            SubSport::Whitewater => writeln!(f, "whitewater"),
            SubSport::SkateSkiing => writeln!(f, "skate_skiing"),
            SubSport::Yoga => writeln!(f, "yoga"),
            SubSport::Pilates => writeln!(f, "pilates"),
            SubSport::IndoorRunning => writeln!(f, "indoor_running"),
            SubSport::GravelCycling => writeln!(f, "gravel_cycling"),
            SubSport::EBikeMountain => writeln!(f, "e_bike_mountain"),
            SubSport::Commuting => writeln!(f, "commuting"),
            SubSport::MixedSurface => writeln!(f, "mixed_surface"),
            SubSport::Navigate => writeln!(f, "navigate"),
            SubSport::TrackMe => writeln!(f, "track_me"),
            SubSport::Map => writeln!(f, "map"),
            SubSport::SingleGasDiving => writeln!(f, "single_gas_diving"),
            SubSport::MultiGasDiving => writeln!(f, "multi_gas_diving"),
            SubSport::GaugeDiving => writeln!(f, "gauge_diving"),
            SubSport::ApneaDiving => writeln!(f, "apnea_diving"),
            SubSport::ApneaHunting => writeln!(f, "apnea_hunting"),
            SubSport::VirtualActivity => writeln!(f, "virtual_activity"),
            SubSport::Obstacle => writeln!(f, "obstacle"),
            SubSport::All => writeln!(f, "all"),
            SubSport::UnknownVariant(value) => writeln!(f, "unknown_variant_{}", *value),
        }
    }
}
impl convert::From<u8> for SubSport {
    fn from(value: u8) -> Self {
        match value {
            0 => SubSport::Generic,
            1 => SubSport::Treadmill,
            2 => SubSport::Street,
            3 => SubSport::Trail,
            4 => SubSport::Track,
            5 => SubSport::Spin,
            6 => SubSport::IndoorCycling,
            7 => SubSport::Road,
            8 => SubSport::Mountain,
            9 => SubSport::Downhill,
            10 => SubSport::Recumbent,
            11 => SubSport::Cyclocross,
            12 => SubSport::HandCycling,
            13 => SubSport::TrackCycling,
            14 => SubSport::IndoorRowing,
            15 => SubSport::Elliptical,
            16 => SubSport::StairClimbing,
            17 => SubSport::LapSwimming,
            18 => SubSport::OpenWater,
            19 => SubSport::FlexibilityTraining,
            20 => SubSport::StrengthTraining,
            21 => SubSport::WarmUp,
            22 => SubSport::Match,
            23 => SubSport::Exercise,
            24 => SubSport::Challenge,
            25 => SubSport::IndoorSkiing,
            26 => SubSport::CardioTraining,
            27 => SubSport::IndoorWalking,
            28 => SubSport::EBikeFitness,
            29 => SubSport::Bmx,
            30 => SubSport::CasualWalking,
            31 => SubSport::SpeedWalking,
            32 => SubSport::BikeToRunTransition,
            33 => SubSport::RunToBikeTransition,
            34 => SubSport::SwimToBikeTransition,
            35 => SubSport::Atv,
            36 => SubSport::Motocross,
            37 => SubSport::Backcountry,
            38 => SubSport::Resort,
            39 => SubSport::RcDrone,
            40 => SubSport::Wingsuit,
            41 => SubSport::Whitewater,
            42 => SubSport::SkateSkiing,
            43 => SubSport::Yoga,
            44 => SubSport::Pilates,
            45 => SubSport::IndoorRunning,
            46 => SubSport::GravelCycling,
            47 => SubSport::EBikeMountain,
            48 => SubSport::Commuting,
            49 => SubSport::MixedSurface,
            50 => SubSport::Navigate,
            51 => SubSport::TrackMe,
            52 => SubSport::Map,
            53 => SubSport::SingleGasDiving,
            54 => SubSport::MultiGasDiving,
            55 => SubSport::GaugeDiving,
            56 => SubSport::ApneaDiving,
            57 => SubSport::ApneaHunting,
            58 => SubSport::VirtualActivity,
            59 => SubSport::Obstacle,
            254 => SubSport::All,
            _ => SubSport::UnknownVariant(value),
        }
    }
}
impl convert::From<i64> for SubSport {
    fn from(value: i64) -> Self {
        SubSport::from(value as u8)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum SportEvent {
    Uncategorized,
    Geocaching,
    Fitness,
    Recreation,
    Race,
    SpecialEvent,
    Training,
    Transportation,
    Touring,
    UnknownVariant(u8),
}
impl SportEvent {
    pub fn as_u8(self) -> u8 {
        match self {
            SportEvent::Uncategorized => 0,
            SportEvent::Geocaching => 1,
            SportEvent::Fitness => 2,
            SportEvent::Recreation => 3,
            SportEvent::Race => 4,
            SportEvent::SpecialEvent => 5,
            SportEvent::Training => 6,
            SportEvent::Transportation => 7,
            SportEvent::Touring => 8,
            SportEvent::UnknownVariant(value) => value,
        }
    }
    pub fn as_i64(self) -> i64 {
        self.as_u8() as i64
    }
}
impl fmt::Display for SportEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            SportEvent::Uncategorized => writeln!(f, "uncategorized"),
            SportEvent::Geocaching => writeln!(f, "geocaching"),
            SportEvent::Fitness => writeln!(f, "fitness"),
            SportEvent::Recreation => writeln!(f, "recreation"),
            SportEvent::Race => writeln!(f, "race"),
            SportEvent::SpecialEvent => writeln!(f, "special_event"),
            SportEvent::Training => writeln!(f, "training"),
            SportEvent::Transportation => writeln!(f, "transportation"),
            SportEvent::Touring => writeln!(f, "touring"),
            SportEvent::UnknownVariant(value) => writeln!(f, "unknown_variant_{}", *value),
        }
    }
}
impl convert::From<u8> for SportEvent {
    fn from(value: u8) -> Self {
        match value {
            0 => SportEvent::Uncategorized,
            1 => SportEvent::Geocaching,
            2 => SportEvent::Fitness,
            3 => SportEvent::Recreation,
            4 => SportEvent::Race,
            5 => SportEvent::SpecialEvent,
            6 => SportEvent::Training,
            7 => SportEvent::Transportation,
            8 => SportEvent::Touring,
            _ => SportEvent::UnknownVariant(value),
        }
    }
}
impl convert::From<i64> for SportEvent {
    fn from(value: i64) -> Self {
        SportEvent::from(value as u8)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum Activity {
    Manual,
    AutoMultiSport,
    UnknownVariant(u8),
}
impl Activity {
    pub fn as_u8(self) -> u8 {
        match self {
            Activity::Manual => 0,
            Activity::AutoMultiSport => 1,
            Activity::UnknownVariant(value) => value,
        }
    }
    pub fn as_i64(self) -> i64 {
        self.as_u8() as i64
    }
}
impl fmt::Display for Activity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Activity::Manual => writeln!(f, "manual"),
            Activity::AutoMultiSport => writeln!(f, "auto_multi_sport"),
            Activity::UnknownVariant(value) => writeln!(f, "unknown_variant_{}", *value),
        }
    }
}
impl convert::From<u8> for Activity {
    fn from(value: u8) -> Self {
        match value {
            0 => Activity::Manual,
            1 => Activity::AutoMultiSport,
            _ => Activity::UnknownVariant(value),
        }
    }
}
impl convert::From<i64> for Activity {
    fn from(value: i64) -> Self {
        Activity::from(value as u8)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum Intensity {
    Active,
    Rest,
    Warmup,
    Cooldown,
    UnknownVariant(u8),
}
impl Intensity {
    pub fn as_u8(self) -> u8 {
        match self {
            Intensity::Active => 0,
            Intensity::Rest => 1,
            Intensity::Warmup => 2,
            Intensity::Cooldown => 3,
            Intensity::UnknownVariant(value) => value,
        }
    }
    pub fn as_i64(self) -> i64 {
        self.as_u8() as i64
    }
}
impl fmt::Display for Intensity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Intensity::Active => writeln!(f, "active"),
            Intensity::Rest => writeln!(f, "rest"),
            Intensity::Warmup => writeln!(f, "warmup"),
            Intensity::Cooldown => writeln!(f, "cooldown"),
            Intensity::UnknownVariant(value) => writeln!(f, "unknown_variant_{}", *value),
        }
    }
}
impl convert::From<u8> for Intensity {
    fn from(value: u8) -> Self {
        match value {
            0 => Intensity::Active,
            1 => Intensity::Rest,
            2 => Intensity::Warmup,
            3 => Intensity::Cooldown,
            _ => Intensity::UnknownVariant(value),
        }
    }
}
impl convert::From<i64> for Intensity {
    fn from(value: i64) -> Self {
        Intensity::from(value as u8)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum SessionTrigger {
    ActivityEnd,
    /// User changed sport.
    Manual,
    /// Auto multi-sport feature is enabled and user pressed lap button to advance session.
    AutoMultiSport,
    /// Auto sport change caused by user linking to fitness equipment.
    FitnessEquipment,
    UnknownVariant(u8),
}
impl SessionTrigger {
    pub fn as_u8(self) -> u8 {
        match self {
            SessionTrigger::ActivityEnd => 0,
            SessionTrigger::Manual => 1,
            SessionTrigger::AutoMultiSport => 2,
            SessionTrigger::FitnessEquipment => 3,
            SessionTrigger::UnknownVariant(value) => value,
        }
    }
    pub fn as_i64(self) -> i64 {
        self.as_u8() as i64
    }
}
impl fmt::Display for SessionTrigger {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            SessionTrigger::ActivityEnd => writeln!(f, "activity_end"),
            SessionTrigger::Manual => writeln!(f, "manual"),
            SessionTrigger::AutoMultiSport => writeln!(f, "auto_multi_sport"),
            SessionTrigger::FitnessEquipment => writeln!(f, "fitness_equipment"),
            SessionTrigger::UnknownVariant(value) => writeln!(f, "unknown_variant_{}", *value),
        }
    }
}
impl convert::From<u8> for SessionTrigger {
    fn from(value: u8) -> Self {
        match value {
            0 => SessionTrigger::ActivityEnd,
            1 => SessionTrigger::Manual,
            2 => SessionTrigger::AutoMultiSport,
            3 => SessionTrigger::FitnessEquipment,
            _ => SessionTrigger::UnknownVariant(value),
        }
    }
}
impl convert::From<i64> for SessionTrigger {
    fn from(value: i64) -> Self {
        SessionTrigger::from(value as u8)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum AutolapTrigger {
    Time,
    Distance,
    PositionStart,
    PositionLap,
    PositionWaypoint,
    PositionMarked,
    Off,
    UnknownVariant(u8),
}
impl AutolapTrigger {
    pub fn as_u8(self) -> u8 {
        match self {
            AutolapTrigger::Time => 0,
            AutolapTrigger::Distance => 1,
            AutolapTrigger::PositionStart => 2,
            AutolapTrigger::PositionLap => 3,
            AutolapTrigger::PositionWaypoint => 4,
            AutolapTrigger::PositionMarked => 5,
            AutolapTrigger::Off => 6,
            AutolapTrigger::UnknownVariant(value) => value,
        }
    }
    pub fn as_i64(self) -> i64 {
        self.as_u8() as i64
    }
}
impl fmt::Display for AutolapTrigger {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            AutolapTrigger::Time => writeln!(f, "time"),
            AutolapTrigger::Distance => writeln!(f, "distance"),
            AutolapTrigger::PositionStart => writeln!(f, "position_start"),
            AutolapTrigger::PositionLap => writeln!(f, "position_lap"),
            AutolapTrigger::PositionWaypoint => writeln!(f, "position_waypoint"),
            AutolapTrigger::PositionMarked => writeln!(f, "position_marked"),
            AutolapTrigger::Off => writeln!(f, "off"),
            AutolapTrigger::UnknownVariant(value) => writeln!(f, "unknown_variant_{}", *value),
        }
    }
}
impl convert::From<u8> for AutolapTrigger {
    fn from(value: u8) -> Self {
        match value {
            0 => AutolapTrigger::Time,
            1 => AutolapTrigger::Distance,
            2 => AutolapTrigger::PositionStart,
            3 => AutolapTrigger::PositionLap,
            4 => AutolapTrigger::PositionWaypoint,
            5 => AutolapTrigger::PositionMarked,
            6 => AutolapTrigger::Off,
            _ => AutolapTrigger::UnknownVariant(value),
        }
    }
}
impl convert::From<i64> for AutolapTrigger {
    fn from(value: i64) -> Self {
        AutolapTrigger::from(value as u8)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum LapTrigger {
    Manual,
    Time,
    Distance,
    PositionStart,
    PositionLap,
    PositionWaypoint,
    PositionMarked,
    SessionEnd,
    FitnessEquipment,
    UnknownVariant(u8),
}
impl LapTrigger {
    pub fn as_u8(self) -> u8 {
        match self {
            LapTrigger::Manual => 0,
            LapTrigger::Time => 1,
            LapTrigger::Distance => 2,
            LapTrigger::PositionStart => 3,
            LapTrigger::PositionLap => 4,
            LapTrigger::PositionWaypoint => 5,
            LapTrigger::PositionMarked => 6,
            LapTrigger::SessionEnd => 7,
            LapTrigger::FitnessEquipment => 8,
            LapTrigger::UnknownVariant(value) => value,
        }
    }
    pub fn as_i64(self) -> i64 {
        self.as_u8() as i64
    }
}
impl fmt::Display for LapTrigger {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            LapTrigger::Manual => writeln!(f, "manual"),
            LapTrigger::Time => writeln!(f, "time"),
            LapTrigger::Distance => writeln!(f, "distance"),
            LapTrigger::PositionStart => writeln!(f, "position_start"),
            LapTrigger::PositionLap => writeln!(f, "position_lap"),
            LapTrigger::PositionWaypoint => writeln!(f, "position_waypoint"),
            LapTrigger::PositionMarked => writeln!(f, "position_marked"),
            LapTrigger::SessionEnd => writeln!(f, "session_end"),
            LapTrigger::FitnessEquipment => writeln!(f, "fitness_equipment"),
            LapTrigger::UnknownVariant(value) => writeln!(f, "unknown_variant_{}", *value),
        }
    }
}
impl convert::From<u8> for LapTrigger {
    fn from(value: u8) -> Self {
        match value {
            0 => LapTrigger::Manual,
            1 => LapTrigger::Time,
            2 => LapTrigger::Distance,
            3 => LapTrigger::PositionStart,
            4 => LapTrigger::PositionLap,
            5 => LapTrigger::PositionWaypoint,
            6 => LapTrigger::PositionMarked,
            7 => LapTrigger::SessionEnd,
            8 => LapTrigger::FitnessEquipment,
            _ => LapTrigger::UnknownVariant(value),
        }
    }
}
impl convert::From<i64> for LapTrigger {
    fn from(value: i64) -> Self {
        LapTrigger::from(value as u8)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum TimeMode {
    Hour12,
    /// Does not use a leading zero and has a colon
    Hour24,
    /// Uses a leading zero and does not have a colon
    Military,
    Hour12WithSeconds,
    Hour24WithSeconds,
    Utc,
    UnknownVariant(u8),
}
impl TimeMode {
    pub fn as_u8(self) -> u8 {
        match self {
            TimeMode::Hour12 => 0,
            TimeMode::Hour24 => 1,
            TimeMode::Military => 2,
            TimeMode::Hour12WithSeconds => 3,
            TimeMode::Hour24WithSeconds => 4,
            TimeMode::Utc => 5,
            TimeMode::UnknownVariant(value) => value,
        }
    }
    pub fn as_i64(self) -> i64 {
        self.as_u8() as i64
    }
}
impl fmt::Display for TimeMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            TimeMode::Hour12 => writeln!(f, "hour12"),
            TimeMode::Hour24 => writeln!(f, "hour24"),
            TimeMode::Military => writeln!(f, "military"),
            TimeMode::Hour12WithSeconds => writeln!(f, "hour_12_with_seconds"),
            TimeMode::Hour24WithSeconds => writeln!(f, "hour_24_with_seconds"),
            TimeMode::Utc => writeln!(f, "utc"),
            TimeMode::UnknownVariant(value) => writeln!(f, "unknown_variant_{}", *value),
        }
    }
}
impl convert::From<u8> for TimeMode {
    fn from(value: u8) -> Self {
        match value {
            0 => TimeMode::Hour12,
            1 => TimeMode::Hour24,
            2 => TimeMode::Military,
            3 => TimeMode::Hour12WithSeconds,
            4 => TimeMode::Hour24WithSeconds,
            5 => TimeMode::Utc,
            _ => TimeMode::UnknownVariant(value),
        }
    }
}
impl convert::From<i64> for TimeMode {
    fn from(value: i64) -> Self {
        TimeMode::from(value as u8)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum BacklightMode {
    Off,
    Manual,
    KeyAndMessages,
    AutoBrightness,
    SmartNotifications,
    KeyAndMessagesNight,
    KeyAndMessagesAndSmartNotifications,
    UnknownVariant(u8),
}
impl BacklightMode {
    pub fn as_u8(self) -> u8 {
        match self {
            BacklightMode::Off => 0,
            BacklightMode::Manual => 1,
            BacklightMode::KeyAndMessages => 2,
            BacklightMode::AutoBrightness => 3,
            BacklightMode::SmartNotifications => 4,
            BacklightMode::KeyAndMessagesNight => 5,
            BacklightMode::KeyAndMessagesAndSmartNotifications => 6,
            BacklightMode::UnknownVariant(value) => value,
        }
    }
    pub fn as_i64(self) -> i64 {
        self.as_u8() as i64
    }
}
impl fmt::Display for BacklightMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            BacklightMode::Off => writeln!(f, "off"),
            BacklightMode::Manual => writeln!(f, "manual"),
            BacklightMode::KeyAndMessages => writeln!(f, "key_and_messages"),
            BacklightMode::AutoBrightness => writeln!(f, "auto_brightness"),
            BacklightMode::SmartNotifications => writeln!(f, "smart_notifications"),
            BacklightMode::KeyAndMessagesNight => writeln!(f, "key_and_messages_night"),
            BacklightMode::KeyAndMessagesAndSmartNotifications => {
                writeln!(f, "key_and_messages_and_smart_notifications")
            }
            BacklightMode::UnknownVariant(value) => writeln!(f, "unknown_variant_{}", *value),
        }
    }
}
impl convert::From<u8> for BacklightMode {
    fn from(value: u8) -> Self {
        match value {
            0 => BacklightMode::Off,
            1 => BacklightMode::Manual,
            2 => BacklightMode::KeyAndMessages,
            3 => BacklightMode::AutoBrightness,
            4 => BacklightMode::SmartNotifications,
            5 => BacklightMode::KeyAndMessagesNight,
            6 => BacklightMode::KeyAndMessagesAndSmartNotifications,
            _ => BacklightMode::UnknownVariant(value),
        }
    }
}
impl convert::From<i64> for BacklightMode {
    fn from(value: i64) -> Self {
        BacklightMode::from(value as u8)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum DateMode {
    DayMonth,
    MonthDay,
    UnknownVariant(u8),
}
impl DateMode {
    pub fn as_u8(self) -> u8 {
        match self {
            DateMode::DayMonth => 0,
            DateMode::MonthDay => 1,
            DateMode::UnknownVariant(value) => value,
        }
    }
    pub fn as_i64(self) -> i64 {
        self.as_u8() as i64
    }
}
impl fmt::Display for DateMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            DateMode::DayMonth => writeln!(f, "day_month"),
            DateMode::MonthDay => writeln!(f, "month_day"),
            DateMode::UnknownVariant(value) => writeln!(f, "unknown_variant_{}", *value),
        }
    }
}
impl convert::From<u8> for DateMode {
    fn from(value: u8) -> Self {
        match value {
            0 => DateMode::DayMonth,
            1 => DateMode::MonthDay,
            _ => DateMode::UnknownVariant(value),
        }
    }
}
impl convert::From<i64> for DateMode {
    fn from(value: i64) -> Self {
        DateMode::from(value as u8)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum BacklightTimeout {
    /// Backlight stays on forever.
    Infinite,
    UnknownVariant(u8),
}
impl BacklightTimeout {
    pub fn as_u8(self) -> u8 {
        match self {
            BacklightTimeout::Infinite => 0,
            BacklightTimeout::UnknownVariant(value) => value,
        }
    }
    pub fn as_i64(self) -> i64 {
        self.as_u8() as i64
    }
}
impl fmt::Display for BacklightTimeout {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            BacklightTimeout::Infinite => writeln!(f, "infinite"),
            BacklightTimeout::UnknownVariant(value) => writeln!(f, "unknown_variant_{}", *value),
        }
    }
}
impl convert::From<u8> for BacklightTimeout {
    fn from(value: u8) -> Self {
        match value {
            0 => BacklightTimeout::Infinite,
            _ => BacklightTimeout::UnknownVariant(value),
        }
    }
}
impl convert::From<i64> for BacklightTimeout {
    fn from(value: i64) -> Self {
        BacklightTimeout::from(value as u8)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum Event {
    /// Group 0.  Start / stop_all
    Timer,
    /// start / stop
    Workout,
    /// Start at beginning of workout.  Stop at end of each step.
    WorkoutStep,
    /// stop_all group 0
    PowerDown,
    /// stop_all group 0
    PowerUp,
    /// start / stop group 0
    OffCourse,
    /// Stop at end of each session.
    Session,
    /// Stop at end of each lap.
    Lap,
    /// marker
    CoursePoint,
    /// marker
    Battery,
    /// Group 1. Start at beginning of activity if VP enabled, when VP pace is changed during activity or VP enabled mid activity.  stop_disable when VP disabled.
    VirtualPartnerPace,
    /// Group 0.  Start / stop when in alert condition.
    HrHighAlert,
    /// Group 0.  Start / stop when in alert condition.
    HrLowAlert,
    /// Group 0.  Start / stop when in alert condition.
    SpeedHighAlert,
    /// Group 0.  Start / stop when in alert condition.
    SpeedLowAlert,
    /// Group 0.  Start / stop when in alert condition.
    CadHighAlert,
    /// Group 0.  Start / stop when in alert condition.
    CadLowAlert,
    /// Group 0.  Start / stop when in alert condition.
    PowerHighAlert,
    /// Group 0.  Start / stop when in alert condition.
    PowerLowAlert,
    /// marker
    RecoveryHr,
    /// marker
    BatteryLow,
    /// Group 1.  Start if enabled mid activity (not required at start of activity). Stop when duration is reached.  stop_disable if disabled.
    TimeDurationAlert,
    /// Group 1.  Start if enabled mid activity (not required at start of activity). Stop when duration is reached.  stop_disable if disabled.
    DistanceDurationAlert,
    /// Group 1.  Start if enabled mid activity (not required at start of activity). Stop when duration is reached.  stop_disable if disabled.
    CalorieDurationAlert,
    /// Group 1..  Stop at end of activity.
    Activity,
    /// marker
    FitnessEquipment,
    /// Stop at end of each length.
    Length,
    /// marker
    UserMarker,
    /// marker
    SportPoint,
    /// start/stop/marker
    Calibration,
    /// marker
    FrontGearChange,
    /// marker
    RearGearChange,
    /// marker
    RiderPositionChange,
    /// Group 0.  Start / stop when in alert condition.
    ElevHighAlert,
    /// Group 0.  Start / stop when in alert condition.
    ElevLowAlert,
    /// marker
    CommTimeout,
    UnknownVariant(u8),
}
impl Event {
    pub fn as_u8(self) -> u8 {
        match self {
            Event::Timer => 0,
            Event::Workout => 3,
            Event::WorkoutStep => 4,
            Event::PowerDown => 5,
            Event::PowerUp => 6,
            Event::OffCourse => 7,
            Event::Session => 8,
            Event::Lap => 9,
            Event::CoursePoint => 10,
            Event::Battery => 11,
            Event::VirtualPartnerPace => 12,
            Event::HrHighAlert => 13,
            Event::HrLowAlert => 14,
            Event::SpeedHighAlert => 15,
            Event::SpeedLowAlert => 16,
            Event::CadHighAlert => 17,
            Event::CadLowAlert => 18,
            Event::PowerHighAlert => 19,
            Event::PowerLowAlert => 20,
            Event::RecoveryHr => 21,
            Event::BatteryLow => 22,
            Event::TimeDurationAlert => 23,
            Event::DistanceDurationAlert => 24,
            Event::CalorieDurationAlert => 25,
            Event::Activity => 26,
            Event::FitnessEquipment => 27,
            Event::Length => 28,
            Event::UserMarker => 32,
            Event::SportPoint => 33,
            Event::Calibration => 36,
            Event::FrontGearChange => 42,
            Event::RearGearChange => 43,
            Event::RiderPositionChange => 44,
            Event::ElevHighAlert => 45,
            Event::ElevLowAlert => 46,
            Event::CommTimeout => 47,
            Event::UnknownVariant(value) => value,
        }
    }
    pub fn as_i64(self) -> i64 {
        self.as_u8() as i64
    }
}
impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Event::Timer => writeln!(f, "timer"),
            Event::Workout => writeln!(f, "workout"),
            Event::WorkoutStep => writeln!(f, "workout_step"),
            Event::PowerDown => writeln!(f, "power_down"),
            Event::PowerUp => writeln!(f, "power_up"),
            Event::OffCourse => writeln!(f, "off_course"),
            Event::Session => writeln!(f, "session"),
            Event::Lap => writeln!(f, "lap"),
            Event::CoursePoint => writeln!(f, "course_point"),
            Event::Battery => writeln!(f, "battery"),
            Event::VirtualPartnerPace => writeln!(f, "virtual_partner_pace"),
            Event::HrHighAlert => writeln!(f, "hr_high_alert"),
            Event::HrLowAlert => writeln!(f, "hr_low_alert"),
            Event::SpeedHighAlert => writeln!(f, "speed_high_alert"),
            Event::SpeedLowAlert => writeln!(f, "speed_low_alert"),
            Event::CadHighAlert => writeln!(f, "cad_high_alert"),
            Event::CadLowAlert => writeln!(f, "cad_low_alert"),
            Event::PowerHighAlert => writeln!(f, "power_high_alert"),
            Event::PowerLowAlert => writeln!(f, "power_low_alert"),
            Event::RecoveryHr => writeln!(f, "recovery_hr"),
            Event::BatteryLow => writeln!(f, "battery_low"),
            Event::TimeDurationAlert => writeln!(f, "time_duration_alert"),
            Event::DistanceDurationAlert => writeln!(f, "distance_duration_alert"),
            Event::CalorieDurationAlert => writeln!(f, "calorie_duration_alert"),
            Event::Activity => writeln!(f, "activity"),
            Event::FitnessEquipment => writeln!(f, "fitness_equipment"),
            Event::Length => writeln!(f, "length"),
            Event::UserMarker => writeln!(f, "user_marker"),
            Event::SportPoint => writeln!(f, "sport_point"),
            Event::Calibration => writeln!(f, "calibration"),
            Event::FrontGearChange => writeln!(f, "front_gear_change"),
            Event::RearGearChange => writeln!(f, "rear_gear_change"),
            Event::RiderPositionChange => writeln!(f, "rider_position_change"),
            Event::ElevHighAlert => writeln!(f, "elev_high_alert"),
            Event::ElevLowAlert => writeln!(f, "elev_low_alert"),
            Event::CommTimeout => writeln!(f, "comm_timeout"),
            Event::UnknownVariant(value) => writeln!(f, "unknown_variant_{}", *value),
        }
    }
}
impl convert::From<u8> for Event {
    fn from(value: u8) -> Self {
        match value {
            0 => Event::Timer,
            3 => Event::Workout,
            4 => Event::WorkoutStep,
            5 => Event::PowerDown,
            6 => Event::PowerUp,
            7 => Event::OffCourse,
            8 => Event::Session,
            9 => Event::Lap,
            10 => Event::CoursePoint,
            11 => Event::Battery,
            12 => Event::VirtualPartnerPace,
            13 => Event::HrHighAlert,
            14 => Event::HrLowAlert,
            15 => Event::SpeedHighAlert,
            16 => Event::SpeedLowAlert,
            17 => Event::CadHighAlert,
            18 => Event::CadLowAlert,
            19 => Event::PowerHighAlert,
            20 => Event::PowerLowAlert,
            21 => Event::RecoveryHr,
            22 => Event::BatteryLow,
            23 => Event::TimeDurationAlert,
            24 => Event::DistanceDurationAlert,
            25 => Event::CalorieDurationAlert,
            26 => Event::Activity,
            27 => Event::FitnessEquipment,
            28 => Event::Length,
            32 => Event::UserMarker,
            33 => Event::SportPoint,
            36 => Event::Calibration,
            42 => Event::FrontGearChange,
            43 => Event::RearGearChange,
            44 => Event::RiderPositionChange,
            45 => Event::ElevHighAlert,
            46 => Event::ElevLowAlert,
            47 => Event::CommTimeout,
            _ => Event::UnknownVariant(value),
        }
    }
}
impl convert::From<i64> for Event {
    fn from(value: i64) -> Self {
        Event::from(value as u8)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum EventType {
    Start,
    Stop,
    ConsecutiveDepreciated,
    Marker,
    StopAll,
    BeginDepreciated,
    EndDepreciated,
    EndAllDepreciated,
    StopDisable,
    StopDisableAll,
    UnknownVariant(u8),
}
impl EventType {
    pub fn as_u8(self) -> u8 {
        match self {
            EventType::Start => 0,
            EventType::Stop => 1,
            EventType::ConsecutiveDepreciated => 2,
            EventType::Marker => 3,
            EventType::StopAll => 4,
            EventType::BeginDepreciated => 5,
            EventType::EndDepreciated => 6,
            EventType::EndAllDepreciated => 7,
            EventType::StopDisable => 8,
            EventType::StopDisableAll => 9,
            EventType::UnknownVariant(value) => value,
        }
    }
    pub fn as_i64(self) -> i64 {
        self.as_u8() as i64
    }
}
impl fmt::Display for EventType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            EventType::Start => writeln!(f, "start"),
            EventType::Stop => writeln!(f, "stop"),
            EventType::ConsecutiveDepreciated => writeln!(f, "consecutive_depreciated"),
            EventType::Marker => writeln!(f, "marker"),
            EventType::StopAll => writeln!(f, "stop_all"),
            EventType::BeginDepreciated => writeln!(f, "begin_depreciated"),
            EventType::EndDepreciated => writeln!(f, "end_depreciated"),
            EventType::EndAllDepreciated => writeln!(f, "end_all_depreciated"),
            EventType::StopDisable => writeln!(f, "stop_disable"),
            EventType::StopDisableAll => writeln!(f, "stop_disable_all"),
            EventType::UnknownVariant(value) => writeln!(f, "unknown_variant_{}", *value),
        }
    }
}
impl convert::From<u8> for EventType {
    fn from(value: u8) -> Self {
        match value {
            0 => EventType::Start,
            1 => EventType::Stop,
            2 => EventType::ConsecutiveDepreciated,
            3 => EventType::Marker,
            4 => EventType::StopAll,
            5 => EventType::BeginDepreciated,
            6 => EventType::EndDepreciated,
            7 => EventType::EndAllDepreciated,
            8 => EventType::StopDisable,
            9 => EventType::StopDisableAll,
            _ => EventType::UnknownVariant(value),
        }
    }
}
impl convert::From<i64> for EventType {
    fn from(value: i64) -> Self {
        EventType::from(value as u8)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum TimerTrigger {
    Manual,
    Auto,
    FitnessEquipment,
    UnknownVariant(u8),
}
impl TimerTrigger {
    pub fn as_u8(self) -> u8 {
        match self {
            TimerTrigger::Manual => 0,
            TimerTrigger::Auto => 1,
            TimerTrigger::FitnessEquipment => 2,
            TimerTrigger::UnknownVariant(value) => value,
        }
    }
    pub fn as_i64(self) -> i64 {
        self.as_u8() as i64
    }
}
impl fmt::Display for TimerTrigger {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            TimerTrigger::Manual => writeln!(f, "manual"),
            TimerTrigger::Auto => writeln!(f, "auto"),
            TimerTrigger::FitnessEquipment => writeln!(f, "fitness_equipment"),
            TimerTrigger::UnknownVariant(value) => writeln!(f, "unknown_variant_{}", *value),
        }
    }
}
impl convert::From<u8> for TimerTrigger {
    fn from(value: u8) -> Self {
        match value {
            0 => TimerTrigger::Manual,
            1 => TimerTrigger::Auto,
            2 => TimerTrigger::FitnessEquipment,
            _ => TimerTrigger::UnknownVariant(value),
        }
    }
}
impl convert::From<i64> for TimerTrigger {
    fn from(value: i64) -> Self {
        TimerTrigger::from(value as u8)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum FitnessEquipmentState {
    Ready,
    InUse,
    Paused,
    /// lost connection to fitness equipment
    Unknown,
    UnknownVariant(u8),
}
impl FitnessEquipmentState {
    pub fn as_u8(self) -> u8 {
        match self {
            FitnessEquipmentState::Ready => 0,
            FitnessEquipmentState::InUse => 1,
            FitnessEquipmentState::Paused => 2,
            FitnessEquipmentState::Unknown => 3,
            FitnessEquipmentState::UnknownVariant(value) => value,
        }
    }
    pub fn as_i64(self) -> i64 {
        self.as_u8() as i64
    }
}
impl fmt::Display for FitnessEquipmentState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            FitnessEquipmentState::Ready => writeln!(f, "ready"),
            FitnessEquipmentState::InUse => writeln!(f, "in_use"),
            FitnessEquipmentState::Paused => writeln!(f, "paused"),
            FitnessEquipmentState::Unknown => writeln!(f, "unknown"),
            FitnessEquipmentState::UnknownVariant(value) => {
                writeln!(f, "unknown_variant_{}", *value)
            }
        }
    }
}
impl convert::From<u8> for FitnessEquipmentState {
    fn from(value: u8) -> Self {
        match value {
            0 => FitnessEquipmentState::Ready,
            1 => FitnessEquipmentState::InUse,
            2 => FitnessEquipmentState::Paused,
            3 => FitnessEquipmentState::Unknown,
            _ => FitnessEquipmentState::UnknownVariant(value),
        }
    }
}
impl convert::From<i64> for FitnessEquipmentState {
    fn from(value: i64) -> Self {
        FitnessEquipmentState::from(value as u8)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum Tone {
    Off,
    Tone,
    Vibrate,
    ToneAndVibrate,
    UnknownVariant(u8),
}
impl Tone {
    pub fn as_u8(self) -> u8 {
        match self {
            Tone::Off => 0,
            Tone::Tone => 1,
            Tone::Vibrate => 2,
            Tone::ToneAndVibrate => 3,
            Tone::UnknownVariant(value) => value,
        }
    }
    pub fn as_i64(self) -> i64 {
        self.as_u8() as i64
    }
}
impl fmt::Display for Tone {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Tone::Off => writeln!(f, "off"),
            Tone::Tone => writeln!(f, "tone"),
            Tone::Vibrate => writeln!(f, "vibrate"),
            Tone::ToneAndVibrate => writeln!(f, "tone_and_vibrate"),
            Tone::UnknownVariant(value) => writeln!(f, "unknown_variant_{}", *value),
        }
    }
}
impl convert::From<u8> for Tone {
    fn from(value: u8) -> Self {
        match value {
            0 => Tone::Off,
            1 => Tone::Tone,
            2 => Tone::Vibrate,
            3 => Tone::ToneAndVibrate,
            _ => Tone::UnknownVariant(value),
        }
    }
}
impl convert::From<i64> for Tone {
    fn from(value: i64) -> Self {
        Tone::from(value as u8)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum Autoscroll {
    None,
    Slow,
    Medium,
    Fast,
    UnknownVariant(u8),
}
impl Autoscroll {
    pub fn as_u8(self) -> u8 {
        match self {
            Autoscroll::None => 0,
            Autoscroll::Slow => 1,
            Autoscroll::Medium => 2,
            Autoscroll::Fast => 3,
            Autoscroll::UnknownVariant(value) => value,
        }
    }
    pub fn as_i64(self) -> i64 {
        self.as_u8() as i64
    }
}
impl fmt::Display for Autoscroll {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Autoscroll::None => writeln!(f, "none"),
            Autoscroll::Slow => writeln!(f, "slow"),
            Autoscroll::Medium => writeln!(f, "medium"),
            Autoscroll::Fast => writeln!(f, "fast"),
            Autoscroll::UnknownVariant(value) => writeln!(f, "unknown_variant_{}", *value),
        }
    }
}
impl convert::From<u8> for Autoscroll {
    fn from(value: u8) -> Self {
        match value {
            0 => Autoscroll::None,
            1 => Autoscroll::Slow,
            2 => Autoscroll::Medium,
            3 => Autoscroll::Fast,
            _ => Autoscroll::UnknownVariant(value),
        }
    }
}
impl convert::From<i64> for Autoscroll {
    fn from(value: i64) -> Self {
        Autoscroll::from(value as u8)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum ActivityClass {
    LevelMax,
    /// 0 to 100
    Level,
    Athlete,
    UnknownVariant(u8),
}
impl ActivityClass {
    pub fn as_u8(self) -> u8 {
        match self {
            ActivityClass::LevelMax => 100,
            ActivityClass::Level => 127,
            ActivityClass::Athlete => 128,
            ActivityClass::UnknownVariant(value) => value,
        }
    }
    pub fn as_i64(self) -> i64 {
        self.as_u8() as i64
    }
}
impl fmt::Display for ActivityClass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            ActivityClass::LevelMax => writeln!(f, "level_max"),
            ActivityClass::Level => writeln!(f, "level"),
            ActivityClass::Athlete => writeln!(f, "athlete"),
            ActivityClass::UnknownVariant(value) => writeln!(f, "unknown_variant_{}", *value),
        }
    }
}
impl convert::From<u8> for ActivityClass {
    fn from(value: u8) -> Self {
        match value {
            100 => ActivityClass::LevelMax,
            127 => ActivityClass::Level,
            128 => ActivityClass::Athlete,
            _ => ActivityClass::UnknownVariant(value),
        }
    }
}
impl convert::From<i64> for ActivityClass {
    fn from(value: i64) -> Self {
        ActivityClass::from(value as u8)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum HrZoneCalc {
    Custom,
    PercentMaxHr,
    PercentHrr,
    UnknownVariant(u8),
}
impl HrZoneCalc {
    pub fn as_u8(self) -> u8 {
        match self {
            HrZoneCalc::Custom => 0,
            HrZoneCalc::PercentMaxHr => 1,
            HrZoneCalc::PercentHrr => 2,
            HrZoneCalc::UnknownVariant(value) => value,
        }
    }
    pub fn as_i64(self) -> i64 {
        self.as_u8() as i64
    }
}
impl fmt::Display for HrZoneCalc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            HrZoneCalc::Custom => writeln!(f, "custom"),
            HrZoneCalc::PercentMaxHr => writeln!(f, "percent_max_hr"),
            HrZoneCalc::PercentHrr => writeln!(f, "percent_hrr"),
            HrZoneCalc::UnknownVariant(value) => writeln!(f, "unknown_variant_{}", *value),
        }
    }
}
impl convert::From<u8> for HrZoneCalc {
    fn from(value: u8) -> Self {
        match value {
            0 => HrZoneCalc::Custom,
            1 => HrZoneCalc::PercentMaxHr,
            2 => HrZoneCalc::PercentHrr,
            _ => HrZoneCalc::UnknownVariant(value),
        }
    }
}
impl convert::From<i64> for HrZoneCalc {
    fn from(value: i64) -> Self {
        HrZoneCalc::from(value as u8)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum PwrZoneCalc {
    Custom,
    PercentFtp,
    UnknownVariant(u8),
}
impl PwrZoneCalc {
    pub fn as_u8(self) -> u8 {
        match self {
            PwrZoneCalc::Custom => 0,
            PwrZoneCalc::PercentFtp => 1,
            PwrZoneCalc::UnknownVariant(value) => value,
        }
    }
    pub fn as_i64(self) -> i64 {
        self.as_u8() as i64
    }
}
impl fmt::Display for PwrZoneCalc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            PwrZoneCalc::Custom => writeln!(f, "custom"),
            PwrZoneCalc::PercentFtp => writeln!(f, "percent_ftp"),
            PwrZoneCalc::UnknownVariant(value) => writeln!(f, "unknown_variant_{}", *value),
        }
    }
}
impl convert::From<u8> for PwrZoneCalc {
    fn from(value: u8) -> Self {
        match value {
            0 => PwrZoneCalc::Custom,
            1 => PwrZoneCalc::PercentFtp,
            _ => PwrZoneCalc::UnknownVariant(value),
        }
    }
}
impl convert::From<i64> for PwrZoneCalc {
    fn from(value: i64) -> Self {
        PwrZoneCalc::from(value as u8)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum WktStepDuration {
    Time,
    Distance,
    HrLessThan,
    HrGreaterThan,
    Calories,
    Open,
    RepeatUntilStepsCmplt,
    RepeatUntilTime,
    RepeatUntilDistance,
    RepeatUntilCalories,
    RepeatUntilHrLessThan,
    RepeatUntilHrGreaterThan,
    RepeatUntilPowerLessThan,
    RepeatUntilPowerGreaterThan,
    PowerLessThan,
    PowerGreaterThan,
    TrainingPeaksTss,
    RepeatUntilPowerLastLapLessThan,
    RepeatUntilMaxPowerLastLapLessThan,
    Power3sLessThan,
    Power10sLessThan,
    Power30sLessThan,
    Power3sGreaterThan,
    Power10sGreaterThan,
    Power30sGreaterThan,
    PowerLapLessThan,
    PowerLapGreaterThan,
    RepeatUntilTrainingPeaksTss,
    RepetitionTime,
    Reps,
    UnknownVariant(u8),
}
impl WktStepDuration {
    pub fn as_u8(self) -> u8 {
        match self {
            WktStepDuration::Time => 0,
            WktStepDuration::Distance => 1,
            WktStepDuration::HrLessThan => 2,
            WktStepDuration::HrGreaterThan => 3,
            WktStepDuration::Calories => 4,
            WktStepDuration::Open => 5,
            WktStepDuration::RepeatUntilStepsCmplt => 6,
            WktStepDuration::RepeatUntilTime => 7,
            WktStepDuration::RepeatUntilDistance => 8,
            WktStepDuration::RepeatUntilCalories => 9,
            WktStepDuration::RepeatUntilHrLessThan => 10,
            WktStepDuration::RepeatUntilHrGreaterThan => 11,
            WktStepDuration::RepeatUntilPowerLessThan => 12,
            WktStepDuration::RepeatUntilPowerGreaterThan => 13,
            WktStepDuration::PowerLessThan => 14,
            WktStepDuration::PowerGreaterThan => 15,
            WktStepDuration::TrainingPeaksTss => 16,
            WktStepDuration::RepeatUntilPowerLastLapLessThan => 17,
            WktStepDuration::RepeatUntilMaxPowerLastLapLessThan => 18,
            WktStepDuration::Power3sLessThan => 19,
            WktStepDuration::Power10sLessThan => 20,
            WktStepDuration::Power30sLessThan => 21,
            WktStepDuration::Power3sGreaterThan => 22,
            WktStepDuration::Power10sGreaterThan => 23,
            WktStepDuration::Power30sGreaterThan => 24,
            WktStepDuration::PowerLapLessThan => 25,
            WktStepDuration::PowerLapGreaterThan => 26,
            WktStepDuration::RepeatUntilTrainingPeaksTss => 27,
            WktStepDuration::RepetitionTime => 28,
            WktStepDuration::Reps => 29,
            WktStepDuration::UnknownVariant(value) => value,
        }
    }
    pub fn as_i64(self) -> i64 {
        self.as_u8() as i64
    }
}
impl fmt::Display for WktStepDuration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            WktStepDuration::Time => writeln!(f, "time"),
            WktStepDuration::Distance => writeln!(f, "distance"),
            WktStepDuration::HrLessThan => writeln!(f, "hr_less_than"),
            WktStepDuration::HrGreaterThan => writeln!(f, "hr_greater_than"),
            WktStepDuration::Calories => writeln!(f, "calories"),
            WktStepDuration::Open => writeln!(f, "open"),
            WktStepDuration::RepeatUntilStepsCmplt => writeln!(f, "repeat_until_steps_cmplt"),
            WktStepDuration::RepeatUntilTime => writeln!(f, "repeat_until_time"),
            WktStepDuration::RepeatUntilDistance => writeln!(f, "repeat_until_distance"),
            WktStepDuration::RepeatUntilCalories => writeln!(f, "repeat_until_calories"),
            WktStepDuration::RepeatUntilHrLessThan => writeln!(f, "repeat_until_hr_less_than"),
            WktStepDuration::RepeatUntilHrGreaterThan => {
                writeln!(f, "repeat_until_hr_greater_than")
            }
            WktStepDuration::RepeatUntilPowerLessThan => {
                writeln!(f, "repeat_until_power_less_than")
            }
            WktStepDuration::RepeatUntilPowerGreaterThan => {
                writeln!(f, "repeat_until_power_greater_than")
            }
            WktStepDuration::PowerLessThan => writeln!(f, "power_less_than"),
            WktStepDuration::PowerGreaterThan => writeln!(f, "power_greater_than"),
            WktStepDuration::TrainingPeaksTss => writeln!(f, "training_peaks_tss"),
            WktStepDuration::RepeatUntilPowerLastLapLessThan => {
                writeln!(f, "repeat_until_power_last_lap_less_than")
            }
            WktStepDuration::RepeatUntilMaxPowerLastLapLessThan => {
                writeln!(f, "repeat_until_max_power_last_lap_less_than")
            }
            WktStepDuration::Power3sLessThan => writeln!(f, "power_3s_less_than"),
            WktStepDuration::Power10sLessThan => writeln!(f, "power_10s_less_than"),
            WktStepDuration::Power30sLessThan => writeln!(f, "power_30s_less_than"),
            WktStepDuration::Power3sGreaterThan => writeln!(f, "power_3s_greater_than"),
            WktStepDuration::Power10sGreaterThan => writeln!(f, "power_10s_greater_than"),
            WktStepDuration::Power30sGreaterThan => writeln!(f, "power_30s_greater_than"),
            WktStepDuration::PowerLapLessThan => writeln!(f, "power_lap_less_than"),
            WktStepDuration::PowerLapGreaterThan => writeln!(f, "power_lap_greater_than"),
            WktStepDuration::RepeatUntilTrainingPeaksTss => {
                writeln!(f, "repeat_until_training_peaks_tss")
            }
            WktStepDuration::RepetitionTime => writeln!(f, "repetition_time"),
            WktStepDuration::Reps => writeln!(f, "reps"),
            WktStepDuration::UnknownVariant(value) => writeln!(f, "unknown_variant_{}", *value),
        }
    }
}
impl convert::From<u8> for WktStepDuration {
    fn from(value: u8) -> Self {
        match value {
            0 => WktStepDuration::Time,
            1 => WktStepDuration::Distance,
            2 => WktStepDuration::HrLessThan,
            3 => WktStepDuration::HrGreaterThan,
            4 => WktStepDuration::Calories,
            5 => WktStepDuration::Open,
            6 => WktStepDuration::RepeatUntilStepsCmplt,
            7 => WktStepDuration::RepeatUntilTime,
            8 => WktStepDuration::RepeatUntilDistance,
            9 => WktStepDuration::RepeatUntilCalories,
            10 => WktStepDuration::RepeatUntilHrLessThan,
            11 => WktStepDuration::RepeatUntilHrGreaterThan,
            12 => WktStepDuration::RepeatUntilPowerLessThan,
            13 => WktStepDuration::RepeatUntilPowerGreaterThan,
            14 => WktStepDuration::PowerLessThan,
            15 => WktStepDuration::PowerGreaterThan,
            16 => WktStepDuration::TrainingPeaksTss,
            17 => WktStepDuration::RepeatUntilPowerLastLapLessThan,
            18 => WktStepDuration::RepeatUntilMaxPowerLastLapLessThan,
            19 => WktStepDuration::Power3sLessThan,
            20 => WktStepDuration::Power10sLessThan,
            21 => WktStepDuration::Power30sLessThan,
            22 => WktStepDuration::Power3sGreaterThan,
            23 => WktStepDuration::Power10sGreaterThan,
            24 => WktStepDuration::Power30sGreaterThan,
            25 => WktStepDuration::PowerLapLessThan,
            26 => WktStepDuration::PowerLapGreaterThan,
            27 => WktStepDuration::RepeatUntilTrainingPeaksTss,
            28 => WktStepDuration::RepetitionTime,
            29 => WktStepDuration::Reps,
            _ => WktStepDuration::UnknownVariant(value),
        }
    }
}
impl convert::From<i64> for WktStepDuration {
    fn from(value: i64) -> Self {
        WktStepDuration::from(value as u8)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum WktStepTarget {
    Speed,
    HeartRate,
    Open,
    Cadence,
    Power,
    Grade,
    Resistance,
    Power3s,
    Power10s,
    Power30s,
    PowerLap,
    SwimStroke,
    SpeedLap,
    HeartRateLap,
    UnknownVariant(u8),
}
impl WktStepTarget {
    pub fn as_u8(self) -> u8 {
        match self {
            WktStepTarget::Speed => 0,
            WktStepTarget::HeartRate => 1,
            WktStepTarget::Open => 2,
            WktStepTarget::Cadence => 3,
            WktStepTarget::Power => 4,
            WktStepTarget::Grade => 5,
            WktStepTarget::Resistance => 6,
            WktStepTarget::Power3s => 7,
            WktStepTarget::Power10s => 8,
            WktStepTarget::Power30s => 9,
            WktStepTarget::PowerLap => 10,
            WktStepTarget::SwimStroke => 11,
            WktStepTarget::SpeedLap => 12,
            WktStepTarget::HeartRateLap => 13,
            WktStepTarget::UnknownVariant(value) => value,
        }
    }
    pub fn as_i64(self) -> i64 {
        self.as_u8() as i64
    }
}
impl fmt::Display for WktStepTarget {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            WktStepTarget::Speed => writeln!(f, "speed"),
            WktStepTarget::HeartRate => writeln!(f, "heart_rate"),
            WktStepTarget::Open => writeln!(f, "open"),
            WktStepTarget::Cadence => writeln!(f, "cadence"),
            WktStepTarget::Power => writeln!(f, "power"),
            WktStepTarget::Grade => writeln!(f, "grade"),
            WktStepTarget::Resistance => writeln!(f, "resistance"),
            WktStepTarget::Power3s => writeln!(f, "power_3s"),
            WktStepTarget::Power10s => writeln!(f, "power_10s"),
            WktStepTarget::Power30s => writeln!(f, "power_30s"),
            WktStepTarget::PowerLap => writeln!(f, "power_lap"),
            WktStepTarget::SwimStroke => writeln!(f, "swim_stroke"),
            WktStepTarget::SpeedLap => writeln!(f, "speed_lap"),
            WktStepTarget::HeartRateLap => writeln!(f, "heart_rate_lap"),
            WktStepTarget::UnknownVariant(value) => writeln!(f, "unknown_variant_{}", *value),
        }
    }
}
impl convert::From<u8> for WktStepTarget {
    fn from(value: u8) -> Self {
        match value {
            0 => WktStepTarget::Speed,
            1 => WktStepTarget::HeartRate,
            2 => WktStepTarget::Open,
            3 => WktStepTarget::Cadence,
            4 => WktStepTarget::Power,
            5 => WktStepTarget::Grade,
            6 => WktStepTarget::Resistance,
            7 => WktStepTarget::Power3s,
            8 => WktStepTarget::Power10s,
            9 => WktStepTarget::Power30s,
            10 => WktStepTarget::PowerLap,
            11 => WktStepTarget::SwimStroke,
            12 => WktStepTarget::SpeedLap,
            13 => WktStepTarget::HeartRateLap,
            _ => WktStepTarget::UnknownVariant(value),
        }
    }
}
impl convert::From<i64> for WktStepTarget {
    fn from(value: i64) -> Self {
        WktStepTarget::from(value as u8)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum Goal {
    Time,
    Distance,
    Calories,
    Frequency,
    Steps,
    Ascent,
    ActiveMinutes,
    UnknownVariant(u8),
}
impl Goal {
    pub fn as_u8(self) -> u8 {
        match self {
            Goal::Time => 0,
            Goal::Distance => 1,
            Goal::Calories => 2,
            Goal::Frequency => 3,
            Goal::Steps => 4,
            Goal::Ascent => 5,
            Goal::ActiveMinutes => 6,
            Goal::UnknownVariant(value) => value,
        }
    }
    pub fn as_i64(self) -> i64 {
        self.as_u8() as i64
    }
}
impl fmt::Display for Goal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Goal::Time => writeln!(f, "time"),
            Goal::Distance => writeln!(f, "distance"),
            Goal::Calories => writeln!(f, "calories"),
            Goal::Frequency => writeln!(f, "frequency"),
            Goal::Steps => writeln!(f, "steps"),
            Goal::Ascent => writeln!(f, "ascent"),
            Goal::ActiveMinutes => writeln!(f, "active_minutes"),
            Goal::UnknownVariant(value) => writeln!(f, "unknown_variant_{}", *value),
        }
    }
}
impl convert::From<u8> for Goal {
    fn from(value: u8) -> Self {
        match value {
            0 => Goal::Time,
            1 => Goal::Distance,
            2 => Goal::Calories,
            3 => Goal::Frequency,
            4 => Goal::Steps,
            5 => Goal::Ascent,
            6 => Goal::ActiveMinutes,
            _ => Goal::UnknownVariant(value),
        }
    }
}
impl convert::From<i64> for Goal {
    fn from(value: i64) -> Self {
        Goal::from(value as u8)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum GoalRecurrence {
    Off,
    Daily,
    Weekly,
    Monthly,
    Yearly,
    Custom,
    UnknownVariant(u8),
}
impl GoalRecurrence {
    pub fn as_u8(self) -> u8 {
        match self {
            GoalRecurrence::Off => 0,
            GoalRecurrence::Daily => 1,
            GoalRecurrence::Weekly => 2,
            GoalRecurrence::Monthly => 3,
            GoalRecurrence::Yearly => 4,
            GoalRecurrence::Custom => 5,
            GoalRecurrence::UnknownVariant(value) => value,
        }
    }
    pub fn as_i64(self) -> i64 {
        self.as_u8() as i64
    }
}
impl fmt::Display for GoalRecurrence {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            GoalRecurrence::Off => writeln!(f, "off"),
            GoalRecurrence::Daily => writeln!(f, "daily"),
            GoalRecurrence::Weekly => writeln!(f, "weekly"),
            GoalRecurrence::Monthly => writeln!(f, "monthly"),
            GoalRecurrence::Yearly => writeln!(f, "yearly"),
            GoalRecurrence::Custom => writeln!(f, "custom"),
            GoalRecurrence::UnknownVariant(value) => writeln!(f, "unknown_variant_{}", *value),
        }
    }
}
impl convert::From<u8> for GoalRecurrence {
    fn from(value: u8) -> Self {
        match value {
            0 => GoalRecurrence::Off,
            1 => GoalRecurrence::Daily,
            2 => GoalRecurrence::Weekly,
            3 => GoalRecurrence::Monthly,
            4 => GoalRecurrence::Yearly,
            5 => GoalRecurrence::Custom,
            _ => GoalRecurrence::UnknownVariant(value),
        }
    }
}
impl convert::From<i64> for GoalRecurrence {
    fn from(value: i64) -> Self {
        GoalRecurrence::from(value as u8)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum GoalSource {
    /// Device generated
    Auto,
    /// Social network sourced goal
    Community,
    /// Manually generated
    User,
    UnknownVariant(u8),
}
impl GoalSource {
    pub fn as_u8(self) -> u8 {
        match self {
            GoalSource::Auto => 0,
            GoalSource::Community => 1,
            GoalSource::User => 2,
            GoalSource::UnknownVariant(value) => value,
        }
    }
    pub fn as_i64(self) -> i64 {
        self.as_u8() as i64
    }
}
impl fmt::Display for GoalSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            GoalSource::Auto => writeln!(f, "auto"),
            GoalSource::Community => writeln!(f, "community"),
            GoalSource::User => writeln!(f, "user"),
            GoalSource::UnknownVariant(value) => writeln!(f, "unknown_variant_{}", *value),
        }
    }
}
impl convert::From<u8> for GoalSource {
    fn from(value: u8) -> Self {
        match value {
            0 => GoalSource::Auto,
            1 => GoalSource::Community,
            2 => GoalSource::User,
            _ => GoalSource::UnknownVariant(value),
        }
    }
}
impl convert::From<i64> for GoalSource {
    fn from(value: i64) -> Self {
        GoalSource::from(value as u8)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum Schedule {
    Workout,
    Course,
    UnknownVariant(u8),
}
impl Schedule {
    pub fn as_u8(self) -> u8 {
        match self {
            Schedule::Workout => 0,
            Schedule::Course => 1,
            Schedule::UnknownVariant(value) => value,
        }
    }
    pub fn as_i64(self) -> i64 {
        self.as_u8() as i64
    }
}
impl fmt::Display for Schedule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Schedule::Workout => writeln!(f, "workout"),
            Schedule::Course => writeln!(f, "course"),
            Schedule::UnknownVariant(value) => writeln!(f, "unknown_variant_{}", *value),
        }
    }
}
impl convert::From<u8> for Schedule {
    fn from(value: u8) -> Self {
        match value {
            0 => Schedule::Workout,
            1 => Schedule::Course,
            _ => Schedule::UnknownVariant(value),
        }
    }
}
impl convert::From<i64> for Schedule {
    fn from(value: i64) -> Self {
        Schedule::from(value as u8)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum CoursePoint {
    Generic,
    Summit,
    Valley,
    Water,
    Food,
    Danger,
    Left,
    Right,
    Straight,
    FirstAid,
    FourthCategory,
    ThirdCategory,
    SecondCategory,
    FirstCategory,
    HorsCategory,
    Sprint,
    LeftFork,
    RightFork,
    MiddleFork,
    SlightLeft,
    SharpLeft,
    SlightRight,
    SharpRight,
    UTurn,
    SegmentStart,
    SegmentEnd,
    UnknownVariant(u8),
}
impl CoursePoint {
    pub fn as_u8(self) -> u8 {
        match self {
            CoursePoint::Generic => 0,
            CoursePoint::Summit => 1,
            CoursePoint::Valley => 2,
            CoursePoint::Water => 3,
            CoursePoint::Food => 4,
            CoursePoint::Danger => 5,
            CoursePoint::Left => 6,
            CoursePoint::Right => 7,
            CoursePoint::Straight => 8,
            CoursePoint::FirstAid => 9,
            CoursePoint::FourthCategory => 10,
            CoursePoint::ThirdCategory => 11,
            CoursePoint::SecondCategory => 12,
            CoursePoint::FirstCategory => 13,
            CoursePoint::HorsCategory => 14,
            CoursePoint::Sprint => 15,
            CoursePoint::LeftFork => 16,
            CoursePoint::RightFork => 17,
            CoursePoint::MiddleFork => 18,
            CoursePoint::SlightLeft => 19,
            CoursePoint::SharpLeft => 20,
            CoursePoint::SlightRight => 21,
            CoursePoint::SharpRight => 22,
            CoursePoint::UTurn => 23,
            CoursePoint::SegmentStart => 24,
            CoursePoint::SegmentEnd => 25,
            CoursePoint::UnknownVariant(value) => value,
        }
    }
    pub fn as_i64(self) -> i64 {
        self.as_u8() as i64
    }
}
impl fmt::Display for CoursePoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            CoursePoint::Generic => writeln!(f, "generic"),
            CoursePoint::Summit => writeln!(f, "summit"),
            CoursePoint::Valley => writeln!(f, "valley"),
            CoursePoint::Water => writeln!(f, "water"),
            CoursePoint::Food => writeln!(f, "food"),
            CoursePoint::Danger => writeln!(f, "danger"),
            CoursePoint::Left => writeln!(f, "left"),
            CoursePoint::Right => writeln!(f, "right"),
            CoursePoint::Straight => writeln!(f, "straight"),
            CoursePoint::FirstAid => writeln!(f, "first_aid"),
            CoursePoint::FourthCategory => writeln!(f, "fourth_category"),
            CoursePoint::ThirdCategory => writeln!(f, "third_category"),
            CoursePoint::SecondCategory => writeln!(f, "second_category"),
            CoursePoint::FirstCategory => writeln!(f, "first_category"),
            CoursePoint::HorsCategory => writeln!(f, "hors_category"),
            CoursePoint::Sprint => writeln!(f, "sprint"),
            CoursePoint::LeftFork => writeln!(f, "left_fork"),
            CoursePoint::RightFork => writeln!(f, "right_fork"),
            CoursePoint::MiddleFork => writeln!(f, "middle_fork"),
            CoursePoint::SlightLeft => writeln!(f, "slight_left"),
            CoursePoint::SharpLeft => writeln!(f, "sharp_left"),
            CoursePoint::SlightRight => writeln!(f, "slight_right"),
            CoursePoint::SharpRight => writeln!(f, "sharp_right"),
            CoursePoint::UTurn => writeln!(f, "u_turn"),
            CoursePoint::SegmentStart => writeln!(f, "segment_start"),
            CoursePoint::SegmentEnd => writeln!(f, "segment_end"),
            CoursePoint::UnknownVariant(value) => writeln!(f, "unknown_variant_{}", *value),
        }
    }
}
impl convert::From<u8> for CoursePoint {
    fn from(value: u8) -> Self {
        match value {
            0 => CoursePoint::Generic,
            1 => CoursePoint::Summit,
            2 => CoursePoint::Valley,
            3 => CoursePoint::Water,
            4 => CoursePoint::Food,
            5 => CoursePoint::Danger,
            6 => CoursePoint::Left,
            7 => CoursePoint::Right,
            8 => CoursePoint::Straight,
            9 => CoursePoint::FirstAid,
            10 => CoursePoint::FourthCategory,
            11 => CoursePoint::ThirdCategory,
            12 => CoursePoint::SecondCategory,
            13 => CoursePoint::FirstCategory,
            14 => CoursePoint::HorsCategory,
            15 => CoursePoint::Sprint,
            16 => CoursePoint::LeftFork,
            17 => CoursePoint::RightFork,
            18 => CoursePoint::MiddleFork,
            19 => CoursePoint::SlightLeft,
            20 => CoursePoint::SharpLeft,
            21 => CoursePoint::SlightRight,
            22 => CoursePoint::SharpRight,
            23 => CoursePoint::UTurn,
            24 => CoursePoint::SegmentStart,
            25 => CoursePoint::SegmentEnd,
            _ => CoursePoint::UnknownVariant(value),
        }
    }
}
impl convert::From<i64> for CoursePoint {
    fn from(value: i64) -> Self {
        CoursePoint::from(value as u8)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum Manufacturer {
    Garmin,
    /// Do not use.  Used by FR405 for ANTFS man id.
    GarminFr405Antfs,
    Zephyr,
    Dayton,
    Idt,
    Srm,
    Quarq,
    Ibike,
    Saris,
    SparkHk,
    Tanita,
    Echowell,
    DynastreamOem,
    Nautilus,
    Dynastream,
    Timex,
    Metrigear,
    Xelic,
    Beurer,
    Cardiosport,
    AAndD,
    Hmm,
    Suunto,
    ThitaElektronik,
    Gpulse,
    CleanMobile,
    PedalBrain,
    Peaksware,
    Saxonar,
    LemondFitness,
    Dexcom,
    WahooFitness,
    OctaneFitness,
    Archinoetics,
    TheHurtBox,
    CitizenSystems,
    Magellan,
    Osynce,
    Holux,
    Concept2,
    OneGiantLeap,
    AceSensor,
    BrimBrothers,
    Xplova,
    PerceptionDigital,
    Bf1systems,
    Pioneer,
    Spantec,
    Metalogics,
    Name4iiiis,
    SeikoEpson,
    SeikoEpsonOem,
    IforPowell,
    MaxwellGuider,
    StarTrac,
    Breakaway,
    AlatechTechnologyLtd,
    MioTechnologyEurope,
    Rotor,
    Geonaute,
    IdBike,
    Specialized,
    Wtek,
    PhysicalEnterprises,
    NorthPoleEngineering,
    Bkool,
    Cateye,
    StagesCycling,
    Sigmasport,
    Tomtom,
    Peripedal,
    Wattbike,
    Moxy,
    Ciclosport,
    Powerbahn,
    AcornProjectsAps,
    Lifebeam,
    Bontrager,
    Wellgo,
    Scosche,
    Magura,
    Woodway,
    Elite,
    NielsenKellerman,
    DkCity,
    Tacx,
    DirectionTechnology,
    Magtonic,
    Name1partcarbon,
    InsideRideTechnologies,
    SoundOfMotion,
    Stryd,
    /// Indoorcycling Group
    Icg,
    MiPulse,
    BsxAthletics,
    Look,
    CampagnoloSrl,
    BodyBikeSmart,
    Praxisworks,
    /// Limits Technology Ltd.
    LimitsTechnology,
    /// TopAction Technology Inc.
    TopactionTechnology,
    Cosinuss,
    Fitcare,
    Magene,
    GiantManufacturingCo,
    /// Tigrasport
    Tigrasport,
    Salutron,
    Technogym,
    BrytonSensors,
    LatitudeLimited,
    SoaringTechnology,
    Igpsport,
    Thinkrider,
    GopherSport,
    Waterrower,
    Orangetheory,
    Inpeak,
    Kinetic,
    JohnsonHealthTech,
    PolarElectro,
    Seesense,
    NciTechnology,
    Iqsquare,
    Leomo,
    IfitCom,
    CorosByte,
    VersaDesign,
    Chileaf,
    Development,
    Healthandlife,
    Lezyne,
    ScribeLabs,
    Zwift,
    Watteam,
    Recon,
    FaveroElectronics,
    Dynovelo,
    Strava,
    /// Amer Sports
    Precor,
    Bryton,
    Sram,
    /// MiTAC Global Corporation (Mio Technology)
    Navman,
    /// COBI GmbH
    Cobi,
    Spivi,
    MioMagellan,
    Evesports,
    SensitivusGauge,
    Podoon,
    LifeTimeFitness,
    /// Falco eMotors Inc.
    FalcoEMotors,
    Minoura,
    Cycliq,
    Luxottica,
    TrainerRoad,
    TheSufferfest,
    Fullspeedahead,
    Virtualtraining,
    Feedbacksports,
    Omata,
    Vdo,
    Magneticdays,
    Hammerhead,
    KineticByKurt,
    Shapelog,
    Dabuziduo,
    Jetblack,
    Coros,
    Virtugo,
    Velosense,
    Cycligentinc,
    Trailforks,
    MahleEbikemotion,
    Nurvv,
    Microprogram,
    Actigraphcorp,
    UnknownVariant(u16),
}
impl Manufacturer {
    pub fn as_u16(self) -> u16 {
        match self {
            Manufacturer::Garmin => 1,
            Manufacturer::GarminFr405Antfs => 2,
            Manufacturer::Zephyr => 3,
            Manufacturer::Dayton => 4,
            Manufacturer::Idt => 5,
            Manufacturer::Srm => 6,
            Manufacturer::Quarq => 7,
            Manufacturer::Ibike => 8,
            Manufacturer::Saris => 9,
            Manufacturer::SparkHk => 10,
            Manufacturer::Tanita => 11,
            Manufacturer::Echowell => 12,
            Manufacturer::DynastreamOem => 13,
            Manufacturer::Nautilus => 14,
            Manufacturer::Dynastream => 15,
            Manufacturer::Timex => 16,
            Manufacturer::Metrigear => 17,
            Manufacturer::Xelic => 18,
            Manufacturer::Beurer => 19,
            Manufacturer::Cardiosport => 20,
            Manufacturer::AAndD => 21,
            Manufacturer::Hmm => 22,
            Manufacturer::Suunto => 23,
            Manufacturer::ThitaElektronik => 24,
            Manufacturer::Gpulse => 25,
            Manufacturer::CleanMobile => 26,
            Manufacturer::PedalBrain => 27,
            Manufacturer::Peaksware => 28,
            Manufacturer::Saxonar => 29,
            Manufacturer::LemondFitness => 30,
            Manufacturer::Dexcom => 31,
            Manufacturer::WahooFitness => 32,
            Manufacturer::OctaneFitness => 33,
            Manufacturer::Archinoetics => 34,
            Manufacturer::TheHurtBox => 35,
            Manufacturer::CitizenSystems => 36,
            Manufacturer::Magellan => 37,
            Manufacturer::Osynce => 38,
            Manufacturer::Holux => 39,
            Manufacturer::Concept2 => 40,
            Manufacturer::OneGiantLeap => 42,
            Manufacturer::AceSensor => 43,
            Manufacturer::BrimBrothers => 44,
            Manufacturer::Xplova => 45,
            Manufacturer::PerceptionDigital => 46,
            Manufacturer::Bf1systems => 47,
            Manufacturer::Pioneer => 48,
            Manufacturer::Spantec => 49,
            Manufacturer::Metalogics => 50,
            Manufacturer::Name4iiiis => 51,
            Manufacturer::SeikoEpson => 52,
            Manufacturer::SeikoEpsonOem => 53,
            Manufacturer::IforPowell => 54,
            Manufacturer::MaxwellGuider => 55,
            Manufacturer::StarTrac => 56,
            Manufacturer::Breakaway => 57,
            Manufacturer::AlatechTechnologyLtd => 58,
            Manufacturer::MioTechnologyEurope => 59,
            Manufacturer::Rotor => 60,
            Manufacturer::Geonaute => 61,
            Manufacturer::IdBike => 62,
            Manufacturer::Specialized => 63,
            Manufacturer::Wtek => 64,
            Manufacturer::PhysicalEnterprises => 65,
            Manufacturer::NorthPoleEngineering => 66,
            Manufacturer::Bkool => 67,
            Manufacturer::Cateye => 68,
            Manufacturer::StagesCycling => 69,
            Manufacturer::Sigmasport => 70,
            Manufacturer::Tomtom => 71,
            Manufacturer::Peripedal => 72,
            Manufacturer::Wattbike => 73,
            Manufacturer::Moxy => 76,
            Manufacturer::Ciclosport => 77,
            Manufacturer::Powerbahn => 78,
            Manufacturer::AcornProjectsAps => 79,
            Manufacturer::Lifebeam => 80,
            Manufacturer::Bontrager => 81,
            Manufacturer::Wellgo => 82,
            Manufacturer::Scosche => 83,
            Manufacturer::Magura => 84,
            Manufacturer::Woodway => 85,
            Manufacturer::Elite => 86,
            Manufacturer::NielsenKellerman => 87,
            Manufacturer::DkCity => 88,
            Manufacturer::Tacx => 89,
            Manufacturer::DirectionTechnology => 90,
            Manufacturer::Magtonic => 91,
            Manufacturer::Name1partcarbon => 92,
            Manufacturer::InsideRideTechnologies => 93,
            Manufacturer::SoundOfMotion => 94,
            Manufacturer::Stryd => 95,
            Manufacturer::Icg => 96,
            Manufacturer::MiPulse => 97,
            Manufacturer::BsxAthletics => 98,
            Manufacturer::Look => 99,
            Manufacturer::CampagnoloSrl => 100,
            Manufacturer::BodyBikeSmart => 101,
            Manufacturer::Praxisworks => 102,
            Manufacturer::LimitsTechnology => 103,
            Manufacturer::TopactionTechnology => 104,
            Manufacturer::Cosinuss => 105,
            Manufacturer::Fitcare => 106,
            Manufacturer::Magene => 107,
            Manufacturer::GiantManufacturingCo => 108,
            Manufacturer::Tigrasport => 109,
            Manufacturer::Salutron => 110,
            Manufacturer::Technogym => 111,
            Manufacturer::BrytonSensors => 112,
            Manufacturer::LatitudeLimited => 113,
            Manufacturer::SoaringTechnology => 114,
            Manufacturer::Igpsport => 115,
            Manufacturer::Thinkrider => 116,
            Manufacturer::GopherSport => 117,
            Manufacturer::Waterrower => 118,
            Manufacturer::Orangetheory => 119,
            Manufacturer::Inpeak => 120,
            Manufacturer::Kinetic => 121,
            Manufacturer::JohnsonHealthTech => 122,
            Manufacturer::PolarElectro => 123,
            Manufacturer::Seesense => 124,
            Manufacturer::NciTechnology => 125,
            Manufacturer::Iqsquare => 126,
            Manufacturer::Leomo => 127,
            Manufacturer::IfitCom => 128,
            Manufacturer::CorosByte => 129,
            Manufacturer::VersaDesign => 130,
            Manufacturer::Chileaf => 131,
            Manufacturer::Development => 255,
            Manufacturer::Healthandlife => 257,
            Manufacturer::Lezyne => 258,
            Manufacturer::ScribeLabs => 259,
            Manufacturer::Zwift => 260,
            Manufacturer::Watteam => 261,
            Manufacturer::Recon => 262,
            Manufacturer::FaveroElectronics => 263,
            Manufacturer::Dynovelo => 264,
            Manufacturer::Strava => 265,
            Manufacturer::Precor => 266,
            Manufacturer::Bryton => 267,
            Manufacturer::Sram => 268,
            Manufacturer::Navman => 269,
            Manufacturer::Cobi => 270,
            Manufacturer::Spivi => 271,
            Manufacturer::MioMagellan => 272,
            Manufacturer::Evesports => 273,
            Manufacturer::SensitivusGauge => 274,
            Manufacturer::Podoon => 275,
            Manufacturer::LifeTimeFitness => 276,
            Manufacturer::FalcoEMotors => 277,
            Manufacturer::Minoura => 278,
            Manufacturer::Cycliq => 279,
            Manufacturer::Luxottica => 280,
            Manufacturer::TrainerRoad => 281,
            Manufacturer::TheSufferfest => 282,
            Manufacturer::Fullspeedahead => 283,
            Manufacturer::Virtualtraining => 284,
            Manufacturer::Feedbacksports => 285,
            Manufacturer::Omata => 286,
            Manufacturer::Vdo => 287,
            Manufacturer::Magneticdays => 288,
            Manufacturer::Hammerhead => 289,
            Manufacturer::KineticByKurt => 290,
            Manufacturer::Shapelog => 291,
            Manufacturer::Dabuziduo => 292,
            Manufacturer::Jetblack => 293,
            Manufacturer::Coros => 294,
            Manufacturer::Virtugo => 295,
            Manufacturer::Velosense => 296,
            Manufacturer::Cycligentinc => 297,
            Manufacturer::Trailforks => 298,
            Manufacturer::MahleEbikemotion => 299,
            Manufacturer::Nurvv => 300,
            Manufacturer::Microprogram => 301,
            Manufacturer::Actigraphcorp => 5759,
            Manufacturer::UnknownVariant(value) => value,
        }
    }
    pub fn as_i64(self) -> i64 {
        self.as_u16() as i64
    }
}
impl fmt::Display for Manufacturer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Manufacturer::Garmin => writeln!(f, "garmin"),
            Manufacturer::GarminFr405Antfs => writeln!(f, "garmin_fr405_antfs"),
            Manufacturer::Zephyr => writeln!(f, "zephyr"),
            Manufacturer::Dayton => writeln!(f, "dayton"),
            Manufacturer::Idt => writeln!(f, "idt"),
            Manufacturer::Srm => writeln!(f, "srm"),
            Manufacturer::Quarq => writeln!(f, "quarq"),
            Manufacturer::Ibike => writeln!(f, "ibike"),
            Manufacturer::Saris => writeln!(f, "saris"),
            Manufacturer::SparkHk => writeln!(f, "spark_hk"),
            Manufacturer::Tanita => writeln!(f, "tanita"),
            Manufacturer::Echowell => writeln!(f, "echowell"),
            Manufacturer::DynastreamOem => writeln!(f, "dynastream_oem"),
            Manufacturer::Nautilus => writeln!(f, "nautilus"),
            Manufacturer::Dynastream => writeln!(f, "dynastream"),
            Manufacturer::Timex => writeln!(f, "timex"),
            Manufacturer::Metrigear => writeln!(f, "metrigear"),
            Manufacturer::Xelic => writeln!(f, "xelic"),
            Manufacturer::Beurer => writeln!(f, "beurer"),
            Manufacturer::Cardiosport => writeln!(f, "cardiosport"),
            Manufacturer::AAndD => writeln!(f, "a_and_d"),
            Manufacturer::Hmm => writeln!(f, "hmm"),
            Manufacturer::Suunto => writeln!(f, "suunto"),
            Manufacturer::ThitaElektronik => writeln!(f, "thita_elektronik"),
            Manufacturer::Gpulse => writeln!(f, "gpulse"),
            Manufacturer::CleanMobile => writeln!(f, "clean_mobile"),
            Manufacturer::PedalBrain => writeln!(f, "pedal_brain"),
            Manufacturer::Peaksware => writeln!(f, "peaksware"),
            Manufacturer::Saxonar => writeln!(f, "saxonar"),
            Manufacturer::LemondFitness => writeln!(f, "lemond_fitness"),
            Manufacturer::Dexcom => writeln!(f, "dexcom"),
            Manufacturer::WahooFitness => writeln!(f, "wahoo_fitness"),
            Manufacturer::OctaneFitness => writeln!(f, "octane_fitness"),
            Manufacturer::Archinoetics => writeln!(f, "archinoetics"),
            Manufacturer::TheHurtBox => writeln!(f, "the_hurt_box"),
            Manufacturer::CitizenSystems => writeln!(f, "citizen_systems"),
            Manufacturer::Magellan => writeln!(f, "magellan"),
            Manufacturer::Osynce => writeln!(f, "osynce"),
            Manufacturer::Holux => writeln!(f, "holux"),
            Manufacturer::Concept2 => writeln!(f, "concept2"),
            Manufacturer::OneGiantLeap => writeln!(f, "one_giant_leap"),
            Manufacturer::AceSensor => writeln!(f, "ace_sensor"),
            Manufacturer::BrimBrothers => writeln!(f, "brim_brothers"),
            Manufacturer::Xplova => writeln!(f, "xplova"),
            Manufacturer::PerceptionDigital => writeln!(f, "perception_digital"),
            Manufacturer::Bf1systems => writeln!(f, "bf1systems"),
            Manufacturer::Pioneer => writeln!(f, "pioneer"),
            Manufacturer::Spantec => writeln!(f, "spantec"),
            Manufacturer::Metalogics => writeln!(f, "metalogics"),
            Manufacturer::Name4iiiis => writeln!(f, "4iiiis"),
            Manufacturer::SeikoEpson => writeln!(f, "seiko_epson"),
            Manufacturer::SeikoEpsonOem => writeln!(f, "seiko_epson_oem"),
            Manufacturer::IforPowell => writeln!(f, "ifor_powell"),
            Manufacturer::MaxwellGuider => writeln!(f, "maxwell_guider"),
            Manufacturer::StarTrac => writeln!(f, "star_trac"),
            Manufacturer::Breakaway => writeln!(f, "breakaway"),
            Manufacturer::AlatechTechnologyLtd => writeln!(f, "alatech_technology_ltd"),
            Manufacturer::MioTechnologyEurope => writeln!(f, "mio_technology_europe"),
            Manufacturer::Rotor => writeln!(f, "rotor"),
            Manufacturer::Geonaute => writeln!(f, "geonaute"),
            Manufacturer::IdBike => writeln!(f, "id_bike"),
            Manufacturer::Specialized => writeln!(f, "specialized"),
            Manufacturer::Wtek => writeln!(f, "wtek"),
            Manufacturer::PhysicalEnterprises => writeln!(f, "physical_enterprises"),
            Manufacturer::NorthPoleEngineering => writeln!(f, "north_pole_engineering"),
            Manufacturer::Bkool => writeln!(f, "bkool"),
            Manufacturer::Cateye => writeln!(f, "cateye"),
            Manufacturer::StagesCycling => writeln!(f, "stages_cycling"),
            Manufacturer::Sigmasport => writeln!(f, "sigmasport"),
            Manufacturer::Tomtom => writeln!(f, "tomtom"),
            Manufacturer::Peripedal => writeln!(f, "peripedal"),
            Manufacturer::Wattbike => writeln!(f, "wattbike"),
            Manufacturer::Moxy => writeln!(f, "moxy"),
            Manufacturer::Ciclosport => writeln!(f, "ciclosport"),
            Manufacturer::Powerbahn => writeln!(f, "powerbahn"),
            Manufacturer::AcornProjectsAps => writeln!(f, "acorn_projects_aps"),
            Manufacturer::Lifebeam => writeln!(f, "lifebeam"),
            Manufacturer::Bontrager => writeln!(f, "bontrager"),
            Manufacturer::Wellgo => writeln!(f, "wellgo"),
            Manufacturer::Scosche => writeln!(f, "scosche"),
            Manufacturer::Magura => writeln!(f, "magura"),
            Manufacturer::Woodway => writeln!(f, "woodway"),
            Manufacturer::Elite => writeln!(f, "elite"),
            Manufacturer::NielsenKellerman => writeln!(f, "nielsen_kellerman"),
            Manufacturer::DkCity => writeln!(f, "dk_city"),
            Manufacturer::Tacx => writeln!(f, "tacx"),
            Manufacturer::DirectionTechnology => writeln!(f, "direction_technology"),
            Manufacturer::Magtonic => writeln!(f, "magtonic"),
            Manufacturer::Name1partcarbon => writeln!(f, "1partcarbon"),
            Manufacturer::InsideRideTechnologies => writeln!(f, "inside_ride_technologies"),
            Manufacturer::SoundOfMotion => writeln!(f, "sound_of_motion"),
            Manufacturer::Stryd => writeln!(f, "stryd"),
            Manufacturer::Icg => writeln!(f, "icg"),
            Manufacturer::MiPulse => writeln!(f, "MiPulse"),
            Manufacturer::BsxAthletics => writeln!(f, "bsx_athletics"),
            Manufacturer::Look => writeln!(f, "look"),
            Manufacturer::CampagnoloSrl => writeln!(f, "campagnolo_srl"),
            Manufacturer::BodyBikeSmart => writeln!(f, "body_bike_smart"),
            Manufacturer::Praxisworks => writeln!(f, "praxisworks"),
            Manufacturer::LimitsTechnology => writeln!(f, "limits_technology"),
            Manufacturer::TopactionTechnology => writeln!(f, "topaction_technology"),
            Manufacturer::Cosinuss => writeln!(f, "cosinuss"),
            Manufacturer::Fitcare => writeln!(f, "fitcare"),
            Manufacturer::Magene => writeln!(f, "magene"),
            Manufacturer::GiantManufacturingCo => writeln!(f, "giant_manufacturing_co"),
            Manufacturer::Tigrasport => writeln!(f, "tigrasport"),
            Manufacturer::Salutron => writeln!(f, "salutron"),
            Manufacturer::Technogym => writeln!(f, "technogym"),
            Manufacturer::BrytonSensors => writeln!(f, "bryton_sensors"),
            Manufacturer::LatitudeLimited => writeln!(f, "latitude_limited"),
            Manufacturer::SoaringTechnology => writeln!(f, "soaring_technology"),
            Manufacturer::Igpsport => writeln!(f, "igpsport"),
            Manufacturer::Thinkrider => writeln!(f, "thinkrider"),
            Manufacturer::GopherSport => writeln!(f, "gopher_sport"),
            Manufacturer::Waterrower => writeln!(f, "waterrower"),
            Manufacturer::Orangetheory => writeln!(f, "orangetheory"),
            Manufacturer::Inpeak => writeln!(f, "inpeak"),
            Manufacturer::Kinetic => writeln!(f, "kinetic"),
            Manufacturer::JohnsonHealthTech => writeln!(f, "johnson_health_tech"),
            Manufacturer::PolarElectro => writeln!(f, "polar_electro"),
            Manufacturer::Seesense => writeln!(f, "seesense"),
            Manufacturer::NciTechnology => writeln!(f, "nci_technology"),
            Manufacturer::Iqsquare => writeln!(f, "iqsquare"),
            Manufacturer::Leomo => writeln!(f, "leomo"),
            Manufacturer::IfitCom => writeln!(f, "ifit_com"),
            Manufacturer::CorosByte => writeln!(f, "coros_byte"),
            Manufacturer::VersaDesign => writeln!(f, "versa_design"),
            Manufacturer::Chileaf => writeln!(f, "chileaf"),
            Manufacturer::Development => writeln!(f, "development"),
            Manufacturer::Healthandlife => writeln!(f, "healthandlife"),
            Manufacturer::Lezyne => writeln!(f, "lezyne"),
            Manufacturer::ScribeLabs => writeln!(f, "scribe_labs"),
            Manufacturer::Zwift => writeln!(f, "zwift"),
            Manufacturer::Watteam => writeln!(f, "watteam"),
            Manufacturer::Recon => writeln!(f, "recon"),
            Manufacturer::FaveroElectronics => writeln!(f, "favero_electronics"),
            Manufacturer::Dynovelo => writeln!(f, "dynovelo"),
            Manufacturer::Strava => writeln!(f, "strava"),
            Manufacturer::Precor => writeln!(f, "precor"),
            Manufacturer::Bryton => writeln!(f, "bryton"),
            Manufacturer::Sram => writeln!(f, "sram"),
            Manufacturer::Navman => writeln!(f, "navman"),
            Manufacturer::Cobi => writeln!(f, "cobi"),
            Manufacturer::Spivi => writeln!(f, "spivi"),
            Manufacturer::MioMagellan => writeln!(f, "mio_magellan"),
            Manufacturer::Evesports => writeln!(f, "evesports"),
            Manufacturer::SensitivusGauge => writeln!(f, "sensitivus_gauge"),
            Manufacturer::Podoon => writeln!(f, "podoon"),
            Manufacturer::LifeTimeFitness => writeln!(f, "life_time_fitness"),
            Manufacturer::FalcoEMotors => writeln!(f, "falco_e_motors"),
            Manufacturer::Minoura => writeln!(f, "minoura"),
            Manufacturer::Cycliq => writeln!(f, "cycliq"),
            Manufacturer::Luxottica => writeln!(f, "luxottica"),
            Manufacturer::TrainerRoad => writeln!(f, "trainer_road"),
            Manufacturer::TheSufferfest => writeln!(f, "the_sufferfest"),
            Manufacturer::Fullspeedahead => writeln!(f, "fullspeedahead"),
            Manufacturer::Virtualtraining => writeln!(f, "virtualtraining"),
            Manufacturer::Feedbacksports => writeln!(f, "feedbacksports"),
            Manufacturer::Omata => writeln!(f, "omata"),
            Manufacturer::Vdo => writeln!(f, "vdo"),
            Manufacturer::Magneticdays => writeln!(f, "magneticdays"),
            Manufacturer::Hammerhead => writeln!(f, "hammerhead"),
            Manufacturer::KineticByKurt => writeln!(f, "kinetic_by_kurt"),
            Manufacturer::Shapelog => writeln!(f, "shapelog"),
            Manufacturer::Dabuziduo => writeln!(f, "dabuziduo"),
            Manufacturer::Jetblack => writeln!(f, "jetblack"),
            Manufacturer::Coros => writeln!(f, "coros"),
            Manufacturer::Virtugo => writeln!(f, "virtugo"),
            Manufacturer::Velosense => writeln!(f, "velosense"),
            Manufacturer::Cycligentinc => writeln!(f, "cycligentinc"),
            Manufacturer::Trailforks => writeln!(f, "trailforks"),
            Manufacturer::MahleEbikemotion => writeln!(f, "mahle_ebikemotion"),
            Manufacturer::Nurvv => writeln!(f, "nurvv"),
            Manufacturer::Microprogram => writeln!(f, "microprogram"),
            Manufacturer::Actigraphcorp => writeln!(f, "actigraphcorp"),
            Manufacturer::UnknownVariant(value) => writeln!(f, "unknown_variant_{}", *value),
        }
    }
}
impl convert::From<u16> for Manufacturer {
    fn from(value: u16) -> Self {
        match value {
            1 => Manufacturer::Garmin,
            2 => Manufacturer::GarminFr405Antfs,
            3 => Manufacturer::Zephyr,
            4 => Manufacturer::Dayton,
            5 => Manufacturer::Idt,
            6 => Manufacturer::Srm,
            7 => Manufacturer::Quarq,
            8 => Manufacturer::Ibike,
            9 => Manufacturer::Saris,
            10 => Manufacturer::SparkHk,
            11 => Manufacturer::Tanita,
            12 => Manufacturer::Echowell,
            13 => Manufacturer::DynastreamOem,
            14 => Manufacturer::Nautilus,
            15 => Manufacturer::Dynastream,
            16 => Manufacturer::Timex,
            17 => Manufacturer::Metrigear,
            18 => Manufacturer::Xelic,
            19 => Manufacturer::Beurer,
            20 => Manufacturer::Cardiosport,
            21 => Manufacturer::AAndD,
            22 => Manufacturer::Hmm,
            23 => Manufacturer::Suunto,
            24 => Manufacturer::ThitaElektronik,
            25 => Manufacturer::Gpulse,
            26 => Manufacturer::CleanMobile,
            27 => Manufacturer::PedalBrain,
            28 => Manufacturer::Peaksware,
            29 => Manufacturer::Saxonar,
            30 => Manufacturer::LemondFitness,
            31 => Manufacturer::Dexcom,
            32 => Manufacturer::WahooFitness,
            33 => Manufacturer::OctaneFitness,
            34 => Manufacturer::Archinoetics,
            35 => Manufacturer::TheHurtBox,
            36 => Manufacturer::CitizenSystems,
            37 => Manufacturer::Magellan,
            38 => Manufacturer::Osynce,
            39 => Manufacturer::Holux,
            40 => Manufacturer::Concept2,
            42 => Manufacturer::OneGiantLeap,
            43 => Manufacturer::AceSensor,
            44 => Manufacturer::BrimBrothers,
            45 => Manufacturer::Xplova,
            46 => Manufacturer::PerceptionDigital,
            47 => Manufacturer::Bf1systems,
            48 => Manufacturer::Pioneer,
            49 => Manufacturer::Spantec,
            50 => Manufacturer::Metalogics,
            51 => Manufacturer::Name4iiiis,
            52 => Manufacturer::SeikoEpson,
            53 => Manufacturer::SeikoEpsonOem,
            54 => Manufacturer::IforPowell,
            55 => Manufacturer::MaxwellGuider,
            56 => Manufacturer::StarTrac,
            57 => Manufacturer::Breakaway,
            58 => Manufacturer::AlatechTechnologyLtd,
            59 => Manufacturer::MioTechnologyEurope,
            60 => Manufacturer::Rotor,
            61 => Manufacturer::Geonaute,
            62 => Manufacturer::IdBike,
            63 => Manufacturer::Specialized,
            64 => Manufacturer::Wtek,
            65 => Manufacturer::PhysicalEnterprises,
            66 => Manufacturer::NorthPoleEngineering,
            67 => Manufacturer::Bkool,
            68 => Manufacturer::Cateye,
            69 => Manufacturer::StagesCycling,
            70 => Manufacturer::Sigmasport,
            71 => Manufacturer::Tomtom,
            72 => Manufacturer::Peripedal,
            73 => Manufacturer::Wattbike,
            76 => Manufacturer::Moxy,
            77 => Manufacturer::Ciclosport,
            78 => Manufacturer::Powerbahn,
            79 => Manufacturer::AcornProjectsAps,
            80 => Manufacturer::Lifebeam,
            81 => Manufacturer::Bontrager,
            82 => Manufacturer::Wellgo,
            83 => Manufacturer::Scosche,
            84 => Manufacturer::Magura,
            85 => Manufacturer::Woodway,
            86 => Manufacturer::Elite,
            87 => Manufacturer::NielsenKellerman,
            88 => Manufacturer::DkCity,
            89 => Manufacturer::Tacx,
            90 => Manufacturer::DirectionTechnology,
            91 => Manufacturer::Magtonic,
            92 => Manufacturer::Name1partcarbon,
            93 => Manufacturer::InsideRideTechnologies,
            94 => Manufacturer::SoundOfMotion,
            95 => Manufacturer::Stryd,
            96 => Manufacturer::Icg,
            97 => Manufacturer::MiPulse,
            98 => Manufacturer::BsxAthletics,
            99 => Manufacturer::Look,
            100 => Manufacturer::CampagnoloSrl,
            101 => Manufacturer::BodyBikeSmart,
            102 => Manufacturer::Praxisworks,
            103 => Manufacturer::LimitsTechnology,
            104 => Manufacturer::TopactionTechnology,
            105 => Manufacturer::Cosinuss,
            106 => Manufacturer::Fitcare,
            107 => Manufacturer::Magene,
            108 => Manufacturer::GiantManufacturingCo,
            109 => Manufacturer::Tigrasport,
            110 => Manufacturer::Salutron,
            111 => Manufacturer::Technogym,
            112 => Manufacturer::BrytonSensors,
            113 => Manufacturer::LatitudeLimited,
            114 => Manufacturer::SoaringTechnology,
            115 => Manufacturer::Igpsport,
            116 => Manufacturer::Thinkrider,
            117 => Manufacturer::GopherSport,
            118 => Manufacturer::Waterrower,
            119 => Manufacturer::Orangetheory,
            120 => Manufacturer::Inpeak,
            121 => Manufacturer::Kinetic,
            122 => Manufacturer::JohnsonHealthTech,
            123 => Manufacturer::PolarElectro,
            124 => Manufacturer::Seesense,
            125 => Manufacturer::NciTechnology,
            126 => Manufacturer::Iqsquare,
            127 => Manufacturer::Leomo,
            128 => Manufacturer::IfitCom,
            129 => Manufacturer::CorosByte,
            130 => Manufacturer::VersaDesign,
            131 => Manufacturer::Chileaf,
            255 => Manufacturer::Development,
            257 => Manufacturer::Healthandlife,
            258 => Manufacturer::Lezyne,
            259 => Manufacturer::ScribeLabs,
            260 => Manufacturer::Zwift,
            261 => Manufacturer::Watteam,
            262 => Manufacturer::Recon,
            263 => Manufacturer::FaveroElectronics,
            264 => Manufacturer::Dynovelo,
            265 => Manufacturer::Strava,
            266 => Manufacturer::Precor,
            267 => Manufacturer::Bryton,
            268 => Manufacturer::Sram,
            269 => Manufacturer::Navman,
            270 => Manufacturer::Cobi,
            271 => Manufacturer::Spivi,
            272 => Manufacturer::MioMagellan,
            273 => Manufacturer::Evesports,
            274 => Manufacturer::SensitivusGauge,
            275 => Manufacturer::Podoon,
            276 => Manufacturer::LifeTimeFitness,
            277 => Manufacturer::FalcoEMotors,
            278 => Manufacturer::Minoura,
            279 => Manufacturer::Cycliq,
            280 => Manufacturer::Luxottica,
            281 => Manufacturer::TrainerRoad,
            282 => Manufacturer::TheSufferfest,
            283 => Manufacturer::Fullspeedahead,
            284 => Manufacturer::Virtualtraining,
            285 => Manufacturer::Feedbacksports,
            286 => Manufacturer::Omata,
            287 => Manufacturer::Vdo,
            288 => Manufacturer::Magneticdays,
            289 => Manufacturer::Hammerhead,
            290 => Manufacturer::KineticByKurt,
            291 => Manufacturer::Shapelog,
            292 => Manufacturer::Dabuziduo,
            293 => Manufacturer::Jetblack,
            294 => Manufacturer::Coros,
            295 => Manufacturer::Virtugo,
            296 => Manufacturer::Velosense,
            297 => Manufacturer::Cycligentinc,
            298 => Manufacturer::Trailforks,
            299 => Manufacturer::MahleEbikemotion,
            300 => Manufacturer::Nurvv,
            301 => Manufacturer::Microprogram,
            5759 => Manufacturer::Actigraphcorp,
            _ => Manufacturer::UnknownVariant(value),
        }
    }
}
impl convert::From<i64> for Manufacturer {
    fn from(value: i64) -> Self {
        Manufacturer::from(value as u16)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum GarminProduct {
    Hrm1,
    /// AXH01 HRM chipset
    Axh01,
    Axb01,
    Axb02,
    Hrm2ss,
    DsiAlf02,
    Hrm3ss,
    /// hrm_run model for HRM ANT+ messaging
    HrmRunSingleByteProductId,
    /// BSM model for ANT+ messaging
    Bsm,
    /// BCM model for ANT+ messaging
    Bcm,
    /// AXS01 HRM Bike Chipset model for ANT+ messaging
    Axs01,
    /// hrm_tri model for HRM ANT+ messaging
    HrmTriSingleByteProductId,
    /// fr225 model for HRM ANT+ messaging
    Fr225SingleByteProductId,
    Fr301China,
    Fr301Japan,
    Fr301Korea,
    Fr301Taiwan,
    /// Forerunner 405
    Fr405,
    /// Forerunner 50
    Fr50,
    Fr405Japan,
    /// Forerunner 60
    Fr60,
    DsiAlf01,
    /// Forerunner 310
    Fr310xt,
    Edge500,
    /// Forerunner 110
    Fr110,
    Edge800,
    Edge500Taiwan,
    Edge500Japan,
    Chirp,
    Fr110Japan,
    Edge200,
    Fr910xt,
    Edge800Taiwan,
    Edge800Japan,
    Alf04,
    Fr610,
    Fr210Japan,
    VectorSs,
    VectorCp,
    Edge800China,
    Edge500China,
    Fr610Japan,
    Edge500Korea,
    Fr70,
    Fr310xt4t,
    Amx,
    Fr10,
    Edge800Korea,
    Swim,
    Fr910xtChina,
    Fenix,
    Edge200Taiwan,
    Edge510,
    Edge810,
    Tempe,
    Fr910xtJapan,
    Fr620,
    Fr220,
    Fr910xtKorea,
    Fr10Japan,
    Edge810Japan,
    VirbElite,
    /// Also Edge Touring Plus
    EdgeTouring,
    Edge510Japan,
    HrmTri,
    HrmRun,
    Fr920xt,
    Edge510Asia,
    Edge810China,
    Edge810Taiwan,
    Edge1000,
    VivoFit,
    VirbRemote,
    VivoKi,
    Fr15,
    VivoActive,
    Edge510Korea,
    Fr620Japan,
    Fr620China,
    Fr220Japan,
    Fr220China,
    ApproachS6,
    VivoSmart,
    Fenix2,
    Epix,
    Fenix3,
    Edge1000Taiwan,
    Edge1000Japan,
    Fr15Japan,
    Edge520,
    Edge1000China,
    Fr620Russia,
    Fr220Russia,
    VectorS,
    Edge1000Korea,
    Fr920xtTaiwan,
    Fr920xtChina,
    Fr920xtJapan,
    Virbx,
    VivoSmartApac,
    EtrexTouch,
    Edge25,
    Fr25,
    VivoFit2,
    Fr225,
    Fr630,
    Fr230,
    Fr735xt,
    VivoActiveApac,
    Vector2,
    Vector2s,
    Virbxe,
    Fr620Taiwan,
    Fr220Taiwan,
    Truswing,
    Fenix3China,
    Fenix3Twn,
    VariaHeadlight,
    VariaTaillightOld,
    EdgeExplore1000,
    Fr225Asia,
    VariaRadarTaillight,
    VariaRadarDisplay,
    Edge20,
    D2Bravo,
    ApproachS20,
    VariaRemote,
    ApproachX40,
    Hrm4Run,
    VivoActiveHr,
    VivoSmartGpsHr,
    VivoSmartHr,
    VivoMove,
    VariaVision,
    VivoFit3,
    Fenix3Hr,
    VirbUltra30,
    IndexSmartScale,
    Fr235,
    Fenix3Chronos,
    Oregon7xx,
    Rino7xx,
    Nautix,
    Edge820,
    EdgeExplore820,
    Fr735xtApac,
    Fr735xtJapan,
    Fenix5s,
    D2BravoTitanium,
    /// Varia UT 800 SW
    VariaUt800,
    RunningDynamicsPod,
    Fenix5x,
    VivoFitJr,
    VivoSmart3,
    VivoSport,
    ApproachS60,
    Virb360,
    Fr935,
    Fenix5,
    Vivoactive3,
    Edge1030,
    Foretrex601701,
    VivoMoveHr,
    ApproachZ80,
    VivoSmart3Apac,
    VivoSportApac,
    Descent,
    Fr645,
    Fr645m,
    Fenix5sPlus,
    Edge130,
    Vivosmart4,
    ApproachX10,
    Vivoactive3mW,
    EdgeExplore,
    Gpsmap66,
    ApproachS10,
    Vivoactive3mL,
    ApproachG80,
    Fenix5Plus,
    Fenix5xPlus,
    Edge520Plus,
    /// HRM-Dual
    HrmDual,
    ApproachS40,
    /// SDM4 footpod
    Sdm4,
    EdgeRemote,
    TrainingCenter,
    ConnectiqSimulator,
    AndroidAntplusPlugin,
    /// Garmin Connect website
    Connect,
    UnknownVariant(u16),
}
impl GarminProduct {
    pub fn as_u16(self) -> u16 {
        match self {
            GarminProduct::Hrm1 => 1,
            GarminProduct::Axh01 => 2,
            GarminProduct::Axb01 => 3,
            GarminProduct::Axb02 => 4,
            GarminProduct::Hrm2ss => 5,
            GarminProduct::DsiAlf02 => 6,
            GarminProduct::Hrm3ss => 7,
            GarminProduct::HrmRunSingleByteProductId => 8,
            GarminProduct::Bsm => 9,
            GarminProduct::Bcm => 10,
            GarminProduct::Axs01 => 11,
            GarminProduct::HrmTriSingleByteProductId => 12,
            GarminProduct::Fr225SingleByteProductId => 14,
            GarminProduct::Fr301China => 473,
            GarminProduct::Fr301Japan => 474,
            GarminProduct::Fr301Korea => 475,
            GarminProduct::Fr301Taiwan => 494,
            GarminProduct::Fr405 => 717,
            GarminProduct::Fr50 => 782,
            GarminProduct::Fr405Japan => 987,
            GarminProduct::Fr60 => 988,
            GarminProduct::DsiAlf01 => 1011,
            GarminProduct::Fr310xt => 1018,
            GarminProduct::Edge500 => 1036,
            GarminProduct::Fr110 => 1124,
            GarminProduct::Edge800 => 1169,
            GarminProduct::Edge500Taiwan => 1199,
            GarminProduct::Edge500Japan => 1213,
            GarminProduct::Chirp => 1253,
            GarminProduct::Fr110Japan => 1274,
            GarminProduct::Edge200 => 1325,
            GarminProduct::Fr910xt => 1328,
            GarminProduct::Edge800Taiwan => 1333,
            GarminProduct::Edge800Japan => 1334,
            GarminProduct::Alf04 => 1341,
            GarminProduct::Fr610 => 1345,
            GarminProduct::Fr210Japan => 1360,
            GarminProduct::VectorSs => 1380,
            GarminProduct::VectorCp => 1381,
            GarminProduct::Edge800China => 1386,
            GarminProduct::Edge500China => 1387,
            GarminProduct::Fr610Japan => 1410,
            GarminProduct::Edge500Korea => 1422,
            GarminProduct::Fr70 => 1436,
            GarminProduct::Fr310xt4t => 1446,
            GarminProduct::Amx => 1461,
            GarminProduct::Fr10 => 1482,
            GarminProduct::Edge800Korea => 1497,
            GarminProduct::Swim => 1499,
            GarminProduct::Fr910xtChina => 1537,
            GarminProduct::Fenix => 1551,
            GarminProduct::Edge200Taiwan => 1555,
            GarminProduct::Edge510 => 1561,
            GarminProduct::Edge810 => 1567,
            GarminProduct::Tempe => 1570,
            GarminProduct::Fr910xtJapan => 1600,
            GarminProduct::Fr620 => 1623,
            GarminProduct::Fr220 => 1632,
            GarminProduct::Fr910xtKorea => 1664,
            GarminProduct::Fr10Japan => 1688,
            GarminProduct::Edge810Japan => 1721,
            GarminProduct::VirbElite => 1735,
            GarminProduct::EdgeTouring => 1736,
            GarminProduct::Edge510Japan => 1742,
            GarminProduct::HrmTri => 1743,
            GarminProduct::HrmRun => 1752,
            GarminProduct::Fr920xt => 1765,
            GarminProduct::Edge510Asia => 1821,
            GarminProduct::Edge810China => 1822,
            GarminProduct::Edge810Taiwan => 1823,
            GarminProduct::Edge1000 => 1836,
            GarminProduct::VivoFit => 1837,
            GarminProduct::VirbRemote => 1853,
            GarminProduct::VivoKi => 1885,
            GarminProduct::Fr15 => 1903,
            GarminProduct::VivoActive => 1907,
            GarminProduct::Edge510Korea => 1918,
            GarminProduct::Fr620Japan => 1928,
            GarminProduct::Fr620China => 1929,
            GarminProduct::Fr220Japan => 1930,
            GarminProduct::Fr220China => 1931,
            GarminProduct::ApproachS6 => 1936,
            GarminProduct::VivoSmart => 1956,
            GarminProduct::Fenix2 => 1967,
            GarminProduct::Epix => 1988,
            GarminProduct::Fenix3 => 2050,
            GarminProduct::Edge1000Taiwan => 2052,
            GarminProduct::Edge1000Japan => 2053,
            GarminProduct::Fr15Japan => 2061,
            GarminProduct::Edge520 => 2067,
            GarminProduct::Edge1000China => 2070,
            GarminProduct::Fr620Russia => 2072,
            GarminProduct::Fr220Russia => 2073,
            GarminProduct::VectorS => 2079,
            GarminProduct::Edge1000Korea => 2100,
            GarminProduct::Fr920xtTaiwan => 2130,
            GarminProduct::Fr920xtChina => 2131,
            GarminProduct::Fr920xtJapan => 2132,
            GarminProduct::Virbx => 2134,
            GarminProduct::VivoSmartApac => 2135,
            GarminProduct::EtrexTouch => 2140,
            GarminProduct::Edge25 => 2147,
            GarminProduct::Fr25 => 2148,
            GarminProduct::VivoFit2 => 2150,
            GarminProduct::Fr225 => 2153,
            GarminProduct::Fr630 => 2156,
            GarminProduct::Fr230 => 2157,
            GarminProduct::Fr735xt => 2158,
            GarminProduct::VivoActiveApac => 2160,
            GarminProduct::Vector2 => 2161,
            GarminProduct::Vector2s => 2162,
            GarminProduct::Virbxe => 2172,
            GarminProduct::Fr620Taiwan => 2173,
            GarminProduct::Fr220Taiwan => 2174,
            GarminProduct::Truswing => 2175,
            GarminProduct::Fenix3China => 2188,
            GarminProduct::Fenix3Twn => 2189,
            GarminProduct::VariaHeadlight => 2192,
            GarminProduct::VariaTaillightOld => 2193,
            GarminProduct::EdgeExplore1000 => 2204,
            GarminProduct::Fr225Asia => 2219,
            GarminProduct::VariaRadarTaillight => 2225,
            GarminProduct::VariaRadarDisplay => 2226,
            GarminProduct::Edge20 => 2238,
            GarminProduct::D2Bravo => 2262,
            GarminProduct::ApproachS20 => 2266,
            GarminProduct::VariaRemote => 2276,
            GarminProduct::ApproachX40 => 2292,
            GarminProduct::Hrm4Run => 2327,
            GarminProduct::VivoActiveHr => 2337,
            GarminProduct::VivoSmartGpsHr => 2347,
            GarminProduct::VivoSmartHr => 2348,
            GarminProduct::VivoMove => 2368,
            GarminProduct::VariaVision => 2398,
            GarminProduct::VivoFit3 => 2406,
            GarminProduct::Fenix3Hr => 2413,
            GarminProduct::VirbUltra30 => 2417,
            GarminProduct::IndexSmartScale => 2429,
            GarminProduct::Fr235 => 2431,
            GarminProduct::Fenix3Chronos => 2432,
            GarminProduct::Oregon7xx => 2441,
            GarminProduct::Rino7xx => 2444,
            GarminProduct::Nautix => 2496,
            GarminProduct::Edge820 => 2530,
            GarminProduct::EdgeExplore820 => 2531,
            GarminProduct::Fr735xtApac => 2533,
            GarminProduct::Fr735xtJapan => 2534,
            GarminProduct::Fenix5s => 2544,
            GarminProduct::D2BravoTitanium => 2547,
            GarminProduct::VariaUt800 => 2567,
            GarminProduct::RunningDynamicsPod => 2593,
            GarminProduct::Fenix5x => 2604,
            GarminProduct::VivoFitJr => 2606,
            GarminProduct::VivoSmart3 => 2622,
            GarminProduct::VivoSport => 2623,
            GarminProduct::ApproachS60 => 2656,
            GarminProduct::Virb360 => 2687,
            GarminProduct::Fr935 => 2691,
            GarminProduct::Fenix5 => 2697,
            GarminProduct::Vivoactive3 => 2700,
            GarminProduct::Edge1030 => 2713,
            GarminProduct::Foretrex601701 => 2769,
            GarminProduct::VivoMoveHr => 2772,
            GarminProduct::ApproachZ80 => 2806,
            GarminProduct::VivoSmart3Apac => 2831,
            GarminProduct::VivoSportApac => 2832,
            GarminProduct::Descent => 2859,
            GarminProduct::Fr645 => 2886,
            GarminProduct::Fr645m => 2888,
            GarminProduct::Fenix5sPlus => 2900,
            GarminProduct::Edge130 => 2909,
            GarminProduct::Vivosmart4 => 2927,
            GarminProduct::ApproachX10 => 2962,
            GarminProduct::Vivoactive3mW => 2988,
            GarminProduct::EdgeExplore => 3011,
            GarminProduct::Gpsmap66 => 3028,
            GarminProduct::ApproachS10 => 3049,
            GarminProduct::Vivoactive3mL => 3066,
            GarminProduct::ApproachG80 => 3085,
            GarminProduct::Fenix5Plus => 3110,
            GarminProduct::Fenix5xPlus => 3111,
            GarminProduct::Edge520Plus => 3112,
            GarminProduct::HrmDual => 3299,
            GarminProduct::ApproachS40 => 3314,
            GarminProduct::Sdm4 => 10007,
            GarminProduct::EdgeRemote => 10014,
            GarminProduct::TrainingCenter => 20119,
            GarminProduct::ConnectiqSimulator => 65531,
            GarminProduct::AndroidAntplusPlugin => 65532,
            GarminProduct::Connect => 65534,
            GarminProduct::UnknownVariant(value) => value,
        }
    }
    pub fn as_i64(self) -> i64 {
        self.as_u16() as i64
    }
}
impl fmt::Display for GarminProduct {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            GarminProduct::Hrm1 => writeln!(f, "hrm1"),
            GarminProduct::Axh01 => writeln!(f, "axh01"),
            GarminProduct::Axb01 => writeln!(f, "axb01"),
            GarminProduct::Axb02 => writeln!(f, "axb02"),
            GarminProduct::Hrm2ss => writeln!(f, "hrm2ss"),
            GarminProduct::DsiAlf02 => writeln!(f, "dsi_alf02"),
            GarminProduct::Hrm3ss => writeln!(f, "hrm3ss"),
            GarminProduct::HrmRunSingleByteProductId => {
                writeln!(f, "hrm_run_single_byte_product_id")
            }
            GarminProduct::Bsm => writeln!(f, "bsm"),
            GarminProduct::Bcm => writeln!(f, "bcm"),
            GarminProduct::Axs01 => writeln!(f, "axs01"),
            GarminProduct::HrmTriSingleByteProductId => {
                writeln!(f, "hrm_tri_single_byte_product_id")
            }
            GarminProduct::Fr225SingleByteProductId => writeln!(f, "fr225_single_byte_product_id"),
            GarminProduct::Fr301China => writeln!(f, "fr301_china"),
            GarminProduct::Fr301Japan => writeln!(f, "fr301_japan"),
            GarminProduct::Fr301Korea => writeln!(f, "fr301_korea"),
            GarminProduct::Fr301Taiwan => writeln!(f, "fr301_taiwan"),
            GarminProduct::Fr405 => writeln!(f, "fr405"),
            GarminProduct::Fr50 => writeln!(f, "fr50"),
            GarminProduct::Fr405Japan => writeln!(f, "fr405_japan"),
            GarminProduct::Fr60 => writeln!(f, "fr60"),
            GarminProduct::DsiAlf01 => writeln!(f, "dsi_alf01"),
            GarminProduct::Fr310xt => writeln!(f, "fr310xt"),
            GarminProduct::Edge500 => writeln!(f, "edge500"),
            GarminProduct::Fr110 => writeln!(f, "fr110"),
            GarminProduct::Edge800 => writeln!(f, "edge800"),
            GarminProduct::Edge500Taiwan => writeln!(f, "edge500_taiwan"),
            GarminProduct::Edge500Japan => writeln!(f, "edge500_japan"),
            GarminProduct::Chirp => writeln!(f, "chirp"),
            GarminProduct::Fr110Japan => writeln!(f, "fr110_japan"),
            GarminProduct::Edge200 => writeln!(f, "edge200"),
            GarminProduct::Fr910xt => writeln!(f, "fr910xt"),
            GarminProduct::Edge800Taiwan => writeln!(f, "edge800_taiwan"),
            GarminProduct::Edge800Japan => writeln!(f, "edge800_japan"),
            GarminProduct::Alf04 => writeln!(f, "alf04"),
            GarminProduct::Fr610 => writeln!(f, "fr610"),
            GarminProduct::Fr210Japan => writeln!(f, "fr210_japan"),
            GarminProduct::VectorSs => writeln!(f, "vector_ss"),
            GarminProduct::VectorCp => writeln!(f, "vector_cp"),
            GarminProduct::Edge800China => writeln!(f, "edge800_china"),
            GarminProduct::Edge500China => writeln!(f, "edge500_china"),
            GarminProduct::Fr610Japan => writeln!(f, "fr610_japan"),
            GarminProduct::Edge500Korea => writeln!(f, "edge500_korea"),
            GarminProduct::Fr70 => writeln!(f, "fr70"),
            GarminProduct::Fr310xt4t => writeln!(f, "fr310xt_4t"),
            GarminProduct::Amx => writeln!(f, "amx"),
            GarminProduct::Fr10 => writeln!(f, "fr10"),
            GarminProduct::Edge800Korea => writeln!(f, "edge800_korea"),
            GarminProduct::Swim => writeln!(f, "swim"),
            GarminProduct::Fr910xtChina => writeln!(f, "fr910xt_china"),
            GarminProduct::Fenix => writeln!(f, "fenix"),
            GarminProduct::Edge200Taiwan => writeln!(f, "edge200_taiwan"),
            GarminProduct::Edge510 => writeln!(f, "edge510"),
            GarminProduct::Edge810 => writeln!(f, "edge810"),
            GarminProduct::Tempe => writeln!(f, "tempe"),
            GarminProduct::Fr910xtJapan => writeln!(f, "fr910xt_japan"),
            GarminProduct::Fr620 => writeln!(f, "fr620"),
            GarminProduct::Fr220 => writeln!(f, "fr220"),
            GarminProduct::Fr910xtKorea => writeln!(f, "fr910xt_korea"),
            GarminProduct::Fr10Japan => writeln!(f, "fr10_japan"),
            GarminProduct::Edge810Japan => writeln!(f, "edge810_japan"),
            GarminProduct::VirbElite => writeln!(f, "virb_elite"),
            GarminProduct::EdgeTouring => writeln!(f, "edge_touring"),
            GarminProduct::Edge510Japan => writeln!(f, "edge510_japan"),
            GarminProduct::HrmTri => writeln!(f, "hrm_tri"),
            GarminProduct::HrmRun => writeln!(f, "hrm_run"),
            GarminProduct::Fr920xt => writeln!(f, "fr920xt"),
            GarminProduct::Edge510Asia => writeln!(f, "edge510_asia"),
            GarminProduct::Edge810China => writeln!(f, "edge810_china"),
            GarminProduct::Edge810Taiwan => writeln!(f, "edge810_taiwan"),
            GarminProduct::Edge1000 => writeln!(f, "edge1000"),
            GarminProduct::VivoFit => writeln!(f, "vivo_fit"),
            GarminProduct::VirbRemote => writeln!(f, "virb_remote"),
            GarminProduct::VivoKi => writeln!(f, "vivo_ki"),
            GarminProduct::Fr15 => writeln!(f, "fr15"),
            GarminProduct::VivoActive => writeln!(f, "vivo_active"),
            GarminProduct::Edge510Korea => writeln!(f, "edge510_korea"),
            GarminProduct::Fr620Japan => writeln!(f, "fr620_japan"),
            GarminProduct::Fr620China => writeln!(f, "fr620_china"),
            GarminProduct::Fr220Japan => writeln!(f, "fr220_japan"),
            GarminProduct::Fr220China => writeln!(f, "fr220_china"),
            GarminProduct::ApproachS6 => writeln!(f, "approach_s6"),
            GarminProduct::VivoSmart => writeln!(f, "vivo_smart"),
            GarminProduct::Fenix2 => writeln!(f, "fenix2"),
            GarminProduct::Epix => writeln!(f, "epix"),
            GarminProduct::Fenix3 => writeln!(f, "fenix3"),
            GarminProduct::Edge1000Taiwan => writeln!(f, "edge1000_taiwan"),
            GarminProduct::Edge1000Japan => writeln!(f, "edge1000_japan"),
            GarminProduct::Fr15Japan => writeln!(f, "fr15_japan"),
            GarminProduct::Edge520 => writeln!(f, "edge520"),
            GarminProduct::Edge1000China => writeln!(f, "edge1000_china"),
            GarminProduct::Fr620Russia => writeln!(f, "fr620_russia"),
            GarminProduct::Fr220Russia => writeln!(f, "fr220_russia"),
            GarminProduct::VectorS => writeln!(f, "vector_s"),
            GarminProduct::Edge1000Korea => writeln!(f, "edge1000_korea"),
            GarminProduct::Fr920xtTaiwan => writeln!(f, "fr920xt_taiwan"),
            GarminProduct::Fr920xtChina => writeln!(f, "fr920xt_china"),
            GarminProduct::Fr920xtJapan => writeln!(f, "fr920xt_japan"),
            GarminProduct::Virbx => writeln!(f, "virbx"),
            GarminProduct::VivoSmartApac => writeln!(f, "vivo_smart_apac"),
            GarminProduct::EtrexTouch => writeln!(f, "etrex_touch"),
            GarminProduct::Edge25 => writeln!(f, "edge25"),
            GarminProduct::Fr25 => writeln!(f, "fr25"),
            GarminProduct::VivoFit2 => writeln!(f, "vivo_fit2"),
            GarminProduct::Fr225 => writeln!(f, "fr225"),
            GarminProduct::Fr630 => writeln!(f, "fr630"),
            GarminProduct::Fr230 => writeln!(f, "fr230"),
            GarminProduct::Fr735xt => writeln!(f, "fr735xt"),
            GarminProduct::VivoActiveApac => writeln!(f, "vivo_active_apac"),
            GarminProduct::Vector2 => writeln!(f, "vector_2"),
            GarminProduct::Vector2s => writeln!(f, "vector_2s"),
            GarminProduct::Virbxe => writeln!(f, "virbxe"),
            GarminProduct::Fr620Taiwan => writeln!(f, "fr620_taiwan"),
            GarminProduct::Fr220Taiwan => writeln!(f, "fr220_taiwan"),
            GarminProduct::Truswing => writeln!(f, "truswing"),
            GarminProduct::Fenix3China => writeln!(f, "fenix3_china"),
            GarminProduct::Fenix3Twn => writeln!(f, "fenix3_twn"),
            GarminProduct::VariaHeadlight => writeln!(f, "varia_headlight"),
            GarminProduct::VariaTaillightOld => writeln!(f, "varia_taillight_old"),
            GarminProduct::EdgeExplore1000 => writeln!(f, "edge_explore_1000"),
            GarminProduct::Fr225Asia => writeln!(f, "fr225_asia"),
            GarminProduct::VariaRadarTaillight => writeln!(f, "varia_radar_taillight"),
            GarminProduct::VariaRadarDisplay => writeln!(f, "varia_radar_display"),
            GarminProduct::Edge20 => writeln!(f, "edge20"),
            GarminProduct::D2Bravo => writeln!(f, "d2_bravo"),
            GarminProduct::ApproachS20 => writeln!(f, "approach_s20"),
            GarminProduct::VariaRemote => writeln!(f, "varia_remote"),
            GarminProduct::ApproachX40 => writeln!(f, "approach_x40"),
            GarminProduct::Hrm4Run => writeln!(f, "hrm4_run"),
            GarminProduct::VivoActiveHr => writeln!(f, "vivo_active_hr"),
            GarminProduct::VivoSmartGpsHr => writeln!(f, "vivo_smart_gps_hr"),
            GarminProduct::VivoSmartHr => writeln!(f, "vivo_smart_hr"),
            GarminProduct::VivoMove => writeln!(f, "vivo_move"),
            GarminProduct::VariaVision => writeln!(f, "varia_vision"),
            GarminProduct::VivoFit3 => writeln!(f, "vivo_fit3"),
            GarminProduct::Fenix3Hr => writeln!(f, "fenix3_hr"),
            GarminProduct::VirbUltra30 => writeln!(f, "virb_ultra_30"),
            GarminProduct::IndexSmartScale => writeln!(f, "index_smart_scale"),
            GarminProduct::Fr235 => writeln!(f, "fr235"),
            GarminProduct::Fenix3Chronos => writeln!(f, "fenix3_chronos"),
            GarminProduct::Oregon7xx => writeln!(f, "oregon7xx"),
            GarminProduct::Rino7xx => writeln!(f, "rino7xx"),
            GarminProduct::Nautix => writeln!(f, "nautix"),
            GarminProduct::Edge820 => writeln!(f, "edge_820"),
            GarminProduct::EdgeExplore820 => writeln!(f, "edge_explore_820"),
            GarminProduct::Fr735xtApac => writeln!(f, "fr735xt_apac"),
            GarminProduct::Fr735xtJapan => writeln!(f, "fr735xt_japan"),
            GarminProduct::Fenix5s => writeln!(f, "fenix5s"),
            GarminProduct::D2BravoTitanium => writeln!(f, "d2_bravo_titanium"),
            GarminProduct::VariaUt800 => writeln!(f, "varia_ut800"),
            GarminProduct::RunningDynamicsPod => writeln!(f, "running_dynamics_pod"),
            GarminProduct::Fenix5x => writeln!(f, "fenix5x"),
            GarminProduct::VivoFitJr => writeln!(f, "vivo_fit_jr"),
            GarminProduct::VivoSmart3 => writeln!(f, "vivo_smart3"),
            GarminProduct::VivoSport => writeln!(f, "vivo_sport"),
            GarminProduct::ApproachS60 => writeln!(f, "approach_s60"),
            GarminProduct::Virb360 => writeln!(f, "virb_360"),
            GarminProduct::Fr935 => writeln!(f, "fr935"),
            GarminProduct::Fenix5 => writeln!(f, "fenix5"),
            GarminProduct::Vivoactive3 => writeln!(f, "vivoactive3"),
            GarminProduct::Edge1030 => writeln!(f, "edge_1030"),
            GarminProduct::Foretrex601701 => writeln!(f, "foretrex_601_701"),
            GarminProduct::VivoMoveHr => writeln!(f, "vivo_move_hr"),
            GarminProduct::ApproachZ80 => writeln!(f, "approach_z80"),
            GarminProduct::VivoSmart3Apac => writeln!(f, "vivo_smart3_apac"),
            GarminProduct::VivoSportApac => writeln!(f, "vivo_sport_apac"),
            GarminProduct::Descent => writeln!(f, "descent"),
            GarminProduct::Fr645 => writeln!(f, "fr645"),
            GarminProduct::Fr645m => writeln!(f, "fr645m"),
            GarminProduct::Fenix5sPlus => writeln!(f, "fenix5s_plus"),
            GarminProduct::Edge130 => writeln!(f, "Edge_130"),
            GarminProduct::Vivosmart4 => writeln!(f, "vivosmart_4"),
            GarminProduct::ApproachX10 => writeln!(f, "approach_x10"),
            GarminProduct::Vivoactive3mW => writeln!(f, "vivoactive3m_w"),
            GarminProduct::EdgeExplore => writeln!(f, "edge_explore"),
            GarminProduct::Gpsmap66 => writeln!(f, "gpsmap66"),
            GarminProduct::ApproachS10 => writeln!(f, "approach_s10"),
            GarminProduct::Vivoactive3mL => writeln!(f, "vivoactive3m_l"),
            GarminProduct::ApproachG80 => writeln!(f, "approach_g80"),
            GarminProduct::Fenix5Plus => writeln!(f, "fenix5_plus"),
            GarminProduct::Fenix5xPlus => writeln!(f, "fenix5x_plus"),
            GarminProduct::Edge520Plus => writeln!(f, "edge_520_plus"),
            GarminProduct::HrmDual => writeln!(f, "hrm_dual"),
            GarminProduct::ApproachS40 => writeln!(f, "approach_s40"),
            GarminProduct::Sdm4 => writeln!(f, "sdm4"),
            GarminProduct::EdgeRemote => writeln!(f, "edge_remote"),
            GarminProduct::TrainingCenter => writeln!(f, "training_center"),
            GarminProduct::ConnectiqSimulator => writeln!(f, "connectiq_simulator"),
            GarminProduct::AndroidAntplusPlugin => writeln!(f, "android_antplus_plugin"),
            GarminProduct::Connect => writeln!(f, "connect"),
            GarminProduct::UnknownVariant(value) => writeln!(f, "unknown_variant_{}", *value),
        }
    }
}
impl convert::From<u16> for GarminProduct {
    fn from(value: u16) -> Self {
        match value {
            1 => GarminProduct::Hrm1,
            2 => GarminProduct::Axh01,
            3 => GarminProduct::Axb01,
            4 => GarminProduct::Axb02,
            5 => GarminProduct::Hrm2ss,
            6 => GarminProduct::DsiAlf02,
            7 => GarminProduct::Hrm3ss,
            8 => GarminProduct::HrmRunSingleByteProductId,
            9 => GarminProduct::Bsm,
            10 => GarminProduct::Bcm,
            11 => GarminProduct::Axs01,
            12 => GarminProduct::HrmTriSingleByteProductId,
            14 => GarminProduct::Fr225SingleByteProductId,
            473 => GarminProduct::Fr301China,
            474 => GarminProduct::Fr301Japan,
            475 => GarminProduct::Fr301Korea,
            494 => GarminProduct::Fr301Taiwan,
            717 => GarminProduct::Fr405,
            782 => GarminProduct::Fr50,
            987 => GarminProduct::Fr405Japan,
            988 => GarminProduct::Fr60,
            1011 => GarminProduct::DsiAlf01,
            1018 => GarminProduct::Fr310xt,
            1036 => GarminProduct::Edge500,
            1124 => GarminProduct::Fr110,
            1169 => GarminProduct::Edge800,
            1199 => GarminProduct::Edge500Taiwan,
            1213 => GarminProduct::Edge500Japan,
            1253 => GarminProduct::Chirp,
            1274 => GarminProduct::Fr110Japan,
            1325 => GarminProduct::Edge200,
            1328 => GarminProduct::Fr910xt,
            1333 => GarminProduct::Edge800Taiwan,
            1334 => GarminProduct::Edge800Japan,
            1341 => GarminProduct::Alf04,
            1345 => GarminProduct::Fr610,
            1360 => GarminProduct::Fr210Japan,
            1380 => GarminProduct::VectorSs,
            1381 => GarminProduct::VectorCp,
            1386 => GarminProduct::Edge800China,
            1387 => GarminProduct::Edge500China,
            1410 => GarminProduct::Fr610Japan,
            1422 => GarminProduct::Edge500Korea,
            1436 => GarminProduct::Fr70,
            1446 => GarminProduct::Fr310xt4t,
            1461 => GarminProduct::Amx,
            1482 => GarminProduct::Fr10,
            1497 => GarminProduct::Edge800Korea,
            1499 => GarminProduct::Swim,
            1537 => GarminProduct::Fr910xtChina,
            1551 => GarminProduct::Fenix,
            1555 => GarminProduct::Edge200Taiwan,
            1561 => GarminProduct::Edge510,
            1567 => GarminProduct::Edge810,
            1570 => GarminProduct::Tempe,
            1600 => GarminProduct::Fr910xtJapan,
            1623 => GarminProduct::Fr620,
            1632 => GarminProduct::Fr220,
            1664 => GarminProduct::Fr910xtKorea,
            1688 => GarminProduct::Fr10Japan,
            1721 => GarminProduct::Edge810Japan,
            1735 => GarminProduct::VirbElite,
            1736 => GarminProduct::EdgeTouring,
            1742 => GarminProduct::Edge510Japan,
            1743 => GarminProduct::HrmTri,
            1752 => GarminProduct::HrmRun,
            1765 => GarminProduct::Fr920xt,
            1821 => GarminProduct::Edge510Asia,
            1822 => GarminProduct::Edge810China,
            1823 => GarminProduct::Edge810Taiwan,
            1836 => GarminProduct::Edge1000,
            1837 => GarminProduct::VivoFit,
            1853 => GarminProduct::VirbRemote,
            1885 => GarminProduct::VivoKi,
            1903 => GarminProduct::Fr15,
            1907 => GarminProduct::VivoActive,
            1918 => GarminProduct::Edge510Korea,
            1928 => GarminProduct::Fr620Japan,
            1929 => GarminProduct::Fr620China,
            1930 => GarminProduct::Fr220Japan,
            1931 => GarminProduct::Fr220China,
            1936 => GarminProduct::ApproachS6,
            1956 => GarminProduct::VivoSmart,
            1967 => GarminProduct::Fenix2,
            1988 => GarminProduct::Epix,
            2050 => GarminProduct::Fenix3,
            2052 => GarminProduct::Edge1000Taiwan,
            2053 => GarminProduct::Edge1000Japan,
            2061 => GarminProduct::Fr15Japan,
            2067 => GarminProduct::Edge520,
            2070 => GarminProduct::Edge1000China,
            2072 => GarminProduct::Fr620Russia,
            2073 => GarminProduct::Fr220Russia,
            2079 => GarminProduct::VectorS,
            2100 => GarminProduct::Edge1000Korea,
            2130 => GarminProduct::Fr920xtTaiwan,
            2131 => GarminProduct::Fr920xtChina,
            2132 => GarminProduct::Fr920xtJapan,
            2134 => GarminProduct::Virbx,
            2135 => GarminProduct::VivoSmartApac,
            2140 => GarminProduct::EtrexTouch,
            2147 => GarminProduct::Edge25,
            2148 => GarminProduct::Fr25,
            2150 => GarminProduct::VivoFit2,
            2153 => GarminProduct::Fr225,
            2156 => GarminProduct::Fr630,
            2157 => GarminProduct::Fr230,
            2158 => GarminProduct::Fr735xt,
            2160 => GarminProduct::VivoActiveApac,
            2161 => GarminProduct::Vector2,
            2162 => GarminProduct::Vector2s,
            2172 => GarminProduct::Virbxe,
            2173 => GarminProduct::Fr620Taiwan,
            2174 => GarminProduct::Fr220Taiwan,
            2175 => GarminProduct::Truswing,
            2188 => GarminProduct::Fenix3China,
            2189 => GarminProduct::Fenix3Twn,
            2192 => GarminProduct::VariaHeadlight,
            2193 => GarminProduct::VariaTaillightOld,
            2204 => GarminProduct::EdgeExplore1000,
            2219 => GarminProduct::Fr225Asia,
            2225 => GarminProduct::VariaRadarTaillight,
            2226 => GarminProduct::VariaRadarDisplay,
            2238 => GarminProduct::Edge20,
            2262 => GarminProduct::D2Bravo,
            2266 => GarminProduct::ApproachS20,
            2276 => GarminProduct::VariaRemote,
            2292 => GarminProduct::ApproachX40,
            2327 => GarminProduct::Hrm4Run,
            2337 => GarminProduct::VivoActiveHr,
            2347 => GarminProduct::VivoSmartGpsHr,
            2348 => GarminProduct::VivoSmartHr,
            2368 => GarminProduct::VivoMove,
            2398 => GarminProduct::VariaVision,
            2406 => GarminProduct::VivoFit3,
            2413 => GarminProduct::Fenix3Hr,
            2417 => GarminProduct::VirbUltra30,
            2429 => GarminProduct::IndexSmartScale,
            2431 => GarminProduct::Fr235,
            2432 => GarminProduct::Fenix3Chronos,
            2441 => GarminProduct::Oregon7xx,
            2444 => GarminProduct::Rino7xx,
            2496 => GarminProduct::Nautix,
            2530 => GarminProduct::Edge820,
            2531 => GarminProduct::EdgeExplore820,
            2533 => GarminProduct::Fr735xtApac,
            2534 => GarminProduct::Fr735xtJapan,
            2544 => GarminProduct::Fenix5s,
            2547 => GarminProduct::D2BravoTitanium,
            2567 => GarminProduct::VariaUt800,
            2593 => GarminProduct::RunningDynamicsPod,
            2604 => GarminProduct::Fenix5x,
            2606 => GarminProduct::VivoFitJr,
            2622 => GarminProduct::VivoSmart3,
            2623 => GarminProduct::VivoSport,
            2656 => GarminProduct::ApproachS60,
            2687 => GarminProduct::Virb360,
            2691 => GarminProduct::Fr935,
            2697 => GarminProduct::Fenix5,
            2700 => GarminProduct::Vivoactive3,
            2713 => GarminProduct::Edge1030,
            2769 => GarminProduct::Foretrex601701,
            2772 => GarminProduct::VivoMoveHr,
            2806 => GarminProduct::ApproachZ80,
            2831 => GarminProduct::VivoSmart3Apac,
            2832 => GarminProduct::VivoSportApac,
            2859 => GarminProduct::Descent,
            2886 => GarminProduct::Fr645,
            2888 => GarminProduct::Fr645m,
            2900 => GarminProduct::Fenix5sPlus,
            2909 => GarminProduct::Edge130,
            2927 => GarminProduct::Vivosmart4,
            2962 => GarminProduct::ApproachX10,
            2988 => GarminProduct::Vivoactive3mW,
            3011 => GarminProduct::EdgeExplore,
            3028 => GarminProduct::Gpsmap66,
            3049 => GarminProduct::ApproachS10,
            3066 => GarminProduct::Vivoactive3mL,
            3085 => GarminProduct::ApproachG80,
            3110 => GarminProduct::Fenix5Plus,
            3111 => GarminProduct::Fenix5xPlus,
            3112 => GarminProduct::Edge520Plus,
            3299 => GarminProduct::HrmDual,
            3314 => GarminProduct::ApproachS40,
            10007 => GarminProduct::Sdm4,
            10014 => GarminProduct::EdgeRemote,
            20119 => GarminProduct::TrainingCenter,
            65531 => GarminProduct::ConnectiqSimulator,
            65532 => GarminProduct::AndroidAntplusPlugin,
            65534 => GarminProduct::Connect,
            _ => GarminProduct::UnknownVariant(value),
        }
    }
}
impl convert::From<i64> for GarminProduct {
    fn from(value: i64) -> Self {
        GarminProduct::from(value as u16)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum AntplusDeviceType {
    Antfs,
    BikePower,
    EnvironmentSensorLegacy,
    MultiSportSpeedDistance,
    Control,
    FitnessEquipment,
    BloodPressure,
    GeocacheNode,
    LightElectricVehicle,
    EnvSensor,
    Racquet,
    ControlHub,
    MuscleOxygen,
    BikeLightMain,
    BikeLightShared,
    Exd,
    BikeRadar,
    BikeAero,
    WeightScale,
    HeartRate,
    BikeSpeedCadence,
    BikeCadence,
    BikeSpeed,
    StrideSpeedDistance,
    UnknownVariant(u8),
}
impl AntplusDeviceType {
    pub fn as_u8(self) -> u8 {
        match self {
            AntplusDeviceType::Antfs => 1,
            AntplusDeviceType::BikePower => 11,
            AntplusDeviceType::EnvironmentSensorLegacy => 12,
            AntplusDeviceType::MultiSportSpeedDistance => 15,
            AntplusDeviceType::Control => 16,
            AntplusDeviceType::FitnessEquipment => 17,
            AntplusDeviceType::BloodPressure => 18,
            AntplusDeviceType::GeocacheNode => 19,
            AntplusDeviceType::LightElectricVehicle => 20,
            AntplusDeviceType::EnvSensor => 25,
            AntplusDeviceType::Racquet => 26,
            AntplusDeviceType::ControlHub => 27,
            AntplusDeviceType::MuscleOxygen => 31,
            AntplusDeviceType::BikeLightMain => 35,
            AntplusDeviceType::BikeLightShared => 36,
            AntplusDeviceType::Exd => 38,
            AntplusDeviceType::BikeRadar => 40,
            AntplusDeviceType::BikeAero => 46,
            AntplusDeviceType::WeightScale => 119,
            AntplusDeviceType::HeartRate => 120,
            AntplusDeviceType::BikeSpeedCadence => 121,
            AntplusDeviceType::BikeCadence => 122,
            AntplusDeviceType::BikeSpeed => 123,
            AntplusDeviceType::StrideSpeedDistance => 124,
            AntplusDeviceType::UnknownVariant(value) => value,
        }
    }
    pub fn as_i64(self) -> i64 {
        self.as_u8() as i64
    }
}
impl fmt::Display for AntplusDeviceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            AntplusDeviceType::Antfs => writeln!(f, "antfs"),
            AntplusDeviceType::BikePower => writeln!(f, "bike_power"),
            AntplusDeviceType::EnvironmentSensorLegacy => writeln!(f, "environment_sensor_legacy"),
            AntplusDeviceType::MultiSportSpeedDistance => writeln!(f, "multi_sport_speed_distance"),
            AntplusDeviceType::Control => writeln!(f, "control"),
            AntplusDeviceType::FitnessEquipment => writeln!(f, "fitness_equipment"),
            AntplusDeviceType::BloodPressure => writeln!(f, "blood_pressure"),
            AntplusDeviceType::GeocacheNode => writeln!(f, "geocache_node"),
            AntplusDeviceType::LightElectricVehicle => writeln!(f, "light_electric_vehicle"),
            AntplusDeviceType::EnvSensor => writeln!(f, "env_sensor"),
            AntplusDeviceType::Racquet => writeln!(f, "racquet"),
            AntplusDeviceType::ControlHub => writeln!(f, "control_hub"),
            AntplusDeviceType::MuscleOxygen => writeln!(f, "muscle_oxygen"),
            AntplusDeviceType::BikeLightMain => writeln!(f, "bike_light_main"),
            AntplusDeviceType::BikeLightShared => writeln!(f, "bike_light_shared"),
            AntplusDeviceType::Exd => writeln!(f, "exd"),
            AntplusDeviceType::BikeRadar => writeln!(f, "bike_radar"),
            AntplusDeviceType::BikeAero => writeln!(f, "bike_aero"),
            AntplusDeviceType::WeightScale => writeln!(f, "weight_scale"),
            AntplusDeviceType::HeartRate => writeln!(f, "heart_rate"),
            AntplusDeviceType::BikeSpeedCadence => writeln!(f, "bike_speed_cadence"),
            AntplusDeviceType::BikeCadence => writeln!(f, "bike_cadence"),
            AntplusDeviceType::BikeSpeed => writeln!(f, "bike_speed"),
            AntplusDeviceType::StrideSpeedDistance => writeln!(f, "stride_speed_distance"),
            AntplusDeviceType::UnknownVariant(value) => writeln!(f, "unknown_variant_{}", *value),
        }
    }
}
impl convert::From<u8> for AntplusDeviceType {
    fn from(value: u8) -> Self {
        match value {
            1 => AntplusDeviceType::Antfs,
            11 => AntplusDeviceType::BikePower,
            12 => AntplusDeviceType::EnvironmentSensorLegacy,
            15 => AntplusDeviceType::MultiSportSpeedDistance,
            16 => AntplusDeviceType::Control,
            17 => AntplusDeviceType::FitnessEquipment,
            18 => AntplusDeviceType::BloodPressure,
            19 => AntplusDeviceType::GeocacheNode,
            20 => AntplusDeviceType::LightElectricVehicle,
            25 => AntplusDeviceType::EnvSensor,
            26 => AntplusDeviceType::Racquet,
            27 => AntplusDeviceType::ControlHub,
            31 => AntplusDeviceType::MuscleOxygen,
            35 => AntplusDeviceType::BikeLightMain,
            36 => AntplusDeviceType::BikeLightShared,
            38 => AntplusDeviceType::Exd,
            40 => AntplusDeviceType::BikeRadar,
            46 => AntplusDeviceType::BikeAero,
            119 => AntplusDeviceType::WeightScale,
            120 => AntplusDeviceType::HeartRate,
            121 => AntplusDeviceType::BikeSpeedCadence,
            122 => AntplusDeviceType::BikeCadence,
            123 => AntplusDeviceType::BikeSpeed,
            124 => AntplusDeviceType::StrideSpeedDistance,
            _ => AntplusDeviceType::UnknownVariant(value),
        }
    }
}
impl convert::From<i64> for AntplusDeviceType {
    fn from(value: i64) -> Self {
        AntplusDeviceType::from(value as u8)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum AntNetwork {
    Public,
    Antplus,
    Antfs,
    Private,
    UnknownVariant(u8),
}
impl AntNetwork {
    pub fn as_u8(self) -> u8 {
        match self {
            AntNetwork::Public => 0,
            AntNetwork::Antplus => 1,
            AntNetwork::Antfs => 2,
            AntNetwork::Private => 3,
            AntNetwork::UnknownVariant(value) => value,
        }
    }
    pub fn as_i64(self) -> i64 {
        self.as_u8() as i64
    }
}
impl fmt::Display for AntNetwork {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            AntNetwork::Public => writeln!(f, "public"),
            AntNetwork::Antplus => writeln!(f, "antplus"),
            AntNetwork::Antfs => writeln!(f, "antfs"),
            AntNetwork::Private => writeln!(f, "private"),
            AntNetwork::UnknownVariant(value) => writeln!(f, "unknown_variant_{}", *value),
        }
    }
}
impl convert::From<u8> for AntNetwork {
    fn from(value: u8) -> Self {
        match value {
            0 => AntNetwork::Public,
            1 => AntNetwork::Antplus,
            2 => AntNetwork::Antfs,
            3 => AntNetwork::Private,
            _ => AntNetwork::UnknownVariant(value),
        }
    }
}
impl convert::From<i64> for AntNetwork {
    fn from(value: i64) -> Self {
        AntNetwork::from(value as u8)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum WorkoutCapabilities {
    Interval,
    Custom,
    FitnessEquipment,
    Firstbeat,
    NewLeaf,
    /// For backwards compatibility.  Watch should add missing id fields then clear flag.
    Tcx,
    /// Speed source required for workout step.
    Speed,
    /// Heart rate source required for workout step.
    HeartRate,
    /// Distance source required for workout step.
    Distance,
    /// Cadence source required for workout step.
    Cadence,
    /// Power source required for workout step.
    Power,
    /// Grade source required for workout step.
    Grade,
    /// Resistance source required for workout step.
    Resistance,
    Protected,
    UnknownVariant(u32),
}
impl WorkoutCapabilities {
    pub fn as_u32(self) -> u32 {
        match self {
            WorkoutCapabilities::Interval => 1,
            WorkoutCapabilities::Custom => 2,
            WorkoutCapabilities::FitnessEquipment => 4,
            WorkoutCapabilities::Firstbeat => 8,
            WorkoutCapabilities::NewLeaf => 16,
            WorkoutCapabilities::Tcx => 32,
            WorkoutCapabilities::Speed => 128,
            WorkoutCapabilities::HeartRate => 256,
            WorkoutCapabilities::Distance => 512,
            WorkoutCapabilities::Cadence => 1024,
            WorkoutCapabilities::Power => 2048,
            WorkoutCapabilities::Grade => 4096,
            WorkoutCapabilities::Resistance => 8192,
            WorkoutCapabilities::Protected => 16384,
            WorkoutCapabilities::UnknownVariant(value) => value,
        }
    }
    pub fn as_i64(self) -> i64 {
        self.as_u32() as i64
    }
}
impl fmt::Display for WorkoutCapabilities {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            WorkoutCapabilities::Interval => writeln!(f, "interval"),
            WorkoutCapabilities::Custom => writeln!(f, "custom"),
            WorkoutCapabilities::FitnessEquipment => writeln!(f, "fitness_equipment"),
            WorkoutCapabilities::Firstbeat => writeln!(f, "firstbeat"),
            WorkoutCapabilities::NewLeaf => writeln!(f, "new_leaf"),
            WorkoutCapabilities::Tcx => writeln!(f, "tcx"),
            WorkoutCapabilities::Speed => writeln!(f, "speed"),
            WorkoutCapabilities::HeartRate => writeln!(f, "heart_rate"),
            WorkoutCapabilities::Distance => writeln!(f, "distance"),
            WorkoutCapabilities::Cadence => writeln!(f, "cadence"),
            WorkoutCapabilities::Power => writeln!(f, "power"),
            WorkoutCapabilities::Grade => writeln!(f, "grade"),
            WorkoutCapabilities::Resistance => writeln!(f, "resistance"),
            WorkoutCapabilities::Protected => writeln!(f, "protected"),
            WorkoutCapabilities::UnknownVariant(value) => writeln!(f, "unknown_variant_{}", *value),
        }
    }
}
impl convert::From<u32> for WorkoutCapabilities {
    fn from(value: u32) -> Self {
        match value {
            1 => WorkoutCapabilities::Interval,
            2 => WorkoutCapabilities::Custom,
            4 => WorkoutCapabilities::FitnessEquipment,
            8 => WorkoutCapabilities::Firstbeat,
            16 => WorkoutCapabilities::NewLeaf,
            32 => WorkoutCapabilities::Tcx,
            128 => WorkoutCapabilities::Speed,
            256 => WorkoutCapabilities::HeartRate,
            512 => WorkoutCapabilities::Distance,
            1024 => WorkoutCapabilities::Cadence,
            2048 => WorkoutCapabilities::Power,
            4096 => WorkoutCapabilities::Grade,
            8192 => WorkoutCapabilities::Resistance,
            16384 => WorkoutCapabilities::Protected,
            _ => WorkoutCapabilities::UnknownVariant(value),
        }
    }
}
impl convert::From<i64> for WorkoutCapabilities {
    fn from(value: i64) -> Self {
        WorkoutCapabilities::from(value as u32)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum BatteryStatus {
    New,
    Good,
    Ok,
    Low,
    Critical,
    Charging,
    Unknown,
    UnknownVariant(u8),
}
impl BatteryStatus {
    pub fn as_u8(self) -> u8 {
        match self {
            BatteryStatus::New => 1,
            BatteryStatus::Good => 2,
            BatteryStatus::Ok => 3,
            BatteryStatus::Low => 4,
            BatteryStatus::Critical => 5,
            BatteryStatus::Charging => 6,
            BatteryStatus::Unknown => 7,
            BatteryStatus::UnknownVariant(value) => value,
        }
    }
    pub fn as_i64(self) -> i64 {
        self.as_u8() as i64
    }
}
impl fmt::Display for BatteryStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            BatteryStatus::New => writeln!(f, "new"),
            BatteryStatus::Good => writeln!(f, "good"),
            BatteryStatus::Ok => writeln!(f, "ok"),
            BatteryStatus::Low => writeln!(f, "low"),
            BatteryStatus::Critical => writeln!(f, "critical"),
            BatteryStatus::Charging => writeln!(f, "charging"),
            BatteryStatus::Unknown => writeln!(f, "unknown"),
            BatteryStatus::UnknownVariant(value) => writeln!(f, "unknown_variant_{}", *value),
        }
    }
}
impl convert::From<u8> for BatteryStatus {
    fn from(value: u8) -> Self {
        match value {
            1 => BatteryStatus::New,
            2 => BatteryStatus::Good,
            3 => BatteryStatus::Ok,
            4 => BatteryStatus::Low,
            5 => BatteryStatus::Critical,
            6 => BatteryStatus::Charging,
            7 => BatteryStatus::Unknown,
            _ => BatteryStatus::UnknownVariant(value),
        }
    }
}
impl convert::From<i64> for BatteryStatus {
    fn from(value: i64) -> Self {
        BatteryStatus::from(value as u8)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum HrType {
    Normal,
    Irregular,
    UnknownVariant(u8),
}
impl HrType {
    pub fn as_u8(self) -> u8 {
        match self {
            HrType::Normal => 0,
            HrType::Irregular => 1,
            HrType::UnknownVariant(value) => value,
        }
    }
    pub fn as_i64(self) -> i64 {
        self.as_u8() as i64
    }
}
impl fmt::Display for HrType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            HrType::Normal => writeln!(f, "normal"),
            HrType::Irregular => writeln!(f, "irregular"),
            HrType::UnknownVariant(value) => writeln!(f, "unknown_variant_{}", *value),
        }
    }
}
impl convert::From<u8> for HrType {
    fn from(value: u8) -> Self {
        match value {
            0 => HrType::Normal,
            1 => HrType::Irregular,
            _ => HrType::UnknownVariant(value),
        }
    }
}
impl convert::From<i64> for HrType {
    fn from(value: i64) -> Self {
        HrType::from(value as u8)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum CourseCapabilities {
    Processed,
    Valid,
    Time,
    Distance,
    Position,
    HeartRate,
    Power,
    Cadence,
    Training,
    Navigation,
    Bikeway,
    UnknownVariant(u32),
}
impl CourseCapabilities {
    pub fn as_u32(self) -> u32 {
        match self {
            CourseCapabilities::Processed => 1,
            CourseCapabilities::Valid => 2,
            CourseCapabilities::Time => 4,
            CourseCapabilities::Distance => 8,
            CourseCapabilities::Position => 16,
            CourseCapabilities::HeartRate => 32,
            CourseCapabilities::Power => 64,
            CourseCapabilities::Cadence => 128,
            CourseCapabilities::Training => 256,
            CourseCapabilities::Navigation => 512,
            CourseCapabilities::Bikeway => 1024,
            CourseCapabilities::UnknownVariant(value) => value,
        }
    }
    pub fn as_i64(self) -> i64 {
        self.as_u32() as i64
    }
}
impl fmt::Display for CourseCapabilities {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            CourseCapabilities::Processed => writeln!(f, "processed"),
            CourseCapabilities::Valid => writeln!(f, "valid"),
            CourseCapabilities::Time => writeln!(f, "time"),
            CourseCapabilities::Distance => writeln!(f, "distance"),
            CourseCapabilities::Position => writeln!(f, "position"),
            CourseCapabilities::HeartRate => writeln!(f, "heart_rate"),
            CourseCapabilities::Power => writeln!(f, "power"),
            CourseCapabilities::Cadence => writeln!(f, "cadence"),
            CourseCapabilities::Training => writeln!(f, "training"),
            CourseCapabilities::Navigation => writeln!(f, "navigation"),
            CourseCapabilities::Bikeway => writeln!(f, "bikeway"),
            CourseCapabilities::UnknownVariant(value) => writeln!(f, "unknown_variant_{}", *value),
        }
    }
}
impl convert::From<u32> for CourseCapabilities {
    fn from(value: u32) -> Self {
        match value {
            1 => CourseCapabilities::Processed,
            2 => CourseCapabilities::Valid,
            4 => CourseCapabilities::Time,
            8 => CourseCapabilities::Distance,
            16 => CourseCapabilities::Position,
            32 => CourseCapabilities::HeartRate,
            64 => CourseCapabilities::Power,
            128 => CourseCapabilities::Cadence,
            256 => CourseCapabilities::Training,
            512 => CourseCapabilities::Navigation,
            1024 => CourseCapabilities::Bikeway,
            _ => CourseCapabilities::UnknownVariant(value),
        }
    }
}
impl convert::From<i64> for CourseCapabilities {
    fn from(value: i64) -> Self {
        CourseCapabilities::from(value as u32)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum Weight {
    Calculating,
    UnknownVariant(u16),
}
impl Weight {
    pub fn as_u16(self) -> u16 {
        match self {
            Weight::Calculating => 65534,
            Weight::UnknownVariant(value) => value,
        }
    }
    pub fn as_i64(self) -> i64 {
        self.as_u16() as i64
    }
}
impl fmt::Display for Weight {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Weight::Calculating => writeln!(f, "calculating"),
            Weight::UnknownVariant(value) => writeln!(f, "unknown_variant_{}", *value),
        }
    }
}
impl convert::From<u16> for Weight {
    fn from(value: u16) -> Self {
        match value {
            65534 => Weight::Calculating,
            _ => Weight::UnknownVariant(value),
        }
    }
}
impl convert::From<i64> for Weight {
    fn from(value: i64) -> Self {
        Weight::from(value as u16)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum WorkoutHr {
    BpmOffset,
    UnknownVariant(u32),
}
impl WorkoutHr {
    pub fn as_u32(self) -> u32 {
        match self {
            WorkoutHr::BpmOffset => 100,
            WorkoutHr::UnknownVariant(value) => value,
        }
    }
    pub fn as_i64(self) -> i64 {
        self.as_u32() as i64
    }
}
impl fmt::Display for WorkoutHr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            WorkoutHr::BpmOffset => writeln!(f, "bpm_offset"),
            WorkoutHr::UnknownVariant(value) => writeln!(f, "unknown_variant_{}", *value),
        }
    }
}
impl convert::From<u32> for WorkoutHr {
    fn from(value: u32) -> Self {
        match value {
            100 => WorkoutHr::BpmOffset,
            _ => WorkoutHr::UnknownVariant(value),
        }
    }
}
impl convert::From<i64> for WorkoutHr {
    fn from(value: i64) -> Self {
        WorkoutHr::from(value as u32)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum WorkoutPower {
    WattsOffset,
    UnknownVariant(u32),
}
impl WorkoutPower {
    pub fn as_u32(self) -> u32 {
        match self {
            WorkoutPower::WattsOffset => 1000,
            WorkoutPower::UnknownVariant(value) => value,
        }
    }
    pub fn as_i64(self) -> i64 {
        self.as_u32() as i64
    }
}
impl fmt::Display for WorkoutPower {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            WorkoutPower::WattsOffset => writeln!(f, "watts_offset"),
            WorkoutPower::UnknownVariant(value) => writeln!(f, "unknown_variant_{}", *value),
        }
    }
}
impl convert::From<u32> for WorkoutPower {
    fn from(value: u32) -> Self {
        match value {
            1000 => WorkoutPower::WattsOffset,
            _ => WorkoutPower::UnknownVariant(value),
        }
    }
}
impl convert::From<i64> for WorkoutPower {
    fn from(value: i64) -> Self {
        WorkoutPower::from(value as u32)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum BpStatus {
    NoError,
    ErrorIncompleteData,
    ErrorNoMeasurement,
    ErrorDataOutOfRange,
    ErrorIrregularHeartRate,
    UnknownVariant(u8),
}
impl BpStatus {
    pub fn as_u8(self) -> u8 {
        match self {
            BpStatus::NoError => 0,
            BpStatus::ErrorIncompleteData => 1,
            BpStatus::ErrorNoMeasurement => 2,
            BpStatus::ErrorDataOutOfRange => 3,
            BpStatus::ErrorIrregularHeartRate => 4,
            BpStatus::UnknownVariant(value) => value,
        }
    }
    pub fn as_i64(self) -> i64 {
        self.as_u8() as i64
    }
}
impl fmt::Display for BpStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            BpStatus::NoError => writeln!(f, "no_error"),
            BpStatus::ErrorIncompleteData => writeln!(f, "error_incomplete_data"),
            BpStatus::ErrorNoMeasurement => writeln!(f, "error_no_measurement"),
            BpStatus::ErrorDataOutOfRange => writeln!(f, "error_data_out_of_range"),
            BpStatus::ErrorIrregularHeartRate => writeln!(f, "error_irregular_heart_rate"),
            BpStatus::UnknownVariant(value) => writeln!(f, "unknown_variant_{}", *value),
        }
    }
}
impl convert::From<u8> for BpStatus {
    fn from(value: u8) -> Self {
        match value {
            0 => BpStatus::NoError,
            1 => BpStatus::ErrorIncompleteData,
            2 => BpStatus::ErrorNoMeasurement,
            3 => BpStatus::ErrorDataOutOfRange,
            4 => BpStatus::ErrorIrregularHeartRate,
            _ => BpStatus::UnknownVariant(value),
        }
    }
}
impl convert::From<i64> for BpStatus {
    fn from(value: i64) -> Self {
        BpStatus::from(value as u8)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum UserLocalId {
    LocalMin,
    LocalMax,
    StationaryMin,
    StationaryMax,
    PortableMin,
    PortableMax,
    UnknownVariant(u16),
}
impl UserLocalId {
    pub fn as_u16(self) -> u16 {
        match self {
            UserLocalId::LocalMin => 0,
            UserLocalId::LocalMax => 15,
            UserLocalId::StationaryMin => 16,
            UserLocalId::StationaryMax => 255,
            UserLocalId::PortableMin => 256,
            UserLocalId::PortableMax => 65534,
            UserLocalId::UnknownVariant(value) => value,
        }
    }
    pub fn as_i64(self) -> i64 {
        self.as_u16() as i64
    }
}
impl fmt::Display for UserLocalId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            UserLocalId::LocalMin => writeln!(f, "local_min"),
            UserLocalId::LocalMax => writeln!(f, "local_max"),
            UserLocalId::StationaryMin => writeln!(f, "stationary_min"),
            UserLocalId::StationaryMax => writeln!(f, "stationary_max"),
            UserLocalId::PortableMin => writeln!(f, "portable_min"),
            UserLocalId::PortableMax => writeln!(f, "portable_max"),
            UserLocalId::UnknownVariant(value) => writeln!(f, "unknown_variant_{}", *value),
        }
    }
}
impl convert::From<u16> for UserLocalId {
    fn from(value: u16) -> Self {
        match value {
            0 => UserLocalId::LocalMin,
            15 => UserLocalId::LocalMax,
            16 => UserLocalId::StationaryMin,
            255 => UserLocalId::StationaryMax,
            256 => UserLocalId::PortableMin,
            65534 => UserLocalId::PortableMax,
            _ => UserLocalId::UnknownVariant(value),
        }
    }
}
impl convert::From<i64> for UserLocalId {
    fn from(value: i64) -> Self {
        UserLocalId::from(value as u16)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum SwimStroke {
    Freestyle,
    Backstroke,
    Breaststroke,
    Butterfly,
    Drill,
    Mixed,
    /// IM is a mixed interval containing the same number of lengths for each of: Butterfly, Backstroke, Breaststroke, Freestyle, swam in that order.
    Im,
    UnknownVariant(u8),
}
impl SwimStroke {
    pub fn as_u8(self) -> u8 {
        match self {
            SwimStroke::Freestyle => 0,
            SwimStroke::Backstroke => 1,
            SwimStroke::Breaststroke => 2,
            SwimStroke::Butterfly => 3,
            SwimStroke::Drill => 4,
            SwimStroke::Mixed => 5,
            SwimStroke::Im => 6,
            SwimStroke::UnknownVariant(value) => value,
        }
    }
    pub fn as_i64(self) -> i64 {
        self.as_u8() as i64
    }
}
impl fmt::Display for SwimStroke {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            SwimStroke::Freestyle => writeln!(f, "freestyle"),
            SwimStroke::Backstroke => writeln!(f, "backstroke"),
            SwimStroke::Breaststroke => writeln!(f, "breaststroke"),
            SwimStroke::Butterfly => writeln!(f, "butterfly"),
            SwimStroke::Drill => writeln!(f, "drill"),
            SwimStroke::Mixed => writeln!(f, "mixed"),
            SwimStroke::Im => writeln!(f, "im"),
            SwimStroke::UnknownVariant(value) => writeln!(f, "unknown_variant_{}", *value),
        }
    }
}
impl convert::From<u8> for SwimStroke {
    fn from(value: u8) -> Self {
        match value {
            0 => SwimStroke::Freestyle,
            1 => SwimStroke::Backstroke,
            2 => SwimStroke::Breaststroke,
            3 => SwimStroke::Butterfly,
            4 => SwimStroke::Drill,
            5 => SwimStroke::Mixed,
            6 => SwimStroke::Im,
            _ => SwimStroke::UnknownVariant(value),
        }
    }
}
impl convert::From<i64> for SwimStroke {
    fn from(value: i64) -> Self {
        SwimStroke::from(value as u8)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum ActivityType {
    Generic,
    Running,
    Cycling,
    /// Mulitsport transition
    Transition,
    FitnessEquipment,
    Swimming,
    Walking,
    Sedentary,
    /// All is for goals only to include all sports.
    All,
    UnknownVariant(u8),
}
impl ActivityType {
    pub fn as_u8(self) -> u8 {
        match self {
            ActivityType::Generic => 0,
            ActivityType::Running => 1,
            ActivityType::Cycling => 2,
            ActivityType::Transition => 3,
            ActivityType::FitnessEquipment => 4,
            ActivityType::Swimming => 5,
            ActivityType::Walking => 6,
            ActivityType::Sedentary => 8,
            ActivityType::All => 254,
            ActivityType::UnknownVariant(value) => value,
        }
    }
    pub fn as_i64(self) -> i64 {
        self.as_u8() as i64
    }
}
impl fmt::Display for ActivityType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            ActivityType::Generic => writeln!(f, "generic"),
            ActivityType::Running => writeln!(f, "running"),
            ActivityType::Cycling => writeln!(f, "cycling"),
            ActivityType::Transition => writeln!(f, "transition"),
            ActivityType::FitnessEquipment => writeln!(f, "fitness_equipment"),
            ActivityType::Swimming => writeln!(f, "swimming"),
            ActivityType::Walking => writeln!(f, "walking"),
            ActivityType::Sedentary => writeln!(f, "sedentary"),
            ActivityType::All => writeln!(f, "all"),
            ActivityType::UnknownVariant(value) => writeln!(f, "unknown_variant_{}", *value),
        }
    }
}
impl convert::From<u8> for ActivityType {
    fn from(value: u8) -> Self {
        match value {
            0 => ActivityType::Generic,
            1 => ActivityType::Running,
            2 => ActivityType::Cycling,
            3 => ActivityType::Transition,
            4 => ActivityType::FitnessEquipment,
            5 => ActivityType::Swimming,
            6 => ActivityType::Walking,
            8 => ActivityType::Sedentary,
            254 => ActivityType::All,
            _ => ActivityType::UnknownVariant(value),
        }
    }
}
impl convert::From<i64> for ActivityType {
    fn from(value: i64) -> Self {
        ActivityType::from(value as u8)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum ActivitySubtype {
    Generic,
    /// Run
    Treadmill,
    /// Run
    Street,
    /// Run
    Trail,
    /// Run
    Track,
    /// Cycling
    Spin,
    /// Cycling
    IndoorCycling,
    /// Cycling
    Road,
    /// Cycling
    Mountain,
    /// Cycling
    Downhill,
    /// Cycling
    Recumbent,
    /// Cycling
    Cyclocross,
    /// Cycling
    HandCycling,
    /// Cycling
    TrackCycling,
    /// Fitness Equipment
    IndoorRowing,
    /// Fitness Equipment
    Elliptical,
    /// Fitness Equipment
    StairClimbing,
    /// Swimming
    LapSwimming,
    /// Swimming
    OpenWater,
    All,
    UnknownVariant(u8),
}
impl ActivitySubtype {
    pub fn as_u8(self) -> u8 {
        match self {
            ActivitySubtype::Generic => 0,
            ActivitySubtype::Treadmill => 1,
            ActivitySubtype::Street => 2,
            ActivitySubtype::Trail => 3,
            ActivitySubtype::Track => 4,
            ActivitySubtype::Spin => 5,
            ActivitySubtype::IndoorCycling => 6,
            ActivitySubtype::Road => 7,
            ActivitySubtype::Mountain => 8,
            ActivitySubtype::Downhill => 9,
            ActivitySubtype::Recumbent => 10,
            ActivitySubtype::Cyclocross => 11,
            ActivitySubtype::HandCycling => 12,
            ActivitySubtype::TrackCycling => 13,
            ActivitySubtype::IndoorRowing => 14,
            ActivitySubtype::Elliptical => 15,
            ActivitySubtype::StairClimbing => 16,
            ActivitySubtype::LapSwimming => 17,
            ActivitySubtype::OpenWater => 18,
            ActivitySubtype::All => 254,
            ActivitySubtype::UnknownVariant(value) => value,
        }
    }
    pub fn as_i64(self) -> i64 {
        self.as_u8() as i64
    }
}
impl fmt::Display for ActivitySubtype {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            ActivitySubtype::Generic => writeln!(f, "generic"),
            ActivitySubtype::Treadmill => writeln!(f, "treadmill"),
            ActivitySubtype::Street => writeln!(f, "street"),
            ActivitySubtype::Trail => writeln!(f, "trail"),
            ActivitySubtype::Track => writeln!(f, "track"),
            ActivitySubtype::Spin => writeln!(f, "spin"),
            ActivitySubtype::IndoorCycling => writeln!(f, "indoor_cycling"),
            ActivitySubtype::Road => writeln!(f, "road"),
            ActivitySubtype::Mountain => writeln!(f, "mountain"),
            ActivitySubtype::Downhill => writeln!(f, "downhill"),
            ActivitySubtype::Recumbent => writeln!(f, "recumbent"),
            ActivitySubtype::Cyclocross => writeln!(f, "cyclocross"),
            ActivitySubtype::HandCycling => writeln!(f, "hand_cycling"),
            ActivitySubtype::TrackCycling => writeln!(f, "track_cycling"),
            ActivitySubtype::IndoorRowing => writeln!(f, "indoor_rowing"),
            ActivitySubtype::Elliptical => writeln!(f, "elliptical"),
            ActivitySubtype::StairClimbing => writeln!(f, "stair_climbing"),
            ActivitySubtype::LapSwimming => writeln!(f, "lap_swimming"),
            ActivitySubtype::OpenWater => writeln!(f, "open_water"),
            ActivitySubtype::All => writeln!(f, "all"),
            ActivitySubtype::UnknownVariant(value) => writeln!(f, "unknown_variant_{}", *value),
        }
    }
}
impl convert::From<u8> for ActivitySubtype {
    fn from(value: u8) -> Self {
        match value {
            0 => ActivitySubtype::Generic,
            1 => ActivitySubtype::Treadmill,
            2 => ActivitySubtype::Street,
            3 => ActivitySubtype::Trail,
            4 => ActivitySubtype::Track,
            5 => ActivitySubtype::Spin,
            6 => ActivitySubtype::IndoorCycling,
            7 => ActivitySubtype::Road,
            8 => ActivitySubtype::Mountain,
            9 => ActivitySubtype::Downhill,
            10 => ActivitySubtype::Recumbent,
            11 => ActivitySubtype::Cyclocross,
            12 => ActivitySubtype::HandCycling,
            13 => ActivitySubtype::TrackCycling,
            14 => ActivitySubtype::IndoorRowing,
            15 => ActivitySubtype::Elliptical,
            16 => ActivitySubtype::StairClimbing,
            17 => ActivitySubtype::LapSwimming,
            18 => ActivitySubtype::OpenWater,
            254 => ActivitySubtype::All,
            _ => ActivitySubtype::UnknownVariant(value),
        }
    }
}
impl convert::From<i64> for ActivitySubtype {
    fn from(value: i64) -> Self {
        ActivitySubtype::from(value as u8)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum ActivityLevel {
    Low,
    Medium,
    High,
    UnknownVariant(u8),
}
impl ActivityLevel {
    pub fn as_u8(self) -> u8 {
        match self {
            ActivityLevel::Low => 0,
            ActivityLevel::Medium => 1,
            ActivityLevel::High => 2,
            ActivityLevel::UnknownVariant(value) => value,
        }
    }
    pub fn as_i64(self) -> i64 {
        self.as_u8() as i64
    }
}
impl fmt::Display for ActivityLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            ActivityLevel::Low => writeln!(f, "low"),
            ActivityLevel::Medium => writeln!(f, "medium"),
            ActivityLevel::High => writeln!(f, "high"),
            ActivityLevel::UnknownVariant(value) => writeln!(f, "unknown_variant_{}", *value),
        }
    }
}
impl convert::From<u8> for ActivityLevel {
    fn from(value: u8) -> Self {
        match value {
            0 => ActivityLevel::Low,
            1 => ActivityLevel::Medium,
            2 => ActivityLevel::High,
            _ => ActivityLevel::UnknownVariant(value),
        }
    }
}
impl convert::From<i64> for ActivityLevel {
    fn from(value: i64) -> Self {
        ActivityLevel::from(value as u8)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum Side {
    Right,
    Left,
    UnknownVariant(u8),
}
impl Side {
    pub fn as_u8(self) -> u8 {
        match self {
            Side::Right => 0,
            Side::Left => 1,
            Side::UnknownVariant(value) => value,
        }
    }
    pub fn as_i64(self) -> i64 {
        self.as_u8() as i64
    }
}
impl fmt::Display for Side {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Side::Right => writeln!(f, "right"),
            Side::Left => writeln!(f, "left"),
            Side::UnknownVariant(value) => writeln!(f, "unknown_variant_{}", *value),
        }
    }
}
impl convert::From<u8> for Side {
    fn from(value: u8) -> Self {
        match value {
            0 => Side::Right,
            1 => Side::Left,
            _ => Side::UnknownVariant(value),
        }
    }
}
impl convert::From<i64> for Side {
    fn from(value: i64) -> Self {
        Side::from(value as u8)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum LeftRightBalance {
    /// % contribution
    Mask,
    /// data corresponds to right if set, otherwise unknown
    Right,
    UnknownVariant(u8),
}
impl LeftRightBalance {
    pub fn as_u8(self) -> u8 {
        match self {
            LeftRightBalance::Mask => 127,
            LeftRightBalance::Right => 128,
            LeftRightBalance::UnknownVariant(value) => value,
        }
    }
    pub fn as_i64(self) -> i64 {
        self.as_u8() as i64
    }
}
impl fmt::Display for LeftRightBalance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            LeftRightBalance::Mask => writeln!(f, "mask"),
            LeftRightBalance::Right => writeln!(f, "right"),
            LeftRightBalance::UnknownVariant(value) => writeln!(f, "unknown_variant_{}", *value),
        }
    }
}
impl convert::From<u8> for LeftRightBalance {
    fn from(value: u8) -> Self {
        match value {
            127 => LeftRightBalance::Mask,
            128 => LeftRightBalance::Right,
            _ => LeftRightBalance::UnknownVariant(value),
        }
    }
}
impl convert::From<i64> for LeftRightBalance {
    fn from(value: i64) -> Self {
        LeftRightBalance::from(value as u8)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum LeftRightBalance100 {
    /// % contribution scaled by 100
    Mask,
    /// data corresponds to right if set, otherwise unknown
    Right,
    UnknownVariant(u16),
}
impl LeftRightBalance100 {
    pub fn as_u16(self) -> u16 {
        match self {
            LeftRightBalance100::Mask => 16383,
            LeftRightBalance100::Right => 32768,
            LeftRightBalance100::UnknownVariant(value) => value,
        }
    }
    pub fn as_i64(self) -> i64 {
        self.as_u16() as i64
    }
}
impl fmt::Display for LeftRightBalance100 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            LeftRightBalance100::Mask => writeln!(f, "mask"),
            LeftRightBalance100::Right => writeln!(f, "right"),
            LeftRightBalance100::UnknownVariant(value) => writeln!(f, "unknown_variant_{}", *value),
        }
    }
}
impl convert::From<u16> for LeftRightBalance100 {
    fn from(value: u16) -> Self {
        match value {
            16383 => LeftRightBalance100::Mask,
            32768 => LeftRightBalance100::Right,
            _ => LeftRightBalance100::UnknownVariant(value),
        }
    }
}
impl convert::From<i64> for LeftRightBalance100 {
    fn from(value: i64) -> Self {
        LeftRightBalance100::from(value as u16)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum LengthType {
    /// Rest period. Length with no strokes
    Idle,
    /// Length with strokes.
    Active,
    UnknownVariant(u8),
}
impl LengthType {
    pub fn as_u8(self) -> u8 {
        match self {
            LengthType::Idle => 0,
            LengthType::Active => 1,
            LengthType::UnknownVariant(value) => value,
        }
    }
    pub fn as_i64(self) -> i64 {
        self.as_u8() as i64
    }
}
impl fmt::Display for LengthType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            LengthType::Idle => writeln!(f, "idle"),
            LengthType::Active => writeln!(f, "active"),
            LengthType::UnknownVariant(value) => writeln!(f, "unknown_variant_{}", *value),
        }
    }
}
impl convert::From<u8> for LengthType {
    fn from(value: u8) -> Self {
        match value {
            0 => LengthType::Idle,
            1 => LengthType::Active,
            _ => LengthType::UnknownVariant(value),
        }
    }
}
impl convert::From<i64> for LengthType {
    fn from(value: i64) -> Self {
        LengthType::from(value as u8)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum DayOfWeek {
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    UnknownVariant(u8),
}
impl DayOfWeek {
    pub fn as_u8(self) -> u8 {
        match self {
            DayOfWeek::Sunday => 0,
            DayOfWeek::Monday => 1,
            DayOfWeek::Tuesday => 2,
            DayOfWeek::Wednesday => 3,
            DayOfWeek::Thursday => 4,
            DayOfWeek::Friday => 5,
            DayOfWeek::Saturday => 6,
            DayOfWeek::UnknownVariant(value) => value,
        }
    }
    pub fn as_i64(self) -> i64 {
        self.as_u8() as i64
    }
}
impl fmt::Display for DayOfWeek {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            DayOfWeek::Sunday => writeln!(f, "sunday"),
            DayOfWeek::Monday => writeln!(f, "monday"),
            DayOfWeek::Tuesday => writeln!(f, "tuesday"),
            DayOfWeek::Wednesday => writeln!(f, "wednesday"),
            DayOfWeek::Thursday => writeln!(f, "thursday"),
            DayOfWeek::Friday => writeln!(f, "friday"),
            DayOfWeek::Saturday => writeln!(f, "saturday"),
            DayOfWeek::UnknownVariant(value) => writeln!(f, "unknown_variant_{}", *value),
        }
    }
}
impl convert::From<u8> for DayOfWeek {
    fn from(value: u8) -> Self {
        match value {
            0 => DayOfWeek::Sunday,
            1 => DayOfWeek::Monday,
            2 => DayOfWeek::Tuesday,
            3 => DayOfWeek::Wednesday,
            4 => DayOfWeek::Thursday,
            5 => DayOfWeek::Friday,
            6 => DayOfWeek::Saturday,
            _ => DayOfWeek::UnknownVariant(value),
        }
    }
}
impl convert::From<i64> for DayOfWeek {
    fn from(value: i64) -> Self {
        DayOfWeek::from(value as u8)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum ConnectivityCapabilities {
    Bluetooth,
    BluetoothLe,
    Ant,
    ActivityUpload,
    CourseDownload,
    WorkoutDownload,
    LiveTrack,
    WeatherConditions,
    WeatherAlerts,
    GpsEphemerisDownload,
    ExplicitArchive,
    SetupIncomplete,
    ContinueSyncAfterSoftwareUpdate,
    ConnectIqAppDownload,
    GolfCourseDownload,
    /// Indicates device is in control of initiating all syncs
    DeviceInitiatesSync,
    ConnectIqWatchAppDownload,
    ConnectIqWidgetDownload,
    ConnectIqWatchFaceDownload,
    ConnectIqDataFieldDownload,
    /// Device supports delete and reorder of apps via GCM
    ConnectIqAppManagment,
    SwingSensor,
    SwingSensorRemote,
    /// Device supports incident detection
    IncidentDetection,
    AudioPrompts,
    /// Device supports reporting wifi verification via GCM
    WifiVerification,
    /// Device supports True Up
    TrueUp,
    /// Device supports Find My Watch
    FindMyWatch,
    RemoteManualSync,
    /// Device supports LiveTrack auto start
    LiveTrackAutoStart,
    /// Device supports LiveTrack Messaging
    LiveTrackMessaging,
    /// Device supports instant input feature
    InstantInput,
    UnknownVariant(u32),
}
impl ConnectivityCapabilities {
    pub fn as_u32(self) -> u32 {
        match self {
            ConnectivityCapabilities::Bluetooth => 1,
            ConnectivityCapabilities::BluetoothLe => 2,
            ConnectivityCapabilities::Ant => 4,
            ConnectivityCapabilities::ActivityUpload => 8,
            ConnectivityCapabilities::CourseDownload => 16,
            ConnectivityCapabilities::WorkoutDownload => 32,
            ConnectivityCapabilities::LiveTrack => 64,
            ConnectivityCapabilities::WeatherConditions => 128,
            ConnectivityCapabilities::WeatherAlerts => 256,
            ConnectivityCapabilities::GpsEphemerisDownload => 512,
            ConnectivityCapabilities::ExplicitArchive => 1024,
            ConnectivityCapabilities::SetupIncomplete => 2048,
            ConnectivityCapabilities::ContinueSyncAfterSoftwareUpdate => 4096,
            ConnectivityCapabilities::ConnectIqAppDownload => 8192,
            ConnectivityCapabilities::GolfCourseDownload => 16384,
            ConnectivityCapabilities::DeviceInitiatesSync => 32768,
            ConnectivityCapabilities::ConnectIqWatchAppDownload => 65536,
            ConnectivityCapabilities::ConnectIqWidgetDownload => 131072,
            ConnectivityCapabilities::ConnectIqWatchFaceDownload => 262144,
            ConnectivityCapabilities::ConnectIqDataFieldDownload => 524288,
            ConnectivityCapabilities::ConnectIqAppManagment => 1048576,
            ConnectivityCapabilities::SwingSensor => 2097152,
            ConnectivityCapabilities::SwingSensorRemote => 4194304,
            ConnectivityCapabilities::IncidentDetection => 8388608,
            ConnectivityCapabilities::AudioPrompts => 16777216,
            ConnectivityCapabilities::WifiVerification => 33554432,
            ConnectivityCapabilities::TrueUp => 67108864,
            ConnectivityCapabilities::FindMyWatch => 134217728,
            ConnectivityCapabilities::RemoteManualSync => 268435456,
            ConnectivityCapabilities::LiveTrackAutoStart => 536870912,
            ConnectivityCapabilities::LiveTrackMessaging => 1073741824,
            ConnectivityCapabilities::InstantInput => 2147483648,
            ConnectivityCapabilities::UnknownVariant(value) => value,
        }
    }
    pub fn as_i64(self) -> i64 {
        self.as_u32() as i64
    }
}
impl fmt::Display for ConnectivityCapabilities {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            ConnectivityCapabilities::Bluetooth => writeln!(f, "bluetooth"),
            ConnectivityCapabilities::BluetoothLe => writeln!(f, "bluetooth_le"),
            ConnectivityCapabilities::Ant => writeln!(f, "ant"),
            ConnectivityCapabilities::ActivityUpload => writeln!(f, "activity_upload"),
            ConnectivityCapabilities::CourseDownload => writeln!(f, "course_download"),
            ConnectivityCapabilities::WorkoutDownload => writeln!(f, "workout_download"),
            ConnectivityCapabilities::LiveTrack => writeln!(f, "live_track"),
            ConnectivityCapabilities::WeatherConditions => writeln!(f, "weather_conditions"),
            ConnectivityCapabilities::WeatherAlerts => writeln!(f, "weather_alerts"),
            ConnectivityCapabilities::GpsEphemerisDownload => writeln!(f, "gps_ephemeris_download"),
            ConnectivityCapabilities::ExplicitArchive => writeln!(f, "explicit_archive"),
            ConnectivityCapabilities::SetupIncomplete => writeln!(f, "setup_incomplete"),
            ConnectivityCapabilities::ContinueSyncAfterSoftwareUpdate => {
                writeln!(f, "continue_sync_after_software_update")
            }
            ConnectivityCapabilities::ConnectIqAppDownload => {
                writeln!(f, "connect_iq_app_download")
            }
            ConnectivityCapabilities::GolfCourseDownload => writeln!(f, "golf_course_download"),
            ConnectivityCapabilities::DeviceInitiatesSync => writeln!(f, "device_initiates_sync"),
            ConnectivityCapabilities::ConnectIqWatchAppDownload => {
                writeln!(f, "connect_iq_watch_app_download")
            }
            ConnectivityCapabilities::ConnectIqWidgetDownload => {
                writeln!(f, "connect_iq_widget_download")
            }
            ConnectivityCapabilities::ConnectIqWatchFaceDownload => {
                writeln!(f, "connect_iq_watch_face_download")
            }
            ConnectivityCapabilities::ConnectIqDataFieldDownload => {
                writeln!(f, "connect_iq_data_field_download")
            }
            ConnectivityCapabilities::ConnectIqAppManagment => {
                writeln!(f, "connect_iq_app_managment")
            }
            ConnectivityCapabilities::SwingSensor => writeln!(f, "swing_sensor"),
            ConnectivityCapabilities::SwingSensorRemote => writeln!(f, "swing_sensor_remote"),
            ConnectivityCapabilities::IncidentDetection => writeln!(f, "incident_detection"),
            ConnectivityCapabilities::AudioPrompts => writeln!(f, "audio_prompts"),
            ConnectivityCapabilities::WifiVerification => writeln!(f, "wifi_verification"),
            ConnectivityCapabilities::TrueUp => writeln!(f, "true_up"),
            ConnectivityCapabilities::FindMyWatch => writeln!(f, "find_my_watch"),
            ConnectivityCapabilities::RemoteManualSync => writeln!(f, "remote_manual_sync"),
            ConnectivityCapabilities::LiveTrackAutoStart => writeln!(f, "live_track_auto_start"),
            ConnectivityCapabilities::LiveTrackMessaging => writeln!(f, "live_track_messaging"),
            ConnectivityCapabilities::InstantInput => writeln!(f, "instant_input"),
            ConnectivityCapabilities::UnknownVariant(value) => {
                writeln!(f, "unknown_variant_{}", *value)
            }
        }
    }
}
impl convert::From<u32> for ConnectivityCapabilities {
    fn from(value: u32) -> Self {
        match value {
            1 => ConnectivityCapabilities::Bluetooth,
            2 => ConnectivityCapabilities::BluetoothLe,
            4 => ConnectivityCapabilities::Ant,
            8 => ConnectivityCapabilities::ActivityUpload,
            16 => ConnectivityCapabilities::CourseDownload,
            32 => ConnectivityCapabilities::WorkoutDownload,
            64 => ConnectivityCapabilities::LiveTrack,
            128 => ConnectivityCapabilities::WeatherConditions,
            256 => ConnectivityCapabilities::WeatherAlerts,
            512 => ConnectivityCapabilities::GpsEphemerisDownload,
            1024 => ConnectivityCapabilities::ExplicitArchive,
            2048 => ConnectivityCapabilities::SetupIncomplete,
            4096 => ConnectivityCapabilities::ContinueSyncAfterSoftwareUpdate,
            8192 => ConnectivityCapabilities::ConnectIqAppDownload,
            16384 => ConnectivityCapabilities::GolfCourseDownload,
            32768 => ConnectivityCapabilities::DeviceInitiatesSync,
            65536 => ConnectivityCapabilities::ConnectIqWatchAppDownload,
            131072 => ConnectivityCapabilities::ConnectIqWidgetDownload,
            262144 => ConnectivityCapabilities::ConnectIqWatchFaceDownload,
            524288 => ConnectivityCapabilities::ConnectIqDataFieldDownload,
            1048576 => ConnectivityCapabilities::ConnectIqAppManagment,
            2097152 => ConnectivityCapabilities::SwingSensor,
            4194304 => ConnectivityCapabilities::SwingSensorRemote,
            8388608 => ConnectivityCapabilities::IncidentDetection,
            16777216 => ConnectivityCapabilities::AudioPrompts,
            33554432 => ConnectivityCapabilities::WifiVerification,
            67108864 => ConnectivityCapabilities::TrueUp,
            134217728 => ConnectivityCapabilities::FindMyWatch,
            268435456 => ConnectivityCapabilities::RemoteManualSync,
            536870912 => ConnectivityCapabilities::LiveTrackAutoStart,
            1073741824 => ConnectivityCapabilities::LiveTrackMessaging,
            2147483648 => ConnectivityCapabilities::InstantInput,
            _ => ConnectivityCapabilities::UnknownVariant(value),
        }
    }
}
impl convert::From<i64> for ConnectivityCapabilities {
    fn from(value: i64) -> Self {
        ConnectivityCapabilities::from(value as u32)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum WeatherReport {
    Current,
    HourlyForecast,
    DailyForecast,
    UnknownVariant(u8),
}
impl WeatherReport {
    pub fn as_u8(self) -> u8 {
        match self {
            WeatherReport::Current => 0,
            WeatherReport::HourlyForecast => 1,
            WeatherReport::DailyForecast => 2,
            WeatherReport::UnknownVariant(value) => value,
        }
    }
    pub fn as_i64(self) -> i64 {
        self.as_u8() as i64
    }
}
impl fmt::Display for WeatherReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            WeatherReport::Current => writeln!(f, "current"),
            WeatherReport::HourlyForecast => writeln!(f, "hourly_forecast"),
            WeatherReport::DailyForecast => writeln!(f, "daily_forecast"),
            WeatherReport::UnknownVariant(value) => writeln!(f, "unknown_variant_{}", *value),
        }
    }
}
impl convert::From<u8> for WeatherReport {
    fn from(value: u8) -> Self {
        match value {
            0 => WeatherReport::Current,
            1 => WeatherReport::HourlyForecast,
            2 => WeatherReport::DailyForecast,
            _ => WeatherReport::UnknownVariant(value),
        }
    }
}
impl convert::From<i64> for WeatherReport {
    fn from(value: i64) -> Self {
        WeatherReport::from(value as u8)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum WeatherStatus {
    Clear,
    PartlyCloudy,
    MostlyCloudy,
    Rain,
    Snow,
    Windy,
    Thunderstorms,
    WintryMix,
    Fog,
    Hazy,
    Hail,
    ScatteredShowers,
    ScatteredThunderstorms,
    UnknownPrecipitation,
    LightRain,
    HeavyRain,
    LightSnow,
    HeavySnow,
    LightRainSnow,
    HeavyRainSnow,
    Cloudy,
    UnknownVariant(u8),
}
impl WeatherStatus {
    pub fn as_u8(self) -> u8 {
        match self {
            WeatherStatus::Clear => 0,
            WeatherStatus::PartlyCloudy => 1,
            WeatherStatus::MostlyCloudy => 2,
            WeatherStatus::Rain => 3,
            WeatherStatus::Snow => 4,
            WeatherStatus::Windy => 5,
            WeatherStatus::Thunderstorms => 6,
            WeatherStatus::WintryMix => 7,
            WeatherStatus::Fog => 8,
            WeatherStatus::Hazy => 11,
            WeatherStatus::Hail => 12,
            WeatherStatus::ScatteredShowers => 13,
            WeatherStatus::ScatteredThunderstorms => 14,
            WeatherStatus::UnknownPrecipitation => 15,
            WeatherStatus::LightRain => 16,
            WeatherStatus::HeavyRain => 17,
            WeatherStatus::LightSnow => 18,
            WeatherStatus::HeavySnow => 19,
            WeatherStatus::LightRainSnow => 20,
            WeatherStatus::HeavyRainSnow => 21,
            WeatherStatus::Cloudy => 22,
            WeatherStatus::UnknownVariant(value) => value,
        }
    }
    pub fn as_i64(self) -> i64 {
        self.as_u8() as i64
    }
}
impl fmt::Display for WeatherStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            WeatherStatus::Clear => writeln!(f, "clear"),
            WeatherStatus::PartlyCloudy => writeln!(f, "partly_cloudy"),
            WeatherStatus::MostlyCloudy => writeln!(f, "mostly_cloudy"),
            WeatherStatus::Rain => writeln!(f, "rain"),
            WeatherStatus::Snow => writeln!(f, "snow"),
            WeatherStatus::Windy => writeln!(f, "windy"),
            WeatherStatus::Thunderstorms => writeln!(f, "thunderstorms"),
            WeatherStatus::WintryMix => writeln!(f, "wintry_mix"),
            WeatherStatus::Fog => writeln!(f, "fog"),
            WeatherStatus::Hazy => writeln!(f, "hazy"),
            WeatherStatus::Hail => writeln!(f, "hail"),
            WeatherStatus::ScatteredShowers => writeln!(f, "scattered_showers"),
            WeatherStatus::ScatteredThunderstorms => writeln!(f, "scattered_thunderstorms"),
            WeatherStatus::UnknownPrecipitation => writeln!(f, "unknown_precipitation"),
            WeatherStatus::LightRain => writeln!(f, "light_rain"),
            WeatherStatus::HeavyRain => writeln!(f, "heavy_rain"),
            WeatherStatus::LightSnow => writeln!(f, "light_snow"),
            WeatherStatus::HeavySnow => writeln!(f, "heavy_snow"),
            WeatherStatus::LightRainSnow => writeln!(f, "light_rain_snow"),
            WeatherStatus::HeavyRainSnow => writeln!(f, "heavy_rain_snow"),
            WeatherStatus::Cloudy => writeln!(f, "cloudy"),
            WeatherStatus::UnknownVariant(value) => writeln!(f, "unknown_variant_{}", *value),
        }
    }
}
impl convert::From<u8> for WeatherStatus {
    fn from(value: u8) -> Self {
        match value {
            0 => WeatherStatus::Clear,
            1 => WeatherStatus::PartlyCloudy,
            2 => WeatherStatus::MostlyCloudy,
            3 => WeatherStatus::Rain,
            4 => WeatherStatus::Snow,
            5 => WeatherStatus::Windy,
            6 => WeatherStatus::Thunderstorms,
            7 => WeatherStatus::WintryMix,
            8 => WeatherStatus::Fog,
            11 => WeatherStatus::Hazy,
            12 => WeatherStatus::Hail,
            13 => WeatherStatus::ScatteredShowers,
            14 => WeatherStatus::ScatteredThunderstorms,
            15 => WeatherStatus::UnknownPrecipitation,
            16 => WeatherStatus::LightRain,
            17 => WeatherStatus::HeavyRain,
            18 => WeatherStatus::LightSnow,
            19 => WeatherStatus::HeavySnow,
            20 => WeatherStatus::LightRainSnow,
            21 => WeatherStatus::HeavyRainSnow,
            22 => WeatherStatus::Cloudy,
            _ => WeatherStatus::UnknownVariant(value),
        }
    }
}
impl convert::From<i64> for WeatherStatus {
    fn from(value: i64) -> Self {
        WeatherStatus::from(value as u8)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum WeatherSeverity {
    Unknown,
    Warning,
    Watch,
    Advisory,
    Statement,
    UnknownVariant(u8),
}
impl WeatherSeverity {
    pub fn as_u8(self) -> u8 {
        match self {
            WeatherSeverity::Unknown => 0,
            WeatherSeverity::Warning => 1,
            WeatherSeverity::Watch => 2,
            WeatherSeverity::Advisory => 3,
            WeatherSeverity::Statement => 4,
            WeatherSeverity::UnknownVariant(value) => value,
        }
    }
    pub fn as_i64(self) -> i64 {
        self.as_u8() as i64
    }
}
impl fmt::Display for WeatherSeverity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            WeatherSeverity::Unknown => writeln!(f, "unknown"),
            WeatherSeverity::Warning => writeln!(f, "warning"),
            WeatherSeverity::Watch => writeln!(f, "watch"),
            WeatherSeverity::Advisory => writeln!(f, "advisory"),
            WeatherSeverity::Statement => writeln!(f, "statement"),
            WeatherSeverity::UnknownVariant(value) => writeln!(f, "unknown_variant_{}", *value),
        }
    }
}
impl convert::From<u8> for WeatherSeverity {
    fn from(value: u8) -> Self {
        match value {
            0 => WeatherSeverity::Unknown,
            1 => WeatherSeverity::Warning,
            2 => WeatherSeverity::Watch,
            3 => WeatherSeverity::Advisory,
            4 => WeatherSeverity::Statement,
            _ => WeatherSeverity::UnknownVariant(value),
        }
    }
}
impl convert::From<i64> for WeatherSeverity {
    fn from(value: i64) -> Self {
        WeatherSeverity::from(value as u8)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum WeatherSevereType {
    Unspecified,
    Tornado,
    Tsunami,
    Hurricane,
    ExtremeWind,
    Typhoon,
    InlandHurricane,
    HurricaneForceWind,
    Waterspout,
    SevereThunderstorm,
    WreckhouseWinds,
    LesSuetesWind,
    Avalanche,
    FlashFlood,
    TropicalStorm,
    InlandTropicalStorm,
    Blizzard,
    IceStorm,
    FreezingRain,
    DebrisFlow,
    FlashFreeze,
    DustStorm,
    HighWind,
    WinterStorm,
    HeavyFreezingSpray,
    ExtremeCold,
    WindChill,
    ColdWave,
    HeavySnowAlert,
    LakeEffectBlowingSnow,
    SnowSquall,
    LakeEffectSnow,
    WinterWeather,
    Sleet,
    Snowfall,
    SnowAndBlowingSnow,
    BlowingSnow,
    SnowAlert,
    ArcticOutflow,
    FreezingDrizzle,
    Storm,
    StormSurge,
    Rainfall,
    ArealFlood,
    CoastalFlood,
    LakeshoreFlood,
    ExcessiveHeat,
    Heat,
    Weather,
    HighHeatAndHumidity,
    HumidexAndHealth,
    Humidex,
    Gale,
    FreezingSpray,
    SpecialMarine,
    Squall,
    StrongWind,
    LakeWind,
    MarineWeather,
    Wind,
    SmallCraftHazardousSeas,
    HazardousSeas,
    SmallCraft,
    SmallCraftWinds,
    SmallCraftRoughBar,
    HighWaterLevel,
    Ashfall,
    FreezingFog,
    DenseFog,
    DenseSmoke,
    BlowingDust,
    HardFreeze,
    Freeze,
    Frost,
    FireWeather,
    Flood,
    RipTide,
    HighSurf,
    Smog,
    AirQuality,
    BriskWind,
    AirStagnation,
    LowWater,
    Hydrological,
    SpecialWeather,
    UnknownVariant(u8),
}
impl WeatherSevereType {
    pub fn as_u8(self) -> u8 {
        match self {
            WeatherSevereType::Unspecified => 0,
            WeatherSevereType::Tornado => 1,
            WeatherSevereType::Tsunami => 2,
            WeatherSevereType::Hurricane => 3,
            WeatherSevereType::ExtremeWind => 4,
            WeatherSevereType::Typhoon => 5,
            WeatherSevereType::InlandHurricane => 6,
            WeatherSevereType::HurricaneForceWind => 7,
            WeatherSevereType::Waterspout => 8,
            WeatherSevereType::SevereThunderstorm => 9,
            WeatherSevereType::WreckhouseWinds => 10,
            WeatherSevereType::LesSuetesWind => 11,
            WeatherSevereType::Avalanche => 12,
            WeatherSevereType::FlashFlood => 13,
            WeatherSevereType::TropicalStorm => 14,
            WeatherSevereType::InlandTropicalStorm => 15,
            WeatherSevereType::Blizzard => 16,
            WeatherSevereType::IceStorm => 17,
            WeatherSevereType::FreezingRain => 18,
            WeatherSevereType::DebrisFlow => 19,
            WeatherSevereType::FlashFreeze => 20,
            WeatherSevereType::DustStorm => 21,
            WeatherSevereType::HighWind => 22,
            WeatherSevereType::WinterStorm => 23,
            WeatherSevereType::HeavyFreezingSpray => 24,
            WeatherSevereType::ExtremeCold => 25,
            WeatherSevereType::WindChill => 26,
            WeatherSevereType::ColdWave => 27,
            WeatherSevereType::HeavySnowAlert => 28,
            WeatherSevereType::LakeEffectBlowingSnow => 29,
            WeatherSevereType::SnowSquall => 30,
            WeatherSevereType::LakeEffectSnow => 31,
            WeatherSevereType::WinterWeather => 32,
            WeatherSevereType::Sleet => 33,
            WeatherSevereType::Snowfall => 34,
            WeatherSevereType::SnowAndBlowingSnow => 35,
            WeatherSevereType::BlowingSnow => 36,
            WeatherSevereType::SnowAlert => 37,
            WeatherSevereType::ArcticOutflow => 38,
            WeatherSevereType::FreezingDrizzle => 39,
            WeatherSevereType::Storm => 40,
            WeatherSevereType::StormSurge => 41,
            WeatherSevereType::Rainfall => 42,
            WeatherSevereType::ArealFlood => 43,
            WeatherSevereType::CoastalFlood => 44,
            WeatherSevereType::LakeshoreFlood => 45,
            WeatherSevereType::ExcessiveHeat => 46,
            WeatherSevereType::Heat => 47,
            WeatherSevereType::Weather => 48,
            WeatherSevereType::HighHeatAndHumidity => 49,
            WeatherSevereType::HumidexAndHealth => 50,
            WeatherSevereType::Humidex => 51,
            WeatherSevereType::Gale => 52,
            WeatherSevereType::FreezingSpray => 53,
            WeatherSevereType::SpecialMarine => 54,
            WeatherSevereType::Squall => 55,
            WeatherSevereType::StrongWind => 56,
            WeatherSevereType::LakeWind => 57,
            WeatherSevereType::MarineWeather => 58,
            WeatherSevereType::Wind => 59,
            WeatherSevereType::SmallCraftHazardousSeas => 60,
            WeatherSevereType::HazardousSeas => 61,
            WeatherSevereType::SmallCraft => 62,
            WeatherSevereType::SmallCraftWinds => 63,
            WeatherSevereType::SmallCraftRoughBar => 64,
            WeatherSevereType::HighWaterLevel => 65,
            WeatherSevereType::Ashfall => 66,
            WeatherSevereType::FreezingFog => 67,
            WeatherSevereType::DenseFog => 68,
            WeatherSevereType::DenseSmoke => 69,
            WeatherSevereType::BlowingDust => 70,
            WeatherSevereType::HardFreeze => 71,
            WeatherSevereType::Freeze => 72,
            WeatherSevereType::Frost => 73,
            WeatherSevereType::FireWeather => 74,
            WeatherSevereType::Flood => 75,
            WeatherSevereType::RipTide => 76,
            WeatherSevereType::HighSurf => 77,
            WeatherSevereType::Smog => 78,
            WeatherSevereType::AirQuality => 79,
            WeatherSevereType::BriskWind => 80,
            WeatherSevereType::AirStagnation => 81,
            WeatherSevereType::LowWater => 82,
            WeatherSevereType::Hydrological => 83,
            WeatherSevereType::SpecialWeather => 84,
            WeatherSevereType::UnknownVariant(value) => value,
        }
    }
    pub fn as_i64(self) -> i64 {
        self.as_u8() as i64
    }
}
impl fmt::Display for WeatherSevereType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            WeatherSevereType::Unspecified => writeln!(f, "unspecified"),
            WeatherSevereType::Tornado => writeln!(f, "tornado"),
            WeatherSevereType::Tsunami => writeln!(f, "tsunami"),
            WeatherSevereType::Hurricane => writeln!(f, "hurricane"),
            WeatherSevereType::ExtremeWind => writeln!(f, "extreme_wind"),
            WeatherSevereType::Typhoon => writeln!(f, "typhoon"),
            WeatherSevereType::InlandHurricane => writeln!(f, "inland_hurricane"),
            WeatherSevereType::HurricaneForceWind => writeln!(f, "hurricane_force_wind"),
            WeatherSevereType::Waterspout => writeln!(f, "waterspout"),
            WeatherSevereType::SevereThunderstorm => writeln!(f, "severe_thunderstorm"),
            WeatherSevereType::WreckhouseWinds => writeln!(f, "wreckhouse_winds"),
            WeatherSevereType::LesSuetesWind => writeln!(f, "les_suetes_wind"),
            WeatherSevereType::Avalanche => writeln!(f, "avalanche"),
            WeatherSevereType::FlashFlood => writeln!(f, "flash_flood"),
            WeatherSevereType::TropicalStorm => writeln!(f, "tropical_storm"),
            WeatherSevereType::InlandTropicalStorm => writeln!(f, "inland_tropical_storm"),
            WeatherSevereType::Blizzard => writeln!(f, "blizzard"),
            WeatherSevereType::IceStorm => writeln!(f, "ice_storm"),
            WeatherSevereType::FreezingRain => writeln!(f, "freezing_rain"),
            WeatherSevereType::DebrisFlow => writeln!(f, "debris_flow"),
            WeatherSevereType::FlashFreeze => writeln!(f, "flash_freeze"),
            WeatherSevereType::DustStorm => writeln!(f, "dust_storm"),
            WeatherSevereType::HighWind => writeln!(f, "high_wind"),
            WeatherSevereType::WinterStorm => writeln!(f, "winter_storm"),
            WeatherSevereType::HeavyFreezingSpray => writeln!(f, "heavy_freezing_spray"),
            WeatherSevereType::ExtremeCold => writeln!(f, "extreme_cold"),
            WeatherSevereType::WindChill => writeln!(f, "wind_chill"),
            WeatherSevereType::ColdWave => writeln!(f, "cold_wave"),
            WeatherSevereType::HeavySnowAlert => writeln!(f, "heavy_snow_alert"),
            WeatherSevereType::LakeEffectBlowingSnow => writeln!(f, "lake_effect_blowing_snow"),
            WeatherSevereType::SnowSquall => writeln!(f, "snow_squall"),
            WeatherSevereType::LakeEffectSnow => writeln!(f, "lake_effect_snow"),
            WeatherSevereType::WinterWeather => writeln!(f, "winter_weather"),
            WeatherSevereType::Sleet => writeln!(f, "sleet"),
            WeatherSevereType::Snowfall => writeln!(f, "snowfall"),
            WeatherSevereType::SnowAndBlowingSnow => writeln!(f, "snow_and_blowing_snow"),
            WeatherSevereType::BlowingSnow => writeln!(f, "blowing_snow"),
            WeatherSevereType::SnowAlert => writeln!(f, "snow_alert"),
            WeatherSevereType::ArcticOutflow => writeln!(f, "arctic_outflow"),
            WeatherSevereType::FreezingDrizzle => writeln!(f, "freezing_drizzle"),
            WeatherSevereType::Storm => writeln!(f, "storm"),
            WeatherSevereType::StormSurge => writeln!(f, "storm_surge"),
            WeatherSevereType::Rainfall => writeln!(f, "rainfall"),
            WeatherSevereType::ArealFlood => writeln!(f, "areal_flood"),
            WeatherSevereType::CoastalFlood => writeln!(f, "coastal_flood"),
            WeatherSevereType::LakeshoreFlood => writeln!(f, "lakeshore_flood"),
            WeatherSevereType::ExcessiveHeat => writeln!(f, "excessive_heat"),
            WeatherSevereType::Heat => writeln!(f, "heat"),
            WeatherSevereType::Weather => writeln!(f, "weather"),
            WeatherSevereType::HighHeatAndHumidity => writeln!(f, "high_heat_and_humidity"),
            WeatherSevereType::HumidexAndHealth => writeln!(f, "humidex_and_health"),
            WeatherSevereType::Humidex => writeln!(f, "humidex"),
            WeatherSevereType::Gale => writeln!(f, "gale"),
            WeatherSevereType::FreezingSpray => writeln!(f, "freezing_spray"),
            WeatherSevereType::SpecialMarine => writeln!(f, "special_marine"),
            WeatherSevereType::Squall => writeln!(f, "squall"),
            WeatherSevereType::StrongWind => writeln!(f, "strong_wind"),
            WeatherSevereType::LakeWind => writeln!(f, "lake_wind"),
            WeatherSevereType::MarineWeather => writeln!(f, "marine_weather"),
            WeatherSevereType::Wind => writeln!(f, "wind"),
            WeatherSevereType::SmallCraftHazardousSeas => writeln!(f, "small_craft_hazardous_seas"),
            WeatherSevereType::HazardousSeas => writeln!(f, "hazardous_seas"),
            WeatherSevereType::SmallCraft => writeln!(f, "small_craft"),
            WeatherSevereType::SmallCraftWinds => writeln!(f, "small_craft_winds"),
            WeatherSevereType::SmallCraftRoughBar => writeln!(f, "small_craft_rough_bar"),
            WeatherSevereType::HighWaterLevel => writeln!(f, "high_water_level"),
            WeatherSevereType::Ashfall => writeln!(f, "ashfall"),
            WeatherSevereType::FreezingFog => writeln!(f, "freezing_fog"),
            WeatherSevereType::DenseFog => writeln!(f, "dense_fog"),
            WeatherSevereType::DenseSmoke => writeln!(f, "dense_smoke"),
            WeatherSevereType::BlowingDust => writeln!(f, "blowing_dust"),
            WeatherSevereType::HardFreeze => writeln!(f, "hard_freeze"),
            WeatherSevereType::Freeze => writeln!(f, "freeze"),
            WeatherSevereType::Frost => writeln!(f, "frost"),
            WeatherSevereType::FireWeather => writeln!(f, "fire_weather"),
            WeatherSevereType::Flood => writeln!(f, "flood"),
            WeatherSevereType::RipTide => writeln!(f, "rip_tide"),
            WeatherSevereType::HighSurf => writeln!(f, "high_surf"),
            WeatherSevereType::Smog => writeln!(f, "smog"),
            WeatherSevereType::AirQuality => writeln!(f, "air_quality"),
            WeatherSevereType::BriskWind => writeln!(f, "brisk_wind"),
            WeatherSevereType::AirStagnation => writeln!(f, "air_stagnation"),
            WeatherSevereType::LowWater => writeln!(f, "low_water"),
            WeatherSevereType::Hydrological => writeln!(f, "hydrological"),
            WeatherSevereType::SpecialWeather => writeln!(f, "special_weather"),
            WeatherSevereType::UnknownVariant(value) => writeln!(f, "unknown_variant_{}", *value),
        }
    }
}
impl convert::From<u8> for WeatherSevereType {
    fn from(value: u8) -> Self {
        match value {
            0 => WeatherSevereType::Unspecified,
            1 => WeatherSevereType::Tornado,
            2 => WeatherSevereType::Tsunami,
            3 => WeatherSevereType::Hurricane,
            4 => WeatherSevereType::ExtremeWind,
            5 => WeatherSevereType::Typhoon,
            6 => WeatherSevereType::InlandHurricane,
            7 => WeatherSevereType::HurricaneForceWind,
            8 => WeatherSevereType::Waterspout,
            9 => WeatherSevereType::SevereThunderstorm,
            10 => WeatherSevereType::WreckhouseWinds,
            11 => WeatherSevereType::LesSuetesWind,
            12 => WeatherSevereType::Avalanche,
            13 => WeatherSevereType::FlashFlood,
            14 => WeatherSevereType::TropicalStorm,
            15 => WeatherSevereType::InlandTropicalStorm,
            16 => WeatherSevereType::Blizzard,
            17 => WeatherSevereType::IceStorm,
            18 => WeatherSevereType::FreezingRain,
            19 => WeatherSevereType::DebrisFlow,
            20 => WeatherSevereType::FlashFreeze,
            21 => WeatherSevereType::DustStorm,
            22 => WeatherSevereType::HighWind,
            23 => WeatherSevereType::WinterStorm,
            24 => WeatherSevereType::HeavyFreezingSpray,
            25 => WeatherSevereType::ExtremeCold,
            26 => WeatherSevereType::WindChill,
            27 => WeatherSevereType::ColdWave,
            28 => WeatherSevereType::HeavySnowAlert,
            29 => WeatherSevereType::LakeEffectBlowingSnow,
            30 => WeatherSevereType::SnowSquall,
            31 => WeatherSevereType::LakeEffectSnow,
            32 => WeatherSevereType::WinterWeather,
            33 => WeatherSevereType::Sleet,
            34 => WeatherSevereType::Snowfall,
            35 => WeatherSevereType::SnowAndBlowingSnow,
            36 => WeatherSevereType::BlowingSnow,
            37 => WeatherSevereType::SnowAlert,
            38 => WeatherSevereType::ArcticOutflow,
            39 => WeatherSevereType::FreezingDrizzle,
            40 => WeatherSevereType::Storm,
            41 => WeatherSevereType::StormSurge,
            42 => WeatherSevereType::Rainfall,
            43 => WeatherSevereType::ArealFlood,
            44 => WeatherSevereType::CoastalFlood,
            45 => WeatherSevereType::LakeshoreFlood,
            46 => WeatherSevereType::ExcessiveHeat,
            47 => WeatherSevereType::Heat,
            48 => WeatherSevereType::Weather,
            49 => WeatherSevereType::HighHeatAndHumidity,
            50 => WeatherSevereType::HumidexAndHealth,
            51 => WeatherSevereType::Humidex,
            52 => WeatherSevereType::Gale,
            53 => WeatherSevereType::FreezingSpray,
            54 => WeatherSevereType::SpecialMarine,
            55 => WeatherSevereType::Squall,
            56 => WeatherSevereType::StrongWind,
            57 => WeatherSevereType::LakeWind,
            58 => WeatherSevereType::MarineWeather,
            59 => WeatherSevereType::Wind,
            60 => WeatherSevereType::SmallCraftHazardousSeas,
            61 => WeatherSevereType::HazardousSeas,
            62 => WeatherSevereType::SmallCraft,
            63 => WeatherSevereType::SmallCraftWinds,
            64 => WeatherSevereType::SmallCraftRoughBar,
            65 => WeatherSevereType::HighWaterLevel,
            66 => WeatherSevereType::Ashfall,
            67 => WeatherSevereType::FreezingFog,
            68 => WeatherSevereType::DenseFog,
            69 => WeatherSevereType::DenseSmoke,
            70 => WeatherSevereType::BlowingDust,
            71 => WeatherSevereType::HardFreeze,
            72 => WeatherSevereType::Freeze,
            73 => WeatherSevereType::Frost,
            74 => WeatherSevereType::FireWeather,
            75 => WeatherSevereType::Flood,
            76 => WeatherSevereType::RipTide,
            77 => WeatherSevereType::HighSurf,
            78 => WeatherSevereType::Smog,
            79 => WeatherSevereType::AirQuality,
            80 => WeatherSevereType::BriskWind,
            81 => WeatherSevereType::AirStagnation,
            82 => WeatherSevereType::LowWater,
            83 => WeatherSevereType::Hydrological,
            84 => WeatherSevereType::SpecialWeather,
            _ => WeatherSevereType::UnknownVariant(value),
        }
    }
}
impl convert::From<i64> for WeatherSevereType {
    fn from(value: i64) -> Self {
        WeatherSevereType::from(value as u8)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum StrokeType {
    NoEvent,
    /// stroke was detected but cannot be identified
    Other,
    Serve,
    Forehand,
    Backhand,
    Smash,
    UnknownVariant(u8),
}
impl StrokeType {
    pub fn as_u8(self) -> u8 {
        match self {
            StrokeType::NoEvent => 0,
            StrokeType::Other => 1,
            StrokeType::Serve => 2,
            StrokeType::Forehand => 3,
            StrokeType::Backhand => 4,
            StrokeType::Smash => 5,
            StrokeType::UnknownVariant(value) => value,
        }
    }
    pub fn as_i64(self) -> i64 {
        self.as_u8() as i64
    }
}
impl fmt::Display for StrokeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            StrokeType::NoEvent => writeln!(f, "no_event"),
            StrokeType::Other => writeln!(f, "other"),
            StrokeType::Serve => writeln!(f, "serve"),
            StrokeType::Forehand => writeln!(f, "forehand"),
            StrokeType::Backhand => writeln!(f, "backhand"),
            StrokeType::Smash => writeln!(f, "smash"),
            StrokeType::UnknownVariant(value) => writeln!(f, "unknown_variant_{}", *value),
        }
    }
}
impl convert::From<u8> for StrokeType {
    fn from(value: u8) -> Self {
        match value {
            0 => StrokeType::NoEvent,
            1 => StrokeType::Other,
            2 => StrokeType::Serve,
            3 => StrokeType::Forehand,
            4 => StrokeType::Backhand,
            5 => StrokeType::Smash,
            _ => StrokeType::UnknownVariant(value),
        }
    }
}
impl convert::From<i64> for StrokeType {
    fn from(value: i64) -> Self {
        StrokeType::from(value as u8)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum BodyLocation {
    LeftLeg,
    LeftCalf,
    LeftShin,
    LeftHamstring,
    LeftQuad,
    LeftGlute,
    RightLeg,
    RightCalf,
    RightShin,
    RightHamstring,
    RightQuad,
    RightGlute,
    TorsoBack,
    LeftLowerBack,
    LeftUpperBack,
    RightLowerBack,
    RightUpperBack,
    TorsoFront,
    LeftAbdomen,
    LeftChest,
    RightAbdomen,
    RightChest,
    LeftArm,
    LeftShoulder,
    LeftBicep,
    LeftTricep,
    /// Left anterior forearm
    LeftBrachioradialis,
    /// Left posterior forearm
    LeftForearmExtensors,
    RightArm,
    RightShoulder,
    RightBicep,
    RightTricep,
    /// Right anterior forearm
    RightBrachioradialis,
    /// Right posterior forearm
    RightForearmExtensors,
    Neck,
    Throat,
    WaistMidBack,
    WaistFront,
    WaistLeft,
    WaistRight,
    UnknownVariant(u8),
}
impl BodyLocation {
    pub fn as_u8(self) -> u8 {
        match self {
            BodyLocation::LeftLeg => 0,
            BodyLocation::LeftCalf => 1,
            BodyLocation::LeftShin => 2,
            BodyLocation::LeftHamstring => 3,
            BodyLocation::LeftQuad => 4,
            BodyLocation::LeftGlute => 5,
            BodyLocation::RightLeg => 6,
            BodyLocation::RightCalf => 7,
            BodyLocation::RightShin => 8,
            BodyLocation::RightHamstring => 9,
            BodyLocation::RightQuad => 10,
            BodyLocation::RightGlute => 11,
            BodyLocation::TorsoBack => 12,
            BodyLocation::LeftLowerBack => 13,
            BodyLocation::LeftUpperBack => 14,
            BodyLocation::RightLowerBack => 15,
            BodyLocation::RightUpperBack => 16,
            BodyLocation::TorsoFront => 17,
            BodyLocation::LeftAbdomen => 18,
            BodyLocation::LeftChest => 19,
            BodyLocation::RightAbdomen => 20,
            BodyLocation::RightChest => 21,
            BodyLocation::LeftArm => 22,
            BodyLocation::LeftShoulder => 23,
            BodyLocation::LeftBicep => 24,
            BodyLocation::LeftTricep => 25,
            BodyLocation::LeftBrachioradialis => 26,
            BodyLocation::LeftForearmExtensors => 27,
            BodyLocation::RightArm => 28,
            BodyLocation::RightShoulder => 29,
            BodyLocation::RightBicep => 30,
            BodyLocation::RightTricep => 31,
            BodyLocation::RightBrachioradialis => 32,
            BodyLocation::RightForearmExtensors => 33,
            BodyLocation::Neck => 34,
            BodyLocation::Throat => 35,
            BodyLocation::WaistMidBack => 36,
            BodyLocation::WaistFront => 37,
            BodyLocation::WaistLeft => 38,
            BodyLocation::WaistRight => 39,
            BodyLocation::UnknownVariant(value) => value,
        }
    }
    pub fn as_i64(self) -> i64 {
        self.as_u8() as i64
    }
}
impl fmt::Display for BodyLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            BodyLocation::LeftLeg => writeln!(f, "left_leg"),
            BodyLocation::LeftCalf => writeln!(f, "left_calf"),
            BodyLocation::LeftShin => writeln!(f, "left_shin"),
            BodyLocation::LeftHamstring => writeln!(f, "left_hamstring"),
            BodyLocation::LeftQuad => writeln!(f, "left_quad"),
            BodyLocation::LeftGlute => writeln!(f, "left_glute"),
            BodyLocation::RightLeg => writeln!(f, "right_leg"),
            BodyLocation::RightCalf => writeln!(f, "right_calf"),
            BodyLocation::RightShin => writeln!(f, "right_shin"),
            BodyLocation::RightHamstring => writeln!(f, "right_hamstring"),
            BodyLocation::RightQuad => writeln!(f, "right_quad"),
            BodyLocation::RightGlute => writeln!(f, "right_glute"),
            BodyLocation::TorsoBack => writeln!(f, "torso_back"),
            BodyLocation::LeftLowerBack => writeln!(f, "left_lower_back"),
            BodyLocation::LeftUpperBack => writeln!(f, "left_upper_back"),
            BodyLocation::RightLowerBack => writeln!(f, "right_lower_back"),
            BodyLocation::RightUpperBack => writeln!(f, "right_upper_back"),
            BodyLocation::TorsoFront => writeln!(f, "torso_front"),
            BodyLocation::LeftAbdomen => writeln!(f, "left_abdomen"),
            BodyLocation::LeftChest => writeln!(f, "left_chest"),
            BodyLocation::RightAbdomen => writeln!(f, "right_abdomen"),
            BodyLocation::RightChest => writeln!(f, "right_chest"),
            BodyLocation::LeftArm => writeln!(f, "left_arm"),
            BodyLocation::LeftShoulder => writeln!(f, "left_shoulder"),
            BodyLocation::LeftBicep => writeln!(f, "left_bicep"),
            BodyLocation::LeftTricep => writeln!(f, "left_tricep"),
            BodyLocation::LeftBrachioradialis => writeln!(f, "left_brachioradialis"),
            BodyLocation::LeftForearmExtensors => writeln!(f, "left_forearm_extensors"),
            BodyLocation::RightArm => writeln!(f, "right_arm"),
            BodyLocation::RightShoulder => writeln!(f, "right_shoulder"),
            BodyLocation::RightBicep => writeln!(f, "right_bicep"),
            BodyLocation::RightTricep => writeln!(f, "right_tricep"),
            BodyLocation::RightBrachioradialis => writeln!(f, "right_brachioradialis"),
            BodyLocation::RightForearmExtensors => writeln!(f, "right_forearm_extensors"),
            BodyLocation::Neck => writeln!(f, "neck"),
            BodyLocation::Throat => writeln!(f, "throat"),
            BodyLocation::WaistMidBack => writeln!(f, "waist_mid_back"),
            BodyLocation::WaistFront => writeln!(f, "waist_front"),
            BodyLocation::WaistLeft => writeln!(f, "waist_left"),
            BodyLocation::WaistRight => writeln!(f, "waist_right"),
            BodyLocation::UnknownVariant(value) => writeln!(f, "unknown_variant_{}", *value),
        }
    }
}
impl convert::From<u8> for BodyLocation {
    fn from(value: u8) -> Self {
        match value {
            0 => BodyLocation::LeftLeg,
            1 => BodyLocation::LeftCalf,
            2 => BodyLocation::LeftShin,
            3 => BodyLocation::LeftHamstring,
            4 => BodyLocation::LeftQuad,
            5 => BodyLocation::LeftGlute,
            6 => BodyLocation::RightLeg,
            7 => BodyLocation::RightCalf,
            8 => BodyLocation::RightShin,
            9 => BodyLocation::RightHamstring,
            10 => BodyLocation::RightQuad,
            11 => BodyLocation::RightGlute,
            12 => BodyLocation::TorsoBack,
            13 => BodyLocation::LeftLowerBack,
            14 => BodyLocation::LeftUpperBack,
            15 => BodyLocation::RightLowerBack,
            16 => BodyLocation::RightUpperBack,
            17 => BodyLocation::TorsoFront,
            18 => BodyLocation::LeftAbdomen,
            19 => BodyLocation::LeftChest,
            20 => BodyLocation::RightAbdomen,
            21 => BodyLocation::RightChest,
            22 => BodyLocation::LeftArm,
            23 => BodyLocation::LeftShoulder,
            24 => BodyLocation::LeftBicep,
            25 => BodyLocation::LeftTricep,
            26 => BodyLocation::LeftBrachioradialis,
            27 => BodyLocation::LeftForearmExtensors,
            28 => BodyLocation::RightArm,
            29 => BodyLocation::RightShoulder,
            30 => BodyLocation::RightBicep,
            31 => BodyLocation::RightTricep,
            32 => BodyLocation::RightBrachioradialis,
            33 => BodyLocation::RightForearmExtensors,
            34 => BodyLocation::Neck,
            35 => BodyLocation::Throat,
            36 => BodyLocation::WaistMidBack,
            37 => BodyLocation::WaistFront,
            38 => BodyLocation::WaistLeft,
            39 => BodyLocation::WaistRight,
            _ => BodyLocation::UnknownVariant(value),
        }
    }
}
impl convert::From<i64> for BodyLocation {
    fn from(value: i64) -> Self {
        BodyLocation::from(value as u8)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum SegmentLapStatus {
    End,
    Fail,
    UnknownVariant(u8),
}
impl SegmentLapStatus {
    pub fn as_u8(self) -> u8 {
        match self {
            SegmentLapStatus::End => 0,
            SegmentLapStatus::Fail => 1,
            SegmentLapStatus::UnknownVariant(value) => value,
        }
    }
    pub fn as_i64(self) -> i64 {
        self.as_u8() as i64
    }
}
impl fmt::Display for SegmentLapStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            SegmentLapStatus::End => writeln!(f, "end"),
            SegmentLapStatus::Fail => writeln!(f, "fail"),
            SegmentLapStatus::UnknownVariant(value) => writeln!(f, "unknown_variant_{}", *value),
        }
    }
}
impl convert::From<u8> for SegmentLapStatus {
    fn from(value: u8) -> Self {
        match value {
            0 => SegmentLapStatus::End,
            1 => SegmentLapStatus::Fail,
            _ => SegmentLapStatus::UnknownVariant(value),
        }
    }
}
impl convert::From<i64> for SegmentLapStatus {
    fn from(value: i64) -> Self {
        SegmentLapStatus::from(value as u8)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum SegmentLeaderboardType {
    Overall,
    PersonalBest,
    Connections,
    Group,
    Challenger,
    Kom,
    Qom,
    Pr,
    Goal,
    Rival,
    ClubLeader,
    UnknownVariant(u8),
}
impl SegmentLeaderboardType {
    pub fn as_u8(self) -> u8 {
        match self {
            SegmentLeaderboardType::Overall => 0,
            SegmentLeaderboardType::PersonalBest => 1,
            SegmentLeaderboardType::Connections => 2,
            SegmentLeaderboardType::Group => 3,
            SegmentLeaderboardType::Challenger => 4,
            SegmentLeaderboardType::Kom => 5,
            SegmentLeaderboardType::Qom => 6,
            SegmentLeaderboardType::Pr => 7,
            SegmentLeaderboardType::Goal => 8,
            SegmentLeaderboardType::Rival => 9,
            SegmentLeaderboardType::ClubLeader => 10,
            SegmentLeaderboardType::UnknownVariant(value) => value,
        }
    }
    pub fn as_i64(self) -> i64 {
        self.as_u8() as i64
    }
}
impl fmt::Display for SegmentLeaderboardType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            SegmentLeaderboardType::Overall => writeln!(f, "overall"),
            SegmentLeaderboardType::PersonalBest => writeln!(f, "personal_best"),
            SegmentLeaderboardType::Connections => writeln!(f, "connections"),
            SegmentLeaderboardType::Group => writeln!(f, "group"),
            SegmentLeaderboardType::Challenger => writeln!(f, "challenger"),
            SegmentLeaderboardType::Kom => writeln!(f, "kom"),
            SegmentLeaderboardType::Qom => writeln!(f, "qom"),
            SegmentLeaderboardType::Pr => writeln!(f, "pr"),
            SegmentLeaderboardType::Goal => writeln!(f, "goal"),
            SegmentLeaderboardType::Rival => writeln!(f, "rival"),
            SegmentLeaderboardType::ClubLeader => writeln!(f, "club_leader"),
            SegmentLeaderboardType::UnknownVariant(value) => {
                writeln!(f, "unknown_variant_{}", *value)
            }
        }
    }
}
impl convert::From<u8> for SegmentLeaderboardType {
    fn from(value: u8) -> Self {
        match value {
            0 => SegmentLeaderboardType::Overall,
            1 => SegmentLeaderboardType::PersonalBest,
            2 => SegmentLeaderboardType::Connections,
            3 => SegmentLeaderboardType::Group,
            4 => SegmentLeaderboardType::Challenger,
            5 => SegmentLeaderboardType::Kom,
            6 => SegmentLeaderboardType::Qom,
            7 => SegmentLeaderboardType::Pr,
            8 => SegmentLeaderboardType::Goal,
            9 => SegmentLeaderboardType::Rival,
            10 => SegmentLeaderboardType::ClubLeader,
            _ => SegmentLeaderboardType::UnknownVariant(value),
        }
    }
}
impl convert::From<i64> for SegmentLeaderboardType {
    fn from(value: i64) -> Self {
        SegmentLeaderboardType::from(value as u8)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum SegmentDeleteStatus {
    DoNotDelete,
    DeleteOne,
    DeleteAll,
    UnknownVariant(u8),
}
impl SegmentDeleteStatus {
    pub fn as_u8(self) -> u8 {
        match self {
            SegmentDeleteStatus::DoNotDelete => 0,
            SegmentDeleteStatus::DeleteOne => 1,
            SegmentDeleteStatus::DeleteAll => 2,
            SegmentDeleteStatus::UnknownVariant(value) => value,
        }
    }
    pub fn as_i64(self) -> i64 {
        self.as_u8() as i64
    }
}
impl fmt::Display for SegmentDeleteStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            SegmentDeleteStatus::DoNotDelete => writeln!(f, "do_not_delete"),
            SegmentDeleteStatus::DeleteOne => writeln!(f, "delete_one"),
            SegmentDeleteStatus::DeleteAll => writeln!(f, "delete_all"),
            SegmentDeleteStatus::UnknownVariant(value) => writeln!(f, "unknown_variant_{}", *value),
        }
    }
}
impl convert::From<u8> for SegmentDeleteStatus {
    fn from(value: u8) -> Self {
        match value {
            0 => SegmentDeleteStatus::DoNotDelete,
            1 => SegmentDeleteStatus::DeleteOne,
            2 => SegmentDeleteStatus::DeleteAll,
            _ => SegmentDeleteStatus::UnknownVariant(value),
        }
    }
}
impl convert::From<i64> for SegmentDeleteStatus {
    fn from(value: i64) -> Self {
        SegmentDeleteStatus::from(value as u8)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum SegmentSelectionType {
    Starred,
    Suggested,
    UnknownVariant(u8),
}
impl SegmentSelectionType {
    pub fn as_u8(self) -> u8 {
        match self {
            SegmentSelectionType::Starred => 0,
            SegmentSelectionType::Suggested => 1,
            SegmentSelectionType::UnknownVariant(value) => value,
        }
    }
    pub fn as_i64(self) -> i64 {
        self.as_u8() as i64
    }
}
impl fmt::Display for SegmentSelectionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            SegmentSelectionType::Starred => writeln!(f, "starred"),
            SegmentSelectionType::Suggested => writeln!(f, "suggested"),
            SegmentSelectionType::UnknownVariant(value) => {
                writeln!(f, "unknown_variant_{}", *value)
            }
        }
    }
}
impl convert::From<u8> for SegmentSelectionType {
    fn from(value: u8) -> Self {
        match value {
            0 => SegmentSelectionType::Starred,
            1 => SegmentSelectionType::Suggested,
            _ => SegmentSelectionType::UnknownVariant(value),
        }
    }
}
impl convert::From<i64> for SegmentSelectionType {
    fn from(value: i64) -> Self {
        SegmentSelectionType::from(value as u8)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum SourceType {
    /// External device connected with ANT
    Ant,
    /// External device connected with ANT+
    Antplus,
    /// External device connected with BT
    Bluetooth,
    /// External device connected with BLE
    BluetoothLowEnergy,
    /// External device connected with Wifi
    Wifi,
    /// Onboard device
    Local,
    UnknownVariant(u8),
}
impl SourceType {
    pub fn as_u8(self) -> u8 {
        match self {
            SourceType::Ant => 0,
            SourceType::Antplus => 1,
            SourceType::Bluetooth => 2,
            SourceType::BluetoothLowEnergy => 3,
            SourceType::Wifi => 4,
            SourceType::Local => 5,
            SourceType::UnknownVariant(value) => value,
        }
    }
    pub fn as_i64(self) -> i64 {
        self.as_u8() as i64
    }
}
impl fmt::Display for SourceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            SourceType::Ant => writeln!(f, "ant"),
            SourceType::Antplus => writeln!(f, "antplus"),
            SourceType::Bluetooth => writeln!(f, "bluetooth"),
            SourceType::BluetoothLowEnergy => writeln!(f, "bluetooth_low_energy"),
            SourceType::Wifi => writeln!(f, "wifi"),
            SourceType::Local => writeln!(f, "local"),
            SourceType::UnknownVariant(value) => writeln!(f, "unknown_variant_{}", *value),
        }
    }
}
impl convert::From<u8> for SourceType {
    fn from(value: u8) -> Self {
        match value {
            0 => SourceType::Ant,
            1 => SourceType::Antplus,
            2 => SourceType::Bluetooth,
            3 => SourceType::BluetoothLowEnergy,
            4 => SourceType::Wifi,
            5 => SourceType::Local,
            _ => SourceType::UnknownVariant(value),
        }
    }
}
impl convert::From<i64> for SourceType {
    fn from(value: i64) -> Self {
        SourceType::from(value as u8)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum DisplayOrientation {
    /// automatic if the device supports it
    Auto,
    Portrait,
    Landscape,
    /// portrait mode but rotated 180 degrees
    PortraitFlipped,
    /// landscape mode but rotated 180 degrees
    LandscapeFlipped,
    UnknownVariant(u8),
}
impl DisplayOrientation {
    pub fn as_u8(self) -> u8 {
        match self {
            DisplayOrientation::Auto => 0,
            DisplayOrientation::Portrait => 1,
            DisplayOrientation::Landscape => 2,
            DisplayOrientation::PortraitFlipped => 3,
            DisplayOrientation::LandscapeFlipped => 4,
            DisplayOrientation::UnknownVariant(value) => value,
        }
    }
    pub fn as_i64(self) -> i64 {
        self.as_u8() as i64
    }
}
impl fmt::Display for DisplayOrientation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            DisplayOrientation::Auto => writeln!(f, "auto"),
            DisplayOrientation::Portrait => writeln!(f, "portrait"),
            DisplayOrientation::Landscape => writeln!(f, "landscape"),
            DisplayOrientation::PortraitFlipped => writeln!(f, "portrait_flipped"),
            DisplayOrientation::LandscapeFlipped => writeln!(f, "landscape_flipped"),
            DisplayOrientation::UnknownVariant(value) => writeln!(f, "unknown_variant_{}", *value),
        }
    }
}
impl convert::From<u8> for DisplayOrientation {
    fn from(value: u8) -> Self {
        match value {
            0 => DisplayOrientation::Auto,
            1 => DisplayOrientation::Portrait,
            2 => DisplayOrientation::Landscape,
            3 => DisplayOrientation::PortraitFlipped,
            4 => DisplayOrientation::LandscapeFlipped,
            _ => DisplayOrientation::UnknownVariant(value),
        }
    }
}
impl convert::From<i64> for DisplayOrientation {
    fn from(value: i64) -> Self {
        DisplayOrientation::from(value as u8)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum WorkoutEquipment {
    None,
    SwimFins,
    SwimKickboard,
    SwimPaddles,
    SwimPullBuoy,
    SwimSnorkel,
    UnknownVariant(u8),
}
impl WorkoutEquipment {
    pub fn as_u8(self) -> u8 {
        match self {
            WorkoutEquipment::None => 0,
            WorkoutEquipment::SwimFins => 1,
            WorkoutEquipment::SwimKickboard => 2,
            WorkoutEquipment::SwimPaddles => 3,
            WorkoutEquipment::SwimPullBuoy => 4,
            WorkoutEquipment::SwimSnorkel => 5,
            WorkoutEquipment::UnknownVariant(value) => value,
        }
    }
    pub fn as_i64(self) -> i64 {
        self.as_u8() as i64
    }
}
impl fmt::Display for WorkoutEquipment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            WorkoutEquipment::None => writeln!(f, "none"),
            WorkoutEquipment::SwimFins => writeln!(f, "swim_fins"),
            WorkoutEquipment::SwimKickboard => writeln!(f, "swim_kickboard"),
            WorkoutEquipment::SwimPaddles => writeln!(f, "swim_paddles"),
            WorkoutEquipment::SwimPullBuoy => writeln!(f, "swim_pull_buoy"),
            WorkoutEquipment::SwimSnorkel => writeln!(f, "swim_snorkel"),
            WorkoutEquipment::UnknownVariant(value) => writeln!(f, "unknown_variant_{}", *value),
        }
    }
}
impl convert::From<u8> for WorkoutEquipment {
    fn from(value: u8) -> Self {
        match value {
            0 => WorkoutEquipment::None,
            1 => WorkoutEquipment::SwimFins,
            2 => WorkoutEquipment::SwimKickboard,
            3 => WorkoutEquipment::SwimPaddles,
            4 => WorkoutEquipment::SwimPullBuoy,
            5 => WorkoutEquipment::SwimSnorkel,
            _ => WorkoutEquipment::UnknownVariant(value),
        }
    }
}
impl convert::From<i64> for WorkoutEquipment {
    fn from(value: i64) -> Self {
        WorkoutEquipment::from(value as u8)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum WatchfaceMode {
    Digital,
    Analog,
    ConnectIq,
    Disabled,
    UnknownVariant(u8),
}
impl WatchfaceMode {
    pub fn as_u8(self) -> u8 {
        match self {
            WatchfaceMode::Digital => 0,
            WatchfaceMode::Analog => 1,
            WatchfaceMode::ConnectIq => 2,
            WatchfaceMode::Disabled => 3,
            WatchfaceMode::UnknownVariant(value) => value,
        }
    }
    pub fn as_i64(self) -> i64 {
        self.as_u8() as i64
    }
}
impl fmt::Display for WatchfaceMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            WatchfaceMode::Digital => writeln!(f, "digital"),
            WatchfaceMode::Analog => writeln!(f, "analog"),
            WatchfaceMode::ConnectIq => writeln!(f, "connect_iq"),
            WatchfaceMode::Disabled => writeln!(f, "disabled"),
            WatchfaceMode::UnknownVariant(value) => writeln!(f, "unknown_variant_{}", *value),
        }
    }
}
impl convert::From<u8> for WatchfaceMode {
    fn from(value: u8) -> Self {
        match value {
            0 => WatchfaceMode::Digital,
            1 => WatchfaceMode::Analog,
            2 => WatchfaceMode::ConnectIq,
            3 => WatchfaceMode::Disabled,
            _ => WatchfaceMode::UnknownVariant(value),
        }
    }
}
impl convert::From<i64> for WatchfaceMode {
    fn from(value: i64) -> Self {
        WatchfaceMode::from(value as u8)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum DigitalWatchfaceLayout {
    Traditional,
    Modern,
    Bold,
    UnknownVariant(u8),
}
impl DigitalWatchfaceLayout {
    pub fn as_u8(self) -> u8 {
        match self {
            DigitalWatchfaceLayout::Traditional => 0,
            DigitalWatchfaceLayout::Modern => 1,
            DigitalWatchfaceLayout::Bold => 2,
            DigitalWatchfaceLayout::UnknownVariant(value) => value,
        }
    }
    pub fn as_i64(self) -> i64 {
        self.as_u8() as i64
    }
}
impl fmt::Display for DigitalWatchfaceLayout {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            DigitalWatchfaceLayout::Traditional => writeln!(f, "traditional"),
            DigitalWatchfaceLayout::Modern => writeln!(f, "modern"),
            DigitalWatchfaceLayout::Bold => writeln!(f, "bold"),
            DigitalWatchfaceLayout::UnknownVariant(value) => {
                writeln!(f, "unknown_variant_{}", *value)
            }
        }
    }
}
impl convert::From<u8> for DigitalWatchfaceLayout {
    fn from(value: u8) -> Self {
        match value {
            0 => DigitalWatchfaceLayout::Traditional,
            1 => DigitalWatchfaceLayout::Modern,
            2 => DigitalWatchfaceLayout::Bold,
            _ => DigitalWatchfaceLayout::UnknownVariant(value),
        }
    }
}
impl convert::From<i64> for DigitalWatchfaceLayout {
    fn from(value: i64) -> Self {
        DigitalWatchfaceLayout::from(value as u8)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum AnalogWatchfaceLayout {
    Minimal,
    Traditional,
    Modern,
    UnknownVariant(u8),
}
impl AnalogWatchfaceLayout {
    pub fn as_u8(self) -> u8 {
        match self {
            AnalogWatchfaceLayout::Minimal => 0,
            AnalogWatchfaceLayout::Traditional => 1,
            AnalogWatchfaceLayout::Modern => 2,
            AnalogWatchfaceLayout::UnknownVariant(value) => value,
        }
    }
    pub fn as_i64(self) -> i64 {
        self.as_u8() as i64
    }
}
impl fmt::Display for AnalogWatchfaceLayout {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            AnalogWatchfaceLayout::Minimal => writeln!(f, "minimal"),
            AnalogWatchfaceLayout::Traditional => writeln!(f, "traditional"),
            AnalogWatchfaceLayout::Modern => writeln!(f, "modern"),
            AnalogWatchfaceLayout::UnknownVariant(value) => {
                writeln!(f, "unknown_variant_{}", *value)
            }
        }
    }
}
impl convert::From<u8> for AnalogWatchfaceLayout {
    fn from(value: u8) -> Self {
        match value {
            0 => AnalogWatchfaceLayout::Minimal,
            1 => AnalogWatchfaceLayout::Traditional,
            2 => AnalogWatchfaceLayout::Modern,
            _ => AnalogWatchfaceLayout::UnknownVariant(value),
        }
    }
}
impl convert::From<i64> for AnalogWatchfaceLayout {
    fn from(value: i64) -> Self {
        AnalogWatchfaceLayout::from(value as u8)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum RiderPositionType {
    Seated,
    Standing,
    TransitionToSeated,
    TransitionToStanding,
    UnknownVariant(u8),
}
impl RiderPositionType {
    pub fn as_u8(self) -> u8 {
        match self {
            RiderPositionType::Seated => 0,
            RiderPositionType::Standing => 1,
            RiderPositionType::TransitionToSeated => 2,
            RiderPositionType::TransitionToStanding => 3,
            RiderPositionType::UnknownVariant(value) => value,
        }
    }
    pub fn as_i64(self) -> i64 {
        self.as_u8() as i64
    }
}
impl fmt::Display for RiderPositionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            RiderPositionType::Seated => writeln!(f, "seated"),
            RiderPositionType::Standing => writeln!(f, "standing"),
            RiderPositionType::TransitionToSeated => writeln!(f, "transition_to_seated"),
            RiderPositionType::TransitionToStanding => writeln!(f, "transition_to_standing"),
            RiderPositionType::UnknownVariant(value) => writeln!(f, "unknown_variant_{}", *value),
        }
    }
}
impl convert::From<u8> for RiderPositionType {
    fn from(value: u8) -> Self {
        match value {
            0 => RiderPositionType::Seated,
            1 => RiderPositionType::Standing,
            2 => RiderPositionType::TransitionToSeated,
            3 => RiderPositionType::TransitionToStanding,
            _ => RiderPositionType::UnknownVariant(value),
        }
    }
}
impl convert::From<i64> for RiderPositionType {
    fn from(value: i64) -> Self {
        RiderPositionType::from(value as u8)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum PowerPhaseType {
    PowerPhaseStartAngle,
    PowerPhaseEndAngle,
    PowerPhaseArcLength,
    PowerPhaseCenter,
    UnknownVariant(u8),
}
impl PowerPhaseType {
    pub fn as_u8(self) -> u8 {
        match self {
            PowerPhaseType::PowerPhaseStartAngle => 0,
            PowerPhaseType::PowerPhaseEndAngle => 1,
            PowerPhaseType::PowerPhaseArcLength => 2,
            PowerPhaseType::PowerPhaseCenter => 3,
            PowerPhaseType::UnknownVariant(value) => value,
        }
    }
    pub fn as_i64(self) -> i64 {
        self.as_u8() as i64
    }
}
impl fmt::Display for PowerPhaseType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            PowerPhaseType::PowerPhaseStartAngle => writeln!(f, "power_phase_start_angle"),
            PowerPhaseType::PowerPhaseEndAngle => writeln!(f, "power_phase_end_angle"),
            PowerPhaseType::PowerPhaseArcLength => writeln!(f, "power_phase_arc_length"),
            PowerPhaseType::PowerPhaseCenter => writeln!(f, "power_phase_center"),
            PowerPhaseType::UnknownVariant(value) => writeln!(f, "unknown_variant_{}", *value),
        }
    }
}
impl convert::From<u8> for PowerPhaseType {
    fn from(value: u8) -> Self {
        match value {
            0 => PowerPhaseType::PowerPhaseStartAngle,
            1 => PowerPhaseType::PowerPhaseEndAngle,
            2 => PowerPhaseType::PowerPhaseArcLength,
            3 => PowerPhaseType::PowerPhaseCenter,
            _ => PowerPhaseType::UnknownVariant(value),
        }
    }
}
impl convert::From<i64> for PowerPhaseType {
    fn from(value: i64) -> Self {
        PowerPhaseType::from(value as u8)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum CameraEventType {
    /// Start of video recording
    VideoStart,
    /// Mark of video file split (end of one file, beginning of the other)
    VideoSplit,
    /// End of video recording
    VideoEnd,
    /// Still photo taken
    PhotoTaken,
    VideoSecondStreamStart,
    VideoSecondStreamSplit,
    VideoSecondStreamEnd,
    /// Mark of video file split start
    VideoSplitStart,
    VideoSecondStreamSplitStart,
    /// Mark when a video recording has been paused
    VideoPause,
    VideoSecondStreamPause,
    /// Mark when a video recording has been resumed
    VideoResume,
    VideoSecondStreamResume,
    UnknownVariant(u8),
}
impl CameraEventType {
    pub fn as_u8(self) -> u8 {
        match self {
            CameraEventType::VideoStart => 0,
            CameraEventType::VideoSplit => 1,
            CameraEventType::VideoEnd => 2,
            CameraEventType::PhotoTaken => 3,
            CameraEventType::VideoSecondStreamStart => 4,
            CameraEventType::VideoSecondStreamSplit => 5,
            CameraEventType::VideoSecondStreamEnd => 6,
            CameraEventType::VideoSplitStart => 7,
            CameraEventType::VideoSecondStreamSplitStart => 8,
            CameraEventType::VideoPause => 11,
            CameraEventType::VideoSecondStreamPause => 12,
            CameraEventType::VideoResume => 13,
            CameraEventType::VideoSecondStreamResume => 14,
            CameraEventType::UnknownVariant(value) => value,
        }
    }
    pub fn as_i64(self) -> i64 {
        self.as_u8() as i64
    }
}
impl fmt::Display for CameraEventType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            CameraEventType::VideoStart => writeln!(f, "video_start"),
            CameraEventType::VideoSplit => writeln!(f, "video_split"),
            CameraEventType::VideoEnd => writeln!(f, "video_end"),
            CameraEventType::PhotoTaken => writeln!(f, "photo_taken"),
            CameraEventType::VideoSecondStreamStart => writeln!(f, "video_second_stream_start"),
            CameraEventType::VideoSecondStreamSplit => writeln!(f, "video_second_stream_split"),
            CameraEventType::VideoSecondStreamEnd => writeln!(f, "video_second_stream_end"),
            CameraEventType::VideoSplitStart => writeln!(f, "video_split_start"),
            CameraEventType::VideoSecondStreamSplitStart => {
                writeln!(f, "video_second_stream_split_start")
            }
            CameraEventType::VideoPause => writeln!(f, "video_pause"),
            CameraEventType::VideoSecondStreamPause => writeln!(f, "video_second_stream_pause"),
            CameraEventType::VideoResume => writeln!(f, "video_resume"),
            CameraEventType::VideoSecondStreamResume => writeln!(f, "video_second_stream_resume"),
            CameraEventType::UnknownVariant(value) => writeln!(f, "unknown_variant_{}", *value),
        }
    }
}
impl convert::From<u8> for CameraEventType {
    fn from(value: u8) -> Self {
        match value {
            0 => CameraEventType::VideoStart,
            1 => CameraEventType::VideoSplit,
            2 => CameraEventType::VideoEnd,
            3 => CameraEventType::PhotoTaken,
            4 => CameraEventType::VideoSecondStreamStart,
            5 => CameraEventType::VideoSecondStreamSplit,
            6 => CameraEventType::VideoSecondStreamEnd,
            7 => CameraEventType::VideoSplitStart,
            8 => CameraEventType::VideoSecondStreamSplitStart,
            11 => CameraEventType::VideoPause,
            12 => CameraEventType::VideoSecondStreamPause,
            13 => CameraEventType::VideoResume,
            14 => CameraEventType::VideoSecondStreamResume,
            _ => CameraEventType::UnknownVariant(value),
        }
    }
}
impl convert::From<i64> for CameraEventType {
    fn from(value: i64) -> Self {
        CameraEventType::from(value as u8)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum SensorType {
    Accelerometer,
    Gyroscope,
    /// Magnetometer
    Compass,
    Barometer,
    UnknownVariant(u8),
}
impl SensorType {
    pub fn as_u8(self) -> u8 {
        match self {
            SensorType::Accelerometer => 0,
            SensorType::Gyroscope => 1,
            SensorType::Compass => 2,
            SensorType::Barometer => 3,
            SensorType::UnknownVariant(value) => value,
        }
    }
    pub fn as_i64(self) -> i64 {
        self.as_u8() as i64
    }
}
impl fmt::Display for SensorType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            SensorType::Accelerometer => writeln!(f, "accelerometer"),
            SensorType::Gyroscope => writeln!(f, "gyroscope"),
            SensorType::Compass => writeln!(f, "compass"),
            SensorType::Barometer => writeln!(f, "barometer"),
            SensorType::UnknownVariant(value) => writeln!(f, "unknown_variant_{}", *value),
        }
    }
}
impl convert::From<u8> for SensorType {
    fn from(value: u8) -> Self {
        match value {
            0 => SensorType::Accelerometer,
            1 => SensorType::Gyroscope,
            2 => SensorType::Compass,
            3 => SensorType::Barometer,
            _ => SensorType::UnknownVariant(value),
        }
    }
}
impl convert::From<i64> for SensorType {
    fn from(value: i64) -> Self {
        SensorType::from(value as u8)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum BikeLightNetworkConfigType {
    Auto,
    Individual,
    HighVisibility,
    Trail,
    UnknownVariant(u8),
}
impl BikeLightNetworkConfigType {
    pub fn as_u8(self) -> u8 {
        match self {
            BikeLightNetworkConfigType::Auto => 0,
            BikeLightNetworkConfigType::Individual => 4,
            BikeLightNetworkConfigType::HighVisibility => 5,
            BikeLightNetworkConfigType::Trail => 6,
            BikeLightNetworkConfigType::UnknownVariant(value) => value,
        }
    }
    pub fn as_i64(self) -> i64 {
        self.as_u8() as i64
    }
}
impl fmt::Display for BikeLightNetworkConfigType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            BikeLightNetworkConfigType::Auto => writeln!(f, "auto"),
            BikeLightNetworkConfigType::Individual => writeln!(f, "individual"),
            BikeLightNetworkConfigType::HighVisibility => writeln!(f, "high_visibility"),
            BikeLightNetworkConfigType::Trail => writeln!(f, "trail"),
            BikeLightNetworkConfigType::UnknownVariant(value) => {
                writeln!(f, "unknown_variant_{}", *value)
            }
        }
    }
}
impl convert::From<u8> for BikeLightNetworkConfigType {
    fn from(value: u8) -> Self {
        match value {
            0 => BikeLightNetworkConfigType::Auto,
            4 => BikeLightNetworkConfigType::Individual,
            5 => BikeLightNetworkConfigType::HighVisibility,
            6 => BikeLightNetworkConfigType::Trail,
            _ => BikeLightNetworkConfigType::UnknownVariant(value),
        }
    }
}
impl convert::From<i64> for BikeLightNetworkConfigType {
    fn from(value: i64) -> Self {
        BikeLightNetworkConfigType::from(value as u8)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum CommTimeoutType {
    /// Timeout pairing to any device
    WildcardPairingTimeout,
    /// Timeout pairing to previously paired device
    PairingTimeout,
    /// Temporary loss of communications
    ConnectionLost,
    /// Connection closed due to extended bad communications
    ConnectionTimeout,
    UnknownVariant(u16),
}
impl CommTimeoutType {
    pub fn as_u16(self) -> u16 {
        match self {
            CommTimeoutType::WildcardPairingTimeout => 0,
            CommTimeoutType::PairingTimeout => 1,
            CommTimeoutType::ConnectionLost => 2,
            CommTimeoutType::ConnectionTimeout => 3,
            CommTimeoutType::UnknownVariant(value) => value,
        }
    }
    pub fn as_i64(self) -> i64 {
        self.as_u16() as i64
    }
}
impl fmt::Display for CommTimeoutType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            CommTimeoutType::WildcardPairingTimeout => writeln!(f, "wildcard_pairing_timeout"),
            CommTimeoutType::PairingTimeout => writeln!(f, "pairing_timeout"),
            CommTimeoutType::ConnectionLost => writeln!(f, "connection_lost"),
            CommTimeoutType::ConnectionTimeout => writeln!(f, "connection_timeout"),
            CommTimeoutType::UnknownVariant(value) => writeln!(f, "unknown_variant_{}", *value),
        }
    }
}
impl convert::From<u16> for CommTimeoutType {
    fn from(value: u16) -> Self {
        match value {
            0 => CommTimeoutType::WildcardPairingTimeout,
            1 => CommTimeoutType::PairingTimeout,
            2 => CommTimeoutType::ConnectionLost,
            3 => CommTimeoutType::ConnectionTimeout,
            _ => CommTimeoutType::UnknownVariant(value),
        }
    }
}
impl convert::From<i64> for CommTimeoutType {
    fn from(value: i64) -> Self {
        CommTimeoutType::from(value as u16)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum CameraOrientationType {
    CameraOrientation0,
    CameraOrientation90,
    CameraOrientation180,
    CameraOrientation270,
    UnknownVariant(u8),
}
impl CameraOrientationType {
    pub fn as_u8(self) -> u8 {
        match self {
            CameraOrientationType::CameraOrientation0 => 0,
            CameraOrientationType::CameraOrientation90 => 1,
            CameraOrientationType::CameraOrientation180 => 2,
            CameraOrientationType::CameraOrientation270 => 3,
            CameraOrientationType::UnknownVariant(value) => value,
        }
    }
    pub fn as_i64(self) -> i64 {
        self.as_u8() as i64
    }
}
impl fmt::Display for CameraOrientationType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            CameraOrientationType::CameraOrientation0 => writeln!(f, "camera_orientation_0"),
            CameraOrientationType::CameraOrientation90 => writeln!(f, "camera_orientation_90"),
            CameraOrientationType::CameraOrientation180 => writeln!(f, "camera_orientation_180"),
            CameraOrientationType::CameraOrientation270 => writeln!(f, "camera_orientation_270"),
            CameraOrientationType::UnknownVariant(value) => {
                writeln!(f, "unknown_variant_{}", *value)
            }
        }
    }
}
impl convert::From<u8> for CameraOrientationType {
    fn from(value: u8) -> Self {
        match value {
            0 => CameraOrientationType::CameraOrientation0,
            1 => CameraOrientationType::CameraOrientation90,
            2 => CameraOrientationType::CameraOrientation180,
            3 => CameraOrientationType::CameraOrientation270,
            _ => CameraOrientationType::UnknownVariant(value),
        }
    }
}
impl convert::From<i64> for CameraOrientationType {
    fn from(value: i64) -> Self {
        CameraOrientationType::from(value as u8)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum AttitudeStage {
    Failed,
    Aligning,
    Degraded,
    Valid,
    UnknownVariant(u8),
}
impl AttitudeStage {
    pub fn as_u8(self) -> u8 {
        match self {
            AttitudeStage::Failed => 0,
            AttitudeStage::Aligning => 1,
            AttitudeStage::Degraded => 2,
            AttitudeStage::Valid => 3,
            AttitudeStage::UnknownVariant(value) => value,
        }
    }
    pub fn as_i64(self) -> i64 {
        self.as_u8() as i64
    }
}
impl fmt::Display for AttitudeStage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            AttitudeStage::Failed => writeln!(f, "failed"),
            AttitudeStage::Aligning => writeln!(f, "aligning"),
            AttitudeStage::Degraded => writeln!(f, "degraded"),
            AttitudeStage::Valid => writeln!(f, "valid"),
            AttitudeStage::UnknownVariant(value) => writeln!(f, "unknown_variant_{}", *value),
        }
    }
}
impl convert::From<u8> for AttitudeStage {
    fn from(value: u8) -> Self {
        match value {
            0 => AttitudeStage::Failed,
            1 => AttitudeStage::Aligning,
            2 => AttitudeStage::Degraded,
            3 => AttitudeStage::Valid,
            _ => AttitudeStage::UnknownVariant(value),
        }
    }
}
impl convert::From<i64> for AttitudeStage {
    fn from(value: i64) -> Self {
        AttitudeStage::from(value as u8)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum AttitudeValidity {
    TrackAngleHeadingValid,
    PitchValid,
    RollValid,
    LateralBodyAccelValid,
    NormalBodyAccelValid,
    TurnRateValid,
    HwFail,
    MagInvalid,
    NoGps,
    GpsInvalid,
    SolutionCoasting,
    TrueTrackAngle,
    MagneticHeading,
    UnknownVariant(u16),
}
impl AttitudeValidity {
    pub fn as_u16(self) -> u16 {
        match self {
            AttitudeValidity::TrackAngleHeadingValid => 1,
            AttitudeValidity::PitchValid => 2,
            AttitudeValidity::RollValid => 4,
            AttitudeValidity::LateralBodyAccelValid => 8,
            AttitudeValidity::NormalBodyAccelValid => 16,
            AttitudeValidity::TurnRateValid => 32,
            AttitudeValidity::HwFail => 64,
            AttitudeValidity::MagInvalid => 128,
            AttitudeValidity::NoGps => 256,
            AttitudeValidity::GpsInvalid => 512,
            AttitudeValidity::SolutionCoasting => 1024,
            AttitudeValidity::TrueTrackAngle => 2048,
            AttitudeValidity::MagneticHeading => 4096,
            AttitudeValidity::UnknownVariant(value) => value,
        }
    }
    pub fn as_i64(self) -> i64 {
        self.as_u16() as i64
    }
}
impl fmt::Display for AttitudeValidity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            AttitudeValidity::TrackAngleHeadingValid => writeln!(f, "track_angle_heading_valid"),
            AttitudeValidity::PitchValid => writeln!(f, "pitch_valid"),
            AttitudeValidity::RollValid => writeln!(f, "roll_valid"),
            AttitudeValidity::LateralBodyAccelValid => writeln!(f, "lateral_body_accel_valid"),
            AttitudeValidity::NormalBodyAccelValid => writeln!(f, "normal_body_accel_valid"),
            AttitudeValidity::TurnRateValid => writeln!(f, "turn_rate_valid"),
            AttitudeValidity::HwFail => writeln!(f, "hw_fail"),
            AttitudeValidity::MagInvalid => writeln!(f, "mag_invalid"),
            AttitudeValidity::NoGps => writeln!(f, "no_gps"),
            AttitudeValidity::GpsInvalid => writeln!(f, "gps_invalid"),
            AttitudeValidity::SolutionCoasting => writeln!(f, "solution_coasting"),
            AttitudeValidity::TrueTrackAngle => writeln!(f, "true_track_angle"),
            AttitudeValidity::MagneticHeading => writeln!(f, "magnetic_heading"),
            AttitudeValidity::UnknownVariant(value) => writeln!(f, "unknown_variant_{}", *value),
        }
    }
}
impl convert::From<u16> for AttitudeValidity {
    fn from(value: u16) -> Self {
        match value {
            1 => AttitudeValidity::TrackAngleHeadingValid,
            2 => AttitudeValidity::PitchValid,
            4 => AttitudeValidity::RollValid,
            8 => AttitudeValidity::LateralBodyAccelValid,
            16 => AttitudeValidity::NormalBodyAccelValid,
            32 => AttitudeValidity::TurnRateValid,
            64 => AttitudeValidity::HwFail,
            128 => AttitudeValidity::MagInvalid,
            256 => AttitudeValidity::NoGps,
            512 => AttitudeValidity::GpsInvalid,
            1024 => AttitudeValidity::SolutionCoasting,
            2048 => AttitudeValidity::TrueTrackAngle,
            4096 => AttitudeValidity::MagneticHeading,
            _ => AttitudeValidity::UnknownVariant(value),
        }
    }
}
impl convert::From<i64> for AttitudeValidity {
    fn from(value: i64) -> Self {
        AttitudeValidity::from(value as u16)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum AutoSyncFrequency {
    Never,
    Occasionally,
    Frequent,
    OnceADay,
    Remote,
    UnknownVariant(u8),
}
impl AutoSyncFrequency {
    pub fn as_u8(self) -> u8 {
        match self {
            AutoSyncFrequency::Never => 0,
            AutoSyncFrequency::Occasionally => 1,
            AutoSyncFrequency::Frequent => 2,
            AutoSyncFrequency::OnceADay => 3,
            AutoSyncFrequency::Remote => 4,
            AutoSyncFrequency::UnknownVariant(value) => value,
        }
    }
    pub fn as_i64(self) -> i64 {
        self.as_u8() as i64
    }
}
impl fmt::Display for AutoSyncFrequency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            AutoSyncFrequency::Never => writeln!(f, "never"),
            AutoSyncFrequency::Occasionally => writeln!(f, "occasionally"),
            AutoSyncFrequency::Frequent => writeln!(f, "frequent"),
            AutoSyncFrequency::OnceADay => writeln!(f, "once_a_day"),
            AutoSyncFrequency::Remote => writeln!(f, "remote"),
            AutoSyncFrequency::UnknownVariant(value) => writeln!(f, "unknown_variant_{}", *value),
        }
    }
}
impl convert::From<u8> for AutoSyncFrequency {
    fn from(value: u8) -> Self {
        match value {
            0 => AutoSyncFrequency::Never,
            1 => AutoSyncFrequency::Occasionally,
            2 => AutoSyncFrequency::Frequent,
            3 => AutoSyncFrequency::OnceADay,
            4 => AutoSyncFrequency::Remote,
            _ => AutoSyncFrequency::UnknownVariant(value),
        }
    }
}
impl convert::From<i64> for AutoSyncFrequency {
    fn from(value: i64) -> Self {
        AutoSyncFrequency::from(value as u8)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum ExdLayout {
    FullScreen,
    HalfVertical,
    HalfHorizontal,
    HalfVerticalRightSplit,
    HalfHorizontalBottomSplit,
    FullQuarterSplit,
    HalfVerticalLeftSplit,
    HalfHorizontalTopSplit,
    UnknownVariant(u8),
}
impl ExdLayout {
    pub fn as_u8(self) -> u8 {
        match self {
            ExdLayout::FullScreen => 0,
            ExdLayout::HalfVertical => 1,
            ExdLayout::HalfHorizontal => 2,
            ExdLayout::HalfVerticalRightSplit => 3,
            ExdLayout::HalfHorizontalBottomSplit => 4,
            ExdLayout::FullQuarterSplit => 5,
            ExdLayout::HalfVerticalLeftSplit => 6,
            ExdLayout::HalfHorizontalTopSplit => 7,
            ExdLayout::UnknownVariant(value) => value,
        }
    }
    pub fn as_i64(self) -> i64 {
        self.as_u8() as i64
    }
}
impl fmt::Display for ExdLayout {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            ExdLayout::FullScreen => writeln!(f, "full_screen"),
            ExdLayout::HalfVertical => writeln!(f, "half_vertical"),
            ExdLayout::HalfHorizontal => writeln!(f, "half_horizontal"),
            ExdLayout::HalfVerticalRightSplit => writeln!(f, "half_vertical_right_split"),
            ExdLayout::HalfHorizontalBottomSplit => writeln!(f, "half_horizontal_bottom_split"),
            ExdLayout::FullQuarterSplit => writeln!(f, "full_quarter_split"),
            ExdLayout::HalfVerticalLeftSplit => writeln!(f, "half_vertical_left_split"),
            ExdLayout::HalfHorizontalTopSplit => writeln!(f, "half_horizontal_top_split"),
            ExdLayout::UnknownVariant(value) => writeln!(f, "unknown_variant_{}", *value),
        }
    }
}
impl convert::From<u8> for ExdLayout {
    fn from(value: u8) -> Self {
        match value {
            0 => ExdLayout::FullScreen,
            1 => ExdLayout::HalfVertical,
            2 => ExdLayout::HalfHorizontal,
            3 => ExdLayout::HalfVerticalRightSplit,
            4 => ExdLayout::HalfHorizontalBottomSplit,
            5 => ExdLayout::FullQuarterSplit,
            6 => ExdLayout::HalfVerticalLeftSplit,
            7 => ExdLayout::HalfHorizontalTopSplit,
            _ => ExdLayout::UnknownVariant(value),
        }
    }
}
impl convert::From<i64> for ExdLayout {
    fn from(value: i64) -> Self {
        ExdLayout::from(value as u8)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum ExdDisplayType {
    Numerical,
    Simple,
    Graph,
    Bar,
    CircleGraph,
    VirtualPartner,
    Balance,
    StringList,
    String,
    SimpleDynamicIcon,
    Gauge,
    UnknownVariant(u8),
}
impl ExdDisplayType {
    pub fn as_u8(self) -> u8 {
        match self {
            ExdDisplayType::Numerical => 0,
            ExdDisplayType::Simple => 1,
            ExdDisplayType::Graph => 2,
            ExdDisplayType::Bar => 3,
            ExdDisplayType::CircleGraph => 4,
            ExdDisplayType::VirtualPartner => 5,
            ExdDisplayType::Balance => 6,
            ExdDisplayType::StringList => 7,
            ExdDisplayType::String => 8,
            ExdDisplayType::SimpleDynamicIcon => 9,
            ExdDisplayType::Gauge => 10,
            ExdDisplayType::UnknownVariant(value) => value,
        }
    }
    pub fn as_i64(self) -> i64 {
        self.as_u8() as i64
    }
}
impl fmt::Display for ExdDisplayType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            ExdDisplayType::Numerical => writeln!(f, "numerical"),
            ExdDisplayType::Simple => writeln!(f, "simple"),
            ExdDisplayType::Graph => writeln!(f, "graph"),
            ExdDisplayType::Bar => writeln!(f, "bar"),
            ExdDisplayType::CircleGraph => writeln!(f, "circle_graph"),
            ExdDisplayType::VirtualPartner => writeln!(f, "virtual_partner"),
            ExdDisplayType::Balance => writeln!(f, "balance"),
            ExdDisplayType::StringList => writeln!(f, "string_list"),
            ExdDisplayType::String => writeln!(f, "string"),
            ExdDisplayType::SimpleDynamicIcon => writeln!(f, "simple_dynamic_icon"),
            ExdDisplayType::Gauge => writeln!(f, "gauge"),
            ExdDisplayType::UnknownVariant(value) => writeln!(f, "unknown_variant_{}", *value),
        }
    }
}
impl convert::From<u8> for ExdDisplayType {
    fn from(value: u8) -> Self {
        match value {
            0 => ExdDisplayType::Numerical,
            1 => ExdDisplayType::Simple,
            2 => ExdDisplayType::Graph,
            3 => ExdDisplayType::Bar,
            4 => ExdDisplayType::CircleGraph,
            5 => ExdDisplayType::VirtualPartner,
            6 => ExdDisplayType::Balance,
            7 => ExdDisplayType::StringList,
            8 => ExdDisplayType::String,
            9 => ExdDisplayType::SimpleDynamicIcon,
            10 => ExdDisplayType::Gauge,
            _ => ExdDisplayType::UnknownVariant(value),
        }
    }
}
impl convert::From<i64> for ExdDisplayType {
    fn from(value: i64) -> Self {
        ExdDisplayType::from(value as u8)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum ExdDataUnits {
    NoUnits,
    Laps,
    MilesPerHour,
    KilometersPerHour,
    FeetPerHour,
    MetersPerHour,
    DegreesCelsius,
    DegreesFarenheit,
    Zone,
    Gear,
    Rpm,
    Bpm,
    Degrees,
    Millimeters,
    Meters,
    Kilometers,
    Feet,
    Yards,
    Kilofeet,
    Miles,
    Time,
    EnumTurnType,
    Percent,
    Watts,
    WattsPerKilogram,
    EnumBatteryStatus,
    EnumBikeLightBeamAngleMode,
    EnumBikeLightBatteryStatus,
    EnumBikeLightNetworkConfigType,
    Lights,
    Seconds,
    Minutes,
    Hours,
    Calories,
    Kilojoules,
    Milliseconds,
    SecondPerMile,
    SecondPerKilometer,
    Centimeter,
    EnumCoursePoint,
    Bradians,
    EnumSport,
    InchesHg,
    MmHg,
    Mbars,
    HectoPascals,
    FeetPerMin,
    MetersPerMin,
    MetersPerSec,
    EightCardinal,
    UnknownVariant(u8),
}
impl ExdDataUnits {
    pub fn as_u8(self) -> u8 {
        match self {
            ExdDataUnits::NoUnits => 0,
            ExdDataUnits::Laps => 1,
            ExdDataUnits::MilesPerHour => 2,
            ExdDataUnits::KilometersPerHour => 3,
            ExdDataUnits::FeetPerHour => 4,
            ExdDataUnits::MetersPerHour => 5,
            ExdDataUnits::DegreesCelsius => 6,
            ExdDataUnits::DegreesFarenheit => 7,
            ExdDataUnits::Zone => 8,
            ExdDataUnits::Gear => 9,
            ExdDataUnits::Rpm => 10,
            ExdDataUnits::Bpm => 11,
            ExdDataUnits::Degrees => 12,
            ExdDataUnits::Millimeters => 13,
            ExdDataUnits::Meters => 14,
            ExdDataUnits::Kilometers => 15,
            ExdDataUnits::Feet => 16,
            ExdDataUnits::Yards => 17,
            ExdDataUnits::Kilofeet => 18,
            ExdDataUnits::Miles => 19,
            ExdDataUnits::Time => 20,
            ExdDataUnits::EnumTurnType => 21,
            ExdDataUnits::Percent => 22,
            ExdDataUnits::Watts => 23,
            ExdDataUnits::WattsPerKilogram => 24,
            ExdDataUnits::EnumBatteryStatus => 25,
            ExdDataUnits::EnumBikeLightBeamAngleMode => 26,
            ExdDataUnits::EnumBikeLightBatteryStatus => 27,
            ExdDataUnits::EnumBikeLightNetworkConfigType => 28,
            ExdDataUnits::Lights => 29,
            ExdDataUnits::Seconds => 30,
            ExdDataUnits::Minutes => 31,
            ExdDataUnits::Hours => 32,
            ExdDataUnits::Calories => 33,
            ExdDataUnits::Kilojoules => 34,
            ExdDataUnits::Milliseconds => 35,
            ExdDataUnits::SecondPerMile => 36,
            ExdDataUnits::SecondPerKilometer => 37,
            ExdDataUnits::Centimeter => 38,
            ExdDataUnits::EnumCoursePoint => 39,
            ExdDataUnits::Bradians => 40,
            ExdDataUnits::EnumSport => 41,
            ExdDataUnits::InchesHg => 42,
            ExdDataUnits::MmHg => 43,
            ExdDataUnits::Mbars => 44,
            ExdDataUnits::HectoPascals => 45,
            ExdDataUnits::FeetPerMin => 46,
            ExdDataUnits::MetersPerMin => 47,
            ExdDataUnits::MetersPerSec => 48,
            ExdDataUnits::EightCardinal => 49,
            ExdDataUnits::UnknownVariant(value) => value,
        }
    }
    pub fn as_i64(self) -> i64 {
        self.as_u8() as i64
    }
}
impl fmt::Display for ExdDataUnits {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            ExdDataUnits::NoUnits => writeln!(f, "no_units"),
            ExdDataUnits::Laps => writeln!(f, "laps"),
            ExdDataUnits::MilesPerHour => writeln!(f, "miles_per_hour"),
            ExdDataUnits::KilometersPerHour => writeln!(f, "kilometers_per_hour"),
            ExdDataUnits::FeetPerHour => writeln!(f, "feet_per_hour"),
            ExdDataUnits::MetersPerHour => writeln!(f, "meters_per_hour"),
            ExdDataUnits::DegreesCelsius => writeln!(f, "degrees_celsius"),
            ExdDataUnits::DegreesFarenheit => writeln!(f, "degrees_farenheit"),
            ExdDataUnits::Zone => writeln!(f, "zone"),
            ExdDataUnits::Gear => writeln!(f, "gear"),
            ExdDataUnits::Rpm => writeln!(f, "rpm"),
            ExdDataUnits::Bpm => writeln!(f, "bpm"),
            ExdDataUnits::Degrees => writeln!(f, "degrees"),
            ExdDataUnits::Millimeters => writeln!(f, "millimeters"),
            ExdDataUnits::Meters => writeln!(f, "meters"),
            ExdDataUnits::Kilometers => writeln!(f, "kilometers"),
            ExdDataUnits::Feet => writeln!(f, "feet"),
            ExdDataUnits::Yards => writeln!(f, "yards"),
            ExdDataUnits::Kilofeet => writeln!(f, "kilofeet"),
            ExdDataUnits::Miles => writeln!(f, "miles"),
            ExdDataUnits::Time => writeln!(f, "time"),
            ExdDataUnits::EnumTurnType => writeln!(f, "enum_turn_type"),
            ExdDataUnits::Percent => writeln!(f, "percent"),
            ExdDataUnits::Watts => writeln!(f, "watts"),
            ExdDataUnits::WattsPerKilogram => writeln!(f, "watts_per_kilogram"),
            ExdDataUnits::EnumBatteryStatus => writeln!(f, "enum_battery_status"),
            ExdDataUnits::EnumBikeLightBeamAngleMode => {
                writeln!(f, "enum_bike_light_beam_angle_mode")
            }
            ExdDataUnits::EnumBikeLightBatteryStatus => {
                writeln!(f, "enum_bike_light_battery_status")
            }
            ExdDataUnits::EnumBikeLightNetworkConfigType => {
                writeln!(f, "enum_bike_light_network_config_type")
            }
            ExdDataUnits::Lights => writeln!(f, "lights"),
            ExdDataUnits::Seconds => writeln!(f, "seconds"),
            ExdDataUnits::Minutes => writeln!(f, "minutes"),
            ExdDataUnits::Hours => writeln!(f, "hours"),
            ExdDataUnits::Calories => writeln!(f, "calories"),
            ExdDataUnits::Kilojoules => writeln!(f, "kilojoules"),
            ExdDataUnits::Milliseconds => writeln!(f, "milliseconds"),
            ExdDataUnits::SecondPerMile => writeln!(f, "second_per_mile"),
            ExdDataUnits::SecondPerKilometer => writeln!(f, "second_per_kilometer"),
            ExdDataUnits::Centimeter => writeln!(f, "centimeter"),
            ExdDataUnits::EnumCoursePoint => writeln!(f, "enum_course_point"),
            ExdDataUnits::Bradians => writeln!(f, "bradians"),
            ExdDataUnits::EnumSport => writeln!(f, "enum_sport"),
            ExdDataUnits::InchesHg => writeln!(f, "inches_hg"),
            ExdDataUnits::MmHg => writeln!(f, "mm_hg"),
            ExdDataUnits::Mbars => writeln!(f, "mbars"),
            ExdDataUnits::HectoPascals => writeln!(f, "hecto_pascals"),
            ExdDataUnits::FeetPerMin => writeln!(f, "feet_per_min"),
            ExdDataUnits::MetersPerMin => writeln!(f, "meters_per_min"),
            ExdDataUnits::MetersPerSec => writeln!(f, "meters_per_sec"),
            ExdDataUnits::EightCardinal => writeln!(f, "eight_cardinal"),
            ExdDataUnits::UnknownVariant(value) => writeln!(f, "unknown_variant_{}", *value),
        }
    }
}
impl convert::From<u8> for ExdDataUnits {
    fn from(value: u8) -> Self {
        match value {
            0 => ExdDataUnits::NoUnits,
            1 => ExdDataUnits::Laps,
            2 => ExdDataUnits::MilesPerHour,
            3 => ExdDataUnits::KilometersPerHour,
            4 => ExdDataUnits::FeetPerHour,
            5 => ExdDataUnits::MetersPerHour,
            6 => ExdDataUnits::DegreesCelsius,
            7 => ExdDataUnits::DegreesFarenheit,
            8 => ExdDataUnits::Zone,
            9 => ExdDataUnits::Gear,
            10 => ExdDataUnits::Rpm,
            11 => ExdDataUnits::Bpm,
            12 => ExdDataUnits::Degrees,
            13 => ExdDataUnits::Millimeters,
            14 => ExdDataUnits::Meters,
            15 => ExdDataUnits::Kilometers,
            16 => ExdDataUnits::Feet,
            17 => ExdDataUnits::Yards,
            18 => ExdDataUnits::Kilofeet,
            19 => ExdDataUnits::Miles,
            20 => ExdDataUnits::Time,
            21 => ExdDataUnits::EnumTurnType,
            22 => ExdDataUnits::Percent,
            23 => ExdDataUnits::Watts,
            24 => ExdDataUnits::WattsPerKilogram,
            25 => ExdDataUnits::EnumBatteryStatus,
            26 => ExdDataUnits::EnumBikeLightBeamAngleMode,
            27 => ExdDataUnits::EnumBikeLightBatteryStatus,
            28 => ExdDataUnits::EnumBikeLightNetworkConfigType,
            29 => ExdDataUnits::Lights,
            30 => ExdDataUnits::Seconds,
            31 => ExdDataUnits::Minutes,
            32 => ExdDataUnits::Hours,
            33 => ExdDataUnits::Calories,
            34 => ExdDataUnits::Kilojoules,
            35 => ExdDataUnits::Milliseconds,
            36 => ExdDataUnits::SecondPerMile,
            37 => ExdDataUnits::SecondPerKilometer,
            38 => ExdDataUnits::Centimeter,
            39 => ExdDataUnits::EnumCoursePoint,
            40 => ExdDataUnits::Bradians,
            41 => ExdDataUnits::EnumSport,
            42 => ExdDataUnits::InchesHg,
            43 => ExdDataUnits::MmHg,
            44 => ExdDataUnits::Mbars,
            45 => ExdDataUnits::HectoPascals,
            46 => ExdDataUnits::FeetPerMin,
            47 => ExdDataUnits::MetersPerMin,
            48 => ExdDataUnits::MetersPerSec,
            49 => ExdDataUnits::EightCardinal,
            _ => ExdDataUnits::UnknownVariant(value),
        }
    }
}
impl convert::From<i64> for ExdDataUnits {
    fn from(value: i64) -> Self {
        ExdDataUnits::from(value as u8)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum ExdQualifiers {
    NoQualifier,
    Instantaneous,
    Average,
    Lap,
    Maximum,
    MaximumAverage,
    MaximumLap,
    LastLap,
    AverageLap,
    ToDestination,
    ToGo,
    ToNext,
    NextCoursePoint,
    Total,
    ThreeSecondAverage,
    TenSecondAverage,
    ThirtySecondAverage,
    PercentMaximum,
    PercentMaximumAverage,
    LapPercentMaximum,
    Elapsed,
    Sunrise,
    Sunset,
    ComparedToVirtualPartner,
    Maximum24h,
    Minimum24h,
    Minimum,
    First,
    Second,
    Third,
    Shifter,
    LastSport,
    Moving,
    Stopped,
    EstimatedTotal,
    Zone9,
    Zone8,
    Zone7,
    Zone6,
    Zone5,
    Zone4,
    Zone3,
    Zone2,
    Zone1,
    UnknownVariant(u8),
}
impl ExdQualifiers {
    pub fn as_u8(self) -> u8 {
        match self {
            ExdQualifiers::NoQualifier => 0,
            ExdQualifiers::Instantaneous => 1,
            ExdQualifiers::Average => 2,
            ExdQualifiers::Lap => 3,
            ExdQualifiers::Maximum => 4,
            ExdQualifiers::MaximumAverage => 5,
            ExdQualifiers::MaximumLap => 6,
            ExdQualifiers::LastLap => 7,
            ExdQualifiers::AverageLap => 8,
            ExdQualifiers::ToDestination => 9,
            ExdQualifiers::ToGo => 10,
            ExdQualifiers::ToNext => 11,
            ExdQualifiers::NextCoursePoint => 12,
            ExdQualifiers::Total => 13,
            ExdQualifiers::ThreeSecondAverage => 14,
            ExdQualifiers::TenSecondAverage => 15,
            ExdQualifiers::ThirtySecondAverage => 16,
            ExdQualifiers::PercentMaximum => 17,
            ExdQualifiers::PercentMaximumAverage => 18,
            ExdQualifiers::LapPercentMaximum => 19,
            ExdQualifiers::Elapsed => 20,
            ExdQualifiers::Sunrise => 21,
            ExdQualifiers::Sunset => 22,
            ExdQualifiers::ComparedToVirtualPartner => 23,
            ExdQualifiers::Maximum24h => 24,
            ExdQualifiers::Minimum24h => 25,
            ExdQualifiers::Minimum => 26,
            ExdQualifiers::First => 27,
            ExdQualifiers::Second => 28,
            ExdQualifiers::Third => 29,
            ExdQualifiers::Shifter => 30,
            ExdQualifiers::LastSport => 31,
            ExdQualifiers::Moving => 32,
            ExdQualifiers::Stopped => 33,
            ExdQualifiers::EstimatedTotal => 34,
            ExdQualifiers::Zone9 => 242,
            ExdQualifiers::Zone8 => 243,
            ExdQualifiers::Zone7 => 244,
            ExdQualifiers::Zone6 => 245,
            ExdQualifiers::Zone5 => 246,
            ExdQualifiers::Zone4 => 247,
            ExdQualifiers::Zone3 => 248,
            ExdQualifiers::Zone2 => 249,
            ExdQualifiers::Zone1 => 250,
            ExdQualifiers::UnknownVariant(value) => value,
        }
    }
    pub fn as_i64(self) -> i64 {
        self.as_u8() as i64
    }
}
impl fmt::Display for ExdQualifiers {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            ExdQualifiers::NoQualifier => writeln!(f, "no_qualifier"),
            ExdQualifiers::Instantaneous => writeln!(f, "instantaneous"),
            ExdQualifiers::Average => writeln!(f, "average"),
            ExdQualifiers::Lap => writeln!(f, "lap"),
            ExdQualifiers::Maximum => writeln!(f, "maximum"),
            ExdQualifiers::MaximumAverage => writeln!(f, "maximum_average"),
            ExdQualifiers::MaximumLap => writeln!(f, "maximum_lap"),
            ExdQualifiers::LastLap => writeln!(f, "last_lap"),
            ExdQualifiers::AverageLap => writeln!(f, "average_lap"),
            ExdQualifiers::ToDestination => writeln!(f, "to_destination"),
            ExdQualifiers::ToGo => writeln!(f, "to_go"),
            ExdQualifiers::ToNext => writeln!(f, "to_next"),
            ExdQualifiers::NextCoursePoint => writeln!(f, "next_course_point"),
            ExdQualifiers::Total => writeln!(f, "total"),
            ExdQualifiers::ThreeSecondAverage => writeln!(f, "three_second_average"),
            ExdQualifiers::TenSecondAverage => writeln!(f, "ten_second_average"),
            ExdQualifiers::ThirtySecondAverage => writeln!(f, "thirty_second_average"),
            ExdQualifiers::PercentMaximum => writeln!(f, "percent_maximum"),
            ExdQualifiers::PercentMaximumAverage => writeln!(f, "percent_maximum_average"),
            ExdQualifiers::LapPercentMaximum => writeln!(f, "lap_percent_maximum"),
            ExdQualifiers::Elapsed => writeln!(f, "elapsed"),
            ExdQualifiers::Sunrise => writeln!(f, "sunrise"),
            ExdQualifiers::Sunset => writeln!(f, "sunset"),
            ExdQualifiers::ComparedToVirtualPartner => writeln!(f, "compared_to_virtual_partner"),
            ExdQualifiers::Maximum24h => writeln!(f, "maximum_24h"),
            ExdQualifiers::Minimum24h => writeln!(f, "minimum_24h"),
            ExdQualifiers::Minimum => writeln!(f, "minimum"),
            ExdQualifiers::First => writeln!(f, "first"),
            ExdQualifiers::Second => writeln!(f, "second"),
            ExdQualifiers::Third => writeln!(f, "third"),
            ExdQualifiers::Shifter => writeln!(f, "shifter"),
            ExdQualifiers::LastSport => writeln!(f, "last_sport"),
            ExdQualifiers::Moving => writeln!(f, "moving"),
            ExdQualifiers::Stopped => writeln!(f, "stopped"),
            ExdQualifiers::EstimatedTotal => writeln!(f, "estimated_total"),
            ExdQualifiers::Zone9 => writeln!(f, "zone_9"),
            ExdQualifiers::Zone8 => writeln!(f, "zone_8"),
            ExdQualifiers::Zone7 => writeln!(f, "zone_7"),
            ExdQualifiers::Zone6 => writeln!(f, "zone_6"),
            ExdQualifiers::Zone5 => writeln!(f, "zone_5"),
            ExdQualifiers::Zone4 => writeln!(f, "zone_4"),
            ExdQualifiers::Zone3 => writeln!(f, "zone_3"),
            ExdQualifiers::Zone2 => writeln!(f, "zone_2"),
            ExdQualifiers::Zone1 => writeln!(f, "zone_1"),
            ExdQualifiers::UnknownVariant(value) => writeln!(f, "unknown_variant_{}", *value),
        }
    }
}
impl convert::From<u8> for ExdQualifiers {
    fn from(value: u8) -> Self {
        match value {
            0 => ExdQualifiers::NoQualifier,
            1 => ExdQualifiers::Instantaneous,
            2 => ExdQualifiers::Average,
            3 => ExdQualifiers::Lap,
            4 => ExdQualifiers::Maximum,
            5 => ExdQualifiers::MaximumAverage,
            6 => ExdQualifiers::MaximumLap,
            7 => ExdQualifiers::LastLap,
            8 => ExdQualifiers::AverageLap,
            9 => ExdQualifiers::ToDestination,
            10 => ExdQualifiers::ToGo,
            11 => ExdQualifiers::ToNext,
            12 => ExdQualifiers::NextCoursePoint,
            13 => ExdQualifiers::Total,
            14 => ExdQualifiers::ThreeSecondAverage,
            15 => ExdQualifiers::TenSecondAverage,
            16 => ExdQualifiers::ThirtySecondAverage,
            17 => ExdQualifiers::PercentMaximum,
            18 => ExdQualifiers::PercentMaximumAverage,
            19 => ExdQualifiers::LapPercentMaximum,
            20 => ExdQualifiers::Elapsed,
            21 => ExdQualifiers::Sunrise,
            22 => ExdQualifiers::Sunset,
            23 => ExdQualifiers::ComparedToVirtualPartner,
            24 => ExdQualifiers::Maximum24h,
            25 => ExdQualifiers::Minimum24h,
            26 => ExdQualifiers::Minimum,
            27 => ExdQualifiers::First,
            28 => ExdQualifiers::Second,
            29 => ExdQualifiers::Third,
            30 => ExdQualifiers::Shifter,
            31 => ExdQualifiers::LastSport,
            32 => ExdQualifiers::Moving,
            33 => ExdQualifiers::Stopped,
            34 => ExdQualifiers::EstimatedTotal,
            242 => ExdQualifiers::Zone9,
            243 => ExdQualifiers::Zone8,
            244 => ExdQualifiers::Zone7,
            245 => ExdQualifiers::Zone6,
            246 => ExdQualifiers::Zone5,
            247 => ExdQualifiers::Zone4,
            248 => ExdQualifiers::Zone3,
            249 => ExdQualifiers::Zone2,
            250 => ExdQualifiers::Zone1,
            _ => ExdQualifiers::UnknownVariant(value),
        }
    }
}
impl convert::From<i64> for ExdQualifiers {
    fn from(value: i64) -> Self {
        ExdQualifiers::from(value as u8)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum ExdDescriptors {
    BikeLightBatteryStatus,
    BeamAngleStatus,
    BateryLevel,
    LightNetworkMode,
    NumberLightsConnected,
    Cadence,
    Distance,
    EstimatedTimeOfArrival,
    Heading,
    Time,
    BatteryLevel,
    TrainerResistance,
    TrainerTargetPower,
    TimeSeated,
    TimeStanding,
    Elevation,
    Grade,
    Ascent,
    Descent,
    VerticalSpeed,
    Di2BatteryLevel,
    FrontGear,
    RearGear,
    GearRatio,
    HeartRate,
    HeartRateZone,
    TimeInHeartRateZone,
    HeartRateReserve,
    Calories,
    GpsAccuracy,
    GpsSignalStrength,
    Temperature,
    TimeOfDay,
    Balance,
    PedalSmoothness,
    Power,
    FunctionalThresholdPower,
    IntensityFactor,
    Work,
    PowerRatio,
    NormalizedPower,
    TrainingStressScore,
    TimeOnZone,
    Speed,
    Laps,
    Reps,
    WorkoutStep,
    CourseDistance,
    NavigationDistance,
    CourseEstimatedTimeOfArrival,
    NavigationEstimatedTimeOfArrival,
    CourseTime,
    NavigationTime,
    CourseHeading,
    NavigationHeading,
    PowerZone,
    TorqueEffectiveness,
    TimerTime,
    PowerWeightRatio,
    LeftPlatformCenterOffset,
    RightPlatformCenterOffset,
    LeftPowerPhaseStartAngle,
    RightPowerPhaseStartAngle,
    LeftPowerPhaseFinishAngle,
    RightPowerPhaseFinishAngle,
    /// Combined gear information
    Gears,
    Pace,
    TrainingEffect,
    VerticalOscillation,
    VerticalRatio,
    GroundContactTime,
    LeftGroundContactTimeBalance,
    RightGroundContactTimeBalance,
    StrideLength,
    RunningCadence,
    PerformanceCondition,
    CourseType,
    TimeInPowerZone,
    NavigationTurn,
    CourseLocation,
    NavigationLocation,
    Compass,
    GearCombo,
    MuscleOxygen,
    Icon,
    CompassHeading,
    GpsHeading,
    GpsElevation,
    AnaerobicTrainingEffect,
    Course,
    OffCourse,
    GlideRatio,
    VerticalDistance,
    Vmg,
    AmbientPressure,
    Pressure,
    Vam,
    UnknownVariant(u8),
}
impl ExdDescriptors {
    pub fn as_u8(self) -> u8 {
        match self {
            ExdDescriptors::BikeLightBatteryStatus => 0,
            ExdDescriptors::BeamAngleStatus => 1,
            ExdDescriptors::BateryLevel => 2,
            ExdDescriptors::LightNetworkMode => 3,
            ExdDescriptors::NumberLightsConnected => 4,
            ExdDescriptors::Cadence => 5,
            ExdDescriptors::Distance => 6,
            ExdDescriptors::EstimatedTimeOfArrival => 7,
            ExdDescriptors::Heading => 8,
            ExdDescriptors::Time => 9,
            ExdDescriptors::BatteryLevel => 10,
            ExdDescriptors::TrainerResistance => 11,
            ExdDescriptors::TrainerTargetPower => 12,
            ExdDescriptors::TimeSeated => 13,
            ExdDescriptors::TimeStanding => 14,
            ExdDescriptors::Elevation => 15,
            ExdDescriptors::Grade => 16,
            ExdDescriptors::Ascent => 17,
            ExdDescriptors::Descent => 18,
            ExdDescriptors::VerticalSpeed => 19,
            ExdDescriptors::Di2BatteryLevel => 20,
            ExdDescriptors::FrontGear => 21,
            ExdDescriptors::RearGear => 22,
            ExdDescriptors::GearRatio => 23,
            ExdDescriptors::HeartRate => 24,
            ExdDescriptors::HeartRateZone => 25,
            ExdDescriptors::TimeInHeartRateZone => 26,
            ExdDescriptors::HeartRateReserve => 27,
            ExdDescriptors::Calories => 28,
            ExdDescriptors::GpsAccuracy => 29,
            ExdDescriptors::GpsSignalStrength => 30,
            ExdDescriptors::Temperature => 31,
            ExdDescriptors::TimeOfDay => 32,
            ExdDescriptors::Balance => 33,
            ExdDescriptors::PedalSmoothness => 34,
            ExdDescriptors::Power => 35,
            ExdDescriptors::FunctionalThresholdPower => 36,
            ExdDescriptors::IntensityFactor => 37,
            ExdDescriptors::Work => 38,
            ExdDescriptors::PowerRatio => 39,
            ExdDescriptors::NormalizedPower => 40,
            ExdDescriptors::TrainingStressScore => 41,
            ExdDescriptors::TimeOnZone => 42,
            ExdDescriptors::Speed => 43,
            ExdDescriptors::Laps => 44,
            ExdDescriptors::Reps => 45,
            ExdDescriptors::WorkoutStep => 46,
            ExdDescriptors::CourseDistance => 47,
            ExdDescriptors::NavigationDistance => 48,
            ExdDescriptors::CourseEstimatedTimeOfArrival => 49,
            ExdDescriptors::NavigationEstimatedTimeOfArrival => 50,
            ExdDescriptors::CourseTime => 51,
            ExdDescriptors::NavigationTime => 52,
            ExdDescriptors::CourseHeading => 53,
            ExdDescriptors::NavigationHeading => 54,
            ExdDescriptors::PowerZone => 55,
            ExdDescriptors::TorqueEffectiveness => 56,
            ExdDescriptors::TimerTime => 57,
            ExdDescriptors::PowerWeightRatio => 58,
            ExdDescriptors::LeftPlatformCenterOffset => 59,
            ExdDescriptors::RightPlatformCenterOffset => 60,
            ExdDescriptors::LeftPowerPhaseStartAngle => 61,
            ExdDescriptors::RightPowerPhaseStartAngle => 62,
            ExdDescriptors::LeftPowerPhaseFinishAngle => 63,
            ExdDescriptors::RightPowerPhaseFinishAngle => 64,
            ExdDescriptors::Gears => 65,
            ExdDescriptors::Pace => 66,
            ExdDescriptors::TrainingEffect => 67,
            ExdDescriptors::VerticalOscillation => 68,
            ExdDescriptors::VerticalRatio => 69,
            ExdDescriptors::GroundContactTime => 70,
            ExdDescriptors::LeftGroundContactTimeBalance => 71,
            ExdDescriptors::RightGroundContactTimeBalance => 72,
            ExdDescriptors::StrideLength => 73,
            ExdDescriptors::RunningCadence => 74,
            ExdDescriptors::PerformanceCondition => 75,
            ExdDescriptors::CourseType => 76,
            ExdDescriptors::TimeInPowerZone => 77,
            ExdDescriptors::NavigationTurn => 78,
            ExdDescriptors::CourseLocation => 79,
            ExdDescriptors::NavigationLocation => 80,
            ExdDescriptors::Compass => 81,
            ExdDescriptors::GearCombo => 82,
            ExdDescriptors::MuscleOxygen => 83,
            ExdDescriptors::Icon => 84,
            ExdDescriptors::CompassHeading => 85,
            ExdDescriptors::GpsHeading => 86,
            ExdDescriptors::GpsElevation => 87,
            ExdDescriptors::AnaerobicTrainingEffect => 88,
            ExdDescriptors::Course => 89,
            ExdDescriptors::OffCourse => 90,
            ExdDescriptors::GlideRatio => 91,
            ExdDescriptors::VerticalDistance => 92,
            ExdDescriptors::Vmg => 93,
            ExdDescriptors::AmbientPressure => 94,
            ExdDescriptors::Pressure => 95,
            ExdDescriptors::Vam => 96,
            ExdDescriptors::UnknownVariant(value) => value,
        }
    }
    pub fn as_i64(self) -> i64 {
        self.as_u8() as i64
    }
}
impl fmt::Display for ExdDescriptors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            ExdDescriptors::BikeLightBatteryStatus => writeln!(f, "bike_light_battery_status"),
            ExdDescriptors::BeamAngleStatus => writeln!(f, "beam_angle_status"),
            ExdDescriptors::BateryLevel => writeln!(f, "batery_level"),
            ExdDescriptors::LightNetworkMode => writeln!(f, "light_network_mode"),
            ExdDescriptors::NumberLightsConnected => writeln!(f, "number_lights_connected"),
            ExdDescriptors::Cadence => writeln!(f, "cadence"),
            ExdDescriptors::Distance => writeln!(f, "distance"),
            ExdDescriptors::EstimatedTimeOfArrival => writeln!(f, "estimated_time_of_arrival"),
            ExdDescriptors::Heading => writeln!(f, "heading"),
            ExdDescriptors::Time => writeln!(f, "time"),
            ExdDescriptors::BatteryLevel => writeln!(f, "battery_level"),
            ExdDescriptors::TrainerResistance => writeln!(f, "trainer_resistance"),
            ExdDescriptors::TrainerTargetPower => writeln!(f, "trainer_target_power"),
            ExdDescriptors::TimeSeated => writeln!(f, "time_seated"),
            ExdDescriptors::TimeStanding => writeln!(f, "time_standing"),
            ExdDescriptors::Elevation => writeln!(f, "elevation"),
            ExdDescriptors::Grade => writeln!(f, "grade"),
            ExdDescriptors::Ascent => writeln!(f, "ascent"),
            ExdDescriptors::Descent => writeln!(f, "descent"),
            ExdDescriptors::VerticalSpeed => writeln!(f, "vertical_speed"),
            ExdDescriptors::Di2BatteryLevel => writeln!(f, "di2_battery_level"),
            ExdDescriptors::FrontGear => writeln!(f, "front_gear"),
            ExdDescriptors::RearGear => writeln!(f, "rear_gear"),
            ExdDescriptors::GearRatio => writeln!(f, "gear_ratio"),
            ExdDescriptors::HeartRate => writeln!(f, "heart_rate"),
            ExdDescriptors::HeartRateZone => writeln!(f, "heart_rate_zone"),
            ExdDescriptors::TimeInHeartRateZone => writeln!(f, "time_in_heart_rate_zone"),
            ExdDescriptors::HeartRateReserve => writeln!(f, "heart_rate_reserve"),
            ExdDescriptors::Calories => writeln!(f, "calories"),
            ExdDescriptors::GpsAccuracy => writeln!(f, "gps_accuracy"),
            ExdDescriptors::GpsSignalStrength => writeln!(f, "gps_signal_strength"),
            ExdDescriptors::Temperature => writeln!(f, "temperature"),
            ExdDescriptors::TimeOfDay => writeln!(f, "time_of_day"),
            ExdDescriptors::Balance => writeln!(f, "balance"),
            ExdDescriptors::PedalSmoothness => writeln!(f, "pedal_smoothness"),
            ExdDescriptors::Power => writeln!(f, "power"),
            ExdDescriptors::FunctionalThresholdPower => writeln!(f, "functional_threshold_power"),
            ExdDescriptors::IntensityFactor => writeln!(f, "intensity_factor"),
            ExdDescriptors::Work => writeln!(f, "work"),
            ExdDescriptors::PowerRatio => writeln!(f, "power_ratio"),
            ExdDescriptors::NormalizedPower => writeln!(f, "normalized_power"),
            ExdDescriptors::TrainingStressScore => writeln!(f, "training_stress_Score"),
            ExdDescriptors::TimeOnZone => writeln!(f, "time_on_zone"),
            ExdDescriptors::Speed => writeln!(f, "speed"),
            ExdDescriptors::Laps => writeln!(f, "laps"),
            ExdDescriptors::Reps => writeln!(f, "reps"),
            ExdDescriptors::WorkoutStep => writeln!(f, "workout_step"),
            ExdDescriptors::CourseDistance => writeln!(f, "course_distance"),
            ExdDescriptors::NavigationDistance => writeln!(f, "navigation_distance"),
            ExdDescriptors::CourseEstimatedTimeOfArrival => {
                writeln!(f, "course_estimated_time_of_arrival")
            }
            ExdDescriptors::NavigationEstimatedTimeOfArrival => {
                writeln!(f, "navigation_estimated_time_of_arrival")
            }
            ExdDescriptors::CourseTime => writeln!(f, "course_time"),
            ExdDescriptors::NavigationTime => writeln!(f, "navigation_time"),
            ExdDescriptors::CourseHeading => writeln!(f, "course_heading"),
            ExdDescriptors::NavigationHeading => writeln!(f, "navigation_heading"),
            ExdDescriptors::PowerZone => writeln!(f, "power_zone"),
            ExdDescriptors::TorqueEffectiveness => writeln!(f, "torque_effectiveness"),
            ExdDescriptors::TimerTime => writeln!(f, "timer_time"),
            ExdDescriptors::PowerWeightRatio => writeln!(f, "power_weight_ratio"),
            ExdDescriptors::LeftPlatformCenterOffset => writeln!(f, "left_platform_center_offset"),
            ExdDescriptors::RightPlatformCenterOffset => {
                writeln!(f, "right_platform_center_offset")
            }
            ExdDescriptors::LeftPowerPhaseStartAngle => writeln!(f, "left_power_phase_start_angle"),
            ExdDescriptors::RightPowerPhaseStartAngle => {
                writeln!(f, "right_power_phase_start_angle")
            }
            ExdDescriptors::LeftPowerPhaseFinishAngle => {
                writeln!(f, "left_power_phase_finish_angle")
            }
            ExdDescriptors::RightPowerPhaseFinishAngle => {
                writeln!(f, "right_power_phase_finish_angle")
            }
            ExdDescriptors::Gears => writeln!(f, "gears"),
            ExdDescriptors::Pace => writeln!(f, "pace"),
            ExdDescriptors::TrainingEffect => writeln!(f, "training_effect"),
            ExdDescriptors::VerticalOscillation => writeln!(f, "vertical_oscillation"),
            ExdDescriptors::VerticalRatio => writeln!(f, "vertical_ratio"),
            ExdDescriptors::GroundContactTime => writeln!(f, "ground_contact_time"),
            ExdDescriptors::LeftGroundContactTimeBalance => {
                writeln!(f, "left_ground_contact_time_balance")
            }
            ExdDescriptors::RightGroundContactTimeBalance => {
                writeln!(f, "right_ground_contact_time_balance")
            }
            ExdDescriptors::StrideLength => writeln!(f, "stride_length"),
            ExdDescriptors::RunningCadence => writeln!(f, "running_cadence"),
            ExdDescriptors::PerformanceCondition => writeln!(f, "performance_condition"),
            ExdDescriptors::CourseType => writeln!(f, "course_type"),
            ExdDescriptors::TimeInPowerZone => writeln!(f, "time_in_power_zone"),
            ExdDescriptors::NavigationTurn => writeln!(f, "navigation_turn"),
            ExdDescriptors::CourseLocation => writeln!(f, "course_location"),
            ExdDescriptors::NavigationLocation => writeln!(f, "navigation_location"),
            ExdDescriptors::Compass => writeln!(f, "compass"),
            ExdDescriptors::GearCombo => writeln!(f, "gear_combo"),
            ExdDescriptors::MuscleOxygen => writeln!(f, "muscle_oxygen"),
            ExdDescriptors::Icon => writeln!(f, "icon"),
            ExdDescriptors::CompassHeading => writeln!(f, "compass_heading"),
            ExdDescriptors::GpsHeading => writeln!(f, "gps_heading"),
            ExdDescriptors::GpsElevation => writeln!(f, "gps_elevation"),
            ExdDescriptors::AnaerobicTrainingEffect => writeln!(f, "anaerobic_training_effect"),
            ExdDescriptors::Course => writeln!(f, "course"),
            ExdDescriptors::OffCourse => writeln!(f, "off_course"),
            ExdDescriptors::GlideRatio => writeln!(f, "glide_ratio"),
            ExdDescriptors::VerticalDistance => writeln!(f, "vertical_distance"),
            ExdDescriptors::Vmg => writeln!(f, "vmg"),
            ExdDescriptors::AmbientPressure => writeln!(f, "ambient_pressure"),
            ExdDescriptors::Pressure => writeln!(f, "pressure"),
            ExdDescriptors::Vam => writeln!(f, "vam"),
            ExdDescriptors::UnknownVariant(value) => writeln!(f, "unknown_variant_{}", *value),
        }
    }
}
impl convert::From<u8> for ExdDescriptors {
    fn from(value: u8) -> Self {
        match value {
            0 => ExdDescriptors::BikeLightBatteryStatus,
            1 => ExdDescriptors::BeamAngleStatus,
            2 => ExdDescriptors::BateryLevel,
            3 => ExdDescriptors::LightNetworkMode,
            4 => ExdDescriptors::NumberLightsConnected,
            5 => ExdDescriptors::Cadence,
            6 => ExdDescriptors::Distance,
            7 => ExdDescriptors::EstimatedTimeOfArrival,
            8 => ExdDescriptors::Heading,
            9 => ExdDescriptors::Time,
            10 => ExdDescriptors::BatteryLevel,
            11 => ExdDescriptors::TrainerResistance,
            12 => ExdDescriptors::TrainerTargetPower,
            13 => ExdDescriptors::TimeSeated,
            14 => ExdDescriptors::TimeStanding,
            15 => ExdDescriptors::Elevation,
            16 => ExdDescriptors::Grade,
            17 => ExdDescriptors::Ascent,
            18 => ExdDescriptors::Descent,
            19 => ExdDescriptors::VerticalSpeed,
            20 => ExdDescriptors::Di2BatteryLevel,
            21 => ExdDescriptors::FrontGear,
            22 => ExdDescriptors::RearGear,
            23 => ExdDescriptors::GearRatio,
            24 => ExdDescriptors::HeartRate,
            25 => ExdDescriptors::HeartRateZone,
            26 => ExdDescriptors::TimeInHeartRateZone,
            27 => ExdDescriptors::HeartRateReserve,
            28 => ExdDescriptors::Calories,
            29 => ExdDescriptors::GpsAccuracy,
            30 => ExdDescriptors::GpsSignalStrength,
            31 => ExdDescriptors::Temperature,
            32 => ExdDescriptors::TimeOfDay,
            33 => ExdDescriptors::Balance,
            34 => ExdDescriptors::PedalSmoothness,
            35 => ExdDescriptors::Power,
            36 => ExdDescriptors::FunctionalThresholdPower,
            37 => ExdDescriptors::IntensityFactor,
            38 => ExdDescriptors::Work,
            39 => ExdDescriptors::PowerRatio,
            40 => ExdDescriptors::NormalizedPower,
            41 => ExdDescriptors::TrainingStressScore,
            42 => ExdDescriptors::TimeOnZone,
            43 => ExdDescriptors::Speed,
            44 => ExdDescriptors::Laps,
            45 => ExdDescriptors::Reps,
            46 => ExdDescriptors::WorkoutStep,
            47 => ExdDescriptors::CourseDistance,
            48 => ExdDescriptors::NavigationDistance,
            49 => ExdDescriptors::CourseEstimatedTimeOfArrival,
            50 => ExdDescriptors::NavigationEstimatedTimeOfArrival,
            51 => ExdDescriptors::CourseTime,
            52 => ExdDescriptors::NavigationTime,
            53 => ExdDescriptors::CourseHeading,
            54 => ExdDescriptors::NavigationHeading,
            55 => ExdDescriptors::PowerZone,
            56 => ExdDescriptors::TorqueEffectiveness,
            57 => ExdDescriptors::TimerTime,
            58 => ExdDescriptors::PowerWeightRatio,
            59 => ExdDescriptors::LeftPlatformCenterOffset,
            60 => ExdDescriptors::RightPlatformCenterOffset,
            61 => ExdDescriptors::LeftPowerPhaseStartAngle,
            62 => ExdDescriptors::RightPowerPhaseStartAngle,
            63 => ExdDescriptors::LeftPowerPhaseFinishAngle,
            64 => ExdDescriptors::RightPowerPhaseFinishAngle,
            65 => ExdDescriptors::Gears,
            66 => ExdDescriptors::Pace,
            67 => ExdDescriptors::TrainingEffect,
            68 => ExdDescriptors::VerticalOscillation,
            69 => ExdDescriptors::VerticalRatio,
            70 => ExdDescriptors::GroundContactTime,
            71 => ExdDescriptors::LeftGroundContactTimeBalance,
            72 => ExdDescriptors::RightGroundContactTimeBalance,
            73 => ExdDescriptors::StrideLength,
            74 => ExdDescriptors::RunningCadence,
            75 => ExdDescriptors::PerformanceCondition,
            76 => ExdDescriptors::CourseType,
            77 => ExdDescriptors::TimeInPowerZone,
            78 => ExdDescriptors::NavigationTurn,
            79 => ExdDescriptors::CourseLocation,
            80 => ExdDescriptors::NavigationLocation,
            81 => ExdDescriptors::Compass,
            82 => ExdDescriptors::GearCombo,
            83 => ExdDescriptors::MuscleOxygen,
            84 => ExdDescriptors::Icon,
            85 => ExdDescriptors::CompassHeading,
            86 => ExdDescriptors::GpsHeading,
            87 => ExdDescriptors::GpsElevation,
            88 => ExdDescriptors::AnaerobicTrainingEffect,
            89 => ExdDescriptors::Course,
            90 => ExdDescriptors::OffCourse,
            91 => ExdDescriptors::GlideRatio,
            92 => ExdDescriptors::VerticalDistance,
            93 => ExdDescriptors::Vmg,
            94 => ExdDescriptors::AmbientPressure,
            95 => ExdDescriptors::Pressure,
            96 => ExdDescriptors::Vam,
            _ => ExdDescriptors::UnknownVariant(value),
        }
    }
}
impl convert::From<i64> for ExdDescriptors {
    fn from(value: i64) -> Self {
        ExdDescriptors::from(value as u8)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum AutoActivityDetect {
    None,
    Running,
    Cycling,
    Swimming,
    Walking,
    Elliptical,
    Sedentary,
    UnknownVariant(u32),
}
impl AutoActivityDetect {
    pub fn as_u32(self) -> u32 {
        match self {
            AutoActivityDetect::None => 0,
            AutoActivityDetect::Running => 1,
            AutoActivityDetect::Cycling => 2,
            AutoActivityDetect::Swimming => 4,
            AutoActivityDetect::Walking => 8,
            AutoActivityDetect::Elliptical => 32,
            AutoActivityDetect::Sedentary => 1024,
            AutoActivityDetect::UnknownVariant(value) => value,
        }
    }
    pub fn as_i64(self) -> i64 {
        self.as_u32() as i64
    }
}
impl fmt::Display for AutoActivityDetect {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            AutoActivityDetect::None => writeln!(f, "none"),
            AutoActivityDetect::Running => writeln!(f, "running"),
            AutoActivityDetect::Cycling => writeln!(f, "cycling"),
            AutoActivityDetect::Swimming => writeln!(f, "swimming"),
            AutoActivityDetect::Walking => writeln!(f, "walking"),
            AutoActivityDetect::Elliptical => writeln!(f, "elliptical"),
            AutoActivityDetect::Sedentary => writeln!(f, "sedentary"),
            AutoActivityDetect::UnknownVariant(value) => writeln!(f, "unknown_variant_{}", *value),
        }
    }
}
impl convert::From<u32> for AutoActivityDetect {
    fn from(value: u32) -> Self {
        match value {
            0 => AutoActivityDetect::None,
            1 => AutoActivityDetect::Running,
            2 => AutoActivityDetect::Cycling,
            4 => AutoActivityDetect::Swimming,
            8 => AutoActivityDetect::Walking,
            32 => AutoActivityDetect::Elliptical,
            1024 => AutoActivityDetect::Sedentary,
            _ => AutoActivityDetect::UnknownVariant(value),
        }
    }
}
impl convert::From<i64> for AutoActivityDetect {
    fn from(value: i64) -> Self {
        AutoActivityDetect::from(value as u32)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum SupportedExdScreenLayouts {
    FullScreen,
    HalfVertical,
    HalfHorizontal,
    HalfVerticalRightSplit,
    HalfHorizontalBottomSplit,
    FullQuarterSplit,
    HalfVerticalLeftSplit,
    HalfHorizontalTopSplit,
    UnknownVariant(u32),
}
impl SupportedExdScreenLayouts {
    pub fn as_u32(self) -> u32 {
        match self {
            SupportedExdScreenLayouts::FullScreen => 1,
            SupportedExdScreenLayouts::HalfVertical => 2,
            SupportedExdScreenLayouts::HalfHorizontal => 4,
            SupportedExdScreenLayouts::HalfVerticalRightSplit => 8,
            SupportedExdScreenLayouts::HalfHorizontalBottomSplit => 16,
            SupportedExdScreenLayouts::FullQuarterSplit => 32,
            SupportedExdScreenLayouts::HalfVerticalLeftSplit => 64,
            SupportedExdScreenLayouts::HalfHorizontalTopSplit => 128,
            SupportedExdScreenLayouts::UnknownVariant(value) => value,
        }
    }
    pub fn as_i64(self) -> i64 {
        self.as_u32() as i64
    }
}
impl fmt::Display for SupportedExdScreenLayouts {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            SupportedExdScreenLayouts::FullScreen => writeln!(f, "full_screen"),
            SupportedExdScreenLayouts::HalfVertical => writeln!(f, "half_vertical"),
            SupportedExdScreenLayouts::HalfHorizontal => writeln!(f, "half_horizontal"),
            SupportedExdScreenLayouts::HalfVerticalRightSplit => {
                writeln!(f, "half_vertical_right_split")
            }
            SupportedExdScreenLayouts::HalfHorizontalBottomSplit => {
                writeln!(f, "half_horizontal_bottom_split")
            }
            SupportedExdScreenLayouts::FullQuarterSplit => writeln!(f, "full_quarter_split"),
            SupportedExdScreenLayouts::HalfVerticalLeftSplit => {
                writeln!(f, "half_vertical_left_split")
            }
            SupportedExdScreenLayouts::HalfHorizontalTopSplit => {
                writeln!(f, "half_horizontal_top_split")
            }
            SupportedExdScreenLayouts::UnknownVariant(value) => {
                writeln!(f, "unknown_variant_{}", *value)
            }
        }
    }
}
impl convert::From<u32> for SupportedExdScreenLayouts {
    fn from(value: u32) -> Self {
        match value {
            1 => SupportedExdScreenLayouts::FullScreen,
            2 => SupportedExdScreenLayouts::HalfVertical,
            4 => SupportedExdScreenLayouts::HalfHorizontal,
            8 => SupportedExdScreenLayouts::HalfVerticalRightSplit,
            16 => SupportedExdScreenLayouts::HalfHorizontalBottomSplit,
            32 => SupportedExdScreenLayouts::FullQuarterSplit,
            64 => SupportedExdScreenLayouts::HalfVerticalLeftSplit,
            128 => SupportedExdScreenLayouts::HalfHorizontalTopSplit,
            _ => SupportedExdScreenLayouts::UnknownVariant(value),
        }
    }
}
impl convert::From<i64> for SupportedExdScreenLayouts {
    fn from(value: i64) -> Self {
        SupportedExdScreenLayouts::from(value as u32)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum FitBaseType {
    Enum,
    Sint8,
    Uint8,
    String,
    Uint8z,
    Byte,
    Sint16,
    Uint16,
    Sint32,
    Uint32,
    Float32,
    Float64,
    Uint16z,
    Uint32z,
    Sint64,
    Uint64,
    Uint64z,
    UnknownVariant(u8),
}
impl FitBaseType {
    pub fn as_u8(self) -> u8 {
        match self {
            FitBaseType::Enum => 0,
            FitBaseType::Sint8 => 1,
            FitBaseType::Uint8 => 2,
            FitBaseType::String => 7,
            FitBaseType::Uint8z => 10,
            FitBaseType::Byte => 13,
            FitBaseType::Sint16 => 131,
            FitBaseType::Uint16 => 132,
            FitBaseType::Sint32 => 133,
            FitBaseType::Uint32 => 134,
            FitBaseType::Float32 => 136,
            FitBaseType::Float64 => 137,
            FitBaseType::Uint16z => 139,
            FitBaseType::Uint32z => 140,
            FitBaseType::Sint64 => 142,
            FitBaseType::Uint64 => 143,
            FitBaseType::Uint64z => 144,
            FitBaseType::UnknownVariant(value) => value,
        }
    }
    pub fn as_i64(self) -> i64 {
        self.as_u8() as i64
    }
}
impl fmt::Display for FitBaseType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            FitBaseType::Enum => writeln!(f, "enum"),
            FitBaseType::Sint8 => writeln!(f, "sint8"),
            FitBaseType::Uint8 => writeln!(f, "uint8"),
            FitBaseType::String => writeln!(f, "string"),
            FitBaseType::Uint8z => writeln!(f, "uint8z"),
            FitBaseType::Byte => writeln!(f, "byte"),
            FitBaseType::Sint16 => writeln!(f, "sint16"),
            FitBaseType::Uint16 => writeln!(f, "uint16"),
            FitBaseType::Sint32 => writeln!(f, "sint32"),
            FitBaseType::Uint32 => writeln!(f, "uint32"),
            FitBaseType::Float32 => writeln!(f, "float32"),
            FitBaseType::Float64 => writeln!(f, "float64"),
            FitBaseType::Uint16z => writeln!(f, "uint16z"),
            FitBaseType::Uint32z => writeln!(f, "uint32z"),
            FitBaseType::Sint64 => writeln!(f, "sint64"),
            FitBaseType::Uint64 => writeln!(f, "uint64"),
            FitBaseType::Uint64z => writeln!(f, "uint64z"),
            FitBaseType::UnknownVariant(value) => writeln!(f, "unknown_variant_{}", *value),
        }
    }
}
impl convert::From<u8> for FitBaseType {
    fn from(value: u8) -> Self {
        match value {
            0 => FitBaseType::Enum,
            1 => FitBaseType::Sint8,
            2 => FitBaseType::Uint8,
            7 => FitBaseType::String,
            10 => FitBaseType::Uint8z,
            13 => FitBaseType::Byte,
            131 => FitBaseType::Sint16,
            132 => FitBaseType::Uint16,
            133 => FitBaseType::Sint32,
            134 => FitBaseType::Uint32,
            136 => FitBaseType::Float32,
            137 => FitBaseType::Float64,
            139 => FitBaseType::Uint16z,
            140 => FitBaseType::Uint32z,
            142 => FitBaseType::Sint64,
            143 => FitBaseType::Uint64,
            144 => FitBaseType::Uint64z,
            _ => FitBaseType::UnknownVariant(value),
        }
    }
}
impl convert::From<i64> for FitBaseType {
    fn from(value: i64) -> Self {
        FitBaseType::from(value as u8)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum TurnType {
    ArrivingIdx,
    ArrivingLeftIdx,
    ArrivingRightIdx,
    ArrivingViaIdx,
    ArrivingViaLeftIdx,
    ArrivingViaRightIdx,
    BearKeepLeftIdx,
    BearKeepRightIdx,
    ContinueIdx,
    ExitLeftIdx,
    ExitRightIdx,
    FerryIdx,
    Roundabout45Idx,
    Roundabout90Idx,
    Roundabout135Idx,
    Roundabout180Idx,
    Roundabout225Idx,
    Roundabout270Idx,
    Roundabout315Idx,
    Roundabout360Idx,
    RoundaboutNeg45Idx,
    RoundaboutNeg90Idx,
    RoundaboutNeg135Idx,
    RoundaboutNeg180Idx,
    RoundaboutNeg225Idx,
    RoundaboutNeg270Idx,
    RoundaboutNeg315Idx,
    RoundaboutNeg360Idx,
    RoundaboutGenericIdx,
    RoundaboutNegGenericIdx,
    SharpTurnLeftIdx,
    SharpTurnRightIdx,
    TurnLeftIdx,
    TurnRightIdx,
    UturnLeftIdx,
    UturnRightIdx,
    IconInvIdx,
    IconIdxCnt,
    UnknownVariant(u8),
}
impl TurnType {
    pub fn as_u8(self) -> u8 {
        match self {
            TurnType::ArrivingIdx => 0,
            TurnType::ArrivingLeftIdx => 1,
            TurnType::ArrivingRightIdx => 2,
            TurnType::ArrivingViaIdx => 3,
            TurnType::ArrivingViaLeftIdx => 4,
            TurnType::ArrivingViaRightIdx => 5,
            TurnType::BearKeepLeftIdx => 6,
            TurnType::BearKeepRightIdx => 7,
            TurnType::ContinueIdx => 8,
            TurnType::ExitLeftIdx => 9,
            TurnType::ExitRightIdx => 10,
            TurnType::FerryIdx => 11,
            TurnType::Roundabout45Idx => 12,
            TurnType::Roundabout90Idx => 13,
            TurnType::Roundabout135Idx => 14,
            TurnType::Roundabout180Idx => 15,
            TurnType::Roundabout225Idx => 16,
            TurnType::Roundabout270Idx => 17,
            TurnType::Roundabout315Idx => 18,
            TurnType::Roundabout360Idx => 19,
            TurnType::RoundaboutNeg45Idx => 20,
            TurnType::RoundaboutNeg90Idx => 21,
            TurnType::RoundaboutNeg135Idx => 22,
            TurnType::RoundaboutNeg180Idx => 23,
            TurnType::RoundaboutNeg225Idx => 24,
            TurnType::RoundaboutNeg270Idx => 25,
            TurnType::RoundaboutNeg315Idx => 26,
            TurnType::RoundaboutNeg360Idx => 27,
            TurnType::RoundaboutGenericIdx => 28,
            TurnType::RoundaboutNegGenericIdx => 29,
            TurnType::SharpTurnLeftIdx => 30,
            TurnType::SharpTurnRightIdx => 31,
            TurnType::TurnLeftIdx => 32,
            TurnType::TurnRightIdx => 33,
            TurnType::UturnLeftIdx => 34,
            TurnType::UturnRightIdx => 35,
            TurnType::IconInvIdx => 36,
            TurnType::IconIdxCnt => 37,
            TurnType::UnknownVariant(value) => value,
        }
    }
    pub fn as_i64(self) -> i64 {
        self.as_u8() as i64
    }
}
impl fmt::Display for TurnType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            TurnType::ArrivingIdx => writeln!(f, "arriving_idx"),
            TurnType::ArrivingLeftIdx => writeln!(f, "arriving_left_idx"),
            TurnType::ArrivingRightIdx => writeln!(f, "arriving_right_idx"),
            TurnType::ArrivingViaIdx => writeln!(f, "arriving_via_idx"),
            TurnType::ArrivingViaLeftIdx => writeln!(f, "arriving_via_left_idx"),
            TurnType::ArrivingViaRightIdx => writeln!(f, "arriving_via_right_idx"),
            TurnType::BearKeepLeftIdx => writeln!(f, "bear_keep_left_idx"),
            TurnType::BearKeepRightIdx => writeln!(f, "bear_keep_right_idx"),
            TurnType::ContinueIdx => writeln!(f, "continue_idx"),
            TurnType::ExitLeftIdx => writeln!(f, "exit_left_idx"),
            TurnType::ExitRightIdx => writeln!(f, "exit_right_idx"),
            TurnType::FerryIdx => writeln!(f, "ferry_idx"),
            TurnType::Roundabout45Idx => writeln!(f, "roundabout_45_idx"),
            TurnType::Roundabout90Idx => writeln!(f, "roundabout_90_idx"),
            TurnType::Roundabout135Idx => writeln!(f, "roundabout_135_idx"),
            TurnType::Roundabout180Idx => writeln!(f, "roundabout_180_idx"),
            TurnType::Roundabout225Idx => writeln!(f, "roundabout_225_idx"),
            TurnType::Roundabout270Idx => writeln!(f, "roundabout_270_idx"),
            TurnType::Roundabout315Idx => writeln!(f, "roundabout_315_idx"),
            TurnType::Roundabout360Idx => writeln!(f, "roundabout_360_idx"),
            TurnType::RoundaboutNeg45Idx => writeln!(f, "roundabout_neg_45_idx"),
            TurnType::RoundaboutNeg90Idx => writeln!(f, "roundabout_neg_90_idx"),
            TurnType::RoundaboutNeg135Idx => writeln!(f, "roundabout_neg_135_idx"),
            TurnType::RoundaboutNeg180Idx => writeln!(f, "roundabout_neg_180_idx"),
            TurnType::RoundaboutNeg225Idx => writeln!(f, "roundabout_neg_225_idx"),
            TurnType::RoundaboutNeg270Idx => writeln!(f, "roundabout_neg_270_idx"),
            TurnType::RoundaboutNeg315Idx => writeln!(f, "roundabout_neg_315_idx"),
            TurnType::RoundaboutNeg360Idx => writeln!(f, "roundabout_neg_360_idx"),
            TurnType::RoundaboutGenericIdx => writeln!(f, "roundabout_generic_idx"),
            TurnType::RoundaboutNegGenericIdx => writeln!(f, "roundabout_neg_generic_idx"),
            TurnType::SharpTurnLeftIdx => writeln!(f, "sharp_turn_left_idx"),
            TurnType::SharpTurnRightIdx => writeln!(f, "sharp_turn_right_idx"),
            TurnType::TurnLeftIdx => writeln!(f, "turn_left_idx"),
            TurnType::TurnRightIdx => writeln!(f, "turn_right_idx"),
            TurnType::UturnLeftIdx => writeln!(f, "uturn_left_idx"),
            TurnType::UturnRightIdx => writeln!(f, "uturn_right_idx"),
            TurnType::IconInvIdx => writeln!(f, "icon_inv_idx"),
            TurnType::IconIdxCnt => writeln!(f, "icon_idx_cnt"),
            TurnType::UnknownVariant(value) => writeln!(f, "unknown_variant_{}", *value),
        }
    }
}
impl convert::From<u8> for TurnType {
    fn from(value: u8) -> Self {
        match value {
            0 => TurnType::ArrivingIdx,
            1 => TurnType::ArrivingLeftIdx,
            2 => TurnType::ArrivingRightIdx,
            3 => TurnType::ArrivingViaIdx,
            4 => TurnType::ArrivingViaLeftIdx,
            5 => TurnType::ArrivingViaRightIdx,
            6 => TurnType::BearKeepLeftIdx,
            7 => TurnType::BearKeepRightIdx,
            8 => TurnType::ContinueIdx,
            9 => TurnType::ExitLeftIdx,
            10 => TurnType::ExitRightIdx,
            11 => TurnType::FerryIdx,
            12 => TurnType::Roundabout45Idx,
            13 => TurnType::Roundabout90Idx,
            14 => TurnType::Roundabout135Idx,
            15 => TurnType::Roundabout180Idx,
            16 => TurnType::Roundabout225Idx,
            17 => TurnType::Roundabout270Idx,
            18 => TurnType::Roundabout315Idx,
            19 => TurnType::Roundabout360Idx,
            20 => TurnType::RoundaboutNeg45Idx,
            21 => TurnType::RoundaboutNeg90Idx,
            22 => TurnType::RoundaboutNeg135Idx,
            23 => TurnType::RoundaboutNeg180Idx,
            24 => TurnType::RoundaboutNeg225Idx,
            25 => TurnType::RoundaboutNeg270Idx,
            26 => TurnType::RoundaboutNeg315Idx,
            27 => TurnType::RoundaboutNeg360Idx,
            28 => TurnType::RoundaboutGenericIdx,
            29 => TurnType::RoundaboutNegGenericIdx,
            30 => TurnType::SharpTurnLeftIdx,
            31 => TurnType::SharpTurnRightIdx,
            32 => TurnType::TurnLeftIdx,
            33 => TurnType::TurnRightIdx,
            34 => TurnType::UturnLeftIdx,
            35 => TurnType::UturnRightIdx,
            36 => TurnType::IconInvIdx,
            37 => TurnType::IconIdxCnt,
            _ => TurnType::UnknownVariant(value),
        }
    }
}
impl convert::From<i64> for TurnType {
    fn from(value: i64) -> Self {
        TurnType::from(value as u8)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum BikeLightBeamAngleMode {
    Manual,
    Auto,
    UnknownVariant(u8),
}
impl BikeLightBeamAngleMode {
    pub fn as_u8(self) -> u8 {
        match self {
            BikeLightBeamAngleMode::Manual => 0,
            BikeLightBeamAngleMode::Auto => 1,
            BikeLightBeamAngleMode::UnknownVariant(value) => value,
        }
    }
    pub fn as_i64(self) -> i64 {
        self.as_u8() as i64
    }
}
impl fmt::Display for BikeLightBeamAngleMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            BikeLightBeamAngleMode::Manual => writeln!(f, "manual"),
            BikeLightBeamAngleMode::Auto => writeln!(f, "auto"),
            BikeLightBeamAngleMode::UnknownVariant(value) => {
                writeln!(f, "unknown_variant_{}", *value)
            }
        }
    }
}
impl convert::From<u8> for BikeLightBeamAngleMode {
    fn from(value: u8) -> Self {
        match value {
            0 => BikeLightBeamAngleMode::Manual,
            1 => BikeLightBeamAngleMode::Auto,
            _ => BikeLightBeamAngleMode::UnknownVariant(value),
        }
    }
}
impl convert::From<i64> for BikeLightBeamAngleMode {
    fn from(value: i64) -> Self {
        BikeLightBeamAngleMode::from(value as u8)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum FitBaseUnit {
    Other,
    Kilogram,
    Pound,
    UnknownVariant(u16),
}
impl FitBaseUnit {
    pub fn as_u16(self) -> u16 {
        match self {
            FitBaseUnit::Other => 0,
            FitBaseUnit::Kilogram => 1,
            FitBaseUnit::Pound => 2,
            FitBaseUnit::UnknownVariant(value) => value,
        }
    }
    pub fn as_i64(self) -> i64 {
        self.as_u16() as i64
    }
}
impl fmt::Display for FitBaseUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            FitBaseUnit::Other => writeln!(f, "other"),
            FitBaseUnit::Kilogram => writeln!(f, "kilogram"),
            FitBaseUnit::Pound => writeln!(f, "pound"),
            FitBaseUnit::UnknownVariant(value) => writeln!(f, "unknown_variant_{}", *value),
        }
    }
}
impl convert::From<u16> for FitBaseUnit {
    fn from(value: u16) -> Self {
        match value {
            0 => FitBaseUnit::Other,
            1 => FitBaseUnit::Kilogram,
            2 => FitBaseUnit::Pound,
            _ => FitBaseUnit::UnknownVariant(value),
        }
    }
}
impl convert::From<i64> for FitBaseUnit {
    fn from(value: i64) -> Self {
        FitBaseUnit::from(value as u16)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum SetType {
    Rest,
    Active,
    UnknownVariant(u8),
}
impl SetType {
    pub fn as_u8(self) -> u8 {
        match self {
            SetType::Rest => 0,
            SetType::Active => 1,
            SetType::UnknownVariant(value) => value,
        }
    }
    pub fn as_i64(self) -> i64 {
        self.as_u8() as i64
    }
}
impl fmt::Display for SetType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            SetType::Rest => writeln!(f, "rest"),
            SetType::Active => writeln!(f, "active"),
            SetType::UnknownVariant(value) => writeln!(f, "unknown_variant_{}", *value),
        }
    }
}
impl convert::From<u8> for SetType {
    fn from(value: u8) -> Self {
        match value {
            0 => SetType::Rest,
            1 => SetType::Active,
            _ => SetType::UnknownVariant(value),
        }
    }
}
impl convert::From<i64> for SetType {
    fn from(value: i64) -> Self {
        SetType::from(value as u8)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum ExerciseCategory {
    BenchPress,
    CalfRaise,
    Cardio,
    Carry,
    Chop,
    Core,
    Crunch,
    Curl,
    Deadlift,
    Flye,
    HipRaise,
    HipStability,
    HipSwing,
    Hyperextension,
    LateralRaise,
    LegCurl,
    LegRaise,
    Lunge,
    OlympicLift,
    Plank,
    Plyo,
    PullUp,
    PushUp,
    Row,
    ShoulderPress,
    ShoulderStability,
    Shrug,
    SitUp,
    Squat,
    TotalBody,
    TricepsExtension,
    WarmUp,
    Run,
    Unknown,
    UnknownVariant(u16),
}
impl ExerciseCategory {
    pub fn as_u16(self) -> u16 {
        match self {
            ExerciseCategory::BenchPress => 0,
            ExerciseCategory::CalfRaise => 1,
            ExerciseCategory::Cardio => 2,
            ExerciseCategory::Carry => 3,
            ExerciseCategory::Chop => 4,
            ExerciseCategory::Core => 5,
            ExerciseCategory::Crunch => 6,
            ExerciseCategory::Curl => 7,
            ExerciseCategory::Deadlift => 8,
            ExerciseCategory::Flye => 9,
            ExerciseCategory::HipRaise => 10,
            ExerciseCategory::HipStability => 11,
            ExerciseCategory::HipSwing => 12,
            ExerciseCategory::Hyperextension => 13,
            ExerciseCategory::LateralRaise => 14,
            ExerciseCategory::LegCurl => 15,
            ExerciseCategory::LegRaise => 16,
            ExerciseCategory::Lunge => 17,
            ExerciseCategory::OlympicLift => 18,
            ExerciseCategory::Plank => 19,
            ExerciseCategory::Plyo => 20,
            ExerciseCategory::PullUp => 21,
            ExerciseCategory::PushUp => 22,
            ExerciseCategory::Row => 23,
            ExerciseCategory::ShoulderPress => 24,
            ExerciseCategory::ShoulderStability => 25,
            ExerciseCategory::Shrug => 26,
            ExerciseCategory::SitUp => 27,
            ExerciseCategory::Squat => 28,
            ExerciseCategory::TotalBody => 29,
            ExerciseCategory::TricepsExtension => 30,
            ExerciseCategory::WarmUp => 31,
            ExerciseCategory::Run => 32,
            ExerciseCategory::Unknown => 65534,
            ExerciseCategory::UnknownVariant(value) => value,
        }
    }
    pub fn as_i64(self) -> i64 {
        self.as_u16() as i64
    }
}
impl fmt::Display for ExerciseCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            ExerciseCategory::BenchPress => writeln!(f, "bench_press"),
            ExerciseCategory::CalfRaise => writeln!(f, "calf_raise"),
            ExerciseCategory::Cardio => writeln!(f, "cardio"),
            ExerciseCategory::Carry => writeln!(f, "carry"),
            ExerciseCategory::Chop => writeln!(f, "chop"),
            ExerciseCategory::Core => writeln!(f, "core"),
            ExerciseCategory::Crunch => writeln!(f, "crunch"),
            ExerciseCategory::Curl => writeln!(f, "curl"),
            ExerciseCategory::Deadlift => writeln!(f, "deadlift"),
            ExerciseCategory::Flye => writeln!(f, "flye"),
            ExerciseCategory::HipRaise => writeln!(f, "hip_raise"),
            ExerciseCategory::HipStability => writeln!(f, "hip_stability"),
            ExerciseCategory::HipSwing => writeln!(f, "hip_swing"),
            ExerciseCategory::Hyperextension => writeln!(f, "hyperextension"),
            ExerciseCategory::LateralRaise => writeln!(f, "lateral_raise"),
            ExerciseCategory::LegCurl => writeln!(f, "leg_curl"),
            ExerciseCategory::LegRaise => writeln!(f, "leg_raise"),
            ExerciseCategory::Lunge => writeln!(f, "lunge"),
            ExerciseCategory::OlympicLift => writeln!(f, "olympic_lift"),
            ExerciseCategory::Plank => writeln!(f, "plank"),
            ExerciseCategory::Plyo => writeln!(f, "plyo"),
            ExerciseCategory::PullUp => writeln!(f, "pull_up"),
            ExerciseCategory::PushUp => writeln!(f, "push_up"),
            ExerciseCategory::Row => writeln!(f, "row"),
            ExerciseCategory::ShoulderPress => writeln!(f, "shoulder_press"),
            ExerciseCategory::ShoulderStability => writeln!(f, "shoulder_stability"),
            ExerciseCategory::Shrug => writeln!(f, "shrug"),
            ExerciseCategory::SitUp => writeln!(f, "sit_up"),
            ExerciseCategory::Squat => writeln!(f, "squat"),
            ExerciseCategory::TotalBody => writeln!(f, "total_body"),
            ExerciseCategory::TricepsExtension => writeln!(f, "triceps_extension"),
            ExerciseCategory::WarmUp => writeln!(f, "warm_up"),
            ExerciseCategory::Run => writeln!(f, "run"),
            ExerciseCategory::Unknown => writeln!(f, "unknown"),
            ExerciseCategory::UnknownVariant(value) => writeln!(f, "unknown_variant_{}", *value),
        }
    }
}
impl convert::From<u16> for ExerciseCategory {
    fn from(value: u16) -> Self {
        match value {
            0 => ExerciseCategory::BenchPress,
            1 => ExerciseCategory::CalfRaise,
            2 => ExerciseCategory::Cardio,
            3 => ExerciseCategory::Carry,
            4 => ExerciseCategory::Chop,
            5 => ExerciseCategory::Core,
            6 => ExerciseCategory::Crunch,
            7 => ExerciseCategory::Curl,
            8 => ExerciseCategory::Deadlift,
            9 => ExerciseCategory::Flye,
            10 => ExerciseCategory::HipRaise,
            11 => ExerciseCategory::HipStability,
            12 => ExerciseCategory::HipSwing,
            13 => ExerciseCategory::Hyperextension,
            14 => ExerciseCategory::LateralRaise,
            15 => ExerciseCategory::LegCurl,
            16 => ExerciseCategory::LegRaise,
            17 => ExerciseCategory::Lunge,
            18 => ExerciseCategory::OlympicLift,
            19 => ExerciseCategory::Plank,
            20 => ExerciseCategory::Plyo,
            21 => ExerciseCategory::PullUp,
            22 => ExerciseCategory::PushUp,
            23 => ExerciseCategory::Row,
            24 => ExerciseCategory::ShoulderPress,
            25 => ExerciseCategory::ShoulderStability,
            26 => ExerciseCategory::Shrug,
            27 => ExerciseCategory::SitUp,
            28 => ExerciseCategory::Squat,
            29 => ExerciseCategory::TotalBody,
            30 => ExerciseCategory::TricepsExtension,
            31 => ExerciseCategory::WarmUp,
            32 => ExerciseCategory::Run,
            65534 => ExerciseCategory::Unknown,
            _ => ExerciseCategory::UnknownVariant(value),
        }
    }
}
impl convert::From<i64> for ExerciseCategory {
    fn from(value: i64) -> Self {
        ExerciseCategory::from(value as u16)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum BenchPressExerciseName {
    AlternatingDumbbellChestPressOnSwissBall,
    BarbellBenchPress,
    BarbellBoardBenchPress,
    BarbellFloorPress,
    CloseGripBarbellBenchPress,
    DeclineDumbbellBenchPress,
    DumbbellBenchPress,
    DumbbellFloorPress,
    InclineBarbellBenchPress,
    InclineDumbbellBenchPress,
    InclineSmithMachineBenchPress,
    IsometricBarbellBenchPress,
    KettlebellChestPress,
    NeutralGripDumbbellBenchPress,
    NeutralGripDumbbellInclineBenchPress,
    OneArmFloorPress,
    WeightedOneArmFloorPress,
    PartialLockout,
    ReverseGripBarbellBenchPress,
    ReverseGripInclineBenchPress,
    SingleArmCableChestPress,
    SingleArmDumbbellBenchPress,
    SmithMachineBenchPress,
    SwissBallDumbbellChestPress,
    TripleStopBarbellBenchPress,
    WideGripBarbellBenchPress,
    AlternatingDumbbellChestPress,
    UnknownVariant(u16),
}
impl BenchPressExerciseName {
    pub fn as_u16(self) -> u16 {
        match self {
            BenchPressExerciseName::AlternatingDumbbellChestPressOnSwissBall => 0,
            BenchPressExerciseName::BarbellBenchPress => 1,
            BenchPressExerciseName::BarbellBoardBenchPress => 2,
            BenchPressExerciseName::BarbellFloorPress => 3,
            BenchPressExerciseName::CloseGripBarbellBenchPress => 4,
            BenchPressExerciseName::DeclineDumbbellBenchPress => 5,
            BenchPressExerciseName::DumbbellBenchPress => 6,
            BenchPressExerciseName::DumbbellFloorPress => 7,
            BenchPressExerciseName::InclineBarbellBenchPress => 8,
            BenchPressExerciseName::InclineDumbbellBenchPress => 9,
            BenchPressExerciseName::InclineSmithMachineBenchPress => 10,
            BenchPressExerciseName::IsometricBarbellBenchPress => 11,
            BenchPressExerciseName::KettlebellChestPress => 12,
            BenchPressExerciseName::NeutralGripDumbbellBenchPress => 13,
            BenchPressExerciseName::NeutralGripDumbbellInclineBenchPress => 14,
            BenchPressExerciseName::OneArmFloorPress => 15,
            BenchPressExerciseName::WeightedOneArmFloorPress => 16,
            BenchPressExerciseName::PartialLockout => 17,
            BenchPressExerciseName::ReverseGripBarbellBenchPress => 18,
            BenchPressExerciseName::ReverseGripInclineBenchPress => 19,
            BenchPressExerciseName::SingleArmCableChestPress => 20,
            BenchPressExerciseName::SingleArmDumbbellBenchPress => 21,
            BenchPressExerciseName::SmithMachineBenchPress => 22,
            BenchPressExerciseName::SwissBallDumbbellChestPress => 23,
            BenchPressExerciseName::TripleStopBarbellBenchPress => 24,
            BenchPressExerciseName::WideGripBarbellBenchPress => 25,
            BenchPressExerciseName::AlternatingDumbbellChestPress => 26,
            BenchPressExerciseName::UnknownVariant(value) => value,
        }
    }
    pub fn as_i64(self) -> i64 {
        self.as_u16() as i64
    }
}
impl fmt::Display for BenchPressExerciseName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            BenchPressExerciseName::AlternatingDumbbellChestPressOnSwissBall => {
                writeln!(f, "alternating_dumbbell_chest_press_on_swiss_ball")
            }
            BenchPressExerciseName::BarbellBenchPress => writeln!(f, "barbell_bench_press"),
            BenchPressExerciseName::BarbellBoardBenchPress => {
                writeln!(f, "barbell_board_bench_press")
            }
            BenchPressExerciseName::BarbellFloorPress => writeln!(f, "barbell_floor_press"),
            BenchPressExerciseName::CloseGripBarbellBenchPress => {
                writeln!(f, "close_grip_barbell_bench_press")
            }
            BenchPressExerciseName::DeclineDumbbellBenchPress => {
                writeln!(f, "decline_dumbbell_bench_press")
            }
            BenchPressExerciseName::DumbbellBenchPress => writeln!(f, "dumbbell_bench_press"),
            BenchPressExerciseName::DumbbellFloorPress => writeln!(f, "dumbbell_floor_press"),
            BenchPressExerciseName::InclineBarbellBenchPress => {
                writeln!(f, "incline_barbell_bench_press")
            }
            BenchPressExerciseName::InclineDumbbellBenchPress => {
                writeln!(f, "incline_dumbbell_bench_press")
            }
            BenchPressExerciseName::InclineSmithMachineBenchPress => {
                writeln!(f, "incline_smith_machine_bench_press")
            }
            BenchPressExerciseName::IsometricBarbellBenchPress => {
                writeln!(f, "isometric_barbell_bench_press")
            }
            BenchPressExerciseName::KettlebellChestPress => writeln!(f, "kettlebell_chest_press"),
            BenchPressExerciseName::NeutralGripDumbbellBenchPress => {
                writeln!(f, "neutral_grip_dumbbell_bench_press")
            }
            BenchPressExerciseName::NeutralGripDumbbellInclineBenchPress => {
                writeln!(f, "neutral_grip_dumbbell_incline_bench_press")
            }
            BenchPressExerciseName::OneArmFloorPress => writeln!(f, "one_arm_floor_press"),
            BenchPressExerciseName::WeightedOneArmFloorPress => {
                writeln!(f, "weighted_one_arm_floor_press")
            }
            BenchPressExerciseName::PartialLockout => writeln!(f, "partial_lockout"),
            BenchPressExerciseName::ReverseGripBarbellBenchPress => {
                writeln!(f, "reverse_grip_barbell_bench_press")
            }
            BenchPressExerciseName::ReverseGripInclineBenchPress => {
                writeln!(f, "reverse_grip_incline_bench_press")
            }
            BenchPressExerciseName::SingleArmCableChestPress => {
                writeln!(f, "single_arm_cable_chest_press")
            }
            BenchPressExerciseName::SingleArmDumbbellBenchPress => {
                writeln!(f, "single_arm_dumbbell_bench_press")
            }
            BenchPressExerciseName::SmithMachineBenchPress => {
                writeln!(f, "smith_machine_bench_press")
            }
            BenchPressExerciseName::SwissBallDumbbellChestPress => {
                writeln!(f, "swiss_ball_dumbbell_chest_press")
            }
            BenchPressExerciseName::TripleStopBarbellBenchPress => {
                writeln!(f, "triple_stop_barbell_bench_press")
            }
            BenchPressExerciseName::WideGripBarbellBenchPress => {
                writeln!(f, "wide_grip_barbell_bench_press")
            }
            BenchPressExerciseName::AlternatingDumbbellChestPress => {
                writeln!(f, "alternating_dumbbell_chest_press")
            }
            BenchPressExerciseName::UnknownVariant(value) => {
                writeln!(f, "unknown_variant_{}", *value)
            }
        }
    }
}
impl convert::From<u16> for BenchPressExerciseName {
    fn from(value: u16) -> Self {
        match value {
            0 => BenchPressExerciseName::AlternatingDumbbellChestPressOnSwissBall,
            1 => BenchPressExerciseName::BarbellBenchPress,
            2 => BenchPressExerciseName::BarbellBoardBenchPress,
            3 => BenchPressExerciseName::BarbellFloorPress,
            4 => BenchPressExerciseName::CloseGripBarbellBenchPress,
            5 => BenchPressExerciseName::DeclineDumbbellBenchPress,
            6 => BenchPressExerciseName::DumbbellBenchPress,
            7 => BenchPressExerciseName::DumbbellFloorPress,
            8 => BenchPressExerciseName::InclineBarbellBenchPress,
            9 => BenchPressExerciseName::InclineDumbbellBenchPress,
            10 => BenchPressExerciseName::InclineSmithMachineBenchPress,
            11 => BenchPressExerciseName::IsometricBarbellBenchPress,
            12 => BenchPressExerciseName::KettlebellChestPress,
            13 => BenchPressExerciseName::NeutralGripDumbbellBenchPress,
            14 => BenchPressExerciseName::NeutralGripDumbbellInclineBenchPress,
            15 => BenchPressExerciseName::OneArmFloorPress,
            16 => BenchPressExerciseName::WeightedOneArmFloorPress,
            17 => BenchPressExerciseName::PartialLockout,
            18 => BenchPressExerciseName::ReverseGripBarbellBenchPress,
            19 => BenchPressExerciseName::ReverseGripInclineBenchPress,
            20 => BenchPressExerciseName::SingleArmCableChestPress,
            21 => BenchPressExerciseName::SingleArmDumbbellBenchPress,
            22 => BenchPressExerciseName::SmithMachineBenchPress,
            23 => BenchPressExerciseName::SwissBallDumbbellChestPress,
            24 => BenchPressExerciseName::TripleStopBarbellBenchPress,
            25 => BenchPressExerciseName::WideGripBarbellBenchPress,
            26 => BenchPressExerciseName::AlternatingDumbbellChestPress,
            _ => BenchPressExerciseName::UnknownVariant(value),
        }
    }
}
impl convert::From<i64> for BenchPressExerciseName {
    fn from(value: i64) -> Self {
        BenchPressExerciseName::from(value as u16)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum CalfRaiseExerciseName {
    Name3WayCalfRaise,
    Name3WayWeightedCalfRaise,
    Name3WaySingleLegCalfRaise,
    Name3WayWeightedSingleLegCalfRaise,
    DonkeyCalfRaise,
    WeightedDonkeyCalfRaise,
    SeatedCalfRaise,
    WeightedSeatedCalfRaise,
    SeatedDumbbellToeRaise,
    SingleLegBentKneeCalfRaise,
    WeightedSingleLegBentKneeCalfRaise,
    SingleLegDeclinePushUp,
    SingleLegDonkeyCalfRaise,
    WeightedSingleLegDonkeyCalfRaise,
    SingleLegHipRaiseWithKneeHold,
    SingleLegStandingCalfRaise,
    SingleLegStandingDumbbellCalfRaise,
    StandingBarbellCalfRaise,
    StandingCalfRaise,
    WeightedStandingCalfRaise,
    StandingDumbbellCalfRaise,
    UnknownVariant(u16),
}
impl CalfRaiseExerciseName {
    pub fn as_u16(self) -> u16 {
        match self {
            CalfRaiseExerciseName::Name3WayCalfRaise => 0,
            CalfRaiseExerciseName::Name3WayWeightedCalfRaise => 1,
            CalfRaiseExerciseName::Name3WaySingleLegCalfRaise => 2,
            CalfRaiseExerciseName::Name3WayWeightedSingleLegCalfRaise => 3,
            CalfRaiseExerciseName::DonkeyCalfRaise => 4,
            CalfRaiseExerciseName::WeightedDonkeyCalfRaise => 5,
            CalfRaiseExerciseName::SeatedCalfRaise => 6,
            CalfRaiseExerciseName::WeightedSeatedCalfRaise => 7,
            CalfRaiseExerciseName::SeatedDumbbellToeRaise => 8,
            CalfRaiseExerciseName::SingleLegBentKneeCalfRaise => 9,
            CalfRaiseExerciseName::WeightedSingleLegBentKneeCalfRaise => 10,
            CalfRaiseExerciseName::SingleLegDeclinePushUp => 11,
            CalfRaiseExerciseName::SingleLegDonkeyCalfRaise => 12,
            CalfRaiseExerciseName::WeightedSingleLegDonkeyCalfRaise => 13,
            CalfRaiseExerciseName::SingleLegHipRaiseWithKneeHold => 14,
            CalfRaiseExerciseName::SingleLegStandingCalfRaise => 15,
            CalfRaiseExerciseName::SingleLegStandingDumbbellCalfRaise => 16,
            CalfRaiseExerciseName::StandingBarbellCalfRaise => 17,
            CalfRaiseExerciseName::StandingCalfRaise => 18,
            CalfRaiseExerciseName::WeightedStandingCalfRaise => 19,
            CalfRaiseExerciseName::StandingDumbbellCalfRaise => 20,
            CalfRaiseExerciseName::UnknownVariant(value) => value,
        }
    }
    pub fn as_i64(self) -> i64 {
        self.as_u16() as i64
    }
}
impl fmt::Display for CalfRaiseExerciseName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            CalfRaiseExerciseName::Name3WayCalfRaise => writeln!(f, "3_way_calf_raise"),
            CalfRaiseExerciseName::Name3WayWeightedCalfRaise => {
                writeln!(f, "3_way_weighted_calf_raise")
            }
            CalfRaiseExerciseName::Name3WaySingleLegCalfRaise => {
                writeln!(f, "3_way_single_leg_calf_raise")
            }
            CalfRaiseExerciseName::Name3WayWeightedSingleLegCalfRaise => {
                writeln!(f, "3_way_weighted_single_leg_calf_raise")
            }
            CalfRaiseExerciseName::DonkeyCalfRaise => writeln!(f, "donkey_calf_raise"),
            CalfRaiseExerciseName::WeightedDonkeyCalfRaise => {
                writeln!(f, "weighted_donkey_calf_raise")
            }
            CalfRaiseExerciseName::SeatedCalfRaise => writeln!(f, "seated_calf_raise"),
            CalfRaiseExerciseName::WeightedSeatedCalfRaise => {
                writeln!(f, "weighted_seated_calf_raise")
            }
            CalfRaiseExerciseName::SeatedDumbbellToeRaise => {
                writeln!(f, "seated_dumbbell_toe_raise")
            }
            CalfRaiseExerciseName::SingleLegBentKneeCalfRaise => {
                writeln!(f, "single_leg_bent_knee_calf_raise")
            }
            CalfRaiseExerciseName::WeightedSingleLegBentKneeCalfRaise => {
                writeln!(f, "weighted_single_leg_bent_knee_calf_raise")
            }
            CalfRaiseExerciseName::SingleLegDeclinePushUp => {
                writeln!(f, "single_leg_decline_push_up")
            }
            CalfRaiseExerciseName::SingleLegDonkeyCalfRaise => {
                writeln!(f, "single_leg_donkey_calf_raise")
            }
            CalfRaiseExerciseName::WeightedSingleLegDonkeyCalfRaise => {
                writeln!(f, "weighted_single_leg_donkey_calf_raise")
            }
            CalfRaiseExerciseName::SingleLegHipRaiseWithKneeHold => {
                writeln!(f, "single_leg_hip_raise_with_knee_hold")
            }
            CalfRaiseExerciseName::SingleLegStandingCalfRaise => {
                writeln!(f, "single_leg_standing_calf_raise")
            }
            CalfRaiseExerciseName::SingleLegStandingDumbbellCalfRaise => {
                writeln!(f, "single_leg_standing_dumbbell_calf_raise")
            }
            CalfRaiseExerciseName::StandingBarbellCalfRaise => {
                writeln!(f, "standing_barbell_calf_raise")
            }
            CalfRaiseExerciseName::StandingCalfRaise => writeln!(f, "standing_calf_raise"),
            CalfRaiseExerciseName::WeightedStandingCalfRaise => {
                writeln!(f, "weighted_standing_calf_raise")
            }
            CalfRaiseExerciseName::StandingDumbbellCalfRaise => {
                writeln!(f, "standing_dumbbell_calf_raise")
            }
            CalfRaiseExerciseName::UnknownVariant(value) => {
                writeln!(f, "unknown_variant_{}", *value)
            }
        }
    }
}
impl convert::From<u16> for CalfRaiseExerciseName {
    fn from(value: u16) -> Self {
        match value {
            0 => CalfRaiseExerciseName::Name3WayCalfRaise,
            1 => CalfRaiseExerciseName::Name3WayWeightedCalfRaise,
            2 => CalfRaiseExerciseName::Name3WaySingleLegCalfRaise,
            3 => CalfRaiseExerciseName::Name3WayWeightedSingleLegCalfRaise,
            4 => CalfRaiseExerciseName::DonkeyCalfRaise,
            5 => CalfRaiseExerciseName::WeightedDonkeyCalfRaise,
            6 => CalfRaiseExerciseName::SeatedCalfRaise,
            7 => CalfRaiseExerciseName::WeightedSeatedCalfRaise,
            8 => CalfRaiseExerciseName::SeatedDumbbellToeRaise,
            9 => CalfRaiseExerciseName::SingleLegBentKneeCalfRaise,
            10 => CalfRaiseExerciseName::WeightedSingleLegBentKneeCalfRaise,
            11 => CalfRaiseExerciseName::SingleLegDeclinePushUp,
            12 => CalfRaiseExerciseName::SingleLegDonkeyCalfRaise,
            13 => CalfRaiseExerciseName::WeightedSingleLegDonkeyCalfRaise,
            14 => CalfRaiseExerciseName::SingleLegHipRaiseWithKneeHold,
            15 => CalfRaiseExerciseName::SingleLegStandingCalfRaise,
            16 => CalfRaiseExerciseName::SingleLegStandingDumbbellCalfRaise,
            17 => CalfRaiseExerciseName::StandingBarbellCalfRaise,
            18 => CalfRaiseExerciseName::StandingCalfRaise,
            19 => CalfRaiseExerciseName::WeightedStandingCalfRaise,
            20 => CalfRaiseExerciseName::StandingDumbbellCalfRaise,
            _ => CalfRaiseExerciseName::UnknownVariant(value),
        }
    }
}
impl convert::From<i64> for CalfRaiseExerciseName {
    fn from(value: i64) -> Self {
        CalfRaiseExerciseName::from(value as u16)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum CardioExerciseName {
    BobAndWeaveCircle,
    WeightedBobAndWeaveCircle,
    CardioCoreCrawl,
    WeightedCardioCoreCrawl,
    DoubleUnder,
    WeightedDoubleUnder,
    JumpRope,
    WeightedJumpRope,
    JumpRopeCrossover,
    WeightedJumpRopeCrossover,
    JumpRopeJog,
    WeightedJumpRopeJog,
    JumpingJacks,
    WeightedJumpingJacks,
    SkiMoguls,
    WeightedSkiMoguls,
    SplitJacks,
    WeightedSplitJacks,
    SquatJacks,
    WeightedSquatJacks,
    TripleUnder,
    WeightedTripleUnder,
    UnknownVariant(u16),
}
impl CardioExerciseName {
    pub fn as_u16(self) -> u16 {
        match self {
            CardioExerciseName::BobAndWeaveCircle => 0,
            CardioExerciseName::WeightedBobAndWeaveCircle => 1,
            CardioExerciseName::CardioCoreCrawl => 2,
            CardioExerciseName::WeightedCardioCoreCrawl => 3,
            CardioExerciseName::DoubleUnder => 4,
            CardioExerciseName::WeightedDoubleUnder => 5,
            CardioExerciseName::JumpRope => 6,
            CardioExerciseName::WeightedJumpRope => 7,
            CardioExerciseName::JumpRopeCrossover => 8,
            CardioExerciseName::WeightedJumpRopeCrossover => 9,
            CardioExerciseName::JumpRopeJog => 10,
            CardioExerciseName::WeightedJumpRopeJog => 11,
            CardioExerciseName::JumpingJacks => 12,
            CardioExerciseName::WeightedJumpingJacks => 13,
            CardioExerciseName::SkiMoguls => 14,
            CardioExerciseName::WeightedSkiMoguls => 15,
            CardioExerciseName::SplitJacks => 16,
            CardioExerciseName::WeightedSplitJacks => 17,
            CardioExerciseName::SquatJacks => 18,
            CardioExerciseName::WeightedSquatJacks => 19,
            CardioExerciseName::TripleUnder => 20,
            CardioExerciseName::WeightedTripleUnder => 21,
            CardioExerciseName::UnknownVariant(value) => value,
        }
    }
    pub fn as_i64(self) -> i64 {
        self.as_u16() as i64
    }
}
impl fmt::Display for CardioExerciseName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            CardioExerciseName::BobAndWeaveCircle => writeln!(f, "bob_and_weave_circle"),
            CardioExerciseName::WeightedBobAndWeaveCircle => {
                writeln!(f, "weighted_bob_and_weave_circle")
            }
            CardioExerciseName::CardioCoreCrawl => writeln!(f, "cardio_core_crawl"),
            CardioExerciseName::WeightedCardioCoreCrawl => {
                writeln!(f, "weighted_cardio_core_crawl")
            }
            CardioExerciseName::DoubleUnder => writeln!(f, "double_under"),
            CardioExerciseName::WeightedDoubleUnder => writeln!(f, "weighted_double_under"),
            CardioExerciseName::JumpRope => writeln!(f, "jump_rope"),
            CardioExerciseName::WeightedJumpRope => writeln!(f, "weighted_jump_rope"),
            CardioExerciseName::JumpRopeCrossover => writeln!(f, "jump_rope_crossover"),
            CardioExerciseName::WeightedJumpRopeCrossover => {
                writeln!(f, "weighted_jump_rope_crossover")
            }
            CardioExerciseName::JumpRopeJog => writeln!(f, "jump_rope_jog"),
            CardioExerciseName::WeightedJumpRopeJog => writeln!(f, "weighted_jump_rope_jog"),
            CardioExerciseName::JumpingJacks => writeln!(f, "jumping_jacks"),
            CardioExerciseName::WeightedJumpingJacks => writeln!(f, "weighted_jumping_jacks"),
            CardioExerciseName::SkiMoguls => writeln!(f, "ski_moguls"),
            CardioExerciseName::WeightedSkiMoguls => writeln!(f, "weighted_ski_moguls"),
            CardioExerciseName::SplitJacks => writeln!(f, "split_jacks"),
            CardioExerciseName::WeightedSplitJacks => writeln!(f, "weighted_split_jacks"),
            CardioExerciseName::SquatJacks => writeln!(f, "squat_jacks"),
            CardioExerciseName::WeightedSquatJacks => writeln!(f, "weighted_squat_jacks"),
            CardioExerciseName::TripleUnder => writeln!(f, "triple_under"),
            CardioExerciseName::WeightedTripleUnder => writeln!(f, "weighted_triple_under"),
            CardioExerciseName::UnknownVariant(value) => writeln!(f, "unknown_variant_{}", *value),
        }
    }
}
impl convert::From<u16> for CardioExerciseName {
    fn from(value: u16) -> Self {
        match value {
            0 => CardioExerciseName::BobAndWeaveCircle,
            1 => CardioExerciseName::WeightedBobAndWeaveCircle,
            2 => CardioExerciseName::CardioCoreCrawl,
            3 => CardioExerciseName::WeightedCardioCoreCrawl,
            4 => CardioExerciseName::DoubleUnder,
            5 => CardioExerciseName::WeightedDoubleUnder,
            6 => CardioExerciseName::JumpRope,
            7 => CardioExerciseName::WeightedJumpRope,
            8 => CardioExerciseName::JumpRopeCrossover,
            9 => CardioExerciseName::WeightedJumpRopeCrossover,
            10 => CardioExerciseName::JumpRopeJog,
            11 => CardioExerciseName::WeightedJumpRopeJog,
            12 => CardioExerciseName::JumpingJacks,
            13 => CardioExerciseName::WeightedJumpingJacks,
            14 => CardioExerciseName::SkiMoguls,
            15 => CardioExerciseName::WeightedSkiMoguls,
            16 => CardioExerciseName::SplitJacks,
            17 => CardioExerciseName::WeightedSplitJacks,
            18 => CardioExerciseName::SquatJacks,
            19 => CardioExerciseName::WeightedSquatJacks,
            20 => CardioExerciseName::TripleUnder,
            21 => CardioExerciseName::WeightedTripleUnder,
            _ => CardioExerciseName::UnknownVariant(value),
        }
    }
}
impl convert::From<i64> for CardioExerciseName {
    fn from(value: i64) -> Self {
        CardioExerciseName::from(value as u16)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum CarryExerciseName {
    BarHolds,
    FarmersWalk,
    FarmersWalkOnToes,
    HexDumbbellHold,
    OverheadCarry,
    UnknownVariant(u16),
}
impl CarryExerciseName {
    pub fn as_u16(self) -> u16 {
        match self {
            CarryExerciseName::BarHolds => 0,
            CarryExerciseName::FarmersWalk => 1,
            CarryExerciseName::FarmersWalkOnToes => 2,
            CarryExerciseName::HexDumbbellHold => 3,
            CarryExerciseName::OverheadCarry => 4,
            CarryExerciseName::UnknownVariant(value) => value,
        }
    }
    pub fn as_i64(self) -> i64 {
        self.as_u16() as i64
    }
}
impl fmt::Display for CarryExerciseName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            CarryExerciseName::BarHolds => writeln!(f, "bar_holds"),
            CarryExerciseName::FarmersWalk => writeln!(f, "farmers_walk"),
            CarryExerciseName::FarmersWalkOnToes => writeln!(f, "farmers_walk_on_toes"),
            CarryExerciseName::HexDumbbellHold => writeln!(f, "hex_dumbbell_hold"),
            CarryExerciseName::OverheadCarry => writeln!(f, "overhead_carry"),
            CarryExerciseName::UnknownVariant(value) => writeln!(f, "unknown_variant_{}", *value),
        }
    }
}
impl convert::From<u16> for CarryExerciseName {
    fn from(value: u16) -> Self {
        match value {
            0 => CarryExerciseName::BarHolds,
            1 => CarryExerciseName::FarmersWalk,
            2 => CarryExerciseName::FarmersWalkOnToes,
            3 => CarryExerciseName::HexDumbbellHold,
            4 => CarryExerciseName::OverheadCarry,
            _ => CarryExerciseName::UnknownVariant(value),
        }
    }
}
impl convert::From<i64> for CarryExerciseName {
    fn from(value: i64) -> Self {
        CarryExerciseName::from(value as u16)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum ChopExerciseName {
    CablePullThrough,
    CableRotationalLift,
    CableWoodchop,
    CrossChopToKnee,
    WeightedCrossChopToKnee,
    DumbbellChop,
    HalfKneelingRotation,
    WeightedHalfKneelingRotation,
    HalfKneelingRotationalChop,
    HalfKneelingRotationalReverseChop,
    HalfKneelingStabilityChop,
    HalfKneelingStabilityReverseChop,
    KneelingRotationalChop,
    KneelingRotationalReverseChop,
    KneelingStabilityChop,
    KneelingWoodchopper,
    MedicineBallWoodChops,
    PowerSquatChops,
    WeightedPowerSquatChops,
    StandingRotationalChop,
    StandingSplitRotationalChop,
    StandingSplitRotationalReverseChop,
    StandingStabilityReverseChop,
    UnknownVariant(u16),
}
impl ChopExerciseName {
    pub fn as_u16(self) -> u16 {
        match self {
            ChopExerciseName::CablePullThrough => 0,
            ChopExerciseName::CableRotationalLift => 1,
            ChopExerciseName::CableWoodchop => 2,
            ChopExerciseName::CrossChopToKnee => 3,
            ChopExerciseName::WeightedCrossChopToKnee => 4,
            ChopExerciseName::DumbbellChop => 5,
            ChopExerciseName::HalfKneelingRotation => 6,
            ChopExerciseName::WeightedHalfKneelingRotation => 7,
            ChopExerciseName::HalfKneelingRotationalChop => 8,
            ChopExerciseName::HalfKneelingRotationalReverseChop => 9,
            ChopExerciseName::HalfKneelingStabilityChop => 10,
            ChopExerciseName::HalfKneelingStabilityReverseChop => 11,
            ChopExerciseName::KneelingRotationalChop => 12,
            ChopExerciseName::KneelingRotationalReverseChop => 13,
            ChopExerciseName::KneelingStabilityChop => 14,
            ChopExerciseName::KneelingWoodchopper => 15,
            ChopExerciseName::MedicineBallWoodChops => 16,
            ChopExerciseName::PowerSquatChops => 17,
            ChopExerciseName::WeightedPowerSquatChops => 18,
            ChopExerciseName::StandingRotationalChop => 19,
            ChopExerciseName::StandingSplitRotationalChop => 20,
            ChopExerciseName::StandingSplitRotationalReverseChop => 21,
            ChopExerciseName::StandingStabilityReverseChop => 22,
            ChopExerciseName::UnknownVariant(value) => value,
        }
    }
    pub fn as_i64(self) -> i64 {
        self.as_u16() as i64
    }
}
impl fmt::Display for ChopExerciseName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            ChopExerciseName::CablePullThrough => writeln!(f, "cable_pull_through"),
            ChopExerciseName::CableRotationalLift => writeln!(f, "cable_rotational_lift"),
            ChopExerciseName::CableWoodchop => writeln!(f, "cable_woodchop"),
            ChopExerciseName::CrossChopToKnee => writeln!(f, "cross_chop_to_knee"),
            ChopExerciseName::WeightedCrossChopToKnee => writeln!(f, "weighted_cross_chop_to_knee"),
            ChopExerciseName::DumbbellChop => writeln!(f, "dumbbell_chop"),
            ChopExerciseName::HalfKneelingRotation => writeln!(f, "half_kneeling_rotation"),
            ChopExerciseName::WeightedHalfKneelingRotation => {
                writeln!(f, "weighted_half_kneeling_rotation")
            }
            ChopExerciseName::HalfKneelingRotationalChop => {
                writeln!(f, "half_kneeling_rotational_chop")
            }
            ChopExerciseName::HalfKneelingRotationalReverseChop => {
                writeln!(f, "half_kneeling_rotational_reverse_chop")
            }
            ChopExerciseName::HalfKneelingStabilityChop => {
                writeln!(f, "half_kneeling_stability_chop")
            }
            ChopExerciseName::HalfKneelingStabilityReverseChop => {
                writeln!(f, "half_kneeling_stability_reverse_chop")
            }
            ChopExerciseName::KneelingRotationalChop => writeln!(f, "kneeling_rotational_chop"),
            ChopExerciseName::KneelingRotationalReverseChop => {
                writeln!(f, "kneeling_rotational_reverse_chop")
            }
            ChopExerciseName::KneelingStabilityChop => writeln!(f, "kneeling_stability_chop"),
            ChopExerciseName::KneelingWoodchopper => writeln!(f, "kneeling_woodchopper"),
            ChopExerciseName::MedicineBallWoodChops => writeln!(f, "medicine_ball_wood_chops"),
            ChopExerciseName::PowerSquatChops => writeln!(f, "power_squat_chops"),
            ChopExerciseName::WeightedPowerSquatChops => writeln!(f, "weighted_power_squat_chops"),
            ChopExerciseName::StandingRotationalChop => writeln!(f, "standing_rotational_chop"),
            ChopExerciseName::StandingSplitRotationalChop => {
                writeln!(f, "standing_split_rotational_chop")
            }
            ChopExerciseName::StandingSplitRotationalReverseChop => {
                writeln!(f, "standing_split_rotational_reverse_chop")
            }
            ChopExerciseName::StandingStabilityReverseChop => {
                writeln!(f, "standing_stability_reverse_chop")
            }
            ChopExerciseName::UnknownVariant(value) => writeln!(f, "unknown_variant_{}", *value),
        }
    }
}
impl convert::From<u16> for ChopExerciseName {
    fn from(value: u16) -> Self {
        match value {
            0 => ChopExerciseName::CablePullThrough,
            1 => ChopExerciseName::CableRotationalLift,
            2 => ChopExerciseName::CableWoodchop,
            3 => ChopExerciseName::CrossChopToKnee,
            4 => ChopExerciseName::WeightedCrossChopToKnee,
            5 => ChopExerciseName::DumbbellChop,
            6 => ChopExerciseName::HalfKneelingRotation,
            7 => ChopExerciseName::WeightedHalfKneelingRotation,
            8 => ChopExerciseName::HalfKneelingRotationalChop,
            9 => ChopExerciseName::HalfKneelingRotationalReverseChop,
            10 => ChopExerciseName::HalfKneelingStabilityChop,
            11 => ChopExerciseName::HalfKneelingStabilityReverseChop,
            12 => ChopExerciseName::KneelingRotationalChop,
            13 => ChopExerciseName::KneelingRotationalReverseChop,
            14 => ChopExerciseName::KneelingStabilityChop,
            15 => ChopExerciseName::KneelingWoodchopper,
            16 => ChopExerciseName::MedicineBallWoodChops,
            17 => ChopExerciseName::PowerSquatChops,
            18 => ChopExerciseName::WeightedPowerSquatChops,
            19 => ChopExerciseName::StandingRotationalChop,
            20 => ChopExerciseName::StandingSplitRotationalChop,
            21 => ChopExerciseName::StandingSplitRotationalReverseChop,
            22 => ChopExerciseName::StandingStabilityReverseChop,
            _ => ChopExerciseName::UnknownVariant(value),
        }
    }
}
impl convert::From<i64> for ChopExerciseName {
    fn from(value: i64) -> Self {
        ChopExerciseName::from(value as u16)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum CoreExerciseName {
    AbsJabs,
    WeightedAbsJabs,
    AlternatingPlateReach,
    BarbellRollout,
    WeightedBarbellRollout,
    BodyBarObliqueTwist,
    CableCorePress,
    CableSideBend,
    SideBend,
    WeightedSideBend,
    CrescentCircle,
    WeightedCrescentCircle,
    CyclingRussianTwist,
    WeightedCyclingRussianTwist,
    ElevatedFeetRussianTwist,
    WeightedElevatedFeetRussianTwist,
    HalfTurkishGetUp,
    KettlebellWindmill,
    KneelingAbWheel,
    WeightedKneelingAbWheel,
    ModifiedFrontLever,
    OpenKneeTucks,
    WeightedOpenKneeTucks,
    SideAbsLegLift,
    WeightedSideAbsLegLift,
    SwissBallJackknife,
    WeightedSwissBallJackknife,
    SwissBallPike,
    WeightedSwissBallPike,
    SwissBallRollout,
    WeightedSwissBallRollout,
    TriangleHipPress,
    WeightedTriangleHipPress,
    TrxSuspendedJackknife,
    WeightedTrxSuspendedJackknife,
    UBoat,
    WeightedUBoat,
    WindmillSwitches,
    WeightedWindmillSwitches,
    AlternatingSlideOut,
    WeightedAlternatingSlideOut,
    GhdBackExtensions,
    WeightedGhdBackExtensions,
    OverheadWalk,
    Inchworm,
    WeightedModifiedFrontLever,
    RussianTwist,
    AbdominalLegRotations,
    ArmAndLegExtensionOnKnees,
    Bicycle,
    BicepCurlWithLegExtension,
    CatCow,
    Corkscrew,
    CrissCross,
    CrissCrossWithBall,
    DoubleLegStretch,
    KneeFolds,
    LowerLift,
    NeckPull,
    PelvicClocks,
    RollOver,
    RollUp,
    Rolling,
    Rowing1,
    Rowing2,
    Scissors,
    SingleLegCircles,
    SingleLegStretch,
    SnakeTwist1And2,
    Swan,
    Swimming,
    Teaser,
    TheHundred,
    UnknownVariant(u16),
}
impl CoreExerciseName {
    pub fn as_u16(self) -> u16 {
        match self {
            CoreExerciseName::AbsJabs => 0,
            CoreExerciseName::WeightedAbsJabs => 1,
            CoreExerciseName::AlternatingPlateReach => 2,
            CoreExerciseName::BarbellRollout => 3,
            CoreExerciseName::WeightedBarbellRollout => 4,
            CoreExerciseName::BodyBarObliqueTwist => 5,
            CoreExerciseName::CableCorePress => 6,
            CoreExerciseName::CableSideBend => 7,
            CoreExerciseName::SideBend => 8,
            CoreExerciseName::WeightedSideBend => 9,
            CoreExerciseName::CrescentCircle => 10,
            CoreExerciseName::WeightedCrescentCircle => 11,
            CoreExerciseName::CyclingRussianTwist => 12,
            CoreExerciseName::WeightedCyclingRussianTwist => 13,
            CoreExerciseName::ElevatedFeetRussianTwist => 14,
            CoreExerciseName::WeightedElevatedFeetRussianTwist => 15,
            CoreExerciseName::HalfTurkishGetUp => 16,
            CoreExerciseName::KettlebellWindmill => 17,
            CoreExerciseName::KneelingAbWheel => 18,
            CoreExerciseName::WeightedKneelingAbWheel => 19,
            CoreExerciseName::ModifiedFrontLever => 20,
            CoreExerciseName::OpenKneeTucks => 21,
            CoreExerciseName::WeightedOpenKneeTucks => 22,
            CoreExerciseName::SideAbsLegLift => 23,
            CoreExerciseName::WeightedSideAbsLegLift => 24,
            CoreExerciseName::SwissBallJackknife => 25,
            CoreExerciseName::WeightedSwissBallJackknife => 26,
            CoreExerciseName::SwissBallPike => 27,
            CoreExerciseName::WeightedSwissBallPike => 28,
            CoreExerciseName::SwissBallRollout => 29,
            CoreExerciseName::WeightedSwissBallRollout => 30,
            CoreExerciseName::TriangleHipPress => 31,
            CoreExerciseName::WeightedTriangleHipPress => 32,
            CoreExerciseName::TrxSuspendedJackknife => 33,
            CoreExerciseName::WeightedTrxSuspendedJackknife => 34,
            CoreExerciseName::UBoat => 35,
            CoreExerciseName::WeightedUBoat => 36,
            CoreExerciseName::WindmillSwitches => 37,
            CoreExerciseName::WeightedWindmillSwitches => 38,
            CoreExerciseName::AlternatingSlideOut => 39,
            CoreExerciseName::WeightedAlternatingSlideOut => 40,
            CoreExerciseName::GhdBackExtensions => 41,
            CoreExerciseName::WeightedGhdBackExtensions => 42,
            CoreExerciseName::OverheadWalk => 43,
            CoreExerciseName::Inchworm => 44,
            CoreExerciseName::WeightedModifiedFrontLever => 45,
            CoreExerciseName::RussianTwist => 46,
            CoreExerciseName::AbdominalLegRotations => 47,
            CoreExerciseName::ArmAndLegExtensionOnKnees => 48,
            CoreExerciseName::Bicycle => 49,
            CoreExerciseName::BicepCurlWithLegExtension => 50,
            CoreExerciseName::CatCow => 51,
            CoreExerciseName::Corkscrew => 52,
            CoreExerciseName::CrissCross => 53,
            CoreExerciseName::CrissCrossWithBall => 54,
            CoreExerciseName::DoubleLegStretch => 55,
            CoreExerciseName::KneeFolds => 56,
            CoreExerciseName::LowerLift => 57,
            CoreExerciseName::NeckPull => 58,
            CoreExerciseName::PelvicClocks => 59,
            CoreExerciseName::RollOver => 60,
            CoreExerciseName::RollUp => 61,
            CoreExerciseName::Rolling => 62,
            CoreExerciseName::Rowing1 => 63,
            CoreExerciseName::Rowing2 => 64,
            CoreExerciseName::Scissors => 65,
            CoreExerciseName::SingleLegCircles => 66,
            CoreExerciseName::SingleLegStretch => 67,
            CoreExerciseName::SnakeTwist1And2 => 68,
            CoreExerciseName::Swan => 69,
            CoreExerciseName::Swimming => 70,
            CoreExerciseName::Teaser => 71,
            CoreExerciseName::TheHundred => 72,
            CoreExerciseName::UnknownVariant(value) => value,
        }
    }
    pub fn as_i64(self) -> i64 {
        self.as_u16() as i64
    }
}
impl fmt::Display for CoreExerciseName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            CoreExerciseName::AbsJabs => writeln!(f, "abs_jabs"),
            CoreExerciseName::WeightedAbsJabs => writeln!(f, "weighted_abs_jabs"),
            CoreExerciseName::AlternatingPlateReach => writeln!(f, "alternating_plate_reach"),
            CoreExerciseName::BarbellRollout => writeln!(f, "barbell_rollout"),
            CoreExerciseName::WeightedBarbellRollout => writeln!(f, "weighted_barbell_rollout"),
            CoreExerciseName::BodyBarObliqueTwist => writeln!(f, "body_bar_oblique_twist"),
            CoreExerciseName::CableCorePress => writeln!(f, "cable_core_press"),
            CoreExerciseName::CableSideBend => writeln!(f, "cable_side_bend"),
            CoreExerciseName::SideBend => writeln!(f, "side_bend"),
            CoreExerciseName::WeightedSideBend => writeln!(f, "weighted_side_bend"),
            CoreExerciseName::CrescentCircle => writeln!(f, "crescent_circle"),
            CoreExerciseName::WeightedCrescentCircle => writeln!(f, "weighted_crescent_circle"),
            CoreExerciseName::CyclingRussianTwist => writeln!(f, "cycling_russian_twist"),
            CoreExerciseName::WeightedCyclingRussianTwist => {
                writeln!(f, "weighted_cycling_russian_twist")
            }
            CoreExerciseName::ElevatedFeetRussianTwist => {
                writeln!(f, "elevated_feet_russian_twist")
            }
            CoreExerciseName::WeightedElevatedFeetRussianTwist => {
                writeln!(f, "weighted_elevated_feet_russian_twist")
            }
            CoreExerciseName::HalfTurkishGetUp => writeln!(f, "half_turkish_get_up"),
            CoreExerciseName::KettlebellWindmill => writeln!(f, "kettlebell_windmill"),
            CoreExerciseName::KneelingAbWheel => writeln!(f, "kneeling_ab_wheel"),
            CoreExerciseName::WeightedKneelingAbWheel => writeln!(f, "weighted_kneeling_ab_wheel"),
            CoreExerciseName::ModifiedFrontLever => writeln!(f, "modified_front_lever"),
            CoreExerciseName::OpenKneeTucks => writeln!(f, "open_knee_tucks"),
            CoreExerciseName::WeightedOpenKneeTucks => writeln!(f, "weighted_open_knee_tucks"),
            CoreExerciseName::SideAbsLegLift => writeln!(f, "side_abs_leg_lift"),
            CoreExerciseName::WeightedSideAbsLegLift => writeln!(f, "weighted_side_abs_leg_lift"),
            CoreExerciseName::SwissBallJackknife => writeln!(f, "swiss_ball_jackknife"),
            CoreExerciseName::WeightedSwissBallJackknife => {
                writeln!(f, "weighted_swiss_ball_jackknife")
            }
            CoreExerciseName::SwissBallPike => writeln!(f, "swiss_ball_pike"),
            CoreExerciseName::WeightedSwissBallPike => writeln!(f, "weighted_swiss_ball_pike"),
            CoreExerciseName::SwissBallRollout => writeln!(f, "swiss_ball_rollout"),
            CoreExerciseName::WeightedSwissBallRollout => {
                writeln!(f, "weighted_swiss_ball_rollout")
            }
            CoreExerciseName::TriangleHipPress => writeln!(f, "triangle_hip_press"),
            CoreExerciseName::WeightedTriangleHipPress => {
                writeln!(f, "weighted_triangle_hip_press")
            }
            CoreExerciseName::TrxSuspendedJackknife => writeln!(f, "trx_suspended_jackknife"),
            CoreExerciseName::WeightedTrxSuspendedJackknife => {
                writeln!(f, "weighted_trx_suspended_jackknife")
            }
            CoreExerciseName::UBoat => writeln!(f, "u_boat"),
            CoreExerciseName::WeightedUBoat => writeln!(f, "weighted_u_boat"),
            CoreExerciseName::WindmillSwitches => writeln!(f, "windmill_switches"),
            CoreExerciseName::WeightedWindmillSwitches => writeln!(f, "weighted_windmill_switches"),
            CoreExerciseName::AlternatingSlideOut => writeln!(f, "alternating_slide_out"),
            CoreExerciseName::WeightedAlternatingSlideOut => {
                writeln!(f, "weighted_alternating_slide_out")
            }
            CoreExerciseName::GhdBackExtensions => writeln!(f, "ghd_back_extensions"),
            CoreExerciseName::WeightedGhdBackExtensions => {
                writeln!(f, "weighted_ghd_back_extensions")
            }
            CoreExerciseName::OverheadWalk => writeln!(f, "overhead_walk"),
            CoreExerciseName::Inchworm => writeln!(f, "inchworm"),
            CoreExerciseName::WeightedModifiedFrontLever => {
                writeln!(f, "weighted_modified_front_lever")
            }
            CoreExerciseName::RussianTwist => writeln!(f, "russian_twist"),
            CoreExerciseName::AbdominalLegRotations => writeln!(f, "abdominal_leg_rotations"),
            CoreExerciseName::ArmAndLegExtensionOnKnees => {
                writeln!(f, "arm_and_leg_extension_on_knees")
            }
            CoreExerciseName::Bicycle => writeln!(f, "bicycle"),
            CoreExerciseName::BicepCurlWithLegExtension => {
                writeln!(f, "bicep_curl_with_leg_extension")
            }
            CoreExerciseName::CatCow => writeln!(f, "cat_cow"),
            CoreExerciseName::Corkscrew => writeln!(f, "corkscrew"),
            CoreExerciseName::CrissCross => writeln!(f, "criss_cross"),
            CoreExerciseName::CrissCrossWithBall => writeln!(f, "criss_cross_with_ball"),
            CoreExerciseName::DoubleLegStretch => writeln!(f, "double_leg_stretch"),
            CoreExerciseName::KneeFolds => writeln!(f, "knee_folds"),
            CoreExerciseName::LowerLift => writeln!(f, "lower_lift"),
            CoreExerciseName::NeckPull => writeln!(f, "neck_pull"),
            CoreExerciseName::PelvicClocks => writeln!(f, "pelvic_clocks"),
            CoreExerciseName::RollOver => writeln!(f, "roll_over"),
            CoreExerciseName::RollUp => writeln!(f, "roll_up"),
            CoreExerciseName::Rolling => writeln!(f, "rolling"),
            CoreExerciseName::Rowing1 => writeln!(f, "rowing_1"),
            CoreExerciseName::Rowing2 => writeln!(f, "rowing_2"),
            CoreExerciseName::Scissors => writeln!(f, "scissors"),
            CoreExerciseName::SingleLegCircles => writeln!(f, "single_leg_circles"),
            CoreExerciseName::SingleLegStretch => writeln!(f, "single_leg_stretch"),
            CoreExerciseName::SnakeTwist1And2 => writeln!(f, "snake_twist_1_and_2"),
            CoreExerciseName::Swan => writeln!(f, "swan"),
            CoreExerciseName::Swimming => writeln!(f, "swimming"),
            CoreExerciseName::Teaser => writeln!(f, "teaser"),
            CoreExerciseName::TheHundred => writeln!(f, "the_hundred"),
            CoreExerciseName::UnknownVariant(value) => writeln!(f, "unknown_variant_{}", *value),
        }
    }
}
impl convert::From<u16> for CoreExerciseName {
    fn from(value: u16) -> Self {
        match value {
            0 => CoreExerciseName::AbsJabs,
            1 => CoreExerciseName::WeightedAbsJabs,
            2 => CoreExerciseName::AlternatingPlateReach,
            3 => CoreExerciseName::BarbellRollout,
            4 => CoreExerciseName::WeightedBarbellRollout,
            5 => CoreExerciseName::BodyBarObliqueTwist,
            6 => CoreExerciseName::CableCorePress,
            7 => CoreExerciseName::CableSideBend,
            8 => CoreExerciseName::SideBend,
            9 => CoreExerciseName::WeightedSideBend,
            10 => CoreExerciseName::CrescentCircle,
            11 => CoreExerciseName::WeightedCrescentCircle,
            12 => CoreExerciseName::CyclingRussianTwist,
            13 => CoreExerciseName::WeightedCyclingRussianTwist,
            14 => CoreExerciseName::ElevatedFeetRussianTwist,
            15 => CoreExerciseName::WeightedElevatedFeetRussianTwist,
            16 => CoreExerciseName::HalfTurkishGetUp,
            17 => CoreExerciseName::KettlebellWindmill,
            18 => CoreExerciseName::KneelingAbWheel,
            19 => CoreExerciseName::WeightedKneelingAbWheel,
            20 => CoreExerciseName::ModifiedFrontLever,
            21 => CoreExerciseName::OpenKneeTucks,
            22 => CoreExerciseName::WeightedOpenKneeTucks,
            23 => CoreExerciseName::SideAbsLegLift,
            24 => CoreExerciseName::WeightedSideAbsLegLift,
            25 => CoreExerciseName::SwissBallJackknife,
            26 => CoreExerciseName::WeightedSwissBallJackknife,
            27 => CoreExerciseName::SwissBallPike,
            28 => CoreExerciseName::WeightedSwissBallPike,
            29 => CoreExerciseName::SwissBallRollout,
            30 => CoreExerciseName::WeightedSwissBallRollout,
            31 => CoreExerciseName::TriangleHipPress,
            32 => CoreExerciseName::WeightedTriangleHipPress,
            33 => CoreExerciseName::TrxSuspendedJackknife,
            34 => CoreExerciseName::WeightedTrxSuspendedJackknife,
            35 => CoreExerciseName::UBoat,
            36 => CoreExerciseName::WeightedUBoat,
            37 => CoreExerciseName::WindmillSwitches,
            38 => CoreExerciseName::WeightedWindmillSwitches,
            39 => CoreExerciseName::AlternatingSlideOut,
            40 => CoreExerciseName::WeightedAlternatingSlideOut,
            41 => CoreExerciseName::GhdBackExtensions,
            42 => CoreExerciseName::WeightedGhdBackExtensions,
            43 => CoreExerciseName::OverheadWalk,
            44 => CoreExerciseName::Inchworm,
            45 => CoreExerciseName::WeightedModifiedFrontLever,
            46 => CoreExerciseName::RussianTwist,
            47 => CoreExerciseName::AbdominalLegRotations,
            48 => CoreExerciseName::ArmAndLegExtensionOnKnees,
            49 => CoreExerciseName::Bicycle,
            50 => CoreExerciseName::BicepCurlWithLegExtension,
            51 => CoreExerciseName::CatCow,
            52 => CoreExerciseName::Corkscrew,
            53 => CoreExerciseName::CrissCross,
            54 => CoreExerciseName::CrissCrossWithBall,
            55 => CoreExerciseName::DoubleLegStretch,
            56 => CoreExerciseName::KneeFolds,
            57 => CoreExerciseName::LowerLift,
            58 => CoreExerciseName::NeckPull,
            59 => CoreExerciseName::PelvicClocks,
            60 => CoreExerciseName::RollOver,
            61 => CoreExerciseName::RollUp,
            62 => CoreExerciseName::Rolling,
            63 => CoreExerciseName::Rowing1,
            64 => CoreExerciseName::Rowing2,
            65 => CoreExerciseName::Scissors,
            66 => CoreExerciseName::SingleLegCircles,
            67 => CoreExerciseName::SingleLegStretch,
            68 => CoreExerciseName::SnakeTwist1And2,
            69 => CoreExerciseName::Swan,
            70 => CoreExerciseName::Swimming,
            71 => CoreExerciseName::Teaser,
            72 => CoreExerciseName::TheHundred,
            _ => CoreExerciseName::UnknownVariant(value),
        }
    }
}
impl convert::From<i64> for CoreExerciseName {
    fn from(value: i64) -> Self {
        CoreExerciseName::from(value as u16)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum CrunchExerciseName {
    BicycleCrunch,
    CableCrunch,
    CircularArmCrunch,
    CrossedArmsCrunch,
    WeightedCrossedArmsCrunch,
    CrossLegReverseCrunch,
    WeightedCrossLegReverseCrunch,
    CrunchChop,
    WeightedCrunchChop,
    DoubleCrunch,
    WeightedDoubleCrunch,
    ElbowToKneeCrunch,
    WeightedElbowToKneeCrunch,
    FlutterKicks,
    WeightedFlutterKicks,
    FoamRollerReverseCrunchOnBench,
    WeightedFoamRollerReverseCrunchOnBench,
    FoamRollerReverseCrunchWithDumbbell,
    FoamRollerReverseCrunchWithMedicineBall,
    FrogPress,
    HangingKneeRaiseObliqueCrunch,
    WeightedHangingKneeRaiseObliqueCrunch,
    HipCrossover,
    WeightedHipCrossover,
    HollowRock,
    WeightedHollowRock,
    InclineReverseCrunch,
    WeightedInclineReverseCrunch,
    KneelingCableCrunch,
    KneelingCrossCrunch,
    WeightedKneelingCrossCrunch,
    KneelingObliqueCableCrunch,
    KneesToElbow,
    LegExtensions,
    WeightedLegExtensions,
    LegLevers,
    McgillCurlUp,
    WeightedMcgillCurlUp,
    ModifiedPilatesRollUpWithBall,
    WeightedModifiedPilatesRollUpWithBall,
    PilatesCrunch,
    WeightedPilatesCrunch,
    PilatesRollUpWithBall,
    WeightedPilatesRollUpWithBall,
    RaisedLegsCrunch,
    WeightedRaisedLegsCrunch,
    ReverseCrunch,
    WeightedReverseCrunch,
    ReverseCrunchOnABench,
    WeightedReverseCrunchOnABench,
    ReverseCurlAndLift,
    WeightedReverseCurlAndLift,
    RotationalLift,
    WeightedRotationalLift,
    SeatedAlternatingReverseCrunch,
    WeightedSeatedAlternatingReverseCrunch,
    SeatedLegU,
    WeightedSeatedLegU,
    SideToSideCrunchAndWeave,
    WeightedSideToSideCrunchAndWeave,
    SingleLegReverseCrunch,
    WeightedSingleLegReverseCrunch,
    SkaterCrunchCross,
    WeightedSkaterCrunchCross,
    StandingCableCrunch,
    StandingSideCrunch,
    StepClimb,
    WeightedStepClimb,
    SwissBallCrunch,
    SwissBallReverseCrunch,
    WeightedSwissBallReverseCrunch,
    SwissBallRussianTwist,
    WeightedSwissBallRussianTwist,
    SwissBallSideCrunch,
    WeightedSwissBallSideCrunch,
    ThoracicCrunchesOnFoamRoller,
    WeightedThoracicCrunchesOnFoamRoller,
    TricepsCrunch,
    WeightedBicycleCrunch,
    WeightedCrunch,
    WeightedSwissBallCrunch,
    ToesToBar,
    WeightedToesToBar,
    Crunch,
    StraightLegCrunchWithBall,
    UnknownVariant(u16),
}
impl CrunchExerciseName {
    pub fn as_u16(self) -> u16 {
        match self {
            CrunchExerciseName::BicycleCrunch => 0,
            CrunchExerciseName::CableCrunch => 1,
            CrunchExerciseName::CircularArmCrunch => 2,
            CrunchExerciseName::CrossedArmsCrunch => 3,
            CrunchExerciseName::WeightedCrossedArmsCrunch => 4,
            CrunchExerciseName::CrossLegReverseCrunch => 5,
            CrunchExerciseName::WeightedCrossLegReverseCrunch => 6,
            CrunchExerciseName::CrunchChop => 7,
            CrunchExerciseName::WeightedCrunchChop => 8,
            CrunchExerciseName::DoubleCrunch => 9,
            CrunchExerciseName::WeightedDoubleCrunch => 10,
            CrunchExerciseName::ElbowToKneeCrunch => 11,
            CrunchExerciseName::WeightedElbowToKneeCrunch => 12,
            CrunchExerciseName::FlutterKicks => 13,
            CrunchExerciseName::WeightedFlutterKicks => 14,
            CrunchExerciseName::FoamRollerReverseCrunchOnBench => 15,
            CrunchExerciseName::WeightedFoamRollerReverseCrunchOnBench => 16,
            CrunchExerciseName::FoamRollerReverseCrunchWithDumbbell => 17,
            CrunchExerciseName::FoamRollerReverseCrunchWithMedicineBall => 18,
            CrunchExerciseName::FrogPress => 19,
            CrunchExerciseName::HangingKneeRaiseObliqueCrunch => 20,
            CrunchExerciseName::WeightedHangingKneeRaiseObliqueCrunch => 21,
            CrunchExerciseName::HipCrossover => 22,
            CrunchExerciseName::WeightedHipCrossover => 23,
            CrunchExerciseName::HollowRock => 24,
            CrunchExerciseName::WeightedHollowRock => 25,
            CrunchExerciseName::InclineReverseCrunch => 26,
            CrunchExerciseName::WeightedInclineReverseCrunch => 27,
            CrunchExerciseName::KneelingCableCrunch => 28,
            CrunchExerciseName::KneelingCrossCrunch => 29,
            CrunchExerciseName::WeightedKneelingCrossCrunch => 30,
            CrunchExerciseName::KneelingObliqueCableCrunch => 31,
            CrunchExerciseName::KneesToElbow => 32,
            CrunchExerciseName::LegExtensions => 33,
            CrunchExerciseName::WeightedLegExtensions => 34,
            CrunchExerciseName::LegLevers => 35,
            CrunchExerciseName::McgillCurlUp => 36,
            CrunchExerciseName::WeightedMcgillCurlUp => 37,
            CrunchExerciseName::ModifiedPilatesRollUpWithBall => 38,
            CrunchExerciseName::WeightedModifiedPilatesRollUpWithBall => 39,
            CrunchExerciseName::PilatesCrunch => 40,
            CrunchExerciseName::WeightedPilatesCrunch => 41,
            CrunchExerciseName::PilatesRollUpWithBall => 42,
            CrunchExerciseName::WeightedPilatesRollUpWithBall => 43,
            CrunchExerciseName::RaisedLegsCrunch => 44,
            CrunchExerciseName::WeightedRaisedLegsCrunch => 45,
            CrunchExerciseName::ReverseCrunch => 46,
            CrunchExerciseName::WeightedReverseCrunch => 47,
            CrunchExerciseName::ReverseCrunchOnABench => 48,
            CrunchExerciseName::WeightedReverseCrunchOnABench => 49,
            CrunchExerciseName::ReverseCurlAndLift => 50,
            CrunchExerciseName::WeightedReverseCurlAndLift => 51,
            CrunchExerciseName::RotationalLift => 52,
            CrunchExerciseName::WeightedRotationalLift => 53,
            CrunchExerciseName::SeatedAlternatingReverseCrunch => 54,
            CrunchExerciseName::WeightedSeatedAlternatingReverseCrunch => 55,
            CrunchExerciseName::SeatedLegU => 56,
            CrunchExerciseName::WeightedSeatedLegU => 57,
            CrunchExerciseName::SideToSideCrunchAndWeave => 58,
            CrunchExerciseName::WeightedSideToSideCrunchAndWeave => 59,
            CrunchExerciseName::SingleLegReverseCrunch => 60,
            CrunchExerciseName::WeightedSingleLegReverseCrunch => 61,
            CrunchExerciseName::SkaterCrunchCross => 62,
            CrunchExerciseName::WeightedSkaterCrunchCross => 63,
            CrunchExerciseName::StandingCableCrunch => 64,
            CrunchExerciseName::StandingSideCrunch => 65,
            CrunchExerciseName::StepClimb => 66,
            CrunchExerciseName::WeightedStepClimb => 67,
            CrunchExerciseName::SwissBallCrunch => 68,
            CrunchExerciseName::SwissBallReverseCrunch => 69,
            CrunchExerciseName::WeightedSwissBallReverseCrunch => 70,
            CrunchExerciseName::SwissBallRussianTwist => 71,
            CrunchExerciseName::WeightedSwissBallRussianTwist => 72,
            CrunchExerciseName::SwissBallSideCrunch => 73,
            CrunchExerciseName::WeightedSwissBallSideCrunch => 74,
            CrunchExerciseName::ThoracicCrunchesOnFoamRoller => 75,
            CrunchExerciseName::WeightedThoracicCrunchesOnFoamRoller => 76,
            CrunchExerciseName::TricepsCrunch => 77,
            CrunchExerciseName::WeightedBicycleCrunch => 78,
            CrunchExerciseName::WeightedCrunch => 79,
            CrunchExerciseName::WeightedSwissBallCrunch => 80,
            CrunchExerciseName::ToesToBar => 81,
            CrunchExerciseName::WeightedToesToBar => 82,
            CrunchExerciseName::Crunch => 83,
            CrunchExerciseName::StraightLegCrunchWithBall => 84,
            CrunchExerciseName::UnknownVariant(value) => value,
        }
    }
    pub fn as_i64(self) -> i64 {
        self.as_u16() as i64
    }
}
impl fmt::Display for CrunchExerciseName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            CrunchExerciseName::BicycleCrunch => writeln!(f, "bicycle_crunch"),
            CrunchExerciseName::CableCrunch => writeln!(f, "cable_crunch"),
            CrunchExerciseName::CircularArmCrunch => writeln!(f, "circular_arm_crunch"),
            CrunchExerciseName::CrossedArmsCrunch => writeln!(f, "crossed_arms_crunch"),
            CrunchExerciseName::WeightedCrossedArmsCrunch => {
                writeln!(f, "weighted_crossed_arms_crunch")
            }
            CrunchExerciseName::CrossLegReverseCrunch => writeln!(f, "cross_leg_reverse_crunch"),
            CrunchExerciseName::WeightedCrossLegReverseCrunch => {
                writeln!(f, "weighted_cross_leg_reverse_crunch")
            }
            CrunchExerciseName::CrunchChop => writeln!(f, "crunch_chop"),
            CrunchExerciseName::WeightedCrunchChop => writeln!(f, "weighted_crunch_chop"),
            CrunchExerciseName::DoubleCrunch => writeln!(f, "double_crunch"),
            CrunchExerciseName::WeightedDoubleCrunch => writeln!(f, "weighted_double_crunch"),
            CrunchExerciseName::ElbowToKneeCrunch => writeln!(f, "elbow_to_knee_crunch"),
            CrunchExerciseName::WeightedElbowToKneeCrunch => {
                writeln!(f, "weighted_elbow_to_knee_crunch")
            }
            CrunchExerciseName::FlutterKicks => writeln!(f, "flutter_kicks"),
            CrunchExerciseName::WeightedFlutterKicks => writeln!(f, "weighted_flutter_kicks"),
            CrunchExerciseName::FoamRollerReverseCrunchOnBench => {
                writeln!(f, "foam_roller_reverse_crunch_on_bench")
            }
            CrunchExerciseName::WeightedFoamRollerReverseCrunchOnBench => {
                writeln!(f, "weighted_foam_roller_reverse_crunch_on_bench")
            }
            CrunchExerciseName::FoamRollerReverseCrunchWithDumbbell => {
                writeln!(f, "foam_roller_reverse_crunch_with_dumbbell")
            }
            CrunchExerciseName::FoamRollerReverseCrunchWithMedicineBall => {
                writeln!(f, "foam_roller_reverse_crunch_with_medicine_ball")
            }
            CrunchExerciseName::FrogPress => writeln!(f, "frog_press"),
            CrunchExerciseName::HangingKneeRaiseObliqueCrunch => {
                writeln!(f, "hanging_knee_raise_oblique_crunch")
            }
            CrunchExerciseName::WeightedHangingKneeRaiseObliqueCrunch => {
                writeln!(f, "weighted_hanging_knee_raise_oblique_crunch")
            }
            CrunchExerciseName::HipCrossover => writeln!(f, "hip_crossover"),
            CrunchExerciseName::WeightedHipCrossover => writeln!(f, "weighted_hip_crossover"),
            CrunchExerciseName::HollowRock => writeln!(f, "hollow_rock"),
            CrunchExerciseName::WeightedHollowRock => writeln!(f, "weighted_hollow_rock"),
            CrunchExerciseName::InclineReverseCrunch => writeln!(f, "incline_reverse_crunch"),
            CrunchExerciseName::WeightedInclineReverseCrunch => {
                writeln!(f, "weighted_incline_reverse_crunch")
            }
            CrunchExerciseName::KneelingCableCrunch => writeln!(f, "kneeling_cable_crunch"),
            CrunchExerciseName::KneelingCrossCrunch => writeln!(f, "kneeling_cross_crunch"),
            CrunchExerciseName::WeightedKneelingCrossCrunch => {
                writeln!(f, "weighted_kneeling_cross_crunch")
            }
            CrunchExerciseName::KneelingObliqueCableCrunch => {
                writeln!(f, "kneeling_oblique_cable_crunch")
            }
            CrunchExerciseName::KneesToElbow => writeln!(f, "knees_to_elbow"),
            CrunchExerciseName::LegExtensions => writeln!(f, "leg_extensions"),
            CrunchExerciseName::WeightedLegExtensions => writeln!(f, "weighted_leg_extensions"),
            CrunchExerciseName::LegLevers => writeln!(f, "leg_levers"),
            CrunchExerciseName::McgillCurlUp => writeln!(f, "mcgill_curl_up"),
            CrunchExerciseName::WeightedMcgillCurlUp => writeln!(f, "weighted_mcgill_curl_up"),
            CrunchExerciseName::ModifiedPilatesRollUpWithBall => {
                writeln!(f, "modified_pilates_roll_up_with_ball")
            }
            CrunchExerciseName::WeightedModifiedPilatesRollUpWithBall => {
                writeln!(f, "weighted_modified_pilates_roll_up_with_ball")
            }
            CrunchExerciseName::PilatesCrunch => writeln!(f, "pilates_crunch"),
            CrunchExerciseName::WeightedPilatesCrunch => writeln!(f, "weighted_pilates_crunch"),
            CrunchExerciseName::PilatesRollUpWithBall => writeln!(f, "pilates_roll_up_with_ball"),
            CrunchExerciseName::WeightedPilatesRollUpWithBall => {
                writeln!(f, "weighted_pilates_roll_up_with_ball")
            }
            CrunchExerciseName::RaisedLegsCrunch => writeln!(f, "raised_legs_crunch"),
            CrunchExerciseName::WeightedRaisedLegsCrunch => {
                writeln!(f, "weighted_raised_legs_crunch")
            }
            CrunchExerciseName::ReverseCrunch => writeln!(f, "reverse_crunch"),
            CrunchExerciseName::WeightedReverseCrunch => writeln!(f, "weighted_reverse_crunch"),
            CrunchExerciseName::ReverseCrunchOnABench => writeln!(f, "reverse_crunch_on_a_bench"),
            CrunchExerciseName::WeightedReverseCrunchOnABench => {
                writeln!(f, "weighted_reverse_crunch_on_a_bench")
            }
            CrunchExerciseName::ReverseCurlAndLift => writeln!(f, "reverse_curl_and_lift"),
            CrunchExerciseName::WeightedReverseCurlAndLift => {
                writeln!(f, "weighted_reverse_curl_and_lift")
            }
            CrunchExerciseName::RotationalLift => writeln!(f, "rotational_lift"),
            CrunchExerciseName::WeightedRotationalLift => writeln!(f, "weighted_rotational_lift"),
            CrunchExerciseName::SeatedAlternatingReverseCrunch => {
                writeln!(f, "seated_alternating_reverse_crunch")
            }
            CrunchExerciseName::WeightedSeatedAlternatingReverseCrunch => {
                writeln!(f, "weighted_seated_alternating_reverse_crunch")
            }
            CrunchExerciseName::SeatedLegU => writeln!(f, "seated_leg_u"),
            CrunchExerciseName::WeightedSeatedLegU => writeln!(f, "weighted_seated_leg_u"),
            CrunchExerciseName::SideToSideCrunchAndWeave => {
                writeln!(f, "side_to_side_crunch_and_weave")
            }
            CrunchExerciseName::WeightedSideToSideCrunchAndWeave => {
                writeln!(f, "weighted_side_to_side_crunch_and_weave")
            }
            CrunchExerciseName::SingleLegReverseCrunch => writeln!(f, "single_leg_reverse_crunch"),
            CrunchExerciseName::WeightedSingleLegReverseCrunch => {
                writeln!(f, "weighted_single_leg_reverse_crunch")
            }
            CrunchExerciseName::SkaterCrunchCross => writeln!(f, "skater_crunch_cross"),
            CrunchExerciseName::WeightedSkaterCrunchCross => {
                writeln!(f, "weighted_skater_crunch_cross")
            }
            CrunchExerciseName::StandingCableCrunch => writeln!(f, "standing_cable_crunch"),
            CrunchExerciseName::StandingSideCrunch => writeln!(f, "standing_side_crunch"),
            CrunchExerciseName::StepClimb => writeln!(f, "step_climb"),
            CrunchExerciseName::WeightedStepClimb => writeln!(f, "weighted_step_climb"),
            CrunchExerciseName::SwissBallCrunch => writeln!(f, "swiss_ball_crunch"),
            CrunchExerciseName::SwissBallReverseCrunch => writeln!(f, "swiss_ball_reverse_crunch"),
            CrunchExerciseName::WeightedSwissBallReverseCrunch => {
                writeln!(f, "weighted_swiss_ball_reverse_crunch")
            }
            CrunchExerciseName::SwissBallRussianTwist => writeln!(f, "swiss_ball_russian_twist"),
            CrunchExerciseName::WeightedSwissBallRussianTwist => {
                writeln!(f, "weighted_swiss_ball_russian_twist")
            }
            CrunchExerciseName::SwissBallSideCrunch => writeln!(f, "swiss_ball_side_crunch"),
            CrunchExerciseName::WeightedSwissBallSideCrunch => {
                writeln!(f, "weighted_swiss_ball_side_crunch")
            }
            CrunchExerciseName::ThoracicCrunchesOnFoamRoller => {
                writeln!(f, "thoracic_crunches_on_foam_roller")
            }
            CrunchExerciseName::WeightedThoracicCrunchesOnFoamRoller => {
                writeln!(f, "weighted_thoracic_crunches_on_foam_roller")
            }
            CrunchExerciseName::TricepsCrunch => writeln!(f, "triceps_crunch"),
            CrunchExerciseName::WeightedBicycleCrunch => writeln!(f, "weighted_bicycle_crunch"),
            CrunchExerciseName::WeightedCrunch => writeln!(f, "weighted_crunch"),
            CrunchExerciseName::WeightedSwissBallCrunch => {
                writeln!(f, "weighted_swiss_ball_crunch")
            }
            CrunchExerciseName::ToesToBar => writeln!(f, "toes_to_bar"),
            CrunchExerciseName::WeightedToesToBar => writeln!(f, "weighted_toes_to_bar"),
            CrunchExerciseName::Crunch => writeln!(f, "crunch"),
            CrunchExerciseName::StraightLegCrunchWithBall => {
                writeln!(f, "straight_leg_crunch_with_ball")
            }
            CrunchExerciseName::UnknownVariant(value) => writeln!(f, "unknown_variant_{}", *value),
        }
    }
}
impl convert::From<u16> for CrunchExerciseName {
    fn from(value: u16) -> Self {
        match value {
            0 => CrunchExerciseName::BicycleCrunch,
            1 => CrunchExerciseName::CableCrunch,
            2 => CrunchExerciseName::CircularArmCrunch,
            3 => CrunchExerciseName::CrossedArmsCrunch,
            4 => CrunchExerciseName::WeightedCrossedArmsCrunch,
            5 => CrunchExerciseName::CrossLegReverseCrunch,
            6 => CrunchExerciseName::WeightedCrossLegReverseCrunch,
            7 => CrunchExerciseName::CrunchChop,
            8 => CrunchExerciseName::WeightedCrunchChop,
            9 => CrunchExerciseName::DoubleCrunch,
            10 => CrunchExerciseName::WeightedDoubleCrunch,
            11 => CrunchExerciseName::ElbowToKneeCrunch,
            12 => CrunchExerciseName::WeightedElbowToKneeCrunch,
            13 => CrunchExerciseName::FlutterKicks,
            14 => CrunchExerciseName::WeightedFlutterKicks,
            15 => CrunchExerciseName::FoamRollerReverseCrunchOnBench,
            16 => CrunchExerciseName::WeightedFoamRollerReverseCrunchOnBench,
            17 => CrunchExerciseName::FoamRollerReverseCrunchWithDumbbell,
            18 => CrunchExerciseName::FoamRollerReverseCrunchWithMedicineBall,
            19 => CrunchExerciseName::FrogPress,
            20 => CrunchExerciseName::HangingKneeRaiseObliqueCrunch,
            21 => CrunchExerciseName::WeightedHangingKneeRaiseObliqueCrunch,
            22 => CrunchExerciseName::HipCrossover,
            23 => CrunchExerciseName::WeightedHipCrossover,
            24 => CrunchExerciseName::HollowRock,
            25 => CrunchExerciseName::WeightedHollowRock,
            26 => CrunchExerciseName::InclineReverseCrunch,
            27 => CrunchExerciseName::WeightedInclineReverseCrunch,
            28 => CrunchExerciseName::KneelingCableCrunch,
            29 => CrunchExerciseName::KneelingCrossCrunch,
            30 => CrunchExerciseName::WeightedKneelingCrossCrunch,
            31 => CrunchExerciseName::KneelingObliqueCableCrunch,
            32 => CrunchExerciseName::KneesToElbow,
            33 => CrunchExerciseName::LegExtensions,
            34 => CrunchExerciseName::WeightedLegExtensions,
            35 => CrunchExerciseName::LegLevers,
            36 => CrunchExerciseName::McgillCurlUp,
            37 => CrunchExerciseName::WeightedMcgillCurlUp,
            38 => CrunchExerciseName::ModifiedPilatesRollUpWithBall,
            39 => CrunchExerciseName::WeightedModifiedPilatesRollUpWithBall,
            40 => CrunchExerciseName::PilatesCrunch,
            41 => CrunchExerciseName::WeightedPilatesCrunch,
            42 => CrunchExerciseName::PilatesRollUpWithBall,
            43 => CrunchExerciseName::WeightedPilatesRollUpWithBall,
            44 => CrunchExerciseName::RaisedLegsCrunch,
            45 => CrunchExerciseName::WeightedRaisedLegsCrunch,
            46 => CrunchExerciseName::ReverseCrunch,
            47 => CrunchExerciseName::WeightedReverseCrunch,
            48 => CrunchExerciseName::ReverseCrunchOnABench,
            49 => CrunchExerciseName::WeightedReverseCrunchOnABench,
            50 => CrunchExerciseName::ReverseCurlAndLift,
            51 => CrunchExerciseName::WeightedReverseCurlAndLift,
            52 => CrunchExerciseName::RotationalLift,
            53 => CrunchExerciseName::WeightedRotationalLift,
            54 => CrunchExerciseName::SeatedAlternatingReverseCrunch,
            55 => CrunchExerciseName::WeightedSeatedAlternatingReverseCrunch,
            56 => CrunchExerciseName::SeatedLegU,
            57 => CrunchExerciseName::WeightedSeatedLegU,
            58 => CrunchExerciseName::SideToSideCrunchAndWeave,
            59 => CrunchExerciseName::WeightedSideToSideCrunchAndWeave,
            60 => CrunchExerciseName::SingleLegReverseCrunch,
            61 => CrunchExerciseName::WeightedSingleLegReverseCrunch,
            62 => CrunchExerciseName::SkaterCrunchCross,
            63 => CrunchExerciseName::WeightedSkaterCrunchCross,
            64 => CrunchExerciseName::StandingCableCrunch,
            65 => CrunchExerciseName::StandingSideCrunch,
            66 => CrunchExerciseName::StepClimb,
            67 => CrunchExerciseName::WeightedStepClimb,
            68 => CrunchExerciseName::SwissBallCrunch,
            69 => CrunchExerciseName::SwissBallReverseCrunch,
            70 => CrunchExerciseName::WeightedSwissBallReverseCrunch,
            71 => CrunchExerciseName::SwissBallRussianTwist,
            72 => CrunchExerciseName::WeightedSwissBallRussianTwist,
            73 => CrunchExerciseName::SwissBallSideCrunch,
            74 => CrunchExerciseName::WeightedSwissBallSideCrunch,
            75 => CrunchExerciseName::ThoracicCrunchesOnFoamRoller,
            76 => CrunchExerciseName::WeightedThoracicCrunchesOnFoamRoller,
            77 => CrunchExerciseName::TricepsCrunch,
            78 => CrunchExerciseName::WeightedBicycleCrunch,
            79 => CrunchExerciseName::WeightedCrunch,
            80 => CrunchExerciseName::WeightedSwissBallCrunch,
            81 => CrunchExerciseName::ToesToBar,
            82 => CrunchExerciseName::WeightedToesToBar,
            83 => CrunchExerciseName::Crunch,
            84 => CrunchExerciseName::StraightLegCrunchWithBall,
            _ => CrunchExerciseName::UnknownVariant(value),
        }
    }
}
impl convert::From<i64> for CrunchExerciseName {
    fn from(value: i64) -> Self {
        CrunchExerciseName::from(value as u16)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum CurlExerciseName {
    AlternatingDumbbellBicepsCurl,
    AlternatingDumbbellBicepsCurlOnSwissBall,
    AlternatingInclineDumbbellBicepsCurl,
    BarbellBicepsCurl,
    BarbellReverseWristCurl,
    BarbellWristCurl,
    BehindTheBackBarbellReverseWristCurl,
    BehindTheBackOneArmCableCurl,
    CableBicepsCurl,
    CableHammerCurl,
    CheatingBarbellBicepsCurl,
    CloseGripEzBarBicepsCurl,
    CrossBodyDumbbellHammerCurl,
    DeadHangBicepsCurl,
    DeclineHammerCurl,
    DumbbellBicepsCurlWithStaticHold,
    DumbbellHammerCurl,
    DumbbellReverseWristCurl,
    DumbbellWristCurl,
    EzBarPreacherCurl,
    ForwardBendBicepsCurl,
    HammerCurlToPress,
    InclineDumbbellBicepsCurl,
    InclineOffsetThumbDumbbellCurl,
    KettlebellBicepsCurl,
    LyingConcentrationCableCurl,
    OneArmPreacherCurl,
    PlatePinchCurl,
    PreacherCurlWithCable,
    ReverseEzBarCurl,
    ReverseGripWristCurl,
    ReverseGripBarbellBicepsCurl,
    SeatedAlternatingDumbbellBicepsCurl,
    SeatedDumbbellBicepsCurl,
    SeatedReverseDumbbellCurl,
    SplitStanceOffsetPinkyDumbbellCurl,
    StandingAlternatingDumbbellCurls,
    StandingDumbbellBicepsCurl,
    StandingEzBarBicepsCurl,
    StaticCurl,
    SwissBallDumbbellOverheadTricepsExtension,
    SwissBallEzBarPreacherCurl,
    TwistingStandingDumbbellBicepsCurl,
    WideGripEzBarBicepsCurl,
    UnknownVariant(u16),
}
impl CurlExerciseName {
    pub fn as_u16(self) -> u16 {
        match self {
            CurlExerciseName::AlternatingDumbbellBicepsCurl => 0,
            CurlExerciseName::AlternatingDumbbellBicepsCurlOnSwissBall => 1,
            CurlExerciseName::AlternatingInclineDumbbellBicepsCurl => 2,
            CurlExerciseName::BarbellBicepsCurl => 3,
            CurlExerciseName::BarbellReverseWristCurl => 4,
            CurlExerciseName::BarbellWristCurl => 5,
            CurlExerciseName::BehindTheBackBarbellReverseWristCurl => 6,
            CurlExerciseName::BehindTheBackOneArmCableCurl => 7,
            CurlExerciseName::CableBicepsCurl => 8,
            CurlExerciseName::CableHammerCurl => 9,
            CurlExerciseName::CheatingBarbellBicepsCurl => 10,
            CurlExerciseName::CloseGripEzBarBicepsCurl => 11,
            CurlExerciseName::CrossBodyDumbbellHammerCurl => 12,
            CurlExerciseName::DeadHangBicepsCurl => 13,
            CurlExerciseName::DeclineHammerCurl => 14,
            CurlExerciseName::DumbbellBicepsCurlWithStaticHold => 15,
            CurlExerciseName::DumbbellHammerCurl => 16,
            CurlExerciseName::DumbbellReverseWristCurl => 17,
            CurlExerciseName::DumbbellWristCurl => 18,
            CurlExerciseName::EzBarPreacherCurl => 19,
            CurlExerciseName::ForwardBendBicepsCurl => 20,
            CurlExerciseName::HammerCurlToPress => 21,
            CurlExerciseName::InclineDumbbellBicepsCurl => 22,
            CurlExerciseName::InclineOffsetThumbDumbbellCurl => 23,
            CurlExerciseName::KettlebellBicepsCurl => 24,
            CurlExerciseName::LyingConcentrationCableCurl => 25,
            CurlExerciseName::OneArmPreacherCurl => 26,
            CurlExerciseName::PlatePinchCurl => 27,
            CurlExerciseName::PreacherCurlWithCable => 28,
            CurlExerciseName::ReverseEzBarCurl => 29,
            CurlExerciseName::ReverseGripWristCurl => 30,
            CurlExerciseName::ReverseGripBarbellBicepsCurl => 31,
            CurlExerciseName::SeatedAlternatingDumbbellBicepsCurl => 32,
            CurlExerciseName::SeatedDumbbellBicepsCurl => 33,
            CurlExerciseName::SeatedReverseDumbbellCurl => 34,
            CurlExerciseName::SplitStanceOffsetPinkyDumbbellCurl => 35,
            CurlExerciseName::StandingAlternatingDumbbellCurls => 36,
            CurlExerciseName::StandingDumbbellBicepsCurl => 37,
            CurlExerciseName::StandingEzBarBicepsCurl => 38,
            CurlExerciseName::StaticCurl => 39,
            CurlExerciseName::SwissBallDumbbellOverheadTricepsExtension => 40,
            CurlExerciseName::SwissBallEzBarPreacherCurl => 41,
            CurlExerciseName::TwistingStandingDumbbellBicepsCurl => 42,
            CurlExerciseName::WideGripEzBarBicepsCurl => 43,
            CurlExerciseName::UnknownVariant(value) => value,
        }
    }
    pub fn as_i64(self) -> i64 {
        self.as_u16() as i64
    }
}
impl fmt::Display for CurlExerciseName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            CurlExerciseName::AlternatingDumbbellBicepsCurl => {
                writeln!(f, "alternating_dumbbell_biceps_curl")
            }
            CurlExerciseName::AlternatingDumbbellBicepsCurlOnSwissBall => {
                writeln!(f, "alternating_dumbbell_biceps_curl_on_swiss_ball")
            }
            CurlExerciseName::AlternatingInclineDumbbellBicepsCurl => {
                writeln!(f, "alternating_incline_dumbbell_biceps_curl")
            }
            CurlExerciseName::BarbellBicepsCurl => writeln!(f, "barbell_biceps_curl"),
            CurlExerciseName::BarbellReverseWristCurl => writeln!(f, "barbell_reverse_wrist_curl"),
            CurlExerciseName::BarbellWristCurl => writeln!(f, "barbell_wrist_curl"),
            CurlExerciseName::BehindTheBackBarbellReverseWristCurl => {
                writeln!(f, "behind_the_back_barbell_reverse_wrist_curl")
            }
            CurlExerciseName::BehindTheBackOneArmCableCurl => {
                writeln!(f, "behind_the_back_one_arm_cable_curl")
            }
            CurlExerciseName::CableBicepsCurl => writeln!(f, "cable_biceps_curl"),
            CurlExerciseName::CableHammerCurl => writeln!(f, "cable_hammer_curl"),
            CurlExerciseName::CheatingBarbellBicepsCurl => {
                writeln!(f, "cheating_barbell_biceps_curl")
            }
            CurlExerciseName::CloseGripEzBarBicepsCurl => {
                writeln!(f, "close_grip_ez_bar_biceps_curl")
            }
            CurlExerciseName::CrossBodyDumbbellHammerCurl => {
                writeln!(f, "cross_body_dumbbell_hammer_curl")
            }
            CurlExerciseName::DeadHangBicepsCurl => writeln!(f, "dead_hang_biceps_curl"),
            CurlExerciseName::DeclineHammerCurl => writeln!(f, "decline_hammer_curl"),
            CurlExerciseName::DumbbellBicepsCurlWithStaticHold => {
                writeln!(f, "dumbbell_biceps_curl_with_static_hold")
            }
            CurlExerciseName::DumbbellHammerCurl => writeln!(f, "dumbbell_hammer_curl"),
            CurlExerciseName::DumbbellReverseWristCurl => {
                writeln!(f, "dumbbell_reverse_wrist_curl")
            }
            CurlExerciseName::DumbbellWristCurl => writeln!(f, "dumbbell_wrist_curl"),
            CurlExerciseName::EzBarPreacherCurl => writeln!(f, "ez_bar_preacher_curl"),
            CurlExerciseName::ForwardBendBicepsCurl => writeln!(f, "forward_bend_biceps_curl"),
            CurlExerciseName::HammerCurlToPress => writeln!(f, "hammer_curl_to_press"),
            CurlExerciseName::InclineDumbbellBicepsCurl => {
                writeln!(f, "incline_dumbbell_biceps_curl")
            }
            CurlExerciseName::InclineOffsetThumbDumbbellCurl => {
                writeln!(f, "incline_offset_thumb_dumbbell_curl")
            }
            CurlExerciseName::KettlebellBicepsCurl => writeln!(f, "kettlebell_biceps_curl"),
            CurlExerciseName::LyingConcentrationCableCurl => {
                writeln!(f, "lying_concentration_cable_curl")
            }
            CurlExerciseName::OneArmPreacherCurl => writeln!(f, "one_arm_preacher_curl"),
            CurlExerciseName::PlatePinchCurl => writeln!(f, "plate_pinch_curl"),
            CurlExerciseName::PreacherCurlWithCable => writeln!(f, "preacher_curl_with_cable"),
            CurlExerciseName::ReverseEzBarCurl => writeln!(f, "reverse_ez_bar_curl"),
            CurlExerciseName::ReverseGripWristCurl => writeln!(f, "reverse_grip_wrist_curl"),
            CurlExerciseName::ReverseGripBarbellBicepsCurl => {
                writeln!(f, "reverse_grip_barbell_biceps_curl")
            }
            CurlExerciseName::SeatedAlternatingDumbbellBicepsCurl => {
                writeln!(f, "seated_alternating_dumbbell_biceps_curl")
            }
            CurlExerciseName::SeatedDumbbellBicepsCurl => {
                writeln!(f, "seated_dumbbell_biceps_curl")
            }
            CurlExerciseName::SeatedReverseDumbbellCurl => {
                writeln!(f, "seated_reverse_dumbbell_curl")
            }
            CurlExerciseName::SplitStanceOffsetPinkyDumbbellCurl => {
                writeln!(f, "split_stance_offset_pinky_dumbbell_curl")
            }
            CurlExerciseName::StandingAlternatingDumbbellCurls => {
                writeln!(f, "standing_alternating_dumbbell_curls")
            }
            CurlExerciseName::StandingDumbbellBicepsCurl => {
                writeln!(f, "standing_dumbbell_biceps_curl")
            }
            CurlExerciseName::StandingEzBarBicepsCurl => writeln!(f, "standing_ez_bar_biceps_curl"),
            CurlExerciseName::StaticCurl => writeln!(f, "static_curl"),
            CurlExerciseName::SwissBallDumbbellOverheadTricepsExtension => {
                writeln!(f, "swiss_ball_dumbbell_overhead_triceps_extension")
            }
            CurlExerciseName::SwissBallEzBarPreacherCurl => {
                writeln!(f, "swiss_ball_ez_bar_preacher_curl")
            }
            CurlExerciseName::TwistingStandingDumbbellBicepsCurl => {
                writeln!(f, "twisting_standing_dumbbell_biceps_curl")
            }
            CurlExerciseName::WideGripEzBarBicepsCurl => {
                writeln!(f, "wide_grip_ez_bar_biceps_curl")
            }
            CurlExerciseName::UnknownVariant(value) => writeln!(f, "unknown_variant_{}", *value),
        }
    }
}
impl convert::From<u16> for CurlExerciseName {
    fn from(value: u16) -> Self {
        match value {
            0 => CurlExerciseName::AlternatingDumbbellBicepsCurl,
            1 => CurlExerciseName::AlternatingDumbbellBicepsCurlOnSwissBall,
            2 => CurlExerciseName::AlternatingInclineDumbbellBicepsCurl,
            3 => CurlExerciseName::BarbellBicepsCurl,
            4 => CurlExerciseName::BarbellReverseWristCurl,
            5 => CurlExerciseName::BarbellWristCurl,
            6 => CurlExerciseName::BehindTheBackBarbellReverseWristCurl,
            7 => CurlExerciseName::BehindTheBackOneArmCableCurl,
            8 => CurlExerciseName::CableBicepsCurl,
            9 => CurlExerciseName::CableHammerCurl,
            10 => CurlExerciseName::CheatingBarbellBicepsCurl,
            11 => CurlExerciseName::CloseGripEzBarBicepsCurl,
            12 => CurlExerciseName::CrossBodyDumbbellHammerCurl,
            13 => CurlExerciseName::DeadHangBicepsCurl,
            14 => CurlExerciseName::DeclineHammerCurl,
            15 => CurlExerciseName::DumbbellBicepsCurlWithStaticHold,
            16 => CurlExerciseName::DumbbellHammerCurl,
            17 => CurlExerciseName::DumbbellReverseWristCurl,
            18 => CurlExerciseName::DumbbellWristCurl,
            19 => CurlExerciseName::EzBarPreacherCurl,
            20 => CurlExerciseName::ForwardBendBicepsCurl,
            21 => CurlExerciseName::HammerCurlToPress,
            22 => CurlExerciseName::InclineDumbbellBicepsCurl,
            23 => CurlExerciseName::InclineOffsetThumbDumbbellCurl,
            24 => CurlExerciseName::KettlebellBicepsCurl,
            25 => CurlExerciseName::LyingConcentrationCableCurl,
            26 => CurlExerciseName::OneArmPreacherCurl,
            27 => CurlExerciseName::PlatePinchCurl,
            28 => CurlExerciseName::PreacherCurlWithCable,
            29 => CurlExerciseName::ReverseEzBarCurl,
            30 => CurlExerciseName::ReverseGripWristCurl,
            31 => CurlExerciseName::ReverseGripBarbellBicepsCurl,
            32 => CurlExerciseName::SeatedAlternatingDumbbellBicepsCurl,
            33 => CurlExerciseName::SeatedDumbbellBicepsCurl,
            34 => CurlExerciseName::SeatedReverseDumbbellCurl,
            35 => CurlExerciseName::SplitStanceOffsetPinkyDumbbellCurl,
            36 => CurlExerciseName::StandingAlternatingDumbbellCurls,
            37 => CurlExerciseName::StandingDumbbellBicepsCurl,
            38 => CurlExerciseName::StandingEzBarBicepsCurl,
            39 => CurlExerciseName::StaticCurl,
            40 => CurlExerciseName::SwissBallDumbbellOverheadTricepsExtension,
            41 => CurlExerciseName::SwissBallEzBarPreacherCurl,
            42 => CurlExerciseName::TwistingStandingDumbbellBicepsCurl,
            43 => CurlExerciseName::WideGripEzBarBicepsCurl,
            _ => CurlExerciseName::UnknownVariant(value),
        }
    }
}
impl convert::From<i64> for CurlExerciseName {
    fn from(value: i64) -> Self {
        CurlExerciseName::from(value as u16)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum DeadliftExerciseName {
    BarbellDeadlift,
    BarbellStraightLegDeadlift,
    DumbbellDeadlift,
    DumbbellSingleLegDeadliftToRow,
    DumbbellStraightLegDeadlift,
    KettlebellFloorToShelf,
    OneArmOneLegDeadlift,
    RackPull,
    RotationalDumbbellStraightLegDeadlift,
    SingleArmDeadlift,
    SingleLegBarbellDeadlift,
    SingleLegBarbellStraightLegDeadlift,
    SingleLegDeadliftWithBarbell,
    SingleLegRdlCircuit,
    SingleLegRomanianDeadliftWithDumbbell,
    SumoDeadlift,
    SumoDeadliftHighPull,
    TrapBarDeadlift,
    WideGripBarbellDeadlift,
    UnknownVariant(u16),
}
impl DeadliftExerciseName {
    pub fn as_u16(self) -> u16 {
        match self {
            DeadliftExerciseName::BarbellDeadlift => 0,
            DeadliftExerciseName::BarbellStraightLegDeadlift => 1,
            DeadliftExerciseName::DumbbellDeadlift => 2,
            DeadliftExerciseName::DumbbellSingleLegDeadliftToRow => 3,
            DeadliftExerciseName::DumbbellStraightLegDeadlift => 4,
            DeadliftExerciseName::KettlebellFloorToShelf => 5,
            DeadliftExerciseName::OneArmOneLegDeadlift => 6,
            DeadliftExerciseName::RackPull => 7,
            DeadliftExerciseName::RotationalDumbbellStraightLegDeadlift => 8,
            DeadliftExerciseName::SingleArmDeadlift => 9,
            DeadliftExerciseName::SingleLegBarbellDeadlift => 10,
            DeadliftExerciseName::SingleLegBarbellStraightLegDeadlift => 11,
            DeadliftExerciseName::SingleLegDeadliftWithBarbell => 12,
            DeadliftExerciseName::SingleLegRdlCircuit => 13,
            DeadliftExerciseName::SingleLegRomanianDeadliftWithDumbbell => 14,
            DeadliftExerciseName::SumoDeadlift => 15,
            DeadliftExerciseName::SumoDeadliftHighPull => 16,
            DeadliftExerciseName::TrapBarDeadlift => 17,
            DeadliftExerciseName::WideGripBarbellDeadlift => 18,
            DeadliftExerciseName::UnknownVariant(value) => value,
        }
    }
    pub fn as_i64(self) -> i64 {
        self.as_u16() as i64
    }
}
impl fmt::Display for DeadliftExerciseName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            DeadliftExerciseName::BarbellDeadlift => writeln!(f, "barbell_deadlift"),
            DeadliftExerciseName::BarbellStraightLegDeadlift => {
                writeln!(f, "barbell_straight_leg_deadlift")
            }
            DeadliftExerciseName::DumbbellDeadlift => writeln!(f, "dumbbell_deadlift"),
            DeadliftExerciseName::DumbbellSingleLegDeadliftToRow => {
                writeln!(f, "dumbbell_single_leg_deadlift_to_row")
            }
            DeadliftExerciseName::DumbbellStraightLegDeadlift => {
                writeln!(f, "dumbbell_straight_leg_deadlift")
            }
            DeadliftExerciseName::KettlebellFloorToShelf => {
                writeln!(f, "kettlebell_floor_to_shelf")
            }
            DeadliftExerciseName::OneArmOneLegDeadlift => writeln!(f, "one_arm_one_leg_deadlift"),
            DeadliftExerciseName::RackPull => writeln!(f, "rack_pull"),
            DeadliftExerciseName::RotationalDumbbellStraightLegDeadlift => {
                writeln!(f, "rotational_dumbbell_straight_leg_deadlift")
            }
            DeadliftExerciseName::SingleArmDeadlift => writeln!(f, "single_arm_deadlift"),
            DeadliftExerciseName::SingleLegBarbellDeadlift => {
                writeln!(f, "single_leg_barbell_deadlift")
            }
            DeadliftExerciseName::SingleLegBarbellStraightLegDeadlift => {
                writeln!(f, "single_leg_barbell_straight_leg_deadlift")
            }
            DeadliftExerciseName::SingleLegDeadliftWithBarbell => {
                writeln!(f, "single_leg_deadlift_with_barbell")
            }
            DeadliftExerciseName::SingleLegRdlCircuit => writeln!(f, "single_leg_rdl_circuit"),
            DeadliftExerciseName::SingleLegRomanianDeadliftWithDumbbell => {
                writeln!(f, "single_leg_romanian_deadlift_with_dumbbell")
            }
            DeadliftExerciseName::SumoDeadlift => writeln!(f, "sumo_deadlift"),
            DeadliftExerciseName::SumoDeadliftHighPull => writeln!(f, "sumo_deadlift_high_pull"),
            DeadliftExerciseName::TrapBarDeadlift => writeln!(f, "trap_bar_deadlift"),
            DeadliftExerciseName::WideGripBarbellDeadlift => {
                writeln!(f, "wide_grip_barbell_deadlift")
            }
            DeadliftExerciseName::UnknownVariant(value) => {
                writeln!(f, "unknown_variant_{}", *value)
            }
        }
    }
}
impl convert::From<u16> for DeadliftExerciseName {
    fn from(value: u16) -> Self {
        match value {
            0 => DeadliftExerciseName::BarbellDeadlift,
            1 => DeadliftExerciseName::BarbellStraightLegDeadlift,
            2 => DeadliftExerciseName::DumbbellDeadlift,
            3 => DeadliftExerciseName::DumbbellSingleLegDeadliftToRow,
            4 => DeadliftExerciseName::DumbbellStraightLegDeadlift,
            5 => DeadliftExerciseName::KettlebellFloorToShelf,
            6 => DeadliftExerciseName::OneArmOneLegDeadlift,
            7 => DeadliftExerciseName::RackPull,
            8 => DeadliftExerciseName::RotationalDumbbellStraightLegDeadlift,
            9 => DeadliftExerciseName::SingleArmDeadlift,
            10 => DeadliftExerciseName::SingleLegBarbellDeadlift,
            11 => DeadliftExerciseName::SingleLegBarbellStraightLegDeadlift,
            12 => DeadliftExerciseName::SingleLegDeadliftWithBarbell,
            13 => DeadliftExerciseName::SingleLegRdlCircuit,
            14 => DeadliftExerciseName::SingleLegRomanianDeadliftWithDumbbell,
            15 => DeadliftExerciseName::SumoDeadlift,
            16 => DeadliftExerciseName::SumoDeadliftHighPull,
            17 => DeadliftExerciseName::TrapBarDeadlift,
            18 => DeadliftExerciseName::WideGripBarbellDeadlift,
            _ => DeadliftExerciseName::UnknownVariant(value),
        }
    }
}
impl convert::From<i64> for DeadliftExerciseName {
    fn from(value: i64) -> Self {
        DeadliftExerciseName::from(value as u16)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum FlyeExerciseName {
    CableCrossover,
    DeclineDumbbellFlye,
    DumbbellFlye,
    InclineDumbbellFlye,
    KettlebellFlye,
    KneelingRearFlye,
    SingleArmStandingCableReverseFlye,
    SwissBallDumbbellFlye,
    ArmRotations,
    HugATree,
    UnknownVariant(u16),
}
impl FlyeExerciseName {
    pub fn as_u16(self) -> u16 {
        match self {
            FlyeExerciseName::CableCrossover => 0,
            FlyeExerciseName::DeclineDumbbellFlye => 1,
            FlyeExerciseName::DumbbellFlye => 2,
            FlyeExerciseName::InclineDumbbellFlye => 3,
            FlyeExerciseName::KettlebellFlye => 4,
            FlyeExerciseName::KneelingRearFlye => 5,
            FlyeExerciseName::SingleArmStandingCableReverseFlye => 6,
            FlyeExerciseName::SwissBallDumbbellFlye => 7,
            FlyeExerciseName::ArmRotations => 8,
            FlyeExerciseName::HugATree => 9,
            FlyeExerciseName::UnknownVariant(value) => value,
        }
    }
    pub fn as_i64(self) -> i64 {
        self.as_u16() as i64
    }
}
impl fmt::Display for FlyeExerciseName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            FlyeExerciseName::CableCrossover => writeln!(f, "cable_crossover"),
            FlyeExerciseName::DeclineDumbbellFlye => writeln!(f, "decline_dumbbell_flye"),
            FlyeExerciseName::DumbbellFlye => writeln!(f, "dumbbell_flye"),
            FlyeExerciseName::InclineDumbbellFlye => writeln!(f, "incline_dumbbell_flye"),
            FlyeExerciseName::KettlebellFlye => writeln!(f, "kettlebell_flye"),
            FlyeExerciseName::KneelingRearFlye => writeln!(f, "kneeling_rear_flye"),
            FlyeExerciseName::SingleArmStandingCableReverseFlye => {
                writeln!(f, "single_arm_standing_cable_reverse_flye")
            }
            FlyeExerciseName::SwissBallDumbbellFlye => writeln!(f, "swiss_ball_dumbbell_flye"),
            FlyeExerciseName::ArmRotations => writeln!(f, "arm_rotations"),
            FlyeExerciseName::HugATree => writeln!(f, "hug_a_tree"),
            FlyeExerciseName::UnknownVariant(value) => writeln!(f, "unknown_variant_{}", *value),
        }
    }
}
impl convert::From<u16> for FlyeExerciseName {
    fn from(value: u16) -> Self {
        match value {
            0 => FlyeExerciseName::CableCrossover,
            1 => FlyeExerciseName::DeclineDumbbellFlye,
            2 => FlyeExerciseName::DumbbellFlye,
            3 => FlyeExerciseName::InclineDumbbellFlye,
            4 => FlyeExerciseName::KettlebellFlye,
            5 => FlyeExerciseName::KneelingRearFlye,
            6 => FlyeExerciseName::SingleArmStandingCableReverseFlye,
            7 => FlyeExerciseName::SwissBallDumbbellFlye,
            8 => FlyeExerciseName::ArmRotations,
            9 => FlyeExerciseName::HugATree,
            _ => FlyeExerciseName::UnknownVariant(value),
        }
    }
}
impl convert::From<i64> for FlyeExerciseName {
    fn from(value: i64) -> Self {
        FlyeExerciseName::from(value as u16)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum HipRaiseExerciseName {
    BarbellHipThrustOnFloor,
    BarbellHipThrustWithBench,
    BentKneeSwissBallReverseHipRaise,
    WeightedBentKneeSwissBallReverseHipRaise,
    BridgeWithLegExtension,
    WeightedBridgeWithLegExtension,
    ClamBridge,
    FrontKickTabletop,
    WeightedFrontKickTabletop,
    HipExtensionAndCross,
    WeightedHipExtensionAndCross,
    HipRaise,
    WeightedHipRaise,
    HipRaiseWithFeetOnSwissBall,
    WeightedHipRaiseWithFeetOnSwissBall,
    HipRaiseWithHeadOnBosuBall,
    WeightedHipRaiseWithHeadOnBosuBall,
    HipRaiseWithHeadOnSwissBall,
    WeightedHipRaiseWithHeadOnSwissBall,
    HipRaiseWithKneeSqueeze,
    WeightedHipRaiseWithKneeSqueeze,
    InclineRearLegExtension,
    WeightedInclineRearLegExtension,
    KettlebellSwing,
    MarchingHipRaise,
    WeightedMarchingHipRaise,
    MarchingHipRaiseWithFeetOnASwissBall,
    WeightedMarchingHipRaiseWithFeetOnASwissBall,
    ReverseHipRaise,
    WeightedReverseHipRaise,
    SingleLegHipRaise,
    WeightedSingleLegHipRaise,
    SingleLegHipRaiseWithFootOnBench,
    WeightedSingleLegHipRaiseWithFootOnBench,
    SingleLegHipRaiseWithFootOnBosuBall,
    WeightedSingleLegHipRaiseWithFootOnBosuBall,
    SingleLegHipRaiseWithFootOnFoamRoller,
    WeightedSingleLegHipRaiseWithFootOnFoamRoller,
    SingleLegHipRaiseWithFootOnMedicineBall,
    WeightedSingleLegHipRaiseWithFootOnMedicineBall,
    SingleLegHipRaiseWithHeadOnBosuBall,
    WeightedSingleLegHipRaiseWithHeadOnBosuBall,
    WeightedClamBridge,
    SingleLegSwissBallHipRaiseAndLegCurl,
    Clams,
    InnerThighCircles,
    InnerThighSideLift,
    LegCircles,
    LegLift,
    LegLiftInExternalRotation,
    UnknownVariant(u16),
}
impl HipRaiseExerciseName {
    pub fn as_u16(self) -> u16 {
        match self {
            HipRaiseExerciseName::BarbellHipThrustOnFloor => 0,
            HipRaiseExerciseName::BarbellHipThrustWithBench => 1,
            HipRaiseExerciseName::BentKneeSwissBallReverseHipRaise => 2,
            HipRaiseExerciseName::WeightedBentKneeSwissBallReverseHipRaise => 3,
            HipRaiseExerciseName::BridgeWithLegExtension => 4,
            HipRaiseExerciseName::WeightedBridgeWithLegExtension => 5,
            HipRaiseExerciseName::ClamBridge => 6,
            HipRaiseExerciseName::FrontKickTabletop => 7,
            HipRaiseExerciseName::WeightedFrontKickTabletop => 8,
            HipRaiseExerciseName::HipExtensionAndCross => 9,
            HipRaiseExerciseName::WeightedHipExtensionAndCross => 10,
            HipRaiseExerciseName::HipRaise => 11,
            HipRaiseExerciseName::WeightedHipRaise => 12,
            HipRaiseExerciseName::HipRaiseWithFeetOnSwissBall => 13,
            HipRaiseExerciseName::WeightedHipRaiseWithFeetOnSwissBall => 14,
            HipRaiseExerciseName::HipRaiseWithHeadOnBosuBall => 15,
            HipRaiseExerciseName::WeightedHipRaiseWithHeadOnBosuBall => 16,
            HipRaiseExerciseName::HipRaiseWithHeadOnSwissBall => 17,
            HipRaiseExerciseName::WeightedHipRaiseWithHeadOnSwissBall => 18,
            HipRaiseExerciseName::HipRaiseWithKneeSqueeze => 19,
            HipRaiseExerciseName::WeightedHipRaiseWithKneeSqueeze => 20,
            HipRaiseExerciseName::InclineRearLegExtension => 21,
            HipRaiseExerciseName::WeightedInclineRearLegExtension => 22,
            HipRaiseExerciseName::KettlebellSwing => 23,
            HipRaiseExerciseName::MarchingHipRaise => 24,
            HipRaiseExerciseName::WeightedMarchingHipRaise => 25,
            HipRaiseExerciseName::MarchingHipRaiseWithFeetOnASwissBall => 26,
            HipRaiseExerciseName::WeightedMarchingHipRaiseWithFeetOnASwissBall => 27,
            HipRaiseExerciseName::ReverseHipRaise => 28,
            HipRaiseExerciseName::WeightedReverseHipRaise => 29,
            HipRaiseExerciseName::SingleLegHipRaise => 30,
            HipRaiseExerciseName::WeightedSingleLegHipRaise => 31,
            HipRaiseExerciseName::SingleLegHipRaiseWithFootOnBench => 32,
            HipRaiseExerciseName::WeightedSingleLegHipRaiseWithFootOnBench => 33,
            HipRaiseExerciseName::SingleLegHipRaiseWithFootOnBosuBall => 34,
            HipRaiseExerciseName::WeightedSingleLegHipRaiseWithFootOnBosuBall => 35,
            HipRaiseExerciseName::SingleLegHipRaiseWithFootOnFoamRoller => 36,
            HipRaiseExerciseName::WeightedSingleLegHipRaiseWithFootOnFoamRoller => 37,
            HipRaiseExerciseName::SingleLegHipRaiseWithFootOnMedicineBall => 38,
            HipRaiseExerciseName::WeightedSingleLegHipRaiseWithFootOnMedicineBall => 39,
            HipRaiseExerciseName::SingleLegHipRaiseWithHeadOnBosuBall => 40,
            HipRaiseExerciseName::WeightedSingleLegHipRaiseWithHeadOnBosuBall => 41,
            HipRaiseExerciseName::WeightedClamBridge => 42,
            HipRaiseExerciseName::SingleLegSwissBallHipRaiseAndLegCurl => 43,
            HipRaiseExerciseName::Clams => 44,
            HipRaiseExerciseName::InnerThighCircles => 45,
            HipRaiseExerciseName::InnerThighSideLift => 46,
            HipRaiseExerciseName::LegCircles => 47,
            HipRaiseExerciseName::LegLift => 48,
            HipRaiseExerciseName::LegLiftInExternalRotation => 49,
            HipRaiseExerciseName::UnknownVariant(value) => value,
        }
    }
    pub fn as_i64(self) -> i64 {
        self.as_u16() as i64
    }
}
impl fmt::Display for HipRaiseExerciseName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            HipRaiseExerciseName::BarbellHipThrustOnFloor => {
                writeln!(f, "barbell_hip_thrust_on_floor")
            }
            HipRaiseExerciseName::BarbellHipThrustWithBench => {
                writeln!(f, "barbell_hip_thrust_with_bench")
            }
            HipRaiseExerciseName::BentKneeSwissBallReverseHipRaise => {
                writeln!(f, "bent_knee_swiss_ball_reverse_hip_raise")
            }
            HipRaiseExerciseName::WeightedBentKneeSwissBallReverseHipRaise => {
                writeln!(f, "weighted_bent_knee_swiss_ball_reverse_hip_raise")
            }
            HipRaiseExerciseName::BridgeWithLegExtension => {
                writeln!(f, "bridge_with_leg_extension")
            }
            HipRaiseExerciseName::WeightedBridgeWithLegExtension => {
                writeln!(f, "weighted_bridge_with_leg_extension")
            }
            HipRaiseExerciseName::ClamBridge => writeln!(f, "clam_bridge"),
            HipRaiseExerciseName::FrontKickTabletop => writeln!(f, "front_kick_tabletop"),
            HipRaiseExerciseName::WeightedFrontKickTabletop => {
                writeln!(f, "weighted_front_kick_tabletop")
            }
            HipRaiseExerciseName::HipExtensionAndCross => writeln!(f, "hip_extension_and_cross"),
            HipRaiseExerciseName::WeightedHipExtensionAndCross => {
                writeln!(f, "weighted_hip_extension_and_cross")
            }
            HipRaiseExerciseName::HipRaise => writeln!(f, "hip_raise"),
            HipRaiseExerciseName::WeightedHipRaise => writeln!(f, "weighted_hip_raise"),
            HipRaiseExerciseName::HipRaiseWithFeetOnSwissBall => {
                writeln!(f, "hip_raise_with_feet_on_swiss_ball")
            }
            HipRaiseExerciseName::WeightedHipRaiseWithFeetOnSwissBall => {
                writeln!(f, "weighted_hip_raise_with_feet_on_swiss_ball")
            }
            HipRaiseExerciseName::HipRaiseWithHeadOnBosuBall => {
                writeln!(f, "hip_raise_with_head_on_bosu_ball")
            }
            HipRaiseExerciseName::WeightedHipRaiseWithHeadOnBosuBall => {
                writeln!(f, "weighted_hip_raise_with_head_on_bosu_ball")
            }
            HipRaiseExerciseName::HipRaiseWithHeadOnSwissBall => {
                writeln!(f, "hip_raise_with_head_on_swiss_ball")
            }
            HipRaiseExerciseName::WeightedHipRaiseWithHeadOnSwissBall => {
                writeln!(f, "weighted_hip_raise_with_head_on_swiss_ball")
            }
            HipRaiseExerciseName::HipRaiseWithKneeSqueeze => {
                writeln!(f, "hip_raise_with_knee_squeeze")
            }
            HipRaiseExerciseName::WeightedHipRaiseWithKneeSqueeze => {
                writeln!(f, "weighted_hip_raise_with_knee_squeeze")
            }
            HipRaiseExerciseName::InclineRearLegExtension => {
                writeln!(f, "incline_rear_leg_extension")
            }
            HipRaiseExerciseName::WeightedInclineRearLegExtension => {
                writeln!(f, "weighted_incline_rear_leg_extension")
            }
            HipRaiseExerciseName::KettlebellSwing => writeln!(f, "kettlebell_swing"),
            HipRaiseExerciseName::MarchingHipRaise => writeln!(f, "marching_hip_raise"),
            HipRaiseExerciseName::WeightedMarchingHipRaise => {
                writeln!(f, "weighted_marching_hip_raise")
            }
            HipRaiseExerciseName::MarchingHipRaiseWithFeetOnASwissBall => {
                writeln!(f, "marching_hip_raise_with_feet_on_a_swiss_ball")
            }
            HipRaiseExerciseName::WeightedMarchingHipRaiseWithFeetOnASwissBall => {
                writeln!(f, "weighted_marching_hip_raise_with_feet_on_a_swiss_ball")
            }
            HipRaiseExerciseName::ReverseHipRaise => writeln!(f, "reverse_hip_raise"),
            HipRaiseExerciseName::WeightedReverseHipRaise => {
                writeln!(f, "weighted_reverse_hip_raise")
            }
            HipRaiseExerciseName::SingleLegHipRaise => writeln!(f, "single_leg_hip_raise"),
            HipRaiseExerciseName::WeightedSingleLegHipRaise => {
                writeln!(f, "weighted_single_leg_hip_raise")
            }
            HipRaiseExerciseName::SingleLegHipRaiseWithFootOnBench => {
                writeln!(f, "single_leg_hip_raise_with_foot_on_bench")
            }
            HipRaiseExerciseName::WeightedSingleLegHipRaiseWithFootOnBench => {
                writeln!(f, "weighted_single_leg_hip_raise_with_foot_on_bench")
            }
            HipRaiseExerciseName::SingleLegHipRaiseWithFootOnBosuBall => {
                writeln!(f, "single_leg_hip_raise_with_foot_on_bosu_ball")
            }
            HipRaiseExerciseName::WeightedSingleLegHipRaiseWithFootOnBosuBall => {
                writeln!(f, "weighted_single_leg_hip_raise_with_foot_on_bosu_ball")
            }
            HipRaiseExerciseName::SingleLegHipRaiseWithFootOnFoamRoller => {
                writeln!(f, "single_leg_hip_raise_with_foot_on_foam_roller")
            }
            HipRaiseExerciseName::WeightedSingleLegHipRaiseWithFootOnFoamRoller => {
                writeln!(f, "weighted_single_leg_hip_raise_with_foot_on_foam_roller")
            }
            HipRaiseExerciseName::SingleLegHipRaiseWithFootOnMedicineBall => {
                writeln!(f, "single_leg_hip_raise_with_foot_on_medicine_ball")
            }
            HipRaiseExerciseName::WeightedSingleLegHipRaiseWithFootOnMedicineBall => writeln!(
                f,
                "weighted_single_leg_hip_raise_with_foot_on_medicine_ball"
            ),
            HipRaiseExerciseName::SingleLegHipRaiseWithHeadOnBosuBall => {
                writeln!(f, "single_leg_hip_raise_with_head_on_bosu_ball")
            }
            HipRaiseExerciseName::WeightedSingleLegHipRaiseWithHeadOnBosuBall => {
                writeln!(f, "weighted_single_leg_hip_raise_with_head_on_bosu_ball")
            }
            HipRaiseExerciseName::WeightedClamBridge => writeln!(f, "weighted_clam_bridge"),
            HipRaiseExerciseName::SingleLegSwissBallHipRaiseAndLegCurl => {
                writeln!(f, "single_leg_swiss_ball_hip_raise_and_leg_curl")
            }
            HipRaiseExerciseName::Clams => writeln!(f, "clams"),
            HipRaiseExerciseName::InnerThighCircles => writeln!(f, "inner_thigh_circles"),
            HipRaiseExerciseName::InnerThighSideLift => writeln!(f, "inner_thigh_side_lift"),
            HipRaiseExerciseName::LegCircles => writeln!(f, "leg_circles"),
            HipRaiseExerciseName::LegLift => writeln!(f, "leg_lift"),
            HipRaiseExerciseName::LegLiftInExternalRotation => {
                writeln!(f, "leg_lift_in_external_rotation")
            }
            HipRaiseExerciseName::UnknownVariant(value) => {
                writeln!(f, "unknown_variant_{}", *value)
            }
        }
    }
}
impl convert::From<u16> for HipRaiseExerciseName {
    fn from(value: u16) -> Self {
        match value {
            0 => HipRaiseExerciseName::BarbellHipThrustOnFloor,
            1 => HipRaiseExerciseName::BarbellHipThrustWithBench,
            2 => HipRaiseExerciseName::BentKneeSwissBallReverseHipRaise,
            3 => HipRaiseExerciseName::WeightedBentKneeSwissBallReverseHipRaise,
            4 => HipRaiseExerciseName::BridgeWithLegExtension,
            5 => HipRaiseExerciseName::WeightedBridgeWithLegExtension,
            6 => HipRaiseExerciseName::ClamBridge,
            7 => HipRaiseExerciseName::FrontKickTabletop,
            8 => HipRaiseExerciseName::WeightedFrontKickTabletop,
            9 => HipRaiseExerciseName::HipExtensionAndCross,
            10 => HipRaiseExerciseName::WeightedHipExtensionAndCross,
            11 => HipRaiseExerciseName::HipRaise,
            12 => HipRaiseExerciseName::WeightedHipRaise,
            13 => HipRaiseExerciseName::HipRaiseWithFeetOnSwissBall,
            14 => HipRaiseExerciseName::WeightedHipRaiseWithFeetOnSwissBall,
            15 => HipRaiseExerciseName::HipRaiseWithHeadOnBosuBall,
            16 => HipRaiseExerciseName::WeightedHipRaiseWithHeadOnBosuBall,
            17 => HipRaiseExerciseName::HipRaiseWithHeadOnSwissBall,
            18 => HipRaiseExerciseName::WeightedHipRaiseWithHeadOnSwissBall,
            19 => HipRaiseExerciseName::HipRaiseWithKneeSqueeze,
            20 => HipRaiseExerciseName::WeightedHipRaiseWithKneeSqueeze,
            21 => HipRaiseExerciseName::InclineRearLegExtension,
            22 => HipRaiseExerciseName::WeightedInclineRearLegExtension,
            23 => HipRaiseExerciseName::KettlebellSwing,
            24 => HipRaiseExerciseName::MarchingHipRaise,
            25 => HipRaiseExerciseName::WeightedMarchingHipRaise,
            26 => HipRaiseExerciseName::MarchingHipRaiseWithFeetOnASwissBall,
            27 => HipRaiseExerciseName::WeightedMarchingHipRaiseWithFeetOnASwissBall,
            28 => HipRaiseExerciseName::ReverseHipRaise,
            29 => HipRaiseExerciseName::WeightedReverseHipRaise,
            30 => HipRaiseExerciseName::SingleLegHipRaise,
            31 => HipRaiseExerciseName::WeightedSingleLegHipRaise,
            32 => HipRaiseExerciseName::SingleLegHipRaiseWithFootOnBench,
            33 => HipRaiseExerciseName::WeightedSingleLegHipRaiseWithFootOnBench,
            34 => HipRaiseExerciseName::SingleLegHipRaiseWithFootOnBosuBall,
            35 => HipRaiseExerciseName::WeightedSingleLegHipRaiseWithFootOnBosuBall,
            36 => HipRaiseExerciseName::SingleLegHipRaiseWithFootOnFoamRoller,
            37 => HipRaiseExerciseName::WeightedSingleLegHipRaiseWithFootOnFoamRoller,
            38 => HipRaiseExerciseName::SingleLegHipRaiseWithFootOnMedicineBall,
            39 => HipRaiseExerciseName::WeightedSingleLegHipRaiseWithFootOnMedicineBall,
            40 => HipRaiseExerciseName::SingleLegHipRaiseWithHeadOnBosuBall,
            41 => HipRaiseExerciseName::WeightedSingleLegHipRaiseWithHeadOnBosuBall,
            42 => HipRaiseExerciseName::WeightedClamBridge,
            43 => HipRaiseExerciseName::SingleLegSwissBallHipRaiseAndLegCurl,
            44 => HipRaiseExerciseName::Clams,
            45 => HipRaiseExerciseName::InnerThighCircles,
            46 => HipRaiseExerciseName::InnerThighSideLift,
            47 => HipRaiseExerciseName::LegCircles,
            48 => HipRaiseExerciseName::LegLift,
            49 => HipRaiseExerciseName::LegLiftInExternalRotation,
            _ => HipRaiseExerciseName::UnknownVariant(value),
        }
    }
}
impl convert::From<i64> for HipRaiseExerciseName {
    fn from(value: i64) -> Self {
        HipRaiseExerciseName::from(value as u16)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum HipStabilityExerciseName {
    BandSideLyingLegRaise,
    DeadBug,
    WeightedDeadBug,
    ExternalHipRaise,
    WeightedExternalHipRaise,
    FireHydrantKicks,
    WeightedFireHydrantKicks,
    HipCircles,
    WeightedHipCircles,
    InnerThighLift,
    WeightedInnerThighLift,
    LateralWalksWithBandAtAnkles,
    PretzelSideKick,
    WeightedPretzelSideKick,
    ProneHipInternalRotation,
    WeightedProneHipInternalRotation,
    Quadruped,
    QuadrupedHipExtension,
    WeightedQuadrupedHipExtension,
    QuadrupedWithLegLift,
    WeightedQuadrupedWithLegLift,
    SideLyingLegRaise,
    WeightedSideLyingLegRaise,
    SlidingHipAdduction,
    WeightedSlidingHipAdduction,
    StandingAdduction,
    WeightedStandingAdduction,
    StandingCableHipAbduction,
    StandingHipAbduction,
    WeightedStandingHipAbduction,
    StandingRearLegRaise,
    WeightedStandingRearLegRaise,
    SupineHipInternalRotation,
    WeightedSupineHipInternalRotation,
    UnknownVariant(u16),
}
impl HipStabilityExerciseName {
    pub fn as_u16(self) -> u16 {
        match self {
            HipStabilityExerciseName::BandSideLyingLegRaise => 0,
            HipStabilityExerciseName::DeadBug => 1,
            HipStabilityExerciseName::WeightedDeadBug => 2,
            HipStabilityExerciseName::ExternalHipRaise => 3,
            HipStabilityExerciseName::WeightedExternalHipRaise => 4,
            HipStabilityExerciseName::FireHydrantKicks => 5,
            HipStabilityExerciseName::WeightedFireHydrantKicks => 6,
            HipStabilityExerciseName::HipCircles => 7,
            HipStabilityExerciseName::WeightedHipCircles => 8,
            HipStabilityExerciseName::InnerThighLift => 9,
            HipStabilityExerciseName::WeightedInnerThighLift => 10,
            HipStabilityExerciseName::LateralWalksWithBandAtAnkles => 11,
            HipStabilityExerciseName::PretzelSideKick => 12,
            HipStabilityExerciseName::WeightedPretzelSideKick => 13,
            HipStabilityExerciseName::ProneHipInternalRotation => 14,
            HipStabilityExerciseName::WeightedProneHipInternalRotation => 15,
            HipStabilityExerciseName::Quadruped => 16,
            HipStabilityExerciseName::QuadrupedHipExtension => 17,
            HipStabilityExerciseName::WeightedQuadrupedHipExtension => 18,
            HipStabilityExerciseName::QuadrupedWithLegLift => 19,
            HipStabilityExerciseName::WeightedQuadrupedWithLegLift => 20,
            HipStabilityExerciseName::SideLyingLegRaise => 21,
            HipStabilityExerciseName::WeightedSideLyingLegRaise => 22,
            HipStabilityExerciseName::SlidingHipAdduction => 23,
            HipStabilityExerciseName::WeightedSlidingHipAdduction => 24,
            HipStabilityExerciseName::StandingAdduction => 25,
            HipStabilityExerciseName::WeightedStandingAdduction => 26,
            HipStabilityExerciseName::StandingCableHipAbduction => 27,
            HipStabilityExerciseName::StandingHipAbduction => 28,
            HipStabilityExerciseName::WeightedStandingHipAbduction => 29,
            HipStabilityExerciseName::StandingRearLegRaise => 30,
            HipStabilityExerciseName::WeightedStandingRearLegRaise => 31,
            HipStabilityExerciseName::SupineHipInternalRotation => 32,
            HipStabilityExerciseName::WeightedSupineHipInternalRotation => 33,
            HipStabilityExerciseName::UnknownVariant(value) => value,
        }
    }
    pub fn as_i64(self) -> i64 {
        self.as_u16() as i64
    }
}
impl fmt::Display for HipStabilityExerciseName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            HipStabilityExerciseName::BandSideLyingLegRaise => {
                writeln!(f, "band_side_lying_leg_raise")
            }
            HipStabilityExerciseName::DeadBug => writeln!(f, "dead_bug"),
            HipStabilityExerciseName::WeightedDeadBug => writeln!(f, "weighted_dead_bug"),
            HipStabilityExerciseName::ExternalHipRaise => writeln!(f, "external_hip_raise"),
            HipStabilityExerciseName::WeightedExternalHipRaise => {
                writeln!(f, "weighted_external_hip_raise")
            }
            HipStabilityExerciseName::FireHydrantKicks => writeln!(f, "fire_hydrant_kicks"),
            HipStabilityExerciseName::WeightedFireHydrantKicks => {
                writeln!(f, "weighted_fire_hydrant_kicks")
            }
            HipStabilityExerciseName::HipCircles => writeln!(f, "hip_circles"),
            HipStabilityExerciseName::WeightedHipCircles => writeln!(f, "weighted_hip_circles"),
            HipStabilityExerciseName::InnerThighLift => writeln!(f, "inner_thigh_lift"),
            HipStabilityExerciseName::WeightedInnerThighLift => {
                writeln!(f, "weighted_inner_thigh_lift")
            }
            HipStabilityExerciseName::LateralWalksWithBandAtAnkles => {
                writeln!(f, "lateral_walks_with_band_at_ankles")
            }
            HipStabilityExerciseName::PretzelSideKick => writeln!(f, "pretzel_side_kick"),
            HipStabilityExerciseName::WeightedPretzelSideKick => {
                writeln!(f, "weighted_pretzel_side_kick")
            }
            HipStabilityExerciseName::ProneHipInternalRotation => {
                writeln!(f, "prone_hip_internal_rotation")
            }
            HipStabilityExerciseName::WeightedProneHipInternalRotation => {
                writeln!(f, "weighted_prone_hip_internal_rotation")
            }
            HipStabilityExerciseName::Quadruped => writeln!(f, "quadruped"),
            HipStabilityExerciseName::QuadrupedHipExtension => {
                writeln!(f, "quadruped_hip_extension")
            }
            HipStabilityExerciseName::WeightedQuadrupedHipExtension => {
                writeln!(f, "weighted_quadruped_hip_extension")
            }
            HipStabilityExerciseName::QuadrupedWithLegLift => {
                writeln!(f, "quadruped_with_leg_lift")
            }
            HipStabilityExerciseName::WeightedQuadrupedWithLegLift => {
                writeln!(f, "weighted_quadruped_with_leg_lift")
            }
            HipStabilityExerciseName::SideLyingLegRaise => writeln!(f, "side_lying_leg_raise"),
            HipStabilityExerciseName::WeightedSideLyingLegRaise => {
                writeln!(f, "weighted_side_lying_leg_raise")
            }
            HipStabilityExerciseName::SlidingHipAdduction => writeln!(f, "sliding_hip_adduction"),
            HipStabilityExerciseName::WeightedSlidingHipAdduction => {
                writeln!(f, "weighted_sliding_hip_adduction")
            }
            HipStabilityExerciseName::StandingAdduction => writeln!(f, "standing_adduction"),
            HipStabilityExerciseName::WeightedStandingAdduction => {
                writeln!(f, "weighted_standing_adduction")
            }
            HipStabilityExerciseName::StandingCableHipAbduction => {
                writeln!(f, "standing_cable_hip_abduction")
            }
            HipStabilityExerciseName::StandingHipAbduction => writeln!(f, "standing_hip_abduction"),
            HipStabilityExerciseName::WeightedStandingHipAbduction => {
                writeln!(f, "weighted_standing_hip_abduction")
            }
            HipStabilityExerciseName::StandingRearLegRaise => {
                writeln!(f, "standing_rear_leg_raise")
            }
            HipStabilityExerciseName::WeightedStandingRearLegRaise => {
                writeln!(f, "weighted_standing_rear_leg_raise")
            }
            HipStabilityExerciseName::SupineHipInternalRotation => {
                writeln!(f, "supine_hip_internal_rotation")
            }
            HipStabilityExerciseName::WeightedSupineHipInternalRotation => {
                writeln!(f, "weighted_supine_hip_internal_rotation")
            }
            HipStabilityExerciseName::UnknownVariant(value) => {
                writeln!(f, "unknown_variant_{}", *value)
            }
        }
    }
}
impl convert::From<u16> for HipStabilityExerciseName {
    fn from(value: u16) -> Self {
        match value {
            0 => HipStabilityExerciseName::BandSideLyingLegRaise,
            1 => HipStabilityExerciseName::DeadBug,
            2 => HipStabilityExerciseName::WeightedDeadBug,
            3 => HipStabilityExerciseName::ExternalHipRaise,
            4 => HipStabilityExerciseName::WeightedExternalHipRaise,
            5 => HipStabilityExerciseName::FireHydrantKicks,
            6 => HipStabilityExerciseName::WeightedFireHydrantKicks,
            7 => HipStabilityExerciseName::HipCircles,
            8 => HipStabilityExerciseName::WeightedHipCircles,
            9 => HipStabilityExerciseName::InnerThighLift,
            10 => HipStabilityExerciseName::WeightedInnerThighLift,
            11 => HipStabilityExerciseName::LateralWalksWithBandAtAnkles,
            12 => HipStabilityExerciseName::PretzelSideKick,
            13 => HipStabilityExerciseName::WeightedPretzelSideKick,
            14 => HipStabilityExerciseName::ProneHipInternalRotation,
            15 => HipStabilityExerciseName::WeightedProneHipInternalRotation,
            16 => HipStabilityExerciseName::Quadruped,
            17 => HipStabilityExerciseName::QuadrupedHipExtension,
            18 => HipStabilityExerciseName::WeightedQuadrupedHipExtension,
            19 => HipStabilityExerciseName::QuadrupedWithLegLift,
            20 => HipStabilityExerciseName::WeightedQuadrupedWithLegLift,
            21 => HipStabilityExerciseName::SideLyingLegRaise,
            22 => HipStabilityExerciseName::WeightedSideLyingLegRaise,
            23 => HipStabilityExerciseName::SlidingHipAdduction,
            24 => HipStabilityExerciseName::WeightedSlidingHipAdduction,
            25 => HipStabilityExerciseName::StandingAdduction,
            26 => HipStabilityExerciseName::WeightedStandingAdduction,
            27 => HipStabilityExerciseName::StandingCableHipAbduction,
            28 => HipStabilityExerciseName::StandingHipAbduction,
            29 => HipStabilityExerciseName::WeightedStandingHipAbduction,
            30 => HipStabilityExerciseName::StandingRearLegRaise,
            31 => HipStabilityExerciseName::WeightedStandingRearLegRaise,
            32 => HipStabilityExerciseName::SupineHipInternalRotation,
            33 => HipStabilityExerciseName::WeightedSupineHipInternalRotation,
            _ => HipStabilityExerciseName::UnknownVariant(value),
        }
    }
}
impl convert::From<i64> for HipStabilityExerciseName {
    fn from(value: i64) -> Self {
        HipStabilityExerciseName::from(value as u16)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum HipSwingExerciseName {
    SingleArmKettlebellSwing,
    SingleArmDumbbellSwing,
    StepOutSwing,
    UnknownVariant(u16),
}
impl HipSwingExerciseName {
    pub fn as_u16(self) -> u16 {
        match self {
            HipSwingExerciseName::SingleArmKettlebellSwing => 0,
            HipSwingExerciseName::SingleArmDumbbellSwing => 1,
            HipSwingExerciseName::StepOutSwing => 2,
            HipSwingExerciseName::UnknownVariant(value) => value,
        }
    }
    pub fn as_i64(self) -> i64 {
        self.as_u16() as i64
    }
}
impl fmt::Display for HipSwingExerciseName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            HipSwingExerciseName::SingleArmKettlebellSwing => {
                writeln!(f, "single_arm_kettlebell_swing")
            }
            HipSwingExerciseName::SingleArmDumbbellSwing => {
                writeln!(f, "single_arm_dumbbell_swing")
            }
            HipSwingExerciseName::StepOutSwing => writeln!(f, "step_out_swing"),
            HipSwingExerciseName::UnknownVariant(value) => {
                writeln!(f, "unknown_variant_{}", *value)
            }
        }
    }
}
impl convert::From<u16> for HipSwingExerciseName {
    fn from(value: u16) -> Self {
        match value {
            0 => HipSwingExerciseName::SingleArmKettlebellSwing,
            1 => HipSwingExerciseName::SingleArmDumbbellSwing,
            2 => HipSwingExerciseName::StepOutSwing,
            _ => HipSwingExerciseName::UnknownVariant(value),
        }
    }
}
impl convert::From<i64> for HipSwingExerciseName {
    fn from(value: i64) -> Self {
        HipSwingExerciseName::from(value as u16)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum HyperextensionExerciseName {
    BackExtensionWithOppositeArmAndLegReach,
    WeightedBackExtensionWithOppositeArmAndLegReach,
    BaseRotations,
    WeightedBaseRotations,
    BentKneeReverseHyperextension,
    WeightedBentKneeReverseHyperextension,
    HollowHoldAndRoll,
    WeightedHollowHoldAndRoll,
    Kicks,
    WeightedKicks,
    KneeRaises,
    WeightedKneeRaises,
    KneelingSuperman,
    WeightedKneelingSuperman,
    LatPullDownWithRow,
    MedicineBallDeadliftToReach,
    OneArmOneLegRow,
    OneArmRowWithBand,
    OverheadLungeWithMedicineBall,
    PlankKneeTucks,
    WeightedPlankKneeTucks,
    SideStep,
    WeightedSideStep,
    SingleLegBackExtension,
    WeightedSingleLegBackExtension,
    SpineExtension,
    WeightedSpineExtension,
    StaticBackExtension,
    WeightedStaticBackExtension,
    SupermanFromFloor,
    WeightedSupermanFromFloor,
    SwissBallBackExtension,
    WeightedSwissBallBackExtension,
    SwissBallHyperextension,
    WeightedSwissBallHyperextension,
    SwissBallOppositeArmAndLegLift,
    WeightedSwissBallOppositeArmAndLegLift,
    SupermanOnSwissBall,
    Cobra,
    SupineFloorBarre,
    UnknownVariant(u16),
}
impl HyperextensionExerciseName {
    pub fn as_u16(self) -> u16 {
        match self {
            HyperextensionExerciseName::BackExtensionWithOppositeArmAndLegReach => 0,
            HyperextensionExerciseName::WeightedBackExtensionWithOppositeArmAndLegReach => 1,
            HyperextensionExerciseName::BaseRotations => 2,
            HyperextensionExerciseName::WeightedBaseRotations => 3,
            HyperextensionExerciseName::BentKneeReverseHyperextension => 4,
            HyperextensionExerciseName::WeightedBentKneeReverseHyperextension => 5,
            HyperextensionExerciseName::HollowHoldAndRoll => 6,
            HyperextensionExerciseName::WeightedHollowHoldAndRoll => 7,
            HyperextensionExerciseName::Kicks => 8,
            HyperextensionExerciseName::WeightedKicks => 9,
            HyperextensionExerciseName::KneeRaises => 10,
            HyperextensionExerciseName::WeightedKneeRaises => 11,
            HyperextensionExerciseName::KneelingSuperman => 12,
            HyperextensionExerciseName::WeightedKneelingSuperman => 13,
            HyperextensionExerciseName::LatPullDownWithRow => 14,
            HyperextensionExerciseName::MedicineBallDeadliftToReach => 15,
            HyperextensionExerciseName::OneArmOneLegRow => 16,
            HyperextensionExerciseName::OneArmRowWithBand => 17,
            HyperextensionExerciseName::OverheadLungeWithMedicineBall => 18,
            HyperextensionExerciseName::PlankKneeTucks => 19,
            HyperextensionExerciseName::WeightedPlankKneeTucks => 20,
            HyperextensionExerciseName::SideStep => 21,
            HyperextensionExerciseName::WeightedSideStep => 22,
            HyperextensionExerciseName::SingleLegBackExtension => 23,
            HyperextensionExerciseName::WeightedSingleLegBackExtension => 24,
            HyperextensionExerciseName::SpineExtension => 25,
            HyperextensionExerciseName::WeightedSpineExtension => 26,
            HyperextensionExerciseName::StaticBackExtension => 27,
            HyperextensionExerciseName::WeightedStaticBackExtension => 28,
            HyperextensionExerciseName::SupermanFromFloor => 29,
            HyperextensionExerciseName::WeightedSupermanFromFloor => 30,
            HyperextensionExerciseName::SwissBallBackExtension => 31,
            HyperextensionExerciseName::WeightedSwissBallBackExtension => 32,
            HyperextensionExerciseName::SwissBallHyperextension => 33,
            HyperextensionExerciseName::WeightedSwissBallHyperextension => 34,
            HyperextensionExerciseName::SwissBallOppositeArmAndLegLift => 35,
            HyperextensionExerciseName::WeightedSwissBallOppositeArmAndLegLift => 36,
            HyperextensionExerciseName::SupermanOnSwissBall => 37,
            HyperextensionExerciseName::Cobra => 38,
            HyperextensionExerciseName::SupineFloorBarre => 39,
            HyperextensionExerciseName::UnknownVariant(value) => value,
        }
    }
    pub fn as_i64(self) -> i64 {
        self.as_u16() as i64
    }
}
impl fmt::Display for HyperextensionExerciseName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            HyperextensionExerciseName::BackExtensionWithOppositeArmAndLegReach => {
                writeln!(f, "back_extension_with_opposite_arm_and_leg_reach")
            }
            HyperextensionExerciseName::WeightedBackExtensionWithOppositeArmAndLegReach => {
                writeln!(f, "weighted_back_extension_with_opposite_arm_and_leg_reach")
            }
            HyperextensionExerciseName::BaseRotations => writeln!(f, "base_rotations"),
            HyperextensionExerciseName::WeightedBaseRotations => {
                writeln!(f, "weighted_base_rotations")
            }
            HyperextensionExerciseName::BentKneeReverseHyperextension => {
                writeln!(f, "bent_knee_reverse_hyperextension")
            }
            HyperextensionExerciseName::WeightedBentKneeReverseHyperextension => {
                writeln!(f, "weighted_bent_knee_reverse_hyperextension")
            }
            HyperextensionExerciseName::HollowHoldAndRoll => writeln!(f, "hollow_hold_and_roll"),
            HyperextensionExerciseName::WeightedHollowHoldAndRoll => {
                writeln!(f, "weighted_hollow_hold_and_roll")
            }
            HyperextensionExerciseName::Kicks => writeln!(f, "kicks"),
            HyperextensionExerciseName::WeightedKicks => writeln!(f, "weighted_kicks"),
            HyperextensionExerciseName::KneeRaises => writeln!(f, "knee_raises"),
            HyperextensionExerciseName::WeightedKneeRaises => writeln!(f, "weighted_knee_raises"),
            HyperextensionExerciseName::KneelingSuperman => writeln!(f, "kneeling_superman"),
            HyperextensionExerciseName::WeightedKneelingSuperman => {
                writeln!(f, "weighted_kneeling_superman")
            }
            HyperextensionExerciseName::LatPullDownWithRow => writeln!(f, "lat_pull_down_with_row"),
            HyperextensionExerciseName::MedicineBallDeadliftToReach => {
                writeln!(f, "medicine_ball_deadlift_to_reach")
            }
            HyperextensionExerciseName::OneArmOneLegRow => writeln!(f, "one_arm_one_leg_row"),
            HyperextensionExerciseName::OneArmRowWithBand => writeln!(f, "one_arm_row_with_band"),
            HyperextensionExerciseName::OverheadLungeWithMedicineBall => {
                writeln!(f, "overhead_lunge_with_medicine_ball")
            }
            HyperextensionExerciseName::PlankKneeTucks => writeln!(f, "plank_knee_tucks"),
            HyperextensionExerciseName::WeightedPlankKneeTucks => {
                writeln!(f, "weighted_plank_knee_tucks")
            }
            HyperextensionExerciseName::SideStep => writeln!(f, "side_step"),
            HyperextensionExerciseName::WeightedSideStep => writeln!(f, "weighted_side_step"),
            HyperextensionExerciseName::SingleLegBackExtension => {
                writeln!(f, "single_leg_back_extension")
            }
            HyperextensionExerciseName::WeightedSingleLegBackExtension => {
                writeln!(f, "weighted_single_leg_back_extension")
            }
            HyperextensionExerciseName::SpineExtension => writeln!(f, "spine_extension"),
            HyperextensionExerciseName::WeightedSpineExtension => {
                writeln!(f, "weighted_spine_extension")
            }
            HyperextensionExerciseName::StaticBackExtension => writeln!(f, "static_back_extension"),
            HyperextensionExerciseName::WeightedStaticBackExtension => {
                writeln!(f, "weighted_static_back_extension")
            }
            HyperextensionExerciseName::SupermanFromFloor => writeln!(f, "superman_from_floor"),
            HyperextensionExerciseName::WeightedSupermanFromFloor => {
                writeln!(f, "weighted_superman_from_floor")
            }
            HyperextensionExerciseName::SwissBallBackExtension => {
                writeln!(f, "swiss_ball_back_extension")
            }
            HyperextensionExerciseName::WeightedSwissBallBackExtension => {
                writeln!(f, "weighted_swiss_ball_back_extension")
            }
            HyperextensionExerciseName::SwissBallHyperextension => {
                writeln!(f, "swiss_ball_hyperextension")
            }
            HyperextensionExerciseName::WeightedSwissBallHyperextension => {
                writeln!(f, "weighted_swiss_ball_hyperextension")
            }
            HyperextensionExerciseName::SwissBallOppositeArmAndLegLift => {
                writeln!(f, "swiss_ball_opposite_arm_and_leg_lift")
            }
            HyperextensionExerciseName::WeightedSwissBallOppositeArmAndLegLift => {
                writeln!(f, "weighted_swiss_ball_opposite_arm_and_leg_lift")
            }
            HyperextensionExerciseName::SupermanOnSwissBall => {
                writeln!(f, "superman_on_swiss_ball")
            }
            HyperextensionExerciseName::Cobra => writeln!(f, "cobra"),
            HyperextensionExerciseName::SupineFloorBarre => writeln!(f, "supine_floor_barre"),
            HyperextensionExerciseName::UnknownVariant(value) => {
                writeln!(f, "unknown_variant_{}", *value)
            }
        }
    }
}
impl convert::From<u16> for HyperextensionExerciseName {
    fn from(value: u16) -> Self {
        match value {
            0 => HyperextensionExerciseName::BackExtensionWithOppositeArmAndLegReach,
            1 => HyperextensionExerciseName::WeightedBackExtensionWithOppositeArmAndLegReach,
            2 => HyperextensionExerciseName::BaseRotations,
            3 => HyperextensionExerciseName::WeightedBaseRotations,
            4 => HyperextensionExerciseName::BentKneeReverseHyperextension,
            5 => HyperextensionExerciseName::WeightedBentKneeReverseHyperextension,
            6 => HyperextensionExerciseName::HollowHoldAndRoll,
            7 => HyperextensionExerciseName::WeightedHollowHoldAndRoll,
            8 => HyperextensionExerciseName::Kicks,
            9 => HyperextensionExerciseName::WeightedKicks,
            10 => HyperextensionExerciseName::KneeRaises,
            11 => HyperextensionExerciseName::WeightedKneeRaises,
            12 => HyperextensionExerciseName::KneelingSuperman,
            13 => HyperextensionExerciseName::WeightedKneelingSuperman,
            14 => HyperextensionExerciseName::LatPullDownWithRow,
            15 => HyperextensionExerciseName::MedicineBallDeadliftToReach,
            16 => HyperextensionExerciseName::OneArmOneLegRow,
            17 => HyperextensionExerciseName::OneArmRowWithBand,
            18 => HyperextensionExerciseName::OverheadLungeWithMedicineBall,
            19 => HyperextensionExerciseName::PlankKneeTucks,
            20 => HyperextensionExerciseName::WeightedPlankKneeTucks,
            21 => HyperextensionExerciseName::SideStep,
            22 => HyperextensionExerciseName::WeightedSideStep,
            23 => HyperextensionExerciseName::SingleLegBackExtension,
            24 => HyperextensionExerciseName::WeightedSingleLegBackExtension,
            25 => HyperextensionExerciseName::SpineExtension,
            26 => HyperextensionExerciseName::WeightedSpineExtension,
            27 => HyperextensionExerciseName::StaticBackExtension,
            28 => HyperextensionExerciseName::WeightedStaticBackExtension,
            29 => HyperextensionExerciseName::SupermanFromFloor,
            30 => HyperextensionExerciseName::WeightedSupermanFromFloor,
            31 => HyperextensionExerciseName::SwissBallBackExtension,
            32 => HyperextensionExerciseName::WeightedSwissBallBackExtension,
            33 => HyperextensionExerciseName::SwissBallHyperextension,
            34 => HyperextensionExerciseName::WeightedSwissBallHyperextension,
            35 => HyperextensionExerciseName::SwissBallOppositeArmAndLegLift,
            36 => HyperextensionExerciseName::WeightedSwissBallOppositeArmAndLegLift,
            37 => HyperextensionExerciseName::SupermanOnSwissBall,
            38 => HyperextensionExerciseName::Cobra,
            39 => HyperextensionExerciseName::SupineFloorBarre,
            _ => HyperextensionExerciseName::UnknownVariant(value),
        }
    }
}
impl convert::From<i64> for HyperextensionExerciseName {
    fn from(value: i64) -> Self {
        HyperextensionExerciseName::from(value as u16)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum LateralRaiseExerciseName {
    Name45DegreeCableExternalRotation,
    AlternatingLateralRaiseWithStaticHold,
    BarMuscleUp,
    BentOverLateralRaise,
    CableDiagonalRaise,
    CableFrontRaise,
    CalorieRow,
    ComboShoulderRaise,
    DumbbellDiagonalRaise,
    DumbbellVRaise,
    FrontRaise,
    LeaningDumbbellLateralRaise,
    LyingDumbbellRaise,
    MuscleUp,
    OneArmCableLateralRaise,
    OverhandGripRearLateralRaise,
    PlateRaises,
    RingDip,
    WeightedRingDip,
    RingMuscleUp,
    WeightedRingMuscleUp,
    RopeClimb,
    WeightedRopeClimb,
    Scaption,
    SeatedLateralRaise,
    SeatedRearLateralRaise,
    SideLyingLateralRaise,
    StandingLift,
    SuspendedRow,
    UnderhandGripRearLateralRaise,
    WallSlide,
    WeightedWallSlide,
    ArmCircles,
    ShavingTheHead,
    UnknownVariant(u16),
}
impl LateralRaiseExerciseName {
    pub fn as_u16(self) -> u16 {
        match self {
            LateralRaiseExerciseName::Name45DegreeCableExternalRotation => 0,
            LateralRaiseExerciseName::AlternatingLateralRaiseWithStaticHold => 1,
            LateralRaiseExerciseName::BarMuscleUp => 2,
            LateralRaiseExerciseName::BentOverLateralRaise => 3,
            LateralRaiseExerciseName::CableDiagonalRaise => 4,
            LateralRaiseExerciseName::CableFrontRaise => 5,
            LateralRaiseExerciseName::CalorieRow => 6,
            LateralRaiseExerciseName::ComboShoulderRaise => 7,
            LateralRaiseExerciseName::DumbbellDiagonalRaise => 8,
            LateralRaiseExerciseName::DumbbellVRaise => 9,
            LateralRaiseExerciseName::FrontRaise => 10,
            LateralRaiseExerciseName::LeaningDumbbellLateralRaise => 11,
            LateralRaiseExerciseName::LyingDumbbellRaise => 12,
            LateralRaiseExerciseName::MuscleUp => 13,
            LateralRaiseExerciseName::OneArmCableLateralRaise => 14,
            LateralRaiseExerciseName::OverhandGripRearLateralRaise => 15,
            LateralRaiseExerciseName::PlateRaises => 16,
            LateralRaiseExerciseName::RingDip => 17,
            LateralRaiseExerciseName::WeightedRingDip => 18,
            LateralRaiseExerciseName::RingMuscleUp => 19,
            LateralRaiseExerciseName::WeightedRingMuscleUp => 20,
            LateralRaiseExerciseName::RopeClimb => 21,
            LateralRaiseExerciseName::WeightedRopeClimb => 22,
            LateralRaiseExerciseName::Scaption => 23,
            LateralRaiseExerciseName::SeatedLateralRaise => 24,
            LateralRaiseExerciseName::SeatedRearLateralRaise => 25,
            LateralRaiseExerciseName::SideLyingLateralRaise => 26,
            LateralRaiseExerciseName::StandingLift => 27,
            LateralRaiseExerciseName::SuspendedRow => 28,
            LateralRaiseExerciseName::UnderhandGripRearLateralRaise => 29,
            LateralRaiseExerciseName::WallSlide => 30,
            LateralRaiseExerciseName::WeightedWallSlide => 31,
            LateralRaiseExerciseName::ArmCircles => 32,
            LateralRaiseExerciseName::ShavingTheHead => 33,
            LateralRaiseExerciseName::UnknownVariant(value) => value,
        }
    }
    pub fn as_i64(self) -> i64 {
        self.as_u16() as i64
    }
}
impl fmt::Display for LateralRaiseExerciseName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            LateralRaiseExerciseName::Name45DegreeCableExternalRotation => {
                writeln!(f, "45_degree_cable_external_rotation")
            }
            LateralRaiseExerciseName::AlternatingLateralRaiseWithStaticHold => {
                writeln!(f, "alternating_lateral_raise_with_static_hold")
            }
            LateralRaiseExerciseName::BarMuscleUp => writeln!(f, "bar_muscle_up"),
            LateralRaiseExerciseName::BentOverLateralRaise => {
                writeln!(f, "bent_over_lateral_raise")
            }
            LateralRaiseExerciseName::CableDiagonalRaise => writeln!(f, "cable_diagonal_raise"),
            LateralRaiseExerciseName::CableFrontRaise => writeln!(f, "cable_front_raise"),
            LateralRaiseExerciseName::CalorieRow => writeln!(f, "calorie_row"),
            LateralRaiseExerciseName::ComboShoulderRaise => writeln!(f, "combo_shoulder_raise"),
            LateralRaiseExerciseName::DumbbellDiagonalRaise => {
                writeln!(f, "dumbbell_diagonal_raise")
            }
            LateralRaiseExerciseName::DumbbellVRaise => writeln!(f, "dumbbell_v_raise"),
            LateralRaiseExerciseName::FrontRaise => writeln!(f, "front_raise"),
            LateralRaiseExerciseName::LeaningDumbbellLateralRaise => {
                writeln!(f, "leaning_dumbbell_lateral_raise")
            }
            LateralRaiseExerciseName::LyingDumbbellRaise => writeln!(f, "lying_dumbbell_raise"),
            LateralRaiseExerciseName::MuscleUp => writeln!(f, "muscle_up"),
            LateralRaiseExerciseName::OneArmCableLateralRaise => {
                writeln!(f, "one_arm_cable_lateral_raise")
            }
            LateralRaiseExerciseName::OverhandGripRearLateralRaise => {
                writeln!(f, "overhand_grip_rear_lateral_raise")
            }
            LateralRaiseExerciseName::PlateRaises => writeln!(f, "plate_raises"),
            LateralRaiseExerciseName::RingDip => writeln!(f, "ring_dip"),
            LateralRaiseExerciseName::WeightedRingDip => writeln!(f, "weighted_ring_dip"),
            LateralRaiseExerciseName::RingMuscleUp => writeln!(f, "ring_muscle_up"),
            LateralRaiseExerciseName::WeightedRingMuscleUp => {
                writeln!(f, "weighted_ring_muscle_up")
            }
            LateralRaiseExerciseName::RopeClimb => writeln!(f, "rope_climb"),
            LateralRaiseExerciseName::WeightedRopeClimb => writeln!(f, "weighted_rope_climb"),
            LateralRaiseExerciseName::Scaption => writeln!(f, "scaption"),
            LateralRaiseExerciseName::SeatedLateralRaise => writeln!(f, "seated_lateral_raise"),
            LateralRaiseExerciseName::SeatedRearLateralRaise => {
                writeln!(f, "seated_rear_lateral_raise")
            }
            LateralRaiseExerciseName::SideLyingLateralRaise => {
                writeln!(f, "side_lying_lateral_raise")
            }
            LateralRaiseExerciseName::StandingLift => writeln!(f, "standing_lift"),
            LateralRaiseExerciseName::SuspendedRow => writeln!(f, "suspended_row"),
            LateralRaiseExerciseName::UnderhandGripRearLateralRaise => {
                writeln!(f, "underhand_grip_rear_lateral_raise")
            }
            LateralRaiseExerciseName::WallSlide => writeln!(f, "wall_slide"),
            LateralRaiseExerciseName::WeightedWallSlide => writeln!(f, "weighted_wall_slide"),
            LateralRaiseExerciseName::ArmCircles => writeln!(f, "arm_circles"),
            LateralRaiseExerciseName::ShavingTheHead => writeln!(f, "shaving_the_head"),
            LateralRaiseExerciseName::UnknownVariant(value) => {
                writeln!(f, "unknown_variant_{}", *value)
            }
        }
    }
}
impl convert::From<u16> for LateralRaiseExerciseName {
    fn from(value: u16) -> Self {
        match value {
            0 => LateralRaiseExerciseName::Name45DegreeCableExternalRotation,
            1 => LateralRaiseExerciseName::AlternatingLateralRaiseWithStaticHold,
            2 => LateralRaiseExerciseName::BarMuscleUp,
            3 => LateralRaiseExerciseName::BentOverLateralRaise,
            4 => LateralRaiseExerciseName::CableDiagonalRaise,
            5 => LateralRaiseExerciseName::CableFrontRaise,
            6 => LateralRaiseExerciseName::CalorieRow,
            7 => LateralRaiseExerciseName::ComboShoulderRaise,
            8 => LateralRaiseExerciseName::DumbbellDiagonalRaise,
            9 => LateralRaiseExerciseName::DumbbellVRaise,
            10 => LateralRaiseExerciseName::FrontRaise,
            11 => LateralRaiseExerciseName::LeaningDumbbellLateralRaise,
            12 => LateralRaiseExerciseName::LyingDumbbellRaise,
            13 => LateralRaiseExerciseName::MuscleUp,
            14 => LateralRaiseExerciseName::OneArmCableLateralRaise,
            15 => LateralRaiseExerciseName::OverhandGripRearLateralRaise,
            16 => LateralRaiseExerciseName::PlateRaises,
            17 => LateralRaiseExerciseName::RingDip,
            18 => LateralRaiseExerciseName::WeightedRingDip,
            19 => LateralRaiseExerciseName::RingMuscleUp,
            20 => LateralRaiseExerciseName::WeightedRingMuscleUp,
            21 => LateralRaiseExerciseName::RopeClimb,
            22 => LateralRaiseExerciseName::WeightedRopeClimb,
            23 => LateralRaiseExerciseName::Scaption,
            24 => LateralRaiseExerciseName::SeatedLateralRaise,
            25 => LateralRaiseExerciseName::SeatedRearLateralRaise,
            26 => LateralRaiseExerciseName::SideLyingLateralRaise,
            27 => LateralRaiseExerciseName::StandingLift,
            28 => LateralRaiseExerciseName::SuspendedRow,
            29 => LateralRaiseExerciseName::UnderhandGripRearLateralRaise,
            30 => LateralRaiseExerciseName::WallSlide,
            31 => LateralRaiseExerciseName::WeightedWallSlide,
            32 => LateralRaiseExerciseName::ArmCircles,
            33 => LateralRaiseExerciseName::ShavingTheHead,
            _ => LateralRaiseExerciseName::UnknownVariant(value),
        }
    }
}
impl convert::From<i64> for LateralRaiseExerciseName {
    fn from(value: i64) -> Self {
        LateralRaiseExerciseName::from(value as u16)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum LegCurlExerciseName {
    LegCurl,
    WeightedLegCurl,
    GoodMorning,
    SeatedBarbellGoodMorning,
    SingleLegBarbellGoodMorning,
    SingleLegSlidingLegCurl,
    SlidingLegCurl,
    SplitBarbellGoodMorning,
    SplitStanceExtension,
    StaggeredStanceGoodMorning,
    SwissBallHipRaiseAndLegCurl,
    ZercherGoodMorning,
    UnknownVariant(u16),
}
impl LegCurlExerciseName {
    pub fn as_u16(self) -> u16 {
        match self {
            LegCurlExerciseName::LegCurl => 0,
            LegCurlExerciseName::WeightedLegCurl => 1,
            LegCurlExerciseName::GoodMorning => 2,
            LegCurlExerciseName::SeatedBarbellGoodMorning => 3,
            LegCurlExerciseName::SingleLegBarbellGoodMorning => 4,
            LegCurlExerciseName::SingleLegSlidingLegCurl => 5,
            LegCurlExerciseName::SlidingLegCurl => 6,
            LegCurlExerciseName::SplitBarbellGoodMorning => 7,
            LegCurlExerciseName::SplitStanceExtension => 8,
            LegCurlExerciseName::StaggeredStanceGoodMorning => 9,
            LegCurlExerciseName::SwissBallHipRaiseAndLegCurl => 10,
            LegCurlExerciseName::ZercherGoodMorning => 11,
            LegCurlExerciseName::UnknownVariant(value) => value,
        }
    }
    pub fn as_i64(self) -> i64 {
        self.as_u16() as i64
    }
}
impl fmt::Display for LegCurlExerciseName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            LegCurlExerciseName::LegCurl => writeln!(f, "leg_curl"),
            LegCurlExerciseName::WeightedLegCurl => writeln!(f, "weighted_leg_curl"),
            LegCurlExerciseName::GoodMorning => writeln!(f, "good_morning"),
            LegCurlExerciseName::SeatedBarbellGoodMorning => {
                writeln!(f, "seated_barbell_good_morning")
            }
            LegCurlExerciseName::SingleLegBarbellGoodMorning => {
                writeln!(f, "single_leg_barbell_good_morning")
            }
            LegCurlExerciseName::SingleLegSlidingLegCurl => {
                writeln!(f, "single_leg_sliding_leg_curl")
            }
            LegCurlExerciseName::SlidingLegCurl => writeln!(f, "sliding_leg_curl"),
            LegCurlExerciseName::SplitBarbellGoodMorning => {
                writeln!(f, "split_barbell_good_morning")
            }
            LegCurlExerciseName::SplitStanceExtension => writeln!(f, "split_stance_extension"),
            LegCurlExerciseName::StaggeredStanceGoodMorning => {
                writeln!(f, "staggered_stance_good_morning")
            }
            LegCurlExerciseName::SwissBallHipRaiseAndLegCurl => {
                writeln!(f, "swiss_ball_hip_raise_and_leg_curl")
            }
            LegCurlExerciseName::ZercherGoodMorning => writeln!(f, "zercher_good_morning"),
            LegCurlExerciseName::UnknownVariant(value) => writeln!(f, "unknown_variant_{}", *value),
        }
    }
}
impl convert::From<u16> for LegCurlExerciseName {
    fn from(value: u16) -> Self {
        match value {
            0 => LegCurlExerciseName::LegCurl,
            1 => LegCurlExerciseName::WeightedLegCurl,
            2 => LegCurlExerciseName::GoodMorning,
            3 => LegCurlExerciseName::SeatedBarbellGoodMorning,
            4 => LegCurlExerciseName::SingleLegBarbellGoodMorning,
            5 => LegCurlExerciseName::SingleLegSlidingLegCurl,
            6 => LegCurlExerciseName::SlidingLegCurl,
            7 => LegCurlExerciseName::SplitBarbellGoodMorning,
            8 => LegCurlExerciseName::SplitStanceExtension,
            9 => LegCurlExerciseName::StaggeredStanceGoodMorning,
            10 => LegCurlExerciseName::SwissBallHipRaiseAndLegCurl,
            11 => LegCurlExerciseName::ZercherGoodMorning,
            _ => LegCurlExerciseName::UnknownVariant(value),
        }
    }
}
impl convert::From<i64> for LegCurlExerciseName {
    fn from(value: i64) -> Self {
        LegCurlExerciseName::from(value as u16)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum LegRaiseExerciseName {
    HangingKneeRaise,
    HangingLegRaise,
    WeightedHangingLegRaise,
    HangingSingleLegRaise,
    WeightedHangingSingleLegRaise,
    KettlebellLegRaises,
    LegLoweringDrill,
    WeightedLegLoweringDrill,
    LyingStraightLegRaise,
    WeightedLyingStraightLegRaise,
    MedicineBallLegDrops,
    QuadrupedLegRaise,
    WeightedQuadrupedLegRaise,
    ReverseLegRaise,
    WeightedReverseLegRaise,
    ReverseLegRaiseOnSwissBall,
    WeightedReverseLegRaiseOnSwissBall,
    SingleLegLoweringDrill,
    WeightedSingleLegLoweringDrill,
    WeightedHangingKneeRaise,
    LateralStepover,
    WeightedLateralStepover,
    UnknownVariant(u16),
}
impl LegRaiseExerciseName {
    pub fn as_u16(self) -> u16 {
        match self {
            LegRaiseExerciseName::HangingKneeRaise => 0,
            LegRaiseExerciseName::HangingLegRaise => 1,
            LegRaiseExerciseName::WeightedHangingLegRaise => 2,
            LegRaiseExerciseName::HangingSingleLegRaise => 3,
            LegRaiseExerciseName::WeightedHangingSingleLegRaise => 4,
            LegRaiseExerciseName::KettlebellLegRaises => 5,
            LegRaiseExerciseName::LegLoweringDrill => 6,
            LegRaiseExerciseName::WeightedLegLoweringDrill => 7,
            LegRaiseExerciseName::LyingStraightLegRaise => 8,
            LegRaiseExerciseName::WeightedLyingStraightLegRaise => 9,
            LegRaiseExerciseName::MedicineBallLegDrops => 10,
            LegRaiseExerciseName::QuadrupedLegRaise => 11,
            LegRaiseExerciseName::WeightedQuadrupedLegRaise => 12,
            LegRaiseExerciseName::ReverseLegRaise => 13,
            LegRaiseExerciseName::WeightedReverseLegRaise => 14,
            LegRaiseExerciseName::ReverseLegRaiseOnSwissBall => 15,
            LegRaiseExerciseName::WeightedReverseLegRaiseOnSwissBall => 16,
            LegRaiseExerciseName::SingleLegLoweringDrill => 17,
            LegRaiseExerciseName::WeightedSingleLegLoweringDrill => 18,
            LegRaiseExerciseName::WeightedHangingKneeRaise => 19,
            LegRaiseExerciseName::LateralStepover => 20,
            LegRaiseExerciseName::WeightedLateralStepover => 21,
            LegRaiseExerciseName::UnknownVariant(value) => value,
        }
    }
    pub fn as_i64(self) -> i64 {
        self.as_u16() as i64
    }
}
impl fmt::Display for LegRaiseExerciseName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            LegRaiseExerciseName::HangingKneeRaise => writeln!(f, "hanging_knee_raise"),
            LegRaiseExerciseName::HangingLegRaise => writeln!(f, "hanging_leg_raise"),
            LegRaiseExerciseName::WeightedHangingLegRaise => {
                writeln!(f, "weighted_hanging_leg_raise")
            }
            LegRaiseExerciseName::HangingSingleLegRaise => writeln!(f, "hanging_single_leg_raise"),
            LegRaiseExerciseName::WeightedHangingSingleLegRaise => {
                writeln!(f, "weighted_hanging_single_leg_raise")
            }
            LegRaiseExerciseName::KettlebellLegRaises => writeln!(f, "kettlebell_leg_raises"),
            LegRaiseExerciseName::LegLoweringDrill => writeln!(f, "leg_lowering_drill"),
            LegRaiseExerciseName::WeightedLegLoweringDrill => {
                writeln!(f, "weighted_leg_lowering_drill")
            }
            LegRaiseExerciseName::LyingStraightLegRaise => writeln!(f, "lying_straight_leg_raise"),
            LegRaiseExerciseName::WeightedLyingStraightLegRaise => {
                writeln!(f, "weighted_lying_straight_leg_raise")
            }
            LegRaiseExerciseName::MedicineBallLegDrops => writeln!(f, "medicine_ball_leg_drops"),
            LegRaiseExerciseName::QuadrupedLegRaise => writeln!(f, "quadruped_leg_raise"),
            LegRaiseExerciseName::WeightedQuadrupedLegRaise => {
                writeln!(f, "weighted_quadruped_leg_raise")
            }
            LegRaiseExerciseName::ReverseLegRaise => writeln!(f, "reverse_leg_raise"),
            LegRaiseExerciseName::WeightedReverseLegRaise => {
                writeln!(f, "weighted_reverse_leg_raise")
            }
            LegRaiseExerciseName::ReverseLegRaiseOnSwissBall => {
                writeln!(f, "reverse_leg_raise_on_swiss_ball")
            }
            LegRaiseExerciseName::WeightedReverseLegRaiseOnSwissBall => {
                writeln!(f, "weighted_reverse_leg_raise_on_swiss_ball")
            }
            LegRaiseExerciseName::SingleLegLoweringDrill => {
                writeln!(f, "single_leg_lowering_drill")
            }
            LegRaiseExerciseName::WeightedSingleLegLoweringDrill => {
                writeln!(f, "weighted_single_leg_lowering_drill")
            }
            LegRaiseExerciseName::WeightedHangingKneeRaise => {
                writeln!(f, "weighted_hanging_knee_raise")
            }
            LegRaiseExerciseName::LateralStepover => writeln!(f, "lateral_stepover"),
            LegRaiseExerciseName::WeightedLateralStepover => {
                writeln!(f, "weighted_lateral_stepover")
            }
            LegRaiseExerciseName::UnknownVariant(value) => {
                writeln!(f, "unknown_variant_{}", *value)
            }
        }
    }
}
impl convert::From<u16> for LegRaiseExerciseName {
    fn from(value: u16) -> Self {
        match value {
            0 => LegRaiseExerciseName::HangingKneeRaise,
            1 => LegRaiseExerciseName::HangingLegRaise,
            2 => LegRaiseExerciseName::WeightedHangingLegRaise,
            3 => LegRaiseExerciseName::HangingSingleLegRaise,
            4 => LegRaiseExerciseName::WeightedHangingSingleLegRaise,
            5 => LegRaiseExerciseName::KettlebellLegRaises,
            6 => LegRaiseExerciseName::LegLoweringDrill,
            7 => LegRaiseExerciseName::WeightedLegLoweringDrill,
            8 => LegRaiseExerciseName::LyingStraightLegRaise,
            9 => LegRaiseExerciseName::WeightedLyingStraightLegRaise,
            10 => LegRaiseExerciseName::MedicineBallLegDrops,
            11 => LegRaiseExerciseName::QuadrupedLegRaise,
            12 => LegRaiseExerciseName::WeightedQuadrupedLegRaise,
            13 => LegRaiseExerciseName::ReverseLegRaise,
            14 => LegRaiseExerciseName::WeightedReverseLegRaise,
            15 => LegRaiseExerciseName::ReverseLegRaiseOnSwissBall,
            16 => LegRaiseExerciseName::WeightedReverseLegRaiseOnSwissBall,
            17 => LegRaiseExerciseName::SingleLegLoweringDrill,
            18 => LegRaiseExerciseName::WeightedSingleLegLoweringDrill,
            19 => LegRaiseExerciseName::WeightedHangingKneeRaise,
            20 => LegRaiseExerciseName::LateralStepover,
            21 => LegRaiseExerciseName::WeightedLateralStepover,
            _ => LegRaiseExerciseName::UnknownVariant(value),
        }
    }
}
impl convert::From<i64> for LegRaiseExerciseName {
    fn from(value: i64) -> Self {
        LegRaiseExerciseName::from(value as u16)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum LungeExerciseName {
    OverheadLunge,
    LungeMatrix,
    WeightedLungeMatrix,
    AlternatingBarbellForwardLunge,
    AlternatingDumbbellLungeWithReach,
    BackFootElevatedDumbbellSplitSquat,
    BarbellBoxLunge,
    BarbellBulgarianSplitSquat,
    BarbellCrossoverLunge,
    BarbellFrontSplitSquat,
    BarbellLunge,
    BarbellReverseLunge,
    BarbellSideLunge,
    BarbellSplitSquat,
    CoreControlRearLunge,
    DiagonalLunge,
    DropLunge,
    DumbbellBoxLunge,
    DumbbellBulgarianSplitSquat,
    DumbbellCrossoverLunge,
    DumbbellDiagonalLunge,
    DumbbellLunge,
    DumbbellLungeAndRotation,
    DumbbellOverheadBulgarianSplitSquat,
    DumbbellReverseLungeToHighKneeAndPress,
    DumbbellSideLunge,
    ElevatedFrontFootBarbellSplitSquat,
    FrontFootElevatedDumbbellSplitSquat,
    GunslingerLunge,
    LawnmowerLunge,
    LowLungeWithIsometricAdduction,
    LowSideToSideLunge,
    Lunge,
    WeightedLunge,
    LungeWithArmReach,
    LungeWithDiagonalReach,
    LungeWithSideBend,
    OffsetDumbbellLunge,
    OffsetDumbbellReverseLunge,
    OverheadBulgarianSplitSquat,
    OverheadDumbbellReverseLunge,
    OverheadDumbbellSplitSquat,
    OverheadLungeWithRotation,
    ReverseBarbellBoxLunge,
    ReverseBoxLunge,
    ReverseDumbbellBoxLunge,
    ReverseDumbbellCrossoverLunge,
    ReverseDumbbellDiagonalLunge,
    ReverseLungeWithReachBack,
    WeightedReverseLungeWithReachBack,
    ReverseLungeWithTwistAndOverheadReach,
    WeightedReverseLungeWithTwistAndOverheadReach,
    ReverseSlidingBoxLunge,
    WeightedReverseSlidingBoxLunge,
    ReverseSlidingLunge,
    WeightedReverseSlidingLunge,
    RunnersLungeToBalance,
    WeightedRunnersLungeToBalance,
    ShiftingSideLunge,
    SideAndCrossoverLunge,
    WeightedSideAndCrossoverLunge,
    SideLunge,
    WeightedSideLunge,
    SideLungeAndPress,
    SideLungeJumpOff,
    SideLungeSweep,
    WeightedSideLungeSweep,
    SideLungeToCrossoverTap,
    WeightedSideLungeToCrossoverTap,
    SideToSideLungeChops,
    WeightedSideToSideLungeChops,
    SiffJumpLunge,
    WeightedSiffJumpLunge,
    SingleArmReverseLungeAndPress,
    SlidingLateralLunge,
    WeightedSlidingLateralLunge,
    WalkingBarbellLunge,
    WalkingDumbbellLunge,
    WalkingLunge,
    WeightedWalkingLunge,
    WideGripOverheadBarbellSplitSquat,
    UnknownVariant(u16),
}
impl LungeExerciseName {
    pub fn as_u16(self) -> u16 {
        match self {
            LungeExerciseName::OverheadLunge => 0,
            LungeExerciseName::LungeMatrix => 1,
            LungeExerciseName::WeightedLungeMatrix => 2,
            LungeExerciseName::AlternatingBarbellForwardLunge => 3,
            LungeExerciseName::AlternatingDumbbellLungeWithReach => 4,
            LungeExerciseName::BackFootElevatedDumbbellSplitSquat => 5,
            LungeExerciseName::BarbellBoxLunge => 6,
            LungeExerciseName::BarbellBulgarianSplitSquat => 7,
            LungeExerciseName::BarbellCrossoverLunge => 8,
            LungeExerciseName::BarbellFrontSplitSquat => 9,
            LungeExerciseName::BarbellLunge => 10,
            LungeExerciseName::BarbellReverseLunge => 11,
            LungeExerciseName::BarbellSideLunge => 12,
            LungeExerciseName::BarbellSplitSquat => 13,
            LungeExerciseName::CoreControlRearLunge => 14,
            LungeExerciseName::DiagonalLunge => 15,
            LungeExerciseName::DropLunge => 16,
            LungeExerciseName::DumbbellBoxLunge => 17,
            LungeExerciseName::DumbbellBulgarianSplitSquat => 18,
            LungeExerciseName::DumbbellCrossoverLunge => 19,
            LungeExerciseName::DumbbellDiagonalLunge => 20,
            LungeExerciseName::DumbbellLunge => 21,
            LungeExerciseName::DumbbellLungeAndRotation => 22,
            LungeExerciseName::DumbbellOverheadBulgarianSplitSquat => 23,
            LungeExerciseName::DumbbellReverseLungeToHighKneeAndPress => 24,
            LungeExerciseName::DumbbellSideLunge => 25,
            LungeExerciseName::ElevatedFrontFootBarbellSplitSquat => 26,
            LungeExerciseName::FrontFootElevatedDumbbellSplitSquat => 27,
            LungeExerciseName::GunslingerLunge => 28,
            LungeExerciseName::LawnmowerLunge => 29,
            LungeExerciseName::LowLungeWithIsometricAdduction => 30,
            LungeExerciseName::LowSideToSideLunge => 31,
            LungeExerciseName::Lunge => 32,
            LungeExerciseName::WeightedLunge => 33,
            LungeExerciseName::LungeWithArmReach => 34,
            LungeExerciseName::LungeWithDiagonalReach => 35,
            LungeExerciseName::LungeWithSideBend => 36,
            LungeExerciseName::OffsetDumbbellLunge => 37,
            LungeExerciseName::OffsetDumbbellReverseLunge => 38,
            LungeExerciseName::OverheadBulgarianSplitSquat => 39,
            LungeExerciseName::OverheadDumbbellReverseLunge => 40,
            LungeExerciseName::OverheadDumbbellSplitSquat => 41,
            LungeExerciseName::OverheadLungeWithRotation => 42,
            LungeExerciseName::ReverseBarbellBoxLunge => 43,
            LungeExerciseName::ReverseBoxLunge => 44,
            LungeExerciseName::ReverseDumbbellBoxLunge => 45,
            LungeExerciseName::ReverseDumbbellCrossoverLunge => 46,
            LungeExerciseName::ReverseDumbbellDiagonalLunge => 47,
            LungeExerciseName::ReverseLungeWithReachBack => 48,
            LungeExerciseName::WeightedReverseLungeWithReachBack => 49,
            LungeExerciseName::ReverseLungeWithTwistAndOverheadReach => 50,
            LungeExerciseName::WeightedReverseLungeWithTwistAndOverheadReach => 51,
            LungeExerciseName::ReverseSlidingBoxLunge => 52,
            LungeExerciseName::WeightedReverseSlidingBoxLunge => 53,
            LungeExerciseName::ReverseSlidingLunge => 54,
            LungeExerciseName::WeightedReverseSlidingLunge => 55,
            LungeExerciseName::RunnersLungeToBalance => 56,
            LungeExerciseName::WeightedRunnersLungeToBalance => 57,
            LungeExerciseName::ShiftingSideLunge => 58,
            LungeExerciseName::SideAndCrossoverLunge => 59,
            LungeExerciseName::WeightedSideAndCrossoverLunge => 60,
            LungeExerciseName::SideLunge => 61,
            LungeExerciseName::WeightedSideLunge => 62,
            LungeExerciseName::SideLungeAndPress => 63,
            LungeExerciseName::SideLungeJumpOff => 64,
            LungeExerciseName::SideLungeSweep => 65,
            LungeExerciseName::WeightedSideLungeSweep => 66,
            LungeExerciseName::SideLungeToCrossoverTap => 67,
            LungeExerciseName::WeightedSideLungeToCrossoverTap => 68,
            LungeExerciseName::SideToSideLungeChops => 69,
            LungeExerciseName::WeightedSideToSideLungeChops => 70,
            LungeExerciseName::SiffJumpLunge => 71,
            LungeExerciseName::WeightedSiffJumpLunge => 72,
            LungeExerciseName::SingleArmReverseLungeAndPress => 73,
            LungeExerciseName::SlidingLateralLunge => 74,
            LungeExerciseName::WeightedSlidingLateralLunge => 75,
            LungeExerciseName::WalkingBarbellLunge => 76,
            LungeExerciseName::WalkingDumbbellLunge => 77,
            LungeExerciseName::WalkingLunge => 78,
            LungeExerciseName::WeightedWalkingLunge => 79,
            LungeExerciseName::WideGripOverheadBarbellSplitSquat => 80,
            LungeExerciseName::UnknownVariant(value) => value,
        }
    }
    pub fn as_i64(self) -> i64 {
        self.as_u16() as i64
    }
}
impl fmt::Display for LungeExerciseName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            LungeExerciseName::OverheadLunge => writeln!(f, "overhead_lunge"),
            LungeExerciseName::LungeMatrix => writeln!(f, "lunge_matrix"),
            LungeExerciseName::WeightedLungeMatrix => writeln!(f, "weighted_lunge_matrix"),
            LungeExerciseName::AlternatingBarbellForwardLunge => {
                writeln!(f, "alternating_barbell_forward_lunge")
            }
            LungeExerciseName::AlternatingDumbbellLungeWithReach => {
                writeln!(f, "alternating_dumbbell_lunge_with_reach")
            }
            LungeExerciseName::BackFootElevatedDumbbellSplitSquat => {
                writeln!(f, "back_foot_elevated_dumbbell_split_squat")
            }
            LungeExerciseName::BarbellBoxLunge => writeln!(f, "barbell_box_lunge"),
            LungeExerciseName::BarbellBulgarianSplitSquat => {
                writeln!(f, "barbell_bulgarian_split_squat")
            }
            LungeExerciseName::BarbellCrossoverLunge => writeln!(f, "barbell_crossover_lunge"),
            LungeExerciseName::BarbellFrontSplitSquat => writeln!(f, "barbell_front_split_squat"),
            LungeExerciseName::BarbellLunge => writeln!(f, "barbell_lunge"),
            LungeExerciseName::BarbellReverseLunge => writeln!(f, "barbell_reverse_lunge"),
            LungeExerciseName::BarbellSideLunge => writeln!(f, "barbell_side_lunge"),
            LungeExerciseName::BarbellSplitSquat => writeln!(f, "barbell_split_squat"),
            LungeExerciseName::CoreControlRearLunge => writeln!(f, "core_control_rear_lunge"),
            LungeExerciseName::DiagonalLunge => writeln!(f, "diagonal_lunge"),
            LungeExerciseName::DropLunge => writeln!(f, "drop_lunge"),
            LungeExerciseName::DumbbellBoxLunge => writeln!(f, "dumbbell_box_lunge"),
            LungeExerciseName::DumbbellBulgarianSplitSquat => {
                writeln!(f, "dumbbell_bulgarian_split_squat")
            }
            LungeExerciseName::DumbbellCrossoverLunge => writeln!(f, "dumbbell_crossover_lunge"),
            LungeExerciseName::DumbbellDiagonalLunge => writeln!(f, "dumbbell_diagonal_lunge"),
            LungeExerciseName::DumbbellLunge => writeln!(f, "dumbbell_lunge"),
            LungeExerciseName::DumbbellLungeAndRotation => {
                writeln!(f, "dumbbell_lunge_and_rotation")
            }
            LungeExerciseName::DumbbellOverheadBulgarianSplitSquat => {
                writeln!(f, "dumbbell_overhead_bulgarian_split_squat")
            }
            LungeExerciseName::DumbbellReverseLungeToHighKneeAndPress => {
                writeln!(f, "dumbbell_reverse_lunge_to_high_knee_and_press")
            }
            LungeExerciseName::DumbbellSideLunge => writeln!(f, "dumbbell_side_lunge"),
            LungeExerciseName::ElevatedFrontFootBarbellSplitSquat => {
                writeln!(f, "elevated_front_foot_barbell_split_squat")
            }
            LungeExerciseName::FrontFootElevatedDumbbellSplitSquat => {
                writeln!(f, "front_foot_elevated_dumbbell_split_squat")
            }
            LungeExerciseName::GunslingerLunge => writeln!(f, "gunslinger_lunge"),
            LungeExerciseName::LawnmowerLunge => writeln!(f, "lawnmower_lunge"),
            LungeExerciseName::LowLungeWithIsometricAdduction => {
                writeln!(f, "low_lunge_with_isometric_adduction")
            }
            LungeExerciseName::LowSideToSideLunge => writeln!(f, "low_side_to_side_lunge"),
            LungeExerciseName::Lunge => writeln!(f, "lunge"),
            LungeExerciseName::WeightedLunge => writeln!(f, "weighted_lunge"),
            LungeExerciseName::LungeWithArmReach => writeln!(f, "lunge_with_arm_reach"),
            LungeExerciseName::LungeWithDiagonalReach => writeln!(f, "lunge_with_diagonal_reach"),
            LungeExerciseName::LungeWithSideBend => writeln!(f, "lunge_with_side_bend"),
            LungeExerciseName::OffsetDumbbellLunge => writeln!(f, "offset_dumbbell_lunge"),
            LungeExerciseName::OffsetDumbbellReverseLunge => {
                writeln!(f, "offset_dumbbell_reverse_lunge")
            }
            LungeExerciseName::OverheadBulgarianSplitSquat => {
                writeln!(f, "overhead_bulgarian_split_squat")
            }
            LungeExerciseName::OverheadDumbbellReverseLunge => {
                writeln!(f, "overhead_dumbbell_reverse_lunge")
            }
            LungeExerciseName::OverheadDumbbellSplitSquat => {
                writeln!(f, "overhead_dumbbell_split_squat")
            }
            LungeExerciseName::OverheadLungeWithRotation => {
                writeln!(f, "overhead_lunge_with_rotation")
            }
            LungeExerciseName::ReverseBarbellBoxLunge => writeln!(f, "reverse_barbell_box_lunge"),
            LungeExerciseName::ReverseBoxLunge => writeln!(f, "reverse_box_lunge"),
            LungeExerciseName::ReverseDumbbellBoxLunge => writeln!(f, "reverse_dumbbell_box_lunge"),
            LungeExerciseName::ReverseDumbbellCrossoverLunge => {
                writeln!(f, "reverse_dumbbell_crossover_lunge")
            }
            LungeExerciseName::ReverseDumbbellDiagonalLunge => {
                writeln!(f, "reverse_dumbbell_diagonal_lunge")
            }
            LungeExerciseName::ReverseLungeWithReachBack => {
                writeln!(f, "reverse_lunge_with_reach_back")
            }
            LungeExerciseName::WeightedReverseLungeWithReachBack => {
                writeln!(f, "weighted_reverse_lunge_with_reach_back")
            }
            LungeExerciseName::ReverseLungeWithTwistAndOverheadReach => {
                writeln!(f, "reverse_lunge_with_twist_and_overhead_reach")
            }
            LungeExerciseName::WeightedReverseLungeWithTwistAndOverheadReach => {
                writeln!(f, "weighted_reverse_lunge_with_twist_and_overhead_reach")
            }
            LungeExerciseName::ReverseSlidingBoxLunge => writeln!(f, "reverse_sliding_box_lunge"),
            LungeExerciseName::WeightedReverseSlidingBoxLunge => {
                writeln!(f, "weighted_reverse_sliding_box_lunge")
            }
            LungeExerciseName::ReverseSlidingLunge => writeln!(f, "reverse_sliding_lunge"),
            LungeExerciseName::WeightedReverseSlidingLunge => {
                writeln!(f, "weighted_reverse_sliding_lunge")
            }
            LungeExerciseName::RunnersLungeToBalance => writeln!(f, "runners_lunge_to_balance"),
            LungeExerciseName::WeightedRunnersLungeToBalance => {
                writeln!(f, "weighted_runners_lunge_to_balance")
            }
            LungeExerciseName::ShiftingSideLunge => writeln!(f, "shifting_side_lunge"),
            LungeExerciseName::SideAndCrossoverLunge => writeln!(f, "side_and_crossover_lunge"),
            LungeExerciseName::WeightedSideAndCrossoverLunge => {
                writeln!(f, "weighted_side_and_crossover_lunge")
            }
            LungeExerciseName::SideLunge => writeln!(f, "side_lunge"),
            LungeExerciseName::WeightedSideLunge => writeln!(f, "weighted_side_lunge"),
            LungeExerciseName::SideLungeAndPress => writeln!(f, "side_lunge_and_press"),
            LungeExerciseName::SideLungeJumpOff => writeln!(f, "side_lunge_jump_off"),
            LungeExerciseName::SideLungeSweep => writeln!(f, "side_lunge_sweep"),
            LungeExerciseName::WeightedSideLungeSweep => writeln!(f, "weighted_side_lunge_sweep"),
            LungeExerciseName::SideLungeToCrossoverTap => {
                writeln!(f, "side_lunge_to_crossover_tap")
            }
            LungeExerciseName::WeightedSideLungeToCrossoverTap => {
                writeln!(f, "weighted_side_lunge_to_crossover_tap")
            }
            LungeExerciseName::SideToSideLungeChops => writeln!(f, "side_to_side_lunge_chops"),
            LungeExerciseName::WeightedSideToSideLungeChops => {
                writeln!(f, "weighted_side_to_side_lunge_chops")
            }
            LungeExerciseName::SiffJumpLunge => writeln!(f, "siff_jump_lunge"),
            LungeExerciseName::WeightedSiffJumpLunge => writeln!(f, "weighted_siff_jump_lunge"),
            LungeExerciseName::SingleArmReverseLungeAndPress => {
                writeln!(f, "single_arm_reverse_lunge_and_press")
            }
            LungeExerciseName::SlidingLateralLunge => writeln!(f, "sliding_lateral_lunge"),
            LungeExerciseName::WeightedSlidingLateralLunge => {
                writeln!(f, "weighted_sliding_lateral_lunge")
            }
            LungeExerciseName::WalkingBarbellLunge => writeln!(f, "walking_barbell_lunge"),
            LungeExerciseName::WalkingDumbbellLunge => writeln!(f, "walking_dumbbell_lunge"),
            LungeExerciseName::WalkingLunge => writeln!(f, "walking_lunge"),
            LungeExerciseName::WeightedWalkingLunge => writeln!(f, "weighted_walking_lunge"),
            LungeExerciseName::WideGripOverheadBarbellSplitSquat => {
                writeln!(f, "wide_grip_overhead_barbell_split_squat")
            }
            LungeExerciseName::UnknownVariant(value) => writeln!(f, "unknown_variant_{}", *value),
        }
    }
}
impl convert::From<u16> for LungeExerciseName {
    fn from(value: u16) -> Self {
        match value {
            0 => LungeExerciseName::OverheadLunge,
            1 => LungeExerciseName::LungeMatrix,
            2 => LungeExerciseName::WeightedLungeMatrix,
            3 => LungeExerciseName::AlternatingBarbellForwardLunge,
            4 => LungeExerciseName::AlternatingDumbbellLungeWithReach,
            5 => LungeExerciseName::BackFootElevatedDumbbellSplitSquat,
            6 => LungeExerciseName::BarbellBoxLunge,
            7 => LungeExerciseName::BarbellBulgarianSplitSquat,
            8 => LungeExerciseName::BarbellCrossoverLunge,
            9 => LungeExerciseName::BarbellFrontSplitSquat,
            10 => LungeExerciseName::BarbellLunge,
            11 => LungeExerciseName::BarbellReverseLunge,
            12 => LungeExerciseName::BarbellSideLunge,
            13 => LungeExerciseName::BarbellSplitSquat,
            14 => LungeExerciseName::CoreControlRearLunge,
            15 => LungeExerciseName::DiagonalLunge,
            16 => LungeExerciseName::DropLunge,
            17 => LungeExerciseName::DumbbellBoxLunge,
            18 => LungeExerciseName::DumbbellBulgarianSplitSquat,
            19 => LungeExerciseName::DumbbellCrossoverLunge,
            20 => LungeExerciseName::DumbbellDiagonalLunge,
            21 => LungeExerciseName::DumbbellLunge,
            22 => LungeExerciseName::DumbbellLungeAndRotation,
            23 => LungeExerciseName::DumbbellOverheadBulgarianSplitSquat,
            24 => LungeExerciseName::DumbbellReverseLungeToHighKneeAndPress,
            25 => LungeExerciseName::DumbbellSideLunge,
            26 => LungeExerciseName::ElevatedFrontFootBarbellSplitSquat,
            27 => LungeExerciseName::FrontFootElevatedDumbbellSplitSquat,
            28 => LungeExerciseName::GunslingerLunge,
            29 => LungeExerciseName::LawnmowerLunge,
            30 => LungeExerciseName::LowLungeWithIsometricAdduction,
            31 => LungeExerciseName::LowSideToSideLunge,
            32 => LungeExerciseName::Lunge,
            33 => LungeExerciseName::WeightedLunge,
            34 => LungeExerciseName::LungeWithArmReach,
            35 => LungeExerciseName::LungeWithDiagonalReach,
            36 => LungeExerciseName::LungeWithSideBend,
            37 => LungeExerciseName::OffsetDumbbellLunge,
            38 => LungeExerciseName::OffsetDumbbellReverseLunge,
            39 => LungeExerciseName::OverheadBulgarianSplitSquat,
            40 => LungeExerciseName::OverheadDumbbellReverseLunge,
            41 => LungeExerciseName::OverheadDumbbellSplitSquat,
            42 => LungeExerciseName::OverheadLungeWithRotation,
            43 => LungeExerciseName::ReverseBarbellBoxLunge,
            44 => LungeExerciseName::ReverseBoxLunge,
            45 => LungeExerciseName::ReverseDumbbellBoxLunge,
            46 => LungeExerciseName::ReverseDumbbellCrossoverLunge,
            47 => LungeExerciseName::ReverseDumbbellDiagonalLunge,
            48 => LungeExerciseName::ReverseLungeWithReachBack,
            49 => LungeExerciseName::WeightedReverseLungeWithReachBack,
            50 => LungeExerciseName::ReverseLungeWithTwistAndOverheadReach,
            51 => LungeExerciseName::WeightedReverseLungeWithTwistAndOverheadReach,
            52 => LungeExerciseName::ReverseSlidingBoxLunge,
            53 => LungeExerciseName::WeightedReverseSlidingBoxLunge,
            54 => LungeExerciseName::ReverseSlidingLunge,
            55 => LungeExerciseName::WeightedReverseSlidingLunge,
            56 => LungeExerciseName::RunnersLungeToBalance,
            57 => LungeExerciseName::WeightedRunnersLungeToBalance,
            58 => LungeExerciseName::ShiftingSideLunge,
            59 => LungeExerciseName::SideAndCrossoverLunge,
            60 => LungeExerciseName::WeightedSideAndCrossoverLunge,
            61 => LungeExerciseName::SideLunge,
            62 => LungeExerciseName::WeightedSideLunge,
            63 => LungeExerciseName::SideLungeAndPress,
            64 => LungeExerciseName::SideLungeJumpOff,
            65 => LungeExerciseName::SideLungeSweep,
            66 => LungeExerciseName::WeightedSideLungeSweep,
            67 => LungeExerciseName::SideLungeToCrossoverTap,
            68 => LungeExerciseName::WeightedSideLungeToCrossoverTap,
            69 => LungeExerciseName::SideToSideLungeChops,
            70 => LungeExerciseName::WeightedSideToSideLungeChops,
            71 => LungeExerciseName::SiffJumpLunge,
            72 => LungeExerciseName::WeightedSiffJumpLunge,
            73 => LungeExerciseName::SingleArmReverseLungeAndPress,
            74 => LungeExerciseName::SlidingLateralLunge,
            75 => LungeExerciseName::WeightedSlidingLateralLunge,
            76 => LungeExerciseName::WalkingBarbellLunge,
            77 => LungeExerciseName::WalkingDumbbellLunge,
            78 => LungeExerciseName::WalkingLunge,
            79 => LungeExerciseName::WeightedWalkingLunge,
            80 => LungeExerciseName::WideGripOverheadBarbellSplitSquat,
            _ => LungeExerciseName::UnknownVariant(value),
        }
    }
}
impl convert::From<i64> for LungeExerciseName {
    fn from(value: i64) -> Self {
        LungeExerciseName::from(value as u16)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum OlympicLiftExerciseName {
    BarbellHangPowerClean,
    BarbellHangSquatClean,
    BarbellPowerClean,
    BarbellPowerSnatch,
    BarbellSquatClean,
    CleanAndJerk,
    BarbellHangPowerSnatch,
    BarbellHangPull,
    BarbellHighPull,
    BarbellSnatch,
    BarbellSplitJerk,
    Clean,
    DumbbellClean,
    DumbbellHangPull,
    OneHandDumbbellSplitSnatch,
    PushJerk,
    SingleArmDumbbellSnatch,
    SingleArmHangSnatch,
    SingleArmKettlebellSnatch,
    SplitJerk,
    SquatCleanAndJerk,
    UnknownVariant(u16),
}
impl OlympicLiftExerciseName {
    pub fn as_u16(self) -> u16 {
        match self {
            OlympicLiftExerciseName::BarbellHangPowerClean => 0,
            OlympicLiftExerciseName::BarbellHangSquatClean => 1,
            OlympicLiftExerciseName::BarbellPowerClean => 2,
            OlympicLiftExerciseName::BarbellPowerSnatch => 3,
            OlympicLiftExerciseName::BarbellSquatClean => 4,
            OlympicLiftExerciseName::CleanAndJerk => 5,
            OlympicLiftExerciseName::BarbellHangPowerSnatch => 6,
            OlympicLiftExerciseName::BarbellHangPull => 7,
            OlympicLiftExerciseName::BarbellHighPull => 8,
            OlympicLiftExerciseName::BarbellSnatch => 9,
            OlympicLiftExerciseName::BarbellSplitJerk => 10,
            OlympicLiftExerciseName::Clean => 11,
            OlympicLiftExerciseName::DumbbellClean => 12,
            OlympicLiftExerciseName::DumbbellHangPull => 13,
            OlympicLiftExerciseName::OneHandDumbbellSplitSnatch => 14,
            OlympicLiftExerciseName::PushJerk => 15,
            OlympicLiftExerciseName::SingleArmDumbbellSnatch => 16,
            OlympicLiftExerciseName::SingleArmHangSnatch => 17,
            OlympicLiftExerciseName::SingleArmKettlebellSnatch => 18,
            OlympicLiftExerciseName::SplitJerk => 19,
            OlympicLiftExerciseName::SquatCleanAndJerk => 20,
            OlympicLiftExerciseName::UnknownVariant(value) => value,
        }
    }
    pub fn as_i64(self) -> i64 {
        self.as_u16() as i64
    }
}
impl fmt::Display for OlympicLiftExerciseName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            OlympicLiftExerciseName::BarbellHangPowerClean => {
                writeln!(f, "barbell_hang_power_clean")
            }
            OlympicLiftExerciseName::BarbellHangSquatClean => {
                writeln!(f, "barbell_hang_squat_clean")
            }
            OlympicLiftExerciseName::BarbellPowerClean => writeln!(f, "barbell_power_clean"),
            OlympicLiftExerciseName::BarbellPowerSnatch => writeln!(f, "barbell_power_snatch"),
            OlympicLiftExerciseName::BarbellSquatClean => writeln!(f, "barbell_squat_clean"),
            OlympicLiftExerciseName::CleanAndJerk => writeln!(f, "clean_and_jerk"),
            OlympicLiftExerciseName::BarbellHangPowerSnatch => {
                writeln!(f, "barbell_hang_power_snatch")
            }
            OlympicLiftExerciseName::BarbellHangPull => writeln!(f, "barbell_hang_pull"),
            OlympicLiftExerciseName::BarbellHighPull => writeln!(f, "barbell_high_pull"),
            OlympicLiftExerciseName::BarbellSnatch => writeln!(f, "barbell_snatch"),
            OlympicLiftExerciseName::BarbellSplitJerk => writeln!(f, "barbell_split_jerk"),
            OlympicLiftExerciseName::Clean => writeln!(f, "clean"),
            OlympicLiftExerciseName::DumbbellClean => writeln!(f, "dumbbell_clean"),
            OlympicLiftExerciseName::DumbbellHangPull => writeln!(f, "dumbbell_hang_pull"),
            OlympicLiftExerciseName::OneHandDumbbellSplitSnatch => {
                writeln!(f, "one_hand_dumbbell_split_snatch")
            }
            OlympicLiftExerciseName::PushJerk => writeln!(f, "push_jerk"),
            OlympicLiftExerciseName::SingleArmDumbbellSnatch => {
                writeln!(f, "single_arm_dumbbell_snatch")
            }
            OlympicLiftExerciseName::SingleArmHangSnatch => writeln!(f, "single_arm_hang_snatch"),
            OlympicLiftExerciseName::SingleArmKettlebellSnatch => {
                writeln!(f, "single_arm_kettlebell_snatch")
            }
            OlympicLiftExerciseName::SplitJerk => writeln!(f, "split_jerk"),
            OlympicLiftExerciseName::SquatCleanAndJerk => writeln!(f, "squat_clean_and_jerk"),
            OlympicLiftExerciseName::UnknownVariant(value) => {
                writeln!(f, "unknown_variant_{}", *value)
            }
        }
    }
}
impl convert::From<u16> for OlympicLiftExerciseName {
    fn from(value: u16) -> Self {
        match value {
            0 => OlympicLiftExerciseName::BarbellHangPowerClean,
            1 => OlympicLiftExerciseName::BarbellHangSquatClean,
            2 => OlympicLiftExerciseName::BarbellPowerClean,
            3 => OlympicLiftExerciseName::BarbellPowerSnatch,
            4 => OlympicLiftExerciseName::BarbellSquatClean,
            5 => OlympicLiftExerciseName::CleanAndJerk,
            6 => OlympicLiftExerciseName::BarbellHangPowerSnatch,
            7 => OlympicLiftExerciseName::BarbellHangPull,
            8 => OlympicLiftExerciseName::BarbellHighPull,
            9 => OlympicLiftExerciseName::BarbellSnatch,
            10 => OlympicLiftExerciseName::BarbellSplitJerk,
            11 => OlympicLiftExerciseName::Clean,
            12 => OlympicLiftExerciseName::DumbbellClean,
            13 => OlympicLiftExerciseName::DumbbellHangPull,
            14 => OlympicLiftExerciseName::OneHandDumbbellSplitSnatch,
            15 => OlympicLiftExerciseName::PushJerk,
            16 => OlympicLiftExerciseName::SingleArmDumbbellSnatch,
            17 => OlympicLiftExerciseName::SingleArmHangSnatch,
            18 => OlympicLiftExerciseName::SingleArmKettlebellSnatch,
            19 => OlympicLiftExerciseName::SplitJerk,
            20 => OlympicLiftExerciseName::SquatCleanAndJerk,
            _ => OlympicLiftExerciseName::UnknownVariant(value),
        }
    }
}
impl convert::From<i64> for OlympicLiftExerciseName {
    fn from(value: i64) -> Self {
        OlympicLiftExerciseName::from(value as u16)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum PlankExerciseName {
    Name45DegreePlank,
    Weighted45DegreePlank,
    Name90DegreeStaticHold,
    Weighted90DegreeStaticHold,
    BearCrawl,
    WeightedBearCrawl,
    CrossBodyMountainClimber,
    WeightedCrossBodyMountainClimber,
    ElbowPlankPikeJacks,
    WeightedElbowPlankPikeJacks,
    ElevatedFeetPlank,
    WeightedElevatedFeetPlank,
    ElevatorAbs,
    WeightedElevatorAbs,
    ExtendedPlank,
    WeightedExtendedPlank,
    FullPlankPasseTwist,
    WeightedFullPlankPasseTwist,
    InchingElbowPlank,
    WeightedInchingElbowPlank,
    InchwormToSidePlank,
    WeightedInchwormToSidePlank,
    KneelingPlank,
    WeightedKneelingPlank,
    KneelingSidePlankWithLegLift,
    WeightedKneelingSidePlankWithLegLift,
    LateralRoll,
    WeightedLateralRoll,
    LyingReversePlank,
    WeightedLyingReversePlank,
    MedicineBallMountainClimber,
    WeightedMedicineBallMountainClimber,
    ModifiedMountainClimberAndExtension,
    WeightedModifiedMountainClimberAndExtension,
    MountainClimber,
    WeightedMountainClimber,
    MountainClimberOnSlidingDiscs,
    WeightedMountainClimberOnSlidingDiscs,
    MountainClimberWithFeetOnBosuBall,
    WeightedMountainClimberWithFeetOnBosuBall,
    MountainClimberWithHandsOnBench,
    MountainClimberWithHandsOnSwissBall,
    WeightedMountainClimberWithHandsOnSwissBall,
    Plank,
    PlankJacksWithFeetOnSlidingDiscs,
    WeightedPlankJacksWithFeetOnSlidingDiscs,
    PlankKneeTwist,
    WeightedPlankKneeTwist,
    PlankPikeJumps,
    WeightedPlankPikeJumps,
    PlankPikes,
    WeightedPlankPikes,
    PlankToStandUp,
    WeightedPlankToStandUp,
    PlankWithArmRaise,
    WeightedPlankWithArmRaise,
    PlankWithKneeToElbow,
    WeightedPlankWithKneeToElbow,
    PlankWithObliqueCrunch,
    WeightedPlankWithObliqueCrunch,
    PlyometricSidePlank,
    WeightedPlyometricSidePlank,
    RollingSidePlank,
    WeightedRollingSidePlank,
    SideKickPlank,
    WeightedSideKickPlank,
    SidePlank,
    WeightedSidePlank,
    SidePlankAndRow,
    WeightedSidePlankAndRow,
    SidePlankLift,
    WeightedSidePlankLift,
    SidePlankWithElbowOnBosuBall,
    WeightedSidePlankWithElbowOnBosuBall,
    SidePlankWithFeetOnBench,
    WeightedSidePlankWithFeetOnBench,
    SidePlankWithKneeCircle,
    WeightedSidePlankWithKneeCircle,
    SidePlankWithKneeTuck,
    WeightedSidePlankWithKneeTuck,
    SidePlankWithLegLift,
    WeightedSidePlankWithLegLift,
    SidePlankWithReachUnder,
    WeightedSidePlankWithReachUnder,
    SingleLegElevatedFeetPlank,
    WeightedSingleLegElevatedFeetPlank,
    SingleLegFlexAndExtend,
    WeightedSingleLegFlexAndExtend,
    SingleLegSidePlank,
    WeightedSingleLegSidePlank,
    SpidermanPlank,
    WeightedSpidermanPlank,
    StraightArmPlank,
    WeightedStraightArmPlank,
    StraightArmPlankWithShoulderTouch,
    WeightedStraightArmPlankWithShoulderTouch,
    SwissBallPlank,
    WeightedSwissBallPlank,
    SwissBallPlankLegLift,
    WeightedSwissBallPlankLegLift,
    SwissBallPlankLegLiftAndHold,
    SwissBallPlankWithFeetOnBench,
    WeightedSwissBallPlankWithFeetOnBench,
    SwissBallProneJackknife,
    WeightedSwissBallProneJackknife,
    SwissBallSidePlank,
    WeightedSwissBallSidePlank,
    ThreeWayPlank,
    WeightedThreeWayPlank,
    TowelPlankAndKneeIn,
    WeightedTowelPlankAndKneeIn,
    TStabilization,
    WeightedTStabilization,
    TurkishGetUpToSidePlank,
    WeightedTurkishGetUpToSidePlank,
    TwoPointPlank,
    WeightedTwoPointPlank,
    WeightedPlank,
    WideStancePlankWithDiagonalArmLift,
    WeightedWideStancePlankWithDiagonalArmLift,
    WideStancePlankWithDiagonalLegLift,
    WeightedWideStancePlankWithDiagonalLegLift,
    WideStancePlankWithLegLift,
    WeightedWideStancePlankWithLegLift,
    WideStancePlankWithOppositeArmAndLegLift,
    WeightedMountainClimberWithHandsOnBench,
    WeightedSwissBallPlankLegLiftAndHold,
    WeightedWideStancePlankWithOppositeArmAndLegLift,
    PlankWithFeetOnSwissBall,
    SidePlankToPlankWithReachUnder,
    BridgeWithGluteLowerLift,
    BridgeOneLegBridge,
    PlankWithArmVariations,
    PlankWithLegLift,
    ReversePlankWithLegPull,
    UnknownVariant(u16),
}
impl PlankExerciseName {
    pub fn as_u16(self) -> u16 {
        match self {
            PlankExerciseName::Name45DegreePlank => 0,
            PlankExerciseName::Weighted45DegreePlank => 1,
            PlankExerciseName::Name90DegreeStaticHold => 2,
            PlankExerciseName::Weighted90DegreeStaticHold => 3,
            PlankExerciseName::BearCrawl => 4,
            PlankExerciseName::WeightedBearCrawl => 5,
            PlankExerciseName::CrossBodyMountainClimber => 6,
            PlankExerciseName::WeightedCrossBodyMountainClimber => 7,
            PlankExerciseName::ElbowPlankPikeJacks => 8,
            PlankExerciseName::WeightedElbowPlankPikeJacks => 9,
            PlankExerciseName::ElevatedFeetPlank => 10,
            PlankExerciseName::WeightedElevatedFeetPlank => 11,
            PlankExerciseName::ElevatorAbs => 12,
            PlankExerciseName::WeightedElevatorAbs => 13,
            PlankExerciseName::ExtendedPlank => 14,
            PlankExerciseName::WeightedExtendedPlank => 15,
            PlankExerciseName::FullPlankPasseTwist => 16,
            PlankExerciseName::WeightedFullPlankPasseTwist => 17,
            PlankExerciseName::InchingElbowPlank => 18,
            PlankExerciseName::WeightedInchingElbowPlank => 19,
            PlankExerciseName::InchwormToSidePlank => 20,
            PlankExerciseName::WeightedInchwormToSidePlank => 21,
            PlankExerciseName::KneelingPlank => 22,
            PlankExerciseName::WeightedKneelingPlank => 23,
            PlankExerciseName::KneelingSidePlankWithLegLift => 24,
            PlankExerciseName::WeightedKneelingSidePlankWithLegLift => 25,
            PlankExerciseName::LateralRoll => 26,
            PlankExerciseName::WeightedLateralRoll => 27,
            PlankExerciseName::LyingReversePlank => 28,
            PlankExerciseName::WeightedLyingReversePlank => 29,
            PlankExerciseName::MedicineBallMountainClimber => 30,
            PlankExerciseName::WeightedMedicineBallMountainClimber => 31,
            PlankExerciseName::ModifiedMountainClimberAndExtension => 32,
            PlankExerciseName::WeightedModifiedMountainClimberAndExtension => 33,
            PlankExerciseName::MountainClimber => 34,
            PlankExerciseName::WeightedMountainClimber => 35,
            PlankExerciseName::MountainClimberOnSlidingDiscs => 36,
            PlankExerciseName::WeightedMountainClimberOnSlidingDiscs => 37,
            PlankExerciseName::MountainClimberWithFeetOnBosuBall => 38,
            PlankExerciseName::WeightedMountainClimberWithFeetOnBosuBall => 39,
            PlankExerciseName::MountainClimberWithHandsOnBench => 40,
            PlankExerciseName::MountainClimberWithHandsOnSwissBall => 41,
            PlankExerciseName::WeightedMountainClimberWithHandsOnSwissBall => 42,
            PlankExerciseName::Plank => 43,
            PlankExerciseName::PlankJacksWithFeetOnSlidingDiscs => 44,
            PlankExerciseName::WeightedPlankJacksWithFeetOnSlidingDiscs => 45,
            PlankExerciseName::PlankKneeTwist => 46,
            PlankExerciseName::WeightedPlankKneeTwist => 47,
            PlankExerciseName::PlankPikeJumps => 48,
            PlankExerciseName::WeightedPlankPikeJumps => 49,
            PlankExerciseName::PlankPikes => 50,
            PlankExerciseName::WeightedPlankPikes => 51,
            PlankExerciseName::PlankToStandUp => 52,
            PlankExerciseName::WeightedPlankToStandUp => 53,
            PlankExerciseName::PlankWithArmRaise => 54,
            PlankExerciseName::WeightedPlankWithArmRaise => 55,
            PlankExerciseName::PlankWithKneeToElbow => 56,
            PlankExerciseName::WeightedPlankWithKneeToElbow => 57,
            PlankExerciseName::PlankWithObliqueCrunch => 58,
            PlankExerciseName::WeightedPlankWithObliqueCrunch => 59,
            PlankExerciseName::PlyometricSidePlank => 60,
            PlankExerciseName::WeightedPlyometricSidePlank => 61,
            PlankExerciseName::RollingSidePlank => 62,
            PlankExerciseName::WeightedRollingSidePlank => 63,
            PlankExerciseName::SideKickPlank => 64,
            PlankExerciseName::WeightedSideKickPlank => 65,
            PlankExerciseName::SidePlank => 66,
            PlankExerciseName::WeightedSidePlank => 67,
            PlankExerciseName::SidePlankAndRow => 68,
            PlankExerciseName::WeightedSidePlankAndRow => 69,
            PlankExerciseName::SidePlankLift => 70,
            PlankExerciseName::WeightedSidePlankLift => 71,
            PlankExerciseName::SidePlankWithElbowOnBosuBall => 72,
            PlankExerciseName::WeightedSidePlankWithElbowOnBosuBall => 73,
            PlankExerciseName::SidePlankWithFeetOnBench => 74,
            PlankExerciseName::WeightedSidePlankWithFeetOnBench => 75,
            PlankExerciseName::SidePlankWithKneeCircle => 76,
            PlankExerciseName::WeightedSidePlankWithKneeCircle => 77,
            PlankExerciseName::SidePlankWithKneeTuck => 78,
            PlankExerciseName::WeightedSidePlankWithKneeTuck => 79,
            PlankExerciseName::SidePlankWithLegLift => 80,
            PlankExerciseName::WeightedSidePlankWithLegLift => 81,
            PlankExerciseName::SidePlankWithReachUnder => 82,
            PlankExerciseName::WeightedSidePlankWithReachUnder => 83,
            PlankExerciseName::SingleLegElevatedFeetPlank => 84,
            PlankExerciseName::WeightedSingleLegElevatedFeetPlank => 85,
            PlankExerciseName::SingleLegFlexAndExtend => 86,
            PlankExerciseName::WeightedSingleLegFlexAndExtend => 87,
            PlankExerciseName::SingleLegSidePlank => 88,
            PlankExerciseName::WeightedSingleLegSidePlank => 89,
            PlankExerciseName::SpidermanPlank => 90,
            PlankExerciseName::WeightedSpidermanPlank => 91,
            PlankExerciseName::StraightArmPlank => 92,
            PlankExerciseName::WeightedStraightArmPlank => 93,
            PlankExerciseName::StraightArmPlankWithShoulderTouch => 94,
            PlankExerciseName::WeightedStraightArmPlankWithShoulderTouch => 95,
            PlankExerciseName::SwissBallPlank => 96,
            PlankExerciseName::WeightedSwissBallPlank => 97,
            PlankExerciseName::SwissBallPlankLegLift => 98,
            PlankExerciseName::WeightedSwissBallPlankLegLift => 99,
            PlankExerciseName::SwissBallPlankLegLiftAndHold => 100,
            PlankExerciseName::SwissBallPlankWithFeetOnBench => 101,
            PlankExerciseName::WeightedSwissBallPlankWithFeetOnBench => 102,
            PlankExerciseName::SwissBallProneJackknife => 103,
            PlankExerciseName::WeightedSwissBallProneJackknife => 104,
            PlankExerciseName::SwissBallSidePlank => 105,
            PlankExerciseName::WeightedSwissBallSidePlank => 106,
            PlankExerciseName::ThreeWayPlank => 107,
            PlankExerciseName::WeightedThreeWayPlank => 108,
            PlankExerciseName::TowelPlankAndKneeIn => 109,
            PlankExerciseName::WeightedTowelPlankAndKneeIn => 110,
            PlankExerciseName::TStabilization => 111,
            PlankExerciseName::WeightedTStabilization => 112,
            PlankExerciseName::TurkishGetUpToSidePlank => 113,
            PlankExerciseName::WeightedTurkishGetUpToSidePlank => 114,
            PlankExerciseName::TwoPointPlank => 115,
            PlankExerciseName::WeightedTwoPointPlank => 116,
            PlankExerciseName::WeightedPlank => 117,
            PlankExerciseName::WideStancePlankWithDiagonalArmLift => 118,
            PlankExerciseName::WeightedWideStancePlankWithDiagonalArmLift => 119,
            PlankExerciseName::WideStancePlankWithDiagonalLegLift => 120,
            PlankExerciseName::WeightedWideStancePlankWithDiagonalLegLift => 121,
            PlankExerciseName::WideStancePlankWithLegLift => 122,
            PlankExerciseName::WeightedWideStancePlankWithLegLift => 123,
            PlankExerciseName::WideStancePlankWithOppositeArmAndLegLift => 124,
            PlankExerciseName::WeightedMountainClimberWithHandsOnBench => 125,
            PlankExerciseName::WeightedSwissBallPlankLegLiftAndHold => 126,
            PlankExerciseName::WeightedWideStancePlankWithOppositeArmAndLegLift => 127,
            PlankExerciseName::PlankWithFeetOnSwissBall => 128,
            PlankExerciseName::SidePlankToPlankWithReachUnder => 129,
            PlankExerciseName::BridgeWithGluteLowerLift => 130,
            PlankExerciseName::BridgeOneLegBridge => 131,
            PlankExerciseName::PlankWithArmVariations => 132,
            PlankExerciseName::PlankWithLegLift => 133,
            PlankExerciseName::ReversePlankWithLegPull => 134,
            PlankExerciseName::UnknownVariant(value) => value,
        }
    }
    pub fn as_i64(self) -> i64 {
        self.as_u16() as i64
    }
}
impl fmt::Display for PlankExerciseName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            PlankExerciseName::Name45DegreePlank => writeln!(f, "45_degree_plank"),
            PlankExerciseName::Weighted45DegreePlank => writeln!(f, "weighted_45_degree_plank"),
            PlankExerciseName::Name90DegreeStaticHold => writeln!(f, "90_degree_static_hold"),
            PlankExerciseName::Weighted90DegreeStaticHold => {
                writeln!(f, "weighted_90_degree_static_hold")
            }
            PlankExerciseName::BearCrawl => writeln!(f, "bear_crawl"),
            PlankExerciseName::WeightedBearCrawl => writeln!(f, "weighted_bear_crawl"),
            PlankExerciseName::CrossBodyMountainClimber => {
                writeln!(f, "cross_body_mountain_climber")
            }
            PlankExerciseName::WeightedCrossBodyMountainClimber => {
                writeln!(f, "weighted_cross_body_mountain_climber")
            }
            PlankExerciseName::ElbowPlankPikeJacks => writeln!(f, "elbow_plank_pike_jacks"),
            PlankExerciseName::WeightedElbowPlankPikeJacks => {
                writeln!(f, "weighted_elbow_plank_pike_jacks")
            }
            PlankExerciseName::ElevatedFeetPlank => writeln!(f, "elevated_feet_plank"),
            PlankExerciseName::WeightedElevatedFeetPlank => {
                writeln!(f, "weighted_elevated_feet_plank")
            }
            PlankExerciseName::ElevatorAbs => writeln!(f, "elevator_abs"),
            PlankExerciseName::WeightedElevatorAbs => writeln!(f, "weighted_elevator_abs"),
            PlankExerciseName::ExtendedPlank => writeln!(f, "extended_plank"),
            PlankExerciseName::WeightedExtendedPlank => writeln!(f, "weighted_extended_plank"),
            PlankExerciseName::FullPlankPasseTwist => writeln!(f, "full_plank_passe_twist"),
            PlankExerciseName::WeightedFullPlankPasseTwist => {
                writeln!(f, "weighted_full_plank_passe_twist")
            }
            PlankExerciseName::InchingElbowPlank => writeln!(f, "inching_elbow_plank"),
            PlankExerciseName::WeightedInchingElbowPlank => {
                writeln!(f, "weighted_inching_elbow_plank")
            }
            PlankExerciseName::InchwormToSidePlank => writeln!(f, "inchworm_to_side_plank"),
            PlankExerciseName::WeightedInchwormToSidePlank => {
                writeln!(f, "weighted_inchworm_to_side_plank")
            }
            PlankExerciseName::KneelingPlank => writeln!(f, "kneeling_plank"),
            PlankExerciseName::WeightedKneelingPlank => writeln!(f, "weighted_kneeling_plank"),
            PlankExerciseName::KneelingSidePlankWithLegLift => {
                writeln!(f, "kneeling_side_plank_with_leg_lift")
            }
            PlankExerciseName::WeightedKneelingSidePlankWithLegLift => {
                writeln!(f, "weighted_kneeling_side_plank_with_leg_lift")
            }
            PlankExerciseName::LateralRoll => writeln!(f, "lateral_roll"),
            PlankExerciseName::WeightedLateralRoll => writeln!(f, "weighted_lateral_roll"),
            PlankExerciseName::LyingReversePlank => writeln!(f, "lying_reverse_plank"),
            PlankExerciseName::WeightedLyingReversePlank => {
                writeln!(f, "weighted_lying_reverse_plank")
            }
            PlankExerciseName::MedicineBallMountainClimber => {
                writeln!(f, "medicine_ball_mountain_climber")
            }
            PlankExerciseName::WeightedMedicineBallMountainClimber => {
                writeln!(f, "weighted_medicine_ball_mountain_climber")
            }
            PlankExerciseName::ModifiedMountainClimberAndExtension => {
                writeln!(f, "modified_mountain_climber_and_extension")
            }
            PlankExerciseName::WeightedModifiedMountainClimberAndExtension => {
                writeln!(f, "weighted_modified_mountain_climber_and_extension")
            }
            PlankExerciseName::MountainClimber => writeln!(f, "mountain_climber"),
            PlankExerciseName::WeightedMountainClimber => writeln!(f, "weighted_mountain_climber"),
            PlankExerciseName::MountainClimberOnSlidingDiscs => {
                writeln!(f, "mountain_climber_on_sliding_discs")
            }
            PlankExerciseName::WeightedMountainClimberOnSlidingDiscs => {
                writeln!(f, "weighted_mountain_climber_on_sliding_discs")
            }
            PlankExerciseName::MountainClimberWithFeetOnBosuBall => {
                writeln!(f, "mountain_climber_with_feet_on_bosu_ball")
            }
            PlankExerciseName::WeightedMountainClimberWithFeetOnBosuBall => {
                writeln!(f, "weighted_mountain_climber_with_feet_on_bosu_ball")
            }
            PlankExerciseName::MountainClimberWithHandsOnBench => {
                writeln!(f, "mountain_climber_with_hands_on_bench")
            }
            PlankExerciseName::MountainClimberWithHandsOnSwissBall => {
                writeln!(f, "mountain_climber_with_hands_on_swiss_ball")
            }
            PlankExerciseName::WeightedMountainClimberWithHandsOnSwissBall => {
                writeln!(f, "weighted_mountain_climber_with_hands_on_swiss_ball")
            }
            PlankExerciseName::Plank => writeln!(f, "plank"),
            PlankExerciseName::PlankJacksWithFeetOnSlidingDiscs => {
                writeln!(f, "plank_jacks_with_feet_on_sliding_discs")
            }
            PlankExerciseName::WeightedPlankJacksWithFeetOnSlidingDiscs => {
                writeln!(f, "weighted_plank_jacks_with_feet_on_sliding_discs")
            }
            PlankExerciseName::PlankKneeTwist => writeln!(f, "plank_knee_twist"),
            PlankExerciseName::WeightedPlankKneeTwist => writeln!(f, "weighted_plank_knee_twist"),
            PlankExerciseName::PlankPikeJumps => writeln!(f, "plank_pike_jumps"),
            PlankExerciseName::WeightedPlankPikeJumps => writeln!(f, "weighted_plank_pike_jumps"),
            PlankExerciseName::PlankPikes => writeln!(f, "plank_pikes"),
            PlankExerciseName::WeightedPlankPikes => writeln!(f, "weighted_plank_pikes"),
            PlankExerciseName::PlankToStandUp => writeln!(f, "plank_to_stand_up"),
            PlankExerciseName::WeightedPlankToStandUp => writeln!(f, "weighted_plank_to_stand_up"),
            PlankExerciseName::PlankWithArmRaise => writeln!(f, "plank_with_arm_raise"),
            PlankExerciseName::WeightedPlankWithArmRaise => {
                writeln!(f, "weighted_plank_with_arm_raise")
            }
            PlankExerciseName::PlankWithKneeToElbow => writeln!(f, "plank_with_knee_to_elbow"),
            PlankExerciseName::WeightedPlankWithKneeToElbow => {
                writeln!(f, "weighted_plank_with_knee_to_elbow")
            }
            PlankExerciseName::PlankWithObliqueCrunch => writeln!(f, "plank_with_oblique_crunch"),
            PlankExerciseName::WeightedPlankWithObliqueCrunch => {
                writeln!(f, "weighted_plank_with_oblique_crunch")
            }
            PlankExerciseName::PlyometricSidePlank => writeln!(f, "plyometric_side_plank"),
            PlankExerciseName::WeightedPlyometricSidePlank => {
                writeln!(f, "weighted_plyometric_side_plank")
            }
            PlankExerciseName::RollingSidePlank => writeln!(f, "rolling_side_plank"),
            PlankExerciseName::WeightedRollingSidePlank => {
                writeln!(f, "weighted_rolling_side_plank")
            }
            PlankExerciseName::SideKickPlank => writeln!(f, "side_kick_plank"),
            PlankExerciseName::WeightedSideKickPlank => writeln!(f, "weighted_side_kick_plank"),
            PlankExerciseName::SidePlank => writeln!(f, "side_plank"),
            PlankExerciseName::WeightedSidePlank => writeln!(f, "weighted_side_plank"),
            PlankExerciseName::SidePlankAndRow => writeln!(f, "side_plank_and_row"),
            PlankExerciseName::WeightedSidePlankAndRow => {
                writeln!(f, "weighted_side_plank_and_row")
            }
            PlankExerciseName::SidePlankLift => writeln!(f, "side_plank_lift"),
            PlankExerciseName::WeightedSidePlankLift => writeln!(f, "weighted_side_plank_lift"),
            PlankExerciseName::SidePlankWithElbowOnBosuBall => {
                writeln!(f, "side_plank_with_elbow_on_bosu_ball")
            }
            PlankExerciseName::WeightedSidePlankWithElbowOnBosuBall => {
                writeln!(f, "weighted_side_plank_with_elbow_on_bosu_ball")
            }
            PlankExerciseName::SidePlankWithFeetOnBench => {
                writeln!(f, "side_plank_with_feet_on_bench")
            }
            PlankExerciseName::WeightedSidePlankWithFeetOnBench => {
                writeln!(f, "weighted_side_plank_with_feet_on_bench")
            }
            PlankExerciseName::SidePlankWithKneeCircle => {
                writeln!(f, "side_plank_with_knee_circle")
            }
            PlankExerciseName::WeightedSidePlankWithKneeCircle => {
                writeln!(f, "weighted_side_plank_with_knee_circle")
            }
            PlankExerciseName::SidePlankWithKneeTuck => writeln!(f, "side_plank_with_knee_tuck"),
            PlankExerciseName::WeightedSidePlankWithKneeTuck => {
                writeln!(f, "weighted_side_plank_with_knee_tuck")
            }
            PlankExerciseName::SidePlankWithLegLift => writeln!(f, "side_plank_with_leg_lift"),
            PlankExerciseName::WeightedSidePlankWithLegLift => {
                writeln!(f, "weighted_side_plank_with_leg_lift")
            }
            PlankExerciseName::SidePlankWithReachUnder => {
                writeln!(f, "side_plank_with_reach_under")
            }
            PlankExerciseName::WeightedSidePlankWithReachUnder => {
                writeln!(f, "weighted_side_plank_with_reach_under")
            }
            PlankExerciseName::SingleLegElevatedFeetPlank => {
                writeln!(f, "single_leg_elevated_feet_plank")
            }
            PlankExerciseName::WeightedSingleLegElevatedFeetPlank => {
                writeln!(f, "weighted_single_leg_elevated_feet_plank")
            }
            PlankExerciseName::SingleLegFlexAndExtend => writeln!(f, "single_leg_flex_and_extend"),
            PlankExerciseName::WeightedSingleLegFlexAndExtend => {
                writeln!(f, "weighted_single_leg_flex_and_extend")
            }
            PlankExerciseName::SingleLegSidePlank => writeln!(f, "single_leg_side_plank"),
            PlankExerciseName::WeightedSingleLegSidePlank => {
                writeln!(f, "weighted_single_leg_side_plank")
            }
            PlankExerciseName::SpidermanPlank => writeln!(f, "spiderman_plank"),
            PlankExerciseName::WeightedSpidermanPlank => writeln!(f, "weighted_spiderman_plank"),
            PlankExerciseName::StraightArmPlank => writeln!(f, "straight_arm_plank"),
            PlankExerciseName::WeightedStraightArmPlank => {
                writeln!(f, "weighted_straight_arm_plank")
            }
            PlankExerciseName::StraightArmPlankWithShoulderTouch => {
                writeln!(f, "straight_arm_plank_with_shoulder_touch")
            }
            PlankExerciseName::WeightedStraightArmPlankWithShoulderTouch => {
                writeln!(f, "weighted_straight_arm_plank_with_shoulder_touch")
            }
            PlankExerciseName::SwissBallPlank => writeln!(f, "swiss_ball_plank"),
            PlankExerciseName::WeightedSwissBallPlank => writeln!(f, "weighted_swiss_ball_plank"),
            PlankExerciseName::SwissBallPlankLegLift => writeln!(f, "swiss_ball_plank_leg_lift"),
            PlankExerciseName::WeightedSwissBallPlankLegLift => {
                writeln!(f, "weighted_swiss_ball_plank_leg_lift")
            }
            PlankExerciseName::SwissBallPlankLegLiftAndHold => {
                writeln!(f, "swiss_ball_plank_leg_lift_and_hold")
            }
            PlankExerciseName::SwissBallPlankWithFeetOnBench => {
                writeln!(f, "swiss_ball_plank_with_feet_on_bench")
            }
            PlankExerciseName::WeightedSwissBallPlankWithFeetOnBench => {
                writeln!(f, "weighted_swiss_ball_plank_with_feet_on_bench")
            }
            PlankExerciseName::SwissBallProneJackknife => writeln!(f, "swiss_ball_prone_jackknife"),
            PlankExerciseName::WeightedSwissBallProneJackknife => {
                writeln!(f, "weighted_swiss_ball_prone_jackknife")
            }
            PlankExerciseName::SwissBallSidePlank => writeln!(f, "swiss_ball_side_plank"),
            PlankExerciseName::WeightedSwissBallSidePlank => {
                writeln!(f, "weighted_swiss_ball_side_plank")
            }
            PlankExerciseName::ThreeWayPlank => writeln!(f, "three_way_plank"),
            PlankExerciseName::WeightedThreeWayPlank => writeln!(f, "weighted_three_way_plank"),
            PlankExerciseName::TowelPlankAndKneeIn => writeln!(f, "towel_plank_and_knee_in"),
            PlankExerciseName::WeightedTowelPlankAndKneeIn => {
                writeln!(f, "weighted_towel_plank_and_knee_in")
            }
            PlankExerciseName::TStabilization => writeln!(f, "t_stabilization"),
            PlankExerciseName::WeightedTStabilization => writeln!(f, "weighted_t_stabilization"),
            PlankExerciseName::TurkishGetUpToSidePlank => {
                writeln!(f, "turkish_get_up_to_side_plank")
            }
            PlankExerciseName::WeightedTurkishGetUpToSidePlank => {
                writeln!(f, "weighted_turkish_get_up_to_side_plank")
            }
            PlankExerciseName::TwoPointPlank => writeln!(f, "two_point_plank"),
            PlankExerciseName::WeightedTwoPointPlank => writeln!(f, "weighted_two_point_plank"),
            PlankExerciseName::WeightedPlank => writeln!(f, "weighted_plank"),
            PlankExerciseName::WideStancePlankWithDiagonalArmLift => {
                writeln!(f, "wide_stance_plank_with_diagonal_arm_lift")
            }
            PlankExerciseName::WeightedWideStancePlankWithDiagonalArmLift => {
                writeln!(f, "weighted_wide_stance_plank_with_diagonal_arm_lift")
            }
            PlankExerciseName::WideStancePlankWithDiagonalLegLift => {
                writeln!(f, "wide_stance_plank_with_diagonal_leg_lift")
            }
            PlankExerciseName::WeightedWideStancePlankWithDiagonalLegLift => {
                writeln!(f, "weighted_wide_stance_plank_with_diagonal_leg_lift")
            }
            PlankExerciseName::WideStancePlankWithLegLift => {
                writeln!(f, "wide_stance_plank_with_leg_lift")
            }
            PlankExerciseName::WeightedWideStancePlankWithLegLift => {
                writeln!(f, "weighted_wide_stance_plank_with_leg_lift")
            }
            PlankExerciseName::WideStancePlankWithOppositeArmAndLegLift => {
                writeln!(f, "wide_stance_plank_with_opposite_arm_and_leg_lift")
            }
            PlankExerciseName::WeightedMountainClimberWithHandsOnBench => {
                writeln!(f, "weighted_mountain_climber_with_hands_on_bench")
            }
            PlankExerciseName::WeightedSwissBallPlankLegLiftAndHold => {
                writeln!(f, "weighted_swiss_ball_plank_leg_lift_and_hold")
            }
            PlankExerciseName::WeightedWideStancePlankWithOppositeArmAndLegLift => writeln!(
                f,
                "weighted_wide_stance_plank_with_opposite_arm_and_leg_lift"
            ),
            PlankExerciseName::PlankWithFeetOnSwissBall => {
                writeln!(f, "plank_with_feet_on_swiss_ball")
            }
            PlankExerciseName::SidePlankToPlankWithReachUnder => {
                writeln!(f, "side_plank_to_plank_with_reach_under")
            }
            PlankExerciseName::BridgeWithGluteLowerLift => {
                writeln!(f, "bridge_with_glute_lower_lift")
            }
            PlankExerciseName::BridgeOneLegBridge => writeln!(f, "bridge_one_leg_bridge"),
            PlankExerciseName::PlankWithArmVariations => writeln!(f, "plank_with_arm_variations"),
            PlankExerciseName::PlankWithLegLift => writeln!(f, "plank_with_leg_lift"),
            PlankExerciseName::ReversePlankWithLegPull => {
                writeln!(f, "reverse_plank_with_leg_pull")
            }
            PlankExerciseName::UnknownVariant(value) => writeln!(f, "unknown_variant_{}", *value),
        }
    }
}
impl convert::From<u16> for PlankExerciseName {
    fn from(value: u16) -> Self {
        match value {
            0 => PlankExerciseName::Name45DegreePlank,
            1 => PlankExerciseName::Weighted45DegreePlank,
            2 => PlankExerciseName::Name90DegreeStaticHold,
            3 => PlankExerciseName::Weighted90DegreeStaticHold,
            4 => PlankExerciseName::BearCrawl,
            5 => PlankExerciseName::WeightedBearCrawl,
            6 => PlankExerciseName::CrossBodyMountainClimber,
            7 => PlankExerciseName::WeightedCrossBodyMountainClimber,
            8 => PlankExerciseName::ElbowPlankPikeJacks,
            9 => PlankExerciseName::WeightedElbowPlankPikeJacks,
            10 => PlankExerciseName::ElevatedFeetPlank,
            11 => PlankExerciseName::WeightedElevatedFeetPlank,
            12 => PlankExerciseName::ElevatorAbs,
            13 => PlankExerciseName::WeightedElevatorAbs,
            14 => PlankExerciseName::ExtendedPlank,
            15 => PlankExerciseName::WeightedExtendedPlank,
            16 => PlankExerciseName::FullPlankPasseTwist,
            17 => PlankExerciseName::WeightedFullPlankPasseTwist,
            18 => PlankExerciseName::InchingElbowPlank,
            19 => PlankExerciseName::WeightedInchingElbowPlank,
            20 => PlankExerciseName::InchwormToSidePlank,
            21 => PlankExerciseName::WeightedInchwormToSidePlank,
            22 => PlankExerciseName::KneelingPlank,
            23 => PlankExerciseName::WeightedKneelingPlank,
            24 => PlankExerciseName::KneelingSidePlankWithLegLift,
            25 => PlankExerciseName::WeightedKneelingSidePlankWithLegLift,
            26 => PlankExerciseName::LateralRoll,
            27 => PlankExerciseName::WeightedLateralRoll,
            28 => PlankExerciseName::LyingReversePlank,
            29 => PlankExerciseName::WeightedLyingReversePlank,
            30 => PlankExerciseName::MedicineBallMountainClimber,
            31 => PlankExerciseName::WeightedMedicineBallMountainClimber,
            32 => PlankExerciseName::ModifiedMountainClimberAndExtension,
            33 => PlankExerciseName::WeightedModifiedMountainClimberAndExtension,
            34 => PlankExerciseName::MountainClimber,
            35 => PlankExerciseName::WeightedMountainClimber,
            36 => PlankExerciseName::MountainClimberOnSlidingDiscs,
            37 => PlankExerciseName::WeightedMountainClimberOnSlidingDiscs,
            38 => PlankExerciseName::MountainClimberWithFeetOnBosuBall,
            39 => PlankExerciseName::WeightedMountainClimberWithFeetOnBosuBall,
            40 => PlankExerciseName::MountainClimberWithHandsOnBench,
            41 => PlankExerciseName::MountainClimberWithHandsOnSwissBall,
            42 => PlankExerciseName::WeightedMountainClimberWithHandsOnSwissBall,
            43 => PlankExerciseName::Plank,
            44 => PlankExerciseName::PlankJacksWithFeetOnSlidingDiscs,
            45 => PlankExerciseName::WeightedPlankJacksWithFeetOnSlidingDiscs,
            46 => PlankExerciseName::PlankKneeTwist,
            47 => PlankExerciseName::WeightedPlankKneeTwist,
            48 => PlankExerciseName::PlankPikeJumps,
            49 => PlankExerciseName::WeightedPlankPikeJumps,
            50 => PlankExerciseName::PlankPikes,
            51 => PlankExerciseName::WeightedPlankPikes,
            52 => PlankExerciseName::PlankToStandUp,
            53 => PlankExerciseName::WeightedPlankToStandUp,
            54 => PlankExerciseName::PlankWithArmRaise,
            55 => PlankExerciseName::WeightedPlankWithArmRaise,
            56 => PlankExerciseName::PlankWithKneeToElbow,
            57 => PlankExerciseName::WeightedPlankWithKneeToElbow,
            58 => PlankExerciseName::PlankWithObliqueCrunch,
            59 => PlankExerciseName::WeightedPlankWithObliqueCrunch,
            60 => PlankExerciseName::PlyometricSidePlank,
            61 => PlankExerciseName::WeightedPlyometricSidePlank,
            62 => PlankExerciseName::RollingSidePlank,
            63 => PlankExerciseName::WeightedRollingSidePlank,
            64 => PlankExerciseName::SideKickPlank,
            65 => PlankExerciseName::WeightedSideKickPlank,
            66 => PlankExerciseName::SidePlank,
            67 => PlankExerciseName::WeightedSidePlank,
            68 => PlankExerciseName::SidePlankAndRow,
            69 => PlankExerciseName::WeightedSidePlankAndRow,
            70 => PlankExerciseName::SidePlankLift,
            71 => PlankExerciseName::WeightedSidePlankLift,
            72 => PlankExerciseName::SidePlankWithElbowOnBosuBall,
            73 => PlankExerciseName::WeightedSidePlankWithElbowOnBosuBall,
            74 => PlankExerciseName::SidePlankWithFeetOnBench,
            75 => PlankExerciseName::WeightedSidePlankWithFeetOnBench,
            76 => PlankExerciseName::SidePlankWithKneeCircle,
            77 => PlankExerciseName::WeightedSidePlankWithKneeCircle,
            78 => PlankExerciseName::SidePlankWithKneeTuck,
            79 => PlankExerciseName::WeightedSidePlankWithKneeTuck,
            80 => PlankExerciseName::SidePlankWithLegLift,
            81 => PlankExerciseName::WeightedSidePlankWithLegLift,
            82 => PlankExerciseName::SidePlankWithReachUnder,
            83 => PlankExerciseName::WeightedSidePlankWithReachUnder,
            84 => PlankExerciseName::SingleLegElevatedFeetPlank,
            85 => PlankExerciseName::WeightedSingleLegElevatedFeetPlank,
            86 => PlankExerciseName::SingleLegFlexAndExtend,
            87 => PlankExerciseName::WeightedSingleLegFlexAndExtend,
            88 => PlankExerciseName::SingleLegSidePlank,
            89 => PlankExerciseName::WeightedSingleLegSidePlank,
            90 => PlankExerciseName::SpidermanPlank,
            91 => PlankExerciseName::WeightedSpidermanPlank,
            92 => PlankExerciseName::StraightArmPlank,
            93 => PlankExerciseName::WeightedStraightArmPlank,
            94 => PlankExerciseName::StraightArmPlankWithShoulderTouch,
            95 => PlankExerciseName::WeightedStraightArmPlankWithShoulderTouch,
            96 => PlankExerciseName::SwissBallPlank,
            97 => PlankExerciseName::WeightedSwissBallPlank,
            98 => PlankExerciseName::SwissBallPlankLegLift,
            99 => PlankExerciseName::WeightedSwissBallPlankLegLift,
            100 => PlankExerciseName::SwissBallPlankLegLiftAndHold,
            101 => PlankExerciseName::SwissBallPlankWithFeetOnBench,
            102 => PlankExerciseName::WeightedSwissBallPlankWithFeetOnBench,
            103 => PlankExerciseName::SwissBallProneJackknife,
            104 => PlankExerciseName::WeightedSwissBallProneJackknife,
            105 => PlankExerciseName::SwissBallSidePlank,
            106 => PlankExerciseName::WeightedSwissBallSidePlank,
            107 => PlankExerciseName::ThreeWayPlank,
            108 => PlankExerciseName::WeightedThreeWayPlank,
            109 => PlankExerciseName::TowelPlankAndKneeIn,
            110 => PlankExerciseName::WeightedTowelPlankAndKneeIn,
            111 => PlankExerciseName::TStabilization,
            112 => PlankExerciseName::WeightedTStabilization,
            113 => PlankExerciseName::TurkishGetUpToSidePlank,
            114 => PlankExerciseName::WeightedTurkishGetUpToSidePlank,
            115 => PlankExerciseName::TwoPointPlank,
            116 => PlankExerciseName::WeightedTwoPointPlank,
            117 => PlankExerciseName::WeightedPlank,
            118 => PlankExerciseName::WideStancePlankWithDiagonalArmLift,
            119 => PlankExerciseName::WeightedWideStancePlankWithDiagonalArmLift,
            120 => PlankExerciseName::WideStancePlankWithDiagonalLegLift,
            121 => PlankExerciseName::WeightedWideStancePlankWithDiagonalLegLift,
            122 => PlankExerciseName::WideStancePlankWithLegLift,
            123 => PlankExerciseName::WeightedWideStancePlankWithLegLift,
            124 => PlankExerciseName::WideStancePlankWithOppositeArmAndLegLift,
            125 => PlankExerciseName::WeightedMountainClimberWithHandsOnBench,
            126 => PlankExerciseName::WeightedSwissBallPlankLegLiftAndHold,
            127 => PlankExerciseName::WeightedWideStancePlankWithOppositeArmAndLegLift,
            128 => PlankExerciseName::PlankWithFeetOnSwissBall,
            129 => PlankExerciseName::SidePlankToPlankWithReachUnder,
            130 => PlankExerciseName::BridgeWithGluteLowerLift,
            131 => PlankExerciseName::BridgeOneLegBridge,
            132 => PlankExerciseName::PlankWithArmVariations,
            133 => PlankExerciseName::PlankWithLegLift,
            134 => PlankExerciseName::ReversePlankWithLegPull,
            _ => PlankExerciseName::UnknownVariant(value),
        }
    }
}
impl convert::From<i64> for PlankExerciseName {
    fn from(value: i64) -> Self {
        PlankExerciseName::from(value as u16)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum PlyoExerciseName {
    AlternatingJumpLunge,
    WeightedAlternatingJumpLunge,
    BarbellJumpSquat,
    BodyWeightJumpSquat,
    WeightedJumpSquat,
    CrossKneeStrike,
    WeightedCrossKneeStrike,
    DepthJump,
    WeightedDepthJump,
    DumbbellJumpSquat,
    DumbbellSplitJump,
    FrontKneeStrike,
    WeightedFrontKneeStrike,
    HighBoxJump,
    WeightedHighBoxJump,
    IsometricExplosiveBodyWeightJumpSquat,
    WeightedIsometricExplosiveJumpSquat,
    LateralLeapAndHop,
    WeightedLateralLeapAndHop,
    LateralPlyoSquats,
    WeightedLateralPlyoSquats,
    LateralSlide,
    WeightedLateralSlide,
    MedicineBallOverheadThrows,
    MedicineBallSideThrow,
    MedicineBallSlam,
    SideToSideMedicineBallThrows,
    SideToSideShuffleJump,
    WeightedSideToSideShuffleJump,
    SquatJumpOntoBox,
    WeightedSquatJumpOntoBox,
    SquatJumpsInAndOut,
    WeightedSquatJumpsInAndOut,
    UnknownVariant(u16),
}
impl PlyoExerciseName {
    pub fn as_u16(self) -> u16 {
        match self {
            PlyoExerciseName::AlternatingJumpLunge => 0,
            PlyoExerciseName::WeightedAlternatingJumpLunge => 1,
            PlyoExerciseName::BarbellJumpSquat => 2,
            PlyoExerciseName::BodyWeightJumpSquat => 3,
            PlyoExerciseName::WeightedJumpSquat => 4,
            PlyoExerciseName::CrossKneeStrike => 5,
            PlyoExerciseName::WeightedCrossKneeStrike => 6,
            PlyoExerciseName::DepthJump => 7,
            PlyoExerciseName::WeightedDepthJump => 8,
            PlyoExerciseName::DumbbellJumpSquat => 9,
            PlyoExerciseName::DumbbellSplitJump => 10,
            PlyoExerciseName::FrontKneeStrike => 11,
            PlyoExerciseName::WeightedFrontKneeStrike => 12,
            PlyoExerciseName::HighBoxJump => 13,
            PlyoExerciseName::WeightedHighBoxJump => 14,
            PlyoExerciseName::IsometricExplosiveBodyWeightJumpSquat => 15,
            PlyoExerciseName::WeightedIsometricExplosiveJumpSquat => 16,
            PlyoExerciseName::LateralLeapAndHop => 17,
            PlyoExerciseName::WeightedLateralLeapAndHop => 18,
            PlyoExerciseName::LateralPlyoSquats => 19,
            PlyoExerciseName::WeightedLateralPlyoSquats => 20,
            PlyoExerciseName::LateralSlide => 21,
            PlyoExerciseName::WeightedLateralSlide => 22,
            PlyoExerciseName::MedicineBallOverheadThrows => 23,
            PlyoExerciseName::MedicineBallSideThrow => 24,
            PlyoExerciseName::MedicineBallSlam => 25,
            PlyoExerciseName::SideToSideMedicineBallThrows => 26,
            PlyoExerciseName::SideToSideShuffleJump => 27,
            PlyoExerciseName::WeightedSideToSideShuffleJump => 28,
            PlyoExerciseName::SquatJumpOntoBox => 29,
            PlyoExerciseName::WeightedSquatJumpOntoBox => 30,
            PlyoExerciseName::SquatJumpsInAndOut => 31,
            PlyoExerciseName::WeightedSquatJumpsInAndOut => 32,
            PlyoExerciseName::UnknownVariant(value) => value,
        }
    }
    pub fn as_i64(self) -> i64 {
        self.as_u16() as i64
    }
}
impl fmt::Display for PlyoExerciseName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            PlyoExerciseName::AlternatingJumpLunge => writeln!(f, "alternating_jump_lunge"),
            PlyoExerciseName::WeightedAlternatingJumpLunge => {
                writeln!(f, "weighted_alternating_jump_lunge")
            }
            PlyoExerciseName::BarbellJumpSquat => writeln!(f, "barbell_jump_squat"),
            PlyoExerciseName::BodyWeightJumpSquat => writeln!(f, "body_weight_jump_squat"),
            PlyoExerciseName::WeightedJumpSquat => writeln!(f, "weighted_jump_squat"),
            PlyoExerciseName::CrossKneeStrike => writeln!(f, "cross_knee_strike"),
            PlyoExerciseName::WeightedCrossKneeStrike => writeln!(f, "weighted_cross_knee_strike"),
            PlyoExerciseName::DepthJump => writeln!(f, "depth_jump"),
            PlyoExerciseName::WeightedDepthJump => writeln!(f, "weighted_depth_jump"),
            PlyoExerciseName::DumbbellJumpSquat => writeln!(f, "dumbbell_jump_squat"),
            PlyoExerciseName::DumbbellSplitJump => writeln!(f, "dumbbell_split_jump"),
            PlyoExerciseName::FrontKneeStrike => writeln!(f, "front_knee_strike"),
            PlyoExerciseName::WeightedFrontKneeStrike => writeln!(f, "weighted_front_knee_strike"),
            PlyoExerciseName::HighBoxJump => writeln!(f, "high_box_jump"),
            PlyoExerciseName::WeightedHighBoxJump => writeln!(f, "weighted_high_box_jump"),
            PlyoExerciseName::IsometricExplosiveBodyWeightJumpSquat => {
                writeln!(f, "isometric_explosive_body_weight_jump_squat")
            }
            PlyoExerciseName::WeightedIsometricExplosiveJumpSquat => {
                writeln!(f, "weighted_isometric_explosive_jump_squat")
            }
            PlyoExerciseName::LateralLeapAndHop => writeln!(f, "lateral_leap_and_hop"),
            PlyoExerciseName::WeightedLateralLeapAndHop => {
                writeln!(f, "weighted_lateral_leap_and_hop")
            }
            PlyoExerciseName::LateralPlyoSquats => writeln!(f, "lateral_plyo_squats"),
            PlyoExerciseName::WeightedLateralPlyoSquats => {
                writeln!(f, "weighted_lateral_plyo_squats")
            }
            PlyoExerciseName::LateralSlide => writeln!(f, "lateral_slide"),
            PlyoExerciseName::WeightedLateralSlide => writeln!(f, "weighted_lateral_slide"),
            PlyoExerciseName::MedicineBallOverheadThrows => {
                writeln!(f, "medicine_ball_overhead_throws")
            }
            PlyoExerciseName::MedicineBallSideThrow => writeln!(f, "medicine_ball_side_throw"),
            PlyoExerciseName::MedicineBallSlam => writeln!(f, "medicine_ball_slam"),
            PlyoExerciseName::SideToSideMedicineBallThrows => {
                writeln!(f, "side_to_side_medicine_ball_throws")
            }
            PlyoExerciseName::SideToSideShuffleJump => writeln!(f, "side_to_side_shuffle_jump"),
            PlyoExerciseName::WeightedSideToSideShuffleJump => {
                writeln!(f, "weighted_side_to_side_shuffle_jump")
            }
            PlyoExerciseName::SquatJumpOntoBox => writeln!(f, "squat_jump_onto_box"),
            PlyoExerciseName::WeightedSquatJumpOntoBox => {
                writeln!(f, "weighted_squat_jump_onto_box")
            }
            PlyoExerciseName::SquatJumpsInAndOut => writeln!(f, "squat_jumps_in_and_out"),
            PlyoExerciseName::WeightedSquatJumpsInAndOut => {
                writeln!(f, "weighted_squat_jumps_in_and_out")
            }
            PlyoExerciseName::UnknownVariant(value) => writeln!(f, "unknown_variant_{}", *value),
        }
    }
}
impl convert::From<u16> for PlyoExerciseName {
    fn from(value: u16) -> Self {
        match value {
            0 => PlyoExerciseName::AlternatingJumpLunge,
            1 => PlyoExerciseName::WeightedAlternatingJumpLunge,
            2 => PlyoExerciseName::BarbellJumpSquat,
            3 => PlyoExerciseName::BodyWeightJumpSquat,
            4 => PlyoExerciseName::WeightedJumpSquat,
            5 => PlyoExerciseName::CrossKneeStrike,
            6 => PlyoExerciseName::WeightedCrossKneeStrike,
            7 => PlyoExerciseName::DepthJump,
            8 => PlyoExerciseName::WeightedDepthJump,
            9 => PlyoExerciseName::DumbbellJumpSquat,
            10 => PlyoExerciseName::DumbbellSplitJump,
            11 => PlyoExerciseName::FrontKneeStrike,
            12 => PlyoExerciseName::WeightedFrontKneeStrike,
            13 => PlyoExerciseName::HighBoxJump,
            14 => PlyoExerciseName::WeightedHighBoxJump,
            15 => PlyoExerciseName::IsometricExplosiveBodyWeightJumpSquat,
            16 => PlyoExerciseName::WeightedIsometricExplosiveJumpSquat,
            17 => PlyoExerciseName::LateralLeapAndHop,
            18 => PlyoExerciseName::WeightedLateralLeapAndHop,
            19 => PlyoExerciseName::LateralPlyoSquats,
            20 => PlyoExerciseName::WeightedLateralPlyoSquats,
            21 => PlyoExerciseName::LateralSlide,
            22 => PlyoExerciseName::WeightedLateralSlide,
            23 => PlyoExerciseName::MedicineBallOverheadThrows,
            24 => PlyoExerciseName::MedicineBallSideThrow,
            25 => PlyoExerciseName::MedicineBallSlam,
            26 => PlyoExerciseName::SideToSideMedicineBallThrows,
            27 => PlyoExerciseName::SideToSideShuffleJump,
            28 => PlyoExerciseName::WeightedSideToSideShuffleJump,
            29 => PlyoExerciseName::SquatJumpOntoBox,
            30 => PlyoExerciseName::WeightedSquatJumpOntoBox,
            31 => PlyoExerciseName::SquatJumpsInAndOut,
            32 => PlyoExerciseName::WeightedSquatJumpsInAndOut,
            _ => PlyoExerciseName::UnknownVariant(value),
        }
    }
}
impl convert::From<i64> for PlyoExerciseName {
    fn from(value: i64) -> Self {
        PlyoExerciseName::from(value as u16)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum PullUpExerciseName {
    BandedPullUps,
    Name30DegreeLatPulldown,
    BandAssistedChinUp,
    CloseGripChinUp,
    WeightedCloseGripChinUp,
    CloseGripLatPulldown,
    CrossoverChinUp,
    WeightedCrossoverChinUp,
    EzBarPullover,
    HangingHurdle,
    WeightedHangingHurdle,
    KneelingLatPulldown,
    KneelingUnderhandGripLatPulldown,
    LatPulldown,
    MixedGripChinUp,
    WeightedMixedGripChinUp,
    MixedGripPullUp,
    WeightedMixedGripPullUp,
    ReverseGripPulldown,
    StandingCablePullover,
    StraightArmPulldown,
    SwissBallEzBarPullover,
    TowelPullUp,
    WeightedTowelPullUp,
    WeightedPullUp,
    WideGripLatPulldown,
    WideGripPullUp,
    WeightedWideGripPullUp,
    BurpeePullUp,
    WeightedBurpeePullUp,
    JumpingPullUps,
    WeightedJumpingPullUps,
    KippingPullUp,
    WeightedKippingPullUp,
    LPullUp,
    WeightedLPullUp,
    SuspendedChinUp,
    WeightedSuspendedChinUp,
    PullUp,
    UnknownVariant(u16),
}
impl PullUpExerciseName {
    pub fn as_u16(self) -> u16 {
        match self {
            PullUpExerciseName::BandedPullUps => 0,
            PullUpExerciseName::Name30DegreeLatPulldown => 1,
            PullUpExerciseName::BandAssistedChinUp => 2,
            PullUpExerciseName::CloseGripChinUp => 3,
            PullUpExerciseName::WeightedCloseGripChinUp => 4,
            PullUpExerciseName::CloseGripLatPulldown => 5,
            PullUpExerciseName::CrossoverChinUp => 6,
            PullUpExerciseName::WeightedCrossoverChinUp => 7,
            PullUpExerciseName::EzBarPullover => 8,
            PullUpExerciseName::HangingHurdle => 9,
            PullUpExerciseName::WeightedHangingHurdle => 10,
            PullUpExerciseName::KneelingLatPulldown => 11,
            PullUpExerciseName::KneelingUnderhandGripLatPulldown => 12,
            PullUpExerciseName::LatPulldown => 13,
            PullUpExerciseName::MixedGripChinUp => 14,
            PullUpExerciseName::WeightedMixedGripChinUp => 15,
            PullUpExerciseName::MixedGripPullUp => 16,
            PullUpExerciseName::WeightedMixedGripPullUp => 17,
            PullUpExerciseName::ReverseGripPulldown => 18,
            PullUpExerciseName::StandingCablePullover => 19,
            PullUpExerciseName::StraightArmPulldown => 20,
            PullUpExerciseName::SwissBallEzBarPullover => 21,
            PullUpExerciseName::TowelPullUp => 22,
            PullUpExerciseName::WeightedTowelPullUp => 23,
            PullUpExerciseName::WeightedPullUp => 24,
            PullUpExerciseName::WideGripLatPulldown => 25,
            PullUpExerciseName::WideGripPullUp => 26,
            PullUpExerciseName::WeightedWideGripPullUp => 27,
            PullUpExerciseName::BurpeePullUp => 28,
            PullUpExerciseName::WeightedBurpeePullUp => 29,
            PullUpExerciseName::JumpingPullUps => 30,
            PullUpExerciseName::WeightedJumpingPullUps => 31,
            PullUpExerciseName::KippingPullUp => 32,
            PullUpExerciseName::WeightedKippingPullUp => 33,
            PullUpExerciseName::LPullUp => 34,
            PullUpExerciseName::WeightedLPullUp => 35,
            PullUpExerciseName::SuspendedChinUp => 36,
            PullUpExerciseName::WeightedSuspendedChinUp => 37,
            PullUpExerciseName::PullUp => 38,
            PullUpExerciseName::UnknownVariant(value) => value,
        }
    }
    pub fn as_i64(self) -> i64 {
        self.as_u16() as i64
    }
}
impl fmt::Display for PullUpExerciseName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            PullUpExerciseName::BandedPullUps => writeln!(f, "banded_pull_ups"),
            PullUpExerciseName::Name30DegreeLatPulldown => writeln!(f, "30_degree_lat_pulldown"),
            PullUpExerciseName::BandAssistedChinUp => writeln!(f, "band_assisted_chin_up"),
            PullUpExerciseName::CloseGripChinUp => writeln!(f, "close_grip_chin_up"),
            PullUpExerciseName::WeightedCloseGripChinUp => {
                writeln!(f, "weighted_close_grip_chin_up")
            }
            PullUpExerciseName::CloseGripLatPulldown => writeln!(f, "close_grip_lat_pulldown"),
            PullUpExerciseName::CrossoverChinUp => writeln!(f, "crossover_chin_up"),
            PullUpExerciseName::WeightedCrossoverChinUp => {
                writeln!(f, "weighted_crossover_chin_up")
            }
            PullUpExerciseName::EzBarPullover => writeln!(f, "ez_bar_pullover"),
            PullUpExerciseName::HangingHurdle => writeln!(f, "hanging_hurdle"),
            PullUpExerciseName::WeightedHangingHurdle => writeln!(f, "weighted_hanging_hurdle"),
            PullUpExerciseName::KneelingLatPulldown => writeln!(f, "kneeling_lat_pulldown"),
            PullUpExerciseName::KneelingUnderhandGripLatPulldown => {
                writeln!(f, "kneeling_underhand_grip_lat_pulldown")
            }
            PullUpExerciseName::LatPulldown => writeln!(f, "lat_pulldown"),
            PullUpExerciseName::MixedGripChinUp => writeln!(f, "mixed_grip_chin_up"),
            PullUpExerciseName::WeightedMixedGripChinUp => {
                writeln!(f, "weighted_mixed_grip_chin_up")
            }
            PullUpExerciseName::MixedGripPullUp => writeln!(f, "mixed_grip_pull_up"),
            PullUpExerciseName::WeightedMixedGripPullUp => {
                writeln!(f, "weighted_mixed_grip_pull_up")
            }
            PullUpExerciseName::ReverseGripPulldown => writeln!(f, "reverse_grip_pulldown"),
            PullUpExerciseName::StandingCablePullover => writeln!(f, "standing_cable_pullover"),
            PullUpExerciseName::StraightArmPulldown => writeln!(f, "straight_arm_pulldown"),
            PullUpExerciseName::SwissBallEzBarPullover => writeln!(f, "swiss_ball_ez_bar_pullover"),
            PullUpExerciseName::TowelPullUp => writeln!(f, "towel_pull_up"),
            PullUpExerciseName::WeightedTowelPullUp => writeln!(f, "weighted_towel_pull_up"),
            PullUpExerciseName::WeightedPullUp => writeln!(f, "weighted_pull_up"),
            PullUpExerciseName::WideGripLatPulldown => writeln!(f, "wide_grip_lat_pulldown"),
            PullUpExerciseName::WideGripPullUp => writeln!(f, "wide_grip_pull_up"),
            PullUpExerciseName::WeightedWideGripPullUp => writeln!(f, "weighted_wide_grip_pull_up"),
            PullUpExerciseName::BurpeePullUp => writeln!(f, "burpee_pull_up"),
            PullUpExerciseName::WeightedBurpeePullUp => writeln!(f, "weighted_burpee_pull_up"),
            PullUpExerciseName::JumpingPullUps => writeln!(f, "jumping_pull_ups"),
            PullUpExerciseName::WeightedJumpingPullUps => writeln!(f, "weighted_jumping_pull_ups"),
            PullUpExerciseName::KippingPullUp => writeln!(f, "kipping_pull_up"),
            PullUpExerciseName::WeightedKippingPullUp => writeln!(f, "weighted_kipping_pull_up"),
            PullUpExerciseName::LPullUp => writeln!(f, "l_pull_up"),
            PullUpExerciseName::WeightedLPullUp => writeln!(f, "weighted_l_pull_up"),
            PullUpExerciseName::SuspendedChinUp => writeln!(f, "suspended_chin_up"),
            PullUpExerciseName::WeightedSuspendedChinUp => {
                writeln!(f, "weighted_suspended_chin_up")
            }
            PullUpExerciseName::PullUp => writeln!(f, "pull_up"),
            PullUpExerciseName::UnknownVariant(value) => writeln!(f, "unknown_variant_{}", *value),
        }
    }
}
impl convert::From<u16> for PullUpExerciseName {
    fn from(value: u16) -> Self {
        match value {
            0 => PullUpExerciseName::BandedPullUps,
            1 => PullUpExerciseName::Name30DegreeLatPulldown,
            2 => PullUpExerciseName::BandAssistedChinUp,
            3 => PullUpExerciseName::CloseGripChinUp,
            4 => PullUpExerciseName::WeightedCloseGripChinUp,
            5 => PullUpExerciseName::CloseGripLatPulldown,
            6 => PullUpExerciseName::CrossoverChinUp,
            7 => PullUpExerciseName::WeightedCrossoverChinUp,
            8 => PullUpExerciseName::EzBarPullover,
            9 => PullUpExerciseName::HangingHurdle,
            10 => PullUpExerciseName::WeightedHangingHurdle,
            11 => PullUpExerciseName::KneelingLatPulldown,
            12 => PullUpExerciseName::KneelingUnderhandGripLatPulldown,
            13 => PullUpExerciseName::LatPulldown,
            14 => PullUpExerciseName::MixedGripChinUp,
            15 => PullUpExerciseName::WeightedMixedGripChinUp,
            16 => PullUpExerciseName::MixedGripPullUp,
            17 => PullUpExerciseName::WeightedMixedGripPullUp,
            18 => PullUpExerciseName::ReverseGripPulldown,
            19 => PullUpExerciseName::StandingCablePullover,
            20 => PullUpExerciseName::StraightArmPulldown,
            21 => PullUpExerciseName::SwissBallEzBarPullover,
            22 => PullUpExerciseName::TowelPullUp,
            23 => PullUpExerciseName::WeightedTowelPullUp,
            24 => PullUpExerciseName::WeightedPullUp,
            25 => PullUpExerciseName::WideGripLatPulldown,
            26 => PullUpExerciseName::WideGripPullUp,
            27 => PullUpExerciseName::WeightedWideGripPullUp,
            28 => PullUpExerciseName::BurpeePullUp,
            29 => PullUpExerciseName::WeightedBurpeePullUp,
            30 => PullUpExerciseName::JumpingPullUps,
            31 => PullUpExerciseName::WeightedJumpingPullUps,
            32 => PullUpExerciseName::KippingPullUp,
            33 => PullUpExerciseName::WeightedKippingPullUp,
            34 => PullUpExerciseName::LPullUp,
            35 => PullUpExerciseName::WeightedLPullUp,
            36 => PullUpExerciseName::SuspendedChinUp,
            37 => PullUpExerciseName::WeightedSuspendedChinUp,
            38 => PullUpExerciseName::PullUp,
            _ => PullUpExerciseName::UnknownVariant(value),
        }
    }
}
impl convert::From<i64> for PullUpExerciseName {
    fn from(value: i64) -> Self {
        PullUpExerciseName::from(value as u16)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum PushUpExerciseName {
    ChestPressWithBand,
    AlternatingStaggeredPushUp,
    WeightedAlternatingStaggeredPushUp,
    AlternatingHandsMedicineBallPushUp,
    WeightedAlternatingHandsMedicineBallPushUp,
    BosuBallPushUp,
    WeightedBosuBallPushUp,
    ClappingPushUp,
    WeightedClappingPushUp,
    CloseGripMedicineBallPushUp,
    WeightedCloseGripMedicineBallPushUp,
    CloseHandsPushUp,
    WeightedCloseHandsPushUp,
    DeclinePushUp,
    WeightedDeclinePushUp,
    DiamondPushUp,
    WeightedDiamondPushUp,
    ExplosiveCrossoverPushUp,
    WeightedExplosiveCrossoverPushUp,
    ExplosivePushUp,
    WeightedExplosivePushUp,
    FeetElevatedSideToSidePushUp,
    WeightedFeetElevatedSideToSidePushUp,
    HandReleasePushUp,
    WeightedHandReleasePushUp,
    HandstandPushUp,
    WeightedHandstandPushUp,
    InclinePushUp,
    WeightedInclinePushUp,
    IsometricExplosivePushUp,
    WeightedIsometricExplosivePushUp,
    JudoPushUp,
    WeightedJudoPushUp,
    KneelingPushUp,
    WeightedKneelingPushUp,
    MedicineBallChestPass,
    MedicineBallPushUp,
    WeightedMedicineBallPushUp,
    OneArmPushUp,
    WeightedOneArmPushUp,
    WeightedPushUp,
    PushUpAndRow,
    WeightedPushUpAndRow,
    PushUpPlus,
    WeightedPushUpPlus,
    PushUpWithFeetOnSwissBall,
    WeightedPushUpWithFeetOnSwissBall,
    PushUpWithOneHandOnMedicineBall,
    WeightedPushUpWithOneHandOnMedicineBall,
    ShoulderPushUp,
    WeightedShoulderPushUp,
    SingleArmMedicineBallPushUp,
    WeightedSingleArmMedicineBallPushUp,
    SpidermanPushUp,
    WeightedSpidermanPushUp,
    StackedFeetPushUp,
    WeightedStackedFeetPushUp,
    StaggeredHandsPushUp,
    WeightedStaggeredHandsPushUp,
    SuspendedPushUp,
    WeightedSuspendedPushUp,
    SwissBallPushUp,
    WeightedSwissBallPushUp,
    SwissBallPushUpPlus,
    WeightedSwissBallPushUpPlus,
    TPushUp,
    WeightedTPushUp,
    TripleStopPushUp,
    WeightedTripleStopPushUp,
    WideHandsPushUp,
    WeightedWideHandsPushUp,
    ParalletteHandstandPushUp,
    WeightedParalletteHandstandPushUp,
    RingHandstandPushUp,
    WeightedRingHandstandPushUp,
    RingPushUp,
    WeightedRingPushUp,
    PushUp,
    PilatesPushup,
    UnknownVariant(u16),
}
impl PushUpExerciseName {
    pub fn as_u16(self) -> u16 {
        match self {
            PushUpExerciseName::ChestPressWithBand => 0,
            PushUpExerciseName::AlternatingStaggeredPushUp => 1,
            PushUpExerciseName::WeightedAlternatingStaggeredPushUp => 2,
            PushUpExerciseName::AlternatingHandsMedicineBallPushUp => 3,
            PushUpExerciseName::WeightedAlternatingHandsMedicineBallPushUp => 4,
            PushUpExerciseName::BosuBallPushUp => 5,
            PushUpExerciseName::WeightedBosuBallPushUp => 6,
            PushUpExerciseName::ClappingPushUp => 7,
            PushUpExerciseName::WeightedClappingPushUp => 8,
            PushUpExerciseName::CloseGripMedicineBallPushUp => 9,
            PushUpExerciseName::WeightedCloseGripMedicineBallPushUp => 10,
            PushUpExerciseName::CloseHandsPushUp => 11,
            PushUpExerciseName::WeightedCloseHandsPushUp => 12,
            PushUpExerciseName::DeclinePushUp => 13,
            PushUpExerciseName::WeightedDeclinePushUp => 14,
            PushUpExerciseName::DiamondPushUp => 15,
            PushUpExerciseName::WeightedDiamondPushUp => 16,
            PushUpExerciseName::ExplosiveCrossoverPushUp => 17,
            PushUpExerciseName::WeightedExplosiveCrossoverPushUp => 18,
            PushUpExerciseName::ExplosivePushUp => 19,
            PushUpExerciseName::WeightedExplosivePushUp => 20,
            PushUpExerciseName::FeetElevatedSideToSidePushUp => 21,
            PushUpExerciseName::WeightedFeetElevatedSideToSidePushUp => 22,
            PushUpExerciseName::HandReleasePushUp => 23,
            PushUpExerciseName::WeightedHandReleasePushUp => 24,
            PushUpExerciseName::HandstandPushUp => 25,
            PushUpExerciseName::WeightedHandstandPushUp => 26,
            PushUpExerciseName::InclinePushUp => 27,
            PushUpExerciseName::WeightedInclinePushUp => 28,
            PushUpExerciseName::IsometricExplosivePushUp => 29,
            PushUpExerciseName::WeightedIsometricExplosivePushUp => 30,
            PushUpExerciseName::JudoPushUp => 31,
            PushUpExerciseName::WeightedJudoPushUp => 32,
            PushUpExerciseName::KneelingPushUp => 33,
            PushUpExerciseName::WeightedKneelingPushUp => 34,
            PushUpExerciseName::MedicineBallChestPass => 35,
            PushUpExerciseName::MedicineBallPushUp => 36,
            PushUpExerciseName::WeightedMedicineBallPushUp => 37,
            PushUpExerciseName::OneArmPushUp => 38,
            PushUpExerciseName::WeightedOneArmPushUp => 39,
            PushUpExerciseName::WeightedPushUp => 40,
            PushUpExerciseName::PushUpAndRow => 41,
            PushUpExerciseName::WeightedPushUpAndRow => 42,
            PushUpExerciseName::PushUpPlus => 43,
            PushUpExerciseName::WeightedPushUpPlus => 44,
            PushUpExerciseName::PushUpWithFeetOnSwissBall => 45,
            PushUpExerciseName::WeightedPushUpWithFeetOnSwissBall => 46,
            PushUpExerciseName::PushUpWithOneHandOnMedicineBall => 47,
            PushUpExerciseName::WeightedPushUpWithOneHandOnMedicineBall => 48,
            PushUpExerciseName::ShoulderPushUp => 49,
            PushUpExerciseName::WeightedShoulderPushUp => 50,
            PushUpExerciseName::SingleArmMedicineBallPushUp => 51,
            PushUpExerciseName::WeightedSingleArmMedicineBallPushUp => 52,
            PushUpExerciseName::SpidermanPushUp => 53,
            PushUpExerciseName::WeightedSpidermanPushUp => 54,
            PushUpExerciseName::StackedFeetPushUp => 55,
            PushUpExerciseName::WeightedStackedFeetPushUp => 56,
            PushUpExerciseName::StaggeredHandsPushUp => 57,
            PushUpExerciseName::WeightedStaggeredHandsPushUp => 58,
            PushUpExerciseName::SuspendedPushUp => 59,
            PushUpExerciseName::WeightedSuspendedPushUp => 60,
            PushUpExerciseName::SwissBallPushUp => 61,
            PushUpExerciseName::WeightedSwissBallPushUp => 62,
            PushUpExerciseName::SwissBallPushUpPlus => 63,
            PushUpExerciseName::WeightedSwissBallPushUpPlus => 64,
            PushUpExerciseName::TPushUp => 65,
            PushUpExerciseName::WeightedTPushUp => 66,
            PushUpExerciseName::TripleStopPushUp => 67,
            PushUpExerciseName::WeightedTripleStopPushUp => 68,
            PushUpExerciseName::WideHandsPushUp => 69,
            PushUpExerciseName::WeightedWideHandsPushUp => 70,
            PushUpExerciseName::ParalletteHandstandPushUp => 71,
            PushUpExerciseName::WeightedParalletteHandstandPushUp => 72,
            PushUpExerciseName::RingHandstandPushUp => 73,
            PushUpExerciseName::WeightedRingHandstandPushUp => 74,
            PushUpExerciseName::RingPushUp => 75,
            PushUpExerciseName::WeightedRingPushUp => 76,
            PushUpExerciseName::PushUp => 77,
            PushUpExerciseName::PilatesPushup => 78,
            PushUpExerciseName::UnknownVariant(value) => value,
        }
    }
    pub fn as_i64(self) -> i64 {
        self.as_u16() as i64
    }
}
impl fmt::Display for PushUpExerciseName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            PushUpExerciseName::ChestPressWithBand => writeln!(f, "chest_press_with_band"),
            PushUpExerciseName::AlternatingStaggeredPushUp => {
                writeln!(f, "alternating_staggered_push_up")
            }
            PushUpExerciseName::WeightedAlternatingStaggeredPushUp => {
                writeln!(f, "weighted_alternating_staggered_push_up")
            }
            PushUpExerciseName::AlternatingHandsMedicineBallPushUp => {
                writeln!(f, "alternating_hands_medicine_ball_push_up")
            }
            PushUpExerciseName::WeightedAlternatingHandsMedicineBallPushUp => {
                writeln!(f, "weighted_alternating_hands_medicine_ball_push_up")
            }
            PushUpExerciseName::BosuBallPushUp => writeln!(f, "bosu_ball_push_up"),
            PushUpExerciseName::WeightedBosuBallPushUp => writeln!(f, "weighted_bosu_ball_push_up"),
            PushUpExerciseName::ClappingPushUp => writeln!(f, "clapping_push_up"),
            PushUpExerciseName::WeightedClappingPushUp => writeln!(f, "weighted_clapping_push_up"),
            PushUpExerciseName::CloseGripMedicineBallPushUp => {
                writeln!(f, "close_grip_medicine_ball_push_up")
            }
            PushUpExerciseName::WeightedCloseGripMedicineBallPushUp => {
                writeln!(f, "weighted_close_grip_medicine_ball_push_up")
            }
            PushUpExerciseName::CloseHandsPushUp => writeln!(f, "close_hands_push_up"),
            PushUpExerciseName::WeightedCloseHandsPushUp => {
                writeln!(f, "weighted_close_hands_push_up")
            }
            PushUpExerciseName::DeclinePushUp => writeln!(f, "decline_push_up"),
            PushUpExerciseName::WeightedDeclinePushUp => writeln!(f, "weighted_decline_push_up"),
            PushUpExerciseName::DiamondPushUp => writeln!(f, "diamond_push_up"),
            PushUpExerciseName::WeightedDiamondPushUp => writeln!(f, "weighted_diamond_push_up"),
            PushUpExerciseName::ExplosiveCrossoverPushUp => {
                writeln!(f, "explosive_crossover_push_up")
            }
            PushUpExerciseName::WeightedExplosiveCrossoverPushUp => {
                writeln!(f, "weighted_explosive_crossover_push_up")
            }
            PushUpExerciseName::ExplosivePushUp => writeln!(f, "explosive_push_up"),
            PushUpExerciseName::WeightedExplosivePushUp => {
                writeln!(f, "weighted_explosive_push_up")
            }
            PushUpExerciseName::FeetElevatedSideToSidePushUp => {
                writeln!(f, "feet_elevated_side_to_side_push_up")
            }
            PushUpExerciseName::WeightedFeetElevatedSideToSidePushUp => {
                writeln!(f, "weighted_feet_elevated_side_to_side_push_up")
            }
            PushUpExerciseName::HandReleasePushUp => writeln!(f, "hand_release_push_up"),
            PushUpExerciseName::WeightedHandReleasePushUp => {
                writeln!(f, "weighted_hand_release_push_up")
            }
            PushUpExerciseName::HandstandPushUp => writeln!(f, "handstand_push_up"),
            PushUpExerciseName::WeightedHandstandPushUp => {
                writeln!(f, "weighted_handstand_push_up")
            }
            PushUpExerciseName::InclinePushUp => writeln!(f, "incline_push_up"),
            PushUpExerciseName::WeightedInclinePushUp => writeln!(f, "weighted_incline_push_up"),
            PushUpExerciseName::IsometricExplosivePushUp => {
                writeln!(f, "isometric_explosive_push_up")
            }
            PushUpExerciseName::WeightedIsometricExplosivePushUp => {
                writeln!(f, "weighted_isometric_explosive_push_up")
            }
            PushUpExerciseName::JudoPushUp => writeln!(f, "judo_push_up"),
            PushUpExerciseName::WeightedJudoPushUp => writeln!(f, "weighted_judo_push_up"),
            PushUpExerciseName::KneelingPushUp => writeln!(f, "kneeling_push_up"),
            PushUpExerciseName::WeightedKneelingPushUp => writeln!(f, "weighted_kneeling_push_up"),
            PushUpExerciseName::MedicineBallChestPass => writeln!(f, "medicine_ball_chest_pass"),
            PushUpExerciseName::MedicineBallPushUp => writeln!(f, "medicine_ball_push_up"),
            PushUpExerciseName::WeightedMedicineBallPushUp => {
                writeln!(f, "weighted_medicine_ball_push_up")
            }
            PushUpExerciseName::OneArmPushUp => writeln!(f, "one_arm_push_up"),
            PushUpExerciseName::WeightedOneArmPushUp => writeln!(f, "weighted_one_arm_push_up"),
            PushUpExerciseName::WeightedPushUp => writeln!(f, "weighted_push_up"),
            PushUpExerciseName::PushUpAndRow => writeln!(f, "push_up_and_row"),
            PushUpExerciseName::WeightedPushUpAndRow => writeln!(f, "weighted_push_up_and_row"),
            PushUpExerciseName::PushUpPlus => writeln!(f, "push_up_plus"),
            PushUpExerciseName::WeightedPushUpPlus => writeln!(f, "weighted_push_up_plus"),
            PushUpExerciseName::PushUpWithFeetOnSwissBall => {
                writeln!(f, "push_up_with_feet_on_swiss_ball")
            }
            PushUpExerciseName::WeightedPushUpWithFeetOnSwissBall => {
                writeln!(f, "weighted_push_up_with_feet_on_swiss_ball")
            }
            PushUpExerciseName::PushUpWithOneHandOnMedicineBall => {
                writeln!(f, "push_up_with_one_hand_on_medicine_ball")
            }
            PushUpExerciseName::WeightedPushUpWithOneHandOnMedicineBall => {
                writeln!(f, "weighted_push_up_with_one_hand_on_medicine_ball")
            }
            PushUpExerciseName::ShoulderPushUp => writeln!(f, "shoulder_push_up"),
            PushUpExerciseName::WeightedShoulderPushUp => writeln!(f, "weighted_shoulder_push_up"),
            PushUpExerciseName::SingleArmMedicineBallPushUp => {
                writeln!(f, "single_arm_medicine_ball_push_up")
            }
            PushUpExerciseName::WeightedSingleArmMedicineBallPushUp => {
                writeln!(f, "weighted_single_arm_medicine_ball_push_up")
            }
            PushUpExerciseName::SpidermanPushUp => writeln!(f, "spiderman_push_up"),
            PushUpExerciseName::WeightedSpidermanPushUp => {
                writeln!(f, "weighted_spiderman_push_up")
            }
            PushUpExerciseName::StackedFeetPushUp => writeln!(f, "stacked_feet_push_up"),
            PushUpExerciseName::WeightedStackedFeetPushUp => {
                writeln!(f, "weighted_stacked_feet_push_up")
            }
            PushUpExerciseName::StaggeredHandsPushUp => writeln!(f, "staggered_hands_push_up"),
            PushUpExerciseName::WeightedStaggeredHandsPushUp => {
                writeln!(f, "weighted_staggered_hands_push_up")
            }
            PushUpExerciseName::SuspendedPushUp => writeln!(f, "suspended_push_up"),
            PushUpExerciseName::WeightedSuspendedPushUp => {
                writeln!(f, "weighted_suspended_push_up")
            }
            PushUpExerciseName::SwissBallPushUp => writeln!(f, "swiss_ball_push_up"),
            PushUpExerciseName::WeightedSwissBallPushUp => {
                writeln!(f, "weighted_swiss_ball_push_up")
            }
            PushUpExerciseName::SwissBallPushUpPlus => writeln!(f, "swiss_ball_push_up_plus"),
            PushUpExerciseName::WeightedSwissBallPushUpPlus => {
                writeln!(f, "weighted_swiss_ball_push_up_plus")
            }
            PushUpExerciseName::TPushUp => writeln!(f, "t_push_up"),
            PushUpExerciseName::WeightedTPushUp => writeln!(f, "weighted_t_push_up"),
            PushUpExerciseName::TripleStopPushUp => writeln!(f, "triple_stop_push_up"),
            PushUpExerciseName::WeightedTripleStopPushUp => {
                writeln!(f, "weighted_triple_stop_push_up")
            }
            PushUpExerciseName::WideHandsPushUp => writeln!(f, "wide_hands_push_up"),
            PushUpExerciseName::WeightedWideHandsPushUp => {
                writeln!(f, "weighted_wide_hands_push_up")
            }
            PushUpExerciseName::ParalletteHandstandPushUp => {
                writeln!(f, "parallette_handstand_push_up")
            }
            PushUpExerciseName::WeightedParalletteHandstandPushUp => {
                writeln!(f, "weighted_parallette_handstand_push_up")
            }
            PushUpExerciseName::RingHandstandPushUp => writeln!(f, "ring_handstand_push_up"),
            PushUpExerciseName::WeightedRingHandstandPushUp => {
                writeln!(f, "weighted_ring_handstand_push_up")
            }
            PushUpExerciseName::RingPushUp => writeln!(f, "ring_push_up"),
            PushUpExerciseName::WeightedRingPushUp => writeln!(f, "weighted_ring_push_up"),
            PushUpExerciseName::PushUp => writeln!(f, "push_up"),
            PushUpExerciseName::PilatesPushup => writeln!(f, "pilates_pushup"),
            PushUpExerciseName::UnknownVariant(value) => writeln!(f, "unknown_variant_{}", *value),
        }
    }
}
impl convert::From<u16> for PushUpExerciseName {
    fn from(value: u16) -> Self {
        match value {
            0 => PushUpExerciseName::ChestPressWithBand,
            1 => PushUpExerciseName::AlternatingStaggeredPushUp,
            2 => PushUpExerciseName::WeightedAlternatingStaggeredPushUp,
            3 => PushUpExerciseName::AlternatingHandsMedicineBallPushUp,
            4 => PushUpExerciseName::WeightedAlternatingHandsMedicineBallPushUp,
            5 => PushUpExerciseName::BosuBallPushUp,
            6 => PushUpExerciseName::WeightedBosuBallPushUp,
            7 => PushUpExerciseName::ClappingPushUp,
            8 => PushUpExerciseName::WeightedClappingPushUp,
            9 => PushUpExerciseName::CloseGripMedicineBallPushUp,
            10 => PushUpExerciseName::WeightedCloseGripMedicineBallPushUp,
            11 => PushUpExerciseName::CloseHandsPushUp,
            12 => PushUpExerciseName::WeightedCloseHandsPushUp,
            13 => PushUpExerciseName::DeclinePushUp,
            14 => PushUpExerciseName::WeightedDeclinePushUp,
            15 => PushUpExerciseName::DiamondPushUp,
            16 => PushUpExerciseName::WeightedDiamondPushUp,
            17 => PushUpExerciseName::ExplosiveCrossoverPushUp,
            18 => PushUpExerciseName::WeightedExplosiveCrossoverPushUp,
            19 => PushUpExerciseName::ExplosivePushUp,
            20 => PushUpExerciseName::WeightedExplosivePushUp,
            21 => PushUpExerciseName::FeetElevatedSideToSidePushUp,
            22 => PushUpExerciseName::WeightedFeetElevatedSideToSidePushUp,
            23 => PushUpExerciseName::HandReleasePushUp,
            24 => PushUpExerciseName::WeightedHandReleasePushUp,
            25 => PushUpExerciseName::HandstandPushUp,
            26 => PushUpExerciseName::WeightedHandstandPushUp,
            27 => PushUpExerciseName::InclinePushUp,
            28 => PushUpExerciseName::WeightedInclinePushUp,
            29 => PushUpExerciseName::IsometricExplosivePushUp,
            30 => PushUpExerciseName::WeightedIsometricExplosivePushUp,
            31 => PushUpExerciseName::JudoPushUp,
            32 => PushUpExerciseName::WeightedJudoPushUp,
            33 => PushUpExerciseName::KneelingPushUp,
            34 => PushUpExerciseName::WeightedKneelingPushUp,
            35 => PushUpExerciseName::MedicineBallChestPass,
            36 => PushUpExerciseName::MedicineBallPushUp,
            37 => PushUpExerciseName::WeightedMedicineBallPushUp,
            38 => PushUpExerciseName::OneArmPushUp,
            39 => PushUpExerciseName::WeightedOneArmPushUp,
            40 => PushUpExerciseName::WeightedPushUp,
            41 => PushUpExerciseName::PushUpAndRow,
            42 => PushUpExerciseName::WeightedPushUpAndRow,
            43 => PushUpExerciseName::PushUpPlus,
            44 => PushUpExerciseName::WeightedPushUpPlus,
            45 => PushUpExerciseName::PushUpWithFeetOnSwissBall,
            46 => PushUpExerciseName::WeightedPushUpWithFeetOnSwissBall,
            47 => PushUpExerciseName::PushUpWithOneHandOnMedicineBall,
            48 => PushUpExerciseName::WeightedPushUpWithOneHandOnMedicineBall,
            49 => PushUpExerciseName::ShoulderPushUp,
            50 => PushUpExerciseName::WeightedShoulderPushUp,
            51 => PushUpExerciseName::SingleArmMedicineBallPushUp,
            52 => PushUpExerciseName::WeightedSingleArmMedicineBallPushUp,
            53 => PushUpExerciseName::SpidermanPushUp,
            54 => PushUpExerciseName::WeightedSpidermanPushUp,
            55 => PushUpExerciseName::StackedFeetPushUp,
            56 => PushUpExerciseName::WeightedStackedFeetPushUp,
            57 => PushUpExerciseName::StaggeredHandsPushUp,
            58 => PushUpExerciseName::WeightedStaggeredHandsPushUp,
            59 => PushUpExerciseName::SuspendedPushUp,
            60 => PushUpExerciseName::WeightedSuspendedPushUp,
            61 => PushUpExerciseName::SwissBallPushUp,
            62 => PushUpExerciseName::WeightedSwissBallPushUp,
            63 => PushUpExerciseName::SwissBallPushUpPlus,
            64 => PushUpExerciseName::WeightedSwissBallPushUpPlus,
            65 => PushUpExerciseName::TPushUp,
            66 => PushUpExerciseName::WeightedTPushUp,
            67 => PushUpExerciseName::TripleStopPushUp,
            68 => PushUpExerciseName::WeightedTripleStopPushUp,
            69 => PushUpExerciseName::WideHandsPushUp,
            70 => PushUpExerciseName::WeightedWideHandsPushUp,
            71 => PushUpExerciseName::ParalletteHandstandPushUp,
            72 => PushUpExerciseName::WeightedParalletteHandstandPushUp,
            73 => PushUpExerciseName::RingHandstandPushUp,
            74 => PushUpExerciseName::WeightedRingHandstandPushUp,
            75 => PushUpExerciseName::RingPushUp,
            76 => PushUpExerciseName::WeightedRingPushUp,
            77 => PushUpExerciseName::PushUp,
            78 => PushUpExerciseName::PilatesPushup,
            _ => PushUpExerciseName::UnknownVariant(value),
        }
    }
}
impl convert::From<i64> for PushUpExerciseName {
    fn from(value: i64) -> Self {
        PushUpExerciseName::from(value as u16)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum RowExerciseName {
    BarbellStraightLegDeadliftToRow,
    CableRowStanding,
    DumbbellRow,
    ElevatedFeetInvertedRow,
    WeightedElevatedFeetInvertedRow,
    FacePull,
    FacePullWithExternalRotation,
    InvertedRowWithFeetOnSwissBall,
    WeightedInvertedRowWithFeetOnSwissBall,
    KettlebellRow,
    ModifiedInvertedRow,
    WeightedModifiedInvertedRow,
    NeutralGripAlternatingDumbbellRow,
    OneArmBentOverRow,
    OneLeggedDumbbellRow,
    RenegadeRow,
    ReverseGripBarbellRow,
    RopeHandleCableRow,
    SeatedCableRow,
    SeatedDumbbellRow,
    SingleArmCableRow,
    SingleArmCableRowAndRotation,
    SingleArmInvertedRow,
    WeightedSingleArmInvertedRow,
    SingleArmNeutralGripDumbbellRow,
    SingleArmNeutralGripDumbbellRowAndRotation,
    SuspendedInvertedRow,
    WeightedSuspendedInvertedRow,
    TBarRow,
    TowelGripInvertedRow,
    WeightedTowelGripInvertedRow,
    UnderhandGripCableRow,
    VGripCableRow,
    WideGripSeatedCableRow,
    UnknownVariant(u16),
}
impl RowExerciseName {
    pub fn as_u16(self) -> u16 {
        match self {
            RowExerciseName::BarbellStraightLegDeadliftToRow => 0,
            RowExerciseName::CableRowStanding => 1,
            RowExerciseName::DumbbellRow => 2,
            RowExerciseName::ElevatedFeetInvertedRow => 3,
            RowExerciseName::WeightedElevatedFeetInvertedRow => 4,
            RowExerciseName::FacePull => 5,
            RowExerciseName::FacePullWithExternalRotation => 6,
            RowExerciseName::InvertedRowWithFeetOnSwissBall => 7,
            RowExerciseName::WeightedInvertedRowWithFeetOnSwissBall => 8,
            RowExerciseName::KettlebellRow => 9,
            RowExerciseName::ModifiedInvertedRow => 10,
            RowExerciseName::WeightedModifiedInvertedRow => 11,
            RowExerciseName::NeutralGripAlternatingDumbbellRow => 12,
            RowExerciseName::OneArmBentOverRow => 13,
            RowExerciseName::OneLeggedDumbbellRow => 14,
            RowExerciseName::RenegadeRow => 15,
            RowExerciseName::ReverseGripBarbellRow => 16,
            RowExerciseName::RopeHandleCableRow => 17,
            RowExerciseName::SeatedCableRow => 18,
            RowExerciseName::SeatedDumbbellRow => 19,
            RowExerciseName::SingleArmCableRow => 20,
            RowExerciseName::SingleArmCableRowAndRotation => 21,
            RowExerciseName::SingleArmInvertedRow => 22,
            RowExerciseName::WeightedSingleArmInvertedRow => 23,
            RowExerciseName::SingleArmNeutralGripDumbbellRow => 24,
            RowExerciseName::SingleArmNeutralGripDumbbellRowAndRotation => 25,
            RowExerciseName::SuspendedInvertedRow => 26,
            RowExerciseName::WeightedSuspendedInvertedRow => 27,
            RowExerciseName::TBarRow => 28,
            RowExerciseName::TowelGripInvertedRow => 29,
            RowExerciseName::WeightedTowelGripInvertedRow => 30,
            RowExerciseName::UnderhandGripCableRow => 31,
            RowExerciseName::VGripCableRow => 32,
            RowExerciseName::WideGripSeatedCableRow => 33,
            RowExerciseName::UnknownVariant(value) => value,
        }
    }
    pub fn as_i64(self) -> i64 {
        self.as_u16() as i64
    }
}
impl fmt::Display for RowExerciseName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            RowExerciseName::BarbellStraightLegDeadliftToRow => {
                writeln!(f, "barbell_straight_leg_deadlift_to_row")
            }
            RowExerciseName::CableRowStanding => writeln!(f, "cable_row_standing"),
            RowExerciseName::DumbbellRow => writeln!(f, "dumbbell_row"),
            RowExerciseName::ElevatedFeetInvertedRow => writeln!(f, "elevated_feet_inverted_row"),
            RowExerciseName::WeightedElevatedFeetInvertedRow => {
                writeln!(f, "weighted_elevated_feet_inverted_row")
            }
            RowExerciseName::FacePull => writeln!(f, "face_pull"),
            RowExerciseName::FacePullWithExternalRotation => {
                writeln!(f, "face_pull_with_external_rotation")
            }
            RowExerciseName::InvertedRowWithFeetOnSwissBall => {
                writeln!(f, "inverted_row_with_feet_on_swiss_ball")
            }
            RowExerciseName::WeightedInvertedRowWithFeetOnSwissBall => {
                writeln!(f, "weighted_inverted_row_with_feet_on_swiss_ball")
            }
            RowExerciseName::KettlebellRow => writeln!(f, "kettlebell_row"),
            RowExerciseName::ModifiedInvertedRow => writeln!(f, "modified_inverted_row"),
            RowExerciseName::WeightedModifiedInvertedRow => {
                writeln!(f, "weighted_modified_inverted_row")
            }
            RowExerciseName::NeutralGripAlternatingDumbbellRow => {
                writeln!(f, "neutral_grip_alternating_dumbbell_row")
            }
            RowExerciseName::OneArmBentOverRow => writeln!(f, "one_arm_bent_over_row"),
            RowExerciseName::OneLeggedDumbbellRow => writeln!(f, "one_legged_dumbbell_row"),
            RowExerciseName::RenegadeRow => writeln!(f, "renegade_row"),
            RowExerciseName::ReverseGripBarbellRow => writeln!(f, "reverse_grip_barbell_row"),
            RowExerciseName::RopeHandleCableRow => writeln!(f, "rope_handle_cable_row"),
            RowExerciseName::SeatedCableRow => writeln!(f, "seated_cable_row"),
            RowExerciseName::SeatedDumbbellRow => writeln!(f, "seated_dumbbell_row"),
            RowExerciseName::SingleArmCableRow => writeln!(f, "single_arm_cable_row"),
            RowExerciseName::SingleArmCableRowAndRotation => {
                writeln!(f, "single_arm_cable_row_and_rotation")
            }
            RowExerciseName::SingleArmInvertedRow => writeln!(f, "single_arm_inverted_row"),
            RowExerciseName::WeightedSingleArmInvertedRow => {
                writeln!(f, "weighted_single_arm_inverted_row")
            }
            RowExerciseName::SingleArmNeutralGripDumbbellRow => {
                writeln!(f, "single_arm_neutral_grip_dumbbell_row")
            }
            RowExerciseName::SingleArmNeutralGripDumbbellRowAndRotation => {
                writeln!(f, "single_arm_neutral_grip_dumbbell_row_and_rotation")
            }
            RowExerciseName::SuspendedInvertedRow => writeln!(f, "suspended_inverted_row"),
            RowExerciseName::WeightedSuspendedInvertedRow => {
                writeln!(f, "weighted_suspended_inverted_row")
            }
            RowExerciseName::TBarRow => writeln!(f, "t_bar_row"),
            RowExerciseName::TowelGripInvertedRow => writeln!(f, "towel_grip_inverted_row"),
            RowExerciseName::WeightedTowelGripInvertedRow => {
                writeln!(f, "weighted_towel_grip_inverted_row")
            }
            RowExerciseName::UnderhandGripCableRow => writeln!(f, "underhand_grip_cable_row"),
            RowExerciseName::VGripCableRow => writeln!(f, "v_grip_cable_row"),
            RowExerciseName::WideGripSeatedCableRow => writeln!(f, "wide_grip_seated_cable_row"),
            RowExerciseName::UnknownVariant(value) => writeln!(f, "unknown_variant_{}", *value),
        }
    }
}
impl convert::From<u16> for RowExerciseName {
    fn from(value: u16) -> Self {
        match value {
            0 => RowExerciseName::BarbellStraightLegDeadliftToRow,
            1 => RowExerciseName::CableRowStanding,
            2 => RowExerciseName::DumbbellRow,
            3 => RowExerciseName::ElevatedFeetInvertedRow,
            4 => RowExerciseName::WeightedElevatedFeetInvertedRow,
            5 => RowExerciseName::FacePull,
            6 => RowExerciseName::FacePullWithExternalRotation,
            7 => RowExerciseName::InvertedRowWithFeetOnSwissBall,
            8 => RowExerciseName::WeightedInvertedRowWithFeetOnSwissBall,
            9 => RowExerciseName::KettlebellRow,
            10 => RowExerciseName::ModifiedInvertedRow,
            11 => RowExerciseName::WeightedModifiedInvertedRow,
            12 => RowExerciseName::NeutralGripAlternatingDumbbellRow,
            13 => RowExerciseName::OneArmBentOverRow,
            14 => RowExerciseName::OneLeggedDumbbellRow,
            15 => RowExerciseName::RenegadeRow,
            16 => RowExerciseName::ReverseGripBarbellRow,
            17 => RowExerciseName::RopeHandleCableRow,
            18 => RowExerciseName::SeatedCableRow,
            19 => RowExerciseName::SeatedDumbbellRow,
            20 => RowExerciseName::SingleArmCableRow,
            21 => RowExerciseName::SingleArmCableRowAndRotation,
            22 => RowExerciseName::SingleArmInvertedRow,
            23 => RowExerciseName::WeightedSingleArmInvertedRow,
            24 => RowExerciseName::SingleArmNeutralGripDumbbellRow,
            25 => RowExerciseName::SingleArmNeutralGripDumbbellRowAndRotation,
            26 => RowExerciseName::SuspendedInvertedRow,
            27 => RowExerciseName::WeightedSuspendedInvertedRow,
            28 => RowExerciseName::TBarRow,
            29 => RowExerciseName::TowelGripInvertedRow,
            30 => RowExerciseName::WeightedTowelGripInvertedRow,
            31 => RowExerciseName::UnderhandGripCableRow,
            32 => RowExerciseName::VGripCableRow,
            33 => RowExerciseName::WideGripSeatedCableRow,
            _ => RowExerciseName::UnknownVariant(value),
        }
    }
}
impl convert::From<i64> for RowExerciseName {
    fn from(value: i64) -> Self {
        RowExerciseName::from(value as u16)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum ShoulderPressExerciseName {
    AlternatingDumbbellShoulderPress,
    ArnoldPress,
    BarbellFrontSquatToPushPress,
    BarbellPushPress,
    BarbellShoulderPress,
    DeadCurlPress,
    DumbbellAlternatingShoulderPressAndTwist,
    DumbbellHammerCurlToLungeToPress,
    DumbbellPushPress,
    FloorInvertedShoulderPress,
    WeightedFloorInvertedShoulderPress,
    InvertedShoulderPress,
    WeightedInvertedShoulderPress,
    OneArmPushPress,
    OverheadBarbellPress,
    OverheadDumbbellPress,
    SeatedBarbellShoulderPress,
    SeatedDumbbellShoulderPress,
    SingleArmDumbbellShoulderPress,
    SingleArmStepUpAndPress,
    SmithMachineOverheadPress,
    SplitStanceHammerCurlToPress,
    SwissBallDumbbellShoulderPress,
    WeightPlateFrontRaise,
    UnknownVariant(u16),
}
impl ShoulderPressExerciseName {
    pub fn as_u16(self) -> u16 {
        match self {
            ShoulderPressExerciseName::AlternatingDumbbellShoulderPress => 0,
            ShoulderPressExerciseName::ArnoldPress => 1,
            ShoulderPressExerciseName::BarbellFrontSquatToPushPress => 2,
            ShoulderPressExerciseName::BarbellPushPress => 3,
            ShoulderPressExerciseName::BarbellShoulderPress => 4,
            ShoulderPressExerciseName::DeadCurlPress => 5,
            ShoulderPressExerciseName::DumbbellAlternatingShoulderPressAndTwist => 6,
            ShoulderPressExerciseName::DumbbellHammerCurlToLungeToPress => 7,
            ShoulderPressExerciseName::DumbbellPushPress => 8,
            ShoulderPressExerciseName::FloorInvertedShoulderPress => 9,
            ShoulderPressExerciseName::WeightedFloorInvertedShoulderPress => 10,
            ShoulderPressExerciseName::InvertedShoulderPress => 11,
            ShoulderPressExerciseName::WeightedInvertedShoulderPress => 12,
            ShoulderPressExerciseName::OneArmPushPress => 13,
            ShoulderPressExerciseName::OverheadBarbellPress => 14,
            ShoulderPressExerciseName::OverheadDumbbellPress => 15,
            ShoulderPressExerciseName::SeatedBarbellShoulderPress => 16,
            ShoulderPressExerciseName::SeatedDumbbellShoulderPress => 17,
            ShoulderPressExerciseName::SingleArmDumbbellShoulderPress => 18,
            ShoulderPressExerciseName::SingleArmStepUpAndPress => 19,
            ShoulderPressExerciseName::SmithMachineOverheadPress => 20,
            ShoulderPressExerciseName::SplitStanceHammerCurlToPress => 21,
            ShoulderPressExerciseName::SwissBallDumbbellShoulderPress => 22,
            ShoulderPressExerciseName::WeightPlateFrontRaise => 23,
            ShoulderPressExerciseName::UnknownVariant(value) => value,
        }
    }
    pub fn as_i64(self) -> i64 {
        self.as_u16() as i64
    }
}
impl fmt::Display for ShoulderPressExerciseName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            ShoulderPressExerciseName::AlternatingDumbbellShoulderPress => {
                writeln!(f, "alternating_dumbbell_shoulder_press")
            }
            ShoulderPressExerciseName::ArnoldPress => writeln!(f, "arnold_press"),
            ShoulderPressExerciseName::BarbellFrontSquatToPushPress => {
                writeln!(f, "barbell_front_squat_to_push_press")
            }
            ShoulderPressExerciseName::BarbellPushPress => writeln!(f, "barbell_push_press"),
            ShoulderPressExerciseName::BarbellShoulderPress => {
                writeln!(f, "barbell_shoulder_press")
            }
            ShoulderPressExerciseName::DeadCurlPress => writeln!(f, "dead_curl_press"),
            ShoulderPressExerciseName::DumbbellAlternatingShoulderPressAndTwist => {
                writeln!(f, "dumbbell_alternating_shoulder_press_and_twist")
            }
            ShoulderPressExerciseName::DumbbellHammerCurlToLungeToPress => {
                writeln!(f, "dumbbell_hammer_curl_to_lunge_to_press")
            }
            ShoulderPressExerciseName::DumbbellPushPress => writeln!(f, "dumbbell_push_press"),
            ShoulderPressExerciseName::FloorInvertedShoulderPress => {
                writeln!(f, "floor_inverted_shoulder_press")
            }
            ShoulderPressExerciseName::WeightedFloorInvertedShoulderPress => {
                writeln!(f, "weighted_floor_inverted_shoulder_press")
            }
            ShoulderPressExerciseName::InvertedShoulderPress => {
                writeln!(f, "inverted_shoulder_press")
            }
            ShoulderPressExerciseName::WeightedInvertedShoulderPress => {
                writeln!(f, "weighted_inverted_shoulder_press")
            }
            ShoulderPressExerciseName::OneArmPushPress => writeln!(f, "one_arm_push_press"),
            ShoulderPressExerciseName::OverheadBarbellPress => {
                writeln!(f, "overhead_barbell_press")
            }
            ShoulderPressExerciseName::OverheadDumbbellPress => {
                writeln!(f, "overhead_dumbbell_press")
            }
            ShoulderPressExerciseName::SeatedBarbellShoulderPress => {
                writeln!(f, "seated_barbell_shoulder_press")
            }
            ShoulderPressExerciseName::SeatedDumbbellShoulderPress => {
                writeln!(f, "seated_dumbbell_shoulder_press")
            }
            ShoulderPressExerciseName::SingleArmDumbbellShoulderPress => {
                writeln!(f, "single_arm_dumbbell_shoulder_press")
            }
            ShoulderPressExerciseName::SingleArmStepUpAndPress => {
                writeln!(f, "single_arm_step_up_and_press")
            }
            ShoulderPressExerciseName::SmithMachineOverheadPress => {
                writeln!(f, "smith_machine_overhead_press")
            }
            ShoulderPressExerciseName::SplitStanceHammerCurlToPress => {
                writeln!(f, "split_stance_hammer_curl_to_press")
            }
            ShoulderPressExerciseName::SwissBallDumbbellShoulderPress => {
                writeln!(f, "swiss_ball_dumbbell_shoulder_press")
            }
            ShoulderPressExerciseName::WeightPlateFrontRaise => {
                writeln!(f, "weight_plate_front_raise")
            }
            ShoulderPressExerciseName::UnknownVariant(value) => {
                writeln!(f, "unknown_variant_{}", *value)
            }
        }
    }
}
impl convert::From<u16> for ShoulderPressExerciseName {
    fn from(value: u16) -> Self {
        match value {
            0 => ShoulderPressExerciseName::AlternatingDumbbellShoulderPress,
            1 => ShoulderPressExerciseName::ArnoldPress,
            2 => ShoulderPressExerciseName::BarbellFrontSquatToPushPress,
            3 => ShoulderPressExerciseName::BarbellPushPress,
            4 => ShoulderPressExerciseName::BarbellShoulderPress,
            5 => ShoulderPressExerciseName::DeadCurlPress,
            6 => ShoulderPressExerciseName::DumbbellAlternatingShoulderPressAndTwist,
            7 => ShoulderPressExerciseName::DumbbellHammerCurlToLungeToPress,
            8 => ShoulderPressExerciseName::DumbbellPushPress,
            9 => ShoulderPressExerciseName::FloorInvertedShoulderPress,
            10 => ShoulderPressExerciseName::WeightedFloorInvertedShoulderPress,
            11 => ShoulderPressExerciseName::InvertedShoulderPress,
            12 => ShoulderPressExerciseName::WeightedInvertedShoulderPress,
            13 => ShoulderPressExerciseName::OneArmPushPress,
            14 => ShoulderPressExerciseName::OverheadBarbellPress,
            15 => ShoulderPressExerciseName::OverheadDumbbellPress,
            16 => ShoulderPressExerciseName::SeatedBarbellShoulderPress,
            17 => ShoulderPressExerciseName::SeatedDumbbellShoulderPress,
            18 => ShoulderPressExerciseName::SingleArmDumbbellShoulderPress,
            19 => ShoulderPressExerciseName::SingleArmStepUpAndPress,
            20 => ShoulderPressExerciseName::SmithMachineOverheadPress,
            21 => ShoulderPressExerciseName::SplitStanceHammerCurlToPress,
            22 => ShoulderPressExerciseName::SwissBallDumbbellShoulderPress,
            23 => ShoulderPressExerciseName::WeightPlateFrontRaise,
            _ => ShoulderPressExerciseName::UnknownVariant(value),
        }
    }
}
impl convert::From<i64> for ShoulderPressExerciseName {
    fn from(value: i64) -> Self {
        ShoulderPressExerciseName::from(value as u16)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum ShoulderStabilityExerciseName {
    Name90DegreeCableExternalRotation,
    BandExternalRotation,
    BandInternalRotation,
    BentArmLateralRaiseAndExternalRotation,
    CableExternalRotation,
    DumbbellFacePullWithExternalRotation,
    FloorIRaise,
    WeightedFloorIRaise,
    FloorTRaise,
    WeightedFloorTRaise,
    FloorYRaise,
    WeightedFloorYRaise,
    InclineIRaise,
    WeightedInclineIRaise,
    InclineLRaise,
    WeightedInclineLRaise,
    InclineTRaise,
    WeightedInclineTRaise,
    InclineWRaise,
    WeightedInclineWRaise,
    InclineYRaise,
    WeightedInclineYRaise,
    LyingExternalRotation,
    SeatedDumbbellExternalRotation,
    StandingLRaise,
    SwissBallIRaise,
    WeightedSwissBallIRaise,
    SwissBallTRaise,
    WeightedSwissBallTRaise,
    SwissBallWRaise,
    WeightedSwissBallWRaise,
    SwissBallYRaise,
    WeightedSwissBallYRaise,
    UnknownVariant(u16),
}
impl ShoulderStabilityExerciseName {
    pub fn as_u16(self) -> u16 {
        match self {
            ShoulderStabilityExerciseName::Name90DegreeCableExternalRotation => 0,
            ShoulderStabilityExerciseName::BandExternalRotation => 1,
            ShoulderStabilityExerciseName::BandInternalRotation => 2,
            ShoulderStabilityExerciseName::BentArmLateralRaiseAndExternalRotation => 3,
            ShoulderStabilityExerciseName::CableExternalRotation => 4,
            ShoulderStabilityExerciseName::DumbbellFacePullWithExternalRotation => 5,
            ShoulderStabilityExerciseName::FloorIRaise => 6,
            ShoulderStabilityExerciseName::WeightedFloorIRaise => 7,
            ShoulderStabilityExerciseName::FloorTRaise => 8,
            ShoulderStabilityExerciseName::WeightedFloorTRaise => 9,
            ShoulderStabilityExerciseName::FloorYRaise => 10,
            ShoulderStabilityExerciseName::WeightedFloorYRaise => 11,
            ShoulderStabilityExerciseName::InclineIRaise => 12,
            ShoulderStabilityExerciseName::WeightedInclineIRaise => 13,
            ShoulderStabilityExerciseName::InclineLRaise => 14,
            ShoulderStabilityExerciseName::WeightedInclineLRaise => 15,
            ShoulderStabilityExerciseName::InclineTRaise => 16,
            ShoulderStabilityExerciseName::WeightedInclineTRaise => 17,
            ShoulderStabilityExerciseName::InclineWRaise => 18,
            ShoulderStabilityExerciseName::WeightedInclineWRaise => 19,
            ShoulderStabilityExerciseName::InclineYRaise => 20,
            ShoulderStabilityExerciseName::WeightedInclineYRaise => 21,
            ShoulderStabilityExerciseName::LyingExternalRotation => 22,
            ShoulderStabilityExerciseName::SeatedDumbbellExternalRotation => 23,
            ShoulderStabilityExerciseName::StandingLRaise => 24,
            ShoulderStabilityExerciseName::SwissBallIRaise => 25,
            ShoulderStabilityExerciseName::WeightedSwissBallIRaise => 26,
            ShoulderStabilityExerciseName::SwissBallTRaise => 27,
            ShoulderStabilityExerciseName::WeightedSwissBallTRaise => 28,
            ShoulderStabilityExerciseName::SwissBallWRaise => 29,
            ShoulderStabilityExerciseName::WeightedSwissBallWRaise => 30,
            ShoulderStabilityExerciseName::SwissBallYRaise => 31,
            ShoulderStabilityExerciseName::WeightedSwissBallYRaise => 32,
            ShoulderStabilityExerciseName::UnknownVariant(value) => value,
        }
    }
    pub fn as_i64(self) -> i64 {
        self.as_u16() as i64
    }
}
impl fmt::Display for ShoulderStabilityExerciseName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            ShoulderStabilityExerciseName::Name90DegreeCableExternalRotation => {
                writeln!(f, "90_degree_cable_external_rotation")
            }
            ShoulderStabilityExerciseName::BandExternalRotation => {
                writeln!(f, "band_external_rotation")
            }
            ShoulderStabilityExerciseName::BandInternalRotation => {
                writeln!(f, "band_internal_rotation")
            }
            ShoulderStabilityExerciseName::BentArmLateralRaiseAndExternalRotation => {
                writeln!(f, "bent_arm_lateral_raise_and_external_rotation")
            }
            ShoulderStabilityExerciseName::CableExternalRotation => {
                writeln!(f, "cable_external_rotation")
            }
            ShoulderStabilityExerciseName::DumbbellFacePullWithExternalRotation => {
                writeln!(f, "dumbbell_face_pull_with_external_rotation")
            }
            ShoulderStabilityExerciseName::FloorIRaise => writeln!(f, "floor_i_raise"),
            ShoulderStabilityExerciseName::WeightedFloorIRaise => {
                writeln!(f, "weighted_floor_i_raise")
            }
            ShoulderStabilityExerciseName::FloorTRaise => writeln!(f, "floor_t_raise"),
            ShoulderStabilityExerciseName::WeightedFloorTRaise => {
                writeln!(f, "weighted_floor_t_raise")
            }
            ShoulderStabilityExerciseName::FloorYRaise => writeln!(f, "floor_y_raise"),
            ShoulderStabilityExerciseName::WeightedFloorYRaise => {
                writeln!(f, "weighted_floor_y_raise")
            }
            ShoulderStabilityExerciseName::InclineIRaise => writeln!(f, "incline_i_raise"),
            ShoulderStabilityExerciseName::WeightedInclineIRaise => {
                writeln!(f, "weighted_incline_i_raise")
            }
            ShoulderStabilityExerciseName::InclineLRaise => writeln!(f, "incline_l_raise"),
            ShoulderStabilityExerciseName::WeightedInclineLRaise => {
                writeln!(f, "weighted_incline_l_raise")
            }
            ShoulderStabilityExerciseName::InclineTRaise => writeln!(f, "incline_t_raise"),
            ShoulderStabilityExerciseName::WeightedInclineTRaise => {
                writeln!(f, "weighted_incline_t_raise")
            }
            ShoulderStabilityExerciseName::InclineWRaise => writeln!(f, "incline_w_raise"),
            ShoulderStabilityExerciseName::WeightedInclineWRaise => {
                writeln!(f, "weighted_incline_w_raise")
            }
            ShoulderStabilityExerciseName::InclineYRaise => writeln!(f, "incline_y_raise"),
            ShoulderStabilityExerciseName::WeightedInclineYRaise => {
                writeln!(f, "weighted_incline_y_raise")
            }
            ShoulderStabilityExerciseName::LyingExternalRotation => {
                writeln!(f, "lying_external_rotation")
            }
            ShoulderStabilityExerciseName::SeatedDumbbellExternalRotation => {
                writeln!(f, "seated_dumbbell_external_rotation")
            }
            ShoulderStabilityExerciseName::StandingLRaise => writeln!(f, "standing_l_raise"),
            ShoulderStabilityExerciseName::SwissBallIRaise => writeln!(f, "swiss_ball_i_raise"),
            ShoulderStabilityExerciseName::WeightedSwissBallIRaise => {
                writeln!(f, "weighted_swiss_ball_i_raise")
            }
            ShoulderStabilityExerciseName::SwissBallTRaise => writeln!(f, "swiss_ball_t_raise"),
            ShoulderStabilityExerciseName::WeightedSwissBallTRaise => {
                writeln!(f, "weighted_swiss_ball_t_raise")
            }
            ShoulderStabilityExerciseName::SwissBallWRaise => writeln!(f, "swiss_ball_w_raise"),
            ShoulderStabilityExerciseName::WeightedSwissBallWRaise => {
                writeln!(f, "weighted_swiss_ball_w_raise")
            }
            ShoulderStabilityExerciseName::SwissBallYRaise => writeln!(f, "swiss_ball_y_raise"),
            ShoulderStabilityExerciseName::WeightedSwissBallYRaise => {
                writeln!(f, "weighted_swiss_ball_y_raise")
            }
            ShoulderStabilityExerciseName::UnknownVariant(value) => {
                writeln!(f, "unknown_variant_{}", *value)
            }
        }
    }
}
impl convert::From<u16> for ShoulderStabilityExerciseName {
    fn from(value: u16) -> Self {
        match value {
            0 => ShoulderStabilityExerciseName::Name90DegreeCableExternalRotation,
            1 => ShoulderStabilityExerciseName::BandExternalRotation,
            2 => ShoulderStabilityExerciseName::BandInternalRotation,
            3 => ShoulderStabilityExerciseName::BentArmLateralRaiseAndExternalRotation,
            4 => ShoulderStabilityExerciseName::CableExternalRotation,
            5 => ShoulderStabilityExerciseName::DumbbellFacePullWithExternalRotation,
            6 => ShoulderStabilityExerciseName::FloorIRaise,
            7 => ShoulderStabilityExerciseName::WeightedFloorIRaise,
            8 => ShoulderStabilityExerciseName::FloorTRaise,
            9 => ShoulderStabilityExerciseName::WeightedFloorTRaise,
            10 => ShoulderStabilityExerciseName::FloorYRaise,
            11 => ShoulderStabilityExerciseName::WeightedFloorYRaise,
            12 => ShoulderStabilityExerciseName::InclineIRaise,
            13 => ShoulderStabilityExerciseName::WeightedInclineIRaise,
            14 => ShoulderStabilityExerciseName::InclineLRaise,
            15 => ShoulderStabilityExerciseName::WeightedInclineLRaise,
            16 => ShoulderStabilityExerciseName::InclineTRaise,
            17 => ShoulderStabilityExerciseName::WeightedInclineTRaise,
            18 => ShoulderStabilityExerciseName::InclineWRaise,
            19 => ShoulderStabilityExerciseName::WeightedInclineWRaise,
            20 => ShoulderStabilityExerciseName::InclineYRaise,
            21 => ShoulderStabilityExerciseName::WeightedInclineYRaise,
            22 => ShoulderStabilityExerciseName::LyingExternalRotation,
            23 => ShoulderStabilityExerciseName::SeatedDumbbellExternalRotation,
            24 => ShoulderStabilityExerciseName::StandingLRaise,
            25 => ShoulderStabilityExerciseName::SwissBallIRaise,
            26 => ShoulderStabilityExerciseName::WeightedSwissBallIRaise,
            27 => ShoulderStabilityExerciseName::SwissBallTRaise,
            28 => ShoulderStabilityExerciseName::WeightedSwissBallTRaise,
            29 => ShoulderStabilityExerciseName::SwissBallWRaise,
            30 => ShoulderStabilityExerciseName::WeightedSwissBallWRaise,
            31 => ShoulderStabilityExerciseName::SwissBallYRaise,
            32 => ShoulderStabilityExerciseName::WeightedSwissBallYRaise,
            _ => ShoulderStabilityExerciseName::UnknownVariant(value),
        }
    }
}
impl convert::From<i64> for ShoulderStabilityExerciseName {
    fn from(value: i64) -> Self {
        ShoulderStabilityExerciseName::from(value as u16)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum ShrugExerciseName {
    BarbellJumpShrug,
    BarbellShrug,
    BarbellUprightRow,
    BehindTheBackSmithMachineShrug,
    DumbbellJumpShrug,
    DumbbellShrug,
    DumbbellUprightRow,
    InclineDumbbellShrug,
    OverheadBarbellShrug,
    OverheadDumbbellShrug,
    ScaptionAndShrug,
    ScapularRetraction,
    SerratusChairShrug,
    WeightedSerratusChairShrug,
    SerratusShrug,
    WeightedSerratusShrug,
    WideGripJumpShrug,
    UnknownVariant(u16),
}
impl ShrugExerciseName {
    pub fn as_u16(self) -> u16 {
        match self {
            ShrugExerciseName::BarbellJumpShrug => 0,
            ShrugExerciseName::BarbellShrug => 1,
            ShrugExerciseName::BarbellUprightRow => 2,
            ShrugExerciseName::BehindTheBackSmithMachineShrug => 3,
            ShrugExerciseName::DumbbellJumpShrug => 4,
            ShrugExerciseName::DumbbellShrug => 5,
            ShrugExerciseName::DumbbellUprightRow => 6,
            ShrugExerciseName::InclineDumbbellShrug => 7,
            ShrugExerciseName::OverheadBarbellShrug => 8,
            ShrugExerciseName::OverheadDumbbellShrug => 9,
            ShrugExerciseName::ScaptionAndShrug => 10,
            ShrugExerciseName::ScapularRetraction => 11,
            ShrugExerciseName::SerratusChairShrug => 12,
            ShrugExerciseName::WeightedSerratusChairShrug => 13,
            ShrugExerciseName::SerratusShrug => 14,
            ShrugExerciseName::WeightedSerratusShrug => 15,
            ShrugExerciseName::WideGripJumpShrug => 16,
            ShrugExerciseName::UnknownVariant(value) => value,
        }
    }
    pub fn as_i64(self) -> i64 {
        self.as_u16() as i64
    }
}
impl fmt::Display for ShrugExerciseName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            ShrugExerciseName::BarbellJumpShrug => writeln!(f, "barbell_jump_shrug"),
            ShrugExerciseName::BarbellShrug => writeln!(f, "barbell_shrug"),
            ShrugExerciseName::BarbellUprightRow => writeln!(f, "barbell_upright_row"),
            ShrugExerciseName::BehindTheBackSmithMachineShrug => {
                writeln!(f, "behind_the_back_smith_machine_shrug")
            }
            ShrugExerciseName::DumbbellJumpShrug => writeln!(f, "dumbbell_jump_shrug"),
            ShrugExerciseName::DumbbellShrug => writeln!(f, "dumbbell_shrug"),
            ShrugExerciseName::DumbbellUprightRow => writeln!(f, "dumbbell_upright_row"),
            ShrugExerciseName::InclineDumbbellShrug => writeln!(f, "incline_dumbbell_shrug"),
            ShrugExerciseName::OverheadBarbellShrug => writeln!(f, "overhead_barbell_shrug"),
            ShrugExerciseName::OverheadDumbbellShrug => writeln!(f, "overhead_dumbbell_shrug"),
            ShrugExerciseName::ScaptionAndShrug => writeln!(f, "scaption_and_shrug"),
            ShrugExerciseName::ScapularRetraction => writeln!(f, "scapular_retraction"),
            ShrugExerciseName::SerratusChairShrug => writeln!(f, "serratus_chair_shrug"),
            ShrugExerciseName::WeightedSerratusChairShrug => {
                writeln!(f, "weighted_serratus_chair_shrug")
            }
            ShrugExerciseName::SerratusShrug => writeln!(f, "serratus_shrug"),
            ShrugExerciseName::WeightedSerratusShrug => writeln!(f, "weighted_serratus_shrug"),
            ShrugExerciseName::WideGripJumpShrug => writeln!(f, "wide_grip_jump_shrug"),
            ShrugExerciseName::UnknownVariant(value) => writeln!(f, "unknown_variant_{}", *value),
        }
    }
}
impl convert::From<u16> for ShrugExerciseName {
    fn from(value: u16) -> Self {
        match value {
            0 => ShrugExerciseName::BarbellJumpShrug,
            1 => ShrugExerciseName::BarbellShrug,
            2 => ShrugExerciseName::BarbellUprightRow,
            3 => ShrugExerciseName::BehindTheBackSmithMachineShrug,
            4 => ShrugExerciseName::DumbbellJumpShrug,
            5 => ShrugExerciseName::DumbbellShrug,
            6 => ShrugExerciseName::DumbbellUprightRow,
            7 => ShrugExerciseName::InclineDumbbellShrug,
            8 => ShrugExerciseName::OverheadBarbellShrug,
            9 => ShrugExerciseName::OverheadDumbbellShrug,
            10 => ShrugExerciseName::ScaptionAndShrug,
            11 => ShrugExerciseName::ScapularRetraction,
            12 => ShrugExerciseName::SerratusChairShrug,
            13 => ShrugExerciseName::WeightedSerratusChairShrug,
            14 => ShrugExerciseName::SerratusShrug,
            15 => ShrugExerciseName::WeightedSerratusShrug,
            16 => ShrugExerciseName::WideGripJumpShrug,
            _ => ShrugExerciseName::UnknownVariant(value),
        }
    }
}
impl convert::From<i64> for ShrugExerciseName {
    fn from(value: i64) -> Self {
        ShrugExerciseName::from(value as u16)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum SitUpExerciseName {
    AlternatingSitUp,
    WeightedAlternatingSitUp,
    BentKneeVUp,
    WeightedBentKneeVUp,
    ButterflySitUp,
    WeightedButterflySitup,
    CrossPunchRollUp,
    WeightedCrossPunchRollUp,
    CrossedArmsSitUp,
    WeightedCrossedArmsSitUp,
    GetUpSitUp,
    WeightedGetUpSitUp,
    HoveringSitUp,
    WeightedHoveringSitUp,
    KettlebellSitUp,
    MedicineBallAlternatingVUp,
    MedicineBallSitUp,
    MedicineBallVUp,
    ModifiedSitUp,
    NegativeSitUp,
    OneArmFullSitUp,
    RecliningCircle,
    WeightedRecliningCircle,
    ReverseCurlUp,
    WeightedReverseCurlUp,
    SingleLegSwissBallJackknife,
    WeightedSingleLegSwissBallJackknife,
    TheTeaser,
    TheTeaserWeighted,
    ThreePartRollDown,
    WeightedThreePartRollDown,
    VUp,
    WeightedVUp,
    WeightedRussianTwistOnSwissBall,
    WeightedSitUp,
    XAbs,
    WeightedXAbs,
    SitUp,
    UnknownVariant(u16),
}
impl SitUpExerciseName {
    pub fn as_u16(self) -> u16 {
        match self {
            SitUpExerciseName::AlternatingSitUp => 0,
            SitUpExerciseName::WeightedAlternatingSitUp => 1,
            SitUpExerciseName::BentKneeVUp => 2,
            SitUpExerciseName::WeightedBentKneeVUp => 3,
            SitUpExerciseName::ButterflySitUp => 4,
            SitUpExerciseName::WeightedButterflySitup => 5,
            SitUpExerciseName::CrossPunchRollUp => 6,
            SitUpExerciseName::WeightedCrossPunchRollUp => 7,
            SitUpExerciseName::CrossedArmsSitUp => 8,
            SitUpExerciseName::WeightedCrossedArmsSitUp => 9,
            SitUpExerciseName::GetUpSitUp => 10,
            SitUpExerciseName::WeightedGetUpSitUp => 11,
            SitUpExerciseName::HoveringSitUp => 12,
            SitUpExerciseName::WeightedHoveringSitUp => 13,
            SitUpExerciseName::KettlebellSitUp => 14,
            SitUpExerciseName::MedicineBallAlternatingVUp => 15,
            SitUpExerciseName::MedicineBallSitUp => 16,
            SitUpExerciseName::MedicineBallVUp => 17,
            SitUpExerciseName::ModifiedSitUp => 18,
            SitUpExerciseName::NegativeSitUp => 19,
            SitUpExerciseName::OneArmFullSitUp => 20,
            SitUpExerciseName::RecliningCircle => 21,
            SitUpExerciseName::WeightedRecliningCircle => 22,
            SitUpExerciseName::ReverseCurlUp => 23,
            SitUpExerciseName::WeightedReverseCurlUp => 24,
            SitUpExerciseName::SingleLegSwissBallJackknife => 25,
            SitUpExerciseName::WeightedSingleLegSwissBallJackknife => 26,
            SitUpExerciseName::TheTeaser => 27,
            SitUpExerciseName::TheTeaserWeighted => 28,
            SitUpExerciseName::ThreePartRollDown => 29,
            SitUpExerciseName::WeightedThreePartRollDown => 30,
            SitUpExerciseName::VUp => 31,
            SitUpExerciseName::WeightedVUp => 32,
            SitUpExerciseName::WeightedRussianTwistOnSwissBall => 33,
            SitUpExerciseName::WeightedSitUp => 34,
            SitUpExerciseName::XAbs => 35,
            SitUpExerciseName::WeightedXAbs => 36,
            SitUpExerciseName::SitUp => 37,
            SitUpExerciseName::UnknownVariant(value) => value,
        }
    }
    pub fn as_i64(self) -> i64 {
        self.as_u16() as i64
    }
}
impl fmt::Display for SitUpExerciseName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            SitUpExerciseName::AlternatingSitUp => writeln!(f, "alternating_sit_up"),
            SitUpExerciseName::WeightedAlternatingSitUp => {
                writeln!(f, "weighted_alternating_sit_up")
            }
            SitUpExerciseName::BentKneeVUp => writeln!(f, "bent_knee_v_up"),
            SitUpExerciseName::WeightedBentKneeVUp => writeln!(f, "weighted_bent_knee_v_up"),
            SitUpExerciseName::ButterflySitUp => writeln!(f, "butterfly_sit_up"),
            SitUpExerciseName::WeightedButterflySitup => writeln!(f, "weighted_butterfly_situp"),
            SitUpExerciseName::CrossPunchRollUp => writeln!(f, "cross_punch_roll_up"),
            SitUpExerciseName::WeightedCrossPunchRollUp => {
                writeln!(f, "weighted_cross_punch_roll_up")
            }
            SitUpExerciseName::CrossedArmsSitUp => writeln!(f, "crossed_arms_sit_up"),
            SitUpExerciseName::WeightedCrossedArmsSitUp => {
                writeln!(f, "weighted_crossed_arms_sit_up")
            }
            SitUpExerciseName::GetUpSitUp => writeln!(f, "get_up_sit_up"),
            SitUpExerciseName::WeightedGetUpSitUp => writeln!(f, "weighted_get_up_sit_up"),
            SitUpExerciseName::HoveringSitUp => writeln!(f, "hovering_sit_up"),
            SitUpExerciseName::WeightedHoveringSitUp => writeln!(f, "weighted_hovering_sit_up"),
            SitUpExerciseName::KettlebellSitUp => writeln!(f, "kettlebell_sit_up"),
            SitUpExerciseName::MedicineBallAlternatingVUp => {
                writeln!(f, "medicine_ball_alternating_v_up")
            }
            SitUpExerciseName::MedicineBallSitUp => writeln!(f, "medicine_ball_sit_up"),
            SitUpExerciseName::MedicineBallVUp => writeln!(f, "medicine_ball_v_up"),
            SitUpExerciseName::ModifiedSitUp => writeln!(f, "modified_sit_up"),
            SitUpExerciseName::NegativeSitUp => writeln!(f, "negative_sit_up"),
            SitUpExerciseName::OneArmFullSitUp => writeln!(f, "one_arm_full_sit_up"),
            SitUpExerciseName::RecliningCircle => writeln!(f, "reclining_circle"),
            SitUpExerciseName::WeightedRecliningCircle => writeln!(f, "weighted_reclining_circle"),
            SitUpExerciseName::ReverseCurlUp => writeln!(f, "reverse_curl_up"),
            SitUpExerciseName::WeightedReverseCurlUp => writeln!(f, "weighted_reverse_curl_up"),
            SitUpExerciseName::SingleLegSwissBallJackknife => {
                writeln!(f, "single_leg_swiss_ball_jackknife")
            }
            SitUpExerciseName::WeightedSingleLegSwissBallJackknife => {
                writeln!(f, "weighted_single_leg_swiss_ball_jackknife")
            }
            SitUpExerciseName::TheTeaser => writeln!(f, "the_teaser"),
            SitUpExerciseName::TheTeaserWeighted => writeln!(f, "the_teaser_weighted"),
            SitUpExerciseName::ThreePartRollDown => writeln!(f, "three_part_roll_down"),
            SitUpExerciseName::WeightedThreePartRollDown => {
                writeln!(f, "weighted_three_part_roll_down")
            }
            SitUpExerciseName::VUp => writeln!(f, "v_up"),
            SitUpExerciseName::WeightedVUp => writeln!(f, "weighted_v_up"),
            SitUpExerciseName::WeightedRussianTwistOnSwissBall => {
                writeln!(f, "weighted_russian_twist_on_swiss_ball")
            }
            SitUpExerciseName::WeightedSitUp => writeln!(f, "weighted_sit_up"),
            SitUpExerciseName::XAbs => writeln!(f, "x_abs"),
            SitUpExerciseName::WeightedXAbs => writeln!(f, "weighted_x_abs"),
            SitUpExerciseName::SitUp => writeln!(f, "sit_up"),
            SitUpExerciseName::UnknownVariant(value) => writeln!(f, "unknown_variant_{}", *value),
        }
    }
}
impl convert::From<u16> for SitUpExerciseName {
    fn from(value: u16) -> Self {
        match value {
            0 => SitUpExerciseName::AlternatingSitUp,
            1 => SitUpExerciseName::WeightedAlternatingSitUp,
            2 => SitUpExerciseName::BentKneeVUp,
            3 => SitUpExerciseName::WeightedBentKneeVUp,
            4 => SitUpExerciseName::ButterflySitUp,
            5 => SitUpExerciseName::WeightedButterflySitup,
            6 => SitUpExerciseName::CrossPunchRollUp,
            7 => SitUpExerciseName::WeightedCrossPunchRollUp,
            8 => SitUpExerciseName::CrossedArmsSitUp,
            9 => SitUpExerciseName::WeightedCrossedArmsSitUp,
            10 => SitUpExerciseName::GetUpSitUp,
            11 => SitUpExerciseName::WeightedGetUpSitUp,
            12 => SitUpExerciseName::HoveringSitUp,
            13 => SitUpExerciseName::WeightedHoveringSitUp,
            14 => SitUpExerciseName::KettlebellSitUp,
            15 => SitUpExerciseName::MedicineBallAlternatingVUp,
            16 => SitUpExerciseName::MedicineBallSitUp,
            17 => SitUpExerciseName::MedicineBallVUp,
            18 => SitUpExerciseName::ModifiedSitUp,
            19 => SitUpExerciseName::NegativeSitUp,
            20 => SitUpExerciseName::OneArmFullSitUp,
            21 => SitUpExerciseName::RecliningCircle,
            22 => SitUpExerciseName::WeightedRecliningCircle,
            23 => SitUpExerciseName::ReverseCurlUp,
            24 => SitUpExerciseName::WeightedReverseCurlUp,
            25 => SitUpExerciseName::SingleLegSwissBallJackknife,
            26 => SitUpExerciseName::WeightedSingleLegSwissBallJackknife,
            27 => SitUpExerciseName::TheTeaser,
            28 => SitUpExerciseName::TheTeaserWeighted,
            29 => SitUpExerciseName::ThreePartRollDown,
            30 => SitUpExerciseName::WeightedThreePartRollDown,
            31 => SitUpExerciseName::VUp,
            32 => SitUpExerciseName::WeightedVUp,
            33 => SitUpExerciseName::WeightedRussianTwistOnSwissBall,
            34 => SitUpExerciseName::WeightedSitUp,
            35 => SitUpExerciseName::XAbs,
            36 => SitUpExerciseName::WeightedXAbs,
            37 => SitUpExerciseName::SitUp,
            _ => SitUpExerciseName::UnknownVariant(value),
        }
    }
}
impl convert::From<i64> for SitUpExerciseName {
    fn from(value: i64) -> Self {
        SitUpExerciseName::from(value as u16)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum SquatExerciseName {
    LegPress,
    BackSquatWithBodyBar,
    BackSquats,
    WeightedBackSquats,
    BalancingSquat,
    WeightedBalancingSquat,
    BarbellBackSquat,
    BarbellBoxSquat,
    BarbellFrontSquat,
    BarbellHackSquat,
    BarbellHangSquatSnatch,
    BarbellLateralStepUp,
    BarbellQuarterSquat,
    BarbellSiffSquat,
    BarbellSquatSnatch,
    BarbellSquatWithHeelsRaised,
    BarbellStepover,
    BarbellStepUp,
    BenchSquatWithRotationalChop,
    WeightedBenchSquatWithRotationalChop,
    BodyWeightWallSquat,
    WeightedWallSquat,
    BoxStepSquat,
    WeightedBoxStepSquat,
    BracedSquat,
    CrossedArmBarbellFrontSquat,
    CrossoverDumbbellStepUp,
    DumbbellFrontSquat,
    DumbbellSplitSquat,
    DumbbellSquat,
    DumbbellSquatClean,
    DumbbellStepover,
    DumbbellStepUp,
    ElevatedSingleLegSquat,
    WeightedElevatedSingleLegSquat,
    FigureFourSquats,
    WeightedFigureFourSquats,
    GobletSquat,
    KettlebellSquat,
    KettlebellSwingOverhead,
    KettlebellSwingWithFlipToSquat,
    LateralDumbbellStepUp,
    OneLeggedSquat,
    OverheadDumbbellSquat,
    OverheadSquat,
    PartialSingleLegSquat,
    WeightedPartialSingleLegSquat,
    PistolSquat,
    WeightedPistolSquat,
    PlieSlides,
    WeightedPlieSlides,
    PlieSquat,
    WeightedPlieSquat,
    PrisonerSquat,
    WeightedPrisonerSquat,
    SingleLegBenchGetUp,
    WeightedSingleLegBenchGetUp,
    SingleLegBenchSquat,
    WeightedSingleLegBenchSquat,
    SingleLegSquatOnSwissBall,
    WeightedSingleLegSquatOnSwissBall,
    Squat,
    WeightedSquat,
    SquatsWithBand,
    StaggeredSquat,
    WeightedStaggeredSquat,
    StepUp,
    WeightedStepUp,
    SuitcaseSquats,
    SumoSquat,
    SumoSquatSlideIn,
    WeightedSumoSquatSlideIn,
    SumoSquatToHighPull,
    SumoSquatToStand,
    WeightedSumoSquatToStand,
    SumoSquatWithRotation,
    WeightedSumoSquatWithRotation,
    SwissBallBodyWeightWallSquat,
    WeightedSwissBallWallSquat,
    Thrusters,
    UnevenSquat,
    WeightedUnevenSquat,
    WaistSlimmingSquat,
    WallBall,
    WideStanceBarbellSquat,
    WideStanceGobletSquat,
    ZercherSquat,
    KbsOverhead,
    SquatAndSideKick,
    SquatJumpsInNOut,
    PilatesPlieSquatsParallelTurnedOutFlatAndHeels,
    ReleveStraightLegAndKneeBentWithOneLegVariation,
    UnknownVariant(u16),
}
impl SquatExerciseName {
    pub fn as_u16(self) -> u16 {
        match self {
            SquatExerciseName::LegPress => 0,
            SquatExerciseName::BackSquatWithBodyBar => 1,
            SquatExerciseName::BackSquats => 2,
            SquatExerciseName::WeightedBackSquats => 3,
            SquatExerciseName::BalancingSquat => 4,
            SquatExerciseName::WeightedBalancingSquat => 5,
            SquatExerciseName::BarbellBackSquat => 6,
            SquatExerciseName::BarbellBoxSquat => 7,
            SquatExerciseName::BarbellFrontSquat => 8,
            SquatExerciseName::BarbellHackSquat => 9,
            SquatExerciseName::BarbellHangSquatSnatch => 10,
            SquatExerciseName::BarbellLateralStepUp => 11,
            SquatExerciseName::BarbellQuarterSquat => 12,
            SquatExerciseName::BarbellSiffSquat => 13,
            SquatExerciseName::BarbellSquatSnatch => 14,
            SquatExerciseName::BarbellSquatWithHeelsRaised => 15,
            SquatExerciseName::BarbellStepover => 16,
            SquatExerciseName::BarbellStepUp => 17,
            SquatExerciseName::BenchSquatWithRotationalChop => 18,
            SquatExerciseName::WeightedBenchSquatWithRotationalChop => 19,
            SquatExerciseName::BodyWeightWallSquat => 20,
            SquatExerciseName::WeightedWallSquat => 21,
            SquatExerciseName::BoxStepSquat => 22,
            SquatExerciseName::WeightedBoxStepSquat => 23,
            SquatExerciseName::BracedSquat => 24,
            SquatExerciseName::CrossedArmBarbellFrontSquat => 25,
            SquatExerciseName::CrossoverDumbbellStepUp => 26,
            SquatExerciseName::DumbbellFrontSquat => 27,
            SquatExerciseName::DumbbellSplitSquat => 28,
            SquatExerciseName::DumbbellSquat => 29,
            SquatExerciseName::DumbbellSquatClean => 30,
            SquatExerciseName::DumbbellStepover => 31,
            SquatExerciseName::DumbbellStepUp => 32,
            SquatExerciseName::ElevatedSingleLegSquat => 33,
            SquatExerciseName::WeightedElevatedSingleLegSquat => 34,
            SquatExerciseName::FigureFourSquats => 35,
            SquatExerciseName::WeightedFigureFourSquats => 36,
            SquatExerciseName::GobletSquat => 37,
            SquatExerciseName::KettlebellSquat => 38,
            SquatExerciseName::KettlebellSwingOverhead => 39,
            SquatExerciseName::KettlebellSwingWithFlipToSquat => 40,
            SquatExerciseName::LateralDumbbellStepUp => 41,
            SquatExerciseName::OneLeggedSquat => 42,
            SquatExerciseName::OverheadDumbbellSquat => 43,
            SquatExerciseName::OverheadSquat => 44,
            SquatExerciseName::PartialSingleLegSquat => 45,
            SquatExerciseName::WeightedPartialSingleLegSquat => 46,
            SquatExerciseName::PistolSquat => 47,
            SquatExerciseName::WeightedPistolSquat => 48,
            SquatExerciseName::PlieSlides => 49,
            SquatExerciseName::WeightedPlieSlides => 50,
            SquatExerciseName::PlieSquat => 51,
            SquatExerciseName::WeightedPlieSquat => 52,
            SquatExerciseName::PrisonerSquat => 53,
            SquatExerciseName::WeightedPrisonerSquat => 54,
            SquatExerciseName::SingleLegBenchGetUp => 55,
            SquatExerciseName::WeightedSingleLegBenchGetUp => 56,
            SquatExerciseName::SingleLegBenchSquat => 57,
            SquatExerciseName::WeightedSingleLegBenchSquat => 58,
            SquatExerciseName::SingleLegSquatOnSwissBall => 59,
            SquatExerciseName::WeightedSingleLegSquatOnSwissBall => 60,
            SquatExerciseName::Squat => 61,
            SquatExerciseName::WeightedSquat => 62,
            SquatExerciseName::SquatsWithBand => 63,
            SquatExerciseName::StaggeredSquat => 64,
            SquatExerciseName::WeightedStaggeredSquat => 65,
            SquatExerciseName::StepUp => 66,
            SquatExerciseName::WeightedStepUp => 67,
            SquatExerciseName::SuitcaseSquats => 68,
            SquatExerciseName::SumoSquat => 69,
            SquatExerciseName::SumoSquatSlideIn => 70,
            SquatExerciseName::WeightedSumoSquatSlideIn => 71,
            SquatExerciseName::SumoSquatToHighPull => 72,
            SquatExerciseName::SumoSquatToStand => 73,
            SquatExerciseName::WeightedSumoSquatToStand => 74,
            SquatExerciseName::SumoSquatWithRotation => 75,
            SquatExerciseName::WeightedSumoSquatWithRotation => 76,
            SquatExerciseName::SwissBallBodyWeightWallSquat => 77,
            SquatExerciseName::WeightedSwissBallWallSquat => 78,
            SquatExerciseName::Thrusters => 79,
            SquatExerciseName::UnevenSquat => 80,
            SquatExerciseName::WeightedUnevenSquat => 81,
            SquatExerciseName::WaistSlimmingSquat => 82,
            SquatExerciseName::WallBall => 83,
            SquatExerciseName::WideStanceBarbellSquat => 84,
            SquatExerciseName::WideStanceGobletSquat => 85,
            SquatExerciseName::ZercherSquat => 86,
            SquatExerciseName::KbsOverhead => 87,
            SquatExerciseName::SquatAndSideKick => 88,
            SquatExerciseName::SquatJumpsInNOut => 89,
            SquatExerciseName::PilatesPlieSquatsParallelTurnedOutFlatAndHeels => 90,
            SquatExerciseName::ReleveStraightLegAndKneeBentWithOneLegVariation => 91,
            SquatExerciseName::UnknownVariant(value) => value,
        }
    }
    pub fn as_i64(self) -> i64 {
        self.as_u16() as i64
    }
}
impl fmt::Display for SquatExerciseName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            SquatExerciseName::LegPress => writeln!(f, "leg_press"),
            SquatExerciseName::BackSquatWithBodyBar => writeln!(f, "back_squat_with_body_bar"),
            SquatExerciseName::BackSquats => writeln!(f, "back_squats"),
            SquatExerciseName::WeightedBackSquats => writeln!(f, "weighted_back_squats"),
            SquatExerciseName::BalancingSquat => writeln!(f, "balancing_squat"),
            SquatExerciseName::WeightedBalancingSquat => writeln!(f, "weighted_balancing_squat"),
            SquatExerciseName::BarbellBackSquat => writeln!(f, "barbell_back_squat"),
            SquatExerciseName::BarbellBoxSquat => writeln!(f, "barbell_box_squat"),
            SquatExerciseName::BarbellFrontSquat => writeln!(f, "barbell_front_squat"),
            SquatExerciseName::BarbellHackSquat => writeln!(f, "barbell_hack_squat"),
            SquatExerciseName::BarbellHangSquatSnatch => writeln!(f, "barbell_hang_squat_snatch"),
            SquatExerciseName::BarbellLateralStepUp => writeln!(f, "barbell_lateral_step_up"),
            SquatExerciseName::BarbellQuarterSquat => writeln!(f, "barbell_quarter_squat"),
            SquatExerciseName::BarbellSiffSquat => writeln!(f, "barbell_siff_squat"),
            SquatExerciseName::BarbellSquatSnatch => writeln!(f, "barbell_squat_snatch"),
            SquatExerciseName::BarbellSquatWithHeelsRaised => {
                writeln!(f, "barbell_squat_with_heels_raised")
            }
            SquatExerciseName::BarbellStepover => writeln!(f, "barbell_stepover"),
            SquatExerciseName::BarbellStepUp => writeln!(f, "barbell_step_up"),
            SquatExerciseName::BenchSquatWithRotationalChop => {
                writeln!(f, "bench_squat_with_rotational_chop")
            }
            SquatExerciseName::WeightedBenchSquatWithRotationalChop => {
                writeln!(f, "weighted_bench_squat_with_rotational_chop")
            }
            SquatExerciseName::BodyWeightWallSquat => writeln!(f, "body_weight_wall_squat"),
            SquatExerciseName::WeightedWallSquat => writeln!(f, "weighted_wall_squat"),
            SquatExerciseName::BoxStepSquat => writeln!(f, "box_step_squat"),
            SquatExerciseName::WeightedBoxStepSquat => writeln!(f, "weighted_box_step_squat"),
            SquatExerciseName::BracedSquat => writeln!(f, "braced_squat"),
            SquatExerciseName::CrossedArmBarbellFrontSquat => {
                writeln!(f, "crossed_arm_barbell_front_squat")
            }
            SquatExerciseName::CrossoverDumbbellStepUp => writeln!(f, "crossover_dumbbell_step_up"),
            SquatExerciseName::DumbbellFrontSquat => writeln!(f, "dumbbell_front_squat"),
            SquatExerciseName::DumbbellSplitSquat => writeln!(f, "dumbbell_split_squat"),
            SquatExerciseName::DumbbellSquat => writeln!(f, "dumbbell_squat"),
            SquatExerciseName::DumbbellSquatClean => writeln!(f, "dumbbell_squat_clean"),
            SquatExerciseName::DumbbellStepover => writeln!(f, "dumbbell_stepover"),
            SquatExerciseName::DumbbellStepUp => writeln!(f, "dumbbell_step_up"),
            SquatExerciseName::ElevatedSingleLegSquat => writeln!(f, "elevated_single_leg_squat"),
            SquatExerciseName::WeightedElevatedSingleLegSquat => {
                writeln!(f, "weighted_elevated_single_leg_squat")
            }
            SquatExerciseName::FigureFourSquats => writeln!(f, "figure_four_squats"),
            SquatExerciseName::WeightedFigureFourSquats => {
                writeln!(f, "weighted_figure_four_squats")
            }
            SquatExerciseName::GobletSquat => writeln!(f, "goblet_squat"),
            SquatExerciseName::KettlebellSquat => writeln!(f, "kettlebell_squat"),
            SquatExerciseName::KettlebellSwingOverhead => writeln!(f, "kettlebell_swing_overhead"),
            SquatExerciseName::KettlebellSwingWithFlipToSquat => {
                writeln!(f, "kettlebell_swing_with_flip_to_squat")
            }
            SquatExerciseName::LateralDumbbellStepUp => writeln!(f, "lateral_dumbbell_step_up"),
            SquatExerciseName::OneLeggedSquat => writeln!(f, "one_legged_squat"),
            SquatExerciseName::OverheadDumbbellSquat => writeln!(f, "overhead_dumbbell_squat"),
            SquatExerciseName::OverheadSquat => writeln!(f, "overhead_squat"),
            SquatExerciseName::PartialSingleLegSquat => writeln!(f, "partial_single_leg_squat"),
            SquatExerciseName::WeightedPartialSingleLegSquat => {
                writeln!(f, "weighted_partial_single_leg_squat")
            }
            SquatExerciseName::PistolSquat => writeln!(f, "pistol_squat"),
            SquatExerciseName::WeightedPistolSquat => writeln!(f, "weighted_pistol_squat"),
            SquatExerciseName::PlieSlides => writeln!(f, "plie_slides"),
            SquatExerciseName::WeightedPlieSlides => writeln!(f, "weighted_plie_slides"),
            SquatExerciseName::PlieSquat => writeln!(f, "plie_squat"),
            SquatExerciseName::WeightedPlieSquat => writeln!(f, "weighted_plie_squat"),
            SquatExerciseName::PrisonerSquat => writeln!(f, "prisoner_squat"),
            SquatExerciseName::WeightedPrisonerSquat => writeln!(f, "weighted_prisoner_squat"),
            SquatExerciseName::SingleLegBenchGetUp => writeln!(f, "single_leg_bench_get_up"),
            SquatExerciseName::WeightedSingleLegBenchGetUp => {
                writeln!(f, "weighted_single_leg_bench_get_up")
            }
            SquatExerciseName::SingleLegBenchSquat => writeln!(f, "single_leg_bench_squat"),
            SquatExerciseName::WeightedSingleLegBenchSquat => {
                writeln!(f, "weighted_single_leg_bench_squat")
            }
            SquatExerciseName::SingleLegSquatOnSwissBall => {
                writeln!(f, "single_leg_squat_on_swiss_ball")
            }
            SquatExerciseName::WeightedSingleLegSquatOnSwissBall => {
                writeln!(f, "weighted_single_leg_squat_on_swiss_ball")
            }
            SquatExerciseName::Squat => writeln!(f, "squat"),
            SquatExerciseName::WeightedSquat => writeln!(f, "weighted_squat"),
            SquatExerciseName::SquatsWithBand => writeln!(f, "squats_with_band"),
            SquatExerciseName::StaggeredSquat => writeln!(f, "staggered_squat"),
            SquatExerciseName::WeightedStaggeredSquat => writeln!(f, "weighted_staggered_squat"),
            SquatExerciseName::StepUp => writeln!(f, "step_up"),
            SquatExerciseName::WeightedStepUp => writeln!(f, "weighted_step_up"),
            SquatExerciseName::SuitcaseSquats => writeln!(f, "suitcase_squats"),
            SquatExerciseName::SumoSquat => writeln!(f, "sumo_squat"),
            SquatExerciseName::SumoSquatSlideIn => writeln!(f, "sumo_squat_slide_in"),
            SquatExerciseName::WeightedSumoSquatSlideIn => {
                writeln!(f, "weighted_sumo_squat_slide_in")
            }
            SquatExerciseName::SumoSquatToHighPull => writeln!(f, "sumo_squat_to_high_pull"),
            SquatExerciseName::SumoSquatToStand => writeln!(f, "sumo_squat_to_stand"),
            SquatExerciseName::WeightedSumoSquatToStand => {
                writeln!(f, "weighted_sumo_squat_to_stand")
            }
            SquatExerciseName::SumoSquatWithRotation => writeln!(f, "sumo_squat_with_rotation"),
            SquatExerciseName::WeightedSumoSquatWithRotation => {
                writeln!(f, "weighted_sumo_squat_with_rotation")
            }
            SquatExerciseName::SwissBallBodyWeightWallSquat => {
                writeln!(f, "swiss_ball_body_weight_wall_squat")
            }
            SquatExerciseName::WeightedSwissBallWallSquat => {
                writeln!(f, "weighted_swiss_ball_wall_squat")
            }
            SquatExerciseName::Thrusters => writeln!(f, "thrusters"),
            SquatExerciseName::UnevenSquat => writeln!(f, "uneven_squat"),
            SquatExerciseName::WeightedUnevenSquat => writeln!(f, "weighted_uneven_squat"),
            SquatExerciseName::WaistSlimmingSquat => writeln!(f, "waist_slimming_squat"),
            SquatExerciseName::WallBall => writeln!(f, "wall_ball"),
            SquatExerciseName::WideStanceBarbellSquat => writeln!(f, "wide_stance_barbell_squat"),
            SquatExerciseName::WideStanceGobletSquat => writeln!(f, "wide_stance_goblet_squat"),
            SquatExerciseName::ZercherSquat => writeln!(f, "zercher_squat"),
            SquatExerciseName::KbsOverhead => writeln!(f, "kbs_overhead"),
            SquatExerciseName::SquatAndSideKick => writeln!(f, "squat_and_side_kick"),
            SquatExerciseName::SquatJumpsInNOut => writeln!(f, "squat_jumps_in_n_out"),
            SquatExerciseName::PilatesPlieSquatsParallelTurnedOutFlatAndHeels => {
                writeln!(f, "pilates_plie_squats_parallel_turned_out_flat_and_heels")
            }
            SquatExerciseName::ReleveStraightLegAndKneeBentWithOneLegVariation => writeln!(
                f,
                "releve_straight_leg_and_knee_bent_with_one_leg_variation"
            ),
            SquatExerciseName::UnknownVariant(value) => writeln!(f, "unknown_variant_{}", *value),
        }
    }
}
impl convert::From<u16> for SquatExerciseName {
    fn from(value: u16) -> Self {
        match value {
            0 => SquatExerciseName::LegPress,
            1 => SquatExerciseName::BackSquatWithBodyBar,
            2 => SquatExerciseName::BackSquats,
            3 => SquatExerciseName::WeightedBackSquats,
            4 => SquatExerciseName::BalancingSquat,
            5 => SquatExerciseName::WeightedBalancingSquat,
            6 => SquatExerciseName::BarbellBackSquat,
            7 => SquatExerciseName::BarbellBoxSquat,
            8 => SquatExerciseName::BarbellFrontSquat,
            9 => SquatExerciseName::BarbellHackSquat,
            10 => SquatExerciseName::BarbellHangSquatSnatch,
            11 => SquatExerciseName::BarbellLateralStepUp,
            12 => SquatExerciseName::BarbellQuarterSquat,
            13 => SquatExerciseName::BarbellSiffSquat,
            14 => SquatExerciseName::BarbellSquatSnatch,
            15 => SquatExerciseName::BarbellSquatWithHeelsRaised,
            16 => SquatExerciseName::BarbellStepover,
            17 => SquatExerciseName::BarbellStepUp,
            18 => SquatExerciseName::BenchSquatWithRotationalChop,
            19 => SquatExerciseName::WeightedBenchSquatWithRotationalChop,
            20 => SquatExerciseName::BodyWeightWallSquat,
            21 => SquatExerciseName::WeightedWallSquat,
            22 => SquatExerciseName::BoxStepSquat,
            23 => SquatExerciseName::WeightedBoxStepSquat,
            24 => SquatExerciseName::BracedSquat,
            25 => SquatExerciseName::CrossedArmBarbellFrontSquat,
            26 => SquatExerciseName::CrossoverDumbbellStepUp,
            27 => SquatExerciseName::DumbbellFrontSquat,
            28 => SquatExerciseName::DumbbellSplitSquat,
            29 => SquatExerciseName::DumbbellSquat,
            30 => SquatExerciseName::DumbbellSquatClean,
            31 => SquatExerciseName::DumbbellStepover,
            32 => SquatExerciseName::DumbbellStepUp,
            33 => SquatExerciseName::ElevatedSingleLegSquat,
            34 => SquatExerciseName::WeightedElevatedSingleLegSquat,
            35 => SquatExerciseName::FigureFourSquats,
            36 => SquatExerciseName::WeightedFigureFourSquats,
            37 => SquatExerciseName::GobletSquat,
            38 => SquatExerciseName::KettlebellSquat,
            39 => SquatExerciseName::KettlebellSwingOverhead,
            40 => SquatExerciseName::KettlebellSwingWithFlipToSquat,
            41 => SquatExerciseName::LateralDumbbellStepUp,
            42 => SquatExerciseName::OneLeggedSquat,
            43 => SquatExerciseName::OverheadDumbbellSquat,
            44 => SquatExerciseName::OverheadSquat,
            45 => SquatExerciseName::PartialSingleLegSquat,
            46 => SquatExerciseName::WeightedPartialSingleLegSquat,
            47 => SquatExerciseName::PistolSquat,
            48 => SquatExerciseName::WeightedPistolSquat,
            49 => SquatExerciseName::PlieSlides,
            50 => SquatExerciseName::WeightedPlieSlides,
            51 => SquatExerciseName::PlieSquat,
            52 => SquatExerciseName::WeightedPlieSquat,
            53 => SquatExerciseName::PrisonerSquat,
            54 => SquatExerciseName::WeightedPrisonerSquat,
            55 => SquatExerciseName::SingleLegBenchGetUp,
            56 => SquatExerciseName::WeightedSingleLegBenchGetUp,
            57 => SquatExerciseName::SingleLegBenchSquat,
            58 => SquatExerciseName::WeightedSingleLegBenchSquat,
            59 => SquatExerciseName::SingleLegSquatOnSwissBall,
            60 => SquatExerciseName::WeightedSingleLegSquatOnSwissBall,
            61 => SquatExerciseName::Squat,
            62 => SquatExerciseName::WeightedSquat,
            63 => SquatExerciseName::SquatsWithBand,
            64 => SquatExerciseName::StaggeredSquat,
            65 => SquatExerciseName::WeightedStaggeredSquat,
            66 => SquatExerciseName::StepUp,
            67 => SquatExerciseName::WeightedStepUp,
            68 => SquatExerciseName::SuitcaseSquats,
            69 => SquatExerciseName::SumoSquat,
            70 => SquatExerciseName::SumoSquatSlideIn,
            71 => SquatExerciseName::WeightedSumoSquatSlideIn,
            72 => SquatExerciseName::SumoSquatToHighPull,
            73 => SquatExerciseName::SumoSquatToStand,
            74 => SquatExerciseName::WeightedSumoSquatToStand,
            75 => SquatExerciseName::SumoSquatWithRotation,
            76 => SquatExerciseName::WeightedSumoSquatWithRotation,
            77 => SquatExerciseName::SwissBallBodyWeightWallSquat,
            78 => SquatExerciseName::WeightedSwissBallWallSquat,
            79 => SquatExerciseName::Thrusters,
            80 => SquatExerciseName::UnevenSquat,
            81 => SquatExerciseName::WeightedUnevenSquat,
            82 => SquatExerciseName::WaistSlimmingSquat,
            83 => SquatExerciseName::WallBall,
            84 => SquatExerciseName::WideStanceBarbellSquat,
            85 => SquatExerciseName::WideStanceGobletSquat,
            86 => SquatExerciseName::ZercherSquat,
            87 => SquatExerciseName::KbsOverhead,
            88 => SquatExerciseName::SquatAndSideKick,
            89 => SquatExerciseName::SquatJumpsInNOut,
            90 => SquatExerciseName::PilatesPlieSquatsParallelTurnedOutFlatAndHeels,
            91 => SquatExerciseName::ReleveStraightLegAndKneeBentWithOneLegVariation,
            _ => SquatExerciseName::UnknownVariant(value),
        }
    }
}
impl convert::From<i64> for SquatExerciseName {
    fn from(value: i64) -> Self {
        SquatExerciseName::from(value as u16)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum TotalBodyExerciseName {
    Burpee,
    WeightedBurpee,
    BurpeeBoxJump,
    WeightedBurpeeBoxJump,
    HighPullBurpee,
    ManMakers,
    OneArmBurpee,
    SquatThrusts,
    WeightedSquatThrusts,
    SquatPlankPushUp,
    WeightedSquatPlankPushUp,
    StandingTRotationBalance,
    WeightedStandingTRotationBalance,
    UnknownVariant(u16),
}
impl TotalBodyExerciseName {
    pub fn as_u16(self) -> u16 {
        match self {
            TotalBodyExerciseName::Burpee => 0,
            TotalBodyExerciseName::WeightedBurpee => 1,
            TotalBodyExerciseName::BurpeeBoxJump => 2,
            TotalBodyExerciseName::WeightedBurpeeBoxJump => 3,
            TotalBodyExerciseName::HighPullBurpee => 4,
            TotalBodyExerciseName::ManMakers => 5,
            TotalBodyExerciseName::OneArmBurpee => 6,
            TotalBodyExerciseName::SquatThrusts => 7,
            TotalBodyExerciseName::WeightedSquatThrusts => 8,
            TotalBodyExerciseName::SquatPlankPushUp => 9,
            TotalBodyExerciseName::WeightedSquatPlankPushUp => 10,
            TotalBodyExerciseName::StandingTRotationBalance => 11,
            TotalBodyExerciseName::WeightedStandingTRotationBalance => 12,
            TotalBodyExerciseName::UnknownVariant(value) => value,
        }
    }
    pub fn as_i64(self) -> i64 {
        self.as_u16() as i64
    }
}
impl fmt::Display for TotalBodyExerciseName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            TotalBodyExerciseName::Burpee => writeln!(f, "burpee"),
            TotalBodyExerciseName::WeightedBurpee => writeln!(f, "weighted_burpee"),
            TotalBodyExerciseName::BurpeeBoxJump => writeln!(f, "burpee_box_jump"),
            TotalBodyExerciseName::WeightedBurpeeBoxJump => writeln!(f, "weighted_burpee_box_jump"),
            TotalBodyExerciseName::HighPullBurpee => writeln!(f, "high_pull_burpee"),
            TotalBodyExerciseName::ManMakers => writeln!(f, "man_makers"),
            TotalBodyExerciseName::OneArmBurpee => writeln!(f, "one_arm_burpee"),
            TotalBodyExerciseName::SquatThrusts => writeln!(f, "squat_thrusts"),
            TotalBodyExerciseName::WeightedSquatThrusts => writeln!(f, "weighted_squat_thrusts"),
            TotalBodyExerciseName::SquatPlankPushUp => writeln!(f, "squat_plank_push_up"),
            TotalBodyExerciseName::WeightedSquatPlankPushUp => {
                writeln!(f, "weighted_squat_plank_push_up")
            }
            TotalBodyExerciseName::StandingTRotationBalance => {
                writeln!(f, "standing_t_rotation_balance")
            }
            TotalBodyExerciseName::WeightedStandingTRotationBalance => {
                writeln!(f, "weighted_standing_t_rotation_balance")
            }
            TotalBodyExerciseName::UnknownVariant(value) => {
                writeln!(f, "unknown_variant_{}", *value)
            }
        }
    }
}
impl convert::From<u16> for TotalBodyExerciseName {
    fn from(value: u16) -> Self {
        match value {
            0 => TotalBodyExerciseName::Burpee,
            1 => TotalBodyExerciseName::WeightedBurpee,
            2 => TotalBodyExerciseName::BurpeeBoxJump,
            3 => TotalBodyExerciseName::WeightedBurpeeBoxJump,
            4 => TotalBodyExerciseName::HighPullBurpee,
            5 => TotalBodyExerciseName::ManMakers,
            6 => TotalBodyExerciseName::OneArmBurpee,
            7 => TotalBodyExerciseName::SquatThrusts,
            8 => TotalBodyExerciseName::WeightedSquatThrusts,
            9 => TotalBodyExerciseName::SquatPlankPushUp,
            10 => TotalBodyExerciseName::WeightedSquatPlankPushUp,
            11 => TotalBodyExerciseName::StandingTRotationBalance,
            12 => TotalBodyExerciseName::WeightedStandingTRotationBalance,
            _ => TotalBodyExerciseName::UnknownVariant(value),
        }
    }
}
impl convert::From<i64> for TotalBodyExerciseName {
    fn from(value: i64) -> Self {
        TotalBodyExerciseName::from(value as u16)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum TricepsExtensionExerciseName {
    BenchDip,
    WeightedBenchDip,
    BodyWeightDip,
    CableKickback,
    CableLyingTricepsExtension,
    CableOverheadTricepsExtension,
    DumbbellKickback,
    DumbbellLyingTricepsExtension,
    EzBarOverheadTricepsExtension,
    InclineDip,
    WeightedInclineDip,
    InclineEzBarLyingTricepsExtension,
    LyingDumbbellPulloverToExtension,
    LyingEzBarTricepsExtension,
    LyingTricepsExtensionToCloseGripBenchPress,
    OverheadDumbbellTricepsExtension,
    RecliningTricepsPress,
    ReverseGripPressdown,
    ReverseGripTricepsPressdown,
    RopePressdown,
    SeatedBarbellOverheadTricepsExtension,
    SeatedDumbbellOverheadTricepsExtension,
    SeatedEzBarOverheadTricepsExtension,
    SeatedSingleArmOverheadDumbbellExtension,
    SingleArmDumbbellOverheadTricepsExtension,
    SingleDumbbellSeatedOverheadTricepsExtension,
    SingleLegBenchDipAndKick,
    WeightedSingleLegBenchDipAndKick,
    SingleLegDip,
    WeightedSingleLegDip,
    StaticLyingTricepsExtension,
    SuspendedDip,
    WeightedSuspendedDip,
    SwissBallDumbbellLyingTricepsExtension,
    SwissBallEzBarLyingTricepsExtension,
    SwissBallEzBarOverheadTricepsExtension,
    TabletopDip,
    WeightedTabletopDip,
    TricepsExtensionOnFloor,
    TricepsPressdown,
    WeightedDip,
    UnknownVariant(u16),
}
impl TricepsExtensionExerciseName {
    pub fn as_u16(self) -> u16 {
        match self {
            TricepsExtensionExerciseName::BenchDip => 0,
            TricepsExtensionExerciseName::WeightedBenchDip => 1,
            TricepsExtensionExerciseName::BodyWeightDip => 2,
            TricepsExtensionExerciseName::CableKickback => 3,
            TricepsExtensionExerciseName::CableLyingTricepsExtension => 4,
            TricepsExtensionExerciseName::CableOverheadTricepsExtension => 5,
            TricepsExtensionExerciseName::DumbbellKickback => 6,
            TricepsExtensionExerciseName::DumbbellLyingTricepsExtension => 7,
            TricepsExtensionExerciseName::EzBarOverheadTricepsExtension => 8,
            TricepsExtensionExerciseName::InclineDip => 9,
            TricepsExtensionExerciseName::WeightedInclineDip => 10,
            TricepsExtensionExerciseName::InclineEzBarLyingTricepsExtension => 11,
            TricepsExtensionExerciseName::LyingDumbbellPulloverToExtension => 12,
            TricepsExtensionExerciseName::LyingEzBarTricepsExtension => 13,
            TricepsExtensionExerciseName::LyingTricepsExtensionToCloseGripBenchPress => 14,
            TricepsExtensionExerciseName::OverheadDumbbellTricepsExtension => 15,
            TricepsExtensionExerciseName::RecliningTricepsPress => 16,
            TricepsExtensionExerciseName::ReverseGripPressdown => 17,
            TricepsExtensionExerciseName::ReverseGripTricepsPressdown => 18,
            TricepsExtensionExerciseName::RopePressdown => 19,
            TricepsExtensionExerciseName::SeatedBarbellOverheadTricepsExtension => 20,
            TricepsExtensionExerciseName::SeatedDumbbellOverheadTricepsExtension => 21,
            TricepsExtensionExerciseName::SeatedEzBarOverheadTricepsExtension => 22,
            TricepsExtensionExerciseName::SeatedSingleArmOverheadDumbbellExtension => 23,
            TricepsExtensionExerciseName::SingleArmDumbbellOverheadTricepsExtension => 24,
            TricepsExtensionExerciseName::SingleDumbbellSeatedOverheadTricepsExtension => 25,
            TricepsExtensionExerciseName::SingleLegBenchDipAndKick => 26,
            TricepsExtensionExerciseName::WeightedSingleLegBenchDipAndKick => 27,
            TricepsExtensionExerciseName::SingleLegDip => 28,
            TricepsExtensionExerciseName::WeightedSingleLegDip => 29,
            TricepsExtensionExerciseName::StaticLyingTricepsExtension => 30,
            TricepsExtensionExerciseName::SuspendedDip => 31,
            TricepsExtensionExerciseName::WeightedSuspendedDip => 32,
            TricepsExtensionExerciseName::SwissBallDumbbellLyingTricepsExtension => 33,
            TricepsExtensionExerciseName::SwissBallEzBarLyingTricepsExtension => 34,
            TricepsExtensionExerciseName::SwissBallEzBarOverheadTricepsExtension => 35,
            TricepsExtensionExerciseName::TabletopDip => 36,
            TricepsExtensionExerciseName::WeightedTabletopDip => 37,
            TricepsExtensionExerciseName::TricepsExtensionOnFloor => 38,
            TricepsExtensionExerciseName::TricepsPressdown => 39,
            TricepsExtensionExerciseName::WeightedDip => 40,
            TricepsExtensionExerciseName::UnknownVariant(value) => value,
        }
    }
    pub fn as_i64(self) -> i64 {
        self.as_u16() as i64
    }
}
impl fmt::Display for TricepsExtensionExerciseName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            TricepsExtensionExerciseName::BenchDip => writeln!(f, "bench_dip"),
            TricepsExtensionExerciseName::WeightedBenchDip => writeln!(f, "weighted_bench_dip"),
            TricepsExtensionExerciseName::BodyWeightDip => writeln!(f, "body_weight_dip"),
            TricepsExtensionExerciseName::CableKickback => writeln!(f, "cable_kickback"),
            TricepsExtensionExerciseName::CableLyingTricepsExtension => {
                writeln!(f, "cable_lying_triceps_extension")
            }
            TricepsExtensionExerciseName::CableOverheadTricepsExtension => {
                writeln!(f, "cable_overhead_triceps_extension")
            }
            TricepsExtensionExerciseName::DumbbellKickback => writeln!(f, "dumbbell_kickback"),
            TricepsExtensionExerciseName::DumbbellLyingTricepsExtension => {
                writeln!(f, "dumbbell_lying_triceps_extension")
            }
            TricepsExtensionExerciseName::EzBarOverheadTricepsExtension => {
                writeln!(f, "ez_bar_overhead_triceps_extension")
            }
            TricepsExtensionExerciseName::InclineDip => writeln!(f, "incline_dip"),
            TricepsExtensionExerciseName::WeightedInclineDip => writeln!(f, "weighted_incline_dip"),
            TricepsExtensionExerciseName::InclineEzBarLyingTricepsExtension => {
                writeln!(f, "incline_ez_bar_lying_triceps_extension")
            }
            TricepsExtensionExerciseName::LyingDumbbellPulloverToExtension => {
                writeln!(f, "lying_dumbbell_pullover_to_extension")
            }
            TricepsExtensionExerciseName::LyingEzBarTricepsExtension => {
                writeln!(f, "lying_ez_bar_triceps_extension")
            }
            TricepsExtensionExerciseName::LyingTricepsExtensionToCloseGripBenchPress => {
                writeln!(f, "lying_triceps_extension_to_close_grip_bench_press")
            }
            TricepsExtensionExerciseName::OverheadDumbbellTricepsExtension => {
                writeln!(f, "overhead_dumbbell_triceps_extension")
            }
            TricepsExtensionExerciseName::RecliningTricepsPress => {
                writeln!(f, "reclining_triceps_press")
            }
            TricepsExtensionExerciseName::ReverseGripPressdown => {
                writeln!(f, "reverse_grip_pressdown")
            }
            TricepsExtensionExerciseName::ReverseGripTricepsPressdown => {
                writeln!(f, "reverse_grip_triceps_pressdown")
            }
            TricepsExtensionExerciseName::RopePressdown => writeln!(f, "rope_pressdown"),
            TricepsExtensionExerciseName::SeatedBarbellOverheadTricepsExtension => {
                writeln!(f, "seated_barbell_overhead_triceps_extension")
            }
            TricepsExtensionExerciseName::SeatedDumbbellOverheadTricepsExtension => {
                writeln!(f, "seated_dumbbell_overhead_triceps_extension")
            }
            TricepsExtensionExerciseName::SeatedEzBarOverheadTricepsExtension => {
                writeln!(f, "seated_ez_bar_overhead_triceps_extension")
            }
            TricepsExtensionExerciseName::SeatedSingleArmOverheadDumbbellExtension => {
                writeln!(f, "seated_single_arm_overhead_dumbbell_extension")
            }
            TricepsExtensionExerciseName::SingleArmDumbbellOverheadTricepsExtension => {
                writeln!(f, "single_arm_dumbbell_overhead_triceps_extension")
            }
            TricepsExtensionExerciseName::SingleDumbbellSeatedOverheadTricepsExtension => {
                writeln!(f, "single_dumbbell_seated_overhead_triceps_extension")
            }
            TricepsExtensionExerciseName::SingleLegBenchDipAndKick => {
                writeln!(f, "single_leg_bench_dip_and_kick")
            }
            TricepsExtensionExerciseName::WeightedSingleLegBenchDipAndKick => {
                writeln!(f, "weighted_single_leg_bench_dip_and_kick")
            }
            TricepsExtensionExerciseName::SingleLegDip => writeln!(f, "single_leg_dip"),
            TricepsExtensionExerciseName::WeightedSingleLegDip => {
                writeln!(f, "weighted_single_leg_dip")
            }
            TricepsExtensionExerciseName::StaticLyingTricepsExtension => {
                writeln!(f, "static_lying_triceps_extension")
            }
            TricepsExtensionExerciseName::SuspendedDip => writeln!(f, "suspended_dip"),
            TricepsExtensionExerciseName::WeightedSuspendedDip => {
                writeln!(f, "weighted_suspended_dip")
            }
            TricepsExtensionExerciseName::SwissBallDumbbellLyingTricepsExtension => {
                writeln!(f, "swiss_ball_dumbbell_lying_triceps_extension")
            }
            TricepsExtensionExerciseName::SwissBallEzBarLyingTricepsExtension => {
                writeln!(f, "swiss_ball_ez_bar_lying_triceps_extension")
            }
            TricepsExtensionExerciseName::SwissBallEzBarOverheadTricepsExtension => {
                writeln!(f, "swiss_ball_ez_bar_overhead_triceps_extension")
            }
            TricepsExtensionExerciseName::TabletopDip => writeln!(f, "tabletop_dip"),
            TricepsExtensionExerciseName::WeightedTabletopDip => {
                writeln!(f, "weighted_tabletop_dip")
            }
            TricepsExtensionExerciseName::TricepsExtensionOnFloor => {
                writeln!(f, "triceps_extension_on_floor")
            }
            TricepsExtensionExerciseName::TricepsPressdown => writeln!(f, "triceps_pressdown"),
            TricepsExtensionExerciseName::WeightedDip => writeln!(f, "weighted_dip"),
            TricepsExtensionExerciseName::UnknownVariant(value) => {
                writeln!(f, "unknown_variant_{}", *value)
            }
        }
    }
}
impl convert::From<u16> for TricepsExtensionExerciseName {
    fn from(value: u16) -> Self {
        match value {
            0 => TricepsExtensionExerciseName::BenchDip,
            1 => TricepsExtensionExerciseName::WeightedBenchDip,
            2 => TricepsExtensionExerciseName::BodyWeightDip,
            3 => TricepsExtensionExerciseName::CableKickback,
            4 => TricepsExtensionExerciseName::CableLyingTricepsExtension,
            5 => TricepsExtensionExerciseName::CableOverheadTricepsExtension,
            6 => TricepsExtensionExerciseName::DumbbellKickback,
            7 => TricepsExtensionExerciseName::DumbbellLyingTricepsExtension,
            8 => TricepsExtensionExerciseName::EzBarOverheadTricepsExtension,
            9 => TricepsExtensionExerciseName::InclineDip,
            10 => TricepsExtensionExerciseName::WeightedInclineDip,
            11 => TricepsExtensionExerciseName::InclineEzBarLyingTricepsExtension,
            12 => TricepsExtensionExerciseName::LyingDumbbellPulloverToExtension,
            13 => TricepsExtensionExerciseName::LyingEzBarTricepsExtension,
            14 => TricepsExtensionExerciseName::LyingTricepsExtensionToCloseGripBenchPress,
            15 => TricepsExtensionExerciseName::OverheadDumbbellTricepsExtension,
            16 => TricepsExtensionExerciseName::RecliningTricepsPress,
            17 => TricepsExtensionExerciseName::ReverseGripPressdown,
            18 => TricepsExtensionExerciseName::ReverseGripTricepsPressdown,
            19 => TricepsExtensionExerciseName::RopePressdown,
            20 => TricepsExtensionExerciseName::SeatedBarbellOverheadTricepsExtension,
            21 => TricepsExtensionExerciseName::SeatedDumbbellOverheadTricepsExtension,
            22 => TricepsExtensionExerciseName::SeatedEzBarOverheadTricepsExtension,
            23 => TricepsExtensionExerciseName::SeatedSingleArmOverheadDumbbellExtension,
            24 => TricepsExtensionExerciseName::SingleArmDumbbellOverheadTricepsExtension,
            25 => TricepsExtensionExerciseName::SingleDumbbellSeatedOverheadTricepsExtension,
            26 => TricepsExtensionExerciseName::SingleLegBenchDipAndKick,
            27 => TricepsExtensionExerciseName::WeightedSingleLegBenchDipAndKick,
            28 => TricepsExtensionExerciseName::SingleLegDip,
            29 => TricepsExtensionExerciseName::WeightedSingleLegDip,
            30 => TricepsExtensionExerciseName::StaticLyingTricepsExtension,
            31 => TricepsExtensionExerciseName::SuspendedDip,
            32 => TricepsExtensionExerciseName::WeightedSuspendedDip,
            33 => TricepsExtensionExerciseName::SwissBallDumbbellLyingTricepsExtension,
            34 => TricepsExtensionExerciseName::SwissBallEzBarLyingTricepsExtension,
            35 => TricepsExtensionExerciseName::SwissBallEzBarOverheadTricepsExtension,
            36 => TricepsExtensionExerciseName::TabletopDip,
            37 => TricepsExtensionExerciseName::WeightedTabletopDip,
            38 => TricepsExtensionExerciseName::TricepsExtensionOnFloor,
            39 => TricepsExtensionExerciseName::TricepsPressdown,
            40 => TricepsExtensionExerciseName::WeightedDip,
            _ => TricepsExtensionExerciseName::UnknownVariant(value),
        }
    }
}
impl convert::From<i64> for TricepsExtensionExerciseName {
    fn from(value: i64) -> Self {
        TricepsExtensionExerciseName::from(value as u16)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum WarmUpExerciseName {
    QuadrupedRocking,
    NeckTilts,
    AnkleCircles,
    AnkleDorsiflexionWithBand,
    AnkleInternalRotation,
    ArmCircles,
    BentOverReachToSky,
    CatCamel,
    ElbowToFootLunge,
    ForwardAndBackwardLegSwings,
    Groiners,
    InvertedHamstringStretch,
    LateralDuckUnder,
    NeckRotations,
    OppositeArmAndLegBalance,
    ReachRollAndLift,
    Scorpion,
    ShoulderCircles,
    SideToSideLegSwings,
    SleeperStretch,
    SlideOut,
    SwissBallHipCrossover,
    SwissBallReachRollAndLift,
    SwissBallWindshieldWipers,
    ThoracicRotation,
    WalkingHighKicks,
    WalkingHighKnees,
    WalkingKneeHugs,
    WalkingLegCradles,
    Walkout,
    WalkoutFromPushUpPosition,
    UnknownVariant(u16),
}
impl WarmUpExerciseName {
    pub fn as_u16(self) -> u16 {
        match self {
            WarmUpExerciseName::QuadrupedRocking => 0,
            WarmUpExerciseName::NeckTilts => 1,
            WarmUpExerciseName::AnkleCircles => 2,
            WarmUpExerciseName::AnkleDorsiflexionWithBand => 3,
            WarmUpExerciseName::AnkleInternalRotation => 4,
            WarmUpExerciseName::ArmCircles => 5,
            WarmUpExerciseName::BentOverReachToSky => 6,
            WarmUpExerciseName::CatCamel => 7,
            WarmUpExerciseName::ElbowToFootLunge => 8,
            WarmUpExerciseName::ForwardAndBackwardLegSwings => 9,
            WarmUpExerciseName::Groiners => 10,
            WarmUpExerciseName::InvertedHamstringStretch => 11,
            WarmUpExerciseName::LateralDuckUnder => 12,
            WarmUpExerciseName::NeckRotations => 13,
            WarmUpExerciseName::OppositeArmAndLegBalance => 14,
            WarmUpExerciseName::ReachRollAndLift => 15,
            WarmUpExerciseName::Scorpion => 16,
            WarmUpExerciseName::ShoulderCircles => 17,
            WarmUpExerciseName::SideToSideLegSwings => 18,
            WarmUpExerciseName::SleeperStretch => 19,
            WarmUpExerciseName::SlideOut => 20,
            WarmUpExerciseName::SwissBallHipCrossover => 21,
            WarmUpExerciseName::SwissBallReachRollAndLift => 22,
            WarmUpExerciseName::SwissBallWindshieldWipers => 23,
            WarmUpExerciseName::ThoracicRotation => 24,
            WarmUpExerciseName::WalkingHighKicks => 25,
            WarmUpExerciseName::WalkingHighKnees => 26,
            WarmUpExerciseName::WalkingKneeHugs => 27,
            WarmUpExerciseName::WalkingLegCradles => 28,
            WarmUpExerciseName::Walkout => 29,
            WarmUpExerciseName::WalkoutFromPushUpPosition => 30,
            WarmUpExerciseName::UnknownVariant(value) => value,
        }
    }
    pub fn as_i64(self) -> i64 {
        self.as_u16() as i64
    }
}
impl fmt::Display for WarmUpExerciseName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            WarmUpExerciseName::QuadrupedRocking => writeln!(f, "quadruped_rocking"),
            WarmUpExerciseName::NeckTilts => writeln!(f, "neck_tilts"),
            WarmUpExerciseName::AnkleCircles => writeln!(f, "ankle_circles"),
            WarmUpExerciseName::AnkleDorsiflexionWithBand => {
                writeln!(f, "ankle_dorsiflexion_with_band")
            }
            WarmUpExerciseName::AnkleInternalRotation => writeln!(f, "ankle_internal_rotation"),
            WarmUpExerciseName::ArmCircles => writeln!(f, "arm_circles"),
            WarmUpExerciseName::BentOverReachToSky => writeln!(f, "bent_over_reach_to_sky"),
            WarmUpExerciseName::CatCamel => writeln!(f, "cat_camel"),
            WarmUpExerciseName::ElbowToFootLunge => writeln!(f, "elbow_to_foot_lunge"),
            WarmUpExerciseName::ForwardAndBackwardLegSwings => {
                writeln!(f, "forward_and_backward_leg_swings")
            }
            WarmUpExerciseName::Groiners => writeln!(f, "groiners"),
            WarmUpExerciseName::InvertedHamstringStretch => {
                writeln!(f, "inverted_hamstring_stretch")
            }
            WarmUpExerciseName::LateralDuckUnder => writeln!(f, "lateral_duck_under"),
            WarmUpExerciseName::NeckRotations => writeln!(f, "neck_rotations"),
            WarmUpExerciseName::OppositeArmAndLegBalance => {
                writeln!(f, "opposite_arm_and_leg_balance")
            }
            WarmUpExerciseName::ReachRollAndLift => writeln!(f, "reach_roll_and_lift"),
            WarmUpExerciseName::Scorpion => writeln!(f, "scorpion"),
            WarmUpExerciseName::ShoulderCircles => writeln!(f, "shoulder_circles"),
            WarmUpExerciseName::SideToSideLegSwings => writeln!(f, "side_to_side_leg_swings"),
            WarmUpExerciseName::SleeperStretch => writeln!(f, "sleeper_stretch"),
            WarmUpExerciseName::SlideOut => writeln!(f, "slide_out"),
            WarmUpExerciseName::SwissBallHipCrossover => writeln!(f, "swiss_ball_hip_crossover"),
            WarmUpExerciseName::SwissBallReachRollAndLift => {
                writeln!(f, "swiss_ball_reach_roll_and_lift")
            }
            WarmUpExerciseName::SwissBallWindshieldWipers => {
                writeln!(f, "swiss_ball_windshield_wipers")
            }
            WarmUpExerciseName::ThoracicRotation => writeln!(f, "thoracic_rotation"),
            WarmUpExerciseName::WalkingHighKicks => writeln!(f, "walking_high_kicks"),
            WarmUpExerciseName::WalkingHighKnees => writeln!(f, "walking_high_knees"),
            WarmUpExerciseName::WalkingKneeHugs => writeln!(f, "walking_knee_hugs"),
            WarmUpExerciseName::WalkingLegCradles => writeln!(f, "walking_leg_cradles"),
            WarmUpExerciseName::Walkout => writeln!(f, "walkout"),
            WarmUpExerciseName::WalkoutFromPushUpPosition => {
                writeln!(f, "walkout_from_push_up_position")
            }
            WarmUpExerciseName::UnknownVariant(value) => writeln!(f, "unknown_variant_{}", *value),
        }
    }
}
impl convert::From<u16> for WarmUpExerciseName {
    fn from(value: u16) -> Self {
        match value {
            0 => WarmUpExerciseName::QuadrupedRocking,
            1 => WarmUpExerciseName::NeckTilts,
            2 => WarmUpExerciseName::AnkleCircles,
            3 => WarmUpExerciseName::AnkleDorsiflexionWithBand,
            4 => WarmUpExerciseName::AnkleInternalRotation,
            5 => WarmUpExerciseName::ArmCircles,
            6 => WarmUpExerciseName::BentOverReachToSky,
            7 => WarmUpExerciseName::CatCamel,
            8 => WarmUpExerciseName::ElbowToFootLunge,
            9 => WarmUpExerciseName::ForwardAndBackwardLegSwings,
            10 => WarmUpExerciseName::Groiners,
            11 => WarmUpExerciseName::InvertedHamstringStretch,
            12 => WarmUpExerciseName::LateralDuckUnder,
            13 => WarmUpExerciseName::NeckRotations,
            14 => WarmUpExerciseName::OppositeArmAndLegBalance,
            15 => WarmUpExerciseName::ReachRollAndLift,
            16 => WarmUpExerciseName::Scorpion,
            17 => WarmUpExerciseName::ShoulderCircles,
            18 => WarmUpExerciseName::SideToSideLegSwings,
            19 => WarmUpExerciseName::SleeperStretch,
            20 => WarmUpExerciseName::SlideOut,
            21 => WarmUpExerciseName::SwissBallHipCrossover,
            22 => WarmUpExerciseName::SwissBallReachRollAndLift,
            23 => WarmUpExerciseName::SwissBallWindshieldWipers,
            24 => WarmUpExerciseName::ThoracicRotation,
            25 => WarmUpExerciseName::WalkingHighKicks,
            26 => WarmUpExerciseName::WalkingHighKnees,
            27 => WarmUpExerciseName::WalkingKneeHugs,
            28 => WarmUpExerciseName::WalkingLegCradles,
            29 => WarmUpExerciseName::Walkout,
            30 => WarmUpExerciseName::WalkoutFromPushUpPosition,
            _ => WarmUpExerciseName::UnknownVariant(value),
        }
    }
}
impl convert::From<i64> for WarmUpExerciseName {
    fn from(value: i64) -> Self {
        WarmUpExerciseName::from(value as u16)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum RunExerciseName {
    Run,
    Walk,
    Jog,
    Sprint,
    UnknownVariant(u16),
}
impl RunExerciseName {
    pub fn as_u16(self) -> u16 {
        match self {
            RunExerciseName::Run => 0,
            RunExerciseName::Walk => 1,
            RunExerciseName::Jog => 2,
            RunExerciseName::Sprint => 3,
            RunExerciseName::UnknownVariant(value) => value,
        }
    }
    pub fn as_i64(self) -> i64 {
        self.as_u16() as i64
    }
}
impl fmt::Display for RunExerciseName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            RunExerciseName::Run => writeln!(f, "run"),
            RunExerciseName::Walk => writeln!(f, "walk"),
            RunExerciseName::Jog => writeln!(f, "jog"),
            RunExerciseName::Sprint => writeln!(f, "sprint"),
            RunExerciseName::UnknownVariant(value) => writeln!(f, "unknown_variant_{}", *value),
        }
    }
}
impl convert::From<u16> for RunExerciseName {
    fn from(value: u16) -> Self {
        match value {
            0 => RunExerciseName::Run,
            1 => RunExerciseName::Walk,
            2 => RunExerciseName::Jog,
            3 => RunExerciseName::Sprint,
            _ => RunExerciseName::UnknownVariant(value),
        }
    }
}
impl convert::From<i64> for RunExerciseName {
    fn from(value: i64) -> Self {
        RunExerciseName::from(value as u16)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum WaterType {
    Fresh,
    Salt,
    En13319,
    Custom,
    UnknownVariant(u8),
}
impl WaterType {
    pub fn as_u8(self) -> u8 {
        match self {
            WaterType::Fresh => 0,
            WaterType::Salt => 1,
            WaterType::En13319 => 2,
            WaterType::Custom => 3,
            WaterType::UnknownVariant(value) => value,
        }
    }
    pub fn as_i64(self) -> i64 {
        self.as_u8() as i64
    }
}
impl fmt::Display for WaterType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            WaterType::Fresh => writeln!(f, "fresh"),
            WaterType::Salt => writeln!(f, "salt"),
            WaterType::En13319 => writeln!(f, "en13319"),
            WaterType::Custom => writeln!(f, "custom"),
            WaterType::UnknownVariant(value) => writeln!(f, "unknown_variant_{}", *value),
        }
    }
}
impl convert::From<u8> for WaterType {
    fn from(value: u8) -> Self {
        match value {
            0 => WaterType::Fresh,
            1 => WaterType::Salt,
            2 => WaterType::En13319,
            3 => WaterType::Custom,
            _ => WaterType::UnknownVariant(value),
        }
    }
}
impl convert::From<i64> for WaterType {
    fn from(value: i64) -> Self {
        WaterType::from(value as u8)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum TissueModelType {
    /// Buhlmann's decompression algorithm, version C
    Zhl16c,
    UnknownVariant(u8),
}
impl TissueModelType {
    pub fn as_u8(self) -> u8 {
        match self {
            TissueModelType::Zhl16c => 0,
            TissueModelType::UnknownVariant(value) => value,
        }
    }
    pub fn as_i64(self) -> i64 {
        self.as_u8() as i64
    }
}
impl fmt::Display for TissueModelType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            TissueModelType::Zhl16c => writeln!(f, "zhl_16c"),
            TissueModelType::UnknownVariant(value) => writeln!(f, "unknown_variant_{}", *value),
        }
    }
}
impl convert::From<u8> for TissueModelType {
    fn from(value: u8) -> Self {
        match value {
            0 => TissueModelType::Zhl16c,
            _ => TissueModelType::UnknownVariant(value),
        }
    }
}
impl convert::From<i64> for TissueModelType {
    fn from(value: i64) -> Self {
        TissueModelType::from(value as u8)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum DiveGasStatus {
    Disabled,
    Enabled,
    BackupOnly,
    UnknownVariant(u8),
}
impl DiveGasStatus {
    pub fn as_u8(self) -> u8 {
        match self {
            DiveGasStatus::Disabled => 0,
            DiveGasStatus::Enabled => 1,
            DiveGasStatus::BackupOnly => 2,
            DiveGasStatus::UnknownVariant(value) => value,
        }
    }
    pub fn as_i64(self) -> i64 {
        self.as_u8() as i64
    }
}
impl fmt::Display for DiveGasStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            DiveGasStatus::Disabled => writeln!(f, "disabled"),
            DiveGasStatus::Enabled => writeln!(f, "enabled"),
            DiveGasStatus::BackupOnly => writeln!(f, "backup_only"),
            DiveGasStatus::UnknownVariant(value) => writeln!(f, "unknown_variant_{}", *value),
        }
    }
}
impl convert::From<u8> for DiveGasStatus {
    fn from(value: u8) -> Self {
        match value {
            0 => DiveGasStatus::Disabled,
            1 => DiveGasStatus::Enabled,
            2 => DiveGasStatus::BackupOnly,
            _ => DiveGasStatus::UnknownVariant(value),
        }
    }
}
impl convert::From<i64> for DiveGasStatus {
    fn from(value: i64) -> Self {
        DiveGasStatus::from(value as u8)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum DiveAlarmType {
    Depth,
    Time,
    UnknownVariant(u8),
}
impl DiveAlarmType {
    pub fn as_u8(self) -> u8 {
        match self {
            DiveAlarmType::Depth => 0,
            DiveAlarmType::Time => 1,
            DiveAlarmType::UnknownVariant(value) => value,
        }
    }
    pub fn as_i64(self) -> i64 {
        self.as_u8() as i64
    }
}
impl fmt::Display for DiveAlarmType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            DiveAlarmType::Depth => writeln!(f, "depth"),
            DiveAlarmType::Time => writeln!(f, "time"),
            DiveAlarmType::UnknownVariant(value) => writeln!(f, "unknown_variant_{}", *value),
        }
    }
}
impl convert::From<u8> for DiveAlarmType {
    fn from(value: u8) -> Self {
        match value {
            0 => DiveAlarmType::Depth,
            1 => DiveAlarmType::Time,
            _ => DiveAlarmType::UnknownVariant(value),
        }
    }
}
impl convert::From<i64> for DiveAlarmType {
    fn from(value: i64) -> Self {
        DiveAlarmType::from(value as u8)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum DiveBacklightMode {
    AtDepth,
    AlwaysOn,
    UnknownVariant(u8),
}
impl DiveBacklightMode {
    pub fn as_u8(self) -> u8 {
        match self {
            DiveBacklightMode::AtDepth => 0,
            DiveBacklightMode::AlwaysOn => 1,
            DiveBacklightMode::UnknownVariant(value) => value,
        }
    }
    pub fn as_i64(self) -> i64 {
        self.as_u8() as i64
    }
}
impl fmt::Display for DiveBacklightMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            DiveBacklightMode::AtDepth => writeln!(f, "at_depth"),
            DiveBacklightMode::AlwaysOn => writeln!(f, "always_on"),
            DiveBacklightMode::UnknownVariant(value) => writeln!(f, "unknown_variant_{}", *value),
        }
    }
}
impl convert::From<u8> for DiveBacklightMode {
    fn from(value: u8) -> Self {
        match value {
            0 => DiveBacklightMode::AtDepth,
            1 => DiveBacklightMode::AlwaysOn,
            _ => DiveBacklightMode::UnknownVariant(value),
        }
    }
}
impl convert::From<i64> for DiveBacklightMode {
    fn from(value: i64) -> Self {
        DiveBacklightMode::from(value as u8)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum FaveroProduct {
    AssiomaUno,
    AssiomaDuo,
    UnknownVariant(u16),
}
impl FaveroProduct {
    pub fn as_u16(self) -> u16 {
        match self {
            FaveroProduct::AssiomaUno => 10,
            FaveroProduct::AssiomaDuo => 12,
            FaveroProduct::UnknownVariant(value) => value,
        }
    }
    pub fn as_i64(self) -> i64 {
        self.as_u16() as i64
    }
}
impl fmt::Display for FaveroProduct {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            FaveroProduct::AssiomaUno => writeln!(f, "assioma_uno"),
            FaveroProduct::AssiomaDuo => writeln!(f, "assioma_duo"),
            FaveroProduct::UnknownVariant(value) => writeln!(f, "unknown_variant_{}", *value),
        }
    }
}
impl convert::From<u16> for FaveroProduct {
    fn from(value: u16) -> Self {
        match value {
            10 => FaveroProduct::AssiomaUno,
            12 => FaveroProduct::AssiomaDuo,
            _ => FaveroProduct::UnknownVariant(value),
        }
    }
}
impl convert::From<i64> for FaveroProduct {
    fn from(value: i64) -> Self {
        FaveroProduct::from(value as u16)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum ClimbProEvent {
    Approach,
    Start,
    Complete,
    UnknownVariant(u8),
}
impl ClimbProEvent {
    pub fn as_u8(self) -> u8 {
        match self {
            ClimbProEvent::Approach => 0,
            ClimbProEvent::Start => 1,
            ClimbProEvent::Complete => 2,
            ClimbProEvent::UnknownVariant(value) => value,
        }
    }
    pub fn as_i64(self) -> i64 {
        self.as_u8() as i64
    }
}
impl fmt::Display for ClimbProEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            ClimbProEvent::Approach => writeln!(f, "approach"),
            ClimbProEvent::Start => writeln!(f, "start"),
            ClimbProEvent::Complete => writeln!(f, "complete"),
            ClimbProEvent::UnknownVariant(value) => writeln!(f, "unknown_variant_{}", *value),
        }
    }
}
impl convert::From<u8> for ClimbProEvent {
    fn from(value: u8) -> Self {
        match value {
            0 => ClimbProEvent::Approach,
            1 => ClimbProEvent::Start,
            2 => ClimbProEvent::Complete,
            _ => ClimbProEvent::UnknownVariant(value),
        }
    }
}
impl convert::From<i64> for ClimbProEvent {
    fn from(value: i64) -> Self {
        ClimbProEvent::from(value as u8)
    }
}

/// Describe all possible data types of a field
///
/// The Enum type's value is actually an enum of enums.
#[derive(Clone, Copy, Debug)]
pub enum FieldDataType {
    Bool,
    SInt8,
    UInt8,
    SInt16,
    UInt16,
    SInt32,
    UInt32,
    String,
    Float32,
    Float64,
    UInt8z,
    UInt16z,
    UInt32z,
    Byte,
    SInt64,
    UInt64,
    UInt64z,
    File,
    MesgNum,
    Checksum,
    FileFlags,
    MesgCount,
    DateTime,
    LocalDateTime,
    MessageIndex,
    DeviceIndex,
    Gender,
    Language,
    LanguageBits0,
    LanguageBits1,
    LanguageBits2,
    LanguageBits3,
    LanguageBits4,
    TimeZone,
    DisplayMeasure,
    DisplayHeart,
    DisplayPower,
    DisplayPosition,
    Switch,
    Sport,
    SportBits0,
    SportBits1,
    SportBits2,
    SportBits3,
    SportBits4,
    SportBits5,
    SportBits6,
    SubSport,
    SportEvent,
    Activity,
    Intensity,
    SessionTrigger,
    AutolapTrigger,
    LapTrigger,
    TimeMode,
    BacklightMode,
    DateMode,
    BacklightTimeout,
    Event,
    EventType,
    TimerTrigger,
    FitnessEquipmentState,
    Tone,
    Autoscroll,
    ActivityClass,
    HrZoneCalc,
    PwrZoneCalc,
    WktStepDuration,
    WktStepTarget,
    Goal,
    GoalRecurrence,
    GoalSource,
    Schedule,
    CoursePoint,
    Manufacturer,
    GarminProduct,
    AntplusDeviceType,
    AntNetwork,
    WorkoutCapabilities,
    BatteryStatus,
    HrType,
    CourseCapabilities,
    Weight,
    WorkoutHr,
    WorkoutPower,
    BpStatus,
    UserLocalId,
    SwimStroke,
    ActivityType,
    ActivitySubtype,
    ActivityLevel,
    Side,
    LeftRightBalance,
    LeftRightBalance100,
    LengthType,
    DayOfWeek,
    ConnectivityCapabilities,
    WeatherReport,
    WeatherStatus,
    WeatherSeverity,
    WeatherSevereType,
    TimeIntoDay,
    LocaltimeIntoDay,
    StrokeType,
    BodyLocation,
    SegmentLapStatus,
    SegmentLeaderboardType,
    SegmentDeleteStatus,
    SegmentSelectionType,
    SourceType,
    LocalDeviceType,
    DisplayOrientation,
    WorkoutEquipment,
    WatchfaceMode,
    DigitalWatchfaceLayout,
    AnalogWatchfaceLayout,
    RiderPositionType,
    PowerPhaseType,
    CameraEventType,
    SensorType,
    BikeLightNetworkConfigType,
    CommTimeoutType,
    CameraOrientationType,
    AttitudeStage,
    AttitudeValidity,
    AutoSyncFrequency,
    ExdLayout,
    ExdDisplayType,
    ExdDataUnits,
    ExdQualifiers,
    ExdDescriptors,
    AutoActivityDetect,
    SupportedExdScreenLayouts,
    FitBaseType,
    TurnType,
    BikeLightBeamAngleMode,
    FitBaseUnit,
    SetType,
    ExerciseCategory,
    BenchPressExerciseName,
    CalfRaiseExerciseName,
    CardioExerciseName,
    CarryExerciseName,
    ChopExerciseName,
    CoreExerciseName,
    CrunchExerciseName,
    CurlExerciseName,
    DeadliftExerciseName,
    FlyeExerciseName,
    HipRaiseExerciseName,
    HipStabilityExerciseName,
    HipSwingExerciseName,
    HyperextensionExerciseName,
    LateralRaiseExerciseName,
    LegCurlExerciseName,
    LegRaiseExerciseName,
    LungeExerciseName,
    OlympicLiftExerciseName,
    PlankExerciseName,
    PlyoExerciseName,
    PullUpExerciseName,
    PushUpExerciseName,
    RowExerciseName,
    ShoulderPressExerciseName,
    ShoulderStabilityExerciseName,
    ShrugExerciseName,
    SitUpExerciseName,
    SquatExerciseName,
    TotalBodyExerciseName,
    TricepsExtensionExerciseName,
    WarmUpExerciseName,
    RunExerciseName,
    WaterType,
    TissueModelType,
    DiveGasStatus,
    DiveAlarmType,
    DiveBacklightMode,
    FaveroProduct,
    ClimbProEvent,
}
impl FieldDataType {
    pub fn is_enum_type(self) -> bool {
        match self {
            FieldDataType::File => true,
            FieldDataType::MesgNum => true,
            FieldDataType::Checksum => true,
            FieldDataType::FileFlags => true,
            FieldDataType::MesgCount => true,
            FieldDataType::MessageIndex => true,
            FieldDataType::DeviceIndex => true,
            FieldDataType::Gender => true,
            FieldDataType::Language => true,
            FieldDataType::LanguageBits0 => true,
            FieldDataType::LanguageBits1 => true,
            FieldDataType::LanguageBits2 => true,
            FieldDataType::LanguageBits3 => true,
            FieldDataType::LanguageBits4 => true,
            FieldDataType::TimeZone => true,
            FieldDataType::DisplayMeasure => true,
            FieldDataType::DisplayHeart => true,
            FieldDataType::DisplayPower => true,
            FieldDataType::DisplayPosition => true,
            FieldDataType::Switch => true,
            FieldDataType::Sport => true,
            FieldDataType::SportBits0 => true,
            FieldDataType::SportBits1 => true,
            FieldDataType::SportBits2 => true,
            FieldDataType::SportBits3 => true,
            FieldDataType::SportBits4 => true,
            FieldDataType::SportBits5 => true,
            FieldDataType::SportBits6 => true,
            FieldDataType::SubSport => true,
            FieldDataType::SportEvent => true,
            FieldDataType::Activity => true,
            FieldDataType::Intensity => true,
            FieldDataType::SessionTrigger => true,
            FieldDataType::AutolapTrigger => true,
            FieldDataType::LapTrigger => true,
            FieldDataType::TimeMode => true,
            FieldDataType::BacklightMode => true,
            FieldDataType::DateMode => true,
            FieldDataType::BacklightTimeout => true,
            FieldDataType::Event => true,
            FieldDataType::EventType => true,
            FieldDataType::TimerTrigger => true,
            FieldDataType::FitnessEquipmentState => true,
            FieldDataType::Tone => true,
            FieldDataType::Autoscroll => true,
            FieldDataType::ActivityClass => true,
            FieldDataType::HrZoneCalc => true,
            FieldDataType::PwrZoneCalc => true,
            FieldDataType::WktStepDuration => true,
            FieldDataType::WktStepTarget => true,
            FieldDataType::Goal => true,
            FieldDataType::GoalRecurrence => true,
            FieldDataType::GoalSource => true,
            FieldDataType::Schedule => true,
            FieldDataType::CoursePoint => true,
            FieldDataType::Manufacturer => true,
            FieldDataType::GarminProduct => true,
            FieldDataType::AntplusDeviceType => true,
            FieldDataType::AntNetwork => true,
            FieldDataType::WorkoutCapabilities => true,
            FieldDataType::BatteryStatus => true,
            FieldDataType::HrType => true,
            FieldDataType::CourseCapabilities => true,
            FieldDataType::Weight => true,
            FieldDataType::WorkoutHr => true,
            FieldDataType::WorkoutPower => true,
            FieldDataType::BpStatus => true,
            FieldDataType::UserLocalId => true,
            FieldDataType::SwimStroke => true,
            FieldDataType::ActivityType => true,
            FieldDataType::ActivitySubtype => true,
            FieldDataType::ActivityLevel => true,
            FieldDataType::Side => true,
            FieldDataType::LeftRightBalance => true,
            FieldDataType::LeftRightBalance100 => true,
            FieldDataType::LengthType => true,
            FieldDataType::DayOfWeek => true,
            FieldDataType::ConnectivityCapabilities => true,
            FieldDataType::WeatherReport => true,
            FieldDataType::WeatherStatus => true,
            FieldDataType::WeatherSeverity => true,
            FieldDataType::WeatherSevereType => true,
            FieldDataType::StrokeType => true,
            FieldDataType::BodyLocation => true,
            FieldDataType::SegmentLapStatus => true,
            FieldDataType::SegmentLeaderboardType => true,
            FieldDataType::SegmentDeleteStatus => true,
            FieldDataType::SegmentSelectionType => true,
            FieldDataType::SourceType => true,
            FieldDataType::DisplayOrientation => true,
            FieldDataType::WorkoutEquipment => true,
            FieldDataType::WatchfaceMode => true,
            FieldDataType::DigitalWatchfaceLayout => true,
            FieldDataType::AnalogWatchfaceLayout => true,
            FieldDataType::RiderPositionType => true,
            FieldDataType::PowerPhaseType => true,
            FieldDataType::CameraEventType => true,
            FieldDataType::SensorType => true,
            FieldDataType::BikeLightNetworkConfigType => true,
            FieldDataType::CommTimeoutType => true,
            FieldDataType::CameraOrientationType => true,
            FieldDataType::AttitudeStage => true,
            FieldDataType::AttitudeValidity => true,
            FieldDataType::AutoSyncFrequency => true,
            FieldDataType::ExdLayout => true,
            FieldDataType::ExdDisplayType => true,
            FieldDataType::ExdDataUnits => true,
            FieldDataType::ExdQualifiers => true,
            FieldDataType::ExdDescriptors => true,
            FieldDataType::AutoActivityDetect => true,
            FieldDataType::SupportedExdScreenLayouts => true,
            FieldDataType::FitBaseType => true,
            FieldDataType::TurnType => true,
            FieldDataType::BikeLightBeamAngleMode => true,
            FieldDataType::FitBaseUnit => true,
            FieldDataType::SetType => true,
            FieldDataType::ExerciseCategory => true,
            FieldDataType::BenchPressExerciseName => true,
            FieldDataType::CalfRaiseExerciseName => true,
            FieldDataType::CardioExerciseName => true,
            FieldDataType::CarryExerciseName => true,
            FieldDataType::ChopExerciseName => true,
            FieldDataType::CoreExerciseName => true,
            FieldDataType::CrunchExerciseName => true,
            FieldDataType::CurlExerciseName => true,
            FieldDataType::DeadliftExerciseName => true,
            FieldDataType::FlyeExerciseName => true,
            FieldDataType::HipRaiseExerciseName => true,
            FieldDataType::HipStabilityExerciseName => true,
            FieldDataType::HipSwingExerciseName => true,
            FieldDataType::HyperextensionExerciseName => true,
            FieldDataType::LateralRaiseExerciseName => true,
            FieldDataType::LegCurlExerciseName => true,
            FieldDataType::LegRaiseExerciseName => true,
            FieldDataType::LungeExerciseName => true,
            FieldDataType::OlympicLiftExerciseName => true,
            FieldDataType::PlankExerciseName => true,
            FieldDataType::PlyoExerciseName => true,
            FieldDataType::PullUpExerciseName => true,
            FieldDataType::PushUpExerciseName => true,
            FieldDataType::RowExerciseName => true,
            FieldDataType::ShoulderPressExerciseName => true,
            FieldDataType::ShoulderStabilityExerciseName => true,
            FieldDataType::ShrugExerciseName => true,
            FieldDataType::SitUpExerciseName => true,
            FieldDataType::SquatExerciseName => true,
            FieldDataType::TotalBodyExerciseName => true,
            FieldDataType::TricepsExtensionExerciseName => true,
            FieldDataType::WarmUpExerciseName => true,
            FieldDataType::RunExerciseName => true,
            FieldDataType::WaterType => true,
            FieldDataType::TissueModelType => true,
            FieldDataType::DiveGasStatus => true,
            FieldDataType::DiveAlarmType => true,
            FieldDataType::DiveBacklightMode => true,
            FieldDataType::FaveroProduct => true,
            FieldDataType::ClimbProEvent => true,
            _ => false,
        }
    }
}
pub fn get_field_variant_as_string(field_type: FieldDataType, value: i64) -> String {
    match field_type {
        FieldDataType::File => File::from(value).to_string(),
        FieldDataType::MesgNum => MesgNum::from(value).to_string(),
        FieldDataType::Checksum => Checksum::from(value).to_string(),
        FieldDataType::FileFlags => FileFlags::from(value).to_string(),
        FieldDataType::MesgCount => MesgCount::from(value).to_string(),
        FieldDataType::MessageIndex => MessageIndex::from(value).to_string(),
        FieldDataType::DeviceIndex => DeviceIndex::from(value).to_string(),
        FieldDataType::Gender => Gender::from(value).to_string(),
        FieldDataType::Language => Language::from(value).to_string(),
        FieldDataType::LanguageBits0 => LanguageBits0::from(value).to_string(),
        FieldDataType::LanguageBits1 => LanguageBits1::from(value).to_string(),
        FieldDataType::LanguageBits2 => LanguageBits2::from(value).to_string(),
        FieldDataType::LanguageBits3 => LanguageBits3::from(value).to_string(),
        FieldDataType::LanguageBits4 => LanguageBits4::from(value).to_string(),
        FieldDataType::TimeZone => TimeZone::from(value).to_string(),
        FieldDataType::DisplayMeasure => DisplayMeasure::from(value).to_string(),
        FieldDataType::DisplayHeart => DisplayHeart::from(value).to_string(),
        FieldDataType::DisplayPower => DisplayPower::from(value).to_string(),
        FieldDataType::DisplayPosition => DisplayPosition::from(value).to_string(),
        FieldDataType::Switch => Switch::from(value).to_string(),
        FieldDataType::Sport => Sport::from(value).to_string(),
        FieldDataType::SportBits0 => SportBits0::from(value).to_string(),
        FieldDataType::SportBits1 => SportBits1::from(value).to_string(),
        FieldDataType::SportBits2 => SportBits2::from(value).to_string(),
        FieldDataType::SportBits3 => SportBits3::from(value).to_string(),
        FieldDataType::SportBits4 => SportBits4::from(value).to_string(),
        FieldDataType::SportBits5 => SportBits5::from(value).to_string(),
        FieldDataType::SportBits6 => SportBits6::from(value).to_string(),
        FieldDataType::SubSport => SubSport::from(value).to_string(),
        FieldDataType::SportEvent => SportEvent::from(value).to_string(),
        FieldDataType::Activity => Activity::from(value).to_string(),
        FieldDataType::Intensity => Intensity::from(value).to_string(),
        FieldDataType::SessionTrigger => SessionTrigger::from(value).to_string(),
        FieldDataType::AutolapTrigger => AutolapTrigger::from(value).to_string(),
        FieldDataType::LapTrigger => LapTrigger::from(value).to_string(),
        FieldDataType::TimeMode => TimeMode::from(value).to_string(),
        FieldDataType::BacklightMode => BacklightMode::from(value).to_string(),
        FieldDataType::DateMode => DateMode::from(value).to_string(),
        FieldDataType::BacklightTimeout => BacklightTimeout::from(value).to_string(),
        FieldDataType::Event => Event::from(value).to_string(),
        FieldDataType::EventType => EventType::from(value).to_string(),
        FieldDataType::TimerTrigger => TimerTrigger::from(value).to_string(),
        FieldDataType::FitnessEquipmentState => FitnessEquipmentState::from(value).to_string(),
        FieldDataType::Tone => Tone::from(value).to_string(),
        FieldDataType::Autoscroll => Autoscroll::from(value).to_string(),
        FieldDataType::ActivityClass => ActivityClass::from(value).to_string(),
        FieldDataType::HrZoneCalc => HrZoneCalc::from(value).to_string(),
        FieldDataType::PwrZoneCalc => PwrZoneCalc::from(value).to_string(),
        FieldDataType::WktStepDuration => WktStepDuration::from(value).to_string(),
        FieldDataType::WktStepTarget => WktStepTarget::from(value).to_string(),
        FieldDataType::Goal => Goal::from(value).to_string(),
        FieldDataType::GoalRecurrence => GoalRecurrence::from(value).to_string(),
        FieldDataType::GoalSource => GoalSource::from(value).to_string(),
        FieldDataType::Schedule => Schedule::from(value).to_string(),
        FieldDataType::CoursePoint => CoursePoint::from(value).to_string(),
        FieldDataType::Manufacturer => Manufacturer::from(value).to_string(),
        FieldDataType::GarminProduct => GarminProduct::from(value).to_string(),
        FieldDataType::AntplusDeviceType => AntplusDeviceType::from(value).to_string(),
        FieldDataType::AntNetwork => AntNetwork::from(value).to_string(),
        FieldDataType::WorkoutCapabilities => WorkoutCapabilities::from(value).to_string(),
        FieldDataType::BatteryStatus => BatteryStatus::from(value).to_string(),
        FieldDataType::HrType => HrType::from(value).to_string(),
        FieldDataType::CourseCapabilities => CourseCapabilities::from(value).to_string(),
        FieldDataType::Weight => Weight::from(value).to_string(),
        FieldDataType::WorkoutHr => WorkoutHr::from(value).to_string(),
        FieldDataType::WorkoutPower => WorkoutPower::from(value).to_string(),
        FieldDataType::BpStatus => BpStatus::from(value).to_string(),
        FieldDataType::UserLocalId => UserLocalId::from(value).to_string(),
        FieldDataType::SwimStroke => SwimStroke::from(value).to_string(),
        FieldDataType::ActivityType => ActivityType::from(value).to_string(),
        FieldDataType::ActivitySubtype => ActivitySubtype::from(value).to_string(),
        FieldDataType::ActivityLevel => ActivityLevel::from(value).to_string(),
        FieldDataType::Side => Side::from(value).to_string(),
        FieldDataType::LeftRightBalance => LeftRightBalance::from(value).to_string(),
        FieldDataType::LeftRightBalance100 => LeftRightBalance100::from(value).to_string(),
        FieldDataType::LengthType => LengthType::from(value).to_string(),
        FieldDataType::DayOfWeek => DayOfWeek::from(value).to_string(),
        FieldDataType::ConnectivityCapabilities => {
            ConnectivityCapabilities::from(value).to_string()
        }
        FieldDataType::WeatherReport => WeatherReport::from(value).to_string(),
        FieldDataType::WeatherStatus => WeatherStatus::from(value).to_string(),
        FieldDataType::WeatherSeverity => WeatherSeverity::from(value).to_string(),
        FieldDataType::WeatherSevereType => WeatherSevereType::from(value).to_string(),
        FieldDataType::StrokeType => StrokeType::from(value).to_string(),
        FieldDataType::BodyLocation => BodyLocation::from(value).to_string(),
        FieldDataType::SegmentLapStatus => SegmentLapStatus::from(value).to_string(),
        FieldDataType::SegmentLeaderboardType => SegmentLeaderboardType::from(value).to_string(),
        FieldDataType::SegmentDeleteStatus => SegmentDeleteStatus::from(value).to_string(),
        FieldDataType::SegmentSelectionType => SegmentSelectionType::from(value).to_string(),
        FieldDataType::SourceType => SourceType::from(value).to_string(),
        FieldDataType::DisplayOrientation => DisplayOrientation::from(value).to_string(),
        FieldDataType::WorkoutEquipment => WorkoutEquipment::from(value).to_string(),
        FieldDataType::WatchfaceMode => WatchfaceMode::from(value).to_string(),
        FieldDataType::DigitalWatchfaceLayout => DigitalWatchfaceLayout::from(value).to_string(),
        FieldDataType::AnalogWatchfaceLayout => AnalogWatchfaceLayout::from(value).to_string(),
        FieldDataType::RiderPositionType => RiderPositionType::from(value).to_string(),
        FieldDataType::PowerPhaseType => PowerPhaseType::from(value).to_string(),
        FieldDataType::CameraEventType => CameraEventType::from(value).to_string(),
        FieldDataType::SensorType => SensorType::from(value).to_string(),
        FieldDataType::BikeLightNetworkConfigType => {
            BikeLightNetworkConfigType::from(value).to_string()
        }
        FieldDataType::CommTimeoutType => CommTimeoutType::from(value).to_string(),
        FieldDataType::CameraOrientationType => CameraOrientationType::from(value).to_string(),
        FieldDataType::AttitudeStage => AttitudeStage::from(value).to_string(),
        FieldDataType::AttitudeValidity => AttitudeValidity::from(value).to_string(),
        FieldDataType::AutoSyncFrequency => AutoSyncFrequency::from(value).to_string(),
        FieldDataType::ExdLayout => ExdLayout::from(value).to_string(),
        FieldDataType::ExdDisplayType => ExdDisplayType::from(value).to_string(),
        FieldDataType::ExdDataUnits => ExdDataUnits::from(value).to_string(),
        FieldDataType::ExdQualifiers => ExdQualifiers::from(value).to_string(),
        FieldDataType::ExdDescriptors => ExdDescriptors::from(value).to_string(),
        FieldDataType::AutoActivityDetect => AutoActivityDetect::from(value).to_string(),
        FieldDataType::SupportedExdScreenLayouts => {
            SupportedExdScreenLayouts::from(value).to_string()
        }
        FieldDataType::FitBaseType => FitBaseType::from(value).to_string(),
        FieldDataType::TurnType => TurnType::from(value).to_string(),
        FieldDataType::BikeLightBeamAngleMode => BikeLightBeamAngleMode::from(value).to_string(),
        FieldDataType::FitBaseUnit => FitBaseUnit::from(value).to_string(),
        FieldDataType::SetType => SetType::from(value).to_string(),
        FieldDataType::ExerciseCategory => ExerciseCategory::from(value).to_string(),
        FieldDataType::BenchPressExerciseName => BenchPressExerciseName::from(value).to_string(),
        FieldDataType::CalfRaiseExerciseName => CalfRaiseExerciseName::from(value).to_string(),
        FieldDataType::CardioExerciseName => CardioExerciseName::from(value).to_string(),
        FieldDataType::CarryExerciseName => CarryExerciseName::from(value).to_string(),
        FieldDataType::ChopExerciseName => ChopExerciseName::from(value).to_string(),
        FieldDataType::CoreExerciseName => CoreExerciseName::from(value).to_string(),
        FieldDataType::CrunchExerciseName => CrunchExerciseName::from(value).to_string(),
        FieldDataType::CurlExerciseName => CurlExerciseName::from(value).to_string(),
        FieldDataType::DeadliftExerciseName => DeadliftExerciseName::from(value).to_string(),
        FieldDataType::FlyeExerciseName => FlyeExerciseName::from(value).to_string(),
        FieldDataType::HipRaiseExerciseName => HipRaiseExerciseName::from(value).to_string(),
        FieldDataType::HipStabilityExerciseName => {
            HipStabilityExerciseName::from(value).to_string()
        }
        FieldDataType::HipSwingExerciseName => HipSwingExerciseName::from(value).to_string(),
        FieldDataType::HyperextensionExerciseName => {
            HyperextensionExerciseName::from(value).to_string()
        }
        FieldDataType::LateralRaiseExerciseName => {
            LateralRaiseExerciseName::from(value).to_string()
        }
        FieldDataType::LegCurlExerciseName => LegCurlExerciseName::from(value).to_string(),
        FieldDataType::LegRaiseExerciseName => LegRaiseExerciseName::from(value).to_string(),
        FieldDataType::LungeExerciseName => LungeExerciseName::from(value).to_string(),
        FieldDataType::OlympicLiftExerciseName => OlympicLiftExerciseName::from(value).to_string(),
        FieldDataType::PlankExerciseName => PlankExerciseName::from(value).to_string(),
        FieldDataType::PlyoExerciseName => PlyoExerciseName::from(value).to_string(),
        FieldDataType::PullUpExerciseName => PullUpExerciseName::from(value).to_string(),
        FieldDataType::PushUpExerciseName => PushUpExerciseName::from(value).to_string(),
        FieldDataType::RowExerciseName => RowExerciseName::from(value).to_string(),
        FieldDataType::ShoulderPressExerciseName => {
            ShoulderPressExerciseName::from(value).to_string()
        }
        FieldDataType::ShoulderStabilityExerciseName => {
            ShoulderStabilityExerciseName::from(value).to_string()
        }
        FieldDataType::ShrugExerciseName => ShrugExerciseName::from(value).to_string(),
        FieldDataType::SitUpExerciseName => SitUpExerciseName::from(value).to_string(),
        FieldDataType::SquatExerciseName => SquatExerciseName::from(value).to_string(),
        FieldDataType::TotalBodyExerciseName => TotalBodyExerciseName::from(value).to_string(),
        FieldDataType::TricepsExtensionExerciseName => {
            TricepsExtensionExerciseName::from(value).to_string()
        }
        FieldDataType::WarmUpExerciseName => WarmUpExerciseName::from(value).to_string(),
        FieldDataType::RunExerciseName => RunExerciseName::from(value).to_string(),
        FieldDataType::WaterType => WaterType::from(value).to_string(),
        FieldDataType::TissueModelType => TissueModelType::from(value).to_string(),
        FieldDataType::DiveGasStatus => DiveGasStatus::from(value).to_string(),
        FieldDataType::DiveAlarmType => DiveAlarmType::from(value).to_string(),
        FieldDataType::DiveBacklightMode => DiveBacklightMode::from(value).to_string(),
        FieldDataType::FaveroProduct => FaveroProduct::from(value).to_string(),
        FieldDataType::ClimbProEvent => ClimbProEvent::from(value).to_string(),
        _ => format!("Undefined{}", value),
    }
}
