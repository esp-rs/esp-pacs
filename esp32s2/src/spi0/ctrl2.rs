///Register `CTRL2` reader
pub type R = crate::R<CTRL2_SPEC>;
///Register `CTRL2` writer
pub type W = crate::W<CTRL2_SPEC>;
///Field `CS_SETUP_TIME` reader - (cycles+1) of prepare phase by spi clock this bits are combined with SPI_CS_SETUP bit. Can be configured in CONF state.
pub type CS_SETUP_TIME_R = crate::FieldReader<u16>;
///Field `CS_SETUP_TIME` writer - (cycles+1) of prepare phase by spi clock this bits are combined with SPI_CS_SETUP bit. Can be configured in CONF state.
pub type CS_SETUP_TIME_W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
///Field `CS_HOLD_TIME` reader - delay cycles of cs pin by spi clock this bits are combined with SPI_CS_HOLD bit. Can be configured in CONF state.
pub type CS_HOLD_TIME_R = crate::FieldReader<u16>;
///Field `CS_HOLD_TIME` writer - delay cycles of cs pin by spi clock this bits are combined with SPI_CS_HOLD bit. Can be configured in CONF state.
pub type CS_HOLD_TIME_W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
///Field `CS_DELAY_MODE` reader - spi_cs signal is delayed by spi_clk . 0: zero 1: if SPI_CK_OUT_EDGE or SPI_CK_IDLE_EDGE is set 1 delayed by half cycle else delayed by one cycle 2: if SPI_CK_OUT_EDGE or SPI_CK_IDLE_EDGE is set 1 delayed by one cycle, else delayed by half cycle 3: delayed one cycle. Can be configured in CONF state.
pub type CS_DELAY_MODE_R = crate::FieldReader;
///Field `CS_DELAY_MODE` writer - spi_cs signal is delayed by spi_clk . 0: zero 1: if SPI_CK_OUT_EDGE or SPI_CK_IDLE_EDGE is set 1 delayed by half cycle else delayed by one cycle 2: if SPI_CK_OUT_EDGE or SPI_CK_IDLE_EDGE is set 1 delayed by one cycle, else delayed by half cycle 3: delayed one cycle. Can be configured in CONF state.
pub type CS_DELAY_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `CS_DELAY_NUM` reader - spi_cs signal is delayed by system clock cycles. Can be configured in CONF state.
pub type CS_DELAY_NUM_R = crate::FieldReader;
///Field `CS_DELAY_NUM` writer - spi_cs signal is delayed by system clock cycles. Can be configured in CONF state.
pub type CS_DELAY_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:12 - (cycles+1) of prepare phase by spi clock this bits are combined with SPI_CS_SETUP bit. Can be configured in CONF state.
    #[inline(always)]
    pub fn cs_setup_time(&self) -> CS_SETUP_TIME_R {
        CS_SETUP_TIME_R::new((self.bits & 0x1fff) as u16)
    }
    ///Bits 13:25 - delay cycles of cs pin by spi clock this bits are combined with SPI_CS_HOLD bit. Can be configured in CONF state.
    #[inline(always)]
    pub fn cs_hold_time(&self) -> CS_HOLD_TIME_R {
        CS_HOLD_TIME_R::new(((self.bits >> 13) & 0x1fff) as u16)
    }
    ///Bits 26:28 - spi_cs signal is delayed by spi_clk . 0: zero 1: if SPI_CK_OUT_EDGE or SPI_CK_IDLE_EDGE is set 1 delayed by half cycle else delayed by one cycle 2: if SPI_CK_OUT_EDGE or SPI_CK_IDLE_EDGE is set 1 delayed by one cycle, else delayed by half cycle 3: delayed one cycle. Can be configured in CONF state.
    #[inline(always)]
    pub fn cs_delay_mode(&self) -> CS_DELAY_MODE_R {
        CS_DELAY_MODE_R::new(((self.bits >> 26) & 7) as u8)
    }
    ///Bits 29:30 - spi_cs signal is delayed by system clock cycles. Can be configured in CONF state.
    #[inline(always)]
    pub fn cs_delay_num(&self) -> CS_DELAY_NUM_R {
        CS_DELAY_NUM_R::new(((self.bits >> 29) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL2")
            .field("cs_setup_time", &self.cs_setup_time())
            .field("cs_hold_time", &self.cs_hold_time())
            .field("cs_delay_mode", &self.cs_delay_mode())
            .field("cs_delay_num", &self.cs_delay_num())
            .finish()
    }
}
impl W {
    ///Bits 0:12 - (cycles+1) of prepare phase by spi clock this bits are combined with SPI_CS_SETUP bit. Can be configured in CONF state.
    #[inline(always)]
    #[must_use]
    pub fn cs_setup_time(&mut self) -> CS_SETUP_TIME_W<CTRL2_SPEC> {
        CS_SETUP_TIME_W::new(self, 0)
    }
    ///Bits 13:25 - delay cycles of cs pin by spi clock this bits are combined with SPI_CS_HOLD bit. Can be configured in CONF state.
    #[inline(always)]
    #[must_use]
    pub fn cs_hold_time(&mut self) -> CS_HOLD_TIME_W<CTRL2_SPEC> {
        CS_HOLD_TIME_W::new(self, 13)
    }
    ///Bits 26:28 - spi_cs signal is delayed by spi_clk . 0: zero 1: if SPI_CK_OUT_EDGE or SPI_CK_IDLE_EDGE is set 1 delayed by half cycle else delayed by one cycle 2: if SPI_CK_OUT_EDGE or SPI_CK_IDLE_EDGE is set 1 delayed by one cycle, else delayed by half cycle 3: delayed one cycle. Can be configured in CONF state.
    #[inline(always)]
    #[must_use]
    pub fn cs_delay_mode(&mut self) -> CS_DELAY_MODE_W<CTRL2_SPEC> {
        CS_DELAY_MODE_W::new(self, 26)
    }
    ///Bits 29:30 - spi_cs signal is delayed by system clock cycles. Can be configured in CONF state.
    #[inline(always)]
    #[must_use]
    pub fn cs_delay_num(&mut self) -> CS_DELAY_NUM_W<CTRL2_SPEC> {
        CS_DELAY_NUM_W::new(self, 29)
    }
}
/**SPI control register 2

You can [`read`](crate::generic::Reg::read) this register and get [`ctrl2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CTRL2_SPEC;
impl crate::RegisterSpec for CTRL2_SPEC {
    type Ux = u32;
}
///`read()` method returns [`ctrl2::R`](R) reader structure
impl crate::Readable for CTRL2_SPEC {}
///`write(|w| ..)` method takes [`ctrl2::W`](W) writer structure
impl crate::Writable for CTRL2_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CTRL2 to value 0x2000
impl crate::Resettable for CTRL2_SPEC {
    const RESET_VALUE: u32 = 0x2000;
}
