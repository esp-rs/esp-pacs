#[doc = "Register `PLAIN_%s` reader"]
pub type R = crate::R<PLAIN__SPEC>;
#[doc = "Register `PLAIN_%s` writer"]
pub type W = crate::W<PLAIN__SPEC>;
#[doc = "Field `PLAIN` reader - Stores the nth 32-bit piece of plaintext."]
pub type PLAIN_R = crate::FieldReader<u32>;
#[doc = "Field `PLAIN` writer - Stores the nth 32-bit piece of plaintext."]
pub type PLAIN_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Stores the nth 32-bit piece of plaintext."]
    #[inline(always)]
    pub fn plain(&self) -> PLAIN_R {
        PLAIN_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PLAIN_")
            .field("plain", &self.plain())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Stores the nth 32-bit piece of plaintext."]
    #[inline(always)]
    pub fn plain(&mut self) -> PLAIN_W<'_, PLAIN__SPEC> {
        PLAIN_W::new(self, 0)
    }
}
#[doc = "Plaintext register %s\n\nYou can [`read`](crate::Reg::read) this register and get [`plain_::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`plain_::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PLAIN__SPEC;
impl crate::RegisterSpec for PLAIN__SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`plain_::R`](R) reader structure"]
impl crate::Readable for PLAIN__SPEC {}
#[doc = "`write(|w| ..)` method takes [`plain_::W`](W) writer structure"]
impl crate::Writable for PLAIN__SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PLAIN_%s to value 0"]
impl crate::Resettable for PLAIN__SPEC {}
