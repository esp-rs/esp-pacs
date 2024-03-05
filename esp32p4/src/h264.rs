#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    sys_ctrl: SYS_CTRL,
    gop_conf: GOP_CONF,
    a_sys_mb_res: A_SYS_MB_RES,
    a_sys_conf: A_SYS_CONF,
    a_deci_score: A_DECI_SCORE,
    a_deci_score_offset: A_DECI_SCORE_OFFSET,
    a_rc_conf0: A_RC_CONF0,
    a_rc_conf1: A_RC_CONF1,
    a_db_bypass: A_DB_BYPASS,
    a_roi_region0: A_ROI_REGION0,
    a_roi_region1: A_ROI_REGION1,
    a_roi_region2: A_ROI_REGION2,
    a_roi_region3: A_ROI_REGION3,
    a_roi_region4: A_ROI_REGION4,
    a_roi_region5: A_ROI_REGION5,
    a_roi_region6: A_ROI_REGION6,
    a_roi_region7: A_ROI_REGION7,
    a_roi_region0_3_qp: A_ROI_REGION0_3_QP,
    a_roi_region4_7_qp: A_ROI_REGION4_7_QP,
    a_no_roi_region_qp_offset: A_NO_ROI_REGION_QP_OFFSET,
    a_roi_config: A_ROI_CONFIG,
    b_sys_mb_res: B_SYS_MB_RES,
    b_sys_conf: B_SYS_CONF,
    b_deci_score: B_DECI_SCORE,
    b_deci_score_offset: B_DECI_SCORE_OFFSET,
    b_rc_conf0: B_RC_CONF0,
    b_rc_conf1: B_RC_CONF1,
    b_db_bypass: B_DB_BYPASS,
    b_roi_region0: B_ROI_REGION0,
    b_roi_region1: B_ROI_REGION1,
    b_roi_region2: B_ROI_REGION2,
    b_roi_region3: B_ROI_REGION3,
    b_roi_region4: B_ROI_REGION4,
    b_roi_region5: B_ROI_REGION5,
    b_roi_region6: B_ROI_REGION6,
    b_roi_region7: B_ROI_REGION7,
    b_roi_region0_3_qp: B_ROI_REGION0_3_QP,
    b_roi_region4_7_qp: B_ROI_REGION4_7_QP,
    b_no_roi_region_qp_offset: B_NO_ROI_REGION_QP_OFFSET,
    b_roi_config: B_ROI_CONFIG,
    rc_status0: RC_STATUS0,
    rc_status1: RC_STATUS1,
    rc_status2: RC_STATUS2,
    slice_header_remain: SLICE_HEADER_REMAIN,
    slice_header_byte_length: SLICE_HEADER_BYTE_LENGTH,
    bs_threshold: BS_THRESHOLD,
    slice_header_byte0: SLICE_HEADER_BYTE0,
    slice_header_byte1: SLICE_HEADER_BYTE1,
    int_raw: INT_RAW,
    int_st: INT_ST,
    int_ena: INT_ENA,
    int_clr: INT_CLR,
    conf: CONF,
    mv_merge_config: MV_MERGE_CONFIG,
    debug_dma_sel: DEBUG_DMA_SEL,
    sys_status: SYS_STATUS,
    frame_code_length: FRAME_CODE_LENGTH,
    debug_info0: DEBUG_INFO0,
    debug_info1: DEBUG_INFO1,
    debug_info2: DEBUG_INFO2,
    date: DATE,
}
impl RegisterBlock {
    #[doc = "0x00 - H264 system level control register."]
    #[inline(always)]
    pub const fn sys_ctrl(&self) -> &SYS_CTRL {
        &self.sys_ctrl
    }
    #[doc = "0x04 - GOP related configuration register."]
    #[inline(always)]
    pub const fn gop_conf(&self) -> &GOP_CONF {
        &self.gop_conf
    }
    #[doc = "0x08 - Video A horizontal and vertical MB resolution register."]
    #[inline(always)]
    pub const fn a_sys_mb_res(&self) -> &A_SYS_MB_RES {
        &self.a_sys_mb_res
    }
    #[doc = "0x0c - Video A system level configuration register."]
    #[inline(always)]
    pub const fn a_sys_conf(&self) -> &A_SYS_CONF {
        &self.a_sys_conf
    }
    #[doc = "0x10 - Video A luma and chroma MB decimate score Register."]
    #[inline(always)]
    pub const fn a_deci_score(&self) -> &A_DECI_SCORE {
        &self.a_deci_score
    }
    #[doc = "0x14 - Video A luma and chroma MB decimate score offset Register."]
    #[inline(always)]
    pub const fn a_deci_score_offset(&self) -> &A_DECI_SCORE_OFFSET {
        &self.a_deci_score_offset
    }
    #[doc = "0x18 - Video A rate control configuration register0."]
    #[inline(always)]
    pub const fn a_rc_conf0(&self) -> &A_RC_CONF0 {
        &self.a_rc_conf0
    }
    #[doc = "0x1c - Video A rate control configuration register1."]
    #[inline(always)]
    pub const fn a_rc_conf1(&self) -> &A_RC_CONF1 {
        &self.a_rc_conf1
    }
    #[doc = "0x20 - Video A Deblocking bypass register"]
    #[inline(always)]
    pub const fn a_db_bypass(&self) -> &A_DB_BYPASS {
        &self.a_db_bypass
    }
    #[doc = "0x24 - Video A H264 ROI region0 range configure register."]
    #[inline(always)]
    pub const fn a_roi_region0(&self) -> &A_ROI_REGION0 {
        &self.a_roi_region0
    }
    #[doc = "0x28 - Video A H264 ROI region1 range configure register."]
    #[inline(always)]
    pub const fn a_roi_region1(&self) -> &A_ROI_REGION1 {
        &self.a_roi_region1
    }
    #[doc = "0x2c - Video A H264 ROI region2 range configure register."]
    #[inline(always)]
    pub const fn a_roi_region2(&self) -> &A_ROI_REGION2 {
        &self.a_roi_region2
    }
    #[doc = "0x30 - Video A H264 ROI region3 range configure register."]
    #[inline(always)]
    pub const fn a_roi_region3(&self) -> &A_ROI_REGION3 {
        &self.a_roi_region3
    }
    #[doc = "0x34 - Video A H264 ROI region4 range configure register."]
    #[inline(always)]
    pub const fn a_roi_region4(&self) -> &A_ROI_REGION4 {
        &self.a_roi_region4
    }
    #[doc = "0x38 - Video A H264 ROI region5 range configure register."]
    #[inline(always)]
    pub const fn a_roi_region5(&self) -> &A_ROI_REGION5 {
        &self.a_roi_region5
    }
    #[doc = "0x3c - Video A H264 ROI region6 range configure register."]
    #[inline(always)]
    pub const fn a_roi_region6(&self) -> &A_ROI_REGION6 {
        &self.a_roi_region6
    }
    #[doc = "0x40 - Video A H264 ROI region7 range configure register."]
    #[inline(always)]
    pub const fn a_roi_region7(&self) -> &A_ROI_REGION7 {
        &self.a_roi_region7
    }
    #[doc = "0x44 - Video A H264 ROI region0, region1,region2,region3 QP register."]
    #[inline(always)]
    pub const fn a_roi_region0_3_qp(&self) -> &A_ROI_REGION0_3_QP {
        &self.a_roi_region0_3_qp
    }
    #[doc = "0x48 - Video A H264 ROI region4, region5,region6,region7 QP register."]
    #[inline(always)]
    pub const fn a_roi_region4_7_qp(&self) -> &A_ROI_REGION4_7_QP {
        &self.a_roi_region4_7_qp
    }
    #[doc = "0x4c - Video A H264 no roi region QP register."]
    #[inline(always)]
    pub const fn a_no_roi_region_qp_offset(&self) -> &A_NO_ROI_REGION_QP_OFFSET {
        &self.a_no_roi_region_qp_offset
    }
    #[doc = "0x50 - Video A H264 ROI configure register."]
    #[inline(always)]
    pub const fn a_roi_config(&self) -> &A_ROI_CONFIG {
        &self.a_roi_config
    }
    #[doc = "0x54 - Video B horizontal and vertical MB resolution register."]
    #[inline(always)]
    pub const fn b_sys_mb_res(&self) -> &B_SYS_MB_RES {
        &self.b_sys_mb_res
    }
    #[doc = "0x58 - Video B system level configuration register."]
    #[inline(always)]
    pub const fn b_sys_conf(&self) -> &B_SYS_CONF {
        &self.b_sys_conf
    }
    #[doc = "0x5c - Video B luma and chroma MB decimate score Register."]
    #[inline(always)]
    pub const fn b_deci_score(&self) -> &B_DECI_SCORE {
        &self.b_deci_score
    }
    #[doc = "0x60 - Video B luma and chroma MB decimate score offset Register."]
    #[inline(always)]
    pub const fn b_deci_score_offset(&self) -> &B_DECI_SCORE_OFFSET {
        &self.b_deci_score_offset
    }
    #[doc = "0x64 - Video B rate control configuration register0."]
    #[inline(always)]
    pub const fn b_rc_conf0(&self) -> &B_RC_CONF0 {
        &self.b_rc_conf0
    }
    #[doc = "0x68 - Video B rate control configuration register1."]
    #[inline(always)]
    pub const fn b_rc_conf1(&self) -> &B_RC_CONF1 {
        &self.b_rc_conf1
    }
    #[doc = "0x6c - Video B Deblocking bypass register"]
    #[inline(always)]
    pub const fn b_db_bypass(&self) -> &B_DB_BYPASS {
        &self.b_db_bypass
    }
    #[doc = "0x70 - Video B H264 ROI region0 range configure register."]
    #[inline(always)]
    pub const fn b_roi_region0(&self) -> &B_ROI_REGION0 {
        &self.b_roi_region0
    }
    #[doc = "0x74 - Video B H264 ROI region1 range configure register."]
    #[inline(always)]
    pub const fn b_roi_region1(&self) -> &B_ROI_REGION1 {
        &self.b_roi_region1
    }
    #[doc = "0x78 - Video B H264 ROI region2 range configure register."]
    #[inline(always)]
    pub const fn b_roi_region2(&self) -> &B_ROI_REGION2 {
        &self.b_roi_region2
    }
    #[doc = "0x7c - Video B H264 ROI region3 range configure register."]
    #[inline(always)]
    pub const fn b_roi_region3(&self) -> &B_ROI_REGION3 {
        &self.b_roi_region3
    }
    #[doc = "0x80 - Video B H264 ROI region4 range configure register."]
    #[inline(always)]
    pub const fn b_roi_region4(&self) -> &B_ROI_REGION4 {
        &self.b_roi_region4
    }
    #[doc = "0x84 - Video B H264 ROI region5 range configure register."]
    #[inline(always)]
    pub const fn b_roi_region5(&self) -> &B_ROI_REGION5 {
        &self.b_roi_region5
    }
    #[doc = "0x88 - Video B H264 ROI region6 range configure register."]
    #[inline(always)]
    pub const fn b_roi_region6(&self) -> &B_ROI_REGION6 {
        &self.b_roi_region6
    }
    #[doc = "0x8c - Video B H264 ROI region7 range configure register."]
    #[inline(always)]
    pub const fn b_roi_region7(&self) -> &B_ROI_REGION7 {
        &self.b_roi_region7
    }
    #[doc = "0x90 - Video B H264 ROI region0, region1,region2,region3 QP register."]
    #[inline(always)]
    pub const fn b_roi_region0_3_qp(&self) -> &B_ROI_REGION0_3_QP {
        &self.b_roi_region0_3_qp
    }
    #[doc = "0x94 - Video B H264 ROI region4, region5,region6,region7 QP register."]
    #[inline(always)]
    pub const fn b_roi_region4_7_qp(&self) -> &B_ROI_REGION4_7_QP {
        &self.b_roi_region4_7_qp
    }
    #[doc = "0x98 - Video B H264 no roi region QP register."]
    #[inline(always)]
    pub const fn b_no_roi_region_qp_offset(&self) -> &B_NO_ROI_REGION_QP_OFFSET {
        &self.b_no_roi_region_qp_offset
    }
    #[doc = "0x9c - Video B H264 ROI configure register."]
    #[inline(always)]
    pub const fn b_roi_config(&self) -> &B_ROI_CONFIG {
        &self.b_roi_config
    }
    #[doc = "0xa0 - Rate control status register0."]
    #[inline(always)]
    pub const fn rc_status0(&self) -> &RC_STATUS0 {
        &self.rc_status0
    }
    #[doc = "0xa4 - Rate control status register1."]
    #[inline(always)]
    pub const fn rc_status1(&self) -> &RC_STATUS1 {
        &self.rc_status1
    }
    #[doc = "0xa8 - Rate control status register2."]
    #[inline(always)]
    pub const fn rc_status2(&self) -> &RC_STATUS2 {
        &self.rc_status2
    }
    #[doc = "0xac - Frame Slice Header remain bit register."]
    #[inline(always)]
    pub const fn slice_header_remain(&self) -> &SLICE_HEADER_REMAIN {
        &self.slice_header_remain
    }
    #[doc = "0xb0 - Frame Slice Header byte length register."]
    #[inline(always)]
    pub const fn slice_header_byte_length(&self) -> &SLICE_HEADER_BYTE_LENGTH {
        &self.slice_header_byte_length
    }
    #[doc = "0xb4 - Bitstream buffer overflow threshold register"]
    #[inline(always)]
    pub const fn bs_threshold(&self) -> &BS_THRESHOLD {
        &self.bs_threshold
    }
    #[doc = "0xb8 - Frame Slice Header byte low 32 bit register."]
    #[inline(always)]
    pub const fn slice_header_byte0(&self) -> &SLICE_HEADER_BYTE0 {
        &self.slice_header_byte0
    }
    #[doc = "0xbc - Frame Slice Header byte high 32 bit register."]
    #[inline(always)]
    pub const fn slice_header_byte1(&self) -> &SLICE_HEADER_BYTE1 {
        &self.slice_header_byte1
    }
    #[doc = "0xc0 - Interrupt raw status register"]
    #[inline(always)]
    pub const fn int_raw(&self) -> &INT_RAW {
        &self.int_raw
    }
    #[doc = "0xc4 - Interrupt masked status register"]
    #[inline(always)]
    pub const fn int_st(&self) -> &INT_ST {
        &self.int_st
    }
    #[doc = "0xc8 - Interrupt enable register"]
    #[inline(always)]
    pub const fn int_ena(&self) -> &INT_ENA {
        &self.int_ena
    }
    #[doc = "0xcc - Interrupt clear register"]
    #[inline(always)]
    pub const fn int_clr(&self) -> &INT_CLR {
        &self.int_clr
    }
    #[doc = "0xd0 - General configuration register."]
    #[inline(always)]
    pub const fn conf(&self) -> &CONF {
        &self.conf
    }
    #[doc = "0xd4 - Mv merge configuration register."]
    #[inline(always)]
    pub const fn mv_merge_config(&self) -> &MV_MERGE_CONFIG {
        &self.mv_merge_config
    }
    #[doc = "0xd8 - Debug H264 DMA select register"]
    #[inline(always)]
    pub const fn debug_dma_sel(&self) -> &DEBUG_DMA_SEL {
        &self.debug_dma_sel
    }
    #[doc = "0xdc - System status register."]
    #[inline(always)]
    pub const fn sys_status(&self) -> &SYS_STATUS {
        &self.sys_status
    }
    #[doc = "0xe0 - Frame code byte length register."]
    #[inline(always)]
    pub const fn frame_code_length(&self) -> &FRAME_CODE_LENGTH {
        &self.frame_code_length
    }
    #[doc = "0xe4 - Debug information register0."]
    #[inline(always)]
    pub const fn debug_info0(&self) -> &DEBUG_INFO0 {
        &self.debug_info0
    }
    #[doc = "0xe8 - Debug information register1."]
    #[inline(always)]
    pub const fn debug_info1(&self) -> &DEBUG_INFO1 {
        &self.debug_info1
    }
    #[doc = "0xec - Debug information register2."]
    #[inline(always)]
    pub const fn debug_info2(&self) -> &DEBUG_INFO2 {
        &self.debug_info2
    }
    #[doc = "0xf0 - Version control register"]
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
}
#[doc = "SYS_CTRL (rw) register accessor: H264 system level control register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_ctrl`] module"]
pub type SYS_CTRL = crate::Reg<sys_ctrl::SYS_CTRL_SPEC>;
#[doc = "H264 system level control register."]
pub mod sys_ctrl;
#[doc = "GOP_CONF (rw) register accessor: GOP related configuration register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gop_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gop_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gop_conf`] module"]
pub type GOP_CONF = crate::Reg<gop_conf::GOP_CONF_SPEC>;
#[doc = "GOP related configuration register."]
pub mod gop_conf;
#[doc = "A_SYS_MB_RES (rw) register accessor: Video A horizontal and vertical MB resolution register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`a_sys_mb_res::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`a_sys_mb_res::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@a_sys_mb_res`] module"]
pub type A_SYS_MB_RES = crate::Reg<a_sys_mb_res::A_SYS_MB_RES_SPEC>;
#[doc = "Video A horizontal and vertical MB resolution register."]
pub mod a_sys_mb_res;
#[doc = "A_SYS_CONF (rw) register accessor: Video A system level configuration register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`a_sys_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`a_sys_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@a_sys_conf`] module"]
pub type A_SYS_CONF = crate::Reg<a_sys_conf::A_SYS_CONF_SPEC>;
#[doc = "Video A system level configuration register."]
pub mod a_sys_conf;
#[doc = "A_DECI_SCORE (rw) register accessor: Video A luma and chroma MB decimate score Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`a_deci_score::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`a_deci_score::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@a_deci_score`] module"]
pub type A_DECI_SCORE = crate::Reg<a_deci_score::A_DECI_SCORE_SPEC>;
#[doc = "Video A luma and chroma MB decimate score Register."]
pub mod a_deci_score;
#[doc = "A_DECI_SCORE_OFFSET (rw) register accessor: Video A luma and chroma MB decimate score offset Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`a_deci_score_offset::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`a_deci_score_offset::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@a_deci_score_offset`] module"]
pub type A_DECI_SCORE_OFFSET = crate::Reg<a_deci_score_offset::A_DECI_SCORE_OFFSET_SPEC>;
#[doc = "Video A luma and chroma MB decimate score offset Register."]
pub mod a_deci_score_offset;
#[doc = "A_RC_CONF0 (rw) register accessor: Video A rate control configuration register0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`a_rc_conf0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`a_rc_conf0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@a_rc_conf0`] module"]
pub type A_RC_CONF0 = crate::Reg<a_rc_conf0::A_RC_CONF0_SPEC>;
#[doc = "Video A rate control configuration register0."]
pub mod a_rc_conf0;
#[doc = "A_RC_CONF1 (rw) register accessor: Video A rate control configuration register1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`a_rc_conf1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`a_rc_conf1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@a_rc_conf1`] module"]
pub type A_RC_CONF1 = crate::Reg<a_rc_conf1::A_RC_CONF1_SPEC>;
#[doc = "Video A rate control configuration register1."]
pub mod a_rc_conf1;
#[doc = "A_DB_BYPASS (rw) register accessor: Video A Deblocking bypass register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`a_db_bypass::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`a_db_bypass::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@a_db_bypass`] module"]
pub type A_DB_BYPASS = crate::Reg<a_db_bypass::A_DB_BYPASS_SPEC>;
#[doc = "Video A Deblocking bypass register"]
pub mod a_db_bypass;
#[doc = "A_ROI_REGION0 (rw) register accessor: Video A H264 ROI region0 range configure register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`a_roi_region0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`a_roi_region0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@a_roi_region0`] module"]
pub type A_ROI_REGION0 = crate::Reg<a_roi_region0::A_ROI_REGION0_SPEC>;
#[doc = "Video A H264 ROI region0 range configure register."]
pub mod a_roi_region0;
#[doc = "A_ROI_REGION1 (rw) register accessor: Video A H264 ROI region1 range configure register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`a_roi_region1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`a_roi_region1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@a_roi_region1`] module"]
pub type A_ROI_REGION1 = crate::Reg<a_roi_region1::A_ROI_REGION1_SPEC>;
#[doc = "Video A H264 ROI region1 range configure register."]
pub mod a_roi_region1;
#[doc = "A_ROI_REGION2 (rw) register accessor: Video A H264 ROI region2 range configure register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`a_roi_region2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`a_roi_region2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@a_roi_region2`] module"]
pub type A_ROI_REGION2 = crate::Reg<a_roi_region2::A_ROI_REGION2_SPEC>;
#[doc = "Video A H264 ROI region2 range configure register."]
pub mod a_roi_region2;
#[doc = "A_ROI_REGION3 (rw) register accessor: Video A H264 ROI region3 range configure register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`a_roi_region3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`a_roi_region3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@a_roi_region3`] module"]
pub type A_ROI_REGION3 = crate::Reg<a_roi_region3::A_ROI_REGION3_SPEC>;
#[doc = "Video A H264 ROI region3 range configure register."]
pub mod a_roi_region3;
#[doc = "A_ROI_REGION4 (rw) register accessor: Video A H264 ROI region4 range configure register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`a_roi_region4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`a_roi_region4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@a_roi_region4`] module"]
pub type A_ROI_REGION4 = crate::Reg<a_roi_region4::A_ROI_REGION4_SPEC>;
#[doc = "Video A H264 ROI region4 range configure register."]
pub mod a_roi_region4;
#[doc = "A_ROI_REGION5 (rw) register accessor: Video A H264 ROI region5 range configure register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`a_roi_region5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`a_roi_region5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@a_roi_region5`] module"]
pub type A_ROI_REGION5 = crate::Reg<a_roi_region5::A_ROI_REGION5_SPEC>;
#[doc = "Video A H264 ROI region5 range configure register."]
pub mod a_roi_region5;
#[doc = "A_ROI_REGION6 (rw) register accessor: Video A H264 ROI region6 range configure register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`a_roi_region6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`a_roi_region6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@a_roi_region6`] module"]
pub type A_ROI_REGION6 = crate::Reg<a_roi_region6::A_ROI_REGION6_SPEC>;
#[doc = "Video A H264 ROI region6 range configure register."]
pub mod a_roi_region6;
#[doc = "A_ROI_REGION7 (rw) register accessor: Video A H264 ROI region7 range configure register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`a_roi_region7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`a_roi_region7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@a_roi_region7`] module"]
pub type A_ROI_REGION7 = crate::Reg<a_roi_region7::A_ROI_REGION7_SPEC>;
#[doc = "Video A H264 ROI region7 range configure register."]
pub mod a_roi_region7;
#[doc = "A_ROI_REGION0_3_QP (rw) register accessor: Video A H264 ROI region0, region1,region2,region3 QP register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`a_roi_region0_3_qp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`a_roi_region0_3_qp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@a_roi_region0_3_qp`] module"]
pub type A_ROI_REGION0_3_QP = crate::Reg<a_roi_region0_3_qp::A_ROI_REGION0_3_QP_SPEC>;
#[doc = "Video A H264 ROI region0, region1,region2,region3 QP register."]
pub mod a_roi_region0_3_qp;
#[doc = "A_ROI_REGION4_7_QP (rw) register accessor: Video A H264 ROI region4, region5,region6,region7 QP register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`a_roi_region4_7_qp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`a_roi_region4_7_qp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@a_roi_region4_7_qp`] module"]
pub type A_ROI_REGION4_7_QP = crate::Reg<a_roi_region4_7_qp::A_ROI_REGION4_7_QP_SPEC>;
#[doc = "Video A H264 ROI region4, region5,region6,region7 QP register."]
pub mod a_roi_region4_7_qp;
#[doc = "A_NO_ROI_REGION_QP_OFFSET (rw) register accessor: Video A H264 no roi region QP register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`a_no_roi_region_qp_offset::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`a_no_roi_region_qp_offset::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@a_no_roi_region_qp_offset`] module"]
pub type A_NO_ROI_REGION_QP_OFFSET =
    crate::Reg<a_no_roi_region_qp_offset::A_NO_ROI_REGION_QP_OFFSET_SPEC>;
