#[doc = "Register `LCD_USER` reader"]
pub struct R(crate::R<LCD_USER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LCD_USER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LCD_USER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LCD_USER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LCD_USER` writer"]
pub struct W(crate::W<LCD_USER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LCD_USER_SPEC>;
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
impl From<crate::W<LCD_USER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LCD_USER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LCD_DOUT_CYCLELEN` reader - The output data cycles minus 1 of LCD module."]
pub type LCD_DOUT_CYCLELEN_R = crate::FieldReader<u16>;
#[doc = "Field `LCD_DOUT_CYCLELEN` writer - The output data cycles minus 1 of LCD module."]
pub type LCD_DOUT_CYCLELEN_W<'a, const O: u8> = crate::FieldWriter<'a, LCD_USER_SPEC, 13, O, u16>;
#[doc = "Field `LCD_ALWAYS_OUT_EN` reader - LCD always output when LCD is in LCD_DOUT state, unless reg_lcd_start is cleared or reg_lcd_reset is set."]
pub type LCD_ALWAYS_OUT_EN_R = crate::BitReader;
#[doc = "Field `LCD_ALWAYS_OUT_EN` writer - LCD always output when LCD is in LCD_DOUT state, unless reg_lcd_start is cleared or reg_lcd_reset is set."]
pub type LCD_ALWAYS_OUT_EN_W<'a, const O: u8> = crate::BitWriter<'a, LCD_USER_SPEC, O>;
#[doc = "Field `LCD_8BITS_ORDER` reader - 1: invert every two data byte, valid in 1 byte mode. 0: Not change."]
pub type LCD_8BITS_ORDER_R = crate::BitReader;
#[doc = "Field `LCD_8BITS_ORDER` writer - 1: invert every two data byte, valid in 1 byte mode. 0: Not change."]
pub type LCD_8BITS_ORDER_W<'a, const O: u8> = crate::BitWriter<'a, LCD_USER_SPEC, O>;
#[doc = "Field `LCD_UPDATE` reader - 1: Update LCD registers, will be cleared by hardware. 0 : Not care."]
pub type LCD_UPDATE_R = crate::BitReader;
#[doc = "Field `LCD_UPDATE` writer - 1: Update LCD registers, will be cleared by hardware. 0 : Not care."]
pub type LCD_UPDATE_W<'a, const O: u8> = crate::BitWriter<'a, LCD_USER_SPEC, O>;
#[doc = "Field `LCD_BIT_ORDER` reader - 1: Change data bit order, change LCD_DATA_out\\[7:0\\] to LCD_DATA_out\\[0:7\\] in one byte mode, and bits\\[15:0\\] to bits\\[0:15\\] in two byte mode. 0: Not change."]
pub type LCD_BIT_ORDER_R = crate::BitReader;
#[doc = "Field `LCD_BIT_ORDER` writer - 1: Change data bit order, change LCD_DATA_out\\[7:0\\] to LCD_DATA_out\\[0:7\\] in one byte mode, and bits\\[15:0\\] to bits\\[0:15\\] in two byte mode. 0: Not change."]
pub type LCD_BIT_ORDER_W<'a, const O: u8> = crate::BitWriter<'a, LCD_USER_SPEC, O>;
#[doc = "Field `LCD_BYTE_ORDER` reader - 1: invert data byte order, only valid in 2 byte mode. 0: Not change."]
pub type LCD_BYTE_ORDER_R = crate::BitReader;
#[doc = "Field `LCD_BYTE_ORDER` writer - 1: invert data byte order, only valid in 2 byte mode. 0: Not change."]
pub type LCD_BYTE_ORDER_W<'a, const O: u8> = crate::BitWriter<'a, LCD_USER_SPEC, O>;
#[doc = "Field `LCD_2BYTE_EN` reader - 1: The bit number of output LCD data is 9~16. 0: The bit number of output LCD data is 0~8."]
pub type LCD_2BYTE_EN_R = crate::BitReader;
#[doc = "Field `LCD_2BYTE_EN` writer - 1: The bit number of output LCD data is 9~16. 0: The bit number of output LCD data is 0~8."]
pub type LCD_2BYTE_EN_W<'a, const O: u8> = crate::BitWriter<'a, LCD_USER_SPEC, O>;
#[doc = "Field `LCD_DOUT` reader - 1: Be able to send data out in LCD sequence when LCD starts. 0: Disable."]
pub type LCD_DOUT_R = crate::BitReader;
#[doc = "Field `LCD_DOUT` writer - 1: Be able to send data out in LCD sequence when LCD starts. 0: Disable."]
pub type LCD_DOUT_W<'a, const O: u8> = crate::BitWriter<'a, LCD_USER_SPEC, O>;
#[doc = "Field `LCD_DUMMY` reader - 1: Enable DUMMY phase in LCD sequence when LCD starts. 0: Disable."]
pub type LCD_DUMMY_R = crate::BitReader;
#[doc = "Field `LCD_DUMMY` writer - 1: Enable DUMMY phase in LCD sequence when LCD starts. 0: Disable."]
pub type LCD_DUMMY_W<'a, const O: u8> = crate::BitWriter<'a, LCD_USER_SPEC, O>;
#[doc = "Field `LCD_CMD` reader - 1: Be able to send command in LCD sequence when LCD starts. 0: Disable."]
pub type LCD_CMD_R = crate::BitReader;
#[doc = "Field `LCD_CMD` writer - 1: Be able to send command in LCD sequence when LCD starts. 0: Disable."]
pub type LCD_CMD_W<'a, const O: u8> = crate::BitWriter<'a, LCD_USER_SPEC, O>;
#[doc = "Field `LCD_START` reader - LCD start sending data enable signal, valid in high level."]
pub type LCD_START_R = crate::BitReader;
#[doc = "Field `LCD_START` writer - LCD start sending data enable signal, valid in high level."]
pub type LCD_START_W<'a, const O: u8> = crate::BitWriter<'a, LCD_USER_SPEC, O>;
#[doc = "Field `LCD_RESET` writer - The value of command."]
pub type LCD_RESET_W<'a, const O: u8> = crate::BitWriter<'a, LCD_USER_SPEC, O>;
#[doc = "Field `LCD_DUMMY_CYCLELEN` reader - The dummy cycle length minus 1."]
pub type LCD_DUMMY_CYCLELEN_R = crate::FieldReader;
#[doc = "Field `LCD_DUMMY_CYCLELEN` writer - The dummy cycle length minus 1."]
pub type LCD_DUMMY_CYCLELEN_W<'a, const O: u8> = crate::FieldWriter<'a, LCD_USER_SPEC, 2, O>;
#[doc = "Field `LCD_CMD_2_CYCLE_EN` reader - The cycle length of command phase. 1: 2 cycles. 0: 1 cycle."]
pub type LCD_CMD_2_CYCLE_EN_R = crate::BitReader;
#[doc = "Field `LCD_CMD_2_CYCLE_EN` writer - The cycle length of command phase. 1: 2 cycles. 0: 1 cycle."]
pub type LCD_CMD_2_CYCLE_EN_W<'a, const O: u8> = crate::BitWriter<'a, LCD_USER_SPEC, O>;
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
    #[doc = "Bit 19 - 1: invert every two data byte, valid in 1 byte mode. 0: Not change."]
    #[inline(always)]
    pub fn lcd_8bits_order(&self) -> LCD_8BITS_ORDER_R {
        LCD_8BITS_ORDER_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - 1: Update LCD registers, will be cleared by hardware. 0 : Not care."]
    #[inline(always)]
    pub fn lcd_update(&self) -> LCD_UPDATE_R {
        LCD_UPDATE_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - 1: Change data bit order, change LCD_DATA_out\\[7:0\\] to LCD_DATA_out\\[0:7\\] in one byte mode, and bits\\[15:0\\] to bits\\[0:15\\] in two byte mode. 0: Not change."]
    #[inline(always)]
    pub fn lcd_bit_order(&self) -> LCD_BIT_ORDER_R {
        LCD_BIT_ORDER_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - 1: invert data byte order, only valid in 2 byte mode. 0: Not change."]
    #[inline(always)]
    pub fn lcd_byte_order(&self) -> LCD_BYTE_ORDER_R {
        LCD_BYTE_ORDER_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - 1: The bit number of output LCD data is 9~16. 0: The bit number of output LCD data is 0~8."]
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
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:12 - The output data cycles minus 1 of LCD module."]
    #[inline(always)]
    #[must_use]
    pub fn lcd_dout_cyclelen(&mut self) -> LCD_DOUT_CYCLELEN_W<0> {
        LCD_DOUT_CYCLELEN_W::new(self)
    }
    #[doc = "Bit 13 - LCD always output when LCD is in LCD_DOUT state, unless reg_lcd_start is cleared or reg_lcd_reset is set."]
    #[inline(always)]
    #[must_use]
    pub fn lcd_always_out_en(&mut self) -> LCD_ALWAYS_OUT_EN_W<13> {
        LCD_ALWAYS_OUT_EN_W::new(self)
    }
    #[doc = "Bit 19 - 1: invert every two data byte, valid in 1 byte mode. 0: Not change."]
    #[inline(always)]
    #[must_use]
    pub fn lcd_8bits_order(&mut self) -> LCD_8BITS_ORDER_W<19> {
        LCD_8BITS_ORDER_W::new(self)
    }
    #[doc = "Bit 20 - 1: Update LCD registers, will be cleared by hardware. 0 : Not care."]
    #[inline(always)]
    #[must_use]
    pub fn lcd_update(&mut self) -> LCD_UPDATE_W<20> {
        LCD_UPDATE_W::new(self)
    }
    #[doc = "Bit 21 - 1: Change data bit order, change LCD_DATA_out\\[7:0\\] to LCD_DATA_out\\[0:7\\] in one byte mode, and bits\\[15:0\\] to bits\\[0:15\\] in two byte mode. 0: Not change."]
    #[inline(always)]
    #[must_use]
    pub fn lcd_bit_order(&mut self) -> LCD_BIT_ORDER_W<21> {
        LCD_BIT_ORDER_W::new(self)
    }
    #[doc = "Bit 22 - 1: invert data byte order, only valid in 2 byte mode. 0: Not change."]
    #[inline(always)]
    #[must_use]
    pub fn lcd_byte_order(&mut self) -> LCD_BYTE_ORDER_W<22> {
        LCD_BYTE_ORDER_W::new(self)
    }
    #[doc = "Bit 23 - 1: The bit number of output LCD data is 9~16. 0: The bit number of output LCD data is 0~8."]
    #[inline(always)]
    #[must_use]
    pub fn lcd_2byte_en(&mut self) -> LCD_2BYTE_EN_W<23> {
        LCD_2BYTE_EN_W::new(self)
    }
    #[doc = "Bit 24 - 1: Be able to send data out in LCD sequence when LCD starts. 0: Disable."]
    #[inline(always)]
    #[must_use]
    pub fn lcd_dout(&mut self) -> LCD_DOUT_W<24> {
        LCD_DOUT_W::new(self)
    }
    #[doc = "Bit 25 - 1: Enable DUMMY phase in LCD sequence when LCD starts. 0: Disable."]
    #[inline(always)]
    #[must_use]
    pub fn lcd_dummy(&mut self) -> LCD_DUMMY_W<25> {
        LCD_DUMMY_W::new(self)
    }
    #[doc = "Bit 26 - 1: Be able to send command in LCD sequence when LCD starts. 0: Disable."]
    #[inline(always)]
    #[must_use]
    pub fn lcd_cmd(&mut self) -> LCD_CMD_W<26> {
        LCD_CMD_W::new(self)
    }
    #[doc = "Bit 27 - LCD start sending data enable signal, valid in high level."]
    #[inline(always)]
    #[must_use]
    pub fn lcd_start(&mut self) -> LCD_START_W<27> {
        LCD_START_W::new(self)
    }
    #[doc = "Bit 28 - The value of command."]
    #[inline(always)]
    #[must_use]
    pub fn lcd_reset(&mut self) -> LCD_RESET_W<28> {
        LCD_RESET_W::new(self)
    }
    #[doc = "Bits 29:30 - The dummy cycle length minus 1."]
    #[inline(always)]
    #[must_use]
    pub fn lcd_dummy_cyclelen(&mut self) -> LCD_DUMMY_CYCLELEN_W<29> {
        LCD_DUMMY_CYCLELEN_W::new(self)
    }
    #[doc = "Bit 31 - The cycle length of command phase. 1: 2 cycles. 0: 1 cycle."]
    #[inline(always)]
    #[must_use]
    pub fn lcd_cmd_2_cycle_en(&mut self) -> LCD_CMD_2_CYCLE_EN_W<31> {
        LCD_CMD_2_CYCLE_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LCD configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcd_user](index.html) module"]
pub struct LCD_USER_SPEC;
impl crate::RegisterSpec for LCD_USER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lcd_user::R](R) reader structure"]
impl crate::Readable for LCD_USER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lcd_user::W](W) writer structure"]
impl crate::Writable for LCD_USER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LCD_USER to value 0x01"]
impl crate::Resettable for LCD_USER_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
