#[doc = "Register `L2_CACHE_ADDR` reader"]
pub type R = crate::R<L2_CACHE_ADDR_SPEC>;
#[doc = "Field `L2_CACHE_ADDR` reader - Those bits stores the address which will decide where inside the specified tag memory object will be accessed."]
pub type L2_CACHE_ADDR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Those bits stores the address which will decide where inside the specified tag memory object will be accessed."]
    #[inline(always)]
    pub fn l2_cache_addr(&self) -> L2_CACHE_ADDR_R {
        L2_CACHE_ADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L2_CACHE_ADDR")
            .field("l2_cache_addr", &self.l2_cache_addr())
            .finish()
    }
}
#[doc = "Cache address register\n\nYou can [`read`](crate::Reg::read) this register and get [`l2_cache_addr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L2_CACHE_ADDR_SPEC;
impl crate::RegisterSpec for L2_CACHE_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l2_cache_addr::R`](R) reader structure"]
impl crate::Readable for L2_CACHE_ADDR_SPEC {}
#[doc = "`reset()` method sets L2_CACHE_ADDR to value 0x4000_0000"]
impl crate::Resettable for L2_CACHE_ADDR_SPEC {
    const RESET_VALUE: u32 = 0x4000_0000;
}
