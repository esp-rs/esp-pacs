#[doc = "Register `CH%s_HPOINT` reader"]
pub struct R(crate::R<CH_HPOINT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CH_HPOINT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CH_HPOINT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CH_HPOINT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CH%s_HPOINT` writer"]
pub struct W(crate::W<CH_HPOINT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CH_HPOINT_SPEC>;
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
impl From<crate::W<CH_HPOINT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CH_HPOINT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HPOINT_CH0` reader - The output value changes to high when the selected timers has reached the value specified by this register."]
pub type HPOINT_CH0_R = crate::FieldReader<u16, u16>;
#[doc = "Field `HPOINT_CH0` writer - The output value changes to high when the selected timers has reached the value specified by this register."]
pub type HPOINT_CH0_W<'a> = crate::FieldWriter<'a, u32, CH_HPOINT_SPEC, u16, u16, 14, 0>;
impl R {
    #[doc = "Bits 0:13 - The output value changes to high when the selected timers has reached the value specified by this register."]
    #[inline(always)]
    pub fn hpoint_ch0(&self) -> HPOINT_CH0_R {
        HPOINT_CH0_R::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - The output value changes to high when the selected timers has reached the value specified by this register."]
    #[inline(always)]
    pub fn hpoint_ch0(&mut self) -> HPOINT_CH0_W {
        HPOINT_CH0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "High point register for channel %s\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch_hpoint](index.html) module"]
pub struct CH_HPOINT_SPEC;
impl crate::RegisterSpec for CH_HPOINT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ch_hpoint::R](R) reader structure"]
impl crate::Readable for CH_HPOINT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ch_hpoint::W](W) writer structure"]
impl crate::Writable for CH_HPOINT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CH%s_HPOINT to value 0"]
impl crate::Resettable for CH_HPOINT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
