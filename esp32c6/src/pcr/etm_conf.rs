#[doc = "Register `ETM_CONF` reader"]
pub type R = crate::R<ETM_CONF_SPEC>;
#[doc = "Register `ETM_CONF` writer"]
pub type W = crate::W<ETM_CONF_SPEC>;
#[doc = "Field `ETM_CLK_EN` reader - Set 1 to enable etm clock"]
pub type ETM_CLK_EN_R = crate::BitReader;
#[doc = "Field `ETM_CLK_EN` writer - Set 1 to enable etm clock"]
pub type ETM_CLK_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ETM_RST_EN` reader - Set 0 to reset etm module"]
pub type ETM_RST_EN_R = crate::BitReader;
#[doc = "Field `ETM_RST_EN` writer - Set 0 to reset etm module"]
pub type ETM_RST_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Set 1 to enable etm clock"]
    #[inline(always)]
    pub fn etm_clk_en(&self) -> ETM_CLK_EN_R {
        ETM_CLK_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set 0 to reset etm module"]
    #[inline(always)]
    pub fn etm_rst_en(&self) -> ETM_RST_EN_R {
        ETM_RST_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ETM_CONF")
            .field("etm_clk_en", &format_args!("{}", self.etm_clk_en().bit()))
            .field("etm_rst_en", &format_args!("{}", self.etm_rst_en().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<ETM_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Set 1 to enable etm clock"]
    #[inline(always)]
    #[must_use]
    pub fn etm_clk_en(&mut self) -> ETM_CLK_EN_W<ETM_CONF_SPEC, 0> {
        ETM_CLK_EN_W::new(self)
    }
    #[doc = "Bit 1 - Set 0 to reset etm module"]
    #[inline(always)]
    #[must_use]
    pub fn etm_rst_en(&mut self) -> ETM_RST_EN_W<ETM_CONF_SPEC, 1> {
        ETM_RST_EN_W::new(self)
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
#[doc = "ETM configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`etm_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`etm_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ETM_CONF_SPEC;
impl crate::RegisterSpec for ETM_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`etm_conf::R`](R) reader structure"]
impl crate::Readable for ETM_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`etm_conf::W`](W) writer structure"]
impl crate::Writable for ETM_CONF_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ETM_CONF to value 0x01"]
impl crate::Resettable for ETM_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
