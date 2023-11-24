#[doc = "Register `LEDC_CONF` reader"]
pub type R = crate::R<LEDC_CONF_SPEC>;
#[doc = "Register `LEDC_CONF` writer"]
pub type W = crate::W<LEDC_CONF_SPEC>;
#[doc = "Field `LEDC_CLK_EN` reader - Set 1 to enable ledc apb clock"]
pub type LEDC_CLK_EN_R = crate::BitReader;
#[doc = "Field `LEDC_CLK_EN` writer - Set 1 to enable ledc apb clock"]
pub type LEDC_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LEDC_RST_EN` reader - Set 0 to reset ledc module"]
pub type LEDC_RST_EN_R = crate::BitReader;
#[doc = "Field `LEDC_RST_EN` writer - Set 0 to reset ledc module"]
pub type LEDC_RST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Set 1 to enable ledc apb clock"]
    #[inline(always)]
    pub fn ledc_clk_en(&self) -> LEDC_CLK_EN_R {
        LEDC_CLK_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set 0 to reset ledc module"]
    #[inline(always)]
    pub fn ledc_rst_en(&self) -> LEDC_RST_EN_R {
        LEDC_RST_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LEDC_CONF")
            .field("ledc_clk_en", &format_args!("{}", self.ledc_clk_en().bit()))
            .field("ledc_rst_en", &format_args!("{}", self.ledc_rst_en().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<LEDC_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - Set 1 to enable ledc apb clock"]
    #[inline(always)]
    #[must_use]
    pub fn ledc_clk_en(&mut self) -> LEDC_CLK_EN_W<LEDC_CONF_SPEC> {
        LEDC_CLK_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Set 0 to reset ledc module"]
    #[inline(always)]
    #[must_use]
    pub fn ledc_rst_en(&mut self) -> LEDC_RST_EN_W<LEDC_CONF_SPEC> {
        LEDC_RST_EN_W::new(self, 1)
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
#[doc = "LEDC configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ledc_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ledc_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LEDC_CONF_SPEC;
impl crate::RegisterSpec for LEDC_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ledc_conf::R`](R) reader structure"]
impl crate::Readable for LEDC_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ledc_conf::W`](W) writer structure"]
impl crate::Writable for LEDC_CONF_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LEDC_CONF to value 0x01"]
impl crate::Resettable for LEDC_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
