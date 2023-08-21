#[doc = "Register `ECC_CONF` reader"]
pub type R = crate::R<ECC_CONF_SPEC>;
#[doc = "Register `ECC_CONF` writer"]
pub type W = crate::W<ECC_CONF_SPEC>;
#[doc = "Field `ECC_CLK_EN` reader - Set 1 to enable ecc clock"]
pub type ECC_CLK_EN_R = crate::BitReader;
#[doc = "Field `ECC_CLK_EN` writer - Set 1 to enable ecc clock"]
pub type ECC_CLK_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ECC_RST_EN` reader - Set 0 to reset ecc module"]
pub type ECC_RST_EN_R = crate::BitReader;
#[doc = "Field `ECC_RST_EN` writer - Set 0 to reset ecc module"]
pub type ECC_RST_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Set 1 to enable ecc clock"]
    #[inline(always)]
    pub fn ecc_clk_en(&self) -> ECC_CLK_EN_R {
        ECC_CLK_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set 0 to reset ecc module"]
    #[inline(always)]
    pub fn ecc_rst_en(&self) -> ECC_RST_EN_R {
        ECC_RST_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ECC_CONF")
            .field("ecc_clk_en", &format_args!("{}", self.ecc_clk_en().bit()))
            .field("ecc_rst_en", &format_args!("{}", self.ecc_rst_en().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<ECC_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Set 1 to enable ecc clock"]
    #[inline(always)]
    #[must_use]
    pub fn ecc_clk_en(&mut self) -> ECC_CLK_EN_W<ECC_CONF_SPEC, 0> {
        ECC_CLK_EN_W::new(self)
    }
    #[doc = "Bit 1 - Set 0 to reset ecc module"]
    #[inline(always)]
    #[must_use]
    pub fn ecc_rst_en(&mut self) -> ECC_RST_EN_W<ECC_CONF_SPEC, 1> {
        ECC_RST_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "ECC configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecc_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecc_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ECC_CONF_SPEC;
impl crate::RegisterSpec for ECC_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ecc_conf::R`](R) reader structure"]
impl crate::Readable for ECC_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ecc_conf::W`](W) writer structure"]
impl crate::Writable for ECC_CONF_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ECC_CONF to value 0x01"]
impl crate::Resettable for ECC_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
