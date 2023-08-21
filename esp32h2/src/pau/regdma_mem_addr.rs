#[doc = "Register `REGDMA_MEM_ADDR` reader"]
pub type R = crate::R<REGDMA_MEM_ADDR_SPEC>;
#[doc = "Field `MEM_ADDR` reader - mem addr reg"]
pub type MEM_ADDR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - mem addr reg"]
    #[inline(always)]
    pub fn mem_addr(&self) -> MEM_ADDR_R {
        MEM_ADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REGDMA_MEM_ADDR")
            .field("mem_addr", &format_args!("{}", self.mem_addr().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<REGDMA_MEM_ADDR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "mem addr\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regdma_mem_addr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REGDMA_MEM_ADDR_SPEC;
impl crate::RegisterSpec for REGDMA_MEM_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`regdma_mem_addr::R`](R) reader structure"]
impl crate::Readable for REGDMA_MEM_ADDR_SPEC {}
#[doc = "`reset()` method sets REGDMA_MEM_ADDR to value 0"]
impl crate::Resettable for REGDMA_MEM_ADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
