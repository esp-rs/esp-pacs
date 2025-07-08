#[doc = "Register `LOG_MEM_CURRENT_ADDR` reader"]
pub type R = crate::R<LOG_MEM_CURRENT_ADDR_SPEC>;
#[doc = "Field `LOG_MEM_CURRENT_ADDR` reader - Represents the address of the next write."]
pub type LOG_MEM_CURRENT_ADDR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Represents the address of the next write."]
    #[inline(always)]
    pub fn log_mem_current_addr(&self) -> LOG_MEM_CURRENT_ADDR_R {
        LOG_MEM_CURRENT_ADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LOG_MEM_CURRENT_ADDR")
            .field("log_mem_current_addr", &self.log_mem_current_addr())
            .finish()
    }
}
#[doc = "Represents the address for the next write\n\nYou can [`read`](crate::Reg::read) this register and get [`log_mem_current_addr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LOG_MEM_CURRENT_ADDR_SPEC;
impl crate::RegisterSpec for LOG_MEM_CURRENT_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`log_mem_current_addr::R`](R) reader structure"]
impl crate::Readable for LOG_MEM_CURRENT_ADDR_SPEC {}
#[doc = "`reset()` method sets LOG_MEM_CURRENT_ADDR to value 0"]
impl crate::Resettable for LOG_MEM_CURRENT_ADDR_SPEC {}
