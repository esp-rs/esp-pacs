#[doc = "Register `STATE` reader"]
pub struct R(crate::R<STATE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TX_IDLE` reader - "]
pub type TX_IDLE_R = crate::BitReader;
#[doc = "Field `TX_FIFO_RESET_BACK` reader - "]
pub type TX_FIFO_RESET_BACK_R = crate::BitReader;
#[doc = "Field `RX_FIFO_RESET_BACK` reader - "]
pub type RX_FIFO_RESET_BACK_R = crate::BitReader;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn tx_idle(&self) -> TX_IDLE_R {
        TX_IDLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn tx_fifo_reset_back(&self) -> TX_FIFO_RESET_BACK_R {
        TX_FIFO_RESET_BACK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rx_fifo_reset_back(&self) -> RX_FIFO_RESET_BACK_R {
        RX_FIFO_RESET_BACK_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STATE")
            .field("tx_idle", &format_args!("{}", self.tx_idle().bit()))
            .field(
                "tx_fifo_reset_back",
                &format_args!("{}", self.tx_fifo_reset_back().bit()),
            )
            .field(
                "rx_fifo_reset_back",
                &format_args!("{}", self.rx_fifo_reset_back().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<STATE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [state](index.html) module"]
pub struct STATE_SPEC;
impl crate::RegisterSpec for STATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [state::R](R) reader structure"]
impl crate::Readable for STATE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STATE to value 0x07"]
impl crate::Resettable for STATE_SPEC {
    const RESET_VALUE: Self::Ux = 0x07;
}
