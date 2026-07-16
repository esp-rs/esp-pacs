#[doc = "Register `TASK_ST11` reader"]
pub type R = crate::R<TASK_ST11_SPEC>;
#[doc = "Register `TASK_ST11` writer"]
pub type W = crate::W<TASK_ST11_SPEC>;
#[doc = "Field `MODEM_TASK_G3_ST` reader - Represents MODEM_task_g3 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type MODEM_TASK_G3_ST_R = crate::BitReader;
#[doc = "Field `MODEM_TASK_G3_ST` writer - Represents MODEM_task_g3 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type MODEM_TASK_G3_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORDIC_TASK_CAL_START_ST` reader - Represents CORDIC_task_cal_start trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type CORDIC_TASK_CAL_START_ST_R = crate::BitReader;
#[doc = "Field `CORDIC_TASK_CAL_START_ST` writer - Represents CORDIC_task_cal_start trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type CORDIC_TASK_CAL_START_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ZERO_DET_TASK_START_ST` reader - Represents ZERO_DET_task_start trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type ZERO_DET_TASK_START_ST_R = crate::BitReader;
#[doc = "Field `ZERO_DET_TASK_START_ST` writer - Represents ZERO_DET_task_start trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type ZERO_DET_TASK_START_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYSTIMER_TASK_CNT_UPD0_ST` reader - Represents SYSTIMER_task_cnt_upd0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type SYSTIMER_TASK_CNT_UPD0_ST_R = crate::BitReader;
#[doc = "Field `SYSTIMER_TASK_CNT_UPD0_ST` writer - Represents SYSTIMER_task_cnt_upd0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type SYSTIMER_TASK_CNT_UPD0_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYSTIMER_TASK_CNT_UPD1_ST` reader - Represents SYSTIMER_task_cnt_upd1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type SYSTIMER_TASK_CNT_UPD1_ST_R = crate::BitReader;
#[doc = "Field `SYSTIMER_TASK_CNT_UPD1_ST` writer - Represents SYSTIMER_task_cnt_upd1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type SYSTIMER_TASK_CNT_UPD1_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Represents MODEM_task_g3 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn modem_task_g3_st(&self) -> MODEM_TASK_G3_ST_R {
        MODEM_TASK_G3_ST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Represents CORDIC_task_cal_start trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn cordic_task_cal_start_st(&self) -> CORDIC_TASK_CAL_START_ST_R {
        CORDIC_TASK_CAL_START_ST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Represents ZERO_DET_task_start trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn zero_det_task_start_st(&self) -> ZERO_DET_TASK_START_ST_R {
        ZERO_DET_TASK_START_ST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Represents SYSTIMER_task_cnt_upd0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn systimer_task_cnt_upd0_st(&self) -> SYSTIMER_TASK_CNT_UPD0_ST_R {
        SYSTIMER_TASK_CNT_UPD0_ST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Represents SYSTIMER_task_cnt_upd1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn systimer_task_cnt_upd1_st(&self) -> SYSTIMER_TASK_CNT_UPD1_ST_R {
        SYSTIMER_TASK_CNT_UPD1_ST_R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TASK_ST11")
            .field("modem_task_g3_st", &self.modem_task_g3_st())
            .field("cordic_task_cal_start_st", &self.cordic_task_cal_start_st())
            .field("zero_det_task_start_st", &self.zero_det_task_start_st())
            .field(
                "systimer_task_cnt_upd0_st",
                &self.systimer_task_cnt_upd0_st(),
            )
            .field(
                "systimer_task_cnt_upd1_st",
                &self.systimer_task_cnt_upd1_st(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Represents MODEM_task_g3 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn modem_task_g3_st(&mut self) -> MODEM_TASK_G3_ST_W<'_, TASK_ST11_SPEC> {
        MODEM_TASK_G3_ST_W::new(self, 0)
    }
    #[doc = "Bit 1 - Represents CORDIC_task_cal_start trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn cordic_task_cal_start_st(&mut self) -> CORDIC_TASK_CAL_START_ST_W<'_, TASK_ST11_SPEC> {
        CORDIC_TASK_CAL_START_ST_W::new(self, 1)
    }
    #[doc = "Bit 2 - Represents ZERO_DET_task_start trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn zero_det_task_start_st(&mut self) -> ZERO_DET_TASK_START_ST_W<'_, TASK_ST11_SPEC> {
        ZERO_DET_TASK_START_ST_W::new(self, 2)
    }
    #[doc = "Bit 3 - Represents SYSTIMER_task_cnt_upd0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn systimer_task_cnt_upd0_st(&mut self) -> SYSTIMER_TASK_CNT_UPD0_ST_W<'_, TASK_ST11_SPEC> {
        SYSTIMER_TASK_CNT_UPD0_ST_W::new(self, 3)
    }
    #[doc = "Bit 4 - Represents SYSTIMER_task_cnt_upd1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn systimer_task_cnt_upd1_st(&mut self) -> SYSTIMER_TASK_CNT_UPD1_ST_W<'_, TASK_ST11_SPEC> {
        SYSTIMER_TASK_CNT_UPD1_ST_W::new(self, 4)
    }
}
#[doc = "Tasks trigger status register\n\nYou can [`read`](crate::Reg::read) this register and get [`task_st11::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`task_st11::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TASK_ST11_SPEC;
impl crate::RegisterSpec for TASK_ST11_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`task_st11::R`](R) reader structure"]
impl crate::Readable for TASK_ST11_SPEC {}
#[doc = "`write(|w| ..)` method takes [`task_st11::W`](W) writer structure"]
impl crate::Writable for TASK_ST11_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TASK_ST11 to value 0"]
impl crate::Resettable for TASK_ST11_SPEC {}
