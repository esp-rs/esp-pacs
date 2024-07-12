#[doc = "Register `SOC_CLK_CTRL3` reader"]
pub type R = crate::R<SOC_CLK_CTRL3_SPEC>;
#[doc = "Register `SOC_CLK_CTRL3` writer"]
pub type W = crate::W<SOC_CLK_CTRL3_SPEC>;
#[doc = "Field `LEDC_APB_CLK_EN` reader - Reserved"]
pub type LEDC_APB_CLK_EN_R = crate::BitReader;
#[doc = "Field `LEDC_APB_CLK_EN` writer - Reserved"]
pub type LEDC_APB_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCDCAM_APB_CLK_EN` reader - Reserved"]
pub type LCDCAM_APB_CLK_EN_R = crate::BitReader;
#[doc = "Field `LCDCAM_APB_CLK_EN` writer - Reserved"]
pub type LCDCAM_APB_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETM_APB_CLK_EN` reader - Reserved"]
pub type ETM_APB_CLK_EN_R = crate::BitReader;
#[doc = "Field `ETM_APB_CLK_EN` writer - Reserved"]
pub type ETM_APB_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOMUX_APB_CLK_EN` reader - Reserved"]
pub type IOMUX_APB_CLK_EN_R = crate::BitReader;
#[doc = "Field `IOMUX_APB_CLK_EN` writer - Reserved"]
pub type IOMUX_APB_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Reserved"]
    #[inline(always)]
    pub fn ledc_apb_clk_en(&self) -> LEDC_APB_CLK_EN_R {
        LEDC_APB_CLK_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Reserved"]
    #[inline(always)]
    pub fn lcdcam_apb_clk_en(&self) -> LCDCAM_APB_CLK_EN_R {
        LCDCAM_APB_CLK_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Reserved"]
    #[inline(always)]
    pub fn etm_apb_clk_en(&self) -> ETM_APB_CLK_EN_R {
        ETM_APB_CLK_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Reserved"]
    #[inline(always)]
    pub fn iomux_apb_clk_en(&self) -> IOMUX_APB_CLK_EN_R {
        IOMUX_APB_CLK_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SOC_CLK_CTRL3")
            .field("ledc_apb_clk_en", &self.ledc_apb_clk_en())
            .field("lcdcam_apb_clk_en", &self.lcdcam_apb_clk_en())
            .field("etm_apb_clk_en", &self.etm_apb_clk_en())
            .field("iomux_apb_clk_en", &self.iomux_apb_clk_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn ledc_apb_clk_en(&mut self) -> LEDC_APB_CLK_EN_W<SOC_CLK_CTRL3_SPEC> {
        LEDC_APB_CLK_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn lcdcam_apb_clk_en(&mut self) -> LCDCAM_APB_CLK_EN_W<SOC_CLK_CTRL3_SPEC> {
        LCDCAM_APB_CLK_EN_W::new(self, 1)
    }
    #[doc = "Bit 2 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn etm_apb_clk_en(&mut self) -> ETM_APB_CLK_EN_W<SOC_CLK_CTRL3_SPEC> {
        ETM_APB_CLK_EN_W::new(self, 2)
    }
    #[doc = "Bit 3 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn iomux_apb_clk_en(&mut self) -> IOMUX_APB_CLK_EN_W<SOC_CLK_CTRL3_SPEC> {
        IOMUX_APB_CLK_EN_W::new(self, 3)
    }
}
#[doc = "Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`soc_clk_ctrl3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`soc_clk_ctrl3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SOC_CLK_CTRL3_SPEC;
impl crate::RegisterSpec for SOC_CLK_CTRL3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`soc_clk_ctrl3::R`](R) reader structure"]
impl crate::Readable for SOC_CLK_CTRL3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`soc_clk_ctrl3::W`](W) writer structure"]
impl crate::Writable for SOC_CLK_CTRL3_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SOC_CLK_CTRL3 to value 0x08"]
impl crate::Resettable for SOC_CLK_CTRL3_SPEC {
    const RESET_VALUE: u32 = 0x08;
}
