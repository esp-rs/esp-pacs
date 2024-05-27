///Register `CPU_INTR_FROM_CPU_0` reader
pub type R = crate::R<CPU_INTR_FROM_CPU_0_SPEC>;
///Register `CPU_INTR_FROM_CPU_0` writer
pub type W = crate::W<CPU_INTR_FROM_CPU_0_SPEC>;
///Field `CPU_INTR_FROM_CPU_0` reader - Set 1 to generate cpu interrupt 0
pub type CPU_INTR_FROM_CPU_0_R = crate::BitReader;
///Field `CPU_INTR_FROM_CPU_0` writer - Set 1 to generate cpu interrupt 0
pub type CPU_INTR_FROM_CPU_0_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Set 1 to generate cpu interrupt 0
    #[inline(always)]
    pub fn cpu_intr_from_cpu_0(&self) -> CPU_INTR_FROM_CPU_0_R {
        CPU_INTR_FROM_CPU_0_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CPU_INTR_FROM_CPU_0")
            .field("cpu_intr_from_cpu_0", &self.cpu_intr_from_cpu_0())
            .finish()
    }
}
impl W {
    ///Bit 0 - Set 1 to generate cpu interrupt 0
    #[inline(always)]
    #[must_use]
    pub fn cpu_intr_from_cpu_0(&mut self) -> CPU_INTR_FROM_CPU_0_W<CPU_INTR_FROM_CPU_0_SPEC> {
        CPU_INTR_FROM_CPU_0_W::new(self, 0)
    }
}
/**interrupt generate register

You can [`read`](crate::generic::Reg::read) this register and get [`cpu_intr_from_cpu_0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_intr_from_cpu_0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CPU_INTR_FROM_CPU_0_SPEC;
impl crate::RegisterSpec for CPU_INTR_FROM_CPU_0_SPEC {
    type Ux = u32;
}
///`read()` method returns [`cpu_intr_from_cpu_0::R`](R) reader structure
impl crate::Readable for CPU_INTR_FROM_CPU_0_SPEC {}
///`write(|w| ..)` method takes [`cpu_intr_from_cpu_0::W`](W) writer structure
impl crate::Writable for CPU_INTR_FROM_CPU_0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CPU_INTR_FROM_CPU_0 to value 0
impl crate::Resettable for CPU_INTR_FROM_CPU_0_SPEC {
    const RESET_VALUE: u32 = 0;
}
