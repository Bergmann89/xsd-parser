#![recursion_limit = "256"]
#![allow(missing_docs, clippy::missing_errors_doc)]

use std::env::var;
use std::error::Error as StdError;
use std::fs::read;
use std::hint::black_box;
use std::io::BufReader;
use std::iter::Sum;
use std::path::PathBuf;
use std::str::FromStr;
use std::time::{Duration, Instant};
use std::{fs::read_to_string, path::Path};

use anyhow::{Context, Error};
use bytesize::ByteSize;
use clap::Parser;
use comfy_table::{presets::ASCII_MARKDOWN, Cell, CellAlignment, Table};
use serde::{Deserialize, Serialize};
use sysinfo::{CpuRefreshKind, RefreshKind, System};
use xsd_parser_types::quick_xml::{
    DeserializeSync, ErrorReader, SerializeSync, SliceReader, Writer, XmlReader,
};

pub mod schemas;

#[derive(Debug, Parser)]
pub struct Args {
    #[arg(short, long, default_value = "10")]
    pub warmup: usize,

    #[arg(short, long, default_value = "100")]
    pub runs: usize,
}

#[derive(Debug)]
pub struct Runner {
    args: Args,
    table: Table,
    cargo_dir: PathBuf,
}

#[derive(Debug)]
pub struct Executor<'a> {
    runner: &'a mut Runner,
    times: Vec<Duration>,

    schema: &'a str,
    document: &'a str,
    engine: &'a str,
    op: &'a str,
}

impl Runner {
    pub fn new(args: Args) -> Result<Self, Error> {
        let mut table = Table::new();
        table.load_preset(ASCII_MARKDOWN).set_header([
            "Schema", "Document", "Engine", "Op", "Min", "Avg", "Mdn", "Max", "Stack",
        ]);

        let cargo_dir = var("CARGO_MANIFEST_DIR")
            .context("Missing environment variable `CARGO_MANIFEST_DIR`")?;
        let cargo_dir = Path::new(&cargo_dir)
            .canonicalize()
            .context("Unable to canonicalizing path `CARGO_MANIFEST_DIR`")?;

        Ok(Self {
            args,
            table,
            cargo_dir,
        })
    }

    pub fn summary(&self) {
        let Self {
            table,
            args: Args { warmup, runs },
            ..
        } = self;

        let sys = System::new_with_specifics(
            RefreshKind::default().with_cpu(CpuRefreshKind::everything()),
        );

        println!();
        println!("--== System ==--");
        println!();

        if let Some(value) = System::long_os_version() {
            println!("OS:                  {value}");
        }

        println!("Kernel:              {}", System::kernel_long_version());

        if let Some(cpu) = sys.cpus().first() {
            println!("CPU Arch:            {}", System::cpu_arch());
            println!("CPU Brand:           {}", cpu.brand());
            println!("CPU Vendor:          {}", cpu.vendor_id());
            println!("CPU Cores (logical): {}", sys.cpus().len());

            let max_freq = read_to_string("/sys/devices/system/cpu/cpu0/cpufreq/cpuinfo_max_freq")
                .map_err(Error::from)
                .and_then(|s| f64::from_str(s.trim()).map_err(Error::from));
            if let Ok(max_freq) = max_freq {
                let max_freq = max_freq / 1000.0;
                println!("CPU max Frequency:   {max_freq:.02} MHz");
            }
        }

        println!();
        println!("--== Config ==--");
        println!();
        println!("Warmup: {warmup}");
        println!("Runs:   {runs}");

        println!();
        println!("--== Results ==--");
        println!();
        println!("{table}");
    }

    pub fn resolve_path<P>(&self, path: P) -> PathBuf
    where
        P: AsRef<Path>,
    {
        self.cargo_dir.join(path)
    }

    pub fn load_bytes<P>(&self, path: P) -> Result<Vec<u8>, Error>
    where
        P: AsRef<Path>,
    {
        let path = self.cargo_dir.join(path);
        let path = path.canonicalize().with_context(|| {
            format!("Unable to canonicalize source path (`{}`)", path.display())
        })?;

        let bytes = read(&path)
            .with_context(|| format!("Unable to read source document (`{}`)", path.display()))?;

        Ok(bytes)
    }

