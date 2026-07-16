#[doc = "Register `HP_CPUPERI1_0` reader"]
pub type R = crate::R<HP_CPUPERI1_0_SPEC>;
#[doc = "Register `HP_CPUPERI1_0` writer"]
pub type W = crate::W<HP_CPUPERI1_0_SPEC>;
#[doc = "Field `HP_CPUPERI1_PMS_EXCEPTION_CLR` writer - Configures whether or not to clear hp_cpuperi1 peri_pms_record_reg.\\\\ 0: No clear\\\\ 1: Clear peri_pms_record_reg\\\\"]
pub type HP_CPUPERI1_PMS_EXCEPTION_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_CPUPERI1_PMS_EXCEPTION_DET` reader - Represents whether the hp_cpuperi1 pms has been triggered.\\\\ 0: No triggered\\\\ 1: Has been triggered\\\\"]
pub type HP_CPUPERI1_PMS_EXCEPTION_DET_R = crate::BitReader;
#[doc = "Field `HP_CPUPERI1_PMS_EXCEPTION_ID` reader - Represents the master id when hp_cpuperi1 pms has been triggered.\\\\"]
pub type HP_CPUPERI1_PMS_EXCEPTION_ID_R = crate::FieldReader;
#[doc = "Field `HP_CPUPERI1_PMS_EXCEPTION_MODE` reader - Represents the security mode when hp_cpuperi1 pms has been triggered.\\\\"]
pub type HP_CPUPERI1_PMS_EXCEPTION_MODE_R = crate::FieldReader;
impl R {
    #[doc = "Bit 1 - Represents whether the hp_cpuperi1 pms has been triggered.\\\\ 0: No triggered\\\\ 1: Has been triggered\\\\"]
    #[inline(always)]
    pub fn hp_cpuperi1_pms_exception_det(&self) -> HP_CPUPERI1_PMS_EXCEPTION_DET_R {
        HP_CPUPERI1_PMS_EXCEPTION_DET_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:7 - Represents the master id when hp_cpuperi1 pms has been triggered.\\\\"]
    #[inline(always)]
    pub fn hp_cpuperi1_pms_exception_id(&self) -> HP_CPUPERI1_PMS_EXCEPTION_ID_R {
        HP_CPUPERI1_PMS_EXCEPTION_ID_R::new(((self.bits >> 2) & 0x3f) as u8)
    }
    #[doc = "Bits 8:9 - Represents the security mode when hp_cpuperi1 pms has been triggered.\\\\"]
    #[inline(always)]
    pub fn hp_cpuperi1_pms_exception_mode(&self) -> HP_CPUPERI1_PMS_EXCEPTION_MODE_R {
        HP_CPUPERI1_PMS_EXCEPTION_MODE_R::new(((self.bits >> 8) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HP_CPUPERI1_0")
            .field(
                "hp_cpuperi1_pms_exception_det",
                &self.hp_cpuperi1_pms_exception_det(),
            )
            .field(
                "hp_cpuperi1_pms_exception_id",
                &self.hp_cpuperi1_pms_exception_id(),
            )
            .field(
                "hp_cpuperi1_pms_exception_mode",
                &self.hp_cpuperi1_pms_exception_mode(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Configures whether or not to clear hp_cpuperi1 peri_pms_record_reg.\\\\ 0: No clear\\\\ 1: Clear peri_pms_record_reg\\\\"]
    #[inline(always)]
    pub fn hp_cpuperi1_pms_exception_clr(
        &mut self,
    ) -> HP_CPUPERI1_PMS_EXCEPTION_CLR_W<'_, HP_CPUPERI1_0_SPEC> {
        HP_CPUPERI1_PMS_EXCEPTION_CLR_W::new(self, 0)
    }
}
#[doc = "HP_CPUPERI1 PMS configuration & info register\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_cpuperi1_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_cpuperi1_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HP_CPUPERI1_0_SPEC;
impl crate::RegisterSpec for HP_CPUPERI1_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hp_cpuperi1_0::R`](R) reader structure"]
impl crate::Readable for HP_CPUPERI1_0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hp_cpuperi1_0::W`](W) writer structure"]
impl crate::Writable for HP_CPUPERI1_0_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HP_CPUPERI1_0 to value 0"]
impl crate::Resettable for HP_CPUPERI1_0_SPEC {}
