#[doc = "Register `LCD_USER` reader"]
pub type R = crate::R<LCD_USER_SPEC>;
#[doc = "Register `LCD_USER` writer"]
pub type W = crate::W<LCD_USER_SPEC>;
#[doc = "Field `LCD_DOUT_CYCLELEN` reader - The output data cycles minus 1 of LCD module."]
pub type LCD_DOUT_CYCLELEN_R = crate::FieldReader<u16>;
#[doc = "Field `LCD_DOUT_CYCLELEN` writer - The output data cycles minus 1 of LCD module."]
pub type LCD_DOUT_CYCLELEN_W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
#[doc = "Field `LCD_ALWAYS_OUT_EN` reader - LCD always output when LCD is in LCD_DOUT state, unless reg_lcd_start is cleared or reg_lcd_reset is set."]
pub type LCD_ALWAYS_OUT_EN_R = crate::BitReader;
#[doc = "Field `LCD_ALWAYS_OUT_EN` writer - LCD always output when LCD is in LCD_DOUT state, unless reg_lcd_start is cleared or reg_lcd_reset is set."]
pub type LCD_ALWAYS_OUT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCD_DOUT_BYTE_SWIZZLE_MODE` reader - 0: ABAB->BABA. 1: ABC->ACB. 2: ABC->BAC. 3: ABC->BCA. 4:ABC->CAB. 5:ABC->CBA"]
pub type LCD_DOUT_BYTE_SWIZZLE_MODE_R = crate::FieldReader;
#[doc = "Field `LCD_DOUT_BYTE_SWIZZLE_MODE` writer - 0: ABAB->BABA. 1: ABC->ACB. 2: ABC->BAC. 3: ABC->BCA. 4:ABC->CAB. 5:ABC->CBA"]
pub type LCD_DOUT_BYTE_SWIZZLE_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `LCD_DOUT_BYTE_SWIZZLE_ENABLE` reader - 1: enable byte swizzle 0: disable"]
pub type LCD_DOUT_BYTE_SWIZZLE_ENABLE_R = crate::BitReader;
#[doc = "Field `LCD_DOUT_BYTE_SWIZZLE_ENABLE` writer - 1: enable byte swizzle 0: disable"]
pub type LCD_DOUT_BYTE_SWIZZLE_ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCD_DOUT_BIT_ORDER` reader - 1: change bit order in every byte. 0: Not change."]
pub type LCD_DOUT_BIT_ORDER_R = crate::BitReader;
#[doc = "Field `LCD_DOUT_BIT_ORDER` writer - 1: change bit order in every byte. 0: Not change."]
pub type LCD_DOUT_BIT_ORDER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCD_BYTE_MODE` reader - 2: 24bit mode. 1: 16bit mode. 0: 8bit mode"]
pub type LCD_BYTE_MODE_R = crate::FieldReader;
#[doc = "Field `LCD_BYTE_MODE` writer - 2: 24bit mode. 1: 16bit mode. 0: 8bit mode"]
pub type LCD_BYTE_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LCD_UPDATE` reader - 1: Update LCD registers, will be cleared by hardware. 0 : Not care."]
pub type LCD_UPDATE_R = crate::BitReader;
#[doc = "Field `LCD_UPDATE` writer - 1: Update LCD registers, will be cleared by hardware. 0 : Not care."]
pub type LCD_UPDATE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCD_BIT_ORDER` reader - 1: Change data bit order, change LCD_DATA_out\\[7:0\\] to LCD_DATA_out\\[0:7\\] in one byte mode, and bits\\[15:0\\] to bits\\[0:15\\] in two byte mode. 0: Not change."]
pub type LCD_BIT_ORDER_R = crate::BitReader;
#[doc = "Field `LCD_BIT_ORDER` writer - 1: Change data bit order, change LCD_DATA_out\\[7:0\\] to LCD_DATA_out\\[0:7\\] in one byte mode, and bits\\[15:0\\] to bits\\[0:15\\] in two byte mode. 0: Not change."]
pub type LCD_BIT_ORDER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCD_BYTE_ORDER` reader - 1: invert data byte order, only valid in 2 byte mode. 0: Not change."]
pub type LCD_BYTE_ORDER_R = crate::BitReader;
#[doc = "Field `LCD_BYTE_ORDER` writer - 1: invert data byte order, only valid in 2 byte mode. 0: Not change."]
pub type LCD_BYTE_ORDER_W<'a, REG> = crate::BitWriter<'a, REG>;
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
#[doc = "Field `LCD_START` reader - LCD start sending data enable signal, valid in high level."]
pub type LCD_START_R = crate::BitReader;
#[doc = "Field `LCD_START` writer - LCD start sending data enable signal, valid in high level."]
pub type LCD_START_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCD_RESET` writer - The value of command."]
pub type LCD_RESET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCD_DUMMY_CYCLELEN` reader - The dummy cycle length minus 1."]
pub type LCD_DUMMY_CYCLELEN_R = crate::FieldReader;
#[doc = "Field `LCD_DUMMY_CYCLELEN` writer - The dummy cycle length minus 1."]
pub type LCD_DUMMY_CYCLELEN_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LCD_CMD_2_CYCLE_EN` reader - The cycle length of command phase. 1: 2 cycles. 0: 1 cycle."]
pub type LCD_CMD_2_CYCLE_EN_R = crate::BitReader;
#[doc = "Field `LCD_CMD_2_CYCLE_EN` writer - The cycle length of command phase. 1: 2 cycles. 0: 1 cycle."]
pub type LCD_CMD_2_CYCLE_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:12 - The output data cycles minus 1 of LCD module."]
    #[inline(always)]
    pub fn lcd_dout_cyclelen(&self) -> LCD_DOUT_CYCLELEN_R {
        LCD_DOUT_CYCLELEN_R::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bit 13 - LCD always output when LCD is in LCD_DOUT state, unless reg_lcd_start is cleared or reg_lcd_reset is set."]
    #[inline(always)]
    pub fn lcd_always_out_en(&self) -> LCD_ALWAYS_OUT_EN_R {
        LCD_ALWAYS_OUT_EN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:16 - 0: ABAB->BABA. 1: ABC->ACB. 2: ABC->BAC. 3: ABC->BCA. 4:ABC->CAB. 5:ABC->CBA"]
    #[inline(always)]
    pub fn lcd_dout_byte_swizzle_mode(&self) -> LCD_DOUT_BYTE_SWIZZLE_MODE_R {
        LCD_DOUT_BYTE_SWIZZLE_MODE_R::new(((self.bits >> 14) & 7) as u8)
    }
    #[doc = "Bit 17 - 1: enable byte swizzle 0: disable"]
    #[inline(always)]
    pub fn lcd_dout_byte_swizzle_enable(&self) -> LCD_DOUT_BYTE_SWIZZLE_ENABLE_R {
        LCD_DOUT_BYTE_SWIZZLE_ENABLE_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 1: change bit order in every byte. 0: Not change."]
    #[inline(always)]
    pub fn lcd_dout_bit_order(&self) -> LCD_DOUT_BIT_ORDER_R {
        LCD_DOUT_BIT_ORDER_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:20 - 2: 24bit mode. 1: 16bit mode. 0: 8bit mode"]
    #[inline(always)]
    pub fn lcd_byte_mode(&self) -> LCD_BYTE_MODE_R {
        LCD_BYTE_MODE_R::new(((self.bits >> 19) & 3) as u8)
    }
    #[doc = "Bit 21 - 1: Update LCD registers, will be cleared by hardware. 0 : Not care."]
    #[inline(always)]
    pub fn lcd_update(&self) -> LCD_UPDATE_R {
        LCD_UPDATE_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - 1: Change data bit order, change LCD_DATA_out\\[7:0\\] to LCD_DATA_out\\[0:7\\] in one byte mode, and bits\\[15:0\\] to bits\\[0:15\\] in two byte mode. 0: Not change."]
    #[inline(always)]
    pub fn lcd_bit_order(&self) -> LCD_BIT_ORDER_R {
        LCD_BIT_ORDER_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - 1: invert data byte order, only valid in 2 byte mode. 0: Not change."]
    #[inline(always)]
    pub fn lcd_byte_order(&self) -> LCD_BYTE_ORDER_R {
        LCD_BYTE_ORDER_R::new(((self.bits >> 23) & 1) != 0)
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
    #[doc = "Bit 27 - LCD start sending data enable signal, valid in high level."]
    #[inline(always)]
    pub fn lcd_start(&self) -> LCD_START_R {
        LCD_START_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 29:30 - The dummy cycle length minus 1."]
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
            .field("lcd_dout_cyclelen", &self.lcd_dout_cyclelen().bits())
            .field("lcd_always_out_en", &self.lcd_always_out_en().bit())
            .field(
                "lcd_dout_byte_swizzle_mode",
                &self.lcd_dout_byte_swizzle_mode().bits(),
            )
            .field(
                "lcd_dout_byte_swizzle_enable",
                &self.lcd_dout_byte_swizzle_enable().bit(),
            )
            .field("lcd_dout_bit_order", &self.lcd_dout_bit_order().bit())
            .field("lcd_byte_mode", &self.lcd_byte_mode().bits())
            .field("lcd_update", &self.lcd_update().bit())
            .field("lcd_bit_order", &self.lcd_bit_order().bit())
            .field("lcd_byte_order", &self.lcd_byte_order().bit())
            .field("lcd_dout", &self.lcd_dout().bit())
            .field("lcd_dummy", &self.lcd_dummy().bit())
            .field("lcd_cmd", &self.lcd_cmd().bit())
            .field("lcd_start", &self.lcd_start().bit())
            .field("lcd_dummy_cyclelen", &self.lcd_dummy_cyclelen().bits())
            .field("lcd_cmd_2_cycle_en", &self.lcd_cmd_2_cycle_en().bit())
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
    #[doc = "Bits 0:12 - The output data cycles minus 1 of LCD module."]
    #[inline(always)]
    #[must_use]
    pub fn lcd_dout_cyclelen(&mut self) -> LCD_DOUT_CYCLELEN_W<LCD_USER_SPEC> {
        LCD_DOUT_CYCLELEN_W::new(self, 0)
    }
    #[doc = "Bit 13 - LCD always output when LCD is in LCD_DOUT state, unless reg_lcd_start is cleared or reg_lcd_reset is set."]
    #[inline(always)]
    #[must_use]
    pub fn lcd_always_out_en(&mut self) -> LCD_ALWAYS_OUT_EN_W<LCD_USER_SPEC> {
        LCD_ALWAYS_OUT_EN_W::new(self, 13)
    }
    #[doc = "Bits 14:16 - 0: ABAB->BABA. 1: ABC->ACB. 2: ABC->BAC. 3: ABC->BCA. 4:ABC->CAB. 5:ABC->CBA"]
    #[inline(always)]
    #[must_use]
    pub fn lcd_dout_byte_swizzle_mode(&mut self) -> LCD_DOUT_BYTE_SWIZZLE_MODE_W<LCD_USER_SPEC> {
        LCD_DOUT_BYTE_SWIZZLE_MODE_W::new(self, 14)
    }
    #[doc = "Bit 17 - 1: enable byte swizzle 0: disable"]
    #[inline(always)]
    #[must_use]
    pub fn lcd_dout_byte_swizzle_enable(
        &mut self,
    ) -> LCD_DOUT_BYTE_SWIZZLE_ENABLE_W<LCD_USER_SPEC> {
        LCD_DOUT_BYTE_SWIZZLE_ENABLE_W::new(self, 17)
    }
    #[doc = "Bit 18 - 1: change bit order in every byte. 0: Not change."]
    #[inline(always)]
    #[must_use]
    pub fn lcd_dout_bit_order(&mut self) -> LCD_DOUT_BIT_ORDER_W<LCD_USER_SPEC> {
        LCD_DOUT_BIT_ORDER_W::new(self, 18)
    }
    #[doc = "Bits 19:20 - 2: 24bit mode. 1: 16bit mode. 0: 8bit mode"]
    #[inline(always)]
    #[must_use]
    pub fn lcd_byte_mode(&mut self) -> LCD_BYTE_MODE_W<LCD_USER_SPEC> {
        LCD_BYTE_MODE_W::new(self, 19)
    }
    #[doc = "Bit 21 - 1: Update LCD registers, will be cleared by hardware. 0 : Not care."]
    #[inline(always)]
    #[must_use]
    pub fn lcd_update(&mut self) -> LCD_UPDATE_W<LCD_USER_SPEC> {
        LCD_UPDATE_W::new(self, 21)
    }
    #[doc = "Bit 22 - 1: Change data bit order, change LCD_DATA_out\\[7:0\\] to LCD_DATA_out\\[0:7\\] in one byte mode, and bits\\[15:0\\] to bits\\[0:15\\] in two byte mode. 0: Not change."]
    #[inline(always)]
    #[must_use]
    pub fn lcd_bit_order(&mut self) -> LCD_BIT_ORDER_W<LCD_USER_SPEC> {
        LCD_BIT_ORDER_W::new(self, 22)
    }
    #[doc = "Bit 23 - 1: invert data byte order, only valid in 2 byte mode. 0: Not change."]
    #[inline(always)]
    #[must_use]
    pub fn lcd_byte_order(&mut self) -> LCD_BYTE_ORDER_W<LCD_USER_SPEC> {
        LCD_BYTE_ORDER_W::new(self, 23)
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
    #[doc = "Bit 27 - LCD start sending data enable signal, valid in high level."]
    #[inline(always)]
    #[must_use]
    pub fn lcd_start(&mut self) -> LCD_START_W<LCD_USER_SPEC> {
        LCD_START_W::new(self, 27)
    }
    #[doc = "Bit 28 - The value of command."]
    #[inline(always)]
    #[must_use]
    pub fn lcd_reset(&mut self) -> LCD_RESET_W<LCD_USER_SPEC> {
        LCD_RESET_W::new(self, 28)
    }
    #[doc = "Bits 29:30 - The dummy cycle length minus 1."]
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
}
#[doc = "LCD config register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lcd_user::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcd_user::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LCD_USER_SPEC;
impl crate::RegisterSpec for LCD_USER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcd_user::R`](R) reader structure"]
impl crate::Readable for LCD_USER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lcd_user::W`](W) writer structure"]
impl crate::Writable for LCD_USER_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LCD_USER to value 0x01"]
impl crate::Resettable for LCD_USER_SPEC {
    const RESET_VALUE: u32 = 0x01;
}