    pub fn quick_xml_deserialize_file<T, P>(&mut self, path: P) -> Result<T, Error>
    where
        for<'x> T: DeserializeSync<'x, ErrorReader<SliceReader<'x>>>,
        for<'x> <T as DeserializeSync<'x, ErrorReader<SliceReader<'x>>>>::Error:
            StdError + Send + Sync + 'static,
        P: AsRef<Path>,
    {
        let bytes = self.load_bytes(path)?;
        let mut reader = SliceReader::from_bytes(&bytes).with_error_info();
        let value = T::deserialize(&mut reader).context("Unable to deserialize data")?;

        Ok(value)
    }

    pub fn serde_quick_xml_deserialize_file<T, P>(&mut self, path: P) -> Result<T, Error>
    where
        T: Serialize + for<'de> Deserialize<'de> + Send + Sync + 'static,
        P: AsRef<Path>,
    {
        use quick_xml::de::from_reader;

        let bytes = self.load_bytes(path)?;
        let reader = BufReader::new(&bytes[..]);
        let value = from_reader::<_, T>(reader).context("Unable to deserialize data")?;

        Ok(value)
    }

    pub fn serde_xml_rs_deserialize_file<T, P>(&mut self, path: P) -> Result<T, Error>
    where
        T: Serialize + for<'de> Deserialize<'de> + Send + Sync + 'static,
        P: AsRef<Path>,
    {
        use serde_xml_rs::from_reader;

        let bytes = self.load_bytes(path)?;
        let reader = BufReader::new(&bytes[..]);
        let value = from_reader::<T, _>(reader).context("Unable to deserialize data")?;

        Ok(value)
    }

    pub fn serde_xml_rs_v7_deserialize_file<T, P>(&mut self, path: P) -> Result<T, Error>
    where
        T: Serialize + for<'de> Deserialize<'de> + Send + Sync + 'static,
        P: AsRef<Path>,
    {
        use serde_xml_rs_v7::from_reader;

        let bytes = self.load_bytes(path)?;
        let reader = BufReader::new(&bytes[..]);
        let value = from_reader::<_, T>(reader).context("Unable to deserialize data")?;

        Ok(value)
    }

    pub fn add_test_quick_xml_serialize_file<T, P>(
        &mut self,
        schema: &str,
        document: &str,
        path: P,
    ) -> Result<(), Error>
    where
        for<'x> T: SerializeSync
            + DeserializeSync<'x, ErrorReader<SliceReader<'x>>>
            + Send
            + Sync
            + 'static,
        for<'x> <T as DeserializeSync<'x, ErrorReader<SliceReader<'x>>>>::Error:
            StdError + Send + Sync + 'static,
        <T as SerializeSync>::Error: StdError + Send + Sync + 'static,
        P: AsRef<Path>,
    {
        let value = self.quick_xml_deserialize_file(path)?;

        self.add_test_quick_xml_serialize_value::<T>(schema, document, &value)
    }

    pub fn add_test_quick_xml_serialize_value<T>(
        &mut self,
        schema: &str,
        document: &str,
        value: &T,
    ) -> Result<(), Error>
    where
        for<'x> T: SerializeSync
            + DeserializeSync<'x, ErrorReader<SliceReader<'x>>>
            + Send
            + Sync
            + 'static,
        for<'x> <T as DeserializeSync<'x, ErrorReader<SliceReader<'x>>>>::Error:
            StdError + Send + Sync + 'static,
        <T as SerializeSync>::Error: StdError + Send + Sync + 'static,
    {
        self.executor(schema, document, "quick_xml", "serialize")
            .run(move || {
                let mut buffer = Vec::new();
                let mut writer = Writer::new_with_indent(&mut buffer, b'\t', 1);

                black_box(black_box(&value).serialize("document", &mut writer))?;
                black_box(buffer);

                Ok(())
            })
    }

    pub fn add_test_quick_xml_deserialize_file<T, P>(
        &mut self,
        schema: &str,
        document: &str,
        path: P,
    ) -> Result<(), Error>
    where
        for<'x> T: DeserializeSync<'x, ErrorReader<SliceReader<'x>>>,
        for<'x> <T as DeserializeSync<'x, ErrorReader<SliceReader<'x>>>>::Error:
            StdError + Send + Sync + 'static,
        P: AsRef<Path>,
    {
        let bytes = self.load_bytes(path)?;

        self.add_test_quick_xml_deserialize_bytes::<T>(schema, document, &bytes)
    }

