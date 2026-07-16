#[doc = "Register `MODSLP_STATUS` reader"]
pub type R = crate::R<MODSLP_STATUS_SPEC>;
#[doc = "Register `MODSLP_STATUS` writer"]
pub type W = crate::W<MODSLP_STATUS_SPEC>;
#[doc = "Field `MODSLP_RUN_TIME_WAKE` reader - "]
pub type MODSLP_RUN_TIME_WAKE_R = crate::BitReader;
#[doc = "Field `MODSLP_RUN_TIME_WAKE` writer - "]
pub type MODSLP_RUN_TIME_WAKE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODSLP_RFOFF_WAKE` reader - "]
pub type MODSLP_RFOFF_WAKE_R = crate::BitReader;
#[doc = "Field `MODSLP_RFOFF_WAKE` writer - "]
pub type MODSLP_RFOFF_WAKE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODSLP_INTR_WAKE` reader - "]
pub type MODSLP_INTR_WAKE_R = crate::BitReader;
#[doc = "Field `MODSLP_INTR_WAKE` writer - "]
pub type MODSLP_INTR_WAKE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODSLP_RUN_TIME_CNT` reader - "]
pub type MODSLP_RUN_TIME_CNT_R = crate::FieldReader;
#[doc = "Field `MODSLP_RUN_TIME_CNT` writer - "]
pub type MODSLP_RUN_TIME_CNT_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn modslp_run_time_wake(&self) -> MODSLP_RUN_TIME_WAKE_R {
        MODSLP_RUN_TIME_WAKE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn modslp_rfoff_wake(&self) -> MODSLP_RFOFF_WAKE_R {
        MODSLP_RFOFF_WAKE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn modslp_intr_wake(&self) -> MODSLP_INTR_WAKE_R {
        MODSLP_INTR_WAKE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn modslp_run_time_cnt(&self) -> MODSLP_RUN_TIME_CNT_R {
        MODSLP_RUN_TIME_CNT_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MODSLP_STATUS")
            .field("modslp_run_time_wake", &self.modslp_run_time_wake())
            .field("modslp_rfoff_wake", &self.modslp_rfoff_wake())
            .field("modslp_intr_wake", &self.modslp_intr_wake())
            .field("modslp_run_time_cnt", &self.modslp_run_time_cnt())
            .finish()
    }
}
impl W {
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn modslp_run_time_wake(&mut self) -> MODSLP_RUN_TIME_WAKE_W<'_, MODSLP_STATUS_SPEC> {
        MODSLP_RUN_TIME_WAKE_W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn modslp_rfoff_wake(&mut self) -> MODSLP_RFOFF_WAKE_W<'_, MODSLP_STATUS_SPEC> {
        MODSLP_RFOFF_WAKE_W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn modslp_intr_wake(&mut self) -> MODSLP_INTR_WAKE_W<'_, MODSLP_STATUS_SPEC> {
        MODSLP_INTR_WAKE_W::new(self, 3)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn modslp_run_time_cnt(&mut self) -> MODSLP_RUN_TIME_CNT_W<'_, MODSLP_STATUS_SPEC> {
        MODSLP_RUN_TIME_CNT_W::new(self, 4)
    }
}
#[doc = "MODSLP_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`modslp_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`modslp_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MODSLP_STATUS_SPEC;
impl crate::RegisterSpec for MODSLP_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`modslp_status::R`](R) reader structure"]
impl crate::Readable for MODSLP_STATUS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`modslp_status::W`](W) writer structure"]
impl crate::Writable for MODSLP_STATUS_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MODSLP_STATUS to value 0"]
impl crate::Resettable for MODSLP_STATUS_SPEC {}
