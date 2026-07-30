#[doc = "Register `CORE_0_LASTPC_BEFORE_EXCEPTION` reader"]
pub type R = crate::R<CORE_0_LASTPC_BEFORE_EXCEPTION_SPEC>;
#[doc = "Field `CORE_0_LASTPC_BEFORE_EXC` reader - cpu's lastpc before exception"]
pub type CORE_0_LASTPC_BEFORE_EXC_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - cpu's lastpc before exception"]
    #[inline(always)]
    pub fn core_0_lastpc_before_exc(&self) -> CORE_0_LASTPC_BEFORE_EXC_R {
        CORE_0_LASTPC_BEFORE_EXC_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORE_0_LASTPC_BEFORE_EXCEPTION")
            .field("core_0_lastpc_before_exc", &self.core_0_lastpc_before_exc())
            .finish()
    }
}
#[doc = "cpu status register\n\nYou can [`read`](crate::Reg::read) this register and get [`core_0_lastpc_before_exception::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CORE_0_LASTPC_BEFORE_EXCEPTION_SPEC;
impl crate::RegisterSpec for CORE_0_LASTPC_BEFORE_EXCEPTION_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core_0_lastpc_before_exception::R`](R) reader structure"]
impl crate::Readable for CORE_0_LASTPC_BEFORE_EXCEPTION_SPEC {}
#[doc = "`reset()` method sets CORE_0_LASTPC_BEFORE_EXCEPTION to value 0"]
impl crate::Resettable for CORE_0_LASTPC_BEFORE_EXCEPTION_SPEC {}
