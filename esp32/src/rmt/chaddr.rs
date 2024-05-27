#[doc = "Register `CH%sADDR` reader"]
pub type R = crate::R<CHADDR_SPEC>;
#[doc = "Field `APB_MEM_ADDR` reader - The ram relative address in channel0 by apb fifo access"]
pub type APB_MEM_ADDR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - The ram relative address in channel0 by apb fifo access"]
    #[inline(always)]
    pub fn apb_mem_addr(&self) -> APB_MEM_ADDR_R {
        APB_MEM_ADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CHADDR")
            .field("apb_mem_addr", &self.apb_mem_addr())
            .finish()
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chaddr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CHADDR_SPEC;
impl crate::RegisterSpec for CHADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chaddr::R`](R) reader structure"]
impl crate::Readable for CHADDR_SPEC {}
#[doc = "`reset()` method sets CH%sADDR to value 0"]
impl crate::Resettable for CHADDR_SPEC {
    const RESET_VALUE: u32 = 0;
}
