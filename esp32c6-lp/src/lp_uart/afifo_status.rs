#[doc = "Register `AFIFO_STATUS` reader"]
pub struct R(crate::R<AFIFO_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AFIFO_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AFIFO_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AFIFO_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TX_AFIFO_FULL` reader - Full signal of APB TX AFIFO."]
pub type TX_AFIFO_FULL_R = crate::BitReader;
#[doc = "Field `TX_AFIFO_EMPTY` reader - Empty signal of APB TX AFIFO."]
pub type TX_AFIFO_EMPTY_R = crate::BitReader;
#[doc = "Field `RX_AFIFO_FULL` reader - Full signal of APB RX AFIFO."]
pub type RX_AFIFO_FULL_R = crate::BitReader;
#[doc = "Field `RX_AFIFO_EMPTY` reader - Empty signal of APB RX AFIFO."]
pub type RX_AFIFO_EMPTY_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Full signal of APB TX AFIFO."]
    #[inline(always)]
    pub fn tx_afifo_full(&self) -> TX_AFIFO_FULL_R {
        TX_AFIFO_FULL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Empty signal of APB TX AFIFO."]
    #[inline(always)]
    pub fn tx_afifo_empty(&self) -> TX_AFIFO_EMPTY_R {
        TX_AFIFO_EMPTY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Full signal of APB RX AFIFO."]
    #[inline(always)]
    pub fn rx_afifo_full(&self) -> RX_AFIFO_FULL_R {
        RX_AFIFO_FULL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Empty signal of APB RX AFIFO."]
    #[inline(always)]
    pub fn rx_afifo_empty(&self) -> RX_AFIFO_EMPTY_R {
        RX_AFIFO_EMPTY_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AFIFO_STATUS")
            .field(
                "tx_afifo_full",
                &format_args!("{}", self.tx_afifo_full().bit()),
            )
            .field(
                "tx_afifo_empty",
                &format_args!("{}", self.tx_afifo_empty().bit()),
            )
            .field(
                "rx_afifo_full",
                &format_args!("{}", self.rx_afifo_full().bit()),
            )
            .field(
                "rx_afifo_empty",
                &format_args!("{}", self.rx_afifo_empty().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<AFIFO_STATUS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "UART AFIFO Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [afifo_status](index.html) module"]
pub struct AFIFO_STATUS_SPEC;
impl crate::RegisterSpec for AFIFO_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [afifo_status::R](R) reader structure"]
impl crate::Readable for AFIFO_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets AFIFO_STATUS to value 0x0a"]
impl crate::Resettable for AFIFO_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0x0a;
}
