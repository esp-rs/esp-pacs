#[doc = "Register `CLK_CONF` reader"]
pub type R = crate::R<CLK_CONF_SPEC>;
#[doc = "Register `CLK_CONF` writer"]
pub type W = crate::W<CLK_CONF_SPEC>;
#[doc = "Field `CLK_EN` reader - Tsens regbank clock gating enable."]
pub type CLK_EN_R = crate::BitReader;
#[doc = "Field `CLK_EN` writer - Tsens regbank clock gating enable."]
pub type CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Tsens regbank clock gating enable."]
    #[inline(always)]
    pub fn clk_en(&self) -> CLK_EN_R {
        CLK_EN_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLK_CONF")
            .field("clk_en", &self.clk_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Tsens regbank clock gating enable."]
    #[inline(always)]
    #[must_use]
    pub fn clk_en(&mut self) -> CLK_EN_W<CLK_CONF_SPEC> {
        CLK_EN_W::new(self, 0)
    }
}
#[doc = "Tsens regbank configuration registers.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLK_CONF_SPEC;
impl crate::RegisterSpec for CLK_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_conf::R`](R) reader structure"]
impl crate::Readable for CLK_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`clk_conf::W`](W) writer structure"]
impl crate::Writable for CLK_CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLK_CONF to value 0"]
impl crate::Resettable for CLK_CONF_SPEC {
    const RESET_VALUE: u32 = 0;
}
