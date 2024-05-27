///Register `CPU_INTR_FROM_CPU_2` reader
pub type R = crate::R<CPU_INTR_FROM_CPU_2_SPEC>;
///Register `CPU_INTR_FROM_CPU_2` writer
pub type W = crate::W<CPU_INTR_FROM_CPU_2_SPEC>;
///Field `CPU_INTR_FROM_CPU_2` reader - Set 1 to generate cpu interrupt 2
pub type CPU_INTR_FROM_CPU_2_R = crate::BitReader;
///Field `CPU_INTR_FROM_CPU_2` writer - Set 1 to generate cpu interrupt 2
pub type CPU_INTR_FROM_CPU_2_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Set 1 to generate cpu interrupt 2
    #[inline(always)]
    pub fn cpu_intr_from_cpu_2(&self) -> CPU_INTR_FROM_CPU_2_R {
        CPU_INTR_FROM_CPU_2_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CPU_INTR_FROM_CPU_2")
            .field("cpu_intr_from_cpu_2", &self.cpu_intr_from_cpu_2())
            .finish()
    }
}
impl W {
    ///Bit 0 - Set 1 to generate cpu interrupt 2
    #[inline(always)]
    #[must_use]
    pub fn cpu_intr_from_cpu_2(&mut self) -> CPU_INTR_FROM_CPU_2_W<CPU_INTR_FROM_CPU_2_SPEC> {
        CPU_INTR_FROM_CPU_2_W::new(self, 0)
    }
}
/**interrupt generate register

You can [`read`](crate::generic::Reg::read) this register and get [`cpu_intr_from_cpu_2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_intr_from_cpu_2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CPU_INTR_FROM_CPU_2_SPEC;
impl crate::RegisterSpec for CPU_INTR_FROM_CPU_2_SPEC {
    type Ux = u32;
}
///`read()` method returns [`cpu_intr_from_cpu_2::R`](R) reader structure
impl crate::Readable for CPU_INTR_FROM_CPU_2_SPEC {}
///`write(|w| ..)` method takes [`cpu_intr_from_cpu_2::W`](W) writer structure
impl crate::Writable for CPU_INTR_FROM_CPU_2_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CPU_INTR_FROM_CPU_2 to value 0
impl crate::Resettable for CPU_INTR_FROM_CPU_2_SPEC {
    const RESET_VALUE: u32 = 0;
}
