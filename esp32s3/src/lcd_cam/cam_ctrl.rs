#[doc = "Register `CAM_CTRL` reader"]
pub type R = crate::R<CAM_CTRL_SPEC>;
#[doc = "Register `CAM_CTRL` writer"]
pub type W = crate::W<CAM_CTRL_SPEC>;
#[doc = "Field `CAM_STOP_EN` reader - Camera stop enable signal, 1: camera stops when GDMA Rx FIFO is full. 0: Do not stop."]
pub type CAM_STOP_EN_R = crate::BitReader;
#[doc = "Field `CAM_STOP_EN` writer - Camera stop enable signal, 1: camera stops when GDMA Rx FIFO is full. 0: Do not stop."]
pub type CAM_STOP_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAM_VSYNC_FILTER_THRES` reader - Filter threshold value for CAM_VSYNC signal."]
pub type CAM_VSYNC_FILTER_THRES_R = crate::FieldReader;
#[doc = "Field `CAM_VSYNC_FILTER_THRES` writer - Filter threshold value for CAM_VSYNC signal."]
pub type CAM_VSYNC_FILTER_THRES_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CAM_UPDATE` reader - 1: Update camera registers. This bit is cleared by hardware. 0: Do not care."]
pub type CAM_UPDATE_R = crate::BitReader;
#[doc = "Field `CAM_UPDATE` writer - 1: Update camera registers. This bit is cleared by hardware. 0: Do not care."]
pub type CAM_UPDATE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAM_BYTE_ORDER` reader - 1: Invert data byte order, only valid in 16-bit mode. 0: Do not change."]
pub type CAM_BYTE_ORDER_R = crate::BitReader;
#[doc = "Field `CAM_BYTE_ORDER` writer - 1: Invert data byte order, only valid in 16-bit mode. 0: Do not change."]
pub type CAM_BYTE_ORDER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAM_BIT_ORDER` reader - 1: Change data bit order, change CAM_DATA_in\\[7:0\\] to CAM_DATA_in\\[0:7\\] in 8-bit mode, and bits\\[15:0\\] to bits\\[0:15\\] in 16-bit mode. 0: Do not change."]
pub type CAM_BIT_ORDER_R = crate::BitReader;
#[doc = "Field `CAM_BIT_ORDER` writer - 1: Change data bit order, change CAM_DATA_in\\[7:0\\] to CAM_DATA_in\\[0:7\\] in 8-bit mode, and bits\\[15:0\\] to bits\\[0:15\\] in 16-bit mode. 0: Do not change."]
pub type CAM_BIT_ORDER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAM_LINE_INT_EN` reader - 1: Enable to generate LCD_CAM_CAM_HS_INT. 0: Disable."]
pub type CAM_LINE_INT_EN_R = crate::BitReader;
#[doc = "Field `CAM_LINE_INT_EN` writer - 1: Enable to generate LCD_CAM_CAM_HS_INT. 0: Disable."]
pub type CAM_LINE_INT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAM_VS_EOF_EN` reader - 1: Enable CAM_VSYNC to generate in_suc_eof. 0: in_suc_eof is controlled by LCD_CAM_CAM_REC_DATA_BYTELEN."]
pub type CAM_VS_EOF_EN_R = crate::BitReader;
#[doc = "Field `CAM_VS_EOF_EN` writer - 1: Enable CAM_VSYNC to generate in_suc_eof. 0: in_suc_eof is controlled by LCD_CAM_CAM_REC_DATA_BYTELEN."]
pub type CAM_VS_EOF_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAM_CLKM_DIV_NUM` reader - Integral camera clock divider value."]
pub type CAM_CLKM_DIV_NUM_R = crate::FieldReader;
#[doc = "Field `CAM_CLKM_DIV_NUM` writer - Integral camera clock divider value."]
pub type CAM_CLKM_DIV_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CAM_CLKM_DIV_B` reader - Fractional clock divider numerator value."]
pub type CAM_CLKM_DIV_B_R = crate::FieldReader;
#[doc = "Field `CAM_CLKM_DIV_B` writer - Fractional clock divider numerator value."]
pub type CAM_CLKM_DIV_B_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `CAM_CLKM_DIV_A` reader - Fractional clock divider denominator value."]
pub type CAM_CLKM_DIV_A_R = crate::FieldReader;
#[doc = "Field `CAM_CLKM_DIV_A` writer - Fractional clock divider denominator value."]
pub type CAM_CLKM_DIV_A_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `CAM_CLK_SEL` reader - Select camera module source clock. 0: Clock source is disabled. 1: XTAL_CLK. 2: PLL_D2_CLK. 3: PLL_F160M_CLK."]
pub type CAM_CLK_SEL_R = crate::FieldReader;
#[doc = "Field `CAM_CLK_SEL` writer - Select camera module source clock. 0: Clock source is disabled. 1: XTAL_CLK. 2: PLL_D2_CLK. 3: PLL_F160M_CLK."]
pub type CAM_CLK_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - Camera stop enable signal, 1: camera stops when GDMA Rx FIFO is full. 0: Do not stop."]
    #[inline(always)]
    pub fn cam_stop_en(&self) -> CAM_STOP_EN_R {
        CAM_STOP_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - Filter threshold value for CAM_VSYNC signal."]
    #[inline(always)]
    pub fn cam_vsync_filter_thres(&self) -> CAM_VSYNC_FILTER_THRES_R {
        CAM_VSYNC_FILTER_THRES_R::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bit 4 - 1: Update camera registers. This bit is cleared by hardware. 0: Do not care."]
    #[inline(always)]
    pub fn cam_update(&self) -> CAM_UPDATE_R {
        CAM_UPDATE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 1: Invert data byte order, only valid in 16-bit mode. 0: Do not change."]
    #[inline(always)]
    pub fn cam_byte_order(&self) -> CAM_BYTE_ORDER_R {
        CAM_BYTE_ORDER_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 1: Change data bit order, change CAM_DATA_in\\[7:0\\] to CAM_DATA_in\\[0:7\\] in 8-bit mode, and bits\\[15:0\\] to bits\\[0:15\\] in 16-bit mode. 0: Do not change."]
    #[inline(always)]
    pub fn cam_bit_order(&self) -> CAM_BIT_ORDER_R {
        CAM_BIT_ORDER_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 1: Enable to generate LCD_CAM_CAM_HS_INT. 0: Disable."]
    #[inline(always)]
    pub fn cam_line_int_en(&self) -> CAM_LINE_INT_EN_R {
        CAM_LINE_INT_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 1: Enable CAM_VSYNC to generate in_suc_eof. 0: in_suc_eof is controlled by LCD_CAM_CAM_REC_DATA_BYTELEN."]
    #[inline(always)]
    pub fn cam_vs_eof_en(&self) -> CAM_VS_EOF_EN_R {
        CAM_VS_EOF_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:16 - Integral camera clock divider value."]
    #[inline(always)]
    pub fn cam_clkm_div_num(&self) -> CAM_CLKM_DIV_NUM_R {
        CAM_CLKM_DIV_NUM_R::new(((self.bits >> 9) & 0xff) as u8)
    }
    #[doc = "Bits 17:22 - Fractional clock divider numerator value."]
    #[inline(always)]
    pub fn cam_clkm_div_b(&self) -> CAM_CLKM_DIV_B_R {
        CAM_CLKM_DIV_B_R::new(((self.bits >> 17) & 0x3f) as u8)
    }
    #[doc = "Bits 23:28 - Fractional clock divider denominator value."]
    #[inline(always)]
    pub fn cam_clkm_div_a(&self) -> CAM_CLKM_DIV_A_R {
        CAM_CLKM_DIV_A_R::new(((self.bits >> 23) & 0x3f) as u8)
    }
    #[doc = "Bits 29:30 - Select camera module source clock. 0: Clock source is disabled. 1: XTAL_CLK. 2: PLL_D2_CLK. 3: PLL_F160M_CLK."]
    #[inline(always)]
    pub fn cam_clk_sel(&self) -> CAM_CLK_SEL_R {
        CAM_CLK_SEL_R::new(((self.bits >> 29) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CAM_CTRL")
            .field("cam_stop_en", &self.cam_stop_en())
            .field("cam_vsync_filter_thres", &self.cam_vsync_filter_thres())
            .field("cam_update", &self.cam_update())
            .field("cam_byte_order", &self.cam_byte_order())
            .field("cam_bit_order", &self.cam_bit_order())
            .field("cam_line_int_en", &self.cam_line_int_en())
            .field("cam_vs_eof_en", &self.cam_vs_eof_en())
            .field("cam_clkm_div_num", &self.cam_clkm_div_num())
            .field("cam_clkm_div_b", &self.cam_clkm_div_b())
            .field("cam_clkm_div_a", &self.cam_clkm_div_a())
            .field("cam_clk_sel", &self.cam_clk_sel())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Camera stop enable signal, 1: camera stops when GDMA Rx FIFO is full. 0: Do not stop."]
    #[inline(always)]
    pub fn cam_stop_en(&mut self) -> CAM_STOP_EN_W<CAM_CTRL_SPEC> {
        CAM_STOP_EN_W::new(self, 0)
    }
    #[doc = "Bits 1:3 - Filter threshold value for CAM_VSYNC signal."]
    #[inline(always)]
    pub fn cam_vsync_filter_thres(&mut self) -> CAM_VSYNC_FILTER_THRES_W<CAM_CTRL_SPEC> {
        CAM_VSYNC_FILTER_THRES_W::new(self, 1)
    }
    #[doc = "Bit 4 - 1: Update camera registers. This bit is cleared by hardware. 0: Do not care."]
    #[inline(always)]
    pub fn cam_update(&mut self) -> CAM_UPDATE_W<CAM_CTRL_SPEC> {
        CAM_UPDATE_W::new(self, 4)
    }
    #[doc = "Bit 5 - 1: Invert data byte order, only valid in 16-bit mode. 0: Do not change."]
    #[inline(always)]
    pub fn cam_byte_order(&mut self) -> CAM_BYTE_ORDER_W<CAM_CTRL_SPEC> {
        CAM_BYTE_ORDER_W::new(self, 5)
    }
    #[doc = "Bit 6 - 1: Change data bit order, change CAM_DATA_in\\[7:0\\] to CAM_DATA_in\\[0:7\\] in 8-bit mode, and bits\\[15:0\\] to bits\\[0:15\\] in 16-bit mode. 0: Do not change."]
    #[inline(always)]
    pub fn cam_bit_order(&mut self) -> CAM_BIT_ORDER_W<CAM_CTRL_SPEC> {
        CAM_BIT_ORDER_W::new(self, 6)
    }
    #[doc = "Bit 7 - 1: Enable to generate LCD_CAM_CAM_HS_INT. 0: Disable."]
    #[inline(always)]
    pub fn cam_line_int_en(&mut self) -> CAM_LINE_INT_EN_W<CAM_CTRL_SPEC> {
        CAM_LINE_INT_EN_W::new(self, 7)
    }
    #[doc = "Bit 8 - 1: Enable CAM_VSYNC to generate in_suc_eof. 0: in_suc_eof is controlled by LCD_CAM_CAM_REC_DATA_BYTELEN."]
    #[inline(always)]
    pub fn cam_vs_eof_en(&mut self) -> CAM_VS_EOF_EN_W<CAM_CTRL_SPEC> {
        CAM_VS_EOF_EN_W::new(self, 8)
    }
    #[doc = "Bits 9:16 - Integral camera clock divider value."]
    #[inline(always)]
    pub fn cam_clkm_div_num(&mut self) -> CAM_CLKM_DIV_NUM_W<CAM_CTRL_SPEC> {
        CAM_CLKM_DIV_NUM_W::new(self, 9)
    }
    #[doc = "Bits 17:22 - Fractional clock divider numerator value."]
    #[inline(always)]
    pub fn cam_clkm_div_b(&mut self) -> CAM_CLKM_DIV_B_W<CAM_CTRL_SPEC> {
        CAM_CLKM_DIV_B_W::new(self, 17)
    }
    #[doc = "Bits 23:28 - Fractional clock divider denominator value."]
    #[inline(always)]
    pub fn cam_clkm_div_a(&mut self) -> CAM_CLKM_DIV_A_W<CAM_CTRL_SPEC> {
        CAM_CLKM_DIV_A_W::new(self, 23)
    }
    #[doc = "Bits 29:30 - Select camera module source clock. 0: Clock source is disabled. 1: XTAL_CLK. 2: PLL_D2_CLK. 3: PLL_F160M_CLK."]
    #[inline(always)]
    pub fn cam_clk_sel(&mut self) -> CAM_CLK_SEL_W<CAM_CTRL_SPEC> {
        CAM_CLK_SEL_W::new(self, 29)
    }
}
#[doc = "Camera clock configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`cam_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cam_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CAM_CTRL_SPEC;
impl crate::RegisterSpec for CAM_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cam_ctrl::R`](R) reader structure"]
impl crate::Readable for CAM_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cam_ctrl::W`](W) writer structure"]
impl crate::Writable for CAM_CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CAM_CTRL to value 0"]
impl crate::Resettable for CAM_CTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
