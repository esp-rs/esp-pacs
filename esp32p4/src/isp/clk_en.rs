#[doc = "Register `CLK_EN` reader"]
pub type R = crate::R<CLK_EN_SPEC>;
#[doc = "Register `CLK_EN` writer"]
pub type W = crate::W<CLK_EN_SPEC>;
#[doc = "Field `CLK_EN` reader - this bit configures the clk force on of isp reg. 0: disable, 1: enable"]
pub type CLK_EN_R = crate::BitReader;
#[doc = "Field `CLK_EN` writer - this bit configures the clk force on of isp reg. 0: disable, 1: enable"]
pub type CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_BLC_FORCE_ON` reader - this bit configures the clk force on of blc. 0: disable, 1: enable"]
pub type CLK_BLC_FORCE_ON_R = crate::BitReader;
#[doc = "Field `CLK_BLC_FORCE_ON` writer - this bit configures the clk force on of blc. 0: disable, 1: enable"]
pub type CLK_BLC_FORCE_ON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_DPC_FORCE_ON` reader - this bit configures the clk force on of dpc. 0: disable, 1: enable"]
pub type CLK_DPC_FORCE_ON_R = crate::BitReader;
#[doc = "Field `CLK_DPC_FORCE_ON` writer - this bit configures the clk force on of dpc. 0: disable, 1: enable"]
pub type CLK_DPC_FORCE_ON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_BF_FORCE_ON` reader - this bit configures the clk force on of bf. 0: disable, 1: enable"]
pub type CLK_BF_FORCE_ON_R = crate::BitReader;
#[doc = "Field `CLK_BF_FORCE_ON` writer - this bit configures the clk force on of bf. 0: disable, 1: enable"]
pub type CLK_BF_FORCE_ON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_LSC_FORCE_ON` reader - this bit configures the clk force on of lsc. 0: disable, 1: enable"]
pub type CLK_LSC_FORCE_ON_R = crate::BitReader;
#[doc = "Field `CLK_LSC_FORCE_ON` writer - this bit configures the clk force on of lsc. 0: disable, 1: enable"]
pub type CLK_LSC_FORCE_ON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_DEMOSAIC_FORCE_ON` reader - this bit configures the clk force on of demosaic. 0: disable, 1: enable"]
pub type CLK_DEMOSAIC_FORCE_ON_R = crate::BitReader;
#[doc = "Field `CLK_DEMOSAIC_FORCE_ON` writer - this bit configures the clk force on of demosaic. 0: disable, 1: enable"]
pub type CLK_DEMOSAIC_FORCE_ON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_MEDIAN_FORCE_ON` reader - this bit configures the clk force on of median. 0: disable, 1: enable"]
pub type CLK_MEDIAN_FORCE_ON_R = crate::BitReader;
#[doc = "Field `CLK_MEDIAN_FORCE_ON` writer - this bit configures the clk force on of median. 0: disable, 1: enable"]
pub type CLK_MEDIAN_FORCE_ON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_CCM_FORCE_ON` reader - this bit configures the clk force on of ccm. 0: disable, 1: enable"]
pub type CLK_CCM_FORCE_ON_R = crate::BitReader;
#[doc = "Field `CLK_CCM_FORCE_ON` writer - this bit configures the clk force on of ccm. 0: disable, 1: enable"]
pub type CLK_CCM_FORCE_ON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_GAMMA_FORCE_ON` reader - this bit configures the clk force on of gamma. 0: disable, 1: enable"]
pub type CLK_GAMMA_FORCE_ON_R = crate::BitReader;
#[doc = "Field `CLK_GAMMA_FORCE_ON` writer - this bit configures the clk force on of gamma. 0: disable, 1: enable"]
pub type CLK_GAMMA_FORCE_ON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_RGB2YUV_FORCE_ON` reader - this bit configures the clk force on of rgb2yuv. 0: disable, 1: enable"]
pub type CLK_RGB2YUV_FORCE_ON_R = crate::BitReader;
#[doc = "Field `CLK_RGB2YUV_FORCE_ON` writer - this bit configures the clk force on of rgb2yuv. 0: disable, 1: enable"]
pub type CLK_RGB2YUV_FORCE_ON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_SHARP_FORCE_ON` reader - this bit configures the clk force on of sharp. 0: disable, 1: enable"]
pub type CLK_SHARP_FORCE_ON_R = crate::BitReader;
#[doc = "Field `CLK_SHARP_FORCE_ON` writer - this bit configures the clk force on of sharp. 0: disable, 1: enable"]
pub type CLK_SHARP_FORCE_ON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_COLOR_FORCE_ON` reader - this bit configures the clk force on of color. 0: disable, 1: enable"]
pub type CLK_COLOR_FORCE_ON_R = crate::BitReader;
#[doc = "Field `CLK_COLOR_FORCE_ON` writer - this bit configures the clk force on of color. 0: disable, 1: enable"]
pub type CLK_COLOR_FORCE_ON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_YUV2RGB_FORCE_ON` reader - this bit configures the clk force on of yuv2rgb. 0: disable, 1: enable"]
pub type CLK_YUV2RGB_FORCE_ON_R = crate::BitReader;
#[doc = "Field `CLK_YUV2RGB_FORCE_ON` writer - this bit configures the clk force on of yuv2rgb. 0: disable, 1: enable"]
pub type CLK_YUV2RGB_FORCE_ON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_AE_FORCE_ON` reader - this bit configures the clk force on of ae. 0: disable, 1: enable"]
pub type CLK_AE_FORCE_ON_R = crate::BitReader;
#[doc = "Field `CLK_AE_FORCE_ON` writer - this bit configures the clk force on of ae. 0: disable, 1: enable"]
pub type CLK_AE_FORCE_ON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_AF_FORCE_ON` reader - this bit configures the clk force on of af. 0: disable, 1: enable"]
pub type CLK_AF_FORCE_ON_R = crate::BitReader;
#[doc = "Field `CLK_AF_FORCE_ON` writer - this bit configures the clk force on of af. 0: disable, 1: enable"]
pub type CLK_AF_FORCE_ON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_AWB_FORCE_ON` reader - this bit configures the clk force on of awb. 0: disable, 1: enable"]
pub type CLK_AWB_FORCE_ON_R = crate::BitReader;
#[doc = "Field `CLK_AWB_FORCE_ON` writer - this bit configures the clk force on of awb. 0: disable, 1: enable"]
pub type CLK_AWB_FORCE_ON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_HIST_FORCE_ON` reader - this bit configures the clk force on of hist. 0: disable, 1: enable"]
pub type CLK_HIST_FORCE_ON_R = crate::BitReader;
#[doc = "Field `CLK_HIST_FORCE_ON` writer - this bit configures the clk force on of hist. 0: disable, 1: enable"]
pub type CLK_HIST_FORCE_ON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_MIPI_IDI_FORCE_ON` reader - this bit configures the clk force on of mipi idi input. 0: disable, 1: enable"]
pub type CLK_MIPI_IDI_FORCE_ON_R = crate::BitReader;
#[doc = "Field `CLK_MIPI_IDI_FORCE_ON` writer - this bit configures the clk force on of mipi idi input. 0: disable, 1: enable"]
pub type CLK_MIPI_IDI_FORCE_ON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ISP_MEM_CLK_FORCE_ON` reader - this bit configures the clk force on of all isp memory. 0: disable, 1: enable"]
pub type ISP_MEM_CLK_FORCE_ON_R = crate::BitReader;
#[doc = "Field `ISP_MEM_CLK_FORCE_ON` writer - this bit configures the clk force on of all isp memory. 0: disable, 1: enable"]
pub type ISP_MEM_CLK_FORCE_ON_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - this bit configures the clk force on of isp reg. 0: disable, 1: enable"]
    #[inline(always)]
    pub fn clk_en(&self) -> CLK_EN_R {
        CLK_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - this bit configures the clk force on of blc. 0: disable, 1: enable"]
    #[inline(always)]
    pub fn clk_blc_force_on(&self) -> CLK_BLC_FORCE_ON_R {
        CLK_BLC_FORCE_ON_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - this bit configures the clk force on of dpc. 0: disable, 1: enable"]
    #[inline(always)]
    pub fn clk_dpc_force_on(&self) -> CLK_DPC_FORCE_ON_R {
        CLK_DPC_FORCE_ON_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - this bit configures the clk force on of bf. 0: disable, 1: enable"]
    #[inline(always)]
    pub fn clk_bf_force_on(&self) -> CLK_BF_FORCE_ON_R {
        CLK_BF_FORCE_ON_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - this bit configures the clk force on of lsc. 0: disable, 1: enable"]
    #[inline(always)]
    pub fn clk_lsc_force_on(&self) -> CLK_LSC_FORCE_ON_R {
        CLK_LSC_FORCE_ON_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - this bit configures the clk force on of demosaic. 0: disable, 1: enable"]
    #[inline(always)]
    pub fn clk_demosaic_force_on(&self) -> CLK_DEMOSAIC_FORCE_ON_R {
        CLK_DEMOSAIC_FORCE_ON_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - this bit configures the clk force on of median. 0: disable, 1: enable"]
    #[inline(always)]
    pub fn clk_median_force_on(&self) -> CLK_MEDIAN_FORCE_ON_R {
        CLK_MEDIAN_FORCE_ON_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - this bit configures the clk force on of ccm. 0: disable, 1: enable"]
    #[inline(always)]
    pub fn clk_ccm_force_on(&self) -> CLK_CCM_FORCE_ON_R {
        CLK_CCM_FORCE_ON_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - this bit configures the clk force on of gamma. 0: disable, 1: enable"]
    #[inline(always)]
    pub fn clk_gamma_force_on(&self) -> CLK_GAMMA_FORCE_ON_R {
        CLK_GAMMA_FORCE_ON_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - this bit configures the clk force on of rgb2yuv. 0: disable, 1: enable"]
    #[inline(always)]
    pub fn clk_rgb2yuv_force_on(&self) -> CLK_RGB2YUV_FORCE_ON_R {
        CLK_RGB2YUV_FORCE_ON_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - this bit configures the clk force on of sharp. 0: disable, 1: enable"]
    #[inline(always)]
    pub fn clk_sharp_force_on(&self) -> CLK_SHARP_FORCE_ON_R {
        CLK_SHARP_FORCE_ON_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - this bit configures the clk force on of color. 0: disable, 1: enable"]
    #[inline(always)]
    pub fn clk_color_force_on(&self) -> CLK_COLOR_FORCE_ON_R {
        CLK_COLOR_FORCE_ON_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - this bit configures the clk force on of yuv2rgb. 0: disable, 1: enable"]
    #[inline(always)]
    pub fn clk_yuv2rgb_force_on(&self) -> CLK_YUV2RGB_FORCE_ON_R {
        CLK_YUV2RGB_FORCE_ON_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - this bit configures the clk force on of ae. 0: disable, 1: enable"]
    #[inline(always)]
    pub fn clk_ae_force_on(&self) -> CLK_AE_FORCE_ON_R {
        CLK_AE_FORCE_ON_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - this bit configures the clk force on of af. 0: disable, 1: enable"]
    #[inline(always)]
    pub fn clk_af_force_on(&self) -> CLK_AF_FORCE_ON_R {
        CLK_AF_FORCE_ON_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - this bit configures the clk force on of awb. 0: disable, 1: enable"]
    #[inline(always)]
    pub fn clk_awb_force_on(&self) -> CLK_AWB_FORCE_ON_R {
        CLK_AWB_FORCE_ON_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - this bit configures the clk force on of hist. 0: disable, 1: enable"]
    #[inline(always)]
    pub fn clk_hist_force_on(&self) -> CLK_HIST_FORCE_ON_R {
        CLK_HIST_FORCE_ON_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - this bit configures the clk force on of mipi idi input. 0: disable, 1: enable"]
    #[inline(always)]
    pub fn clk_mipi_idi_force_on(&self) -> CLK_MIPI_IDI_FORCE_ON_R {
        CLK_MIPI_IDI_FORCE_ON_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - this bit configures the clk force on of all isp memory. 0: disable, 1: enable"]
    #[inline(always)]
    pub fn isp_mem_clk_force_on(&self) -> ISP_MEM_CLK_FORCE_ON_R {
        ISP_MEM_CLK_FORCE_ON_R::new(((self.bits >> 18) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLK_EN")
            .field("clk_en", &self.clk_en().bit())
            .field("clk_blc_force_on", &self.clk_blc_force_on().bit())
            .field("clk_dpc_force_on", &self.clk_dpc_force_on().bit())
            .field("clk_bf_force_on", &self.clk_bf_force_on().bit())
            .field("clk_lsc_force_on", &self.clk_lsc_force_on().bit())
            .field("clk_demosaic_force_on", &self.clk_demosaic_force_on().bit())
            .field("clk_median_force_on", &self.clk_median_force_on().bit())
            .field("clk_ccm_force_on", &self.clk_ccm_force_on().bit())
            .field("clk_gamma_force_on", &self.clk_gamma_force_on().bit())
            .field("clk_rgb2yuv_force_on", &self.clk_rgb2yuv_force_on().bit())
            .field("clk_sharp_force_on", &self.clk_sharp_force_on().bit())
            .field("clk_color_force_on", &self.clk_color_force_on().bit())
            .field("clk_yuv2rgb_force_on", &self.clk_yuv2rgb_force_on().bit())
            .field("clk_ae_force_on", &self.clk_ae_force_on().bit())
            .field("clk_af_force_on", &self.clk_af_force_on().bit())
            .field("clk_awb_force_on", &self.clk_awb_force_on().bit())
            .field("clk_hist_force_on", &self.clk_hist_force_on().bit())
            .field("clk_mipi_idi_force_on", &self.clk_mipi_idi_force_on().bit())
            .field("isp_mem_clk_force_on", &self.isp_mem_clk_force_on().bit())
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CLK_EN_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - this bit configures the clk force on of isp reg. 0: disable, 1: enable"]
    #[inline(always)]
    #[must_use]
    pub fn clk_en(&mut self) -> CLK_EN_W<CLK_EN_SPEC> {
        CLK_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - this bit configures the clk force on of blc. 0: disable, 1: enable"]
    #[inline(always)]
    #[must_use]
    pub fn clk_blc_force_on(&mut self) -> CLK_BLC_FORCE_ON_W<CLK_EN_SPEC> {
        CLK_BLC_FORCE_ON_W::new(self, 1)
    }
    #[doc = "Bit 2 - this bit configures the clk force on of dpc. 0: disable, 1: enable"]
    #[inline(always)]
    #[must_use]
    pub fn clk_dpc_force_on(&mut self) -> CLK_DPC_FORCE_ON_W<CLK_EN_SPEC> {
        CLK_DPC_FORCE_ON_W::new(self, 2)
    }
    #[doc = "Bit 3 - this bit configures the clk force on of bf. 0: disable, 1: enable"]
    #[inline(always)]
    #[must_use]
    pub fn clk_bf_force_on(&mut self) -> CLK_BF_FORCE_ON_W<CLK_EN_SPEC> {
        CLK_BF_FORCE_ON_W::new(self, 3)
    }
    #[doc = "Bit 4 - this bit configures the clk force on of lsc. 0: disable, 1: enable"]
    #[inline(always)]
    #[must_use]
    pub fn clk_lsc_force_on(&mut self) -> CLK_LSC_FORCE_ON_W<CLK_EN_SPEC> {
        CLK_LSC_FORCE_ON_W::new(self, 4)
    }
    #[doc = "Bit 5 - this bit configures the clk force on of demosaic. 0: disable, 1: enable"]
    #[inline(always)]
    #[must_use]
    pub fn clk_demosaic_force_on(&mut self) -> CLK_DEMOSAIC_FORCE_ON_W<CLK_EN_SPEC> {
        CLK_DEMOSAIC_FORCE_ON_W::new(self, 5)
    }
    #[doc = "Bit 6 - this bit configures the clk force on of median. 0: disable, 1: enable"]
    #[inline(always)]
    #[must_use]
    pub fn clk_median_force_on(&mut self) -> CLK_MEDIAN_FORCE_ON_W<CLK_EN_SPEC> {
        CLK_MEDIAN_FORCE_ON_W::new(self, 6)
    }
    #[doc = "Bit 7 - this bit configures the clk force on of ccm. 0: disable, 1: enable"]
    #[inline(always)]
    #[must_use]
    pub fn clk_ccm_force_on(&mut self) -> CLK_CCM_FORCE_ON_W<CLK_EN_SPEC> {
        CLK_CCM_FORCE_ON_W::new(self, 7)
    }
    #[doc = "Bit 8 - this bit configures the clk force on of gamma. 0: disable, 1: enable"]
    #[inline(always)]
    #[must_use]
    pub fn clk_gamma_force_on(&mut self) -> CLK_GAMMA_FORCE_ON_W<CLK_EN_SPEC> {
        CLK_GAMMA_FORCE_ON_W::new(self, 8)
    }
    #[doc = "Bit 9 - this bit configures the clk force on of rgb2yuv. 0: disable, 1: enable"]
    #[inline(always)]
    #[must_use]
    pub fn clk_rgb2yuv_force_on(&mut self) -> CLK_RGB2YUV_FORCE_ON_W<CLK_EN_SPEC> {
        CLK_RGB2YUV_FORCE_ON_W::new(self, 9)
    }
    #[doc = "Bit 10 - this bit configures the clk force on of sharp. 0: disable, 1: enable"]
    #[inline(always)]
    #[must_use]
    pub fn clk_sharp_force_on(&mut self) -> CLK_SHARP_FORCE_ON_W<CLK_EN_SPEC> {
        CLK_SHARP_FORCE_ON_W::new(self, 10)
    }
    #[doc = "Bit 11 - this bit configures the clk force on of color. 0: disable, 1: enable"]
    #[inline(always)]
    #[must_use]
    pub fn clk_color_force_on(&mut self) -> CLK_COLOR_FORCE_ON_W<CLK_EN_SPEC> {
        CLK_COLOR_FORCE_ON_W::new(self, 11)
    }
    #[doc = "Bit 12 - this bit configures the clk force on of yuv2rgb. 0: disable, 1: enable"]
    #[inline(always)]
    #[must_use]
    pub fn clk_yuv2rgb_force_on(&mut self) -> CLK_YUV2RGB_FORCE_ON_W<CLK_EN_SPEC> {
        CLK_YUV2RGB_FORCE_ON_W::new(self, 12)
    }
    #[doc = "Bit 13 - this bit configures the clk force on of ae. 0: disable, 1: enable"]
    #[inline(always)]
    #[must_use]
    pub fn clk_ae_force_on(&mut self) -> CLK_AE_FORCE_ON_W<CLK_EN_SPEC> {
        CLK_AE_FORCE_ON_W::new(self, 13)
    }
    #[doc = "Bit 14 - this bit configures the clk force on of af. 0: disable, 1: enable"]
    #[inline(always)]
    #[must_use]
    pub fn clk_af_force_on(&mut self) -> CLK_AF_FORCE_ON_W<CLK_EN_SPEC> {
        CLK_AF_FORCE_ON_W::new(self, 14)
    }
    #[doc = "Bit 15 - this bit configures the clk force on of awb. 0: disable, 1: enable"]
    #[inline(always)]
    #[must_use]
    pub fn clk_awb_force_on(&mut self) -> CLK_AWB_FORCE_ON_W<CLK_EN_SPEC> {
        CLK_AWB_FORCE_ON_W::new(self, 15)
    }
    #[doc = "Bit 16 - this bit configures the clk force on of hist. 0: disable, 1: enable"]
    #[inline(always)]
    #[must_use]
    pub fn clk_hist_force_on(&mut self) -> CLK_HIST_FORCE_ON_W<CLK_EN_SPEC> {
        CLK_HIST_FORCE_ON_W::new(self, 16)
    }
    #[doc = "Bit 17 - this bit configures the clk force on of mipi idi input. 0: disable, 1: enable"]
    #[inline(always)]
    #[must_use]
    pub fn clk_mipi_idi_force_on(&mut self) -> CLK_MIPI_IDI_FORCE_ON_W<CLK_EN_SPEC> {
        CLK_MIPI_IDI_FORCE_ON_W::new(self, 17)
    }
    #[doc = "Bit 18 - this bit configures the clk force on of all isp memory. 0: disable, 1: enable"]
    #[inline(always)]
    #[must_use]
    pub fn isp_mem_clk_force_on(&mut self) -> ISP_MEM_CLK_FORCE_ON_W<CLK_EN_SPEC> {
        ISP_MEM_CLK_FORCE_ON_W::new(self, 18)
    }
}
#[doc = "isp clk control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_en::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_en::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLK_EN_SPEC;
impl crate::RegisterSpec for CLK_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_en::R`](R) reader structure"]
impl crate::Readable for CLK_EN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`clk_en::W`](W) writer structure"]
impl crate::Writable for CLK_EN_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLK_EN to value 0"]
impl crate::Resettable for CLK_EN_SPEC {
    const RESET_VALUE: u32 = 0;
}
