#[doc = "Register `ECDSA_CONF` reader"]
pub type R = crate::R<ECDSA_CONF_SPEC>;
#[doc = "Register `ECDSA_CONF` writer"]
pub type W = crate::W<ECDSA_CONF_SPEC>;
#[doc = "Field `ECDSA_CLK_EN` reader - Set 1 to enable ecdsa clock"]
pub type ECDSA_CLK_EN_R = crate::BitReader;
#[doc = "Field `ECDSA_CLK_EN` writer - Set 1 to enable ecdsa clock"]
pub type ECDSA_CLK_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ECDSA_RST_EN` reader - Set 0 to reset ecdsa module"]
pub type ECDSA_RST_EN_R = crate::BitReader;
#[doc = "Field `ECDSA_RST_EN` writer - Set 0 to reset ecdsa module"]
pub type ECDSA_RST_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ECDSA_READY` reader - Query this field after reset ecdsa module"]
pub type ECDSA_READY_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Set 1 to enable ecdsa clock"]
    #[inline(always)]
    pub fn ecdsa_clk_en(&self) -> ECDSA_CLK_EN_R {
        ECDSA_CLK_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set 0 to reset ecdsa module"]
    #[inline(always)]
    pub fn ecdsa_rst_en(&self) -> ECDSA_RST_EN_R {
        ECDSA_RST_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Query this field after reset ecdsa module"]
    #[inline(always)]
    pub fn ecdsa_ready(&self) -> ECDSA_READY_R {
        ECDSA_READY_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ECDSA_CONF")
            .field(
                "ecdsa_clk_en",
                &format_args!("{}", self.ecdsa_clk_en().bit()),
            )
            .field(
                "ecdsa_rst_en",
                &format_args!("{}", self.ecdsa_rst_en().bit()),
            )
            .field("ecdsa_ready", &format_args!("{}", self.ecdsa_ready().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<ECDSA_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Set 1 to enable ecdsa clock"]
    #[inline(always)]
    #[must_use]
    pub fn ecdsa_clk_en(&mut self) -> ECDSA_CLK_EN_W<ECDSA_CONF_SPEC, 0> {
        ECDSA_CLK_EN_W::new(self)
    }
    #[doc = "Bit 1 - Set 0 to reset ecdsa module"]
    #[inline(always)]
    #[must_use]
    pub fn ecdsa_rst_en(&mut self) -> ECDSA_RST_EN_W<ECDSA_CONF_SPEC, 1> {
        ECDSA_RST_EN_W::new(self)
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
#[doc = "ECDSA configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecdsa_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecdsa_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ECDSA_CONF_SPEC;
impl crate::RegisterSpec for ECDSA_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ecdsa_conf::R`](R) reader structure"]
impl crate::Readable for ECDSA_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ecdsa_conf::W`](W) writer structure"]
impl crate::Writable for ECDSA_CONF_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ECDSA_CONF to value 0x05"]
impl crate::Resettable for ECDSA_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0x05;
}
