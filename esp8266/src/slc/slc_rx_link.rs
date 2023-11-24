#[doc = "Register `SLC_RX_LINK` reader"]
pub type R = crate::R<SLC_RX_LINK_SPEC>;
#[doc = "Register `SLC_RX_LINK` writer"]
pub type W = crate::W<SLC_RX_LINK_SPEC>;
#[doc = "Field `SLC_RXLINK_ADDR` reader - "]
pub type SLC_RXLINK_ADDR_R = crate::FieldReader<u32>;
#[doc = "Field `SLC_RXLINK_ADDR` writer - "]
pub type SLC_RXLINK_ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
#[doc = "Field `SLC_RXLINK_STOP` reader - "]
pub type SLC_RXLINK_STOP_R = crate::BitReader;
#[doc = "Field `SLC_RXLINK_STOP` writer - "]
pub type SLC_RXLINK_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLC_RXLINK_START` reader - "]
pub type SLC_RXLINK_START_R = crate::BitReader;
#[doc = "Field `SLC_RXLINK_START` writer - "]
pub type SLC_RXLINK_START_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLC_RXLINK_RESTART` reader - "]
pub type SLC_RXLINK_RESTART_R = crate::BitReader;
#[doc = "Field `SLC_RXLINK_RESTART` writer - "]
pub type SLC_RXLINK_RESTART_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLC_RXLINK_PARK` reader - "]
pub type SLC_RXLINK_PARK_R = crate::BitReader;
#[doc = "Field `SLC_RXLINK_PARK` writer - "]
pub type SLC_RXLINK_PARK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:19"]
    #[inline(always)]
    pub fn slc_rxlink_addr(&self) -> SLC_RXLINK_ADDR_R {
        SLC_RXLINK_ADDR_R::new(self.bits & 0x000f_ffff)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn slc_rxlink_stop(&self) -> SLC_RXLINK_STOP_R {
        SLC_RXLINK_STOP_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn slc_rxlink_start(&self) -> SLC_RXLINK_START_R {
        SLC_RXLINK_START_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn slc_rxlink_restart(&self) -> SLC_RXLINK_RESTART_R {
        SLC_RXLINK_RESTART_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn slc_rxlink_park(&self) -> SLC_RXLINK_PARK_R {
        SLC_RXLINK_PARK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SLC_RX_LINK")
            .field(
                "slc_rxlink_park",
                &format_args!("{}", self.slc_rxlink_park().bit()),
            )
            .field(
                "slc_rxlink_restart",
                &format_args!("{}", self.slc_rxlink_restart().bit()),
            )
            .field(
                "slc_rxlink_start",
                &format_args!("{}", self.slc_rxlink_start().bit()),
            )
            .field(
                "slc_rxlink_stop",
                &format_args!("{}", self.slc_rxlink_stop().bit()),
            )
            .field(
                "slc_rxlink_addr",
                &format_args!("{}", self.slc_rxlink_addr().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SLC_RX_LINK_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:19"]
    #[inline(always)]
    #[must_use]
    pub fn slc_rxlink_addr(&mut self) -> SLC_RXLINK_ADDR_W<SLC_RX_LINK_SPEC> {
        SLC_RXLINK_ADDR_W::new(self, 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    #[must_use]
    pub fn slc_rxlink_stop(&mut self) -> SLC_RXLINK_STOP_W<SLC_RX_LINK_SPEC> {
        SLC_RXLINK_STOP_W::new(self, 28)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    #[must_use]
    pub fn slc_rxlink_start(&mut self) -> SLC_RXLINK_START_W<SLC_RX_LINK_SPEC> {
        SLC_RXLINK_START_W::new(self, 29)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    #[must_use]
    pub fn slc_rxlink_restart(&mut self) -> SLC_RXLINK_RESTART_W<SLC_RX_LINK_SPEC> {
        SLC_RXLINK_RESTART_W::new(self, 30)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn slc_rxlink_park(&mut self) -> SLC_RXLINK_PARK_W<SLC_RX_LINK_SPEC> {
        SLC_RXLINK_PARK_W::new(self, 31)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "SLC_RX_LINK\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slc_rx_link::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slc_rx_link::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SLC_RX_LINK_SPEC;
impl crate::RegisterSpec for SLC_RX_LINK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`slc_rx_link::R`](R) reader structure"]
impl crate::Readable for SLC_RX_LINK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`slc_rx_link::W`](W) writer structure"]
impl crate::Writable for SLC_RX_LINK_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SLC_RX_LINK to value 0"]
impl crate::Resettable for SLC_RX_LINK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
