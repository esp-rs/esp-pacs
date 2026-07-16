#[doc = "Register `REGISTER13_LPITIMERSCONTROLREGISTER` reader"]
pub type R = crate::R<REGISTER13_LPITIMERSCONTROLREGISTER_SPEC>;
#[doc = "Register `REGISTER13_LPITIMERSCONTROLREGISTER` writer"]
pub type W = crate::W<REGISTER13_LPITIMERSCONTROLREGISTER_SPEC>;
#[doc = "Field `TWT` reader - LPI TW TIMER This field specifies the minimum time _in microseconds_ for which the MAC waits after it stops transmitting the LPI pattern to the PHY and before it resumes the normal transmission The TLPIEX status bit is set after the expiry of this timer"]
pub type TWT_R = crate::FieldReader<u16>;
#[doc = "Field `TWT` writer - LPI TW TIMER This field specifies the minimum time _in microseconds_ for which the MAC waits after it stops transmitting the LPI pattern to the PHY and before it resumes the normal transmission The TLPIEX status bit is set after the expiry of this timer"]
pub type TWT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `LST` reader - LPI LS TIMER This field specifies the minimum time _in milliseconds_ for which the link status from the PHY should be up _OKAY_ before the LPI pattern can be transmitted to the PHY The MAC does not transmit the LPI pattern even when the LPIEN bit is set unless the LPI LS Timer reaches the programmed terminal count The default value of the LPI LS Timer is 1000 _1 sec_ as defined in the IEEE standard"]
pub type LST_R = crate::FieldReader<u16>;
#[doc = "Field `LST` writer - LPI LS TIMER This field specifies the minimum time _in milliseconds_ for which the link status from the PHY should be up _OKAY_ before the LPI pattern can be transmitted to the PHY The MAC does not transmit the LPI pattern even when the LPIEN bit is set unless the LPI LS Timer reaches the programmed terminal count The default value of the LPI LS Timer is 1000 _1 sec_ as defined in the IEEE standard"]
pub type LST_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
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
        f.debug_struct("REGISTER13_LPITIMERSCONTROLREGISTER")
            .field("twt", &self.twt())
            .field("lst", &self.lst())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - LPI TW TIMER This field specifies the minimum time _in microseconds_ for which the MAC waits after it stops transmitting the LPI pattern to the PHY and before it resumes the normal transmission The TLPIEX status bit is set after the expiry of this timer"]
    #[inline(always)]
    pub fn twt(&mut self) -> TWT_W<'_, REGISTER13_LPITIMERSCONTROLREGISTER_SPEC> {
        TWT_W::new(self, 0)
    }
    #[doc = "Bits 16:25 - LPI LS TIMER This field specifies the minimum time _in milliseconds_ for which the link status from the PHY should be up _OKAY_ before the LPI pattern can be transmitted to the PHY The MAC does not transmit the LPI pattern even when the LPIEN bit is set unless the LPI LS Timer reaches the programmed terminal count The default value of the LPI LS Timer is 1000 _1 sec_ as defined in the IEEE standard"]
    #[inline(always)]
    pub fn lst(&mut self) -> LST_W<'_, REGISTER13_LPITIMERSCONTROLREGISTER_SPEC> {
        LST_W::new(self, 16)
    }
}
#[doc = "Controls the timeout values in LPI states This register is present only when you select the Energy Efficient Ethernet feature in coreConsultant\n\nYou can [`read`](crate::Reg::read) this register and get [`register13_lpitimerscontrolregister::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register13_lpitimerscontrolregister::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REGISTER13_LPITIMERSCONTROLREGISTER_SPEC;
impl crate::RegisterSpec for REGISTER13_LPITIMERSCONTROLREGISTER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`register13_lpitimerscontrolregister::R`](R) reader structure"]
impl crate::Readable for REGISTER13_LPITIMERSCONTROLREGISTER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`register13_lpitimerscontrolregister::W`](W) writer structure"]
impl crate::Writable for REGISTER13_LPITIMERSCONTROLREGISTER_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REGISTER13_LPITIMERSCONTROLREGISTER to value 0x03e8_0000"]
impl crate::Resettable for REGISTER13_LPITIMERSCONTROLREGISTER_SPEC {
    const RESET_VALUE: u32 = 0x03e8_0000;
}
