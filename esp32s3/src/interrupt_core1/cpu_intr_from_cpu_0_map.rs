#[doc = "Register `CPU_INTR_FROM_CPU_0_MAP` reader"]
pub type R = crate::R<CPU_INTR_FROM_CPU_0_MAP_SPEC>;
#[doc = "Register `CPU_INTR_FROM_CPU_0_MAP` writer"]
pub type W = crate::W<CPU_INTR_FROM_CPU_0_MAP_SPEC>;
#[doc = "Field `CPU_INTR_FROM_CPU_0_MAP` reader - this register used to map cpu_intr_from_cpu_0 interrupt to one of core1's external interrupt"]
pub type CPU_INTR_FROM_CPU_0_MAP_R = crate::FieldReader;
#[doc = "Field `CPU_INTR_FROM_CPU_0_MAP` writer - this register used to map cpu_intr_from_cpu_0 interrupt to one of core1's external interrupt"]
pub type CPU_INTR_FROM_CPU_0_MAP_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - this register used to map cpu_intr_from_cpu_0 interrupt to one of core1's external interrupt"]
    #[inline(always)]
    pub fn cpu_intr_from_cpu_0_map(&self) -> CPU_INTR_FROM_CPU_0_MAP_R {
        CPU_INTR_FROM_CPU_0_MAP_R::new((self.bits & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CPU_INTR_FROM_CPU_0_MAP")
            .field("cpu_intr_from_cpu_0_map", &self.cpu_intr_from_cpu_0_map())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:4 - this register used to map cpu_intr_from_cpu_0 interrupt to one of core1's external interrupt"]
    #[inline(always)]
    pub fn cpu_intr_from_cpu_0_map(
        &mut self,
    ) -> CPU_INTR_FROM_CPU_0_MAP_W<CPU_INTR_FROM_CPU_0_MAP_SPEC> {
        CPU_INTR_FROM_CPU_0_MAP_W::new(self, 0)
    }
}
#[doc = "cpu_intr_from_cpu_0 interrupt configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu_intr_from_cpu_0_map::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu_intr_from_cpu_0_map::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CPU_INTR_FROM_CPU_0_MAP_SPEC;
impl crate::RegisterSpec for CPU_INTR_FROM_CPU_0_MAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpu_intr_from_cpu_0_map::R`](R) reader structure"]
impl crate::Readable for CPU_INTR_FROM_CPU_0_MAP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cpu_intr_from_cpu_0_map::W`](W) writer structure"]
impl crate::Writable for CPU_INTR_FROM_CPU_0_MAP_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPU_INTR_FROM_CPU_0_MAP to value 0x10"]
impl crate::Resettable for CPU_INTR_FROM_CPU_0_MAP_SPEC {
    const RESET_VALUE: u32 = 0x10;
}
