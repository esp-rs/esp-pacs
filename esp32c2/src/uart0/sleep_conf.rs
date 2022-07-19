#[doc = "Register `SLEEP_CONF` reader"]
pub struct R(crate::R<SLEEP_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SLEEP_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SLEEP_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SLEEP_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SLEEP_CONF` writer"]
pub struct W(crate::W<SLEEP_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SLEEP_CONF_SPEC>;
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
impl From<crate::W<SLEEP_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SLEEP_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ACTIVE_THRESHOLD` reader - The uart is activated from light sleeping mode when the input rxd edge changes more times than this register value."]
pub type ACTIVE_THRESHOLD_R = crate::FieldReader<u16, u16>;
#[doc = "Field `ACTIVE_THRESHOLD` writer - The uart is activated from light sleeping mode when the input rxd edge changes more times than this register value."]
pub type ACTIVE_THRESHOLD_W<'a> = crate::FieldWriter<'a, u32, SLEEP_CONF_SPEC, u16, u16, 10, 0>;
impl R {
    #[doc = "Bits 0:9 - The uart is activated from light sleeping mode when the input rxd edge changes more times than this register value."]
    #[inline(always)]
    pub fn active_threshold(&self) -> ACTIVE_THRESHOLD_R {
        ACTIVE_THRESHOLD_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - The uart is activated from light sleeping mode when the input rxd edge changes more times than this register value."]
    #[inline(always)]
    pub fn active_threshold(&mut self) -> ACTIVE_THRESHOLD_W {
        ACTIVE_THRESHOLD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Sleep-mode configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sleep_conf](index.html) module"]
pub struct SLEEP_CONF_SPEC;
impl crate::RegisterSpec for SLEEP_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sleep_conf::R](R) reader structure"]
impl crate::Readable for SLEEP_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sleep_conf::W](W) writer structure"]
impl crate::Writable for SLEEP_CONF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SLEEP_CONF to value 0xf0"]
impl crate::Resettable for SLEEP_CONF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xf0
    }
}
