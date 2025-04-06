use eframe::egui;
use eframe::App;
use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink, Source};
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use std::time::{Duration, Instant};

struct Mp3PlayerApp {
    mp3_files: Vec<PathBuf>,
    current_sink: Option<Sink>,
    stream_handle: OutputStreamHandle,
    _stream: OutputStream,
    is_paused: bool,
    current_index: Option<usize>,
    current_duration: Option<Duration>,
    start_time: Option<Instant>,
}

impl Mp3PlayerApp {
    fn new(music_dir: &str) -> Self {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let mp3_files = WalkDir::new(music_dir)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().extension().map_or(false, |ext| ext == "mp3"))
            .map(|e| e.path().to_path_buf())
            .collect();

        Mp3PlayerApp {
            mp3_files,
            current_sink: None,
            stream_handle,
            _stream,
            is_paused: false,
            current_index: None,
            current_duration: None,
            start_time: None,
        }
    }

    fn play_file(&mut self, path: &Path, index: usize) {
        if let Some(sink) = &self.current_sink {
            sink.stop();
        }

        let file = File::open(path).unwrap();
        let source = Decoder::new(BufReader::new(file)).unwrap();

        let duration = source.total_duration();

        let sink = Sink::try_new(&self.stream_handle).unwrap();
        sink.append(source);
        self.current_sink = Some(sink);
        self.is_paused = false;
        self.current_index = Some(index);
        self.start_time = Some(Instant::now());
        self.current_duration = duration;
    }

    fn format_duration(d: Duration) -> String {
        let secs = d.as_secs();
        format!("{:02}:{:02}", secs / 60, secs % 60)
    }
}

impl App for Mp3PlayerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Rust MP3 Player");

            let files = self.mp3_files.clone();
            for (i, file) in files.iter().enumerate() {
                let file_name = file.file_name().unwrap().to_string_lossy();
                if ui.button(file_name).clicked() {
                    self.play_file(file, i);
                }
            }

            if let Some(sink) = &self.current_sink {
                ui.separator();
                //interface controls
                let label = if self.is_paused { "▶️ Resume" } else { "⏸ Pause" };
                if ui.button(label).clicked() {
                    if self.is_paused {
                        sink.play();
                    } else {
                        sink.pause();
                    }
                    self.is_paused = !self.is_paused;
                }

               if let Some(i) = self.current_index {
                ui.horizontal(|ui| {
                if i > 0 && ui.button("⏮ Prev").clicked() {
                    let prev = self.mp3_files[i - 1].clone(); 
                    self.play_file(&prev, i - 1);
                }
                    if i + 1 < self.mp3_files.len() && ui.button("⏭ Next").clicked() {
                        let next = self.mp3_files[i + 1].clone();
                        self.play_file(&next, i + 1);
                        }
                    });
                }

                // WIP interactive progress bar 
                if let (Some(total), Some(start)) = (self.current_duration, self.start_time) {
                let elapsed = if self.is_paused {
                total.min(total)
                } else {
                    start.elapsed().min(total)
                };
                let progress = elapsed.as_secs_f32() / total.as_secs_f32();
                ui.add(egui::ProgressBar::new(progress).text(format!(
                    "{} / {}",
                    Self::format_duration(elapsed),
                    Self::format_duration(total)
                )));
                }
            }
        });
    ctx.request_repaint(); 
    }
}

fn main() -> Result<(), eframe::Error> {
    let music_dir = "./music";

    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Rust MP3 Player Project",
        options,
        Box::new(|_cc| Box::new(Mp3PlayerApp::new(music_dir))),
    )
}
