#[doc = "Register `PRO_ICACHE_REJECT_VADDR` reader"]
pub struct R(crate::R<PRO_ICACHE_REJECT_VADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRO_ICACHE_REJECT_VADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRO_ICACHE_REJECT_VADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRO_ICACHE_REJECT_VADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PRO_ICACHE_CPU_VADDR` reader - The bits are used to indicate the virtual address of CPU access icache when authentication fail."]
pub type PRO_ICACHE_CPU_VADDR_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - The bits are used to indicate the virtual address of CPU access icache when authentication fail."]
    #[inline(always)]
    pub fn pro_icache_cpu_vaddr(&self) -> PRO_ICACHE_CPU_VADDR_R {
        PRO_ICACHE_CPU_VADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRO_ICACHE_REJECT_VADDR")
            .field(
                "pro_icache_cpu_vaddr",
                &format_args!("{}", self.pro_icache_cpu_vaddr().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PRO_ICACHE_REJECT_VADDR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "register description\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pro_icache_reject_vaddr](index.html) module"]
pub struct PRO_ICACHE_REJECT_VADDR_SPEC;
impl crate::RegisterSpec for PRO_ICACHE_REJECT_VADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pro_icache_reject_vaddr::R](R) reader structure"]
impl crate::Readable for PRO_ICACHE_REJECT_VADDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PRO_ICACHE_REJECT_VADDR to value 0"]
impl crate::Resettable for PRO_ICACHE_REJECT_VADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
