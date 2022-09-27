#[doc = "Register `CTRL2` reader"]
pub struct R(crate::R<CTRL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL2` writer"]
pub struct W(crate::W<CTRL2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CTRL2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CS_SETUP_TIME` reader - (cycles+1) of prepare phase by spi clock this bits are combined with SPI_CS_SETUP bit. Can be configured in CONF state."]
pub type CS_SETUP_TIME_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CS_SETUP_TIME` writer - (cycles+1) of prepare phase by spi clock this bits are combined with SPI_CS_SETUP bit. Can be configured in CONF state."]
pub type CS_SETUP_TIME_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CTRL2_SPEC, u16, u16, 13, O>;
#[doc = "Field `CS_HOLD_TIME` reader - delay cycles of cs pin by spi clock this bits are combined with SPI_CS_HOLD bit. Can be configured in CONF state."]
pub type CS_HOLD_TIME_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CS_HOLD_TIME` writer - delay cycles of cs pin by spi clock this bits are combined with SPI_CS_HOLD bit. Can be configured in CONF state."]
pub type CS_HOLD_TIME_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL2_SPEC, u16, u16, 13, O>;
#[doc = "Field `CS_DELAY_MODE` reader - spi_cs signal is delayed by spi_clk . 0: zero 1: if SPI_CK_OUT_EDGE or SPI_CK_IDLE_EDGE is set 1 delayed by half cycle else delayed by one cycle 2: if SPI_CK_OUT_EDGE or SPI_CK_IDLE_EDGE is set 1 delayed by one cycle, else delayed by half cycle 3: delayed one cycle. Can be configured in CONF state."]
pub type CS_DELAY_MODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CS_DELAY_MODE` writer - spi_cs signal is delayed by spi_clk . 0: zero 1: if SPI_CK_OUT_EDGE or SPI_CK_IDLE_EDGE is set 1 delayed by half cycle else delayed by one cycle 2: if SPI_CK_OUT_EDGE or SPI_CK_IDLE_EDGE is set 1 delayed by one cycle, else delayed by half cycle 3: delayed one cycle. Can be configured in CONF state."]
pub type CS_DELAY_MODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL2_SPEC, u8, u8, 3, O>;
#[doc = "Field `CS_DELAY_NUM` reader - spi_cs signal is delayed by system clock cycles. Can be configured in CONF state."]
pub type CS_DELAY_NUM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CS_DELAY_NUM` writer - spi_cs signal is delayed by system clock cycles. Can be configured in CONF state."]
pub type CS_DELAY_NUM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL2_SPEC, u8, u8, 2, O>;
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
impl W {
    #[doc = "Bits 0:12 - (cycles+1) of prepare phase by spi clock this bits are combined with SPI_CS_SETUP bit. Can be configured in CONF state."]
    #[inline(always)]
    pub fn cs_setup_time(&mut self) -> CS_SETUP_TIME_W<0> {
        CS_SETUP_TIME_W::new(self)
    }
    #[doc = "Bits 13:25 - delay cycles of cs pin by spi clock this bits are combined with SPI_CS_HOLD bit. Can be configured in CONF state."]
    #[inline(always)]
    pub fn cs_hold_time(&mut self) -> CS_HOLD_TIME_W<13> {
        CS_HOLD_TIME_W::new(self)
    }
    #[doc = "Bits 26:28 - spi_cs signal is delayed by spi_clk . 0: zero 1: if SPI_CK_OUT_EDGE or SPI_CK_IDLE_EDGE is set 1 delayed by half cycle else delayed by one cycle 2: if SPI_CK_OUT_EDGE or SPI_CK_IDLE_EDGE is set 1 delayed by one cycle, else delayed by half cycle 3: delayed one cycle. Can be configured in CONF state."]
    #[inline(always)]
    pub fn cs_delay_mode(&mut self) -> CS_DELAY_MODE_W<26> {
        CS_DELAY_MODE_W::new(self)
    }
    #[doc = "Bits 29:30 - spi_cs signal is delayed by system clock cycles. Can be configured in CONF state."]
    #[inline(always)]
    pub fn cs_delay_num(&mut self) -> CS_DELAY_NUM_W<29> {
        CS_DELAY_NUM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI control register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl2](index.html) module"]
pub struct CTRL2_SPEC;
impl crate::RegisterSpec for CTRL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl2::R](R) reader structure"]
impl crate::Readable for CTRL2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl2::W](W) writer structure"]
impl crate::Writable for CTRL2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRL2 to value 0x2000"]
impl crate::Resettable for CTRL2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x2000
    }
}
