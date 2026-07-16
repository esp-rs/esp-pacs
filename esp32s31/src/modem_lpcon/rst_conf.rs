#[doc = "Register `RST_CONF` reader"]
pub type R = crate::R<RST_CONF_SPEC>;
#[doc = "Register `RST_CONF` writer"]
pub type W = crate::W<RST_CONF_SPEC>;
#[doc = "Field `RST_WIFIPWR` reader - "]
pub type RST_WIFIPWR_R = crate::BitReader;
#[doc = "Field `RST_WIFIPWR` writer - "]
pub type RST_WIFIPWR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RST_COEX` reader - "]
pub type RST_COEX_R = crate::BitReader;
#[doc = "Field `RST_COEX` writer - "]
pub type RST_COEX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RST_I2C_MST` reader - "]
pub type RST_I2C_MST_R = crate::BitReader;
#[doc = "Field `RST_I2C_MST` writer - "]
pub type RST_I2C_MST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RST_LP_TIMER` reader - "]
pub type RST_LP_TIMER_R = crate::BitReader;
#[doc = "Field `RST_LP_TIMER` writer - "]
pub type RST_LP_TIMER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RST_DCMEM` reader - "]
pub type RST_DCMEM_R = crate::BitReader;
#[doc = "Field `RST_DCMEM` writer - "]
pub type RST_DCMEM_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rst_wifipwr(&self) -> RST_WIFIPWR_R {
        RST_WIFIPWR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rst_coex(&self) -> RST_COEX_R {
        RST_COEX_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rst_i2c_mst(&self) -> RST_I2C_MST_R {
        RST_I2C_MST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rst_lp_timer(&self) -> RST_LP_TIMER_R {
        RST_LP_TIMER_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn rst_dcmem(&self) -> RST_DCMEM_R {
        RST_DCMEM_R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RST_CONF")
            .field("rst_wifipwr", &self.rst_wifipwr())
            .field("rst_coex", &self.rst_coex())
            .field("rst_i2c_mst", &self.rst_i2c_mst())
            .field("rst_lp_timer", &self.rst_lp_timer())
            .field("rst_dcmem", &self.rst_dcmem())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rst_wifipwr(&mut self) -> RST_WIFIPWR_W<'_, RST_CONF_SPEC> {
        RST_WIFIPWR_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rst_coex(&mut self) -> RST_COEX_W<'_, RST_CONF_SPEC> {
        RST_COEX_W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rst_i2c_mst(&mut self) -> RST_I2C_MST_W<'_, RST_CONF_SPEC> {
        RST_I2C_MST_W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rst_lp_timer(&mut self) -> RST_LP_TIMER_W<'_, RST_CONF_SPEC> {
        RST_LP_TIMER_W::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn rst_dcmem(&mut self) -> RST_DCMEM_W<'_, RST_CONF_SPEC> {
        RST_DCMEM_W::new(self, 4)
    }
}
#[doc = "RST_CONF\n\nYou can [`read`](crate::Reg::read) this register and get [`rst_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rst_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RST_CONF_SPEC;
impl crate::RegisterSpec for RST_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rst_conf::R`](R) reader structure"]
impl crate::Readable for RST_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rst_conf::W`](W) writer structure"]
impl crate::Writable for RST_CONF_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RST_CONF to value 0"]
impl crate::Resettable for RST_CONF_SPEC {}
