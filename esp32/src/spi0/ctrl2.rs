#[doc = "Register `CTRL2` reader"]
pub type R = crate::R<CTRL2_SPEC>;
#[doc = "Register `CTRL2` writer"]
pub type W = crate::W<CTRL2_SPEC>;
#[doc = "Field `SETUP_TIME` reader - (cycles-1) of ¡°prepare¡± phase by spi clock, this bits combined with spi_cs_setup bit."]
pub type SETUP_TIME_R = crate::FieldReader;
#[doc = "Field `SETUP_TIME` writer - (cycles-1) of ¡°prepare¡± phase by spi clock, this bits combined with spi_cs_setup bit."]
pub type SETUP_TIME_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `HOLD_TIME` reader - delay cycles of cs pin by spi clock, this bits combined with spi_cs_hold bit."]
pub type HOLD_TIME_R = crate::FieldReader;
#[doc = "Field `HOLD_TIME` writer - delay cycles of cs pin by spi clock, this bits combined with spi_cs_hold bit."]
pub type HOLD_TIME_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CK_OUT_LOW_MODE` reader - modify spi clock duty ratio when the value is lager than 8, the bits are combined with spi_clkcnt_N bits and spi_clkcnt_L bits."]
pub type CK_OUT_LOW_MODE_R = crate::FieldReader;
#[doc = "Field `CK_OUT_LOW_MODE` writer - modify spi clock duty ratio when the value is lager than 8, the bits are combined with spi_clkcnt_N bits and spi_clkcnt_L bits."]
pub type CK_OUT_LOW_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CK_OUT_HIGH_MODE` reader - modify spi clock duty ratio when the value is lager than 8, the bits are combined with spi_clkcnt_N bits and spi_clkcnt_H bits."]
pub type CK_OUT_HIGH_MODE_R = crate::FieldReader;
#[doc = "Field `CK_OUT_HIGH_MODE` writer - modify spi clock duty ratio when the value is lager than 8, the bits are combined with spi_clkcnt_N bits and spi_clkcnt_H bits."]
pub type CK_OUT_HIGH_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `MISO_DELAY_MODE` reader - MISO signals are delayed by spi_clk. 0: zero 1: if spi_ck_out_edge or spi_ck_i_edge is set 1 delayed by half cycle else delayed by one cycle 2: if spi_ck_out_edge or spi_ck_i_edge is set 1 delayed by one cycle else delayed by half cycle 3: delayed one cycle"]
pub type MISO_DELAY_MODE_R = crate::FieldReader;
#[doc = "Field `MISO_DELAY_MODE` writer - MISO signals are delayed by spi_clk. 0: zero 1: if spi_ck_out_edge or spi_ck_i_edge is set 1 delayed by half cycle else delayed by one cycle 2: if spi_ck_out_edge or spi_ck_i_edge is set 1 delayed by one cycle else delayed by half cycle 3: delayed one cycle"]
pub type MISO_DELAY_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MISO_DELAY_NUM` reader - MISO signals are delayed by system clock cycles"]
pub type MISO_DELAY_NUM_R = crate::FieldReader;
#[doc = "Field `MISO_DELAY_NUM` writer - MISO signals are delayed by system clock cycles"]
pub type MISO_DELAY_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `MOSI_DELAY_MODE` reader - MOSI signals are delayed by spi_clk. 0: zero 1: if spi_ck_out_edge or spi_ck_i_edge is set 1 delayed by half cycle else delayed by one cycle 2: if spi_ck_out_edge or spi_ck_i_edge is set 1 delayed by one cycle else delayed by half cycle 3: delayed one cycle"]
pub type MOSI_DELAY_MODE_R = crate::FieldReader;
#[doc = "Field `MOSI_DELAY_MODE` writer - MOSI signals are delayed by spi_clk. 0: zero 1: if spi_ck_out_edge or spi_ck_i_edge is set 1 delayed by half cycle else delayed by one cycle 2: if spi_ck_out_edge or spi_ck_i_edge is set 1 delayed by one cycle else delayed by half cycle 3: delayed one cycle"]
pub type MOSI_DELAY_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MOSI_DELAY_NUM` reader - MOSI signals are delayed by system clock cycles"]
pub type MOSI_DELAY_NUM_R = crate::FieldReader;
#[doc = "Field `MOSI_DELAY_NUM` writer - MOSI signals are delayed by system clock cycles"]
pub type MOSI_DELAY_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CS_DELAY_MODE` reader - spi_cs signal is delayed by spi_clk . 0: zero 1: if spi_ck_out_edge or spi_ck_i_edge is set 1 delayed by half cycle else delayed by one cycle 2: if spi_ck_out_edge or spi_ck_i_edge is set 1 delayed by one cycle else delayed by half cycle 3: delayed one cycle"]
pub type CS_DELAY_MODE_R = crate::FieldReader;
#[doc = "Field `CS_DELAY_MODE` writer - spi_cs signal is delayed by spi_clk . 0: zero 1: if spi_ck_out_edge or spi_ck_i_edge is set 1 delayed by half cycle else delayed by one cycle 2: if spi_ck_out_edge or spi_ck_i_edge is set 1 delayed by one cycle else delayed by half cycle 3: delayed one cycle"]
pub type CS_DELAY_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CS_DELAY_NUM` reader - spi_cs signal is delayed by system clock cycles"]
pub type CS_DELAY_NUM_R = crate::FieldReader;
#[doc = "Field `CS_DELAY_NUM` writer - spi_cs signal is delayed by system clock cycles"]
pub type CS_DELAY_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - (cycles-1) of ¡°prepare¡± phase by spi clock, this bits combined with spi_cs_setup bit."]
    #[inline(always)]
    pub fn setup_time(&self) -> SETUP_TIME_R {
        SETUP_TIME_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - delay cycles of cs pin by spi clock, this bits combined with spi_cs_hold bit."]
    #[inline(always)]
    pub fn hold_time(&self) -> HOLD_TIME_R {
        HOLD_TIME_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - modify spi clock duty ratio when the value is lager than 8, the bits are combined with spi_clkcnt_N bits and spi_clkcnt_L bits."]
    #[inline(always)]
    pub fn ck_out_low_mode(&self) -> CK_OUT_LOW_MODE_R {
        CK_OUT_LOW_MODE_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - modify spi clock duty ratio when the value is lager than 8, the bits are combined with spi_clkcnt_N bits and spi_clkcnt_H bits."]
    #[inline(always)]
    pub fn ck_out_high_mode(&self) -> CK_OUT_HIGH_MODE_R {
        CK_OUT_HIGH_MODE_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:17 - MISO signals are delayed by spi_clk. 0: zero 1: if spi_ck_out_edge or spi_ck_i_edge is set 1 delayed by half cycle else delayed by one cycle 2: if spi_ck_out_edge or spi_ck_i_edge is set 1 delayed by one cycle else delayed by half cycle 3: delayed one cycle"]
    #[inline(always)]
    pub fn miso_delay_mode(&self) -> MISO_DELAY_MODE_R {
        MISO_DELAY_MODE_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:20 - MISO signals are delayed by system clock cycles"]
    #[inline(always)]
    pub fn miso_delay_num(&self) -> MISO_DELAY_NUM_R {
        MISO_DELAY_NUM_R::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bits 21:22 - MOSI signals are delayed by spi_clk. 0: zero 1: if spi_ck_out_edge or spi_ck_i_edge is set 1 delayed by half cycle else delayed by one cycle 2: if spi_ck_out_edge or spi_ck_i_edge is set 1 delayed by one cycle else delayed by half cycle 3: delayed one cycle"]
    #[inline(always)]
    pub fn mosi_delay_mode(&self) -> MOSI_DELAY_MODE_R {
        MOSI_DELAY_MODE_R::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bits 23:25 - MOSI signals are delayed by system clock cycles"]
    #[inline(always)]
    pub fn mosi_delay_num(&self) -> MOSI_DELAY_NUM_R {
        MOSI_DELAY_NUM_R::new(((self.bits >> 23) & 7) as u8)
    }
    #[doc = "Bits 26:27 - spi_cs signal is delayed by spi_clk . 0: zero 1: if spi_ck_out_edge or spi_ck_i_edge is set 1 delayed by half cycle else delayed by one cycle 2: if spi_ck_out_edge or spi_ck_i_edge is set 1 delayed by one cycle else delayed by half cycle 3: delayed one cycle"]
    #[inline(always)]
    pub fn cs_delay_mode(&self) -> CS_DELAY_MODE_R {
        CS_DELAY_MODE_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:31 - spi_cs signal is delayed by system clock cycles"]
    #[inline(always)]
    pub fn cs_delay_num(&self) -> CS_DELAY_NUM_R {
        CS_DELAY_NUM_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL2")
            .field("setup_time", &self.setup_time().bits())
            .field("hold_time", &self.hold_time().bits())
            .field("ck_out_low_mode", &self.ck_out_low_mode().bits())
            .field("ck_out_high_mode", &self.ck_out_high_mode().bits())
            .field("miso_delay_mode", &self.miso_delay_mode().bits())
            .field("miso_delay_num", &self.miso_delay_num().bits())
            .field("mosi_delay_mode", &self.mosi_delay_mode().bits())
            .field("mosi_delay_num", &self.mosi_delay_num().bits())
            .field("cs_delay_mode", &self.cs_delay_mode().bits())
            .field("cs_delay_num", &self.cs_delay_num().bits())
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
    #[doc = "Bits 0:3 - (cycles-1) of ¡°prepare¡± phase by spi clock, this bits combined with spi_cs_setup bit."]
    #[inline(always)]
    #[must_use]
    pub fn setup_time(&mut self) -> SETUP_TIME_W<CTRL2_SPEC> {
        SETUP_TIME_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - delay cycles of cs pin by spi clock, this bits combined with spi_cs_hold bit."]
    #[inline(always)]
    #[must_use]
    pub fn hold_time(&mut self) -> HOLD_TIME_W<CTRL2_SPEC> {
        HOLD_TIME_W::new(self, 4)
    }
    #[doc = "Bits 8:11 - modify spi clock duty ratio when the value is lager than 8, the bits are combined with spi_clkcnt_N bits and spi_clkcnt_L bits."]
    #[inline(always)]
    #[must_use]
    pub fn ck_out_low_mode(&mut self) -> CK_OUT_LOW_MODE_W<CTRL2_SPEC> {
        CK_OUT_LOW_MODE_W::new(self, 8)
    }
    #[doc = "Bits 12:15 - modify spi clock duty ratio when the value is lager than 8, the bits are combined with spi_clkcnt_N bits and spi_clkcnt_H bits."]
    #[inline(always)]
    #[must_use]
    pub fn ck_out_high_mode(&mut self) -> CK_OUT_HIGH_MODE_W<CTRL2_SPEC> {
        CK_OUT_HIGH_MODE_W::new(self, 12)
    }
    #[doc = "Bits 16:17 - MISO signals are delayed by spi_clk. 0: zero 1: if spi_ck_out_edge or spi_ck_i_edge is set 1 delayed by half cycle else delayed by one cycle 2: if spi_ck_out_edge or spi_ck_i_edge is set 1 delayed by one cycle else delayed by half cycle 3: delayed one cycle"]
    #[inline(always)]
    #[must_use]
    pub fn miso_delay_mode(&mut self) -> MISO_DELAY_MODE_W<CTRL2_SPEC> {
        MISO_DELAY_MODE_W::new(self, 16)
    }
    #[doc = "Bits 18:20 - MISO signals are delayed by system clock cycles"]
    #[inline(always)]
    #[must_use]
    pub fn miso_delay_num(&mut self) -> MISO_DELAY_NUM_W<CTRL2_SPEC> {
        MISO_DELAY_NUM_W::new(self, 18)
    }
    #[doc = "Bits 21:22 - MOSI signals are delayed by spi_clk. 0: zero 1: if spi_ck_out_edge or spi_ck_i_edge is set 1 delayed by half cycle else delayed by one cycle 2: if spi_ck_out_edge or spi_ck_i_edge is set 1 delayed by one cycle else delayed by half cycle 3: delayed one cycle"]
    #[inline(always)]
    #[must_use]
    pub fn mosi_delay_mode(&mut self) -> MOSI_DELAY_MODE_W<CTRL2_SPEC> {
        MOSI_DELAY_MODE_W::new(self, 21)
    }
    #[doc = "Bits 23:25 - MOSI signals are delayed by system clock cycles"]
    #[inline(always)]
    #[must_use]
    pub fn mosi_delay_num(&mut self) -> MOSI_DELAY_NUM_W<CTRL2_SPEC> {
        MOSI_DELAY_NUM_W::new(self, 23)
    }
    #[doc = "Bits 26:27 - spi_cs signal is delayed by spi_clk . 0: zero 1: if spi_ck_out_edge or spi_ck_i_edge is set 1 delayed by half cycle else delayed by one cycle 2: if spi_ck_out_edge or spi_ck_i_edge is set 1 delayed by one cycle else delayed by half cycle 3: delayed one cycle"]
    #[inline(always)]
    #[must_use]
    pub fn cs_delay_mode(&mut self) -> CS_DELAY_MODE_W<CTRL2_SPEC> {
        CS_DELAY_MODE_W::new(self, 26)
    }
    #[doc = "Bits 28:31 - spi_cs signal is delayed by system clock cycles"]
    #[inline(always)]
    #[must_use]
    pub fn cs_delay_num(&mut self) -> CS_DELAY_NUM_W<CTRL2_SPEC> {
        CS_DELAY_NUM_W::new(self, 28)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRL2_SPEC;
impl crate::RegisterSpec for CTRL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl2::R`](R) reader structure"]
impl crate::Readable for CTRL2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrl2::W`](W) writer structure"]
impl crate::Writable for CTRL2_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL2 to value 0x11"]
impl crate::Resettable for CTRL2_SPEC {
    const RESET_VALUE: u32 = 0x11;
}
