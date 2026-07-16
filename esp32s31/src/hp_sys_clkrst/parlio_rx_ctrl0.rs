#[doc = "Register `PARLIO_RX_CTRL0` reader"]
pub type R = crate::R<PARLIO_RX_CTRL0_SPEC>;
#[doc = "Register `PARLIO_RX_CTRL0` writer"]
pub type W = crate::W<PARLIO_RX_CTRL0_SPEC>;
#[doc = "Field `PARLIO_RX_RST_EN` reader - need_des"]
pub type PARLIO_RX_RST_EN_R = crate::BitReader;
#[doc = "Field `PARLIO_RX_RST_EN` writer - need_des"]
pub type PARLIO_RX_RST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PARLIO_RX_FORCE_NORST` reader - need_des"]
pub type PARLIO_RX_FORCE_NORST_R = crate::BitReader;
#[doc = "Field `PARLIO_RX_FORCE_NORST` writer - need_des"]
pub type PARLIO_RX_FORCE_NORST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PARLIO_RX_CLK_SRC_SEL` reader - need_des"]
pub type PARLIO_RX_CLK_SRC_SEL_R = crate::FieldReader;
#[doc = "Field `PARLIO_RX_CLK_SRC_SEL` writer - need_des"]
pub type PARLIO_RX_CLK_SRC_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PARLIO_RX_CLK_EN` reader - need_des"]
pub type PARLIO_RX_CLK_EN_R = crate::BitReader;
#[doc = "Field `PARLIO_RX_CLK_EN` writer - need_des"]
pub type PARLIO_RX_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PARLIO_RX_CLK_DIV_NUM` reader - need_des"]
pub type PARLIO_RX_CLK_DIV_NUM_R = crate::FieldReader;
#[doc = "Field `PARLIO_RX_CLK_DIV_NUM` writer - need_des"]
pub type PARLIO_RX_CLK_DIV_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PARLIO_RX_CLK_DIV_NUMERATOR` reader - need_des"]
pub type PARLIO_RX_CLK_DIV_NUMERATOR_R = crate::FieldReader;
#[doc = "Field `PARLIO_RX_CLK_DIV_NUMERATOR` writer - need_des"]
pub type PARLIO_RX_CLK_DIV_NUMERATOR_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PARLIO_RX_CLK_DIV_DENOMINATOR` reader - need_des"]
pub type PARLIO_RX_CLK_DIV_DENOMINATOR_R = crate::FieldReader;
#[doc = "Field `PARLIO_RX_CLK_DIV_DENOMINATOR` writer - need_des"]
pub type PARLIO_RX_CLK_DIV_DENOMINATOR_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    pub fn parlio_rx_rst_en(&self) -> PARLIO_RX_RST_EN_R {
        PARLIO_RX_RST_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - need_des"]
    #[inline(always)]
    pub fn parlio_rx_force_norst(&self) -> PARLIO_RX_FORCE_NORST_R {
        PARLIO_RX_FORCE_NORST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - need_des"]
    #[inline(always)]
    pub fn parlio_rx_clk_src_sel(&self) -> PARLIO_RX_CLK_SRC_SEL_R {
        PARLIO_RX_CLK_SRC_SEL_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - need_des"]
    #[inline(always)]
    pub fn parlio_rx_clk_en(&self) -> PARLIO_RX_CLK_EN_R {
        PARLIO_RX_CLK_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:12 - need_des"]
    #[inline(always)]
    pub fn parlio_rx_clk_div_num(&self) -> PARLIO_RX_CLK_DIV_NUM_R {
        PARLIO_RX_CLK_DIV_NUM_R::new(((self.bits >> 5) & 0xff) as u8)
    }
    #[doc = "Bits 13:20 - need_des"]
    #[inline(always)]
    pub fn parlio_rx_clk_div_numerator(&self) -> PARLIO_RX_CLK_DIV_NUMERATOR_R {
        PARLIO_RX_CLK_DIV_NUMERATOR_R::new(((self.bits >> 13) & 0xff) as u8)
    }
    #[doc = "Bits 21:28 - need_des"]
    #[inline(always)]
    pub fn parlio_rx_clk_div_denominator(&self) -> PARLIO_RX_CLK_DIV_DENOMINATOR_R {
        PARLIO_RX_CLK_DIV_DENOMINATOR_R::new(((self.bits >> 21) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PARLIO_RX_CTRL0")
            .field("parlio_rx_rst_en", &self.parlio_rx_rst_en())
            .field("parlio_rx_force_norst", &self.parlio_rx_force_norst())
            .field("parlio_rx_clk_src_sel", &self.parlio_rx_clk_src_sel())
            .field("parlio_rx_clk_en", &self.parlio_rx_clk_en())
            .field("parlio_rx_clk_div_num", &self.parlio_rx_clk_div_num())
            .field(
                "parlio_rx_clk_div_numerator",
                &self.parlio_rx_clk_div_numerator(),
            )
            .field(
                "parlio_rx_clk_div_denominator",
                &self.parlio_rx_clk_div_denominator(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    pub fn parlio_rx_rst_en(&mut self) -> PARLIO_RX_RST_EN_W<'_, PARLIO_RX_CTRL0_SPEC> {
        PARLIO_RX_RST_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - need_des"]
    #[inline(always)]
    pub fn parlio_rx_force_norst(&mut self) -> PARLIO_RX_FORCE_NORST_W<'_, PARLIO_RX_CTRL0_SPEC> {
        PARLIO_RX_FORCE_NORST_W::new(self, 1)
    }
    #[doc = "Bits 2:3 - need_des"]
    #[inline(always)]
    pub fn parlio_rx_clk_src_sel(&mut self) -> PARLIO_RX_CLK_SRC_SEL_W<'_, PARLIO_RX_CTRL0_SPEC> {
        PARLIO_RX_CLK_SRC_SEL_W::new(self, 2)
    }
    #[doc = "Bit 4 - need_des"]
    #[inline(always)]
    pub fn parlio_rx_clk_en(&mut self) -> PARLIO_RX_CLK_EN_W<'_, PARLIO_RX_CTRL0_SPEC> {
        PARLIO_RX_CLK_EN_W::new(self, 4)
    }
    #[doc = "Bits 5:12 - need_des"]
    #[inline(always)]
    pub fn parlio_rx_clk_div_num(&mut self) -> PARLIO_RX_CLK_DIV_NUM_W<'_, PARLIO_RX_CTRL0_SPEC> {
        PARLIO_RX_CLK_DIV_NUM_W::new(self, 5)
    }
    #[doc = "Bits 13:20 - need_des"]
    #[inline(always)]
    pub fn parlio_rx_clk_div_numerator(
        &mut self,
    ) -> PARLIO_RX_CLK_DIV_NUMERATOR_W<'_, PARLIO_RX_CTRL0_SPEC> {
        PARLIO_RX_CLK_DIV_NUMERATOR_W::new(self, 13)
    }
    #[doc = "Bits 21:28 - need_des"]
    #[inline(always)]
    pub fn parlio_rx_clk_div_denominator(
        &mut self,
    ) -> PARLIO_RX_CLK_DIV_DENOMINATOR_W<'_, PARLIO_RX_CTRL0_SPEC> {
        PARLIO_RX_CLK_DIV_DENOMINATOR_W::new(self, 21)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`parlio_rx_ctrl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`parlio_rx_ctrl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PARLIO_RX_CTRL0_SPEC;
impl crate::RegisterSpec for PARLIO_RX_CTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`parlio_rx_ctrl0::R`](R) reader structure"]
impl crate::Readable for PARLIO_RX_CTRL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`parlio_rx_ctrl0::W`](W) writer structure"]
impl crate::Writable for PARLIO_RX_CTRL0_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PARLIO_RX_CTRL0 to value 0"]
impl crate::Resettable for PARLIO_RX_CTRL0_SPEC {}
