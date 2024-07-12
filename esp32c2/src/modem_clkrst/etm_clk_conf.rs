#[doc = "Register `ETM_CLK_CONF` reader"]
pub type R = crate::R<ETM_CLK_CONF_SPEC>;
#[doc = "Register `ETM_CLK_CONF` writer"]
pub type W = crate::W<ETM_CLK_CONF_SPEC>;
#[doc = "Field `ETM_CLK_SEL` reader - ."]
pub type ETM_CLK_SEL_R = crate::BitReader;
#[doc = "Field `ETM_CLK_SEL` writer - ."]
pub type ETM_CLK_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETM_CLK_ACTIVE` reader - ."]
pub type ETM_CLK_ACTIVE_R = crate::BitReader;
#[doc = "Field `ETM_CLK_ACTIVE` writer - ."]
pub type ETM_CLK_ACTIVE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - ."]
    #[inline(always)]
    pub fn etm_clk_sel(&self) -> ETM_CLK_SEL_R {
        ETM_CLK_SEL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ."]
    #[inline(always)]
    pub fn etm_clk_active(&self) -> ETM_CLK_ACTIVE_R {
        ETM_CLK_ACTIVE_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ETM_CLK_CONF")
            .field("etm_clk_sel", &self.etm_clk_sel())
            .field("etm_clk_active", &self.etm_clk_active())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - ."]
    #[inline(always)]
    #[must_use]
    pub fn etm_clk_sel(&mut self) -> ETM_CLK_SEL_W<ETM_CLK_CONF_SPEC> {
        ETM_CLK_SEL_W::new(self, 0)
    }
    #[doc = "Bit 1 - ."]
    #[inline(always)]
    #[must_use]
    pub fn etm_clk_active(&mut self) -> ETM_CLK_ACTIVE_W<ETM_CLK_CONF_SPEC> {
        ETM_CLK_ACTIVE_W::new(self, 1)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`etm_clk_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`etm_clk_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ETM_CLK_CONF_SPEC;
impl crate::RegisterSpec for ETM_CLK_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`etm_clk_conf::R`](R) reader structure"]
impl crate::Readable for ETM_CLK_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`etm_clk_conf::W`](W) writer structure"]
impl crate::Writable for ETM_CLK_CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ETM_CLK_CONF to value 0"]
impl crate::Resettable for ETM_CLK_CONF_SPEC {
    const RESET_VALUE: u32 = 0;
}
