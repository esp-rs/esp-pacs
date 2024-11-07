#[doc = "Register `LCD_MISC` reader"]
pub type R = crate::R<LCD_MISC_SPEC>;
#[doc = "Register `LCD_MISC` writer"]
pub type W = crate::W<LCD_MISC_SPEC>;
#[doc = "Field `LCD_WIRE_MODE` reader - The wire width of LCD output. 0: 8bit. 1: 16bit 2: 24bit"]
pub type LCD_WIRE_MODE_R = crate::FieldReader;
#[doc = "Field `LCD_WIRE_MODE` writer - The wire width of LCD output. 0: 8bit. 1: 16bit 2: 24bit"]
pub type LCD_WIRE_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LCD_VFK_CYCLELEN` reader - The setup cycle length minus 1 in LCD non-RGB mode."]
pub type LCD_VFK_CYCLELEN_R = crate::FieldReader;
#[doc = "Field `LCD_VFK_CYCLELEN` writer - The setup cycle length minus 1 in LCD non-RGB mode."]
pub type LCD_VFK_CYCLELEN_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `LCD_VBK_CYCLELEN` reader - The vertical back blank region cycle length minus 1 in LCD RGB mode, or the hold time cycle length in LCD non-RGB mode."]
pub type LCD_VBK_CYCLELEN_R = crate::FieldReader<u16>;
#[doc = "Field `LCD_VBK_CYCLELEN` writer - The vertical back blank region cycle length minus 1 in LCD RGB mode, or the hold time cycle length in LCD non-RGB mode."]
pub type LCD_VBK_CYCLELEN_W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
#[doc = "Field `LCD_NEXT_FRAME_EN` reader - 1: Send the next frame data when the current frame is sent out. 0: LCD stops when the current frame is sent out."]
pub type LCD_NEXT_FRAME_EN_R = crate::BitReader;
#[doc = "Field `LCD_NEXT_FRAME_EN` writer - 1: Send the next frame data when the current frame is sent out. 0: LCD stops when the current frame is sent out."]
pub type LCD_NEXT_FRAME_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCD_BK_EN` reader - 1: Enable blank region when LCD sends data out. 0: No blank region."]
pub type LCD_BK_EN_R = crate::BitReader;
#[doc = "Field `LCD_BK_EN` writer - 1: Enable blank region when LCD sends data out. 0: No blank region."]
pub type LCD_BK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCD_AFIFO_RESET` writer - LCD AFIFO reset signal."]
pub type LCD_AFIFO_RESET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCD_CD_DATA_SET` reader - 1: LCD_CD = !reg_cd_idle_edge when lcd_st\\[2:0\\] is in LCD_DOUT state. 0: LCD_CD = reg_cd_idle_edge."]
pub type LCD_CD_DATA_SET_R = crate::BitReader;
#[doc = "Field `LCD_CD_DATA_SET` writer - 1: LCD_CD = !reg_cd_idle_edge when lcd_st\\[2:0\\] is in LCD_DOUT state. 0: LCD_CD = reg_cd_idle_edge."]
pub type LCD_CD_DATA_SET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCD_CD_DUMMY_SET` reader - 1: LCD_CD = !reg_cd_idle_edge when lcd_st\\[2:0\\] is in LCD_DUMMY state. 0: LCD_CD = reg_cd_idle_edge."]
pub type LCD_CD_DUMMY_SET_R = crate::BitReader;
#[doc = "Field `LCD_CD_DUMMY_SET` writer - 1: LCD_CD = !reg_cd_idle_edge when lcd_st\\[2:0\\] is in LCD_DUMMY state. 0: LCD_CD = reg_cd_idle_edge."]
pub type LCD_CD_DUMMY_SET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCD_CD_CMD_SET` reader - 1: LCD_CD = !reg_cd_idle_edge when lcd_st\\[2:0\\] is in LCD_CMD state. 0: LCD_CD = reg_cd_idle_edge."]
pub type LCD_CD_CMD_SET_R = crate::BitReader;
#[doc = "Field `LCD_CD_CMD_SET` writer - 1: LCD_CD = !reg_cd_idle_edge when lcd_st\\[2:0\\] is in LCD_CMD state. 0: LCD_CD = reg_cd_idle_edge."]
pub type LCD_CD_CMD_SET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCD_CD_IDLE_EDGE` reader - The default value of LCD_CD."]
pub type LCD_CD_IDLE_EDGE_R = crate::BitReader;
#[doc = "Field `LCD_CD_IDLE_EDGE` writer - The default value of LCD_CD."]
pub type LCD_CD_IDLE_EDGE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 4:5 - The wire width of LCD output. 0: 8bit. 1: 16bit 2: 24bit"]
    #[inline(always)]
    pub fn lcd_wire_mode(&self) -> LCD_WIRE_MODE_R {
        LCD_WIRE_MODE_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:11 - The setup cycle length minus 1 in LCD non-RGB mode."]
    #[inline(always)]
    pub fn lcd_vfk_cyclelen(&self) -> LCD_VFK_CYCLELEN_R {
        LCD_VFK_CYCLELEN_R::new(((self.bits >> 6) & 0x3f) as u8)
    }
    #[doc = "Bits 12:24 - The vertical back blank region cycle length minus 1 in LCD RGB mode, or the hold time cycle length in LCD non-RGB mode."]
    #[inline(always)]
    pub fn lcd_vbk_cyclelen(&self) -> LCD_VBK_CYCLELEN_R {
        LCD_VBK_CYCLELEN_R::new(((self.bits >> 12) & 0x1fff) as u16)
    }
    #[doc = "Bit 25 - 1: Send the next frame data when the current frame is sent out. 0: LCD stops when the current frame is sent out."]
    #[inline(always)]
    pub fn lcd_next_frame_en(&self) -> LCD_NEXT_FRAME_EN_R {
        LCD_NEXT_FRAME_EN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - 1: Enable blank region when LCD sends data out. 0: No blank region."]
    #[inline(always)]
    pub fn lcd_bk_en(&self) -> LCD_BK_EN_R {
        LCD_BK_EN_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 28 - 1: LCD_CD = !reg_cd_idle_edge when lcd_st\\[2:0\\] is in LCD_DOUT state. 0: LCD_CD = reg_cd_idle_edge."]
    #[inline(always)]
    pub fn lcd_cd_data_set(&self) -> LCD_CD_DATA_SET_R {
        LCD_CD_DATA_SET_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - 1: LCD_CD = !reg_cd_idle_edge when lcd_st\\[2:0\\] is in LCD_DUMMY state. 0: LCD_CD = reg_cd_idle_edge."]
    #[inline(always)]
    pub fn lcd_cd_dummy_set(&self) -> LCD_CD_DUMMY_SET_R {
        LCD_CD_DUMMY_SET_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - 1: LCD_CD = !reg_cd_idle_edge when lcd_st\\[2:0\\] is in LCD_CMD state. 0: LCD_CD = reg_cd_idle_edge."]
    #[inline(always)]
    pub fn lcd_cd_cmd_set(&self) -> LCD_CD_CMD_SET_R {
        LCD_CD_CMD_SET_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - The default value of LCD_CD."]
    #[inline(always)]
    pub fn lcd_cd_idle_edge(&self) -> LCD_CD_IDLE_EDGE_R {
        LCD_CD_IDLE_EDGE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LCD_MISC")
            .field("lcd_wire_mode", &self.lcd_wire_mode())
            .field("lcd_vfk_cyclelen", &self.lcd_vfk_cyclelen())
            .field("lcd_vbk_cyclelen", &self.lcd_vbk_cyclelen())
            .field("lcd_next_frame_en", &self.lcd_next_frame_en())
            .field("lcd_bk_en", &self.lcd_bk_en())
            .field("lcd_cd_data_set", &self.lcd_cd_data_set())
            .field("lcd_cd_dummy_set", &self.lcd_cd_dummy_set())
            .field("lcd_cd_cmd_set", &self.lcd_cd_cmd_set())
            .field("lcd_cd_idle_edge", &self.lcd_cd_idle_edge())
            .finish()
    }
}
impl W {
    #[doc = "Bits 4:5 - The wire width of LCD output. 0: 8bit. 1: 16bit 2: 24bit"]
    #[inline(always)]
    pub fn lcd_wire_mode(&mut self) -> LCD_WIRE_MODE_W<LCD_MISC_SPEC> {
        LCD_WIRE_MODE_W::new(self, 4)
    }
    #[doc = "Bits 6:11 - The setup cycle length minus 1 in LCD non-RGB mode."]
    #[inline(always)]
    pub fn lcd_vfk_cyclelen(&mut self) -> LCD_VFK_CYCLELEN_W<LCD_MISC_SPEC> {
        LCD_VFK_CYCLELEN_W::new(self, 6)
    }
    #[doc = "Bits 12:24 - The vertical back blank region cycle length minus 1 in LCD RGB mode, or the hold time cycle length in LCD non-RGB mode."]
    #[inline(always)]
    pub fn lcd_vbk_cyclelen(&mut self) -> LCD_VBK_CYCLELEN_W<LCD_MISC_SPEC> {
        LCD_VBK_CYCLELEN_W::new(self, 12)
    }
    #[doc = "Bit 25 - 1: Send the next frame data when the current frame is sent out. 0: LCD stops when the current frame is sent out."]
    #[inline(always)]
    pub fn lcd_next_frame_en(&mut self) -> LCD_NEXT_FRAME_EN_W<LCD_MISC_SPEC> {
        LCD_NEXT_FRAME_EN_W::new(self, 25)
    }
    #[doc = "Bit 26 - 1: Enable blank region when LCD sends data out. 0: No blank region."]
    #[inline(always)]
    pub fn lcd_bk_en(&mut self) -> LCD_BK_EN_W<LCD_MISC_SPEC> {
        LCD_BK_EN_W::new(self, 26)
    }
    #[doc = "Bit 27 - LCD AFIFO reset signal."]
    #[inline(always)]
    pub fn lcd_afifo_reset(&mut self) -> LCD_AFIFO_RESET_W<LCD_MISC_SPEC> {
        LCD_AFIFO_RESET_W::new(self, 27)
    }
    #[doc = "Bit 28 - 1: LCD_CD = !reg_cd_idle_edge when lcd_st\\[2:0\\] is in LCD_DOUT state. 0: LCD_CD = reg_cd_idle_edge."]
    #[inline(always)]
    pub fn lcd_cd_data_set(&mut self) -> LCD_CD_DATA_SET_W<LCD_MISC_SPEC> {
        LCD_CD_DATA_SET_W::new(self, 28)
    }
    #[doc = "Bit 29 - 1: LCD_CD = !reg_cd_idle_edge when lcd_st\\[2:0\\] is in LCD_DUMMY state. 0: LCD_CD = reg_cd_idle_edge."]
    #[inline(always)]
    pub fn lcd_cd_dummy_set(&mut self) -> LCD_CD_DUMMY_SET_W<LCD_MISC_SPEC> {
        LCD_CD_DUMMY_SET_W::new(self, 29)
    }
    #[doc = "Bit 30 - 1: LCD_CD = !reg_cd_idle_edge when lcd_st\\[2:0\\] is in LCD_CMD state. 0: LCD_CD = reg_cd_idle_edge."]
    #[inline(always)]
    pub fn lcd_cd_cmd_set(&mut self) -> LCD_CD_CMD_SET_W<LCD_MISC_SPEC> {
        LCD_CD_CMD_SET_W::new(self, 30)
    }
    #[doc = "Bit 31 - The default value of LCD_CD."]
    #[inline(always)]
    pub fn lcd_cd_idle_edge(&mut self) -> LCD_CD_IDLE_EDGE_W<LCD_MISC_SPEC> {
        LCD_CD_IDLE_EDGE_W::new(self, 31)
    }
}
#[doc = "LCD config register.\n\nYou can [`read`](crate::Reg::read) this register and get [`lcd_misc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_misc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LCD_MISC_SPEC;
impl crate::RegisterSpec for LCD_MISC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcd_misc::R`](R) reader structure"]
impl crate::Readable for LCD_MISC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lcd_misc::W`](W) writer structure"]
impl crate::Writable for LCD_MISC_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LCD_MISC to value 0xc0"]
impl crate::Resettable for LCD_MISC_SPEC {
    const RESET_VALUE: u32 = 0xc0;
}
