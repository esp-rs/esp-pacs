#[doc = "Register `TRACE_CONF` reader"]
pub struct R(crate::R<TRACE_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TRACE_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TRACE_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TRACE_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TRACE_CONF` writer"]
pub struct W(crate::W<TRACE_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TRACE_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<TRACE_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TRACE_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TRACE_CLK_EN` reader - Set 1 to enable trace clock"]
pub type TRACE_CLK_EN_R = crate::BitReader;
#[doc = "Field `TRACE_CLK_EN` writer - Set 1 to enable trace clock"]
pub type TRACE_CLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, TRACE_CONF_SPEC, O>;
#[doc = "Field `TRACE_RST_EN` reader - Set 0 to reset trace module"]
pub type TRACE_RST_EN_R = crate::BitReader;
#[doc = "Field `TRACE_RST_EN` writer - Set 0 to reset trace module"]
pub type TRACE_RST_EN_W<'a, const O: u8> = crate::BitWriter<'a, TRACE_CONF_SPEC, O>;
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
    pub fn trace_clk_en(&mut self) -> TRACE_CLK_EN_W<0> {
        TRACE_CLK_EN_W::new(self)
    }
    #[doc = "Bit 1 - Set 0 to reset trace module"]
    #[inline(always)]
    #[must_use]
    pub fn trace_rst_en(&mut self) -> TRACE_RST_EN_W<1> {
        TRACE_RST_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TRACE configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trace_conf](index.html) module"]
pub struct TRACE_CONF_SPEC;
impl crate::RegisterSpec for TRACE_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [trace_conf::R](R) reader structure"]
impl crate::Readable for TRACE_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [trace_conf::W](W) writer structure"]
impl crate::Writable for TRACE_CONF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TRACE_CONF to value 0x01"]
impl crate::Resettable for TRACE_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
