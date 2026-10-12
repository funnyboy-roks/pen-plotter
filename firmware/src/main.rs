#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]
#![deny(clippy::large_stack_frames)]

use core::sync::atomic::{AtomicBool, Ordering};

use defmt::info;
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use esp_hal::gpio::{Input, InputConfig, Level, OutputConfig, Pull};
use esp_hal::timer::timg::TimerGroup;
use esp_hal::{clock::CpuClock, gpio::Output};
use {esp_backtrace as _, esp_println as _};

extern crate alloc;

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

#[allow(
    clippy::large_stack_frames,
    reason = "it's not unusual to allocate larger buffers etc. in main"
)]
#[esp_rtos::main]
async fn main(spawner: Spawner) -> ! {
    // generator version: 1.3.0
    // generator parameters: --chip esp32 -o unstable-hal -o alloc -o wifi -o embassy -o defmt -o esp-backtrace -o ci -o 1.93-x86_64-unknown-linux-gnu

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    esp_alloc::heap_allocator!(#[esp_hal::ram(reclaimed)] size: 98768);

    let timg0 = TimerGroup::new(peripherals.TIMG0);
    let sw_interrupt =
        esp_hal::interrupt::software::SoftwareInterruptControl::new(peripherals.SW_INTERRUPT);
    esp_rtos::start(timg0.timer0, sw_interrupt.software_interrupt0);

    info!("Embassy initialized!");

    let (mut _wifi_controller, _interfaces) =
        esp_radio::wifi::new(peripherals.WIFI, Default::default())
            .expect("Failed to initialize Wi-Fi controller");

    let mut x = Axis {
        pos: Input::new(
            peripherals.GPIO2,
            InputConfig::default().with_pull(Pull::Down),
        ),
        neg: Input::new(
            peripherals.GPIO4,
            InputConfig::default().with_pull(Pull::Down),
        ),
        dir: Output::new(peripherals.GPIO15, Level::Low, OutputConfig::default()),
        step: Output::new(peripherals.GPIO0, Level::Low, OutputConfig::default()),
    };
    let mut y = Axis {
        pos: Input::new(
            peripherals.GPIO32,
            InputConfig::default().with_pull(Pull::Down),
        ),
        neg: Input::new(
            peripherals.GPIO33,
            InputConfig::default().with_pull(Pull::Down),
        ),
        dir: Output::new(peripherals.GPIO14, Level::Low, OutputConfig::default()),
        step: Output::new(peripherals.GPIO27, Level::Low, OutputConfig::default()),
    };

    let servo = Output::new(peripherals.GPIO12, Level::Low, OutputConfig::default());
    let servo_toggle = Input::new(
        peripherals.GPIO13,
        InputConfig::default().with_pull(Pull::Down),
    );
    spawner.spawn(handle_pen(servo).unwrap());
    spawner.spawn(listen_pen_toggle(servo_toggle).unwrap());

    loop {
        if x.step(x.delta()) | y.step(y.delta()) {
            Timer::after_millis(2).await;
            x.unstep();
            y.unstep();
            Timer::after_millis(2).await;
        } else {
            Timer::after(Duration::from_millis(10)).await;
        }
    }

    // for inspiration have a look at the examples at https://github.com/esp-rs/esp-hal/tree/esp-hal-v1.1.0/examples
}

pub static PEN_POS: AtomicBool = AtomicBool::new(false);

#[embassy_executor::task]
async fn listen_pen_toggle(mut input: Input<'static>) {
    loop {
        input.wait_for_rising_edge().await;
        PEN_DOWN.fetch_not(Ordering::SeqCst);
        Timer::after_millis(10).await;
    }
}

#[embassy_executor::task]
async fn handle_pen(mut output: Output<'static>) {
    loop {
        output.set_high();
        if PEN_DOWN.load(Ordering::SeqCst) {
            Timer::after_micros(1000).await;
        } else {
            Timer::after_micros(2000).await;
        }
        output.set_low();
        Timer::after_millis(20).await;
    }
}

struct Axis<'a> {
    pos: Input<'a>,
    neg: Input<'a>,
    dir: Output<'a>,
    step: Output<'a>,
}

#[derive(Debug)]
enum Delta {
    Pos,
    Zero,
    Neg,
}

impl Axis<'_> {
    fn delta(&self) -> Delta {
        if self.pos.is_high() && self.neg.is_low() {
            Delta::Pos
        } else if self.pos.is_low() && self.neg.is_high() {
            Delta::Neg
        } else {
            Delta::Zero
        }
    }

    /// Set the step to high, returns if a step is needed
    fn step(&mut self, delta: Delta) -> bool {
        match delta {
            Delta::Pos => self.dir.set_high(),
            Delta::Zero => return false,
            Delta::Neg => self.dir.set_low(),
        }

        self.step.set_high();
        true
    }

    /// Set step to low
    fn unstep(&mut self) {
        self.step.set_low();
    }
}
