#[doc = "Register `DAC_CTRL` reader"]
pub type R = crate::R<DAC_CTRL_SPEC>;
#[doc = "Register `DAC_CTRL` writer"]
pub type W = crate::W<DAC_CTRL_SPEC>;
#[doc = "Field `LP_DAC_DIV_NUM` reader - need_des"]
pub type LP_DAC_DIV_NUM_R = crate::FieldReader;
#[doc = "Field `LP_DAC_DIV_NUM` writer - need_des"]
pub type LP_DAC_DIV_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `LP_DAC_CLK_SEL` reader - need_des"]
pub type LP_DAC_CLK_SEL_R = crate::FieldReader;
#[doc = "Field `LP_DAC_CLK_SEL` writer - need_des"]
pub type LP_DAC_CLK_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LP_DAC_CLK_EN` reader - need_des"]
pub type LP_DAC_CLK_EN_R = crate::BitReader;
#[doc = "Field `LP_DAC_CLK_EN` writer - need_des"]
pub type LP_DAC_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_DAC_RST_EN` reader - need_des"]
pub type LP_DAC_RST_EN_R = crate::BitReader;
#[doc = "Field `LP_DAC_RST_EN` writer - need_des"]
pub type LP_DAC_RST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - need_des"]
    #[inline(always)]
    pub fn lp_dac_div_num(&self) -> LP_DAC_DIV_NUM_R {
        LP_DAC_DIV_NUM_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 28:29 - need_des"]
    #[inline(always)]
    pub fn lp_dac_clk_sel(&self) -> LP_DAC_CLK_SEL_R {
        LP_DAC_CLK_SEL_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    pub fn lp_dac_clk_en(&self) -> LP_DAC_CLK_EN_R {
        LP_DAC_CLK_EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn lp_dac_rst_en(&self) -> LP_DAC_RST_EN_R {
        LP_DAC_RST_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DAC_CTRL")
            .field("lp_dac_div_num", &self.lp_dac_div_num())
            .field("lp_dac_clk_sel", &self.lp_dac_clk_sel())
            .field("lp_dac_clk_en", &self.lp_dac_clk_en())
            .field("lp_dac_rst_en", &self.lp_dac_rst_en())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - need_des"]
    #[inline(always)]
    pub fn lp_dac_div_num(&mut self) -> LP_DAC_DIV_NUM_W<'_, DAC_CTRL_SPEC> {
        LP_DAC_DIV_NUM_W::new(self, 0)
    }
    #[doc = "Bits 28:29 - need_des"]
    #[inline(always)]
    pub fn lp_dac_clk_sel(&mut self) -> LP_DAC_CLK_SEL_W<'_, DAC_CTRL_SPEC> {
        LP_DAC_CLK_SEL_W::new(self, 28)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    pub fn lp_dac_clk_en(&mut self) -> LP_DAC_CLK_EN_W<'_, DAC_CTRL_SPEC> {
        LP_DAC_CLK_EN_W::new(self, 30)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn lp_dac_rst_en(&mut self) -> LP_DAC_RST_EN_W<'_, DAC_CTRL_SPEC> {
        LP_DAC_RST_EN_W::new(self, 31)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`dac_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DAC_CTRL_SPEC;
impl crate::RegisterSpec for DAC_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dac_ctrl::R`](R) reader structure"]
impl crate::Readable for DAC_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dac_ctrl::W`](W) writer structure"]
impl crate::Writable for DAC_CTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DAC_CTRL to value 0x04"]
impl crate::Resettable for DAC_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x04;
}
