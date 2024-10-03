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
        serial::{self, Config, Serial},
    };
    use pinktyl::{
        key::{
            Message,
            StateChange::{self, *},
        },
        matrix::Matrix,
    };
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
    const DEBOUNCE_CYCLE_COUNT: u8 = 3;

    // Scan offset should be equal to the amount of column pins in the left half
    #[cfg(feature = "left")]
    const SCAN_OFFSET: usize = 0;
    #[cfg(feature = "right")]
    const SCAN_OFFSET: usize = 6;

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
        tx: serial::Tx<hal::pac::USART1>,
        rx: serial::Rx<hal::pac::USART1>,
        buffer: [u8; 4],
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

        let usart_pins = (gpiob.pb6, gpiob.pb7);
        let mut serial = Serial::new(
            dp.USART1,
            usart_pins,
            Config::default().baudrate(36_400.bps()).wordlength_8(),
            &clocks,
        )
        .unwrap()
        .with_u8_data();
        serial.listen(serial::Event::RxNotEmpty);
        let (tx, rx) = serial.split();
        let buffer = [0_u8; 4];

        // Mutability is necessary when building for right half to reverse an array of pins
        #[allow(unused_mut)]
        let mut outputs = [
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

        #[cfg(feature = "right")]
        outputs.reverse();

        let usb = USB {
            usb_global: dp.OTG_FS_GLOBAL,
            usb_device: dp.OTG_FS_DEVICE,
            usb_pwrclk: dp.OTG_FS_PWRCLK,
            pin_dm: gpioa.pa11.into(),
            pin_dp: gpioa.pa12.into(),
            hclk: clocks.hclk(),
        };

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
        scan::spawn().ok();
        kb_report::spawn().ok();
        defmt::info!("Tasks spawned, init complete");

        (
            Shared {
                keyboard,
                usb_dev,
                matrix: Matrix::new(),
            },
            Local {
                outputs,
                inputs,
                tx,
                rx,
                buffer,
            },
        )
    }

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

    #[task(priority = 2, local = [outputs, inputs, tx], shared = [matrix,usb_dev])]
    async fn scan(mut cx: scan::Context) {
        loop {
            (0..OUT_PINS).for_each(|col| {
                cx.local.outputs[col].set_high();
                (0..IN_PINS).for_each(|row| {
                    #[cfg(any(feature = "right", feature = "left"))]
                    let col = col + SCAN_OFFSET;
                    if let Some(m) = cx.shared.matrix.lock(|matrix| {
                        matrix.layout[row][col].sync_state(
                            cx.local.inputs[row].is_high(),
                            DEBOUNCE_CYCLE_COUNT,
                            matrix.active_layer,
                        )
                    }) {
                        if cx
                            .shared
                            .usb_dev
                            .lock(|usb_dev| usb_dev.state() != UsbDeviceState::Configured)
                        {
                            cx.local.tx.bwrite_all(&serialize(m, row, col)).unwrap();
                            cx.local.tx.bflush().unwrap();
                        } else {
                            match m {
                                LayerUp => cx.shared.matrix.lock(|matrix| matrix.increment_layer()),
                                LayerDown => {
                                    cx.shared.matrix.lock(|matrix| matrix.decrement_layer())
                                }
                                _ => {}
                            }
                        }
                    }
                });
                cx.local.outputs[col].set_low();
            });
            Mono::delay(10.millis()).await;
        }
    }

    #[task(binds = USART1,priority = 3, local = [rx,buffer])]
    fn usart_rx(cx: usart_rx::Context) {
        if let Ok(byte) = cx.local.rx.read() {
            cx.local.buffer.rotate_left(1);
            cx.local.buffer[3] = byte;

            if cx.local.buffer[3] == b'\n' {
                if let Some(message) = try_deserialize(cx.local.buffer) {
                    handle_message::spawn(message).unwrap();
                }
            }
        }
    }

    fn serialize(state: StateChange, row: usize, col: usize) -> [u8; 4] {
        [state as u8, row as u8, col as u8, b'\n']
    }

    fn try_deserialize(message: &[u8; 4]) -> Option<Message> {
        if let Ok(sc) = StateChange::try_from(message[0]) {
            Some(Message::new(sc, message[1] as usize, message[2] as usize))
        } else {
            None
        }
    }

    #[task(priority = 3, shared = [matrix])]
    async fn handle_message(mut cx: handle_message::Context, message: Message) {
        cx.shared.matrix.lock(|matrix| match message.state_change {
            SetActive => matrix.layout[message.row][message.col].sync_state(
                true,
                DEBOUNCE_CYCLE_COUNT,
                matrix.active_layer,
            ),
            SetInactive => matrix.layout[message.row][message.col].sync_state(
                false,
                DEBOUNCE_CYCLE_COUNT,
                matrix.active_layer,
            ),
            DebounceTick => {
                matrix.layout[message.row][message.col].tick_debounce();
                Some(DebounceTick)
            }
            // TODO: Add handling for fn key to be able to connect usb on both sides
            _ => None,
        });
    }
}