#[doc = "Video A H264 no roi region QP register."]
pub mod a_no_roi_region_qp_offset;
#[doc = "A_ROI_CONFIG (rw) register accessor: Video A H264 ROI configure register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`a_roi_config::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`a_roi_config::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@a_roi_config`] module"]
pub type A_ROI_CONFIG = crate::Reg<a_roi_config::A_ROI_CONFIG_SPEC>;
#[doc = "Video A H264 ROI configure register."]
pub mod a_roi_config;
#[doc = "B_SYS_MB_RES (rw) register accessor: Video B horizontal and vertical MB resolution register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`b_sys_mb_res::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`b_sys_mb_res::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@b_sys_mb_res`] module"]
pub type B_SYS_MB_RES = crate::Reg<b_sys_mb_res::B_SYS_MB_RES_SPEC>;
#[doc = "Video B horizontal and vertical MB resolution register."]
pub mod b_sys_mb_res;
#[doc = "B_SYS_CONF (rw) register accessor: Video B system level configuration register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`b_sys_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`b_sys_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@b_sys_conf`] module"]
pub type B_SYS_CONF = crate::Reg<b_sys_conf::B_SYS_CONF_SPEC>;
#[doc = "Video B system level configuration register."]
pub mod b_sys_conf;
#[doc = "B_DECI_SCORE (rw) register accessor: Video B luma and chroma MB decimate score Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`b_deci_score::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`b_deci_score::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@b_deci_score`] module"]
pub type B_DECI_SCORE = crate::Reg<b_deci_score::B_DECI_SCORE_SPEC>;
#[doc = "Video B luma and chroma MB decimate score Register."]
pub mod b_deci_score;
#[doc = "B_DECI_SCORE_OFFSET (rw) register accessor: Video B luma and chroma MB decimate score offset Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`b_deci_score_offset::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`b_deci_score_offset::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@b_deci_score_offset`] module"]
pub type B_DECI_SCORE_OFFSET = crate::Reg<b_deci_score_offset::B_DECI_SCORE_OFFSET_SPEC>;
#[doc = "Video B luma and chroma MB decimate score offset Register."]
pub mod b_deci_score_offset;
#[doc = "B_RC_CONF0 (rw) register accessor: Video B rate control configuration register0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`b_rc_conf0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`b_rc_conf0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@b_rc_conf0`] module"]
pub type B_RC_CONF0 = crate::Reg<b_rc_conf0::B_RC_CONF0_SPEC>;
#[doc = "Video B rate control configuration register0."]
pub mod b_rc_conf0;
#[doc = "B_RC_CONF1 (rw) register accessor: Video B rate control configuration register1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`b_rc_conf1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`b_rc_conf1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@b_rc_conf1`] module"]
pub type B_RC_CONF1 = crate::Reg<b_rc_conf1::B_RC_CONF1_SPEC>;
#[doc = "Video B rate control configuration register1."]
pub mod b_rc_conf1;
#[doc = "B_DB_BYPASS (rw) register accessor: Video B Deblocking bypass register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`b_db_bypass::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`b_db_bypass::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@b_db_bypass`] module"]
pub type B_DB_BYPASS = crate::Reg<b_db_bypass::B_DB_BYPASS_SPEC>;
#[doc = "Video B Deblocking bypass register"]
pub mod b_db_bypass;
#[doc = "B_ROI_REGION0 (rw) register accessor: Video B H264 ROI region0 range configure register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`b_roi_region0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`b_roi_region0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@b_roi_region0`] module"]
pub type B_ROI_REGION0 = crate::Reg<b_roi_region0::B_ROI_REGION0_SPEC>;
#[doc = "Video B H264 ROI region0 range configure register."]
pub mod b_roi_region0;
#[doc = "B_ROI_REGION1 (rw) register accessor: Video B H264 ROI region1 range configure register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`b_roi_region1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`b_roi_region1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@b_roi_region1`] module"]
pub type B_ROI_REGION1 = crate::Reg<b_roi_region1::B_ROI_REGION1_SPEC>;
#[doc = "Video B H264 ROI region1 range configure register."]
pub mod b_roi_region1;
#[doc = "B_ROI_REGION2 (rw) register accessor: Video B H264 ROI region2 range configure register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`b_roi_region2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`b_roi_region2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@b_roi_region2`] module"]
pub type B_ROI_REGION2 = crate::Reg<b_roi_region2::B_ROI_REGION2_SPEC>;
#[doc = "Video B H264 ROI region2 range configure register."]
pub mod b_roi_region2;
#[doc = "B_ROI_REGION3 (rw) register accessor: Video B H264 ROI region3 range configure register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`b_roi_region3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`b_roi_region3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@b_roi_region3`] module"]
pub type B_ROI_REGION3 = crate::Reg<b_roi_region3::B_ROI_REGION3_SPEC>;
#[doc = "Video B H264 ROI region3 range configure register."]
pub mod b_roi_region3;
#[doc = "B_ROI_REGION4 (rw) register accessor: Video B H264 ROI region4 range configure register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`b_roi_region4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`b_roi_region4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@b_roi_region4`] module"]
pub type B_ROI_REGION4 = crate::Reg<b_roi_region4::B_ROI_REGION4_SPEC>;
#[doc = "Video B H264 ROI region4 range configure register."]
pub mod b_roi_region4;
#[doc = "B_ROI_REGION5 (rw) register accessor: Video B H264 ROI region5 range configure register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`b_roi_region5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`b_roi_region5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@b_roi_region5`] module"]
pub type B_ROI_REGION5 = crate::Reg<b_roi_region5::B_ROI_REGION5_SPEC>;
#[doc = "Video B H264 ROI region5 range configure register."]
pub mod b_roi_region5;
#[doc = "B_ROI_REGION6 (rw) register accessor: Video B H264 ROI region6 range configure register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`b_roi_region6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`b_roi_region6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@b_roi_region6`] module"]
pub type B_ROI_REGION6 = crate::Reg<b_roi_region6::B_ROI_REGION6_SPEC>;
#[doc = "Video B H264 ROI region6 range configure register."]
pub mod b_roi_region6;
#[doc = "B_ROI_REGION7 (rw) register accessor: Video B H264 ROI region7 range configure register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`b_roi_region7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`b_roi_region7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@b_roi_region7`] module"]
pub type B_ROI_REGION7 = crate::Reg<b_roi_region7::B_ROI_REGION7_SPEC>;
#[doc = "Video B H264 ROI region7 range configure register."]
pub mod b_roi_region7;
#[doc = "B_ROI_REGION0_3_QP (rw) register accessor: Video B H264 ROI region0, region1,region2,region3 QP register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`b_roi_region0_3_qp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`b_roi_region0_3_qp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@b_roi_region0_3_qp`] module"]
pub type B_ROI_REGION0_3_QP = crate::Reg<b_roi_region0_3_qp::B_ROI_REGION0_3_QP_SPEC>;
#[doc = "Video B H264 ROI region0, region1,region2,region3 QP register."]
pub mod b_roi_region0_3_qp;
#[doc = "B_ROI_REGION4_7_QP (rw) register accessor: Video B H264 ROI region4, region5,region6,region7 QP register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`b_roi_region4_7_qp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`b_roi_region4_7_qp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@b_roi_region4_7_qp`] module"]
pub type B_ROI_REGION4_7_QP = crate::Reg<b_roi_region4_7_qp::B_ROI_REGION4_7_QP_SPEC>;
#[doc = "Video B H264 ROI region4, region5,region6,region7 QP register."]
pub mod b_roi_region4_7_qp;
#[doc = "B_NO_ROI_REGION_QP_OFFSET (rw) register accessor: Video B H264 no roi region QP register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`b_no_roi_region_qp_offset::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`b_no_roi_region_qp_offset::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@b_no_roi_region_qp_offset`] module"]
pub type B_NO_ROI_REGION_QP_OFFSET =
    crate::Reg<b_no_roi_region_qp_offset::B_NO_ROI_REGION_QP_OFFSET_SPEC>;
