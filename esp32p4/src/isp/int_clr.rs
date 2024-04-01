#[doc = "Register `INT_CLR` writer"]
pub type W = crate::W<INT_CLR_SPEC>;
#[doc = "Field `ISP_DATA_TYPE_ERR` writer - write 1 to clear input data type error"]
pub type ISP_DATA_TYPE_ERR_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `ISP_ASYNC_FIFO_OVF` writer - write 1 to clear isp input fifo overflow"]
pub type ISP_ASYNC_FIFO_OVF_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `ISP_BUF_FULL` writer - write 1 to clear isp input buffer full"]
pub type ISP_BUF_FULL_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `ISP_HVNUM_SETTING_ERR` writer - write 1 to clear hnum and vnum setting format error"]
pub type ISP_HVNUM_SETTING_ERR_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `ISP_DATA_TYPE_SETTING_ERR` writer - write 1 to clear setting invalid reg_data_type"]
pub type ISP_DATA_TYPE_SETTING_ERR_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `ISP_MIPI_HNUM_UNMATCH` writer - write 1 to clear hnum setting unmatch with mipi input"]
pub type ISP_MIPI_HNUM_UNMATCH_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `DPC_CHECK_DONE` writer - write 1 to clear dpc check done"]
pub type DPC_CHECK_DONE_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `GAMMA_XCOORD_ERR` writer - write 1 to clear gamma setting error"]
pub type GAMMA_XCOORD_ERR_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `AE_MONITOR` writer - write 1 to clear ae monitor"]
pub type AE_MONITOR_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `AE_FRAME_DONE` writer - write 1 to clear ae"]
pub type AE_FRAME_DONE_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `AF_FDONE` writer - write 1 to clear af statistic"]
pub type AF_FDONE_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `AF_ENV` writer - write 1 to clear af monitor"]
pub type AF_ENV_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `AWB_FDONE` writer - write 1 to clear awb"]
pub type AWB_FDONE_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `HIST_FDONE` writer - write 1 to clear histogram"]
pub type HIST_FDONE_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `FRAME` writer - write 1 to clear isp frame end"]
pub type FRAME_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `BLC_FRAME` writer - write 1 to clear blc frame done"]
pub type BLC_FRAME_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `LSC_FRAME` writer - write 1 to clear lsc frame done"]
pub type LSC_FRAME_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `DPC_FRAME` writer - write 1 to clear dpc frame done"]
pub type DPC_FRAME_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `BF_FRAME` writer - write 1 to clear bf frame done"]
pub type BF_FRAME_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `DEMOSAIC_FRAME` writer - write 1 to clear demosaic frame done"]
pub type DEMOSAIC_FRAME_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `MEDIAN_FRAME` writer - write 1 to clear median frame done"]
pub type MEDIAN_FRAME_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `CCM_FRAME` writer - write 1 to clear ccm frame done"]
pub type CCM_FRAME_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `GAMMA_FRAME` writer - write 1 to clear gamma frame done"]
pub type GAMMA_FRAME_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `RGB2YUV_FRAME` writer - write 1 to clear rgb2yuv frame done"]
pub type RGB2YUV_FRAME_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `SHARP_FRAME` writer - write 1 to clear sharp frame done"]
pub type SHARP_FRAME_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `COLOR_FRAME` writer - write 1 to clear color frame done"]
pub type COLOR_FRAME_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `YUV2RGB_FRAME` writer - write 1 to clear yuv2rgb frame done"]
pub type YUV2RGB_FRAME_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `TAIL_IDI_FRAME` writer - write 1 to clear isp_tail idi frame_end"]
pub type TAIL_IDI_FRAME_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `HEADER_IDI_FRAME` writer - write 1 to clear real input frame end of isp_input"]
pub type HEADER_IDI_FRAME_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - write 1 to clear input data type error"]
    #[inline(always)]
    #[must_use]
    pub fn isp_data_type_err(&mut self) -> ISP_DATA_TYPE_ERR_W<INT_CLR_SPEC> {
        ISP_DATA_TYPE_ERR_W::new(self, 0)
    }
    #[doc = "Bit 1 - write 1 to clear isp input fifo overflow"]
    #[inline(always)]
    #[must_use]
    pub fn isp_async_fifo_ovf(&mut self) -> ISP_ASYNC_FIFO_OVF_W<INT_CLR_SPEC> {
        ISP_ASYNC_FIFO_OVF_W::new(self, 1)
    }
    #[doc = "Bit 2 - write 1 to clear isp input buffer full"]
    #[inline(always)]
    #[must_use]
    pub fn isp_buf_full(&mut self) -> ISP_BUF_FULL_W<INT_CLR_SPEC> {
        ISP_BUF_FULL_W::new(self, 2)
    }
    #[doc = "Bit 3 - write 1 to clear hnum and vnum setting format error"]
    #[inline(always)]
    #[must_use]
    pub fn isp_hvnum_setting_err(&mut self) -> ISP_HVNUM_SETTING_ERR_W<INT_CLR_SPEC> {
        ISP_HVNUM_SETTING_ERR_W::new(self, 3)
    }
    #[doc = "Bit 4 - write 1 to clear setting invalid reg_data_type"]
    #[inline(always)]
    #[must_use]
    pub fn isp_data_type_setting_err(&mut self) -> ISP_DATA_TYPE_SETTING_ERR_W<INT_CLR_SPEC> {
        ISP_DATA_TYPE_SETTING_ERR_W::new(self, 4)
    }
    #[doc = "Bit 5 - write 1 to clear hnum setting unmatch with mipi input"]
    #[inline(always)]
    #[must_use]
    pub fn isp_mipi_hnum_unmatch(&mut self) -> ISP_MIPI_HNUM_UNMATCH_W<INT_CLR_SPEC> {
        ISP_MIPI_HNUM_UNMATCH_W::new(self, 5)
    }
    #[doc = "Bit 6 - write 1 to clear dpc check done"]
    #[inline(always)]
    #[must_use]
    pub fn dpc_check_done(&mut self) -> DPC_CHECK_DONE_W<INT_CLR_SPEC> {
        DPC_CHECK_DONE_W::new(self, 6)
    }
    #[doc = "Bit 7 - write 1 to clear gamma setting error"]
    #[inline(always)]
    #[must_use]
    pub fn gamma_xcoord_err(&mut self) -> GAMMA_XCOORD_ERR_W<INT_CLR_SPEC> {
        GAMMA_XCOORD_ERR_W::new(self, 7)
    }
    #[doc = "Bit 8 - write 1 to clear ae monitor"]
    #[inline(always)]
    #[must_use]
    pub fn ae_monitor(&mut self) -> AE_MONITOR_W<INT_CLR_SPEC> {
        AE_MONITOR_W::new(self, 8)
    }
    #[doc = "Bit 9 - write 1 to clear ae"]
    #[inline(always)]
    #[must_use]
    pub fn ae_frame_done(&mut self) -> AE_FRAME_DONE_W<INT_CLR_SPEC> {
        AE_FRAME_DONE_W::new(self, 9)
    }
    #[doc = "Bit 10 - write 1 to clear af statistic"]
    #[inline(always)]
    #[must_use]
    pub fn af_fdone(&mut self) -> AF_FDONE_W<INT_CLR_SPEC> {
        AF_FDONE_W::new(self, 10)
    }
    #[doc = "Bit 11 - write 1 to clear af monitor"]
    #[inline(always)]
    #[must_use]
    pub fn af_env(&mut self) -> AF_ENV_W<INT_CLR_SPEC> {
        AF_ENV_W::new(self, 11)
    }
    #[doc = "Bit 12 - write 1 to clear awb"]
    #[inline(always)]
    #[must_use]
    pub fn awb_fdone(&mut self) -> AWB_FDONE_W<INT_CLR_SPEC> {
        AWB_FDONE_W::new(self, 12)
    }
    #[doc = "Bit 13 - write 1 to clear histogram"]
    #[inline(always)]
    #[must_use]
    pub fn hist_fdone(&mut self) -> HIST_FDONE_W<INT_CLR_SPEC> {
        HIST_FDONE_W::new(self, 13)
    }
    #[doc = "Bit 14 - write 1 to clear isp frame end"]
    #[inline(always)]
    #[must_use]
    pub fn frame(&mut self) -> FRAME_W<INT_CLR_SPEC> {
        FRAME_W::new(self, 14)
    }
    #[doc = "Bit 15 - write 1 to clear blc frame done"]
    #[inline(always)]
    #[must_use]
    pub fn blc_frame(&mut self) -> BLC_FRAME_W<INT_CLR_SPEC> {
        BLC_FRAME_W::new(self, 15)
    }
    #[doc = "Bit 16 - write 1 to clear lsc frame done"]
    #[inline(always)]
    #[must_use]
    pub fn lsc_frame(&mut self) -> LSC_FRAME_W<INT_CLR_SPEC> {
        LSC_FRAME_W::new(self, 16)
    }
    #[doc = "Bit 17 - write 1 to clear dpc frame done"]
    #[inline(always)]
    #[must_use]
    pub fn dpc_frame(&mut self) -> DPC_FRAME_W<INT_CLR_SPEC> {
        DPC_FRAME_W::new(self, 17)
    }
    #[doc = "Bit 18 - write 1 to clear bf frame done"]
    #[inline(always)]
    #[must_use]
    pub fn bf_frame(&mut self) -> BF_FRAME_W<INT_CLR_SPEC> {
        BF_FRAME_W::new(self, 18)
    }
    #[doc = "Bit 19 - write 1 to clear demosaic frame done"]
    #[inline(always)]
    #[must_use]
    pub fn demosaic_frame(&mut self) -> DEMOSAIC_FRAME_W<INT_CLR_SPEC> {
        DEMOSAIC_FRAME_W::new(self, 19)
    }
    #[doc = "Bit 20 - write 1 to clear median frame done"]
    #[inline(always)]
    #[must_use]
    pub fn median_frame(&mut self) -> MEDIAN_FRAME_W<INT_CLR_SPEC> {
        MEDIAN_FRAME_W::new(self, 20)
    }
    #[doc = "Bit 21 - write 1 to clear ccm frame done"]
    #[inline(always)]
    #[must_use]
    pub fn ccm_frame(&mut self) -> CCM_FRAME_W<INT_CLR_SPEC> {
        CCM_FRAME_W::new(self, 21)
    }
    #[doc = "Bit 22 - write 1 to clear gamma frame done"]
    #[inline(always)]
    #[must_use]
    pub fn gamma_frame(&mut self) -> GAMMA_FRAME_W<INT_CLR_SPEC> {
        GAMMA_FRAME_W::new(self, 22)
    }
    #[doc = "Bit 23 - write 1 to clear rgb2yuv frame done"]
    #[inline(always)]
    #[must_use]
    pub fn rgb2yuv_frame(&mut self) -> RGB2YUV_FRAME_W<INT_CLR_SPEC> {
        RGB2YUV_FRAME_W::new(self, 23)
    }
    #[doc = "Bit 24 - write 1 to clear sharp frame done"]
    #[inline(always)]
    #[must_use]
    pub fn sharp_frame(&mut self) -> SHARP_FRAME_W<INT_CLR_SPEC> {
        SHARP_FRAME_W::new(self, 24)
    }
    #[doc = "Bit 25 - write 1 to clear color frame done"]
    #[inline(always)]
    #[must_use]
    pub fn color_frame(&mut self) -> COLOR_FRAME_W<INT_CLR_SPEC> {
        COLOR_FRAME_W::new(self, 25)
    }
    #[doc = "Bit 26 - write 1 to clear yuv2rgb frame done"]
    #[inline(always)]
    #[must_use]
    pub fn yuv2rgb_frame(&mut self) -> YUV2RGB_FRAME_W<INT_CLR_SPEC> {
        YUV2RGB_FRAME_W::new(self, 26)
    }
    #[doc = "Bit 27 - write 1 to clear isp_tail idi frame_end"]
    #[inline(always)]
    #[must_use]
    pub fn tail_idi_frame(&mut self) -> TAIL_IDI_FRAME_W<INT_CLR_SPEC> {
        TAIL_IDI_FRAME_W::new(self, 27)
    }
    #[doc = "Bit 28 - write 1 to clear real input frame end of isp_input"]
    #[inline(always)]
    #[must_use]
    pub fn header_idi_frame(&mut self) -> HEADER_IDI_FRAME_W<INT_CLR_SPEC> {
        HEADER_IDI_FRAME_W::new(self, 28)
    }
}
#[doc = "interrupt clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_CLR_SPEC;
impl crate::RegisterSpec for INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`int_clr::W`](W) writer structure"]
impl crate::Writable for INT_CLR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x1fff_ffff;
}
#[doc = "`reset()` method sets INT_CLR to value 0"]
impl crate::Resettable for INT_CLR_SPEC {
    const RESET_VALUE: u32 = 0;
}