    pub fn add_test_quick_xml_deserialize_bytes<T>(
        &mut self,
        schema: &str,
        document: &str,
        bytes: &[u8],
    ) -> Result<(), Error>
    where
        for<'x> T: DeserializeSync<'x, ErrorReader<SliceReader<'x>>>,
        for<'x> <T as DeserializeSync<'x, ErrorReader<SliceReader<'x>>>>::Error:
            StdError + Send + Sync + 'static,
    {
        self.executor(schema, document, "quick_xml", "deserialize")
            .run(move || {
                let mut reader = SliceReader::from_bytes(bytes).with_error_info();

                black_box(T::deserialize(black_box(&mut reader)))
                    .context("Unable to deserialize data")?;

                Ok(())
            })
    }

    pub fn add_test_serde_quick_xml_serialize_file<T, P>(
        &mut self,
        schema: &str,
        document: &str,
        path: P,
    ) -> Result<(), Error>
    where
        T: Serialize + for<'de> Deserialize<'de> + Send + Sync + 'static,
        P: AsRef<Path>,
    {
        let value = self.serde_quick_xml_deserialize_file(path)?;

        self.add_test_serde_quick_xml_serialize_value::<T>(schema, document, &value)
    }

    pub fn add_test_serde_quick_xml_serialize_value<T>(
        &mut self,
        schema: &str,
        document: &str,
        value: &T,
    ) -> Result<(), Error>
    where
        T: Serialize + for<'de> Deserialize<'de> + Send + Sync + 'static,
    {
        use quick_xml::se::to_writer;

        self.executor(schema, document, "serde (quick_xml)", "serialize")
            .run(move || {
                let mut buffer = String::new();

                black_box(to_writer(&mut buffer, black_box(value)))?;
                black_box(buffer);

                Ok(())
            })
    }

    pub fn add_test_serde_quick_xml_deserialize_file<T, P>(
        &mut self,
        schema: &str,
        document: &str,
        path: P,
    ) -> Result<(), Error>
    where
        T: for<'de> Deserialize<'de> + Send + 'static,
        P: AsRef<Path>,
    {
        let bytes = self.load_bytes(path)?;

        self.add_test_serde_quick_xml_deserialize_bytes::<T>(schema, document, &bytes)
    }

    pub fn add_test_serde_quick_xml_deserialize_bytes<T>(
        &mut self,
        schema: &str,
        document: &str,
        bytes: &[u8],
    ) -> Result<(), Error>
    where
        T: for<'de> Deserialize<'de> + Send + 'static,
    {
        use quick_xml::de::from_reader;

        self.executor(schema, document, "serde (quick_xml)", "deserialize")
            .run(move || {
                let reader = BufReader::new(bytes);
                black_box(from_reader::<_, T>(reader)).context("Unable to deserialize data")?;

                Ok(())
            })
    }

    pub fn add_test_serde_xml_rs_serialize_file<T, P>(
        &mut self,
        schema: &str,
        document: &str,
        path: P,
    ) -> Result<(), Error>
    where
        T: Serialize + for<'de> Deserialize<'de> + Send + Sync + 'static,
        P: AsRef<Path>,
    {
        let value = self.serde_xml_rs_deserialize_file(path)?;

        self.add_test_serde_xml_rs_serialize_value::<T>(schema, document, &value)
    }

    pub fn add_test_serde_xml_rs_serialize_value<T>(
        &mut self,
        schema: &str,
        document: &str,
        value: &T,
    ) -> Result<(), Error>
    where
        T: Serialize + for<'de> Deserialize<'de> + Send + Sync + 'static,
    {
        use serde_xml_rs::to_writer;

        self.executor(schema, document, "serde (xml_rs)", "serialize")
            .run(move || {
                let mut buffer = Vec::new();

                black_box(to_writer(&mut buffer, black_box(value)))?;
                black_box(buffer);

                Ok(())
            })
    }

    pub fn add_test_serde_xml_rs_deserialize_file<T, P>(
        &mut self,
        schema: &str,
        document: &str,
        path: P,
    ) -> Result<(), Error>
    where
        T: for<'de> Deserialize<'de> + Send + 'static,
        P: AsRef<Path>,
    {
        let bytes = self.load_bytes(path)?;

        self.add_test_serde_xml_rs_deserialize_bytes::<T>(schema, document, &bytes)
    }

