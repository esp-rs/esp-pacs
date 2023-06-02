#[doc = "Register `C0RE_0_LASTPC_BEFORE_EXCEPTION` reader"]
pub struct R(crate::R<C0RE_0_LASTPC_BEFORE_EXCEPTION_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C0RE_0_LASTPC_BEFORE_EXCEPTION_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C0RE_0_LASTPC_BEFORE_EXCEPTION_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C0RE_0_LASTPC_BEFORE_EXCEPTION_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CORE_0_LASTPC_BEFORE_EXC` reader - cpu's lastpc before exception"]
pub type CORE_0_LASTPC_BEFORE_EXC_R = crate::FieldReader<u32, u32>;
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
        self.read().fmt(f)
    }
}
#[doc = "cpu status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c0re_0_lastpc_before_exception](index.html) module"]
pub struct C0RE_0_LASTPC_BEFORE_EXCEPTION_SPEC;
impl crate::RegisterSpec for C0RE_0_LASTPC_BEFORE_EXCEPTION_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [c0re_0_lastpc_before_exception::R](R) reader structure"]
impl crate::Readable for C0RE_0_LASTPC_BEFORE_EXCEPTION_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets C0RE_0_LASTPC_BEFORE_EXCEPTION to value 0"]
impl crate::Resettable for C0RE_0_LASTPC_BEFORE_EXCEPTION_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
