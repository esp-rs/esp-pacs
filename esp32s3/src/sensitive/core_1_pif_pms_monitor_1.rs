#[doc = "Register `CORE_1_PIF_PMS_MONITOR_1` reader"]
pub type R = crate::R<CORE_1_PIF_PMS_MONITOR_1_SPEC>;
#[doc = "Register `CORE_1_PIF_PMS_MONITOR_1` writer"]
pub type W = crate::W<CORE_1_PIF_PMS_MONITOR_1_SPEC>;
#[doc = "Field `CORE_1_PIF_PMS_MONITOR_VIOLATE_CLR` reader - Set 1 to clear interrupt that core1 initiate illegal PIF bus access."]
pub type CORE_1_PIF_PMS_MONITOR_VIOLATE_CLR_R = crate::BitReader;
#[doc = "Field `CORE_1_PIF_PMS_MONITOR_VIOLATE_CLR` writer - Set 1 to clear interrupt that core1 initiate illegal PIF bus access."]
pub type CORE_1_PIF_PMS_MONITOR_VIOLATE_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORE_1_PIF_PMS_MONITOR_VIOLATE_EN` reader - Set 1 to enable interrupt that core1 initiate illegal PIF bus access."]
pub type CORE_1_PIF_PMS_MONITOR_VIOLATE_EN_R = crate::BitReader;
#[doc = "Field `CORE_1_PIF_PMS_MONITOR_VIOLATE_EN` writer - Set 1 to enable interrupt that core1 initiate illegal PIF bus access."]
pub type CORE_1_PIF_PMS_MONITOR_VIOLATE_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Set 1 to clear interrupt that core1 initiate illegal PIF bus access."]
    #[inline(always)]
    pub fn core_1_pif_pms_monitor_violate_clr(&self) -> CORE_1_PIF_PMS_MONITOR_VIOLATE_CLR_R {
        CORE_1_PIF_PMS_MONITOR_VIOLATE_CLR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set 1 to enable interrupt that core1 initiate illegal PIF bus access."]
    #[inline(always)]
    pub fn core_1_pif_pms_monitor_violate_en(&self) -> CORE_1_PIF_PMS_MONITOR_VIOLATE_EN_R {
        CORE_1_PIF_PMS_MONITOR_VIOLATE_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORE_1_PIF_PMS_MONITOR_1")
            .field(
                "core_1_pif_pms_monitor_violate_clr",
                &format_args!("{}", self.core_1_pif_pms_monitor_violate_clr().bit()),
            )
            .field(
                "core_1_pif_pms_monitor_violate_en",
                &format_args!("{}", self.core_1_pif_pms_monitor_violate_en().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CORE_1_PIF_PMS_MONITOR_1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - Set 1 to clear interrupt that core1 initiate illegal PIF bus access."]
    #[inline(always)]
    #[must_use]
    pub fn core_1_pif_pms_monitor_violate_clr(
        &mut self,
    ) -> CORE_1_PIF_PMS_MONITOR_VIOLATE_CLR_W<CORE_1_PIF_PMS_MONITOR_1_SPEC> {
        CORE_1_PIF_PMS_MONITOR_VIOLATE_CLR_W::new(self, 0)
    }
    #[doc = "Bit 1 - Set 1 to enable interrupt that core1 initiate illegal PIF bus access."]
    #[inline(always)]
    #[must_use]
    pub fn core_1_pif_pms_monitor_violate_en(
        &mut self,
    ) -> CORE_1_PIF_PMS_MONITOR_VIOLATE_EN_W<CORE_1_PIF_PMS_MONITOR_1_SPEC> {
        CORE_1_PIF_PMS_MONITOR_VIOLATE_EN_W::new(self, 1)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "core1 permission report register 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_1_pif_pms_monitor_1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_1_pif_pms_monitor_1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CORE_1_PIF_PMS_MONITOR_1_SPEC;
impl crate::RegisterSpec for CORE_1_PIF_PMS_MONITOR_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core_1_pif_pms_monitor_1::R`](R) reader structure"]
impl crate::Readable for CORE_1_PIF_PMS_MONITOR_1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`core_1_pif_pms_monitor_1::W`](W) writer structure"]
impl crate::Writable for CORE_1_PIF_PMS_MONITOR_1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CORE_1_PIF_PMS_MONITOR_1 to value 0x03"]
impl crate::Resettable for CORE_1_PIF_PMS_MONITOR_1_SPEC {
    const RESET_VALUE: u32 = 0x03;
}
