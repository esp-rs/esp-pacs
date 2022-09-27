#[doc = "Register `INTERRUPT` reader"]
pub struct R(crate::R<INTERRUPT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTERRUPT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTERRUPT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTERRUPT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTERRUPT` writer"]
pub struct W(crate::W<INTERRUPT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTERRUPT_SPEC>;
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
impl From<crate::W<INTERRUPT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTERRUPT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INTERRUPT` reader - RSA interrupt status register. Will read 1 once an operation has completed."]
pub type INTERRUPT_R = crate::BitReader<bool>;
#[doc = "Field `INTERRUPT` writer - RSA interrupt status register. Will read 1 once an operation has completed."]
pub type INTERRUPT_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTERRUPT_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - RSA interrupt status register. Will read 1 once an operation has completed."]
    #[inline(always)]
    pub fn interrupt(&self) -> INTERRUPT_R {
        INTERRUPT_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RSA interrupt status register. Will read 1 once an operation has completed."]
    #[inline(always)]
    pub fn interrupt(&mut self) -> INTERRUPT_W<0> {
        INTERRUPT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [interrupt](index.html) module"]
pub struct INTERRUPT_SPEC;
impl crate::RegisterSpec for INTERRUPT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [interrupt::R](R) reader structure"]
impl crate::Readable for INTERRUPT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [interrupt::W](W) writer structure"]
impl crate::Writable for INTERRUPT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTERRUPT to value 0"]
impl crate::Resettable for INTERRUPT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
