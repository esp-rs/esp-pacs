#[doc = "Register `BB_CLK` reader"]
pub type R = crate::R<BB_CLK_SPEC>;
#[doc = "Register `BB_CLK` writer"]
pub type W = crate::W<BB_CLK_SPEC>;
#[doc = "Field `BB_CLK_FREQ_MINUS_1` reader - "]
pub type BB_CLK_FREQ_MINUS_1_R = crate::FieldReader;
#[doc = "Field `BB_CLK_FREQ_MINUS_1` writer - "]
pub type BB_CLK_FREQ_MINUS_1_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn bb_clk_freq_minus_1(&self) -> BB_CLK_FREQ_MINUS_1_R {
        BB_CLK_FREQ_MINUS_1_R::new((self.bits & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BB_CLK")
            .field("bb_clk_freq_minus_1", &self.bb_clk_freq_minus_1())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn bb_clk_freq_minus_1(&mut self) -> BB_CLK_FREQ_MINUS_1_W<'_, BB_CLK_SPEC> {
        BB_CLK_FREQ_MINUS_1_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`bb_clk::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bb_clk::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BB_CLK_SPEC;
impl crate::RegisterSpec for BB_CLK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bb_clk::R`](R) reader structure"]
impl crate::Readable for BB_CLK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bb_clk::W`](W) writer structure"]
impl crate::Writable for BB_CLK_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BB_CLK to value 0"]
impl crate::Resettable for BB_CLK_SPEC {}
