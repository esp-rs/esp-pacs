#[doc = "Register `CAM_CTRL1` reader"]
pub type R = crate::R<CAM_CTRL1_SPEC>;
#[doc = "Register `CAM_CTRL1` writer"]
pub type W = crate::W<CAM_CTRL1_SPEC>;
#[doc = "Field `CAM_REC_DATA_BYTELEN` reader - Configure camera received data byte length. When the length of received data reaches this value + 1, GDMA in_suc_eof_int is triggered."]
pub type CAM_REC_DATA_BYTELEN_R = crate::FieldReader<u16>;
#[doc = "Field `CAM_REC_DATA_BYTELEN` writer - Configure camera received data byte length. When the length of received data reaches this value + 1, GDMA in_suc_eof_int is triggered."]
pub type CAM_REC_DATA_BYTELEN_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `CAM_LINE_INT_NUM` reader - Configure line number. When the number of received lines reaches this value + 1, LCD_CAM_CAM_HS_INT is triggered."]
pub type CAM_LINE_INT_NUM_R = crate::FieldReader;
#[doc = "Field `CAM_LINE_INT_NUM` writer - Configure line number. When the number of received lines reaches this value + 1, LCD_CAM_CAM_HS_INT is triggered."]
pub type CAM_LINE_INT_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `CAM_CLK_INV` reader - 1: Invert the input signal CAM_PCLK. 0: Not invert."]
pub type CAM_CLK_INV_R = crate::BitReader;
#[doc = "Field `CAM_CLK_INV` writer - 1: Invert the input signal CAM_PCLK. 0: Not invert."]
pub type CAM_CLK_INV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAM_VSYNC_FILTER_EN` reader - 1: Enable CAM_VSYNC filter function. 0: bypass."]
pub type CAM_VSYNC_FILTER_EN_R = crate::BitReader;
#[doc = "Field `CAM_VSYNC_FILTER_EN` writer - 1: Enable CAM_VSYNC filter function. 0: bypass."]
pub type CAM_VSYNC_FILTER_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAM_2BYTE_EN` reader - 1: The width of input data is 16 bits. 0: The width of input data is 8 bits."]
pub type CAM_2BYTE_EN_R = crate::BitReader;
#[doc = "Field `CAM_2BYTE_EN` writer - 1: The width of input data is 16 bits. 0: The width of input data is 8 bits."]
pub type CAM_2BYTE_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAM_DE_INV` reader - CAM_DE invert enable signal, valid in high level."]
pub type CAM_DE_INV_R = crate::BitReader;
#[doc = "Field `CAM_DE_INV` writer - CAM_DE invert enable signal, valid in high level."]
pub type CAM_DE_INV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAM_HSYNC_INV` reader - CAM_HSYNC invert enable signal, valid in high level."]
pub type CAM_HSYNC_INV_R = crate::BitReader;
#[doc = "Field `CAM_HSYNC_INV` writer - CAM_HSYNC invert enable signal, valid in high level."]
pub type CAM_HSYNC_INV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAM_VSYNC_INV` reader - CAM_VSYNC invert enable signal, valid in high level."]
pub type CAM_VSYNC_INV_R = crate::BitReader;
#[doc = "Field `CAM_VSYNC_INV` writer - CAM_VSYNC invert enable signal, valid in high level."]
pub type CAM_VSYNC_INV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAM_VH_DE_MODE_EN` reader - 1: Input control signals are CAM_DE CAM_HSYNC and CAM_VSYNC is 1. 0: Input control signals are CAM_DE and CAM_VSYNC. CAM_HSYNC and CAM_DE are all 1 the the same time."]
pub type CAM_VH_DE_MODE_EN_R = crate::BitReader;
#[doc = "Field `CAM_VH_DE_MODE_EN` writer - 1: Input control signals are CAM_DE CAM_HSYNC and CAM_VSYNC is 1. 0: Input control signals are CAM_DE and CAM_VSYNC. CAM_HSYNC and CAM_DE are all 1 the the same time."]
pub type CAM_VH_DE_MODE_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAM_START` reader - Camera module start signal."]
pub type CAM_START_R = crate::BitReader;
#[doc = "Field `CAM_START` writer - Camera module start signal."]
pub type CAM_START_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAM_RESET` writer - Camera module reset signal."]
pub type CAM_RESET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAM_AFIFO_RESET` writer - Camera Async Rx FIFO reset signal."]
pub type CAM_AFIFO_RESET_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - Configure camera received data byte length. When the length of received data reaches this value + 1, GDMA in_suc_eof_int is triggered."]
    #[inline(always)]
    pub fn cam_rec_data_bytelen(&self) -> CAM_REC_DATA_BYTELEN_R {
        CAM_REC_DATA_BYTELEN_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:21 - Configure line number. When the number of received lines reaches this value + 1, LCD_CAM_CAM_HS_INT is triggered."]
    #[inline(always)]
    pub fn cam_line_int_num(&self) -> CAM_LINE_INT_NUM_R {
        CAM_LINE_INT_NUM_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 22 - 1: Invert the input signal CAM_PCLK. 0: Not invert."]
    #[inline(always)]
    pub fn cam_clk_inv(&self) -> CAM_CLK_INV_R {
        CAM_CLK_INV_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - 1: Enable CAM_VSYNC filter function. 0: bypass."]
    #[inline(always)]
    pub fn cam_vsync_filter_en(&self) -> CAM_VSYNC_FILTER_EN_R {
        CAM_VSYNC_FILTER_EN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - 1: The width of input data is 16 bits. 0: The width of input data is 8 bits."]
    #[inline(always)]
    pub fn cam_2byte_en(&self) -> CAM_2BYTE_EN_R {
        CAM_2BYTE_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - CAM_DE invert enable signal, valid in high level."]
    #[inline(always)]
    pub fn cam_de_inv(&self) -> CAM_DE_INV_R {
        CAM_DE_INV_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - CAM_HSYNC invert enable signal, valid in high level."]
    #[inline(always)]
    pub fn cam_hsync_inv(&self) -> CAM_HSYNC_INV_R {
        CAM_HSYNC_INV_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - CAM_VSYNC invert enable signal, valid in high level."]
    #[inline(always)]
    pub fn cam_vsync_inv(&self) -> CAM_VSYNC_INV_R {
        CAM_VSYNC_INV_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - 1: Input control signals are CAM_DE CAM_HSYNC and CAM_VSYNC is 1. 0: Input control signals are CAM_DE and CAM_VSYNC. CAM_HSYNC and CAM_DE are all 1 the the same time."]
    #[inline(always)]
    pub fn cam_vh_de_mode_en(&self) -> CAM_VH_DE_MODE_EN_R {
        CAM_VH_DE_MODE_EN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Camera module start signal."]
    #[inline(always)]
    pub fn cam_start(&self) -> CAM_START_R {
        CAM_START_R::new(((self.bits >> 29) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CAM_CTRL1")
            .field(
                "cam_rec_data_bytelen",
                &format_args!("{}", self.cam_rec_data_bytelen().bits()),
            )
            .field(
                "cam_line_int_num",
                &format_args!("{}", self.cam_line_int_num().bits()),
            )
            .field("cam_clk_inv", &format_args!("{}", self.cam_clk_inv().bit()))
            .field(
                "cam_vsync_filter_en",
                &format_args!("{}", self.cam_vsync_filter_en().bit()),
            )
            .field(
                "cam_2byte_en",
                &format_args!("{}", self.cam_2byte_en().bit()),
            )
            .field("cam_de_inv", &format_args!("{}", self.cam_de_inv().bit()))
            .field(
                "cam_hsync_inv",
                &format_args!("{}", self.cam_hsync_inv().bit()),
            )
            .field(
                "cam_vsync_inv",
                &format_args!("{}", self.cam_vsync_inv().bit()),
            )
            .field(
                "cam_vh_de_mode_en",
                &format_args!("{}", self.cam_vh_de_mode_en().bit()),
            )
            .field("cam_start", &format_args!("{}", self.cam_start().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CAM_CTRL1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:15 - Configure camera received data byte length. When the length of received data reaches this value + 1, GDMA in_suc_eof_int is triggered."]
    #[inline(always)]
    #[must_use]
    pub fn cam_rec_data_bytelen(&mut self) -> CAM_REC_DATA_BYTELEN_W<CAM_CTRL1_SPEC> {
        CAM_REC_DATA_BYTELEN_W::new(self, 0)
    }
    #[doc = "Bits 16:21 - Configure line number. When the number of received lines reaches this value + 1, LCD_CAM_CAM_HS_INT is triggered."]
    #[inline(always)]
    #[must_use]
    pub fn cam_line_int_num(&mut self) -> CAM_LINE_INT_NUM_W<CAM_CTRL1_SPEC> {
        CAM_LINE_INT_NUM_W::new(self, 16)
    }
    #[doc = "Bit 22 - 1: Invert the input signal CAM_PCLK. 0: Not invert."]
    #[inline(always)]
    #[must_use]
    pub fn cam_clk_inv(&mut self) -> CAM_CLK_INV_W<CAM_CTRL1_SPEC> {
        CAM_CLK_INV_W::new(self, 22)
    }
    #[doc = "Bit 23 - 1: Enable CAM_VSYNC filter function. 0: bypass."]
    #[inline(always)]
    #[must_use]
    pub fn cam_vsync_filter_en(&mut self) -> CAM_VSYNC_FILTER_EN_W<CAM_CTRL1_SPEC> {
        CAM_VSYNC_FILTER_EN_W::new(self, 23)
    }
    #[doc = "Bit 24 - 1: The width of input data is 16 bits. 0: The width of input data is 8 bits."]
    #[inline(always)]
    #[must_use]
    pub fn cam_2byte_en(&mut self) -> CAM_2BYTE_EN_W<CAM_CTRL1_SPEC> {
        CAM_2BYTE_EN_W::new(self, 24)
    }
    #[doc = "Bit 25 - CAM_DE invert enable signal, valid in high level."]
    #[inline(always)]
    #[must_use]
    pub fn cam_de_inv(&mut self) -> CAM_DE_INV_W<CAM_CTRL1_SPEC> {
        CAM_DE_INV_W::new(self, 25)
    }
    #[doc = "Bit 26 - CAM_HSYNC invert enable signal, valid in high level."]
    #[inline(always)]
    #[must_use]
    pub fn cam_hsync_inv(&mut self) -> CAM_HSYNC_INV_W<CAM_CTRL1_SPEC> {
        CAM_HSYNC_INV_W::new(self, 26)
    }
    #[doc = "Bit 27 - CAM_VSYNC invert enable signal, valid in high level."]
    #[inline(always)]
    #[must_use]
    pub fn cam_vsync_inv(&mut self) -> CAM_VSYNC_INV_W<CAM_CTRL1_SPEC> {
        CAM_VSYNC_INV_W::new(self, 27)
    }
    #[doc = "Bit 28 - 1: Input control signals are CAM_DE CAM_HSYNC and CAM_VSYNC is 1. 0: Input control signals are CAM_DE and CAM_VSYNC. CAM_HSYNC and CAM_DE are all 1 the the same time."]
    #[inline(always)]
    #[must_use]
    pub fn cam_vh_de_mode_en(&mut self) -> CAM_VH_DE_MODE_EN_W<CAM_CTRL1_SPEC> {
        CAM_VH_DE_MODE_EN_W::new(self, 28)
    }
    #[doc = "Bit 29 - Camera module start signal."]
    #[inline(always)]
    #[must_use]
    pub fn cam_start(&mut self) -> CAM_START_W<CAM_CTRL1_SPEC> {
        CAM_START_W::new(self, 29)
    }
    #[doc = "Bit 30 - Camera module reset signal."]
    #[inline(always)]
    #[must_use]
    pub fn cam_reset(&mut self) -> CAM_RESET_W<CAM_CTRL1_SPEC> {
        CAM_RESET_W::new(self, 30)
    }
    #[doc = "Bit 31 - Camera Async Rx FIFO reset signal."]
    #[inline(always)]
    #[must_use]
    pub fn cam_afifo_reset(&mut self) -> CAM_AFIFO_RESET_W<CAM_CTRL1_SPEC> {
        CAM_AFIFO_RESET_W::new(self, 31)
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
#[doc = "Camera configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cam_ctrl1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cam_ctrl1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CAM_CTRL1_SPEC;
impl crate::RegisterSpec for CAM_CTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cam_ctrl1::R`](R) reader structure"]
impl crate::Readable for CAM_CTRL1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cam_ctrl1::W`](W) writer structure"]
impl crate::Writable for CAM_CTRL1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CAM_CTRL1 to value 0"]
impl crate::Resettable for CAM_CTRL1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
