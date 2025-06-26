#[doc = "Register `TIMGCLK` reader"]
pub type R = crate::R<TIMGCLK_SPEC>;
#[doc = "Register `TIMGCLK` writer"]
pub type W = crate::W<TIMGCLK_SPEC>;
#[doc = "Field `CLK_EN` reader - Force clock enable for this regfile"]
pub type CLK_EN_R = crate::BitReader;
#[doc = "Field `CLK_EN` writer - Force clock enable for this regfile"]
pub type CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 31 - Force clock enable for this regfile"]
    #[inline(always)]
    pub fn clk_en(&self) -> CLK_EN_R {
        CLK_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIMGCLK")
            .field("clk_en", &self.clk_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 31 - Force clock enable for this regfile"]
    #[inline(always)]
    pub fn clk_en(&mut self) -> CLK_EN_W<TIMGCLK_SPEC> {
        CLK_EN_W::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`timgclk::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timgclk::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIMGCLK_SPEC;
impl crate::RegisterSpec for TIMGCLK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timgclk::R`](R) reader structure"]
impl crate::Readable for TIMGCLK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`timgclk::W`](W) writer structure"]
impl crate::Writable for TIMGCLK_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TIMGCLK to value 0"]
impl crate::Resettable for TIMGCLK_SPEC {}
