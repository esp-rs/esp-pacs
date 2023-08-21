#[doc = "Register `UART_RXD_CNT` reader"]
pub type R = crate::R<UART_RXD_CNT_SPEC>;
#[doc = "Field `rxd_edge_cnt` reader - used in baudrate detect"]
pub type RXD_EDGE_CNT_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:9 - used in baudrate detect"]
    #[inline(always)]
    pub fn rxd_edge_cnt(&self) -> RXD_EDGE_CNT_R {
        RXD_EDGE_CNT_R::new((self.bits & 0x03ff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UART_RXD_CNT")
            .field(
                "rxd_edge_cnt",
                &format_args!("{}", self.rxd_edge_cnt().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<UART_RXD_CNT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "UART_RXD_CNT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart_rxd_cnt::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UART_RXD_CNT_SPEC;
impl crate::RegisterSpec for UART_RXD_CNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uart_rxd_cnt::R`](R) reader structure"]
impl crate::Readable for UART_RXD_CNT_SPEC {}
#[doc = "`reset()` method sets UART_RXD_CNT to value 0"]
impl crate::Resettable for UART_RXD_CNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
