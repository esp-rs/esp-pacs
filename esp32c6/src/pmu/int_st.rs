#[doc = "Register `INT_ST` reader"]
pub type R = crate::R<INT_ST_SPEC>;
#[doc = "Field `LP_CPU_EXC` reader - need_des"]
pub type LP_CPU_EXC_R = crate::BitReader;
#[doc = "Field `SDIO_IDLE` reader - need_des"]
pub type SDIO_IDLE_R = crate::BitReader;
#[doc = "Field `SW` reader - need_des"]
pub type SW_R = crate::BitReader;
#[doc = "Field `SOC_SLEEP_REJECT` reader - need_des"]
pub type SOC_SLEEP_REJECT_R = crate::BitReader;
#[doc = "Field `SOC_WAKEUP` reader - need_des"]
pub type SOC_WAKEUP_R = crate::BitReader;
impl R {
    #[doc = "Bit 27 - need_des"]
    #[inline(always)]
    pub fn lp_cpu_exc(&self) -> LP_CPU_EXC_R {
        LP_CPU_EXC_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - need_des"]
    #[inline(always)]
    pub fn sdio_idle(&self) -> SDIO_IDLE_R {
        SDIO_IDLE_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - need_des"]
    #[inline(always)]
    pub fn sw(&self) -> SW_R {
        SW_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    pub fn soc_sleep_reject(&self) -> SOC_SLEEP_REJECT_R {
        SOC_SLEEP_REJECT_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn soc_wakeup(&self) -> SOC_WAKEUP_R {
        SOC_WAKEUP_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ST")
            .field("lp_cpu_exc", &format_args!("{}", self.lp_cpu_exc().bit()))
            .field("sdio_idle", &format_args!("{}", self.sdio_idle().bit()))
            .field("sw", &format_args!("{}", self.sw().bit()))
            .field(
                "soc_sleep_reject",
                &format_args!("{}", self.soc_sleep_reject().bit()),
            )
            .field("soc_wakeup", &format_args!("{}", self.soc_wakeup().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_ST_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_st::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_ST_SPEC;
impl crate::RegisterSpec for INT_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_st::R`](R) reader structure"]
impl crate::Readable for INT_ST_SPEC {}
#[doc = "`reset()` method sets INT_ST to value 0"]
impl crate::Resettable for INT_ST_SPEC {
    const RESET_VALUE: u32 = 0;
}
