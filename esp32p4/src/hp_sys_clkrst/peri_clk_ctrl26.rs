#[doc = "Register `PERI_CLK_CTRL26` reader"]
pub type R = crate::R<PERI_CLK_CTRL26_SPEC>;
#[doc = "Register `PERI_CLK_CTRL26` writer"]
pub type W = crate::W<PERI_CLK_CTRL26_SPEC>;
#[doc = "Field `ISP_CLK_DIV_NUM` reader - Reserved"]
pub type ISP_CLK_DIV_NUM_R = crate::FieldReader;
#[doc = "Field `ISP_CLK_DIV_NUM` writer - Reserved"]
pub type ISP_CLK_DIV_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `IOMUX_CLK_SRC_SEL` reader - Reserved"]
pub type IOMUX_CLK_SRC_SEL_R = crate::BitReader;
#[doc = "Field `IOMUX_CLK_SRC_SEL` writer - Reserved"]
pub type IOMUX_CLK_SRC_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOMUX_CLK_EN` reader - Reserved"]
pub type IOMUX_CLK_EN_R = crate::BitReader;
#[doc = "Field `IOMUX_CLK_EN` writer - Reserved"]
pub type IOMUX_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOMUX_CLK_DIV_NUM` reader - Reserved"]
pub type IOMUX_CLK_DIV_NUM_R = crate::FieldReader;
#[doc = "Field `IOMUX_CLK_DIV_NUM` writer - Reserved"]
pub type IOMUX_CLK_DIV_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `H264_CLK_SRC_SEL` reader - Reserved"]
pub type H264_CLK_SRC_SEL_R = crate::BitReader;
#[doc = "Field `H264_CLK_SRC_SEL` writer - Reserved"]
pub type H264_CLK_SRC_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `H264_CLK_EN` reader - Reserved"]
pub type H264_CLK_EN_R = crate::BitReader;
#[doc = "Field `H264_CLK_EN` writer - Reserved"]
pub type H264_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `H264_CLK_DIV_NUM` reader - Reserved"]
pub type H264_CLK_DIV_NUM_R = crate::FieldReader;
#[doc = "Field `H264_CLK_DIV_NUM` writer - Reserved"]
pub type H264_CLK_DIV_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PADBIST_RX_CLK_SRC_SEL` reader - Reserved"]
pub type PADBIST_RX_CLK_SRC_SEL_R = crate::BitReader;
#[doc = "Field `PADBIST_RX_CLK_SRC_SEL` writer - Reserved"]
pub type PADBIST_RX_CLK_SRC_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADBIST_RX_CLK_EN` reader - Reserved"]
pub type PADBIST_RX_CLK_EN_R = crate::BitReader;
#[doc = "Field `PADBIST_RX_CLK_EN` writer - Reserved"]
pub type PADBIST_RX_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Reserved"]
    #[inline(always)]
    pub fn isp_clk_div_num(&self) -> ISP_CLK_DIV_NUM_R {
        ISP_CLK_DIV_NUM_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - Reserved"]
    #[inline(always)]
    pub fn iomux_clk_src_sel(&self) -> IOMUX_CLK_SRC_SEL_R {
        IOMUX_CLK_SRC_SEL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Reserved"]
    #[inline(always)]
    pub fn iomux_clk_en(&self) -> IOMUX_CLK_EN_R {
        IOMUX_CLK_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:17 - Reserved"]
    #[inline(always)]
    pub fn iomux_clk_div_num(&self) -> IOMUX_CLK_DIV_NUM_R {
        IOMUX_CLK_DIV_NUM_R::new(((self.bits >> 10) & 0xff) as u8)
    }
    #[doc = "Bit 18 - Reserved"]
    #[inline(always)]
    pub fn h264_clk_src_sel(&self) -> H264_CLK_SRC_SEL_R {
        H264_CLK_SRC_SEL_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Reserved"]
    #[inline(always)]
    pub fn h264_clk_en(&self) -> H264_CLK_EN_R {
        H264_CLK_EN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:27 - Reserved"]
    #[inline(always)]
    pub fn h264_clk_div_num(&self) -> H264_CLK_DIV_NUM_R {
        H264_CLK_DIV_NUM_R::new(((self.bits >> 20) & 0xff) as u8)
    }
    #[doc = "Bit 28 - Reserved"]
    #[inline(always)]
    pub fn padbist_rx_clk_src_sel(&self) -> PADBIST_RX_CLK_SRC_SEL_R {
        PADBIST_RX_CLK_SRC_SEL_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Reserved"]
    #[inline(always)]
    pub fn padbist_rx_clk_en(&self) -> PADBIST_RX_CLK_EN_R {
        PADBIST_RX_CLK_EN_R::new(((self.bits >> 29) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PERI_CLK_CTRL26")
            .field(
                "isp_clk_div_num",
                &format_args!("{}", self.isp_clk_div_num().bits()),
            )
            .field(
                "iomux_clk_src_sel",
                &format_args!("{}", self.iomux_clk_src_sel().bit()),
            )
            .field(
                "iomux_clk_en",
                &format_args!("{}", self.iomux_clk_en().bit()),
            )
            .field(
                "iomux_clk_div_num",
                &format_args!("{}", self.iomux_clk_div_num().bits()),
            )
            .field(
                "h264_clk_src_sel",
                &format_args!("{}", self.h264_clk_src_sel().bit()),
            )
            .field("h264_clk_en", &format_args!("{}", self.h264_clk_en().bit()))
            .field(
                "h264_clk_div_num",
                &format_args!("{}", self.h264_clk_div_num().bits()),
            )
            .field(
                "padbist_rx_clk_src_sel",
                &format_args!("{}", self.padbist_rx_clk_src_sel().bit()),
            )
            .field(
                "padbist_rx_clk_en",
                &format_args!("{}", self.padbist_rx_clk_en().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PERI_CLK_CTRL26_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:7 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn isp_clk_div_num(&mut self) -> ISP_CLK_DIV_NUM_W<PERI_CLK_CTRL26_SPEC> {
        ISP_CLK_DIV_NUM_W::new(self, 0)
    }
    #[doc = "Bit 8 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn iomux_clk_src_sel(&mut self) -> IOMUX_CLK_SRC_SEL_W<PERI_CLK_CTRL26_SPEC> {
        IOMUX_CLK_SRC_SEL_W::new(self, 8)
    }
    #[doc = "Bit 9 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn iomux_clk_en(&mut self) -> IOMUX_CLK_EN_W<PERI_CLK_CTRL26_SPEC> {
        IOMUX_CLK_EN_W::new(self, 9)
    }
    #[doc = "Bits 10:17 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn iomux_clk_div_num(&mut self) -> IOMUX_CLK_DIV_NUM_W<PERI_CLK_CTRL26_SPEC> {
        IOMUX_CLK_DIV_NUM_W::new(self, 10)
    }
    #[doc = "Bit 18 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn h264_clk_src_sel(&mut self) -> H264_CLK_SRC_SEL_W<PERI_CLK_CTRL26_SPEC> {
        H264_CLK_SRC_SEL_W::new(self, 18)
    }
    #[doc = "Bit 19 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn h264_clk_en(&mut self) -> H264_CLK_EN_W<PERI_CLK_CTRL26_SPEC> {
        H264_CLK_EN_W::new(self, 19)
    }
    #[doc = "Bits 20:27 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn h264_clk_div_num(&mut self) -> H264_CLK_DIV_NUM_W<PERI_CLK_CTRL26_SPEC> {
        H264_CLK_DIV_NUM_W::new(self, 20)
    }
    #[doc = "Bit 28 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn padbist_rx_clk_src_sel(&mut self) -> PADBIST_RX_CLK_SRC_SEL_W<PERI_CLK_CTRL26_SPEC> {
        PADBIST_RX_CLK_SRC_SEL_W::new(self, 28)
    }
    #[doc = "Bit 29 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn padbist_rx_clk_en(&mut self) -> PADBIST_RX_CLK_EN_W<PERI_CLK_CTRL26_SPEC> {
        PADBIST_RX_CLK_EN_W::new(self, 29)
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
#[doc = "Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`peri_clk_ctrl26::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`peri_clk_ctrl26::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PERI_CLK_CTRL26_SPEC;
impl crate::RegisterSpec for PERI_CLK_CTRL26_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`peri_clk_ctrl26::R`](R) reader structure"]
impl crate::Readable for PERI_CLK_CTRL26_SPEC {}
#[doc = "`write(|w| ..)` method takes [`peri_clk_ctrl26::W`](W) writer structure"]
impl crate::Writable for PERI_CLK_CTRL26_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PERI_CLK_CTRL26 to value 0x0200"]
impl crate::Resettable for PERI_CLK_CTRL26_SPEC {
    const RESET_VALUE: u32 = 0x0200;
}
