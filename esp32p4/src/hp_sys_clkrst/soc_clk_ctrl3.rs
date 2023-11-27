#[doc = "Register `SOC_CLK_CTRL3` reader"]
pub type R = crate::R<SOC_CLK_CTRL3_SPEC>;
#[doc = "Register `SOC_CLK_CTRL3` writer"]
pub type W = crate::W<SOC_CLK_CTRL3_SPEC>;
#[doc = "Field `REG_LEDC_APB_CLK_EN` reader - Reserved"]
pub type REG_LEDC_APB_CLK_EN_R = crate::BitReader;
#[doc = "Field `REG_LEDC_APB_CLK_EN` writer - Reserved"]
pub type REG_LEDC_APB_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_LCDCAM_APB_CLK_EN` reader - Reserved"]
pub type REG_LCDCAM_APB_CLK_EN_R = crate::BitReader;
#[doc = "Field `REG_LCDCAM_APB_CLK_EN` writer - Reserved"]
pub type REG_LCDCAM_APB_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_ETM_APB_CLK_EN` reader - Reserved"]
pub type REG_ETM_APB_CLK_EN_R = crate::BitReader;
#[doc = "Field `REG_ETM_APB_CLK_EN` writer - Reserved"]
pub type REG_ETM_APB_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_IOMUX_APB_CLK_EN` reader - Reserved"]
pub type REG_IOMUX_APB_CLK_EN_R = crate::BitReader;
#[doc = "Field `REG_IOMUX_APB_CLK_EN` writer - Reserved"]
pub type REG_IOMUX_APB_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Reserved"]
    #[inline(always)]
    pub fn reg_ledc_apb_clk_en(&self) -> REG_LEDC_APB_CLK_EN_R {
        REG_LEDC_APB_CLK_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Reserved"]
    #[inline(always)]
    pub fn reg_lcdcam_apb_clk_en(&self) -> REG_LCDCAM_APB_CLK_EN_R {
        REG_LCDCAM_APB_CLK_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Reserved"]
    #[inline(always)]
    pub fn reg_etm_apb_clk_en(&self) -> REG_ETM_APB_CLK_EN_R {
        REG_ETM_APB_CLK_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Reserved"]
    #[inline(always)]
    pub fn reg_iomux_apb_clk_en(&self) -> REG_IOMUX_APB_CLK_EN_R {
        REG_IOMUX_APB_CLK_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SOC_CLK_CTRL3")
            .field(
                "reg_ledc_apb_clk_en",
                &format_args!("{}", self.reg_ledc_apb_clk_en().bit()),
            )
            .field(
                "reg_lcdcam_apb_clk_en",
                &format_args!("{}", self.reg_lcdcam_apb_clk_en().bit()),
            )
            .field(
                "reg_etm_apb_clk_en",
                &format_args!("{}", self.reg_etm_apb_clk_en().bit()),
            )
            .field(
                "reg_iomux_apb_clk_en",
                &format_args!("{}", self.reg_iomux_apb_clk_en().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SOC_CLK_CTRL3_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reg_ledc_apb_clk_en(&mut self) -> REG_LEDC_APB_CLK_EN_W<SOC_CLK_CTRL3_SPEC> {
        REG_LEDC_APB_CLK_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reg_lcdcam_apb_clk_en(&mut self) -> REG_LCDCAM_APB_CLK_EN_W<SOC_CLK_CTRL3_SPEC> {
        REG_LCDCAM_APB_CLK_EN_W::new(self, 1)
    }
    #[doc = "Bit 2 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reg_etm_apb_clk_en(&mut self) -> REG_ETM_APB_CLK_EN_W<SOC_CLK_CTRL3_SPEC> {
        REG_ETM_APB_CLK_EN_W::new(self, 2)
    }
    #[doc = "Bit 3 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reg_iomux_apb_clk_en(&mut self) -> REG_IOMUX_APB_CLK_EN_W<SOC_CLK_CTRL3_SPEC> {
        REG_IOMUX_APB_CLK_EN_W::new(self, 3)
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
#[doc = "Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`soc_clk_ctrl3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`soc_clk_ctrl3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SOC_CLK_CTRL3_SPEC;
impl crate::RegisterSpec for SOC_CLK_CTRL3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`soc_clk_ctrl3::R`](R) reader structure"]
impl crate::Readable for SOC_CLK_CTRL3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`soc_clk_ctrl3::W`](W) writer structure"]
impl crate::Writable for SOC_CLK_CTRL3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SOC_CLK_CTRL3 to value 0x08"]
impl crate::Resettable for SOC_CLK_CTRL3_SPEC {
    const RESET_VALUE: Self::Ux = 0x08;
}
