#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    btcntl: BTCNTL,
    btversion: BTVERSION,
    _reserved2: [u8; 0x04],
    btintcntl: BTINTCNTL,
    btintstat: BTINTSTAT,
    btintrawstat: BTINTRAWSTAT,
    btintack: BTINTACK,
    _reserved6: [u8; 0x01e4],
    blecntl: BLECNTL,
    bleversion: BLEVERSION,
    bleconf: BLECONF,
    bleintcntl: BLEINTCNTL,
    bleintstat: BLEINTSTAT,
    bleintrawstat: BLEINTRAWSTAT,
    bleintack: BLEINTACK,
    blebasetimecnt: BLEBASETIMECNT,
    blefinetimecnt: BLEFINETIMECNT,
    blebdaddrl: BLEBDADDRL,
    blebdaddru: BLEBDADDRU,
    blecurrentrxdescptr: BLECURRENTRXDESCPTR,
    _reserved18: [u8; 0x20],
    blediagcntl: BLEDIAGCNTL,
    blediagstat: BLEDIAGSTAT,
    _reserved20: [u8; 0x08],
    bleerrortypestat: BLEERRORTYPESTAT,
    _reserved21: [u8; 0x0c],
    bleradiocntl0: BLERADIOCNTL0,
    bleradiocntl1: BLERADIOCNTL1,
    _reserved23: [u8; 0x08],
    bleradiopwrupdn: BLERADIOPWRUPDN,
    _reserved24: [u8; 0x0c],
    bleadvchmap: BLEADVCHMAP,
    _reserved25: [u8; 0x0c],
    bleadvtim: BLEADVTIM,
    _reserved26: [u8; 0x0c],
    blewlpubaddrptr: BLEWLPUBADDRPTR,
    blewlprivaddrptr: BLEWLPRIVADDRPTR,
    blewlnbdev: BLEWLNBDEV,
    _reserved29: [u8; 0x04],
    bleaescntl: BLEAESCNTL,
    bleaeskey0: BLEAESKEY0,
    bleaeskey1: BLEAESKEY1,
    bleaeskey2: BLEAESKEY2,
    bleaeskey3: BLEAESKEY3,
    bleaesptr: BLEAESPTR,
    _reserved35: [u8; 0x08],
    blerftestcntl: BLERFTESTCNTL,
    blerftesttxstat: BLERFTESTTXSTAT,
    blerftestrxstat: BLERFTESTRXSTAT,
    _reserved38: [u8; 0x04],
    bletimgencntl: BLETIMGENCNTL,
    _reserved39: [u8; 0x0c],
    blecoexifcntl0: BLECOEXIFCNTL0,
    _reserved40: [u8; 0x1c],
    bleralptr: BLERALPTR,
    bleralnbdev: BLERALNBDEV,
}
impl RegisterBlock {
    #[doc = "0x00 - BR/EDR control register"]
    #[inline(always)]
    pub const fn btcntl(&self) -> &BTCNTL {
        &self.btcntl
    }
    #[doc = "0x04 - BR/EDR peripheral version"]
    #[inline(always)]
    pub const fn btversion(&self) -> &BTVERSION {
        &self.btversion
    }
    #[doc = "0x0c - BR/EDR interrupt control register"]
    #[inline(always)]
    pub const fn btintcntl(&self) -> &BTINTCNTL {
        &self.btintcntl
    }
    #[doc = "0x10 - BR/EDR interrupt status register"]
    #[inline(always)]
    pub const fn btintstat(&self) -> &BTINTSTAT {
        &self.btintstat
    }
    #[doc = "0x14 - BR/EDR interrupt raw status register"]
    #[inline(always)]
    pub const fn btintrawstat(&self) -> &BTINTRAWSTAT {
        &self.btintrawstat
    }
    #[doc = "0x18 - BR/EDR interrupt acknowledgement register"]
    #[inline(always)]
    pub const fn btintack(&self) -> &BTINTACK {
        &self.btintack
    }
    #[doc = "0x200 - BLE control register"]
    #[inline(always)]
    pub const fn blecntl(&self) -> &BLECNTL {
        &self.blecntl
    }
    #[doc = "0x204 - BLE peripheral version"]
    #[inline(always)]
    pub const fn bleversion(&self) -> &BLEVERSION {
        &self.bleversion
    }
    #[doc = "0x208 - BLE configuration register"]
    #[inline(always)]
    pub const fn bleconf(&self) -> &BLECONF {
        &self.bleconf
    }
    #[doc = "0x20c - BLE interrupt controller register"]
    #[inline(always)]
    pub const fn bleintcntl(&self) -> &BLEINTCNTL {
        &self.bleintcntl
    }
    #[doc = "0x210 - BLE interrupt status register"]
    #[inline(always)]
    pub const fn bleintstat(&self) -> &BLEINTSTAT {
        &self.bleintstat
    }
    #[doc = "0x214 - BLE interrupt raw status register"]
    #[inline(always)]
    pub const fn bleintrawstat(&self) -> &BLEINTRAWSTAT {
        &self.bleintrawstat
    }
    #[doc = "0x218 - BLE interrupt acknowledgement register"]
    #[inline(always)]
    pub const fn bleintack(&self) -> &BLEINTACK {
        &self.bleintack
    }
    #[doc = "0x21c - Base time reference counter"]
    #[inline(always)]
    pub const fn blebasetimecnt(&self) -> &BLEBASETIMECNT {
        &self.blebasetimecnt
    }
    #[doc = "0x220 - Fine time reference counter"]
    #[inline(always)]
    pub const fn blefinetimecnt(&self) -> &BLEFINETIMECNT {
        &self.blefinetimecnt
    }
    #[doc = "0x224 - BLE device address LSB register"]
    #[inline(always)]
    pub const fn blebdaddrl(&self) -> &BLEBDADDRL {
        &self.blebdaddrl
    }
    #[doc = "0x228 - BLE device address MSB register"]
    #[inline(always)]
    pub const fn blebdaddru(&self) -> &BLEBDADDRU {
        &self.blebdaddru
    }
    #[doc = "0x22c - BLE RX descriptor pointer for the receive buffer chained list"]
    #[inline(always)]
    pub const fn blecurrentrxdescptr(&self) -> &BLECURRENTRXDESCPTR {
        &self.blecurrentrxdescptr
    }
    #[doc = "0x250 - Diagnostics control register"]
    #[inline(always)]
    pub const fn blediagcntl(&self) -> &BLEDIAGCNTL {
        &self.blediagcntl
    }
    #[doc = "0x254 - Diagnostics status register"]
    #[inline(always)]
    pub const fn blediagstat(&self) -> &BLEDIAGSTAT {
        &self.blediagstat
    }
    #[doc = "0x260 - Error type status register"]
    #[inline(always)]
    pub const fn bleerrortypestat(&self) -> &BLEERRORTYPESTAT {
        &self.bleerrortypestat
    }
    #[doc = "0x270 - Radio interface control register 0"]
    #[inline(always)]
    pub const fn bleradiocntl0(&self) -> &BLERADIOCNTL0 {
        &self.bleradiocntl0
    }
    #[doc = "0x274 - Radio interface control register 1"]
    #[inline(always)]
    pub const fn bleradiocntl1(&self) -> &BLERADIOCNTL1 {
        &self.bleradiocntl1
    }
    #[doc = "0x280 - RX/TX power up/down phase register"]
    #[inline(always)]
    pub const fn bleradiopwrupdn(&self) -> &BLERADIOPWRUPDN {
        &self.bleradiopwrupdn
    }
    #[doc = "0x290 - Advertising Channel Map"]
    #[inline(always)]
    pub const fn bleadvchmap(&self) -> &BLEADVCHMAP {
        &self.bleadvchmap
    }
    #[doc = "0x2a0 - Advertising Packet Interval"]
    #[inline(always)]
    pub const fn bleadvtim(&self) -> &BLEADVTIM {
        &self.bleadvtim
    }
    #[doc = "0x2b0 - Pointer to public devices whitelist"]
    #[inline(always)]
    pub const fn blewlpubaddrptr(&self) -> &BLEWLPUBADDRPTR {
        &self.blewlpubaddrptr
    }
    #[doc = "0x2b4 - Pointer to private devices whitelist"]
    #[inline(always)]
    pub const fn blewlprivaddrptr(&self) -> &BLEWLPRIVADDRPTR {
        &self.blewlprivaddrptr
    }
    #[doc = "0x2b8 - Number of devices in whitelist"]
    #[inline(always)]
    pub const fn blewlnbdev(&self) -> &BLEWLNBDEV {
        &self.blewlnbdev
    }
    #[doc = "0x2c0 - AES encryption control register"]
    #[inline(always)]
    pub const fn bleaescntl(&self) -> &BLEAESCNTL {
        &self.bleaescntl
    }
    #[doc = "0x2c4 - AES key\\[31:0\\]"]
    #[inline(always)]
    pub const fn bleaeskey0(&self) -> &BLEAESKEY0 {
        &self.bleaeskey0
    }
    #[doc = "0x2c8 - AES key\\[63:32\\]"]
    #[inline(always)]
    pub const fn bleaeskey1(&self) -> &BLEAESKEY1 {
        &self.bleaeskey1
    }
    #[doc = "0x2cc - AES key\\[95:64\\]"]
    #[inline(always)]
    pub const fn bleaeskey2(&self) -> &BLEAESKEY2 {
        &self.bleaeskey2
    }
    #[doc = "0x2d0 - AES key\\[127:96\\]"]
    #[inline(always)]
    pub const fn bleaeskey3(&self) -> &BLEAESKEY3 {
        &self.bleaeskey3
    }
    #[doc = "0x2d4 - Pointer to the block to encrypt/decrypt with AES"]
    #[inline(always)]
    pub const fn bleaesptr(&self) -> &BLEAESPTR {
        &self.bleaesptr
    }
    #[doc = "0x2e0 - Test Mode control register"]
    #[inline(always)]
    pub const fn blerftestcntl(&self) -> &BLERFTESTCNTL {
        &self.blerftestcntl
    }
    #[doc = "0x2e4 - Number of valid packets transmited during Test"]
    #[inline(always)]
    pub const fn blerftesttxstat(&self) -> &BLERFTESTTXSTAT {
        &self.blerftesttxstat
    }
    #[doc = "0x2e8 - Number of valid packets received during Test"]
    #[inline(always)]
    pub const fn blerftestrxstat(&self) -> &BLERFTESTRXSTAT {
        &self.blerftestrxstat
    }
    #[doc = "0x2f0 - Pre-Fecth mechanism control"]
    #[inline(always)]
    pub const fn bletimgencntl(&self) -> &BLETIMGENCNTL {
        &self.bletimgencntl
    }
    #[doc = "0x300 - Interface Coexistance Control"]
    #[inline(always)]
    pub const fn blecoexifcntl0(&self) -> &BLECOEXIFCNTL0 {
        &self.blecoexifcntl0
    }
    #[doc = "0x320 - Resolve Address List pointer"]
    #[inline(always)]
    pub const fn bleralptr(&self) -> &BLERALPTR {
        &self.bleralptr
    }
    #[doc = "0x324 - Number of devices in Resolve Address List"]
    #[inline(always)]
    pub const fn bleralnbdev(&self) -> &BLERALNBDEV {
        &self.bleralnbdev
    }
}
#[doc = "BTCNTL (rw) register accessor: BR/EDR control register\n\nYou can [`read`](crate::Reg::read) this register and get [`btcntl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`btcntl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@btcntl`] module"]
pub type BTCNTL = crate::Reg<btcntl::BTCNTL_SPEC>;
#[doc = "BR/EDR control register"]
pub mod btcntl;
#[doc = "BTVERSION (rw) register accessor: BR/EDR peripheral version\n\nYou can [`read`](crate::Reg::read) this register and get [`btversion::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`btversion::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@btversion`] module"]
pub type BTVERSION = crate::Reg<btversion::BTVERSION_SPEC>;
#[doc = "BR/EDR peripheral version"]
pub mod btversion;
#[doc = "BTINTCNTL (rw) register accessor: BR/EDR interrupt control register\n\nYou can [`read`](crate::Reg::read) this register and get [`btintcntl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`btintcntl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@btintcntl`] module"]
pub type BTINTCNTL = crate::Reg<btintcntl::BTINTCNTL_SPEC>;
#[doc = "BR/EDR interrupt control register"]
pub mod btintcntl;
#[doc = "BTINTSTAT (rw) register accessor: BR/EDR interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`btintstat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`btintstat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@btintstat`] module"]
pub type BTINTSTAT = crate::Reg<btintstat::BTINTSTAT_SPEC>;
#[doc = "BR/EDR interrupt status register"]
pub mod btintstat;
#[doc = "BTINTRAWSTAT (rw) register accessor: BR/EDR interrupt raw status register\n\nYou can [`read`](crate::Reg::read) this register and get [`btintrawstat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`btintrawstat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@btintrawstat`] module"]
pub type BTINTRAWSTAT = crate::Reg<btintrawstat::BTINTRAWSTAT_SPEC>;
#[doc = "BR/EDR interrupt raw status register"]
pub mod btintrawstat;
#[doc = "BTINTACK (rw) register accessor: BR/EDR interrupt acknowledgement register\n\nYou can [`read`](crate::Reg::read) this register and get [`btintack::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`btintack::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@btintack`] module"]
pub type BTINTACK = crate::Reg<btintack::BTINTACK_SPEC>;
#[doc = "BR/EDR interrupt acknowledgement register"]
pub mod btintack;
#[doc = "BLECNTL (rw) register accessor: BLE control register\n\nYou can [`read`](crate::Reg::read) this register and get [`blecntl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blecntl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blecntl`] module"]
pub type BLECNTL = crate::Reg<blecntl::BLECNTL_SPEC>;
#[doc = "BLE control register"]
pub mod blecntl;
#[doc = "BLEVERSION (rw) register accessor: BLE peripheral version\n\nYou can [`read`](crate::Reg::read) this register and get [`bleversion::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bleversion::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bleversion`] module"]
pub type BLEVERSION = crate::Reg<bleversion::BLEVERSION_SPEC>;
#[doc = "BLE peripheral version"]
pub mod bleversion;
#[doc = "BLECONF (rw) register accessor: BLE configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`bleconf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bleconf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bleconf`] module"]
pub type BLECONF = crate::Reg<bleconf::BLECONF_SPEC>;
#[doc = "BLE configuration register"]
pub mod bleconf;
#[doc = "BLEINTCNTL (rw) register accessor: BLE interrupt controller register\n\nYou can [`read`](crate::Reg::read) this register and get [`bleintcntl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bleintcntl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bleintcntl`] module"]
pub type BLEINTCNTL = crate::Reg<bleintcntl::BLEINTCNTL_SPEC>;
#[doc = "BLE interrupt controller register"]
pub mod bleintcntl;
#[doc = "BLEINTSTAT (rw) register accessor: BLE interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`bleintstat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bleintstat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bleintstat`] module"]
pub type BLEINTSTAT = crate::Reg<bleintstat::BLEINTSTAT_SPEC>;
#[doc = "BLE interrupt status register"]
pub mod bleintstat;
#[doc = "BLEINTRAWSTAT (rw) register accessor: BLE interrupt raw status register\n\nYou can [`read`](crate::Reg::read) this register and get [`bleintrawstat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bleintrawstat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bleintrawstat`] module"]
pub type BLEINTRAWSTAT = crate::Reg<bleintrawstat::BLEINTRAWSTAT_SPEC>;
#[doc = "BLE interrupt raw status register"]
pub mod bleintrawstat;
#[doc = "BLEINTACK (rw) register accessor: BLE interrupt acknowledgement register\n\nYou can [`read`](crate::Reg::read) this register and get [`bleintack::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bleintack::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bleintack`] module"]
pub type BLEINTACK = crate::Reg<bleintack::BLEINTACK_SPEC>;
#[doc = "BLE interrupt acknowledgement register"]
pub mod bleintack;
#[doc = "BLEBASETIMECNT (rw) register accessor: Base time reference counter\n\nYou can [`read`](crate::Reg::read) this register and get [`blebasetimecnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blebasetimecnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blebasetimecnt`] module"]
pub type BLEBASETIMECNT = crate::Reg<blebasetimecnt::BLEBASETIMECNT_SPEC>;
#[doc = "Base time reference counter"]
pub mod blebasetimecnt;
#[doc = "BLEFINETIMECNT (rw) register accessor: Fine time reference counter\n\nYou can [`read`](crate::Reg::read) this register and get [`blefinetimecnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blefinetimecnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blefinetimecnt`] module"]
pub type BLEFINETIMECNT = crate::Reg<blefinetimecnt::BLEFINETIMECNT_SPEC>;
#[doc = "Fine time reference counter"]
pub mod blefinetimecnt;
#[doc = "BLEBDADDRL (rw) register accessor: BLE device address LSB register\n\nYou can [`read`](crate::Reg::read) this register and get [`blebdaddrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blebdaddrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blebdaddrl`] module"]
pub type BLEBDADDRL = crate::Reg<blebdaddrl::BLEBDADDRL_SPEC>;
#[doc = "BLE device address LSB register"]
pub mod blebdaddrl;
#[doc = "BLEBDADDRU (rw) register accessor: BLE device address MSB register\n\nYou can [`read`](crate::Reg::read) this register and get [`blebdaddru::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blebdaddru::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blebdaddru`] module"]
pub type BLEBDADDRU = crate::Reg<blebdaddru::BLEBDADDRU_SPEC>;
#[doc = "BLE device address MSB register"]
pub mod blebdaddru;
#[doc = "BLECURRENTRXDESCPTR (rw) register accessor: BLE RX descriptor pointer for the receive buffer chained list\n\nYou can [`read`](crate::Reg::read) this register and get [`blecurrentrxdescptr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blecurrentrxdescptr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blecurrentrxdescptr`] module"]
pub type BLECURRENTRXDESCPTR = crate::Reg<blecurrentrxdescptr::BLECURRENTRXDESCPTR_SPEC>;
#[doc = "BLE RX descriptor pointer for the receive buffer chained list"]
pub mod blecurrentrxdescptr;
#[doc = "BLEDIAGCNTL (rw) register accessor: Diagnostics control register\n\nYou can [`read`](crate::Reg::read) this register and get [`blediagcntl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blediagcntl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blediagcntl`] module"]
pub type BLEDIAGCNTL = crate::Reg<blediagcntl::BLEDIAGCNTL_SPEC>;
#[doc = "Diagnostics control register"]
pub mod blediagcntl;
#[doc = "BLEDIAGSTAT (rw) register accessor: Diagnostics status register\n\nYou can [`read`](crate::Reg::read) this register and get [`blediagstat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blediagstat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blediagstat`] module"]
pub type BLEDIAGSTAT = crate::Reg<blediagstat::BLEDIAGSTAT_SPEC>;
#[doc = "Diagnostics status register"]
pub mod blediagstat;
#[doc = "BLEERRORTYPESTAT (rw) register accessor: Error type status register\n\nYou can [`read`](crate::Reg::read) this register and get [`bleerrortypestat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bleerrortypestat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bleerrortypestat`] module"]
pub type BLEERRORTYPESTAT = crate::Reg<bleerrortypestat::BLEERRORTYPESTAT_SPEC>;
#[doc = "Error type status register"]
pub mod bleerrortypestat;
#[doc = "BLERADIOCNTL0 (rw) register accessor: Radio interface control register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`bleradiocntl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bleradiocntl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bleradiocntl0`] module"]
pub type BLERADIOCNTL0 = crate::Reg<bleradiocntl0::BLERADIOCNTL0_SPEC>;
#[doc = "Radio interface control register 0"]
pub mod bleradiocntl0;
#[doc = "BLERADIOCNTL1 (rw) register accessor: Radio interface control register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`bleradiocntl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bleradiocntl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bleradiocntl1`] module"]
pub type BLERADIOCNTL1 = crate::Reg<bleradiocntl1::BLERADIOCNTL1_SPEC>;
#[doc = "Radio interface control register 1"]
pub mod bleradiocntl1;
#[doc = "BLERADIOPWRUPDN (rw) register accessor: RX/TX power up/down phase register\n\nYou can [`read`](crate::Reg::read) this register and get [`bleradiopwrupdn::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bleradiopwrupdn::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bleradiopwrupdn`] module"]
pub type BLERADIOPWRUPDN = crate::Reg<bleradiopwrupdn::BLERADIOPWRUPDN_SPEC>;
#[doc = "RX/TX power up/down phase register"]
pub mod bleradiopwrupdn;
#[doc = "BLEADVCHMAP (rw) register accessor: Advertising Channel Map\n\nYou can [`read`](crate::Reg::read) this register and get [`bleadvchmap::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bleadvchmap::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bleadvchmap`] module"]
pub type BLEADVCHMAP = crate::Reg<bleadvchmap::BLEADVCHMAP_SPEC>;
#[doc = "Advertising Channel Map"]
pub mod bleadvchmap;
#[doc = "BLEADVTIM (rw) register accessor: Advertising Packet Interval\n\nYou can [`read`](crate::Reg::read) this register and get [`bleadvtim::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bleadvtim::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bleadvtim`] module"]
pub type BLEADVTIM = crate::Reg<bleadvtim::BLEADVTIM_SPEC>;
#[doc = "Advertising Packet Interval"]
pub mod bleadvtim;
#[doc = "BLEWLPUBADDRPTR (rw) register accessor: Pointer to public devices whitelist\n\nYou can [`read`](crate::Reg::read) this register and get [`blewlpubaddrptr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blewlpubaddrptr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blewlpubaddrptr`] module"]
pub type BLEWLPUBADDRPTR = crate::Reg<blewlpubaddrptr::BLEWLPUBADDRPTR_SPEC>;
#[doc = "Pointer to public devices whitelist"]
pub mod blewlpubaddrptr;
#[doc = "BLEWLPRIVADDRPTR (rw) register accessor: Pointer to private devices whitelist\n\nYou can [`read`](crate::Reg::read) this register and get [`blewlprivaddrptr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blewlprivaddrptr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blewlprivaddrptr`] module"]
pub type BLEWLPRIVADDRPTR = crate::Reg<blewlprivaddrptr::BLEWLPRIVADDRPTR_SPEC>;
#[doc = "Pointer to private devices whitelist"]
pub mod blewlprivaddrptr;
#[doc = "BLEWLNBDEV (rw) register accessor: Number of devices in whitelist\n\nYou can [`read`](crate::Reg::read) this register and get [`blewlnbdev::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blewlnbdev::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blewlnbdev`] module"]
pub type BLEWLNBDEV = crate::Reg<blewlnbdev::BLEWLNBDEV_SPEC>;
#[doc = "Number of devices in whitelist"]
pub mod blewlnbdev;
#[doc = "BLEAESCNTL (rw) register accessor: AES encryption control register\n\nYou can [`read`](crate::Reg::read) this register and get [`bleaescntl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bleaescntl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bleaescntl`] module"]
pub type BLEAESCNTL = crate::Reg<bleaescntl::BLEAESCNTL_SPEC>;
#[doc = "AES encryption control register"]
pub mod bleaescntl;
#[doc = "BLEAESKEY0 (rw) register accessor: AES key\\[31:0\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`bleaeskey0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bleaeskey0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bleaeskey0`] module"]
pub type BLEAESKEY0 = crate::Reg<bleaeskey0::BLEAESKEY0_SPEC>;
#[doc = "AES key\\[31:0\\]"]
pub mod bleaeskey0;
#[doc = "BLEAESKEY1 (rw) register accessor: AES key\\[63:32\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`bleaeskey1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bleaeskey1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bleaeskey1`] module"]
pub type BLEAESKEY1 = crate::Reg<bleaeskey1::BLEAESKEY1_SPEC>;
#[doc = "AES key\\[63:32\\]"]
pub mod bleaeskey1;
#[doc = "BLEAESKEY2 (rw) register accessor: AES key\\[95:64\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`bleaeskey2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bleaeskey2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bleaeskey2`] module"]
pub type BLEAESKEY2 = crate::Reg<bleaeskey2::BLEAESKEY2_SPEC>;
#[doc = "AES key\\[95:64\\]"]
pub mod bleaeskey2;
#[doc = "BLEAESKEY3 (rw) register accessor: AES key\\[127:96\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`bleaeskey3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bleaeskey3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bleaeskey3`] module"]
pub type BLEAESKEY3 = crate::Reg<bleaeskey3::BLEAESKEY3_SPEC>;
#[doc = "AES key\\[127:96\\]"]
pub mod bleaeskey3;
#[doc = "BLEAESPTR (rw) register accessor: Pointer to the block to encrypt/decrypt with AES\n\nYou can [`read`](crate::Reg::read) this register and get [`bleaesptr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bleaesptr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bleaesptr`] module"]
pub type BLEAESPTR = crate::Reg<bleaesptr::BLEAESPTR_SPEC>;
#[doc = "Pointer to the block to encrypt/decrypt with AES"]
pub mod bleaesptr;
#[doc = "BLERFTESTCNTL (rw) register accessor: Test Mode control register\n\nYou can [`read`](crate::Reg::read) this register and get [`blerftestcntl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blerftestcntl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blerftestcntl`] module"]
pub type BLERFTESTCNTL = crate::Reg<blerftestcntl::BLERFTESTCNTL_SPEC>;
#[doc = "Test Mode control register"]
pub mod blerftestcntl;
#[doc = "BLERFTESTTXSTAT (rw) register accessor: Number of valid packets transmited during Test\n\nYou can [`read`](crate::Reg::read) this register and get [`blerftesttxstat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blerftesttxstat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blerftesttxstat`] module"]
pub type BLERFTESTTXSTAT = crate::Reg<blerftesttxstat::BLERFTESTTXSTAT_SPEC>;
#[doc = "Number of valid packets transmited during Test"]
pub mod blerftesttxstat;
#[doc = "BLERFTESTRXSTAT (rw) register accessor: Number of valid packets received during Test\n\nYou can [`read`](crate::Reg::read) this register and get [`blerftestrxstat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blerftestrxstat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blerftestrxstat`] module"]
pub type BLERFTESTRXSTAT = crate::Reg<blerftestrxstat::BLERFTESTRXSTAT_SPEC>;
#[doc = "Number of valid packets received during Test"]
pub mod blerftestrxstat;
#[doc = "BLETIMGENCNTL (rw) register accessor: Pre-Fecth mechanism control\n\nYou can [`read`](crate::Reg::read) this register and get [`bletimgencntl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bletimgencntl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bletimgencntl`] module"]
pub type BLETIMGENCNTL = crate::Reg<bletimgencntl::BLETIMGENCNTL_SPEC>;
#[doc = "Pre-Fecth mechanism control"]
pub mod bletimgencntl;
#[doc = "BLECOEXIFCNTL0 (rw) register accessor: Interface Coexistance Control\n\nYou can [`read`](crate::Reg::read) this register and get [`blecoexifcntl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blecoexifcntl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blecoexifcntl0`] module"]
pub type BLECOEXIFCNTL0 = crate::Reg<blecoexifcntl0::BLECOEXIFCNTL0_SPEC>;
#[doc = "Interface Coexistance Control"]
pub mod blecoexifcntl0;
#[doc = "BLERALPTR (rw) register accessor: Resolve Address List pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`bleralptr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bleralptr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bleralptr`] module"]
pub type BLERALPTR = crate::Reg<bleralptr::BLERALPTR_SPEC>;
#[doc = "Resolve Address List pointer"]
pub mod bleralptr;
#[doc = "BLERALNBDEV (rw) register accessor: Number of devices in Resolve Address List\n\nYou can [`read`](crate::Reg::read) this register and get [`bleralnbdev::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bleralnbdev::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bleralnbdev`] module"]
pub type BLERALNBDEV = crate::Reg<bleralnbdev::BLERALNBDEV_SPEC>;
#[doc = "Number of devices in Resolve Address List"]
pub mod bleralnbdev;