#[doc = "Video B H264 no roi region QP register."]
pub mod b_no_roi_region_qp_offset;
#[doc = "B_ROI_CONFIG (rw) register accessor: Video B H264 ROI configure register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`b_roi_config::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`b_roi_config::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@b_roi_config`] module"]
pub type B_ROI_CONFIG = crate::Reg<b_roi_config::B_ROI_CONFIG_SPEC>;
#[doc = "Video B H264 ROI configure register."]
pub mod b_roi_config;
#[doc = "RC_STATUS0 (r) register accessor: Rate control status register0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rc_status0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rc_status0`] module"]
pub type RC_STATUS0 = crate::Reg<rc_status0::RC_STATUS0_SPEC>;
#[doc = "Rate control status register0."]
pub mod rc_status0;
#[doc = "RC_STATUS1 (r) register accessor: Rate control status register1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rc_status1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rc_status1`] module"]
pub type RC_STATUS1 = crate::Reg<rc_status1::RC_STATUS1_SPEC>;
#[doc = "Rate control status register1."]
pub mod rc_status1;
#[doc = "RC_STATUS2 (r) register accessor: Rate control status register2.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rc_status2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rc_status2`] module"]
pub type RC_STATUS2 = crate::Reg<rc_status2::RC_STATUS2_SPEC>;
#[doc = "Rate control status register2."]
pub mod rc_status2;
#[doc = "SLICE_HEADER_REMAIN (rw) register accessor: Frame Slice Header remain bit register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slice_header_remain::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slice_header_remain::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slice_header_remain`] module"]
pub type SLICE_HEADER_REMAIN = crate::Reg<slice_header_remain::SLICE_HEADER_REMAIN_SPEC>;
#[doc = "Frame Slice Header remain bit register."]
pub mod slice_header_remain;
#[doc = "SLICE_HEADER_BYTE_LENGTH (rw) register accessor: Frame Slice Header byte length register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slice_header_byte_length::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slice_header_byte_length::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slice_header_byte_length`] module"]
pub type SLICE_HEADER_BYTE_LENGTH =
    crate::Reg<slice_header_byte_length::SLICE_HEADER_BYTE_LENGTH_SPEC>;
