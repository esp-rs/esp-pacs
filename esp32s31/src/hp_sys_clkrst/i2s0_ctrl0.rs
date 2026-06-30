#[doc = "Register `I2S0_CTRL0` reader"]
pub type R = crate::R<I2S0_CTRL0_SPEC>;
#[doc = "Register `I2S0_CTRL0` writer"]
pub type W = crate::W<I2S0_CTRL0_SPEC>;
#[doc = "Field `REG_I2S0_APB_CLK_EN` reader - need_des"]
pub type REG_I2S0_APB_CLK_EN_R = crate::BitReader;
#[doc = "Field `REG_I2S0_APB_CLK_EN` writer - need_des"]
pub type REG_I2S0_APB_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_I2S0_APB_RST_EN` reader - need_des"]
pub type REG_I2S0_APB_RST_EN_R = crate::BitReader;
#[doc = "Field `REG_I2S0_APB_RST_EN` writer - need_des"]
pub type REG_I2S0_APB_RST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_I2S0_FORCE_NORST` reader - need_des"]
pub type REG_I2S0_FORCE_NORST_R = crate::BitReader;
#[doc = "Field `REG_I2S0_FORCE_NORST` writer - need_des"]
pub type REG_I2S0_FORCE_NORST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    pub fn reg_i2s0_apb_clk_en(&self) -> REG_I2S0_APB_CLK_EN_R {
        REG_I2S0_APB_CLK_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - need_des"]
    #[inline(always)]
    pub fn reg_i2s0_apb_rst_en(&self) -> REG_I2S0_APB_RST_EN_R {
        REG_I2S0_APB_RST_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - need_des"]
    #[inline(always)]
    pub fn reg_i2s0_force_norst(&self) -> REG_I2S0_FORCE_NORST_R {
        REG_I2S0_FORCE_NORST_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I2S0_CTRL0")
            .field("reg_i2s0_apb_clk_en", &self.reg_i2s0_apb_clk_en())
            .field("reg_i2s0_apb_rst_en", &self.reg_i2s0_apb_rst_en())
            .field("reg_i2s0_force_norst", &self.reg_i2s0_force_norst())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    pub fn reg_i2s0_apb_clk_en(&mut self) -> REG_I2S0_APB_CLK_EN_W<'_, I2S0_CTRL0_SPEC> {
        REG_I2S0_APB_CLK_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - need_des"]
    #[inline(always)]
    pub fn reg_i2s0_apb_rst_en(&mut self) -> REG_I2S0_APB_RST_EN_W<'_, I2S0_CTRL0_SPEC> {
        REG_I2S0_APB_RST_EN_W::new(self, 1)
    }
    #[doc = "Bit 2 - need_des"]
    #[inline(always)]
    pub fn reg_i2s0_force_norst(&mut self) -> REG_I2S0_FORCE_NORST_W<'_, I2S0_CTRL0_SPEC> {
        REG_I2S0_FORCE_NORST_W::new(self, 2)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`i2s0_ctrl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2s0_ctrl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2S0_CTRL0_SPEC;
impl crate::RegisterSpec for I2S0_CTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2s0_ctrl0::R`](R) reader structure"]
impl crate::Readable for I2S0_CTRL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`i2s0_ctrl0::W`](W) writer structure"]
impl crate::Writable for I2S0_CTRL0_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2S0_CTRL0 to value 0"]
impl crate::Resettable for I2S0_CTRL0_SPEC {}
