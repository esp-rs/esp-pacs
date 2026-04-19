#[doc = "Register `UNALLOCATE_BUFFER_CLEAR` reader"]
pub type R = crate::R<UNALLOCATE_BUFFER_CLEAR_SPEC>;
#[doc = "Register `UNALLOCATE_BUFFER_CLEAR` writer"]
pub type W = crate::W<UNALLOCATE_BUFFER_CLEAR_SPEC>;
#[doc = "Field `ICACHE2_UNALLOC_CLR` reader - Reserved"]
pub type ICACHE2_UNALLOC_CLR_R = crate::BitReader;
#[doc = "Field `ICACHE2_UNALLOC_CLR` writer - Reserved"]
pub type ICACHE2_UNALLOC_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CACHE_UNALLOC_CLR` reader - The bit is used to clear the unallocate request buffer of cache where the unallocate request is responded but not completed."]
pub type CACHE_UNALLOC_CLR_R = crate::BitReader;
#[doc = "Field `CACHE_UNALLOC_CLR` writer - The bit is used to clear the unallocate request buffer of cache where the unallocate request is responded but not completed."]
pub type CACHE_UNALLOC_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 2 - Reserved"]
    #[inline(always)]
    pub fn icache2_unalloc_clr(&self) -> ICACHE2_UNALLOC_CLR_R {
        ICACHE2_UNALLOC_CLR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - The bit is used to clear the unallocate request buffer of cache where the unallocate request is responded but not completed."]
    #[inline(always)]
    pub fn cache_unalloc_clr(&self) -> CACHE_UNALLOC_CLR_R {
        CACHE_UNALLOC_CLR_R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UNALLOCATE_BUFFER_CLEAR")
            .field("icache2_unalloc_clr", &self.icache2_unalloc_clr())
            .field("cache_unalloc_clr", &self.cache_unalloc_clr())
            .finish()
    }
}
impl W {
    #[doc = "Bit 2 - Reserved"]
    #[inline(always)]
    pub fn icache2_unalloc_clr(
        &mut self,
    ) -> ICACHE2_UNALLOC_CLR_W<'_, UNALLOCATE_BUFFER_CLEAR_SPEC> {
        ICACHE2_UNALLOC_CLR_W::new(self, 2)
    }
    #[doc = "Bit 4 - The bit is used to clear the unallocate request buffer of cache where the unallocate request is responded but not completed."]
    #[inline(always)]
    pub fn cache_unalloc_clr(&mut self) -> CACHE_UNALLOC_CLR_W<'_, UNALLOCATE_BUFFER_CLEAR_SPEC> {
        CACHE_UNALLOC_CLR_W::new(self, 4)
    }
}
#[doc = "Unallocate request buffer clear registers\n\nYou can [`read`](crate::Reg::read) this register and get [`unallocate_buffer_clear::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`unallocate_buffer_clear::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UNALLOCATE_BUFFER_CLEAR_SPEC;
impl crate::RegisterSpec for UNALLOCATE_BUFFER_CLEAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`unallocate_buffer_clear::R`](R) reader structure"]
impl crate::Readable for UNALLOCATE_BUFFER_CLEAR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`unallocate_buffer_clear::W`](W) writer structure"]
impl crate::Writable for UNALLOCATE_BUFFER_CLEAR_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UNALLOCATE_BUFFER_CLEAR to value 0"]
impl crate::Resettable for UNALLOCATE_BUFFER_CLEAR_SPEC {}
