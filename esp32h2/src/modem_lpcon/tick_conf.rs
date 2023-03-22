#[doc = "Register `TICK_CONF` reader"]
pub struct R(crate::R<TICK_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TICK_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TICK_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TICK_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TICK_CONF` writer"]
pub struct W(crate::W<TICK_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TICK_CONF_SPEC>;
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
impl From<crate::W<TICK_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TICK_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PWR_TICK_TARGET` reader - "]
pub type PWR_TICK_TARGET_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PWR_TICK_TARGET` writer - "]
pub type PWR_TICK_TARGET_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TICK_CONF_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn pwr_tick_target(&self) -> PWR_TICK_TARGET_R {
        PWR_TICK_TARGET_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    #[must_use]
    pub fn pwr_tick_target(&mut self) -> PWR_TICK_TARGET_W<0> {
        PWR_TICK_TARGET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tick_conf](index.html) module"]
pub struct TICK_CONF_SPEC;
impl crate::RegisterSpec for TICK_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tick_conf::R](R) reader structure"]
impl crate::Readable for TICK_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tick_conf::W](W) writer structure"]
impl crate::Writable for TICK_CONF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TICK_CONF to value 0x1f"]
impl crate::Resettable for TICK_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0x1f;
}
