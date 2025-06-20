#[doc = "Register `BS_CONF` reader"]
pub type R = crate::R<BS_CONF_SPEC>;
#[doc = "Register `BS_CONF` writer"]
pub type W = crate::W<BS_CONF_SPEC>;
#[doc = "Field `BS_CLK_EN` reader - Set 1 to enable bs clock"]
pub type BS_CLK_EN_R = crate::BitReader;
#[doc = "Field `BS_CLK_EN` writer - Set 1 to enable bs clock"]
pub type BS_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BS_RST_EN` reader - Set 0 to reset bs module"]
pub type BS_RST_EN_R = crate::BitReader;
#[doc = "Field `BS_RST_EN` writer - Set 0 to reset bs module"]
pub type BS_RST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Set 1 to enable bs clock"]
    #[inline(always)]
    pub fn bs_clk_en(&self) -> BS_CLK_EN_R {
        BS_CLK_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set 0 to reset bs module"]
    #[inline(always)]
    pub fn bs_rst_en(&self) -> BS_RST_EN_R {
        BS_RST_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BS_CONF")
            .field("bs_clk_en", &self.bs_clk_en())
            .field("bs_rst_en", &self.bs_rst_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Set 1 to enable bs clock"]
    #[inline(always)]
    pub fn bs_clk_en(&mut self) -> BS_CLK_EN_W<BS_CONF_SPEC> {
        BS_CLK_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Set 0 to reset bs module"]
    #[inline(always)]
    pub fn bs_rst_en(&mut self) -> BS_RST_EN_W<BS_CONF_SPEC> {
        BS_RST_EN_W::new(self, 1)
    }
}
#[doc = "BS configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`bs_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bs_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BS_CONF_SPEC;
impl crate::RegisterSpec for BS_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bs_conf::R`](R) reader structure"]
impl crate::Readable for BS_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bs_conf::W`](W) writer structure"]
impl crate::Writable for BS_CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BS_CONF to value 0"]
impl crate::Resettable for BS_CONF_SPEC {
    const RESET_VALUE: u32 = 0;
}
