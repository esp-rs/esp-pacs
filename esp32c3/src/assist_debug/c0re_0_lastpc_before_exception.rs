#[doc = "Register `C0RE_0_LASTPC_BEFORE_EXCEPTION` reader"]
pub type R = crate::R<C0RE_0_LASTPC_BEFORE_EXCEPTION_SPEC>;
#[doc = "Field `CORE_0_LASTPC_BEFORE_EXC` reader - reg_core_0_lastpc_before_exc"]
pub type CORE_0_LASTPC_BEFORE_EXC_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - reg_core_0_lastpc_before_exc"]
    #[inline(always)]
    pub fn core_0_lastpc_before_exc(&self) -> CORE_0_LASTPC_BEFORE_EXC_R {
        CORE_0_LASTPC_BEFORE_EXC_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("C0RE_0_LASTPC_BEFORE_EXCEPTION")
            .field(
                "core_0_lastpc_before_exc",
                &format_args!("{}", self.core_0_lastpc_before_exc().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<C0RE_0_LASTPC_BEFORE_EXCEPTION_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "ASSIST_DEBUG_C0RE_0_LASTPC_BEFORE_EXCEPTION\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c0re_0_lastpc_before_exception::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C0RE_0_LASTPC_BEFORE_EXCEPTION_SPEC;
impl crate::RegisterSpec for C0RE_0_LASTPC_BEFORE_EXCEPTION_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`c0re_0_lastpc_before_exception::R`](R) reader structure"]
impl crate::Readable for C0RE_0_LASTPC_BEFORE_EXCEPTION_SPEC {}
#[doc = "`reset()` method sets C0RE_0_LASTPC_BEFORE_EXCEPTION to value 0"]
impl crate::Resettable for C0RE_0_LASTPC_BEFORE_EXCEPTION_SPEC {
    const RESET_VALUE: u32 = 0;
}