    pub fn add_test_serde_xml_rs_deserialize_bytes<T>(
        &mut self,
        schema: &str,
        document: &str,
        bytes: &[u8],
    ) -> Result<(), Error>
    where
        T: for<'de> Deserialize<'de> + Send + 'static,
    {
        use serde_xml_rs::from_reader;

        self.executor(schema, document, "serde (xml_rs)", "deserialize")
            .run(move || {
                let reader = BufReader::new(bytes);
                black_box(from_reader::<T, _>(reader)).context("Unable to deserialize data")?;

                Ok(())
            })
    }

    pub fn add_test_serde_xml_rs_v7_serialize_file<T, P>(
        &mut self,
        schema: &str,
        document: &str,
        path: P,
    ) -> Result<(), Error>
    where
        T: Serialize + for<'de> Deserialize<'de> + Send + Sync + 'static,
        P: AsRef<Path>,
    {
        let value = self.serde_xml_rs_v7_deserialize_file(path)?;

        self.add_test_serde_xml_rs_v7_serialize_value::<T>(schema, document, &value)
    }

    pub fn add_test_serde_xml_rs_v7_serialize_value<T>(
        &mut self,
        schema: &str,
        document: &str,
        value: &T,
    ) -> Result<(), Error>
    where
        T: Serialize + for<'de> Deserialize<'de> + Send + Sync + 'static,
    {
        use serde_xml_rs_v7::to_writer;

        self.executor(schema, document, "serde (xml_rs v0.7)", "serialize")
            .run(move || {
                let mut buffer = Vec::new();

                black_box(to_writer(&mut buffer, black_box(value)))?;
                black_box(buffer);

                Ok(())
            })
    }

    pub fn add_test_serde_xml_rs_v7_deserialize_file<T, P>(
        &mut self,
        schema: &str,
        document: &str,
        path: P,
    ) -> Result<(), Error>
    where
        T: for<'de> Deserialize<'de> + Send + 'static,
        P: AsRef<Path>,
    {
        let bytes = self.load_bytes(path)?;

        self.add_test_serde_xml_rs_v7_deserialize_bytes::<T>(schema, document, &bytes)
    }

    pub fn add_test_serde_xml_rs_v7_deserialize_bytes<T>(
        &mut self,
        schema: &str,
        document: &str,
        bytes: &[u8],
    ) -> Result<(), Error>
    where
        T: for<'de> Deserialize<'de> + Send + 'static,
    {
        use serde_xml_rs_v7::from_reader;

        self.executor(schema, document, "serde (xml_rs v0.7)", "deserialize")
            .run(move || {
                let reader = BufReader::new(bytes);
                black_box(from_reader::<_, T>(reader)).context("Unable to deserialize data")?;

                Ok(())
            })
    }

    fn executor<'a>(
        &'a mut self,
        schema: &'a str,
        document: &'a str,
        engine: &'a str,
        op: &'a str,
    ) -> Executor<'a> {
        Executor {
            runner: self,
            times: Vec::new(),
            schema,
            document,
            engine,
            op,
        }
    }

    fn add_results(
        &mut self,
        schema: &str,
        document: &str,
        engine: &str,
        op: &str,
        mut times: Vec<Duration>,
        stack: Option<usize>,
    ) {
        fn time_str(d: Duration) -> String {
            const SEC: Duration = Duration::from_secs(1);

            if d > SEC {
                return format!("{:.02} sec", d.as_secs_f64());
            }

            let d = 1000 * d;
            if d > SEC {
                return format!("{:.02} ms ", d.as_secs_f64());
            }

            let d = 1000 * d;
            format!("{:.02} µs ", d.as_secs_f64())
        }

        times.sort();

        let min = time_str(*times.iter().min().unwrap());
        let avg =
            time_str(Duration::sum(times.iter().copied()) / u32::try_from(times.len()).unwrap());
        let mdn = time_str(times[times.len() / 2]);
        let max = time_str(*times.iter().max().unwrap());

        let schema = Cell::new(schema.to_string());
        let document = Cell::new(document.to_string());
        let engine = Cell::new(engine.to_string());
        let op = Cell::new(op.to_string());

        let min = Cell::new(min).set_alignment(CellAlignment::Right);
        let avg = Cell::new(avg).set_alignment(CellAlignment::Right);
        let mdn = Cell::new(mdn).set_alignment(CellAlignment::Right);
        let max = Cell::new(max).set_alignment(CellAlignment::Right);
        let stack = stack.map_or_else(|| String::from("-"), |b| ByteSize::b(b as u64).to_string());
        let stack = Cell::new(stack).set_alignment(CellAlignment::Right);

        self.table
            .add_row([schema, document, engine, op, min, avg, mdn, max, stack]);
    }
}

