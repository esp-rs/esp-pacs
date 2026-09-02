#[doc = "Register `LPITIMERSCONTROL` reader"]
pub type R = crate::R<LPITIMERSCONTROL_SPEC>;
#[doc = "Field `TWT` reader - LPI TW TIMER This field specifies the minimum time _in microseconds_ for which the MAC waits after it stops transmitting the LPI pattern to the PHY and before it resumes the normal transmission The TLPIEX status bit is set after the expiry of this timer"]
pub type TWT_R = crate::FieldReader<u16>;
#[doc = "Field `LST` reader - LPI LS TIMER This field specifies the minimum time _in milliseconds_ for which the link status from the PHY should be up _OKAY_ before the LPI pattern can be transmitted to the PHY The MAC does not transmit the LPI pattern even when the LPIEN bit is set unless the LPI LS Timer reaches the programmed terminal count The default value of the LPI LS Timer is 1000 _1 sec_ as defined in the IEEE standard"]
pub type LST_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - LPI TW TIMER This field specifies the minimum time _in microseconds_ for which the MAC waits after it stops transmitting the LPI pattern to the PHY and before it resumes the normal transmission The TLPIEX status bit is set after the expiry of this timer"]
    #[inline(always)]
    pub fn twt(&self) -> TWT_R {
        TWT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:25 - LPI LS TIMER This field specifies the minimum time _in milliseconds_ for which the link status from the PHY should be up _OKAY_ before the LPI pattern can be transmitted to the PHY The MAC does not transmit the LPI pattern even when the LPIEN bit is set unless the LPI LS Timer reaches the programmed terminal count The default value of the LPI LS Timer is 1000 _1 sec_ as defined in the IEEE standard"]
    #[inline(always)]
    pub fn lst(&self) -> LST_R {
        LST_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPITIMERSCONTROL")
            .field("twt", &self.twt())
            .field("lst", &self.lst())
            .finish()
    }
}
#[doc = "LPI Timers Control\n\nYou can [`read`](crate::Reg::read) this register and get [`lpitimerscontrol::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LPITIMERSCONTROL_SPEC;
impl crate::RegisterSpec for LPITIMERSCONTROL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lpitimerscontrol::R`](R) reader structure"]
impl crate::Readable for LPITIMERSCONTROL_SPEC {}
#[doc = "`reset()` method sets LPITIMERSCONTROL to value 0x03e8_0000"]
impl crate::Resettable for LPITIMERSCONTROL_SPEC {
    const RESET_VALUE: u32 = 0x03e8_0000;
}
