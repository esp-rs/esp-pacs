#[doc = "Register `CPU_INTR_FROM_CPU_%s` reader"]
pub type R = crate::R<CPU_INTR_FROM_CPU__SPEC>;
#[doc = "Register `CPU_INTR_FROM_CPU_%s` writer"]
pub type W = crate::W<CPU_INTR_FROM_CPU__SPEC>;
#[doc = "Field `CPU_INTR_FROM_CPU_0` reader - reg_cpu_intr_from_cpu_0"]
pub type CPU_INTR_FROM_CPU_0_R = crate::BitReader;
#[doc = "Field `CPU_INTR_FROM_CPU_0` writer - reg_cpu_intr_from_cpu_0"]
pub type CPU_INTR_FROM_CPU_0_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - reg_cpu_intr_from_cpu_0"]
    #[inline(always)]
    pub fn cpu_intr_from_cpu_0(&self) -> CPU_INTR_FROM_CPU_0_R {
        CPU_INTR_FROM_CPU_0_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CPU_INTR_FROM_CPU_")
            .field("cpu_intr_from_cpu_0", &self.cpu_intr_from_cpu_0())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - reg_cpu_intr_from_cpu_0"]
    #[inline(always)]
    pub fn cpu_intr_from_cpu_0(&mut self) -> CPU_INTR_FROM_CPU_0_W<CPU_INTR_FROM_CPU__SPEC> {
        CPU_INTR_FROM_CPU_0_W::new(self, 0)
    }
}
#[doc = "interrupt generate register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu_intr_from_cpu_::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu_intr_from_cpu_::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CPU_INTR_FROM_CPU__SPEC;
impl crate::RegisterSpec for CPU_INTR_FROM_CPU__SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpu_intr_from_cpu_::R`](R) reader structure"]
impl crate::Readable for CPU_INTR_FROM_CPU__SPEC {}
#[doc = "`write(|w| ..)` method takes [`cpu_intr_from_cpu_::W`](W) writer structure"]
impl crate::Writable for CPU_INTR_FROM_CPU__SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CPU_INTR_FROM_CPU_%s to value 0"]
impl crate::Resettable for CPU_INTR_FROM_CPU__SPEC {}
