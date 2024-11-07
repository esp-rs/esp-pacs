#[doc = "Register `SAR_COCPU_INT_CLR` writer"]
pub type W = crate::W<SAR_COCPU_INT_CLR_SPEC>;
#[doc = "Field `COCPU_TOUCH_DONE_INT_CLR` writer - TOUCH_DONE_INT interrupt clear bit"]
pub type COCPU_TOUCH_DONE_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COCPU_TOUCH_INACTIVE_INT_CLR` writer - TOUCH_INACTIVE_INT interrupt clear bit"]
pub type COCPU_TOUCH_INACTIVE_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COCPU_TOUCH_ACTIVE_INT_CLR` writer - TOUCH_ACTIVE_INT interrupt clear bit"]
pub type COCPU_TOUCH_ACTIVE_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COCPU_SARADC1_INT_CLR` writer - SARADC1_DONE_INT interrupt clear bit"]
pub type COCPU_SARADC1_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COCPU_SARADC2_INT_CLR` writer - SARADC2_DONE_INT interrupt clear bit"]
pub type COCPU_SARADC2_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COCPU_TSENS_INT_CLR` writer - TSENS_DONE_INT interrupt clear bit"]
pub type COCPU_TSENS_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COCPU_START_INT_CLR` writer - RISCV_START_INT interrupt clear bit"]
pub type COCPU_START_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COCPU_SW_INT_CLR` writer - SW_INT interrupt clear bit"]
pub type COCPU_SW_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COCPU_SWD_INT_CLR` writer - SWD_INT interrupt clear bit"]
pub type COCPU_SWD_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SAR_COCPU_INT_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - TOUCH_DONE_INT interrupt clear bit"]
    #[inline(always)]
    pub fn cocpu_touch_done_int_clr(
        &mut self,
    ) -> COCPU_TOUCH_DONE_INT_CLR_W<SAR_COCPU_INT_CLR_SPEC> {
        COCPU_TOUCH_DONE_INT_CLR_W::new(self, 0)
    }
    #[doc = "Bit 1 - TOUCH_INACTIVE_INT interrupt clear bit"]
    #[inline(always)]
    pub fn cocpu_touch_inactive_int_clr(
        &mut self,
    ) -> COCPU_TOUCH_INACTIVE_INT_CLR_W<SAR_COCPU_INT_CLR_SPEC> {
        COCPU_TOUCH_INACTIVE_INT_CLR_W::new(self, 1)
    }
    #[doc = "Bit 2 - TOUCH_ACTIVE_INT interrupt clear bit"]
    #[inline(always)]
    pub fn cocpu_touch_active_int_clr(
        &mut self,
    ) -> COCPU_TOUCH_ACTIVE_INT_CLR_W<SAR_COCPU_INT_CLR_SPEC> {
        COCPU_TOUCH_ACTIVE_INT_CLR_W::new(self, 2)
    }
    #[doc = "Bit 3 - SARADC1_DONE_INT interrupt clear bit"]
    #[inline(always)]
    pub fn cocpu_saradc1_int_clr(&mut self) -> COCPU_SARADC1_INT_CLR_W<SAR_COCPU_INT_CLR_SPEC> {
        COCPU_SARADC1_INT_CLR_W::new(self, 3)
    }
    #[doc = "Bit 4 - SARADC2_DONE_INT interrupt clear bit"]
    #[inline(always)]
    pub fn cocpu_saradc2_int_clr(&mut self) -> COCPU_SARADC2_INT_CLR_W<SAR_COCPU_INT_CLR_SPEC> {
        COCPU_SARADC2_INT_CLR_W::new(self, 4)
    }
    #[doc = "Bit 5 - TSENS_DONE_INT interrupt clear bit"]
    #[inline(always)]
    pub fn cocpu_tsens_int_clr(&mut self) -> COCPU_TSENS_INT_CLR_W<SAR_COCPU_INT_CLR_SPEC> {
        COCPU_TSENS_INT_CLR_W::new(self, 5)
    }
    #[doc = "Bit 6 - RISCV_START_INT interrupt clear bit"]
    #[inline(always)]
    pub fn cocpu_start_int_clr(&mut self) -> COCPU_START_INT_CLR_W<SAR_COCPU_INT_CLR_SPEC> {
        COCPU_START_INT_CLR_W::new(self, 6)
    }
    #[doc = "Bit 7 - SW_INT interrupt clear bit"]
    #[inline(always)]
    pub fn cocpu_sw_int_clr(&mut self) -> COCPU_SW_INT_CLR_W<SAR_COCPU_INT_CLR_SPEC> {
        COCPU_SW_INT_CLR_W::new(self, 7)
    }
    #[doc = "Bit 8 - SWD_INT interrupt clear bit"]
    #[inline(always)]
    pub fn cocpu_swd_int_clr(&mut self) -> COCPU_SWD_INT_CLR_W<SAR_COCPU_INT_CLR_SPEC> {
        COCPU_SWD_INT_CLR_W::new(self, 8)
    }
}
#[doc = "Interrupt clear bit of ULP-RISCV\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar_cocpu_int_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SAR_COCPU_INT_CLR_SPEC;
impl crate::RegisterSpec for SAR_COCPU_INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`sar_cocpu_int_clr::W`](W) writer structure"]
impl crate::Writable for SAR_COCPU_INT_CLR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SAR_COCPU_INT_CLR to value 0"]
impl crate::Resettable for SAR_COCPU_INT_CLR_SPEC {
    const RESET_VALUE: u32 = 0;
}
