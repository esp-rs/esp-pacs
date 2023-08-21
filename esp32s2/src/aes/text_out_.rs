#[doc = "Register `TEXT_OUT_%s` reader"]
pub type R = crate::R<TEXT_OUT__SPEC>;
#[doc = "Register `TEXT_OUT_%s` writer"]
pub type W = crate::W<TEXT_OUT__SPEC>;
#[doc = "Field `TEXT_OUT` reader - Stores the result data when the AES Accelerator operates in the Typical AES working mode."]
pub type TEXT_OUT_R = crate::FieldReader<u32>;
#[doc = "Field `TEXT_OUT` writer - Stores the result data when the AES Accelerator operates in the Typical AES working mode."]
pub type TEXT_OUT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Stores the result data when the AES Accelerator operates in the Typical AES working mode."]
    #[inline(always)]
    pub fn text_out(&self) -> TEXT_OUT_R {
        TEXT_OUT_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TEXT_OUT_")
            .field("text_out", &format_args!("{}", self.text_out().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TEXT_OUT__SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - Stores the result data when the AES Accelerator operates in the Typical AES working mode."]
    #[inline(always)]
    #[must_use]
    pub fn text_out(&mut self) -> TEXT_OUT_W<TEXT_OUT__SPEC, 0> {
        TEXT_OUT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Result data register %s\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`text_out_::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`text_out_::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TEXT_OUT__SPEC;
impl crate::RegisterSpec for TEXT_OUT__SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`text_out_::R`](R) reader structure"]
impl crate::Readable for TEXT_OUT__SPEC {}
#[doc = "`write(|w| ..)` method takes [`text_out_::W`](W) writer structure"]
impl crate::Writable for TEXT_OUT__SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TEXT_OUT_%s to value 0"]
impl crate::Resettable for TEXT_OUT__SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
