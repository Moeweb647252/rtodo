use actix_web::web;
use serde::{Deserialize, Serialize};
use std::{
  collections::HashMap,
  path::PathBuf,
  sync::{Arc, RwLock},
};

pub type RS = web::Data<RtodoState>;
pub type ReqData = web::Json<serde_json::Value>;
pub type ReqDataT<T> = web::Json<ReqCommonData<T>>;

#[derive(Serialize, Deserialize, Clone)]
pub struct ReqCommonData<T> {
  pub token: String,
  pub data: Option<T>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ResCommonData<T> {
  pub code: i32,
  pub data: T,
}

pub struct RtodoState {
  pub rtodo: Arc<RwLock<Rtodo>>,
}

pub struct Rtodo {
  pub config: Config,
  pub works: Vec<RwLock<Work>>,
  pub cur_entry_id: u32,
  pub conf_path: String,
  pub executor_pid: i32,
  pub checker_pid: i32,
  pub server_pid: i32,
  pub daemon_status: RtodoDaemonStatus,
  pub rcli: reqwest::blocking::Client,
}

pub enum RtodoDaemonStatus {
  Running,
  Stopped,
}

#[derive(Serialize, Deserialize)]
pub struct Config {
  #[serde(default)]
  pub entries: Vec<Entry>,
  pub address: String,
  pub token: String,
}

#[derive(Serialize, Deserialize, Clone, Default, Copy)]
pub enum Status {
  Error,
  Running,
  Paused,
  #[default]
  Pending,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum TimeZone {
  Utc,
  Local,
  Offset(i8),
}

#[derive(Serialize, Deserialize, Clone)]
pub struct DateTime {
  pub year: i32,
  pub month: u32,
  pub day: u32,
  pub hour: u32,
  pub min: u32,
  pub sec: u32,
  pub timestamp: i64,
  pub time_zone: TimeZone,
}

impl Default for DateTime {
  fn default() -> Self {
    Self {
      sec: 0,
      min: 0,
      hour: 0,
      day: 0,
      month: 0,
      year: 0,
      timestamp: 0,
      time_zone: TimeZone::Local,
    }
  }
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Duration {
  pub year: i32,
  pub month: u32,
  pub day: u32,
  pub hour: u32,
  pub min: u32,
  pub sec: u32,
  pub total_sec: u64,
}

#[derive(Serialize, Clone)]
pub struct Process {
  pub pid: i32,
  pub output_tmp_file: Option<PathBuf>,
}

#[derive(Serialize, Clone)]
pub struct Work {
  pub status: Status,
  pub entry: Entry,
  pub trigger_state: TriggerState,
  pub running_processes: Vec<Process>,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum EntryIdentifier {
  Id(u32),
  Name(String),
}

#[derive(Serialize, Deserialize, Clone)]
pub enum Logger {
  File(String),
  Default,
  Off,
}

impl Default for Logger {
  fn default() -> Self {
    Self::Default
  }
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub enum Timer {
  Repeat(Duration),
  Once(DateTime),
  ManyTimes(Duration, u32),
  #[default]
  Never,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct UnixUser {
  pub uid: u32,
  pub gid: u32,
  pub username: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct WindowsUser {
  pub username: String,
  pub group_windows: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum SystemUser {
  Unix(UnixUser),
  Windows(WindowsUser),
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Execute {
  pub env: Option<HashMap<String, String>>,
  pub working_dir: Option<String>,
  pub executable: PathBuf,
  pub user: Option<SystemUser>,
  pub args: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub enum Action {
  Exec(Execute),
  #[default]
  None,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub enum DoIfRunning {
  #[default]
  StartNew,
  Stop,
  Restart,
  Continue,
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Entry {
  pub id: u32,
  pub name: String,
  pub action: Action,
  pub logger: Logger,
  pub trigger: Trigger,
  pub status: Status,
  pub do_if_running: DoIfRunning,
  pub enabled: bool,
}

pub enum OperationType {
  Add,
  Delete,
  Start,
  Pause,
  StartDaemon,
  StopDaemon,
  List,
  Detail,
  Help,
  Version,
}

pub enum Operation {
  Add(Entry),
  Delete(EntryIdentifier),
  Start(EntryIdentifier),
  Pause(EntryIdentifier),
  StartDaemon(),
  StopDaemon(),
  List(),
  Detail(EntryIdentifier),
  Help(Option<OperationType>),
  Version,
}

pub trait CommandHelp {
  fn cmd_help() -> String;
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub enum Trigger {
  Timer(Timer),
  #[default]
  None,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct TriggerState {
  pub exec_time: Option<DateTime>,
  pub exec_times: u32,
}
