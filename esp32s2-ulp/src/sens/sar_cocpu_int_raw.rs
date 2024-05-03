#[doc = "Register `SAR_COCPU_INT_RAW` reader"]
pub type R = crate::R<SAR_COCPU_INT_RAW_SPEC>;
#[doc = "Field `COCPU_TOUCH_DONE_INT_RAW` reader - TOUCH_DONE_INT interrupt raw bit"]
pub type COCPU_TOUCH_DONE_INT_RAW_R = crate::BitReader;
#[doc = "Field `COCPU_TOUCH_INACTIVE_INT_RAW` reader - TOUCH_INACTIVE_INT interrupt raw bit"]
pub type COCPU_TOUCH_INACTIVE_INT_RAW_R = crate::BitReader;
#[doc = "Field `COCPU_TOUCH_ACTIVE_INT_RAW` reader - TOUCH_ACTIVE_INT interrupt raw bit"]
pub type COCPU_TOUCH_ACTIVE_INT_RAW_R = crate::BitReader;
#[doc = "Field `COCPU_SARADC1_INT_RAW` reader - SARADC1_DONE_INT interrupt raw bit"]
pub type COCPU_SARADC1_INT_RAW_R = crate::BitReader;
#[doc = "Field `COCPU_SARADC2_INT_RAW` reader - SARADC2_DONE_INT interrupt raw bit"]
pub type COCPU_SARADC2_INT_RAW_R = crate::BitReader;
#[doc = "Field `COCPU_TSENS_INT_RAW` reader - TSENS_DONE_INT interrupt raw bit"]
pub type COCPU_TSENS_INT_RAW_R = crate::BitReader;
#[doc = "Field `COCPU_START_INT_RAW` reader - RISCV_START_INT interrupt raw bit"]
pub type COCPU_START_INT_RAW_R = crate::BitReader;
#[doc = "Field `COCPU_SW_INT_RAW` reader - SW_INT interrupt raw bit"]
pub type COCPU_SW_INT_RAW_R = crate::BitReader;
#[doc = "Field `COCPU_SWD_INT_RAW` reader - SWD_INT interrupt raw bit"]
pub type COCPU_SWD_INT_RAW_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - TOUCH_DONE_INT interrupt raw bit"]
    #[inline(always)]
    pub fn cocpu_touch_done_int_raw(&self) -> COCPU_TOUCH_DONE_INT_RAW_R {
        COCPU_TOUCH_DONE_INT_RAW_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TOUCH_INACTIVE_INT interrupt raw bit"]
    #[inline(always)]
    pub fn cocpu_touch_inactive_int_raw(&self) -> COCPU_TOUCH_INACTIVE_INT_RAW_R {
        COCPU_TOUCH_INACTIVE_INT_RAW_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TOUCH_ACTIVE_INT interrupt raw bit"]
    #[inline(always)]
    pub fn cocpu_touch_active_int_raw(&self) -> COCPU_TOUCH_ACTIVE_INT_RAW_R {
        COCPU_TOUCH_ACTIVE_INT_RAW_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SARADC1_DONE_INT interrupt raw bit"]
    #[inline(always)]
    pub fn cocpu_saradc1_int_raw(&self) -> COCPU_SARADC1_INT_RAW_R {
        COCPU_SARADC1_INT_RAW_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SARADC2_DONE_INT interrupt raw bit"]
    #[inline(always)]
    pub fn cocpu_saradc2_int_raw(&self) -> COCPU_SARADC2_INT_RAW_R {
        COCPU_SARADC2_INT_RAW_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TSENS_DONE_INT interrupt raw bit"]
    #[inline(always)]
    pub fn cocpu_tsens_int_raw(&self) -> COCPU_TSENS_INT_RAW_R {
        COCPU_TSENS_INT_RAW_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RISCV_START_INT interrupt raw bit"]
    #[inline(always)]
    pub fn cocpu_start_int_raw(&self) -> COCPU_START_INT_RAW_R {
        COCPU_START_INT_RAW_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SW_INT interrupt raw bit"]
    #[inline(always)]
    pub fn cocpu_sw_int_raw(&self) -> COCPU_SW_INT_RAW_R {
        COCPU_SW_INT_RAW_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - SWD_INT interrupt raw bit"]
    #[inline(always)]
    pub fn cocpu_swd_int_raw(&self) -> COCPU_SWD_INT_RAW_R {
        COCPU_SWD_INT_RAW_R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAR_COCPU_INT_RAW")
            .field(
                "cocpu_touch_done_int_raw",
                &self.cocpu_touch_done_int_raw().bit(),
            )
            .field(
                "cocpu_touch_inactive_int_raw",
                &self.cocpu_touch_inactive_int_raw().bit(),
            )
            .field(
                "cocpu_touch_active_int_raw",
                &self.cocpu_touch_active_int_raw().bit(),
            )
            .field("cocpu_saradc1_int_raw", &self.cocpu_saradc1_int_raw().bit())
            .field("cocpu_saradc2_int_raw", &self.cocpu_saradc2_int_raw().bit())
            .field("cocpu_tsens_int_raw", &self.cocpu_tsens_int_raw().bit())
            .field("cocpu_start_int_raw", &self.cocpu_start_int_raw().bit())
            .field("cocpu_sw_int_raw", &self.cocpu_sw_int_raw().bit())
            .field("cocpu_swd_int_raw", &self.cocpu_swd_int_raw().bit())
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SAR_COCPU_INT_RAW_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Interrupt raw bit of ULP-RISCV\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sar_cocpu_int_raw::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SAR_COCPU_INT_RAW_SPEC;
impl crate::RegisterSpec for SAR_COCPU_INT_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sar_cocpu_int_raw::R`](R) reader structure"]
impl crate::Readable for SAR_COCPU_INT_RAW_SPEC {}
#[doc = "`reset()` method sets SAR_COCPU_INT_RAW to value 0"]
impl crate::Resettable for SAR_COCPU_INT_RAW_SPEC {
    const RESET_VALUE: u32 = 0;
}
