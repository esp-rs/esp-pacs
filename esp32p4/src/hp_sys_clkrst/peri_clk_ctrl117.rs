#[doc = "Register `PERI_CLK_CTRL117` reader"]
pub type R = crate::R<PERI_CLK_CTRL117_SPEC>;
#[doc = "Register `PERI_CLK_CTRL117` writer"]
pub type W = crate::W<PERI_CLK_CTRL117_SPEC>;
#[doc = "Field `GPSPI3_HS_CLK_DIV_NUM` reader - Reserved"]
pub type GPSPI3_HS_CLK_DIV_NUM_R = crate::FieldReader;
#[doc = "Field `GPSPI3_HS_CLK_DIV_NUM` writer - Reserved"]
pub type GPSPI3_HS_CLK_DIV_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPSPI3_MST_CLK_DIV_NUM` reader - Reserved"]
pub type GPSPI3_MST_CLK_DIV_NUM_R = crate::FieldReader;
#[doc = "Field `GPSPI3_MST_CLK_DIV_NUM` writer - Reserved"]
pub type GPSPI3_MST_CLK_DIV_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPSPI3_MST_CLK_EN` reader - Reserved"]
pub type GPSPI3_MST_CLK_EN_R = crate::BitReader;
#[doc = "Field `GPSPI3_MST_CLK_EN` writer - Reserved"]
pub type GPSPI3_MST_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PARLIO_RX_CLK_SRC_SEL` reader - Reserved"]
pub type PARLIO_RX_CLK_SRC_SEL_R = crate::FieldReader;
#[doc = "Field `PARLIO_RX_CLK_SRC_SEL` writer - Reserved"]
pub type PARLIO_RX_CLK_SRC_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PARLIO_RX_CLK_EN` reader - Reserved"]
pub type PARLIO_RX_CLK_EN_R = crate::BitReader;
#[doc = "Field `PARLIO_RX_CLK_EN` writer - Reserved"]
pub type PARLIO_RX_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PARLIO_RX_CLK_DIV_NUM` reader - Reserved"]
pub type PARLIO_RX_CLK_DIV_NUM_R = crate::FieldReader;
#[doc = "Field `PARLIO_RX_CLK_DIV_NUM` writer - Reserved"]
pub type PARLIO_RX_CLK_DIV_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Reserved"]
    #[inline(always)]
    pub fn gpspi3_hs_clk_div_num(&self) -> GPSPI3_HS_CLK_DIV_NUM_R {
        GPSPI3_HS_CLK_DIV_NUM_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Reserved"]
    #[inline(always)]
    pub fn gpspi3_mst_clk_div_num(&self) -> GPSPI3_MST_CLK_DIV_NUM_R {
        GPSPI3_MST_CLK_DIV_NUM_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16 - Reserved"]
    #[inline(always)]
    pub fn gpspi3_mst_clk_en(&self) -> GPSPI3_MST_CLK_EN_R {
        GPSPI3_MST_CLK_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:18 - Reserved"]
    #[inline(always)]
    pub fn parlio_rx_clk_src_sel(&self) -> PARLIO_RX_CLK_SRC_SEL_R {
        PARLIO_RX_CLK_SRC_SEL_R::new(((self.bits >> 17) & 3) as u8)
    }
    #[doc = "Bit 19 - Reserved"]
    #[inline(always)]
    pub fn parlio_rx_clk_en(&self) -> PARLIO_RX_CLK_EN_R {
        PARLIO_RX_CLK_EN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:27 - Reserved"]
    #[inline(always)]
    pub fn parlio_rx_clk_div_num(&self) -> PARLIO_RX_CLK_DIV_NUM_R {
        PARLIO_RX_CLK_DIV_NUM_R::new(((self.bits >> 20) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PERI_CLK_CTRL117")
            .field(
                "gpspi3_hs_clk_div_num",
                &format_args!("{}", self.gpspi3_hs_clk_div_num().bits()),
            )
            .field(
                "gpspi3_mst_clk_div_num",
                &format_args!("{}", self.gpspi3_mst_clk_div_num().bits()),
            )
            .field(
                "gpspi3_mst_clk_en",
                &format_args!("{}", self.gpspi3_mst_clk_en().bit()),
            )
            .field(
                "parlio_rx_clk_src_sel",
                &format_args!("{}", self.parlio_rx_clk_src_sel().bits()),
            )
            .field(
                "parlio_rx_clk_en",
                &format_args!("{}", self.parlio_rx_clk_en().bit()),
            )
            .field(
                "parlio_rx_clk_div_num",
                &format_args!("{}", self.parlio_rx_clk_div_num().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PERI_CLK_CTRL117_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:7 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn gpspi3_hs_clk_div_num(&mut self) -> GPSPI3_HS_CLK_DIV_NUM_W<PERI_CLK_CTRL117_SPEC> {
        GPSPI3_HS_CLK_DIV_NUM_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn gpspi3_mst_clk_div_num(&mut self) -> GPSPI3_MST_CLK_DIV_NUM_W<PERI_CLK_CTRL117_SPEC> {
        GPSPI3_MST_CLK_DIV_NUM_W::new(self, 8)
    }
    #[doc = "Bit 16 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn gpspi3_mst_clk_en(&mut self) -> GPSPI3_MST_CLK_EN_W<PERI_CLK_CTRL117_SPEC> {
        GPSPI3_MST_CLK_EN_W::new(self, 16)
    }
    #[doc = "Bits 17:18 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn parlio_rx_clk_src_sel(&mut self) -> PARLIO_RX_CLK_SRC_SEL_W<PERI_CLK_CTRL117_SPEC> {
        PARLIO_RX_CLK_SRC_SEL_W::new(self, 17)
    }
    #[doc = "Bit 19 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn parlio_rx_clk_en(&mut self) -> PARLIO_RX_CLK_EN_W<PERI_CLK_CTRL117_SPEC> {
        PARLIO_RX_CLK_EN_W::new(self, 19)
    }
    #[doc = "Bits 20:27 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn parlio_rx_clk_div_num(&mut self) -> PARLIO_RX_CLK_DIV_NUM_W<PERI_CLK_CTRL117_SPEC> {
        PARLIO_RX_CLK_DIV_NUM_W::new(self, 20)
    }
}
#[doc = "Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`peri_clk_ctrl117::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`peri_clk_ctrl117::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PERI_CLK_CTRL117_SPEC;
impl crate::RegisterSpec for PERI_CLK_CTRL117_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`peri_clk_ctrl117::R`](R) reader structure"]
impl crate::Readable for PERI_CLK_CTRL117_SPEC {}
#[doc = "`write(|w| ..)` method takes [`peri_clk_ctrl117::W`](W) writer structure"]
impl crate::Writable for PERI_CLK_CTRL117_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PERI_CLK_CTRL117 to value 0x0001_0000"]
impl crate::Resettable for PERI_CLK_CTRL117_SPEC {
    const RESET_VALUE: u32 = 0x0001_0000;
}
