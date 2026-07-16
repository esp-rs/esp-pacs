#[doc = "Register `I2CMST_CTRL` reader"]
pub type R = crate::R<I2CMST_CTRL_SPEC>;
#[doc = "Register `I2CMST_CTRL` writer"]
pub type W = crate::W<I2CMST_CTRL_SPEC>;
#[doc = "Field `LP_I2CMST_CLK_EN` reader - need_des"]
pub type LP_I2CMST_CLK_EN_R = crate::BitReader;
#[doc = "Field `LP_I2CMST_CLK_EN` writer - need_des"]
pub type LP_I2CMST_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_I2CMST_RST_EN` reader - need_des"]
pub type LP_I2CMST_RST_EN_R = crate::BitReader;
#[doc = "Field `LP_I2CMST_RST_EN` writer - need_des"]
pub type LP_I2CMST_RST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    pub fn lp_i2cmst_clk_en(&self) -> LP_I2CMST_CLK_EN_R {
        LP_I2CMST_CLK_EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn lp_i2cmst_rst_en(&self) -> LP_I2CMST_RST_EN_R {
        LP_I2CMST_RST_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I2CMST_CTRL")
            .field("lp_i2cmst_clk_en", &self.lp_i2cmst_clk_en())
            .field("lp_i2cmst_rst_en", &self.lp_i2cmst_rst_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    pub fn lp_i2cmst_clk_en(&mut self) -> LP_I2CMST_CLK_EN_W<'_, I2CMST_CTRL_SPEC> {
        LP_I2CMST_CLK_EN_W::new(self, 30)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn lp_i2cmst_rst_en(&mut self) -> LP_I2CMST_RST_EN_W<'_, I2CMST_CTRL_SPEC> {
        LP_I2CMST_RST_EN_W::new(self, 31)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`i2cmst_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2cmst_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2CMST_CTRL_SPEC;
impl crate::RegisterSpec for I2CMST_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2cmst_ctrl::R`](R) reader structure"]
impl crate::Readable for I2CMST_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`i2cmst_ctrl::W`](W) writer structure"]
impl crate::Writable for I2CMST_CTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2CMST_CTRL to value 0x4000_0000"]
impl crate::Resettable for I2CMST_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x4000_0000;
}
