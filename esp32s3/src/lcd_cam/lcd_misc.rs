#[doc = "Register `LCD_MISC` reader"]
pub struct R(crate::R<LCD_MISC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LCD_MISC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LCD_MISC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LCD_MISC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LCD_MISC` writer"]
pub struct W(crate::W<LCD_MISC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LCD_MISC_SPEC>;
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
impl From<crate::W<LCD_MISC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LCD_MISC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LCD_AFIFO_THRESHOLD_NUM` reader - The awfull threshold number of lcd_afifo."]
pub type LCD_AFIFO_THRESHOLD_NUM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LCD_AFIFO_THRESHOLD_NUM` writer - The awfull threshold number of lcd_afifo."]
pub type LCD_AFIFO_THRESHOLD_NUM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LCD_MISC_SPEC, u8, u8, 5, O>;
#[doc = "Field `LCD_VFK_CYCLELEN` reader - The setup cycle length minus 1 in LCD non-RGB mode."]
pub type LCD_VFK_CYCLELEN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LCD_VFK_CYCLELEN` writer - The setup cycle length minus 1 in LCD non-RGB mode."]
pub type LCD_VFK_CYCLELEN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LCD_MISC_SPEC, u8, u8, 6, O>;
#[doc = "Field `LCD_VBK_CYCLELEN` reader - The vertical back blank region cycle length minus 1 in LCD RGB mode, or the hold time cycle length in LCD non-RGB mode."]
pub type LCD_VBK_CYCLELEN_R = crate::FieldReader<u16, u16>;
#[doc = "Field `LCD_VBK_CYCLELEN` writer - The vertical back blank region cycle length minus 1 in LCD RGB mode, or the hold time cycle length in LCD non-RGB mode."]
pub type LCD_VBK_CYCLELEN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LCD_MISC_SPEC, u16, u16, 13, O>;
#[doc = "Field `LCD_NEXT_FRAME_EN` reader - 1: Send the next frame data when the current frame is sent out. 0: LCD stops when the current frame is sent out."]
pub type LCD_NEXT_FRAME_EN_R = crate::BitReader<bool>;
#[doc = "Field `LCD_NEXT_FRAME_EN` writer - 1: Send the next frame data when the current frame is sent out. 0: LCD stops when the current frame is sent out."]
pub type LCD_NEXT_FRAME_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCD_MISC_SPEC, bool, O>;
#[doc = "Field `LCD_BK_EN` reader - 1: Enable blank region when LCD sends data out. 0: No blank region."]
pub type LCD_BK_EN_R = crate::BitReader<bool>;
#[doc = "Field `LCD_BK_EN` writer - 1: Enable blank region when LCD sends data out. 0: No blank region."]
pub type LCD_BK_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCD_MISC_SPEC, bool, O>;
#[doc = "Field `LCD_AFIFO_RESET` writer - LCD AFIFO reset signal."]
pub type LCD_AFIFO_RESET_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCD_MISC_SPEC, bool, O>;
#[doc = "Field `LCD_CD_DATA_SET` reader - 1: LCD_CD = !reg_cd_idle_edge when lcd_st\\[2:0\\] is in LCD_DOUT state. 0: LCD_CD = reg_cd_idle_edge."]
pub type LCD_CD_DATA_SET_R = crate::BitReader<bool>;
#[doc = "Field `LCD_CD_DATA_SET` writer - 1: LCD_CD = !reg_cd_idle_edge when lcd_st\\[2:0\\] is in LCD_DOUT state. 0: LCD_CD = reg_cd_idle_edge."]
pub type LCD_CD_DATA_SET_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCD_MISC_SPEC, bool, O>;
#[doc = "Field `LCD_CD_DUMMY_SET` reader - 1: LCD_CD = !reg_cd_idle_edge when lcd_st\\[2:0\\] is in LCD_DUMMY state. 0: LCD_CD = reg_cd_idle_edge."]
pub type LCD_CD_DUMMY_SET_R = crate::BitReader<bool>;
#[doc = "Field `LCD_CD_DUMMY_SET` writer - 1: LCD_CD = !reg_cd_idle_edge when lcd_st\\[2:0\\] is in LCD_DUMMY state. 0: LCD_CD = reg_cd_idle_edge."]
pub type LCD_CD_DUMMY_SET_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCD_MISC_SPEC, bool, O>;
#[doc = "Field `LCD_CD_CMD_SET` reader - 1: LCD_CD = !reg_cd_idle_edge when lcd_st\\[2:0\\] is in LCD_CMD state. 0: LCD_CD = reg_cd_idle_edge."]
pub type LCD_CD_CMD_SET_R = crate::BitReader<bool>;
#[doc = "Field `LCD_CD_CMD_SET` writer - 1: LCD_CD = !reg_cd_idle_edge when lcd_st\\[2:0\\] is in LCD_CMD state. 0: LCD_CD = reg_cd_idle_edge."]
pub type LCD_CD_CMD_SET_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCD_MISC_SPEC, bool, O>;
#[doc = "Field `LCD_CD_IDLE_EDGE` reader - The default value of LCD_CD."]
pub type LCD_CD_IDLE_EDGE_R = crate::BitReader<bool>;
#[doc = "Field `LCD_CD_IDLE_EDGE` writer - The default value of LCD_CD."]
pub type LCD_CD_IDLE_EDGE_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCD_MISC_SPEC, bool, O>;
impl R {
    #[doc = "Bits 1:5 - The awfull threshold number of lcd_afifo."]
    #[inline(always)]
    pub fn lcd_afifo_threshold_num(&self) -> LCD_AFIFO_THRESHOLD_NUM_R {
        LCD_AFIFO_THRESHOLD_NUM_R::new(((self.bits >> 1) & 0x1f) as u8)
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
impl W {
    #[doc = "Bits 1:5 - The awfull threshold number of lcd_afifo."]
    #[inline(always)]
    pub fn lcd_afifo_threshold_num(&mut self) -> LCD_AFIFO_THRESHOLD_NUM_W<1> {
        LCD_AFIFO_THRESHOLD_NUM_W::new(self)
    }
    #[doc = "Bits 6:11 - The setup cycle length minus 1 in LCD non-RGB mode."]
    #[inline(always)]
    pub fn lcd_vfk_cyclelen(&mut self) -> LCD_VFK_CYCLELEN_W<6> {
        LCD_VFK_CYCLELEN_W::new(self)
    }
    #[doc = "Bits 12:24 - The vertical back blank region cycle length minus 1 in LCD RGB mode, or the hold time cycle length in LCD non-RGB mode."]
    #[inline(always)]
    pub fn lcd_vbk_cyclelen(&mut self) -> LCD_VBK_CYCLELEN_W<12> {
        LCD_VBK_CYCLELEN_W::new(self)
    }
    #[doc = "Bit 25 - 1: Send the next frame data when the current frame is sent out. 0: LCD stops when the current frame is sent out."]
    #[inline(always)]
    pub fn lcd_next_frame_en(&mut self) -> LCD_NEXT_FRAME_EN_W<25> {
        LCD_NEXT_FRAME_EN_W::new(self)
    }
    #[doc = "Bit 26 - 1: Enable blank region when LCD sends data out. 0: No blank region."]
    #[inline(always)]
    pub fn lcd_bk_en(&mut self) -> LCD_BK_EN_W<26> {
        LCD_BK_EN_W::new(self)
    }
    #[doc = "Bit 27 - LCD AFIFO reset signal."]
    #[inline(always)]
    pub fn lcd_afifo_reset(&mut self) -> LCD_AFIFO_RESET_W<27> {
        LCD_AFIFO_RESET_W::new(self)
    }
    #[doc = "Bit 28 - 1: LCD_CD = !reg_cd_idle_edge when lcd_st\\[2:0\\] is in LCD_DOUT state. 0: LCD_CD = reg_cd_idle_edge."]
    #[inline(always)]
    pub fn lcd_cd_data_set(&mut self) -> LCD_CD_DATA_SET_W<28> {
        LCD_CD_DATA_SET_W::new(self)
    }
    #[doc = "Bit 29 - 1: LCD_CD = !reg_cd_idle_edge when lcd_st\\[2:0\\] is in LCD_DUMMY state. 0: LCD_CD = reg_cd_idle_edge."]
    #[inline(always)]
    pub fn lcd_cd_dummy_set(&mut self) -> LCD_CD_DUMMY_SET_W<29> {
        LCD_CD_DUMMY_SET_W::new(self)
    }
    #[doc = "Bit 30 - 1: LCD_CD = !reg_cd_idle_edge when lcd_st\\[2:0\\] is in LCD_CMD state. 0: LCD_CD = reg_cd_idle_edge."]
    #[inline(always)]
    pub fn lcd_cd_cmd_set(&mut self) -> LCD_CD_CMD_SET_W<30> {
        LCD_CD_CMD_SET_W::new(self)
    }
    #[doc = "Bit 31 - The default value of LCD_CD."]
    #[inline(always)]
    pub fn lcd_cd_idle_edge(&mut self) -> LCD_CD_IDLE_EDGE_W<31> {
        LCD_CD_IDLE_EDGE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LCD configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcd_misc](index.html) module"]
pub struct LCD_MISC_SPEC;
impl crate::RegisterSpec for LCD_MISC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lcd_misc::R](R) reader structure"]
impl crate::Readable for LCD_MISC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lcd_misc::W](W) writer structure"]
impl crate::Writable for LCD_MISC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LCD_MISC to value 0xd6"]
impl crate::Resettable for LCD_MISC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xd6
    }
}
