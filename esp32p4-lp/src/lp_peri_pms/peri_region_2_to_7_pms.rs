#[doc = "Register `PERI_REGION_2_TO_7_PMS` reader"]
pub type R = crate::R<PERI_REGION_2_TO_7_PMS_SPEC>;
#[doc = "Register `PERI_REGION_2_TO_7_PMS` writer"]
pub type W = crate::W<PERI_REGION_2_TO_7_PMS_SPEC>;
#[doc = "Field `REG_LP_CORE_REGION_2_TO_7_PMS` reader - NA"]
pub type REG_LP_CORE_REGION_2_TO_7_PMS_R = crate::FieldReader;
#[doc = "Field `REG_LP_CORE_REGION_2_TO_7_PMS` writer - NA"]
pub type REG_LP_CORE_REGION_2_TO_7_PMS_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `REG_HP_CORE0_UM_REGION_2_TO_7_PMS` reader - NA"]
pub type REG_HP_CORE0_UM_REGION_2_TO_7_PMS_R = crate::FieldReader;
#[doc = "Field `REG_HP_CORE0_UM_REGION_2_TO_7_PMS` writer - NA"]
pub type REG_HP_CORE0_UM_REGION_2_TO_7_PMS_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `REG_HP_CORE0_MM_REGION_2_TO_7_PMS` reader - NA"]
pub type REG_HP_CORE0_MM_REGION_2_TO_7_PMS_R = crate::FieldReader;
#[doc = "Field `REG_HP_CORE0_MM_REGION_2_TO_7_PMS` writer - NA"]
pub type REG_HP_CORE0_MM_REGION_2_TO_7_PMS_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `REG_HP_CORE1_UM_REGION_2_TO_7_PMS` reader - NA"]
pub type REG_HP_CORE1_UM_REGION_2_TO_7_PMS_R = crate::FieldReader;
#[doc = "Field `REG_HP_CORE1_UM_REGION_2_TO_7_PMS` writer - NA"]
pub type REG_HP_CORE1_UM_REGION_2_TO_7_PMS_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `REG_HP_CORE1_MM_REGION_2_TO_7_PMS` reader - NA"]
pub type REG_HP_CORE1_MM_REGION_2_TO_7_PMS_R = crate::FieldReader;
#[doc = "Field `REG_HP_CORE1_MM_REGION_2_TO_7_PMS` writer - NA"]
pub type REG_HP_CORE1_MM_REGION_2_TO_7_PMS_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - NA"]
    #[inline(always)]
    pub fn reg_lp_core_region_2_to_7_pms(&self) -> REG_LP_CORE_REGION_2_TO_7_PMS_R {
        REG_LP_CORE_REGION_2_TO_7_PMS_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:11 - NA"]
    #[inline(always)]
    pub fn reg_hp_core0_um_region_2_to_7_pms(&self) -> REG_HP_CORE0_UM_REGION_2_TO_7_PMS_R {
        REG_HP_CORE0_UM_REGION_2_TO_7_PMS_R::new(((self.bits >> 6) & 0x3f) as u8)
    }
    #[doc = "Bits 12:17 - NA"]
    #[inline(always)]
    pub fn reg_hp_core0_mm_region_2_to_7_pms(&self) -> REG_HP_CORE0_MM_REGION_2_TO_7_PMS_R {
        REG_HP_CORE0_MM_REGION_2_TO_7_PMS_R::new(((self.bits >> 12) & 0x3f) as u8)
    }
    #[doc = "Bits 18:23 - NA"]
    #[inline(always)]
    pub fn reg_hp_core1_um_region_2_to_7_pms(&self) -> REG_HP_CORE1_UM_REGION_2_TO_7_PMS_R {
        REG_HP_CORE1_UM_REGION_2_TO_7_PMS_R::new(((self.bits >> 18) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - NA"]
    #[inline(always)]
    pub fn reg_hp_core1_mm_region_2_to_7_pms(&self) -> REG_HP_CORE1_MM_REGION_2_TO_7_PMS_R {
        REG_HP_CORE1_MM_REGION_2_TO_7_PMS_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PERI_REGION_2_TO_7_PMS")
            .field(
                "reg_lp_core_region_2_to_7_pms",
                &self.reg_lp_core_region_2_to_7_pms(),
            )
            .field(
                "reg_hp_core0_um_region_2_to_7_pms",
                &self.reg_hp_core0_um_region_2_to_7_pms(),
            )
            .field(
                "reg_hp_core0_mm_region_2_to_7_pms",
                &self.reg_hp_core0_mm_region_2_to_7_pms(),
            )
            .field(
                "reg_hp_core1_um_region_2_to_7_pms",
                &self.reg_hp_core1_um_region_2_to_7_pms(),
            )
            .field(
                "reg_hp_core1_mm_region_2_to_7_pms",
                &self.reg_hp_core1_mm_region_2_to_7_pms(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:5 - NA"]
    #[inline(always)]
    pub fn reg_lp_core_region_2_to_7_pms(
        &mut self,
    ) -> REG_LP_CORE_REGION_2_TO_7_PMS_W<'_, PERI_REGION_2_TO_7_PMS_SPEC> {
        REG_LP_CORE_REGION_2_TO_7_PMS_W::new(self, 0)
    }
    #[doc = "Bits 6:11 - NA"]
    #[inline(always)]
    pub fn reg_hp_core0_um_region_2_to_7_pms(
        &mut self,
    ) -> REG_HP_CORE0_UM_REGION_2_TO_7_PMS_W<'_, PERI_REGION_2_TO_7_PMS_SPEC> {
        REG_HP_CORE0_UM_REGION_2_TO_7_PMS_W::new(self, 6)
    }
    #[doc = "Bits 12:17 - NA"]
    #[inline(always)]
    pub fn reg_hp_core0_mm_region_2_to_7_pms(
        &mut self,
    ) -> REG_HP_CORE0_MM_REGION_2_TO_7_PMS_W<'_, PERI_REGION_2_TO_7_PMS_SPEC> {
        REG_HP_CORE0_MM_REGION_2_TO_7_PMS_W::new(self, 12)
    }
    #[doc = "Bits 18:23 - NA"]
    #[inline(always)]
    pub fn reg_hp_core1_um_region_2_to_7_pms(
        &mut self,
    ) -> REG_HP_CORE1_UM_REGION_2_TO_7_PMS_W<'_, PERI_REGION_2_TO_7_PMS_SPEC> {
        REG_HP_CORE1_UM_REGION_2_TO_7_PMS_W::new(self, 18)
    }
    #[doc = "Bits 24:29 - NA"]
    #[inline(always)]
    pub fn reg_hp_core1_mm_region_2_to_7_pms(
        &mut self,
    ) -> REG_HP_CORE1_MM_REGION_2_TO_7_PMS_W<'_, PERI_REGION_2_TO_7_PMS_SPEC> {
        REG_HP_CORE1_MM_REGION_2_TO_7_PMS_W::new(self, 24)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`peri_region_2_to_7_pms::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peri_region_2_to_7_pms::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PERI_REGION_2_TO_7_PMS_SPEC;
impl crate::RegisterSpec for PERI_REGION_2_TO_7_PMS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`peri_region_2_to_7_pms::R`](R) reader structure"]
impl crate::Readable for PERI_REGION_2_TO_7_PMS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`peri_region_2_to_7_pms::W`](W) writer structure"]
impl crate::Writable for PERI_REGION_2_TO_7_PMS_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PERI_REGION_2_TO_7_PMS to value 0"]
impl crate::Resettable for PERI_REGION_2_TO_7_PMS_SPEC {}
