#[doc = "Register `INTERNAL_SRAM_USAGE_4` reader"]
pub type R = crate::R<INTERNAL_SRAM_USAGE_4_SPEC>;
#[doc = "Register `INTERNAL_SRAM_USAGE_4` writer"]
pub type W = crate::W<INTERNAL_SRAM_USAGE_4_SPEC>;
#[doc = "Field `INTERNAL_SRAM_LOG_USAGE` reader - Set 1 to someone bit means corresponding internal SRAM level can be accessed by log bus."]
pub type INTERNAL_SRAM_LOG_USAGE_R = crate::FieldReader;
#[doc = "Field `INTERNAL_SRAM_LOG_USAGE` writer - Set 1 to someone bit means corresponding internal SRAM level can be accessed by log bus."]
pub type INTERNAL_SRAM_LOG_USAGE_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - Set 1 to someone bit means corresponding internal SRAM level can be accessed by log bus."]
    #[inline(always)]
    pub fn internal_sram_log_usage(&self) -> INTERNAL_SRAM_LOG_USAGE_R {
        INTERNAL_SRAM_LOG_USAGE_R::new((self.bits & 0x7f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTERNAL_SRAM_USAGE_4")
            .field(
                "internal_sram_log_usage",
                &format_args!("{}", self.internal_sram_log_usage().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INTERNAL_SRAM_USAGE_4_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:6 - Set 1 to someone bit means corresponding internal SRAM level can be accessed by log bus."]
    #[inline(always)]
    #[must_use]
    pub fn internal_sram_log_usage(
        &mut self,
    ) -> INTERNAL_SRAM_LOG_USAGE_W<INTERNAL_SRAM_USAGE_4_SPEC> {
        INTERNAL_SRAM_LOG_USAGE_W::new(self, 0)
    }
}
#[doc = "Internal SRAM configuration register 4.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`internal_sram_usage_4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`internal_sram_usage_4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTERNAL_SRAM_USAGE_4_SPEC;
impl crate::RegisterSpec for INTERNAL_SRAM_USAGE_4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`internal_sram_usage_4::R`](R) reader structure"]
impl crate::Readable for INTERNAL_SRAM_USAGE_4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`internal_sram_usage_4::W`](W) writer structure"]
impl crate::Writable for INTERNAL_SRAM_USAGE_4_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTERNAL_SRAM_USAGE_4 to value 0"]
impl crate::Resettable for INTERNAL_SRAM_USAGE_4_SPEC {
    const RESET_VALUE: u32 = 0;
}
