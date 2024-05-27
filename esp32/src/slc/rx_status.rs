#[doc = "Register `RX_STATUS` reader"]
pub type R = crate::R<RX_STATUS_SPEC>;
#[doc = "Field `SLC0_RX_FULL` reader - "]
pub type SLC0_RX_FULL_R = crate::BitReader;
#[doc = "Field `SLC0_RX_EMPTY` reader - "]
pub type SLC0_RX_EMPTY_R = crate::BitReader;
#[doc = "Field `SLC1_RX_FULL` reader - "]
pub type SLC1_RX_FULL_R = crate::BitReader;
#[doc = "Field `SLC1_RX_EMPTY` reader - "]
pub type SLC1_RX_EMPTY_R = crate::BitReader;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn slc0_rx_full(&self) -> SLC0_RX_FULL_R {
        SLC0_RX_FULL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn slc0_rx_empty(&self) -> SLC0_RX_EMPTY_R {
        SLC0_RX_EMPTY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn slc1_rx_full(&self) -> SLC1_RX_FULL_R {
        SLC1_RX_FULL_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn slc1_rx_empty(&self) -> SLC1_RX_EMPTY_R {
        SLC1_RX_EMPTY_R::new(((self.bits >> 17) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RX_STATUS")
            .field("slc0_rx_full", &self.slc0_rx_full())
            .field("slc0_rx_empty", &self.slc0_rx_empty())
            .field("slc1_rx_full", &self.slc1_rx_full())
            .field("slc1_rx_empty", &self.slc1_rx_empty())
            .finish()
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RX_STATUS_SPEC;
impl crate::RegisterSpec for RX_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_status::R`](R) reader structure"]
impl crate::Readable for RX_STATUS_SPEC {}
#[doc = "`reset()` method sets RX_STATUS to value 0x0002_0002"]
impl crate::Resettable for RX_STATUS_SPEC {
    const RESET_VALUE: u32 = 0x0002_0002;
}
