#[doc = "Register `TRACE_CONF` reader"]
pub type R = crate::R<TRACE_CONF_SPEC>;
#[doc = "Register `TRACE_CONF` writer"]
pub type W = crate::W<TRACE_CONF_SPEC>;
#[doc = "Field `TRACE_CLK_EN` reader - Set 1 to enable trace clock"]
pub type TRACE_CLK_EN_R = crate::BitReader;
#[doc = "Field `TRACE_CLK_EN` writer - Set 1 to enable trace clock"]
pub type TRACE_CLK_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TRACE_RST_EN` reader - Set 0 to reset trace module"]
pub type TRACE_RST_EN_R = crate::BitReader;
#[doc = "Field `TRACE_RST_EN` writer - Set 0 to reset trace module"]
pub type TRACE_RST_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Set 1 to enable trace clock"]
    #[inline(always)]
    pub fn trace_clk_en(&self) -> TRACE_CLK_EN_R {
        TRACE_CLK_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set 0 to reset trace module"]
    #[inline(always)]
    pub fn trace_rst_en(&self) -> TRACE_RST_EN_R {
        TRACE_RST_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TRACE_CONF")
            .field(
                "trace_clk_en",
                &format_args!("{}", self.trace_clk_en().bit()),
            )
            .field(
                "trace_rst_en",
                &format_args!("{}", self.trace_rst_en().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TRACE_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Set 1 to enable trace clock"]
    #[inline(always)]
    #[must_use]
    pub fn trace_clk_en(&mut self) -> TRACE_CLK_EN_W<TRACE_CONF_SPEC, 0> {
        TRACE_CLK_EN_W::new(self)
    }
    #[doc = "Bit 1 - Set 0 to reset trace module"]
    #[inline(always)]
    #[must_use]
    pub fn trace_rst_en(&mut self) -> TRACE_RST_EN_W<TRACE_CONF_SPEC, 1> {
        TRACE_RST_EN_W::new(self)
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
#[doc = "TRACE configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`trace_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`trace_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TRACE_CONF_SPEC;
impl crate::RegisterSpec for TRACE_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trace_conf::R`](R) reader structure"]
impl crate::Readable for TRACE_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`trace_conf::W`](W) writer structure"]
impl crate::Writable for TRACE_CONF_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TRACE_CONF to value 0x01"]
impl crate::Resettable for TRACE_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
