#[doc = "Register `INT_ENA` reader"]
pub type R = crate::R<INT_ENA_SPEC>;
#[doc = "Register `INT_ENA` writer"]
pub type W = crate::W<INT_ENA_SPEC>;
#[doc = "Field `ISP_DATA_TYPE_ERR_INT_ENA` reader - write 1 to enable input data type error"]
pub type ISP_DATA_TYPE_ERR_INT_ENA_R = crate::BitReader;
#[doc = "Field `ISP_DATA_TYPE_ERR_INT_ENA` writer - write 1 to enable input data type error"]
pub type ISP_DATA_TYPE_ERR_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ISP_ASYNC_FIFO_OVF_INT_ENA` reader - write 1 to enable isp input fifo overflow"]
pub type ISP_ASYNC_FIFO_OVF_INT_ENA_R = crate::BitReader;
#[doc = "Field `ISP_ASYNC_FIFO_OVF_INT_ENA` writer - write 1 to enable isp input fifo overflow"]
pub type ISP_ASYNC_FIFO_OVF_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ISP_BUF_FULL_INT_ENA` reader - write 1 to enable isp input buffer full"]
pub type ISP_BUF_FULL_INT_ENA_R = crate::BitReader;
#[doc = "Field `ISP_BUF_FULL_INT_ENA` writer - write 1 to enable isp input buffer full"]
pub type ISP_BUF_FULL_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ISP_HVNUM_SETTING_ERR_INT_ENA` reader - write 1 to enable hnum and vnum setting format error"]
pub type ISP_HVNUM_SETTING_ERR_INT_ENA_R = crate::BitReader;
#[doc = "Field `ISP_HVNUM_SETTING_ERR_INT_ENA` writer - write 1 to enable hnum and vnum setting format error"]
pub type ISP_HVNUM_SETTING_ERR_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ISP_DATA_TYPE_SETTING_ERR_INT_ENA` reader - write 1 to enable setting invalid reg_data_type"]
pub type ISP_DATA_TYPE_SETTING_ERR_INT_ENA_R = crate::BitReader;
#[doc = "Field `ISP_DATA_TYPE_SETTING_ERR_INT_ENA` writer - write 1 to enable setting invalid reg_data_type"]
pub type ISP_DATA_TYPE_SETTING_ERR_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ISP_MIPI_HNUM_UNMATCH_INT_ENA` reader - write 1 to enable hnum setting unmatch with mipi input"]
pub type ISP_MIPI_HNUM_UNMATCH_INT_ENA_R = crate::BitReader;
#[doc = "Field `ISP_MIPI_HNUM_UNMATCH_INT_ENA` writer - write 1 to enable hnum setting unmatch with mipi input"]
pub type ISP_MIPI_HNUM_UNMATCH_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DPC_CHECK_DONE_INT_ENA` reader - write 1 to enable dpc check done"]
pub type DPC_CHECK_DONE_INT_ENA_R = crate::BitReader;
#[doc = "Field `DPC_CHECK_DONE_INT_ENA` writer - write 1 to enable dpc check done"]
pub type DPC_CHECK_DONE_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GAMMA_XCOORD_ERR_INT_ENA` reader - write 1 to enable gamma setting error"]
pub type GAMMA_XCOORD_ERR_INT_ENA_R = crate::BitReader;
#[doc = "Field `GAMMA_XCOORD_ERR_INT_ENA` writer - write 1 to enable gamma setting error"]
pub type GAMMA_XCOORD_ERR_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AE_MONITOR_INT_ENA` reader - write 1 to enable ae monitor"]
pub type AE_MONITOR_INT_ENA_R = crate::BitReader;
#[doc = "Field `AE_MONITOR_INT_ENA` writer - write 1 to enable ae monitor"]
pub type AE_MONITOR_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AE_FRAME_DONE_INT_ENA` reader - write 1 to enable ae"]
pub type AE_FRAME_DONE_INT_ENA_R = crate::BitReader;
#[doc = "Field `AE_FRAME_DONE_INT_ENA` writer - write 1 to enable ae"]
pub type AE_FRAME_DONE_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AF_FDONE_INT_ENA` reader - write 1 to enable af statistic"]
pub type AF_FDONE_INT_ENA_R = crate::BitReader;
#[doc = "Field `AF_FDONE_INT_ENA` writer - write 1 to enable af statistic"]
pub type AF_FDONE_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AF_ENV_INT_ENA` reader - write 1 to enable af monitor"]
pub type AF_ENV_INT_ENA_R = crate::BitReader;
#[doc = "Field `AF_ENV_INT_ENA` writer - write 1 to enable af monitor"]
pub type AF_ENV_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AWB_FDONE_INT_ENA` reader - write 1 to enable awb"]
pub type AWB_FDONE_INT_ENA_R = crate::BitReader;
#[doc = "Field `AWB_FDONE_INT_ENA` writer - write 1 to enable awb"]
pub type AWB_FDONE_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HIST_FDONE_INT_ENA` reader - write 1 to enable histogram"]
pub type HIST_FDONE_INT_ENA_R = crate::BitReader;
#[doc = "Field `HIST_FDONE_INT_ENA` writer - write 1 to enable histogram"]
pub type HIST_FDONE_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRAME_INT_ENA` reader - write 1 to enable isp frame end"]
pub type FRAME_INT_ENA_R = crate::BitReader;
#[doc = "Field `FRAME_INT_ENA` writer - write 1 to enable isp frame end"]
pub type FRAME_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLC_FRAME_INT_ENA` reader - write 1 to enable blc frame done"]
pub type BLC_FRAME_INT_ENA_R = crate::BitReader;
#[doc = "Field `BLC_FRAME_INT_ENA` writer - write 1 to enable blc frame done"]
pub type BLC_FRAME_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSC_FRAME_INT_ENA` reader - write 1 to enable lsc frame done"]
pub type LSC_FRAME_INT_ENA_R = crate::BitReader;
#[doc = "Field `LSC_FRAME_INT_ENA` writer - write 1 to enable lsc frame done"]
pub type LSC_FRAME_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DPC_FRAME_INT_ENA` reader - write 1 to enable dpc frame done"]
pub type DPC_FRAME_INT_ENA_R = crate::BitReader;
#[doc = "Field `DPC_FRAME_INT_ENA` writer - write 1 to enable dpc frame done"]
pub type DPC_FRAME_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BF_FRAME_INT_ENA` reader - write 1 to enable bf frame done"]
pub type BF_FRAME_INT_ENA_R = crate::BitReader;
#[doc = "Field `BF_FRAME_INT_ENA` writer - write 1 to enable bf frame done"]
pub type BF_FRAME_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEMOSAIC_FRAME_INT_ENA` reader - write 1 to enable demosaic frame done"]
pub type DEMOSAIC_FRAME_INT_ENA_R = crate::BitReader;
#[doc = "Field `DEMOSAIC_FRAME_INT_ENA` writer - write 1 to enable demosaic frame done"]
pub type DEMOSAIC_FRAME_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEDIAN_FRAME_INT_ENA` reader - write 1 to enable median frame done"]
pub type MEDIAN_FRAME_INT_ENA_R = crate::BitReader;
#[doc = "Field `MEDIAN_FRAME_INT_ENA` writer - write 1 to enable median frame done"]
pub type MEDIAN_FRAME_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCM_FRAME_INT_ENA` reader - write 1 to enable ccm frame done"]
pub type CCM_FRAME_INT_ENA_R = crate::BitReader;
#[doc = "Field `CCM_FRAME_INT_ENA` writer - write 1 to enable ccm frame done"]
pub type CCM_FRAME_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GAMMA_FRAME_INT_ENA` reader - write 1 to enable gamma frame done"]
pub type GAMMA_FRAME_INT_ENA_R = crate::BitReader;
#[doc = "Field `GAMMA_FRAME_INT_ENA` writer - write 1 to enable gamma frame done"]
pub type GAMMA_FRAME_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RGB2YUV_FRAME_INT_ENA` reader - write 1 to enable rgb2yuv frame done"]
pub type RGB2YUV_FRAME_INT_ENA_R = crate::BitReader;
#[doc = "Field `RGB2YUV_FRAME_INT_ENA` writer - write 1 to enable rgb2yuv frame done"]
pub type RGB2YUV_FRAME_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SHARP_FRAME_INT_ENA` reader - write 1 to enable sharp frame done"]
pub type SHARP_FRAME_INT_ENA_R = crate::BitReader;
#[doc = "Field `SHARP_FRAME_INT_ENA` writer - write 1 to enable sharp frame done"]
pub type SHARP_FRAME_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COLOR_FRAME_INT_ENA` reader - write 1 to enable color frame done"]
pub type COLOR_FRAME_INT_ENA_R = crate::BitReader;
#[doc = "Field `COLOR_FRAME_INT_ENA` writer - write 1 to enable color frame done"]
pub type COLOR_FRAME_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `YUV2RGB_FRAME_INT_ENA` reader - write 1 to enable yuv2rgb frame done"]
pub type YUV2RGB_FRAME_INT_ENA_R = crate::BitReader;
#[doc = "Field `YUV2RGB_FRAME_INT_ENA` writer - write 1 to enable yuv2rgb frame done"]
pub type YUV2RGB_FRAME_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAIL_IDI_FRAME_INT_ENA` reader - write 1 to enable isp_tail idi frame_end"]
pub type TAIL_IDI_FRAME_INT_ENA_R = crate::BitReader;
#[doc = "Field `TAIL_IDI_FRAME_INT_ENA` writer - write 1 to enable isp_tail idi frame_end"]
pub type TAIL_IDI_FRAME_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HEADER_IDI_FRAME_INT_ENA` reader - write 1 to enable real input frame end of isp_input"]
pub type HEADER_IDI_FRAME_INT_ENA_R = crate::BitReader;
#[doc = "Field `HEADER_IDI_FRAME_INT_ENA` writer - write 1 to enable real input frame end of isp_input"]
pub type HEADER_IDI_FRAME_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - write 1 to enable input data type error"]
    #[inline(always)]
    pub fn isp_data_type_err_int_ena(&self) -> ISP_DATA_TYPE_ERR_INT_ENA_R {
        ISP_DATA_TYPE_ERR_INT_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - write 1 to enable isp input fifo overflow"]
    #[inline(always)]
    pub fn isp_async_fifo_ovf_int_ena(&self) -> ISP_ASYNC_FIFO_OVF_INT_ENA_R {
        ISP_ASYNC_FIFO_OVF_INT_ENA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - write 1 to enable isp input buffer full"]
    #[inline(always)]
    pub fn isp_buf_full_int_ena(&self) -> ISP_BUF_FULL_INT_ENA_R {
        ISP_BUF_FULL_INT_ENA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - write 1 to enable hnum and vnum setting format error"]
    #[inline(always)]
    pub fn isp_hvnum_setting_err_int_ena(&self) -> ISP_HVNUM_SETTING_ERR_INT_ENA_R {
        ISP_HVNUM_SETTING_ERR_INT_ENA_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - write 1 to enable setting invalid reg_data_type"]
    #[inline(always)]
    pub fn isp_data_type_setting_err_int_ena(&self) -> ISP_DATA_TYPE_SETTING_ERR_INT_ENA_R {
        ISP_DATA_TYPE_SETTING_ERR_INT_ENA_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - write 1 to enable hnum setting unmatch with mipi input"]
    #[inline(always)]
    pub fn isp_mipi_hnum_unmatch_int_ena(&self) -> ISP_MIPI_HNUM_UNMATCH_INT_ENA_R {
        ISP_MIPI_HNUM_UNMATCH_INT_ENA_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - write 1 to enable dpc check done"]
    #[inline(always)]
    pub fn dpc_check_done_int_ena(&self) -> DPC_CHECK_DONE_INT_ENA_R {
        DPC_CHECK_DONE_INT_ENA_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - write 1 to enable gamma setting error"]
    #[inline(always)]
    pub fn gamma_xcoord_err_int_ena(&self) -> GAMMA_XCOORD_ERR_INT_ENA_R {
        GAMMA_XCOORD_ERR_INT_ENA_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - write 1 to enable ae monitor"]
    #[inline(always)]
    pub fn ae_monitor_int_ena(&self) -> AE_MONITOR_INT_ENA_R {
        AE_MONITOR_INT_ENA_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - write 1 to enable ae"]
    #[inline(always)]
    pub fn ae_frame_done_int_ena(&self) -> AE_FRAME_DONE_INT_ENA_R {
        AE_FRAME_DONE_INT_ENA_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - write 1 to enable af statistic"]
    #[inline(always)]
    pub fn af_fdone_int_ena(&self) -> AF_FDONE_INT_ENA_R {
        AF_FDONE_INT_ENA_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - write 1 to enable af monitor"]
    #[inline(always)]
    pub fn af_env_int_ena(&self) -> AF_ENV_INT_ENA_R {
        AF_ENV_INT_ENA_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - write 1 to enable awb"]
    #[inline(always)]
    pub fn awb_fdone_int_ena(&self) -> AWB_FDONE_INT_ENA_R {
        AWB_FDONE_INT_ENA_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - write 1 to enable histogram"]
    #[inline(always)]
    pub fn hist_fdone_int_ena(&self) -> HIST_FDONE_INT_ENA_R {
        HIST_FDONE_INT_ENA_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - write 1 to enable isp frame end"]
    #[inline(always)]
    pub fn frame_int_ena(&self) -> FRAME_INT_ENA_R {
        FRAME_INT_ENA_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - write 1 to enable blc frame done"]
    #[inline(always)]
    pub fn blc_frame_int_ena(&self) -> BLC_FRAME_INT_ENA_R {
        BLC_FRAME_INT_ENA_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - write 1 to enable lsc frame done"]
    #[inline(always)]
    pub fn lsc_frame_int_ena(&self) -> LSC_FRAME_INT_ENA_R {
        LSC_FRAME_INT_ENA_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - write 1 to enable dpc frame done"]
    #[inline(always)]
    pub fn dpc_frame_int_ena(&self) -> DPC_FRAME_INT_ENA_R {
        DPC_FRAME_INT_ENA_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - write 1 to enable bf frame done"]
    #[inline(always)]
    pub fn bf_frame_int_ena(&self) -> BF_FRAME_INT_ENA_R {
        BF_FRAME_INT_ENA_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - write 1 to enable demosaic frame done"]
    #[inline(always)]
    pub fn demosaic_frame_int_ena(&self) -> DEMOSAIC_FRAME_INT_ENA_R {
        DEMOSAIC_FRAME_INT_ENA_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - write 1 to enable median frame done"]
    #[inline(always)]
    pub fn median_frame_int_ena(&self) -> MEDIAN_FRAME_INT_ENA_R {
        MEDIAN_FRAME_INT_ENA_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - write 1 to enable ccm frame done"]
    #[inline(always)]
    pub fn ccm_frame_int_ena(&self) -> CCM_FRAME_INT_ENA_R {
        CCM_FRAME_INT_ENA_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - write 1 to enable gamma frame done"]
    #[inline(always)]
    pub fn gamma_frame_int_ena(&self) -> GAMMA_FRAME_INT_ENA_R {
        GAMMA_FRAME_INT_ENA_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - write 1 to enable rgb2yuv frame done"]
    #[inline(always)]
    pub fn rgb2yuv_frame_int_ena(&self) -> RGB2YUV_FRAME_INT_ENA_R {
        RGB2YUV_FRAME_INT_ENA_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - write 1 to enable sharp frame done"]
    #[inline(always)]
    pub fn sharp_frame_int_ena(&self) -> SHARP_FRAME_INT_ENA_R {
        SHARP_FRAME_INT_ENA_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - write 1 to enable color frame done"]
    #[inline(always)]
    pub fn color_frame_int_ena(&self) -> COLOR_FRAME_INT_ENA_R {
        COLOR_FRAME_INT_ENA_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - write 1 to enable yuv2rgb frame done"]
    #[inline(always)]
    pub fn yuv2rgb_frame_int_ena(&self) -> YUV2RGB_FRAME_INT_ENA_R {
        YUV2RGB_FRAME_INT_ENA_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - write 1 to enable isp_tail idi frame_end"]
    #[inline(always)]
    pub fn tail_idi_frame_int_ena(&self) -> TAIL_IDI_FRAME_INT_ENA_R {
        TAIL_IDI_FRAME_INT_ENA_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - write 1 to enable real input frame end of isp_input"]
    #[inline(always)]
    pub fn header_idi_frame_int_ena(&self) -> HEADER_IDI_FRAME_INT_ENA_R {
        HEADER_IDI_FRAME_INT_ENA_R::new(((self.bits >> 28) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ENA")
            .field(
                "isp_data_type_err_int_ena",
                &format_args!("{}", self.isp_data_type_err_int_ena().bit()),
            )
            .field(
                "isp_async_fifo_ovf_int_ena",
                &format_args!("{}", self.isp_async_fifo_ovf_int_ena().bit()),
            )
            .field(
                "isp_buf_full_int_ena",
                &format_args!("{}", self.isp_buf_full_int_ena().bit()),
            )
            .field(
                "isp_hvnum_setting_err_int_ena",
                &format_args!("{}", self.isp_hvnum_setting_err_int_ena().bit()),
            )
            .field(
                "isp_data_type_setting_err_int_ena",
                &format_args!("{}", self.isp_data_type_setting_err_int_ena().bit()),
            )
            .field(
                "isp_mipi_hnum_unmatch_int_ena",
                &format_args!("{}", self.isp_mipi_hnum_unmatch_int_ena().bit()),
            )
            .field(
                "dpc_check_done_int_ena",
                &format_args!("{}", self.dpc_check_done_int_ena().bit()),
            )
            .field(
                "gamma_xcoord_err_int_ena",
                &format_args!("{}", self.gamma_xcoord_err_int_ena().bit()),
            )
            .field(
                "ae_monitor_int_ena",
                &format_args!("{}", self.ae_monitor_int_ena().bit()),
            )
            .field(
                "ae_frame_done_int_ena",
                &format_args!("{}", self.ae_frame_done_int_ena().bit()),
            )
            .field(
                "af_fdone_int_ena",
                &format_args!("{}", self.af_fdone_int_ena().bit()),
            )
            .field(
                "af_env_int_ena",
                &format_args!("{}", self.af_env_int_ena().bit()),
            )
            .field(
                "awb_fdone_int_ena",
                &format_args!("{}", self.awb_fdone_int_ena().bit()),
            )
            .field(
                "hist_fdone_int_ena",
                &format_args!("{}", self.hist_fdone_int_ena().bit()),
            )
            .field(
                "frame_int_ena",
                &format_args!("{}", self.frame_int_ena().bit()),
            )
            .field(
                "blc_frame_int_ena",
                &format_args!("{}", self.blc_frame_int_ena().bit()),
            )
            .field(
                "lsc_frame_int_ena",
                &format_args!("{}", self.lsc_frame_int_ena().bit()),
            )
            .field(
                "dpc_frame_int_ena",
                &format_args!("{}", self.dpc_frame_int_ena().bit()),
            )
            .field(
                "bf_frame_int_ena",
                &format_args!("{}", self.bf_frame_int_ena().bit()),
            )
            .field(
                "demosaic_frame_int_ena",
                &format_args!("{}", self.demosaic_frame_int_ena().bit()),
            )
            .field(
                "median_frame_int_ena",
                &format_args!("{}", self.median_frame_int_ena().bit()),
            )
            .field(
                "ccm_frame_int_ena",
                &format_args!("{}", self.ccm_frame_int_ena().bit()),
            )
            .field(
                "gamma_frame_int_ena",
                &format_args!("{}", self.gamma_frame_int_ena().bit()),
            )
            .field(
                "rgb2yuv_frame_int_ena",
                &format_args!("{}", self.rgb2yuv_frame_int_ena().bit()),
            )
            .field(
                "sharp_frame_int_ena",
                &format_args!("{}", self.sharp_frame_int_ena().bit()),
            )
            .field(
                "color_frame_int_ena",
                &format_args!("{}", self.color_frame_int_ena().bit()),
            )
            .field(
                "yuv2rgb_frame_int_ena",
                &format_args!("{}", self.yuv2rgb_frame_int_ena().bit()),
            )
            .field(
                "tail_idi_frame_int_ena",
                &format_args!("{}", self.tail_idi_frame_int_ena().bit()),
            )
            .field(
                "header_idi_frame_int_ena",
                &format_args!("{}", self.header_idi_frame_int_ena().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_ENA_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - write 1 to enable input data type error"]
    #[inline(always)]
    #[must_use]
    pub fn isp_data_type_err_int_ena(&mut self) -> ISP_DATA_TYPE_ERR_INT_ENA_W<INT_ENA_SPEC> {
        ISP_DATA_TYPE_ERR_INT_ENA_W::new(self, 0)
    }
    #[doc = "Bit 1 - write 1 to enable isp input fifo overflow"]
    #[inline(always)]
    #[must_use]
    pub fn isp_async_fifo_ovf_int_ena(&mut self) -> ISP_ASYNC_FIFO_OVF_INT_ENA_W<INT_ENA_SPEC> {
        ISP_ASYNC_FIFO_OVF_INT_ENA_W::new(self, 1)
    }
    #[doc = "Bit 2 - write 1 to enable isp input buffer full"]
    #[inline(always)]
    #[must_use]
    pub fn isp_buf_full_int_ena(&mut self) -> ISP_BUF_FULL_INT_ENA_W<INT_ENA_SPEC> {
        ISP_BUF_FULL_INT_ENA_W::new(self, 2)
    }
    #[doc = "Bit 3 - write 1 to enable hnum and vnum setting format error"]
    #[inline(always)]
    #[must_use]
    pub fn isp_hvnum_setting_err_int_ena(
        &mut self,
    ) -> ISP_HVNUM_SETTING_ERR_INT_ENA_W<INT_ENA_SPEC> {
        ISP_HVNUM_SETTING_ERR_INT_ENA_W::new(self, 3)
    }
    #[doc = "Bit 4 - write 1 to enable setting invalid reg_data_type"]
    #[inline(always)]
    #[must_use]
    pub fn isp_data_type_setting_err_int_ena(
        &mut self,
    ) -> ISP_DATA_TYPE_SETTING_ERR_INT_ENA_W<INT_ENA_SPEC> {
        ISP_DATA_TYPE_SETTING_ERR_INT_ENA_W::new(self, 4)
    }
    #[doc = "Bit 5 - write 1 to enable hnum setting unmatch with mipi input"]
    #[inline(always)]
    #[must_use]
    pub fn isp_mipi_hnum_unmatch_int_ena(
        &mut self,
    ) -> ISP_MIPI_HNUM_UNMATCH_INT_ENA_W<INT_ENA_SPEC> {
        ISP_MIPI_HNUM_UNMATCH_INT_ENA_W::new(self, 5)
    }
    #[doc = "Bit 6 - write 1 to enable dpc check done"]
    #[inline(always)]
    #[must_use]
    pub fn dpc_check_done_int_ena(&mut self) -> DPC_CHECK_DONE_INT_ENA_W<INT_ENA_SPEC> {
        DPC_CHECK_DONE_INT_ENA_W::new(self, 6)
    }
    #[doc = "Bit 7 - write 1 to enable gamma setting error"]
    #[inline(always)]
    #[must_use]
    pub fn gamma_xcoord_err_int_ena(&mut self) -> GAMMA_XCOORD_ERR_INT_ENA_W<INT_ENA_SPEC> {
        GAMMA_XCOORD_ERR_INT_ENA_W::new(self, 7)
    }
    #[doc = "Bit 8 - write 1 to enable ae monitor"]
    #[inline(always)]
    #[must_use]
    pub fn ae_monitor_int_ena(&mut self) -> AE_MONITOR_INT_ENA_W<INT_ENA_SPEC> {
        AE_MONITOR_INT_ENA_W::new(self, 8)
    }
    #[doc = "Bit 9 - write 1 to enable ae"]
    #[inline(always)]
    #[must_use]
    pub fn ae_frame_done_int_ena(&mut self) -> AE_FRAME_DONE_INT_ENA_W<INT_ENA_SPEC> {
        AE_FRAME_DONE_INT_ENA_W::new(self, 9)
    }
    #[doc = "Bit 10 - write 1 to enable af statistic"]
    #[inline(always)]
    #[must_use]
    pub fn af_fdone_int_ena(&mut self) -> AF_FDONE_INT_ENA_W<INT_ENA_SPEC> {
        AF_FDONE_INT_ENA_W::new(self, 10)
    }
    #[doc = "Bit 11 - write 1 to enable af monitor"]
    #[inline(always)]
    #[must_use]
    pub fn af_env_int_ena(&mut self) -> AF_ENV_INT_ENA_W<INT_ENA_SPEC> {
        AF_ENV_INT_ENA_W::new(self, 11)
    }
    #[doc = "Bit 12 - write 1 to enable awb"]
    #[inline(always)]
    #[must_use]
    pub fn awb_fdone_int_ena(&mut self) -> AWB_FDONE_INT_ENA_W<INT_ENA_SPEC> {
        AWB_FDONE_INT_ENA_W::new(self, 12)
    }
    #[doc = "Bit 13 - write 1 to enable histogram"]
    #[inline(always)]
    #[must_use]
    pub fn hist_fdone_int_ena(&mut self) -> HIST_FDONE_INT_ENA_W<INT_ENA_SPEC> {
        HIST_FDONE_INT_ENA_W::new(self, 13)
    }
    #[doc = "Bit 14 - write 1 to enable isp frame end"]
    #[inline(always)]
    #[must_use]
    pub fn frame_int_ena(&mut self) -> FRAME_INT_ENA_W<INT_ENA_SPEC> {
        FRAME_INT_ENA_W::new(self, 14)
    }
    #[doc = "Bit 15 - write 1 to enable blc frame done"]
    #[inline(always)]
    #[must_use]
    pub fn blc_frame_int_ena(&mut self) -> BLC_FRAME_INT_ENA_W<INT_ENA_SPEC> {
        BLC_FRAME_INT_ENA_W::new(self, 15)
    }
    #[doc = "Bit 16 - write 1 to enable lsc frame done"]
    #[inline(always)]
    #[must_use]
    pub fn lsc_frame_int_ena(&mut self) -> LSC_FRAME_INT_ENA_W<INT_ENA_SPEC> {
        LSC_FRAME_INT_ENA_W::new(self, 16)
    }
    #[doc = "Bit 17 - write 1 to enable dpc frame done"]
    #[inline(always)]
    #[must_use]
    pub fn dpc_frame_int_ena(&mut self) -> DPC_FRAME_INT_ENA_W<INT_ENA_SPEC> {
        DPC_FRAME_INT_ENA_W::new(self, 17)
    }
    #[doc = "Bit 18 - write 1 to enable bf frame done"]
    #[inline(always)]
    #[must_use]
    pub fn bf_frame_int_ena(&mut self) -> BF_FRAME_INT_ENA_W<INT_ENA_SPEC> {
        BF_FRAME_INT_ENA_W::new(self, 18)
    }
    #[doc = "Bit 19 - write 1 to enable demosaic frame done"]
    #[inline(always)]
    #[must_use]
    pub fn demosaic_frame_int_ena(&mut self) -> DEMOSAIC_FRAME_INT_ENA_W<INT_ENA_SPEC> {
        DEMOSAIC_FRAME_INT_ENA_W::new(self, 19)
    }
    #[doc = "Bit 20 - write 1 to enable median frame done"]
    #[inline(always)]
    #[must_use]
    pub fn median_frame_int_ena(&mut self) -> MEDIAN_FRAME_INT_ENA_W<INT_ENA_SPEC> {
        MEDIAN_FRAME_INT_ENA_W::new(self, 20)
    }
    #[doc = "Bit 21 - write 1 to enable ccm frame done"]
    #[inline(always)]
    #[must_use]
    pub fn ccm_frame_int_ena(&mut self) -> CCM_FRAME_INT_ENA_W<INT_ENA_SPEC> {
        CCM_FRAME_INT_ENA_W::new(self, 21)
    }
    #[doc = "Bit 22 - write 1 to enable gamma frame done"]
    #[inline(always)]
    #[must_use]
    pub fn gamma_frame_int_ena(&mut self) -> GAMMA_FRAME_INT_ENA_W<INT_ENA_SPEC> {
        GAMMA_FRAME_INT_ENA_W::new(self, 22)
    }
    #[doc = "Bit 23 - write 1 to enable rgb2yuv frame done"]
    #[inline(always)]
    #[must_use]
    pub fn rgb2yuv_frame_int_ena(&mut self) -> RGB2YUV_FRAME_INT_ENA_W<INT_ENA_SPEC> {
        RGB2YUV_FRAME_INT_ENA_W::new(self, 23)
    }
    #[doc = "Bit 24 - write 1 to enable sharp frame done"]
    #[inline(always)]
    #[must_use]
    pub fn sharp_frame_int_ena(&mut self) -> SHARP_FRAME_INT_ENA_W<INT_ENA_SPEC> {
        SHARP_FRAME_INT_ENA_W::new(self, 24)
    }
    #[doc = "Bit 25 - write 1 to enable color frame done"]
    #[inline(always)]
    #[must_use]
    pub fn color_frame_int_ena(&mut self) -> COLOR_FRAME_INT_ENA_W<INT_ENA_SPEC> {
        COLOR_FRAME_INT_ENA_W::new(self, 25)
    }
    #[doc = "Bit 26 - write 1 to enable yuv2rgb frame done"]
    #[inline(always)]
    #[must_use]
    pub fn yuv2rgb_frame_int_ena(&mut self) -> YUV2RGB_FRAME_INT_ENA_W<INT_ENA_SPEC> {
        YUV2RGB_FRAME_INT_ENA_W::new(self, 26)
    }
    #[doc = "Bit 27 - write 1 to enable isp_tail idi frame_end"]
    #[inline(always)]
    #[must_use]
    pub fn tail_idi_frame_int_ena(&mut self) -> TAIL_IDI_FRAME_INT_ENA_W<INT_ENA_SPEC> {
        TAIL_IDI_FRAME_INT_ENA_W::new(self, 27)
    }
    #[doc = "Bit 28 - write 1 to enable real input frame end of isp_input"]
    #[inline(always)]
    #[must_use]
    pub fn header_idi_frame_int_ena(&mut self) -> HEADER_IDI_FRAME_INT_ENA_W<INT_ENA_SPEC> {
        HEADER_IDI_FRAME_INT_ENA_W::new(self, 28)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INT_ENA to value 0xc3"]
impl crate::Resettable for INT_ENA_SPEC {
    const RESET_VALUE: Self::Ux = 0xc3;
}
