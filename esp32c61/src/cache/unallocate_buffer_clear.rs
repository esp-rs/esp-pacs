#[doc = "Register `UNALLOCATE_BUFFER_CLEAR` reader"]
pub type R = crate::R<UNALLOCATE_BUFFER_CLEAR_SPEC>;
#[doc = "Register `UNALLOCATE_BUFFER_CLEAR` writer"]
pub type W = crate::W<UNALLOCATE_BUFFER_CLEAR_SPEC>;
#[doc = "Field `ICACHE0_UNALLOC_CLR` reader - The bit is used to clear the unallocate request buffer of l1 icache0 where the unallocate request is responsed but not completed."]
pub type ICACHE0_UNALLOC_CLR_R = crate::BitReader;
#[doc = "Field `ICACHE1_UNALLOC_CLR` reader - The bit is used to clear the unallocate request buffer of l1 icache1 where the unallocate request is responsed but not completed."]
pub type ICACHE1_UNALLOC_CLR_R = crate::BitReader;
#[doc = "Field `ICACHE2_UNALLOC_CLR` reader - Reserved"]
pub type ICACHE2_UNALLOC_CLR_R = crate::BitReader;
#[doc = "Field `ICACHE3_UNALLOC_CLR` reader - Reserved"]
pub type ICACHE3_UNALLOC_CLR_R = crate::BitReader;
#[doc = "Field `CACHE_UNALLOC_CLR` reader - The bit is used to clear the unallocate request buffer of l1 cache where the unallocate request is responsed but not completed."]
pub type CACHE_UNALLOC_CLR_R = crate::BitReader;
#[doc = "Field `CACHE_UNALLOC_CLR` writer - The bit is used to clear the unallocate request buffer of l1 cache where the unallocate request is responsed but not completed."]
pub type CACHE_UNALLOC_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - The bit is used to clear the unallocate request buffer of l1 icache0 where the unallocate request is responsed but not completed."]
    #[inline(always)]
    pub fn icache0_unalloc_clr(&self) -> ICACHE0_UNALLOC_CLR_R {
        ICACHE0_UNALLOC_CLR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The bit is used to clear the unallocate request buffer of l1 icache1 where the unallocate request is responsed but not completed."]
    #[inline(always)]
    pub fn icache1_unalloc_clr(&self) -> ICACHE1_UNALLOC_CLR_R {
        ICACHE1_UNALLOC_CLR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Reserved"]
    #[inline(always)]
    pub fn icache2_unalloc_clr(&self) -> ICACHE2_UNALLOC_CLR_R {
        ICACHE2_UNALLOC_CLR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Reserved"]
    #[inline(always)]
    pub fn icache3_unalloc_clr(&self) -> ICACHE3_UNALLOC_CLR_R {
        ICACHE3_UNALLOC_CLR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The bit is used to clear the unallocate request buffer of l1 cache where the unallocate request is responsed but not completed."]
    #[inline(always)]
    pub fn cache_unalloc_clr(&self) -> CACHE_UNALLOC_CLR_R {
        CACHE_UNALLOC_CLR_R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UNALLOCATE_BUFFER_CLEAR")
            .field("icache0_unalloc_clr", &self.icache0_unalloc_clr())
            .field("icache1_unalloc_clr", &self.icache1_unalloc_clr())
            .field("icache2_unalloc_clr", &self.icache2_unalloc_clr())
            .field("icache3_unalloc_clr", &self.icache3_unalloc_clr())
            .field("cache_unalloc_clr", &self.cache_unalloc_clr())
            .finish()
    }
}
impl W {
    #[doc = "Bit 4 - The bit is used to clear the unallocate request buffer of l1 cache where the unallocate request is responsed but not completed."]
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
