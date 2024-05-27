///Register `SAR_COCPU_INT_ENA` reader
pub type R = crate::R<SAR_COCPU_INT_ENA_SPEC>;
///Register `SAR_COCPU_INT_ENA` writer
pub type W = crate::W<SAR_COCPU_INT_ENA_SPEC>;
///Field `SAR_COCPU_TOUCH_DONE_INT_ENA` reader - int enable of touch done
pub type SAR_COCPU_TOUCH_DONE_INT_ENA_R = crate::BitReader;
///Field `SAR_COCPU_TOUCH_DONE_INT_ENA` writer - int enable of touch done
pub type SAR_COCPU_TOUCH_DONE_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SAR_COCPU_TOUCH_INACTIVE_INT_ENA` reader - int enable of from touch inactive
pub type SAR_COCPU_TOUCH_INACTIVE_INT_ENA_R = crate::BitReader;
///Field `SAR_COCPU_TOUCH_INACTIVE_INT_ENA` writer - int enable of from touch inactive
pub type SAR_COCPU_TOUCH_INACTIVE_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SAR_COCPU_TOUCH_ACTIVE_INT_ENA` reader - int enable of touch active
pub type SAR_COCPU_TOUCH_ACTIVE_INT_ENA_R = crate::BitReader;
///Field `SAR_COCPU_TOUCH_ACTIVE_INT_ENA` writer - int enable of touch active
pub type SAR_COCPU_TOUCH_ACTIVE_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SAR_COCPU_SARADC1_INT_ENA` reader - int enable of from saradc1
pub type SAR_COCPU_SARADC1_INT_ENA_R = crate::BitReader;
///Field `SAR_COCPU_SARADC1_INT_ENA` writer - int enable of from saradc1
pub type SAR_COCPU_SARADC1_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SAR_COCPU_SARADC2_INT_ENA` reader - int enable of from saradc2
pub type SAR_COCPU_SARADC2_INT_ENA_R = crate::BitReader;
///Field `SAR_COCPU_SARADC2_INT_ENA` writer - int enable of from saradc2
pub type SAR_COCPU_SARADC2_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SAR_COCPU_TSENS_INT_ENA` reader - int enable of tsens
pub type SAR_COCPU_TSENS_INT_ENA_R = crate::BitReader;
///Field `SAR_COCPU_TSENS_INT_ENA` writer - int enable of tsens
pub type SAR_COCPU_TSENS_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SAR_COCPU_START_INT_ENA` reader - int enable of start
pub type SAR_COCPU_START_INT_ENA_R = crate::BitReader;
///Field `SAR_COCPU_START_INT_ENA` writer - int enable of start
pub type SAR_COCPU_START_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SAR_COCPU_SW_INT_ENA` reader - int enable of software
pub type SAR_COCPU_SW_INT_ENA_R = crate::BitReader;
///Field `SAR_COCPU_SW_INT_ENA` writer - int enable of software
pub type SAR_COCPU_SW_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SAR_COCPU_SWD_INT_ENA` reader - int enable of super watch dog
pub type SAR_COCPU_SWD_INT_ENA_R = crate::BitReader;
///Field `SAR_COCPU_SWD_INT_ENA` writer - int enable of super watch dog
pub type SAR_COCPU_SWD_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SAR_COCPU_TOUCH_TIMEOUT_INT_ENA` reader - int enable of timeout done
pub type SAR_COCPU_TOUCH_TIMEOUT_INT_ENA_R = crate::BitReader;
///Field `SAR_COCPU_TOUCH_TIMEOUT_INT_ENA` writer - int enable of timeout done
pub type SAR_COCPU_TOUCH_TIMEOUT_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SAR_COCPU_TOUCH_APPROACH_LOOP_DONE_INT_ENA` reader - int enable of approach loop done
pub type SAR_COCPU_TOUCH_APPROACH_LOOP_DONE_INT_ENA_R = crate::BitReader;
///Field `SAR_COCPU_TOUCH_APPROACH_LOOP_DONE_INT_ENA` writer - int enable of approach loop done
pub type SAR_COCPU_TOUCH_APPROACH_LOOP_DONE_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SAR_COCPU_TOUCH_SCAN_DONE_INT_ENA` reader - int enable of touch scan done
pub type SAR_COCPU_TOUCH_SCAN_DONE_INT_ENA_R = crate::BitReader;
///Field `SAR_COCPU_TOUCH_SCAN_DONE_INT_ENA` writer - int enable of touch scan done
pub type SAR_COCPU_TOUCH_SCAN_DONE_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - int enable of touch done
    #[inline(always)]
    pub fn sar_cocpu_touch_done_int_ena(&self) -> SAR_COCPU_TOUCH_DONE_INT_ENA_R {
        SAR_COCPU_TOUCH_DONE_INT_ENA_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - int enable of from touch inactive
    #[inline(always)]
    pub fn sar_cocpu_touch_inactive_int_ena(&self) -> SAR_COCPU_TOUCH_INACTIVE_INT_ENA_R {
        SAR_COCPU_TOUCH_INACTIVE_INT_ENA_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - int enable of touch active
    #[inline(always)]
    pub fn sar_cocpu_touch_active_int_ena(&self) -> SAR_COCPU_TOUCH_ACTIVE_INT_ENA_R {
        SAR_COCPU_TOUCH_ACTIVE_INT_ENA_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - int enable of from saradc1
    #[inline(always)]
    pub fn sar_cocpu_saradc1_int_ena(&self) -> SAR_COCPU_SARADC1_INT_ENA_R {
        SAR_COCPU_SARADC1_INT_ENA_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - int enable of from saradc2
    #[inline(always)]
    pub fn sar_cocpu_saradc2_int_ena(&self) -> SAR_COCPU_SARADC2_INT_ENA_R {
        SAR_COCPU_SARADC2_INT_ENA_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - int enable of tsens
    #[inline(always)]
    pub fn sar_cocpu_tsens_int_ena(&self) -> SAR_COCPU_TSENS_INT_ENA_R {
        SAR_COCPU_TSENS_INT_ENA_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - int enable of start
    #[inline(always)]
    pub fn sar_cocpu_start_int_ena(&self) -> SAR_COCPU_START_INT_ENA_R {
        SAR_COCPU_START_INT_ENA_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - int enable of software
    #[inline(always)]
    pub fn sar_cocpu_sw_int_ena(&self) -> SAR_COCPU_SW_INT_ENA_R {
        SAR_COCPU_SW_INT_ENA_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - int enable of super watch dog
    #[inline(always)]
    pub fn sar_cocpu_swd_int_ena(&self) -> SAR_COCPU_SWD_INT_ENA_R {
        SAR_COCPU_SWD_INT_ENA_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - int enable of timeout done
    #[inline(always)]
    pub fn sar_cocpu_touch_timeout_int_ena(&self) -> SAR_COCPU_TOUCH_TIMEOUT_INT_ENA_R {
        SAR_COCPU_TOUCH_TIMEOUT_INT_ENA_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - int enable of approach loop done
    #[inline(always)]
    pub fn sar_cocpu_touch_approach_loop_done_int_ena(
        &self,
    ) -> SAR_COCPU_TOUCH_APPROACH_LOOP_DONE_INT_ENA_R {
        SAR_COCPU_TOUCH_APPROACH_LOOP_DONE_INT_ENA_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - int enable of touch scan done
    #[inline(always)]
    pub fn sar_cocpu_touch_scan_done_int_ena(&self) -> SAR_COCPU_TOUCH_SCAN_DONE_INT_ENA_R {
        SAR_COCPU_TOUCH_SCAN_DONE_INT_ENA_R::new(((self.bits >> 11) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAR_COCPU_INT_ENA")
            .field(
                "sar_cocpu_touch_done_int_ena",
                &self.sar_cocpu_touch_done_int_ena(),
            )
            .field(
                "sar_cocpu_touch_inactive_int_ena",
                &self.sar_cocpu_touch_inactive_int_ena(),
            )
            .field(
                "sar_cocpu_touch_active_int_ena",
                &self.sar_cocpu_touch_active_int_ena(),
            )
            .field(
                "sar_cocpu_saradc1_int_ena",
                &self.sar_cocpu_saradc1_int_ena(),
            )
            .field(
                "sar_cocpu_saradc2_int_ena",
                &self.sar_cocpu_saradc2_int_ena(),
            )
            .field("sar_cocpu_tsens_int_ena", &self.sar_cocpu_tsens_int_ena())
            .field("sar_cocpu_start_int_ena", &self.sar_cocpu_start_int_ena())
            .field("sar_cocpu_sw_int_ena", &self.sar_cocpu_sw_int_ena())
            .field("sar_cocpu_swd_int_ena", &self.sar_cocpu_swd_int_ena())
            .field(
                "sar_cocpu_touch_timeout_int_ena",
                &self.sar_cocpu_touch_timeout_int_ena(),
            )
            .field(
                "sar_cocpu_touch_approach_loop_done_int_ena",
                &self.sar_cocpu_touch_approach_loop_done_int_ena(),
            )
            .field(
                "sar_cocpu_touch_scan_done_int_ena",
                &self.sar_cocpu_touch_scan_done_int_ena(),
            )
            .finish()
    }
}
impl W {
    ///Bit 0 - int enable of touch done
    #[inline(always)]
    #[must_use]
    pub fn sar_cocpu_touch_done_int_ena(
        &mut self,
    ) -> SAR_COCPU_TOUCH_DONE_INT_ENA_W<SAR_COCPU_INT_ENA_SPEC> {
        SAR_COCPU_TOUCH_DONE_INT_ENA_W::new(self, 0)
    }
    ///Bit 1 - int enable of from touch inactive
    #[inline(always)]
    #[must_use]
    pub fn sar_cocpu_touch_inactive_int_ena(
        &mut self,
    ) -> SAR_COCPU_TOUCH_INACTIVE_INT_ENA_W<SAR_COCPU_INT_ENA_SPEC> {
        SAR_COCPU_TOUCH_INACTIVE_INT_ENA_W::new(self, 1)
    }
    ///Bit 2 - int enable of touch active
    #[inline(always)]
    #[must_use]
    pub fn sar_cocpu_touch_active_int_ena(
        &mut self,
    ) -> SAR_COCPU_TOUCH_ACTIVE_INT_ENA_W<SAR_COCPU_INT_ENA_SPEC> {
        SAR_COCPU_TOUCH_ACTIVE_INT_ENA_W::new(self, 2)
    }
    ///Bit 3 - int enable of from saradc1
    #[inline(always)]
    #[must_use]
    pub fn sar_cocpu_saradc1_int_ena(
        &mut self,
    ) -> SAR_COCPU_SARADC1_INT_ENA_W<SAR_COCPU_INT_ENA_SPEC> {
        SAR_COCPU_SARADC1_INT_ENA_W::new(self, 3)
    }
    ///Bit 4 - int enable of from saradc2
    #[inline(always)]
    #[must_use]
    pub fn sar_cocpu_saradc2_int_ena(
        &mut self,
    ) -> SAR_COCPU_SARADC2_INT_ENA_W<SAR_COCPU_INT_ENA_SPEC> {
        SAR_COCPU_SARADC2_INT_ENA_W::new(self, 4)
    }
    ///Bit 5 - int enable of tsens
    #[inline(always)]
    #[must_use]
    pub fn sar_cocpu_tsens_int_ena(&mut self) -> SAR_COCPU_TSENS_INT_ENA_W<SAR_COCPU_INT_ENA_SPEC> {
        SAR_COCPU_TSENS_INT_ENA_W::new(self, 5)
    }
    ///Bit 6 - int enable of start
    #[inline(always)]
    #[must_use]
    pub fn sar_cocpu_start_int_ena(&mut self) -> SAR_COCPU_START_INT_ENA_W<SAR_COCPU_INT_ENA_SPEC> {
        SAR_COCPU_START_INT_ENA_W::new(self, 6)
    }
    ///Bit 7 - int enable of software
    #[inline(always)]
    #[must_use]
    pub fn sar_cocpu_sw_int_ena(&mut self) -> SAR_COCPU_SW_INT_ENA_W<SAR_COCPU_INT_ENA_SPEC> {
        SAR_COCPU_SW_INT_ENA_W::new(self, 7)
    }
    ///Bit 8 - int enable of super watch dog
    #[inline(always)]
    #[must_use]
    pub fn sar_cocpu_swd_int_ena(&mut self) -> SAR_COCPU_SWD_INT_ENA_W<SAR_COCPU_INT_ENA_SPEC> {
        SAR_COCPU_SWD_INT_ENA_W::new(self, 8)
    }
    ///Bit 9 - int enable of timeout done
    #[inline(always)]
    #[must_use]
    pub fn sar_cocpu_touch_timeout_int_ena(
        &mut self,
    ) -> SAR_COCPU_TOUCH_TIMEOUT_INT_ENA_W<SAR_COCPU_INT_ENA_SPEC> {
        SAR_COCPU_TOUCH_TIMEOUT_INT_ENA_W::new(self, 9)
    }
    ///Bit 10 - int enable of approach loop done
    #[inline(always)]
    #[must_use]
    pub fn sar_cocpu_touch_approach_loop_done_int_ena(
        &mut self,
    ) -> SAR_COCPU_TOUCH_APPROACH_LOOP_DONE_INT_ENA_W<SAR_COCPU_INT_ENA_SPEC> {
        SAR_COCPU_TOUCH_APPROACH_LOOP_DONE_INT_ENA_W::new(self, 10)
    }
    ///Bit 11 - int enable of touch scan done
    #[inline(always)]
    #[must_use]
    pub fn sar_cocpu_touch_scan_done_int_ena(
        &mut self,
    ) -> SAR_COCPU_TOUCH_SCAN_DONE_INT_ENA_W<SAR_COCPU_INT_ENA_SPEC> {
        SAR_COCPU_TOUCH_SCAN_DONE_INT_ENA_W::new(self, 11)
    }
}
/**the interrupt enable of ulp

You can [`read`](crate::generic::Reg::read) this register and get [`sar_cocpu_int_ena::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sar_cocpu_int_ena::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SAR_COCPU_INT_ENA_SPEC;
impl crate::RegisterSpec for SAR_COCPU_INT_ENA_SPEC {
    type Ux = u32;
}
///`read()` method returns [`sar_cocpu_int_ena::R`](R) reader structure
impl crate::Readable for SAR_COCPU_INT_ENA_SPEC {}
///`write(|w| ..)` method takes [`sar_cocpu_int_ena::W`](W) writer structure
impl crate::Writable for SAR_COCPU_INT_ENA_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SAR_COCPU_INT_ENA to value 0
impl crate::Resettable for SAR_COCPU_INT_ENA_SPEC {
    const RESET_VALUE: u32 = 0;
}
