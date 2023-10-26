#[doc = "Register `KEY_7` reader"]
pub type R = crate::R<KEY_7_SPEC>;
#[doc = "Register `KEY_7` writer"]
pub type W = crate::W<KEY_7_SPEC>;
#[doc = "Field `KEY_7` reader - This bits stores key_7 that is a part of key material."]
pub type KEY_7_R = crate::FieldReader<u32>;
#[doc = "Field `KEY_7` writer - This bits stores key_7 that is a part of key material."]
pub type KEY_7_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - This bits stores key_7 that is a part of key material."]
    #[inline(always)]
    pub fn key_7(&self) -> KEY_7_R {
        KEY_7_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("KEY_7")
            .field("key_7", &format_args!("{}", self.key_7().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<KEY_7_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - This bits stores key_7 that is a part of key material."]
    #[inline(always)]
    #[must_use]
    pub fn key_7(&mut self) -> KEY_7_W<KEY_7_SPEC, 0> {
        KEY_7_W::new(self)
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
#[doc = "Key material key_7 configure register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`key_7::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`key_7::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KEY_7_SPEC;
impl crate::RegisterSpec for KEY_7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`key_7::R`](R) reader structure"]
impl crate::Readable for KEY_7_SPEC {}
#[doc = "`write(|w| ..)` method takes [`key_7::W`](W) writer structure"]
impl crate::Writable for KEY_7_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets KEY_7 to value 0"]
impl crate::Resettable for KEY_7_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
