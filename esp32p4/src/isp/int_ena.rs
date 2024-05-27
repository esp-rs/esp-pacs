#[doc = "Register `INT_ENA` reader"]
pub type R = crate::R<INT_ENA_SPEC>;
#[doc = "Register `INT_ENA` writer"]
pub type W = crate::W<INT_ENA_SPEC>;
#[doc = "Field `ISP_DATA_TYPE_ERR` reader - write 1 to enable input data type error"]
pub type ISP_DATA_TYPE_ERR_R = crate::BitReader;
#[doc = "Field `ISP_DATA_TYPE_ERR` writer - write 1 to enable input data type error"]
pub type ISP_DATA_TYPE_ERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ISP_ASYNC_FIFO_OVF` reader - write 1 to enable isp input fifo overflow"]
pub type ISP_ASYNC_FIFO_OVF_R = crate::BitReader;
#[doc = "Field `ISP_ASYNC_FIFO_OVF` writer - write 1 to enable isp input fifo overflow"]
pub type ISP_ASYNC_FIFO_OVF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ISP_BUF_FULL` reader - write 1 to enable isp input buffer full"]
pub type ISP_BUF_FULL_R = crate::BitReader;
#[doc = "Field `ISP_BUF_FULL` writer - write 1 to enable isp input buffer full"]
pub type ISP_BUF_FULL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ISP_HVNUM_SETTING_ERR` reader - write 1 to enable hnum and vnum setting format error"]
pub type ISP_HVNUM_SETTING_ERR_R = crate::BitReader;
#[doc = "Field `ISP_HVNUM_SETTING_ERR` writer - write 1 to enable hnum and vnum setting format error"]
pub type ISP_HVNUM_SETTING_ERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ISP_DATA_TYPE_SETTING_ERR` reader - write 1 to enable setting invalid reg_data_type"]
pub type ISP_DATA_TYPE_SETTING_ERR_R = crate::BitReader;
#[doc = "Field `ISP_DATA_TYPE_SETTING_ERR` writer - write 1 to enable setting invalid reg_data_type"]
pub type ISP_DATA_TYPE_SETTING_ERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ISP_MIPI_HNUM_UNMATCH` reader - write 1 to enable hnum setting unmatch with mipi input"]
pub type ISP_MIPI_HNUM_UNMATCH_R = crate::BitReader;
#[doc = "Field `ISP_MIPI_HNUM_UNMATCH` writer - write 1 to enable hnum setting unmatch with mipi input"]
pub type ISP_MIPI_HNUM_UNMATCH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DPC_CHECK_DONE` reader - write 1 to enable dpc check done"]
pub type DPC_CHECK_DONE_R = crate::BitReader;
#[doc = "Field `DPC_CHECK_DONE` writer - write 1 to enable dpc check done"]
pub type DPC_CHECK_DONE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GAMMA_XCOORD_ERR` reader - write 1 to enable gamma setting error"]
pub type GAMMA_XCOORD_ERR_R = crate::BitReader;
#[doc = "Field `GAMMA_XCOORD_ERR` writer - write 1 to enable gamma setting error"]
pub type GAMMA_XCOORD_ERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AE_MONITOR` reader - write 1 to enable ae monitor"]
pub type AE_MONITOR_R = crate::BitReader;
#[doc = "Field `AE_MONITOR` writer - write 1 to enable ae monitor"]
pub type AE_MONITOR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AE_FRAME_DONE` reader - write 1 to enable ae"]
pub type AE_FRAME_DONE_R = crate::BitReader;
#[doc = "Field `AE_FRAME_DONE` writer - write 1 to enable ae"]
pub type AE_FRAME_DONE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AF_FDONE` reader - write 1 to enable af statistic"]
pub type AF_FDONE_R = crate::BitReader;
#[doc = "Field `AF_FDONE` writer - write 1 to enable af statistic"]
pub type AF_FDONE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AF_ENV` reader - write 1 to enable af monitor"]
pub type AF_ENV_R = crate::BitReader;
#[doc = "Field `AF_ENV` writer - write 1 to enable af monitor"]
pub type AF_ENV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AWB_FDONE` reader - write 1 to enable awb"]
pub type AWB_FDONE_R = crate::BitReader;
#[doc = "Field `AWB_FDONE` writer - write 1 to enable awb"]
pub type AWB_FDONE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HIST_FDONE` reader - write 1 to enable histogram"]
pub type HIST_FDONE_R = crate::BitReader;
#[doc = "Field `HIST_FDONE` writer - write 1 to enable histogram"]
pub type HIST_FDONE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRAME` reader - write 1 to enable isp frame end"]
pub type FRAME_R = crate::BitReader;
#[doc = "Field `FRAME` writer - write 1 to enable isp frame end"]
pub type FRAME_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLC_FRAME` reader - write 1 to enable blc frame done"]
pub type BLC_FRAME_R = crate::BitReader;
#[doc = "Field `BLC_FRAME` writer - write 1 to enable blc frame done"]
pub type BLC_FRAME_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSC_FRAME` reader - write 1 to enable lsc frame done"]
pub type LSC_FRAME_R = crate::BitReader;
#[doc = "Field `LSC_FRAME` writer - write 1 to enable lsc frame done"]
pub type LSC_FRAME_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DPC_FRAME` reader - write 1 to enable dpc frame done"]
pub type DPC_FRAME_R = crate::BitReader;
#[doc = "Field `DPC_FRAME` writer - write 1 to enable dpc frame done"]
pub type DPC_FRAME_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BF_FRAME` reader - write 1 to enable bf frame done"]
pub type BF_FRAME_R = crate::BitReader;
#[doc = "Field `BF_FRAME` writer - write 1 to enable bf frame done"]
pub type BF_FRAME_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEMOSAIC_FRAME` reader - write 1 to enable demosaic frame done"]
pub type DEMOSAIC_FRAME_R = crate::BitReader;
#[doc = "Field `DEMOSAIC_FRAME` writer - write 1 to enable demosaic frame done"]
pub type DEMOSAIC_FRAME_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEDIAN_FRAME` reader - write 1 to enable median frame done"]
pub type MEDIAN_FRAME_R = crate::BitReader;
#[doc = "Field `MEDIAN_FRAME` writer - write 1 to enable median frame done"]
pub type MEDIAN_FRAME_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCM_FRAME` reader - write 1 to enable ccm frame done"]
pub type CCM_FRAME_R = crate::BitReader;
#[doc = "Field `CCM_FRAME` writer - write 1 to enable ccm frame done"]
pub type CCM_FRAME_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GAMMA_FRAME` reader - write 1 to enable gamma frame done"]
pub type GAMMA_FRAME_R = crate::BitReader;
#[doc = "Field `GAMMA_FRAME` writer - write 1 to enable gamma frame done"]
pub type GAMMA_FRAME_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RGB2YUV_FRAME` reader - write 1 to enable rgb2yuv frame done"]
pub type RGB2YUV_FRAME_R = crate::BitReader;
#[doc = "Field `RGB2YUV_FRAME` writer - write 1 to enable rgb2yuv frame done"]
pub type RGB2YUV_FRAME_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SHARP_FRAME` reader - write 1 to enable sharp frame done"]
pub type SHARP_FRAME_R = crate::BitReader;
#[doc = "Field `SHARP_FRAME` writer - write 1 to enable sharp frame done"]
pub type SHARP_FRAME_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COLOR_FRAME` reader - write 1 to enable color frame done"]
pub type COLOR_FRAME_R = crate::BitReader;
#[doc = "Field `COLOR_FRAME` writer - write 1 to enable color frame done"]
pub type COLOR_FRAME_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `YUV2RGB_FRAME` reader - write 1 to enable yuv2rgb frame done"]
pub type YUV2RGB_FRAME_R = crate::BitReader;
#[doc = "Field `YUV2RGB_FRAME` writer - write 1 to enable yuv2rgb frame done"]
pub type YUV2RGB_FRAME_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAIL_IDI_FRAME` reader - write 1 to enable isp_tail idi frame_end"]
pub type TAIL_IDI_FRAME_R = crate::BitReader;
#[doc = "Field `TAIL_IDI_FRAME` writer - write 1 to enable isp_tail idi frame_end"]
pub type TAIL_IDI_FRAME_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HEADER_IDI_FRAME` reader - write 1 to enable real input frame end of isp_input"]
pub type HEADER_IDI_FRAME_R = crate::BitReader;
#[doc = "Field `HEADER_IDI_FRAME` writer - write 1 to enable real input frame end of isp_input"]
pub type HEADER_IDI_FRAME_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - write 1 to enable input data type error"]
    #[inline(always)]
    pub fn isp_data_type_err(&self) -> ISP_DATA_TYPE_ERR_R {
        ISP_DATA_TYPE_ERR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - write 1 to enable isp input fifo overflow"]
    #[inline(always)]
    pub fn isp_async_fifo_ovf(&self) -> ISP_ASYNC_FIFO_OVF_R {
        ISP_ASYNC_FIFO_OVF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - write 1 to enable isp input buffer full"]
    #[inline(always)]
    pub fn isp_buf_full(&self) -> ISP_BUF_FULL_R {
        ISP_BUF_FULL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - write 1 to enable hnum and vnum setting format error"]
    #[inline(always)]
    pub fn isp_hvnum_setting_err(&self) -> ISP_HVNUM_SETTING_ERR_R {
        ISP_HVNUM_SETTING_ERR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - write 1 to enable setting invalid reg_data_type"]
    #[inline(always)]
    pub fn isp_data_type_setting_err(&self) -> ISP_DATA_TYPE_SETTING_ERR_R {
        ISP_DATA_TYPE_SETTING_ERR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - write 1 to enable hnum setting unmatch with mipi input"]
    #[inline(always)]
    pub fn isp_mipi_hnum_unmatch(&self) -> ISP_MIPI_HNUM_UNMATCH_R {
        ISP_MIPI_HNUM_UNMATCH_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - write 1 to enable dpc check done"]
    #[inline(always)]
    pub fn dpc_check_done(&self) -> DPC_CHECK_DONE_R {
        DPC_CHECK_DONE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - write 1 to enable gamma setting error"]
    #[inline(always)]
    pub fn gamma_xcoord_err(&self) -> GAMMA_XCOORD_ERR_R {
        GAMMA_XCOORD_ERR_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - write 1 to enable ae monitor"]
    #[inline(always)]
    pub fn ae_monitor(&self) -> AE_MONITOR_R {
        AE_MONITOR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - write 1 to enable ae"]
    #[inline(always)]
    pub fn ae_frame_done(&self) -> AE_FRAME_DONE_R {
        AE_FRAME_DONE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - write 1 to enable af statistic"]
    #[inline(always)]
    pub fn af_fdone(&self) -> AF_FDONE_R {
        AF_FDONE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - write 1 to enable af monitor"]
    #[inline(always)]
    pub fn af_env(&self) -> AF_ENV_R {
        AF_ENV_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - write 1 to enable awb"]
    #[inline(always)]
    pub fn awb_fdone(&self) -> AWB_FDONE_R {
        AWB_FDONE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - write 1 to enable histogram"]
    #[inline(always)]
    pub fn hist_fdone(&self) -> HIST_FDONE_R {
        HIST_FDONE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - write 1 to enable isp frame end"]
    #[inline(always)]
    pub fn frame(&self) -> FRAME_R {
        FRAME_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - write 1 to enable blc frame done"]
    #[inline(always)]
    pub fn blc_frame(&self) -> BLC_FRAME_R {
        BLC_FRAME_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - write 1 to enable lsc frame done"]
    #[inline(always)]
    pub fn lsc_frame(&self) -> LSC_FRAME_R {
        LSC_FRAME_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - write 1 to enable dpc frame done"]
    #[inline(always)]
    pub fn dpc_frame(&self) -> DPC_FRAME_R {
        DPC_FRAME_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - write 1 to enable bf frame done"]
    #[inline(always)]
    pub fn bf_frame(&self) -> BF_FRAME_R {
        BF_FRAME_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - write 1 to enable demosaic frame done"]
    #[inline(always)]
    pub fn demosaic_frame(&self) -> DEMOSAIC_FRAME_R {
        DEMOSAIC_FRAME_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - write 1 to enable median frame done"]
    #[inline(always)]
    pub fn median_frame(&self) -> MEDIAN_FRAME_R {
        MEDIAN_FRAME_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - write 1 to enable ccm frame done"]
    #[inline(always)]
    pub fn ccm_frame(&self) -> CCM_FRAME_R {
        CCM_FRAME_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - write 1 to enable gamma frame done"]
    #[inline(always)]
    pub fn gamma_frame(&self) -> GAMMA_FRAME_R {
        GAMMA_FRAME_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - write 1 to enable rgb2yuv frame done"]
    #[inline(always)]
    pub fn rgb2yuv_frame(&self) -> RGB2YUV_FRAME_R {
        RGB2YUV_FRAME_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - write 1 to enable sharp frame done"]
    #[inline(always)]
    pub fn sharp_frame(&self) -> SHARP_FRAME_R {
        SHARP_FRAME_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - write 1 to enable color frame done"]
    #[inline(always)]
    pub fn color_frame(&self) -> COLOR_FRAME_R {
        COLOR_FRAME_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - write 1 to enable yuv2rgb frame done"]
    #[inline(always)]
    pub fn yuv2rgb_frame(&self) -> YUV2RGB_FRAME_R {
        YUV2RGB_FRAME_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - write 1 to enable isp_tail idi frame_end"]
    #[inline(always)]
    pub fn tail_idi_frame(&self) -> TAIL_IDI_FRAME_R {
        TAIL_IDI_FRAME_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - write 1 to enable real input frame end of isp_input"]
    #[inline(always)]
    pub fn header_idi_frame(&self) -> HEADER_IDI_FRAME_R {
        HEADER_IDI_FRAME_R::new(((self.bits >> 28) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ENA")
            .field("isp_data_type_err", &self.isp_data_type_err())
            .field("isp_async_fifo_ovf", &self.isp_async_fifo_ovf())
            .field("isp_buf_full", &self.isp_buf_full())
            .field("isp_hvnum_setting_err", &self.isp_hvnum_setting_err())
            .field(
                "isp_data_type_setting_err",
                &self.isp_data_type_setting_err(),
            )
            .field("isp_mipi_hnum_unmatch", &self.isp_mipi_hnum_unmatch())
            .field("dpc_check_done", &self.dpc_check_done())
            .field("gamma_xcoord_err", &self.gamma_xcoord_err())
            .field("ae_monitor", &self.ae_monitor())
            .field("ae_frame_done", &self.ae_frame_done())
            .field("af_fdone", &self.af_fdone())
            .field("af_env", &self.af_env())
            .field("awb_fdone", &self.awb_fdone())
            .field("hist_fdone", &self.hist_fdone())
            .field("frame", &self.frame())
            .field("blc_frame", &self.blc_frame())
            .field("lsc_frame", &self.lsc_frame())
            .field("dpc_frame", &self.dpc_frame())
            .field("bf_frame", &self.bf_frame())
            .field("demosaic_frame", &self.demosaic_frame())
            .field("median_frame", &self.median_frame())
            .field("ccm_frame", &self.ccm_frame())
            .field("gamma_frame", &self.gamma_frame())
            .field("rgb2yuv_frame", &self.rgb2yuv_frame())
            .field("sharp_frame", &self.sharp_frame())
            .field("color_frame", &self.color_frame())
            .field("yuv2rgb_frame", &self.yuv2rgb_frame())
            .field("tail_idi_frame", &self.tail_idi_frame())
            .field("header_idi_frame", &self.header_idi_frame())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - write 1 to enable input data type error"]
    #[inline(always)]
    #[must_use]
    pub fn isp_data_type_err(&mut self) -> ISP_DATA_TYPE_ERR_W<INT_ENA_SPEC> {
        ISP_DATA_TYPE_ERR_W::new(self, 0)
    }
    #[doc = "Bit 1 - write 1 to enable isp input fifo overflow"]
    #[inline(always)]
    #[must_use]
    pub fn isp_async_fifo_ovf(&mut self) -> ISP_ASYNC_FIFO_OVF_W<INT_ENA_SPEC> {
        ISP_ASYNC_FIFO_OVF_W::new(self, 1)
    }
    #[doc = "Bit 2 - write 1 to enable isp input buffer full"]
    #[inline(always)]
    #[must_use]
    pub fn isp_buf_full(&mut self) -> ISP_BUF_FULL_W<INT_ENA_SPEC> {
        ISP_BUF_FULL_W::new(self, 2)
    }
    #[doc = "Bit 3 - write 1 to enable hnum and vnum setting format error"]
    #[inline(always)]
    #[must_use]
    pub fn isp_hvnum_setting_err(&mut self) -> ISP_HVNUM_SETTING_ERR_W<INT_ENA_SPEC> {
        ISP_HVNUM_SETTING_ERR_W::new(self, 3)
    }
    #[doc = "Bit 4 - write 1 to enable setting invalid reg_data_type"]
    #[inline(always)]
    #[must_use]
    pub fn isp_data_type_setting_err(&mut self) -> ISP_DATA_TYPE_SETTING_ERR_W<INT_ENA_SPEC> {
        ISP_DATA_TYPE_SETTING_ERR_W::new(self, 4)
    }
    #[doc = "Bit 5 - write 1 to enable hnum setting unmatch with mipi input"]
    #[inline(always)]
    #[must_use]
    pub fn isp_mipi_hnum_unmatch(&mut self) -> ISP_MIPI_HNUM_UNMATCH_W<INT_ENA_SPEC> {
        ISP_MIPI_HNUM_UNMATCH_W::new(self, 5)
    }
    #[doc = "Bit 6 - write 1 to enable dpc check done"]
    #[inline(always)]
    #[must_use]
    pub fn dpc_check_done(&mut self) -> DPC_CHECK_DONE_W<INT_ENA_SPEC> {
        DPC_CHECK_DONE_W::new(self, 6)
    }
    #[doc = "Bit 7 - write 1 to enable gamma setting error"]
    #[inline(always)]
    #[must_use]
    pub fn gamma_xcoord_err(&mut self) -> GAMMA_XCOORD_ERR_W<INT_ENA_SPEC> {
        GAMMA_XCOORD_ERR_W::new(self, 7)
    }
    #[doc = "Bit 8 - write 1 to enable ae monitor"]
    #[inline(always)]
    #[must_use]
    pub fn ae_monitor(&mut self) -> AE_MONITOR_W<INT_ENA_SPEC> {
        AE_MONITOR_W::new(self, 8)
    }
    #[doc = "Bit 9 - write 1 to enable ae"]
    #[inline(always)]
    #[must_use]
    pub fn ae_frame_done(&mut self) -> AE_FRAME_DONE_W<INT_ENA_SPEC> {
        AE_FRAME_DONE_W::new(self, 9)
    }
    #[doc = "Bit 10 - write 1 to enable af statistic"]
    #[inline(always)]
    #[must_use]
    pub fn af_fdone(&mut self) -> AF_FDONE_W<INT_ENA_SPEC> {
        AF_FDONE_W::new(self, 10)
    }
    #[doc = "Bit 11 - write 1 to enable af monitor"]
    #[inline(always)]
    #[must_use]
    pub fn af_env(&mut self) -> AF_ENV_W<INT_ENA_SPEC> {
        AF_ENV_W::new(self, 11)
    }
    #[doc = "Bit 12 - write 1 to enable awb"]
    #[inline(always)]
    #[must_use]
    pub fn awb_fdone(&mut self) -> AWB_FDONE_W<INT_ENA_SPEC> {
        AWB_FDONE_W::new(self, 12)
    }
    #[doc = "Bit 13 - write 1 to enable histogram"]
    #[inline(always)]
    #[must_use]
    pub fn hist_fdone(&mut self) -> HIST_FDONE_W<INT_ENA_SPEC> {
        HIST_FDONE_W::new(self, 13)
    }
    #[doc = "Bit 14 - write 1 to enable isp frame end"]
    #[inline(always)]
    #[must_use]
    pub fn frame(&mut self) -> FRAME_W<INT_ENA_SPEC> {
        FRAME_W::new(self, 14)
    }
    #[doc = "Bit 15 - write 1 to enable blc frame done"]
    #[inline(always)]
    #[must_use]
    pub fn blc_frame(&mut self) -> BLC_FRAME_W<INT_ENA_SPEC> {
        BLC_FRAME_W::new(self, 15)
    }
    #[doc = "Bit 16 - write 1 to enable lsc frame done"]
    #[inline(always)]
    #[must_use]
    pub fn lsc_frame(&mut self) -> LSC_FRAME_W<INT_ENA_SPEC> {
        LSC_FRAME_W::new(self, 16)
    }
    #[doc = "Bit 17 - write 1 to enable dpc frame done"]
    #[inline(always)]
    #[must_use]
    pub fn dpc_frame(&mut self) -> DPC_FRAME_W<INT_ENA_SPEC> {
        DPC_FRAME_W::new(self, 17)
    }
    #[doc = "Bit 18 - write 1 to enable bf frame done"]
    #[inline(always)]
    #[must_use]
    pub fn bf_frame(&mut self) -> BF_FRAME_W<INT_ENA_SPEC> {
        BF_FRAME_W::new(self, 18)
    }
    #[doc = "Bit 19 - write 1 to enable demosaic frame done"]
    #[inline(always)]
    #[must_use]
    pub fn demosaic_frame(&mut self) -> DEMOSAIC_FRAME_W<INT_ENA_SPEC> {
        DEMOSAIC_FRAME_W::new(self, 19)
    }
    #[doc = "Bit 20 - write 1 to enable median frame done"]
    #[inline(always)]
    #[must_use]
    pub fn median_frame(&mut self) -> MEDIAN_FRAME_W<INT_ENA_SPEC> {
        MEDIAN_FRAME_W::new(self, 20)
    }
    #[doc = "Bit 21 - write 1 to enable ccm frame done"]
    #[inline(always)]
    #[must_use]
    pub fn ccm_frame(&mut self) -> CCM_FRAME_W<INT_ENA_SPEC> {
        CCM_FRAME_W::new(self, 21)
    }
    #[doc = "Bit 22 - write 1 to enable gamma frame done"]
    #[inline(always)]
    #[must_use]
    pub fn gamma_frame(&mut self) -> GAMMA_FRAME_W<INT_ENA_SPEC> {
        GAMMA_FRAME_W::new(self, 22)
    }
    #[doc = "Bit 23 - write 1 to enable rgb2yuv frame done"]
    #[inline(always)]
    #[must_use]
    pub fn rgb2yuv_frame(&mut self) -> RGB2YUV_FRAME_W<INT_ENA_SPEC> {
        RGB2YUV_FRAME_W::new(self, 23)
    }
    #[doc = "Bit 24 - write 1 to enable sharp frame done"]
    #[inline(always)]
    #[must_use]
    pub fn sharp_frame(&mut self) -> SHARP_FRAME_W<INT_ENA_SPEC> {
        SHARP_FRAME_W::new(self, 24)
    }
    #[doc = "Bit 25 - write 1 to enable color frame done"]
    #[inline(always)]
    #[must_use]
    pub fn color_frame(&mut self) -> COLOR_FRAME_W<INT_ENA_SPEC> {
        COLOR_FRAME_W::new(self, 25)
    }
    #[doc = "Bit 26 - write 1 to enable yuv2rgb frame done"]
    #[inline(always)]
    #[must_use]
    pub fn yuv2rgb_frame(&mut self) -> YUV2RGB_FRAME_W<INT_ENA_SPEC> {
        YUV2RGB_FRAME_W::new(self, 26)
    }
    #[doc = "Bit 27 - write 1 to enable isp_tail idi frame_end"]
    #[inline(always)]
    #[must_use]
    pub fn tail_idi_frame(&mut self) -> TAIL_IDI_FRAME_W<INT_ENA_SPEC> {
        TAIL_IDI_FRAME_W::new(self, 27)
    }
    #[doc = "Bit 28 - write 1 to enable real input frame end of isp_input"]
    #[inline(always)]
    #[must_use]
    pub fn header_idi_frame(&mut self) -> HEADER_IDI_FRAME_W<INT_ENA_SPEC> {
        HEADER_IDI_FRAME_W::new(self, 28)
    }
}
#[doc = "interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_ena::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ena::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_ENA_SPEC;
impl crate::RegisterSpec for INT_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_ena::R`](R) reader structure"]
impl crate::Readable for INT_ENA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`int_ena::W`](W) writer structure"]
impl crate::Writable for INT_ENA_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT_ENA to value 0xc3"]
impl crate::Resettable for INT_ENA_SPEC {
    const RESET_VALUE: u32 = 0xc3;
}
