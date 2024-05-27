///Register `CPU_INT_ENABLE` reader
pub type R = crate::R<CPU_INT_ENABLE_SPEC>;
///Register `CPU_INT_ENABLE` writer
pub type W = crate::W<CPU_INT_ENABLE_SPEC>;
///Field `CPU_INT_ENABLE` reader - reg_core0_cpu_int_enable
pub type CPU_INT_ENABLE_R = crate::FieldReader<u32>;
///Field `CPU_INT_ENABLE` writer - reg_core0_cpu_int_enable
pub type CPU_INT_ENABLE_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - reg_core0_cpu_int_enable
    #[inline(always)]
    pub fn cpu_int_enable(&self) -> CPU_INT_ENABLE_R {
        CPU_INT_ENABLE_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CPU_INT_ENABLE")
            .field("cpu_int_enable", &self.cpu_int_enable())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - reg_core0_cpu_int_enable
    #[inline(always)]
    #[must_use]
    pub fn cpu_int_enable(&mut self) -> CPU_INT_ENABLE_W<CPU_INT_ENABLE_SPEC> {
        CPU_INT_ENABLE_W::new(self, 0)
    }
}
/**mac intr map register

You can [`read`](crate::generic::Reg::read) this register and get [`cpu_int_enable::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_int_enable::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CPU_INT_ENABLE_SPEC;
impl crate::RegisterSpec for CPU_INT_ENABLE_SPEC {
    type Ux = u32;
}
///`read()` method returns [`cpu_int_enable::R`](R) reader structure
impl crate::Readable for CPU_INT_ENABLE_SPEC {}
///`write(|w| ..)` method takes [`cpu_int_enable::W`](W) writer structure
impl crate::Writable for CPU_INT_ENABLE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CPU_INT_ENABLE to value 0
impl crate::Resettable for CPU_INT_ENABLE_SPEC {
    const RESET_VALUE: u32 = 0;
}
