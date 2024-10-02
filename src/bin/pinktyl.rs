#![no_main]
#![no_std]
#![feature(type_alias_impl_trait)]

use pinktyl as _; // global logger + panicking-behavior + memory layout

#[rtic::app(
    device = stm32f4xx_hal::pac,
    peripherals = true,
    dispatchers = [SPI1, SPI2]
)]
mod app {
    use frunk::HList;
    use hal::{
        gpio::{EPin, Input, Output},
        otg_fs::{UsbBus, USB},
        prelude::*,
    };
    use pinktyl::{key::StateChange::*, matrix::Matrix};
    use rtic_monotonics::systick::prelude::*;
    use stm32f4xx_hal as hal;
    use usb_device::{
        class_prelude::*,
        device::{StringDescriptors, UsbDeviceBuilder, UsbVidPid},
        prelude::*,
    };
    use usbd_human_interface_device::{
        device::keyboard::{NKROBootKeyboard, NKROBootKeyboardConfig},
        prelude::*,
        usb_class::UsbHidClass,
    };

    systick_monotonic!(Mono, 1000);

    const OUT_PINS: usize = 6;
    const IN_PINS: usize = 6;
    const DEBOUNCE_CYCLE_COUNT: u8 = 5;
    // static mut EP_MEMORY: [u32; 1024] = [0; 1024];
    #[shared]
    struct Shared {
        keyboard: UsbHidClass<
            'static,
            hal::otg_fs::UsbBus<USB>,
            HList!(NKROBootKeyboard<'static, hal::otg_fs::UsbBus<USB>>),
        >,
        usb_dev: UsbDevice<'static, hal::otg_fs::UsbBus<USB>>,
        matrix: Matrix,
    }

    #[local]
    struct Local {
        outputs: [EPin<Output>; OUT_PINS],
        inputs: [EPin<Input>; IN_PINS],
    }

