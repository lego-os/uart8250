use bitflags::bitflags;
use tom_device::reg_map;
reg_map!(
    RBR 0,
    DLL 0,
    THR 0,
    DLH 0x4,
    IER 0x4,
    FCR 0x8,
    IIR 0x8,
    LCR 0xC,
    MCR 0x10,
    LSR 0x14,
    MSR 0x18,
    SCR 0x1C,
    LPDLL 0x20,
    LPDLH 0x24,
    SRBR0 0x30,
    SRBR1 0x34,
    SRBR2 0x38,
    SRBR3 0x3C,
    SRBR4 0x40,
    SRBR5 0x44,
    SRBR6 0x48,
    SRBR7 0x4C,
    SRBR8 0x50,
    SRBR9 0x54,
    SRBR10 0x58,
    SRBR11 0x5C,
    SRBR12 0x60,
    SRBR13 0x64,
    SRBR14 0x68,
    SRBR15 0x6C,
    STHR0 0x30,
    STHR1 0x34,
    STHR2 0x38,
    STHR3 0x3C,
    STHR4 0x40,
    STHR5 0x44,
    STHR6 0x48,
    STHR7 0x4C,
    STHR8 0x50,
    STHR9 0x54,
    STHR10 0x58,
    STHR11 0x5C,
    STHR12 0x60,
    STHR13 0x64,
    STHR14 0x68,
    STHR15 0x6C,
    FAR 0x70,
    TFR 0x74,
    RFW 0x78,
    USR 0x7C,
    TFL 0x80,
    RFL 0x84,
    SRR 0x88,
    SRTS 0x8C,
    SBCR 0x90,
    SDMAM 0x94,
    SFE 0x98,
    SRT 0x9C,
    STET 0xA0,
    HTX 0xA4,
    DMASA 0xA8,
    TCR 0xAC,
    DE_EN 0xB0,
    RE_EN 0xB4,
    DET 0xB8,
    TAT 0xBC,
    DLF 0xC0,
    RAR 0xC4,
    TAR 0xC8,
    LCR_EXT 0xCC,
    UART_PORT_LEVEL 0xD0,
    REG_TIMEOUT_RST 0xD4,
    CPR 0xF4,
    UCV 0xF8,
    CTR 0xFC
);

bitflags! {
    #[derive(Debug,Clone, Copy)]
    pub struct Lcr:u8{
        const word_len =0x11;
        const stop_bits = 0x1 << 2;
        const parity = 0x1 << 3;
        const parity_select = 0x1 << 4;
        const stick_parity = 0x1 << 5;
        const brk = 0x1 << 6;
        const div_latch = 0x1 << 7;
    }
    #[derive(Debug,Clone, Copy)]
    pub struct Lsr:u8{
        const data_ready =0x1;
        const overrun = 0x1 << 1;
        const parity_error = 0x1 << 2;
        const frame_error = 0x1 << 3;
        const brk_interrupt = 0x1 << 4;
        const thre = 0x1 << 5;
        const temt = 0x1 << 6;
        const rcvr_fifo_error = 0x1 <<7;
    }

    #[derive(Debug,Clone, Copy)]
    pub struct Ier:u8{
        const rdai =0x1;
        const threi = 0x1 << 1;
        const rlsi = 0x1 << 2;
        const msi = 0x1 << 3;
        const alc = 0x1 << 4;
        const ptime = 0x1 << 7;
    }

    #[derive(Debug,Clone, Copy)]
    pub struct Iir:u8{
        const id = 0xF;
        const fifo_enabled = 0b11 << 6;
    }

    #[derive(Debug,Clone, Copy)]
    pub struct Fcr:u8{
        const enable =0x1;
        const rc_reset = 0x1 << 1;
        const ts_reset = 0x1 << 2;
        const dma_mode = 0x1 << 3;
        const ts_trigger = 0b11 << 4;
        const rc_trigger = 0b11 << 6;
    }
}
