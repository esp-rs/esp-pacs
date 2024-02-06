#[doc = "Register `CORE_1_LASTPC_BEFORE_EXCEPTION` reader"]
pub type R = crate::R<CORE_1_LASTPC_BEFORE_EXCEPTION_SPEC>;
#[doc = "Field `CORE_1_LASTPC_BEFORE_EXC` reader - cpu's lastpc before exception"]
pub type CORE_1_LASTPC_BEFORE_EXC_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - cpu's lastpc before exception"]
    #[inline(always)]
    pub fn core_1_lastpc_before_exc(&self) -> CORE_1_LASTPC_BEFORE_EXC_R {
        CORE_1_LASTPC_BEFORE_EXC_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORE_1_LASTPC_BEFORE_EXCEPTION")
            .field(
                "core_1_lastpc_before_exc",
                &format_args!("{}", self.core_1_lastpc_before_exc().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CORE_1_LASTPC_BEFORE_EXCEPTION_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "cpu status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_1_lastpc_before_exception::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CORE_1_LASTPC_BEFORE_EXCEPTION_SPEC;
impl crate::RegisterSpec for CORE_1_LASTPC_BEFORE_EXCEPTION_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core_1_lastpc_before_exception::R`](R) reader structure"]
impl crate::Readable for CORE_1_LASTPC_BEFORE_EXCEPTION_SPEC {}
#[doc = "`reset()` method sets CORE_1_LASTPC_BEFORE_EXCEPTION to value 0"]
impl crate::Resettable for CORE_1_LASTPC_BEFORE_EXCEPTION_SPEC {
    const RESET_VALUE: u32 = 0;
}
