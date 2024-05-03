#[doc = "Register `LP_CORE_BOOT_ADDR` reader"]
pub type R = crate::R<LP_CORE_BOOT_ADDR_SPEC>;
#[doc = "Register `LP_CORE_BOOT_ADDR` writer"]
pub type W = crate::W<LP_CORE_BOOT_ADDR_SPEC>;
#[doc = "Field `LP_CPU_BOOT_ADDR` reader - need_des"]
pub type LP_CPU_BOOT_ADDR_R = crate::FieldReader<u32>;
#[doc = "Field `LP_CPU_BOOT_ADDR` writer - need_des"]
pub type LP_CPU_BOOT_ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - need_des"]
    #[inline(always)]
    pub fn lp_cpu_boot_addr(&self) -> LP_CPU_BOOT_ADDR_R {
        LP_CPU_BOOT_ADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LP_CORE_BOOT_ADDR")
            .field("lp_cpu_boot_addr", &self.lp_cpu_boot_addr().bits())
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<LP_CORE_BOOT_ADDR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_cpu_boot_addr(&mut self) -> LP_CPU_BOOT_ADDR_W<LP_CORE_BOOT_ADDR_SPEC> {
        LP_CPU_BOOT_ADDR_W::new(self, 0)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_core_boot_addr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_core_boot_addr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LP_CORE_BOOT_ADDR_SPEC;
impl crate::RegisterSpec for LP_CORE_BOOT_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lp_core_boot_addr::R`](R) reader structure"]
impl crate::Readable for LP_CORE_BOOT_ADDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lp_core_boot_addr::W`](W) writer structure"]
impl crate::Writable for LP_CORE_BOOT_ADDR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LP_CORE_BOOT_ADDR to value 0x5010_0000"]
impl crate::Resettable for LP_CORE_BOOT_ADDR_SPEC {
    const RESET_VALUE: u32 = 0x5010_0000;
}
