#[doc = "Register `EMACLPITIMERSCONTROL` reader"]
pub type R = crate::R<EMACLPITIMERSCONTROL_SPEC>;
#[doc = "Field `LPI_TW_TIMER` reader - This field specifies the minimum time (in microseconds) for which the MAC waits after it stops transmitting the LPI pattern to the PHY and before it resumes the normal transmission. The TLPIEX status bit is set after the expiry of this timer."]
pub type LPI_TW_TIMER_R = crate::FieldReader<u16>;
#[doc = "Field `LPI_LS_TIMER` reader - This field specifies the minimum time (in milliseconds) for which the link status from the PHY should be up (OKAY) before the LPI pattern can be transmitted to the PHY. The MAC does not transmit the LPI pattern even when the LPIEN bit is set unless the LPI_LS_Timer reaches the programmed terminal count. The default value of the LPI_LS_Timer is 1000 (1 sec) as defined in the IEEE standard."]
pub type LPI_LS_TIMER_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - This field specifies the minimum time (in microseconds) for which the MAC waits after it stops transmitting the LPI pattern to the PHY and before it resumes the normal transmission. The TLPIEX status bit is set after the expiry of this timer."]
    #[inline(always)]
    pub fn lpi_tw_timer(&self) -> LPI_TW_TIMER_R {
        LPI_TW_TIMER_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:25 - This field specifies the minimum time (in milliseconds) for which the link status from the PHY should be up (OKAY) before the LPI pattern can be transmitted to the PHY. The MAC does not transmit the LPI pattern even when the LPIEN bit is set unless the LPI_LS_Timer reaches the programmed terminal count. The default value of the LPI_LS_Timer is 1000 (1 sec) as defined in the IEEE standard."]
    #[inline(always)]
    pub fn lpi_ls_timer(&self) -> LPI_LS_TIMER_R {
        LPI_LS_TIMER_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EMACLPITIMERSCONTROL")
            .field(
                "lpi_tw_timer",
                &format_args!("{}", self.lpi_tw_timer().bits()),
            )
            .field(
                "lpi_ls_timer",
                &format_args!("{}", self.lpi_ls_timer().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<EMACLPITIMERSCONTROL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "LPI Timers Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emaclpitimerscontrol::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EMACLPITIMERSCONTROL_SPEC;
impl crate::RegisterSpec for EMACLPITIMERSCONTROL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`emaclpitimerscontrol::R`](R) reader structure"]
impl crate::Readable for EMACLPITIMERSCONTROL_SPEC {}
#[doc = "`reset()` method sets EMACLPITIMERSCONTROL to value 0"]
impl crate::Resettable for EMACLPITIMERSCONTROL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
