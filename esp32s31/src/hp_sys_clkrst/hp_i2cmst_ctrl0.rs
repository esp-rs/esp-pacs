#[doc = "Register `HP_I2CMST_CTRL0` reader"]
pub type R = crate::R<HP_I2CMST_CTRL0_SPEC>;
#[doc = "Register `HP_I2CMST_CTRL0` writer"]
pub type W = crate::W<HP_I2CMST_CTRL0_SPEC>;
#[doc = "Field `APB_CLK_EN` reader - need_des"]
pub type APB_CLK_EN_R = crate::BitReader;
#[doc = "Field `APB_CLK_EN` writer - need_des"]
pub type APB_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RST_EN` reader - need_des"]
pub type RST_EN_R = crate::BitReader;
#[doc = "Field `RST_EN` writer - need_des"]
pub type RST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_NORST` reader - need_des"]
pub type FORCE_NORST_R = crate::BitReader;
#[doc = "Field `FORCE_NORST` writer - need_des"]
pub type FORCE_NORST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    pub fn apb_clk_en(&self) -> APB_CLK_EN_R {
        APB_CLK_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - need_des"]
    #[inline(always)]
    pub fn rst_en(&self) -> RST_EN_R {
        RST_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - need_des"]
    #[inline(always)]
    pub fn force_norst(&self) -> FORCE_NORST_R {
        FORCE_NORST_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HP_I2CMST_CTRL0")
            .field("apb_clk_en", &self.apb_clk_en())
            .field("rst_en", &self.rst_en())
            .field("force_norst", &self.force_norst())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    pub fn apb_clk_en(&mut self) -> APB_CLK_EN_W<'_, HP_I2CMST_CTRL0_SPEC> {
        APB_CLK_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - need_des"]
    #[inline(always)]
    pub fn rst_en(&mut self) -> RST_EN_W<'_, HP_I2CMST_CTRL0_SPEC> {
        RST_EN_W::new(self, 1)
    }
    #[doc = "Bit 2 - need_des"]
    #[inline(always)]
    pub fn force_norst(&mut self) -> FORCE_NORST_W<'_, HP_I2CMST_CTRL0_SPEC> {
        FORCE_NORST_W::new(self, 2)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_i2cmst_ctrl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_i2cmst_ctrl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HP_I2CMST_CTRL0_SPEC;
impl crate::RegisterSpec for HP_I2CMST_CTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hp_i2cmst_ctrl0::R`](R) reader structure"]
impl crate::Readable for HP_I2CMST_CTRL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hp_i2cmst_ctrl0::W`](W) writer structure"]
impl crate::Writable for HP_I2CMST_CTRL0_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HP_I2CMST_CTRL0 to value 0x01"]
impl crate::Resettable for HP_I2CMST_CTRL0_SPEC {
    const RESET_VALUE: u32 = 0x01;
}
