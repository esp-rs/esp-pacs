#[doc = "Register `CLK_CONF_FORCE_ON` reader"]
pub type R = crate::R<CLK_CONF_FORCE_ON_SPEC>;
#[doc = "Register `CLK_CONF_FORCE_ON` writer"]
pub type W = crate::W<CLK_CONF_FORCE_ON_SPEC>;
#[doc = "Field `CLK_COEX_FO` reader - "]
pub type CLK_COEX_FO_R = crate::BitReader;
#[doc = "Field `CLK_COEX_FO` writer - "]
pub type CLK_COEX_FO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_I2C_MST_FO` reader - "]
pub type CLK_I2C_MST_FO_R = crate::BitReader;
#[doc = "Field `CLK_I2C_MST_FO` writer - "]
pub type CLK_I2C_MST_FO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_FE_MEM_FO` reader - "]
pub type CLK_FE_MEM_FO_R = crate::BitReader;
#[doc = "Field `CLK_FE_MEM_FO` writer - "]
pub type CLK_FE_MEM_FO_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn clk_coex_fo(&self) -> CLK_COEX_FO_R {
        CLK_COEX_FO_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn clk_i2c_mst_fo(&self) -> CLK_I2C_MST_FO_R {
        CLK_I2C_MST_FO_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn clk_fe_mem_fo(&self) -> CLK_FE_MEM_FO_R {
        CLK_FE_MEM_FO_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLK_CONF_FORCE_ON")
            .field("clk_coex_fo", &self.clk_coex_fo())
            .field("clk_i2c_mst_fo", &self.clk_i2c_mst_fo())
            .field("clk_fe_mem_fo", &self.clk_fe_mem_fo())
            .finish()
    }
}
impl W {
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn clk_coex_fo(&mut self) -> CLK_COEX_FO_W<CLK_CONF_FORCE_ON_SPEC> {
        CLK_COEX_FO_W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn clk_i2c_mst_fo(&mut self) -> CLK_I2C_MST_FO_W<CLK_CONF_FORCE_ON_SPEC> {
        CLK_I2C_MST_FO_W::new(self, 2)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn clk_fe_mem_fo(&mut self) -> CLK_FE_MEM_FO_W<CLK_CONF_FORCE_ON_SPEC> {
        CLK_FE_MEM_FO_W::new(self, 5)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_conf_force_on::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_conf_force_on::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLK_CONF_FORCE_ON_SPEC;
impl crate::RegisterSpec for CLK_CONF_FORCE_ON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_conf_force_on::R`](R) reader structure"]
impl crate::Readable for CLK_CONF_FORCE_ON_SPEC {}
#[doc = "`write(|w| ..)` method takes [`clk_conf_force_on::W`](W) writer structure"]
impl crate::Writable for CLK_CONF_FORCE_ON_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLK_CONF_FORCE_ON to value 0"]
impl crate::Resettable for CLK_CONF_FORCE_ON_SPEC {
    const RESET_VALUE: u32 = 0;
}
