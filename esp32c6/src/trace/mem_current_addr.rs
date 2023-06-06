#[doc = "Register `MEM_CURRENT_ADDR` reader"]
pub struct R(crate::R<MEM_CURRENT_ADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MEM_CURRENT_ADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MEM_CURRENT_ADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MEM_CURRENT_ADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `MEM_CURRENT_ADDR` reader - current_mem_addr,indicate that next writing addr"]
pub type MEM_CURRENT_ADDR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - current_mem_addr,indicate that next writing addr"]
    #[inline(always)]
    pub fn mem_current_addr(&self) -> MEM_CURRENT_ADDR_R {
        MEM_CURRENT_ADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MEM_CURRENT_ADDR")
            .field(
                "mem_current_addr",
                &format_args!("{}", self.mem_current_addr().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<MEM_CURRENT_ADDR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "mem current addr\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mem_current_addr](index.html) module"]
pub struct MEM_CURRENT_ADDR_SPEC;
impl crate::RegisterSpec for MEM_CURRENT_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mem_current_addr::R](R) reader structure"]
impl crate::Readable for MEM_CURRENT_ADDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MEM_CURRENT_ADDR to value 0"]
impl crate::Resettable for MEM_CURRENT_ADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
