#[doc = "Register `CAM_CTRL` reader"]
pub struct R(crate::R<CAM_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CAM_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CAM_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CAM_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CAM_CTRL` writer"]
pub struct W(crate::W<CAM_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CAM_CTRL_SPEC>;
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
impl From<crate::W<CAM_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CAM_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CAM_STOP_EN` reader - Camera stop enable signal, 1: camera stops when DMA Rx FIFO is full. 0: Not stop."]
pub type CAM_STOP_EN_R = crate::BitReader;
#[doc = "Field `CAM_STOP_EN` writer - Camera stop enable signal, 1: camera stops when DMA Rx FIFO is full. 0: Not stop."]
pub type CAM_STOP_EN_W<'a, const O: u8> = crate::BitWriter<'a, CAM_CTRL_SPEC, O>;
#[doc = "Field `CAM_VSYNC_FILTER_THRES` reader - Filter threshold value for CAM_VSYNC signal."]
pub type CAM_VSYNC_FILTER_THRES_R = crate::FieldReader;
#[doc = "Field `CAM_VSYNC_FILTER_THRES` writer - Filter threshold value for CAM_VSYNC signal."]
pub type CAM_VSYNC_FILTER_THRES_W<'a, const O: u8> = crate::FieldWriter<'a, CAM_CTRL_SPEC, 3, O>;
#[doc = "Field `CAM_UPDATE` reader - 1: Update Camera registers, will be cleared by hardware. 0 : Not care."]
pub type CAM_UPDATE_R = crate::BitReader;
#[doc = "Field `CAM_UPDATE` writer - 1: Update Camera registers, will be cleared by hardware. 0 : Not care."]
pub type CAM_UPDATE_W<'a, const O: u8> = crate::BitWriter<'a, CAM_CTRL_SPEC, O>;
#[doc = "Field `CAM_BYTE_ORDER` reader - 1: Change data bit order, change CAM_DATA_in\\[7:0\\] to CAM_DATA_in\\[0:7\\] in one byte mode, and bits\\[15:0\\] to bits\\[0:15\\] in two byte mode. 0: Not change."]
pub type CAM_BYTE_ORDER_R = crate::BitReader;
#[doc = "Field `CAM_BYTE_ORDER` writer - 1: Change data bit order, change CAM_DATA_in\\[7:0\\] to CAM_DATA_in\\[0:7\\] in one byte mode, and bits\\[15:0\\] to bits\\[0:15\\] in two byte mode. 0: Not change."]
pub type CAM_BYTE_ORDER_W<'a, const O: u8> = crate::BitWriter<'a, CAM_CTRL_SPEC, O>;
#[doc = "Field `CAM_BIT_ORDER` reader - 1: invert data byte order, only valid in 2 byte mode. 0: Not change."]
pub type CAM_BIT_ORDER_R = crate::BitReader;
#[doc = "Field `CAM_BIT_ORDER` writer - 1: invert data byte order, only valid in 2 byte mode. 0: Not change."]
pub type CAM_BIT_ORDER_W<'a, const O: u8> = crate::BitWriter<'a, CAM_CTRL_SPEC, O>;
#[doc = "Field `CAM_LINE_INT_EN` reader - 1: Enable to generate CAM_HS_INT. 0: Disable."]
pub type CAM_LINE_INT_EN_R = crate::BitReader;
#[doc = "Field `CAM_LINE_INT_EN` writer - 1: Enable to generate CAM_HS_INT. 0: Disable."]
pub type CAM_LINE_INT_EN_W<'a, const O: u8> = crate::BitWriter<'a, CAM_CTRL_SPEC, O>;
#[doc = "Field `CAM_VS_EOF_EN` reader - 1: CAM_VSYNC to generate in_suc_eof. 0: in_suc_eof is controlled by reg_cam_rec_data_cyclelen."]
pub type CAM_VS_EOF_EN_R = crate::BitReader;
#[doc = "Field `CAM_VS_EOF_EN` writer - 1: CAM_VSYNC to generate in_suc_eof. 0: in_suc_eof is controlled by reg_cam_rec_data_cyclelen."]
pub type CAM_VS_EOF_EN_W<'a, const O: u8> = crate::BitWriter<'a, CAM_CTRL_SPEC, O>;
#[doc = "Field `CAM_CLKM_DIV_NUM` reader - Integral Camera clock divider value"]
pub type CAM_CLKM_DIV_NUM_R = crate::FieldReader;
#[doc = "Field `CAM_CLKM_DIV_NUM` writer - Integral Camera clock divider value"]
pub type CAM_CLKM_DIV_NUM_W<'a, const O: u8> = crate::FieldWriter<'a, CAM_CTRL_SPEC, 8, O>;
#[doc = "Field `CAM_CLKM_DIV_B` reader - Fractional clock divider numerator value"]
pub type CAM_CLKM_DIV_B_R = crate::FieldReader;
#[doc = "Field `CAM_CLKM_DIV_B` writer - Fractional clock divider numerator value"]
pub type CAM_CLKM_DIV_B_W<'a, const O: u8> = crate::FieldWriter<'a, CAM_CTRL_SPEC, 6, O>;
#[doc = "Field `CAM_CLKM_DIV_A` reader - Fractional clock divider denominator value"]
pub type CAM_CLKM_DIV_A_R = crate::FieldReader;
#[doc = "Field `CAM_CLKM_DIV_A` writer - Fractional clock divider denominator value"]
pub type CAM_CLKM_DIV_A_W<'a, const O: u8> = crate::FieldWriter<'a, CAM_CTRL_SPEC, 6, O>;
#[doc = "Field `CAM_CLK_SEL` reader - Select Camera module source clock. 0: no clock. 1: APLL. 2: CLK160. 3: no clock."]
pub type CAM_CLK_SEL_R = crate::FieldReader;
#[doc = "Field `CAM_CLK_SEL` writer - Select Camera module source clock. 0: no clock. 1: APLL. 2: CLK160. 3: no clock."]
pub type CAM_CLK_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, CAM_CTRL_SPEC, 2, O>;
impl R {
    #[doc = "Bit 0 - Camera stop enable signal, 1: camera stops when DMA Rx FIFO is full. 0: Not stop."]
    #[inline(always)]
    pub fn cam_stop_en(&self) -> CAM_STOP_EN_R {
        CAM_STOP_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - Filter threshold value for CAM_VSYNC signal."]
    #[inline(always)]
    pub fn cam_vsync_filter_thres(&self) -> CAM_VSYNC_FILTER_THRES_R {
        CAM_VSYNC_FILTER_THRES_R::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bit 4 - 1: Update Camera registers, will be cleared by hardware. 0 : Not care."]
    #[inline(always)]
    pub fn cam_update(&self) -> CAM_UPDATE_R {
        CAM_UPDATE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 1: Change data bit order, change CAM_DATA_in\\[7:0\\] to CAM_DATA_in\\[0:7\\] in one byte mode, and bits\\[15:0\\] to bits\\[0:15\\] in two byte mode. 0: Not change."]
    #[inline(always)]
    pub fn cam_byte_order(&self) -> CAM_BYTE_ORDER_R {
        CAM_BYTE_ORDER_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 1: invert data byte order, only valid in 2 byte mode. 0: Not change."]
    #[inline(always)]
    pub fn cam_bit_order(&self) -> CAM_BIT_ORDER_R {
        CAM_BIT_ORDER_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 1: Enable to generate CAM_HS_INT. 0: Disable."]
    #[inline(always)]
    pub fn cam_line_int_en(&self) -> CAM_LINE_INT_EN_R {
        CAM_LINE_INT_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 1: CAM_VSYNC to generate in_suc_eof. 0: in_suc_eof is controlled by reg_cam_rec_data_cyclelen."]
    #[inline(always)]
    pub fn cam_vs_eof_en(&self) -> CAM_VS_EOF_EN_R {
        CAM_VS_EOF_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:16 - Integral Camera clock divider value"]
    #[inline(always)]
    pub fn cam_clkm_div_num(&self) -> CAM_CLKM_DIV_NUM_R {
        CAM_CLKM_DIV_NUM_R::new(((self.bits >> 9) & 0xff) as u8)
    }
    #[doc = "Bits 17:22 - Fractional clock divider numerator value"]
    #[inline(always)]
    pub fn cam_clkm_div_b(&self) -> CAM_CLKM_DIV_B_R {
        CAM_CLKM_DIV_B_R::new(((self.bits >> 17) & 0x3f) as u8)
    }
    #[doc = "Bits 23:28 - Fractional clock divider denominator value"]
    #[inline(always)]
    pub fn cam_clkm_div_a(&self) -> CAM_CLKM_DIV_A_R {
        CAM_CLKM_DIV_A_R::new(((self.bits >> 23) & 0x3f) as u8)
    }
    #[doc = "Bits 29:30 - Select Camera module source clock. 0: no clock. 1: APLL. 2: CLK160. 3: no clock."]
    #[inline(always)]
    pub fn cam_clk_sel(&self) -> CAM_CLK_SEL_R {
        CAM_CLK_SEL_R::new(((self.bits >> 29) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CAM_CTRL")
            .field("cam_stop_en", &format_args!("{}", self.cam_stop_en().bit()))
            .field(
                "cam_vsync_filter_thres",
                &format_args!("{}", self.cam_vsync_filter_thres().bits()),
            )
            .field("cam_update", &format_args!("{}", self.cam_update().bit()))
            .field(
                "cam_byte_order",
                &format_args!("{}", self.cam_byte_order().bit()),
            )
            .field(
                "cam_bit_order",
                &format_args!("{}", self.cam_bit_order().bit()),
            )
            .field(
                "cam_line_int_en",
                &format_args!("{}", self.cam_line_int_en().bit()),
            )
            .field(
                "cam_vs_eof_en",
                &format_args!("{}", self.cam_vs_eof_en().bit()),
            )
            .field(
                "cam_clkm_div_num",
                &format_args!("{}", self.cam_clkm_div_num().bits()),
            )
            .field(
                "cam_clkm_div_b",
                &format_args!("{}", self.cam_clkm_div_b().bits()),
            )
            .field(
                "cam_clkm_div_a",
                &format_args!("{}", self.cam_clkm_div_a().bits()),
            )
            .field(
                "cam_clk_sel",
                &format_args!("{}", self.cam_clk_sel().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CAM_CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Camera stop enable signal, 1: camera stops when DMA Rx FIFO is full. 0: Not stop."]
    #[inline(always)]
    #[must_use]
    pub fn cam_stop_en(&mut self) -> CAM_STOP_EN_W<0> {
        CAM_STOP_EN_W::new(self)
    }
    #[doc = "Bits 1:3 - Filter threshold value for CAM_VSYNC signal."]
    #[inline(always)]
    #[must_use]
    pub fn cam_vsync_filter_thres(&mut self) -> CAM_VSYNC_FILTER_THRES_W<1> {
        CAM_VSYNC_FILTER_THRES_W::new(self)
    }
    #[doc = "Bit 4 - 1: Update Camera registers, will be cleared by hardware. 0 : Not care."]
    #[inline(always)]
    #[must_use]
    pub fn cam_update(&mut self) -> CAM_UPDATE_W<4> {
        CAM_UPDATE_W::new(self)
    }
    #[doc = "Bit 5 - 1: Change data bit order, change CAM_DATA_in\\[7:0\\] to CAM_DATA_in\\[0:7\\] in one byte mode, and bits\\[15:0\\] to bits\\[0:15\\] in two byte mode. 0: Not change."]
    #[inline(always)]
    #[must_use]
    pub fn cam_byte_order(&mut self) -> CAM_BYTE_ORDER_W<5> {
        CAM_BYTE_ORDER_W::new(self)
    }
    #[doc = "Bit 6 - 1: invert data byte order, only valid in 2 byte mode. 0: Not change."]
    #[inline(always)]
    #[must_use]
    pub fn cam_bit_order(&mut self) -> CAM_BIT_ORDER_W<6> {
        CAM_BIT_ORDER_W::new(self)
    }
    #[doc = "Bit 7 - 1: Enable to generate CAM_HS_INT. 0: Disable."]
    #[inline(always)]
    #[must_use]
    pub fn cam_line_int_en(&mut self) -> CAM_LINE_INT_EN_W<7> {
        CAM_LINE_INT_EN_W::new(self)
    }
    #[doc = "Bit 8 - 1: CAM_VSYNC to generate in_suc_eof. 0: in_suc_eof is controlled by reg_cam_rec_data_cyclelen."]
    #[inline(always)]
    #[must_use]
    pub fn cam_vs_eof_en(&mut self) -> CAM_VS_EOF_EN_W<8> {
        CAM_VS_EOF_EN_W::new(self)
    }
    #[doc = "Bits 9:16 - Integral Camera clock divider value"]
    #[inline(always)]
    #[must_use]
    pub fn cam_clkm_div_num(&mut self) -> CAM_CLKM_DIV_NUM_W<9> {
        CAM_CLKM_DIV_NUM_W::new(self)
    }
    #[doc = "Bits 17:22 - Fractional clock divider numerator value"]
    #[inline(always)]
    #[must_use]
    pub fn cam_clkm_div_b(&mut self) -> CAM_CLKM_DIV_B_W<17> {
        CAM_CLKM_DIV_B_W::new(self)
    }
    #[doc = "Bits 23:28 - Fractional clock divider denominator value"]
    #[inline(always)]
    #[must_use]
    pub fn cam_clkm_div_a(&mut self) -> CAM_CLKM_DIV_A_W<23> {
        CAM_CLKM_DIV_A_W::new(self)
    }
    #[doc = "Bits 29:30 - Select Camera module source clock. 0: no clock. 1: APLL. 2: CLK160. 3: no clock."]
    #[inline(always)]
    #[must_use]
    pub fn cam_clk_sel(&mut self) -> CAM_CLK_SEL_W<29> {
        CAM_CLK_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Camera configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cam_ctrl](index.html) module"]
pub struct CAM_CTRL_SPEC;
impl crate::RegisterSpec for CAM_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cam_ctrl::R](R) reader structure"]
impl crate::Readable for CAM_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cam_ctrl::W](W) writer structure"]
impl crate::Writable for CAM_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CAM_CTRL to value 0x0800"]
impl crate::Resettable for CAM_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0800;
}
