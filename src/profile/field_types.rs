/// Auto generated profile from FIT SDK Release: XXX

#[derive(Clone, Copy, Debug)]
pub enum File {
    Device,          // Read only, single file. Must be in root directory.
    Settings,        // Read/write, single file. Directory=Settings
    Sport,           // Read/write, multiple files, file number = sport type. Directory=Sports
    Activity,        // Read/erase, multiple files. Directory=Activities
    Workout,         // Read/write/erase, multiple files. Directory=Workouts
    Course,          // Read/write/erase, multiple files. Directory=Courses
    Schedules,       // Read/write, single file. Directory=Schedules
    Weight, // Read only, single file. Circular buffer. All message definitions at start of file. Directory=Weight
    Totals, // Read only, single file. Directory=Totals
    Goals,  // Read/write, single file. Directory=Goals
    BloodPressure, // Read only. Directory=Blood Pressure
    MonitoringA, // Read only. Directory=Monitoring. File number=sub type.
    ActivitySummary, // Read/erase, multiple files. Directory=Activities
    MonitoringDaily,
    MonitoringB,      // Read only. Directory=Monitoring. File number=identifier
    Segment,          // Read/write/erase. Multiple Files.  Directory=Segments
    SegmentList,      // Read/write/erase. Single File.  Directory=Segments
    ExdConfiguration, // Read/write/erase. Single File. Directory=Settings
    MfgRangeMin,      // 0xF7 - 0xFE reserved for manufacturer specific file types
    MfgRangeMax,      // 0xF7 - 0xFE reserved for manufacturer specific file types
    UnknownVariant(u8),
}
impl File {
    pub fn from_u8(value: u8) -> File {
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
    pub fn from_i64(value: i64) -> File {
        File::from_u8(value as u8)
    }
    pub fn as_u8(&self) -> u8 {
        match &self {
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
            File::UnknownVariant(value) => *value,
        }
    }
    pub fn to_string(&self) -> String {
        match &self {
            File::Device => "device".to_string(),
            File::Settings => "settings".to_string(),
            File::Sport => "sport".to_string(),
            File::Activity => "activity".to_string(),
            File::Workout => "workout".to_string(),
            File::Course => "course".to_string(),
            File::Schedules => "schedules".to_string(),
            File::Weight => "weight".to_string(),
            File::Totals => "totals".to_string(),
            File::Goals => "goals".to_string(),
            File::BloodPressure => "blood_pressure".to_string(),
            File::MonitoringA => "monitoring_a".to_string(),
            File::ActivitySummary => "activity_summary".to_string(),
            File::MonitoringDaily => "monitoring_daily".to_string(),
            File::MonitoringB => "monitoring_b".to_string(),
            File::Segment => "segment".to_string(),
            File::SegmentList => "segment_list".to_string(),
            File::ExdConfiguration => "exd_configuration".to_string(),
            File::MfgRangeMin => "mfg_range_min".to_string(),
            File::MfgRangeMax => "mfg_range_max".to_string(),
            File::UnknownVariant(value) => format!("unknown_variant_{}", *value),
        }
    }
}

#[derive(Clone, Copy, Debug)]
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
    MfgRangeMin, // 0xFF00 - 0xFFFE reserved for manufacturer specific messages
    MfgRangeMax, // 0xFF00 - 0xFFFE reserved for manufacturer specific messages
    UnknownVariant(u16),
}
impl MesgNum {
    pub fn from_u16(value: u16) -> MesgNum {
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
    pub fn from_i64(value: i64) -> MesgNum {
        MesgNum::from_u16(value as u16)
    }
    pub fn as_u16(&self) -> u16 {
        match &self {
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
            MesgNum::UnknownVariant(value) => *value,
        }
    }
    pub fn to_string(&self) -> String {
        match &self {
            MesgNum::FileId => "file_id".to_string(),
            MesgNum::Capabilities => "capabilities".to_string(),
            MesgNum::DeviceSettings => "device_settings".to_string(),
            MesgNum::UserProfile => "user_profile".to_string(),
            MesgNum::HrmProfile => "hrm_profile".to_string(),
            MesgNum::SdmProfile => "sdm_profile".to_string(),
            MesgNum::BikeProfile => "bike_profile".to_string(),
            MesgNum::ZonesTarget => "zones_target".to_string(),
            MesgNum::HrZone => "hr_zone".to_string(),
            MesgNum::PowerZone => "power_zone".to_string(),
            MesgNum::MetZone => "met_zone".to_string(),
            MesgNum::Sport => "sport".to_string(),
            MesgNum::Goal => "goal".to_string(),
            MesgNum::Session => "session".to_string(),
            MesgNum::Lap => "lap".to_string(),
            MesgNum::Record => "record".to_string(),
            MesgNum::Event => "event".to_string(),
            MesgNum::DeviceInfo => "device_info".to_string(),
            MesgNum::Workout => "workout".to_string(),
            MesgNum::WorkoutStep => "workout_step".to_string(),
            MesgNum::Schedule => "schedule".to_string(),
            MesgNum::WeightScale => "weight_scale".to_string(),
            MesgNum::Course => "course".to_string(),
            MesgNum::CoursePoint => "course_point".to_string(),
            MesgNum::Totals => "totals".to_string(),
            MesgNum::Activity => "activity".to_string(),
            MesgNum::Software => "software".to_string(),
            MesgNum::FileCapabilities => "file_capabilities".to_string(),
            MesgNum::MesgCapabilities => "mesg_capabilities".to_string(),
            MesgNum::FieldCapabilities => "field_capabilities".to_string(),
            MesgNum::FileCreator => "file_creator".to_string(),
            MesgNum::BloodPressure => "blood_pressure".to_string(),
            MesgNum::SpeedZone => "speed_zone".to_string(),
            MesgNum::Monitoring => "monitoring".to_string(),
            MesgNum::TrainingFile => "training_file".to_string(),
            MesgNum::Hrv => "hrv".to_string(),
            MesgNum::AntRx => "ant_rx".to_string(),
            MesgNum::AntTx => "ant_tx".to_string(),
            MesgNum::AntChannelId => "ant_channel_id".to_string(),
            MesgNum::Length => "length".to_string(),
            MesgNum::MonitoringInfo => "monitoring_info".to_string(),
            MesgNum::Pad => "pad".to_string(),
            MesgNum::SlaveDevice => "slave_device".to_string(),
            MesgNum::Connectivity => "connectivity".to_string(),
            MesgNum::WeatherConditions => "weather_conditions".to_string(),
            MesgNum::WeatherAlert => "weather_alert".to_string(),
            MesgNum::CadenceZone => "cadence_zone".to_string(),
            MesgNum::Hr => "hr".to_string(),
            MesgNum::SegmentLap => "segment_lap".to_string(),
            MesgNum::MemoGlob => "memo_glob".to_string(),
            MesgNum::SegmentId => "segment_id".to_string(),
            MesgNum::SegmentLeaderboardEntry => "segment_leaderboard_entry".to_string(),
            MesgNum::SegmentPoint => "segment_point".to_string(),
            MesgNum::SegmentFile => "segment_file".to_string(),
            MesgNum::WorkoutSession => "workout_session".to_string(),
            MesgNum::WatchfaceSettings => "watchface_settings".to_string(),
            MesgNum::GpsMetadata => "gps_metadata".to_string(),
            MesgNum::CameraEvent => "camera_event".to_string(),
            MesgNum::TimestampCorrelation => "timestamp_correlation".to_string(),
            MesgNum::GyroscopeData => "gyroscope_data".to_string(),
            MesgNum::AccelerometerData => "accelerometer_data".to_string(),
            MesgNum::ThreeDSensorCalibration => "three_d_sensor_calibration".to_string(),
            MesgNum::VideoFrame => "video_frame".to_string(),
            MesgNum::ObdiiData => "obdii_data".to_string(),
            MesgNum::NmeaSentence => "nmea_sentence".to_string(),
            MesgNum::AviationAttitude => "aviation_attitude".to_string(),
            MesgNum::Video => "video".to_string(),
            MesgNum::VideoTitle => "video_title".to_string(),
            MesgNum::VideoDescription => "video_description".to_string(),
            MesgNum::VideoClip => "video_clip".to_string(),
            MesgNum::OhrSettings => "ohr_settings".to_string(),
            MesgNum::ExdScreenConfiguration => "exd_screen_configuration".to_string(),
            MesgNum::ExdDataFieldConfiguration => "exd_data_field_configuration".to_string(),
            MesgNum::ExdDataConceptConfiguration => "exd_data_concept_configuration".to_string(),
            MesgNum::FieldDescription => "field_description".to_string(),
            MesgNum::DeveloperDataId => "developer_data_id".to_string(),
            MesgNum::MagnetometerData => "magnetometer_data".to_string(),
            MesgNum::BarometerData => "barometer_data".to_string(),
            MesgNum::OneDSensorCalibration => "one_d_sensor_calibration".to_string(),
            MesgNum::Set => "set".to_string(),
            MesgNum::StressLevel => "stress_level".to_string(),
            MesgNum::DiveSettings => "dive_settings".to_string(),
            MesgNum::DiveGas => "dive_gas".to_string(),
            MesgNum::DiveAlarm => "dive_alarm".to_string(),
            MesgNum::ExerciseTitle => "exercise_title".to_string(),
            MesgNum::DiveSummary => "dive_summary".to_string(),
            MesgNum::Jump => "jump".to_string(),
            MesgNum::ClimbPro => "climb_pro".to_string(),
            MesgNum::MfgRangeMin => "mfg_range_min".to_string(),
            MesgNum::MfgRangeMax => "mfg_range_max".to_string(),
            MesgNum::UnknownVariant(value) => format!("unknown_variant_{}", *value),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Checksum {
    Clear, // Allows clear of checksum for flash memory where can only write 1 to 0 without erasing sector.
    Ok, // Set to mark checksum as valid if computes to invalid values 0 or 0xFF.  Checksum can also be set to ok to save encoding computation time.
    UnknownVariant(u8),
}
impl Checksum {
    pub fn from_u8(value: u8) -> Checksum {
        match value {
            0 => Checksum::Clear,
            1 => Checksum::Ok,
            _ => Checksum::UnknownVariant(value),
        }
    }
    pub fn from_i64(value: i64) -> Checksum {
        Checksum::from_u8(value as u8)
    }
    pub fn as_u8(&self) -> u8 {
        match &self {
            Checksum::Clear => 0,
            Checksum::Ok => 1,
            Checksum::UnknownVariant(value) => *value,
        }
    }
    pub fn to_string(&self) -> String {
        match &self {
            Checksum::Clear => "clear".to_string(),
            Checksum::Ok => "ok".to_string(),
            Checksum::UnknownVariant(value) => format!("unknown_variant_{}", *value),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum FileFlags {
    Read,
    Write,
    Erase,
    UnknownVariant(u8),
}
impl FileFlags {
    pub fn from_u8(value: u8) -> FileFlags {
        match value {
            2 => FileFlags::Read,
            4 => FileFlags::Write,
            8 => FileFlags::Erase,
            _ => FileFlags::UnknownVariant(value),
        }
    }
    pub fn from_i64(value: i64) -> FileFlags {
        FileFlags::from_u8(value as u8)
    }
    pub fn as_u8(&self) -> u8 {
        match &self {
            FileFlags::Read => 2,
            FileFlags::Write => 4,
            FileFlags::Erase => 8,
            FileFlags::UnknownVariant(value) => *value,
        }
    }
    pub fn to_string(&self) -> String {
        match &self {
            FileFlags::Read => "read".to_string(),
            FileFlags::Write => "write".to_string(),
            FileFlags::Erase => "erase".to_string(),
            FileFlags::UnknownVariant(value) => format!("unknown_variant_{}", *value),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum MesgCount {
    NumPerFile,
    MaxPerFile,
    MaxPerFileType,
    UnknownVariant(u8),
}
impl MesgCount {
    pub fn from_u8(value: u8) -> MesgCount {
        match value {
            0 => MesgCount::NumPerFile,
            1 => MesgCount::MaxPerFile,
            2 => MesgCount::MaxPerFileType,
            _ => MesgCount::UnknownVariant(value),
        }
    }
    pub fn from_i64(value: i64) -> MesgCount {
        MesgCount::from_u8(value as u8)
    }
    pub fn as_u8(&self) -> u8 {
        match &self {
            MesgCount::NumPerFile => 0,
            MesgCount::MaxPerFile => 1,
            MesgCount::MaxPerFileType => 2,
            MesgCount::UnknownVariant(value) => *value,
        }
    }
    pub fn to_string(&self) -> String {
        match &self {
            MesgCount::NumPerFile => "num_per_file".to_string(),
            MesgCount::MaxPerFile => "max_per_file".to_string(),
            MesgCount::MaxPerFileType => "max_per_file_type".to_string(),
            MesgCount::UnknownVariant(value) => format!("unknown_variant_{}", *value),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum DateTime {
    Min, // if date_time is < 0x10000000 then it is system time (seconds from device power on)
    UnknownVariant(u32),
}
impl DateTime {
    pub fn from_u32(value: u32) -> DateTime {
        match value {
            268435456 => DateTime::Min,
            _ => DateTime::UnknownVariant(value),
        }
    }
    pub fn from_i64(value: i64) -> DateTime {
        DateTime::from_u32(value as u32)
    }
    pub fn as_u32(&self) -> u32 {
        match &self {
            DateTime::Min => 268435456,
            DateTime::UnknownVariant(value) => *value,
        }
    }
    pub fn to_string(&self) -> String {
        match &self {
            DateTime::Min => "min".to_string(),
            DateTime::UnknownVariant(value) => format!("unknown_variant_{}", *value),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum LocalDateTime {
    Min, // if date_time is < 0x10000000 then it is system time (seconds from device power on)
    UnknownVariant(u32),
}
impl LocalDateTime {
    pub fn from_u32(value: u32) -> LocalDateTime {
        match value {
            268435456 => LocalDateTime::Min,
            _ => LocalDateTime::UnknownVariant(value),
        }
    }
    pub fn from_i64(value: i64) -> LocalDateTime {
        LocalDateTime::from_u32(value as u32)
    }
    pub fn as_u32(&self) -> u32 {
        match &self {
            LocalDateTime::Min => 268435456,
            LocalDateTime::UnknownVariant(value) => *value,
        }
    }
    pub fn to_string(&self) -> String {
        match &self {
            LocalDateTime::Min => "min".to_string(),
            LocalDateTime::UnknownVariant(value) => format!("unknown_variant_{}", *value),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum MessageIndex {
    Mask,     // index
    Reserved, // reserved (default 0)
    Selected, // message is selected if set
    UnknownVariant(u16),
}
impl MessageIndex {
    pub fn from_u16(value: u16) -> MessageIndex {
        match value {
            4095 => MessageIndex::Mask,
            28672 => MessageIndex::Reserved,
            32768 => MessageIndex::Selected,
            _ => MessageIndex::UnknownVariant(value),
        }
    }
    pub fn from_i64(value: i64) -> MessageIndex {
        MessageIndex::from_u16(value as u16)
    }
    pub fn as_u16(&self) -> u16 {
        match &self {
            MessageIndex::Mask => 4095,
            MessageIndex::Reserved => 28672,
            MessageIndex::Selected => 32768,
            MessageIndex::UnknownVariant(value) => *value,
        }
    }
    pub fn to_string(&self) -> String {
        match &self {
            MessageIndex::Mask => "mask".to_string(),
            MessageIndex::Reserved => "reserved".to_string(),
            MessageIndex::Selected => "selected".to_string(),
            MessageIndex::UnknownVariant(value) => format!("unknown_variant_{}", *value),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum DeviceIndex {
    Creator, // Creator of the file is always device index 0.
    UnknownVariant(u8),
}
impl DeviceIndex {
    pub fn from_u8(value: u8) -> DeviceIndex {
        match value {
            0 => DeviceIndex::Creator,
            _ => DeviceIndex::UnknownVariant(value),
        }
    }
    pub fn from_i64(value: i64) -> DeviceIndex {
        DeviceIndex::from_u8(value as u8)
    }
    pub fn as_u8(&self) -> u8 {
        match &self {
            DeviceIndex::Creator => 0,
            DeviceIndex::UnknownVariant(value) => *value,
        }
    }
    pub fn to_string(&self) -> String {
        match &self {
            DeviceIndex::Creator => "creator".to_string(),
            DeviceIndex::UnknownVariant(value) => format!("unknown_variant_{}", *value),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Gender {
    Female,
    Male,
    UnknownVariant(u8),
}
impl Gender {
    pub fn from_u8(value: u8) -> Gender {
        match value {
            0 => Gender::Female,
            1 => Gender::Male,
            _ => Gender::UnknownVariant(value),
        }
    }
    pub fn from_i64(value: i64) -> Gender {
        Gender::from_u8(value as u8)
    }
    pub fn as_u8(&self) -> u8 {
        match &self {
            Gender::Female => 0,
            Gender::Male => 1,
            Gender::UnknownVariant(value) => *value,
        }
    }
    pub fn to_string(&self) -> String {
        match &self {
            Gender::Female => "female".to_string(),
            Gender::Male => "male".to_string(),
            Gender::UnknownVariant(value) => format!("unknown_variant_{}", *value),
        }
    }
}

#[derive(Clone, Copy, Debug)]
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
    pub fn from_u8(value: u8) -> Language {
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
    pub fn from_i64(value: i64) -> Language {
        Language::from_u8(value as u8)
    }
    pub fn as_u8(&self) -> u8 {
        match &self {
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
            Language::UnknownVariant(value) => *value,
        }
    }
    pub fn to_string(&self) -> String {
        match &self {
            Language::English => "english".to_string(),
            Language::French => "french".to_string(),
            Language::Italian => "italian".to_string(),
            Language::German => "german".to_string(),
            Language::Spanish => "spanish".to_string(),
            Language::Croatian => "croatian".to_string(),
            Language::Czech => "czech".to_string(),
            Language::Danish => "danish".to_string(),
            Language::Dutch => "dutch".to_string(),
            Language::Finnish => "finnish".to_string(),
            Language::Greek => "greek".to_string(),
            Language::Hungarian => "hungarian".to_string(),
            Language::Norwegian => "norwegian".to_string(),
            Language::Polish => "polish".to_string(),
            Language::Portuguese => "portuguese".to_string(),
            Language::Slovakian => "slovakian".to_string(),
            Language::Slovenian => "slovenian".to_string(),
            Language::Swedish => "swedish".to_string(),
            Language::Russian => "russian".to_string(),
            Language::Turkish => "turkish".to_string(),
            Language::Latvian => "latvian".to_string(),
            Language::Ukrainian => "ukrainian".to_string(),
            Language::Arabic => "arabic".to_string(),
            Language::Farsi => "farsi".to_string(),
            Language::Bulgarian => "bulgarian".to_string(),
            Language::Romanian => "romanian".to_string(),
            Language::Chinese => "chinese".to_string(),
            Language::Japanese => "japanese".to_string(),
            Language::Korean => "korean".to_string(),
            Language::Taiwanese => "taiwanese".to_string(),
            Language::Thai => "thai".to_string(),
            Language::Hebrew => "hebrew".to_string(),
            Language::BrazilianPortuguese => "brazilian_portuguese".to_string(),
            Language::Indonesian => "indonesian".to_string(),
            Language::Malaysian => "malaysian".to_string(),
            Language::Vietnamese => "vietnamese".to_string(),
            Language::Burmese => "burmese".to_string(),
            Language::Mongolian => "mongolian".to_string(),
            Language::Custom => "custom".to_string(),
            Language::UnknownVariant(value) => format!("unknown_variant_{}", *value),
        }
    }
}

#[derive(Clone, Copy, Debug)]
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
    pub fn from_u8(value: u8) -> LanguageBits0 {
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
    pub fn from_i64(value: i64) -> LanguageBits0 {
        LanguageBits0::from_u8(value as u8)
    }
    pub fn as_u8(&self) -> u8 {
        match &self {
            LanguageBits0::English => 1,
            LanguageBits0::French => 2,
            LanguageBits0::Italian => 4,
            LanguageBits0::German => 8,
            LanguageBits0::Spanish => 16,
            LanguageBits0::Croatian => 32,
            LanguageBits0::Czech => 64,
            LanguageBits0::Danish => 128,
            LanguageBits0::UnknownVariant(value) => *value,
        }
    }
    pub fn to_string(&self) -> String {
        match &self {
            LanguageBits0::English => "english".to_string(),
            LanguageBits0::French => "french".to_string(),
            LanguageBits0::Italian => "italian".to_string(),
            LanguageBits0::German => "german".to_string(),
            LanguageBits0::Spanish => "spanish".to_string(),
            LanguageBits0::Croatian => "croatian".to_string(),
            LanguageBits0::Czech => "czech".to_string(),
            LanguageBits0::Danish => "danish".to_string(),
            LanguageBits0::UnknownVariant(value) => format!("unknown_variant_{}", *value),
        }
    }
}

#[derive(Clone, Copy, Debug)]
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
    pub fn from_u8(value: u8) -> LanguageBits1 {
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
    pub fn from_i64(value: i64) -> LanguageBits1 {
        LanguageBits1::from_u8(value as u8)
    }
    pub fn as_u8(&self) -> u8 {
        match &self {
            LanguageBits1::Dutch => 1,
            LanguageBits1::Finnish => 2,
            LanguageBits1::Greek => 4,
            LanguageBits1::Hungarian => 8,
            LanguageBits1::Norwegian => 16,
            LanguageBits1::Polish => 32,
            LanguageBits1::Portuguese => 64,
            LanguageBits1::Slovakian => 128,
            LanguageBits1::UnknownVariant(value) => *value,
        }
    }
    pub fn to_string(&self) -> String {
        match &self {
            LanguageBits1::Dutch => "dutch".to_string(),
            LanguageBits1::Finnish => "finnish".to_string(),
            LanguageBits1::Greek => "greek".to_string(),
            LanguageBits1::Hungarian => "hungarian".to_string(),
            LanguageBits1::Norwegian => "norwegian".to_string(),
            LanguageBits1::Polish => "polish".to_string(),
            LanguageBits1::Portuguese => "portuguese".to_string(),
            LanguageBits1::Slovakian => "slovakian".to_string(),
            LanguageBits1::UnknownVariant(value) => format!("unknown_variant_{}", *value),
        }
    }
}

#[derive(Clone, Copy, Debug)]
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
    pub fn from_u8(value: u8) -> LanguageBits2 {
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
    pub fn from_i64(value: i64) -> LanguageBits2 {
        LanguageBits2::from_u8(value as u8)
    }
    pub fn as_u8(&self) -> u8 {
        match &self {
            LanguageBits2::Slovenian => 1,
            LanguageBits2::Swedish => 2,
            LanguageBits2::Russian => 4,
            LanguageBits2::Turkish => 8,
            LanguageBits2::Latvian => 16,
            LanguageBits2::Ukrainian => 32,
            LanguageBits2::Arabic => 64,
            LanguageBits2::Farsi => 128,
            LanguageBits2::UnknownVariant(value) => *value,
        }
    }
    pub fn to_string(&self) -> String {
        match &self {
            LanguageBits2::Slovenian => "slovenian".to_string(),
            LanguageBits2::Swedish => "swedish".to_string(),
            LanguageBits2::Russian => "russian".to_string(),
            LanguageBits2::Turkish => "turkish".to_string(),
            LanguageBits2::Latvian => "latvian".to_string(),
            LanguageBits2::Ukrainian => "ukrainian".to_string(),
            LanguageBits2::Arabic => "arabic".to_string(),
            LanguageBits2::Farsi => "farsi".to_string(),
            LanguageBits2::UnknownVariant(value) => format!("unknown_variant_{}", *value),
        }
    }
}

#[derive(Clone, Copy, Debug)]
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
    pub fn from_u8(value: u8) -> LanguageBits3 {
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
    pub fn from_i64(value: i64) -> LanguageBits3 {
        LanguageBits3::from_u8(value as u8)
    }
    pub fn as_u8(&self) -> u8 {
        match &self {
            LanguageBits3::Bulgarian => 1,
            LanguageBits3::Romanian => 2,
            LanguageBits3::Chinese => 4,
            LanguageBits3::Japanese => 8,
            LanguageBits3::Korean => 16,
            LanguageBits3::Taiwanese => 32,
            LanguageBits3::Thai => 64,
            LanguageBits3::Hebrew => 128,
            LanguageBits3::UnknownVariant(value) => *value,
        }
    }
    pub fn to_string(&self) -> String {
        match &self {
            LanguageBits3::Bulgarian => "bulgarian".to_string(),
            LanguageBits3::Romanian => "romanian".to_string(),
            LanguageBits3::Chinese => "chinese".to_string(),
            LanguageBits3::Japanese => "japanese".to_string(),
            LanguageBits3::Korean => "korean".to_string(),
            LanguageBits3::Taiwanese => "taiwanese".to_string(),
            LanguageBits3::Thai => "thai".to_string(),
            LanguageBits3::Hebrew => "hebrew".to_string(),
            LanguageBits3::UnknownVariant(value) => format!("unknown_variant_{}", *value),
        }
    }
}

#[derive(Clone, Copy, Debug)]
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
    pub fn from_u8(value: u8) -> LanguageBits4 {
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
    pub fn from_i64(value: i64) -> LanguageBits4 {
        LanguageBits4::from_u8(value as u8)
    }
    pub fn as_u8(&self) -> u8 {
        match &self {
            LanguageBits4::BrazilianPortuguese => 1,
            LanguageBits4::Indonesian => 2,
            LanguageBits4::Malaysian => 4,
            LanguageBits4::Vietnamese => 8,
            LanguageBits4::Burmese => 16,
            LanguageBits4::Mongolian => 32,
            LanguageBits4::UnknownVariant(value) => *value,
        }
    }
    pub fn to_string(&self) -> String {
        match &self {
            LanguageBits4::BrazilianPortuguese => "brazilian_portuguese".to_string(),
            LanguageBits4::Indonesian => "indonesian".to_string(),
            LanguageBits4::Malaysian => "malaysian".to_string(),
            LanguageBits4::Vietnamese => "vietnamese".to_string(),
            LanguageBits4::Burmese => "burmese".to_string(),
            LanguageBits4::Mongolian => "mongolian".to_string(),
            LanguageBits4::UnknownVariant(value) => format!("unknown_variant_{}", *value),
        }
    }
}

#[derive(Clone, Copy, Debug)]
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
    pub fn from_u8(value: u8) -> TimeZone {
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
    pub fn from_i64(value: i64) -> TimeZone {
        TimeZone::from_u8(value as u8)
    }
    pub fn as_u8(&self) -> u8 {
        match &self {
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
            TimeZone::UnknownVariant(value) => *value,
        }
    }
    pub fn to_string(&self) -> String {
        match &self {
            TimeZone::Almaty => "almaty".to_string(),
            TimeZone::Bangkok => "bangkok".to_string(),
            TimeZone::Bombay => "bombay".to_string(),
            TimeZone::Brasilia => "brasilia".to_string(),
            TimeZone::Cairo => "cairo".to_string(),
            TimeZone::CapeVerdeIs => "cape_verde_is".to_string(),
            TimeZone::Darwin => "darwin".to_string(),
            TimeZone::Eniwetok => "eniwetok".to_string(),
            TimeZone::Fiji => "fiji".to_string(),
            TimeZone::HongKong => "hong_kong".to_string(),
            TimeZone::Islamabad => "islamabad".to_string(),
            TimeZone::Kabul => "kabul".to_string(),
            TimeZone::Magadan => "magadan".to_string(),
            TimeZone::MidAtlantic => "mid_atlantic".to_string(),
            TimeZone::Moscow => "moscow".to_string(),
            TimeZone::Muscat => "muscat".to_string(),
            TimeZone::Newfoundland => "newfoundland".to_string(),
            TimeZone::Samoa => "samoa".to_string(),
            TimeZone::Sydney => "sydney".to_string(),
            TimeZone::Tehran => "tehran".to_string(),
            TimeZone::Tokyo => "tokyo".to_string(),
            TimeZone::UsAlaska => "us_alaska".to_string(),
            TimeZone::UsAtlantic => "us_atlantic".to_string(),
            TimeZone::UsCentral => "us_central".to_string(),
            TimeZone::UsEastern => "us_eastern".to_string(),
            TimeZone::UsHawaii => "us_hawaii".to_string(),
            TimeZone::UsMountain => "us_mountain".to_string(),
            TimeZone::UsPacific => "us_pacific".to_string(),
            TimeZone::Other => "other".to_string(),
            TimeZone::Auckland => "auckland".to_string(),
            TimeZone::Kathmandu => "kathmandu".to_string(),
            TimeZone::EuropeWesternWet => "europe_western_wet".to_string(),
            TimeZone::EuropeCentralCet => "europe_central_cet".to_string(),
            TimeZone::EuropeEasternEet => "europe_eastern_eet".to_string(),
            TimeZone::Jakarta => "jakarta".to_string(),
            TimeZone::Perth => "perth".to_string(),
            TimeZone::Adelaide => "adelaide".to_string(),
            TimeZone::Brisbane => "brisbane".to_string(),
            TimeZone::Tasmania => "tasmania".to_string(),
            TimeZone::Iceland => "iceland".to_string(),
            TimeZone::Amsterdam => "amsterdam".to_string(),
            TimeZone::Athens => "athens".to_string(),
            TimeZone::Barcelona => "barcelona".to_string(),
            TimeZone::Berlin => "berlin".to_string(),
            TimeZone::Brussels => "brussels".to_string(),
            TimeZone::Budapest => "budapest".to_string(),
            TimeZone::Copenhagen => "copenhagen".to_string(),
            TimeZone::Dublin => "dublin".to_string(),
            TimeZone::Helsinki => "helsinki".to_string(),
            TimeZone::Lisbon => "lisbon".to_string(),
            TimeZone::London => "london".to_string(),
            TimeZone::Madrid => "madrid".to_string(),
            TimeZone::Munich => "munich".to_string(),
            TimeZone::Oslo => "oslo".to_string(),
            TimeZone::Paris => "paris".to_string(),
            TimeZone::Prague => "prague".to_string(),
            TimeZone::Reykjavik => "reykjavik".to_string(),
            TimeZone::Rome => "rome".to_string(),
            TimeZone::Stockholm => "stockholm".to_string(),
            TimeZone::Vienna => "vienna".to_string(),
            TimeZone::Warsaw => "warsaw".to_string(),
            TimeZone::Zurich => "zurich".to_string(),
            TimeZone::Quebec => "quebec".to_string(),
            TimeZone::Ontario => "ontario".to_string(),
            TimeZone::Manitoba => "manitoba".to_string(),
            TimeZone::Saskatchewan => "saskatchewan".to_string(),
            TimeZone::Alberta => "alberta".to_string(),
            TimeZone::BritishColumbia => "british_columbia".to_string(),
            TimeZone::Boise => "boise".to_string(),
            TimeZone::Boston => "boston".to_string(),
            TimeZone::Chicago => "chicago".to_string(),
            TimeZone::Dallas => "dallas".to_string(),
            TimeZone::Denver => "denver".to_string(),
            TimeZone::KansasCity => "kansas_city".to_string(),
            TimeZone::LasVegas => "las_vegas".to_string(),
            TimeZone::LosAngeles => "los_angeles".to_string(),
            TimeZone::Miami => "miami".to_string(),
            TimeZone::Minneapolis => "minneapolis".to_string(),
            TimeZone::NewYork => "new_york".to_string(),
            TimeZone::NewOrleans => "new_orleans".to_string(),
            TimeZone::Phoenix => "phoenix".to_string(),
            TimeZone::SantaFe => "santa_fe".to_string(),
            TimeZone::Seattle => "seattle".to_string(),
            TimeZone::WashingtonDc => "washington_dc".to_string(),
            TimeZone::UsArizona => "us_arizona".to_string(),
            TimeZone::Chita => "chita".to_string(),
            TimeZone::Ekaterinburg => "ekaterinburg".to_string(),
            TimeZone::Irkutsk => "irkutsk".to_string(),
            TimeZone::Kaliningrad => "kaliningrad".to_string(),
            TimeZone::Krasnoyarsk => "krasnoyarsk".to_string(),
            TimeZone::Novosibirsk => "novosibirsk".to_string(),
            TimeZone::PetropavlovskKamchatskiy => "petropavlovsk_kamchatskiy".to_string(),
            TimeZone::Samara => "samara".to_string(),
            TimeZone::Vladivostok => "vladivostok".to_string(),
            TimeZone::MexicoCentral => "mexico_central".to_string(),
            TimeZone::MexicoMountain => "mexico_mountain".to_string(),
            TimeZone::MexicoPacific => "mexico_pacific".to_string(),
            TimeZone::CapeTown => "cape_town".to_string(),
            TimeZone::Winkhoek => "winkhoek".to_string(),
            TimeZone::Lagos => "lagos".to_string(),
            TimeZone::Riyahd => "riyahd".to_string(),
            TimeZone::Venezuela => "venezuela".to_string(),
            TimeZone::AustraliaLh => "australia_lh".to_string(),
            TimeZone::Santiago => "santiago".to_string(),
            TimeZone::Manual => "manual".to_string(),
            TimeZone::Automatic => "automatic".to_string(),
            TimeZone::UnknownVariant(value) => format!("unknown_variant_{}", *value),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum DisplayMeasure {
    Metric,
    Statute,
    Nautical,
    UnknownVariant(u8),
}
impl DisplayMeasure {
    pub fn from_u8(value: u8) -> DisplayMeasure {
        match value {
            0 => DisplayMeasure::Metric,
            1 => DisplayMeasure::Statute,
            2 => DisplayMeasure::Nautical,
            _ => DisplayMeasure::UnknownVariant(value),
        }
    }
    pub fn from_i64(value: i64) -> DisplayMeasure {
        DisplayMeasure::from_u8(value as u8)
    }
    pub fn as_u8(&self) -> u8 {
        match &self {
            DisplayMeasure::Metric => 0,
            DisplayMeasure::Statute => 1,
            DisplayMeasure::Nautical => 2,
            DisplayMeasure::UnknownVariant(value) => *value,
        }
    }
    pub fn to_string(&self) -> String {
        match &self {
            DisplayMeasure::Metric => "metric".to_string(),
            DisplayMeasure::Statute => "statute".to_string(),
            DisplayMeasure::Nautical => "nautical".to_string(),
            DisplayMeasure::UnknownVariant(value) => format!("unknown_variant_{}", *value),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum DisplayHeart {
    Bpm,
    Max,
    Reserve,
    UnknownVariant(u8),
}
impl DisplayHeart {
    pub fn from_u8(value: u8) -> DisplayHeart {
        match value {
            0 => DisplayHeart::Bpm,
            1 => DisplayHeart::Max,
            2 => DisplayHeart::Reserve,
            _ => DisplayHeart::UnknownVariant(value),
        }
    }
    pub fn from_i64(value: i64) -> DisplayHeart {
        DisplayHeart::from_u8(value as u8)
    }
    pub fn as_u8(&self) -> u8 {
        match &self {
            DisplayHeart::Bpm => 0,
            DisplayHeart::Max => 1,
            DisplayHeart::Reserve => 2,
            DisplayHeart::UnknownVariant(value) => *value,
        }
    }
    pub fn to_string(&self) -> String {
        match &self {
            DisplayHeart::Bpm => "bpm".to_string(),
            DisplayHeart::Max => "max".to_string(),
            DisplayHeart::Reserve => "reserve".to_string(),
            DisplayHeart::UnknownVariant(value) => format!("unknown_variant_{}", *value),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum DisplayPower {
    Watts,
    PercentFtp,
    UnknownVariant(u8),
}
impl DisplayPower {
    pub fn from_u8(value: u8) -> DisplayPower {
        match value {
            0 => DisplayPower::Watts,
            1 => DisplayPower::PercentFtp,
            _ => DisplayPower::UnknownVariant(value),
        }
    }
    pub fn from_i64(value: i64) -> DisplayPower {
        DisplayPower::from_u8(value as u8)
    }
    pub fn as_u8(&self) -> u8 {
        match &self {
            DisplayPower::Watts => 0,
            DisplayPower::PercentFtp => 1,
            DisplayPower::UnknownVariant(value) => *value,
        }
    }
    pub fn to_string(&self) -> String {
        match &self {
            DisplayPower::Watts => "watts".to_string(),
            DisplayPower::PercentFtp => "percent_ftp".to_string(),
            DisplayPower::UnknownVariant(value) => format!("unknown_variant_{}", *value),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum DisplayPosition {
    Degree,               // dd.dddddd
    DegreeMinute,         // dddmm.mmm
    DegreeMinuteSecond,   // dddmmss
    AustrianGrid,         // Austrian Grid (BMN)
    BritishGrid,          // British National Grid
    DutchGrid,            // Dutch grid system
    HungarianGrid,        // Hungarian grid system
    FinnishGrid,          // Finnish grid system Zone3 KKJ27
    GermanGrid,           // Gausss Krueger (German)
    IcelandicGrid,        // Icelandic Grid
    IndonesianEquatorial, // Indonesian Equatorial LCO
    IndonesianIrian,      // Indonesian Irian LCO
    IndonesianSouthern,   // Indonesian Southern LCO
    IndiaZone0,           // India zone 0
    IndiaZoneIA,          // India zone IA
    IndiaZoneIB,          // India zone IB
    IndiaZoneIIA,         // India zone IIA
    IndiaZoneIIB,         // India zone IIB
    IndiaZoneIIIA,        // India zone IIIA
    IndiaZoneIIIB,        // India zone IIIB
    IndiaZoneIVA,         // India zone IVA
    IndiaZoneIVB,         // India zone IVB
    IrishTransverse,      // Irish Transverse Mercator
    IrishGrid,            // Irish Grid
    Loran,                // Loran TD
    MaidenheadGrid,       // Maidenhead grid system
    MgrsGrid,             // MGRS grid system
    NewZealandGrid,       // New Zealand grid system
    NewZealandTransverse, // New Zealand Transverse Mercator
    QatarGrid,            // Qatar National Grid
    ModifiedSwedishGrid,  // Modified RT-90 (Sweden)
    SwedishGrid,          // RT-90 (Sweden)
    SouthAfricanGrid,     // South African Grid
    SwissGrid,            // Swiss CH-1903 grid
    TaiwanGrid,           // Taiwan Grid
    UnitedStatesGrid,     // United States National Grid
    UtmUpsGrid,           // UTM/UPS grid system
    WestMalayan,          // West Malayan RSO
    BorneoRso,            // Borneo RSO
    EstonianGrid,         // Estonian grid system
    LatvianGrid,          // Latvian Transverse Mercator
    SwedishRef99Grid,     // Reference Grid 99 TM (Swedish)
    UnknownVariant(u8),
}
impl DisplayPosition {
    pub fn from_u8(value: u8) -> DisplayPosition {
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
    pub fn from_i64(value: i64) -> DisplayPosition {
        DisplayPosition::from_u8(value as u8)
    }
    pub fn as_u8(&self) -> u8 {
        match &self {
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
            DisplayPosition::UnknownVariant(value) => *value,
        }
    }
    pub fn to_string(&self) -> String {
        match &self {
            DisplayPosition::Degree => "degree".to_string(),
            DisplayPosition::DegreeMinute => "degree_minute".to_string(),
            DisplayPosition::DegreeMinuteSecond => "degree_minute_second".to_string(),
            DisplayPosition::AustrianGrid => "austrian_grid".to_string(),
            DisplayPosition::BritishGrid => "british_grid".to_string(),
            DisplayPosition::DutchGrid => "dutch_grid".to_string(),
            DisplayPosition::HungarianGrid => "hungarian_grid".to_string(),
            DisplayPosition::FinnishGrid => "finnish_grid".to_string(),
            DisplayPosition::GermanGrid => "german_grid".to_string(),
            DisplayPosition::IcelandicGrid => "icelandic_grid".to_string(),
            DisplayPosition::IndonesianEquatorial => "indonesian_equatorial".to_string(),
            DisplayPosition::IndonesianIrian => "indonesian_irian".to_string(),
            DisplayPosition::IndonesianSouthern => "indonesian_southern".to_string(),
            DisplayPosition::IndiaZone0 => "india_zone_0".to_string(),
            DisplayPosition::IndiaZoneIA => "india_zone_IA".to_string(),
            DisplayPosition::IndiaZoneIB => "india_zone_IB".to_string(),
            DisplayPosition::IndiaZoneIIA => "india_zone_IIA".to_string(),
            DisplayPosition::IndiaZoneIIB => "india_zone_IIB".to_string(),
            DisplayPosition::IndiaZoneIIIA => "india_zone_IIIA".to_string(),
            DisplayPosition::IndiaZoneIIIB => "india_zone_IIIB".to_string(),
            DisplayPosition::IndiaZoneIVA => "india_zone_IVA".to_string(),
            DisplayPosition::IndiaZoneIVB => "india_zone_IVB".to_string(),
            DisplayPosition::IrishTransverse => "irish_transverse".to_string(),
            DisplayPosition::IrishGrid => "irish_grid".to_string(),
            DisplayPosition::Loran => "loran".to_string(),
            DisplayPosition::MaidenheadGrid => "maidenhead_grid".to_string(),
            DisplayPosition::MgrsGrid => "mgrs_grid".to_string(),
            DisplayPosition::NewZealandGrid => "new_zealand_grid".to_string(),
            DisplayPosition::NewZealandTransverse => "new_zealand_transverse".to_string(),
            DisplayPosition::QatarGrid => "qatar_grid".to_string(),
            DisplayPosition::ModifiedSwedishGrid => "modified_swedish_grid".to_string(),
            DisplayPosition::SwedishGrid => "swedish_grid".to_string(),
            DisplayPosition::SouthAfricanGrid => "south_african_grid".to_string(),
            DisplayPosition::SwissGrid => "swiss_grid".to_string(),
            DisplayPosition::TaiwanGrid => "taiwan_grid".to_string(),
            DisplayPosition::UnitedStatesGrid => "united_states_grid".to_string(),
            DisplayPosition::UtmUpsGrid => "utm_ups_grid".to_string(),
            DisplayPosition::WestMalayan => "west_malayan".to_string(),
            DisplayPosition::BorneoRso => "borneo_rso".to_string(),
            DisplayPosition::EstonianGrid => "estonian_grid".to_string(),
            DisplayPosition::LatvianGrid => "latvian_grid".to_string(),
            DisplayPosition::SwedishRef99Grid => "swedish_ref_99_grid".to_string(),
            DisplayPosition::UnknownVariant(value) => format!("unknown_variant_{}", *value),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Switch {
    Off,
    On,
    Auto,
    UnknownVariant(u8),
}
impl Switch {
    pub fn from_u8(value: u8) -> Switch {
        match value {
            0 => Switch::Off,
            1 => Switch::On,
            2 => Switch::Auto,
            _ => Switch::UnknownVariant(value),
        }
    }
    pub fn from_i64(value: i64) -> Switch {
        Switch::from_u8(value as u8)
    }
    pub fn as_u8(&self) -> u8 {
        match &self {
            Switch::Off => 0,
            Switch::On => 1,
            Switch::Auto => 2,
            Switch::UnknownVariant(value) => *value,
        }
    }
    pub fn to_string(&self) -> String {
        match &self {
            Switch::Off => "off".to_string(),
            Switch::On => "on".to_string(),
            Switch::Auto => "auto".to_string(),
            Switch::UnknownVariant(value) => format!("unknown_variant_{}", *value),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Sport {
    Generic,
    Running,
    Cycling,
    Transition, // Mulitsport transition
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
    All, // All is for goals only to include all sports.
    UnknownVariant(u8),
}
impl Sport {
    pub fn from_u8(value: u8) -> Sport {
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
    pub fn from_i64(value: i64) -> Sport {
        Sport::from_u8(value as u8)
    }
    pub fn as_u8(&self) -> u8 {
        match &self {
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
            Sport::UnknownVariant(value) => *value,
        }
    }
    pub fn to_string(&self) -> String {
        match &self {
            Sport::Generic => "generic".to_string(),
            Sport::Running => "running".to_string(),
            Sport::Cycling => "cycling".to_string(),
            Sport::Transition => "transition".to_string(),
            Sport::FitnessEquipment => "fitness_equipment".to_string(),
            Sport::Swimming => "swimming".to_string(),
            Sport::Basketball => "basketball".to_string(),
            Sport::Soccer => "soccer".to_string(),
            Sport::Tennis => "tennis".to_string(),
            Sport::AmericanFootball => "american_football".to_string(),
            Sport::Training => "training".to_string(),
            Sport::Walking => "walking".to_string(),
            Sport::CrossCountrySkiing => "cross_country_skiing".to_string(),
            Sport::AlpineSkiing => "alpine_skiing".to_string(),
            Sport::Snowboarding => "snowboarding".to_string(),
            Sport::Rowing => "rowing".to_string(),
            Sport::Mountaineering => "mountaineering".to_string(),
            Sport::Hiking => "hiking".to_string(),
            Sport::Multisport => "multisport".to_string(),
            Sport::Paddling => "paddling".to_string(),
            Sport::Flying => "flying".to_string(),
            Sport::EBiking => "e_biking".to_string(),
            Sport::Motorcycling => "motorcycling".to_string(),
            Sport::Boating => "boating".to_string(),
            Sport::Driving => "driving".to_string(),
            Sport::Golf => "golf".to_string(),
            Sport::HangGliding => "hang_gliding".to_string(),
            Sport::HorsebackRiding => "horseback_riding".to_string(),
            Sport::Hunting => "hunting".to_string(),
            Sport::Fishing => "fishing".to_string(),
            Sport::InlineSkating => "inline_skating".to_string(),
            Sport::RockClimbing => "rock_climbing".to_string(),
            Sport::Sailing => "sailing".to_string(),
            Sport::IceSkating => "ice_skating".to_string(),
            Sport::SkyDiving => "sky_diving".to_string(),
            Sport::Snowshoeing => "snowshoeing".to_string(),
            Sport::Snowmobiling => "snowmobiling".to_string(),
            Sport::StandUpPaddleboarding => "stand_up_paddleboarding".to_string(),
            Sport::Surfing => "surfing".to_string(),
            Sport::Wakeboarding => "wakeboarding".to_string(),
            Sport::WaterSkiing => "water_skiing".to_string(),
            Sport::Kayaking => "kayaking".to_string(),
            Sport::Rafting => "rafting".to_string(),
            Sport::Windsurfing => "windsurfing".to_string(),
            Sport::Kitesurfing => "kitesurfing".to_string(),
            Sport::Tactical => "tactical".to_string(),
            Sport::Jumpmaster => "jumpmaster".to_string(),
            Sport::Boxing => "boxing".to_string(),
            Sport::FloorClimbing => "floor_climbing".to_string(),
            Sport::All => "all".to_string(),
            Sport::UnknownVariant(value) => format!("unknown_variant_{}", *value),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum SportBits0 {
    Generic,
    Running,
    Cycling,
    Transition, // Mulitsport transition
    FitnessEquipment,
    Swimming,
    Basketball,
    Soccer,
    UnknownVariant(u8),
}
impl SportBits0 {
    pub fn from_u8(value: u8) -> SportBits0 {
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
    pub fn from_i64(value: i64) -> SportBits0 {
        SportBits0::from_u8(value as u8)
    }
    pub fn as_u8(&self) -> u8 {
        match &self {
            SportBits0::Generic => 1,
            SportBits0::Running => 2,
            SportBits0::Cycling => 4,
            SportBits0::Transition => 8,
            SportBits0::FitnessEquipment => 16,
            SportBits0::Swimming => 32,
            SportBits0::Basketball => 64,
            SportBits0::Soccer => 128,
            SportBits0::UnknownVariant(value) => *value,
        }
    }
    pub fn to_string(&self) -> String {
        match &self {
            SportBits0::Generic => "generic".to_string(),
            SportBits0::Running => "running".to_string(),
            SportBits0::Cycling => "cycling".to_string(),
            SportBits0::Transition => "transition".to_string(),
            SportBits0::FitnessEquipment => "fitness_equipment".to_string(),
            SportBits0::Swimming => "swimming".to_string(),
            SportBits0::Basketball => "basketball".to_string(),
            SportBits0::Soccer => "soccer".to_string(),
            SportBits0::UnknownVariant(value) => format!("unknown_variant_{}", *value),
        }
    }
}

#[derive(Clone, Copy, Debug)]
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
    pub fn from_u8(value: u8) -> SportBits1 {
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
    pub fn from_i64(value: i64) -> SportBits1 {
        SportBits1::from_u8(value as u8)
    }
    pub fn as_u8(&self) -> u8 {
        match &self {
            SportBits1::Tennis => 1,
            SportBits1::AmericanFootball => 2,
            SportBits1::Training => 4,
            SportBits1::Walking => 8,
            SportBits1::CrossCountrySkiing => 16,
            SportBits1::AlpineSkiing => 32,
            SportBits1::Snowboarding => 64,
            SportBits1::Rowing => 128,
            SportBits1::UnknownVariant(value) => *value,
        }
    }
    pub fn to_string(&self) -> String {
        match &self {
            SportBits1::Tennis => "tennis".to_string(),
            SportBits1::AmericanFootball => "american_football".to_string(),
            SportBits1::Training => "training".to_string(),
            SportBits1::Walking => "walking".to_string(),
            SportBits1::CrossCountrySkiing => "cross_country_skiing".to_string(),
            SportBits1::AlpineSkiing => "alpine_skiing".to_string(),
            SportBits1::Snowboarding => "snowboarding".to_string(),
            SportBits1::Rowing => "rowing".to_string(),
            SportBits1::UnknownVariant(value) => format!("unknown_variant_{}", *value),
        }
    }
}

#[derive(Clone, Copy, Debug)]
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
    pub fn from_u8(value: u8) -> SportBits2 {
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
    pub fn from_i64(value: i64) -> SportBits2 {
        SportBits2::from_u8(value as u8)
    }
    pub fn as_u8(&self) -> u8 {
        match &self {
            SportBits2::Mountaineering => 1,
            SportBits2::Hiking => 2,
            SportBits2::Multisport => 4,
            SportBits2::Paddling => 8,
            SportBits2::Flying => 16,
            SportBits2::EBiking => 32,
            SportBits2::Motorcycling => 64,
            SportBits2::Boating => 128,
            SportBits2::UnknownVariant(value) => *value,
        }
    }
    pub fn to_string(&self) -> String {
        match &self {
            SportBits2::Mountaineering => "mountaineering".to_string(),
            SportBits2::Hiking => "hiking".to_string(),
            SportBits2::Multisport => "multisport".to_string(),
            SportBits2::Paddling => "paddling".to_string(),
            SportBits2::Flying => "flying".to_string(),
            SportBits2::EBiking => "e_biking".to_string(),
            SportBits2::Motorcycling => "motorcycling".to_string(),
            SportBits2::Boating => "boating".to_string(),
            SportBits2::UnknownVariant(value) => format!("unknown_variant_{}", *value),
        }
    }
}

#[derive(Clone, Copy, Debug)]
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
    pub fn from_u8(value: u8) -> SportBits3 {
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
    pub fn from_i64(value: i64) -> SportBits3 {
        SportBits3::from_u8(value as u8)
    }
    pub fn as_u8(&self) -> u8 {
        match &self {
            SportBits3::Driving => 1,
            SportBits3::Golf => 2,
            SportBits3::HangGliding => 4,
            SportBits3::HorsebackRiding => 8,
            SportBits3::Hunting => 16,
            SportBits3::Fishing => 32,
            SportBits3::InlineSkating => 64,
            SportBits3::RockClimbing => 128,
            SportBits3::UnknownVariant(value) => *value,
        }
    }
    pub fn to_string(&self) -> String {
        match &self {
            SportBits3::Driving => "driving".to_string(),
            SportBits3::Golf => "golf".to_string(),
            SportBits3::HangGliding => "hang_gliding".to_string(),
            SportBits3::HorsebackRiding => "horseback_riding".to_string(),
            SportBits3::Hunting => "hunting".to_string(),
            SportBits3::Fishing => "fishing".to_string(),
            SportBits3::InlineSkating => "inline_skating".to_string(),
            SportBits3::RockClimbing => "rock_climbing".to_string(),
            SportBits3::UnknownVariant(value) => format!("unknown_variant_{}", *value),
        }
    }
}

#[derive(Clone, Copy, Debug)]
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
    pub fn from_u8(value: u8) -> SportBits4 {
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
    pub fn from_i64(value: i64) -> SportBits4 {
        SportBits4::from_u8(value as u8)
    }
    pub fn as_u8(&self) -> u8 {
        match &self {
            SportBits4::Sailing => 1,
            SportBits4::IceSkating => 2,
            SportBits4::SkyDiving => 4,
            SportBits4::Snowshoeing => 8,
            SportBits4::Snowmobiling => 16,
            SportBits4::StandUpPaddleboarding => 32,
            SportBits4::Surfing => 64,
            SportBits4::Wakeboarding => 128,
            SportBits4::UnknownVariant(value) => *value,
        }
    }
    pub fn to_string(&self) -> String {
        match &self {
            SportBits4::Sailing => "sailing".to_string(),
            SportBits4::IceSkating => "ice_skating".to_string(),
            SportBits4::SkyDiving => "sky_diving".to_string(),
            SportBits4::Snowshoeing => "snowshoeing".to_string(),
            SportBits4::Snowmobiling => "snowmobiling".to_string(),
            SportBits4::StandUpPaddleboarding => "stand_up_paddleboarding".to_string(),
            SportBits4::Surfing => "surfing".to_string(),
            SportBits4::Wakeboarding => "wakeboarding".to_string(),
            SportBits4::UnknownVariant(value) => format!("unknown_variant_{}", *value),
        }
    }
}

#[derive(Clone, Copy, Debug)]
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
    pub fn from_u8(value: u8) -> SportBits5 {
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
    pub fn from_i64(value: i64) -> SportBits5 {
        SportBits5::from_u8(value as u8)
    }
    pub fn as_u8(&self) -> u8 {
        match &self {
            SportBits5::WaterSkiing => 1,
            SportBits5::Kayaking => 2,
            SportBits5::Rafting => 4,
            SportBits5::Windsurfing => 8,
            SportBits5::Kitesurfing => 16,
            SportBits5::Tactical => 32,
            SportBits5::Jumpmaster => 64,
            SportBits5::Boxing => 128,
            SportBits5::UnknownVariant(value) => *value,
        }
    }
    pub fn to_string(&self) -> String {
        match &self {
            SportBits5::WaterSkiing => "water_skiing".to_string(),
            SportBits5::Kayaking => "kayaking".to_string(),
            SportBits5::Rafting => "rafting".to_string(),
            SportBits5::Windsurfing => "windsurfing".to_string(),
            SportBits5::Kitesurfing => "kitesurfing".to_string(),
            SportBits5::Tactical => "tactical".to_string(),
            SportBits5::Jumpmaster => "jumpmaster".to_string(),
            SportBits5::Boxing => "boxing".to_string(),
            SportBits5::UnknownVariant(value) => format!("unknown_variant_{}", *value),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum SportBits6 {
    FloorClimbing,
    UnknownVariant(u8),
}
impl SportBits6 {
    pub fn from_u8(value: u8) -> SportBits6 {
        match value {
            1 => SportBits6::FloorClimbing,
            _ => SportBits6::UnknownVariant(value),
        }
    }
    pub fn from_i64(value: i64) -> SportBits6 {
        SportBits6::from_u8(value as u8)
    }
    pub fn as_u8(&self) -> u8 {
        match &self {
            SportBits6::FloorClimbing => 1,
            SportBits6::UnknownVariant(value) => *value,
        }
    }
    pub fn to_string(&self) -> String {
        match &self {
            SportBits6::FloorClimbing => "floor_climbing".to_string(),
            SportBits6::UnknownVariant(value) => format!("unknown_variant_{}", *value),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum SubSport {
    Generic,
    Treadmill,           // Run/Fitness Equipment
    Street,              // Run
    Trail,               // Run
    Track,               // Run
    Spin,                // Cycling
    IndoorCycling,       // Cycling/Fitness Equipment
    Road,                // Cycling
    Mountain,            // Cycling
    Downhill,            // Cycling
    Recumbent,           // Cycling
    Cyclocross,          // Cycling
    HandCycling,         // Cycling
    TrackCycling,        // Cycling
    IndoorRowing,        // Fitness Equipment
    Elliptical,          // Fitness Equipment
    StairClimbing,       // Fitness Equipment
    LapSwimming,         // Swimming
    OpenWater,           // Swimming
    FlexibilityTraining, // Training
    StrengthTraining,    // Training
    WarmUp,              // Tennis
    Match,               // Tennis
    Exercise,            // Tennis
    Challenge,
    IndoorSkiing,         // Fitness Equipment
    CardioTraining,       // Training
    IndoorWalking,        // Walking/Fitness Equipment
    EBikeFitness,         // E-Biking
    Bmx,                  // Cycling
    CasualWalking,        // Walking
    SpeedWalking,         // Walking
    BikeToRunTransition,  // Transition
    RunToBikeTransition,  // Transition
    SwimToBikeTransition, // Transition
    Atv,                  // Motorcycling
    Motocross,            // Motorcycling
    Backcountry,          // Alpine Skiing/Snowboarding
    Resort,               // Alpine Skiing/Snowboarding
    RcDrone,              // Flying
    Wingsuit,             // Flying
    Whitewater,           // Kayaking/Rafting
    SkateSkiing,          // Cross Country Skiing
    Yoga,                 // Training
    Pilates,              // Fitness Equipment
    IndoorRunning,        // Run
    GravelCycling,        // Cycling
    EBikeMountain,        // Cycling
    Commuting,            // Cycling
    MixedSurface,         // Cycling
    Navigate,
    TrackMe,
    Map,
    SingleGasDiving, // Diving
    MultiGasDiving,  // Diving
    GaugeDiving,     // Diving
    ApneaDiving,     // Diving
    ApneaHunting,    // Diving
    VirtualActivity,
    Obstacle, // Used for events where participants run, crawl through mud, climb over walls, etc.
    All,
    UnknownVariant(u8),
}
impl SubSport {
    pub fn from_u8(value: u8) -> SubSport {
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
    pub fn from_i64(value: i64) -> SubSport {
        SubSport::from_u8(value as u8)
    }
    pub fn as_u8(&self) -> u8 {
        match &self {
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
            SubSport::UnknownVariant(value) => *value,
        }
    }
    pub fn to_string(&self) -> String {
        match &self {
            SubSport::Generic => "generic".to_string(),
            SubSport::Treadmill => "treadmill".to_string(),
            SubSport::Street => "street".to_string(),
            SubSport::Trail => "trail".to_string(),
            SubSport::Track => "track".to_string(),
            SubSport::Spin => "spin".to_string(),
            SubSport::IndoorCycling => "indoor_cycling".to_string(),
            SubSport::Road => "road".to_string(),
            SubSport::Mountain => "mountain".to_string(),
            SubSport::Downhill => "downhill".to_string(),
            SubSport::Recumbent => "recumbent".to_string(),
            SubSport::Cyclocross => "cyclocross".to_string(),
            SubSport::HandCycling => "hand_cycling".to_string(),
            SubSport::TrackCycling => "track_cycling".to_string(),
            SubSport::IndoorRowing => "indoor_rowing".to_string(),
            SubSport::Elliptical => "elliptical".to_string(),
            SubSport::StairClimbing => "stair_climbing".to_string(),
            SubSport::LapSwimming => "lap_swimming".to_string(),
            SubSport::OpenWater => "open_water".to_string(),
            SubSport::FlexibilityTraining => "flexibility_training".to_string(),
            SubSport::StrengthTraining => "strength_training".to_string(),
            SubSport::WarmUp => "warm_up".to_string(),
            SubSport::Match => "match".to_string(),
            SubSport::Exercise => "exercise".to_string(),
            SubSport::Challenge => "challenge".to_string(),
            SubSport::IndoorSkiing => "indoor_skiing".to_string(),
            SubSport::CardioTraining => "cardio_training".to_string(),
            SubSport::IndoorWalking => "indoor_walking".to_string(),
            SubSport::EBikeFitness => "e_bike_fitness".to_string(),
            SubSport::Bmx => "bmx".to_string(),
            SubSport::CasualWalking => "casual_walking".to_string(),
            SubSport::SpeedWalking => "speed_walking".to_string(),
            SubSport::BikeToRunTransition => "bike_to_run_transition".to_string(),
            SubSport::RunToBikeTransition => "run_to_bike_transition".to_string(),
            SubSport::SwimToBikeTransition => "swim_to_bike_transition".to_string(),
            SubSport::Atv => "atv".to_string(),
            SubSport::Motocross => "motocross".to_string(),
            SubSport::Backcountry => "backcountry".to_string(),
            SubSport::Resort => "resort".to_string(),
            SubSport::RcDrone => "rc_drone".to_string(),
            SubSport::Wingsuit => "wingsuit".to_string(),
            SubSport::Whitewater => "whitewater".to_string(),
            SubSport::SkateSkiing => "skate_skiing".to_string(),
            SubSport::Yoga => "yoga".to_string(),
            SubSport::Pilates => "pilates".to_string(),
            SubSport::IndoorRunning => "indoor_running".to_string(),
            SubSport::GravelCycling => "gravel_cycling".to_string(),
            SubSport::EBikeMountain => "e_bike_mountain".to_string(),
            SubSport::Commuting => "commuting".to_string(),
            SubSport::MixedSurface => "mixed_surface".to_string(),
            SubSport::Navigate => "navigate".to_string(),
            SubSport::TrackMe => "track_me".to_string(),
            SubSport::Map => "map".to_string(),
            SubSport::SingleGasDiving => "single_gas_diving".to_string(),
            SubSport::MultiGasDiving => "multi_gas_diving".to_string(),
            SubSport::GaugeDiving => "gauge_diving".to_string(),
            SubSport::ApneaDiving => "apnea_diving".to_string(),
            SubSport::ApneaHunting => "apnea_hunting".to_string(),
            SubSport::VirtualActivity => "virtual_activity".to_string(),
            SubSport::Obstacle => "obstacle".to_string(),
            SubSport::All => "all".to_string(),
            SubSport::UnknownVariant(value) => format!("unknown_variant_{}", *value),
        }
    }
}

#[derive(Clone, Copy, Debug)]
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
    pub fn from_u8(value: u8) -> SportEvent {
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
    pub fn from_i64(value: i64) -> SportEvent {
        SportEvent::from_u8(value as u8)
    }
    pub fn as_u8(&self) -> u8 {
        match &self {
            SportEvent::Uncategorized => 0,
            SportEvent::Geocaching => 1,
            SportEvent::Fitness => 2,
            SportEvent::Recreation => 3,
            SportEvent::Race => 4,
            SportEvent::SpecialEvent => 5,
            SportEvent::Training => 6,
            SportEvent::Transportation => 7,
            SportEvent::Touring => 8,
            SportEvent::UnknownVariant(value) => *value,
        }
    }
    pub fn to_string(&self) -> String {
        match &self {
            SportEvent::Uncategorized => "uncategorized".to_string(),
            SportEvent::Geocaching => "geocaching".to_string(),
            SportEvent::Fitness => "fitness".to_string(),
            SportEvent::Recreation => "recreation".to_string(),
            SportEvent::Race => "race".to_string(),
            SportEvent::SpecialEvent => "special_event".to_string(),
            SportEvent::Training => "training".to_string(),
            SportEvent::Transportation => "transportation".to_string(),
            SportEvent::Touring => "touring".to_string(),
            SportEvent::UnknownVariant(value) => format!("unknown_variant_{}", *value),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Activity {
    Manual,
    AutoMultiSport,
    UnknownVariant(u8),
}
impl Activity {
    pub fn from_u8(value: u8) -> Activity {
        match value {
            0 => Activity::Manual,
            1 => Activity::AutoMultiSport,
            _ => Activity::UnknownVariant(value),
        }
    }
    pub fn from_i64(value: i64) -> Activity {
        Activity::from_u8(value as u8)
    }
    pub fn as_u8(&self) -> u8 {
        match &self {
            Activity::Manual => 0,
            Activity::AutoMultiSport => 1,
            Activity::UnknownVariant(value) => *value,
        }
    }
    pub fn to_string(&self) -> String {
        match &self {
            Activity::Manual => "manual".to_string(),
            Activity::AutoMultiSport => "auto_multi_sport".to_string(),
            Activity::UnknownVariant(value) => format!("unknown_variant_{}", *value),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Intensity {
    Active,
    Rest,
    Warmup,
    Cooldown,
    UnknownVariant(u8),
}
impl Intensity {
    pub fn from_u8(value: u8) -> Intensity {
        match value {
            0 => Intensity::Active,
            1 => Intensity::Rest,
            2 => Intensity::Warmup,
            3 => Intensity::Cooldown,
            _ => Intensity::UnknownVariant(value),
        }
    }
    pub fn from_i64(value: i64) -> Intensity {
        Intensity::from_u8(value as u8)
    }
    pub fn as_u8(&self) -> u8 {
        match &self {
            Intensity::Active => 0,
            Intensity::Rest => 1,
            Intensity::Warmup => 2,
            Intensity::Cooldown => 3,
            Intensity::UnknownVariant(value) => *value,
        }
    }
    pub fn to_string(&self) -> String {
        match &self {
            Intensity::Active => "active".to_string(),
            Intensity::Rest => "rest".to_string(),
            Intensity::Warmup => "warmup".to_string(),
            Intensity::Cooldown => "cooldown".to_string(),
            Intensity::UnknownVariant(value) => format!("unknown_variant_{}", *value),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum SessionTrigger {
    ActivityEnd,
    Manual,           // User changed sport.
    AutoMultiSport, // Auto multi-sport feature is enabled and user pressed lap button to advance session.
    FitnessEquipment, // Auto sport change caused by user linking to fitness equipment.
    UnknownVariant(u8),
}
impl SessionTrigger {
    pub fn from_u8(value: u8) -> SessionTrigger {
        match value {
            0 => SessionTrigger::ActivityEnd,
            1 => SessionTrigger::Manual,
            2 => SessionTrigger::AutoMultiSport,
            3 => SessionTrigger::FitnessEquipment,
            _ => SessionTrigger::UnknownVariant(value),
        }
    }
    pub fn from_i64(value: i64) -> SessionTrigger {
        SessionTrigger::from_u8(value as u8)
    }
    pub fn as_u8(&self) -> u8 {
        match &self {
            SessionTrigger::ActivityEnd => 0,
            SessionTrigger::Manual => 1,
            SessionTrigger::AutoMultiSport => 2,
            SessionTrigger::FitnessEquipment => 3,
            SessionTrigger::UnknownVariant(value) => *value,
        }
    }
    pub fn to_string(&self) -> String {
        match &self {
            SessionTrigger::ActivityEnd => "activity_end".to_string(),
            SessionTrigger::Manual => "manual".to_string(),
            SessionTrigger::AutoMultiSport => "auto_multi_sport".to_string(),
            SessionTrigger::FitnessEquipment => "fitness_equipment".to_string(),
            SessionTrigger::UnknownVariant(value) => format!("unknown_variant_{}", *value),
        }
    }
}

#[derive(Clone, Copy, Debug)]
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
    pub fn from_u8(value: u8) -> AutolapTrigger {
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
    pub fn from_i64(value: i64) -> AutolapTrigger {
        AutolapTrigger::from_u8(value as u8)
    }
    pub fn as_u8(&self) -> u8 {
        match &self {
            AutolapTrigger::Time => 0,
            AutolapTrigger::Distance => 1,
            AutolapTrigger::PositionStart => 2,
            AutolapTrigger::PositionLap => 3,
            AutolapTrigger::PositionWaypoint => 4,
            AutolapTrigger::PositionMarked => 5,
            AutolapTrigger::Off => 6,
            AutolapTrigger::UnknownVariant(value) => *value,
        }
    }
    pub fn to_string(&self) -> String {
        match &self {
            AutolapTrigger::Time => "time".to_string(),
            AutolapTrigger::Distance => "distance".to_string(),
            AutolapTrigger::PositionStart => "position_start".to_string(),
            AutolapTrigger::PositionLap => "position_lap".to_string(),
            AutolapTrigger::PositionWaypoint => "position_waypoint".to_string(),
            AutolapTrigger::PositionMarked => "position_marked".to_string(),
            AutolapTrigger::Off => "off".to_string(),
            AutolapTrigger::UnknownVariant(value) => format!("unknown_variant_{}", *value),
        }
    }
}

#[derive(Clone, Copy, Debug)]
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
    pub fn from_u8(value: u8) -> LapTrigger {
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
    pub fn from_i64(value: i64) -> LapTrigger {
        LapTrigger::from_u8(value as u8)
    }
    pub fn as_u8(&self) -> u8 {
        match &self {
            LapTrigger::Manual => 0,
            LapTrigger::Time => 1,
            LapTrigger::Distance => 2,
            LapTrigger::PositionStart => 3,
            LapTrigger::PositionLap => 4,
            LapTrigger::PositionWaypoint => 5,
            LapTrigger::PositionMarked => 6,
            LapTrigger::SessionEnd => 7,
            LapTrigger::FitnessEquipment => 8,
            LapTrigger::UnknownVariant(value) => *value,
        }
    }
    pub fn to_string(&self) -> String {
        match &self {
            LapTrigger::Manual => "manual".to_string(),
            LapTrigger::Time => "time".to_string(),
            LapTrigger::Distance => "distance".to_string(),
            LapTrigger::PositionStart => "position_start".to_string(),
            LapTrigger::PositionLap => "position_lap".to_string(),
            LapTrigger::PositionWaypoint => "position_waypoint".to_string(),
            LapTrigger::PositionMarked => "position_marked".to_string(),
            LapTrigger::SessionEnd => "session_end".to_string(),
            LapTrigger::FitnessEquipment => "fitness_equipment".to_string(),
            LapTrigger::UnknownVariant(value) => format!("unknown_variant_{}", *value),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum TimeMode {
    Hour12,
    Hour24,   // Does not use a leading zero and has a colon
    Military, // Uses a leading zero and does not have a colon
    Hour12WithSeconds,
    Hour24WithSeconds,
    Utc,
    UnknownVariant(u8),
}
impl TimeMode {
    pub fn from_u8(value: u8) -> TimeMode {
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
    pub fn from_i64(value: i64) -> TimeMode {
        TimeMode::from_u8(value as u8)
    }
    pub fn as_u8(&self) -> u8 {
        match &self {
            TimeMode::Hour12 => 0,
            TimeMode::Hour24 => 1,
            TimeMode::Military => 2,
            TimeMode::Hour12WithSeconds => 3,
            TimeMode::Hour24WithSeconds => 4,
            TimeMode::Utc => 5,
            TimeMode::UnknownVariant(value) => *value,
        }
    }
    pub fn to_string(&self) -> String {
        match &self {
            TimeMode::Hour12 => "hour12".to_string(),
            TimeMode::Hour24 => "hour24".to_string(),
            TimeMode::Military => "military".to_string(),
            TimeMode::Hour12WithSeconds => "hour_12_with_seconds".to_string(),
            TimeMode::Hour24WithSeconds => "hour_24_with_seconds".to_string(),
            TimeMode::Utc => "utc".to_string(),
            TimeMode::UnknownVariant(value) => format!("unknown_variant_{}", *value),
        }
    }
}

#[derive(Clone, Copy, Debug)]
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
    pub fn from_u8(value: u8) -> BacklightMode {
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
    pub fn from_i64(value: i64) -> BacklightMode {
        BacklightMode::from_u8(value as u8)
    }
    pub fn as_u8(&self) -> u8 {
        match &self {
            BacklightMode::Off => 0,
            BacklightMode::Manual => 1,
            BacklightMode::KeyAndMessages => 2,
            BacklightMode::AutoBrightness => 3,
            BacklightMode::SmartNotifications => 4,
            BacklightMode::KeyAndMessagesNight => 5,
            BacklightMode::KeyAndMessagesAndSmartNotifications => 6,
            BacklightMode::UnknownVariant(value) => *value,
        }
    }
    pub fn to_string(&self) -> String {
        match &self {
            BacklightMode::Off => "off".to_string(),
            BacklightMode::Manual => "manual".to_string(),
            BacklightMode::KeyAndMessages => "key_and_messages".to_string(),
            BacklightMode::AutoBrightness => "auto_brightness".to_string(),
            BacklightMode::SmartNotifications => "smart_notifications".to_string(),
            BacklightMode::KeyAndMessagesNight => "key_and_messages_night".to_string(),
            BacklightMode::KeyAndMessagesAndSmartNotifications => {
                "key_and_messages_and_smart_notifications".to_string()
            }
            BacklightMode::UnknownVariant(value) => format!("unknown_variant_{}", *value),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum DateMode {
    DayMonth,
    MonthDay,
    UnknownVariant(u8),
}
impl DateMode {
    pub fn from_u8(value: u8) -> DateMode {
        match value {
            0 => DateMode::DayMonth,
            1 => DateMode::MonthDay,
            _ => DateMode::UnknownVariant(value),
        }
    }
    pub fn from_i64(value: i64) -> DateMode {
        DateMode::from_u8(value as u8)
    }
    pub fn as_u8(&self) -> u8 {
        match &self {
            DateMode::DayMonth => 0,
            DateMode::MonthDay => 1,
            DateMode::UnknownVariant(value) => *value,
        }
    }
    pub fn to_string(&self) -> String {
        match &self {
            DateMode::DayMonth => "day_month".to_string(),
            DateMode::MonthDay => "month_day".to_string(),
            DateMode::UnknownVariant(value) => format!("unknown_variant_{}", *value),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum BacklightTimeout {
    Infinite, // Backlight stays on forever.
    UnknownVariant(u8),
}
impl BacklightTimeout {
    pub fn from_u8(value: u8) -> BacklightTimeout {
        match value {
            0 => BacklightTimeout::Infinite,
            _ => BacklightTimeout::UnknownVariant(value),
        }
    }
    pub fn from_i64(value: i64) -> BacklightTimeout {
        BacklightTimeout::from_u8(value as u8)
    }
    pub fn as_u8(&self) -> u8 {
        match &self {
            BacklightTimeout::Infinite => 0,
            BacklightTimeout::UnknownVariant(value) => *value,
        }
    }
    pub fn to_string(&self) -> String {
        match &self {
            BacklightTimeout::Infinite => "infinite".to_string(),
            BacklightTimeout::UnknownVariant(value) => format!("unknown_variant_{}", *value),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Event {
    Timer,                 // Group 0.  Start / stop_all
    Workout,               // start / stop
    WorkoutStep,           // Start at beginning of workout.  Stop at end of each step.
    PowerDown,             // stop_all group 0
    PowerUp,               // stop_all group 0
    OffCourse,             // start / stop group 0
    Session,               // Stop at end of each session.
    Lap,                   // Stop at end of each lap.
    CoursePoint,           // marker
    Battery,               // marker
    VirtualPartnerPace, // Group 1. Start at beginning of activity if VP enabled, when VP pace is changed during activity or VP enabled mid activity.  stop_disable when VP disabled.
    HrHighAlert,        // Group 0.  Start / stop when in alert condition.
    HrLowAlert,         // Group 0.  Start / stop when in alert condition.
    SpeedHighAlert,     // Group 0.  Start / stop when in alert condition.
    SpeedLowAlert,      // Group 0.  Start / stop when in alert condition.
    CadHighAlert,       // Group 0.  Start / stop when in alert condition.
    CadLowAlert,        // Group 0.  Start / stop when in alert condition.
    PowerHighAlert,     // Group 0.  Start / stop when in alert condition.
    PowerLowAlert,      // Group 0.  Start / stop when in alert condition.
    RecoveryHr,         // marker
    BatteryLow,         // marker
    TimeDurationAlert, // Group 1.  Start if enabled mid activity (not required at start of activity). Stop when duration is reached.  stop_disable if disabled.
    DistanceDurationAlert, // Group 1.  Start if enabled mid activity (not required at start of activity). Stop when duration is reached.  stop_disable if disabled.
    CalorieDurationAlert, // Group 1.  Start if enabled mid activity (not required at start of activity). Stop when duration is reached.  stop_disable if disabled.
    Activity,             // Group 1..  Stop at end of activity.
    FitnessEquipment,     // marker
    Length,               // Stop at end of each length.
    UserMarker,           // marker
    SportPoint,           // marker
    Calibration,          // start/stop/marker
    FrontGearChange,      // marker
    RearGearChange,       // marker
    RiderPositionChange,  // marker
    ElevHighAlert,        // Group 0.  Start / stop when in alert condition.
    ElevLowAlert,         // Group 0.  Start / stop when in alert condition.
    CommTimeout,          // marker
    UnknownVariant(u8),
}
impl Event {
    pub fn from_u8(value: u8) -> Event {
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
    pub fn from_i64(value: i64) -> Event {
        Event::from_u8(value as u8)
    }
    pub fn as_u8(&self) -> u8 {
        match &self {
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
            Event::UnknownVariant(value) => *value,
        }
    }
    pub fn to_string(&self) -> String {
        match &self {
            Event::Timer => "timer".to_string(),
            Event::Workout => "workout".to_string(),
            Event::WorkoutStep => "workout_step".to_string(),
            Event::PowerDown => "power_down".to_string(),
            Event::PowerUp => "power_up".to_string(),
            Event::OffCourse => "off_course".to_string(),
            Event::Session => "session".to_string(),
            Event::Lap => "lap".to_string(),
            Event::CoursePoint => "course_point".to_string(),
            Event::Battery => "battery".to_string(),
            Event::VirtualPartnerPace => "virtual_partner_pace".to_string(),
            Event::HrHighAlert => "hr_high_alert".to_string(),
            Event::HrLowAlert => "hr_low_alert".to_string(),
            Event::SpeedHighAlert => "speed_high_alert".to_string(),
            Event::SpeedLowAlert => "speed_low_alert".to_string(),
            Event::CadHighAlert => "cad_high_alert".to_string(),
            Event::CadLowAlert => "cad_low_alert".to_string(),
            Event::PowerHighAlert => "power_high_alert".to_string(),
            Event::PowerLowAlert => "power_low_alert".to_string(),
            Event::RecoveryHr => "recovery_hr".to_string(),
            Event::BatteryLow => "battery_low".to_string(),
            Event::TimeDurationAlert => "time_duration_alert".to_string(),
            Event::DistanceDurationAlert => "distance_duration_alert".to_string(),
            Event::CalorieDurationAlert => "calorie_duration_alert".to_string(),
            Event::Activity => "activity".to_string(),
            Event::FitnessEquipment => "fitness_equipment".to_string(),
            Event::Length => "length".to_string(),
            Event::UserMarker => "user_marker".to_string(),
            Event::SportPoint => "sport_point".to_string(),
            Event::Calibration => "calibration".to_string(),
            Event::FrontGearChange => "front_gear_change".to_string(),
            Event::RearGearChange => "rear_gear_change".to_string(),
            Event::RiderPositionChange => "rider_position_change".to_string(),
            Event::ElevHighAlert => "elev_high_alert".to_string(),
            Event::ElevLowAlert => "elev_low_alert".to_string(),
            Event::CommTimeout => "comm_timeout".to_string(),
            Event::UnknownVariant(value) => format!("unknown_variant_{}", *value),
        }
    }
}

#[derive(Clone, Copy, Debug)]
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
    pub fn from_u8(value: u8) -> EventType {
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
    pub fn from_i64(value: i64) -> EventType {
        EventType::from_u8(value as u8)
    }
    pub fn as_u8(&self) -> u8 {
        match &self {
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
            EventType::UnknownVariant(value) => *value,
        }
    }
    pub fn to_string(&self) -> String {
        match &self {
            EventType::Start => "start".to_string(),
            EventType::Stop => "stop".to_string(),
            EventType::ConsecutiveDepreciated => "consecutive_depreciated".to_string(),
            EventType::Marker => "marker".to_string(),
            EventType::StopAll => "stop_all".to_string(),
            EventType::BeginDepreciated => "begin_depreciated".to_string(),
            EventType::EndDepreciated => "end_depreciated".to_string(),
            EventType::EndAllDepreciated => "end_all_depreciated".to_string(),
            EventType::StopDisable => "stop_disable".to_string(),
            EventType::StopDisableAll => "stop_disable_all".to_string(),
            EventType::UnknownVariant(value) => format!("unknown_variant_{}", *value),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum TimerTrigger {
    Manual,
    Auto,
    FitnessEquipment,
    UnknownVariant(u8),
}
impl TimerTrigger {
    pub fn from_u8(value: u8) -> TimerTrigger {
        match value {
            0 => TimerTrigger::Manual,
            1 => TimerTrigger::Auto,
            2 => TimerTrigger::FitnessEquipment,
            _ => TimerTrigger::UnknownVariant(value),
        }
    }
    pub fn from_i64(value: i64) -> TimerTrigger {
        TimerTrigger::from_u8(value as u8)
    }
    pub fn as_u8(&self) -> u8 {
        match &self {
            TimerTrigger::Manual => 0,
            TimerTrigger::Auto => 1,
            TimerTrigger::FitnessEquipment => 2,
            TimerTrigger::UnknownVariant(value) => *value,
        }
    }
    pub fn to_string(&self) -> String {
        match &self {
            TimerTrigger::Manual => "manual".to_string(),
            TimerTrigger::Auto => "auto".to_string(),
            TimerTrigger::FitnessEquipment => "fitness_equipment".to_string(),
            TimerTrigger::UnknownVariant(value) => format!("unknown_variant_{}", *value),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum FitnessEquipmentState {
    Ready,
    InUse,
    Paused,
    Unknown, // lost connection to fitness equipment
    UnknownVariant(u8),
}
impl FitnessEquipmentState {
    pub fn from_u8(value: u8) -> FitnessEquipmentState {
        match value {
            0 => FitnessEquipmentState::Ready,
            1 => FitnessEquipmentState::InUse,
            2 => FitnessEquipmentState::Paused,
            3 => FitnessEquipmentState::Unknown,
            _ => FitnessEquipmentState::UnknownVariant(value),
        }
    }
    pub fn from_i64(value: i64) -> FitnessEquipmentState {
        FitnessEquipmentState::from_u8(value as u8)
    }
    pub fn as_u8(&self) -> u8 {
        match &self {
            FitnessEquipmentState::Ready => 0,
            FitnessEquipmentState::InUse => 1,
            FitnessEquipmentState::Paused => 2,
            FitnessEquipmentState::Unknown => 3,
            FitnessEquipmentState::UnknownVariant(value) => *value,
        }
    }
    pub fn to_string(&self) -> String {
        match &self {
            FitnessEquipmentState::Ready => "ready".to_string(),
            FitnessEquipmentState::InUse => "in_use".to_string(),
            FitnessEquipmentState::Paused => "paused".to_string(),
            FitnessEquipmentState::Unknown => "unknown".to_string(),
            FitnessEquipmentState::UnknownVariant(value) => format!("unknown_variant_{}", *value),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Tone {
    Off,
    Tone,
    Vibrate,
    ToneAndVibrate,
    UnknownVariant(u8),
}
impl Tone {
    pub fn from_u8(value: u8) -> Tone {
        match value {
            0 => Tone::Off,
            1 => Tone::Tone,
            2 => Tone::Vibrate,
            3 => Tone::ToneAndVibrate,
            _ => Tone::UnknownVariant(value),
        }
    }
    pub fn from_i64(value: i64) -> Tone {
        Tone::from_u8(value as u8)
    }
    pub fn as_u8(&self) -> u8 {
        match &self {
            Tone::Off => 0,
            Tone::Tone => 1,
            Tone::Vibrate => 2,
            Tone::ToneAndVibrate => 3,
            Tone::UnknownVariant(value) => *value,
        }
    }
    pub fn to_string(&self) -> String {
        match &self {
            Tone::Off => "off".to_string(),
            Tone::Tone => "tone".to_string(),
            Tone::Vibrate => "vibrate".to_string(),
            Tone::ToneAndVibrate => "tone_and_vibrate".to_string(),
            Tone::UnknownVariant(value) => format!("unknown_variant_{}", *value),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Autoscroll {
    None,
    Slow,
    Medium,
    Fast,
    UnknownVariant(u8),
}
impl Autoscroll {
    pub fn from_u8(value: u8) -> Autoscroll {
        match value {
            0 => Autoscroll::None,
            1 => Autoscroll::Slow,
            2 => Autoscroll::Medium,
            3 => Autoscroll::Fast,
            _ => Autoscroll::UnknownVariant(value),
        }
    }
    pub fn from_i64(value: i64) -> Autoscroll {
        Autoscroll::from_u8(value as u8)
    }
    pub fn as_u8(&self) -> u8 {
        match &self {
            Autoscroll::None => 0,
            Autoscroll::Slow => 1,
            Autoscroll::Medium => 2,
            Autoscroll::Fast => 3,
            Autoscroll::UnknownVariant(value) => *value,
        }
    }
    pub fn to_string(&self) -> String {
        match &self {
            Autoscroll::None => "none".to_string(),
            Autoscroll::Slow => "slow".to_string(),
            Autoscroll::Medium => "medium".to_string(),
            Autoscroll::Fast => "fast".to_string(),
            Autoscroll::UnknownVariant(value) => format!("unknown_variant_{}", *value),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum ActivityClass {
    LevelMax,
    Level, // 0 to 100
    Athlete,
    UnknownVariant(u8),
}
impl ActivityClass {
    pub fn from_u8(value: u8) -> ActivityClass {
        match value {
            100 => ActivityClass::LevelMax,
            127 => ActivityClass::Level,
            128 => ActivityClass::Athlete,
            _ => ActivityClass::UnknownVariant(value),
        }
    }
    pub fn from_i64(value: i64) -> ActivityClass {
        ActivityClass::from_u8(value as u8)
    }
    pub fn as_u8(&self) -> u8 {
        match &self {
            ActivityClass::LevelMax => 100,
            ActivityClass::Level => 127,
            ActivityClass::Athlete => 128,
            ActivityClass::UnknownVariant(value) => *value,
        }
    }
    pub fn to_string(&self) -> String {
        match &self {
            ActivityClass::LevelMax => "level_max".to_string(),
            ActivityClass::Level => "level".to_string(),
            ActivityClass::Athlete => "athlete".to_string(),
            ActivityClass::UnknownVariant(value) => format!("unknown_variant_{}", *value),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum HrZoneCalc {
    Custom,
    PercentMaxHr,
    PercentHrr,
    UnknownVariant(u8),
}
impl HrZoneCalc {
    pub fn from_u8(value: u8) -> HrZoneCalc {
        match value {
            0 => HrZoneCalc::Custom,
            1 => HrZoneCalc::PercentMaxHr,
            2 => HrZoneCalc::PercentHrr,
            _ => HrZoneCalc::UnknownVariant(value),
        }
    }
    pub fn from_i64(value: i64) -> HrZoneCalc {
        HrZoneCalc::from_u8(value as u8)
    }
    pub fn as_u8(&self) -> u8 {
        match &self {
            HrZoneCalc::Custom => 0,
            HrZoneCalc::PercentMaxHr => 1,
            HrZoneCalc::PercentHrr => 2,
            HrZoneCalc::UnknownVariant(value) => *value,
        }
    }
    pub fn to_string(&self) -> String {
        match &self {
            HrZoneCalc::Custom => "custom".to_string(),
            HrZoneCalc::PercentMaxHr => "percent_max_hr".to_string(),
            HrZoneCalc::PercentHrr => "percent_hrr".to_string(),
            HrZoneCalc::UnknownVariant(value) => format!("unknown_variant_{}", *value),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum PwrZoneCalc {
    Custom,
    PercentFtp,
    UnknownVariant(u8),
}
impl PwrZoneCalc {
    pub fn from_u8(value: u8) -> PwrZoneCalc {
        match value {
            0 => PwrZoneCalc::Custom,
            1 => PwrZoneCalc::PercentFtp,
            _ => PwrZoneCalc::UnknownVariant(value),
        }
    }
    pub fn from_i64(value: i64) -> PwrZoneCalc {
        PwrZoneCalc::from_u8(value as u8)
    }
    pub fn as_u8(&self) -> u8 {
        match &self {
            PwrZoneCalc::Custom => 0,
            PwrZoneCalc::PercentFtp => 1,
            PwrZoneCalc::UnknownVariant(value) => *value,
        }
    }
    pub fn to_string(&self) -> String {
        match &self {
            PwrZoneCalc::Custom => "custom".to_string(),
            PwrZoneCalc::PercentFtp => "percent_ftp".to_string(),
            PwrZoneCalc::UnknownVariant(value) => format!("unknown_variant_{}", *value),
        }
    }
}

#[derive(Clone, Copy, Debug)]
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
    pub fn from_u8(value: u8) -> WktStepDuration {
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
    pub fn from_i64(value: i64) -> WktStepDuration {
        WktStepDuration::from_u8(value as u8)
    }
    pub fn as_u8(&self) -> u8 {
        match &self {
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
            WktStepDuration::UnknownVariant(value) => *value,
        }
    }
    pub fn to_string(&self) -> String {
        match &self {
            WktStepDuration::Time => "time".to_string(),
            WktStepDuration::Distance => "distance".to_string(),
            WktStepDuration::HrLessThan => "hr_less_than".to_string(),
            WktStepDuration::HrGreaterThan => "hr_greater_than".to_string(),
            WktStepDuration::Calories => "calories".to_string(),
            WktStepDuration::Open => "open".to_string(),
            WktStepDuration::RepeatUntilStepsCmplt => "repeat_until_steps_cmplt".to_string(),
            WktStepDuration::RepeatUntilTime => "repeat_until_time".to_string(),
            WktStepDuration::RepeatUntilDistance => "repeat_until_distance".to_string(),
            WktStepDuration::RepeatUntilCalories => "repeat_until_calories".to_string(),
            WktStepDuration::RepeatUntilHrLessThan => "repeat_until_hr_less_than".to_string(),
            WktStepDuration::RepeatUntilHrGreaterThan => "repeat_until_hr_greater_than".to_string(),
            WktStepDuration::RepeatUntilPowerLessThan => "repeat_until_power_less_than".to_string(),
            WktStepDuration::RepeatUntilPowerGreaterThan => {
                "repeat_until_power_greater_than".to_string()
            }
            WktStepDuration::PowerLessThan => "power_less_than".to_string(),
            WktStepDuration::PowerGreaterThan => "power_greater_than".to_string(),
            WktStepDuration::TrainingPeaksTss => "training_peaks_tss".to_string(),
            WktStepDuration::RepeatUntilPowerLastLapLessThan => {
                "repeat_until_power_last_lap_less_than".to_string()
            }
            WktStepDuration::RepeatUntilMaxPowerLastLapLessThan => {
                "repeat_until_max_power_last_lap_less_than".to_string()
            }
            WktStepDuration::Power3sLessThan => "power_3s_less_than".to_string(),
            WktStepDuration::Power10sLessThan => "power_10s_less_than".to_string(),
            WktStepDuration::Power30sLessThan => "power_30s_less_than".to_string(),
            WktStepDuration::Power3sGreaterThan => "power_3s_greater_than".to_string(),
            WktStepDuration::Power10sGreaterThan => "power_10s_greater_than".to_string(),
            WktStepDuration::Power30sGreaterThan => "power_30s_greater_than".to_string(),
            WktStepDuration::PowerLapLessThan => "power_lap_less_than".to_string(),
            WktStepDuration::PowerLapGreaterThan => "power_lap_greater_than".to_string(),
            WktStepDuration::RepeatUntilTrainingPeaksTss => {
                "repeat_until_training_peaks_tss".to_string()
            }
            WktStepDuration::RepetitionTime => "repetition_time".to_string(),
            WktStepDuration::Reps => "reps".to_string(),
            WktStepDuration::UnknownVariant(value) => format!("unknown_variant_{}", *value),
        }
    }
}

#[derive(Clone, Copy, Debug)]
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
    pub fn from_u8(value: u8) -> WktStepTarget {
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
    pub fn from_i64(value: i64) -> WktStepTarget {
        WktStepTarget::from_u8(value as u8)
    }
    pub fn as_u8(&self) -> u8 {
        match &self {
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
            WktStepTarget::UnknownVariant(value) => *value,
        }
    }
    pub fn to_string(&self) -> String {
        match &self {
            WktStepTarget::Speed => "speed".to_string(),
            WktStepTarget::HeartRate => "heart_rate".to_string(),
            WktStepTarget::Open => "open".to_string(),
            WktStepTarget::Cadence => "cadence".to_string(),
            WktStepTarget::Power => "power".to_string(),
            WktStepTarget::Grade => "grade".to_string(),
            WktStepTarget::Resistance => "resistance".to_string(),
            WktStepTarget::Power3s => "power_3s".to_string(),
            WktStepTarget::Power10s => "power_10s".to_string(),
            WktStepTarget::Power30s => "power_30s".to_string(),
            WktStepTarget::PowerLap => "power_lap".to_string(),
            WktStepTarget::SwimStroke => "swim_stroke".to_string(),
            WktStepTarget::SpeedLap => "speed_lap".to_string(),
            WktStepTarget::HeartRateLap => "heart_rate_lap".to_string(),
            WktStepTarget::UnknownVariant(value) => format!("unknown_variant_{}", *value),
        }
    }
}

#[derive(Clone, Copy, Debug)]
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
    pub fn from_u8(value: u8) -> Goal {
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
    pub fn from_i64(value: i64) -> Goal {
        Goal::from_u8(value as u8)
    }
    pub fn as_u8(&self) -> u8 {
        match &self {
            Goal::Time => 0,
            Goal::Distance => 1,
            Goal::Calories => 2,
            Goal::Frequency => 3,
            Goal::Steps => 4,
            Goal::Ascent => 5,
            Goal::ActiveMinutes => 6,
            Goal::UnknownVariant(value) => *value,
        }
    }
    pub fn to_string(&self) -> String {
        match &self {
            Goal::Time => "time".to_string(),
            Goal::Distance => "distance".to_string(),
            Goal::Calories => "calories".to_string(),
            Goal::Frequency => "frequency".to_string(),
            Goal::Steps => "steps".to_string(),
            Goal::Ascent => "ascent".to_string(),
            Goal::ActiveMinutes => "active_minutes".to_string(),
            Goal::UnknownVariant(value) => format!("unknown_variant_{}", *value),
        }
    }
}

#[derive(Clone, Copy, Debug)]
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
    pub fn from_u8(value: u8) -> GoalRecurrence {
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
    pub fn from_i64(value: i64) -> GoalRecurrence {
        GoalRecurrence::from_u8(value as u8)
    }
    pub fn as_u8(&self) -> u8 {
        match &self {
            GoalRecurrence::Off => 0,
            GoalRecurrence::Daily => 1,
            GoalRecurrence::Weekly => 2,
            GoalRecurrence::Monthly => 3,
            GoalRecurrence::Yearly => 4,
            GoalRecurrence::Custom => 5,
            GoalRecurrence::UnknownVariant(value) => *value,
        }
    }
    pub fn to_string(&self) -> String {
        match &self {
            GoalRecurrence::Off => "off".to_string(),
            GoalRecurrence::Daily => "daily".to_string(),
            GoalRecurrence::Weekly => "weekly".to_string(),
            GoalRecurrence::Monthly => "monthly".to_string(),
            GoalRecurrence::Yearly => "yearly".to_string(),
            GoalRecurrence::Custom => "custom".to_string(),
            GoalRecurrence::UnknownVariant(value) => format!("unknown_variant_{}", *value),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum GoalSource {
    Auto,      // Device generated
    Community, // Social network sourced goal
    User,      // Manually generated
    UnknownVariant(u8),
}
impl GoalSource {
    pub fn from_u8(value: u8) -> GoalSource {
        match value {
            0 => GoalSource::Auto,
            1 => GoalSource::Community,
            2 => GoalSource::User,
            _ => GoalSource::UnknownVariant(value),
        }
    }
    pub fn from_i64(value: i64) -> GoalSource {
        GoalSource::from_u8(value as u8)
    }
    pub fn as_u8(&self) -> u8 {
        match &self {
            GoalSource::Auto => 0,
            GoalSource::Community => 1,
            GoalSource::User => 2,
            GoalSource::UnknownVariant(value) => *value,
        }
    }
    pub fn to_string(&self) -> String {
        match &self {
            GoalSource::Auto => "auto".to_string(),
            GoalSource::Community => "community".to_string(),
            GoalSource::User => "user".to_string(),
            GoalSource::UnknownVariant(value) => format!("unknown_variant_{}", *value),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Schedule {
    Workout,
    Course,
    UnknownVariant(u8),
}
impl Schedule {
    pub fn from_u8(value: u8) -> Schedule {
        match value {
            0 => Schedule::Workout,
            1 => Schedule::Course,
            _ => Schedule::UnknownVariant(value),
        }
    }
    pub fn from_i64(value: i64) -> Schedule {
        Schedule::from_u8(value as u8)
    }
    pub fn as_u8(&self) -> u8 {
        match &self {
            Schedule::Workout => 0,
            Schedule::Course => 1,
            Schedule::UnknownVariant(value) => *value,
        }
    }
    pub fn to_string(&self) -> String {
        match &self {
            Schedule::Workout => "workout".to_string(),
            Schedule::Course => "course".to_string(),
            Schedule::UnknownVariant(value) => format!("unknown_variant_{}", *value),
        }
    }
}

#[derive(Clone, Copy, Debug)]
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
    pub fn from_u8(value: u8) -> CoursePoint {
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
    pub fn from_i64(value: i64) -> CoursePoint {
        CoursePoint::from_u8(value as u8)
    }
    pub fn as_u8(&self) -> u8 {
        match &self {
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
            CoursePoint::UnknownVariant(value) => *value,
        }
    }
    pub fn to_string(&self) -> String {
        match &self {
            CoursePoint::Generic => "generic".to_string(),
            CoursePoint::Summit => "summit".to_string(),
            CoursePoint::Valley => "valley".to_string(),
            CoursePoint::Water => "water".to_string(),
            CoursePoint::Food => "food".to_string(),
            CoursePoint::Danger => "danger".to_string(),
            CoursePoint::Left => "left".to_string(),
            CoursePoint::Right => "right".to_string(),
            CoursePoint::Straight => "straight".to_string(),
            CoursePoint::FirstAid => "first_aid".to_string(),
            CoursePoint::FourthCategory => "fourth_category".to_string(),
            CoursePoint::ThirdCategory => "third_category".to_string(),
            CoursePoint::SecondCategory => "second_category".to_string(),
            CoursePoint::FirstCategory => "first_category".to_string(),
            CoursePoint::HorsCategory => "hors_category".to_string(),
            CoursePoint::Sprint => "sprint".to_string(),
            CoursePoint::LeftFork => "left_fork".to_string(),
            CoursePoint::RightFork => "right_fork".to_string(),
            CoursePoint::MiddleFork => "middle_fork".to_string(),
            CoursePoint::SlightLeft => "slight_left".to_string(),
            CoursePoint::SharpLeft => "sharp_left".to_string(),
            CoursePoint::SlightRight => "slight_right".to_string(),
            CoursePoint::SharpRight => "sharp_right".to_string(),
            CoursePoint::UTurn => "u_turn".to_string(),
            CoursePoint::SegmentStart => "segment_start".to_string(),
            CoursePoint::SegmentEnd => "segment_end".to_string(),
            CoursePoint::UnknownVariant(value) => format!("unknown_variant_{}", *value),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Manufacturer {
    Garmin,
    GarminFr405Antfs, // Do not use.  Used by FR405 for ANTFS man id.
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
    Icg, // Indoorcycling Group
    MiPulse,
    BsxAthletics,
    Look,
    CampagnoloSrl,
    BodyBikeSmart,
    Praxisworks,
    LimitsTechnology,    // Limits Technology Ltd.
    TopactionTechnology, // TopAction Technology Inc.
    Cosinuss,
    Fitcare,
    Magene,
    GiantManufacturingCo,
    Tigrasport, // Tigrasport
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
    Precor, // Amer Sports
    Bryton,
    Sram,
    Navman, // MiTAC Global Corporation (Mio Technology)
    Cobi,   // COBI GmbH
    Spivi,
    MioMagellan,
    Evesports,
    SensitivusGauge,
    Podoon,
    LifeTimeFitness,
    FalcoEMotors, // Falco eMotors Inc.
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
    pub fn from_u16(value: u16) -> Manufacturer {
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
    pub fn from_i64(value: i64) -> Manufacturer {
        Manufacturer::from_u16(value as u16)
    }
    pub fn as_u16(&self) -> u16 {
        match &self {
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
            Manufacturer::UnknownVariant(value) => *value,
        }
    }
    pub fn to_string(&self) -> String {
        match &self {
            Manufacturer::Garmin => "garmin".to_string(),
            Manufacturer::GarminFr405Antfs => "garmin_fr405_antfs".to_string(),
            Manufacturer::Zephyr => "zephyr".to_string(),
            Manufacturer::Dayton => "dayton".to_string(),
            Manufacturer::Idt => "idt".to_string(),
            Manufacturer::Srm => "srm".to_string(),
            Manufacturer::Quarq => "quarq".to_string(),
            Manufacturer::Ibike => "ibike".to_string(),
            Manufacturer::Saris => "saris".to_string(),
            Manufacturer::SparkHk => "spark_hk".to_string(),
            Manufacturer::Tanita => "tanita".to_string(),
            Manufacturer::Echowell => "echowell".to_string(),
            Manufacturer::DynastreamOem => "dynastream_oem".to_string(),
            Manufacturer::Nautilus => "nautilus".to_string(),
            Manufacturer::Dynastream => "dynastream".to_string(),
            Manufacturer::Timex => "timex".to_string(),
            Manufacturer::Metrigear => "metrigear".to_string(),
            Manufacturer::Xelic => "xelic".to_string(),
            Manufacturer::Beurer => "beurer".to_string(),
            Manufacturer::Cardiosport => "cardiosport".to_string(),
            Manufacturer::AAndD => "a_and_d".to_string(),
            Manufacturer::Hmm => "hmm".to_string(),
            Manufacturer::Suunto => "suunto".to_string(),
            Manufacturer::ThitaElektronik => "thita_elektronik".to_string(),
            Manufacturer::Gpulse => "gpulse".to_string(),
            Manufacturer::CleanMobile => "clean_mobile".to_string(),
            Manufacturer::PedalBrain => "pedal_brain".to_string(),
            Manufacturer::Peaksware => "peaksware".to_string(),
            Manufacturer::Saxonar => "saxonar".to_string(),
            Manufacturer::LemondFitness => "lemond_fitness".to_string(),
            Manufacturer::Dexcom => "dexcom".to_string(),
            Manufacturer::WahooFitness => "wahoo_fitness".to_string(),
            Manufacturer::OctaneFitness => "octane_fitness".to_string(),
            Manufacturer::Archinoetics => "archinoetics".to_string(),
            Manufacturer::TheHurtBox => "the_hurt_box".to_string(),
            Manufacturer::CitizenSystems => "citizen_systems".to_string(),
            Manufacturer::Magellan => "magellan".to_string(),
            Manufacturer::Osynce => "osynce".to_string(),
            Manufacturer::Holux => "holux".to_string(),
            Manufacturer::Concept2 => "concept2".to_string(),
            Manufacturer::OneGiantLeap => "one_giant_leap".to_string(),
            Manufacturer::AceSensor => "ace_sensor".to_string(),
            Manufacturer::BrimBrothers => "brim_brothers".to_string(),
            Manufacturer::Xplova => "xplova".to_string(),
            Manufacturer::PerceptionDigital => "perception_digital".to_string(),
            Manufacturer::Bf1systems => "bf1systems".to_string(),
            Manufacturer::Pioneer => "pioneer".to_string(),
            Manufacturer::Spantec => "spantec".to_string(),
            Manufacturer::Metalogics => "metalogics".to_string(),
            Manufacturer::Name4iiiis => "4iiiis".to_string(),
            Manufacturer::SeikoEpson => "seiko_epson".to_string(),
            Manufacturer::SeikoEpsonOem => "seiko_epson_oem".to_string(),
            Manufacturer::IforPowell => "ifor_powell".to_string(),
            Manufacturer::MaxwellGuider => "maxwell_guider".to_string(),
            Manufacturer::StarTrac => "star_trac".to_string(),
            Manufacturer::Breakaway => "breakaway".to_string(),
            Manufacturer::AlatechTechnologyLtd => "alatech_technology_ltd".to_string(),
            Manufacturer::MioTechnologyEurope => "mio_technology_europe".to_string(),
            Manufacturer::Rotor => "rotor".to_string(),
            Manufacturer::Geonaute => "geonaute".to_string(),
            Manufacturer::IdBike => "id_bike".to_string(),
            Manufacturer::Specialized => "specialized".to_string(),
            Manufacturer::Wtek => "wtek".to_string(),
            Manufacturer::PhysicalEnterprises => "physical_enterprises".to_string(),
            Manufacturer::NorthPoleEngineering => "north_pole_engineering".to_string(),
            Manufacturer::Bkool => "bkool".to_string(),
            Manufacturer::Cateye => "cateye".to_string(),
            Manufacturer::StagesCycling => "stages_cycling".to_string(),
            Manufacturer::Sigmasport => "sigmasport".to_string(),
            Manufacturer::Tomtom => "tomtom".to_string(),
            Manufacturer::Peripedal => "peripedal".to_string(),
            Manufacturer::Wattbike => "wattbike".to_string(),
            Manufacturer::Moxy => "moxy".to_string(),
            Manufacturer::Ciclosport => "ciclosport".to_string(),
            Manufacturer::Powerbahn => "powerbahn".to_string(),
            Manufacturer::AcornProjectsAps => "acorn_projects_aps".to_string(),
            Manufacturer::Lifebeam => "lifebeam".to_string(),
            Manufacturer::Bontrager => "bontrager".to_string(),
            Manufacturer::Wellgo => "wellgo".to_string(),
            Manufacturer::Scosche => "scosche".to_string(),
            Manufacturer::Magura => "magura".to_string(),
            Manufacturer::Woodway => "woodway".to_string(),
            Manufacturer::Elite => "elite".to_string(),
            Manufacturer::NielsenKellerman => "nielsen_kellerman".to_string(),
            Manufacturer::DkCity => "dk_city".to_string(),
            Manufacturer::Tacx => "tacx".to_string(),
            Manufacturer::DirectionTechnology => "direction_technology".to_string(),
            Manufacturer::Magtonic => "magtonic".to_string(),
            Manufacturer::Name1partcarbon => "1partcarbon".to_string(),
            Manufacturer::InsideRideTechnologies => "inside_ride_technologies".to_string(),
            Manufacturer::SoundOfMotion => "sound_of_motion".to_string(),
            Manufacturer::Stryd => "stryd".to_string(),
            Manufacturer::Icg => "icg".to_string(),
            Manufacturer::MiPulse => "MiPulse".to_string(),
            Manufacturer::BsxAthletics => "bsx_athletics".to_string(),
            Manufacturer::Look => "look".to_string(),
            Manufacturer::CampagnoloSrl => "campagnolo_srl".to_string(),
            Manufacturer::BodyBikeSmart => "body_bike_smart".to_string(),
            Manufacturer::Praxisworks => "praxisworks".to_string(),
            Manufacturer::LimitsTechnology => "limits_technology".to_string(),
            Manufacturer::TopactionTechnology => "topaction_technology".to_string(),
            Manufacturer::Cosinuss => "cosinuss".to_string(),
            Manufacturer::Fitcare => "fitcare".to_string(),
            Manufacturer::Magene => "magene".to_string(),
            Manufacturer::GiantManufacturingCo => "giant_manufacturing_co".to_string(),
            Manufacturer::Tigrasport => "tigrasport".to_string(),
            Manufacturer::Salutron => "salutron".to_string(),
            Manufacturer::Technogym => "technogym".to_string(),
            Manufacturer::BrytonSensors => "bryton_sensors".to_string(),
            Manufacturer::LatitudeLimited => "latitude_limited".to_string(),
            Manufacturer::SoaringTechnology => "soaring_technology".to_string(),
            Manufacturer::Igpsport => "igpsport".to_string(),
            Manufacturer::Thinkrider => "thinkrider".to_string(),
            Manufacturer::GopherSport => "gopher_sport".to_string(),
            Manufacturer::Waterrower => "waterrower".to_string(),
            Manufacturer::Orangetheory => "orangetheory".to_string(),
            Manufacturer::Inpeak => "inpeak".to_string(),
            Manufacturer::Kinetic => "kinetic".to_string(),
            Manufacturer::JohnsonHealthTech => "johnson_health_tech".to_string(),
            Manufacturer::PolarElectro => "polar_electro".to_string(),
            Manufacturer::Seesense => "seesense".to_string(),
            Manufacturer::NciTechnology => "nci_technology".to_string(),
            Manufacturer::Iqsquare => "iqsquare".to_string(),
            Manufacturer::Leomo => "leomo".to_string(),
            Manufacturer::IfitCom => "ifit_com".to_string(),
            Manufacturer::CorosByte => "coros_byte".to_string(),
            Manufacturer::VersaDesign => "versa_design".to_string(),
            Manufacturer::Chileaf => "chileaf".to_string(),
            Manufacturer::Development => "development".to_string(),
            Manufacturer::Healthandlife => "healthandlife".to_string(),
            Manufacturer::Lezyne => "lezyne".to_string(),
            Manufacturer::ScribeLabs => "scribe_labs".to_string(),
            Manufacturer::Zwift => "zwift".to_string(),
            Manufacturer::Watteam => "watteam".to_string(),
            Manufacturer::Recon => "recon".to_string(),
            Manufacturer::FaveroElectronics => "favero_electronics".to_string(),
            Manufacturer::Dynovelo => "dynovelo".to_string(),
            Manufacturer::Strava => "strava".to_string(),
            Manufacturer::Precor => "precor".to_string(),
            Manufacturer::Bryton => "bryton".to_string(),
            Manufacturer::Sram => "sram".to_string(),
            Manufacturer::Navman => "navman".to_string(),
            Manufacturer::Cobi => "cobi".to_string(),
            Manufacturer::Spivi => "spivi".to_string(),
            Manufacturer::MioMagellan => "mio_magellan".to_string(),
            Manufacturer::Evesports => "evesports".to_string(),
            Manufacturer::SensitivusGauge => "sensitivus_gauge".to_string(),
            Manufacturer::Podoon => "podoon".to_string(),
            Manufacturer::LifeTimeFitness => "life_time_fitness".to_string(),
            Manufacturer::FalcoEMotors => "falco_e_motors".to_string(),
            Manufacturer::Minoura => "minoura".to_string(),
            Manufacturer::Cycliq => "cycliq".to_string(),
            Manufacturer::Luxottica => "luxottica".to_string(),
            Manufacturer::TrainerRoad => "trainer_road".to_string(),
            Manufacturer::TheSufferfest => "the_sufferfest".to_string(),
            Manufacturer::Fullspeedahead => "fullspeedahead".to_string(),
            Manufacturer::Virtualtraining => "virtualtraining".to_string(),
            Manufacturer::Feedbacksports => "feedbacksports".to_string(),
            Manufacturer::Omata => "omata".to_string(),
            Manufacturer::Vdo => "vdo".to_string(),
            Manufacturer::Magneticdays => "magneticdays".to_string(),
            Manufacturer::Hammerhead => "hammerhead".to_string(),
            Manufacturer::KineticByKurt => "kinetic_by_kurt".to_string(),
            Manufacturer::Shapelog => "shapelog".to_string(),
            Manufacturer::Dabuziduo => "dabuziduo".to_string(),
            Manufacturer::Jetblack => "jetblack".to_string(),
            Manufacturer::Coros => "coros".to_string(),
            Manufacturer::Virtugo => "virtugo".to_string(),
            Manufacturer::Velosense => "velosense".to_string(),
            Manufacturer::Cycligentinc => "cycligentinc".to_string(),
            Manufacturer::Trailforks => "trailforks".to_string(),
            Manufacturer::MahleEbikemotion => "mahle_ebikemotion".to_string(),
            Manufacturer::Nurvv => "nurvv".to_string(),
            Manufacturer::Microprogram => "microprogram".to_string(),
            Manufacturer::Actigraphcorp => "actigraphcorp".to_string(),
            Manufacturer::UnknownVariant(value) => format!("unknown_variant_{}", *value),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum GarminProduct {
    Hrm1,
    Axh01, // AXH01 HRM chipset
    Axb01,
    Axb02,
    Hrm2ss,
    DsiAlf02,
    Hrm3ss,
    HrmRunSingleByteProductId, // hrm_run model for HRM ANT+ messaging
    Bsm,                       // BSM model for ANT+ messaging
    Bcm,                       // BCM model for ANT+ messaging
    Axs01,                     // AXS01 HRM Bike Chipset model for ANT+ messaging
    HrmTriSingleByteProductId, // hrm_tri model for HRM ANT+ messaging
    Fr225SingleByteProductId,  // fr225 model for HRM ANT+ messaging
    Fr301China,
    Fr301Japan,
    Fr301Korea,
    Fr301Taiwan,
    Fr405, // Forerunner 405
    Fr50,  // Forerunner 50
    Fr405Japan,
    Fr60, // Forerunner 60
    DsiAlf01,
    Fr310xt, // Forerunner 310
    Edge500,
    Fr110, // Forerunner 110
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
    EdgeTouring, // Also Edge Touring Plus
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
    VariaUt800, // Varia UT 800 SW
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
    HrmDual, // HRM-Dual
    ApproachS40,
    Sdm4, // SDM4 footpod
    EdgeRemote,
    TrainingCenter,
    ConnectiqSimulator,
    AndroidAntplusPlugin,
    Connect, // Garmin Connect website
    UnknownVariant(u16),
}
impl GarminProduct {
    pub fn from_u16(value: u16) -> GarminProduct {
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
    pub fn from_i64(value: i64) -> GarminProduct {
        GarminProduct::from_u16(value as u16)
    }
    pub fn as_u16(&self) -> u16 {
        match &self {
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
            GarminProduct::UnknownVariant(value) => *value,
        }
    }
    pub fn to_string(&self) -> String {
        match &self {
            GarminProduct::Hrm1 => "hrm1".to_string(),
            GarminProduct::Axh01 => "axh01".to_string(),
            GarminProduct::Axb01 => "axb01".to_string(),
            GarminProduct::Axb02 => "axb02".to_string(),
            GarminProduct::Hrm2ss => "hrm2ss".to_string(),
            GarminProduct::DsiAlf02 => "dsi_alf02".to_string(),
            GarminProduct::Hrm3ss => "hrm3ss".to_string(),
            GarminProduct::HrmRunSingleByteProductId => {
                "hrm_run_single_byte_product_id".to_string()
            }
            GarminProduct::Bsm => "bsm".to_string(),
            GarminProduct::Bcm => "bcm".to_string(),
            GarminProduct::Axs01 => "axs01".to_string(),
            GarminProduct::HrmTriSingleByteProductId => {
                "hrm_tri_single_byte_product_id".to_string()
            }
            GarminProduct::Fr225SingleByteProductId => "fr225_single_byte_product_id".to_string(),
            GarminProduct::Fr301China => "fr301_china".to_string(),
            GarminProduct::Fr301Japan => "fr301_japan".to_string(),
            GarminProduct::Fr301Korea => "fr301_korea".to_string(),
            GarminProduct::Fr301Taiwan => "fr301_taiwan".to_string(),
            GarminProduct::Fr405 => "fr405".to_string(),
            GarminProduct::Fr50 => "fr50".to_string(),
            GarminProduct::Fr405Japan => "fr405_japan".to_string(),
            GarminProduct::Fr60 => "fr60".to_string(),
            GarminProduct::DsiAlf01 => "dsi_alf01".to_string(),
            GarminProduct::Fr310xt => "fr310xt".to_string(),
            GarminProduct::Edge500 => "edge500".to_string(),
            GarminProduct::Fr110 => "fr110".to_string(),
            GarminProduct::Edge800 => "edge800".to_string(),
            GarminProduct::Edge500Taiwan => "edge500_taiwan".to_string(),
            GarminProduct::Edge500Japan => "edge500_japan".to_string(),
            GarminProduct::Chirp => "chirp".to_string(),
            GarminProduct::Fr110Japan => "fr110_japan".to_string(),
            GarminProduct::Edge200 => "edge200".to_string(),
            GarminProduct::Fr910xt => "fr910xt".to_string(),
            GarminProduct::Edge800Taiwan => "edge800_taiwan".to_string(),
            GarminProduct::Edge800Japan => "edge800_japan".to_string(),
            GarminProduct::Alf04 => "alf04".to_string(),
            GarminProduct::Fr610 => "fr610".to_string(),
            GarminProduct::Fr210Japan => "fr210_japan".to_string(),
            GarminProduct::VectorSs => "vector_ss".to_string(),
            GarminProduct::VectorCp => "vector_cp".to_string(),
            GarminProduct::Edge800China => "edge800_china".to_string(),
            GarminProduct::Edge500China => "edge500_china".to_string(),
            GarminProduct::Fr610Japan => "fr610_japan".to_string(),
            GarminProduct::Edge500Korea => "edge500_korea".to_string(),
            GarminProduct::Fr70 => "fr70".to_string(),
            GarminProduct::Fr310xt4t => "fr310xt_4t".to_string(),
            GarminProduct::Amx => "amx".to_string(),
            GarminProduct::Fr10 => "fr10".to_string(),
            GarminProduct::Edge800Korea => "edge800_korea".to_string(),
            GarminProduct::Swim => "swim".to_string(),
            GarminProduct::Fr910xtChina => "fr910xt_china".to_string(),
            GarminProduct::Fenix => "fenix".to_string(),
            GarminProduct::Edge200Taiwan => "edge200_taiwan".to_string(),
            GarminProduct::Edge510 => "edge510".to_string(),
            GarminProduct::Edge810 => "edge810".to_string(),
            GarminProduct::Tempe => "tempe".to_string(),
            GarminProduct::Fr910xtJapan => "fr910xt_japan".to_string(),
            GarminProduct::Fr620 => "fr620".to_string(),
            GarminProduct::Fr220 => "fr220".to_string(),
            GarminProduct::Fr910xtKorea => "fr910xt_korea".to_string(),
            GarminProduct::Fr10Japan => "fr10_japan".to_string(),
            GarminProduct::Edge810Japan => "edge810_japan".to_string(),
            GarminProduct::VirbElite => "virb_elite".to_string(),
            GarminProduct::EdgeTouring => "edge_touring".to_string(),
            GarminProduct::Edge510Japan => "edge510_japan".to_string(),
            GarminProduct::HrmTri => "hrm_tri".to_string(),
            GarminProduct::HrmRun => "hrm_run".to_string(),
            GarminProduct::Fr920xt => "fr920xt".to_string(),
            GarminProduct::Edge510Asia => "edge510_asia".to_string(),
            GarminProduct::Edge810China => "edge810_china".to_string(),
            GarminProduct::Edge810Taiwan => "edge810_taiwan".to_string(),
            GarminProduct::Edge1000 => "edge1000".to_string(),
            GarminProduct::VivoFit => "vivo_fit".to_string(),
            GarminProduct::VirbRemote => "virb_remote".to_string(),
            GarminProduct::VivoKi => "vivo_ki".to_string(),
            GarminProduct::Fr15 => "fr15".to_string(),
            GarminProduct::VivoActive => "vivo_active".to_string(),
            GarminProduct::Edge510Korea => "edge510_korea".to_string(),
            GarminProduct::Fr620Japan => "fr620_japan".to_string(),
            GarminProduct::Fr620China => "fr620_china".to_string(),
            GarminProduct::Fr220Japan => "fr220_japan".to_string(),
            GarminProduct::Fr220China => "fr220_china".to_string(),
            GarminProduct::ApproachS6 => "approach_s6".to_string(),
            GarminProduct::VivoSmart => "vivo_smart".to_string(),
            GarminProduct::Fenix2 => "fenix2".to_string(),
            GarminProduct::Epix => "epix".to_string(),
            GarminProduct::Fenix3 => "fenix3".to_string(),
            GarminProduct::Edge1000Taiwan => "edge1000_taiwan".to_string(),
            GarminProduct::Edge1000Japan => "edge1000_japan".to_string(),
            GarminProduct::Fr15Japan => "fr15_japan".to_string(),
            GarminProduct::Edge520 => "edge520".to_string(),
            GarminProduct::Edge1000China => "edge1000_china".to_string(),
            GarminProduct::Fr620Russia => "fr620_russia".to_string(),
            GarminProduct::Fr220Russia => "fr220_russia".to_string(),
            GarminProduct::VectorS => "vector_s".to_string(),
            GarminProduct::Edge1000Korea => "edge1000_korea".to_string(),
            GarminProduct::Fr920xtTaiwan => "fr920xt_taiwan".to_string(),
            GarminProduct::Fr920xtChina => "fr920xt_china".to_string(),
            GarminProduct::Fr920xtJapan => "fr920xt_japan".to_string(),
            GarminProduct::Virbx => "virbx".to_string(),
            GarminProduct::VivoSmartApac => "vivo_smart_apac".to_string(),
            GarminProduct::EtrexTouch => "etrex_touch".to_string(),
            GarminProduct::Edge25 => "edge25".to_string(),
            GarminProduct::Fr25 => "fr25".to_string(),
            GarminProduct::VivoFit2 => "vivo_fit2".to_string(),
            GarminProduct::Fr225 => "fr225".to_string(),
            GarminProduct::Fr630 => "fr630".to_string(),
            GarminProduct::Fr230 => "fr230".to_string(),
            GarminProduct::Fr735xt => "fr735xt".to_string(),
            GarminProduct::VivoActiveApac => "vivo_active_apac".to_string(),
            GarminProduct::Vector2 => "vector_2".to_string(),
            GarminProduct::Vector2s => "vector_2s".to_string(),
            GarminProduct::Virbxe => "virbxe".to_string(),
            GarminProduct::Fr620Taiwan => "fr620_taiwan".to_string(),
            GarminProduct::Fr220Taiwan => "fr220_taiwan".to_string(),
            GarminProduct::Truswing => "truswing".to_string(),
            GarminProduct::Fenix3China => "fenix3_china".to_string(),
            GarminProduct::Fenix3Twn => "fenix3_twn".to_string(),
            GarminProduct::VariaHeadlight => "varia_headlight".to_string(),
            GarminProduct::VariaTaillightOld => "varia_taillight_old".to_string(),
            GarminProduct::EdgeExplore1000 => "edge_explore_1000".to_string(),
            GarminProduct::Fr225Asia => "fr225_asia".to_string(),
            GarminProduct::VariaRadarTaillight => "varia_radar_taillight".to_string(),
            GarminProduct::VariaRadarDisplay => "varia_radar_display".to_string(),
            GarminProduct::Edge20 => "edge20".to_string(),
            GarminProduct::D2Bravo => "d2_bravo".to_string(),
            GarminProduct::ApproachS20 => "approach_s20".to_string(),
            GarminProduct::VariaRemote => "varia_remote".to_string(),
            GarminProduct::ApproachX40 => "approach_x40".to_string(),
            GarminProduct::Hrm4Run => "hrm4_run".to_string(),
            GarminProduct::VivoActiveHr => "vivo_active_hr".to_string(),
            GarminProduct::VivoSmartGpsHr => "vivo_smart_gps_hr".to_string(),
            GarminProduct::VivoSmartHr => "vivo_smart_hr".to_string(),
            GarminProduct::VivoMove => "vivo_move".to_string(),
            GarminProduct::VariaVision => "varia_vision".to_string(),
            GarminProduct::VivoFit3 => "vivo_fit3".to_string(),
            GarminProduct::Fenix3Hr => "fenix3_hr".to_string(),
            GarminProduct::VirbUltra30 => "virb_ultra_30".to_string(),
            GarminProduct::IndexSmartScale => "index_smart_scale".to_string(),
            GarminProduct::Fr235 => "fr235".to_string(),
            GarminProduct::Fenix3Chronos => "fenix3_chronos".to_string(),
            GarminProduct::Oregon7xx => "oregon7xx".to_string(),
            GarminProduct::Rino7xx => "rino7xx".to_string(),
            GarminProduct::Nautix => "nautix".to_string(),
            GarminProduct::Edge820 => "edge_820".to_string(),
            GarminProduct::EdgeExplore820 => "edge_explore_820".to_string(),
            GarminProduct::Fr735xtApac => "fr735xt_apac".to_string(),
            GarminProduct::Fr735xtJapan => "fr735xt_japan".to_string(),
            GarminProduct::Fenix5s => "fenix5s".to_string(),
            GarminProduct::D2BravoTitanium => "d2_bravo_titanium".to_string(),
            GarminProduct::VariaUt800 => "varia_ut800".to_string(),
            GarminProduct::RunningDynamicsPod => "running_dynamics_pod".to_string(),
            GarminProduct::Fenix5x => "fenix5x".to_string(),
            GarminProduct::VivoFitJr => "vivo_fit_jr".to_string(),
            GarminProduct::VivoSmart3 => "vivo_smart3".to_string(),
            GarminProduct::VivoSport => "vivo_sport".to_string(),
            GarminProduct::ApproachS60 => "approach_s60".to_string(),
            GarminProduct::Virb360 => "virb_360".to_string(),
            GarminProduct::Fr935 => "fr935".to_string(),
            GarminProduct::Fenix5 => "fenix5".to_string(),
            GarminProduct::Vivoactive3 => "vivoactive3".to_string(),
            GarminProduct::Edge1030 => "edge_1030".to_string(),
            GarminProduct::Foretrex601701 => "foretrex_601_701".to_string(),
            GarminProduct::VivoMoveHr => "vivo_move_hr".to_string(),
            GarminProduct::ApproachZ80 => "approach_z80".to_string(),
            GarminProduct::VivoSmart3Apac => "vivo_smart3_apac".to_string(),
            GarminProduct::VivoSportApac => "vivo_sport_apac".to_string(),
            GarminProduct::Descent => "descent".to_string(),
            GarminProduct::Fr645 => "fr645".to_string(),
            GarminProduct::Fr645m => "fr645m".to_string(),
            GarminProduct::Fenix5sPlus => "fenix5s_plus".to_string(),
            GarminProduct::Edge130 => "Edge_130".to_string(),
            GarminProduct::Vivosmart4 => "vivosmart_4".to_string(),
            GarminProduct::ApproachX10 => "approach_x10".to_string(),
            GarminProduct::Vivoactive3mW => "vivoactive3m_w".to_string(),
            GarminProduct::EdgeExplore => "edge_explore".to_string(),
            GarminProduct::Gpsmap66 => "gpsmap66".to_string(),
            GarminProduct::ApproachS10 => "approach_s10".to_string(),
            GarminProduct::Vivoactive3mL => "vivoactive3m_l".to_string(),
            GarminProduct::ApproachG80 => "approach_g80".to_string(),
            GarminProduct::Fenix5Plus => "fenix5_plus".to_string(),
            GarminProduct::Fenix5xPlus => "fenix5x_plus".to_string(),
            GarminProduct::Edge520Plus => "edge_520_plus".to_string(),
            GarminProduct::HrmDual => "hrm_dual".to_string(),
            GarminProduct::ApproachS40 => "approach_s40".to_string(),
            GarminProduct::Sdm4 => "sdm4".to_string(),
            GarminProduct::EdgeRemote => "edge_remote".to_string(),
            GarminProduct::TrainingCenter => "training_center".to_string(),
            GarminProduct::ConnectiqSimulator => "connectiq_simulator".to_string(),
            GarminProduct::AndroidAntplusPlugin => "android_antplus_plugin".to_string(),
            GarminProduct::Connect => "connect".to_string(),
            GarminProduct::UnknownVariant(value) => format!("unknown_variant_{}", *value),
        }
    }
}

#[derive(Clone, Copy, Debug)]
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
    pub fn from_u8(value: u8) -> AntplusDeviceType {
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
    pub fn from_i64(value: i64) -> AntplusDeviceType {
        AntplusDeviceType::from_u8(value as u8)
    }
    pub fn as_u8(&self) -> u8 {
        match &self {
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
            AntplusDeviceType::UnknownVariant(value) => *value,
        }
    }
    pub fn to_string(&self) -> String {
        match &self {
            AntplusDeviceType::Antfs => "antfs".to_string(),
            AntplusDeviceType::BikePower => "bike_power".to_string(),
            AntplusDeviceType::EnvironmentSensorLegacy => "environment_sensor_legacy".to_string(),
            AntplusDeviceType::MultiSportSpeedDistance => "multi_sport_speed_distance".to_string(),
            AntplusDeviceType::Control => "control".to_string(),
            AntplusDeviceType::FitnessEquipment => "fitness_equipment".to_string(),
            AntplusDeviceType::BloodPressure => "blood_pressure".to_string(),
            AntplusDeviceType::GeocacheNode => "geocache_node".to_string(),
            AntplusDeviceType::LightElectricVehicle => "light_electric_vehicle".to_string(),
            AntplusDeviceType::EnvSensor => "env_sensor".to_string(),
            AntplusDeviceType::Racquet => "racquet".to_string(),
            AntplusDeviceType::ControlHub => "control_hub".to_string(),
            AntplusDeviceType::MuscleOxygen => "muscle_oxygen".to_string(),
            AntplusDeviceType::BikeLightMain => "bike_light_main".to_string(),
            AntplusDeviceType::BikeLightShared => "bike_light_shared".to_string(),
            AntplusDeviceType::Exd => "exd".to_string(),
            AntplusDeviceType::BikeRadar => "bike_radar".to_string(),
            AntplusDeviceType::BikeAero => "bike_aero".to_string(),
            AntplusDeviceType::WeightScale => "weight_scale".to_string(),
            AntplusDeviceType::HeartRate => "heart_rate".to_string(),
            AntplusDeviceType::BikeSpeedCadence => "bike_speed_cadence".to_string(),
            AntplusDeviceType::BikeCadence => "bike_cadence".to_string(),
            AntplusDeviceType::BikeSpeed => "bike_speed".to_string(),
            AntplusDeviceType::StrideSpeedDistance => "stride_speed_distance".to_string(),
            AntplusDeviceType::UnknownVariant(value) => format!("unknown_variant_{}", *value),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum AntNetwork {
    Public,
    Antplus,
    Antfs,
    Private,
    UnknownVariant(u8),
}
impl AntNetwork {
    pub fn from_u8(value: u8) -> AntNetwork {
        match value {
            0 => AntNetwork::Public,
            1 => AntNetwork::Antplus,
            2 => AntNetwork::Antfs,
            3 => AntNetwork::Private,
            _ => AntNetwork::UnknownVariant(value),
        }
    }
    pub fn from_i64(value: i64) -> AntNetwork {
        AntNetwork::from_u8(value as u8)
    }
    pub fn as_u8(&self) -> u8 {
        match &self {
            AntNetwork::Public => 0,
            AntNetwork::Antplus => 1,
            AntNetwork::Antfs => 2,
            AntNetwork::Private => 3,
            AntNetwork::UnknownVariant(value) => *value,
        }
    }
    pub fn to_string(&self) -> String {
        match &self {
            AntNetwork::Public => "public".to_string(),
            AntNetwork::Antplus => "antplus".to_string(),
            AntNetwork::Antfs => "antfs".to_string(),
            AntNetwork::Private => "private".to_string(),
            AntNetwork::UnknownVariant(value) => format!("unknown_variant_{}", *value),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum WorkoutCapabilities {
    Interval,
    Custom,
    FitnessEquipment,
    Firstbeat,
    NewLeaf,
    Tcx,   // For backwards compatibility.  Watch should add missing id fields then clear flag.
    Speed, // Speed source required for workout step.
    HeartRate, // Heart rate source required for workout step.
    Distance, // Distance source required for workout step.
    Cadence, // Cadence source required for workout step.
    Power, // Power source required for workout step.
    Grade, // Grade source required for workout step.
    Resistance, // Resistance source required for workout step.
    Protected,
    UnknownVariant(u32),
}
impl WorkoutCapabilities {
    pub fn from_u32(value: u32) -> WorkoutCapabilities {
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
    pub fn from_i64(value: i64) -> WorkoutCapabilities {
        WorkoutCapabilities::from_u32(value as u32)
    }
    pub fn as_u32(&self) -> u32 {
        match &self {
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
            WorkoutCapabilities::UnknownVariant(value) => *value,
        }
    }
    pub fn to_string(&self) -> String {
        match &self {
            WorkoutCapabilities::Interval => "interval".to_string(),
            WorkoutCapabilities::Custom => "custom".to_string(),
            WorkoutCapabilities::FitnessEquipment => "fitness_equipment".to_string(),
            WorkoutCapabilities::Firstbeat => "firstbeat".to_string(),
            WorkoutCapabilities::NewLeaf => "new_leaf".to_string(),
            WorkoutCapabilities::Tcx => "tcx".to_string(),
            WorkoutCapabilities::Speed => "speed".to_string(),
            WorkoutCapabilities::HeartRate => "heart_rate".to_string(),
            WorkoutCapabilities::Distance => "distance".to_string(),
            WorkoutCapabilities::Cadence => "cadence".to_string(),
            WorkoutCapabilities::Power => "power".to_string(),
            WorkoutCapabilities::Grade => "grade".to_string(),
            WorkoutCapabilities::Resistance => "resistance".to_string(),
            WorkoutCapabilities::Protected => "protected".to_string(),
            WorkoutCapabilities::UnknownVariant(value) => format!("unknown_variant_{}", *value),
        }
    }
}

#[derive(Clone, Copy, Debug)]
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
    pub fn from_u8(value: u8) -> BatteryStatus {
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
    pub fn from_i64(value: i64) -> BatteryStatus {
        BatteryStatus::from_u8(value as u8)
    }
    pub fn as_u8(&self) -> u8 {
        match &self {
            BatteryStatus::New => 1,
            BatteryStatus::Good => 2,
            BatteryStatus::Ok => 3,
            BatteryStatus::Low => 4,
            BatteryStatus::Critical => 5,
            BatteryStatus::Charging => 6,
            BatteryStatus::Unknown => 7,
            BatteryStatus::UnknownVariant(value) => *value,
        }
    }
    pub fn to_string(&self) -> String {
        match &self {
            BatteryStatus::New => "new".to_string(),
            BatteryStatus::Good => "good".to_string(),
            BatteryStatus::Ok => "ok".to_string(),
            BatteryStatus::Low => "low".to_string(),
            BatteryStatus::Critical => "critical".to_string(),
            BatteryStatus::Charging => "charging".to_string(),
            BatteryStatus::Unknown => "unknown".to_string(),
            BatteryStatus::UnknownVariant(value) => format!("unknown_variant_{}", *value),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum HrType {
    Normal,
    Irregular,
    UnknownVariant(u8),
}
impl HrType {
    pub fn from_u8(value: u8) -> HrType {
        match value {
            0 => HrType::Normal,
            1 => HrType::Irregular,
            _ => HrType::UnknownVariant(value),
        }
    }
    pub fn from_i64(value: i64) -> HrType {
        HrType::from_u8(value as u8)
    }
    pub fn as_u8(&self) -> u8 {
        match &self {
            HrType::Normal => 0,
            HrType::Irregular => 1,
            HrType::UnknownVariant(value) => *value,
        }
    }
    pub fn to_string(&self) -> String {
        match &self {
            HrType::Normal => "normal".to_string(),
            HrType::Irregular => "irregular".to_string(),
            HrType::UnknownVariant(value) => format!("unknown_variant_{}", *value),
        }
    }
}

#[derive(Clone, Copy, Debug)]
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
    pub fn from_u32(value: u32) -> CourseCapabilities {
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
    pub fn from_i64(value: i64) -> CourseCapabilities {
        CourseCapabilities::from_u32(value as u32)
    }
    pub fn as_u32(&self) -> u32 {
        match &self {
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
            CourseCapabilities::UnknownVariant(value) => *value,
        }
    }
    pub fn to_string(&self) -> String {
        match &self {
            CourseCapabilities::Processed => "processed".to_string(),
            CourseCapabilities::Valid => "valid".to_string(),
            CourseCapabilities::Time => "time".to_string(),
            CourseCapabilities::Distance => "distance".to_string(),
            CourseCapabilities::Position => "position".to_string(),
            CourseCapabilities::HeartRate => "heart_rate".to_string(),
            CourseCapabilities::Power => "power".to_string(),
            CourseCapabilities::Cadence => "cadence".to_string(),
            CourseCapabilities::Training => "training".to_string(),
            CourseCapabilities::Navigation => "navigation".to_string(),
            CourseCapabilities::Bikeway => "bikeway".to_string(),
            CourseCapabilities::UnknownVariant(value) => format!("unknown_variant_{}", *value),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Weight {
    Calculating,
    UnknownVariant(u16),
}
impl Weight {
    pub fn from_u16(value: u16) -> Weight {
        match value {
            65534 => Weight::Calculating,
            _ => Weight::UnknownVariant(value),
        }
    }
    pub fn from_i64(value: i64) -> Weight {
        Weight::from_u16(value as u16)
    }
    pub fn as_u16(&self) -> u16 {
        match &self {
            Weight::Calculating => 65534,
            Weight::UnknownVariant(value) => *value,
        }
    }
    pub fn to_string(&self) -> String {
        match &self {
            Weight::Calculating => "calculating".to_string(),
            Weight::UnknownVariant(value) => format!("unknown_variant_{}", *value),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum WorkoutHr {
    BpmOffset,
    UnknownVariant(u32),
}
impl WorkoutHr {
    pub fn from_u32(value: u32) -> WorkoutHr {
        match value {
            100 => WorkoutHr::BpmOffset,
            _ => WorkoutHr::UnknownVariant(value),
        }
    }
    pub fn from_i64(value: i64) -> WorkoutHr {
        WorkoutHr::from_u32(value as u32)
    }
    pub fn as_u32(&self) -> u32 {
        match &self {
            WorkoutHr::BpmOffset => 100,
            WorkoutHr::UnknownVariant(value) => *value,
        }
    }
    pub fn to_string(&self) -> String {
        match &self {
            WorkoutHr::BpmOffset => "bpm_offset".to_string(),
            WorkoutHr::UnknownVariant(value) => format!("unknown_variant_{}", *value),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum WorkoutPower {
    WattsOffset,
    UnknownVariant(u32),
}
impl WorkoutPower {
    pub fn from_u32(value: u32) -> WorkoutPower {
        match value {
            1000 => WorkoutPower::WattsOffset,
            _ => WorkoutPower::UnknownVariant(value),
        }
    }
    pub fn from_i64(value: i64) -> WorkoutPower {
        WorkoutPower::from_u32(value as u32)
    }
    pub fn as_u32(&self) -> u32 {
        match &self {
            WorkoutPower::WattsOffset => 1000,
            WorkoutPower::UnknownVariant(value) => *value,
        }
    }
    pub fn to_string(&self) -> String {
        match &self {
            WorkoutPower::WattsOffset => "watts_offset".to_string(),
            WorkoutPower::UnknownVariant(value) => format!("unknown_variant_{}", *value),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum BpStatus {
    NoError,
    ErrorIncompleteData,
    ErrorNoMeasurement,
    ErrorDataOutOfRange,
    ErrorIrregularHeartRate,
    UnknownVariant(u8),
}
impl BpStatus {
    pub fn from_u8(value: u8) -> BpStatus {
        match value {
            0 => BpStatus::NoError,
            1 => BpStatus::ErrorIncompleteData,
            2 => BpStatus::ErrorNoMeasurement,
            3 => BpStatus::ErrorDataOutOfRange,
            4 => BpStatus::ErrorIrregularHeartRate,
            _ => BpStatus::UnknownVariant(value),
        }
    }
    pub fn from_i64(value: i64) -> BpStatus {
        BpStatus::from_u8(value as u8)
    }
    pub fn as_u8(&self) -> u8 {
        match &self {
            BpStatus::NoError => 0,
            BpStatus::ErrorIncompleteData => 1,
            BpStatus::ErrorNoMeasurement => 2,
            BpStatus::ErrorDataOutOfRange => 3,
            BpStatus::ErrorIrregularHeartRate => 4,
            BpStatus::UnknownVariant(value) => *value,
        }
    }
    pub fn to_string(&self) -> String {
        match &self {
            BpStatus::NoError => "no_error".to_string(),
            BpStatus::ErrorIncompleteData => "error_incomplete_data".to_string(),
            BpStatus::ErrorNoMeasurement => "error_no_measurement".to_string(),
            BpStatus::ErrorDataOutOfRange => "error_data_out_of_range".to_string(),
            BpStatus::ErrorIrregularHeartRate => "error_irregular_heart_rate".to_string(),
            BpStatus::UnknownVariant(value) => format!("unknown_variant_{}", *value),
        }
    }
}

#[derive(Clone, Copy, Debug)]
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
    pub fn from_u16(value: u16) -> UserLocalId {
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
    pub fn from_i64(value: i64) -> UserLocalId {
        UserLocalId::from_u16(value as u16)
    }
    pub fn as_u16(&self) -> u16 {
        match &self {
            UserLocalId::LocalMin => 0,
            UserLocalId::LocalMax => 15,
            UserLocalId::StationaryMin => 16,
            UserLocalId::StationaryMax => 255,
            UserLocalId::PortableMin => 256,
            UserLocalId::PortableMax => 65534,
            UserLocalId::UnknownVariant(value) => *value,
        }
    }
    pub fn to_string(&self) -> String {
        match &self {
            UserLocalId::LocalMin => "local_min".to_string(),
            UserLocalId::LocalMax => "local_max".to_string(),
            UserLocalId::StationaryMin => "stationary_min".to_string(),
            UserLocalId::StationaryMax => "stationary_max".to_string(),
            UserLocalId::PortableMin => "portable_min".to_string(),
            UserLocalId::PortableMax => "portable_max".to_string(),
            UserLocalId::UnknownVariant(value) => format!("unknown_variant_{}", *value),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum SwimStroke {
    Freestyle,
    Backstroke,
    Breaststroke,
    Butterfly,
    Drill,
    Mixed,
    Im, // IM is a mixed interval containing the same number of lengths for each of: Butterfly, Backstroke, Breaststroke, Freestyle, swam in that order.
    UnknownVariant(u8),
}
impl SwimStroke {
    pub fn from_u8(value: u8) -> SwimStroke {
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
    pub fn from_i64(value: i64) -> SwimStroke {
        SwimStroke::from_u8(value as u8)
    }
    pub fn as_u8(&self) -> u8 {
        match &self {
            SwimStroke::Freestyle => 0,
            SwimStroke::Backstroke => 1,
            SwimStroke::Breaststroke => 2,
            SwimStroke::Butterfly => 3,
            SwimStroke::Drill => 4,
            SwimStroke::Mixed => 5,
            SwimStroke::Im => 6,
            SwimStroke::UnknownVariant(value) => *value,
        }
    }
    pub fn to_string(&self) -> String {
        match &self {
            SwimStroke::Freestyle => "freestyle".to_string(),
            SwimStroke::Backstroke => "backstroke".to_string(),
            SwimStroke::Breaststroke => "breaststroke".to_string(),
            SwimStroke::Butterfly => "butterfly".to_string(),
            SwimStroke::Drill => "drill".to_string(),
            SwimStroke::Mixed => "mixed".to_string(),
            SwimStroke::Im => "im".to_string(),
            SwimStroke::UnknownVariant(value) => format!("unknown_variant_{}", *value),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum ActivityType {
    Generic,
    Running,
    Cycling,
    Transition, // Mulitsport transition
    FitnessEquipment,
    Swimming,
    Walking,
    Sedentary,
    All, // All is for goals only to include all sports.
    UnknownVariant(u8),
}
impl ActivityType {
    pub fn from_u8(value: u8) -> ActivityType {
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
    pub fn from_i64(value: i64) -> ActivityType {
        ActivityType::from_u8(value as u8)
    }
    pub fn as_u8(&self) -> u8 {
        match &self {
            ActivityType::Generic => 0,
            ActivityType::Running => 1,
            ActivityType::Cycling => 2,
            ActivityType::Transition => 3,
            ActivityType::FitnessEquipment => 4,
            ActivityType::Swimming => 5,
            ActivityType::Walking => 6,
            ActivityType::Sedentary => 8,
            ActivityType::All => 254,
            ActivityType::UnknownVariant(value) => *value,
        }
    }
    pub fn to_string(&self) -> String {
        match &self {
            ActivityType::Generic => "generic".to_string(),
            ActivityType::Running => "running".to_string(),
            ActivityType::Cycling => "cycling".to_string(),
            ActivityType::Transition => "transition".to_string(),
            ActivityType::FitnessEquipment => "fitness_equipment".to_string(),
            ActivityType::Swimming => "swimming".to_string(),
            ActivityType::Walking => "walking".to_string(),
            ActivityType::Sedentary => "sedentary".to_string(),
            ActivityType::All => "all".to_string(),
            ActivityType::UnknownVariant(value) => format!("unknown_variant_{}", *value),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum ActivitySubtype {
    Generic,
    Treadmill,     // Run
    Street,        // Run
    Trail,         // Run
    Track,         // Run
    Spin,          // Cycling
    IndoorCycling, // Cycling
    Road,          // Cycling
    Mountain,      // Cycling
    Downhill,      // Cycling
    Recumbent,     // Cycling
    Cyclocross,    // Cycling
    HandCycling,   // Cycling
    TrackCycling,  // Cycling
    IndoorRowing,  // Fitness Equipment
    Elliptical,    // Fitness Equipment
    StairClimbing, // Fitness Equipment
    LapSwimming,   // Swimming
    OpenWater,     // Swimming
    All,
    UnknownVariant(u8),
}
impl ActivitySubtype {
    pub fn from_u8(value: u8) -> ActivitySubtype {
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
    pub fn from_i64(value: i64) -> ActivitySubtype {
        ActivitySubtype::from_u8(value as u8)
    }
    pub fn as_u8(&self) -> u8 {
        match &self {
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
            ActivitySubtype::UnknownVariant(value) => *value,
        }
    }
    pub fn to_string(&self) -> String {
        match &self {
            ActivitySubtype::Generic => "generic".to_string(),
            ActivitySubtype::Treadmill => "treadmill".to_string(),
            ActivitySubtype::Street => "street".to_string(),
            ActivitySubtype::Trail => "trail".to_string(),
            ActivitySubtype::Track => "track".to_string(),
            ActivitySubtype::Spin => "spin".to_string(),
            ActivitySubtype::IndoorCycling => "indoor_cycling".to_string(),
            ActivitySubtype::Road => "road".to_string(),
            ActivitySubtype::Mountain => "mountain".to_string(),
            ActivitySubtype::Downhill => "downhill".to_string(),
            ActivitySubtype::Recumbent => "recumbent".to_string(),
            ActivitySubtype::Cyclocross => "cyclocross".to_string(),
            ActivitySubtype::HandCycling => "hand_cycling".to_string(),
            ActivitySubtype::TrackCycling => "track_cycling".to_string(),
            ActivitySubtype::IndoorRowing => "indoor_rowing".to_string(),
            ActivitySubtype::Elliptical => "elliptical".to_string(),
            ActivitySubtype::StairClimbing => "stair_climbing".to_string(),
            ActivitySubtype::LapSwimming => "lap_swimming".to_string(),
            ActivitySubtype::OpenWater => "open_water".to_string(),
            ActivitySubtype::All => "all".to_string(),
            ActivitySubtype::UnknownVariant(value) => format!("unknown_variant_{}", *value),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum ActivityLevel {
    Low,
    Medium,
    High,
    UnknownVariant(u8),
}
impl ActivityLevel {
    pub fn from_u8(value: u8) -> ActivityLevel {
        match value {
            0 => ActivityLevel::Low,
            1 => ActivityLevel::Medium,
            2 => ActivityLevel::High,
            _ => ActivityLevel::UnknownVariant(value),
        }
    }
    pub fn from_i64(value: i64) -> ActivityLevel {
        ActivityLevel::from_u8(value as u8)
    }
    pub fn as_u8(&self) -> u8 {
        match &self {
            ActivityLevel::Low => 0,
            ActivityLevel::Medium => 1,
            ActivityLevel::High => 2,
            ActivityLevel::UnknownVariant(value) => *value,
        }
    }
    pub fn to_string(&self) -> String {
        match &self {
            ActivityLevel::Low => "low".to_string(),
            ActivityLevel::Medium => "medium".to_string(),
            ActivityLevel::High => "high".to_string(),
            ActivityLevel::UnknownVariant(value) => format!("unknown_variant_{}", *value),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Side {
    Right,
    Left,
    UnknownVariant(u8),
}
impl Side {
    pub fn from_u8(value: u8) -> Side {
        match value {
            0 => Side::Right,
            1 => Side::Left,
            _ => Side::UnknownVariant(value),
        }
    }
    pub fn from_i64(value: i64) -> Side {
        Side::from_u8(value as u8)
    }
    pub fn as_u8(&self) -> u8 {
        match &self {
            Side::Right => 0,
            Side::Left => 1,
            Side::UnknownVariant(value) => *value,
        }
    }
    pub fn to_string(&self) -> String {
        match &self {
            Side::Right => "right".to_string(),
            Side::Left => "left".to_string(),
            Side::UnknownVariant(value) => format!("unknown_variant_{}", *value),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum LeftRightBalance {
    Mask,  // % contribution
    Right, // data corresponds to right if set, otherwise unknown
    UnknownVariant(u8),
}
impl LeftRightBalance {
    pub fn from_u8(value: u8) -> LeftRightBalance {
        match value {
            127 => LeftRightBalance::Mask,
            128 => LeftRightBalance::Right,
            _ => LeftRightBalance::UnknownVariant(value),
        }
    }
    pub fn from_i64(value: i64) -> LeftRightBalance {
        LeftRightBalance::from_u8(value as u8)
    }
    pub fn as_u8(&self) -> u8 {
        match &self {
            LeftRightBalance::Mask => 127,
            LeftRightBalance::Right => 128,
            LeftRightBalance::UnknownVariant(value) => *value,
        }
    }
    pub fn to_string(&self) -> String {
        match &self {
            LeftRightBalance::Mask => "mask".to_string(),
            LeftRightBalance::Right => "right".to_string(),
            LeftRightBalance::UnknownVariant(value) => format!("unknown_variant_{}", *value),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum LeftRightBalance100 {
    Mask,  // % contribution scaled by 100
    Right, // data corresponds to right if set, otherwise unknown
    UnknownVariant(u16),
}
impl LeftRightBalance100 {
    pub fn from_u16(value: u16) -> LeftRightBalance100 {
        match value {
            16383 => LeftRightBalance100::Mask,
            32768 => LeftRightBalance100::Right,
            _ => LeftRightBalance100::UnknownVariant(value),
        }
    }
    pub fn from_i64(value: i64) -> LeftRightBalance100 {
        LeftRightBalance100::from_u16(value as u16)
    }
    pub fn as_u16(&self) -> u16 {
        match &self {
            LeftRightBalance100::Mask => 16383,
            LeftRightBalance100::Right => 32768,
            LeftRightBalance100::UnknownVariant(value) => *value,
        }
    }
    pub fn to_string(&self) -> String {
        match &self {
            LeftRightBalance100::Mask => "mask".to_string(),
            LeftRightBalance100::Right => "right".to_string(),
            LeftRightBalance100::UnknownVariant(value) => format!("unknown_variant_{}", *value),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum LengthType {
    Idle,   // Rest period. Length with no strokes
    Active, // Length with strokes.
    UnknownVariant(u8),
}
impl LengthType {
    pub fn from_u8(value: u8) -> LengthType {
        match value {
            0 => LengthType::Idle,
            1 => LengthType::Active,
            _ => LengthType::UnknownVariant(value),
        }
    }
    pub fn from_i64(value: i64) -> LengthType {
        LengthType::from_u8(value as u8)
    }
    pub fn as_u8(&self) -> u8 {
        match &self {
            LengthType::Idle => 0,
            LengthType::Active => 1,
            LengthType::UnknownVariant(value) => *value,
        }
    }
    pub fn to_string(&self) -> String {
        match &self {
            LengthType::Idle => "idle".to_string(),
            LengthType::Active => "active".to_string(),
            LengthType::UnknownVariant(value) => format!("unknown_variant_{}", *value),
        }
    }
}

#[derive(Clone, Copy, Debug)]
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
    pub fn from_u8(value: u8) -> DayOfWeek {
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
    pub fn from_i64(value: i64) -> DayOfWeek {
        DayOfWeek::from_u8(value as u8)
    }
    pub fn as_u8(&self) -> u8 {
        match &self {
            DayOfWeek::Sunday => 0,
            DayOfWeek::Monday => 1,
            DayOfWeek::Tuesday => 2,
            DayOfWeek::Wednesday => 3,
            DayOfWeek::Thursday => 4,
            DayOfWeek::Friday => 5,
            DayOfWeek::Saturday => 6,
            DayOfWeek::UnknownVariant(value) => *value,
        }
    }
    pub fn to_string(&self) -> String {
        match &self {
            DayOfWeek::Sunday => "sunday".to_string(),
            DayOfWeek::Monday => "monday".to_string(),
            DayOfWeek::Tuesday => "tuesday".to_string(),
            DayOfWeek::Wednesday => "wednesday".to_string(),
            DayOfWeek::Thursday => "thursday".to_string(),
            DayOfWeek::Friday => "friday".to_string(),
            DayOfWeek::Saturday => "saturday".to_string(),
            DayOfWeek::UnknownVariant(value) => format!("unknown_variant_{}", *value),
        }
    }
}

#[derive(Clone, Copy, Debug)]
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
    DeviceInitiatesSync, // Indicates device is in control of initiating all syncs
    ConnectIqWatchAppDownload,
    ConnectIqWidgetDownload,
    ConnectIqWatchFaceDownload,
    ConnectIqDataFieldDownload,
    ConnectIqAppManagment, // Device supports delete and reorder of apps via GCM
    SwingSensor,
    SwingSensorRemote,
    IncidentDetection, // Device supports incident detection
    AudioPrompts,
    WifiVerification, // Device supports reporting wifi verification via GCM
    TrueUp,           // Device supports True Up
    FindMyWatch,      // Device supports Find My Watch
    RemoteManualSync,
    LiveTrackAutoStart, // Device supports LiveTrack auto start
    LiveTrackMessaging, // Device supports LiveTrack Messaging
    InstantInput,       // Device supports instant input feature
    UnknownVariant(u32),
}
impl ConnectivityCapabilities {
    pub fn from_u32(value: u32) -> ConnectivityCapabilities {
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
    pub fn from_i64(value: i64) -> ConnectivityCapabilities {
        ConnectivityCapabilities::from_u32(value as u32)
    }
    pub fn as_u32(&self) -> u32 {
        match &self {
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
            ConnectivityCapabilities::UnknownVariant(value) => *value,
        }
    }
    pub fn to_string(&self) -> String {
        match &self {
            ConnectivityCapabilities::Bluetooth => "bluetooth".to_string(),
            ConnectivityCapabilities::BluetoothLe => "bluetooth_le".to_string(),
            ConnectivityCapabilities::Ant => "ant".to_string(),
            ConnectivityCapabilities::ActivityUpload => "activity_upload".to_string(),
            ConnectivityCapabilities::CourseDownload => "course_download".to_string(),
            ConnectivityCapabilities::WorkoutDownload => "workout_download".to_string(),
            ConnectivityCapabilities::LiveTrack => "live_track".to_string(),
            ConnectivityCapabilities::WeatherConditions => "weather_conditions".to_string(),
            ConnectivityCapabilities::WeatherAlerts => "weather_alerts".to_string(),
            ConnectivityCapabilities::GpsEphemerisDownload => "gps_ephemeris_download".to_string(),
            ConnectivityCapabilities::ExplicitArchive => "explicit_archive".to_string(),
            ConnectivityCapabilities::SetupIncomplete => "setup_incomplete".to_string(),
            ConnectivityCapabilities::ContinueSyncAfterSoftwareUpdate => {
                "continue_sync_after_software_update".to_string()
            }
            ConnectivityCapabilities::ConnectIqAppDownload => "connect_iq_app_download".to_string(),
            ConnectivityCapabilities::GolfCourseDownload => "golf_course_download".to_string(),
            ConnectivityCapabilities::DeviceInitiatesSync => "device_initiates_sync".to_string(),
            ConnectivityCapabilities::ConnectIqWatchAppDownload => {
                "connect_iq_watch_app_download".to_string()
            }
            ConnectivityCapabilities::ConnectIqWidgetDownload => {
                "connect_iq_widget_download".to_string()
            }
            ConnectivityCapabilities::ConnectIqWatchFaceDownload => {
                "connect_iq_watch_face_download".to_string()
            }
            ConnectivityCapabilities::ConnectIqDataFieldDownload => {
                "connect_iq_data_field_download".to_string()
            }
            ConnectivityCapabilities::ConnectIqAppManagment => {
                "connect_iq_app_managment".to_string()
            }
            ConnectivityCapabilities::SwingSensor => "swing_sensor".to_string(),
            ConnectivityCapabilities::SwingSensorRemote => "swing_sensor_remote".to_string(),
            ConnectivityCapabilities::IncidentDetection => "incident_detection".to_string(),
            ConnectivityCapabilities::AudioPrompts => "audio_prompts".to_string(),
            ConnectivityCapabilities::WifiVerification => "wifi_verification".to_string(),
            ConnectivityCapabilities::TrueUp => "true_up".to_string(),
            ConnectivityCapabilities::FindMyWatch => "find_my_watch".to_string(),
            ConnectivityCapabilities::RemoteManualSync => "remote_manual_sync".to_string(),
            ConnectivityCapabilities::LiveTrackAutoStart => "live_track_auto_start".to_string(),
            ConnectivityCapabilities::LiveTrackMessaging => "live_track_messaging".to_string(),
            ConnectivityCapabilities::InstantInput => "instant_input".to_string(),
            ConnectivityCapabilities::UnknownVariant(value) => {
                format!("unknown_variant_{}", *value)
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum WeatherReport {
    Current,
    HourlyForecast,
    DailyForecast,
    UnknownVariant(u8),
}
impl WeatherReport {
    pub fn from_u8(value: u8) -> WeatherReport {
        match value {
            0 => WeatherReport::Current,
            1 => WeatherReport::HourlyForecast,
            2 => WeatherReport::DailyForecast,
            _ => WeatherReport::UnknownVariant(value),
        }
    }
    pub fn from_i64(value: i64) -> WeatherReport {
        WeatherReport::from_u8(value as u8)
    }
    pub fn as_u8(&self) -> u8 {
        match &self {
            WeatherReport::Current => 0,
            WeatherReport::HourlyForecast => 1,
            WeatherReport::DailyForecast => 2,
            WeatherReport::UnknownVariant(value) => *value,
        }
    }
    pub fn to_string(&self) -> String {
        match &self {
            WeatherReport::Current => "current".to_string(),
            WeatherReport::HourlyForecast => "hourly_forecast".to_string(),
            WeatherReport::DailyForecast => "daily_forecast".to_string(),
            WeatherReport::UnknownVariant(value) => format!("unknown_variant_{}", *value),
        }
    }
}

#[derive(Clone, Copy, Debug)]
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
    pub fn from_u8(value: u8) -> WeatherStatus {
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
    pub fn from_i64(value: i64) -> WeatherStatus {
        WeatherStatus::from_u8(value as u8)
    }
    pub fn as_u8(&self) -> u8 {
        match &self {
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
            WeatherStatus::UnknownVariant(value) => *value,
        }
    }
    pub fn to_string(&self) -> String {
        match &self {
            WeatherStatus::Clear => "clear".to_string(),
            WeatherStatus::PartlyCloudy => "partly_cloudy".to_string(),
            WeatherStatus::MostlyCloudy => "mostly_cloudy".to_string(),
            WeatherStatus::Rain => "rain".to_string(),
            WeatherStatus::Snow => "snow".to_string(),
            WeatherStatus::Windy => "windy".to_string(),
            WeatherStatus::Thunderstorms => "thunderstorms".to_string(),
            WeatherStatus::WintryMix => "wintry_mix".to_string(),
            WeatherStatus::Fog => "fog".to_string(),
            WeatherStatus::Hazy => "hazy".to_string(),
            WeatherStatus::Hail => "hail".to_string(),
            WeatherStatus::ScatteredShowers => "scattered_showers".to_string(),
            WeatherStatus::ScatteredThunderstorms => "scattered_thunderstorms".to_string(),
            WeatherStatus::UnknownPrecipitation => "unknown_precipitation".to_string(),
            WeatherStatus::LightRain => "light_rain".to_string(),
            WeatherStatus::HeavyRain => "heavy_rain".to_string(),
            WeatherStatus::LightSnow => "light_snow".to_string(),
            WeatherStatus::HeavySnow => "heavy_snow".to_string(),
            WeatherStatus::LightRainSnow => "light_rain_snow".to_string(),
            WeatherStatus::HeavyRainSnow => "heavy_rain_snow".to_string(),
            WeatherStatus::Cloudy => "cloudy".to_string(),
            WeatherStatus::UnknownVariant(value) => format!("unknown_variant_{}", *value),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum WeatherSeverity {
    Unknown,
    Warning,
    Watch,
    Advisory,
    Statement,
    UnknownVariant(u8),
}
impl WeatherSeverity {
    pub fn from_u8(value: u8) -> WeatherSeverity {
        match value {
            0 => WeatherSeverity::Unknown,
            1 => WeatherSeverity::Warning,
            2 => WeatherSeverity::Watch,
            3 => WeatherSeverity::Advisory,
            4 => WeatherSeverity::Statement,
            _ => WeatherSeverity::UnknownVariant(value),
        }
    }
    pub fn from_i64(value: i64) -> WeatherSeverity {
        WeatherSeverity::from_u8(value as u8)
    }
    pub fn as_u8(&self) -> u8 {
        match &self {
            WeatherSeverity::Unknown => 0,
            WeatherSeverity::Warning => 1,
            WeatherSeverity::Watch => 2,
            WeatherSeverity::Advisory => 3,
            WeatherSeverity::Statement => 4,
            WeatherSeverity::UnknownVariant(value) => *value,
        }
    }
    pub fn to_string(&self) -> String {
        match &self {
            WeatherSeverity::Unknown => "unknown".to_string(),
            WeatherSeverity::Warning => "warning".to_string(),
            WeatherSeverity::Watch => "watch".to_string(),
            WeatherSeverity::Advisory => "advisory".to_string(),
            WeatherSeverity::Statement => "statement".to_string(),
            WeatherSeverity::UnknownVariant(value) => format!("unknown_variant_{}", *value),
        }
    }
}

#[derive(Clone, Copy, Debug)]
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
    pub fn from_u8(value: u8) -> WeatherSevereType {
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
    pub fn from_i64(value: i64) -> WeatherSevereType {
        WeatherSevereType::from_u8(value as u8)
    }
    pub fn as_u8(&self) -> u8 {
        match &self {
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
            WeatherSevereType::UnknownVariant(value) => *value,
        }
    }
    pub fn to_string(&self) -> String {
        match &self {
            WeatherSevereType::Unspecified => "unspecified".to_string(),
            WeatherSevereType::Tornado => "tornado".to_string(),
            WeatherSevereType::Tsunami => "tsunami".to_string(),
            WeatherSevereType::Hurricane => "hurricane".to_string(),
            WeatherSevereType::ExtremeWind => "extreme_wind".to_string(),
            WeatherSevereType::Typhoon => "typhoon".to_string(),
            WeatherSevereType::InlandHurricane => "inland_hurricane".to_string(),
            WeatherSevereType::HurricaneForceWind => "hurricane_force_wind".to_string(),
            WeatherSevereType::Waterspout => "waterspout".to_string(),
            WeatherSevereType::SevereThunderstorm => "severe_thunderstorm".to_string(),
            WeatherSevereType::WreckhouseWinds => "wreckhouse_winds".to_string(),
            WeatherSevereType::LesSuetesWind => "les_suetes_wind".to_string(),
            WeatherSevereType::Avalanche => "avalanche".to_string(),
            WeatherSevereType::FlashFlood => "flash_flood".to_string(),
            WeatherSevereType::TropicalStorm => "tropical_storm".to_string(),
            WeatherSevereType::InlandTropicalStorm => "inland_tropical_storm".to_string(),
            WeatherSevereType::Blizzard => "blizzard".to_string(),
            WeatherSevereType::IceStorm => "ice_storm".to_string(),
            WeatherSevereType::FreezingRain => "freezing_rain".to_string(),
            WeatherSevereType::DebrisFlow => "debris_flow".to_string(),
            WeatherSevereType::FlashFreeze => "flash_freeze".to_string(),
            WeatherSevereType::DustStorm => "dust_storm".to_string(),
            WeatherSevereType::HighWind => "high_wind".to_string(),
            WeatherSevereType::WinterStorm => "winter_storm".to_string(),
            WeatherSevereType::HeavyFreezingSpray => "heavy_freezing_spray".to_string(),
            WeatherSevereType::ExtremeCold => "extreme_cold".to_string(),
            WeatherSevereType::WindChill => "wind_chill".to_string(),
            WeatherSevereType::ColdWave => "cold_wave".to_string(),
            WeatherSevereType::HeavySnowAlert => "heavy_snow_alert".to_string(),
            WeatherSevereType::LakeEffectBlowingSnow => "lake_effect_blowing_snow".to_string(),
            WeatherSevereType::SnowSquall => "snow_squall".to_string(),
            WeatherSevereType::LakeEffectSnow => "lake_effect_snow".to_string(),
            WeatherSevereType::WinterWeather => "winter_weather".to_string(),
            WeatherSevereType::Sleet => "sleet".to_string(),
            WeatherSevereType::Snowfall => "snowfall".to_string(),
            WeatherSevereType::SnowAndBlowingSnow => "snow_and_blowing_snow".to_string(),
            WeatherSevereType::BlowingSnow => "blowing_snow".to_string(),
            WeatherSevereType::SnowAlert => "snow_alert".to_string(),
            WeatherSevereType::ArcticOutflow => "arctic_outflow".to_string(),
            WeatherSevereType::FreezingDrizzle => "freezing_drizzle".to_string(),
            WeatherSevereType::Storm => "storm".to_string(),
            WeatherSevereType::StormSurge => "storm_surge".to_string(),
            WeatherSevereType::Rainfall => "rainfall".to_string(),
            WeatherSevereType::ArealFlood => "areal_flood".to_string(),
            WeatherSevereType::CoastalFlood => "coastal_flood".to_string(),
            WeatherSevereType::LakeshoreFlood => "lakeshore_flood".to_string(),
            WeatherSevereType::ExcessiveHeat => "excessive_heat".to_string(),
            WeatherSevereType::Heat => "heat".to_string(),
            WeatherSevereType::Weather => "weather".to_string(),
            WeatherSevereType::HighHeatAndHumidity => "high_heat_and_humidity".to_string(),
            WeatherSevereType::HumidexAndHealth => "humidex_and_health".to_string(),
            WeatherSevereType::Humidex => "humidex".to_string(),
            WeatherSevereType::Gale => "gale".to_string(),
            WeatherSevereType::FreezingSpray => "freezing_spray".to_string(),
            WeatherSevereType::SpecialMarine => "special_marine".to_string(),
            WeatherSevereType::Squall => "squall".to_string(),
            WeatherSevereType::StrongWind => "strong_wind".to_string(),
            WeatherSevereType::LakeWind => "lake_wind".to_string(),
            WeatherSevereType::MarineWeather => "marine_weather".to_string(),
            WeatherSevereType::Wind => "wind".to_string(),
            WeatherSevereType::SmallCraftHazardousSeas => "small_craft_hazardous_seas".to_string(),
            WeatherSevereType::HazardousSeas => "hazardous_seas".to_string(),
            WeatherSevereType::SmallCraft => "small_craft".to_string(),
            WeatherSevereType::SmallCraftWinds => "small_craft_winds".to_string(),
            WeatherSevereType::SmallCraftRoughBar => "small_craft_rough_bar".to_string(),
            WeatherSevereType::HighWaterLevel => "high_water_level".to_string(),
            WeatherSevereType::Ashfall => "ashfall".to_string(),
            WeatherSevereType::FreezingFog => "freezing_fog".to_string(),
            WeatherSevereType::DenseFog => "dense_fog".to_string(),
            WeatherSevereType::DenseSmoke => "dense_smoke".to_string(),
            WeatherSevereType::BlowingDust => "blowing_dust".to_string(),
            WeatherSevereType::HardFreeze => "hard_freeze".to_string(),
            WeatherSevereType::Freeze => "freeze".to_string(),
            WeatherSevereType::Frost => "frost".to_string(),
            WeatherSevereType::FireWeather => "fire_weather".to_string(),
            WeatherSevereType::Flood => "flood".to_string(),
            WeatherSevereType::RipTide => "rip_tide".to_string(),
            WeatherSevereType::HighSurf => "high_surf".to_string(),
            WeatherSevereType::Smog => "smog".to_string(),
            WeatherSevereType::AirQuality => "air_quality".to_string(),
            WeatherSevereType::BriskWind => "brisk_wind".to_string(),
            WeatherSevereType::AirStagnation => "air_stagnation".to_string(),
            WeatherSevereType::LowWater => "low_water".to_string(),
            WeatherSevereType::Hydrological => "hydrological".to_string(),
            WeatherSevereType::SpecialWeather => "special_weather".to_string(),
            WeatherSevereType::UnknownVariant(value) => format!("unknown_variant_{}", *value),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum StrokeType {
    NoEvent,
    Other, // stroke was detected but cannot be identified
    Serve,
    Forehand,
    Backhand,
    Smash,
    UnknownVariant(u8),
}
impl StrokeType {
    pub fn from_u8(value: u8) -> StrokeType {
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
    pub fn from_i64(value: i64) -> StrokeType {
        StrokeType::from_u8(value as u8)
    }
    pub fn as_u8(&self) -> u8 {
        match &self {
            StrokeType::NoEvent => 0,
            StrokeType::Other => 1,
            StrokeType::Serve => 2,
            StrokeType::Forehand => 3,
            StrokeType::Backhand => 4,
            StrokeType::Smash => 5,
            StrokeType::UnknownVariant(value) => *value,
        }
    }
    pub fn to_string(&self) -> String {
        match &self {
            StrokeType::NoEvent => "no_event".to_string(),
            StrokeType::Other => "other".to_string(),
            StrokeType::Serve => "serve".to_string(),
            StrokeType::Forehand => "forehand".to_string(),
            StrokeType::Backhand => "backhand".to_string(),
            StrokeType::Smash => "smash".to_string(),
            StrokeType::UnknownVariant(value) => format!("unknown_variant_{}", *value),
        }
    }
}

#[derive(Clone, Copy, Debug)]
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
    LeftBrachioradialis,  // Left anterior forearm
    LeftForearmExtensors, // Left posterior forearm
    RightArm,
    RightShoulder,
    RightBicep,
    RightTricep,
    RightBrachioradialis,  // Right anterior forearm
    RightForearmExtensors, // Right posterior forearm
    Neck,
    Throat,
    WaistMidBack,
    WaistFront,
    WaistLeft,
    WaistRight,
    UnknownVariant(u8),
}
impl BodyLocation {
    pub fn from_u8(value: u8) -> BodyLocation {
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
    pub fn from_i64(value: i64) -> BodyLocation {
        BodyLocation::from_u8(value as u8)
    }
    pub fn as_u8(&self) -> u8 {
        match &self {
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
            BodyLocation::UnknownVariant(value) => *value,
        }
    }
    pub fn to_string(&self) -> String {
        match &self {
            BodyLocation::LeftLeg => "left_leg".to_string(),
            BodyLocation::LeftCalf => "left_calf".to_string(),
            BodyLocation::LeftShin => "left_shin".to_string(),
            BodyLocation::LeftHamstring => "left_hamstring".to_string(),
            BodyLocation::LeftQuad => "left_quad".to_string(),
            BodyLocation::LeftGlute => "left_glute".to_string(),
            BodyLocation::RightLeg => "right_leg".to_string(),
            BodyLocation::RightCalf => "right_calf".to_string(),
            BodyLocation::RightShin => "right_shin".to_string(),
            BodyLocation::RightHamstring => "right_hamstring".to_string(),
            BodyLocation::RightQuad => "right_quad".to_string(),
            BodyLocation::RightGlute => "right_glute".to_string(),
            BodyLocation::TorsoBack => "torso_back".to_string(),
            BodyLocation::LeftLowerBack => "left_lower_back".to_string(),
            BodyLocation::LeftUpperBack => "left_upper_back".to_string(),
            BodyLocation::RightLowerBack => "right_lower_back".to_string(),
            BodyLocation::RightUpperBack => "right_upper_back".to_string(),
            BodyLocation::TorsoFront => "torso_front".to_string(),
            BodyLocation::LeftAbdomen => "left_abdomen".to_string(),
            BodyLocation::LeftChest => "left_chest".to_string(),
            BodyLocation::RightAbdomen => "right_abdomen".to_string(),
            BodyLocation::RightChest => "right_chest".to_string(),
            BodyLocation::LeftArm => "left_arm".to_string(),
            BodyLocation::LeftShoulder => "left_shoulder".to_string(),
            BodyLocation::LeftBicep => "left_bicep".to_string(),
            BodyLocation::LeftTricep => "left_tricep".to_string(),
            BodyLocation::LeftBrachioradialis => "left_brachioradialis".to_string(),
            BodyLocation::LeftForearmExtensors => "left_forearm_extensors".to_string(),
            BodyLocation::RightArm => "right_arm".to_string(),
            BodyLocation::RightShoulder => "right_shoulder".to_string(),
            BodyLocation::RightBicep => "right_bicep".to_string(),
            BodyLocation::RightTricep => "right_tricep".to_string(),
            BodyLocation::RightBrachioradialis => "right_brachioradialis".to_string(),
            BodyLocation::RightForearmExtensors => "right_forearm_extensors".to_string(),
            BodyLocation::Neck => "neck".to_string(),
            BodyLocation::Throat => "throat".to_string(),
            BodyLocation::WaistMidBack => "waist_mid_back".to_string(),
            BodyLocation::WaistFront => "waist_front".to_string(),
            BodyLocation::WaistLeft => "waist_left".to_string(),
            BodyLocation::WaistRight => "waist_right".to_string(),
            BodyLocation::UnknownVariant(value) => format!("unknown_variant_{}", *value),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum SegmentLapStatus {
    End,
    Fail,
    UnknownVariant(u8),
}
impl SegmentLapStatus {
    pub fn from_u8(value: u8) -> SegmentLapStatus {
        match value {
            0 => SegmentLapStatus::End,
            1 => SegmentLapStatus::Fail,
            _ => SegmentLapStatus::UnknownVariant(value),
        }
    }
    pub fn from_i64(value: i64) -> SegmentLapStatus {
        SegmentLapStatus::from_u8(value as u8)
    }
    pub fn as_u8(&self) -> u8 {
        match &self {
            SegmentLapStatus::End => 0,
            SegmentLapStatus::Fail => 1,
            SegmentLapStatus::UnknownVariant(value) => *value,
        }
    }
    pub fn to_string(&self) -> String {
        match &self {
            SegmentLapStatus::End => "end".to_string(),
            SegmentLapStatus::Fail => "fail".to_string(),
            SegmentLapStatus::UnknownVariant(value) => format!("unknown_variant_{}", *value),
        }
    }
}

#[derive(Clone, Copy, Debug)]
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
    pub fn from_u8(value: u8) -> SegmentLeaderboardType {
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
    pub fn from_i64(value: i64) -> SegmentLeaderboardType {
        SegmentLeaderboardType::from_u8(value as u8)
    }
    pub fn as_u8(&self) -> u8 {
        match &self {
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
            SegmentLeaderboardType::UnknownVariant(value) => *value,
        }
    }
    pub fn to_string(&self) -> String {
        match &self {
            SegmentLeaderboardType::Overall => "overall".to_string(),
            SegmentLeaderboardType::PersonalBest => "personal_best".to_string(),
            SegmentLeaderboardType::Connections => "connections".to_string(),
            SegmentLeaderboardType::Group => "group".to_string(),
            SegmentLeaderboardType::Challenger => "challenger".to_string(),
            SegmentLeaderboardType::Kom => "kom".to_string(),
            SegmentLeaderboardType::Qom => "qom".to_string(),
            SegmentLeaderboardType::Pr => "pr".to_string(),
            SegmentLeaderboardType::Goal => "goal".to_string(),
            SegmentLeaderboardType::Rival => "rival".to_string(),
            SegmentLeaderboardType::ClubLeader => "club_leader".to_string(),
            SegmentLeaderboardType::UnknownVariant(value) => format!("unknown_variant_{}", *value),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum SegmentDeleteStatus {
    DoNotDelete,
    DeleteOne,
    DeleteAll,
    UnknownVariant(u8),
}
impl SegmentDeleteStatus {
    pub fn from_u8(value: u8) -> SegmentDeleteStatus {
        match value {
            0 => SegmentDeleteStatus::DoNotDelete,
            1 => SegmentDeleteStatus::DeleteOne,
            2 => SegmentDeleteStatus::DeleteAll,
            _ => SegmentDeleteStatus::UnknownVariant(value),
        }
    }
    pub fn from_i64(value: i64) -> SegmentDeleteStatus {
        SegmentDeleteStatus::from_u8(value as u8)
    }
    pub fn as_u8(&self) -> u8 {
        match &self {
            SegmentDeleteStatus::DoNotDelete => 0,
            SegmentDeleteStatus::DeleteOne => 1,
            SegmentDeleteStatus::DeleteAll => 2,
            SegmentDeleteStatus::UnknownVariant(value) => *value,
        }
    }
    pub fn to_string(&self) -> String {
        match &self {
            SegmentDeleteStatus::DoNotDelete => "do_not_delete".to_string(),
            SegmentDeleteStatus::DeleteOne => "delete_one".to_string(),
            SegmentDeleteStatus::DeleteAll => "delete_all".to_string(),
            SegmentDeleteStatus::UnknownVariant(value) => format!("unknown_variant_{}", *value),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum SegmentSelectionType {
    Starred,
    Suggested,
    UnknownVariant(u8),
}
impl SegmentSelectionType {
    pub fn from_u8(value: u8) -> SegmentSelectionType {
        match value {
            0 => SegmentSelectionType::Starred,
            1 => SegmentSelectionType::Suggested,
            _ => SegmentSelectionType::UnknownVariant(value),
        }
    }
    pub fn from_i64(value: i64) -> SegmentSelectionType {
        SegmentSelectionType::from_u8(value as u8)
    }
    pub fn as_u8(&self) -> u8 {
        match &self {
            SegmentSelectionType::Starred => 0,
            SegmentSelectionType::Suggested => 1,
            SegmentSelectionType::UnknownVariant(value) => *value,
        }
    }
    pub fn to_string(&self) -> String {
        match &self {
            SegmentSelectionType::Starred => "starred".to_string(),
            SegmentSelectionType::Suggested => "suggested".to_string(),
            SegmentSelectionType::UnknownVariant(value) => format!("unknown_variant_{}", *value),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum SourceType {
    Ant,                // External device connected with ANT
    Antplus,            // External device connected with ANT+
    Bluetooth,          // External device connected with BT
    BluetoothLowEnergy, // External device connected with BLE
    Wifi,               // External device connected with Wifi
    Local,              // Onboard device
    UnknownVariant(u8),
}
impl SourceType {
    pub fn from_u8(value: u8) -> SourceType {
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
    pub fn from_i64(value: i64) -> SourceType {
        SourceType::from_u8(value as u8)
    }
    pub fn as_u8(&self) -> u8 {
        match &self {
            SourceType::Ant => 0,
            SourceType::Antplus => 1,
            SourceType::Bluetooth => 2,
            SourceType::BluetoothLowEnergy => 3,
            SourceType::Wifi => 4,
            SourceType::Local => 5,
            SourceType::UnknownVariant(value) => *value,
        }
    }
    pub fn to_string(&self) -> String {
        match &self {
            SourceType::Ant => "ant".to_string(),
            SourceType::Antplus => "antplus".to_string(),
            SourceType::Bluetooth => "bluetooth".to_string(),
            SourceType::BluetoothLowEnergy => "bluetooth_low_energy".to_string(),
            SourceType::Wifi => "wifi".to_string(),
            SourceType::Local => "local".to_string(),
            SourceType::UnknownVariant(value) => format!("unknown_variant_{}", *value),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum DisplayOrientation {
    Auto, // automatic if the device supports it
    Portrait,
    Landscape,
    PortraitFlipped,  // portrait mode but rotated 180 degrees
    LandscapeFlipped, // landscape mode but rotated 180 degrees
    UnknownVariant(u8),
}
impl DisplayOrientation {
    pub fn from_u8(value: u8) -> DisplayOrientation {
        match value {
            0 => DisplayOrientation::Auto,
            1 => DisplayOrientation::Portrait,
            2 => DisplayOrientation::Landscape,
            3 => DisplayOrientation::PortraitFlipped,
            4 => DisplayOrientation::LandscapeFlipped,
            _ => DisplayOrientation::UnknownVariant(value),
        }
    }
    pub fn from_i64(value: i64) -> DisplayOrientation {
        DisplayOrientation::from_u8(value as u8)
    }
    pub fn as_u8(&self) -> u8 {
        match &self {
            DisplayOrientation::Auto => 0,
            DisplayOrientation::Portrait => 1,
            DisplayOrientation::Landscape => 2,
            DisplayOrientation::PortraitFlipped => 3,
            DisplayOrientation::LandscapeFlipped => 4,
            DisplayOrientation::UnknownVariant(value) => *value,
        }
    }
    pub fn to_string(&self) -> String {
        match &self {
            DisplayOrientation::Auto => "auto".to_string(),
            DisplayOrientation::Portrait => "portrait".to_string(),
            DisplayOrientation::Landscape => "landscape".to_string(),
            DisplayOrientation::PortraitFlipped => "portrait_flipped".to_string(),
            DisplayOrientation::LandscapeFlipped => "landscape_flipped".to_string(),
            DisplayOrientation::UnknownVariant(value) => format!("unknown_variant_{}", *value),
        }
    }
}

#[derive(Clone, Copy, Debug)]
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
    pub fn from_u8(value: u8) -> WorkoutEquipment {
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
    pub fn from_i64(value: i64) -> WorkoutEquipment {
        WorkoutEquipment::from_u8(value as u8)
    }
    pub fn as_u8(&self) -> u8 {
        match &self {
            WorkoutEquipment::None => 0,
            WorkoutEquipment::SwimFins => 1,
            WorkoutEquipment::SwimKickboard => 2,
            WorkoutEquipment::SwimPaddles => 3,
            WorkoutEquipment::SwimPullBuoy => 4,
            WorkoutEquipment::SwimSnorkel => 5,
            WorkoutEquipment::UnknownVariant(value) => *value,
        }
    }
    pub fn to_string(&self) -> String {
        match &self {
            WorkoutEquipment::None => "none".to_string(),
            WorkoutEquipment::SwimFins => "swim_fins".to_string(),
            WorkoutEquipment::SwimKickboard => "swim_kickboard".to_string(),
            WorkoutEquipment::SwimPaddles => "swim_paddles".to_string(),
            WorkoutEquipment::SwimPullBuoy => "swim_pull_buoy".to_string(),
            WorkoutEquipment::SwimSnorkel => "swim_snorkel".to_string(),
            WorkoutEquipment::UnknownVariant(value) => format!("unknown_variant_{}", *value),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum WatchfaceMode {
    Digital,
    Analog,
    ConnectIq,
    Disabled,
    UnknownVariant(u8),
}
impl WatchfaceMode {
    pub fn from_u8(value: u8) -> WatchfaceMode {
        match value {
            0 => WatchfaceMode::Digital,
            1 => WatchfaceMode::Analog,
            2 => WatchfaceMode::ConnectIq,
            3 => WatchfaceMode::Disabled,
            _ => WatchfaceMode::UnknownVariant(value),
        }
    }
    pub fn from_i64(value: i64) -> WatchfaceMode {
        WatchfaceMode::from_u8(value as u8)
    }
    pub fn as_u8(&self) -> u8 {
        match &self {
            WatchfaceMode::Digital => 0,
            WatchfaceMode::Analog => 1,
            WatchfaceMode::ConnectIq => 2,
            WatchfaceMode::Disabled => 3,
            WatchfaceMode::UnknownVariant(value) => *value,
        }
    }
    pub fn to_string(&self) -> String {
        match &self {
            WatchfaceMode::Digital => "digital".to_string(),
            WatchfaceMode::Analog => "analog".to_string(),
            WatchfaceMode::ConnectIq => "connect_iq".to_string(),
            WatchfaceMode::Disabled => "disabled".to_string(),
            WatchfaceMode::UnknownVariant(value) => format!("unknown_variant_{}", *value),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum DigitalWatchfaceLayout {
    Traditional,
    Modern,
    Bold,
    UnknownVariant(u8),
}
impl DigitalWatchfaceLayout {
    pub fn from_u8(value: u8) -> DigitalWatchfaceLayout {
        match value {
            0 => DigitalWatchfaceLayout::Traditional,
            1 => DigitalWatchfaceLayout::Modern,
            2 => DigitalWatchfaceLayout::Bold,
            _ => DigitalWatchfaceLayout::UnknownVariant(value),
        }
    }
    pub fn from_i64(value: i64) -> DigitalWatchfaceLayout {
        DigitalWatchfaceLayout::from_u8(value as u8)
    }
    pub fn as_u8(&self) -> u8 {
        match &self {
            DigitalWatchfaceLayout::Traditional => 0,
            DigitalWatchfaceLayout::Modern => 1,
            DigitalWatchfaceLayout::Bold => 2,
            DigitalWatchfaceLayout::UnknownVariant(value) => *value,
        }
    }
    pub fn to_string(&self) -> String {
        match &self {
            DigitalWatchfaceLayout::Traditional => "traditional".to_string(),
            DigitalWatchfaceLayout::Modern => "modern".to_string(),
            DigitalWatchfaceLayout::Bold => "bold".to_string(),
            DigitalWatchfaceLayout::UnknownVariant(value) => format!("unknown_variant_{}", *value),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum AnalogWatchfaceLayout {
    Minimal,
    Traditional,
    Modern,
    UnknownVariant(u8),
}
impl AnalogWatchfaceLayout {
    pub fn from_u8(value: u8) -> AnalogWatchfaceLayout {
        match value {
            0 => AnalogWatchfaceLayout::Minimal,
            1 => AnalogWatchfaceLayout::Traditional,
            2 => AnalogWatchfaceLayout::Modern,
            _ => AnalogWatchfaceLayout::UnknownVariant(value),
        }
    }
    pub fn from_i64(value: i64) -> AnalogWatchfaceLayout {
        AnalogWatchfaceLayout::from_u8(value as u8)
    }
    pub fn as_u8(&self) -> u8 {
        match &self {
            AnalogWatchfaceLayout::Minimal => 0,
            AnalogWatchfaceLayout::Traditional => 1,
            AnalogWatchfaceLayout::Modern => 2,
            AnalogWatchfaceLayout::UnknownVariant(value) => *value,
        }
    }
    pub fn to_string(&self) -> String {
        match &self {
            AnalogWatchfaceLayout::Minimal => "minimal".to_string(),
            AnalogWatchfaceLayout::Traditional => "traditional".to_string(),
            AnalogWatchfaceLayout::Modern => "modern".to_string(),
            AnalogWatchfaceLayout::UnknownVariant(value) => format!("unknown_variant_{}", *value),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum RiderPositionType {
    Seated,
    Standing,
    TransitionToSeated,
    TransitionToStanding,
    UnknownVariant(u8),
}
impl RiderPositionType {
    pub fn from_u8(value: u8) -> RiderPositionType {
        match value {
            0 => RiderPositionType::Seated,
            1 => RiderPositionType::Standing,
            2 => RiderPositionType::TransitionToSeated,
            3 => RiderPositionType::TransitionToStanding,
            _ => RiderPositionType::UnknownVariant(value),
        }
    }
    pub fn from_i64(value: i64) -> RiderPositionType {
        RiderPositionType::from_u8(value as u8)
    }
    pub fn as_u8(&self) -> u8 {
        match &self {
            RiderPositionType::Seated => 0,
            RiderPositionType::Standing => 1,
            RiderPositionType::TransitionToSeated => 2,
            RiderPositionType::TransitionToStanding => 3,
            RiderPositionType::UnknownVariant(value) => *value,
        }
    }
    pub fn to_string(&self) -> String {
        match &self {
            RiderPositionType::Seated => "seated".to_string(),
            RiderPositionType::Standing => "standing".to_string(),
            RiderPositionType::TransitionToSeated => "transition_to_seated".to_string(),
            RiderPositionType::TransitionToStanding => "transition_to_standing".to_string(),
            RiderPositionType::UnknownVariant(value) => format!("unknown_variant_{}", *value),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum PowerPhaseType {
    PowerPhaseStartAngle,
    PowerPhaseEndAngle,
    PowerPhaseArcLength,
    PowerPhaseCenter,
    UnknownVariant(u8),
}
impl PowerPhaseType {
    pub fn from_u8(value: u8) -> PowerPhaseType {
        match value {
            0 => PowerPhaseType::PowerPhaseStartAngle,
            1 => PowerPhaseType::PowerPhaseEndAngle,
            2 => PowerPhaseType::PowerPhaseArcLength,
            3 => PowerPhaseType::PowerPhaseCenter,
            _ => PowerPhaseType::UnknownVariant(value),
        }
    }
    pub fn from_i64(value: i64) -> PowerPhaseType {
        PowerPhaseType::from_u8(value as u8)
    }
    pub fn as_u8(&self) -> u8 {
        match &self {
            PowerPhaseType::PowerPhaseStartAngle => 0,
            PowerPhaseType::PowerPhaseEndAngle => 1,
            PowerPhaseType::PowerPhaseArcLength => 2,
            PowerPhaseType::PowerPhaseCenter => 3,
            PowerPhaseType::UnknownVariant(value) => *value,
        }
    }
    pub fn to_string(&self) -> String {
        match &self {
            PowerPhaseType::PowerPhaseStartAngle => "power_phase_start_angle".to_string(),
            PowerPhaseType::PowerPhaseEndAngle => "power_phase_end_angle".to_string(),
            PowerPhaseType::PowerPhaseArcLength => "power_phase_arc_length".to_string(),
            PowerPhaseType::PowerPhaseCenter => "power_phase_center".to_string(),
            PowerPhaseType::UnknownVariant(value) => format!("unknown_variant_{}", *value),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum CameraEventType {
    VideoStart, // Start of video recording
    VideoSplit, // Mark of video file split (end of one file, beginning of the other)
    VideoEnd,   // End of video recording
    PhotoTaken, // Still photo taken
    VideoSecondStreamStart,
    VideoSecondStreamSplit,
    VideoSecondStreamEnd,
    VideoSplitStart, // Mark of video file split start
    VideoSecondStreamSplitStart,
    VideoPause, // Mark when a video recording has been paused
    VideoSecondStreamPause,
    VideoResume, // Mark when a video recording has been resumed
    VideoSecondStreamResume,
    UnknownVariant(u8),
}
impl CameraEventType {
    pub fn from_u8(value: u8) -> CameraEventType {
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
    pub fn from_i64(value: i64) -> CameraEventType {
        CameraEventType::from_u8(value as u8)
    }
    pub fn as_u8(&self) -> u8 {
        match &self {
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
            CameraEventType::UnknownVariant(value) => *value,
        }
    }
    pub fn to_string(&self) -> String {
        match &self {
            CameraEventType::VideoStart => "video_start".to_string(),
            CameraEventType::VideoSplit => "video_split".to_string(),
            CameraEventType::VideoEnd => "video_end".to_string(),
            CameraEventType::PhotoTaken => "photo_taken".to_string(),
            CameraEventType::VideoSecondStreamStart => "video_second_stream_start".to_string(),
            CameraEventType::VideoSecondStreamSplit => "video_second_stream_split".to_string(),
            CameraEventType::VideoSecondStreamEnd => "video_second_stream_end".to_string(),
            CameraEventType::VideoSplitStart => "video_split_start".to_string(),
            CameraEventType::VideoSecondStreamSplitStart => {
                "video_second_stream_split_start".to_string()
            }
            CameraEventType::VideoPause => "video_pause".to_string(),
            CameraEventType::VideoSecondStreamPause => "video_second_stream_pause".to_string(),
            CameraEventType::VideoResume => "video_resume".to_string(),
            CameraEventType::VideoSecondStreamResume => "video_second_stream_resume".to_string(),
            CameraEventType::UnknownVariant(value) => format!("unknown_variant_{}", *value),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum SensorType {
    Accelerometer,
    Gyroscope,
    Compass, // Magnetometer
    Barometer,
    UnknownVariant(u8),
}
impl SensorType {
    pub fn from_u8(value: u8) -> SensorType {
        match value {
            0 => SensorType::Accelerometer,
            1 => SensorType::Gyroscope,
            2 => SensorType::Compass,
            3 => SensorType::Barometer,
            _ => SensorType::UnknownVariant(value),
        }
    }
    pub fn from_i64(value: i64) -> SensorType {
        SensorType::from_u8(value as u8)
    }
    pub fn as_u8(&self) -> u8 {
        match &self {
            SensorType::Accelerometer => 0,
            SensorType::Gyroscope => 1,
            SensorType::Compass => 2,
            SensorType::Barometer => 3,
            SensorType::UnknownVariant(value) => *value,
        }
    }
    pub fn to_string(&self) -> String {
        match &self {
            SensorType::Accelerometer => "accelerometer".to_string(),
            SensorType::Gyroscope => "gyroscope".to_string(),
            SensorType::Compass => "compass".to_string(),
            SensorType::Barometer => "barometer".to_string(),
            SensorType::UnknownVariant(value) => format!("unknown_variant_{}", *value),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum BikeLightNetworkConfigType {
    Auto,
    Individual,
    HighVisibility,
    Trail,
    UnknownVariant(u8),
}
impl BikeLightNetworkConfigType {
    pub fn from_u8(value: u8) -> BikeLightNetworkConfigType {
        match value {
            0 => BikeLightNetworkConfigType::Auto,
            4 => BikeLightNetworkConfigType::Individual,
            5 => BikeLightNetworkConfigType::HighVisibility,
            6 => BikeLightNetworkConfigType::Trail,
            _ => BikeLightNetworkConfigType::UnknownVariant(value),
        }
    }
    pub fn from_i64(value: i64) -> BikeLightNetworkConfigType {
        BikeLightNetworkConfigType::from_u8(value as u8)
    }
    pub fn as_u8(&self) -> u8 {
        match &self {
            BikeLightNetworkConfigType::Auto => 0,
            BikeLightNetworkConfigType::Individual => 4,
            BikeLightNetworkConfigType::HighVisibility => 5,
            BikeLightNetworkConfigType::Trail => 6,
            BikeLightNetworkConfigType::UnknownVariant(value) => *value,
        }
    }
    pub fn to_string(&self) -> String {
        match &self {
            BikeLightNetworkConfigType::Auto => "auto".to_string(),
            BikeLightNetworkConfigType::Individual => "individual".to_string(),
            BikeLightNetworkConfigType::HighVisibility => "high_visibility".to_string(),
            BikeLightNetworkConfigType::Trail => "trail".to_string(),
            BikeLightNetworkConfigType::UnknownVariant(value) => {
                format!("unknown_variant_{}", *value)
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum CommTimeoutType {
    WildcardPairingTimeout, // Timeout pairing to any device
    PairingTimeout,         // Timeout pairing to previously paired device
    ConnectionLost,         // Temporary loss of communications
    ConnectionTimeout,      // Connection closed due to extended bad communications
    UnknownVariant(u16),
}
impl CommTimeoutType {
    pub fn from_u16(value: u16) -> CommTimeoutType {
        match value {
            0 => CommTimeoutType::WildcardPairingTimeout,
            1 => CommTimeoutType::PairingTimeout,
            2 => CommTimeoutType::ConnectionLost,
            3 => CommTimeoutType::ConnectionTimeout,
            _ => CommTimeoutType::UnknownVariant(value),
        }
    }
    pub fn from_i64(value: i64) -> CommTimeoutType {
        CommTimeoutType::from_u16(value as u16)
    }
    pub fn as_u16(&self) -> u16 {
        match &self {
            CommTimeoutType::WildcardPairingTimeout => 0,
            CommTimeoutType::PairingTimeout => 1,
            CommTimeoutType::ConnectionLost => 2,
            CommTimeoutType::ConnectionTimeout => 3,
            CommTimeoutType::UnknownVariant(value) => *value,
        }
    }
    pub fn to_string(&self) -> String {
        match &self {
            CommTimeoutType::WildcardPairingTimeout => "wildcard_pairing_timeout".to_string(),
            CommTimeoutType::PairingTimeout => "pairing_timeout".to_string(),
            CommTimeoutType::ConnectionLost => "connection_lost".to_string(),
            CommTimeoutType::ConnectionTimeout => "connection_timeout".to_string(),
            CommTimeoutType::UnknownVariant(value) => format!("unknown_variant_{}", *value),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum CameraOrientationType {
    CameraOrientation0,
    CameraOrientation90,
    CameraOrientation180,
    CameraOrientation270,
    UnknownVariant(u8),
}
impl CameraOrientationType {
    pub fn from_u8(value: u8) -> CameraOrientationType {
        match value {
            0 => CameraOrientationType::CameraOrientation0,
            1 => CameraOrientationType::CameraOrientation90,
            2 => CameraOrientationType::CameraOrientation180,
            3 => CameraOrientationType::CameraOrientation270,
            _ => CameraOrientationType::UnknownVariant(value),
        }
    }
    pub fn from_i64(value: i64) -> CameraOrientationType {
        CameraOrientationType::from_u8(value as u8)
    }
    pub fn as_u8(&self) -> u8 {
        match &self {
            CameraOrientationType::CameraOrientation0 => 0,
            CameraOrientationType::CameraOrientation90 => 1,
            CameraOrientationType::CameraOrientation180 => 2,
            CameraOrientationType::CameraOrientation270 => 3,
            CameraOrientationType::UnknownVariant(value) => *value,
        }
    }
    pub fn to_string(&self) -> String {
        match &self {
            CameraOrientationType::CameraOrientation0 => "camera_orientation_0".to_string(),
            CameraOrientationType::CameraOrientation90 => "camera_orientation_90".to_string(),
            CameraOrientationType::CameraOrientation180 => "camera_orientation_180".to_string(),
            CameraOrientationType::CameraOrientation270 => "camera_orientation_270".to_string(),
            CameraOrientationType::UnknownVariant(value) => format!("unknown_variant_{}", *value),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum AttitudeStage {
    Failed,
    Aligning,
    Degraded,
    Valid,
    UnknownVariant(u8),
}
impl AttitudeStage {
    pub fn from_u8(value: u8) -> AttitudeStage {
        match value {
            0 => AttitudeStage::Failed,
            1 => AttitudeStage::Aligning,
            2 => AttitudeStage::Degraded,
            3 => AttitudeStage::Valid,
            _ => AttitudeStage::UnknownVariant(value),
        }
    }
    pub fn from_i64(value: i64) -> AttitudeStage {
        AttitudeStage::from_u8(value as u8)
    }
    pub fn as_u8(&self) -> u8 {
        match &self {
            AttitudeStage::Failed => 0,
            AttitudeStage::Aligning => 1,
            AttitudeStage::Degraded => 2,
            AttitudeStage::Valid => 3,
            AttitudeStage::UnknownVariant(value) => *value,
        }
    }
    pub fn to_string(&self) -> String {
        match &self {
            AttitudeStage::Failed => "failed".to_string(),
            AttitudeStage::Aligning => "aligning".to_string(),
            AttitudeStage::Degraded => "degraded".to_string(),
            AttitudeStage::Valid => "valid".to_string(),
            AttitudeStage::UnknownVariant(value) => format!("unknown_variant_{}", *value),
        }
    }
}

#[derive(Clone, Copy, Debug)]
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
    pub fn from_u16(value: u16) -> AttitudeValidity {
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
    pub fn from_i64(value: i64) -> AttitudeValidity {
        AttitudeValidity::from_u16(value as u16)
    }
    pub fn as_u16(&self) -> u16 {
        match &self {
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
            AttitudeValidity::UnknownVariant(value) => *value,
        }
    }
    pub fn to_string(&self) -> String {
        match &self {
            AttitudeValidity::TrackAngleHeadingValid => "track_angle_heading_valid".to_string(),
            AttitudeValidity::PitchValid => "pitch_valid".to_string(),
            AttitudeValidity::RollValid => "roll_valid".to_string(),
            AttitudeValidity::LateralBodyAccelValid => "lateral_body_accel_valid".to_string(),
            AttitudeValidity::NormalBodyAccelValid => "normal_body_accel_valid".to_string(),
            AttitudeValidity::TurnRateValid => "turn_rate_valid".to_string(),
            AttitudeValidity::HwFail => "hw_fail".to_string(),
            AttitudeValidity::MagInvalid => "mag_invalid".to_string(),
            AttitudeValidity::NoGps => "no_gps".to_string(),
            AttitudeValidity::GpsInvalid => "gps_invalid".to_string(),
            AttitudeValidity::SolutionCoasting => "solution_coasting".to_string(),
            AttitudeValidity::TrueTrackAngle => "true_track_angle".to_string(),
            AttitudeValidity::MagneticHeading => "magnetic_heading".to_string(),
            AttitudeValidity::UnknownVariant(value) => format!("unknown_variant_{}", *value),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum AutoSyncFrequency {
    Never,
    Occasionally,
    Frequent,
    OnceADay,
    Remote,
    UnknownVariant(u8),
}
impl AutoSyncFrequency {
    pub fn from_u8(value: u8) -> AutoSyncFrequency {
        match value {
            0 => AutoSyncFrequency::Never,
            1 => AutoSyncFrequency::Occasionally,
            2 => AutoSyncFrequency::Frequent,
            3 => AutoSyncFrequency::OnceADay,
            4 => AutoSyncFrequency::Remote,
            _ => AutoSyncFrequency::UnknownVariant(value),
        }
    }
    pub fn from_i64(value: i64) -> AutoSyncFrequency {
        AutoSyncFrequency::from_u8(value as u8)
    }
    pub fn as_u8(&self) -> u8 {
        match &self {
            AutoSyncFrequency::Never => 0,
            AutoSyncFrequency::Occasionally => 1,
            AutoSyncFrequency::Frequent => 2,
            AutoSyncFrequency::OnceADay => 3,
            AutoSyncFrequency::Remote => 4,
            AutoSyncFrequency::UnknownVariant(value) => *value,
        }
    }
    pub fn to_string(&self) -> String {
        match &self {
            AutoSyncFrequency::Never => "never".to_string(),
            AutoSyncFrequency::Occasionally => "occasionally".to_string(),
            AutoSyncFrequency::Frequent => "frequent".to_string(),
            AutoSyncFrequency::OnceADay => "once_a_day".to_string(),
            AutoSyncFrequency::Remote => "remote".to_string(),
            AutoSyncFrequency::UnknownVariant(value) => format!("unknown_variant_{}", *value),
        }
    }
}

#[derive(Clone, Copy, Debug)]
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
    pub fn from_u8(value: u8) -> ExdLayout {
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
    pub fn from_i64(value: i64) -> ExdLayout {
        ExdLayout::from_u8(value as u8)
    }
    pub fn as_u8(&self) -> u8 {
        match &self {
            ExdLayout::FullScreen => 0,
            ExdLayout::HalfVertical => 1,
            ExdLayout::HalfHorizontal => 2,
            ExdLayout::HalfVerticalRightSplit => 3,
            ExdLayout::HalfHorizontalBottomSplit => 4,
            ExdLayout::FullQuarterSplit => 5,
            ExdLayout::HalfVerticalLeftSplit => 6,
            ExdLayout::HalfHorizontalTopSplit => 7,
            ExdLayout::UnknownVariant(value) => *value,
        }
    }
    pub fn to_string(&self) -> String {
        match &self {
            ExdLayout::FullScreen => "full_screen".to_string(),
            ExdLayout::HalfVertical => "half_vertical".to_string(),
            ExdLayout::HalfHorizontal => "half_horizontal".to_string(),
            ExdLayout::HalfVerticalRightSplit => "half_vertical_right_split".to_string(),
            ExdLayout::HalfHorizontalBottomSplit => "half_horizontal_bottom_split".to_string(),
            ExdLayout::FullQuarterSplit => "full_quarter_split".to_string(),
            ExdLayout::HalfVerticalLeftSplit => "half_vertical_left_split".to_string(),
            ExdLayout::HalfHorizontalTopSplit => "half_horizontal_top_split".to_string(),
            ExdLayout::UnknownVariant(value) => format!("unknown_variant_{}", *value),
        }
    }
}

#[derive(Clone, Copy, Debug)]
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
    pub fn from_u8(value: u8) -> ExdDisplayType {
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
    pub fn from_i64(value: i64) -> ExdDisplayType {
        ExdDisplayType::from_u8(value as u8)
    }
    pub fn as_u8(&self) -> u8 {
        match &self {
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
            ExdDisplayType::UnknownVariant(value) => *value,
        }
    }
    pub fn to_string(&self) -> String {
        match &self {
            ExdDisplayType::Numerical => "numerical".to_string(),
            ExdDisplayType::Simple => "simple".to_string(),
            ExdDisplayType::Graph => "graph".to_string(),
            ExdDisplayType::Bar => "bar".to_string(),
            ExdDisplayType::CircleGraph => "circle_graph".to_string(),
            ExdDisplayType::VirtualPartner => "virtual_partner".to_string(),
            ExdDisplayType::Balance => "balance".to_string(),
            ExdDisplayType::StringList => "string_list".to_string(),
            ExdDisplayType::String => "string".to_string(),
            ExdDisplayType::SimpleDynamicIcon => "simple_dynamic_icon".to_string(),
            ExdDisplayType::Gauge => "gauge".to_string(),
            ExdDisplayType::UnknownVariant(value) => format!("unknown_variant_{}", *value),
        }
    }
}

#[derive(Clone, Copy, Debug)]
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
    pub fn from_u8(value: u8) -> ExdDataUnits {
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
    pub fn from_i64(value: i64) -> ExdDataUnits {
        ExdDataUnits::from_u8(value as u8)
    }
    pub fn as_u8(&self) -> u8 {
        match &self {
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
            ExdDataUnits::UnknownVariant(value) => *value,
        }
    }
    pub fn to_string(&self) -> String {
        match &self {
            ExdDataUnits::NoUnits => "no_units".to_string(),
            ExdDataUnits::Laps => "laps".to_string(),
            ExdDataUnits::MilesPerHour => "miles_per_hour".to_string(),
            ExdDataUnits::KilometersPerHour => "kilometers_per_hour".to_string(),
            ExdDataUnits::FeetPerHour => "feet_per_hour".to_string(),
            ExdDataUnits::MetersPerHour => "meters_per_hour".to_string(),
            ExdDataUnits::DegreesCelsius => "degrees_celsius".to_string(),
            ExdDataUnits::DegreesFarenheit => "degrees_farenheit".to_string(),
            ExdDataUnits::Zone => "zone".to_string(),
            ExdDataUnits::Gear => "gear".to_string(),
            ExdDataUnits::Rpm => "rpm".to_string(),
            ExdDataUnits::Bpm => "bpm".to_string(),
            ExdDataUnits::Degrees => "degrees".to_string(),
            ExdDataUnits::Millimeters => "millimeters".to_string(),
            ExdDataUnits::Meters => "meters".to_string(),
            ExdDataUnits::Kilometers => "kilometers".to_string(),
            ExdDataUnits::Feet => "feet".to_string(),
            ExdDataUnits::Yards => "yards".to_string(),
            ExdDataUnits::Kilofeet => "kilofeet".to_string(),
            ExdDataUnits::Miles => "miles".to_string(),
            ExdDataUnits::Time => "time".to_string(),
            ExdDataUnits::EnumTurnType => "enum_turn_type".to_string(),
            ExdDataUnits::Percent => "percent".to_string(),
            ExdDataUnits::Watts => "watts".to_string(),
            ExdDataUnits::WattsPerKilogram => "watts_per_kilogram".to_string(),
            ExdDataUnits::EnumBatteryStatus => "enum_battery_status".to_string(),
            ExdDataUnits::EnumBikeLightBeamAngleMode => {
                "enum_bike_light_beam_angle_mode".to_string()
            }
            ExdDataUnits::EnumBikeLightBatteryStatus => {
                "enum_bike_light_battery_status".to_string()
            }
            ExdDataUnits::EnumBikeLightNetworkConfigType => {
                "enum_bike_light_network_config_type".to_string()
            }
            ExdDataUnits::Lights => "lights".to_string(),
            ExdDataUnits::Seconds => "seconds".to_string(),
            ExdDataUnits::Minutes => "minutes".to_string(),
            ExdDataUnits::Hours => "hours".to_string(),
            ExdDataUnits::Calories => "calories".to_string(),
            ExdDataUnits::Kilojoules => "kilojoules".to_string(),
            ExdDataUnits::Milliseconds => "milliseconds".to_string(),
            ExdDataUnits::SecondPerMile => "second_per_mile".to_string(),
            ExdDataUnits::SecondPerKilometer => "second_per_kilometer".to_string(),
            ExdDataUnits::Centimeter => "centimeter".to_string(),
            ExdDataUnits::EnumCoursePoint => "enum_course_point".to_string(),
            ExdDataUnits::Bradians => "bradians".to_string(),
            ExdDataUnits::EnumSport => "enum_sport".to_string(),
            ExdDataUnits::InchesHg => "inches_hg".to_string(),
            ExdDataUnits::MmHg => "mm_hg".to_string(),
            ExdDataUnits::Mbars => "mbars".to_string(),
            ExdDataUnits::HectoPascals => "hecto_pascals".to_string(),
            ExdDataUnits::FeetPerMin => "feet_per_min".to_string(),
            ExdDataUnits::MetersPerMin => "meters_per_min".to_string(),
            ExdDataUnits::MetersPerSec => "meters_per_sec".to_string(),
            ExdDataUnits::EightCardinal => "eight_cardinal".to_string(),
            ExdDataUnits::UnknownVariant(value) => format!("unknown_variant_{}", *value),
        }
    }
}

#[derive(Clone, Copy, Debug)]
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
    pub fn from_u8(value: u8) -> ExdQualifiers {
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
    pub fn from_i64(value: i64) -> ExdQualifiers {
        ExdQualifiers::from_u8(value as u8)
    }
    pub fn as_u8(&self) -> u8 {
        match &self {
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
            ExdQualifiers::UnknownVariant(value) => *value,
        }
    }
    pub fn to_string(&self) -> String {
        match &self {
            ExdQualifiers::NoQualifier => "no_qualifier".to_string(),
            ExdQualifiers::Instantaneous => "instantaneous".to_string(),
            ExdQualifiers::Average => "average".to_string(),
            ExdQualifiers::Lap => "lap".to_string(),
            ExdQualifiers::Maximum => "maximum".to_string(),
            ExdQualifiers::MaximumAverage => "maximum_average".to_string(),
            ExdQualifiers::MaximumLap => "maximum_lap".to_string(),
            ExdQualifiers::LastLap => "last_lap".to_string(),
            ExdQualifiers::AverageLap => "average_lap".to_string(),
            ExdQualifiers::ToDestination => "to_destination".to_string(),
            ExdQualifiers::ToGo => "to_go".to_string(),
            ExdQualifiers::ToNext => "to_next".to_string(),
            ExdQualifiers::NextCoursePoint => "next_course_point".to_string(),
            ExdQualifiers::Total => "total".to_string(),
            ExdQualifiers::ThreeSecondAverage => "three_second_average".to_string(),
            ExdQualifiers::TenSecondAverage => "ten_second_average".to_string(),
            ExdQualifiers::ThirtySecondAverage => "thirty_second_average".to_string(),
            ExdQualifiers::PercentMaximum => "percent_maximum".to_string(),
            ExdQualifiers::PercentMaximumAverage => "percent_maximum_average".to_string(),
            ExdQualifiers::LapPercentMaximum => "lap_percent_maximum".to_string(),
            ExdQualifiers::Elapsed => "elapsed".to_string(),
            ExdQualifiers::Sunrise => "sunrise".to_string(),
            ExdQualifiers::Sunset => "sunset".to_string(),
            ExdQualifiers::ComparedToVirtualPartner => "compared_to_virtual_partner".to_string(),
            ExdQualifiers::Maximum24h => "maximum_24h".to_string(),
            ExdQualifiers::Minimum24h => "minimum_24h".to_string(),
            ExdQualifiers::Minimum => "minimum".to_string(),
            ExdQualifiers::First => "first".to_string(),
            ExdQualifiers::Second => "second".to_string(),
            ExdQualifiers::Third => "third".to_string(),
            ExdQualifiers::Shifter => "shifter".to_string(),
            ExdQualifiers::LastSport => "last_sport".to_string(),
            ExdQualifiers::Moving => "moving".to_string(),
            ExdQualifiers::Stopped => "stopped".to_string(),
            ExdQualifiers::EstimatedTotal => "estimated_total".to_string(),
            ExdQualifiers::Zone9 => "zone_9".to_string(),
            ExdQualifiers::Zone8 => "zone_8".to_string(),
            ExdQualifiers::Zone7 => "zone_7".to_string(),
            ExdQualifiers::Zone6 => "zone_6".to_string(),
            ExdQualifiers::Zone5 => "zone_5".to_string(),
            ExdQualifiers::Zone4 => "zone_4".to_string(),
            ExdQualifiers::Zone3 => "zone_3".to_string(),
            ExdQualifiers::Zone2 => "zone_2".to_string(),
            ExdQualifiers::Zone1 => "zone_1".to_string(),
            ExdQualifiers::UnknownVariant(value) => format!("unknown_variant_{}", *value),
        }
    }
}

#[derive(Clone, Copy, Debug)]
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
    Gears, // Combined gear information
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
    pub fn from_u8(value: u8) -> ExdDescriptors {
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
    pub fn from_i64(value: i64) -> ExdDescriptors {
        ExdDescriptors::from_u8(value as u8)
    }
    pub fn as_u8(&self) -> u8 {
        match &self {
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
            ExdDescriptors::UnknownVariant(value) => *value,
        }
    }
    pub fn to_string(&self) -> String {
        match &self {
            ExdDescriptors::BikeLightBatteryStatus => "bike_light_battery_status".to_string(),
            ExdDescriptors::BeamAngleStatus => "beam_angle_status".to_string(),
            ExdDescriptors::BateryLevel => "batery_level".to_string(),
            ExdDescriptors::LightNetworkMode => "light_network_mode".to_string(),
            ExdDescriptors::NumberLightsConnected => "number_lights_connected".to_string(),
            ExdDescriptors::Cadence => "cadence".to_string(),
            ExdDescriptors::Distance => "distance".to_string(),
            ExdDescriptors::EstimatedTimeOfArrival => "estimated_time_of_arrival".to_string(),
            ExdDescriptors::Heading => "heading".to_string(),
            ExdDescriptors::Time => "time".to_string(),
            ExdDescriptors::BatteryLevel => "battery_level".to_string(),
            ExdDescriptors::TrainerResistance => "trainer_resistance".to_string(),
            ExdDescriptors::TrainerTargetPower => "trainer_target_power".to_string(),
            ExdDescriptors::TimeSeated => "time_seated".to_string(),
            ExdDescriptors::TimeStanding => "time_standing".to_string(),
            ExdDescriptors::Elevation => "elevation".to_string(),
            ExdDescriptors::Grade => "grade".to_string(),
            ExdDescriptors::Ascent => "ascent".to_string(),
            ExdDescriptors::Descent => "descent".to_string(),
            ExdDescriptors::VerticalSpeed => "vertical_speed".to_string(),
            ExdDescriptors::Di2BatteryLevel => "di2_battery_level".to_string(),
            ExdDescriptors::FrontGear => "front_gear".to_string(),
            ExdDescriptors::RearGear => "rear_gear".to_string(),
            ExdDescriptors::GearRatio => "gear_ratio".to_string(),
            ExdDescriptors::HeartRate => "heart_rate".to_string(),
            ExdDescriptors::HeartRateZone => "heart_rate_zone".to_string(),
            ExdDescriptors::TimeInHeartRateZone => "time_in_heart_rate_zone".to_string(),
            ExdDescriptors::HeartRateReserve => "heart_rate_reserve".to_string(),
            ExdDescriptors::Calories => "calories".to_string(),
            ExdDescriptors::GpsAccuracy => "gps_accuracy".to_string(),
            ExdDescriptors::GpsSignalStrength => "gps_signal_strength".to_string(),
            ExdDescriptors::Temperature => "temperature".to_string(),
            ExdDescriptors::TimeOfDay => "time_of_day".to_string(),
            ExdDescriptors::Balance => "balance".to_string(),
            ExdDescriptors::PedalSmoothness => "pedal_smoothness".to_string(),
            ExdDescriptors::Power => "power".to_string(),
            ExdDescriptors::FunctionalThresholdPower => "functional_threshold_power".to_string(),
            ExdDescriptors::IntensityFactor => "intensity_factor".to_string(),
            ExdDescriptors::Work => "work".to_string(),
            ExdDescriptors::PowerRatio => "power_ratio".to_string(),
            ExdDescriptors::NormalizedPower => "normalized_power".to_string(),
            ExdDescriptors::TrainingStressScore => "training_stress_Score".to_string(),
            ExdDescriptors::TimeOnZone => "time_on_zone".to_string(),
            ExdDescriptors::Speed => "speed".to_string(),
            ExdDescriptors::Laps => "laps".to_string(),
            ExdDescriptors::Reps => "reps".to_string(),
            ExdDescriptors::WorkoutStep => "workout_step".to_string(),
            ExdDescriptors::CourseDistance => "course_distance".to_string(),
            ExdDescriptors::NavigationDistance => "navigation_distance".to_string(),
            ExdDescriptors::CourseEstimatedTimeOfArrival => {
                "course_estimated_time_of_arrival".to_string()
            }
            ExdDescriptors::NavigationEstimatedTimeOfArrival => {
                "navigation_estimated_time_of_arrival".to_string()
            }
            ExdDescriptors::CourseTime => "course_time".to_string(),
            ExdDescriptors::NavigationTime => "navigation_time".to_string(),
            ExdDescriptors::CourseHeading => "course_heading".to_string(),
            ExdDescriptors::NavigationHeading => "navigation_heading".to_string(),
            ExdDescriptors::PowerZone => "power_zone".to_string(),
            ExdDescriptors::TorqueEffectiveness => "torque_effectiveness".to_string(),
            ExdDescriptors::TimerTime => "timer_time".to_string(),
            ExdDescriptors::PowerWeightRatio => "power_weight_ratio".to_string(),
            ExdDescriptors::LeftPlatformCenterOffset => "left_platform_center_offset".to_string(),
            ExdDescriptors::RightPlatformCenterOffset => "right_platform_center_offset".to_string(),
            ExdDescriptors::LeftPowerPhaseStartAngle => "left_power_phase_start_angle".to_string(),
            ExdDescriptors::RightPowerPhaseStartAngle => {
                "right_power_phase_start_angle".to_string()
            }
            ExdDescriptors::LeftPowerPhaseFinishAngle => {
                "left_power_phase_finish_angle".to_string()
            }
            ExdDescriptors::RightPowerPhaseFinishAngle => {
                "right_power_phase_finish_angle".to_string()
            }
            ExdDescriptors::Gears => "gears".to_string(),
            ExdDescriptors::Pace => "pace".to_string(),
            ExdDescriptors::TrainingEffect => "training_effect".to_string(),
            ExdDescriptors::VerticalOscillation => "vertical_oscillation".to_string(),
            ExdDescriptors::VerticalRatio => "vertical_ratio".to_string(),
            ExdDescriptors::GroundContactTime => "ground_contact_time".to_string(),
            ExdDescriptors::LeftGroundContactTimeBalance => {
                "left_ground_contact_time_balance".to_string()
            }
            ExdDescriptors::RightGroundContactTimeBalance => {
                "right_ground_contact_time_balance".to_string()
            }
            ExdDescriptors::StrideLength => "stride_length".to_string(),
            ExdDescriptors::RunningCadence => "running_cadence".to_string(),
            ExdDescriptors::PerformanceCondition => "performance_condition".to_string(),
            ExdDescriptors::CourseType => "course_type".to_string(),
            ExdDescriptors::TimeInPowerZone => "time_in_power_zone".to_string(),
            ExdDescriptors::NavigationTurn => "navigation_turn".to_string(),
            ExdDescriptors::CourseLocation => "course_location".to_string(),
            ExdDescriptors::NavigationLocation => "navigation_location".to_string(),
            ExdDescriptors::Compass => "compass".to_string(),
            ExdDescriptors::GearCombo => "gear_combo".to_string(),
            ExdDescriptors::MuscleOxygen => "muscle_oxygen".to_string(),
            ExdDescriptors::Icon => "icon".to_string(),
            ExdDescriptors::CompassHeading => "compass_heading".to_string(),
            ExdDescriptors::GpsHeading => "gps_heading".to_string(),
            ExdDescriptors::GpsElevation => "gps_elevation".to_string(),
            ExdDescriptors::AnaerobicTrainingEffect => "anaerobic_training_effect".to_string(),
            ExdDescriptors::Course => "course".to_string(),
            ExdDescriptors::OffCourse => "off_course".to_string(),
            ExdDescriptors::GlideRatio => "glide_ratio".to_string(),
            ExdDescriptors::VerticalDistance => "vertical_distance".to_string(),
            ExdDescriptors::Vmg => "vmg".to_string(),
            ExdDescriptors::AmbientPressure => "ambient_pressure".to_string(),
            ExdDescriptors::Pressure => "pressure".to_string(),
            ExdDescriptors::Vam => "vam".to_string(),
            ExdDescriptors::UnknownVariant(value) => format!("unknown_variant_{}", *value),
        }
    }
}

#[derive(Clone, Copy, Debug)]
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
    pub fn from_u32(value: u32) -> AutoActivityDetect {
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
    pub fn from_i64(value: i64) -> AutoActivityDetect {
        AutoActivityDetect::from_u32(value as u32)
    }
    pub fn as_u32(&self) -> u32 {
        match &self {
            AutoActivityDetect::None => 0,
            AutoActivityDetect::Running => 1,
            AutoActivityDetect::Cycling => 2,
            AutoActivityDetect::Swimming => 4,
            AutoActivityDetect::Walking => 8,
            AutoActivityDetect::Elliptical => 32,
            AutoActivityDetect::Sedentary => 1024,
            AutoActivityDetect::UnknownVariant(value) => *value,
        }
    }
    pub fn to_string(&self) -> String {
        match &self {
            AutoActivityDetect::None => "none".to_string(),
            AutoActivityDetect::Running => "running".to_string(),
            AutoActivityDetect::Cycling => "cycling".to_string(),
            AutoActivityDetect::Swimming => "swimming".to_string(),
            AutoActivityDetect::Walking => "walking".to_string(),
            AutoActivityDetect::Elliptical => "elliptical".to_string(),
            AutoActivityDetect::Sedentary => "sedentary".to_string(),
            AutoActivityDetect::UnknownVariant(value) => format!("unknown_variant_{}", *value),
        }
    }
}

#[derive(Clone, Copy, Debug)]
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
    pub fn from_u32(value: u32) -> SupportedExdScreenLayouts {
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
    pub fn from_i64(value: i64) -> SupportedExdScreenLayouts {
        SupportedExdScreenLayouts::from_u32(value as u32)
    }
    pub fn as_u32(&self) -> u32 {
        match &self {
            SupportedExdScreenLayouts::FullScreen => 1,
            SupportedExdScreenLayouts::HalfVertical => 2,
            SupportedExdScreenLayouts::HalfHorizontal => 4,
            SupportedExdScreenLayouts::HalfVerticalRightSplit => 8,
            SupportedExdScreenLayouts::HalfHorizontalBottomSplit => 16,
            SupportedExdScreenLayouts::FullQuarterSplit => 32,
            SupportedExdScreenLayouts::HalfVerticalLeftSplit => 64,
            SupportedExdScreenLayouts::HalfHorizontalTopSplit => 128,
            SupportedExdScreenLayouts::UnknownVariant(value) => *value,
        }
    }
    pub fn to_string(&self) -> String {
        match &self {
            SupportedExdScreenLayouts::FullScreen => "full_screen".to_string(),
            SupportedExdScreenLayouts::HalfVertical => "half_vertical".to_string(),
            SupportedExdScreenLayouts::HalfHorizontal => "half_horizontal".to_string(),
            SupportedExdScreenLayouts::HalfVerticalRightSplit => {
                "half_vertical_right_split".to_string()
            }
            SupportedExdScreenLayouts::HalfHorizontalBottomSplit => {
                "half_horizontal_bottom_split".to_string()
            }
            SupportedExdScreenLayouts::FullQuarterSplit => "full_quarter_split".to_string(),
            SupportedExdScreenLayouts::HalfVerticalLeftSplit => {
                "half_vertical_left_split".to_string()
            }
            SupportedExdScreenLayouts::HalfHorizontalTopSplit => {
                "half_horizontal_top_split".to_string()
            }
            SupportedExdScreenLayouts::UnknownVariant(value) => {
                format!("unknown_variant_{}", *value)
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
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
    pub fn from_u8(value: u8) -> FitBaseType {
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
    pub fn from_i64(value: i64) -> FitBaseType {
        FitBaseType::from_u8(value as u8)
    }
    pub fn as_u8(&self) -> u8 {
        match &self {
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
            FitBaseType::UnknownVariant(value) => *value,
        }
    }
    pub fn to_string(&self) -> String {
        match &self {
            FitBaseType::Enum => "enum".to_string(),
            FitBaseType::Sint8 => "sint8".to_string(),
            FitBaseType::Uint8 => "uint8".to_string(),
            FitBaseType::String => "string".to_string(),
            FitBaseType::Uint8z => "uint8z".to_string(),
            FitBaseType::Byte => "byte".to_string(),
            FitBaseType::Sint16 => "sint16".to_string(),
            FitBaseType::Uint16 => "uint16".to_string(),
            FitBaseType::Sint32 => "sint32".to_string(),
            FitBaseType::Uint32 => "uint32".to_string(),
            FitBaseType::Float32 => "float32".to_string(),
            FitBaseType::Float64 => "float64".to_string(),
            FitBaseType::Uint16z => "uint16z".to_string(),
            FitBaseType::Uint32z => "uint32z".to_string(),
            FitBaseType::Sint64 => "sint64".to_string(),
            FitBaseType::Uint64 => "uint64".to_string(),
            FitBaseType::Uint64z => "uint64z".to_string(),
            FitBaseType::UnknownVariant(value) => format!("unknown_variant_{}", *value),
        }
    }
}

#[derive(Clone, Copy, Debug)]
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
    pub fn from_u8(value: u8) -> TurnType {
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
    pub fn from_i64(value: i64) -> TurnType {
        TurnType::from_u8(value as u8)
    }
    pub fn as_u8(&self) -> u8 {
        match &self {
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
            TurnType::UnknownVariant(value) => *value,
        }
    }
    pub fn to_string(&self) -> String {
        match &self {
            TurnType::ArrivingIdx => "arriving_idx".to_string(),
            TurnType::ArrivingLeftIdx => "arriving_left_idx".to_string(),
            TurnType::ArrivingRightIdx => "arriving_right_idx".to_string(),
            TurnType::ArrivingViaIdx => "arriving_via_idx".to_string(),
            TurnType::ArrivingViaLeftIdx => "arriving_via_left_idx".to_string(),
            TurnType::ArrivingViaRightIdx => "arriving_via_right_idx".to_string(),
            TurnType::BearKeepLeftIdx => "bear_keep_left_idx".to_string(),
            TurnType::BearKeepRightIdx => "bear_keep_right_idx".to_string(),
            TurnType::ContinueIdx => "continue_idx".to_string(),
            TurnType::ExitLeftIdx => "exit_left_idx".to_string(),
            TurnType::ExitRightIdx => "exit_right_idx".to_string(),
            TurnType::FerryIdx => "ferry_idx".to_string(),
            TurnType::Roundabout45Idx => "roundabout_45_idx".to_string(),
            TurnType::Roundabout90Idx => "roundabout_90_idx".to_string(),
            TurnType::Roundabout135Idx => "roundabout_135_idx".to_string(),
            TurnType::Roundabout180Idx => "roundabout_180_idx".to_string(),
            TurnType::Roundabout225Idx => "roundabout_225_idx".to_string(),
            TurnType::Roundabout270Idx => "roundabout_270_idx".to_string(),
            TurnType::Roundabout315Idx => "roundabout_315_idx".to_string(),
            TurnType::Roundabout360Idx => "roundabout_360_idx".to_string(),
            TurnType::RoundaboutNeg45Idx => "roundabout_neg_45_idx".to_string(),
            TurnType::RoundaboutNeg90Idx => "roundabout_neg_90_idx".to_string(),
            TurnType::RoundaboutNeg135Idx => "roundabout_neg_135_idx".to_string(),
            TurnType::RoundaboutNeg180Idx => "roundabout_neg_180_idx".to_string(),
            TurnType::RoundaboutNeg225Idx => "roundabout_neg_225_idx".to_string(),
            TurnType::RoundaboutNeg270Idx => "roundabout_neg_270_idx".to_string(),
            TurnType::RoundaboutNeg315Idx => "roundabout_neg_315_idx".to_string(),
            TurnType::RoundaboutNeg360Idx => "roundabout_neg_360_idx".to_string(),
            TurnType::RoundaboutGenericIdx => "roundabout_generic_idx".to_string(),
            TurnType::RoundaboutNegGenericIdx => "roundabout_neg_generic_idx".to_string(),
            TurnType::SharpTurnLeftIdx => "sharp_turn_left_idx".to_string(),
            TurnType::SharpTurnRightIdx => "sharp_turn_right_idx".to_string(),
            TurnType::TurnLeftIdx => "turn_left_idx".to_string(),
            TurnType::TurnRightIdx => "turn_right_idx".to_string(),
            TurnType::UturnLeftIdx => "uturn_left_idx".to_string(),
            TurnType::UturnRightIdx => "uturn_right_idx".to_string(),
            TurnType::IconInvIdx => "icon_inv_idx".to_string(),
            TurnType::IconIdxCnt => "icon_idx_cnt".to_string(),
            TurnType::UnknownVariant(value) => format!("unknown_variant_{}", *value),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum BikeLightBeamAngleMode {
    Manual,
    Auto,
    UnknownVariant(u8),
}
impl BikeLightBeamAngleMode {
    pub fn from_u8(value: u8) -> BikeLightBeamAngleMode {
        match value {
            0 => BikeLightBeamAngleMode::Manual,
            1 => BikeLightBeamAngleMode::Auto,
            _ => BikeLightBeamAngleMode::UnknownVariant(value),
        }
    }
    pub fn from_i64(value: i64) -> BikeLightBeamAngleMode {
        BikeLightBeamAngleMode::from_u8(value as u8)
    }
    pub fn as_u8(&self) -> u8 {
        match &self {
            BikeLightBeamAngleMode::Manual => 0,
            BikeLightBeamAngleMode::Auto => 1,
            BikeLightBeamAngleMode::UnknownVariant(value) => *value,
        }
    }
    pub fn to_string(&self) -> String {
        match &self {
            BikeLightBeamAngleMode::Manual => "manual".to_string(),
            BikeLightBeamAngleMode::Auto => "auto".to_string(),
            BikeLightBeamAngleMode::UnknownVariant(value) => format!("unknown_variant_{}", *value),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum FitBaseUnit {
    Other,
    Kilogram,
    Pound,
    UnknownVariant(u16),
}
impl FitBaseUnit {
    pub fn from_u16(value: u16) -> FitBaseUnit {
        match value {
            0 => FitBaseUnit::Other,
            1 => FitBaseUnit::Kilogram,
            2 => FitBaseUnit::Pound,
            _ => FitBaseUnit::UnknownVariant(value),
        }
    }
    pub fn from_i64(value: i64) -> FitBaseUnit {
        FitBaseUnit::from_u16(value as u16)
    }
    pub fn as_u16(&self) -> u16 {
        match &self {
            FitBaseUnit::Other => 0,
            FitBaseUnit::Kilogram => 1,
            FitBaseUnit::Pound => 2,
            FitBaseUnit::UnknownVariant(value) => *value,
        }
    }
    pub fn to_string(&self) -> String {
        match &self {
            FitBaseUnit::Other => "other".to_string(),
            FitBaseUnit::Kilogram => "kilogram".to_string(),
            FitBaseUnit::Pound => "pound".to_string(),
            FitBaseUnit::UnknownVariant(value) => format!("unknown_variant_{}", *value),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum SetType {
    Rest,
    Active,
    UnknownVariant(u8),
}
impl SetType {
    pub fn from_u8(value: u8) -> SetType {
        match value {
            0 => SetType::Rest,
            1 => SetType::Active,
            _ => SetType::UnknownVariant(value),
        }
    }
    pub fn from_i64(value: i64) -> SetType {
        SetType::from_u8(value as u8)
    }
    pub fn as_u8(&self) -> u8 {
        match &self {
            SetType::Rest => 0,
            SetType::Active => 1,
            SetType::UnknownVariant(value) => *value,
        }
    }
    pub fn to_string(&self) -> String {
        match &self {
            SetType::Rest => "rest".to_string(),
            SetType::Active => "active".to_string(),
            SetType::UnknownVariant(value) => format!("unknown_variant_{}", *value),
        }
    }
}

#[derive(Clone, Copy, Debug)]
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
    pub fn from_u16(value: u16) -> ExerciseCategory {
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
    pub fn from_i64(value: i64) -> ExerciseCategory {
        ExerciseCategory::from_u16(value as u16)
    }
    pub fn as_u16(&self) -> u16 {
        match &self {
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
            ExerciseCategory::UnknownVariant(value) => *value,
        }
    }
    pub fn to_string(&self) -> String {
        match &self {
            ExerciseCategory::BenchPress => "bench_press".to_string(),
            ExerciseCategory::CalfRaise => "calf_raise".to_string(),
            ExerciseCategory::Cardio => "cardio".to_string(),
            ExerciseCategory::Carry => "carry".to_string(),
            ExerciseCategory::Chop => "chop".to_string(),
            ExerciseCategory::Core => "core".to_string(),
            ExerciseCategory::Crunch => "crunch".to_string(),
            ExerciseCategory::Curl => "curl".to_string(),
            ExerciseCategory::Deadlift => "deadlift".to_string(),
            ExerciseCategory::Flye => "flye".to_string(),
            ExerciseCategory::HipRaise => "hip_raise".to_string(),
            ExerciseCategory::HipStability => "hip_stability".to_string(),
            ExerciseCategory::HipSwing => "hip_swing".to_string(),
            ExerciseCategory::Hyperextension => "hyperextension".to_string(),
            ExerciseCategory::LateralRaise => "lateral_raise".to_string(),
            ExerciseCategory::LegCurl => "leg_curl".to_string(),
            ExerciseCategory::LegRaise => "leg_raise".to_string(),
            ExerciseCategory::Lunge => "lunge".to_string(),
            ExerciseCategory::OlympicLift => "olympic_lift".to_string(),
            ExerciseCategory::Plank => "plank".to_string(),
            ExerciseCategory::Plyo => "plyo".to_string(),
            ExerciseCategory::PullUp => "pull_up".to_string(),
            ExerciseCategory::PushUp => "push_up".to_string(),
            ExerciseCategory::Row => "row".to_string(),
            ExerciseCategory::ShoulderPress => "shoulder_press".to_string(),
            ExerciseCategory::ShoulderStability => "shoulder_stability".to_string(),
            ExerciseCategory::Shrug => "shrug".to_string(),
            ExerciseCategory::SitUp => "sit_up".to_string(),
            ExerciseCategory::Squat => "squat".to_string(),
            ExerciseCategory::TotalBody => "total_body".to_string(),
            ExerciseCategory::TricepsExtension => "triceps_extension".to_string(),
            ExerciseCategory::WarmUp => "warm_up".to_string(),
            ExerciseCategory::Run => "run".to_string(),
            ExerciseCategory::Unknown => "unknown".to_string(),
            ExerciseCategory::UnknownVariant(value) => format!("unknown_variant_{}", *value),
        }
    }
}

#[derive(Clone, Copy, Debug)]
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
    pub fn from_u16(value: u16) -> BenchPressExerciseName {
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
    pub fn from_i64(value: i64) -> BenchPressExerciseName {
        BenchPressExerciseName::from_u16(value as u16)
    }
    pub fn as_u16(&self) -> u16 {
        match &self {
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
            BenchPressExerciseName::UnknownVariant(value) => *value,
        }
    }
    pub fn to_string(&self) -> String {
        match &self {
            BenchPressExerciseName::AlternatingDumbbellChestPressOnSwissBall => {
                "alternating_dumbbell_chest_press_on_swiss_ball".to_string()
            }
            BenchPressExerciseName::BarbellBenchPress => "barbell_bench_press".to_string(),
            BenchPressExerciseName::BarbellBoardBenchPress => {
                "barbell_board_bench_press".to_string()
            }
            BenchPressExerciseName::BarbellFloorPress => "barbell_floor_press".to_string(),
            BenchPressExerciseName::CloseGripBarbellBenchPress => {
                "close_grip_barbell_bench_press".to_string()
            }
            BenchPressExerciseName::DeclineDumbbellBenchPress => {
                "decline_dumbbell_bench_press".to_string()
            }
            BenchPressExerciseName::DumbbellBenchPress => "dumbbell_bench_press".to_string(),
            BenchPressExerciseName::DumbbellFloorPress => "dumbbell_floor_press".to_string(),
            BenchPressExerciseName::InclineBarbellBenchPress => {
                "incline_barbell_bench_press".to_string()
            }
            BenchPressExerciseName::InclineDumbbellBenchPress => {
                "incline_dumbbell_bench_press".to_string()
            }
            BenchPressExerciseName::InclineSmithMachineBenchPress => {
                "incline_smith_machine_bench_press".to_string()
            }
            BenchPressExerciseName::IsometricBarbellBenchPress => {
                "isometric_barbell_bench_press".to_string()
            }
            BenchPressExerciseName::KettlebellChestPress => "kettlebell_chest_press".to_string(),
            BenchPressExerciseName::NeutralGripDumbbellBenchPress => {
                "neutral_grip_dumbbell_bench_press".to_string()
            }
            BenchPressExerciseName::NeutralGripDumbbellInclineBenchPress => {
                "neutral_grip_dumbbell_incline_bench_press".to_string()
            }
            BenchPressExerciseName::OneArmFloorPress => "one_arm_floor_press".to_string(),
            BenchPressExerciseName::WeightedOneArmFloorPress => {
                "weighted_one_arm_floor_press".to_string()
            }
            BenchPressExerciseName::PartialLockout => "partial_lockout".to_string(),
            BenchPressExerciseName::ReverseGripBarbellBenchPress => {
                "reverse_grip_barbell_bench_press".to_string()
            }
            BenchPressExerciseName::ReverseGripInclineBenchPress => {
                "reverse_grip_incline_bench_press".to_string()
            }
            BenchPressExerciseName::SingleArmCableChestPress => {
                "single_arm_cable_chest_press".to_string()
            }
            BenchPressExerciseName::SingleArmDumbbellBenchPress => {
                "single_arm_dumbbell_bench_press".to_string()
            }
            BenchPressExerciseName::SmithMachineBenchPress => {
                "smith_machine_bench_press".to_string()
            }
            BenchPressExerciseName::SwissBallDumbbellChestPress => {
                "swiss_ball_dumbbell_chest_press".to_string()
            }
            BenchPressExerciseName::TripleStopBarbellBenchPress => {
                "triple_stop_barbell_bench_press".to_string()
            }
            BenchPressExerciseName::WideGripBarbellBenchPress => {
                "wide_grip_barbell_bench_press".to_string()
            }
            BenchPressExerciseName::AlternatingDumbbellChestPress => {
                "alternating_dumbbell_chest_press".to_string()
            }
            BenchPressExerciseName::UnknownVariant(value) => format!("unknown_variant_{}", *value),
        }
    }
}

#[derive(Clone, Copy, Debug)]
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
    pub fn from_u16(value: u16) -> CalfRaiseExerciseName {
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
    pub fn from_i64(value: i64) -> CalfRaiseExerciseName {
        CalfRaiseExerciseName::from_u16(value as u16)
    }
    pub fn as_u16(&self) -> u16 {
        match &self {
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
            CalfRaiseExerciseName::UnknownVariant(value) => *value,
        }
    }
    pub fn to_string(&self) -> String {
        match &self {
            CalfRaiseExerciseName::Name3WayCalfRaise => "3_way_calf_raise".to_string(),
            CalfRaiseExerciseName::Name3WayWeightedCalfRaise => {
                "3_way_weighted_calf_raise".to_string()
            }
            CalfRaiseExerciseName::Name3WaySingleLegCalfRaise => {
                "3_way_single_leg_calf_raise".to_string()
            }
            CalfRaiseExerciseName::Name3WayWeightedSingleLegCalfRaise => {
                "3_way_weighted_single_leg_calf_raise".to_string()
            }
            CalfRaiseExerciseName::DonkeyCalfRaise => "donkey_calf_raise".to_string(),
            CalfRaiseExerciseName::WeightedDonkeyCalfRaise => {
                "weighted_donkey_calf_raise".to_string()
            }
            CalfRaiseExerciseName::SeatedCalfRaise => "seated_calf_raise".to_string(),
            CalfRaiseExerciseName::WeightedSeatedCalfRaise => {
                "weighted_seated_calf_raise".to_string()
            }
            CalfRaiseExerciseName::SeatedDumbbellToeRaise => {
                "seated_dumbbell_toe_raise".to_string()
            }
            CalfRaiseExerciseName::SingleLegBentKneeCalfRaise => {
                "single_leg_bent_knee_calf_raise".to_string()
            }
            CalfRaiseExerciseName::WeightedSingleLegBentKneeCalfRaise => {
                "weighted_single_leg_bent_knee_calf_raise".to_string()
            }
            CalfRaiseExerciseName::SingleLegDeclinePushUp => {
                "single_leg_decline_push_up".to_string()
            }
            CalfRaiseExerciseName::SingleLegDonkeyCalfRaise => {
                "single_leg_donkey_calf_raise".to_string()
            }
            CalfRaiseExerciseName::WeightedSingleLegDonkeyCalfRaise => {
                "weighted_single_leg_donkey_calf_raise".to_string()
            }
            CalfRaiseExerciseName::SingleLegHipRaiseWithKneeHold => {
                "single_leg_hip_raise_with_knee_hold".to_string()
            }
            CalfRaiseExerciseName::SingleLegStandingCalfRaise => {
                "single_leg_standing_calf_raise".to_string()
            }
            CalfRaiseExerciseName::SingleLegStandingDumbbellCalfRaise => {
                "single_leg_standing_dumbbell_calf_raise".to_string()
            }
            CalfRaiseExerciseName::StandingBarbellCalfRaise => {
                "standing_barbell_calf_raise".to_string()
            }
            CalfRaiseExerciseName::StandingCalfRaise => "standing_calf_raise".to_string(),
            CalfRaiseExerciseName::WeightedStandingCalfRaise => {
                "weighted_standing_calf_raise".to_string()
            }
            CalfRaiseExerciseName::StandingDumbbellCalfRaise => {
                "standing_dumbbell_calf_raise".to_string()
            }
            CalfRaiseExerciseName::UnknownVariant(value) => format!("unknown_variant_{}", *value),
        }
    }
}

#[derive(Clone, Copy, Debug)]
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
    pub fn from_u16(value: u16) -> CardioExerciseName {
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
    pub fn from_i64(value: i64) -> CardioExerciseName {
        CardioExerciseName::from_u16(value as u16)
    }
    pub fn as_u16(&self) -> u16 {
        match &self {
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
            CardioExerciseName::UnknownVariant(value) => *value,
        }
    }
    pub fn to_string(&self) -> String {
        match &self {
            CardioExerciseName::BobAndWeaveCircle => "bob_and_weave_circle".to_string(),
            CardioExerciseName::WeightedBobAndWeaveCircle => {
                "weighted_bob_and_weave_circle".to_string()
            }
            CardioExerciseName::CardioCoreCrawl => "cardio_core_crawl".to_string(),
            CardioExerciseName::WeightedCardioCoreCrawl => "weighted_cardio_core_crawl".to_string(),
            CardioExerciseName::DoubleUnder => "double_under".to_string(),
            CardioExerciseName::WeightedDoubleUnder => "weighted_double_under".to_string(),
            CardioExerciseName::JumpRope => "jump_rope".to_string(),
            CardioExerciseName::WeightedJumpRope => "weighted_jump_rope".to_string(),
            CardioExerciseName::JumpRopeCrossover => "jump_rope_crossover".to_string(),
            CardioExerciseName::WeightedJumpRopeCrossover => {
                "weighted_jump_rope_crossover".to_string()
            }
            CardioExerciseName::JumpRopeJog => "jump_rope_jog".to_string(),
            CardioExerciseName::WeightedJumpRopeJog => "weighted_jump_rope_jog".to_string(),
            CardioExerciseName::JumpingJacks => "jumping_jacks".to_string(),
            CardioExerciseName::WeightedJumpingJacks => "weighted_jumping_jacks".to_string(),
            CardioExerciseName::SkiMoguls => "ski_moguls".to_string(),
            CardioExerciseName::WeightedSkiMoguls => "weighted_ski_moguls".to_string(),
            CardioExerciseName::SplitJacks => "split_jacks".to_string(),
            CardioExerciseName::WeightedSplitJacks => "weighted_split_jacks".to_string(),
            CardioExerciseName::SquatJacks => "squat_jacks".to_string(),
            CardioExerciseName::WeightedSquatJacks => "weighted_squat_jacks".to_string(),
            CardioExerciseName::TripleUnder => "triple_under".to_string(),
            CardioExerciseName::WeightedTripleUnder => "weighted_triple_under".to_string(),
            CardioExerciseName::UnknownVariant(value) => format!("unknown_variant_{}", *value),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum CarryExerciseName {
    BarHolds,
    FarmersWalk,
    FarmersWalkOnToes,
    HexDumbbellHold,
    OverheadCarry,
    UnknownVariant(u16),
}
impl CarryExerciseName {
    pub fn from_u16(value: u16) -> CarryExerciseName {
        match value {
            0 => CarryExerciseName::BarHolds,
            1 => CarryExerciseName::FarmersWalk,
            2 => CarryExerciseName::FarmersWalkOnToes,
            3 => CarryExerciseName::HexDumbbellHold,
            4 => CarryExerciseName::OverheadCarry,
            _ => CarryExerciseName::UnknownVariant(value),
        }
    }
    pub fn from_i64(value: i64) -> CarryExerciseName {
        CarryExerciseName::from_u16(value as u16)
    }
    pub fn as_u16(&self) -> u16 {
        match &self {
            CarryExerciseName::BarHolds => 0,
            CarryExerciseName::FarmersWalk => 1,
            CarryExerciseName::FarmersWalkOnToes => 2,
            CarryExerciseName::HexDumbbellHold => 3,
            CarryExerciseName::OverheadCarry => 4,
            CarryExerciseName::UnknownVariant(value) => *value,
        }
    }
    pub fn to_string(&self) -> String {
        match &self {
            CarryExerciseName::BarHolds => "bar_holds".to_string(),
            CarryExerciseName::FarmersWalk => "farmers_walk".to_string(),
            CarryExerciseName::FarmersWalkOnToes => "farmers_walk_on_toes".to_string(),
            CarryExerciseName::HexDumbbellHold => "hex_dumbbell_hold".to_string(),
            CarryExerciseName::OverheadCarry => "overhead_carry".to_string(),
            CarryExerciseName::UnknownVariant(value) => format!("unknown_variant_{}", *value),
        }
    }
}

#[derive(Clone, Copy, Debug)]
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
    pub fn from_u16(value: u16) -> ChopExerciseName {
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
    pub fn from_i64(value: i64) -> ChopExerciseName {
        ChopExerciseName::from_u16(value as u16)
    }
    pub fn as_u16(&self) -> u16 {
        match &self {
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
            ChopExerciseName::UnknownVariant(value) => *value,
        }
    }
    pub fn to_string(&self) -> String {
        match &self {
            ChopExerciseName::CablePullThrough => "cable_pull_through".to_string(),
            ChopExerciseName::CableRotationalLift => "cable_rotational_lift".to_string(),
            ChopExerciseName::CableWoodchop => "cable_woodchop".to_string(),
            ChopExerciseName::CrossChopToKnee => "cross_chop_to_knee".to_string(),
            ChopExerciseName::WeightedCrossChopToKnee => "weighted_cross_chop_to_knee".to_string(),
            ChopExerciseName::DumbbellChop => "dumbbell_chop".to_string(),
            ChopExerciseName::HalfKneelingRotation => "half_kneeling_rotation".to_string(),
            ChopExerciseName::WeightedHalfKneelingRotation => {
                "weighted_half_kneeling_rotation".to_string()
            }
            ChopExerciseName::HalfKneelingRotationalChop => {
                "half_kneeling_rotational_chop".to_string()
            }
            ChopExerciseName::HalfKneelingRotationalReverseChop => {
                "half_kneeling_rotational_reverse_chop".to_string()
            }
            ChopExerciseName::HalfKneelingStabilityChop => {
                "half_kneeling_stability_chop".to_string()
            }
            ChopExerciseName::HalfKneelingStabilityReverseChop => {
                "half_kneeling_stability_reverse_chop".to_string()
            }
            ChopExerciseName::KneelingRotationalChop => "kneeling_rotational_chop".to_string(),
            ChopExerciseName::KneelingRotationalReverseChop => {
                "kneeling_rotational_reverse_chop".to_string()
            }
            ChopExerciseName::KneelingStabilityChop => "kneeling_stability_chop".to_string(),
            ChopExerciseName::KneelingWoodchopper => "kneeling_woodchopper".to_string(),
            ChopExerciseName::MedicineBallWoodChops => "medicine_ball_wood_chops".to_string(),
            ChopExerciseName::PowerSquatChops => "power_squat_chops".to_string(),
            ChopExerciseName::WeightedPowerSquatChops => "weighted_power_squat_chops".to_string(),
            ChopExerciseName::StandingRotationalChop => "standing_rotational_chop".to_string(),
            ChopExerciseName::StandingSplitRotationalChop => {
                "standing_split_rotational_chop".to_string()
            }
            ChopExerciseName::StandingSplitRotationalReverseChop => {
                "standing_split_rotational_reverse_chop".to_string()
            }
            ChopExerciseName::StandingStabilityReverseChop => {
                "standing_stability_reverse_chop".to_string()
            }
            ChopExerciseName::UnknownVariant(value) => format!("unknown_variant_{}", *value),
        }
    }
}

#[derive(Clone, Copy, Debug)]
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
    pub fn from_u16(value: u16) -> CoreExerciseName {
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
    pub fn from_i64(value: i64) -> CoreExerciseName {
        CoreExerciseName::from_u16(value as u16)
    }
    pub fn as_u16(&self) -> u16 {
        match &self {
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
            CoreExerciseName::UnknownVariant(value) => *value,
        }
    }
    pub fn to_string(&self) -> String {
        match &self {
            CoreExerciseName::AbsJabs => "abs_jabs".to_string(),
            CoreExerciseName::WeightedAbsJabs => "weighted_abs_jabs".to_string(),
            CoreExerciseName::AlternatingPlateReach => "alternating_plate_reach".to_string(),
            CoreExerciseName::BarbellRollout => "barbell_rollout".to_string(),
            CoreExerciseName::WeightedBarbellRollout => "weighted_barbell_rollout".to_string(),
            CoreExerciseName::BodyBarObliqueTwist => "body_bar_oblique_twist".to_string(),
            CoreExerciseName::CableCorePress => "cable_core_press".to_string(),
            CoreExerciseName::CableSideBend => "cable_side_bend".to_string(),
            CoreExerciseName::SideBend => "side_bend".to_string(),
            CoreExerciseName::WeightedSideBend => "weighted_side_bend".to_string(),
            CoreExerciseName::CrescentCircle => "crescent_circle".to_string(),
            CoreExerciseName::WeightedCrescentCircle => "weighted_crescent_circle".to_string(),
            CoreExerciseName::CyclingRussianTwist => "cycling_russian_twist".to_string(),
            CoreExerciseName::WeightedCyclingRussianTwist => {
                "weighted_cycling_russian_twist".to_string()
            }
            CoreExerciseName::ElevatedFeetRussianTwist => "elevated_feet_russian_twist".to_string(),
            CoreExerciseName::WeightedElevatedFeetRussianTwist => {
                "weighted_elevated_feet_russian_twist".to_string()
            }
            CoreExerciseName::HalfTurkishGetUp => "half_turkish_get_up".to_string(),
            CoreExerciseName::KettlebellWindmill => "kettlebell_windmill".to_string(),
            CoreExerciseName::KneelingAbWheel => "kneeling_ab_wheel".to_string(),
            CoreExerciseName::WeightedKneelingAbWheel => "weighted_kneeling_ab_wheel".to_string(),
            CoreExerciseName::ModifiedFrontLever => "modified_front_lever".to_string(),
            CoreExerciseName::OpenKneeTucks => "open_knee_tucks".to_string(),
            CoreExerciseName::WeightedOpenKneeTucks => "weighted_open_knee_tucks".to_string(),
            CoreExerciseName::SideAbsLegLift => "side_abs_leg_lift".to_string(),
            CoreExerciseName::WeightedSideAbsLegLift => "weighted_side_abs_leg_lift".to_string(),
            CoreExerciseName::SwissBallJackknife => "swiss_ball_jackknife".to_string(),
            CoreExerciseName::WeightedSwissBallJackknife => {
                "weighted_swiss_ball_jackknife".to_string()
            }
            CoreExerciseName::SwissBallPike => "swiss_ball_pike".to_string(),
            CoreExerciseName::WeightedSwissBallPike => "weighted_swiss_ball_pike".to_string(),
            CoreExerciseName::SwissBallRollout => "swiss_ball_rollout".to_string(),
            CoreExerciseName::WeightedSwissBallRollout => "weighted_swiss_ball_rollout".to_string(),
            CoreExerciseName::TriangleHipPress => "triangle_hip_press".to_string(),
            CoreExerciseName::WeightedTriangleHipPress => "weighted_triangle_hip_press".to_string(),
            CoreExerciseName::TrxSuspendedJackknife => "trx_suspended_jackknife".to_string(),
            CoreExerciseName::WeightedTrxSuspendedJackknife => {
                "weighted_trx_suspended_jackknife".to_string()
            }
            CoreExerciseName::UBoat => "u_boat".to_string(),
            CoreExerciseName::WeightedUBoat => "weighted_u_boat".to_string(),
            CoreExerciseName::WindmillSwitches => "windmill_switches".to_string(),
            CoreExerciseName::WeightedWindmillSwitches => "weighted_windmill_switches".to_string(),
            CoreExerciseName::AlternatingSlideOut => "alternating_slide_out".to_string(),
            CoreExerciseName::WeightedAlternatingSlideOut => {
                "weighted_alternating_slide_out".to_string()
            }
            CoreExerciseName::GhdBackExtensions => "ghd_back_extensions".to_string(),
            CoreExerciseName::WeightedGhdBackExtensions => {
                "weighted_ghd_back_extensions".to_string()
            }
            CoreExerciseName::OverheadWalk => "overhead_walk".to_string(),
            CoreExerciseName::Inchworm => "inchworm".to_string(),
            CoreExerciseName::WeightedModifiedFrontLever => {
                "weighted_modified_front_lever".to_string()
            }
            CoreExerciseName::RussianTwist => "russian_twist".to_string(),
            CoreExerciseName::AbdominalLegRotations => "abdominal_leg_rotations".to_string(),
            CoreExerciseName::ArmAndLegExtensionOnKnees => {
                "arm_and_leg_extension_on_knees".to_string()
            }
            CoreExerciseName::Bicycle => "bicycle".to_string(),
            CoreExerciseName::BicepCurlWithLegExtension => {
                "bicep_curl_with_leg_extension".to_string()
            }
            CoreExerciseName::CatCow => "cat_cow".to_string(),
            CoreExerciseName::Corkscrew => "corkscrew".to_string(),
            CoreExerciseName::CrissCross => "criss_cross".to_string(),
            CoreExerciseName::CrissCrossWithBall => "criss_cross_with_ball".to_string(),
            CoreExerciseName::DoubleLegStretch => "double_leg_stretch".to_string(),
            CoreExerciseName::KneeFolds => "knee_folds".to_string(),
            CoreExerciseName::LowerLift => "lower_lift".to_string(),
            CoreExerciseName::NeckPull => "neck_pull".to_string(),
            CoreExerciseName::PelvicClocks => "pelvic_clocks".to_string(),
            CoreExerciseName::RollOver => "roll_over".to_string(),
            CoreExerciseName::RollUp => "roll_up".to_string(),
            CoreExerciseName::Rolling => "rolling".to_string(),
            CoreExerciseName::Rowing1 => "rowing_1".to_string(),
            CoreExerciseName::Rowing2 => "rowing_2".to_string(),
            CoreExerciseName::Scissors => "scissors".to_string(),
            CoreExerciseName::SingleLegCircles => "single_leg_circles".to_string(),
            CoreExerciseName::SingleLegStretch => "single_leg_stretch".to_string(),
            CoreExerciseName::SnakeTwist1And2 => "snake_twist_1_and_2".to_string(),
            CoreExerciseName::Swan => "swan".to_string(),
            CoreExerciseName::Swimming => "swimming".to_string(),
            CoreExerciseName::Teaser => "teaser".to_string(),
            CoreExerciseName::TheHundred => "the_hundred".to_string(),
            CoreExerciseName::UnknownVariant(value) => format!("unknown_variant_{}", *value),
        }
    }
}

#[derive(Clone, Copy, Debug)]
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
    pub fn from_u16(value: u16) -> CrunchExerciseName {
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
    pub fn from_i64(value: i64) -> CrunchExerciseName {
        CrunchExerciseName::from_u16(value as u16)
    }
    pub fn as_u16(&self) -> u16 {
        match &self {
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
            CrunchExerciseName::UnknownVariant(value) => *value,
        }
    }
    pub fn to_string(&self) -> String {
        match &self {
            CrunchExerciseName::BicycleCrunch => "bicycle_crunch".to_string(),
            CrunchExerciseName::CableCrunch => "cable_crunch".to_string(),
            CrunchExerciseName::CircularArmCrunch => "circular_arm_crunch".to_string(),
            CrunchExerciseName::CrossedArmsCrunch => "crossed_arms_crunch".to_string(),
            CrunchExerciseName::WeightedCrossedArmsCrunch => {
                "weighted_crossed_arms_crunch".to_string()
            }
            CrunchExerciseName::CrossLegReverseCrunch => "cross_leg_reverse_crunch".to_string(),
            CrunchExerciseName::WeightedCrossLegReverseCrunch => {
                "weighted_cross_leg_reverse_crunch".to_string()
            }
            CrunchExerciseName::CrunchChop => "crunch_chop".to_string(),
            CrunchExerciseName::WeightedCrunchChop => "weighted_crunch_chop".to_string(),
            CrunchExerciseName::DoubleCrunch => "double_crunch".to_string(),
            CrunchExerciseName::WeightedDoubleCrunch => "weighted_double_crunch".to_string(),
            CrunchExerciseName::ElbowToKneeCrunch => "elbow_to_knee_crunch".to_string(),
            CrunchExerciseName::WeightedElbowToKneeCrunch => {
                "weighted_elbow_to_knee_crunch".to_string()
            }
            CrunchExerciseName::FlutterKicks => "flutter_kicks".to_string(),
            CrunchExerciseName::WeightedFlutterKicks => "weighted_flutter_kicks".to_string(),
            CrunchExerciseName::FoamRollerReverseCrunchOnBench => {
                "foam_roller_reverse_crunch_on_bench".to_string()
            }
            CrunchExerciseName::WeightedFoamRollerReverseCrunchOnBench => {
                "weighted_foam_roller_reverse_crunch_on_bench".to_string()
            }
            CrunchExerciseName::FoamRollerReverseCrunchWithDumbbell => {
                "foam_roller_reverse_crunch_with_dumbbell".to_string()
            }
            CrunchExerciseName::FoamRollerReverseCrunchWithMedicineBall => {
                "foam_roller_reverse_crunch_with_medicine_ball".to_string()
            }
            CrunchExerciseName::FrogPress => "frog_press".to_string(),
            CrunchExerciseName::HangingKneeRaiseObliqueCrunch => {
                "hanging_knee_raise_oblique_crunch".to_string()
            }
            CrunchExerciseName::WeightedHangingKneeRaiseObliqueCrunch => {
                "weighted_hanging_knee_raise_oblique_crunch".to_string()
            }
            CrunchExerciseName::HipCrossover => "hip_crossover".to_string(),
            CrunchExerciseName::WeightedHipCrossover => "weighted_hip_crossover".to_string(),
            CrunchExerciseName::HollowRock => "hollow_rock".to_string(),
            CrunchExerciseName::WeightedHollowRock => "weighted_hollow_rock".to_string(),
            CrunchExerciseName::InclineReverseCrunch => "incline_reverse_crunch".to_string(),
            CrunchExerciseName::WeightedInclineReverseCrunch => {
                "weighted_incline_reverse_crunch".to_string()
            }
            CrunchExerciseName::KneelingCableCrunch => "kneeling_cable_crunch".to_string(),
            CrunchExerciseName::KneelingCrossCrunch => "kneeling_cross_crunch".to_string(),
            CrunchExerciseName::WeightedKneelingCrossCrunch => {
                "weighted_kneeling_cross_crunch".to_string()
            }
            CrunchExerciseName::KneelingObliqueCableCrunch => {
                "kneeling_oblique_cable_crunch".to_string()
            }
            CrunchExerciseName::KneesToElbow => "knees_to_elbow".to_string(),
            CrunchExerciseName::LegExtensions => "leg_extensions".to_string(),
            CrunchExerciseName::WeightedLegExtensions => "weighted_leg_extensions".to_string(),
            CrunchExerciseName::LegLevers => "leg_levers".to_string(),
            CrunchExerciseName::McgillCurlUp => "mcgill_curl_up".to_string(),
            CrunchExerciseName::WeightedMcgillCurlUp => "weighted_mcgill_curl_up".to_string(),
            CrunchExerciseName::ModifiedPilatesRollUpWithBall => {
                "modified_pilates_roll_up_with_ball".to_string()
            }
            CrunchExerciseName::WeightedModifiedPilatesRollUpWithBall => {
                "weighted_modified_pilates_roll_up_with_ball".to_string()
            }
            CrunchExerciseName::PilatesCrunch => "pilates_crunch".to_string(),
            CrunchExerciseName::WeightedPilatesCrunch => "weighted_pilates_crunch".to_string(),
            CrunchExerciseName::PilatesRollUpWithBall => "pilates_roll_up_with_ball".to_string(),
            CrunchExerciseName::WeightedPilatesRollUpWithBall => {
                "weighted_pilates_roll_up_with_ball".to_string()
            }
            CrunchExerciseName::RaisedLegsCrunch => "raised_legs_crunch".to_string(),
            CrunchExerciseName::WeightedRaisedLegsCrunch => {
                "weighted_raised_legs_crunch".to_string()
            }
            CrunchExerciseName::ReverseCrunch => "reverse_crunch".to_string(),
            CrunchExerciseName::WeightedReverseCrunch => "weighted_reverse_crunch".to_string(),
            CrunchExerciseName::ReverseCrunchOnABench => "reverse_crunch_on_a_bench".to_string(),
            CrunchExerciseName::WeightedReverseCrunchOnABench => {
                "weighted_reverse_crunch_on_a_bench".to_string()
            }
            CrunchExerciseName::ReverseCurlAndLift => "reverse_curl_and_lift".to_string(),
            CrunchExerciseName::WeightedReverseCurlAndLift => {
                "weighted_reverse_curl_and_lift".to_string()
            }
            CrunchExerciseName::RotationalLift => "rotational_lift".to_string(),
            CrunchExerciseName::WeightedRotationalLift => "weighted_rotational_lift".to_string(),
            CrunchExerciseName::SeatedAlternatingReverseCrunch => {
                "seated_alternating_reverse_crunch".to_string()
            }
            CrunchExerciseName::WeightedSeatedAlternatingReverseCrunch => {
                "weighted_seated_alternating_reverse_crunch".to_string()
            }
            CrunchExerciseName::SeatedLegU => "seated_leg_u".to_string(),
            CrunchExerciseName::WeightedSeatedLegU => "weighted_seated_leg_u".to_string(),
            CrunchExerciseName::SideToSideCrunchAndWeave => {
                "side_to_side_crunch_and_weave".to_string()
            }
            CrunchExerciseName::WeightedSideToSideCrunchAndWeave => {
                "weighted_side_to_side_crunch_and_weave".to_string()
            }
            CrunchExerciseName::SingleLegReverseCrunch => "single_leg_reverse_crunch".to_string(),
            CrunchExerciseName::WeightedSingleLegReverseCrunch => {
                "weighted_single_leg_reverse_crunch".to_string()
            }
            CrunchExerciseName::SkaterCrunchCross => "skater_crunch_cross".to_string(),
            CrunchExerciseName::WeightedSkaterCrunchCross => {
                "weighted_skater_crunch_cross".to_string()
            }
            CrunchExerciseName::StandingCableCrunch => "standing_cable_crunch".to_string(),
            CrunchExerciseName::StandingSideCrunch => "standing_side_crunch".to_string(),
            CrunchExerciseName::StepClimb => "step_climb".to_string(),
            CrunchExerciseName::WeightedStepClimb => "weighted_step_climb".to_string(),
            CrunchExerciseName::SwissBallCrunch => "swiss_ball_crunch".to_string(),
            CrunchExerciseName::SwissBallReverseCrunch => "swiss_ball_reverse_crunch".to_string(),
            CrunchExerciseName::WeightedSwissBallReverseCrunch => {
                "weighted_swiss_ball_reverse_crunch".to_string()
            }
            CrunchExerciseName::SwissBallRussianTwist => "swiss_ball_russian_twist".to_string(),
            CrunchExerciseName::WeightedSwissBallRussianTwist => {
                "weighted_swiss_ball_russian_twist".to_string()
            }
            CrunchExerciseName::SwissBallSideCrunch => "swiss_ball_side_crunch".to_string(),
            CrunchExerciseName::WeightedSwissBallSideCrunch => {
                "weighted_swiss_ball_side_crunch".to_string()
            }
            CrunchExerciseName::ThoracicCrunchesOnFoamRoller => {
                "thoracic_crunches_on_foam_roller".to_string()
            }
            CrunchExerciseName::WeightedThoracicCrunchesOnFoamRoller => {
                "weighted_thoracic_crunches_on_foam_roller".to_string()
            }
            CrunchExerciseName::TricepsCrunch => "triceps_crunch".to_string(),
            CrunchExerciseName::WeightedBicycleCrunch => "weighted_bicycle_crunch".to_string(),
            CrunchExerciseName::WeightedCrunch => "weighted_crunch".to_string(),
            CrunchExerciseName::WeightedSwissBallCrunch => "weighted_swiss_ball_crunch".to_string(),
            CrunchExerciseName::ToesToBar => "toes_to_bar".to_string(),
            CrunchExerciseName::WeightedToesToBar => "weighted_toes_to_bar".to_string(),
            CrunchExerciseName::Crunch => "crunch".to_string(),
            CrunchExerciseName::StraightLegCrunchWithBall => {
                "straight_leg_crunch_with_ball".to_string()
            }
            CrunchExerciseName::UnknownVariant(value) => format!("unknown_variant_{}", *value),
        }
    }
}

#[derive(Clone, Copy, Debug)]
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
    pub fn from_u16(value: u16) -> CurlExerciseName {
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
    pub fn from_i64(value: i64) -> CurlExerciseName {
        CurlExerciseName::from_u16(value as u16)
    }
    pub fn as_u16(&self) -> u16 {
        match &self {
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
            CurlExerciseName::UnknownVariant(value) => *value,
        }
    }
    pub fn to_string(&self) -> String {
        match &self {
            CurlExerciseName::AlternatingDumbbellBicepsCurl => {
                "alternating_dumbbell_biceps_curl".to_string()
            }
            CurlExerciseName::AlternatingDumbbellBicepsCurlOnSwissBall => {
                "alternating_dumbbell_biceps_curl_on_swiss_ball".to_string()
            }
            CurlExerciseName::AlternatingInclineDumbbellBicepsCurl => {
                "alternating_incline_dumbbell_biceps_curl".to_string()
            }
            CurlExerciseName::BarbellBicepsCurl => "barbell_biceps_curl".to_string(),
            CurlExerciseName::BarbellReverseWristCurl => "barbell_reverse_wrist_curl".to_string(),
            CurlExerciseName::BarbellWristCurl => "barbell_wrist_curl".to_string(),
            CurlExerciseName::BehindTheBackBarbellReverseWristCurl => {
                "behind_the_back_barbell_reverse_wrist_curl".to_string()
            }
            CurlExerciseName::BehindTheBackOneArmCableCurl => {
                "behind_the_back_one_arm_cable_curl".to_string()
            }
            CurlExerciseName::CableBicepsCurl => "cable_biceps_curl".to_string(),
            CurlExerciseName::CableHammerCurl => "cable_hammer_curl".to_string(),
            CurlExerciseName::CheatingBarbellBicepsCurl => {
                "cheating_barbell_biceps_curl".to_string()
            }
            CurlExerciseName::CloseGripEzBarBicepsCurl => {
                "close_grip_ez_bar_biceps_curl".to_string()
            }
            CurlExerciseName::CrossBodyDumbbellHammerCurl => {
                "cross_body_dumbbell_hammer_curl".to_string()
            }
            CurlExerciseName::DeadHangBicepsCurl => "dead_hang_biceps_curl".to_string(),
            CurlExerciseName::DeclineHammerCurl => "decline_hammer_curl".to_string(),
            CurlExerciseName::DumbbellBicepsCurlWithStaticHold => {
                "dumbbell_biceps_curl_with_static_hold".to_string()
            }
            CurlExerciseName::DumbbellHammerCurl => "dumbbell_hammer_curl".to_string(),
            CurlExerciseName::DumbbellReverseWristCurl => "dumbbell_reverse_wrist_curl".to_string(),
            CurlExerciseName::DumbbellWristCurl => "dumbbell_wrist_curl".to_string(),
            CurlExerciseName::EzBarPreacherCurl => "ez_bar_preacher_curl".to_string(),
            CurlExerciseName::ForwardBendBicepsCurl => "forward_bend_biceps_curl".to_string(),
            CurlExerciseName::HammerCurlToPress => "hammer_curl_to_press".to_string(),
            CurlExerciseName::InclineDumbbellBicepsCurl => {
                "incline_dumbbell_biceps_curl".to_string()
            }
            CurlExerciseName::InclineOffsetThumbDumbbellCurl => {
                "incline_offset_thumb_dumbbell_curl".to_string()
            }
            CurlExerciseName::KettlebellBicepsCurl => "kettlebell_biceps_curl".to_string(),
            CurlExerciseName::LyingConcentrationCableCurl => {
                "lying_concentration_cable_curl".to_string()
            }
            CurlExerciseName::OneArmPreacherCurl => "one_arm_preacher_curl".to_string(),
            CurlExerciseName::PlatePinchCurl => "plate_pinch_curl".to_string(),
            CurlExerciseName::PreacherCurlWithCable => "preacher_curl_with_cable".to_string(),
            CurlExerciseName::ReverseEzBarCurl => "reverse_ez_bar_curl".to_string(),
            CurlExerciseName::ReverseGripWristCurl => "reverse_grip_wrist_curl".to_string(),
            CurlExerciseName::ReverseGripBarbellBicepsCurl => {
                "reverse_grip_barbell_biceps_curl".to_string()
            }
            CurlExerciseName::SeatedAlternatingDumbbellBicepsCurl => {
                "seated_alternating_dumbbell_biceps_curl".to_string()
            }
            CurlExerciseName::SeatedDumbbellBicepsCurl => "seated_dumbbell_biceps_curl".to_string(),
            CurlExerciseName::SeatedReverseDumbbellCurl => {
                "seated_reverse_dumbbell_curl".to_string()
            }
            CurlExerciseName::SplitStanceOffsetPinkyDumbbellCurl => {
                "split_stance_offset_pinky_dumbbell_curl".to_string()
            }
            CurlExerciseName::StandingAlternatingDumbbellCurls => {
                "standing_alternating_dumbbell_curls".to_string()
            }
            CurlExerciseName::StandingDumbbellBicepsCurl => {
                "standing_dumbbell_biceps_curl".to_string()
            }
            CurlExerciseName::StandingEzBarBicepsCurl => "standing_ez_bar_biceps_curl".to_string(),
            CurlExerciseName::StaticCurl => "static_curl".to_string(),
            CurlExerciseName::SwissBallDumbbellOverheadTricepsExtension => {
                "swiss_ball_dumbbell_overhead_triceps_extension".to_string()
            }
            CurlExerciseName::SwissBallEzBarPreacherCurl => {
                "swiss_ball_ez_bar_preacher_curl".to_string()
            }
            CurlExerciseName::TwistingStandingDumbbellBicepsCurl => {
                "twisting_standing_dumbbell_biceps_curl".to_string()
            }
            CurlExerciseName::WideGripEzBarBicepsCurl => "wide_grip_ez_bar_biceps_curl".to_string(),
            CurlExerciseName::UnknownVariant(value) => format!("unknown_variant_{}", *value),
        }
    }
}

#[derive(Clone, Copy, Debug)]
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
    pub fn from_u16(value: u16) -> DeadliftExerciseName {
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
    pub fn from_i64(value: i64) -> DeadliftExerciseName {
        DeadliftExerciseName::from_u16(value as u16)
    }
    pub fn as_u16(&self) -> u16 {
        match &self {
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
            DeadliftExerciseName::UnknownVariant(value) => *value,
        }
    }
    pub fn to_string(&self) -> String {
        match &self {
            DeadliftExerciseName::BarbellDeadlift => "barbell_deadlift".to_string(),
            DeadliftExerciseName::BarbellStraightLegDeadlift => {
                "barbell_straight_leg_deadlift".to_string()
            }
            DeadliftExerciseName::DumbbellDeadlift => "dumbbell_deadlift".to_string(),
            DeadliftExerciseName::DumbbellSingleLegDeadliftToRow => {
                "dumbbell_single_leg_deadlift_to_row".to_string()
            }
            DeadliftExerciseName::DumbbellStraightLegDeadlift => {
                "dumbbell_straight_leg_deadlift".to_string()
            }
            DeadliftExerciseName::KettlebellFloorToShelf => "kettlebell_floor_to_shelf".to_string(),
            DeadliftExerciseName::OneArmOneLegDeadlift => "one_arm_one_leg_deadlift".to_string(),
            DeadliftExerciseName::RackPull => "rack_pull".to_string(),
            DeadliftExerciseName::RotationalDumbbellStraightLegDeadlift => {
                "rotational_dumbbell_straight_leg_deadlift".to_string()
            }
            DeadliftExerciseName::SingleArmDeadlift => "single_arm_deadlift".to_string(),
            DeadliftExerciseName::SingleLegBarbellDeadlift => {
                "single_leg_barbell_deadlift".to_string()
            }
            DeadliftExerciseName::SingleLegBarbellStraightLegDeadlift => {
                "single_leg_barbell_straight_leg_deadlift".to_string()
            }
            DeadliftExerciseName::SingleLegDeadliftWithBarbell => {
                "single_leg_deadlift_with_barbell".to_string()
            }
            DeadliftExerciseName::SingleLegRdlCircuit => "single_leg_rdl_circuit".to_string(),
            DeadliftExerciseName::SingleLegRomanianDeadliftWithDumbbell => {
                "single_leg_romanian_deadlift_with_dumbbell".to_string()
            }
            DeadliftExerciseName::SumoDeadlift => "sumo_deadlift".to_string(),
            DeadliftExerciseName::SumoDeadliftHighPull => "sumo_deadlift_high_pull".to_string(),
            DeadliftExerciseName::TrapBarDeadlift => "trap_bar_deadlift".to_string(),
            DeadliftExerciseName::WideGripBarbellDeadlift => {
                "wide_grip_barbell_deadlift".to_string()
            }
            DeadliftExerciseName::UnknownVariant(value) => format!("unknown_variant_{}", *value),
        }
    }
}

#[derive(Clone, Copy, Debug)]
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
    pub fn from_u16(value: u16) -> FlyeExerciseName {
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
    pub fn from_i64(value: i64) -> FlyeExerciseName {
        FlyeExerciseName::from_u16(value as u16)
    }
    pub fn as_u16(&self) -> u16 {
        match &self {
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
            FlyeExerciseName::UnknownVariant(value) => *value,
        }
    }
    pub fn to_string(&self) -> String {
        match &self {
            FlyeExerciseName::CableCrossover => "cable_crossover".to_string(),
            FlyeExerciseName::DeclineDumbbellFlye => "decline_dumbbell_flye".to_string(),
            FlyeExerciseName::DumbbellFlye => "dumbbell_flye".to_string(),
            FlyeExerciseName::InclineDumbbellFlye => "incline_dumbbell_flye".to_string(),
            FlyeExerciseName::KettlebellFlye => "kettlebell_flye".to_string(),
            FlyeExerciseName::KneelingRearFlye => "kneeling_rear_flye".to_string(),
            FlyeExerciseName::SingleArmStandingCableReverseFlye => {
                "single_arm_standing_cable_reverse_flye".to_string()
            }
            FlyeExerciseName::SwissBallDumbbellFlye => "swiss_ball_dumbbell_flye".to_string(),
            FlyeExerciseName::ArmRotations => "arm_rotations".to_string(),
            FlyeExerciseName::HugATree => "hug_a_tree".to_string(),
            FlyeExerciseName::UnknownVariant(value) => format!("unknown_variant_{}", *value),
        }
    }
}

#[derive(Clone, Copy, Debug)]
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
    pub fn from_u16(value: u16) -> HipRaiseExerciseName {
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
    pub fn from_i64(value: i64) -> HipRaiseExerciseName {
        HipRaiseExerciseName::from_u16(value as u16)
    }
    pub fn as_u16(&self) -> u16 {
        match &self {
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
            HipRaiseExerciseName::UnknownVariant(value) => *value,
        }
    }
    pub fn to_string(&self) -> String {
        match &self {
            HipRaiseExerciseName::BarbellHipThrustOnFloor => {
                "barbell_hip_thrust_on_floor".to_string()
            }
            HipRaiseExerciseName::BarbellHipThrustWithBench => {
                "barbell_hip_thrust_with_bench".to_string()
            }
            HipRaiseExerciseName::BentKneeSwissBallReverseHipRaise => {
                "bent_knee_swiss_ball_reverse_hip_raise".to_string()
            }
            HipRaiseExerciseName::WeightedBentKneeSwissBallReverseHipRaise => {
                "weighted_bent_knee_swiss_ball_reverse_hip_raise".to_string()
            }
            HipRaiseExerciseName::BridgeWithLegExtension => "bridge_with_leg_extension".to_string(),
            HipRaiseExerciseName::WeightedBridgeWithLegExtension => {
                "weighted_bridge_with_leg_extension".to_string()
            }
            HipRaiseExerciseName::ClamBridge => "clam_bridge".to_string(),
            HipRaiseExerciseName::FrontKickTabletop => "front_kick_tabletop".to_string(),
            HipRaiseExerciseName::WeightedFrontKickTabletop => {
                "weighted_front_kick_tabletop".to_string()
            }
            HipRaiseExerciseName::HipExtensionAndCross => "hip_extension_and_cross".to_string(),
            HipRaiseExerciseName::WeightedHipExtensionAndCross => {
                "weighted_hip_extension_and_cross".to_string()
            }
            HipRaiseExerciseName::HipRaise => "hip_raise".to_string(),
            HipRaiseExerciseName::WeightedHipRaise => "weighted_hip_raise".to_string(),
            HipRaiseExerciseName::HipRaiseWithFeetOnSwissBall => {
                "hip_raise_with_feet_on_swiss_ball".to_string()
            }
            HipRaiseExerciseName::WeightedHipRaiseWithFeetOnSwissBall => {
                "weighted_hip_raise_with_feet_on_swiss_ball".to_string()
            }
            HipRaiseExerciseName::HipRaiseWithHeadOnBosuBall => {
                "hip_raise_with_head_on_bosu_ball".to_string()
            }
            HipRaiseExerciseName::WeightedHipRaiseWithHeadOnBosuBall => {
                "weighted_hip_raise_with_head_on_bosu_ball".to_string()
            }
            HipRaiseExerciseName::HipRaiseWithHeadOnSwissBall => {
                "hip_raise_with_head_on_swiss_ball".to_string()
            }
            HipRaiseExerciseName::WeightedHipRaiseWithHeadOnSwissBall => {
                "weighted_hip_raise_with_head_on_swiss_ball".to_string()
            }
            HipRaiseExerciseName::HipRaiseWithKneeSqueeze => {
                "hip_raise_with_knee_squeeze".to_string()
            }
            HipRaiseExerciseName::WeightedHipRaiseWithKneeSqueeze => {
                "weighted_hip_raise_with_knee_squeeze".to_string()
            }
            HipRaiseExerciseName::InclineRearLegExtension => {
                "incline_rear_leg_extension".to_string()
            }
            HipRaiseExerciseName::WeightedInclineRearLegExtension => {
                "weighted_incline_rear_leg_extension".to_string()
            }
            HipRaiseExerciseName::KettlebellSwing => "kettlebell_swing".to_string(),
            HipRaiseExerciseName::MarchingHipRaise => "marching_hip_raise".to_string(),
            HipRaiseExerciseName::WeightedMarchingHipRaise => {
                "weighted_marching_hip_raise".to_string()
            }
            HipRaiseExerciseName::MarchingHipRaiseWithFeetOnASwissBall => {
                "marching_hip_raise_with_feet_on_a_swiss_ball".to_string()
            }
            HipRaiseExerciseName::WeightedMarchingHipRaiseWithFeetOnASwissBall => {
                "weighted_marching_hip_raise_with_feet_on_a_swiss_ball".to_string()
            }
            HipRaiseExerciseName::ReverseHipRaise => "reverse_hip_raise".to_string(),
            HipRaiseExerciseName::WeightedReverseHipRaise => {
                "weighted_reverse_hip_raise".to_string()
            }
            HipRaiseExerciseName::SingleLegHipRaise => "single_leg_hip_raise".to_string(),
            HipRaiseExerciseName::WeightedSingleLegHipRaise => {
                "weighted_single_leg_hip_raise".to_string()
            }
            HipRaiseExerciseName::SingleLegHipRaiseWithFootOnBench => {
                "single_leg_hip_raise_with_foot_on_bench".to_string()
            }
            HipRaiseExerciseName::WeightedSingleLegHipRaiseWithFootOnBench => {
                "weighted_single_leg_hip_raise_with_foot_on_bench".to_string()
            }
            HipRaiseExerciseName::SingleLegHipRaiseWithFootOnBosuBall => {
                "single_leg_hip_raise_with_foot_on_bosu_ball".to_string()
            }
            HipRaiseExerciseName::WeightedSingleLegHipRaiseWithFootOnBosuBall => {
                "weighted_single_leg_hip_raise_with_foot_on_bosu_ball".to_string()
            }
            HipRaiseExerciseName::SingleLegHipRaiseWithFootOnFoamRoller => {
                "single_leg_hip_raise_with_foot_on_foam_roller".to_string()
            }
            HipRaiseExerciseName::WeightedSingleLegHipRaiseWithFootOnFoamRoller => {
                "weighted_single_leg_hip_raise_with_foot_on_foam_roller".to_string()
            }
            HipRaiseExerciseName::SingleLegHipRaiseWithFootOnMedicineBall => {
                "single_leg_hip_raise_with_foot_on_medicine_ball".to_string()
            }
            HipRaiseExerciseName::WeightedSingleLegHipRaiseWithFootOnMedicineBall => {
                "weighted_single_leg_hip_raise_with_foot_on_medicine_ball".to_string()
            }
            HipRaiseExerciseName::SingleLegHipRaiseWithHeadOnBosuBall => {
                "single_leg_hip_raise_with_head_on_bosu_ball".to_string()
            }
            HipRaiseExerciseName::WeightedSingleLegHipRaiseWithHeadOnBosuBall => {
                "weighted_single_leg_hip_raise_with_head_on_bosu_ball".to_string()
            }
            HipRaiseExerciseName::WeightedClamBridge => "weighted_clam_bridge".to_string(),
            HipRaiseExerciseName::SingleLegSwissBallHipRaiseAndLegCurl => {
                "single_leg_swiss_ball_hip_raise_and_leg_curl".to_string()
            }
            HipRaiseExerciseName::Clams => "clams".to_string(),
            HipRaiseExerciseName::InnerThighCircles => "inner_thigh_circles".to_string(),
            HipRaiseExerciseName::InnerThighSideLift => "inner_thigh_side_lift".to_string(),
            HipRaiseExerciseName::LegCircles => "leg_circles".to_string(),
            HipRaiseExerciseName::LegLift => "leg_lift".to_string(),
            HipRaiseExerciseName::LegLiftInExternalRotation => {
                "leg_lift_in_external_rotation".to_string()
            }
            HipRaiseExerciseName::UnknownVariant(value) => format!("unknown_variant_{}", *value),
        }
    }
}

#[derive(Clone, Copy, Debug)]
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
    pub fn from_u16(value: u16) -> HipStabilityExerciseName {
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
    pub fn from_i64(value: i64) -> HipStabilityExerciseName {
        HipStabilityExerciseName::from_u16(value as u16)
    }
    pub fn as_u16(&self) -> u16 {
        match &self {
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
            HipStabilityExerciseName::UnknownVariant(value) => *value,
        }
    }
    pub fn to_string(&self) -> String {
        match &self {
            HipStabilityExerciseName::BandSideLyingLegRaise => {
                "band_side_lying_leg_raise".to_string()
            }
            HipStabilityExerciseName::DeadBug => "dead_bug".to_string(),
            HipStabilityExerciseName::WeightedDeadBug => "weighted_dead_bug".to_string(),
            HipStabilityExerciseName::ExternalHipRaise => "external_hip_raise".to_string(),
            HipStabilityExerciseName::WeightedExternalHipRaise => {
                "weighted_external_hip_raise".to_string()
            }
            HipStabilityExerciseName::FireHydrantKicks => "fire_hydrant_kicks".to_string(),
            HipStabilityExerciseName::WeightedFireHydrantKicks => {
                "weighted_fire_hydrant_kicks".to_string()
            }
            HipStabilityExerciseName::HipCircles => "hip_circles".to_string(),
            HipStabilityExerciseName::WeightedHipCircles => "weighted_hip_circles".to_string(),
            HipStabilityExerciseName::InnerThighLift => "inner_thigh_lift".to_string(),
            HipStabilityExerciseName::WeightedInnerThighLift => {
                "weighted_inner_thigh_lift".to_string()
            }
            HipStabilityExerciseName::LateralWalksWithBandAtAnkles => {
                "lateral_walks_with_band_at_ankles".to_string()
            }
            HipStabilityExerciseName::PretzelSideKick => "pretzel_side_kick".to_string(),
            HipStabilityExerciseName::WeightedPretzelSideKick => {
                "weighted_pretzel_side_kick".to_string()
            }
            HipStabilityExerciseName::ProneHipInternalRotation => {
                "prone_hip_internal_rotation".to_string()
            }
            HipStabilityExerciseName::WeightedProneHipInternalRotation => {
                "weighted_prone_hip_internal_rotation".to_string()
            }
            HipStabilityExerciseName::Quadruped => "quadruped".to_string(),
            HipStabilityExerciseName::QuadrupedHipExtension => {
                "quadruped_hip_extension".to_string()
            }
            HipStabilityExerciseName::WeightedQuadrupedHipExtension => {
                "weighted_quadruped_hip_extension".to_string()
            }
            HipStabilityExerciseName::QuadrupedWithLegLift => "quadruped_with_leg_lift".to_string(),
            HipStabilityExerciseName::WeightedQuadrupedWithLegLift => {
                "weighted_quadruped_with_leg_lift".to_string()
            }
            HipStabilityExerciseName::SideLyingLegRaise => "side_lying_leg_raise".to_string(),
            HipStabilityExerciseName::WeightedSideLyingLegRaise => {
                "weighted_side_lying_leg_raise".to_string()
            }
            HipStabilityExerciseName::SlidingHipAdduction => "sliding_hip_adduction".to_string(),
            HipStabilityExerciseName::WeightedSlidingHipAdduction => {
                "weighted_sliding_hip_adduction".to_string()
            }
            HipStabilityExerciseName::StandingAdduction => "standing_adduction".to_string(),
            HipStabilityExerciseName::WeightedStandingAdduction => {
                "weighted_standing_adduction".to_string()
            }
            HipStabilityExerciseName::StandingCableHipAbduction => {
                "standing_cable_hip_abduction".to_string()
            }
            HipStabilityExerciseName::StandingHipAbduction => "standing_hip_abduction".to_string(),
            HipStabilityExerciseName::WeightedStandingHipAbduction => {
                "weighted_standing_hip_abduction".to_string()
            }
            HipStabilityExerciseName::StandingRearLegRaise => "standing_rear_leg_raise".to_string(),
            HipStabilityExerciseName::WeightedStandingRearLegRaise => {
                "weighted_standing_rear_leg_raise".to_string()
            }
            HipStabilityExerciseName::SupineHipInternalRotation => {
                "supine_hip_internal_rotation".to_string()
            }
            HipStabilityExerciseName::WeightedSupineHipInternalRotation => {
                "weighted_supine_hip_internal_rotation".to_string()
            }
            HipStabilityExerciseName::UnknownVariant(value) => {
                format!("unknown_variant_{}", *value)
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum HipSwingExerciseName {
    SingleArmKettlebellSwing,
    SingleArmDumbbellSwing,
    StepOutSwing,
    UnknownVariant(u16),
}
impl HipSwingExerciseName {
    pub fn from_u16(value: u16) -> HipSwingExerciseName {
        match value {
            0 => HipSwingExerciseName::SingleArmKettlebellSwing,
            1 => HipSwingExerciseName::SingleArmDumbbellSwing,
            2 => HipSwingExerciseName::StepOutSwing,
            _ => HipSwingExerciseName::UnknownVariant(value),
        }
    }
    pub fn from_i64(value: i64) -> HipSwingExerciseName {
        HipSwingExerciseName::from_u16(value as u16)
    }
    pub fn as_u16(&self) -> u16 {
        match &self {
            HipSwingExerciseName::SingleArmKettlebellSwing => 0,
            HipSwingExerciseName::SingleArmDumbbellSwing => 1,
            HipSwingExerciseName::StepOutSwing => 2,
            HipSwingExerciseName::UnknownVariant(value) => *value,
        }
    }
    pub fn to_string(&self) -> String {
        match &self {
            HipSwingExerciseName::SingleArmKettlebellSwing => {
                "single_arm_kettlebell_swing".to_string()
            }
            HipSwingExerciseName::SingleArmDumbbellSwing => "single_arm_dumbbell_swing".to_string(),
            HipSwingExerciseName::StepOutSwing => "step_out_swing".to_string(),
            HipSwingExerciseName::UnknownVariant(value) => format!("unknown_variant_{}", *value),
        }
    }
}

#[derive(Clone, Copy, Debug)]
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
    pub fn from_u16(value: u16) -> HyperextensionExerciseName {
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
    pub fn from_i64(value: i64) -> HyperextensionExerciseName {
        HyperextensionExerciseName::from_u16(value as u16)
    }
    pub fn as_u16(&self) -> u16 {
        match &self {
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
            HyperextensionExerciseName::UnknownVariant(value) => *value,
        }
    }
    pub fn to_string(&self) -> String {
        match &self {
            HyperextensionExerciseName::BackExtensionWithOppositeArmAndLegReach => {
                "back_extension_with_opposite_arm_and_leg_reach".to_string()
            }
            HyperextensionExerciseName::WeightedBackExtensionWithOppositeArmAndLegReach => {
                "weighted_back_extension_with_opposite_arm_and_leg_reach".to_string()
            }
            HyperextensionExerciseName::BaseRotations => "base_rotations".to_string(),
            HyperextensionExerciseName::WeightedBaseRotations => {
                "weighted_base_rotations".to_string()
            }
            HyperextensionExerciseName::BentKneeReverseHyperextension => {
                "bent_knee_reverse_hyperextension".to_string()
            }
            HyperextensionExerciseName::WeightedBentKneeReverseHyperextension => {
                "weighted_bent_knee_reverse_hyperextension".to_string()
            }
            HyperextensionExerciseName::HollowHoldAndRoll => "hollow_hold_and_roll".to_string(),
            HyperextensionExerciseName::WeightedHollowHoldAndRoll => {
                "weighted_hollow_hold_and_roll".to_string()
            }
            HyperextensionExerciseName::Kicks => "kicks".to_string(),
            HyperextensionExerciseName::WeightedKicks => "weighted_kicks".to_string(),
            HyperextensionExerciseName::KneeRaises => "knee_raises".to_string(),
            HyperextensionExerciseName::WeightedKneeRaises => "weighted_knee_raises".to_string(),
            HyperextensionExerciseName::KneelingSuperman => "kneeling_superman".to_string(),
            HyperextensionExerciseName::WeightedKneelingSuperman => {
                "weighted_kneeling_superman".to_string()
            }
            HyperextensionExerciseName::LatPullDownWithRow => "lat_pull_down_with_row".to_string(),
            HyperextensionExerciseName::MedicineBallDeadliftToReach => {
                "medicine_ball_deadlift_to_reach".to_string()
            }
            HyperextensionExerciseName::OneArmOneLegRow => "one_arm_one_leg_row".to_string(),
            HyperextensionExerciseName::OneArmRowWithBand => "one_arm_row_with_band".to_string(),
            HyperextensionExerciseName::OverheadLungeWithMedicineBall => {
                "overhead_lunge_with_medicine_ball".to_string()
            }
            HyperextensionExerciseName::PlankKneeTucks => "plank_knee_tucks".to_string(),
            HyperextensionExerciseName::WeightedPlankKneeTucks => {
                "weighted_plank_knee_tucks".to_string()
            }
            HyperextensionExerciseName::SideStep => "side_step".to_string(),
            HyperextensionExerciseName::WeightedSideStep => "weighted_side_step".to_string(),
            HyperextensionExerciseName::SingleLegBackExtension => {
                "single_leg_back_extension".to_string()
            }
            HyperextensionExerciseName::WeightedSingleLegBackExtension => {
                "weighted_single_leg_back_extension".to_string()
            }
            HyperextensionExerciseName::SpineExtension => "spine_extension".to_string(),
            HyperextensionExerciseName::WeightedSpineExtension => {
                "weighted_spine_extension".to_string()
            }
            HyperextensionExerciseName::StaticBackExtension => "static_back_extension".to_string(),
            HyperextensionExerciseName::WeightedStaticBackExtension => {
                "weighted_static_back_extension".to_string()
            }
            HyperextensionExerciseName::SupermanFromFloor => "superman_from_floor".to_string(),
            HyperextensionExerciseName::WeightedSupermanFromFloor => {
                "weighted_superman_from_floor".to_string()
            }
            HyperextensionExerciseName::SwissBallBackExtension => {
                "swiss_ball_back_extension".to_string()
            }
            HyperextensionExerciseName::WeightedSwissBallBackExtension => {
                "weighted_swiss_ball_back_extension".to_string()
            }
            HyperextensionExerciseName::SwissBallHyperextension => {
                "swiss_ball_hyperextension".to_string()
            }
            HyperextensionExerciseName::WeightedSwissBallHyperextension => {
                "weighted_swiss_ball_hyperextension".to_string()
            }
            HyperextensionExerciseName::SwissBallOppositeArmAndLegLift => {
                "swiss_ball_opposite_arm_and_leg_lift".to_string()
            }
            HyperextensionExerciseName::WeightedSwissBallOppositeArmAndLegLift => {
                "weighted_swiss_ball_opposite_arm_and_leg_lift".to_string()
            }
            HyperextensionExerciseName::SupermanOnSwissBall => "superman_on_swiss_ball".to_string(),
            HyperextensionExerciseName::Cobra => "cobra".to_string(),
            HyperextensionExerciseName::SupineFloorBarre => "supine_floor_barre".to_string(),
            HyperextensionExerciseName::UnknownVariant(value) => {
                format!("unknown_variant_{}", *value)
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
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
    pub fn from_u16(value: u16) -> LateralRaiseExerciseName {
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
    pub fn from_i64(value: i64) -> LateralRaiseExerciseName {
        LateralRaiseExerciseName::from_u16(value as u16)
    }
    pub fn as_u16(&self) -> u16 {
        match &self {
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
            LateralRaiseExerciseName::UnknownVariant(value) => *value,
        }
    }
    pub fn to_string(&self) -> String {
        match &self {
            LateralRaiseExerciseName::Name45DegreeCableExternalRotation => {
                "45_degree_cable_external_rotation".to_string()
            }
            LateralRaiseExerciseName::AlternatingLateralRaiseWithStaticHold => {
                "alternating_lateral_raise_with_static_hold".to_string()
            }
            LateralRaiseExerciseName::BarMuscleUp => "bar_muscle_up".to_string(),
            LateralRaiseExerciseName::BentOverLateralRaise => "bent_over_lateral_raise".to_string(),
            LateralRaiseExerciseName::CableDiagonalRaise => "cable_diagonal_raise".to_string(),
            LateralRaiseExerciseName::CableFrontRaise => "cable_front_raise".to_string(),
            LateralRaiseExerciseName::CalorieRow => "calorie_row".to_string(),
            LateralRaiseExerciseName::ComboShoulderRaise => "combo_shoulder_raise".to_string(),
            LateralRaiseExerciseName::DumbbellDiagonalRaise => {
                "dumbbell_diagonal_raise".to_string()
            }
            LateralRaiseExerciseName::DumbbellVRaise => "dumbbell_v_raise".to_string(),
            LateralRaiseExerciseName::FrontRaise => "front_raise".to_string(),
            LateralRaiseExerciseName::LeaningDumbbellLateralRaise => {
                "leaning_dumbbell_lateral_raise".to_string()
            }
            LateralRaiseExerciseName::LyingDumbbellRaise => "lying_dumbbell_raise".to_string(),
            LateralRaiseExerciseName::MuscleUp => "muscle_up".to_string(),
            LateralRaiseExerciseName::OneArmCableLateralRaise => {
                "one_arm_cable_lateral_raise".to_string()
            }
            LateralRaiseExerciseName::OverhandGripRearLateralRaise => {
                "overhand_grip_rear_lateral_raise".to_string()
            }
            LateralRaiseExerciseName::PlateRaises => "plate_raises".to_string(),
            LateralRaiseExerciseName::RingDip => "ring_dip".to_string(),
            LateralRaiseExerciseName::WeightedRingDip => "weighted_ring_dip".to_string(),
            LateralRaiseExerciseName::RingMuscleUp => "ring_muscle_up".to_string(),
            LateralRaiseExerciseName::WeightedRingMuscleUp => "weighted_ring_muscle_up".to_string(),
            LateralRaiseExerciseName::RopeClimb => "rope_climb".to_string(),
            LateralRaiseExerciseName::WeightedRopeClimb => "weighted_rope_climb".to_string(),
            LateralRaiseExerciseName::Scaption => "scaption".to_string(),
            LateralRaiseExerciseName::SeatedLateralRaise => "seated_lateral_raise".to_string(),
            LateralRaiseExerciseName::SeatedRearLateralRaise => {
                "seated_rear_lateral_raise".to_string()
            }
            LateralRaiseExerciseName::SideLyingLateralRaise => {
                "side_lying_lateral_raise".to_string()
            }
            LateralRaiseExerciseName::StandingLift => "standing_lift".to_string(),
            LateralRaiseExerciseName::SuspendedRow => "suspended_row".to_string(),
            LateralRaiseExerciseName::UnderhandGripRearLateralRaise => {
                "underhand_grip_rear_lateral_raise".to_string()
            }
            LateralRaiseExerciseName::WallSlide => "wall_slide".to_string(),
            LateralRaiseExerciseName::WeightedWallSlide => "weighted_wall_slide".to_string(),
            LateralRaiseExerciseName::ArmCircles => "arm_circles".to_string(),
            LateralRaiseExerciseName::ShavingTheHead => "shaving_the_head".to_string(),
            LateralRaiseExerciseName::UnknownVariant(value) => {
                format!("unknown_variant_{}", *value)
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
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
    pub fn from_u16(value: u16) -> LegCurlExerciseName {
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
    pub fn from_i64(value: i64) -> LegCurlExerciseName {
        LegCurlExerciseName::from_u16(value as u16)
    }
    pub fn as_u16(&self) -> u16 {
        match &self {
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
            LegCurlExerciseName::UnknownVariant(value) => *value,
        }
    }
    pub fn to_string(&self) -> String {
        match &self {
            LegCurlExerciseName::LegCurl => "leg_curl".to_string(),
            LegCurlExerciseName::WeightedLegCurl => "weighted_leg_curl".to_string(),
            LegCurlExerciseName::GoodMorning => "good_morning".to_string(),
            LegCurlExerciseName::SeatedBarbellGoodMorning => {
                "seated_barbell_good_morning".to_string()
            }
            LegCurlExerciseName::SingleLegBarbellGoodMorning => {
                "single_leg_barbell_good_morning".to_string()
            }
            LegCurlExerciseName::SingleLegSlidingLegCurl => {
                "single_leg_sliding_leg_curl".to_string()
            }
            LegCurlExerciseName::SlidingLegCurl => "sliding_leg_curl".to_string(),
            LegCurlExerciseName::SplitBarbellGoodMorning => {
                "split_barbell_good_morning".to_string()
            }
            LegCurlExerciseName::SplitStanceExtension => "split_stance_extension".to_string(),
            LegCurlExerciseName::StaggeredStanceGoodMorning => {
                "staggered_stance_good_morning".to_string()
            }
            LegCurlExerciseName::SwissBallHipRaiseAndLegCurl => {
                "swiss_ball_hip_raise_and_leg_curl".to_string()
            }
            LegCurlExerciseName::ZercherGoodMorning => "zercher_good_morning".to_string(),
            LegCurlExerciseName::UnknownVariant(value) => format!("unknown_variant_{}", *value),
        }
    }
}

#[derive(Clone, Copy, Debug)]
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
    pub fn from_u16(value: u16) -> LegRaiseExerciseName {
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
    pub fn from_i64(value: i64) -> LegRaiseExerciseName {
        LegRaiseExerciseName::from_u16(value as u16)
    }
    pub fn as_u16(&self) -> u16 {
        match &self {
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
            LegRaiseExerciseName::UnknownVariant(value) => *value,
        }
    }
    pub fn to_string(&self) -> String {
        match &self {
            LegRaiseExerciseName::HangingKneeRaise => "hanging_knee_raise".to_string(),
            LegRaiseExerciseName::HangingLegRaise => "hanging_leg_raise".to_string(),
            LegRaiseExerciseName::WeightedHangingLegRaise => {
                "weighted_hanging_leg_raise".to_string()
            }
            LegRaiseExerciseName::HangingSingleLegRaise => "hanging_single_leg_raise".to_string(),
            LegRaiseExerciseName::WeightedHangingSingleLegRaise => {
                "weighted_hanging_single_leg_raise".to_string()
            }
            LegRaiseExerciseName::KettlebellLegRaises => "kettlebell_leg_raises".to_string(),
            LegRaiseExerciseName::LegLoweringDrill => "leg_lowering_drill".to_string(),
            LegRaiseExerciseName::WeightedLegLoweringDrill => {
                "weighted_leg_lowering_drill".to_string()
            }
            LegRaiseExerciseName::LyingStraightLegRaise => "lying_straight_leg_raise".to_string(),
            LegRaiseExerciseName::WeightedLyingStraightLegRaise => {
                "weighted_lying_straight_leg_raise".to_string()
            }
            LegRaiseExerciseName::MedicineBallLegDrops => "medicine_ball_leg_drops".to_string(),
            LegRaiseExerciseName::QuadrupedLegRaise => "quadruped_leg_raise".to_string(),
            LegRaiseExerciseName::WeightedQuadrupedLegRaise => {
                "weighted_quadruped_leg_raise".to_string()
            }
            LegRaiseExerciseName::ReverseLegRaise => "reverse_leg_raise".to_string(),
            LegRaiseExerciseName::WeightedReverseLegRaise => {
                "weighted_reverse_leg_raise".to_string()
            }
            LegRaiseExerciseName::ReverseLegRaiseOnSwissBall => {
                "reverse_leg_raise_on_swiss_ball".to_string()
            }
            LegRaiseExerciseName::WeightedReverseLegRaiseOnSwissBall => {
                "weighted_reverse_leg_raise_on_swiss_ball".to_string()
            }
            LegRaiseExerciseName::SingleLegLoweringDrill => "single_leg_lowering_drill".to_string(),
            LegRaiseExerciseName::WeightedSingleLegLoweringDrill => {
                "weighted_single_leg_lowering_drill".to_string()
            }
            LegRaiseExerciseName::WeightedHangingKneeRaise => {
                "weighted_hanging_knee_raise".to_string()
            }
            LegRaiseExerciseName::LateralStepover => "lateral_stepover".to_string(),
            LegRaiseExerciseName::WeightedLateralStepover => {
                "weighted_lateral_stepover".to_string()
            }
            LegRaiseExerciseName::UnknownVariant(value) => format!("unknown_variant_{}", *value),
        }
    }
}

#[derive(Clone, Copy, Debug)]
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
    pub fn from_u16(value: u16) -> LungeExerciseName {
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
    pub fn from_i64(value: i64) -> LungeExerciseName {
        LungeExerciseName::from_u16(value as u16)
    }
    pub fn as_u16(&self) -> u16 {
        match &self {
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
            LungeExerciseName::UnknownVariant(value) => *value,
        }
    }
    pub fn to_string(&self) -> String {
        match &self {
            LungeExerciseName::OverheadLunge => "overhead_lunge".to_string(),
            LungeExerciseName::LungeMatrix => "lunge_matrix".to_string(),
            LungeExerciseName::WeightedLungeMatrix => "weighted_lunge_matrix".to_string(),
            LungeExerciseName::AlternatingBarbellForwardLunge => {
                "alternating_barbell_forward_lunge".to_string()
            }
            LungeExerciseName::AlternatingDumbbellLungeWithReach => {
                "alternating_dumbbell_lunge_with_reach".to_string()
            }
            LungeExerciseName::BackFootElevatedDumbbellSplitSquat => {
                "back_foot_elevated_dumbbell_split_squat".to_string()
            }
            LungeExerciseName::BarbellBoxLunge => "barbell_box_lunge".to_string(),
            LungeExerciseName::BarbellBulgarianSplitSquat => {
                "barbell_bulgarian_split_squat".to_string()
            }
            LungeExerciseName::BarbellCrossoverLunge => "barbell_crossover_lunge".to_string(),
            LungeExerciseName::BarbellFrontSplitSquat => "barbell_front_split_squat".to_string(),
            LungeExerciseName::BarbellLunge => "barbell_lunge".to_string(),
            LungeExerciseName::BarbellReverseLunge => "barbell_reverse_lunge".to_string(),
            LungeExerciseName::BarbellSideLunge => "barbell_side_lunge".to_string(),
            LungeExerciseName::BarbellSplitSquat => "barbell_split_squat".to_string(),
            LungeExerciseName::CoreControlRearLunge => "core_control_rear_lunge".to_string(),
            LungeExerciseName::DiagonalLunge => "diagonal_lunge".to_string(),
            LungeExerciseName::DropLunge => "drop_lunge".to_string(),
            LungeExerciseName::DumbbellBoxLunge => "dumbbell_box_lunge".to_string(),
            LungeExerciseName::DumbbellBulgarianSplitSquat => {
                "dumbbell_bulgarian_split_squat".to_string()
            }
            LungeExerciseName::DumbbellCrossoverLunge => "dumbbell_crossover_lunge".to_string(),
            LungeExerciseName::DumbbellDiagonalLunge => "dumbbell_diagonal_lunge".to_string(),
            LungeExerciseName::DumbbellLunge => "dumbbell_lunge".to_string(),
            LungeExerciseName::DumbbellLungeAndRotation => {
                "dumbbell_lunge_and_rotation".to_string()
            }
            LungeExerciseName::DumbbellOverheadBulgarianSplitSquat => {
                "dumbbell_overhead_bulgarian_split_squat".to_string()
            }
            LungeExerciseName::DumbbellReverseLungeToHighKneeAndPress => {
                "dumbbell_reverse_lunge_to_high_knee_and_press".to_string()
            }
            LungeExerciseName::DumbbellSideLunge => "dumbbell_side_lunge".to_string(),
            LungeExerciseName::ElevatedFrontFootBarbellSplitSquat => {
                "elevated_front_foot_barbell_split_squat".to_string()
            }
            LungeExerciseName::FrontFootElevatedDumbbellSplitSquat => {
                "front_foot_elevated_dumbbell_split_squat".to_string()
            }
            LungeExerciseName::GunslingerLunge => "gunslinger_lunge".to_string(),
            LungeExerciseName::LawnmowerLunge => "lawnmower_lunge".to_string(),
            LungeExerciseName::LowLungeWithIsometricAdduction => {
                "low_lunge_with_isometric_adduction".to_string()
            }
            LungeExerciseName::LowSideToSideLunge => "low_side_to_side_lunge".to_string(),
            LungeExerciseName::Lunge => "lunge".to_string(),
            LungeExerciseName::WeightedLunge => "weighted_lunge".to_string(),
            LungeExerciseName::LungeWithArmReach => "lunge_with_arm_reach".to_string(),
            LungeExerciseName::LungeWithDiagonalReach => "lunge_with_diagonal_reach".to_string(),
            LungeExerciseName::LungeWithSideBend => "lunge_with_side_bend".to_string(),
            LungeExerciseName::OffsetDumbbellLunge => "offset_dumbbell_lunge".to_string(),
            LungeExerciseName::OffsetDumbbellReverseLunge => {
                "offset_dumbbell_reverse_lunge".to_string()
            }
            LungeExerciseName::OverheadBulgarianSplitSquat => {
                "overhead_bulgarian_split_squat".to_string()
            }
            LungeExerciseName::OverheadDumbbellReverseLunge => {
                "overhead_dumbbell_reverse_lunge".to_string()
            }
            LungeExerciseName::OverheadDumbbellSplitSquat => {
                "overhead_dumbbell_split_squat".to_string()
            }
            LungeExerciseName::OverheadLungeWithRotation => {
                "overhead_lunge_with_rotation".to_string()
            }
            LungeExerciseName::ReverseBarbellBoxLunge => "reverse_barbell_box_lunge".to_string(),
            LungeExerciseName::ReverseBoxLunge => "reverse_box_lunge".to_string(),
            LungeExerciseName::ReverseDumbbellBoxLunge => "reverse_dumbbell_box_lunge".to_string(),
            LungeExerciseName::ReverseDumbbellCrossoverLunge => {
                "reverse_dumbbell_crossover_lunge".to_string()
            }
            LungeExerciseName::ReverseDumbbellDiagonalLunge => {
                "reverse_dumbbell_diagonal_lunge".to_string()
            }
            LungeExerciseName::ReverseLungeWithReachBack => {
                "reverse_lunge_with_reach_back".to_string()
            }
            LungeExerciseName::WeightedReverseLungeWithReachBack => {
                "weighted_reverse_lunge_with_reach_back".to_string()
            }
            LungeExerciseName::ReverseLungeWithTwistAndOverheadReach => {
                "reverse_lunge_with_twist_and_overhead_reach".to_string()
            }
            LungeExerciseName::WeightedReverseLungeWithTwistAndOverheadReach => {
                "weighted_reverse_lunge_with_twist_and_overhead_reach".to_string()
            }
            LungeExerciseName::ReverseSlidingBoxLunge => "reverse_sliding_box_lunge".to_string(),
            LungeExerciseName::WeightedReverseSlidingBoxLunge => {
                "weighted_reverse_sliding_box_lunge".to_string()
            }
            LungeExerciseName::ReverseSlidingLunge => "reverse_sliding_lunge".to_string(),
            LungeExerciseName::WeightedReverseSlidingLunge => {
                "weighted_reverse_sliding_lunge".to_string()
            }
            LungeExerciseName::RunnersLungeToBalance => "runners_lunge_to_balance".to_string(),
            LungeExerciseName::WeightedRunnersLungeToBalance => {
                "weighted_runners_lunge_to_balance".to_string()
            }
            LungeExerciseName::ShiftingSideLunge => "shifting_side_lunge".to_string(),
            LungeExerciseName::SideAndCrossoverLunge => "side_and_crossover_lunge".to_string(),
            LungeExerciseName::WeightedSideAndCrossoverLunge => {
                "weighted_side_and_crossover_lunge".to_string()
            }
            LungeExerciseName::SideLunge => "side_lunge".to_string(),
            LungeExerciseName::WeightedSideLunge => "weighted_side_lunge".to_string(),
            LungeExerciseName::SideLungeAndPress => "side_lunge_and_press".to_string(),
            LungeExerciseName::SideLungeJumpOff => "side_lunge_jump_off".to_string(),
            LungeExerciseName::SideLungeSweep => "side_lunge_sweep".to_string(),
            LungeExerciseName::WeightedSideLungeSweep => "weighted_side_lunge_sweep".to_string(),
            LungeExerciseName::SideLungeToCrossoverTap => "side_lunge_to_crossover_tap".to_string(),
            LungeExerciseName::WeightedSideLungeToCrossoverTap => {
                "weighted_side_lunge_to_crossover_tap".to_string()
            }
            LungeExerciseName::SideToSideLungeChops => "side_to_side_lunge_chops".to_string(),
            LungeExerciseName::WeightedSideToSideLungeChops => {
                "weighted_side_to_side_lunge_chops".to_string()
            }
            LungeExerciseName::SiffJumpLunge => "siff_jump_lunge".to_string(),
            LungeExerciseName::WeightedSiffJumpLunge => "weighted_siff_jump_lunge".to_string(),
            LungeExerciseName::SingleArmReverseLungeAndPress => {
                "single_arm_reverse_lunge_and_press".to_string()
            }
            LungeExerciseName::SlidingLateralLunge => "sliding_lateral_lunge".to_string(),
            LungeExerciseName::WeightedSlidingLateralLunge => {
                "weighted_sliding_lateral_lunge".to_string()
            }
            LungeExerciseName::WalkingBarbellLunge => "walking_barbell_lunge".to_string(),
            LungeExerciseName::WalkingDumbbellLunge => "walking_dumbbell_lunge".to_string(),
            LungeExerciseName::WalkingLunge => "walking_lunge".to_string(),
            LungeExerciseName::WeightedWalkingLunge => "weighted_walking_lunge".to_string(),
            LungeExerciseName::WideGripOverheadBarbellSplitSquat => {
                "wide_grip_overhead_barbell_split_squat".to_string()
            }
            LungeExerciseName::UnknownVariant(value) => format!("unknown_variant_{}", *value),
        }
    }
}

#[derive(Clone, Copy, Debug)]
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
    pub fn from_u16(value: u16) -> OlympicLiftExerciseName {
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
    pub fn from_i64(value: i64) -> OlympicLiftExerciseName {
        OlympicLiftExerciseName::from_u16(value as u16)
    }
    pub fn as_u16(&self) -> u16 {
        match &self {
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
            OlympicLiftExerciseName::UnknownVariant(value) => *value,
        }
    }
    pub fn to_string(&self) -> String {
        match &self {
            OlympicLiftExerciseName::BarbellHangPowerClean => {
                "barbell_hang_power_clean".to_string()
            }
            OlympicLiftExerciseName::BarbellHangSquatClean => {
                "barbell_hang_squat_clean".to_string()
            }
            OlympicLiftExerciseName::BarbellPowerClean => "barbell_power_clean".to_string(),
            OlympicLiftExerciseName::BarbellPowerSnatch => "barbell_power_snatch".to_string(),
            OlympicLiftExerciseName::BarbellSquatClean => "barbell_squat_clean".to_string(),
            OlympicLiftExerciseName::CleanAndJerk => "clean_and_jerk".to_string(),
            OlympicLiftExerciseName::BarbellHangPowerSnatch => {
                "barbell_hang_power_snatch".to_string()
            }
            OlympicLiftExerciseName::BarbellHangPull => "barbell_hang_pull".to_string(),
            OlympicLiftExerciseName::BarbellHighPull => "barbell_high_pull".to_string(),
            OlympicLiftExerciseName::BarbellSnatch => "barbell_snatch".to_string(),
            OlympicLiftExerciseName::BarbellSplitJerk => "barbell_split_jerk".to_string(),
            OlympicLiftExerciseName::Clean => "clean".to_string(),
            OlympicLiftExerciseName::DumbbellClean => "dumbbell_clean".to_string(),
            OlympicLiftExerciseName::DumbbellHangPull => "dumbbell_hang_pull".to_string(),
            OlympicLiftExerciseName::OneHandDumbbellSplitSnatch => {
                "one_hand_dumbbell_split_snatch".to_string()
            }
            OlympicLiftExerciseName::PushJerk => "push_jerk".to_string(),
            OlympicLiftExerciseName::SingleArmDumbbellSnatch => {
                "single_arm_dumbbell_snatch".to_string()
            }
            OlympicLiftExerciseName::SingleArmHangSnatch => "single_arm_hang_snatch".to_string(),
            OlympicLiftExerciseName::SingleArmKettlebellSnatch => {
                "single_arm_kettlebell_snatch".to_string()
            }
            OlympicLiftExerciseName::SplitJerk => "split_jerk".to_string(),
            OlympicLiftExerciseName::SquatCleanAndJerk => "squat_clean_and_jerk".to_string(),
            OlympicLiftExerciseName::UnknownVariant(value) => format!("unknown_variant_{}", *value),
        }
    }
}

#[derive(Clone, Copy, Debug)]
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
    pub fn from_u16(value: u16) -> PlankExerciseName {
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
    pub fn from_i64(value: i64) -> PlankExerciseName {
        PlankExerciseName::from_u16(value as u16)
    }
    pub fn as_u16(&self) -> u16 {
        match &self {
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
            PlankExerciseName::UnknownVariant(value) => *value,
        }
    }
    pub fn to_string(&self) -> String {
        match &self {
            PlankExerciseName::Name45DegreePlank => "45_degree_plank".to_string(),
            PlankExerciseName::Weighted45DegreePlank => "weighted_45_degree_plank".to_string(),
            PlankExerciseName::Name90DegreeStaticHold => "90_degree_static_hold".to_string(),
            PlankExerciseName::Weighted90DegreeStaticHold => {
                "weighted_90_degree_static_hold".to_string()
            }
            PlankExerciseName::BearCrawl => "bear_crawl".to_string(),
            PlankExerciseName::WeightedBearCrawl => "weighted_bear_crawl".to_string(),
            PlankExerciseName::CrossBodyMountainClimber => {
                "cross_body_mountain_climber".to_string()
            }
            PlankExerciseName::WeightedCrossBodyMountainClimber => {
                "weighted_cross_body_mountain_climber".to_string()
            }
            PlankExerciseName::ElbowPlankPikeJacks => "elbow_plank_pike_jacks".to_string(),
            PlankExerciseName::WeightedElbowPlankPikeJacks => {
                "weighted_elbow_plank_pike_jacks".to_string()
            }
            PlankExerciseName::ElevatedFeetPlank => "elevated_feet_plank".to_string(),
            PlankExerciseName::WeightedElevatedFeetPlank => {
                "weighted_elevated_feet_plank".to_string()
            }
            PlankExerciseName::ElevatorAbs => "elevator_abs".to_string(),
            PlankExerciseName::WeightedElevatorAbs => "weighted_elevator_abs".to_string(),
            PlankExerciseName::ExtendedPlank => "extended_plank".to_string(),
            PlankExerciseName::WeightedExtendedPlank => "weighted_extended_plank".to_string(),
            PlankExerciseName::FullPlankPasseTwist => "full_plank_passe_twist".to_string(),
            PlankExerciseName::WeightedFullPlankPasseTwist => {
                "weighted_full_plank_passe_twist".to_string()
            }
            PlankExerciseName::InchingElbowPlank => "inching_elbow_plank".to_string(),
            PlankExerciseName::WeightedInchingElbowPlank => {
                "weighted_inching_elbow_plank".to_string()
            }
            PlankExerciseName::InchwormToSidePlank => "inchworm_to_side_plank".to_string(),
            PlankExerciseName::WeightedInchwormToSidePlank => {
                "weighted_inchworm_to_side_plank".to_string()
            }
            PlankExerciseName::KneelingPlank => "kneeling_plank".to_string(),
            PlankExerciseName::WeightedKneelingPlank => "weighted_kneeling_plank".to_string(),
            PlankExerciseName::KneelingSidePlankWithLegLift => {
                "kneeling_side_plank_with_leg_lift".to_string()
            }
            PlankExerciseName::WeightedKneelingSidePlankWithLegLift => {
                "weighted_kneeling_side_plank_with_leg_lift".to_string()
            }
            PlankExerciseName::LateralRoll => "lateral_roll".to_string(),
            PlankExerciseName::WeightedLateralRoll => "weighted_lateral_roll".to_string(),
            PlankExerciseName::LyingReversePlank => "lying_reverse_plank".to_string(),
            PlankExerciseName::WeightedLyingReversePlank => {
                "weighted_lying_reverse_plank".to_string()
            }
            PlankExerciseName::MedicineBallMountainClimber => {
                "medicine_ball_mountain_climber".to_string()
            }
            PlankExerciseName::WeightedMedicineBallMountainClimber => {
                "weighted_medicine_ball_mountain_climber".to_string()
            }
            PlankExerciseName::ModifiedMountainClimberAndExtension => {
                "modified_mountain_climber_and_extension".to_string()
            }
            PlankExerciseName::WeightedModifiedMountainClimberAndExtension => {
                "weighted_modified_mountain_climber_and_extension".to_string()
            }
            PlankExerciseName::MountainClimber => "mountain_climber".to_string(),
            PlankExerciseName::WeightedMountainClimber => "weighted_mountain_climber".to_string(),
            PlankExerciseName::MountainClimberOnSlidingDiscs => {
                "mountain_climber_on_sliding_discs".to_string()
            }
            PlankExerciseName::WeightedMountainClimberOnSlidingDiscs => {
                "weighted_mountain_climber_on_sliding_discs".to_string()
            }
            PlankExerciseName::MountainClimberWithFeetOnBosuBall => {
                "mountain_climber_with_feet_on_bosu_ball".to_string()
            }
            PlankExerciseName::WeightedMountainClimberWithFeetOnBosuBall => {
                "weighted_mountain_climber_with_feet_on_bosu_ball".to_string()
            }
            PlankExerciseName::MountainClimberWithHandsOnBench => {
                "mountain_climber_with_hands_on_bench".to_string()
            }
            PlankExerciseName::MountainClimberWithHandsOnSwissBall => {
                "mountain_climber_with_hands_on_swiss_ball".to_string()
            }
            PlankExerciseName::WeightedMountainClimberWithHandsOnSwissBall => {
                "weighted_mountain_climber_with_hands_on_swiss_ball".to_string()
            }
            PlankExerciseName::Plank => "plank".to_string(),
            PlankExerciseName::PlankJacksWithFeetOnSlidingDiscs => {
                "plank_jacks_with_feet_on_sliding_discs".to_string()
            }
            PlankExerciseName::WeightedPlankJacksWithFeetOnSlidingDiscs => {
                "weighted_plank_jacks_with_feet_on_sliding_discs".to_string()
            }
            PlankExerciseName::PlankKneeTwist => "plank_knee_twist".to_string(),
            PlankExerciseName::WeightedPlankKneeTwist => "weighted_plank_knee_twist".to_string(),
            PlankExerciseName::PlankPikeJumps => "plank_pike_jumps".to_string(),
            PlankExerciseName::WeightedPlankPikeJumps => "weighted_plank_pike_jumps".to_string(),
            PlankExerciseName::PlankPikes => "plank_pikes".to_string(),
            PlankExerciseName::WeightedPlankPikes => "weighted_plank_pikes".to_string(),
            PlankExerciseName::PlankToStandUp => "plank_to_stand_up".to_string(),
            PlankExerciseName::WeightedPlankToStandUp => "weighted_plank_to_stand_up".to_string(),
            PlankExerciseName::PlankWithArmRaise => "plank_with_arm_raise".to_string(),
            PlankExerciseName::WeightedPlankWithArmRaise => {
                "weighted_plank_with_arm_raise".to_string()
            }
            PlankExerciseName::PlankWithKneeToElbow => "plank_with_knee_to_elbow".to_string(),
            PlankExerciseName::WeightedPlankWithKneeToElbow => {
                "weighted_plank_with_knee_to_elbow".to_string()
            }
            PlankExerciseName::PlankWithObliqueCrunch => "plank_with_oblique_crunch".to_string(),
            PlankExerciseName::WeightedPlankWithObliqueCrunch => {
                "weighted_plank_with_oblique_crunch".to_string()
            }
            PlankExerciseName::PlyometricSidePlank => "plyometric_side_plank".to_string(),
            PlankExerciseName::WeightedPlyometricSidePlank => {
                "weighted_plyometric_side_plank".to_string()
            }
            PlankExerciseName::RollingSidePlank => "rolling_side_plank".to_string(),
            PlankExerciseName::WeightedRollingSidePlank => {
                "weighted_rolling_side_plank".to_string()
            }
            PlankExerciseName::SideKickPlank => "side_kick_plank".to_string(),
            PlankExerciseName::WeightedSideKickPlank => "weighted_side_kick_plank".to_string(),
            PlankExerciseName::SidePlank => "side_plank".to_string(),
            PlankExerciseName::WeightedSidePlank => "weighted_side_plank".to_string(),
            PlankExerciseName::SidePlankAndRow => "side_plank_and_row".to_string(),
            PlankExerciseName::WeightedSidePlankAndRow => "weighted_side_plank_and_row".to_string(),
            PlankExerciseName::SidePlankLift => "side_plank_lift".to_string(),
            PlankExerciseName::WeightedSidePlankLift => "weighted_side_plank_lift".to_string(),
            PlankExerciseName::SidePlankWithElbowOnBosuBall => {
                "side_plank_with_elbow_on_bosu_ball".to_string()
            }
            PlankExerciseName::WeightedSidePlankWithElbowOnBosuBall => {
                "weighted_side_plank_with_elbow_on_bosu_ball".to_string()
            }
            PlankExerciseName::SidePlankWithFeetOnBench => {
                "side_plank_with_feet_on_bench".to_string()
            }
            PlankExerciseName::WeightedSidePlankWithFeetOnBench => {
                "weighted_side_plank_with_feet_on_bench".to_string()
            }
            PlankExerciseName::SidePlankWithKneeCircle => "side_plank_with_knee_circle".to_string(),
            PlankExerciseName::WeightedSidePlankWithKneeCircle => {
                "weighted_side_plank_with_knee_circle".to_string()
            }
            PlankExerciseName::SidePlankWithKneeTuck => "side_plank_with_knee_tuck".to_string(),
            PlankExerciseName::WeightedSidePlankWithKneeTuck => {
                "weighted_side_plank_with_knee_tuck".to_string()
            }
            PlankExerciseName::SidePlankWithLegLift => "side_plank_with_leg_lift".to_string(),
            PlankExerciseName::WeightedSidePlankWithLegLift => {
                "weighted_side_plank_with_leg_lift".to_string()
            }
            PlankExerciseName::SidePlankWithReachUnder => "side_plank_with_reach_under".to_string(),
            PlankExerciseName::WeightedSidePlankWithReachUnder => {
                "weighted_side_plank_with_reach_under".to_string()
            }
            PlankExerciseName::SingleLegElevatedFeetPlank => {
                "single_leg_elevated_feet_plank".to_string()
            }
            PlankExerciseName::WeightedSingleLegElevatedFeetPlank => {
                "weighted_single_leg_elevated_feet_plank".to_string()
            }
            PlankExerciseName::SingleLegFlexAndExtend => "single_leg_flex_and_extend".to_string(),
            PlankExerciseName::WeightedSingleLegFlexAndExtend => {
                "weighted_single_leg_flex_and_extend".to_string()
            }
            PlankExerciseName::SingleLegSidePlank => "single_leg_side_plank".to_string(),
            PlankExerciseName::WeightedSingleLegSidePlank => {
                "weighted_single_leg_side_plank".to_string()
            }
            PlankExerciseName::SpidermanPlank => "spiderman_plank".to_string(),
            PlankExerciseName::WeightedSpidermanPlank => "weighted_spiderman_plank".to_string(),
            PlankExerciseName::StraightArmPlank => "straight_arm_plank".to_string(),
            PlankExerciseName::WeightedStraightArmPlank => {
                "weighted_straight_arm_plank".to_string()
            }
            PlankExerciseName::StraightArmPlankWithShoulderTouch => {
                "straight_arm_plank_with_shoulder_touch".to_string()
            }
            PlankExerciseName::WeightedStraightArmPlankWithShoulderTouch => {
                "weighted_straight_arm_plank_with_shoulder_touch".to_string()
            }
            PlankExerciseName::SwissBallPlank => "swiss_ball_plank".to_string(),
            PlankExerciseName::WeightedSwissBallPlank => "weighted_swiss_ball_plank".to_string(),
            PlankExerciseName::SwissBallPlankLegLift => "swiss_ball_plank_leg_lift".to_string(),
            PlankExerciseName::WeightedSwissBallPlankLegLift => {
                "weighted_swiss_ball_plank_leg_lift".to_string()
            }
            PlankExerciseName::SwissBallPlankLegLiftAndHold => {
                "swiss_ball_plank_leg_lift_and_hold".to_string()
            }
            PlankExerciseName::SwissBallPlankWithFeetOnBench => {
                "swiss_ball_plank_with_feet_on_bench".to_string()
            }
            PlankExerciseName::WeightedSwissBallPlankWithFeetOnBench => {
                "weighted_swiss_ball_plank_with_feet_on_bench".to_string()
            }
            PlankExerciseName::SwissBallProneJackknife => "swiss_ball_prone_jackknife".to_string(),
            PlankExerciseName::WeightedSwissBallProneJackknife => {
                "weighted_swiss_ball_prone_jackknife".to_string()
            }
            PlankExerciseName::SwissBallSidePlank => "swiss_ball_side_plank".to_string(),
            PlankExerciseName::WeightedSwissBallSidePlank => {
                "weighted_swiss_ball_side_plank".to_string()
            }
            PlankExerciseName::ThreeWayPlank => "three_way_plank".to_string(),
            PlankExerciseName::WeightedThreeWayPlank => "weighted_three_way_plank".to_string(),
            PlankExerciseName::TowelPlankAndKneeIn => "towel_plank_and_knee_in".to_string(),
            PlankExerciseName::WeightedTowelPlankAndKneeIn => {
                "weighted_towel_plank_and_knee_in".to_string()
            }
            PlankExerciseName::TStabilization => "t_stabilization".to_string(),
            PlankExerciseName::WeightedTStabilization => "weighted_t_stabilization".to_string(),
            PlankExerciseName::TurkishGetUpToSidePlank => {
                "turkish_get_up_to_side_plank".to_string()
            }
            PlankExerciseName::WeightedTurkishGetUpToSidePlank => {
                "weighted_turkish_get_up_to_side_plank".to_string()
            }
            PlankExerciseName::TwoPointPlank => "two_point_plank".to_string(),
            PlankExerciseName::WeightedTwoPointPlank => "weighted_two_point_plank".to_string(),
            PlankExerciseName::WeightedPlank => "weighted_plank".to_string(),
            PlankExerciseName::WideStancePlankWithDiagonalArmLift => {
                "wide_stance_plank_with_diagonal_arm_lift".to_string()
            }
            PlankExerciseName::WeightedWideStancePlankWithDiagonalArmLift => {
                "weighted_wide_stance_plank_with_diagonal_arm_lift".to_string()
            }
            PlankExerciseName::WideStancePlankWithDiagonalLegLift => {
                "wide_stance_plank_with_diagonal_leg_lift".to_string()
            }
            PlankExerciseName::WeightedWideStancePlankWithDiagonalLegLift => {
                "weighted_wide_stance_plank_with_diagonal_leg_lift".to_string()
            }
            PlankExerciseName::WideStancePlankWithLegLift => {
                "wide_stance_plank_with_leg_lift".to_string()
            }
            PlankExerciseName::WeightedWideStancePlankWithLegLift => {
                "weighted_wide_stance_plank_with_leg_lift".to_string()
            }
            PlankExerciseName::WideStancePlankWithOppositeArmAndLegLift => {
                "wide_stance_plank_with_opposite_arm_and_leg_lift".to_string()
            }
            PlankExerciseName::WeightedMountainClimberWithHandsOnBench => {
                "weighted_mountain_climber_with_hands_on_bench".to_string()
            }
            PlankExerciseName::WeightedSwissBallPlankLegLiftAndHold => {
                "weighted_swiss_ball_plank_leg_lift_and_hold".to_string()
            }
            PlankExerciseName::WeightedWideStancePlankWithOppositeArmAndLegLift => {
                "weighted_wide_stance_plank_with_opposite_arm_and_leg_lift".to_string()
            }
            PlankExerciseName::PlankWithFeetOnSwissBall => {
                "plank_with_feet_on_swiss_ball".to_string()
            }
            PlankExerciseName::SidePlankToPlankWithReachUnder => {
                "side_plank_to_plank_with_reach_under".to_string()
            }
            PlankExerciseName::BridgeWithGluteLowerLift => {
                "bridge_with_glute_lower_lift".to_string()
            }
            PlankExerciseName::BridgeOneLegBridge => "bridge_one_leg_bridge".to_string(),
            PlankExerciseName::PlankWithArmVariations => "plank_with_arm_variations".to_string(),
            PlankExerciseName::PlankWithLegLift => "plank_with_leg_lift".to_string(),
            PlankExerciseName::ReversePlankWithLegPull => "reverse_plank_with_leg_pull".to_string(),
            PlankExerciseName::UnknownVariant(value) => format!("unknown_variant_{}", *value),
        }
    }
}

#[derive(Clone, Copy, Debug)]
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
    pub fn from_u16(value: u16) -> PlyoExerciseName {
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
    pub fn from_i64(value: i64) -> PlyoExerciseName {
        PlyoExerciseName::from_u16(value as u16)
    }
    pub fn as_u16(&self) -> u16 {
        match &self {
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
            PlyoExerciseName::UnknownVariant(value) => *value,
        }
    }
    pub fn to_string(&self) -> String {
        match &self {
            PlyoExerciseName::AlternatingJumpLunge => "alternating_jump_lunge".to_string(),
            PlyoExerciseName::WeightedAlternatingJumpLunge => {
                "weighted_alternating_jump_lunge".to_string()
            }
            PlyoExerciseName::BarbellJumpSquat => "barbell_jump_squat".to_string(),
            PlyoExerciseName::BodyWeightJumpSquat => "body_weight_jump_squat".to_string(),
            PlyoExerciseName::WeightedJumpSquat => "weighted_jump_squat".to_string(),
            PlyoExerciseName::CrossKneeStrike => "cross_knee_strike".to_string(),
            PlyoExerciseName::WeightedCrossKneeStrike => "weighted_cross_knee_strike".to_string(),
            PlyoExerciseName::DepthJump => "depth_jump".to_string(),
            PlyoExerciseName::WeightedDepthJump => "weighted_depth_jump".to_string(),
            PlyoExerciseName::DumbbellJumpSquat => "dumbbell_jump_squat".to_string(),
            PlyoExerciseName::DumbbellSplitJump => "dumbbell_split_jump".to_string(),
            PlyoExerciseName::FrontKneeStrike => "front_knee_strike".to_string(),
            PlyoExerciseName::WeightedFrontKneeStrike => "weighted_front_knee_strike".to_string(),
            PlyoExerciseName::HighBoxJump => "high_box_jump".to_string(),
            PlyoExerciseName::WeightedHighBoxJump => "weighted_high_box_jump".to_string(),
            PlyoExerciseName::IsometricExplosiveBodyWeightJumpSquat => {
                "isometric_explosive_body_weight_jump_squat".to_string()
            }
            PlyoExerciseName::WeightedIsometricExplosiveJumpSquat => {
                "weighted_isometric_explosive_jump_squat".to_string()
            }
            PlyoExerciseName::LateralLeapAndHop => "lateral_leap_and_hop".to_string(),
            PlyoExerciseName::WeightedLateralLeapAndHop => {
                "weighted_lateral_leap_and_hop".to_string()
            }
            PlyoExerciseName::LateralPlyoSquats => "lateral_plyo_squats".to_string(),
            PlyoExerciseName::WeightedLateralPlyoSquats => {
                "weighted_lateral_plyo_squats".to_string()
            }
            PlyoExerciseName::LateralSlide => "lateral_slide".to_string(),
            PlyoExerciseName::WeightedLateralSlide => "weighted_lateral_slide".to_string(),
            PlyoExerciseName::MedicineBallOverheadThrows => {
                "medicine_ball_overhead_throws".to_string()
            }
            PlyoExerciseName::MedicineBallSideThrow => "medicine_ball_side_throw".to_string(),
            PlyoExerciseName::MedicineBallSlam => "medicine_ball_slam".to_string(),
            PlyoExerciseName::SideToSideMedicineBallThrows => {
                "side_to_side_medicine_ball_throws".to_string()
            }
            PlyoExerciseName::SideToSideShuffleJump => "side_to_side_shuffle_jump".to_string(),
            PlyoExerciseName::WeightedSideToSideShuffleJump => {
                "weighted_side_to_side_shuffle_jump".to_string()
            }
            PlyoExerciseName::SquatJumpOntoBox => "squat_jump_onto_box".to_string(),
            PlyoExerciseName::WeightedSquatJumpOntoBox => {
                "weighted_squat_jump_onto_box".to_string()
            }
            PlyoExerciseName::SquatJumpsInAndOut => "squat_jumps_in_and_out".to_string(),
            PlyoExerciseName::WeightedSquatJumpsInAndOut => {
                "weighted_squat_jumps_in_and_out".to_string()
            }
            PlyoExerciseName::UnknownVariant(value) => format!("unknown_variant_{}", *value),
        }
    }
}

#[derive(Clone, Copy, Debug)]
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
    pub fn from_u16(value: u16) -> PullUpExerciseName {
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
    pub fn from_i64(value: i64) -> PullUpExerciseName {
        PullUpExerciseName::from_u16(value as u16)
    }
    pub fn as_u16(&self) -> u16 {
        match &self {
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
            PullUpExerciseName::UnknownVariant(value) => *value,
        }
    }
    pub fn to_string(&self) -> String {
        match &self {
            PullUpExerciseName::BandedPullUps => "banded_pull_ups".to_string(),
            PullUpExerciseName::Name30DegreeLatPulldown => "30_degree_lat_pulldown".to_string(),
            PullUpExerciseName::BandAssistedChinUp => "band_assisted_chin_up".to_string(),
            PullUpExerciseName::CloseGripChinUp => "close_grip_chin_up".to_string(),
            PullUpExerciseName::WeightedCloseGripChinUp => {
                "weighted_close_grip_chin_up".to_string()
            }
            PullUpExerciseName::CloseGripLatPulldown => "close_grip_lat_pulldown".to_string(),
            PullUpExerciseName::CrossoverChinUp => "crossover_chin_up".to_string(),
            PullUpExerciseName::WeightedCrossoverChinUp => "weighted_crossover_chin_up".to_string(),
            PullUpExerciseName::EzBarPullover => "ez_bar_pullover".to_string(),
            PullUpExerciseName::HangingHurdle => "hanging_hurdle".to_string(),
            PullUpExerciseName::WeightedHangingHurdle => "weighted_hanging_hurdle".to_string(),
            PullUpExerciseName::KneelingLatPulldown => "kneeling_lat_pulldown".to_string(),
            PullUpExerciseName::KneelingUnderhandGripLatPulldown => {
                "kneeling_underhand_grip_lat_pulldown".to_string()
            }
            PullUpExerciseName::LatPulldown => "lat_pulldown".to_string(),
            PullUpExerciseName::MixedGripChinUp => "mixed_grip_chin_up".to_string(),
            PullUpExerciseName::WeightedMixedGripChinUp => {
                "weighted_mixed_grip_chin_up".to_string()
            }
            PullUpExerciseName::MixedGripPullUp => "mixed_grip_pull_up".to_string(),
            PullUpExerciseName::WeightedMixedGripPullUp => {
                "weighted_mixed_grip_pull_up".to_string()
            }
            PullUpExerciseName::ReverseGripPulldown => "reverse_grip_pulldown".to_string(),
            PullUpExerciseName::StandingCablePullover => "standing_cable_pullover".to_string(),
            PullUpExerciseName::StraightArmPulldown => "straight_arm_pulldown".to_string(),
            PullUpExerciseName::SwissBallEzBarPullover => "swiss_ball_ez_bar_pullover".to_string(),
            PullUpExerciseName::TowelPullUp => "towel_pull_up".to_string(),
            PullUpExerciseName::WeightedTowelPullUp => "weighted_towel_pull_up".to_string(),
            PullUpExerciseName::WeightedPullUp => "weighted_pull_up".to_string(),
            PullUpExerciseName::WideGripLatPulldown => "wide_grip_lat_pulldown".to_string(),
            PullUpExerciseName::WideGripPullUp => "wide_grip_pull_up".to_string(),
            PullUpExerciseName::WeightedWideGripPullUp => "weighted_wide_grip_pull_up".to_string(),
            PullUpExerciseName::BurpeePullUp => "burpee_pull_up".to_string(),
            PullUpExerciseName::WeightedBurpeePullUp => "weighted_burpee_pull_up".to_string(),
            PullUpExerciseName::JumpingPullUps => "jumping_pull_ups".to_string(),
            PullUpExerciseName::WeightedJumpingPullUps => "weighted_jumping_pull_ups".to_string(),
            PullUpExerciseName::KippingPullUp => "kipping_pull_up".to_string(),
            PullUpExerciseName::WeightedKippingPullUp => "weighted_kipping_pull_up".to_string(),
            PullUpExerciseName::LPullUp => "l_pull_up".to_string(),
            PullUpExerciseName::WeightedLPullUp => "weighted_l_pull_up".to_string(),
            PullUpExerciseName::SuspendedChinUp => "suspended_chin_up".to_string(),
            PullUpExerciseName::WeightedSuspendedChinUp => "weighted_suspended_chin_up".to_string(),
            PullUpExerciseName::PullUp => "pull_up".to_string(),
            PullUpExerciseName::UnknownVariant(value) => format!("unknown_variant_{}", *value),
        }
    }
}

#[derive(Clone, Copy, Debug)]
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
    pub fn from_u16(value: u16) -> PushUpExerciseName {
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
    pub fn from_i64(value: i64) -> PushUpExerciseName {
        PushUpExerciseName::from_u16(value as u16)
    }
    pub fn as_u16(&self) -> u16 {
        match &self {
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
            PushUpExerciseName::UnknownVariant(value) => *value,
        }
    }
    pub fn to_string(&self) -> String {
        match &self {
            PushUpExerciseName::ChestPressWithBand => "chest_press_with_band".to_string(),
            PushUpExerciseName::AlternatingStaggeredPushUp => {
                "alternating_staggered_push_up".to_string()
            }
            PushUpExerciseName::WeightedAlternatingStaggeredPushUp => {
                "weighted_alternating_staggered_push_up".to_string()
            }
            PushUpExerciseName::AlternatingHandsMedicineBallPushUp => {
                "alternating_hands_medicine_ball_push_up".to_string()
            }
            PushUpExerciseName::WeightedAlternatingHandsMedicineBallPushUp => {
                "weighted_alternating_hands_medicine_ball_push_up".to_string()
            }
            PushUpExerciseName::BosuBallPushUp => "bosu_ball_push_up".to_string(),
            PushUpExerciseName::WeightedBosuBallPushUp => "weighted_bosu_ball_push_up".to_string(),
            PushUpExerciseName::ClappingPushUp => "clapping_push_up".to_string(),
            PushUpExerciseName::WeightedClappingPushUp => "weighted_clapping_push_up".to_string(),
            PushUpExerciseName::CloseGripMedicineBallPushUp => {
                "close_grip_medicine_ball_push_up".to_string()
            }
            PushUpExerciseName::WeightedCloseGripMedicineBallPushUp => {
                "weighted_close_grip_medicine_ball_push_up".to_string()
            }
            PushUpExerciseName::CloseHandsPushUp => "close_hands_push_up".to_string(),
            PushUpExerciseName::WeightedCloseHandsPushUp => {
                "weighted_close_hands_push_up".to_string()
            }
            PushUpExerciseName::DeclinePushUp => "decline_push_up".to_string(),
            PushUpExerciseName::WeightedDeclinePushUp => "weighted_decline_push_up".to_string(),
            PushUpExerciseName::DiamondPushUp => "diamond_push_up".to_string(),
            PushUpExerciseName::WeightedDiamondPushUp => "weighted_diamond_push_up".to_string(),
            PushUpExerciseName::ExplosiveCrossoverPushUp => {
                "explosive_crossover_push_up".to_string()
            }
            PushUpExerciseName::WeightedExplosiveCrossoverPushUp => {
                "weighted_explosive_crossover_push_up".to_string()
            }
            PushUpExerciseName::ExplosivePushUp => "explosive_push_up".to_string(),
            PushUpExerciseName::WeightedExplosivePushUp => "weighted_explosive_push_up".to_string(),
            PushUpExerciseName::FeetElevatedSideToSidePushUp => {
                "feet_elevated_side_to_side_push_up".to_string()
            }
            PushUpExerciseName::WeightedFeetElevatedSideToSidePushUp => {
                "weighted_feet_elevated_side_to_side_push_up".to_string()
            }
            PushUpExerciseName::HandReleasePushUp => "hand_release_push_up".to_string(),
            PushUpExerciseName::WeightedHandReleasePushUp => {
                "weighted_hand_release_push_up".to_string()
            }
            PushUpExerciseName::HandstandPushUp => "handstand_push_up".to_string(),
            PushUpExerciseName::WeightedHandstandPushUp => "weighted_handstand_push_up".to_string(),
            PushUpExerciseName::InclinePushUp => "incline_push_up".to_string(),
            PushUpExerciseName::WeightedInclinePushUp => "weighted_incline_push_up".to_string(),
            PushUpExerciseName::IsometricExplosivePushUp => {
                "isometric_explosive_push_up".to_string()
            }
            PushUpExerciseName::WeightedIsometricExplosivePushUp => {
                "weighted_isometric_explosive_push_up".to_string()
            }
            PushUpExerciseName::JudoPushUp => "judo_push_up".to_string(),
            PushUpExerciseName::WeightedJudoPushUp => "weighted_judo_push_up".to_string(),
            PushUpExerciseName::KneelingPushUp => "kneeling_push_up".to_string(),
            PushUpExerciseName::WeightedKneelingPushUp => "weighted_kneeling_push_up".to_string(),
            PushUpExerciseName::MedicineBallChestPass => "medicine_ball_chest_pass".to_string(),
            PushUpExerciseName::MedicineBallPushUp => "medicine_ball_push_up".to_string(),
            PushUpExerciseName::WeightedMedicineBallPushUp => {
                "weighted_medicine_ball_push_up".to_string()
            }
            PushUpExerciseName::OneArmPushUp => "one_arm_push_up".to_string(),
            PushUpExerciseName::WeightedOneArmPushUp => "weighted_one_arm_push_up".to_string(),
            PushUpExerciseName::WeightedPushUp => "weighted_push_up".to_string(),
            PushUpExerciseName::PushUpAndRow => "push_up_and_row".to_string(),
            PushUpExerciseName::WeightedPushUpAndRow => "weighted_push_up_and_row".to_string(),
            PushUpExerciseName::PushUpPlus => "push_up_plus".to_string(),
            PushUpExerciseName::WeightedPushUpPlus => "weighted_push_up_plus".to_string(),
            PushUpExerciseName::PushUpWithFeetOnSwissBall => {
                "push_up_with_feet_on_swiss_ball".to_string()
            }
            PushUpExerciseName::WeightedPushUpWithFeetOnSwissBall => {
                "weighted_push_up_with_feet_on_swiss_ball".to_string()
            }
            PushUpExerciseName::PushUpWithOneHandOnMedicineBall => {
                "push_up_with_one_hand_on_medicine_ball".to_string()
            }
            PushUpExerciseName::WeightedPushUpWithOneHandOnMedicineBall => {
                "weighted_push_up_with_one_hand_on_medicine_ball".to_string()
            }
            PushUpExerciseName::ShoulderPushUp => "shoulder_push_up".to_string(),
            PushUpExerciseName::WeightedShoulderPushUp => "weighted_shoulder_push_up".to_string(),
            PushUpExerciseName::SingleArmMedicineBallPushUp => {
                "single_arm_medicine_ball_push_up".to_string()
            }
            PushUpExerciseName::WeightedSingleArmMedicineBallPushUp => {
                "weighted_single_arm_medicine_ball_push_up".to_string()
            }
            PushUpExerciseName::SpidermanPushUp => "spiderman_push_up".to_string(),
            PushUpExerciseName::WeightedSpidermanPushUp => "weighted_spiderman_push_up".to_string(),
            PushUpExerciseName::StackedFeetPushUp => "stacked_feet_push_up".to_string(),
            PushUpExerciseName::WeightedStackedFeetPushUp => {
                "weighted_stacked_feet_push_up".to_string()
            }
            PushUpExerciseName::StaggeredHandsPushUp => "staggered_hands_push_up".to_string(),
            PushUpExerciseName::WeightedStaggeredHandsPushUp => {
                "weighted_staggered_hands_push_up".to_string()
            }
            PushUpExerciseName::SuspendedPushUp => "suspended_push_up".to_string(),
            PushUpExerciseName::WeightedSuspendedPushUp => "weighted_suspended_push_up".to_string(),
            PushUpExerciseName::SwissBallPushUp => "swiss_ball_push_up".to_string(),
            PushUpExerciseName::WeightedSwissBallPushUp => {
                "weighted_swiss_ball_push_up".to_string()
            }
            PushUpExerciseName::SwissBallPushUpPlus => "swiss_ball_push_up_plus".to_string(),
            PushUpExerciseName::WeightedSwissBallPushUpPlus => {
                "weighted_swiss_ball_push_up_plus".to_string()
            }
            PushUpExerciseName::TPushUp => "t_push_up".to_string(),
            PushUpExerciseName::WeightedTPushUp => "weighted_t_push_up".to_string(),
            PushUpExerciseName::TripleStopPushUp => "triple_stop_push_up".to_string(),
            PushUpExerciseName::WeightedTripleStopPushUp => {
                "weighted_triple_stop_push_up".to_string()
            }
            PushUpExerciseName::WideHandsPushUp => "wide_hands_push_up".to_string(),
            PushUpExerciseName::WeightedWideHandsPushUp => {
                "weighted_wide_hands_push_up".to_string()
            }
            PushUpExerciseName::ParalletteHandstandPushUp => {
                "parallette_handstand_push_up".to_string()
            }
            PushUpExerciseName::WeightedParalletteHandstandPushUp => {
                "weighted_parallette_handstand_push_up".to_string()
            }
            PushUpExerciseName::RingHandstandPushUp => "ring_handstand_push_up".to_string(),
            PushUpExerciseName::WeightedRingHandstandPushUp => {
                "weighted_ring_handstand_push_up".to_string()
            }
            PushUpExerciseName::RingPushUp => "ring_push_up".to_string(),
            PushUpExerciseName::WeightedRingPushUp => "weighted_ring_push_up".to_string(),
            PushUpExerciseName::PushUp => "push_up".to_string(),
            PushUpExerciseName::PilatesPushup => "pilates_pushup".to_string(),
            PushUpExerciseName::UnknownVariant(value) => format!("unknown_variant_{}", *value),
        }
    }
}

#[derive(Clone, Copy, Debug)]
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
    pub fn from_u16(value: u16) -> RowExerciseName {
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
    pub fn from_i64(value: i64) -> RowExerciseName {
        RowExerciseName::from_u16(value as u16)
    }
    pub fn as_u16(&self) -> u16 {
        match &self {
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
            RowExerciseName::UnknownVariant(value) => *value,
        }
    }
    pub fn to_string(&self) -> String {
        match &self {
            RowExerciseName::BarbellStraightLegDeadliftToRow => {
                "barbell_straight_leg_deadlift_to_row".to_string()
            }
            RowExerciseName::CableRowStanding => "cable_row_standing".to_string(),
            RowExerciseName::DumbbellRow => "dumbbell_row".to_string(),
            RowExerciseName::ElevatedFeetInvertedRow => "elevated_feet_inverted_row".to_string(),
            RowExerciseName::WeightedElevatedFeetInvertedRow => {
                "weighted_elevated_feet_inverted_row".to_string()
            }
            RowExerciseName::FacePull => "face_pull".to_string(),
            RowExerciseName::FacePullWithExternalRotation => {
                "face_pull_with_external_rotation".to_string()
            }
            RowExerciseName::InvertedRowWithFeetOnSwissBall => {
                "inverted_row_with_feet_on_swiss_ball".to_string()
            }
            RowExerciseName::WeightedInvertedRowWithFeetOnSwissBall => {
                "weighted_inverted_row_with_feet_on_swiss_ball".to_string()
            }
            RowExerciseName::KettlebellRow => "kettlebell_row".to_string(),
            RowExerciseName::ModifiedInvertedRow => "modified_inverted_row".to_string(),
            RowExerciseName::WeightedModifiedInvertedRow => {
                "weighted_modified_inverted_row".to_string()
            }
            RowExerciseName::NeutralGripAlternatingDumbbellRow => {
                "neutral_grip_alternating_dumbbell_row".to_string()
            }
            RowExerciseName::OneArmBentOverRow => "one_arm_bent_over_row".to_string(),
            RowExerciseName::OneLeggedDumbbellRow => "one_legged_dumbbell_row".to_string(),
            RowExerciseName::RenegadeRow => "renegade_row".to_string(),
            RowExerciseName::ReverseGripBarbellRow => "reverse_grip_barbell_row".to_string(),
            RowExerciseName::RopeHandleCableRow => "rope_handle_cable_row".to_string(),
            RowExerciseName::SeatedCableRow => "seated_cable_row".to_string(),
            RowExerciseName::SeatedDumbbellRow => "seated_dumbbell_row".to_string(),
            RowExerciseName::SingleArmCableRow => "single_arm_cable_row".to_string(),
            RowExerciseName::SingleArmCableRowAndRotation => {
                "single_arm_cable_row_and_rotation".to_string()
            }
            RowExerciseName::SingleArmInvertedRow => "single_arm_inverted_row".to_string(),
            RowExerciseName::WeightedSingleArmInvertedRow => {
                "weighted_single_arm_inverted_row".to_string()
            }
            RowExerciseName::SingleArmNeutralGripDumbbellRow => {
                "single_arm_neutral_grip_dumbbell_row".to_string()
            }
            RowExerciseName::SingleArmNeutralGripDumbbellRowAndRotation => {
                "single_arm_neutral_grip_dumbbell_row_and_rotation".to_string()
            }
            RowExerciseName::SuspendedInvertedRow => "suspended_inverted_row".to_string(),
            RowExerciseName::WeightedSuspendedInvertedRow => {
                "weighted_suspended_inverted_row".to_string()
            }
            RowExerciseName::TBarRow => "t_bar_row".to_string(),
            RowExerciseName::TowelGripInvertedRow => "towel_grip_inverted_row".to_string(),
            RowExerciseName::WeightedTowelGripInvertedRow => {
                "weighted_towel_grip_inverted_row".to_string()
            }
            RowExerciseName::UnderhandGripCableRow => "underhand_grip_cable_row".to_string(),
            RowExerciseName::VGripCableRow => "v_grip_cable_row".to_string(),
            RowExerciseName::WideGripSeatedCableRow => "wide_grip_seated_cable_row".to_string(),
            RowExerciseName::UnknownVariant(value) => format!("unknown_variant_{}", *value),
        }
    }
}

#[derive(Clone, Copy, Debug)]
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
    pub fn from_u16(value: u16) -> ShoulderPressExerciseName {
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
    pub fn from_i64(value: i64) -> ShoulderPressExerciseName {
        ShoulderPressExerciseName::from_u16(value as u16)
    }
    pub fn as_u16(&self) -> u16 {
        match &self {
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
            ShoulderPressExerciseName::UnknownVariant(value) => *value,
        }
    }
    pub fn to_string(&self) -> String {
        match &self {
            ShoulderPressExerciseName::AlternatingDumbbellShoulderPress => {
                "alternating_dumbbell_shoulder_press".to_string()
            }
            ShoulderPressExerciseName::ArnoldPress => "arnold_press".to_string(),
            ShoulderPressExerciseName::BarbellFrontSquatToPushPress => {
                "barbell_front_squat_to_push_press".to_string()
            }
            ShoulderPressExerciseName::BarbellPushPress => "barbell_push_press".to_string(),
            ShoulderPressExerciseName::BarbellShoulderPress => "barbell_shoulder_press".to_string(),
            ShoulderPressExerciseName::DeadCurlPress => "dead_curl_press".to_string(),
            ShoulderPressExerciseName::DumbbellAlternatingShoulderPressAndTwist => {
                "dumbbell_alternating_shoulder_press_and_twist".to_string()
            }
            ShoulderPressExerciseName::DumbbellHammerCurlToLungeToPress => {
                "dumbbell_hammer_curl_to_lunge_to_press".to_string()
            }
            ShoulderPressExerciseName::DumbbellPushPress => "dumbbell_push_press".to_string(),
            ShoulderPressExerciseName::FloorInvertedShoulderPress => {
                "floor_inverted_shoulder_press".to_string()
            }
            ShoulderPressExerciseName::WeightedFloorInvertedShoulderPress => {
                "weighted_floor_inverted_shoulder_press".to_string()
            }
            ShoulderPressExerciseName::InvertedShoulderPress => {
                "inverted_shoulder_press".to_string()
            }
            ShoulderPressExerciseName::WeightedInvertedShoulderPress => {
                "weighted_inverted_shoulder_press".to_string()
            }
            ShoulderPressExerciseName::OneArmPushPress => "one_arm_push_press".to_string(),
            ShoulderPressExerciseName::OverheadBarbellPress => "overhead_barbell_press".to_string(),
            ShoulderPressExerciseName::OverheadDumbbellPress => {
                "overhead_dumbbell_press".to_string()
            }
            ShoulderPressExerciseName::SeatedBarbellShoulderPress => {
                "seated_barbell_shoulder_press".to_string()
            }
            ShoulderPressExerciseName::SeatedDumbbellShoulderPress => {
                "seated_dumbbell_shoulder_press".to_string()
            }
            ShoulderPressExerciseName::SingleArmDumbbellShoulderPress => {
                "single_arm_dumbbell_shoulder_press".to_string()
            }
            ShoulderPressExerciseName::SingleArmStepUpAndPress => {
                "single_arm_step_up_and_press".to_string()
            }
            ShoulderPressExerciseName::SmithMachineOverheadPress => {
                "smith_machine_overhead_press".to_string()
            }
            ShoulderPressExerciseName::SplitStanceHammerCurlToPress => {
                "split_stance_hammer_curl_to_press".to_string()
            }
            ShoulderPressExerciseName::SwissBallDumbbellShoulderPress => {
                "swiss_ball_dumbbell_shoulder_press".to_string()
            }
            ShoulderPressExerciseName::WeightPlateFrontRaise => {
                "weight_plate_front_raise".to_string()
            }
            ShoulderPressExerciseName::UnknownVariant(value) => {
                format!("unknown_variant_{}", *value)
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
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
    pub fn from_u16(value: u16) -> ShoulderStabilityExerciseName {
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
    pub fn from_i64(value: i64) -> ShoulderStabilityExerciseName {
        ShoulderStabilityExerciseName::from_u16(value as u16)
    }
    pub fn as_u16(&self) -> u16 {
        match &self {
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
            ShoulderStabilityExerciseName::UnknownVariant(value) => *value,
        }
    }
    pub fn to_string(&self) -> String {
        match &self {
            ShoulderStabilityExerciseName::Name90DegreeCableExternalRotation => {
                "90_degree_cable_external_rotation".to_string()
            }
            ShoulderStabilityExerciseName::BandExternalRotation => {
                "band_external_rotation".to_string()
            }
            ShoulderStabilityExerciseName::BandInternalRotation => {
                "band_internal_rotation".to_string()
            }
            ShoulderStabilityExerciseName::BentArmLateralRaiseAndExternalRotation => {
                "bent_arm_lateral_raise_and_external_rotation".to_string()
            }
            ShoulderStabilityExerciseName::CableExternalRotation => {
                "cable_external_rotation".to_string()
            }
            ShoulderStabilityExerciseName::DumbbellFacePullWithExternalRotation => {
                "dumbbell_face_pull_with_external_rotation".to_string()
            }
            ShoulderStabilityExerciseName::FloorIRaise => "floor_i_raise".to_string(),
            ShoulderStabilityExerciseName::WeightedFloorIRaise => {
                "weighted_floor_i_raise".to_string()
            }
            ShoulderStabilityExerciseName::FloorTRaise => "floor_t_raise".to_string(),
            ShoulderStabilityExerciseName::WeightedFloorTRaise => {
                "weighted_floor_t_raise".to_string()
            }
            ShoulderStabilityExerciseName::FloorYRaise => "floor_y_raise".to_string(),
            ShoulderStabilityExerciseName::WeightedFloorYRaise => {
                "weighted_floor_y_raise".to_string()
            }
            ShoulderStabilityExerciseName::InclineIRaise => "incline_i_raise".to_string(),
            ShoulderStabilityExerciseName::WeightedInclineIRaise => {
                "weighted_incline_i_raise".to_string()
            }
            ShoulderStabilityExerciseName::InclineLRaise => "incline_l_raise".to_string(),
            ShoulderStabilityExerciseName::WeightedInclineLRaise => {
                "weighted_incline_l_raise".to_string()
            }
            ShoulderStabilityExerciseName::InclineTRaise => "incline_t_raise".to_string(),
            ShoulderStabilityExerciseName::WeightedInclineTRaise => {
                "weighted_incline_t_raise".to_string()
            }
            ShoulderStabilityExerciseName::InclineWRaise => "incline_w_raise".to_string(),
            ShoulderStabilityExerciseName::WeightedInclineWRaise => {
                "weighted_incline_w_raise".to_string()
            }
            ShoulderStabilityExerciseName::InclineYRaise => "incline_y_raise".to_string(),
            ShoulderStabilityExerciseName::WeightedInclineYRaise => {
                "weighted_incline_y_raise".to_string()
            }
            ShoulderStabilityExerciseName::LyingExternalRotation => {
                "lying_external_rotation".to_string()
            }
            ShoulderStabilityExerciseName::SeatedDumbbellExternalRotation => {
                "seated_dumbbell_external_rotation".to_string()
            }
            ShoulderStabilityExerciseName::StandingLRaise => "standing_l_raise".to_string(),
            ShoulderStabilityExerciseName::SwissBallIRaise => "swiss_ball_i_raise".to_string(),
            ShoulderStabilityExerciseName::WeightedSwissBallIRaise => {
                "weighted_swiss_ball_i_raise".to_string()
            }
            ShoulderStabilityExerciseName::SwissBallTRaise => "swiss_ball_t_raise".to_string(),
            ShoulderStabilityExerciseName::WeightedSwissBallTRaise => {
                "weighted_swiss_ball_t_raise".to_string()
            }
            ShoulderStabilityExerciseName::SwissBallWRaise => "swiss_ball_w_raise".to_string(),
            ShoulderStabilityExerciseName::WeightedSwissBallWRaise => {
                "weighted_swiss_ball_w_raise".to_string()
            }
            ShoulderStabilityExerciseName::SwissBallYRaise => "swiss_ball_y_raise".to_string(),
            ShoulderStabilityExerciseName::WeightedSwissBallYRaise => {
                "weighted_swiss_ball_y_raise".to_string()
            }
            ShoulderStabilityExerciseName::UnknownVariant(value) => {
                format!("unknown_variant_{}", *value)
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
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
    pub fn from_u16(value: u16) -> ShrugExerciseName {
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
    pub fn from_i64(value: i64) -> ShrugExerciseName {
        ShrugExerciseName::from_u16(value as u16)
    }
    pub fn as_u16(&self) -> u16 {
        match &self {
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
            ShrugExerciseName::UnknownVariant(value) => *value,
        }
    }
    pub fn to_string(&self) -> String {
        match &self {
            ShrugExerciseName::BarbellJumpShrug => "barbell_jump_shrug".to_string(),
            ShrugExerciseName::BarbellShrug => "barbell_shrug".to_string(),
            ShrugExerciseName::BarbellUprightRow => "barbell_upright_row".to_string(),
            ShrugExerciseName::BehindTheBackSmithMachineShrug => {
                "behind_the_back_smith_machine_shrug".to_string()
            }
            ShrugExerciseName::DumbbellJumpShrug => "dumbbell_jump_shrug".to_string(),
            ShrugExerciseName::DumbbellShrug => "dumbbell_shrug".to_string(),
            ShrugExerciseName::DumbbellUprightRow => "dumbbell_upright_row".to_string(),
            ShrugExerciseName::InclineDumbbellShrug => "incline_dumbbell_shrug".to_string(),
            ShrugExerciseName::OverheadBarbellShrug => "overhead_barbell_shrug".to_string(),
            ShrugExerciseName::OverheadDumbbellShrug => "overhead_dumbbell_shrug".to_string(),
            ShrugExerciseName::ScaptionAndShrug => "scaption_and_shrug".to_string(),
            ShrugExerciseName::ScapularRetraction => "scapular_retraction".to_string(),
            ShrugExerciseName::SerratusChairShrug => "serratus_chair_shrug".to_string(),
            ShrugExerciseName::WeightedSerratusChairShrug => {
                "weighted_serratus_chair_shrug".to_string()
            }
            ShrugExerciseName::SerratusShrug => "serratus_shrug".to_string(),
            ShrugExerciseName::WeightedSerratusShrug => "weighted_serratus_shrug".to_string(),
            ShrugExerciseName::WideGripJumpShrug => "wide_grip_jump_shrug".to_string(),
            ShrugExerciseName::UnknownVariant(value) => format!("unknown_variant_{}", *value),
        }
    }
}

#[derive(Clone, Copy, Debug)]
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
    pub fn from_u16(value: u16) -> SitUpExerciseName {
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
    pub fn from_i64(value: i64) -> SitUpExerciseName {
        SitUpExerciseName::from_u16(value as u16)
    }
    pub fn as_u16(&self) -> u16 {
        match &self {
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
            SitUpExerciseName::UnknownVariant(value) => *value,
        }
    }
    pub fn to_string(&self) -> String {
        match &self {
            SitUpExerciseName::AlternatingSitUp => "alternating_sit_up".to_string(),
            SitUpExerciseName::WeightedAlternatingSitUp => {
                "weighted_alternating_sit_up".to_string()
            }
            SitUpExerciseName::BentKneeVUp => "bent_knee_v_up".to_string(),
            SitUpExerciseName::WeightedBentKneeVUp => "weighted_bent_knee_v_up".to_string(),
            SitUpExerciseName::ButterflySitUp => "butterfly_sit_up".to_string(),
            SitUpExerciseName::WeightedButterflySitup => "weighted_butterfly_situp".to_string(),
            SitUpExerciseName::CrossPunchRollUp => "cross_punch_roll_up".to_string(),
            SitUpExerciseName::WeightedCrossPunchRollUp => {
                "weighted_cross_punch_roll_up".to_string()
            }
            SitUpExerciseName::CrossedArmsSitUp => "crossed_arms_sit_up".to_string(),
            SitUpExerciseName::WeightedCrossedArmsSitUp => {
                "weighted_crossed_arms_sit_up".to_string()
            }
            SitUpExerciseName::GetUpSitUp => "get_up_sit_up".to_string(),
            SitUpExerciseName::WeightedGetUpSitUp => "weighted_get_up_sit_up".to_string(),
            SitUpExerciseName::HoveringSitUp => "hovering_sit_up".to_string(),
            SitUpExerciseName::WeightedHoveringSitUp => "weighted_hovering_sit_up".to_string(),
            SitUpExerciseName::KettlebellSitUp => "kettlebell_sit_up".to_string(),
            SitUpExerciseName::MedicineBallAlternatingVUp => {
                "medicine_ball_alternating_v_up".to_string()
            }
            SitUpExerciseName::MedicineBallSitUp => "medicine_ball_sit_up".to_string(),
            SitUpExerciseName::MedicineBallVUp => "medicine_ball_v_up".to_string(),
            SitUpExerciseName::ModifiedSitUp => "modified_sit_up".to_string(),
            SitUpExerciseName::NegativeSitUp => "negative_sit_up".to_string(),
            SitUpExerciseName::OneArmFullSitUp => "one_arm_full_sit_up".to_string(),
            SitUpExerciseName::RecliningCircle => "reclining_circle".to_string(),
            SitUpExerciseName::WeightedRecliningCircle => "weighted_reclining_circle".to_string(),
            SitUpExerciseName::ReverseCurlUp => "reverse_curl_up".to_string(),
            SitUpExerciseName::WeightedReverseCurlUp => "weighted_reverse_curl_up".to_string(),
            SitUpExerciseName::SingleLegSwissBallJackknife => {
                "single_leg_swiss_ball_jackknife".to_string()
            }
            SitUpExerciseName::WeightedSingleLegSwissBallJackknife => {
                "weighted_single_leg_swiss_ball_jackknife".to_string()
            }
            SitUpExerciseName::TheTeaser => "the_teaser".to_string(),
            SitUpExerciseName::TheTeaserWeighted => "the_teaser_weighted".to_string(),
            SitUpExerciseName::ThreePartRollDown => "three_part_roll_down".to_string(),
            SitUpExerciseName::WeightedThreePartRollDown => {
                "weighted_three_part_roll_down".to_string()
            }
            SitUpExerciseName::VUp => "v_up".to_string(),
            SitUpExerciseName::WeightedVUp => "weighted_v_up".to_string(),
            SitUpExerciseName::WeightedRussianTwistOnSwissBall => {
                "weighted_russian_twist_on_swiss_ball".to_string()
            }
            SitUpExerciseName::WeightedSitUp => "weighted_sit_up".to_string(),
            SitUpExerciseName::XAbs => "x_abs".to_string(),
            SitUpExerciseName::WeightedXAbs => "weighted_x_abs".to_string(),
            SitUpExerciseName::SitUp => "sit_up".to_string(),
            SitUpExerciseName::UnknownVariant(value) => format!("unknown_variant_{}", *value),
        }
    }
}

#[derive(Clone, Copy, Debug)]
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
    pub fn from_u16(value: u16) -> SquatExerciseName {
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
    pub fn from_i64(value: i64) -> SquatExerciseName {
        SquatExerciseName::from_u16(value as u16)
    }
    pub fn as_u16(&self) -> u16 {
        match &self {
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
            SquatExerciseName::UnknownVariant(value) => *value,
        }
    }
    pub fn to_string(&self) -> String {
        match &self {
            SquatExerciseName::LegPress => "leg_press".to_string(),
            SquatExerciseName::BackSquatWithBodyBar => "back_squat_with_body_bar".to_string(),
            SquatExerciseName::BackSquats => "back_squats".to_string(),
            SquatExerciseName::WeightedBackSquats => "weighted_back_squats".to_string(),
            SquatExerciseName::BalancingSquat => "balancing_squat".to_string(),
            SquatExerciseName::WeightedBalancingSquat => "weighted_balancing_squat".to_string(),
            SquatExerciseName::BarbellBackSquat => "barbell_back_squat".to_string(),
            SquatExerciseName::BarbellBoxSquat => "barbell_box_squat".to_string(),
            SquatExerciseName::BarbellFrontSquat => "barbell_front_squat".to_string(),
            SquatExerciseName::BarbellHackSquat => "barbell_hack_squat".to_string(),
            SquatExerciseName::BarbellHangSquatSnatch => "barbell_hang_squat_snatch".to_string(),
            SquatExerciseName::BarbellLateralStepUp => "barbell_lateral_step_up".to_string(),
            SquatExerciseName::BarbellQuarterSquat => "barbell_quarter_squat".to_string(),
            SquatExerciseName::BarbellSiffSquat => "barbell_siff_squat".to_string(),
            SquatExerciseName::BarbellSquatSnatch => "barbell_squat_snatch".to_string(),
            SquatExerciseName::BarbellSquatWithHeelsRaised => {
                "barbell_squat_with_heels_raised".to_string()
            }
            SquatExerciseName::BarbellStepover => "barbell_stepover".to_string(),
            SquatExerciseName::BarbellStepUp => "barbell_step_up".to_string(),
            SquatExerciseName::BenchSquatWithRotationalChop => {
                "bench_squat_with_rotational_chop".to_string()
            }
            SquatExerciseName::WeightedBenchSquatWithRotationalChop => {
                "weighted_bench_squat_with_rotational_chop".to_string()
            }
            SquatExerciseName::BodyWeightWallSquat => "body_weight_wall_squat".to_string(),
            SquatExerciseName::WeightedWallSquat => "weighted_wall_squat".to_string(),
            SquatExerciseName::BoxStepSquat => "box_step_squat".to_string(),
            SquatExerciseName::WeightedBoxStepSquat => "weighted_box_step_squat".to_string(),
            SquatExerciseName::BracedSquat => "braced_squat".to_string(),
            SquatExerciseName::CrossedArmBarbellFrontSquat => {
                "crossed_arm_barbell_front_squat".to_string()
            }
            SquatExerciseName::CrossoverDumbbellStepUp => "crossover_dumbbell_step_up".to_string(),
            SquatExerciseName::DumbbellFrontSquat => "dumbbell_front_squat".to_string(),
            SquatExerciseName::DumbbellSplitSquat => "dumbbell_split_squat".to_string(),
            SquatExerciseName::DumbbellSquat => "dumbbell_squat".to_string(),
            SquatExerciseName::DumbbellSquatClean => "dumbbell_squat_clean".to_string(),
            SquatExerciseName::DumbbellStepover => "dumbbell_stepover".to_string(),
            SquatExerciseName::DumbbellStepUp => "dumbbell_step_up".to_string(),
            SquatExerciseName::ElevatedSingleLegSquat => "elevated_single_leg_squat".to_string(),
            SquatExerciseName::WeightedElevatedSingleLegSquat => {
                "weighted_elevated_single_leg_squat".to_string()
            }
            SquatExerciseName::FigureFourSquats => "figure_four_squats".to_string(),
            SquatExerciseName::WeightedFigureFourSquats => {
                "weighted_figure_four_squats".to_string()
            }
            SquatExerciseName::GobletSquat => "goblet_squat".to_string(),
            SquatExerciseName::KettlebellSquat => "kettlebell_squat".to_string(),
            SquatExerciseName::KettlebellSwingOverhead => "kettlebell_swing_overhead".to_string(),
            SquatExerciseName::KettlebellSwingWithFlipToSquat => {
                "kettlebell_swing_with_flip_to_squat".to_string()
            }
            SquatExerciseName::LateralDumbbellStepUp => "lateral_dumbbell_step_up".to_string(),
            SquatExerciseName::OneLeggedSquat => "one_legged_squat".to_string(),
            SquatExerciseName::OverheadDumbbellSquat => "overhead_dumbbell_squat".to_string(),
            SquatExerciseName::OverheadSquat => "overhead_squat".to_string(),
            SquatExerciseName::PartialSingleLegSquat => "partial_single_leg_squat".to_string(),
            SquatExerciseName::WeightedPartialSingleLegSquat => {
                "weighted_partial_single_leg_squat".to_string()
            }
            SquatExerciseName::PistolSquat => "pistol_squat".to_string(),
            SquatExerciseName::WeightedPistolSquat => "weighted_pistol_squat".to_string(),
            SquatExerciseName::PlieSlides => "plie_slides".to_string(),
            SquatExerciseName::WeightedPlieSlides => "weighted_plie_slides".to_string(),
            SquatExerciseName::PlieSquat => "plie_squat".to_string(),
            SquatExerciseName::WeightedPlieSquat => "weighted_plie_squat".to_string(),
            SquatExerciseName::PrisonerSquat => "prisoner_squat".to_string(),
            SquatExerciseName::WeightedPrisonerSquat => "weighted_prisoner_squat".to_string(),
            SquatExerciseName::SingleLegBenchGetUp => "single_leg_bench_get_up".to_string(),
            SquatExerciseName::WeightedSingleLegBenchGetUp => {
                "weighted_single_leg_bench_get_up".to_string()
            }
            SquatExerciseName::SingleLegBenchSquat => "single_leg_bench_squat".to_string(),
            SquatExerciseName::WeightedSingleLegBenchSquat => {
                "weighted_single_leg_bench_squat".to_string()
            }
            SquatExerciseName::SingleLegSquatOnSwissBall => {
                "single_leg_squat_on_swiss_ball".to_string()
            }
            SquatExerciseName::WeightedSingleLegSquatOnSwissBall => {
                "weighted_single_leg_squat_on_swiss_ball".to_string()
            }
            SquatExerciseName::Squat => "squat".to_string(),
            SquatExerciseName::WeightedSquat => "weighted_squat".to_string(),
            SquatExerciseName::SquatsWithBand => "squats_with_band".to_string(),
            SquatExerciseName::StaggeredSquat => "staggered_squat".to_string(),
            SquatExerciseName::WeightedStaggeredSquat => "weighted_staggered_squat".to_string(),
            SquatExerciseName::StepUp => "step_up".to_string(),
            SquatExerciseName::WeightedStepUp => "weighted_step_up".to_string(),
            SquatExerciseName::SuitcaseSquats => "suitcase_squats".to_string(),
            SquatExerciseName::SumoSquat => "sumo_squat".to_string(),
            SquatExerciseName::SumoSquatSlideIn => "sumo_squat_slide_in".to_string(),
            SquatExerciseName::WeightedSumoSquatSlideIn => {
                "weighted_sumo_squat_slide_in".to_string()
            }
            SquatExerciseName::SumoSquatToHighPull => "sumo_squat_to_high_pull".to_string(),
            SquatExerciseName::SumoSquatToStand => "sumo_squat_to_stand".to_string(),
            SquatExerciseName::WeightedSumoSquatToStand => {
                "weighted_sumo_squat_to_stand".to_string()
            }
            SquatExerciseName::SumoSquatWithRotation => "sumo_squat_with_rotation".to_string(),
            SquatExerciseName::WeightedSumoSquatWithRotation => {
                "weighted_sumo_squat_with_rotation".to_string()
            }
            SquatExerciseName::SwissBallBodyWeightWallSquat => {
                "swiss_ball_body_weight_wall_squat".to_string()
            }
            SquatExerciseName::WeightedSwissBallWallSquat => {
                "weighted_swiss_ball_wall_squat".to_string()
            }
            SquatExerciseName::Thrusters => "thrusters".to_string(),
            SquatExerciseName::UnevenSquat => "uneven_squat".to_string(),
            SquatExerciseName::WeightedUnevenSquat => "weighted_uneven_squat".to_string(),
            SquatExerciseName::WaistSlimmingSquat => "waist_slimming_squat".to_string(),
            SquatExerciseName::WallBall => "wall_ball".to_string(),
            SquatExerciseName::WideStanceBarbellSquat => "wide_stance_barbell_squat".to_string(),
            SquatExerciseName::WideStanceGobletSquat => "wide_stance_goblet_squat".to_string(),
            SquatExerciseName::ZercherSquat => "zercher_squat".to_string(),
            SquatExerciseName::KbsOverhead => "kbs_overhead".to_string(),
            SquatExerciseName::SquatAndSideKick => "squat_and_side_kick".to_string(),
            SquatExerciseName::SquatJumpsInNOut => "squat_jumps_in_n_out".to_string(),
            SquatExerciseName::PilatesPlieSquatsParallelTurnedOutFlatAndHeels => {
                "pilates_plie_squats_parallel_turned_out_flat_and_heels".to_string()
            }
            SquatExerciseName::ReleveStraightLegAndKneeBentWithOneLegVariation => {
                "releve_straight_leg_and_knee_bent_with_one_leg_variation".to_string()
            }
            SquatExerciseName::UnknownVariant(value) => format!("unknown_variant_{}", *value),
        }
    }
}

#[derive(Clone, Copy, Debug)]
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
    pub fn from_u16(value: u16) -> TotalBodyExerciseName {
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
    pub fn from_i64(value: i64) -> TotalBodyExerciseName {
        TotalBodyExerciseName::from_u16(value as u16)
    }
    pub fn as_u16(&self) -> u16 {
        match &self {
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
            TotalBodyExerciseName::UnknownVariant(value) => *value,
        }
    }
    pub fn to_string(&self) -> String {
        match &self {
            TotalBodyExerciseName::Burpee => "burpee".to_string(),
            TotalBodyExerciseName::WeightedBurpee => "weighted_burpee".to_string(),
            TotalBodyExerciseName::BurpeeBoxJump => "burpee_box_jump".to_string(),
            TotalBodyExerciseName::WeightedBurpeeBoxJump => "weighted_burpee_box_jump".to_string(),
            TotalBodyExerciseName::HighPullBurpee => "high_pull_burpee".to_string(),
            TotalBodyExerciseName::ManMakers => "man_makers".to_string(),
            TotalBodyExerciseName::OneArmBurpee => "one_arm_burpee".to_string(),
            TotalBodyExerciseName::SquatThrusts => "squat_thrusts".to_string(),
            TotalBodyExerciseName::WeightedSquatThrusts => "weighted_squat_thrusts".to_string(),
            TotalBodyExerciseName::SquatPlankPushUp => "squat_plank_push_up".to_string(),
            TotalBodyExerciseName::WeightedSquatPlankPushUp => {
                "weighted_squat_plank_push_up".to_string()
            }
            TotalBodyExerciseName::StandingTRotationBalance => {
                "standing_t_rotation_balance".to_string()
            }
            TotalBodyExerciseName::WeightedStandingTRotationBalance => {
                "weighted_standing_t_rotation_balance".to_string()
            }
            TotalBodyExerciseName::UnknownVariant(value) => format!("unknown_variant_{}", *value),
        }
    }
}

#[derive(Clone, Copy, Debug)]
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
    pub fn from_u16(value: u16) -> TricepsExtensionExerciseName {
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
    pub fn from_i64(value: i64) -> TricepsExtensionExerciseName {
        TricepsExtensionExerciseName::from_u16(value as u16)
    }
    pub fn as_u16(&self) -> u16 {
        match &self {
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
            TricepsExtensionExerciseName::UnknownVariant(value) => *value,
        }
    }
    pub fn to_string(&self) -> String {
        match &self {
            TricepsExtensionExerciseName::BenchDip => "bench_dip".to_string(),
            TricepsExtensionExerciseName::WeightedBenchDip => "weighted_bench_dip".to_string(),
            TricepsExtensionExerciseName::BodyWeightDip => "body_weight_dip".to_string(),
            TricepsExtensionExerciseName::CableKickback => "cable_kickback".to_string(),
            TricepsExtensionExerciseName::CableLyingTricepsExtension => {
                "cable_lying_triceps_extension".to_string()
            }
            TricepsExtensionExerciseName::CableOverheadTricepsExtension => {
                "cable_overhead_triceps_extension".to_string()
            }
            TricepsExtensionExerciseName::DumbbellKickback => "dumbbell_kickback".to_string(),
            TricepsExtensionExerciseName::DumbbellLyingTricepsExtension => {
                "dumbbell_lying_triceps_extension".to_string()
            }
            TricepsExtensionExerciseName::EzBarOverheadTricepsExtension => {
                "ez_bar_overhead_triceps_extension".to_string()
            }
            TricepsExtensionExerciseName::InclineDip => "incline_dip".to_string(),
            TricepsExtensionExerciseName::WeightedInclineDip => "weighted_incline_dip".to_string(),
            TricepsExtensionExerciseName::InclineEzBarLyingTricepsExtension => {
                "incline_ez_bar_lying_triceps_extension".to_string()
            }
            TricepsExtensionExerciseName::LyingDumbbellPulloverToExtension => {
                "lying_dumbbell_pullover_to_extension".to_string()
            }
            TricepsExtensionExerciseName::LyingEzBarTricepsExtension => {
                "lying_ez_bar_triceps_extension".to_string()
            }
            TricepsExtensionExerciseName::LyingTricepsExtensionToCloseGripBenchPress => {
                "lying_triceps_extension_to_close_grip_bench_press".to_string()
            }
            TricepsExtensionExerciseName::OverheadDumbbellTricepsExtension => {
                "overhead_dumbbell_triceps_extension".to_string()
            }
            TricepsExtensionExerciseName::RecliningTricepsPress => {
                "reclining_triceps_press".to_string()
            }
            TricepsExtensionExerciseName::ReverseGripPressdown => {
                "reverse_grip_pressdown".to_string()
            }
            TricepsExtensionExerciseName::ReverseGripTricepsPressdown => {
                "reverse_grip_triceps_pressdown".to_string()
            }
            TricepsExtensionExerciseName::RopePressdown => "rope_pressdown".to_string(),
            TricepsExtensionExerciseName::SeatedBarbellOverheadTricepsExtension => {
                "seated_barbell_overhead_triceps_extension".to_string()
            }
            TricepsExtensionExerciseName::SeatedDumbbellOverheadTricepsExtension => {
                "seated_dumbbell_overhead_triceps_extension".to_string()
            }
            TricepsExtensionExerciseName::SeatedEzBarOverheadTricepsExtension => {
                "seated_ez_bar_overhead_triceps_extension".to_string()
            }
            TricepsExtensionExerciseName::SeatedSingleArmOverheadDumbbellExtension => {
                "seated_single_arm_overhead_dumbbell_extension".to_string()
            }
            TricepsExtensionExerciseName::SingleArmDumbbellOverheadTricepsExtension => {
                "single_arm_dumbbell_overhead_triceps_extension".to_string()
            }
            TricepsExtensionExerciseName::SingleDumbbellSeatedOverheadTricepsExtension => {
                "single_dumbbell_seated_overhead_triceps_extension".to_string()
            }
            TricepsExtensionExerciseName::SingleLegBenchDipAndKick => {
                "single_leg_bench_dip_and_kick".to_string()
            }
            TricepsExtensionExerciseName::WeightedSingleLegBenchDipAndKick => {
                "weighted_single_leg_bench_dip_and_kick".to_string()
            }
            TricepsExtensionExerciseName::SingleLegDip => "single_leg_dip".to_string(),
            TricepsExtensionExerciseName::WeightedSingleLegDip => {
                "weighted_single_leg_dip".to_string()
            }
            TricepsExtensionExerciseName::StaticLyingTricepsExtension => {
                "static_lying_triceps_extension".to_string()
            }
            TricepsExtensionExerciseName::SuspendedDip => "suspended_dip".to_string(),
            TricepsExtensionExerciseName::WeightedSuspendedDip => {
                "weighted_suspended_dip".to_string()
            }
            TricepsExtensionExerciseName::SwissBallDumbbellLyingTricepsExtension => {
                "swiss_ball_dumbbell_lying_triceps_extension".to_string()
            }
            TricepsExtensionExerciseName::SwissBallEzBarLyingTricepsExtension => {
                "swiss_ball_ez_bar_lying_triceps_extension".to_string()
            }
            TricepsExtensionExerciseName::SwissBallEzBarOverheadTricepsExtension => {
                "swiss_ball_ez_bar_overhead_triceps_extension".to_string()
            }
            TricepsExtensionExerciseName::TabletopDip => "tabletop_dip".to_string(),
            TricepsExtensionExerciseName::WeightedTabletopDip => {
                "weighted_tabletop_dip".to_string()
            }
            TricepsExtensionExerciseName::TricepsExtensionOnFloor => {
                "triceps_extension_on_floor".to_string()
            }
            TricepsExtensionExerciseName::TricepsPressdown => "triceps_pressdown".to_string(),
            TricepsExtensionExerciseName::WeightedDip => "weighted_dip".to_string(),
            TricepsExtensionExerciseName::UnknownVariant(value) => {
                format!("unknown_variant_{}", *value)
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
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
    pub fn from_u16(value: u16) -> WarmUpExerciseName {
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
    pub fn from_i64(value: i64) -> WarmUpExerciseName {
        WarmUpExerciseName::from_u16(value as u16)
    }
    pub fn as_u16(&self) -> u16 {
        match &self {
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
            WarmUpExerciseName::UnknownVariant(value) => *value,
        }
    }
    pub fn to_string(&self) -> String {
        match &self {
            WarmUpExerciseName::QuadrupedRocking => "quadruped_rocking".to_string(),
            WarmUpExerciseName::NeckTilts => "neck_tilts".to_string(),
            WarmUpExerciseName::AnkleCircles => "ankle_circles".to_string(),
            WarmUpExerciseName::AnkleDorsiflexionWithBand => {
                "ankle_dorsiflexion_with_band".to_string()
            }
            WarmUpExerciseName::AnkleInternalRotation => "ankle_internal_rotation".to_string(),
            WarmUpExerciseName::ArmCircles => "arm_circles".to_string(),
            WarmUpExerciseName::BentOverReachToSky => "bent_over_reach_to_sky".to_string(),
            WarmUpExerciseName::CatCamel => "cat_camel".to_string(),
            WarmUpExerciseName::ElbowToFootLunge => "elbow_to_foot_lunge".to_string(),
            WarmUpExerciseName::ForwardAndBackwardLegSwings => {
                "forward_and_backward_leg_swings".to_string()
            }
            WarmUpExerciseName::Groiners => "groiners".to_string(),
            WarmUpExerciseName::InvertedHamstringStretch => {
                "inverted_hamstring_stretch".to_string()
            }
            WarmUpExerciseName::LateralDuckUnder => "lateral_duck_under".to_string(),
            WarmUpExerciseName::NeckRotations => "neck_rotations".to_string(),
            WarmUpExerciseName::OppositeArmAndLegBalance => {
                "opposite_arm_and_leg_balance".to_string()
            }
            WarmUpExerciseName::ReachRollAndLift => "reach_roll_and_lift".to_string(),
            WarmUpExerciseName::Scorpion => "scorpion".to_string(),
            WarmUpExerciseName::ShoulderCircles => "shoulder_circles".to_string(),
            WarmUpExerciseName::SideToSideLegSwings => "side_to_side_leg_swings".to_string(),
            WarmUpExerciseName::SleeperStretch => "sleeper_stretch".to_string(),
            WarmUpExerciseName::SlideOut => "slide_out".to_string(),
            WarmUpExerciseName::SwissBallHipCrossover => "swiss_ball_hip_crossover".to_string(),
            WarmUpExerciseName::SwissBallReachRollAndLift => {
                "swiss_ball_reach_roll_and_lift".to_string()
            }
            WarmUpExerciseName::SwissBallWindshieldWipers => {
                "swiss_ball_windshield_wipers".to_string()
            }
            WarmUpExerciseName::ThoracicRotation => "thoracic_rotation".to_string(),
            WarmUpExerciseName::WalkingHighKicks => "walking_high_kicks".to_string(),
            WarmUpExerciseName::WalkingHighKnees => "walking_high_knees".to_string(),
            WarmUpExerciseName::WalkingKneeHugs => "walking_knee_hugs".to_string(),
            WarmUpExerciseName::WalkingLegCradles => "walking_leg_cradles".to_string(),
            WarmUpExerciseName::Walkout => "walkout".to_string(),
            WarmUpExerciseName::WalkoutFromPushUpPosition => {
                "walkout_from_push_up_position".to_string()
            }
            WarmUpExerciseName::UnknownVariant(value) => format!("unknown_variant_{}", *value),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum RunExerciseName {
    Run,
    Walk,
    Jog,
    Sprint,
    UnknownVariant(u16),
}
impl RunExerciseName {
    pub fn from_u16(value: u16) -> RunExerciseName {
        match value {
            0 => RunExerciseName::Run,
            1 => RunExerciseName::Walk,
            2 => RunExerciseName::Jog,
            3 => RunExerciseName::Sprint,
            _ => RunExerciseName::UnknownVariant(value),
        }
    }
    pub fn from_i64(value: i64) -> RunExerciseName {
        RunExerciseName::from_u16(value as u16)
    }
    pub fn as_u16(&self) -> u16 {
        match &self {
            RunExerciseName::Run => 0,
            RunExerciseName::Walk => 1,
            RunExerciseName::Jog => 2,
            RunExerciseName::Sprint => 3,
            RunExerciseName::UnknownVariant(value) => *value,
        }
    }
    pub fn to_string(&self) -> String {
        match &self {
            RunExerciseName::Run => "run".to_string(),
            RunExerciseName::Walk => "walk".to_string(),
            RunExerciseName::Jog => "jog".to_string(),
            RunExerciseName::Sprint => "sprint".to_string(),
            RunExerciseName::UnknownVariant(value) => format!("unknown_variant_{}", *value),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum WaterType {
    Fresh,
    Salt,
    En13319,
    Custom,
    UnknownVariant(u8),
}
impl WaterType {
    pub fn from_u8(value: u8) -> WaterType {
        match value {
            0 => WaterType::Fresh,
            1 => WaterType::Salt,
            2 => WaterType::En13319,
            3 => WaterType::Custom,
            _ => WaterType::UnknownVariant(value),
        }
    }
    pub fn from_i64(value: i64) -> WaterType {
        WaterType::from_u8(value as u8)
    }
    pub fn as_u8(&self) -> u8 {
        match &self {
            WaterType::Fresh => 0,
            WaterType::Salt => 1,
            WaterType::En13319 => 2,
            WaterType::Custom => 3,
            WaterType::UnknownVariant(value) => *value,
        }
    }
    pub fn to_string(&self) -> String {
        match &self {
            WaterType::Fresh => "fresh".to_string(),
            WaterType::Salt => "salt".to_string(),
            WaterType::En13319 => "en13319".to_string(),
            WaterType::Custom => "custom".to_string(),
            WaterType::UnknownVariant(value) => format!("unknown_variant_{}", *value),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum TissueModelType {
    Zhl16c, // Buhlmann's decompression algorithm, version C
    UnknownVariant(u8),
}
impl TissueModelType {
    pub fn from_u8(value: u8) -> TissueModelType {
        match value {
            0 => TissueModelType::Zhl16c,
            _ => TissueModelType::UnknownVariant(value),
        }
    }
    pub fn from_i64(value: i64) -> TissueModelType {
        TissueModelType::from_u8(value as u8)
    }
    pub fn as_u8(&self) -> u8 {
        match &self {
            TissueModelType::Zhl16c => 0,
            TissueModelType::UnknownVariant(value) => *value,
        }
    }
    pub fn to_string(&self) -> String {
        match &self {
            TissueModelType::Zhl16c => "zhl_16c".to_string(),
            TissueModelType::UnknownVariant(value) => format!("unknown_variant_{}", *value),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum DiveGasStatus {
    Disabled,
    Enabled,
    BackupOnly,
    UnknownVariant(u8),
}
impl DiveGasStatus {
    pub fn from_u8(value: u8) -> DiveGasStatus {
        match value {
            0 => DiveGasStatus::Disabled,
            1 => DiveGasStatus::Enabled,
            2 => DiveGasStatus::BackupOnly,
            _ => DiveGasStatus::UnknownVariant(value),
        }
    }
    pub fn from_i64(value: i64) -> DiveGasStatus {
        DiveGasStatus::from_u8(value as u8)
    }
    pub fn as_u8(&self) -> u8 {
        match &self {
            DiveGasStatus::Disabled => 0,
            DiveGasStatus::Enabled => 1,
            DiveGasStatus::BackupOnly => 2,
            DiveGasStatus::UnknownVariant(value) => *value,
        }
    }
    pub fn to_string(&self) -> String {
        match &self {
            DiveGasStatus::Disabled => "disabled".to_string(),
            DiveGasStatus::Enabled => "enabled".to_string(),
            DiveGasStatus::BackupOnly => "backup_only".to_string(),
            DiveGasStatus::UnknownVariant(value) => format!("unknown_variant_{}", *value),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum DiveAlarmType {
    Depth,
    Time,
    UnknownVariant(u8),
}
impl DiveAlarmType {
    pub fn from_u8(value: u8) -> DiveAlarmType {
        match value {
            0 => DiveAlarmType::Depth,
            1 => DiveAlarmType::Time,
            _ => DiveAlarmType::UnknownVariant(value),
        }
    }
    pub fn from_i64(value: i64) -> DiveAlarmType {
        DiveAlarmType::from_u8(value as u8)
    }
    pub fn as_u8(&self) -> u8 {
        match &self {
            DiveAlarmType::Depth => 0,
            DiveAlarmType::Time => 1,
            DiveAlarmType::UnknownVariant(value) => *value,
        }
    }
    pub fn to_string(&self) -> String {
        match &self {
            DiveAlarmType::Depth => "depth".to_string(),
            DiveAlarmType::Time => "time".to_string(),
            DiveAlarmType::UnknownVariant(value) => format!("unknown_variant_{}", *value),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum DiveBacklightMode {
    AtDepth,
    AlwaysOn,
    UnknownVariant(u8),
}
impl DiveBacklightMode {
    pub fn from_u8(value: u8) -> DiveBacklightMode {
        match value {
            0 => DiveBacklightMode::AtDepth,
            1 => DiveBacklightMode::AlwaysOn,
            _ => DiveBacklightMode::UnknownVariant(value),
        }
    }
    pub fn from_i64(value: i64) -> DiveBacklightMode {
        DiveBacklightMode::from_u8(value as u8)
    }
    pub fn as_u8(&self) -> u8 {
        match &self {
            DiveBacklightMode::AtDepth => 0,
            DiveBacklightMode::AlwaysOn => 1,
            DiveBacklightMode::UnknownVariant(value) => *value,
        }
    }
    pub fn to_string(&self) -> String {
        match &self {
            DiveBacklightMode::AtDepth => "at_depth".to_string(),
            DiveBacklightMode::AlwaysOn => "always_on".to_string(),
            DiveBacklightMode::UnknownVariant(value) => format!("unknown_variant_{}", *value),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum FaveroProduct {
    AssiomaUno,
    AssiomaDuo,
    UnknownVariant(u16),
}
impl FaveroProduct {
    pub fn from_u16(value: u16) -> FaveroProduct {
        match value {
            10 => FaveroProduct::AssiomaUno,
            12 => FaveroProduct::AssiomaDuo,
            _ => FaveroProduct::UnknownVariant(value),
        }
    }
    pub fn from_i64(value: i64) -> FaveroProduct {
        FaveroProduct::from_u16(value as u16)
    }
    pub fn as_u16(&self) -> u16 {
        match &self {
            FaveroProduct::AssiomaUno => 10,
            FaveroProduct::AssiomaDuo => 12,
            FaveroProduct::UnknownVariant(value) => *value,
        }
    }
    pub fn to_string(&self) -> String {
        match &self {
            FaveroProduct::AssiomaUno => "assioma_uno".to_string(),
            FaveroProduct::AssiomaDuo => "assioma_duo".to_string(),
            FaveroProduct::UnknownVariant(value) => format!("unknown_variant_{}", *value),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum ClimbProEvent {
    Approach,
    Start,
    Complete,
    UnknownVariant(u8),
}
impl ClimbProEvent {
    pub fn from_u8(value: u8) -> ClimbProEvent {
        match value {
            0 => ClimbProEvent::Approach,
            1 => ClimbProEvent::Start,
            2 => ClimbProEvent::Complete,
            _ => ClimbProEvent::UnknownVariant(value),
        }
    }
    pub fn from_i64(value: i64) -> ClimbProEvent {
        ClimbProEvent::from_u8(value as u8)
    }
    pub fn as_u8(&self) -> u8 {
        match &self {
            ClimbProEvent::Approach => 0,
            ClimbProEvent::Start => 1,
            ClimbProEvent::Complete => 2,
            ClimbProEvent::UnknownVariant(value) => *value,
        }
    }
    pub fn to_string(&self) -> String {
        match &self {
            ClimbProEvent::Approach => "approach".to_string(),
            ClimbProEvent::Start => "start".to_string(),
            ClimbProEvent::Complete => "complete".to_string(),
            ClimbProEvent::UnknownVariant(value) => format!("unknown_variant_{}", *value),
        }
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
    pub fn is_enum_type(&self) -> bool {
        match self {
            FieldDataType::File => true,
            FieldDataType::MesgNum => true,
            FieldDataType::Checksum => true,
            FieldDataType::FileFlags => true,
            FieldDataType::MesgCount => true,
            FieldDataType::DateTime => true,
            FieldDataType::LocalDateTime => true,
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
        FieldDataType::File => File::from_i64(value).to_string(),
        FieldDataType::MesgNum => MesgNum::from_i64(value).to_string(),
        FieldDataType::Checksum => Checksum::from_i64(value).to_string(),
        FieldDataType::FileFlags => FileFlags::from_i64(value).to_string(),
        FieldDataType::MesgCount => MesgCount::from_i64(value).to_string(),
        FieldDataType::DateTime => DateTime::from_i64(value).to_string(),
        FieldDataType::LocalDateTime => LocalDateTime::from_i64(value).to_string(),
        FieldDataType::MessageIndex => MessageIndex::from_i64(value).to_string(),
        FieldDataType::DeviceIndex => DeviceIndex::from_i64(value).to_string(),
        FieldDataType::Gender => Gender::from_i64(value).to_string(),
        FieldDataType::Language => Language::from_i64(value).to_string(),
        FieldDataType::LanguageBits0 => LanguageBits0::from_i64(value).to_string(),
        FieldDataType::LanguageBits1 => LanguageBits1::from_i64(value).to_string(),
        FieldDataType::LanguageBits2 => LanguageBits2::from_i64(value).to_string(),
        FieldDataType::LanguageBits3 => LanguageBits3::from_i64(value).to_string(),
        FieldDataType::LanguageBits4 => LanguageBits4::from_i64(value).to_string(),
        FieldDataType::TimeZone => TimeZone::from_i64(value).to_string(),
        FieldDataType::DisplayMeasure => DisplayMeasure::from_i64(value).to_string(),
        FieldDataType::DisplayHeart => DisplayHeart::from_i64(value).to_string(),
        FieldDataType::DisplayPower => DisplayPower::from_i64(value).to_string(),
        FieldDataType::DisplayPosition => DisplayPosition::from_i64(value).to_string(),
        FieldDataType::Switch => Switch::from_i64(value).to_string(),
        FieldDataType::Sport => Sport::from_i64(value).to_string(),
        FieldDataType::SportBits0 => SportBits0::from_i64(value).to_string(),
        FieldDataType::SportBits1 => SportBits1::from_i64(value).to_string(),
        FieldDataType::SportBits2 => SportBits2::from_i64(value).to_string(),
        FieldDataType::SportBits3 => SportBits3::from_i64(value).to_string(),
        FieldDataType::SportBits4 => SportBits4::from_i64(value).to_string(),
        FieldDataType::SportBits5 => SportBits5::from_i64(value).to_string(),
        FieldDataType::SportBits6 => SportBits6::from_i64(value).to_string(),
        FieldDataType::SubSport => SubSport::from_i64(value).to_string(),
        FieldDataType::SportEvent => SportEvent::from_i64(value).to_string(),
        FieldDataType::Activity => Activity::from_i64(value).to_string(),
        FieldDataType::Intensity => Intensity::from_i64(value).to_string(),
        FieldDataType::SessionTrigger => SessionTrigger::from_i64(value).to_string(),
        FieldDataType::AutolapTrigger => AutolapTrigger::from_i64(value).to_string(),
        FieldDataType::LapTrigger => LapTrigger::from_i64(value).to_string(),
        FieldDataType::TimeMode => TimeMode::from_i64(value).to_string(),
        FieldDataType::BacklightMode => BacklightMode::from_i64(value).to_string(),
        FieldDataType::DateMode => DateMode::from_i64(value).to_string(),
        FieldDataType::BacklightTimeout => BacklightTimeout::from_i64(value).to_string(),
        FieldDataType::Event => Event::from_i64(value).to_string(),
        FieldDataType::EventType => EventType::from_i64(value).to_string(),
        FieldDataType::TimerTrigger => TimerTrigger::from_i64(value).to_string(),
        FieldDataType::FitnessEquipmentState => FitnessEquipmentState::from_i64(value).to_string(),
        FieldDataType::Tone => Tone::from_i64(value).to_string(),
        FieldDataType::Autoscroll => Autoscroll::from_i64(value).to_string(),
        FieldDataType::ActivityClass => ActivityClass::from_i64(value).to_string(),
        FieldDataType::HrZoneCalc => HrZoneCalc::from_i64(value).to_string(),
        FieldDataType::PwrZoneCalc => PwrZoneCalc::from_i64(value).to_string(),
        FieldDataType::WktStepDuration => WktStepDuration::from_i64(value).to_string(),
        FieldDataType::WktStepTarget => WktStepTarget::from_i64(value).to_string(),
        FieldDataType::Goal => Goal::from_i64(value).to_string(),
        FieldDataType::GoalRecurrence => GoalRecurrence::from_i64(value).to_string(),
        FieldDataType::GoalSource => GoalSource::from_i64(value).to_string(),
        FieldDataType::Schedule => Schedule::from_i64(value).to_string(),
        FieldDataType::CoursePoint => CoursePoint::from_i64(value).to_string(),
        FieldDataType::Manufacturer => Manufacturer::from_i64(value).to_string(),
        FieldDataType::GarminProduct => GarminProduct::from_i64(value).to_string(),
        FieldDataType::AntplusDeviceType => AntplusDeviceType::from_i64(value).to_string(),
        FieldDataType::AntNetwork => AntNetwork::from_i64(value).to_string(),
        FieldDataType::WorkoutCapabilities => WorkoutCapabilities::from_i64(value).to_string(),
        FieldDataType::BatteryStatus => BatteryStatus::from_i64(value).to_string(),
        FieldDataType::HrType => HrType::from_i64(value).to_string(),
        FieldDataType::CourseCapabilities => CourseCapabilities::from_i64(value).to_string(),
        FieldDataType::Weight => Weight::from_i64(value).to_string(),
        FieldDataType::WorkoutHr => WorkoutHr::from_i64(value).to_string(),
        FieldDataType::WorkoutPower => WorkoutPower::from_i64(value).to_string(),
        FieldDataType::BpStatus => BpStatus::from_i64(value).to_string(),
        FieldDataType::UserLocalId => UserLocalId::from_i64(value).to_string(),
        FieldDataType::SwimStroke => SwimStroke::from_i64(value).to_string(),
        FieldDataType::ActivityType => ActivityType::from_i64(value).to_string(),
        FieldDataType::ActivitySubtype => ActivitySubtype::from_i64(value).to_string(),
        FieldDataType::ActivityLevel => ActivityLevel::from_i64(value).to_string(),
        FieldDataType::Side => Side::from_i64(value).to_string(),
        FieldDataType::LeftRightBalance => LeftRightBalance::from_i64(value).to_string(),
        FieldDataType::LeftRightBalance100 => LeftRightBalance100::from_i64(value).to_string(),
        FieldDataType::LengthType => LengthType::from_i64(value).to_string(),
        FieldDataType::DayOfWeek => DayOfWeek::from_i64(value).to_string(),
        FieldDataType::ConnectivityCapabilities => {
            ConnectivityCapabilities::from_i64(value).to_string()
        }
        FieldDataType::WeatherReport => WeatherReport::from_i64(value).to_string(),
        FieldDataType::WeatherStatus => WeatherStatus::from_i64(value).to_string(),
        FieldDataType::WeatherSeverity => WeatherSeverity::from_i64(value).to_string(),
        FieldDataType::WeatherSevereType => WeatherSevereType::from_i64(value).to_string(),
        FieldDataType::StrokeType => StrokeType::from_i64(value).to_string(),
        FieldDataType::BodyLocation => BodyLocation::from_i64(value).to_string(),
        FieldDataType::SegmentLapStatus => SegmentLapStatus::from_i64(value).to_string(),
        FieldDataType::SegmentLeaderboardType => {
            SegmentLeaderboardType::from_i64(value).to_string()
        }
        FieldDataType::SegmentDeleteStatus => SegmentDeleteStatus::from_i64(value).to_string(),
        FieldDataType::SegmentSelectionType => SegmentSelectionType::from_i64(value).to_string(),
        FieldDataType::SourceType => SourceType::from_i64(value).to_string(),
        FieldDataType::DisplayOrientation => DisplayOrientation::from_i64(value).to_string(),
        FieldDataType::WorkoutEquipment => WorkoutEquipment::from_i64(value).to_string(),
        FieldDataType::WatchfaceMode => WatchfaceMode::from_i64(value).to_string(),
        FieldDataType::DigitalWatchfaceLayout => {
            DigitalWatchfaceLayout::from_i64(value).to_string()
        }
        FieldDataType::AnalogWatchfaceLayout => AnalogWatchfaceLayout::from_i64(value).to_string(),
        FieldDataType::RiderPositionType => RiderPositionType::from_i64(value).to_string(),
        FieldDataType::PowerPhaseType => PowerPhaseType::from_i64(value).to_string(),
        FieldDataType::CameraEventType => CameraEventType::from_i64(value).to_string(),
        FieldDataType::SensorType => SensorType::from_i64(value).to_string(),
        FieldDataType::BikeLightNetworkConfigType => {
            BikeLightNetworkConfigType::from_i64(value).to_string()
        }
        FieldDataType::CommTimeoutType => CommTimeoutType::from_i64(value).to_string(),
        FieldDataType::CameraOrientationType => CameraOrientationType::from_i64(value).to_string(),
        FieldDataType::AttitudeStage => AttitudeStage::from_i64(value).to_string(),
        FieldDataType::AttitudeValidity => AttitudeValidity::from_i64(value).to_string(),
        FieldDataType::AutoSyncFrequency => AutoSyncFrequency::from_i64(value).to_string(),
        FieldDataType::ExdLayout => ExdLayout::from_i64(value).to_string(),
        FieldDataType::ExdDisplayType => ExdDisplayType::from_i64(value).to_string(),
        FieldDataType::ExdDataUnits => ExdDataUnits::from_i64(value).to_string(),
        FieldDataType::ExdQualifiers => ExdQualifiers::from_i64(value).to_string(),
        FieldDataType::ExdDescriptors => ExdDescriptors::from_i64(value).to_string(),
        FieldDataType::AutoActivityDetect => AutoActivityDetect::from_i64(value).to_string(),
        FieldDataType::SupportedExdScreenLayouts => {
            SupportedExdScreenLayouts::from_i64(value).to_string()
        }
        FieldDataType::FitBaseType => FitBaseType::from_i64(value).to_string(),
        FieldDataType::TurnType => TurnType::from_i64(value).to_string(),
        FieldDataType::BikeLightBeamAngleMode => {
            BikeLightBeamAngleMode::from_i64(value).to_string()
        }
        FieldDataType::FitBaseUnit => FitBaseUnit::from_i64(value).to_string(),
        FieldDataType::SetType => SetType::from_i64(value).to_string(),
        FieldDataType::ExerciseCategory => ExerciseCategory::from_i64(value).to_string(),
        FieldDataType::BenchPressExerciseName => {
            BenchPressExerciseName::from_i64(value).to_string()
        }
        FieldDataType::CalfRaiseExerciseName => CalfRaiseExerciseName::from_i64(value).to_string(),
        FieldDataType::CardioExerciseName => CardioExerciseName::from_i64(value).to_string(),
        FieldDataType::CarryExerciseName => CarryExerciseName::from_i64(value).to_string(),
        FieldDataType::ChopExerciseName => ChopExerciseName::from_i64(value).to_string(),
        FieldDataType::CoreExerciseName => CoreExerciseName::from_i64(value).to_string(),
        FieldDataType::CrunchExerciseName => CrunchExerciseName::from_i64(value).to_string(),
        FieldDataType::CurlExerciseName => CurlExerciseName::from_i64(value).to_string(),
        FieldDataType::DeadliftExerciseName => DeadliftExerciseName::from_i64(value).to_string(),
        FieldDataType::FlyeExerciseName => FlyeExerciseName::from_i64(value).to_string(),
        FieldDataType::HipRaiseExerciseName => HipRaiseExerciseName::from_i64(value).to_string(),
        FieldDataType::HipStabilityExerciseName => {
            HipStabilityExerciseName::from_i64(value).to_string()
        }
        FieldDataType::HipSwingExerciseName => HipSwingExerciseName::from_i64(value).to_string(),
        FieldDataType::HyperextensionExerciseName => {
            HyperextensionExerciseName::from_i64(value).to_string()
        }
        FieldDataType::LateralRaiseExerciseName => {
            LateralRaiseExerciseName::from_i64(value).to_string()
        }
        FieldDataType::LegCurlExerciseName => LegCurlExerciseName::from_i64(value).to_string(),
        FieldDataType::LegRaiseExerciseName => LegRaiseExerciseName::from_i64(value).to_string(),
        FieldDataType::LungeExerciseName => LungeExerciseName::from_i64(value).to_string(),
        FieldDataType::OlympicLiftExerciseName => {
            OlympicLiftExerciseName::from_i64(value).to_string()
        }
        FieldDataType::PlankExerciseName => PlankExerciseName::from_i64(value).to_string(),
        FieldDataType::PlyoExerciseName => PlyoExerciseName::from_i64(value).to_string(),
        FieldDataType::PullUpExerciseName => PullUpExerciseName::from_i64(value).to_string(),
        FieldDataType::PushUpExerciseName => PushUpExerciseName::from_i64(value).to_string(),
        FieldDataType::RowExerciseName => RowExerciseName::from_i64(value).to_string(),
        FieldDataType::ShoulderPressExerciseName => {
            ShoulderPressExerciseName::from_i64(value).to_string()
        }
        FieldDataType::ShoulderStabilityExerciseName => {
            ShoulderStabilityExerciseName::from_i64(value).to_string()
        }
        FieldDataType::ShrugExerciseName => ShrugExerciseName::from_i64(value).to_string(),
        FieldDataType::SitUpExerciseName => SitUpExerciseName::from_i64(value).to_string(),
        FieldDataType::SquatExerciseName => SquatExerciseName::from_i64(value).to_string(),
        FieldDataType::TotalBodyExerciseName => TotalBodyExerciseName::from_i64(value).to_string(),
        FieldDataType::TricepsExtensionExerciseName => {
            TricepsExtensionExerciseName::from_i64(value).to_string()
        }
        FieldDataType::WarmUpExerciseName => WarmUpExerciseName::from_i64(value).to_string(),
        FieldDataType::RunExerciseName => RunExerciseName::from_i64(value).to_string(),
        FieldDataType::WaterType => WaterType::from_i64(value).to_string(),
        FieldDataType::TissueModelType => TissueModelType::from_i64(value).to_string(),
        FieldDataType::DiveGasStatus => DiveGasStatus::from_i64(value).to_string(),
        FieldDataType::DiveAlarmType => DiveAlarmType::from_i64(value).to_string(),
        FieldDataType::DiveBacklightMode => DiveBacklightMode::from_i64(value).to_string(),
        FieldDataType::FaveroProduct => FaveroProduct::from_i64(value).to_string(),
        FieldDataType::ClimbProEvent => ClimbProEvent::from_i64(value).to_string(),
        _ => format!("Undefined{}", value),
    }
}
