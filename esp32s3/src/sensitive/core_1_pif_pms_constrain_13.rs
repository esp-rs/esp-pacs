#[doc = "Register `CORE_1_PIF_PMS_CONSTRAIN_13` reader"]
pub type R = crate::R<CORE_1_PIF_PMS_CONSTRAIN_13_SPEC>;
#[doc = "Register `CORE_1_PIF_PMS_CONSTRAIN_13` writer"]
pub type W = crate::W<CORE_1_PIF_PMS_CONSTRAIN_13_SPEC>;
#[doc = "Field `CORE_1_PIF_PMS_CONSTRAIN_RTCSLOW_1_SPLTADDR_WORLD_0` reader - RTCSlow_1 memory split address in world 0 for core1."]
pub type CORE_1_PIF_PMS_CONSTRAIN_RTCSLOW_1_SPLTADDR_WORLD_0_R = crate::FieldReader<u16>;
#[doc = "Field `CORE_1_PIF_PMS_CONSTRAIN_RTCSLOW_1_SPLTADDR_WORLD_0` writer - RTCSlow_1 memory split address in world 0 for core1."]
pub type CORE_1_PIF_PMS_CONSTRAIN_RTCSLOW_1_SPLTADDR_WORLD_0_W<'a, REG> =
    crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `CORE_1_PIF_PMS_CONSTRAIN_RTCSLOW_1_SPLTADDR_WORLD_1` reader - RTCSlow_1 memory split address in world 1 for core1."]
pub type CORE_1_PIF_PMS_CONSTRAIN_RTCSLOW_1_SPLTADDR_WORLD_1_R = crate::FieldReader<u16>;
#[doc = "Field `CORE_1_PIF_PMS_CONSTRAIN_RTCSLOW_1_SPLTADDR_WORLD_1` writer - RTCSlow_1 memory split address in world 1 for core1."]
pub type CORE_1_PIF_PMS_CONSTRAIN_RTCSLOW_1_SPLTADDR_WORLD_1_W<'a, REG> =
    crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:10 - RTCSlow_1 memory split address in world 0 for core1."]
    #[inline(always)]
    pub fn core_1_pif_pms_constrain_rtcslow_1_spltaddr_world_0(
        &self,
    ) -> CORE_1_PIF_PMS_CONSTRAIN_RTCSLOW_1_SPLTADDR_WORLD_0_R {
        CORE_1_PIF_PMS_CONSTRAIN_RTCSLOW_1_SPLTADDR_WORLD_0_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:21 - RTCSlow_1 memory split address in world 1 for core1."]
    #[inline(always)]
    pub fn core_1_pif_pms_constrain_rtcslow_1_spltaddr_world_1(
        &self,
    ) -> CORE_1_PIF_PMS_CONSTRAIN_RTCSLOW_1_SPLTADDR_WORLD_1_R {
        CORE_1_PIF_PMS_CONSTRAIN_RTCSLOW_1_SPLTADDR_WORLD_1_R::new(
            ((self.bits >> 11) & 0x07ff) as u16,
        )
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORE_1_PIF_PMS_CONSTRAIN_13")
            .field(
                "core_1_pif_pms_constrain_rtcslow_1_spltaddr_world_0",
                &self.core_1_pif_pms_constrain_rtcslow_1_spltaddr_world_0(),
            )
            .field(
                "core_1_pif_pms_constrain_rtcslow_1_spltaddr_world_1",
                &self.core_1_pif_pms_constrain_rtcslow_1_spltaddr_world_1(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:10 - RTCSlow_1 memory split address in world 0 for core1."]
    #[inline(always)]
    pub fn core_1_pif_pms_constrain_rtcslow_1_spltaddr_world_0(
        &mut self,
    ) -> CORE_1_PIF_PMS_CONSTRAIN_RTCSLOW_1_SPLTADDR_WORLD_0_W<CORE_1_PIF_PMS_CONSTRAIN_13_SPEC>
    {
        CORE_1_PIF_PMS_CONSTRAIN_RTCSLOW_1_SPLTADDR_WORLD_0_W::new(self, 0)
    }
    #[doc = "Bits 11:21 - RTCSlow_1 memory split address in world 1 for core1."]
    #[inline(always)]
    pub fn core_1_pif_pms_constrain_rtcslow_1_spltaddr_world_1(
        &mut self,
    ) -> CORE_1_PIF_PMS_CONSTRAIN_RTCSLOW_1_SPLTADDR_WORLD_1_W<CORE_1_PIF_PMS_CONSTRAIN_13_SPEC>
    {
        CORE_1_PIF_PMS_CONSTRAIN_RTCSLOW_1_SPLTADDR_WORLD_1_W::new(self, 11)
    }
}
#[doc = "core1 access peripherals permission configuration register 13.\n\nYou can [`read`](crate::Reg::read) this register and get [`core_1_pif_pms_constrain_13::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core_1_pif_pms_constrain_13::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CORE_1_PIF_PMS_CONSTRAIN_13_SPEC;
impl crate::RegisterSpec for CORE_1_PIF_PMS_CONSTRAIN_13_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core_1_pif_pms_constrain_13::R`](R) reader structure"]
impl crate::Readable for CORE_1_PIF_PMS_CONSTRAIN_13_SPEC {}
#[doc = "`write(|w| ..)` method takes [`core_1_pif_pms_constrain_13::W`](W) writer structure"]
impl crate::Writable for CORE_1_PIF_PMS_CONSTRAIN_13_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CORE_1_PIF_PMS_CONSTRAIN_13 to value 0x003f_ffff"]
impl crate::Resettable for CORE_1_PIF_PMS_CONSTRAIN_13_SPEC {
    const RESET_VALUE: u32 = 0x003f_ffff;
}
