#[doc = "Register `I2C_CTRL` reader"]
pub type R = crate::R<I2C_CTRL_SPEC>;
#[doc = "Register `I2C_CTRL` writer"]
pub type W = crate::W<I2C_CTRL_SPEC>;
#[doc = "Field `LP_I2C_CLK_DIV_NUM` reader - need_des"]
pub type LP_I2C_CLK_DIV_NUM_R = crate::FieldReader;
#[doc = "Field `LP_I2C_CLK_DIV_NUM` writer - need_des"]
pub type LP_I2C_CLK_DIV_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `LP_I2C_CLK_SEL` reader - need_des"]
pub type LP_I2C_CLK_SEL_R = crate::FieldReader;
#[doc = "Field `LP_I2C_CLK_SEL` writer - need_des"]
pub type LP_I2C_CLK_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LP_I2C_CLK_EN` reader - need_des"]
pub type LP_I2C_CLK_EN_R = crate::BitReader;
#[doc = "Field `LP_I2C_CLK_EN` writer - need_des"]
pub type LP_I2C_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_I2C_RST_EN` reader - need_des"]
pub type LP_I2C_RST_EN_R = crate::BitReader;
#[doc = "Field `LP_I2C_RST_EN` writer - need_des"]
pub type LP_I2C_RST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 20:27 - need_des"]
    #[inline(always)]
    pub fn lp_i2c_clk_div_num(&self) -> LP_I2C_CLK_DIV_NUM_R {
        LP_I2C_CLK_DIV_NUM_R::new(((self.bits >> 20) & 0xff) as u8)
    }
    #[doc = "Bits 28:29 - need_des"]
    #[inline(always)]
    pub fn lp_i2c_clk_sel(&self) -> LP_I2C_CLK_SEL_R {
        LP_I2C_CLK_SEL_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    pub fn lp_i2c_clk_en(&self) -> LP_I2C_CLK_EN_R {
        LP_I2C_CLK_EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn lp_i2c_rst_en(&self) -> LP_I2C_RST_EN_R {
        LP_I2C_RST_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I2C_CTRL")
            .field("lp_i2c_clk_div_num", &self.lp_i2c_clk_div_num())
            .field("lp_i2c_clk_sel", &self.lp_i2c_clk_sel())
            .field("lp_i2c_clk_en", &self.lp_i2c_clk_en())
            .field("lp_i2c_rst_en", &self.lp_i2c_rst_en())
            .finish()
    }
}
impl W {
    #[doc = "Bits 20:27 - need_des"]
    #[inline(always)]
    pub fn lp_i2c_clk_div_num(&mut self) -> LP_I2C_CLK_DIV_NUM_W<'_, I2C_CTRL_SPEC> {
        LP_I2C_CLK_DIV_NUM_W::new(self, 20)
    }
    #[doc = "Bits 28:29 - need_des"]
    #[inline(always)]
    pub fn lp_i2c_clk_sel(&mut self) -> LP_I2C_CLK_SEL_W<'_, I2C_CTRL_SPEC> {
        LP_I2C_CLK_SEL_W::new(self, 28)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    pub fn lp_i2c_clk_en(&mut self) -> LP_I2C_CLK_EN_W<'_, I2C_CTRL_SPEC> {
        LP_I2C_CLK_EN_W::new(self, 30)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn lp_i2c_rst_en(&mut self) -> LP_I2C_RST_EN_W<'_, I2C_CTRL_SPEC> {
        LP_I2C_RST_EN_W::new(self, 31)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2C_CTRL_SPEC;
impl crate::RegisterSpec for I2C_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c_ctrl::R`](R) reader structure"]
impl crate::Readable for I2C_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`i2c_ctrl::W`](W) writer structure"]
impl crate::Writable for I2C_CTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2C_CTRL to value 0"]
impl crate::Resettable for I2C_CTRL_SPEC {}
