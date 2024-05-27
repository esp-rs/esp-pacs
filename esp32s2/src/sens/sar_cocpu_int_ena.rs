///Register `SAR_COCPU_INT_ENA` reader
pub type R = crate::R<SAR_COCPU_INT_ENA_SPEC>;
///Register `SAR_COCPU_INT_ENA` writer
pub type W = crate::W<SAR_COCPU_INT_ENA_SPEC>;
///Field `COCPU_TOUCH_DONE_INT_ENA` reader - TOUCH_DONE_INT interrupt enable bit
pub type COCPU_TOUCH_DONE_INT_ENA_R = crate::BitReader;
///Field `COCPU_TOUCH_DONE_INT_ENA` writer - TOUCH_DONE_INT interrupt enable bit
pub type COCPU_TOUCH_DONE_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `COCPU_TOUCH_INACTIVE_INT_ENA` reader - TOUCH_INACTIVE_INT interrupt enable bit
pub type COCPU_TOUCH_INACTIVE_INT_ENA_R = crate::BitReader;
///Field `COCPU_TOUCH_INACTIVE_INT_ENA` writer - TOUCH_INACTIVE_INT interrupt enable bit
pub type COCPU_TOUCH_INACTIVE_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `COCPU_TOUCH_ACTIVE_INT_ENA` reader - TOUCH_ACTIVE_INT interrupt enable bit
pub type COCPU_TOUCH_ACTIVE_INT_ENA_R = crate::BitReader;
///Field `COCPU_TOUCH_ACTIVE_INT_ENA` writer - TOUCH_ACTIVE_INT interrupt enable bit
pub type COCPU_TOUCH_ACTIVE_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `COCPU_SARADC1_INT_ENA` reader - SARADC1_DONE_INT interrupt enable bit
pub type COCPU_SARADC1_INT_ENA_R = crate::BitReader;
///Field `COCPU_SARADC1_INT_ENA` writer - SARADC1_DONE_INT interrupt enable bit
pub type COCPU_SARADC1_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `COCPU_SARADC2_INT_ENA` reader - SARADC2_DONE_INT interrupt enable bit
pub type COCPU_SARADC2_INT_ENA_R = crate::BitReader;
///Field `COCPU_SARADC2_INT_ENA` writer - SARADC2_DONE_INT interrupt enable bit
pub type COCPU_SARADC2_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `COCPU_TSENS_INT_ENA` reader - TSENS_DONE_INT interrupt enable bit
pub type COCPU_TSENS_INT_ENA_R = crate::BitReader;
///Field `COCPU_TSENS_INT_ENA` writer - TSENS_DONE_INT interrupt enable bit
pub type COCPU_TSENS_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `COCPU_START_INT_ENA` reader - RISCV_START_INT interrupt enable bit
pub type COCPU_START_INT_ENA_R = crate::BitReader;
///Field `COCPU_START_INT_ENA` writer - RISCV_START_INT interrupt enable bit
pub type COCPU_START_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `COCPU_SW_INT_ENA` reader - SW_INT interrupt enable bit
pub type COCPU_SW_INT_ENA_R = crate::BitReader;
///Field `COCPU_SW_INT_ENA` writer - SW_INT interrupt enable bit
pub type COCPU_SW_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `COCPU_SWD_INT_ENA` reader - SWD_INT interrupt enable bit
pub type COCPU_SWD_INT_ENA_R = crate::BitReader;
///Field `COCPU_SWD_INT_ENA` writer - SWD_INT interrupt enable bit
pub type COCPU_SWD_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - TOUCH_DONE_INT interrupt enable bit
    #[inline(always)]
    pub fn cocpu_touch_done_int_ena(&self) -> COCPU_TOUCH_DONE_INT_ENA_R {
        COCPU_TOUCH_DONE_INT_ENA_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - TOUCH_INACTIVE_INT interrupt enable bit
    #[inline(always)]
    pub fn cocpu_touch_inactive_int_ena(&self) -> COCPU_TOUCH_INACTIVE_INT_ENA_R {
        COCPU_TOUCH_INACTIVE_INT_ENA_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - TOUCH_ACTIVE_INT interrupt enable bit
    #[inline(always)]
    pub fn cocpu_touch_active_int_ena(&self) -> COCPU_TOUCH_ACTIVE_INT_ENA_R {
        COCPU_TOUCH_ACTIVE_INT_ENA_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - SARADC1_DONE_INT interrupt enable bit
    #[inline(always)]
    pub fn cocpu_saradc1_int_ena(&self) -> COCPU_SARADC1_INT_ENA_R {
        COCPU_SARADC1_INT_ENA_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - SARADC2_DONE_INT interrupt enable bit
    #[inline(always)]
    pub fn cocpu_saradc2_int_ena(&self) -> COCPU_SARADC2_INT_ENA_R {
        COCPU_SARADC2_INT_ENA_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - TSENS_DONE_INT interrupt enable bit
    #[inline(always)]
    pub fn cocpu_tsens_int_ena(&self) -> COCPU_TSENS_INT_ENA_R {
        COCPU_TSENS_INT_ENA_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - RISCV_START_INT interrupt enable bit
    #[inline(always)]
    pub fn cocpu_start_int_ena(&self) -> COCPU_START_INT_ENA_R {
        COCPU_START_INT_ENA_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - SW_INT interrupt enable bit
    #[inline(always)]
    pub fn cocpu_sw_int_ena(&self) -> COCPU_SW_INT_ENA_R {
        COCPU_SW_INT_ENA_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - SWD_INT interrupt enable bit
    #[inline(always)]
    pub fn cocpu_swd_int_ena(&self) -> COCPU_SWD_INT_ENA_R {
        COCPU_SWD_INT_ENA_R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAR_COCPU_INT_ENA")
            .field("cocpu_touch_done_int_ena", &self.cocpu_touch_done_int_ena())
            .field(
                "cocpu_touch_inactive_int_ena",
                &self.cocpu_touch_inactive_int_ena(),
            )
            .field(
                "cocpu_touch_active_int_ena",
                &self.cocpu_touch_active_int_ena(),
            )
            .field("cocpu_saradc1_int_ena", &self.cocpu_saradc1_int_ena())
            .field("cocpu_saradc2_int_ena", &self.cocpu_saradc2_int_ena())
            .field("cocpu_tsens_int_ena", &self.cocpu_tsens_int_ena())
            .field("cocpu_start_int_ena", &self.cocpu_start_int_ena())
            .field("cocpu_sw_int_ena", &self.cocpu_sw_int_ena())
            .field("cocpu_swd_int_ena", &self.cocpu_swd_int_ena())
            .finish()
    }
}
impl W {
    ///Bit 0 - TOUCH_DONE_INT interrupt enable bit
    #[inline(always)]
    #[must_use]
    pub fn cocpu_touch_done_int_ena(
        &mut self,
    ) -> COCPU_TOUCH_DONE_INT_ENA_W<SAR_COCPU_INT_ENA_SPEC> {
        COCPU_TOUCH_DONE_INT_ENA_W::new(self, 0)
    }
    ///Bit 1 - TOUCH_INACTIVE_INT interrupt enable bit
    #[inline(always)]
    #[must_use]
    pub fn cocpu_touch_inactive_int_ena(
        &mut self,
    ) -> COCPU_TOUCH_INACTIVE_INT_ENA_W<SAR_COCPU_INT_ENA_SPEC> {
        COCPU_TOUCH_INACTIVE_INT_ENA_W::new(self, 1)
    }
    ///Bit 2 - TOUCH_ACTIVE_INT interrupt enable bit
    #[inline(always)]
    #[must_use]
    pub fn cocpu_touch_active_int_ena(
        &mut self,
    ) -> COCPU_TOUCH_ACTIVE_INT_ENA_W<SAR_COCPU_INT_ENA_SPEC> {
        COCPU_TOUCH_ACTIVE_INT_ENA_W::new(self, 2)
    }
    ///Bit 3 - SARADC1_DONE_INT interrupt enable bit
    #[inline(always)]
    #[must_use]
    pub fn cocpu_saradc1_int_ena(&mut self) -> COCPU_SARADC1_INT_ENA_W<SAR_COCPU_INT_ENA_SPEC> {
        COCPU_SARADC1_INT_ENA_W::new(self, 3)
    }
    ///Bit 4 - SARADC2_DONE_INT interrupt enable bit
    #[inline(always)]
    #[must_use]
    pub fn cocpu_saradc2_int_ena(&mut self) -> COCPU_SARADC2_INT_ENA_W<SAR_COCPU_INT_ENA_SPEC> {
        COCPU_SARADC2_INT_ENA_W::new(self, 4)
    }
    ///Bit 5 - TSENS_DONE_INT interrupt enable bit
    #[inline(always)]
    #[must_use]
    pub fn cocpu_tsens_int_ena(&mut self) -> COCPU_TSENS_INT_ENA_W<SAR_COCPU_INT_ENA_SPEC> {
        COCPU_TSENS_INT_ENA_W::new(self, 5)
    }
    ///Bit 6 - RISCV_START_INT interrupt enable bit
    #[inline(always)]
    #[must_use]
    pub fn cocpu_start_int_ena(&mut self) -> COCPU_START_INT_ENA_W<SAR_COCPU_INT_ENA_SPEC> {
        COCPU_START_INT_ENA_W::new(self, 6)
    }
    ///Bit 7 - SW_INT interrupt enable bit
    #[inline(always)]
    #[must_use]
    pub fn cocpu_sw_int_ena(&mut self) -> COCPU_SW_INT_ENA_W<SAR_COCPU_INT_ENA_SPEC> {
        COCPU_SW_INT_ENA_W::new(self, 7)
    }
    ///Bit 8 - SWD_INT interrupt enable bit
    #[inline(always)]
    #[must_use]
    pub fn cocpu_swd_int_ena(&mut self) -> COCPU_SWD_INT_ENA_W<SAR_COCPU_INT_ENA_SPEC> {
        COCPU_SWD_INT_ENA_W::new(self, 8)
    }
}
/**Interrupt enable bit of ULP-RISCV

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
