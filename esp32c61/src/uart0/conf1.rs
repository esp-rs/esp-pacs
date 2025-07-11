#[doc = "Register `CONF1` reader"]
pub type R = crate::R<CONF1_SPEC>;
#[doc = "Register `CONF1` writer"]
pub type W = crate::W<CONF1_SPEC>;
#[doc = "Field `RXFIFO_FULL_THRHD` reader - Configures the threshold for RX FIFO being full.\\\\Measurement unit: byte."]
pub type RXFIFO_FULL_THRHD_R = crate::FieldReader;
#[doc = "Field `RXFIFO_FULL_THRHD` writer - Configures the threshold for RX FIFO being full.\\\\Measurement unit: byte."]
pub type RXFIFO_FULL_THRHD_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TXFIFO_EMPTY_THRHD` reader - Configures the threshold for TX FIFO being empty.\\\\Measurement unit: byte."]
pub type TXFIFO_EMPTY_THRHD_R = crate::FieldReader;
#[doc = "Field `TXFIFO_EMPTY_THRHD` writer - Configures the threshold for TX FIFO being empty.\\\\Measurement unit: byte."]
pub type TXFIFO_EMPTY_THRHD_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CTS_INV` reader - Configures whether or not to invert the level of UART CTS signal.\\\\ 0: Not invert\\\\ 1: Invert\\\\"]
pub type CTS_INV_R = crate::BitReader;
#[doc = "Field `CTS_INV` writer - Configures whether or not to invert the level of UART CTS signal.\\\\ 0: Not invert\\\\ 1: Invert\\\\"]
pub type CTS_INV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSR_INV` reader - Configures whether or not to invert the level of UART DSR signal.\\\\ 0: Not invert\\\\ 1: Invert\\\\"]
pub type DSR_INV_R = crate::BitReader;
#[doc = "Field `DSR_INV` writer - Configures whether or not to invert the level of UART DSR signal.\\\\ 0: Not invert\\\\ 1: Invert\\\\"]
pub type DSR_INV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTS_INV` reader - Configures whether or not to invert the level of UART RTS signal.\\\\ 0: Not invert\\\\ 1: Invert\\\\"]
pub type RTS_INV_R = crate::BitReader;
#[doc = "Field `RTS_INV` writer - Configures whether or not to invert the level of UART RTS signal.\\\\ 0: Not invert\\\\ 1: Invert\\\\"]
pub type RTS_INV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTR_INV` reader - Configures whether or not to invert the level of UART DTR signal.\\\\ 0: Not invert\\\\ 1: Invert\\\\"]
pub type DTR_INV_R = crate::BitReader;
#[doc = "Field `DTR_INV` writer - Configures whether or not to invert the level of UART DTR signal.\\\\ 0: Not invert\\\\ 1: Invert\\\\"]
pub type DTR_INV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_DTR` reader - Configures the DTR signal used in software flow control.\\\\ 0: Data to be transmitted is not ready.\\\\ 1: Data to be transmitted is ready.\\\\"]
pub type SW_DTR_R = crate::BitReader;
#[doc = "Field `SW_DTR` writer - Configures the DTR signal used in software flow control.\\\\ 0: Data to be transmitted is not ready.\\\\ 1: Data to be transmitted is ready.\\\\"]
pub type SW_DTR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_EN` reader - Configures clock gating.\\\\ 0: Support clock only when the application writes registers.\\\\ 1: Always force the clock on for registers.\\\\"]
pub type CLK_EN_R = crate::BitReader;
#[doc = "Field `CLK_EN` writer - Configures clock gating.\\\\ 0: Support clock only when the application writes registers.\\\\ 1: Always force the clock on for registers.\\\\"]
pub type CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Configures the threshold for RX FIFO being full.\\\\Measurement unit: byte."]
    #[inline(always)]
    pub fn rxfifo_full_thrhd(&self) -> RXFIFO_FULL_THRHD_R {
        RXFIFO_FULL_THRHD_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Configures the threshold for TX FIFO being empty.\\\\Measurement unit: byte."]
    #[inline(always)]
    pub fn txfifo_empty_thrhd(&self) -> TXFIFO_EMPTY_THRHD_R {
        TXFIFO_EMPTY_THRHD_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16 - Configures whether or not to invert the level of UART CTS signal.\\\\ 0: Not invert\\\\ 1: Invert\\\\"]
    #[inline(always)]
    pub fn cts_inv(&self) -> CTS_INV_R {
        CTS_INV_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Configures whether or not to invert the level of UART DSR signal.\\\\ 0: Not invert\\\\ 1: Invert\\\\"]
    #[inline(always)]
    pub fn dsr_inv(&self) -> DSR_INV_R {
        DSR_INV_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Configures whether or not to invert the level of UART RTS signal.\\\\ 0: Not invert\\\\ 1: Invert\\\\"]
    #[inline(always)]
    pub fn rts_inv(&self) -> RTS_INV_R {
        RTS_INV_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Configures whether or not to invert the level of UART DTR signal.\\\\ 0: Not invert\\\\ 1: Invert\\\\"]
    #[inline(always)]
    pub fn dtr_inv(&self) -> DTR_INV_R {
        DTR_INV_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Configures the DTR signal used in software flow control.\\\\ 0: Data to be transmitted is not ready.\\\\ 1: Data to be transmitted is ready.\\\\"]
    #[inline(always)]
    pub fn sw_dtr(&self) -> SW_DTR_R {
        SW_DTR_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Configures clock gating.\\\\ 0: Support clock only when the application writes registers.\\\\ 1: Always force the clock on for registers.\\\\"]
    #[inline(always)]
    pub fn clk_en(&self) -> CLK_EN_R {
        CLK_EN_R::new(((self.bits >> 21) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONF1")
            .field("rxfifo_full_thrhd", &self.rxfifo_full_thrhd())
            .field("txfifo_empty_thrhd", &self.txfifo_empty_thrhd())
            .field("cts_inv", &self.cts_inv())
            .field("dsr_inv", &self.dsr_inv())
            .field("rts_inv", &self.rts_inv())
            .field("dtr_inv", &self.dtr_inv())
            .field("sw_dtr", &self.sw_dtr())
            .field("clk_en", &self.clk_en())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - Configures the threshold for RX FIFO being full.\\\\Measurement unit: byte."]
    #[inline(always)]
    pub fn rxfifo_full_thrhd(&mut self) -> RXFIFO_FULL_THRHD_W<CONF1_SPEC> {
        RXFIFO_FULL_THRHD_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Configures the threshold for TX FIFO being empty.\\\\Measurement unit: byte."]
    #[inline(always)]
    pub fn txfifo_empty_thrhd(&mut self) -> TXFIFO_EMPTY_THRHD_W<CONF1_SPEC> {
        TXFIFO_EMPTY_THRHD_W::new(self, 8)
    }
    #[doc = "Bit 16 - Configures whether or not to invert the level of UART CTS signal.\\\\ 0: Not invert\\\\ 1: Invert\\\\"]
    #[inline(always)]
    pub fn cts_inv(&mut self) -> CTS_INV_W<CONF1_SPEC> {
        CTS_INV_W::new(self, 16)
    }
    #[doc = "Bit 17 - Configures whether or not to invert the level of UART DSR signal.\\\\ 0: Not invert\\\\ 1: Invert\\\\"]
    #[inline(always)]
    pub fn dsr_inv(&mut self) -> DSR_INV_W<CONF1_SPEC> {
        DSR_INV_W::new(self, 17)
    }
    #[doc = "Bit 18 - Configures whether or not to invert the level of UART RTS signal.\\\\ 0: Not invert\\\\ 1: Invert\\\\"]
    #[inline(always)]
    pub fn rts_inv(&mut self) -> RTS_INV_W<CONF1_SPEC> {
        RTS_INV_W::new(self, 18)
    }
    #[doc = "Bit 19 - Configures whether or not to invert the level of UART DTR signal.\\\\ 0: Not invert\\\\ 1: Invert\\\\"]
    #[inline(always)]
    pub fn dtr_inv(&mut self) -> DTR_INV_W<CONF1_SPEC> {
        DTR_INV_W::new(self, 19)
    }
    #[doc = "Bit 20 - Configures the DTR signal used in software flow control.\\\\ 0: Data to be transmitted is not ready.\\\\ 1: Data to be transmitted is ready.\\\\"]
    #[inline(always)]
    pub fn sw_dtr(&mut self) -> SW_DTR_W<CONF1_SPEC> {
        SW_DTR_W::new(self, 20)
    }
    #[doc = "Bit 21 - Configures clock gating.\\\\ 0: Support clock only when the application writes registers.\\\\ 1: Always force the clock on for registers.\\\\"]
    #[inline(always)]
    pub fn clk_en(&mut self) -> CLK_EN_W<CONF1_SPEC> {
        CLK_EN_W::new(self, 21)
    }
}
#[doc = "Configuration register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`conf1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`conf1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CONF1_SPEC;
impl crate::RegisterSpec for CONF1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`conf1::R`](R) reader structure"]
impl crate::Readable for CONF1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`conf1::W`](W) writer structure"]
impl crate::Writable for CONF1_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CONF1 to value 0x6060"]
impl crate::Resettable for CONF1_SPEC {
    const RESET_VALUE: u32 = 0x6060;
}
