#[doc = "Register `INT_RAW` reader"]
pub type R = crate::R<INT_RAW_SPEC>;
#[doc = "Field `ISP_DATA_TYPE_ERR` reader - the raw interrupt status of input data type error. isp only support RGB bayer data type, other type will report type_err_int"]
pub type ISP_DATA_TYPE_ERR_R = crate::BitReader;
#[doc = "Field `ISP_ASYNC_FIFO_OVF` reader - the raw interrupt status of isp input fifo overflow"]
pub type ISP_ASYNC_FIFO_OVF_R = crate::BitReader;
#[doc = "Field `ISP_BUF_FULL` reader - the raw interrupt status of isp input buffer full"]
pub type ISP_BUF_FULL_R = crate::BitReader;
#[doc = "Field `ISP_HVNUM_SETTING_ERR` reader - the raw interrupt status of hnum and vnum setting format error"]
pub type ISP_HVNUM_SETTING_ERR_R = crate::BitReader;
#[doc = "Field `ISP_DATA_TYPE_SETTING_ERR` reader - the raw interrupt status of setting invalid reg_data_type"]
pub type ISP_DATA_TYPE_SETTING_ERR_R = crate::BitReader;
#[doc = "Field `ISP_MIPI_HNUM_UNMATCH` reader - the raw interrupt status of hnum setting unmatch with mipi input"]
pub type ISP_MIPI_HNUM_UNMATCH_R = crate::BitReader;
#[doc = "Field `DPC_CHECK_DONE` reader - the raw interrupt status of dpc check done"]
pub type DPC_CHECK_DONE_R = crate::BitReader;
#[doc = "Field `GAMMA_XCOORD_ERR` reader - the raw interrupt status of gamma setting error. it report the sum of the lengths represented by reg_gamma_x00~x0F isn't equal to 256"]
pub type GAMMA_XCOORD_ERR_R = crate::BitReader;
#[doc = "Field `AE_MONITOR` reader - the raw interrupt status of ae monitor"]
pub type AE_MONITOR_R = crate::BitReader;
#[doc = "Field `AE_FRAME_DONE` reader - the raw interrupt status of ae."]
pub type AE_FRAME_DONE_R = crate::BitReader;
#[doc = "Field `AF_FDONE` reader - the raw interrupt status of af statistic. when auto_update enable, each frame done will send one int pulse when manual_update, each time when write 1 to reg_manual_update will send a int pulse when next frame done"]
pub type AF_FDONE_R = crate::BitReader;
#[doc = "Field `AF_ENV` reader - the raw interrupt status of af monitor. send a int pulse when env_det function enabled and environment changes detected"]
pub type AF_ENV_R = crate::BitReader;
#[doc = "Field `AWB_FDONE` reader - the raw interrupt status of awb. send a int pulse when statistic of one awb frame done"]
pub type AWB_FDONE_R = crate::BitReader;
#[doc = "Field `HIST_FDONE` reader - the raw interrupt status of histogram. send a int pulse when statistic of one frame histogram done"]
pub type HIST_FDONE_R = crate::BitReader;
#[doc = "Field `FRAME` reader - the raw interrupt status of isp frame end"]
pub type FRAME_R = crate::BitReader;
#[doc = "Field `BLC_FRAME` reader - the raw interrupt status of blc frame done"]
pub type BLC_FRAME_R = crate::BitReader;
#[doc = "Field `LSC_FRAME` reader - the raw interrupt status of lsc frame done"]
pub type LSC_FRAME_R = crate::BitReader;
#[doc = "Field `DPC_FRAME` reader - the raw interrupt status of dpc frame done"]
pub type DPC_FRAME_R = crate::BitReader;
#[doc = "Field `BF_FRAME` reader - the raw interrupt status of bf frame done"]
pub type BF_FRAME_R = crate::BitReader;
#[doc = "Field `DEMOSAIC_FRAME` reader - the raw interrupt status of demosaic frame done"]
pub type DEMOSAIC_FRAME_R = crate::BitReader;
#[doc = "Field `MEDIAN_FRAME` reader - the raw interrupt status of median frame done"]
pub type MEDIAN_FRAME_R = crate::BitReader;
#[doc = "Field `CCM_FRAME` reader - the raw interrupt status of ccm frame done"]
pub type CCM_FRAME_R = crate::BitReader;
#[doc = "Field `GAMMA_FRAME` reader - the raw interrupt status of gamma frame done"]
pub type GAMMA_FRAME_R = crate::BitReader;
#[doc = "Field `RGB2YUV_FRAME` reader - the raw interrupt status of rgb2yuv frame done"]
pub type RGB2YUV_FRAME_R = crate::BitReader;
#[doc = "Field `SHARP_FRAME` reader - the raw interrupt status of sharp frame done"]
pub type SHARP_FRAME_R = crate::BitReader;
#[doc = "Field `COLOR_FRAME` reader - the raw interrupt status of color frame done"]
pub type COLOR_FRAME_R = crate::BitReader;
#[doc = "Field `YUV2RGB_FRAME` reader - the raw interrupt status of yuv2rgb frame done"]
pub type YUV2RGB_FRAME_R = crate::BitReader;
#[doc = "Field `TAIL_IDI_FRAME` reader - the raw interrupt status of isp_tail idi frame_end"]
pub type TAIL_IDI_FRAME_R = crate::BitReader;
#[doc = "Field `HEADER_IDI_FRAME` reader - the raw interrupt status of real input frame end of isp_input"]
pub type HEADER_IDI_FRAME_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - the raw interrupt status of input data type error. isp only support RGB bayer data type, other type will report type_err_int"]
    #[inline(always)]
    pub fn isp_data_type_err(&self) -> ISP_DATA_TYPE_ERR_R {
        ISP_DATA_TYPE_ERR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - the raw interrupt status of isp input fifo overflow"]
    #[inline(always)]
    pub fn isp_async_fifo_ovf(&self) -> ISP_ASYNC_FIFO_OVF_R {
        ISP_ASYNC_FIFO_OVF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - the raw interrupt status of isp input buffer full"]
    #[inline(always)]
    pub fn isp_buf_full(&self) -> ISP_BUF_FULL_R {
        ISP_BUF_FULL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - the raw interrupt status of hnum and vnum setting format error"]
    #[inline(always)]
    pub fn isp_hvnum_setting_err(&self) -> ISP_HVNUM_SETTING_ERR_R {
        ISP_HVNUM_SETTING_ERR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - the raw interrupt status of setting invalid reg_data_type"]
    #[inline(always)]
    pub fn isp_data_type_setting_err(&self) -> ISP_DATA_TYPE_SETTING_ERR_R {
        ISP_DATA_TYPE_SETTING_ERR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - the raw interrupt status of hnum setting unmatch with mipi input"]
    #[inline(always)]
    pub fn isp_mipi_hnum_unmatch(&self) -> ISP_MIPI_HNUM_UNMATCH_R {
        ISP_MIPI_HNUM_UNMATCH_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - the raw interrupt status of dpc check done"]
    #[inline(always)]
    pub fn dpc_check_done(&self) -> DPC_CHECK_DONE_R {
        DPC_CHECK_DONE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - the raw interrupt status of gamma setting error. it report the sum of the lengths represented by reg_gamma_x00~x0F isn't equal to 256"]
    #[inline(always)]
    pub fn gamma_xcoord_err(&self) -> GAMMA_XCOORD_ERR_R {
        GAMMA_XCOORD_ERR_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - the raw interrupt status of ae monitor"]
    #[inline(always)]
    pub fn ae_monitor(&self) -> AE_MONITOR_R {
        AE_MONITOR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - the raw interrupt status of ae."]
    #[inline(always)]
    pub fn ae_frame_done(&self) -> AE_FRAME_DONE_R {
        AE_FRAME_DONE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - the raw interrupt status of af statistic. when auto_update enable, each frame done will send one int pulse when manual_update, each time when write 1 to reg_manual_update will send a int pulse when next frame done"]
    #[inline(always)]
    pub fn af_fdone(&self) -> AF_FDONE_R {
        AF_FDONE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - the raw interrupt status of af monitor. send a int pulse when env_det function enabled and environment changes detected"]
    #[inline(always)]
    pub fn af_env(&self) -> AF_ENV_R {
        AF_ENV_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - the raw interrupt status of awb. send a int pulse when statistic of one awb frame done"]
    #[inline(always)]
    pub fn awb_fdone(&self) -> AWB_FDONE_R {
        AWB_FDONE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - the raw interrupt status of histogram. send a int pulse when statistic of one frame histogram done"]
    #[inline(always)]
    pub fn hist_fdone(&self) -> HIST_FDONE_R {
        HIST_FDONE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - the raw interrupt status of isp frame end"]
    #[inline(always)]
    pub fn frame(&self) -> FRAME_R {
        FRAME_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - the raw interrupt status of blc frame done"]
    #[inline(always)]
    pub fn blc_frame(&self) -> BLC_FRAME_R {
        BLC_FRAME_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - the raw interrupt status of lsc frame done"]
    #[inline(always)]
    pub fn lsc_frame(&self) -> LSC_FRAME_R {
        LSC_FRAME_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - the raw interrupt status of dpc frame done"]
    #[inline(always)]
    pub fn dpc_frame(&self) -> DPC_FRAME_R {
        DPC_FRAME_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - the raw interrupt status of bf frame done"]
    #[inline(always)]
    pub fn bf_frame(&self) -> BF_FRAME_R {
        BF_FRAME_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - the raw interrupt status of demosaic frame done"]
    #[inline(always)]
    pub fn demosaic_frame(&self) -> DEMOSAIC_FRAME_R {
        DEMOSAIC_FRAME_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - the raw interrupt status of median frame done"]
    #[inline(always)]
    pub fn median_frame(&self) -> MEDIAN_FRAME_R {
        MEDIAN_FRAME_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - the raw interrupt status of ccm frame done"]
    #[inline(always)]
    pub fn ccm_frame(&self) -> CCM_FRAME_R {
        CCM_FRAME_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - the raw interrupt status of gamma frame done"]
    #[inline(always)]
    pub fn gamma_frame(&self) -> GAMMA_FRAME_R {
        GAMMA_FRAME_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - the raw interrupt status of rgb2yuv frame done"]
    #[inline(always)]
    pub fn rgb2yuv_frame(&self) -> RGB2YUV_FRAME_R {
        RGB2YUV_FRAME_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - the raw interrupt status of sharp frame done"]
    #[inline(always)]
    pub fn sharp_frame(&self) -> SHARP_FRAME_R {
        SHARP_FRAME_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - the raw interrupt status of color frame done"]
    #[inline(always)]
    pub fn color_frame(&self) -> COLOR_FRAME_R {
        COLOR_FRAME_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - the raw interrupt status of yuv2rgb frame done"]
    #[inline(always)]
    pub fn yuv2rgb_frame(&self) -> YUV2RGB_FRAME_R {
        YUV2RGB_FRAME_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - the raw interrupt status of isp_tail idi frame_end"]
    #[inline(always)]
    pub fn tail_idi_frame(&self) -> TAIL_IDI_FRAME_R {
        TAIL_IDI_FRAME_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - the raw interrupt status of real input frame end of isp_input"]
    #[inline(always)]
    pub fn header_idi_frame(&self) -> HEADER_IDI_FRAME_R {
        HEADER_IDI_FRAME_R::new(((self.bits >> 28) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_RAW")
            .field(
                "isp_data_type_err",
                &format_args!("{}", self.isp_data_type_err().bit()),
            )
            .field(
                "isp_async_fifo_ovf",
                &format_args!("{}", self.isp_async_fifo_ovf().bit()),
            )
            .field(
                "isp_buf_full",
                &format_args!("{}", self.isp_buf_full().bit()),
            )
            .field(
                "isp_hvnum_setting_err",
                &format_args!("{}", self.isp_hvnum_setting_err().bit()),
            )
            .field(
                "isp_data_type_setting_err",
                &format_args!("{}", self.isp_data_type_setting_err().bit()),
            )
            .field(
                "isp_mipi_hnum_unmatch",
                &format_args!("{}", self.isp_mipi_hnum_unmatch().bit()),
            )
            .field(
                "dpc_check_done",
                &format_args!("{}", self.dpc_check_done().bit()),
            )
            .field(
                "gamma_xcoord_err",
                &format_args!("{}", self.gamma_xcoord_err().bit()),
            )
            .field("ae_monitor", &format_args!("{}", self.ae_monitor().bit()))
            .field(
                "ae_frame_done",
                &format_args!("{}", self.ae_frame_done().bit()),
            )
            .field("af_fdone", &format_args!("{}", self.af_fdone().bit()))
            .field("af_env", &format_args!("{}", self.af_env().bit()))
            .field("awb_fdone", &format_args!("{}", self.awb_fdone().bit()))
            .field("hist_fdone", &format_args!("{}", self.hist_fdone().bit()))
            .field("frame", &format_args!("{}", self.frame().bit()))
            .field("blc_frame", &format_args!("{}", self.blc_frame().bit()))
            .field("lsc_frame", &format_args!("{}", self.lsc_frame().bit()))
            .field("dpc_frame", &format_args!("{}", self.dpc_frame().bit()))
            .field("bf_frame", &format_args!("{}", self.bf_frame().bit()))
            .field(
                "demosaic_frame",
                &format_args!("{}", self.demosaic_frame().bit()),
            )
            .field(
                "median_frame",
                &format_args!("{}", self.median_frame().bit()),
            )
            .field("ccm_frame", &format_args!("{}", self.ccm_frame().bit()))
            .field("gamma_frame", &format_args!("{}", self.gamma_frame().bit()))
            .field(
                "rgb2yuv_frame",
                &format_args!("{}", self.rgb2yuv_frame().bit()),
            )
            .field("sharp_frame", &format_args!("{}", self.sharp_frame().bit()))
            .field("color_frame", &format_args!("{}", self.color_frame().bit()))
            .field(
                "yuv2rgb_frame",
                &format_args!("{}", self.yuv2rgb_frame().bit()),
            )
            .field(
                "tail_idi_frame",
                &format_args!("{}", self.tail_idi_frame().bit()),
            )
            .field(
                "header_idi_frame",
                &format_args!("{}", self.header_idi_frame().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_RAW_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "raw interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_raw::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_RAW_SPEC;
impl crate::RegisterSpec for INT_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_raw::R`](R) reader structure"]
impl crate::Readable for INT_RAW_SPEC {}
#[doc = "`reset()` method sets INT_RAW to value 0"]
impl crate::Resettable for INT_RAW_SPEC {
    const RESET_VALUE: u32 = 0;
}
