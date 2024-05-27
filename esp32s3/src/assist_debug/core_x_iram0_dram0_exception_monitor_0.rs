///Register `CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_0` reader
pub type R = crate::R<CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_0_SPEC>;
///Register `CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_0` writer
pub type W = crate::W<CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_0_SPEC>;
///Field `CORE_X_IRAM0_DRAM0_LIMIT_CYCLE_0` reader - busy monitor window cycle
pub type CORE_X_IRAM0_DRAM0_LIMIT_CYCLE_0_R = crate::FieldReader<u32>;
///Field `CORE_X_IRAM0_DRAM0_LIMIT_CYCLE_0` writer - busy monitor window cycle
pub type CORE_X_IRAM0_DRAM0_LIMIT_CYCLE_0_W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    ///Bits 0:19 - busy monitor window cycle
    #[inline(always)]
    pub fn core_x_iram0_dram0_limit_cycle_0(&self) -> CORE_X_IRAM0_DRAM0_LIMIT_CYCLE_0_R {
        CORE_X_IRAM0_DRAM0_LIMIT_CYCLE_0_R::new(self.bits & 0x000f_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_0")
            .field(
                "core_x_iram0_dram0_limit_cycle_0",
                &self.core_x_iram0_dram0_limit_cycle_0(),
            )
            .finish()
    }
}
impl W {
    ///Bits 0:19 - busy monitor window cycle
    #[inline(always)]
    #[must_use]
    pub fn core_x_iram0_dram0_limit_cycle_0(
        &mut self,
    ) -> CORE_X_IRAM0_DRAM0_LIMIT_CYCLE_0_W<CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_0_SPEC> {
        CORE_X_IRAM0_DRAM0_LIMIT_CYCLE_0_W::new(self, 0)
    }
}
/**bus busy configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`core_x_iram0_dram0_exception_monitor_0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_x_iram0_dram0_exception_monitor_0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_0_SPEC;
impl crate::RegisterSpec for CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_0_SPEC {
    type Ux = u32;
}
///`read()` method returns [`core_x_iram0_dram0_exception_monitor_0::R`](R) reader structure
impl crate::Readable for CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_0_SPEC {}
///`write(|w| ..)` method takes [`core_x_iram0_dram0_exception_monitor_0::W`](W) writer structure
impl crate::Writable for CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_0 to value 0x000f_ffff
impl crate::Resettable for CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_0_SPEC {
    const RESET_VALUE: u32 = 0x000f_ffff;
}
