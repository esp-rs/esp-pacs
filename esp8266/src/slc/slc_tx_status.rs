#[doc = "Register `SLC_TX_STATUS` reader"]
pub type R = crate::R<SLC_TX_STATUS_SPEC>;
#[doc = "Register `SLC_TX_STATUS` writer"]
pub type W = crate::W<SLC_TX_STATUS_SPEC>;
#[doc = "Field `SLC_TX_FULL` reader - "]
pub type SLC_TX_FULL_R = crate::BitReader;
#[doc = "Field `SLC_TX_FULL` writer - "]
pub type SLC_TX_FULL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLC_TX_EMPTY` reader - "]
pub type SLC_TX_EMPTY_R = crate::BitReader;
#[doc = "Field `SLC_TX_EMPTY` writer - "]
pub type SLC_TX_EMPTY_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn slc_tx_full(&self) -> SLC_TX_FULL_R {
        SLC_TX_FULL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn slc_tx_empty(&self) -> SLC_TX_EMPTY_R {
        SLC_TX_EMPTY_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SLC_TX_STATUS")
            .field(
                "slc_tx_empty",
                &format_args!("{}", self.slc_tx_empty().bit()),
            )
            .field("slc_tx_full", &format_args!("{}", self.slc_tx_full().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SLC_TX_STATUS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn slc_tx_full(&mut self) -> SLC_TX_FULL_W<SLC_TX_STATUS_SPEC> {
        SLC_TX_FULL_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn slc_tx_empty(&mut self) -> SLC_TX_EMPTY_W<SLC_TX_STATUS_SPEC> {
        SLC_TX_EMPTY_W::new(self, 1)
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
#[doc = "SLC_TX_STATUS\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slc_tx_status::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slc_tx_status::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SLC_TX_STATUS_SPEC;
impl crate::RegisterSpec for SLC_TX_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`slc_tx_status::R`](R) reader structure"]
impl crate::Readable for SLC_TX_STATUS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`slc_tx_status::W`](W) writer structure"]
impl crate::Writable for SLC_TX_STATUS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SLC_TX_STATUS to value 0"]
impl crate::Resettable for SLC_TX_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
