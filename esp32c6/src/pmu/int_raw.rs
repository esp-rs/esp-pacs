#[doc = "Register `INT_RAW` reader"]
pub struct R(crate::R<INT_RAW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_RAW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT_RAW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT_RAW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `LP_CPU_EXC_INT_RAW` reader - need_des"]
pub type LP_CPU_EXC_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `SDIO_IDLE_INT_RAW` reader - need_des"]
pub type SDIO_IDLE_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `SW_INT_RAW` reader - need_des"]
pub type SW_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `SOC_SLEEP_REJECT_INT_RAW` reader - need_des"]
pub type SOC_SLEEP_REJECT_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `SOC_WAKEUP_INT_RAW` reader - need_des"]
pub type SOC_WAKEUP_INT_RAW_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 27 - need_des"]
    #[inline(always)]
    pub fn lp_cpu_exc_int_raw(&self) -> LP_CPU_EXC_INT_RAW_R {
        LP_CPU_EXC_INT_RAW_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - need_des"]
    #[inline(always)]
    pub fn sdio_idle_int_raw(&self) -> SDIO_IDLE_INT_RAW_R {
        SDIO_IDLE_INT_RAW_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - need_des"]
    #[inline(always)]
    pub fn sw_int_raw(&self) -> SW_INT_RAW_R {
        SW_INT_RAW_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    pub fn soc_sleep_reject_int_raw(&self) -> SOC_SLEEP_REJECT_INT_RAW_R {
        SOC_SLEEP_REJECT_INT_RAW_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn soc_wakeup_int_raw(&self) -> SOC_WAKEUP_INT_RAW_R {
        SOC_WAKEUP_INT_RAW_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "need_des\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_raw](index.html) module"]
pub struct INT_RAW_SPEC;
impl crate::RegisterSpec for INT_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int_raw::R](R) reader structure"]
impl crate::Readable for INT_RAW_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INT_RAW to value 0"]
impl crate::Resettable for INT_RAW_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
