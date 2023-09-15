#[doc = "Register `EMACLPI_CRS` reader"]
pub type R = crate::R<EMACLPI_CRS_SPEC>;
#[doc = "Field `TLPIEN` reader - When set this bit indicates that the MAC Transmitter has entered the LPI state because of the setting of the LPIEN bit. This bit is cleared by a read into this register."]
pub type TLPIEN_R = crate::BitReader;
#[doc = "Field `TLPIEX` reader - When set this bit indicates that the MAC transmitter has exited the LPI state after the user has cleared the LPIEN bit and the LPI_TW_Timer has expired.This bit is cleared by a read into this register."]
pub type TLPIEX_R = crate::BitReader;
#[doc = "Field `RLPIEN` reader - When set this bit indicates that the MAC Receiver has received an LPI pattern and entered the LPI state. This bit is cleared by a read into this register."]
pub type RLPIEN_R = crate::BitReader;
#[doc = "Field `RLPIEX` reader - When set this bit indicates that the MAC Receiver has stopped receiving the LPI pattern on the MII interface exited the LPI state and resumed the normal reception. This bit is cleared by a read into this register."]
pub type RLPIEX_R = crate::BitReader;
#[doc = "Field `TLPIST` reader - When set this bit indicates that the MAC is transmitting the LPI pattern on the MII interface."]
pub type TLPIST_R = crate::BitReader;
#[doc = "Field `RLPIST` reader - When set this bit indicates that the MAC is receiving the LPI pattern on the MII interface."]
pub type RLPIST_R = crate::BitReader;
#[doc = "Field `LPIEN` reader - When set this bit instructs the MAC Transmitter to enter the LPI state. When reset this bit instructs the MAC to exit the LPI state and resume normal transmission.This bit is cleared when the LPITXA bit is set and the MAC exits the LPI state because of the arrival of a new packet for transmission."]
pub type LPIEN_R = crate::BitReader;
#[doc = "Field `PLS` reader - This bit indicates the link status of the PHY.When set the link is considered to be okay (up) and when reset the link is considered to be down."]
pub type PLS_R = crate::BitReader;
#[doc = "Field `LPITXA` reader - This bit controls the behavior of the MAC when it is entering or coming out of the LPI mode on the transmit side.If the LPITXA and LPIEN bits are set to 1 the MAC enters the LPI mode only after all outstanding frames and pending frames have been transmitted. The MAC comes out of the LPI mode when the application sends any frame.When this bit is 0 the LPIEN bit directly controls behavior of the MAC when it is entering or coming out of the LPI mode."]
pub type LPITXA_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - When set this bit indicates that the MAC Transmitter has entered the LPI state because of the setting of the LPIEN bit. This bit is cleared by a read into this register."]
    #[inline(always)]
    pub fn tlpien(&self) -> TLPIEN_R {
        TLPIEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - When set this bit indicates that the MAC transmitter has exited the LPI state after the user has cleared the LPIEN bit and the LPI_TW_Timer has expired.This bit is cleared by a read into this register."]
    #[inline(always)]
    pub fn tlpiex(&self) -> TLPIEX_R {
        TLPIEX_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - When set this bit indicates that the MAC Receiver has received an LPI pattern and entered the LPI state. This bit is cleared by a read into this register."]
    #[inline(always)]
    pub fn rlpien(&self) -> RLPIEN_R {
        RLPIEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - When set this bit indicates that the MAC Receiver has stopped receiving the LPI pattern on the MII interface exited the LPI state and resumed the normal reception. This bit is cleared by a read into this register."]
    #[inline(always)]
    pub fn rlpiex(&self) -> RLPIEX_R {
        RLPIEX_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - When set this bit indicates that the MAC is transmitting the LPI pattern on the MII interface."]
    #[inline(always)]
    pub fn tlpist(&self) -> TLPIST_R {
        TLPIST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - When set this bit indicates that the MAC is receiving the LPI pattern on the MII interface."]
    #[inline(always)]
    pub fn rlpist(&self) -> RLPIST_R {
        RLPIST_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 16 - When set this bit instructs the MAC Transmitter to enter the LPI state. When reset this bit instructs the MAC to exit the LPI state and resume normal transmission.This bit is cleared when the LPITXA bit is set and the MAC exits the LPI state because of the arrival of a new packet for transmission."]
    #[inline(always)]
    pub fn lpien(&self) -> LPIEN_R {
        LPIEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - This bit indicates the link status of the PHY.When set the link is considered to be okay (up) and when reset the link is considered to be down."]
    #[inline(always)]
    pub fn pls(&self) -> PLS_R {
        PLS_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 19 - This bit controls the behavior of the MAC when it is entering or coming out of the LPI mode on the transmit side.If the LPITXA and LPIEN bits are set to 1 the MAC enters the LPI mode only after all outstanding frames and pending frames have been transmitted. The MAC comes out of the LPI mode when the application sends any frame.When this bit is 0 the LPIEN bit directly controls behavior of the MAC when it is entering or coming out of the LPI mode."]
    #[inline(always)]
    pub fn lpitxa(&self) -> LPITXA_R {
        LPITXA_R::new(((self.bits >> 19) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EMACLPI_CRS")
            .field("tlpien", &format_args!("{}", self.tlpien().bit()))
            .field("tlpiex", &format_args!("{}", self.tlpiex().bit()))
            .field("rlpien", &format_args!("{}", self.rlpien().bit()))
            .field("rlpiex", &format_args!("{}", self.rlpiex().bit()))
            .field("tlpist", &format_args!("{}", self.tlpist().bit()))
            .field("rlpist", &format_args!("{}", self.rlpist().bit()))
            .field("lpien", &format_args!("{}", self.lpien().bit()))
            .field("pls", &format_args!("{}", self.pls().bit()))
            .field("lpitxa", &format_args!("{}", self.lpitxa().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<EMACLPI_CRS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "LPI Control and Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emaclpi_crs::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EMACLPI_CRS_SPEC;
impl crate::RegisterSpec for EMACLPI_CRS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`emaclpi_crs::R`](R) reader structure"]
impl crate::Readable for EMACLPI_CRS_SPEC {}
#[doc = "`reset()` method sets EMACLPI_CRS to value 0"]
impl crate::Resettable for EMACLPI_CRS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