    #[init(local = [
        ep_memory: [u32; 1024] = [0; 1024],
        usb_bus: Option<UsbBusAllocator<UsbBus<USB>>> = None
    ])]
    fn init(cx: init::Context) -> (Shared, Local) {
        defmt::info!("init");
        Mono::start(cx.core.SYST, 80_000_000);

        let dp = cx.device;
        let rcc = dp.RCC.constrain();

        let clocks = rcc
            .cfgr
            .use_hse(25.MHz())
            .sysclk(80.MHz())
            .use_hse(25.MHz())
            .freeze();

        defmt::info!("Clocks setup complete");

        let gpioa = dp.GPIOA.split();
        let gpiob = dp.GPIOB.split();

        // let delay = dp.TIM1.delay_ms(&clocks);

        let outputs = [
            gpioa.pa5.into_push_pull_output().erase(),
            gpioa.pa4.into_push_pull_output().erase(),
            gpioa.pa3.into_push_pull_output().erase(),
            gpioa.pa2.into_push_pull_output().erase(),
            gpioa.pa1.into_push_pull_output().erase(),
            gpioa.pa0.into_push_pull_output().erase(),
        ];
        let inputs = [
            gpiob.pb14.into_pull_down_input().erase(),
            gpiob.pb15.into_pull_down_input().erase(),
            gpioa.pa15.into_pull_down_input().erase(),
            gpiob.pb3.into_pull_down_input().erase(),
            gpiob.pb8.into_pull_down_input().erase(),
            gpiob.pb9.into_pull_down_input().erase(),
        ];

        let usb = USB {
            usb_global: dp.OTG_FS_GLOBAL,
            usb_device: dp.OTG_FS_DEVICE,
            usb_pwrclk: dp.OTG_FS_PWRCLK,
            pin_dm: gpioa.pa11.into(),
            pin_dp: gpioa.pa12.into(),
            hclk: clocks.hclk(),
        };

        // let ep_memory: &'static mut [u32] = unsafe { &mut EP_MEMORY };

        *cx.local.usb_bus = Some(hal::otg_fs::UsbBus::new(usb, cx.local.ep_memory));
        let keyboard = UsbHidClassBuilder::new()
            .add_device(NKROBootKeyboardConfig::default())
            .build(cx.local.usb_bus.as_ref().unwrap());

        let usb_dev = UsbDeviceBuilder::new(
            cx.local.usb_bus.as_ref().unwrap(),
            UsbVidPid(0x1209, 0x0001),
        )
        .strings(&[StringDescriptors::default()
            .manufacturer("pinklifeart")
            .product("The Mighty Pinktyl")
            .serial_number("0000")])
        .unwrap()
        .build();

        kb_tick::spawn().ok();
        // kb_poll::spawn().ok();
        scan::spawn().ok();
        kb_report::spawn().ok();
        defmt::info!("Tasks spawned, init complete");

        (
            Shared {
                keyboard,
                usb_dev,
                matrix: Matrix::new(),
            },
            Local { outputs, inputs },
        )
    }

    // Optional idle, can be removed if not needed.
    #[idle]
    fn idle(_: idle::Context) -> ! {
        defmt::info!("idle");

        loop {
            continue;
        }
    }

    #[task(binds = OTG_FS, priority = 2, shared = [keyboard, usb_dev])]
    fn kb_poll(cx: kb_poll::Context) {
        (cx.shared.keyboard, cx.shared.usb_dev).lock(|keyboard, usb_dev| {
            if usb_dev.poll(&mut [keyboard]) {
                match keyboard.device().read_report() {
                    Err(UsbError::WouldBlock) => {
                        // defmt::error!("Usb poll -- Would block")
                    }
                    Err(_e) => {
                        // defmt::error!("Failed to read kb output: {:?}", e)
                    }
                    Ok(_leds) => {
                        // defmt::info!("report received")
                    }
                }
            }
        })
    }

    #[task(priority = 3, shared = [keyboard])]
    async fn kb_tick(mut cx: kb_tick::Context) {
        loop {
            cx.shared.keyboard.lock(|keyboard| match keyboard.tick() {
                Err(UsbHidError::WouldBlock) => {
                    // defmt::error!("Usb tick - Would block")
                }
                Err(_e) => {
                    // defmt::error!("Failed to process keyboard tick: {:?}", e)
                }
                Ok(_) => {}
            });
            Mono::delay(1.millis()).await;
        }
    }

    #[task(priority = 2, shared = [keyboard, matrix])]
    async fn kb_report(mut cx: kb_report::Context) {
        loop {
            cx.shared.keyboard.lock(|keyboard| {
                match keyboard
                    .device()
                    .write_report(cx.shared.matrix.lock(|matrix| matrix.report_active()))
                {
                    Err(UsbHidError::WouldBlock) => {}
                    Err(UsbHidError::Duplicate) => {}
                    Err(_e) => {}
                    Ok(_) => {}
                }
            });
            Mono::delay(10.millis()).await;
        }
    }

    #[task(priority = 2, local = [outputs, inputs], shared = [matrix])]
    async fn scan(mut cx: scan::Context) {
        loop {
            (0..OUT_PINS).for_each(|col| {
                cx.local.outputs[col].set_high();
                (0..IN_PINS).for_each(|row| {
                    if let Some(m) = cx.shared.matrix.lock(|matrix| {
                        matrix.layout[row][col].sync_state(
                            cx.local.inputs[row].is_high(),
                            DEBOUNCE_CYCLE_COUNT,
                            matrix.active_layer,
                        )
                    }) {
                        match m {
                            LayerUp => cx.shared.matrix.lock(|matrix| matrix.increment_layer()),
                            LayerDown => cx.shared.matrix.lock(|matrix| matrix.decrement_layer()),
                            SetActive => {
                                defmt::info!("{:?}", m)
                            }
                            SetInactive => {
                                defmt::info!("{:?}", m)
                            }
                            _ => {}
                        }
                        defmt::info!("Changed state to: {:?}", m);
                        // TODO: Add USART message logic based on message variants
                    }
                });
                cx.local.outputs[col].set_low();
            });
            Mono::delay(10.millis()).await;
        }
    }
}
