#[doc = "Register `PLL_TICK_CONF` reader"]
pub struct R(crate::R<PLL_TICK_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PLL_TICK_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PLL_TICK_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PLL_TICK_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PLL_TICK_CONF` writer"]
pub struct W(crate::W<PLL_TICK_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PLL_TICK_CONF_SPEC>;
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
impl From<crate::W<PLL_TICK_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PLL_TICK_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PLL_TICK_NUM` reader - "]
pub type PLL_TICK_NUM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PLL_TICK_NUM` writer - "]
pub type PLL_TICK_NUM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PLL_TICK_CONF_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn pll_tick_num(&self) -> PLL_TICK_NUM_R {
        PLL_TICK_NUM_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn pll_tick_num(&mut self) -> PLL_TICK_NUM_W<0> {
        PLL_TICK_NUM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pll_tick_conf](index.html) module"]
pub struct PLL_TICK_CONF_SPEC;
impl crate::RegisterSpec for PLL_TICK_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pll_tick_conf::R](R) reader structure"]
impl crate::Readable for PLL_TICK_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pll_tick_conf::W](W) writer structure"]
impl crate::Writable for PLL_TICK_CONF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PLL_TICK_CONF to value 0x4f"]
impl crate::Resettable for PLL_TICK_CONF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x4f
    }
}
