#[doc = "Register `CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_0` reader"]
pub type R = crate::R<CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_0_SPEC>;
#[doc = "Register `CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_0` writer"]
pub type W = crate::W<CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_0_SPEC>;
#[doc = "Field `CORE_X_IRAM0_DRAM0_LIMIT_CYCLE_0` reader - reg_core_x_iram0_dram0_limit_cycle_0"]
pub type CORE_X_IRAM0_DRAM0_LIMIT_CYCLE_0_R = crate::FieldReader<u32>;
#[doc = "Field `CORE_X_IRAM0_DRAM0_LIMIT_CYCLE_0` writer - reg_core_x_iram0_dram0_limit_cycle_0"]
pub type CORE_X_IRAM0_DRAM0_LIMIT_CYCLE_0_W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:19 - reg_core_x_iram0_dram0_limit_cycle_0"]
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
    #[doc = "Bits 0:19 - reg_core_x_iram0_dram0_limit_cycle_0"]
    #[inline(always)]
    pub fn core_x_iram0_dram0_limit_cycle_0(
        &mut self,
    ) -> CORE_X_IRAM0_DRAM0_LIMIT_CYCLE_0_W<'_, CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_0_SPEC> {
        CORE_X_IRAM0_DRAM0_LIMIT_CYCLE_0_W::new(self, 0)
    }
}
#[doc = "exception monitor status register8\n\nYou can [`read`](crate::Reg::read) this register and get [`core_x_iram0_dram0_exception_monitor_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core_x_iram0_dram0_exception_monitor_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_0_SPEC;
impl crate::RegisterSpec for CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core_x_iram0_dram0_exception_monitor_0::R`](R) reader structure"]
impl crate::Readable for CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`core_x_iram0_dram0_exception_monitor_0::W`](W) writer structure"]
impl crate::Writable for CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_0_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_0 to value 0"]
impl crate::Resettable for CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_0_SPEC {}