impl Executor<'_> {
    fn run<F>(self, mut f: F) -> Result<(), Error>
    where
        F: FnMut() -> Result<(), Error> + Send,
    {
        let Self {
            runner,
            mut times,
            schema,
            document,
            engine,
            op,
        } = self;

        // Setup

        println!("Running case: schema={schema}, document={document}, engine={engine}, op={op}");

        let total_beg = Instant::now();

        let Args { warmup, runs } = runner.args;
        let n = warmup + runs;

        // Measure execution time

        for i in 0..n {
            let beg = Instant::now();

            black_box(f())?;

            let end = Instant::now();

            if i >= warmup {
                times.push(end - beg);
            }
        }

        // Measure stack size

        #[cfg(not(target_os = "linux"))]
        let stack = None;

        #[cfg(target_os = "linux")]
        let stack = stack_usage::exec(&mut f)?;

        // Cleanup

        let total_end = Instant::now();
        let elapsed = (total_end - total_beg).as_secs_f64();
        println!("    Done ({elapsed:.02} secs)");

        runner.add_results(schema, document, engine, op, times, stack);

        Ok(())
    }
}

#[cfg(target_os = "linux")]
mod stack_usage {
    use std::{
        mem::zeroed,
        ptr::{null_mut, write_bytes},
        thread::{scope, Builder},
    };

    use anyhow::Error;

    pub(super) fn exec<F>(f: &mut F) -> Result<Option<usize>, Error>
    where
        F: FnMut() -> Result<(), Error> + Send,
    {
        scope(|s| {
            let handle = Builder::new().stack_size(STACK_SIZE).spawn_scoped(
                s,
                || -> Result<Option<usize>, Error> {
                    let Some((stack_base_addr, _size, guard)) = get_stack_info() else {
                        return Ok(None);
                    };

                    let page = page_size();
                    let margin = page;
                    let stack_pointer = current_approximately_stack_pointer();

                    // Start painting above the guard area (if any).
                    let paint_low = unsafe { stack_base_addr.add(guard) };
                    let paint_high = unsafe { stack_pointer.sub(margin) };

                    if paint_high <= paint_low
                        || (paint_low <= stack_pointer && stack_pointer <= paint_high)
                    {
                        return Ok(None);
                    }

                    #[allow(clippy::cast_sign_loss)]
                    let paint_len = unsafe { paint_high.offset_from(paint_low) } as usize;
                    unsafe { write_bytes(paint_low, PATTERN, paint_len) };

                    f()?;

                    let mut i = 0usize;
                    while i < paint_len && unsafe { *paint_low.add(i) == PATTERN } {
                        i += 1;
                    }

                    Ok(Some(paint_len - i))
                },
            )?;

            handle.join().map_err(|_| Error::msg("Join error"))?
        })
    }

    fn get_stack_info() -> Option<(*mut u8, usize, usize)> {
        unsafe {
            let mut attr: libc::pthread_attr_t = zeroed();

            if libc::pthread_getattr_np(libc::pthread_self(), &mut attr) != 0 {
                return None;
            }

            let mut stack_addr: *mut libc::c_void = null_mut();
            let mut stack_size: usize = 0;
            let mut guard_size: usize = 0;

            let rc1 = libc::pthread_attr_getstack(&attr, &mut stack_addr, &mut stack_size);
            let rc2 = libc::pthread_attr_getguardsize(&attr, &mut guard_size);

            libc::pthread_attr_destroy(&mut attr);

            if rc1 != 0 || rc2 != 0 || stack_addr.is_null() || stack_size == 0 {
                None
            } else {
                Some((stack_addr.cast::<u8>(), stack_size, guard_size))
            }
        }
    }

    #[inline]
    fn page_size() -> usize {
        // POSIX page size
        unsafe {
            usize::try_from(libc::sysconf(libc::_SC_PAGESIZE))
                .unwrap()
                .max(4096)
        }
    }

    #[inline]
    fn current_approximately_stack_pointer() -> *mut u8 {
        // Address of a local is “near” the current stack pointer
        let mut x = 0u8;

        &raw mut x
    }

    const PATTERN: u8 = 0xCD;
    const STACK_SIZE: usize = 64 * 1024 * 1024;
}
