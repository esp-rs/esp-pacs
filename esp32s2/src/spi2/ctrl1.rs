#[doc = "Register `CTRL1` reader"]
pub type R = crate::R<CTRL1_SPEC>;
#[doc = "Register `CTRL1` writer"]
pub type W = crate::W<CTRL1_SPEC>;
#[doc = "Field `CLK_MODE` reader - SPI clock mode bits. 0: SPI clock is off when CS inactive 1: SPI clock is delayed one cycle after CS inactive 2: SPI clock is delayed two cycles after CS inactive 3: SPI clock is alwasy on. Can be configured in CONF state."]
pub type CLK_MODE_R = crate::FieldReader;
#[doc = "Field `CLK_MODE` writer - SPI clock mode bits. 0: SPI clock is off when CS inactive 1: SPI clock is delayed one cycle after CS inactive 2: SPI clock is delayed two cycles after CS inactive 3: SPI clock is alwasy on. Can be configured in CONF state."]
pub type CLK_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CLK_MODE_13` reader - {CPOL, CPHA},1: support spi clk mode 1 and 3, first edge output data B\\[0\\]/B\\[7\\]. 0: support spi clk mode 0 and 2, first edge output data B\\[1\\]/B\\[6\\]."]
pub type CLK_MODE_13_R = crate::BitReader;
#[doc = "Field `CLK_MODE_13` writer - {CPOL, CPHA},1: support spi clk mode 1 and 3, first edge output data B\\[0\\]/B\\[7\\]. 0: support spi clk mode 0 and 2, first edge output data B\\[1\\]/B\\[6\\]."]
pub type CLK_MODE_13_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSCK_DATA_OUT` reader - It saves half a cycle when tsck is the same as rsck. 1: output data at rsck posedge 0: output data at tsck posedge"]
pub type RSCK_DATA_OUT_R = crate::BitReader;
#[doc = "Field `RSCK_DATA_OUT` writer - It saves half a cycle when tsck is the same as rsck. 1: output data at rsck posedge 0: output data at tsck posedge"]
pub type RSCK_DATA_OUT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `W16_17_WR_ENA` reader - 1:SPI_BUF16~SPI_BUF17 can be written 0:SPI_BUF16~SPI_BUF17 can not be written. Can be configured in CONF state."]
pub type W16_17_WR_ENA_R = crate::BitReader;
#[doc = "Field `W16_17_WR_ENA` writer - 1:SPI_BUF16~SPI_BUF17 can be written 0:SPI_BUF16~SPI_BUF17 can not be written. Can be configured in CONF state."]
pub type W16_17_WR_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CS_HOLD_DELAY` reader - SPI cs signal is delayed by spi clock cycles. Can be configured in CONF state."]
pub type CS_HOLD_DELAY_R = crate::FieldReader;
#[doc = "Field `CS_HOLD_DELAY` writer - SPI cs signal is delayed by spi clock cycles. Can be configured in CONF state."]
pub type CS_HOLD_DELAY_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:1 - SPI clock mode bits. 0: SPI clock is off when CS inactive 1: SPI clock is delayed one cycle after CS inactive 2: SPI clock is delayed two cycles after CS inactive 3: SPI clock is alwasy on. Can be configured in CONF state."]
    #[inline(always)]
    pub fn clk_mode(&self) -> CLK_MODE_R {
        CLK_MODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - {CPOL, CPHA},1: support spi clk mode 1 and 3, first edge output data B\\[0\\]/B\\[7\\]. 0: support spi clk mode 0 and 2, first edge output data B\\[1\\]/B\\[6\\]."]
    #[inline(always)]
    pub fn clk_mode_13(&self) -> CLK_MODE_13_R {
        CLK_MODE_13_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - It saves half a cycle when tsck is the same as rsck. 1: output data at rsck posedge 0: output data at tsck posedge"]
    #[inline(always)]
    pub fn rsck_data_out(&self) -> RSCK_DATA_OUT_R {
        RSCK_DATA_OUT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 1:SPI_BUF16~SPI_BUF17 can be written 0:SPI_BUF16~SPI_BUF17 can not be written. Can be configured in CONF state."]
    #[inline(always)]
    pub fn w16_17_wr_ena(&self) -> W16_17_WR_ENA_R {
        W16_17_WR_ENA_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 14:19 - SPI cs signal is delayed by spi clock cycles. Can be configured in CONF state."]
    #[inline(always)]
    pub fn cs_hold_delay(&self) -> CS_HOLD_DELAY_R {
        CS_HOLD_DELAY_R::new(((self.bits >> 14) & 0x3f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL1")
            .field("clk_mode", &self.clk_mode())
            .field("clk_mode_13", &self.clk_mode_13())
            .field("rsck_data_out", &self.rsck_data_out())
            .field("w16_17_wr_ena", &self.w16_17_wr_ena())
            .field("cs_hold_delay", &self.cs_hold_delay())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - SPI clock mode bits. 0: SPI clock is off when CS inactive 1: SPI clock is delayed one cycle after CS inactive 2: SPI clock is delayed two cycles after CS inactive 3: SPI clock is alwasy on. Can be configured in CONF state."]
    #[inline(always)]
    pub fn clk_mode(&mut self) -> CLK_MODE_W<'_, CTRL1_SPEC> {
        CLK_MODE_W::new(self, 0)
    }
    #[doc = "Bit 2 - {CPOL, CPHA},1: support spi clk mode 1 and 3, first edge output data B\\[0\\]/B\\[7\\]. 0: support spi clk mode 0 and 2, first edge output data B\\[1\\]/B\\[6\\]."]
    #[inline(always)]
    pub fn clk_mode_13(&mut self) -> CLK_MODE_13_W<'_, CTRL1_SPEC> {
        CLK_MODE_13_W::new(self, 2)
    }
    #[doc = "Bit 3 - It saves half a cycle when tsck is the same as rsck. 1: output data at rsck posedge 0: output data at tsck posedge"]
    #[inline(always)]
    pub fn rsck_data_out(&mut self) -> RSCK_DATA_OUT_W<'_, CTRL1_SPEC> {
        RSCK_DATA_OUT_W::new(self, 3)
    }
    #[doc = "Bit 4 - 1:SPI_BUF16~SPI_BUF17 can be written 0:SPI_BUF16~SPI_BUF17 can not be written. Can be configured in CONF state."]
    #[inline(always)]
    pub fn w16_17_wr_ena(&mut self) -> W16_17_WR_ENA_W<'_, CTRL1_SPEC> {
        W16_17_WR_ENA_W::new(self, 4)
    }
    #[doc = "Bits 14:19 - SPI cs signal is delayed by spi clock cycles. Can be configured in CONF state."]
    #[inline(always)]
    pub fn cs_hold_delay(&mut self) -> CS_HOLD_DELAY_W<'_, CTRL1_SPEC> {
        CS_HOLD_DELAY_W::new(self, 14)
    }
}
#[doc = "SPI control register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRL1_SPEC;
impl crate::RegisterSpec for CTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl1::R`](R) reader structure"]
impl crate::Readable for CTRL1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrl1::W`](W) writer structure"]
impl crate::Writable for CTRL1_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRL1 to value 0x4010"]
impl crate::Resettable for CTRL1_SPEC {
    const RESET_VALUE: u32 = 0x4010;
}
