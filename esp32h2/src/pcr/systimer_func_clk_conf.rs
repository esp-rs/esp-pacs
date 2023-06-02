#[doc = "Register `SYSTIMER_FUNC_CLK_CONF` reader"]
pub struct R(crate::R<SYSTIMER_FUNC_CLK_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYSTIMER_FUNC_CLK_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYSTIMER_FUNC_CLK_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYSTIMER_FUNC_CLK_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYSTIMER_FUNC_CLK_CONF` writer"]
pub struct W(crate::W<SYSTIMER_FUNC_CLK_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYSTIMER_FUNC_CLK_CONF_SPEC>;
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
impl From<crate::W<SYSTIMER_FUNC_CLK_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYSTIMER_FUNC_CLK_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYSTIMER_FUNC_CLK_SEL` reader - set this field to select clock-source. 0(default): XTAL, 1: FOSC."]
pub type SYSTIMER_FUNC_CLK_SEL_R = crate::BitReader;
#[doc = "Field `SYSTIMER_FUNC_CLK_SEL` writer - set this field to select clock-source. 0(default): XTAL, 1: FOSC."]
pub type SYSTIMER_FUNC_CLK_SEL_W<'a, const O: u8> =
    crate::BitWriter<'a, SYSTIMER_FUNC_CLK_CONF_SPEC, O>;
#[doc = "Field `SYSTIMER_FUNC_CLK_EN` reader - Set 1 to enable systimer function clock"]
pub type SYSTIMER_FUNC_CLK_EN_R = crate::BitReader;
#[doc = "Field `SYSTIMER_FUNC_CLK_EN` writer - Set 1 to enable systimer function clock"]
pub type SYSTIMER_FUNC_CLK_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, SYSTIMER_FUNC_CLK_CONF_SPEC, O>;
impl R {
    #[doc = "Bit 20 - set this field to select clock-source. 0(default): XTAL, 1: FOSC."]
    #[inline(always)]
    pub fn systimer_func_clk_sel(&self) -> SYSTIMER_FUNC_CLK_SEL_R {
        SYSTIMER_FUNC_CLK_SEL_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 22 - Set 1 to enable systimer function clock"]
    #[inline(always)]
    pub fn systimer_func_clk_en(&self) -> SYSTIMER_FUNC_CLK_EN_R {
        SYSTIMER_FUNC_CLK_EN_R::new(((self.bits >> 22) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SYSTIMER_FUNC_CLK_CONF")
            .field(
                "systimer_func_clk_sel",
                &format_args!("{}", self.systimer_func_clk_sel().bit()),
            )
            .field(
                "systimer_func_clk_en",
                &format_args!("{}", self.systimer_func_clk_en().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SYSTIMER_FUNC_CLK_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 20 - set this field to select clock-source. 0(default): XTAL, 1: FOSC."]
    #[inline(always)]
    #[must_use]
    pub fn systimer_func_clk_sel(&mut self) -> SYSTIMER_FUNC_CLK_SEL_W<20> {
        SYSTIMER_FUNC_CLK_SEL_W::new(self)
    }
    #[doc = "Bit 22 - Set 1 to enable systimer function clock"]
    #[inline(always)]
    #[must_use]
    pub fn systimer_func_clk_en(&mut self) -> SYSTIMER_FUNC_CLK_EN_W<22> {
        SYSTIMER_FUNC_CLK_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SYSTIMER_FUNC_CLK configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [systimer_func_clk_conf](index.html) module"]
pub struct SYSTIMER_FUNC_CLK_CONF_SPEC;
impl crate::RegisterSpec for SYSTIMER_FUNC_CLK_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [systimer_func_clk_conf::R](R) reader structure"]
impl crate::Readable for SYSTIMER_FUNC_CLK_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [systimer_func_clk_conf::W](W) writer structure"]
impl crate::Writable for SYSTIMER_FUNC_CLK_CONF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SYSTIMER_FUNC_CLK_CONF to value 0x0040_0000"]
impl crate::Resettable for SYSTIMER_FUNC_CLK_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0x0040_0000;
}
