#[doc = "Register `INT_RAW` reader"]
pub type R = crate::R<INT_RAW_SPEC>;
#[doc = "Register `INT_RAW` writer"]
pub type W = crate::W<INT_RAW_SPEC>;
#[doc = "Field `LP_CPU_EXC_INT_RAW` reader - need_des"]
pub type LP_CPU_EXC_INT_RAW_R = crate::BitReader;
#[doc = "Field `LP_CPU_EXC_INT_RAW` writer - need_des"]
pub type LP_CPU_EXC_INT_RAW_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SDIO_IDLE_INT_RAW` reader - need_des"]
pub type SDIO_IDLE_INT_RAW_R = crate::BitReader;
#[doc = "Field `SDIO_IDLE_INT_RAW` writer - need_des"]
pub type SDIO_IDLE_INT_RAW_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SW_INT_RAW` reader - need_des"]
pub type SW_INT_RAW_R = crate::BitReader;
#[doc = "Field `SW_INT_RAW` writer - need_des"]
pub type SW_INT_RAW_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SOC_SLEEP_REJECT_INT_RAW` reader - need_des"]
pub type SOC_SLEEP_REJECT_INT_RAW_R = crate::BitReader;
#[doc = "Field `SOC_SLEEP_REJECT_INT_RAW` writer - need_des"]
pub type SOC_SLEEP_REJECT_INT_RAW_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SOC_WAKEUP_INT_RAW` reader - need_des"]
pub type SOC_WAKEUP_INT_RAW_R = crate::BitReader;
#[doc = "Field `SOC_WAKEUP_INT_RAW` writer - need_des"]
pub type SOC_WAKEUP_INT_RAW_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
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
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_RAW")
            .field(
                "lp_cpu_exc_int_raw",
                &format_args!("{}", self.lp_cpu_exc_int_raw().bit()),
            )
            .field(
                "sdio_idle_int_raw",
                &format_args!("{}", self.sdio_idle_int_raw().bit()),
            )
            .field("sw_int_raw", &format_args!("{}", self.sw_int_raw().bit()))
            .field(
                "soc_sleep_reject_int_raw",
                &format_args!("{}", self.soc_sleep_reject_int_raw().bit()),
            )
            .field(
                "soc_wakeup_int_raw",
                &format_args!("{}", self.soc_wakeup_int_raw().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_RAW_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 27 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_cpu_exc_int_raw(&mut self) -> LP_CPU_EXC_INT_RAW_W<INT_RAW_SPEC, 27> {
        LP_CPU_EXC_INT_RAW_W::new(self)
    }
    #[doc = "Bit 28 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn sdio_idle_int_raw(&mut self) -> SDIO_IDLE_INT_RAW_W<INT_RAW_SPEC, 28> {
        SDIO_IDLE_INT_RAW_W::new(self)
    }
    #[doc = "Bit 29 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn sw_int_raw(&mut self) -> SW_INT_RAW_W<INT_RAW_SPEC, 29> {
        SW_INT_RAW_W::new(self)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn soc_sleep_reject_int_raw(&mut self) -> SOC_SLEEP_REJECT_INT_RAW_W<INT_RAW_SPEC, 30> {
        SOC_SLEEP_REJECT_INT_RAW_W::new(self)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn soc_wakeup_int_raw(&mut self) -> SOC_WAKEUP_INT_RAW_W<INT_RAW_SPEC, 31> {
        SOC_WAKEUP_INT_RAW_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_raw::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_raw::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_RAW_SPEC;
impl crate::RegisterSpec for INT_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_raw::R`](R) reader structure"]
impl crate::Readable for INT_RAW_SPEC {}
#[doc = "`write(|w| ..)` method takes [`int_raw::W`](W) writer structure"]
impl crate::Writable for INT_RAW_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INT_RAW to value 0"]
impl crate::Resettable for INT_RAW_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
