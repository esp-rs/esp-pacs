#[doc = "Register `PRO_DCACHE_REJECT_VADDR` reader"]
pub type R = crate::R<PRO_DCACHE_REJECT_VADDR_SPEC>;
#[doc = "Field `PRO_DCACHE_CPU_VADDR` reader - The bits are used to indicate the virtual address of CPU access dcache when authentication fail."]
pub type PRO_DCACHE_CPU_VADDR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - The bits are used to indicate the virtual address of CPU access dcache when authentication fail."]
    #[inline(always)]
    pub fn pro_dcache_cpu_vaddr(&self) -> PRO_DCACHE_CPU_VADDR_R {
        PRO_DCACHE_CPU_VADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRO_DCACHE_REJECT_VADDR")
            .field(
                "pro_dcache_cpu_vaddr",
                &format_args!("{}", self.pro_dcache_cpu_vaddr().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PRO_DCACHE_REJECT_VADDR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_dcache_reject_vaddr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRO_DCACHE_REJECT_VADDR_SPEC;
impl crate::RegisterSpec for PRO_DCACHE_REJECT_VADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pro_dcache_reject_vaddr::R`](R) reader structure"]
impl crate::Readable for PRO_DCACHE_REJECT_VADDR_SPEC {}
#[doc = "`reset()` method sets PRO_DCACHE_REJECT_VADDR to value 0"]
impl crate::Resettable for PRO_DCACHE_REJECT_VADDR_SPEC {
    const RESET_VALUE: u32 = 0;
}
