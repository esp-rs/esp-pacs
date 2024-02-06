#[doc = "Register `CTRL2` reader"]
pub type R = crate::R<CTRL2_SPEC>;
#[doc = "Register `CTRL2` writer"]
pub type W = crate::W<CTRL2_SPEC>;
#[doc = "Field `CS_SETUP_TIME` reader - (cycles+1) of prepare phase by spi clock this bits are combined with SPI_CS_SETUP bit. Can be configured in CONF state."]
pub type CS_SETUP_TIME_R = crate::FieldReader<u16>;
#[doc = "Field `CS_SETUP_TIME` writer - (cycles+1) of prepare phase by spi clock this bits are combined with SPI_CS_SETUP bit. Can be configured in CONF state."]
pub type CS_SETUP_TIME_W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
#[doc = "Field `CS_HOLD_TIME` reader - delay cycles of cs pin by spi clock this bits are combined with SPI_CS_HOLD bit. Can be configured in CONF state."]
pub type CS_HOLD_TIME_R = crate::FieldReader<u16>;
#[doc = "Field `CS_HOLD_TIME` writer - delay cycles of cs pin by spi clock this bits are combined with SPI_CS_HOLD bit. Can be configured in CONF state."]
pub type CS_HOLD_TIME_W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
#[doc = "Field `CS_DELAY_MODE` reader - spi_cs signal is delayed by spi_clk . 0: zero 1: if SPI_CK_OUT_EDGE or SPI_CK_IDLE_EDGE is set 1 delayed by half cycle else delayed by one cycle 2: if SPI_CK_OUT_EDGE or SPI_CK_IDLE_EDGE is set 1 delayed by one cycle, else delayed by half cycle 3: delayed one cycle. Can be configured in CONF state."]
pub type CS_DELAY_MODE_R = crate::FieldReader;
#[doc = "Field `CS_DELAY_MODE` writer - spi_cs signal is delayed by spi_clk . 0: zero 1: if SPI_CK_OUT_EDGE or SPI_CK_IDLE_EDGE is set 1 delayed by half cycle else delayed by one cycle 2: if SPI_CK_OUT_EDGE or SPI_CK_IDLE_EDGE is set 1 delayed by one cycle, else delayed by half cycle 3: delayed one cycle. Can be configured in CONF state."]
pub type CS_DELAY_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CS_DELAY_NUM` reader - spi_cs signal is delayed by system clock cycles. Can be configured in CONF state."]
pub type CS_DELAY_NUM_R = crate::FieldReader;
#[doc = "Field `CS_DELAY_NUM` writer - spi_cs signal is delayed by system clock cycles. Can be configured in CONF state."]
pub type CS_DELAY_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:12 - (cycles+1) of prepare phase by spi clock this bits are combined with SPI_CS_SETUP bit. Can be configured in CONF state."]
    #[inline(always)]
    pub fn cs_setup_time(&self) -> CS_SETUP_TIME_R {
        CS_SETUP_TIME_R::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bits 13:25 - delay cycles of cs pin by spi clock this bits are combined with SPI_CS_HOLD bit. Can be configured in CONF state."]
    #[inline(always)]
    pub fn cs_hold_time(&self) -> CS_HOLD_TIME_R {
        CS_HOLD_TIME_R::new(((self.bits >> 13) & 0x1fff) as u16)
    }
    #[doc = "Bits 26:28 - spi_cs signal is delayed by spi_clk . 0: zero 1: if SPI_CK_OUT_EDGE or SPI_CK_IDLE_EDGE is set 1 delayed by half cycle else delayed by one cycle 2: if SPI_CK_OUT_EDGE or SPI_CK_IDLE_EDGE is set 1 delayed by one cycle, else delayed by half cycle 3: delayed one cycle. Can be configured in CONF state."]
    #[inline(always)]
    pub fn cs_delay_mode(&self) -> CS_DELAY_MODE_R {
        CS_DELAY_MODE_R::new(((self.bits >> 26) & 7) as u8)
    }
    #[doc = "Bits 29:30 - spi_cs signal is delayed by system clock cycles. Can be configured in CONF state."]
    #[inline(always)]
    pub fn cs_delay_num(&self) -> CS_DELAY_NUM_R {
        CS_DELAY_NUM_R::new(((self.bits >> 29) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL2")
            .field(
                "cs_setup_time",
                &format_args!("{}", self.cs_setup_time().bits()),
            )
            .field(
                "cs_hold_time",
                &format_args!("{}", self.cs_hold_time().bits()),
            )
            .field(
                "cs_delay_mode",
                &format_args!("{}", self.cs_delay_mode().bits()),
            )
            .field(
                "cs_delay_num",
                &format_args!("{}", self.cs_delay_num().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CTRL2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:12 - (cycles+1) of prepare phase by spi clock this bits are combined with SPI_CS_SETUP bit. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn cs_setup_time(&mut self) -> CS_SETUP_TIME_W<CTRL2_SPEC> {
        CS_SETUP_TIME_W::new(self, 0)
    }
    #[doc = "Bits 13:25 - delay cycles of cs pin by spi clock this bits are combined with SPI_CS_HOLD bit. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn cs_hold_time(&mut self) -> CS_HOLD_TIME_W<CTRL2_SPEC> {
        CS_HOLD_TIME_W::new(self, 13)
    }
    #[doc = "Bits 26:28 - spi_cs signal is delayed by spi_clk . 0: zero 1: if SPI_CK_OUT_EDGE or SPI_CK_IDLE_EDGE is set 1 delayed by half cycle else delayed by one cycle 2: if SPI_CK_OUT_EDGE or SPI_CK_IDLE_EDGE is set 1 delayed by one cycle, else delayed by half cycle 3: delayed one cycle. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn cs_delay_mode(&mut self) -> CS_DELAY_MODE_W<CTRL2_SPEC> {
        CS_DELAY_MODE_W::new(self, 26)
    }
    #[doc = "Bits 29:30 - spi_cs signal is delayed by system clock cycles. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn cs_delay_num(&mut self) -> CS_DELAY_NUM_W<CTRL2_SPEC> {
        CS_DELAY_NUM_W::new(self, 29)
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
#[doc = "SPI control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRL2_SPEC;
impl crate::RegisterSpec for CTRL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl2::R`](R) reader structure"]
impl crate::Readable for CTRL2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrl2::W`](W) writer structure"]
impl crate::Writable for CTRL2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL2 to value 0x2000"]
impl crate::Resettable for CTRL2_SPEC {
    const RESET_VALUE: u32 = 0x2000;
}
