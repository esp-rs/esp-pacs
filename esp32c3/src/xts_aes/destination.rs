#[doc = "Register `DESTINATION` reader"]
pub struct R(crate::R<DESTINATION_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DESTINATION_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DESTINATION_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DESTINATION_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DESTINATION` writer"]
pub struct W(crate::W<DESTINATION_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DESTINATION_SPEC>;
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
impl From<crate::W<DESTINATION_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DESTINATION_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DESTINATION` reader - This bit stores the destination. 0: flash(default). 1: reserved."]
pub type DESTINATION_R = crate::BitReader;
#[doc = "Field `DESTINATION` writer - This bit stores the destination. 0: flash(default). 1: reserved."]
pub type DESTINATION_W<'a, const O: u8> = crate::BitWriter<'a, DESTINATION_SPEC, O>;
impl R {
    #[doc = "Bit 0 - This bit stores the destination. 0: flash(default). 1: reserved."]
    #[inline(always)]
    pub fn destination(&self) -> DESTINATION_R {
        DESTINATION_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DESTINATION")
            .field("destination", &format_args!("{}", self.destination().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DESTINATION_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - This bit stores the destination. 0: flash(default). 1: reserved."]
    #[inline(always)]
    #[must_use]
    pub fn destination(&mut self) -> DESTINATION_W<0> {
        DESTINATION_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "XTS-AES destination register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [destination](index.html) module"]
pub struct DESTINATION_SPEC;
impl crate::RegisterSpec for DESTINATION_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [destination::R](R) reader structure"]
impl crate::Readable for DESTINATION_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [destination::W](W) writer structure"]
impl crate::Writable for DESTINATION_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DESTINATION to value 0"]
impl crate::Resettable for DESTINATION_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
