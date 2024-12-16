#[doc = "Register `CONF1` reader"]
pub type R = crate::R<CONF1_SPEC>;
#[doc = "Register `CONF1` writer"]
pub type W = crate::W<CONF1_SPEC>;
#[doc = "Field `RXFIFO_FULL_THRHD` reader - When receiver receives more data than its threshold value.receiver will produce rxfifo_full_int_raw interrupt.the threshold value is (rx_flow_thrhd_h3 rxfifo_full_thrhd)."]
pub type RXFIFO_FULL_THRHD_R = crate::FieldReader;
#[doc = "Field `RXFIFO_FULL_THRHD` writer - When receiver receives more data than its threshold value.receiver will produce rxfifo_full_int_raw interrupt.the threshold value is (rx_flow_thrhd_h3 rxfifo_full_thrhd)."]
pub type RXFIFO_FULL_THRHD_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `TXFIFO_EMPTY_THRHD` reader - when the data amount in transmitter fifo is less than its threshold value. it will produce txfifo_empty_int_raw interrupt. the threshold value is (tx_mem_empty_thrhd txfifo_empty_thrhd)"]
pub type TXFIFO_EMPTY_THRHD_R = crate::FieldReader;
#[doc = "Field `TXFIFO_EMPTY_THRHD` writer - when the data amount in transmitter fifo is less than its threshold value. it will produce txfifo_empty_int_raw interrupt. the threshold value is (tx_mem_empty_thrhd txfifo_empty_thrhd)"]
pub type TXFIFO_EMPTY_THRHD_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `RX_FLOW_THRHD` reader - when receiver receives more data than its threshold value. receiver produce signal to tell the transmitter stop transferring data. the threshold value is (rx_flow_thrhd_h3 rx_flow_thrhd)."]
pub type RX_FLOW_THRHD_R = crate::FieldReader;
#[doc = "Field `RX_FLOW_THRHD` writer - when receiver receives more data than its threshold value. receiver produce signal to tell the transmitter stop transferring data. the threshold value is (rx_flow_thrhd_h3 rx_flow_thrhd)."]
pub type RX_FLOW_THRHD_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `RX_FLOW_EN` reader - This is the flow enable bit for uart receiver. 1:choose software flow control with configuring sw_rts signal"]
pub type RX_FLOW_EN_R = crate::BitReader;
#[doc = "Field `RX_FLOW_EN` writer - This is the flow enable bit for uart receiver. 1:choose software flow control with configuring sw_rts signal"]
pub type RX_FLOW_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_TOUT_THRHD` reader - This register is used to configure the timeout value for uart receiver receiving a byte."]
pub type RX_TOUT_THRHD_R = crate::FieldReader;
#[doc = "Field `RX_TOUT_THRHD` writer - This register is used to configure the timeout value for uart receiver receiving a byte."]
pub type RX_TOUT_THRHD_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `RX_TOUT_EN` reader - This is the enble bit for uart receiver's timeout function."]
pub type RX_TOUT_EN_R = crate::BitReader;
#[doc = "Field `RX_TOUT_EN` writer - This is the enble bit for uart receiver's timeout function."]
pub type RX_TOUT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - When receiver receives more data than its threshold value.receiver will produce rxfifo_full_int_raw interrupt.the threshold value is (rx_flow_thrhd_h3 rxfifo_full_thrhd)."]
    #[inline(always)]
    pub fn rxfifo_full_thrhd(&self) -> RXFIFO_FULL_THRHD_R {
        RXFIFO_FULL_THRHD_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - when the data amount in transmitter fifo is less than its threshold value. it will produce txfifo_empty_int_raw interrupt. the threshold value is (tx_mem_empty_thrhd txfifo_empty_thrhd)"]
    #[inline(always)]
    pub fn txfifo_empty_thrhd(&self) -> TXFIFO_EMPTY_THRHD_R {
        TXFIFO_EMPTY_THRHD_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - when receiver receives more data than its threshold value. receiver produce signal to tell the transmitter stop transferring data. the threshold value is (rx_flow_thrhd_h3 rx_flow_thrhd)."]
    #[inline(always)]
    pub fn rx_flow_thrhd(&self) -> RX_FLOW_THRHD_R {
        RX_FLOW_THRHD_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bit 23 - This is the flow enable bit for uart receiver. 1:choose software flow control with configuring sw_rts signal"]
    #[inline(always)]
    pub fn rx_flow_en(&self) -> RX_FLOW_EN_R {
        RX_FLOW_EN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:30 - This register is used to configure the timeout value for uart receiver receiving a byte."]
    #[inline(always)]
    pub fn rx_tout_thrhd(&self) -> RX_TOUT_THRHD_R {
        RX_TOUT_THRHD_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
    #[doc = "Bit 31 - This is the enble bit for uart receiver's timeout function."]
    #[inline(always)]
    pub fn rx_tout_en(&self) -> RX_TOUT_EN_R {
        RX_TOUT_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONF1")
            .field("rxfifo_full_thrhd", &self.rxfifo_full_thrhd())
            .field("txfifo_empty_thrhd", &self.txfifo_empty_thrhd())
            .field("rx_flow_thrhd", &self.rx_flow_thrhd())
            .field("rx_flow_en", &self.rx_flow_en())
            .field("rx_tout_thrhd", &self.rx_tout_thrhd())
            .field("rx_tout_en", &self.rx_tout_en())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:6 - When receiver receives more data than its threshold value.receiver will produce rxfifo_full_int_raw interrupt.the threshold value is (rx_flow_thrhd_h3 rxfifo_full_thrhd)."]
    #[inline(always)]
    pub fn rxfifo_full_thrhd(&mut self) -> RXFIFO_FULL_THRHD_W<CONF1_SPEC> {
        RXFIFO_FULL_THRHD_W::new(self, 0)
    }
    #[doc = "Bits 8:14 - when the data amount in transmitter fifo is less than its threshold value. it will produce txfifo_empty_int_raw interrupt. the threshold value is (tx_mem_empty_thrhd txfifo_empty_thrhd)"]
    #[inline(always)]
    pub fn txfifo_empty_thrhd(&mut self) -> TXFIFO_EMPTY_THRHD_W<CONF1_SPEC> {
        TXFIFO_EMPTY_THRHD_W::new(self, 8)
    }
    #[doc = "Bits 16:22 - when receiver receives more data than its threshold value. receiver produce signal to tell the transmitter stop transferring data. the threshold value is (rx_flow_thrhd_h3 rx_flow_thrhd)."]
    #[inline(always)]
    pub fn rx_flow_thrhd(&mut self) -> RX_FLOW_THRHD_W<CONF1_SPEC> {
        RX_FLOW_THRHD_W::new(self, 16)
    }
    #[doc = "Bit 23 - This is the flow enable bit for uart receiver. 1:choose software flow control with configuring sw_rts signal"]
    #[inline(always)]
    pub fn rx_flow_en(&mut self) -> RX_FLOW_EN_W<CONF1_SPEC> {
        RX_FLOW_EN_W::new(self, 23)
    }
    #[doc = "Bits 24:30 - This register is used to configure the timeout value for uart receiver receiving a byte."]
    #[inline(always)]
    pub fn rx_tout_thrhd(&mut self) -> RX_TOUT_THRHD_W<CONF1_SPEC> {
        RX_TOUT_THRHD_W::new(self, 24)
    }
    #[doc = "Bit 31 - This is the enble bit for uart receiver's timeout function."]
    #[inline(always)]
    pub fn rx_tout_en(&mut self) -> RX_TOUT_EN_W<CONF1_SPEC> {
        RX_TOUT_EN_W::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`conf1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`conf1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CONF1_SPEC;
impl crate::RegisterSpec for CONF1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`conf1::R`](R) reader structure"]
impl crate::Readable for CONF1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`conf1::W`](W) writer structure"]
impl crate::Writable for CONF1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CONF1 to value 0x6060"]
impl crate::Resettable for CONF1_SPEC {
    const RESET_VALUE: u32 = 0x6060;
}
