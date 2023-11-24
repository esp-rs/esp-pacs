#[doc = "Register `KEY_%s` reader"]
pub type R = crate::R<KEY__SPEC>;
#[doc = "Register `KEY_%s` writer"]
pub type W = crate::W<KEY__SPEC>;
#[doc = "Field `KEY` reader - Stores AES keys."]
pub type KEY_R = crate::FieldReader<u32>;
#[doc = "Field `KEY` writer - Stores AES keys."]
pub type KEY_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Stores AES keys."]
    #[inline(always)]
    pub fn key(&self) -> KEY_R {
        KEY_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("KEY_")
            .field("key", &format_args!("{}", self.key().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<KEY__SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31 - Stores AES keys."]
    #[inline(always)]
    #[must_use]
    pub fn key(&mut self) -> KEY_W<KEY__SPEC> {
        KEY_W::new(self, 0)
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
#[doc = "AES key register %s\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`key_::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`key_::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KEY__SPEC;
impl crate::RegisterSpec for KEY__SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`key_::R`](R) reader structure"]
impl crate::Readable for KEY__SPEC {}
#[doc = "`write(|w| ..)` method takes [`key_::W`](W) writer structure"]
impl crate::Writable for KEY__SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets KEY_%s to value 0"]
impl crate::Resettable for KEY__SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
