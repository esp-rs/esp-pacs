#[doc = "Register `CPU_INT_ENABLE` reader"]
pub type R = crate::R<CPU_INT_ENABLE_SPEC>;
#[doc = "Register `CPU_INT_ENABLE` writer"]
pub type W = crate::W<CPU_INT_ENABLE_SPEC>;
#[doc = "Field `CPU_INT_ENABLE` reader - reg_core0_cpu_int_enable"]
pub type CPU_INT_ENABLE_R = crate::FieldReader<u32>;
#[doc = "Field `CPU_INT_ENABLE` writer - reg_core0_cpu_int_enable"]
pub type CPU_INT_ENABLE_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - reg_core0_cpu_int_enable"]
    #[inline(always)]
    pub fn cpu_int_enable(&self) -> CPU_INT_ENABLE_R {
        CPU_INT_ENABLE_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CPU_INT_ENABLE")
            .field(
                "cpu_int_enable",
                &format_args!("{}", self.cpu_int_enable().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CPU_INT_ENABLE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31 - reg_core0_cpu_int_enable"]
    #[inline(always)]
    #[must_use]
    pub fn cpu_int_enable(&mut self) -> CPU_INT_ENABLE_W<CPU_INT_ENABLE_SPEC> {
        CPU_INT_ENABLE_W::new(self, 0)
    }
}
#[doc = "mac intr map register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpu_int_enable::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_int_enable::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CPU_INT_ENABLE_SPEC;
impl crate::RegisterSpec for CPU_INT_ENABLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpu_int_enable::R`](R) reader structure"]
impl crate::Readable for CPU_INT_ENABLE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cpu_int_enable::W`](W) writer structure"]
impl crate::Writable for CPU_INT_ENABLE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPU_INT_ENABLE to value 0"]
impl crate::Resettable for CPU_INT_ENABLE_SPEC {
    const RESET_VALUE: u32 = 0;
}
