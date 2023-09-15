#[doc = "Register `EMACCSTATUS` reader"]
pub type R = crate::R<EMACCSTATUS_SPEC>;
#[doc = "Field `LINK_MODE` reader - This bit indicates the current mode of operation of the link: 1'b0: Half-duplex mode. 1'b1: Full-duplex mode."]
pub type LINK_MODE_R = crate::BitReader;
#[doc = "Field `LINK_SPEED` reader - This bit indicates the current speed of the link: 2'b00: 2.5 MHz. 2'b01: 25 MHz. 2'b10: 125 MHz."]
pub type LINK_SPEED_R = crate::FieldReader;
#[doc = "Field `JABBER_TIMEOUT` reader - This bit indicates whether there is jabber timeout error (1'b1) in the received Frame."]
pub type JABBER_TIMEOUT_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - This bit indicates the current mode of operation of the link: 1'b0: Half-duplex mode. 1'b1: Full-duplex mode."]
    #[inline(always)]
    pub fn link_mode(&self) -> LINK_MODE_R {
        LINK_MODE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - This bit indicates the current speed of the link: 2'b00: 2.5 MHz. 2'b01: 25 MHz. 2'b10: 125 MHz."]
    #[inline(always)]
    pub fn link_speed(&self) -> LINK_SPEED_R {
        LINK_SPEED_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 4 - This bit indicates whether there is jabber timeout error (1'b1) in the received Frame."]
    #[inline(always)]
    pub fn jabber_timeout(&self) -> JABBER_TIMEOUT_R {
        JABBER_TIMEOUT_R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EMACCSTATUS")
            .field("link_mode", &format_args!("{}", self.link_mode().bit()))
            .field("link_speed", &format_args!("{}", self.link_speed().bits()))
            .field(
                "jabber_timeout",
                &format_args!("{}", self.jabber_timeout().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<EMACCSTATUS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Link communication status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emaccstatus::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EMACCSTATUS_SPEC;
impl crate::RegisterSpec for EMACCSTATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`emaccstatus::R`](R) reader structure"]
impl crate::Readable for EMACCSTATUS_SPEC {}
#[doc = "`reset()` method sets EMACCSTATUS to value 0"]
impl crate::Resettable for EMACCSTATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
