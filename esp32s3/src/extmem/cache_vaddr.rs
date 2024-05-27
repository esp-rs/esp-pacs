#[doc = "Register `CACHE_VADDR` reader"]
pub type R = crate::R<CACHE_VADDR_SPEC>;
#[doc = "Register `CACHE_VADDR` writer"]
pub type W = crate::W<CACHE_VADDR_SPEC>;
#[doc = "Field `CACHE_VADDR` reader - Those bits stores the virtual address which will decide where inside the specified tag memory object will be accessed."]
pub type CACHE_VADDR_R = crate::FieldReader<u32>;
#[doc = "Field `CACHE_VADDR` writer - Those bits stores the virtual address which will decide where inside the specified tag memory object will be accessed."]
pub type CACHE_VADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Those bits stores the virtual address which will decide where inside the specified tag memory object will be accessed."]
    #[inline(always)]
    pub fn cache_vaddr(&self) -> CACHE_VADDR_R {
        CACHE_VADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CACHE_VADDR")
            .field("cache_vaddr", &self.cache_vaddr())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Those bits stores the virtual address which will decide where inside the specified tag memory object will be accessed."]
    #[inline(always)]
    #[must_use]
    pub fn cache_vaddr(&mut self) -> CACHE_VADDR_W<CACHE_VADDR_SPEC> {
        CACHE_VADDR_W::new(self, 0)
    }
}
#[doc = "******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cache_vaddr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cache_vaddr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CACHE_VADDR_SPEC;
impl crate::RegisterSpec for CACHE_VADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cache_vaddr::R`](R) reader structure"]
impl crate::Readable for CACHE_VADDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cache_vaddr::W`](W) writer structure"]
impl crate::Writable for CACHE_VADDR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CACHE_VADDR to value 0"]
impl crate::Resettable for CACHE_VADDR_SPEC {
    const RESET_VALUE: u32 = 0;
}