#[doc = "Frame Slice Header byte length register."]
pub mod slice_header_byte_length;
#[doc = "BS_THRESHOLD (rw) register accessor: Bitstream buffer overflow threshold register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bs_threshold::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bs_threshold::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bs_threshold`] module"]
pub type BS_THRESHOLD = crate::Reg<bs_threshold::BS_THRESHOLD_SPEC>;
#[doc = "Bitstream buffer overflow threshold register"]
pub mod bs_threshold;
#[doc = "SLICE_HEADER_BYTE0 (rw) register accessor: Frame Slice Header byte low 32 bit register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slice_header_byte0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slice_header_byte0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slice_header_byte0`] module"]
pub type SLICE_HEADER_BYTE0 = crate::Reg<slice_header_byte0::SLICE_HEADER_BYTE0_SPEC>;
#[doc = "Frame Slice Header byte low 32 bit register."]
pub mod slice_header_byte0;
#[doc = "SLICE_HEADER_BYTE1 (rw) register accessor: Frame Slice Header byte high 32 bit register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slice_header_byte1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slice_header_byte1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slice_header_byte1`] module"]
pub type SLICE_HEADER_BYTE1 = crate::Reg<slice_header_byte1::SLICE_HEADER_BYTE1_SPEC>;
#[doc = "Frame Slice Header byte high 32 bit register."]
pub mod slice_header_byte1;
#[doc = "INT_RAW (rw) register accessor: Interrupt raw status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_raw::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_raw::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_raw`] module"]
pub type INT_RAW = crate::Reg<int_raw::INT_RAW_SPEC>;
#[doc = "Interrupt raw status register"]
pub mod int_raw;
#[doc = "INT_ST (r) register accessor: Interrupt masked status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_st`] module"]
pub type INT_ST = crate::Reg<int_st::INT_ST_SPEC>;
#[doc = "Interrupt masked status register"]
pub mod int_st;
#[doc = "INT_ENA (rw) register accessor: Interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_ena`] module"]
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
#[doc = "Interrupt enable register"]
pub mod int_ena;
#[doc = "INT_CLR (w) register accessor: Interrupt clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_clr`] module"]
pub type INT_CLR = crate::Reg<int_clr::INT_CLR_SPEC>;
#[doc = "Interrupt clear register"]
pub mod int_clr;
#[doc = "CONF (rw) register accessor: General configuration register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@conf`] module"]
pub type CONF = crate::Reg<conf::CONF_SPEC>;
#[doc = "General configuration register."]
pub mod conf;
#[doc = "MV_MERGE_CONFIG (rw) register accessor: Mv merge configuration register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mv_merge_config::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mv_merge_config::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mv_merge_config`] module"]
pub type MV_MERGE_CONFIG = crate::Reg<mv_merge_config::MV_MERGE_CONFIG_SPEC>;
#[doc = "Mv merge configuration register."]
pub mod mv_merge_config;
#[doc = "DEBUG_DMA_SEL (rw) register accessor: Debug H264 DMA select register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`debug_dma_sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`debug_dma_sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@debug_dma_sel`] module"]
pub type DEBUG_DMA_SEL = crate::Reg<debug_dma_sel::DEBUG_DMA_SEL_SPEC>;
#[doc = "Debug H264 DMA select register"]
pub mod debug_dma_sel;
#[doc = "SYS_STATUS (r) register accessor: System status register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_status`] module"]
pub type SYS_STATUS = crate::Reg<sys_status::SYS_STATUS_SPEC>;
#[doc = "System status register."]
pub mod sys_status;
#[doc = "FRAME_CODE_LENGTH (r) register accessor: Frame code byte length register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`frame_code_length::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@frame_code_length`] module"]
pub type FRAME_CODE_LENGTH = crate::Reg<frame_code_length::FRAME_CODE_LENGTH_SPEC>;
#[doc = "Frame code byte length register."]
pub mod frame_code_length;
#[doc = "DEBUG_INFO0 (r) register accessor: Debug information register0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`debug_info0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@debug_info0`] module"]
pub type DEBUG_INFO0 = crate::Reg<debug_info0::DEBUG_INFO0_SPEC>;
#[doc = "Debug information register0."]
pub mod debug_info0;
#[doc = "DEBUG_INFO1 (r) register accessor: Debug information register1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`debug_info1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@debug_info1`] module"]
pub type DEBUG_INFO1 = crate::Reg<debug_info1::DEBUG_INFO1_SPEC>;
#[doc = "Debug information register1."]
pub mod debug_info1;
#[doc = "DEBUG_INFO2 (r) register accessor: Debug information register2.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`debug_info2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@debug_info2`] module"]
pub type DEBUG_INFO2 = crate::Reg<debug_info2::DEBUG_INFO2_SPEC>;
#[doc = "Debug information register2."]
pub mod debug_info2;
#[doc = "DATE (rw) register accessor: Version control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@date`] module"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "Version control register"]
pub mod date;
