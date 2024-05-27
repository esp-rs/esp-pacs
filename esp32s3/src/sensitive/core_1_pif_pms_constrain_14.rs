#[doc = "Register `CORE_1_PIF_PMS_CONSTRAIN_14` reader"]
pub type R = crate::R<CORE_1_PIF_PMS_CONSTRAIN_14_SPEC>;
#[doc = "Register `CORE_1_PIF_PMS_CONSTRAIN_14` writer"]
pub type W = crate::W<CORE_1_PIF_PMS_CONSTRAIN_14_SPEC>;
#[doc = "Field `CORE_1_PIF_PMS_CONSTRAIN_RTCSLOW_1_WORLD_0_L` reader - RTCSlow_1 memory low region permission in world 0 for core1."]
pub type CORE_1_PIF_PMS_CONSTRAIN_RTCSLOW_1_WORLD_0_L_R = crate::FieldReader;
#[doc = "Field `CORE_1_PIF_PMS_CONSTRAIN_RTCSLOW_1_WORLD_0_L` writer - RTCSlow_1 memory low region permission in world 0 for core1."]
pub type CORE_1_PIF_PMS_CONSTRAIN_RTCSLOW_1_WORLD_0_L_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CORE_1_PIF_PMS_CONSTRAIN_RTCSLOW_1_WORLD_0_H` reader - RTCSlow_1 memory high region permission in world 0 for core1."]
pub type CORE_1_PIF_PMS_CONSTRAIN_RTCSLOW_1_WORLD_0_H_R = crate::FieldReader;
#[doc = "Field `CORE_1_PIF_PMS_CONSTRAIN_RTCSLOW_1_WORLD_0_H` writer - RTCSlow_1 memory high region permission in world 0 for core1."]
pub type CORE_1_PIF_PMS_CONSTRAIN_RTCSLOW_1_WORLD_0_H_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CORE_1_PIF_PMS_CONSTRAIN_RTCSLOW_1_WORLD_1_L` reader - RTCSlow_1 memory low region permission in world 1 for core1."]
pub type CORE_1_PIF_PMS_CONSTRAIN_RTCSLOW_1_WORLD_1_L_R = crate::FieldReader;
#[doc = "Field `CORE_1_PIF_PMS_CONSTRAIN_RTCSLOW_1_WORLD_1_L` writer - RTCSlow_1 memory low region permission in world 1 for core1."]
pub type CORE_1_PIF_PMS_CONSTRAIN_RTCSLOW_1_WORLD_1_L_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CORE_1_PIF_PMS_CONSTRAIN_RTCSLOW_1_WORLD_1_H` reader - RTCSlow_1 memory high region permission in world 1 for core1."]
pub type CORE_1_PIF_PMS_CONSTRAIN_RTCSLOW_1_WORLD_1_H_R = crate::FieldReader;
#[doc = "Field `CORE_1_PIF_PMS_CONSTRAIN_RTCSLOW_1_WORLD_1_H` writer - RTCSlow_1 memory high region permission in world 1 for core1."]
pub type CORE_1_PIF_PMS_CONSTRAIN_RTCSLOW_1_WORLD_1_H_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - RTCSlow_1 memory low region permission in world 0 for core1."]
    #[inline(always)]
    pub fn core_1_pif_pms_constrain_rtcslow_1_world_0_l(
        &self,
    ) -> CORE_1_PIF_PMS_CONSTRAIN_RTCSLOW_1_WORLD_0_L_R {
        CORE_1_PIF_PMS_CONSTRAIN_RTCSLOW_1_WORLD_0_L_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - RTCSlow_1 memory high region permission in world 0 for core1."]
    #[inline(always)]
    pub fn core_1_pif_pms_constrain_rtcslow_1_world_0_h(
        &self,
    ) -> CORE_1_PIF_PMS_CONSTRAIN_RTCSLOW_1_WORLD_0_H_R {
        CORE_1_PIF_PMS_CONSTRAIN_RTCSLOW_1_WORLD_0_H_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:8 - RTCSlow_1 memory low region permission in world 1 for core1."]
    #[inline(always)]
    pub fn core_1_pif_pms_constrain_rtcslow_1_world_1_l(
        &self,
    ) -> CORE_1_PIF_PMS_CONSTRAIN_RTCSLOW_1_WORLD_1_L_R {
        CORE_1_PIF_PMS_CONSTRAIN_RTCSLOW_1_WORLD_1_L_R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 9:11 - RTCSlow_1 memory high region permission in world 1 for core1."]
    #[inline(always)]
    pub fn core_1_pif_pms_constrain_rtcslow_1_world_1_h(
        &self,
    ) -> CORE_1_PIF_PMS_CONSTRAIN_RTCSLOW_1_WORLD_1_H_R {
        CORE_1_PIF_PMS_CONSTRAIN_RTCSLOW_1_WORLD_1_H_R::new(((self.bits >> 9) & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORE_1_PIF_PMS_CONSTRAIN_14")
            .field(
                "core_1_pif_pms_constrain_rtcslow_1_world_0_l",
                &self.core_1_pif_pms_constrain_rtcslow_1_world_0_l(),
            )
            .field(
                "core_1_pif_pms_constrain_rtcslow_1_world_0_h",
                &self.core_1_pif_pms_constrain_rtcslow_1_world_0_h(),
            )
            .field(
                "core_1_pif_pms_constrain_rtcslow_1_world_1_l",
                &self.core_1_pif_pms_constrain_rtcslow_1_world_1_l(),
            )
            .field(
                "core_1_pif_pms_constrain_rtcslow_1_world_1_h",
                &self.core_1_pif_pms_constrain_rtcslow_1_world_1_h(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - RTCSlow_1 memory low region permission in world 0 for core1."]
    #[inline(always)]
    #[must_use]
    pub fn core_1_pif_pms_constrain_rtcslow_1_world_0_l(
        &mut self,
    ) -> CORE_1_PIF_PMS_CONSTRAIN_RTCSLOW_1_WORLD_0_L_W<CORE_1_PIF_PMS_CONSTRAIN_14_SPEC> {
        CORE_1_PIF_PMS_CONSTRAIN_RTCSLOW_1_WORLD_0_L_W::new(self, 0)
    }
    #[doc = "Bits 3:5 - RTCSlow_1 memory high region permission in world 0 for core1."]
    #[inline(always)]
    #[must_use]
    pub fn core_1_pif_pms_constrain_rtcslow_1_world_0_h(
        &mut self,
    ) -> CORE_1_PIF_PMS_CONSTRAIN_RTCSLOW_1_WORLD_0_H_W<CORE_1_PIF_PMS_CONSTRAIN_14_SPEC> {
        CORE_1_PIF_PMS_CONSTRAIN_RTCSLOW_1_WORLD_0_H_W::new(self, 3)
    }
    #[doc = "Bits 6:8 - RTCSlow_1 memory low region permission in world 1 for core1."]
    #[inline(always)]
    #[must_use]
    pub fn core_1_pif_pms_constrain_rtcslow_1_world_1_l(
        &mut self,
    ) -> CORE_1_PIF_PMS_CONSTRAIN_RTCSLOW_1_WORLD_1_L_W<CORE_1_PIF_PMS_CONSTRAIN_14_SPEC> {
        CORE_1_PIF_PMS_CONSTRAIN_RTCSLOW_1_WORLD_1_L_W::new(self, 6)
    }
    #[doc = "Bits 9:11 - RTCSlow_1 memory high region permission in world 1 for core1."]
    #[inline(always)]
    #[must_use]
    pub fn core_1_pif_pms_constrain_rtcslow_1_world_1_h(
        &mut self,
    ) -> CORE_1_PIF_PMS_CONSTRAIN_RTCSLOW_1_WORLD_1_H_W<CORE_1_PIF_PMS_CONSTRAIN_14_SPEC> {
        CORE_1_PIF_PMS_CONSTRAIN_RTCSLOW_1_WORLD_1_H_W::new(self, 9)
    }
}
#[doc = "core1 access peripherals permission configuration register 14.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_1_pif_pms_constrain_14::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_1_pif_pms_constrain_14::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CORE_1_PIF_PMS_CONSTRAIN_14_SPEC;
impl crate::RegisterSpec for CORE_1_PIF_PMS_CONSTRAIN_14_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core_1_pif_pms_constrain_14::R`](R) reader structure"]
impl crate::Readable for CORE_1_PIF_PMS_CONSTRAIN_14_SPEC {}
#[doc = "`write(|w| ..)` method takes [`core_1_pif_pms_constrain_14::W`](W) writer structure"]
impl crate::Writable for CORE_1_PIF_PMS_CONSTRAIN_14_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CORE_1_PIF_PMS_CONSTRAIN_14 to value 0x0fff"]
impl crate::Resettable for CORE_1_PIF_PMS_CONSTRAIN_14_SPEC {
    const RESET_VALUE: u32 = 0x0fff;
}
