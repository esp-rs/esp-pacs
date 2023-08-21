#[doc = "Register `SLC_TX_LINK` reader"]
pub type R = crate::R<SLC_TX_LINK_SPEC>;
#[doc = "Register `SLC_TX_LINK` writer"]
pub type W = crate::W<SLC_TX_LINK_SPEC>;
#[doc = "Field `SLC_TXLINK_ADDR` reader - "]
pub type SLC_TXLINK_ADDR_R = crate::FieldReader<u32>;
#[doc = "Field `SLC_TXLINK_ADDR` writer - "]
pub type SLC_TXLINK_ADDR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 20, O, u32>;
#[doc = "Field `SLC_TXLINK_STOP` reader - "]
pub type SLC_TXLINK_STOP_R = crate::BitReader;
#[doc = "Field `SLC_TXLINK_STOP` writer - "]
pub type SLC_TXLINK_STOP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SLC_TXLINK_START` reader - "]
pub type SLC_TXLINK_START_R = crate::BitReader;
#[doc = "Field `SLC_TXLINK_START` writer - "]
pub type SLC_TXLINK_START_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SLC_TXLINK_RESTART` reader - "]
pub type SLC_TXLINK_RESTART_R = crate::BitReader;
#[doc = "Field `SLC_TXLINK_RESTART` writer - "]
pub type SLC_TXLINK_RESTART_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SLC_TXLINK_PARK` reader - "]
pub type SLC_TXLINK_PARK_R = crate::BitReader;
#[doc = "Field `SLC_TXLINK_PARK` writer - "]
pub type SLC_TXLINK_PARK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:19"]
    #[inline(always)]
    pub fn slc_txlink_addr(&self) -> SLC_TXLINK_ADDR_R {
        SLC_TXLINK_ADDR_R::new(self.bits & 0x000f_ffff)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn slc_txlink_stop(&self) -> SLC_TXLINK_STOP_R {
        SLC_TXLINK_STOP_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn slc_txlink_start(&self) -> SLC_TXLINK_START_R {
        SLC_TXLINK_START_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn slc_txlink_restart(&self) -> SLC_TXLINK_RESTART_R {
        SLC_TXLINK_RESTART_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn slc_txlink_park(&self) -> SLC_TXLINK_PARK_R {
        SLC_TXLINK_PARK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SLC_TX_LINK")
            .field(
                "slc_txlink_park",
                &format_args!("{}", self.slc_txlink_park().bit()),
            )
            .field(
                "slc_txlink_restart",
                &format_args!("{}", self.slc_txlink_restart().bit()),
            )
            .field(
                "slc_txlink_start",
                &format_args!("{}", self.slc_txlink_start().bit()),
            )
            .field(
                "slc_txlink_stop",
                &format_args!("{}", self.slc_txlink_stop().bit()),
            )
            .field(
                "slc_txlink_addr",
                &format_args!("{}", self.slc_txlink_addr().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SLC_TX_LINK_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:19"]
    #[inline(always)]
    #[must_use]
    pub fn slc_txlink_addr(&mut self) -> SLC_TXLINK_ADDR_W<SLC_TX_LINK_SPEC, 0> {
        SLC_TXLINK_ADDR_W::new(self)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    #[must_use]
    pub fn slc_txlink_stop(&mut self) -> SLC_TXLINK_STOP_W<SLC_TX_LINK_SPEC, 28> {
        SLC_TXLINK_STOP_W::new(self)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    #[must_use]
    pub fn slc_txlink_start(&mut self) -> SLC_TXLINK_START_W<SLC_TX_LINK_SPEC, 29> {
        SLC_TXLINK_START_W::new(self)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    #[must_use]
    pub fn slc_txlink_restart(&mut self) -> SLC_TXLINK_RESTART_W<SLC_TX_LINK_SPEC, 30> {
        SLC_TXLINK_RESTART_W::new(self)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn slc_txlink_park(&mut self) -> SLC_TXLINK_PARK_W<SLC_TX_LINK_SPEC, 31> {
        SLC_TXLINK_PARK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "SLC_TX_LINK\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slc_tx_link::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slc_tx_link::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SLC_TX_LINK_SPEC;
impl crate::RegisterSpec for SLC_TX_LINK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`slc_tx_link::R`](R) reader structure"]
impl crate::Readable for SLC_TX_LINK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`slc_tx_link::W`](W) writer structure"]
impl crate::Writable for SLC_TX_LINK_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SLC_TX_LINK to value 0"]
impl crate::Resettable for SLC_TX_LINK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
