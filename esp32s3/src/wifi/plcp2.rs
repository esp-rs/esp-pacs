#[doc = "Register `PLCP2%s` reader"]
pub type R = crate::R<PLCP2_SPEC>;
#[doc = "Register `PLCP2%s` writer"]
pub type W = crate::W<PLCP2_SPEC>;
#[doc = "Field `UNKNOWN` reader - meaning unknown, set to one for TX"]
pub type UNKNOWN_R = crate::BitReader;
#[doc = "Field `UNKNOWN` writer - meaning unknown, set to one for TX"]
pub type UNKNOWN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESPONSE_RATE` reader - Expected response PHY rate, written by mac_tx_set_plcp1"]
pub type RESPONSE_RATE_R = crate::FieldReader;
#[doc = "Field `RESPONSE_RATE` writer - Expected response PHY rate, written by mac_tx_set_plcp1"]
pub type RESPONSE_RATE_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INTERFACE_ID` reader - Virtual interface for this transmit slot"]
pub type INTERFACE_ID_R = crate::FieldReader;
#[doc = "Field `INTERFACE_ID` writer - Virtual interface for this transmit slot"]
pub type INTERFACE_ID_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 5 - meaning unknown, set to one for TX"]
    #[inline(always)]
    pub fn unknown(&self) -> UNKNOWN_R {
        UNKNOWN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:13 - Expected response PHY rate, written by mac_tx_set_plcp1"]
    #[inline(always)]
    pub fn response_rate(&self) -> RESPONSE_RATE_R {
        RESPONSE_RATE_R::new(((self.bits >> 6) & 0xff) as u8)
    }
    #[doc = "Bits 28:29 - Virtual interface for this transmit slot"]
    #[inline(always)]
    pub fn interface_id(&self) -> INTERFACE_ID_R {
        INTERFACE_ID_R::new(((self.bits >> 28) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PLCP2")
            .field("unknown", &self.unknown())
            .field("response_rate", &self.response_rate())
            .field("interface_id", &self.interface_id())
            .finish()
    }
}
impl W {
    #[doc = "Bit 5 - meaning unknown, set to one for TX"]
    #[inline(always)]
    pub fn unknown(&mut self) -> UNKNOWN_W<'_, PLCP2_SPEC> {
        UNKNOWN_W::new(self, 5)
    }
    #[doc = "Bits 6:13 - Expected response PHY rate, written by mac_tx_set_plcp1"]
    #[inline(always)]
    pub fn response_rate(&mut self) -> RESPONSE_RATE_W<'_, PLCP2_SPEC> {
        RESPONSE_RATE_W::new(self, 6)
    }
    #[doc = "Bits 28:29 - Virtual interface for this transmit slot"]
    #[inline(always)]
    pub fn interface_id(&mut self) -> INTERFACE_ID_W<'_, PLCP2_SPEC> {
        INTERFACE_ID_W::new(self, 28)
    }
}
#[doc = "PLCP2\n\nYou can [`read`](crate::Reg::read) this register and get [`plcp2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`plcp2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PLCP2_SPEC;
impl crate::RegisterSpec for PLCP2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`plcp2::R`](R) reader structure"]
impl crate::Readable for PLCP2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`plcp2::W`](W) writer structure"]
impl crate::Writable for PLCP2_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PLCP2%s to value 0"]
impl crate::Resettable for PLCP2_SPEC {}
