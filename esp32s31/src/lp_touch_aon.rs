#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    approach_work_meas_num: APPROACH_WORK_MEAS_NUM,
    scan_ctrl1: SCAN_CTRL1,
    scan_ctrl2: SCAN_CTRL2,
    work: WORK,
    work_meas_num: WORK_MEAS_NUM,
    filter1: FILTER1,
    filter2: FILTER2,
    filter3: FILTER3,
    slp0: SLP0,
    slp1: SLP1,
    clr: CLR,
    approach: APPROACH,
    freq0_scan_para: FREQ0_SCAN_PARA,
    freq1_scan_para: FREQ1_SCAN_PARA,
    freq2_scan_para: FREQ2_SCAN_PARA,
    ana_para: ANA_PARA,
    mux0: MUX0,
    mux1: MUX1,
    pad0_th0: PAD0_TH0,
    pad0_th1: PAD0_TH1,
    pad0_th2: PAD0_TH2,
    pad1_th0: PAD1_TH0,
    pad1_th1: PAD1_TH1,
    pad1_th2: PAD1_TH2,
    pad2_th0: PAD2_TH0,
    pad2_th1: PAD2_TH1,
    pad2_th2: PAD2_TH2,
    pad3_th0: PAD3_TH0,
    pad3_th1: PAD3_TH1,
    pad3_th2: PAD3_TH2,
    pad4_th0: PAD4_TH0,
    pad4_th1: PAD4_TH1,
    pad4_th2: PAD4_TH2,
    pad5_th0: PAD5_TH0,
    pad5_th1: PAD5_TH1,
    pad5_th2: PAD5_TH2,
    pad6_th0: PAD6_TH0,
    pad6_th1: PAD6_TH1,
    pad6_th2: PAD6_TH2,
    pad7_th0: PAD7_TH0,
    pad7_th1: PAD7_TH1,
    pad7_th2: PAD7_TH2,
    pad8_th0: PAD8_TH0,
    pad8_th1: PAD8_TH1,
    pad8_th2: PAD8_TH2,
    pad9_th0: PAD9_TH0,
    pad9_th1: PAD9_TH1,
    pad9_th2: PAD9_TH2,
    pad10_th0: PAD10_TH0,
    pad10_th1: PAD10_TH1,
    pad10_th2: PAD10_TH2,
    pad11_th0: PAD11_TH0,
    pad11_th1: PAD11_TH1,
    pad11_th2: PAD11_TH2,
    pad12_th0: PAD12_TH0,
    pad12_th1: PAD12_TH1,
    pad12_th2: PAD12_TH2,
    pad13_th0: PAD13_TH0,
    pad13_th1: PAD13_TH1,
    pad13_th2: PAD13_TH2,
    pad14_th0: PAD14_TH0,
    pad14_th1: PAD14_TH1,
    pad14_th2: PAD14_TH2,
    date: DATE,
}
impl RegisterBlock {
    #[doc = "0x00 - need_des"]
    #[inline(always)]
    pub const fn approach_work_meas_num(&self) -> &APPROACH_WORK_MEAS_NUM {
        &self.approach_work_meas_num
    }
    #[doc = "0x04 - need_des"]
    #[inline(always)]
    pub const fn scan_ctrl1(&self) -> &SCAN_CTRL1 {
        &self.scan_ctrl1
    }
    #[doc = "0x08 - need_des"]
    #[inline(always)]
    pub const fn scan_ctrl2(&self) -> &SCAN_CTRL2 {
        &self.scan_ctrl2
    }
    #[doc = "0x0c - need_des"]
    #[inline(always)]
    pub const fn work(&self) -> &WORK {
        &self.work
    }
    #[doc = "0x10 - need_des"]
    #[inline(always)]
    pub const fn work_meas_num(&self) -> &WORK_MEAS_NUM {
        &self.work_meas_num
    }
    #[doc = "0x14 - need_des"]
    #[inline(always)]
    pub const fn filter1(&self) -> &FILTER1 {
        &self.filter1
    }
    #[doc = "0x18 - need_des"]
    #[inline(always)]
    pub const fn filter2(&self) -> &FILTER2 {
        &self.filter2
    }
    #[doc = "0x1c - need_des"]
    #[inline(always)]
    pub const fn filter3(&self) -> &FILTER3 {
        &self.filter3
    }
    #[doc = "0x20 - need_des"]
    #[inline(always)]
    pub const fn slp0(&self) -> &SLP0 {
        &self.slp0
    }
    #[doc = "0x24 - need_des"]
    #[inline(always)]
    pub const fn slp1(&self) -> &SLP1 {
        &self.slp1
    }
    #[doc = "0x28 - need_des"]
    #[inline(always)]
    pub const fn clr(&self) -> &CLR {
        &self.clr
    }
    #[doc = "0x2c - need_des"]
    #[inline(always)]
    pub const fn approach(&self) -> &APPROACH {
        &self.approach
    }
    #[doc = "0x30 - need_des"]
    #[inline(always)]
    pub const fn freq0_scan_para(&self) -> &FREQ0_SCAN_PARA {
        &self.freq0_scan_para
    }
    #[doc = "0x34 - need_des"]
    #[inline(always)]
    pub const fn freq1_scan_para(&self) -> &FREQ1_SCAN_PARA {
        &self.freq1_scan_para
    }
    #[doc = "0x38 - need_des"]
    #[inline(always)]
    pub const fn freq2_scan_para(&self) -> &FREQ2_SCAN_PARA {
        &self.freq2_scan_para
    }
    #[doc = "0x3c - need_des"]
    #[inline(always)]
    pub const fn ana_para(&self) -> &ANA_PARA {
        &self.ana_para
    }
    #[doc = "0x40 - need_des"]
    #[inline(always)]
    pub const fn mux0(&self) -> &MUX0 {
        &self.mux0
    }
    #[doc = "0x44 - need_des"]
    #[inline(always)]
    pub const fn mux1(&self) -> &MUX1 {
        &self.mux1
    }
    #[doc = "0x48 - need_des"]
    #[inline(always)]
    pub const fn pad0_th0(&self) -> &PAD0_TH0 {
        &self.pad0_th0
    }
    #[doc = "0x4c - need_des"]
    #[inline(always)]
    pub const fn pad0_th1(&self) -> &PAD0_TH1 {
        &self.pad0_th1
    }
    #[doc = "0x50 - need_des"]
    #[inline(always)]
    pub const fn pad0_th2(&self) -> &PAD0_TH2 {
        &self.pad0_th2
    }
    #[doc = "0x54 - need_des"]
    #[inline(always)]
    pub const fn pad1_th0(&self) -> &PAD1_TH0 {
        &self.pad1_th0
    }
    #[doc = "0x58 - need_des"]
    #[inline(always)]
    pub const fn pad1_th1(&self) -> &PAD1_TH1 {
        &self.pad1_th1
    }
    #[doc = "0x5c - need_des"]
    #[inline(always)]
    pub const fn pad1_th2(&self) -> &PAD1_TH2 {
        &self.pad1_th2
    }
    #[doc = "0x60 - need_des"]
    #[inline(always)]
    pub const fn pad2_th0(&self) -> &PAD2_TH0 {
        &self.pad2_th0
    }
    #[doc = "0x64 - need_des"]
    #[inline(always)]
    pub const fn pad2_th1(&self) -> &PAD2_TH1 {
        &self.pad2_th1
    }
    #[doc = "0x68 - need_des"]
    #[inline(always)]
    pub const fn pad2_th2(&self) -> &PAD2_TH2 {
        &self.pad2_th2
    }
    #[doc = "0x6c - need_des"]
    #[inline(always)]
    pub const fn pad3_th0(&self) -> &PAD3_TH0 {
        &self.pad3_th0
    }
    #[doc = "0x70 - need_des"]
    #[inline(always)]
    pub const fn pad3_th1(&self) -> &PAD3_TH1 {
        &self.pad3_th1
    }
    #[doc = "0x74 - need_des"]
    #[inline(always)]
    pub const fn pad3_th2(&self) -> &PAD3_TH2 {
        &self.pad3_th2
    }
    #[doc = "0x78 - need_des"]
    #[inline(always)]
    pub const fn pad4_th0(&self) -> &PAD4_TH0 {
        &self.pad4_th0
    }
    #[doc = "0x7c - need_des"]
    #[inline(always)]
    pub const fn pad4_th1(&self) -> &PAD4_TH1 {
        &self.pad4_th1
    }
    #[doc = "0x80 - need_des"]
    #[inline(always)]
    pub const fn pad4_th2(&self) -> &PAD4_TH2 {
        &self.pad4_th2
    }
    #[doc = "0x84 - need_des"]
    #[inline(always)]
    pub const fn pad5_th0(&self) -> &PAD5_TH0 {
        &self.pad5_th0
    }
    #[doc = "0x88 - need_des"]
    #[inline(always)]
    pub const fn pad5_th1(&self) -> &PAD5_TH1 {
        &self.pad5_th1
    }
    #[doc = "0x8c - need_des"]
    #[inline(always)]
    pub const fn pad5_th2(&self) -> &PAD5_TH2 {
        &self.pad5_th2
    }
    #[doc = "0x90 - need_des"]
    #[inline(always)]
    pub const fn pad6_th0(&self) -> &PAD6_TH0 {
        &self.pad6_th0
    }
    #[doc = "0x94 - need_des"]
    #[inline(always)]
    pub const fn pad6_th1(&self) -> &PAD6_TH1 {
        &self.pad6_th1
    }
    #[doc = "0x98 - need_des"]
    #[inline(always)]
    pub const fn pad6_th2(&self) -> &PAD6_TH2 {
        &self.pad6_th2
    }
    #[doc = "0x9c - need_des"]
    #[inline(always)]
    pub const fn pad7_th0(&self) -> &PAD7_TH0 {
        &self.pad7_th0
    }
    #[doc = "0xa0 - need_des"]
    #[inline(always)]
    pub const fn pad7_th1(&self) -> &PAD7_TH1 {
        &self.pad7_th1
    }
    #[doc = "0xa4 - need_des"]
    #[inline(always)]
    pub const fn pad7_th2(&self) -> &PAD7_TH2 {
        &self.pad7_th2
    }
    #[doc = "0xa8 - need_des"]
    #[inline(always)]
    pub const fn pad8_th0(&self) -> &PAD8_TH0 {
        &self.pad8_th0
    }
    #[doc = "0xac - need_des"]
    #[inline(always)]
    pub const fn pad8_th1(&self) -> &PAD8_TH1 {
        &self.pad8_th1
    }
    #[doc = "0xb0 - need_des"]
    #[inline(always)]
    pub const fn pad8_th2(&self) -> &PAD8_TH2 {
        &self.pad8_th2
    }
    #[doc = "0xb4 - need_des"]
    #[inline(always)]
    pub const fn pad9_th0(&self) -> &PAD9_TH0 {
        &self.pad9_th0
    }
    #[doc = "0xb8 - need_des"]
    #[inline(always)]
    pub const fn pad9_th1(&self) -> &PAD9_TH1 {
        &self.pad9_th1
    }
    #[doc = "0xbc - need_des"]
    #[inline(always)]
    pub const fn pad9_th2(&self) -> &PAD9_TH2 {
        &self.pad9_th2
    }
    #[doc = "0xc0 - need_des"]
    #[inline(always)]
    pub const fn pad10_th0(&self) -> &PAD10_TH0 {
        &self.pad10_th0
    }
    #[doc = "0xc4 - need_des"]
    #[inline(always)]
    pub const fn pad10_th1(&self) -> &PAD10_TH1 {
        &self.pad10_th1
    }
    #[doc = "0xc8 - need_des"]
    #[inline(always)]
    pub const fn pad10_th2(&self) -> &PAD10_TH2 {
        &self.pad10_th2
    }
    #[doc = "0xcc - need_des"]
    #[inline(always)]
    pub const fn pad11_th0(&self) -> &PAD11_TH0 {
        &self.pad11_th0
    }
    #[doc = "0xd0 - need_des"]
    #[inline(always)]
    pub const fn pad11_th1(&self) -> &PAD11_TH1 {
        &self.pad11_th1
    }
    #[doc = "0xd4 - need_des"]
    #[inline(always)]
    pub const fn pad11_th2(&self) -> &PAD11_TH2 {
        &self.pad11_th2
    }
    #[doc = "0xd8 - need_des"]
    #[inline(always)]
    pub const fn pad12_th0(&self) -> &PAD12_TH0 {
        &self.pad12_th0
    }
    #[doc = "0xdc - need_des"]
    #[inline(always)]
    pub const fn pad12_th1(&self) -> &PAD12_TH1 {
        &self.pad12_th1
    }
    #[doc = "0xe0 - need_des"]
    #[inline(always)]
    pub const fn pad12_th2(&self) -> &PAD12_TH2 {
        &self.pad12_th2
    }
    #[doc = "0xe4 - need_des"]
    #[inline(always)]
    pub const fn pad13_th0(&self) -> &PAD13_TH0 {
        &self.pad13_th0
    }
    #[doc = "0xe8 - need_des"]
    #[inline(always)]
    pub const fn pad13_th1(&self) -> &PAD13_TH1 {
        &self.pad13_th1
    }
    #[doc = "0xec - need_des"]
    #[inline(always)]
    pub const fn pad13_th2(&self) -> &PAD13_TH2 {
        &self.pad13_th2
    }
    #[doc = "0xf0 - need_des"]
    #[inline(always)]
    pub const fn pad14_th0(&self) -> &PAD14_TH0 {
        &self.pad14_th0
    }
    #[doc = "0xf4 - need_des"]
    #[inline(always)]
    pub const fn pad14_th1(&self) -> &PAD14_TH1 {
        &self.pad14_th1
    }
    #[doc = "0xf8 - need_des"]
    #[inline(always)]
    pub const fn pad14_th2(&self) -> &PAD14_TH2 {
        &self.pad14_th2
    }
    #[doc = "0xfc - need_des"]
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
}
#[doc = "APPROACH_WORK_MEAS_NUM (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`approach_work_meas_num::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`approach_work_meas_num::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@approach_work_meas_num`] module"]
pub type APPROACH_WORK_MEAS_NUM = crate::Reg<approach_work_meas_num::APPROACH_WORK_MEAS_NUM_SPEC>;
#[doc = "need_des"]
pub mod approach_work_meas_num;
#[doc = "SCAN_CTRL1 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`scan_ctrl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scan_ctrl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scan_ctrl1`] module"]
pub type SCAN_CTRL1 = crate::Reg<scan_ctrl1::SCAN_CTRL1_SPEC>;
#[doc = "need_des"]
pub mod scan_ctrl1;
#[doc = "SCAN_CTRL2 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`scan_ctrl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scan_ctrl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scan_ctrl2`] module"]
pub type SCAN_CTRL2 = crate::Reg<scan_ctrl2::SCAN_CTRL2_SPEC>;
#[doc = "need_des"]
pub mod scan_ctrl2;
#[doc = "WORK (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`work::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`work::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@work`] module"]
pub type WORK = crate::Reg<work::WORK_SPEC>;
#[doc = "need_des"]
pub mod work;
#[doc = "WORK_MEAS_NUM (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`work_meas_num::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`work_meas_num::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@work_meas_num`] module"]
pub type WORK_MEAS_NUM = crate::Reg<work_meas_num::WORK_MEAS_NUM_SPEC>;
#[doc = "need_des"]
pub mod work_meas_num;
#[doc = "FILTER1 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`filter1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`filter1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@filter1`] module"]
pub type FILTER1 = crate::Reg<filter1::FILTER1_SPEC>;
#[doc = "need_des"]
pub mod filter1;
#[doc = "FILTER2 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`filter2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`filter2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@filter2`] module"]
pub type FILTER2 = crate::Reg<filter2::FILTER2_SPEC>;
#[doc = "need_des"]
pub mod filter2;
#[doc = "FILTER3 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`filter3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`filter3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@filter3`] module"]
pub type FILTER3 = crate::Reg<filter3::FILTER3_SPEC>;
#[doc = "need_des"]
pub mod filter3;
#[doc = "SLP0 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`slp0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`slp0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slp0`] module"]
pub type SLP0 = crate::Reg<slp0::SLP0_SPEC>;
#[doc = "need_des"]
pub mod slp0;
#[doc = "SLP1 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`slp1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`slp1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slp1`] module"]
pub type SLP1 = crate::Reg<slp1::SLP1_SPEC>;
#[doc = "need_des"]
pub mod slp1;
#[doc = "CLR (w) register accessor: need_des\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clr`] module"]
pub type CLR = crate::Reg<clr::CLR_SPEC>;
#[doc = "need_des"]
pub mod clr;
#[doc = "APPROACH (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`approach::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`approach::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@approach`] module"]
pub type APPROACH = crate::Reg<approach::APPROACH_SPEC>;
#[doc = "need_des"]
pub mod approach;
#[doc = "FREQ0_SCAN_PARA (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`freq0_scan_para::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`freq0_scan_para::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@freq0_scan_para`] module"]
pub type FREQ0_SCAN_PARA = crate::Reg<freq0_scan_para::FREQ0_SCAN_PARA_SPEC>;
#[doc = "need_des"]
pub mod freq0_scan_para;
#[doc = "FREQ1_SCAN_PARA (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`freq1_scan_para::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`freq1_scan_para::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@freq1_scan_para`] module"]
pub type FREQ1_SCAN_PARA = crate::Reg<freq1_scan_para::FREQ1_SCAN_PARA_SPEC>;
#[doc = "need_des"]
pub mod freq1_scan_para;
#[doc = "FREQ2_SCAN_PARA (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`freq2_scan_para::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`freq2_scan_para::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@freq2_scan_para`] module"]
pub type FREQ2_SCAN_PARA = crate::Reg<freq2_scan_para::FREQ2_SCAN_PARA_SPEC>;
#[doc = "need_des"]
pub mod freq2_scan_para;
#[doc = "ANA_PARA (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`ana_para::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ana_para::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ana_para`] module"]
pub type ANA_PARA = crate::Reg<ana_para::ANA_PARA_SPEC>;
#[doc = "need_des"]
pub mod ana_para;
#[doc = "MUX0 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`mux0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mux0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mux0`] module"]
pub type MUX0 = crate::Reg<mux0::MUX0_SPEC>;
#[doc = "need_des"]
pub mod mux0;
#[doc = "MUX1 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`mux1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mux1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mux1`] module"]
pub type MUX1 = crate::Reg<mux1::MUX1_SPEC>;
#[doc = "need_des"]
pub mod mux1;
#[doc = "PAD0_TH0 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`pad0_th0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad0_th0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pad0_th0`] module"]
pub type PAD0_TH0 = crate::Reg<pad0_th0::PAD0_TH0_SPEC>;
#[doc = "need_des"]
pub mod pad0_th0;
#[doc = "PAD0_TH1 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`pad0_th1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad0_th1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pad0_th1`] module"]
pub type PAD0_TH1 = crate::Reg<pad0_th1::PAD0_TH1_SPEC>;
#[doc = "need_des"]
pub mod pad0_th1;
#[doc = "PAD0_TH2 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`pad0_th2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad0_th2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pad0_th2`] module"]
pub type PAD0_TH2 = crate::Reg<pad0_th2::PAD0_TH2_SPEC>;
#[doc = "need_des"]
pub mod pad0_th2;
#[doc = "PAD1_TH0 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`pad1_th0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad1_th0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pad1_th0`] module"]
pub type PAD1_TH0 = crate::Reg<pad1_th0::PAD1_TH0_SPEC>;
#[doc = "need_des"]
pub mod pad1_th0;
#[doc = "PAD1_TH1 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`pad1_th1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad1_th1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pad1_th1`] module"]
pub type PAD1_TH1 = crate::Reg<pad1_th1::PAD1_TH1_SPEC>;
#[doc = "need_des"]
pub mod pad1_th1;
#[doc = "PAD1_TH2 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`pad1_th2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad1_th2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pad1_th2`] module"]
pub type PAD1_TH2 = crate::Reg<pad1_th2::PAD1_TH2_SPEC>;
#[doc = "need_des"]
pub mod pad1_th2;
#[doc = "PAD2_TH0 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`pad2_th0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad2_th0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pad2_th0`] module"]
pub type PAD2_TH0 = crate::Reg<pad2_th0::PAD2_TH0_SPEC>;
#[doc = "need_des"]
pub mod pad2_th0;
#[doc = "PAD2_TH1 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`pad2_th1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad2_th1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pad2_th1`] module"]
pub type PAD2_TH1 = crate::Reg<pad2_th1::PAD2_TH1_SPEC>;
#[doc = "need_des"]
pub mod pad2_th1;
#[doc = "PAD2_TH2 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`pad2_th2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad2_th2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pad2_th2`] module"]
pub type PAD2_TH2 = crate::Reg<pad2_th2::PAD2_TH2_SPEC>;
#[doc = "need_des"]
pub mod pad2_th2;
#[doc = "PAD3_TH0 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`pad3_th0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad3_th0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pad3_th0`] module"]
pub type PAD3_TH0 = crate::Reg<pad3_th0::PAD3_TH0_SPEC>;
#[doc = "need_des"]
pub mod pad3_th0;
#[doc = "PAD3_TH1 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`pad3_th1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad3_th1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pad3_th1`] module"]
pub type PAD3_TH1 = crate::Reg<pad3_th1::PAD3_TH1_SPEC>;
#[doc = "need_des"]
pub mod pad3_th1;
#[doc = "PAD3_TH2 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`pad3_th2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad3_th2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pad3_th2`] module"]
pub type PAD3_TH2 = crate::Reg<pad3_th2::PAD3_TH2_SPEC>;
#[doc = "need_des"]
pub mod pad3_th2;
#[doc = "PAD4_TH0 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`pad4_th0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad4_th0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pad4_th0`] module"]
pub type PAD4_TH0 = crate::Reg<pad4_th0::PAD4_TH0_SPEC>;
#[doc = "need_des"]
pub mod pad4_th0;
#[doc = "PAD4_TH1 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`pad4_th1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad4_th1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pad4_th1`] module"]
pub type PAD4_TH1 = crate::Reg<pad4_th1::PAD4_TH1_SPEC>;
#[doc = "need_des"]
pub mod pad4_th1;
#[doc = "PAD4_TH2 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`pad4_th2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad4_th2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pad4_th2`] module"]
pub type PAD4_TH2 = crate::Reg<pad4_th2::PAD4_TH2_SPEC>;
#[doc = "need_des"]
pub mod pad4_th2;
#[doc = "PAD5_TH0 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`pad5_th0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad5_th0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pad5_th0`] module"]
pub type PAD5_TH0 = crate::Reg<pad5_th0::PAD5_TH0_SPEC>;
#[doc = "need_des"]
pub mod pad5_th0;
#[doc = "PAD5_TH1 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`pad5_th1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad5_th1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pad5_th1`] module"]
pub type PAD5_TH1 = crate::Reg<pad5_th1::PAD5_TH1_SPEC>;
#[doc = "need_des"]
pub mod pad5_th1;
#[doc = "PAD5_TH2 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`pad5_th2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad5_th2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pad5_th2`] module"]
pub type PAD5_TH2 = crate::Reg<pad5_th2::PAD5_TH2_SPEC>;
#[doc = "need_des"]
pub mod pad5_th2;
#[doc = "PAD6_TH0 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`pad6_th0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad6_th0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pad6_th0`] module"]
pub type PAD6_TH0 = crate::Reg<pad6_th0::PAD6_TH0_SPEC>;
#[doc = "need_des"]
pub mod pad6_th0;
#[doc = "PAD6_TH1 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`pad6_th1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad6_th1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pad6_th1`] module"]
pub type PAD6_TH1 = crate::Reg<pad6_th1::PAD6_TH1_SPEC>;
#[doc = "need_des"]
pub mod pad6_th1;
#[doc = "PAD6_TH2 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`pad6_th2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad6_th2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pad6_th2`] module"]
pub type PAD6_TH2 = crate::Reg<pad6_th2::PAD6_TH2_SPEC>;
#[doc = "need_des"]
pub mod pad6_th2;
#[doc = "PAD7_TH0 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`pad7_th0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad7_th0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pad7_th0`] module"]
pub type PAD7_TH0 = crate::Reg<pad7_th0::PAD7_TH0_SPEC>;
#[doc = "need_des"]
pub mod pad7_th0;
#[doc = "PAD7_TH1 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`pad7_th1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad7_th1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pad7_th1`] module"]
pub type PAD7_TH1 = crate::Reg<pad7_th1::PAD7_TH1_SPEC>;
#[doc = "need_des"]
pub mod pad7_th1;
#[doc = "PAD7_TH2 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`pad7_th2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad7_th2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pad7_th2`] module"]
pub type PAD7_TH2 = crate::Reg<pad7_th2::PAD7_TH2_SPEC>;
#[doc = "need_des"]
pub mod pad7_th2;
#[doc = "PAD8_TH0 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`pad8_th0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad8_th0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pad8_th0`] module"]
pub type PAD8_TH0 = crate::Reg<pad8_th0::PAD8_TH0_SPEC>;
#[doc = "need_des"]
pub mod pad8_th0;
#[doc = "PAD8_TH1 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`pad8_th1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad8_th1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pad8_th1`] module"]
pub type PAD8_TH1 = crate::Reg<pad8_th1::PAD8_TH1_SPEC>;
#[doc = "need_des"]
pub mod pad8_th1;
#[doc = "PAD8_TH2 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`pad8_th2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad8_th2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pad8_th2`] module"]
pub type PAD8_TH2 = crate::Reg<pad8_th2::PAD8_TH2_SPEC>;
#[doc = "need_des"]
pub mod pad8_th2;
#[doc = "PAD9_TH0 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`pad9_th0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad9_th0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pad9_th0`] module"]
pub type PAD9_TH0 = crate::Reg<pad9_th0::PAD9_TH0_SPEC>;
#[doc = "need_des"]
pub mod pad9_th0;
#[doc = "PAD9_TH1 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`pad9_th1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad9_th1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pad9_th1`] module"]
pub type PAD9_TH1 = crate::Reg<pad9_th1::PAD9_TH1_SPEC>;
#[doc = "need_des"]
pub mod pad9_th1;
#[doc = "PAD9_TH2 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`pad9_th2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad9_th2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pad9_th2`] module"]
pub type PAD9_TH2 = crate::Reg<pad9_th2::PAD9_TH2_SPEC>;
#[doc = "need_des"]
pub mod pad9_th2;
#[doc = "PAD10_TH0 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`pad10_th0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad10_th0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pad10_th0`] module"]
pub type PAD10_TH0 = crate::Reg<pad10_th0::PAD10_TH0_SPEC>;
#[doc = "need_des"]
pub mod pad10_th0;
#[doc = "PAD10_TH1 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`pad10_th1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad10_th1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pad10_th1`] module"]
pub type PAD10_TH1 = crate::Reg<pad10_th1::PAD10_TH1_SPEC>;
#[doc = "need_des"]
pub mod pad10_th1;
#[doc = "PAD10_TH2 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`pad10_th2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad10_th2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pad10_th2`] module"]
pub type PAD10_TH2 = crate::Reg<pad10_th2::PAD10_TH2_SPEC>;
#[doc = "need_des"]
pub mod pad10_th2;
#[doc = "PAD11_TH0 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`pad11_th0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad11_th0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pad11_th0`] module"]
pub type PAD11_TH0 = crate::Reg<pad11_th0::PAD11_TH0_SPEC>;
#[doc = "need_des"]
pub mod pad11_th0;
#[doc = "PAD11_TH1 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`pad11_th1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad11_th1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pad11_th1`] module"]
pub type PAD11_TH1 = crate::Reg<pad11_th1::PAD11_TH1_SPEC>;
#[doc = "need_des"]
pub mod pad11_th1;
#[doc = "PAD11_TH2 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`pad11_th2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad11_th2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pad11_th2`] module"]
pub type PAD11_TH2 = crate::Reg<pad11_th2::PAD11_TH2_SPEC>;
#[doc = "need_des"]
pub mod pad11_th2;
#[doc = "PAD12_TH0 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`pad12_th0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad12_th0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pad12_th0`] module"]
pub type PAD12_TH0 = crate::Reg<pad12_th0::PAD12_TH0_SPEC>;
#[doc = "need_des"]
pub mod pad12_th0;
#[doc = "PAD12_TH1 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`pad12_th1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad12_th1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pad12_th1`] module"]
pub type PAD12_TH1 = crate::Reg<pad12_th1::PAD12_TH1_SPEC>;
#[doc = "need_des"]
pub mod pad12_th1;
#[doc = "PAD12_TH2 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`pad12_th2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad12_th2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pad12_th2`] module"]
pub type PAD12_TH2 = crate::Reg<pad12_th2::PAD12_TH2_SPEC>;
#[doc = "need_des"]
pub mod pad12_th2;
#[doc = "PAD13_TH0 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`pad13_th0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad13_th0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pad13_th0`] module"]
pub type PAD13_TH0 = crate::Reg<pad13_th0::PAD13_TH0_SPEC>;
#[doc = "need_des"]
pub mod pad13_th0;
#[doc = "PAD13_TH1 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`pad13_th1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad13_th1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pad13_th1`] module"]
pub type PAD13_TH1 = crate::Reg<pad13_th1::PAD13_TH1_SPEC>;
#[doc = "need_des"]
pub mod pad13_th1;
#[doc = "PAD13_TH2 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`pad13_th2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad13_th2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pad13_th2`] module"]
pub type PAD13_TH2 = crate::Reg<pad13_th2::PAD13_TH2_SPEC>;
#[doc = "need_des"]
pub mod pad13_th2;
#[doc = "PAD14_TH0 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`pad14_th0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad14_th0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pad14_th0`] module"]
pub type PAD14_TH0 = crate::Reg<pad14_th0::PAD14_TH0_SPEC>;
#[doc = "need_des"]
pub mod pad14_th0;
#[doc = "PAD14_TH1 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`pad14_th1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad14_th1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pad14_th1`] module"]
pub type PAD14_TH1 = crate::Reg<pad14_th1::PAD14_TH1_SPEC>;
#[doc = "need_des"]
pub mod pad14_th1;
#[doc = "PAD14_TH2 (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`pad14_th2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad14_th2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pad14_th2`] module"]
pub type PAD14_TH2 = crate::Reg<pad14_th2::PAD14_TH2_SPEC>;
#[doc = "need_des"]
pub mod pad14_th2;
#[doc = "DATE (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`date::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@date`] module"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "need_des"]
pub mod date;
