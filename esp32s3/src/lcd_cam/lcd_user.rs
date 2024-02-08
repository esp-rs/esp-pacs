#[doc = "Register `LCD_USER` reader"]
pub type R = crate::R<LCD_USER_SPEC>;
#[doc = "Register `LCD_USER` writer"]
pub type W = crate::W<LCD_USER_SPEC>;
#[doc = "Field `LCD_DOUT_CYCLELEN` reader - Configure the cycles for DOUT phase of LCD module. The cycles = this value + 1."]
pub type LCD_DOUT_CYCLELEN_R = crate::FieldReader<u16>;
#[doc = "Field `LCD_DOUT_CYCLELEN` writer - Configure the cycles for DOUT phase of LCD module. The cycles = this value + 1."]
pub type LCD_DOUT_CYCLELEN_W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
#[doc = "Field `LCD_ALWAYS_OUT_EN` reader - LCD continues outputting data when LCD is in DOUT phase, till LCD_CAM_LCD_START is cleared or LCD_CAM_LCD_RESET is set."]
pub type LCD_ALWAYS_OUT_EN_R = crate::BitReader;
#[doc = "Field `LCD_ALWAYS_OUT_EN` writer - LCD continues outputting data when LCD is in DOUT phase, till LCD_CAM_LCD_START is cleared or LCD_CAM_LCD_RESET is set."]
pub type LCD_ALWAYS_OUT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCD_8BITS_ORDER` reader - 1: Swap every two data bytes, valid in 8-bit mode. 0: Do not swap."]
pub type LCD_8BITS_ORDER_R = crate::BitReader;
#[doc = "Field `LCD_8BITS_ORDER` writer - 1: Swap every two data bytes, valid in 8-bit mode. 0: Do not swap."]
pub type LCD_8BITS_ORDER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCD_UPDATE` reader - 1: Update LCD registers. This bit is cleared by hardware. 0: Do not care."]
pub type LCD_UPDATE_R = crate::BitReader;
#[doc = "Field `LCD_UPDATE` writer - 1: Update LCD registers. This bit is cleared by hardware. 0: Do not care."]
pub type LCD_UPDATE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCD_BIT_ORDER` reader - 1: Change data bit order. Change LCD_DATA_out\\[7:0\\] to LCD_DATA_out\\[0:7\\] in 8-bit mode, and bits\\[15:0\\] to bits\\[0:15\\] in 16-bit mode. 0: Do not change."]
pub type LCD_BIT_ORDER_R = crate::BitReader;
#[doc = "Field `LCD_BIT_ORDER` writer - 1: Change data bit order. Change LCD_DATA_out\\[7:0\\] to LCD_DATA_out\\[0:7\\] in 8-bit mode, and bits\\[15:0\\] to bits\\[0:15\\] in 16-bit mode. 0: Do not change."]
pub type LCD_BIT_ORDER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCD_BYTE_ORDER` reader - 1: Invert data byte order, only valid in 16-bit mode. 0: Do not invert."]
pub type LCD_BYTE_ORDER_R = crate::BitReader;
#[doc = "Field `LCD_BYTE_ORDER` writer - 1: Invert data byte order, only valid in 16-bit mode. 0: Do not invert."]
pub type LCD_BYTE_ORDER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCD_2BYTE_EN` reader - 1: The width of output LCD data is 16 bits. 0: The width of output LCD data is 8 bits."]
pub type LCD_2BYTE_EN_R = crate::BitReader;
#[doc = "Field `LCD_2BYTE_EN` writer - 1: The width of output LCD data is 16 bits. 0: The width of output LCD data is 8 bits."]
pub type LCD_2BYTE_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCD_DOUT` reader - 1: Be able to send data out in LCD sequence when LCD starts. 0: Disable."]
pub type LCD_DOUT_R = crate::BitReader;
#[doc = "Field `LCD_DOUT` writer - 1: Be able to send data out in LCD sequence when LCD starts. 0: Disable."]
pub type LCD_DOUT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCD_DUMMY` reader - 1: Enable DUMMY phase in LCD sequence when LCD starts. 0: Disable."]
pub type LCD_DUMMY_R = crate::BitReader;
#[doc = "Field `LCD_DUMMY` writer - 1: Enable DUMMY phase in LCD sequence when LCD starts. 0: Disable."]
pub type LCD_DUMMY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCD_CMD` reader - 1: Be able to send command in LCD sequence when LCD starts. 0: Disable."]
pub type LCD_CMD_R = crate::BitReader;
#[doc = "Field `LCD_CMD` writer - 1: Be able to send command in LCD sequence when LCD starts. 0: Disable."]
pub type LCD_CMD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCD_START` reader - LCD starts sending data enable signal, valid in high level."]
pub type LCD_START_R = crate::BitReader;
#[doc = "Field `LCD_START` writer - LCD starts sending data enable signal, valid in high level."]
pub type LCD_START_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCD_RESET` writer - Reset LCD module."]
pub type LCD_RESET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCD_DUMMY_CYCLELEN` reader - Configure DUMMY cycles. DUMMY cycles = this value + 1."]
pub type LCD_DUMMY_CYCLELEN_R = crate::FieldReader;
#[doc = "Field `LCD_DUMMY_CYCLELEN` writer - Configure DUMMY cycles. DUMMY cycles = this value + 1."]
pub type LCD_DUMMY_CYCLELEN_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LCD_CMD_2_CYCLE_EN` reader - The cycle length of command phase. 1: 2 cycles. 0: 1 cycle."]
pub type LCD_CMD_2_CYCLE_EN_R = crate::BitReader;
#[doc = "Field `LCD_CMD_2_CYCLE_EN` writer - The cycle length of command phase. 1: 2 cycles. 0: 1 cycle."]
pub type LCD_CMD_2_CYCLE_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:12 - Configure the cycles for DOUT phase of LCD module. The cycles = this value + 1."]
    #[inline(always)]
    pub fn lcd_dout_cyclelen(&self) -> LCD_DOUT_CYCLELEN_R {
        LCD_DOUT_CYCLELEN_R::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bit 13 - LCD continues outputting data when LCD is in DOUT phase, till LCD_CAM_LCD_START is cleared or LCD_CAM_LCD_RESET is set."]
    #[inline(always)]
    pub fn lcd_always_out_en(&self) -> LCD_ALWAYS_OUT_EN_R {
        LCD_ALWAYS_OUT_EN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 19 - 1: Swap every two data bytes, valid in 8-bit mode. 0: Do not swap."]
    #[inline(always)]
    pub fn lcd_8bits_order(&self) -> LCD_8BITS_ORDER_R {
        LCD_8BITS_ORDER_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - 1: Update LCD registers. This bit is cleared by hardware. 0: Do not care."]
    #[inline(always)]
    pub fn lcd_update(&self) -> LCD_UPDATE_R {
        LCD_UPDATE_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - 1: Change data bit order. Change LCD_DATA_out\\[7:0\\] to LCD_DATA_out\\[0:7\\] in 8-bit mode, and bits\\[15:0\\] to bits\\[0:15\\] in 16-bit mode. 0: Do not change."]
    #[inline(always)]
    pub fn lcd_bit_order(&self) -> LCD_BIT_ORDER_R {
        LCD_BIT_ORDER_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - 1: Invert data byte order, only valid in 16-bit mode. 0: Do not invert."]
    #[inline(always)]
    pub fn lcd_byte_order(&self) -> LCD_BYTE_ORDER_R {
        LCD_BYTE_ORDER_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - 1: The width of output LCD data is 16 bits. 0: The width of output LCD data is 8 bits."]
    #[inline(always)]
    pub fn lcd_2byte_en(&self) -> LCD_2BYTE_EN_R {
        LCD_2BYTE_EN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - 1: Be able to send data out in LCD sequence when LCD starts. 0: Disable."]
    #[inline(always)]
    pub fn lcd_dout(&self) -> LCD_DOUT_R {
        LCD_DOUT_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 1: Enable DUMMY phase in LCD sequence when LCD starts. 0: Disable."]
    #[inline(always)]
    pub fn lcd_dummy(&self) -> LCD_DUMMY_R {
        LCD_DUMMY_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - 1: Be able to send command in LCD sequence when LCD starts. 0: Disable."]
    #[inline(always)]
    pub fn lcd_cmd(&self) -> LCD_CMD_R {
        LCD_CMD_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - LCD starts sending data enable signal, valid in high level."]
    #[inline(always)]
    pub fn lcd_start(&self) -> LCD_START_R {
        LCD_START_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 29:30 - Configure DUMMY cycles. DUMMY cycles = this value + 1."]
    #[inline(always)]
    pub fn lcd_dummy_cyclelen(&self) -> LCD_DUMMY_CYCLELEN_R {
        LCD_DUMMY_CYCLELEN_R::new(((self.bits >> 29) & 3) as u8)
    }
    #[doc = "Bit 31 - The cycle length of command phase. 1: 2 cycles. 0: 1 cycle."]
    #[inline(always)]
    pub fn lcd_cmd_2_cycle_en(&self) -> LCD_CMD_2_CYCLE_EN_R {
        LCD_CMD_2_CYCLE_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LCD_USER")
            .field(
                "lcd_dout_cyclelen",
                &format_args!("{}", self.lcd_dout_cyclelen().bits()),
            )
            .field(
                "lcd_always_out_en",
                &format_args!("{}", self.lcd_always_out_en().bit()),
            )
            .field(
                "lcd_8bits_order",
                &format_args!("{}", self.lcd_8bits_order().bit()),
            )
            .field("lcd_update", &format_args!("{}", self.lcd_update().bit()))
            .field(
                "lcd_bit_order",
                &format_args!("{}", self.lcd_bit_order().bit()),
            )
            .field(
                "lcd_byte_order",
                &format_args!("{}", self.lcd_byte_order().bit()),
            )
            .field(
                "lcd_2byte_en",
                &format_args!("{}", self.lcd_2byte_en().bit()),
            )
            .field("lcd_dout", &format_args!("{}", self.lcd_dout().bit()))
            .field("lcd_dummy", &format_args!("{}", self.lcd_dummy().bit()))
            .field("lcd_cmd", &format_args!("{}", self.lcd_cmd().bit()))
            .field("lcd_start", &format_args!("{}", self.lcd_start().bit()))
            .field(
                "lcd_dummy_cyclelen",
                &format_args!("{}", self.lcd_dummy_cyclelen().bits()),
            )
            .field(
                "lcd_cmd_2_cycle_en",
                &format_args!("{}", self.lcd_cmd_2_cycle_en().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<LCD_USER_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:12 - Configure the cycles for DOUT phase of LCD module. The cycles = this value + 1."]
    #[inline(always)]
    #[must_use]
    pub fn lcd_dout_cyclelen(&mut self) -> LCD_DOUT_CYCLELEN_W<LCD_USER_SPEC> {
        LCD_DOUT_CYCLELEN_W::new(self, 0)
    }
    #[doc = "Bit 13 - LCD continues outputting data when LCD is in DOUT phase, till LCD_CAM_LCD_START is cleared or LCD_CAM_LCD_RESET is set."]
    #[inline(always)]
    #[must_use]
    pub fn lcd_always_out_en(&mut self) -> LCD_ALWAYS_OUT_EN_W<LCD_USER_SPEC> {
        LCD_ALWAYS_OUT_EN_W::new(self, 13)
    }
    #[doc = "Bit 19 - 1: Swap every two data bytes, valid in 8-bit mode. 0: Do not swap."]
    #[inline(always)]
    #[must_use]
    pub fn lcd_8bits_order(&mut self) -> LCD_8BITS_ORDER_W<LCD_USER_SPEC> {
        LCD_8BITS_ORDER_W::new(self, 19)
    }
    #[doc = "Bit 20 - 1: Update LCD registers. This bit is cleared by hardware. 0: Do not care."]
    #[inline(always)]
    #[must_use]
    pub fn lcd_update(&mut self) -> LCD_UPDATE_W<LCD_USER_SPEC> {
        LCD_UPDATE_W::new(self, 20)
    }
    #[doc = "Bit 21 - 1: Change data bit order. Change LCD_DATA_out\\[7:0\\] to LCD_DATA_out\\[0:7\\] in 8-bit mode, and bits\\[15:0\\] to bits\\[0:15\\] in 16-bit mode. 0: Do not change."]
    #[inline(always)]
    #[must_use]
    pub fn lcd_bit_order(&mut self) -> LCD_BIT_ORDER_W<LCD_USER_SPEC> {
        LCD_BIT_ORDER_W::new(self, 21)
    }
    #[doc = "Bit 22 - 1: Invert data byte order, only valid in 16-bit mode. 0: Do not invert."]
    #[inline(always)]
    #[must_use]
    pub fn lcd_byte_order(&mut self) -> LCD_BYTE_ORDER_W<LCD_USER_SPEC> {
        LCD_BYTE_ORDER_W::new(self, 22)
    }
    #[doc = "Bit 23 - 1: The width of output LCD data is 16 bits. 0: The width of output LCD data is 8 bits."]
    #[inline(always)]
    #[must_use]
    pub fn lcd_2byte_en(&mut self) -> LCD_2BYTE_EN_W<LCD_USER_SPEC> {
        LCD_2BYTE_EN_W::new(self, 23)
    }
    #[doc = "Bit 24 - 1: Be able to send data out in LCD sequence when LCD starts. 0: Disable."]
    #[inline(always)]
    #[must_use]
    pub fn lcd_dout(&mut self) -> LCD_DOUT_W<LCD_USER_SPEC> {
        LCD_DOUT_W::new(self, 24)
    }
    #[doc = "Bit 25 - 1: Enable DUMMY phase in LCD sequence when LCD starts. 0: Disable."]
    #[inline(always)]
    #[must_use]
    pub fn lcd_dummy(&mut self) -> LCD_DUMMY_W<LCD_USER_SPEC> {
        LCD_DUMMY_W::new(self, 25)
    }
    #[doc = "Bit 26 - 1: Be able to send command in LCD sequence when LCD starts. 0: Disable."]
    #[inline(always)]
    #[must_use]
    pub fn lcd_cmd(&mut self) -> LCD_CMD_W<LCD_USER_SPEC> {
        LCD_CMD_W::new(self, 26)
    }
    #[doc = "Bit 27 - LCD starts sending data enable signal, valid in high level."]
    #[inline(always)]
    #[must_use]
    pub fn lcd_start(&mut self) -> LCD_START_W<LCD_USER_SPEC> {
        LCD_START_W::new(self, 27)
    }
    #[doc = "Bit 28 - Reset LCD module."]
    #[inline(always)]
    #[must_use]
    pub fn lcd_reset(&mut self) -> LCD_RESET_W<LCD_USER_SPEC> {
        LCD_RESET_W::new(self, 28)
    }
    #[doc = "Bits 29:30 - Configure DUMMY cycles. DUMMY cycles = this value + 1."]
    #[inline(always)]
    #[must_use]
    pub fn lcd_dummy_cyclelen(&mut self) -> LCD_DUMMY_CYCLELEN_W<LCD_USER_SPEC> {
        LCD_DUMMY_CYCLELEN_W::new(self, 29)
    }
    #[doc = "Bit 31 - The cycle length of command phase. 1: 2 cycles. 0: 1 cycle."]
    #[inline(always)]
    #[must_use]
    pub fn lcd_cmd_2_cycle_en(&mut self) -> LCD_CMD_2_CYCLE_EN_W<LCD_USER_SPEC> {
        LCD_CMD_2_CYCLE_EN_W::new(self, 31)
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
#[doc = "LCD user configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lcd_user::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcd_user::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LCD_USER_SPEC;
impl crate::RegisterSpec for LCD_USER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcd_user::R`](R) reader structure"]
impl crate::Readable for LCD_USER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lcd_user::W`](W) writer structure"]
impl crate::Writable for LCD_USER_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LCD_USER to value 0"]
impl crate::Resettable for LCD_USER_SPEC {
    const RESET_VALUE: u32 = 0;
}
