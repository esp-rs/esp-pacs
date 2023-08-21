#[doc = "Register `SLC_RX_STATUS` reader"]
pub type R = crate::R<SLC_RX_STATUS_SPEC>;
#[doc = "Register `SLC_RX_STATUS` writer"]
pub type W = crate::W<SLC_RX_STATUS_SPEC>;
#[doc = "Field `SLC_RX_FULL` reader - "]
pub type SLC_RX_FULL_R = crate::BitReader;
#[doc = "Field `SLC_RX_FULL` writer - "]
pub type SLC_RX_FULL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SLC_RX_EMPTY` reader - "]
pub type SLC_RX_EMPTY_R = crate::BitReader;
#[doc = "Field `SLC_RX_EMPTY` writer - "]
pub type SLC_RX_EMPTY_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn slc_rx_full(&self) -> SLC_RX_FULL_R {
        SLC_RX_FULL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn slc_rx_empty(&self) -> SLC_RX_EMPTY_R {
        SLC_RX_EMPTY_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SLC_RX_STATUS")
            .field(
                "slc_rx_empty",
                &format_args!("{}", self.slc_rx_empty().bit()),
            )
            .field("slc_rx_full", &format_args!("{}", self.slc_rx_full().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SLC_RX_STATUS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn slc_rx_full(&mut self) -> SLC_RX_FULL_W<SLC_RX_STATUS_SPEC, 0> {
        SLC_RX_FULL_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn slc_rx_empty(&mut self) -> SLC_RX_EMPTY_W<SLC_RX_STATUS_SPEC, 1> {
        SLC_RX_EMPTY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "SLC_RX_STATUS\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slc_rx_status::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slc_rx_status::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SLC_RX_STATUS_SPEC;
impl crate::RegisterSpec for SLC_RX_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`slc_rx_status::R`](R) reader structure"]
impl crate::Readable for SLC_RX_STATUS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`slc_rx_status::W`](W) writer structure"]
impl crate::Writable for SLC_RX_STATUS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SLC_RX_STATUS to value 0"]
impl crate::Resettable for SLC_RX_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
