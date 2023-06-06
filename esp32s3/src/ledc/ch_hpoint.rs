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
#[doc = "Field `HPOINT` reader - The output value changes to high when the selected timers has reached the value specified by this register."]
pub type HPOINT_R = crate::FieldReader<u16>;
#[doc = "Field `HPOINT` writer - The output value changes to high when the selected timers has reached the value specified by this register."]
pub type HPOINT_W<'a, const O: u8> = crate::FieldWriter<'a, CH_HPOINT_SPEC, 14, O, u16>;
impl R {
    #[doc = "Bits 0:13 - The output value changes to high when the selected timers has reached the value specified by this register."]
    #[inline(always)]
    pub fn hpoint(&self) -> HPOINT_R {
        HPOINT_R::new((self.bits & 0x3fff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CH_HPOINT")
            .field("hpoint", &format_args!("{}", self.hpoint().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CH_HPOINT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:13 - The output value changes to high when the selected timers has reached the value specified by this register."]
    #[inline(always)]
    #[must_use]
    pub fn hpoint(&mut self) -> HPOINT_W<0> {
        HPOINT_W::new(self)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CH%s_HPOINT to value 0"]
impl crate::Resettable for CH_HPOINT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
