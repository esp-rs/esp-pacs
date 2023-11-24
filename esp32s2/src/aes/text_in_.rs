#[doc = "Register `TEXT_IN_%s` reader"]
pub type R = crate::R<TEXT_IN__SPEC>;
#[doc = "Register `TEXT_IN_%s` writer"]
pub type W = crate::W<TEXT_IN__SPEC>;
#[doc = "Field `TEXT_IN` reader - Stores the source data when the AES Accelerator operates in the Typical AES working mode."]
pub type TEXT_IN_R = crate::FieldReader<u32>;
#[doc = "Field `TEXT_IN` writer - Stores the source data when the AES Accelerator operates in the Typical AES working mode."]
pub type TEXT_IN_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Stores the source data when the AES Accelerator operates in the Typical AES working mode."]
    #[inline(always)]
    pub fn text_in(&self) -> TEXT_IN_R {
        TEXT_IN_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TEXT_IN_")
            .field("text_in", &format_args!("{}", self.text_in().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TEXT_IN__SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31 - Stores the source data when the AES Accelerator operates in the Typical AES working mode."]
    #[inline(always)]
    #[must_use]
    pub fn text_in(&mut self) -> TEXT_IN_W<TEXT_IN__SPEC> {
        TEXT_IN_W::new(self, 0)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Source data register %s\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`text_in_::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`text_in_::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TEXT_IN__SPEC;
impl crate::RegisterSpec for TEXT_IN__SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`text_in_::R`](R) reader structure"]
impl crate::Readable for TEXT_IN__SPEC {}
#[doc = "`write(|w| ..)` method takes [`text_in_::W`](W) writer structure"]
impl crate::Writable for TEXT_IN__SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TEXT_IN_%s to value 0"]
impl crate::Resettable for TEXT_IN__SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
