#[doc = "Register `MODSLP_CONFIG` reader"]
pub type R = crate::R<MODSLP_CONFIG_SPEC>;
#[doc = "Register `MODSLP_CONFIG` writer"]
pub type W = crate::W<MODSLP_CONFIG_SPEC>;
#[doc = "Field `MODSLP_MODE_ENA` reader - "]
pub type MODSLP_MODE_ENA_R = crate::BitReader;
#[doc = "Field `MODSLP_MODE_ENA` writer - "]
pub type MODSLP_MODE_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODSLP_RUN_TIME_WAKE_EN` reader - "]
pub type MODSLP_RUN_TIME_WAKE_EN_R = crate::BitReader;
#[doc = "Field `MODSLP_RUN_TIME_WAKE_EN` writer - "]
pub type MODSLP_RUN_TIME_WAKE_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODSLP_RFOFF_WAKE_EN` reader - "]
pub type MODSLP_RFOFF_WAKE_EN_R = crate::BitReader;
#[doc = "Field `MODSLP_RFOFF_WAKE_EN` writer - "]
pub type MODSLP_RFOFF_WAKE_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODSLP_INTR_WAKE_EN` reader - "]
pub type MODSLP_INTR_WAKE_EN_R = crate::BitReader;
#[doc = "Field `MODSLP_INTR_WAKE_EN` writer - "]
pub type MODSLP_INTR_WAKE_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODSLP_RUN_TIME_LIMIT` reader - "]
pub type MODSLP_RUN_TIME_LIMIT_R = crate::FieldReader;
#[doc = "Field `MODSLP_RUN_TIME_LIMIT` writer - "]
pub type MODSLP_RUN_TIME_LIMIT_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn modslp_mode_ena(&self) -> MODSLP_MODE_ENA_R {
        MODSLP_MODE_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn modslp_run_time_wake_en(&self) -> MODSLP_RUN_TIME_WAKE_EN_R {
        MODSLP_RUN_TIME_WAKE_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn modslp_rfoff_wake_en(&self) -> MODSLP_RFOFF_WAKE_EN_R {
        MODSLP_RFOFF_WAKE_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn modslp_intr_wake_en(&self) -> MODSLP_INTR_WAKE_EN_R {
        MODSLP_INTR_WAKE_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn modslp_run_time_limit(&self) -> MODSLP_RUN_TIME_LIMIT_R {
        MODSLP_RUN_TIME_LIMIT_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MODSLP_CONFIG")
            .field("modslp_mode_ena", &self.modslp_mode_ena())
            .field("modslp_run_time_wake_en", &self.modslp_run_time_wake_en())
            .field("modslp_rfoff_wake_en", &self.modslp_rfoff_wake_en())
            .field("modslp_intr_wake_en", &self.modslp_intr_wake_en())
            .field("modslp_run_time_limit", &self.modslp_run_time_limit())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn modslp_mode_ena(&mut self) -> MODSLP_MODE_ENA_W<'_, MODSLP_CONFIG_SPEC> {
        MODSLP_MODE_ENA_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn modslp_run_time_wake_en(&mut self) -> MODSLP_RUN_TIME_WAKE_EN_W<'_, MODSLP_CONFIG_SPEC> {
        MODSLP_RUN_TIME_WAKE_EN_W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn modslp_rfoff_wake_en(&mut self) -> MODSLP_RFOFF_WAKE_EN_W<'_, MODSLP_CONFIG_SPEC> {
        MODSLP_RFOFF_WAKE_EN_W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn modslp_intr_wake_en(&mut self) -> MODSLP_INTR_WAKE_EN_W<'_, MODSLP_CONFIG_SPEC> {
        MODSLP_INTR_WAKE_EN_W::new(self, 3)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn modslp_run_time_limit(&mut self) -> MODSLP_RUN_TIME_LIMIT_W<'_, MODSLP_CONFIG_SPEC> {
        MODSLP_RUN_TIME_LIMIT_W::new(self, 4)
    }
}
#[doc = "MODSLP_CONFIG\n\nYou can [`read`](crate::Reg::read) this register and get [`modslp_config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`modslp_config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MODSLP_CONFIG_SPEC;
impl crate::RegisterSpec for MODSLP_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`modslp_config::R`](R) reader structure"]
impl crate::Readable for MODSLP_CONFIG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`modslp_config::W`](W) writer structure"]
impl crate::Writable for MODSLP_CONFIG_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MODSLP_CONFIG to value 0"]
impl crate::Resettable for MODSLP_CONFIG_SPEC {}
