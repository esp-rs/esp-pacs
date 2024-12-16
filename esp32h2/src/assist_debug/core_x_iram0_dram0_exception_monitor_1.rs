#[doc = "Register `CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_1` reader"]
pub type R = crate::R<CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_1_SPEC>;
#[doc = "Register `CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_1` writer"]
pub type W = crate::W<CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_1_SPEC>;
#[doc = "Field `CORE_X_IRAM0_DRAM0_LIMIT_CYCLE_1` reader - reg_core_x_iram0_dram0_limit_cycle_1"]
pub type CORE_X_IRAM0_DRAM0_LIMIT_CYCLE_1_R = crate::FieldReader<u32>;
#[doc = "Field `CORE_X_IRAM0_DRAM0_LIMIT_CYCLE_1` writer - reg_core_x_iram0_dram0_limit_cycle_1"]
pub type CORE_X_IRAM0_DRAM0_LIMIT_CYCLE_1_W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:19 - reg_core_x_iram0_dram0_limit_cycle_1"]
    #[inline(always)]
    pub fn core_x_iram0_dram0_limit_cycle_1(&self) -> CORE_X_IRAM0_DRAM0_LIMIT_CYCLE_1_R {
        CORE_X_IRAM0_DRAM0_LIMIT_CYCLE_1_R::new(self.bits & 0x000f_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_1")
            .field(
                "core_x_iram0_dram0_limit_cycle_1",
                &self.core_x_iram0_dram0_limit_cycle_1(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:19 - reg_core_x_iram0_dram0_limit_cycle_1"]
    #[inline(always)]
    pub fn core_x_iram0_dram0_limit_cycle_1(
        &mut self,
    ) -> CORE_X_IRAM0_DRAM0_LIMIT_CYCLE_1_W<CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_1_SPEC> {
        CORE_X_IRAM0_DRAM0_LIMIT_CYCLE_1_W::new(self, 0)
    }
}
#[doc = "exception monitor status register7\n\nYou can [`read`](crate::Reg::read) this register and get [`core_x_iram0_dram0_exception_monitor_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core_x_iram0_dram0_exception_monitor_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_1_SPEC;
impl crate::RegisterSpec for CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core_x_iram0_dram0_exception_monitor_1::R`](R) reader structure"]
impl crate::Readable for CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`core_x_iram0_dram0_exception_monitor_1::W`](W) writer structure"]
impl crate::Writable for CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_1 to value 0"]
impl crate::Resettable for CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_1_SPEC {
    const RESET_VALUE: u32 = 0;
}
