#[doc = "Register `KEY[%s]` reader"]
pub type R = crate::R<KEY_SPEC>;
#[doc = "Register `KEY[%s]` writer"]
pub type W = crate::W<KEY_SPEC>;
#[doc = "Field `KEY` reader - AES key material register."]
pub type KEY_R = crate::FieldReader;
#[doc = "Field `KEY` writer - AES key material register."]
pub type KEY_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - AES key material register."]
    #[inline(always)]
    pub fn key(&self) -> KEY_R {
        KEY_R::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("KEY").field("key", &self.key()).finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - AES key material register."]
    #[inline(always)]
    #[must_use]
    pub fn key(&mut self) -> KEY_W<KEY_SPEC> {
        KEY_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`key::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`key::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KEY_SPEC;
impl crate::RegisterSpec for KEY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`key::R`](R) reader structure"]
impl crate::Readable for KEY_SPEC {}
#[doc = "`write(|w| ..)` method takes [`key::W`](W) writer structure"]
impl crate::Writable for KEY_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets KEY[%s] to value 0"]
impl crate::Resettable for KEY_SPEC {
    const RESET_VALUE: u32 = 0;
}
