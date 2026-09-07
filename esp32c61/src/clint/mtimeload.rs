#[doc = "Register `MTIMELOAD` reader"]
pub type R = crate::R<MTIMELOAD_SPEC>;
#[doc = "Register `MTIMELOAD` writer"]
pub type W = crate::W<MTIMELOAD_SPEC>;
#[doc = "Field `MTIMELOAD` reader - Represents the value to be loaded to the system counter."]
pub type MTIMELOAD_R = crate::FieldReader<u64>;
#[doc = "Field `MTIMELOAD` writer - Represents the value to be loaded to the system counter."]
pub type MTIMELOAD_W<'a, REG> = crate::FieldWriter<'a, REG, 64, u64>;
impl R {
    #[doc = "Bits 0:63 - Represents the value to be loaded to the system counter."]
    #[inline(always)]
    pub fn mtimeload(&self) -> MTIMELOAD_R {
        MTIMELOAD_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MTIMELOAD")
            .field("mtimeload", &self.mtimeload())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:63 - Represents the value to be loaded to the system counter."]
    #[inline(always)]
    pub fn mtimeload(&mut self) -> MTIMELOAD_W<'_, MTIMELOAD_SPEC> {
        MTIMELOAD_W::new(self, 0)
    }
}
#[doc = "Core-local machine timer load value\n\nYou can [`read`](crate::Reg::read) this register and get [`mtimeload::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtimeload::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MTIMELOAD_SPEC;
impl crate::RegisterSpec for MTIMELOAD_SPEC {
    type Ux = u64;
}
#[doc = "`read()` method returns [`mtimeload::R`](R) reader structure"]
impl crate::Readable for MTIMELOAD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mtimeload::W`](W) writer structure"]
impl crate::Writable for MTIMELOAD_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MTIMELOAD to value 0"]
impl crate::Resettable for MTIMELOAD_SPEC {}
