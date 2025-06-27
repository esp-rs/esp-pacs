#[doc = "Register `CONF` reader"]
pub type R = crate::R<CONF_SPEC>;
#[doc = "Register `CONF` writer"]
pub type W = crate::W<CONF_SPEC>;
#[doc = "Field `CLK_FO` reader - System timer clock force enable."]
pub type CLK_FO_R = crate::BitReader;
#[doc = "Field `CLK_FO` writer - System timer clock force enable."]
pub type CLK_FO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_EN` reader - Register clock enable."]
pub type CLK_EN_R = crate::BitReader;
#[doc = "Field `CLK_EN` writer - Register clock enable."]
pub type CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - System timer clock force enable."]
    #[inline(always)]
    pub fn clk_fo(&self) -> CLK_FO_R {
        CLK_FO_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 31 - Register clock enable."]
    #[inline(always)]
    pub fn clk_en(&self) -> CLK_EN_R {
        CLK_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONF")
            .field("clk_fo", &self.clk_fo())
            .field("clk_en", &self.clk_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - System timer clock force enable."]
    #[inline(always)]
    pub fn clk_fo(&mut self) -> CLK_FO_W<CONF_SPEC> {
        CLK_FO_W::new(self, 0)
    }
    #[doc = "Bit 31 - Register clock enable."]
    #[inline(always)]
    pub fn clk_en(&mut self) -> CLK_EN_W<CONF_SPEC> {
        CLK_EN_W::new(self, 31)
    }
}
#[doc = "Configure system timer clock\n\nYou can [`read`](crate::Reg::read) this register and get [`conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CONF_SPEC;
impl crate::RegisterSpec for CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`conf::R`](R) reader structure"]
impl crate::Readable for CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`conf::W`](W) writer structure"]
impl crate::Writable for CONF_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CONF to value 0"]
impl crate::Resettable for CONF_SPEC {}
