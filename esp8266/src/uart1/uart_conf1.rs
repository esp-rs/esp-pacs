#[doc = "Register `UART_CONF1` reader"]
pub type R = crate::R<UART_CONF1_SPEC>;
#[doc = "Register `UART_CONF1` writer"]
pub type W = crate::W<UART_CONF1_SPEC>;
#[doc = "Field `rxfifo_full_thrhd` reader - The config bits for rx fifo full threshold,0-127"]
pub type RXFIFO_FULL_THRHD_R = crate::FieldReader;
#[doc = "Field `rxfifo_full_thrhd` writer - The config bits for rx fifo full threshold,0-127"]
pub type RXFIFO_FULL_THRHD_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `txfifo_empty_thrhd` reader - The config bits for tx fifo empty threshold,0-127"]
pub type TXFIFO_EMPTY_THRHD_R = crate::FieldReader;
#[doc = "Field `txfifo_empty_thrhd` writer - The config bits for tx fifo empty threshold,0-127"]
pub type TXFIFO_EMPTY_THRHD_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `rx_flow_thrhd` reader - The config bits for rx flow control threshold,0-127"]
pub type RX_FLOW_THRHD_R = crate::FieldReader;
#[doc = "Field `rx_flow_thrhd` writer - The config bits for rx flow control threshold,0-127"]
pub type RX_FLOW_THRHD_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `rx_flow_en` reader - Set this bit to enable rx hardware flow control"]
pub type RX_FLOW_EN_R = crate::BitReader;
#[doc = "Field `rx_flow_en` writer - Set this bit to enable rx hardware flow control"]
pub type RX_FLOW_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rx_tout_thrhd` reader - Config bits for rx time-out threshold,uint: byte,0-127"]
pub type RX_TOUT_THRHD_R = crate::FieldReader;
#[doc = "Field `rx_tout_thrhd` writer - Config bits for rx time-out threshold,uint: byte,0-127"]
pub type RX_TOUT_THRHD_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `rx_tout_en` reader - Set this bit to enable rx time-out function"]
pub type RX_TOUT_EN_R = crate::BitReader;
#[doc = "Field `rx_tout_en` writer - Set this bit to enable rx time-out function"]
pub type RX_TOUT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - The config bits for rx fifo full threshold,0-127"]
    #[inline(always)]
    pub fn rxfifo_full_thrhd(&self) -> RXFIFO_FULL_THRHD_R {
        RXFIFO_FULL_THRHD_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - The config bits for tx fifo empty threshold,0-127"]
    #[inline(always)]
    pub fn txfifo_empty_thrhd(&self) -> TXFIFO_EMPTY_THRHD_R {
        TXFIFO_EMPTY_THRHD_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - The config bits for rx flow control threshold,0-127"]
    #[inline(always)]
    pub fn rx_flow_thrhd(&self) -> RX_FLOW_THRHD_R {
        RX_FLOW_THRHD_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bit 23 - Set this bit to enable rx hardware flow control"]
    #[inline(always)]
    pub fn rx_flow_en(&self) -> RX_FLOW_EN_R {
        RX_FLOW_EN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:30 - Config bits for rx time-out threshold,uint: byte,0-127"]
    #[inline(always)]
    pub fn rx_tout_thrhd(&self) -> RX_TOUT_THRHD_R {
        RX_TOUT_THRHD_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
    #[doc = "Bit 31 - Set this bit to enable rx time-out function"]
    #[inline(always)]
    pub fn rx_tout_en(&self) -> RX_TOUT_EN_R {
        RX_TOUT_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UART_CONF1")
            .field("rx_tout_en", &format_args!("{}", self.rx_tout_en().bit()))
            .field(
                "rx_tout_thrhd",
                &format_args!("{}", self.rx_tout_thrhd().bits()),
            )
            .field("rx_flow_en", &format_args!("{}", self.rx_flow_en().bit()))
            .field(
                "rx_flow_thrhd",
                &format_args!("{}", self.rx_flow_thrhd().bits()),
            )
            .field(
                "txfifo_empty_thrhd",
                &format_args!("{}", self.txfifo_empty_thrhd().bits()),
            )
            .field(
                "rxfifo_full_thrhd",
                &format_args!("{}", self.rxfifo_full_thrhd().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<UART_CONF1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:6 - The config bits for rx fifo full threshold,0-127"]
    #[inline(always)]
    #[must_use]
    pub fn rxfifo_full_thrhd(&mut self) -> RXFIFO_FULL_THRHD_W<UART_CONF1_SPEC> {
        RXFIFO_FULL_THRHD_W::new(self, 0)
    }
    #[doc = "Bits 8:14 - The config bits for tx fifo empty threshold,0-127"]
    #[inline(always)]
    #[must_use]
    pub fn txfifo_empty_thrhd(&mut self) -> TXFIFO_EMPTY_THRHD_W<UART_CONF1_SPEC> {
        TXFIFO_EMPTY_THRHD_W::new(self, 8)
    }
    #[doc = "Bits 16:22 - The config bits for rx flow control threshold,0-127"]
    #[inline(always)]
    #[must_use]
    pub fn rx_flow_thrhd(&mut self) -> RX_FLOW_THRHD_W<UART_CONF1_SPEC> {
        RX_FLOW_THRHD_W::new(self, 16)
    }
    #[doc = "Bit 23 - Set this bit to enable rx hardware flow control"]
    #[inline(always)]
    #[must_use]
    pub fn rx_flow_en(&mut self) -> RX_FLOW_EN_W<UART_CONF1_SPEC> {
        RX_FLOW_EN_W::new(self, 23)
    }
    #[doc = "Bits 24:30 - Config bits for rx time-out threshold,uint: byte,0-127"]
    #[inline(always)]
    #[must_use]
    pub fn rx_tout_thrhd(&mut self) -> RX_TOUT_THRHD_W<UART_CONF1_SPEC> {
        RX_TOUT_THRHD_W::new(self, 24)
    }
    #[doc = "Bit 31 - Set this bit to enable rx time-out function"]
    #[inline(always)]
    #[must_use]
    pub fn rx_tout_en(&mut self) -> RX_TOUT_EN_W<UART_CONF1_SPEC> {
        RX_TOUT_EN_W::new(self, 31)
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
#[doc = "Set this bit to enable rx time-out function\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart_conf1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uart_conf1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UART_CONF1_SPEC;
impl crate::RegisterSpec for UART_CONF1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uart_conf1::R`](R) reader structure"]
impl crate::Readable for UART_CONF1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`uart_conf1::W`](W) writer structure"]
impl crate::Writable for UART_CONF1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UART_CONF1 to value 0"]
impl crate::Resettable for UART_CONF1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
