#[doc = "Register `RNG_DATE` reader"]
pub type R = crate::R<RNG_DATE_SPEC>;
#[doc = "Register `RNG_DATE` writer"]
pub type W = crate::W<RNG_DATE_SPEC>;
#[doc = "Field `RNG_DATE` reader - RNG_DATE"]
pub type RNG_DATE_R = crate::FieldReader<u32>;
#[doc = "Field `RNG_DATE` writer - RNG_DATE"]
pub type RNG_DATE_W<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
#[doc = "Field `RNG_CLK_EN` reader - RNG_CLK_EN"]
pub type RNG_CLK_EN_R = crate::BitReader;
#[doc = "Field `RNG_CLK_EN` writer - RNG_CLK_EN"]
pub type RNG_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:30 - RNG_DATE"]
    #[inline(always)]
    pub fn rng_date(&self) -> RNG_DATE_R {
        RNG_DATE_R::new(self.bits & 0x7fff_ffff)
    }
    #[doc = "Bit 31 - RNG_CLK_EN"]
    #[inline(always)]
    pub fn rng_clk_en(&self) -> RNG_CLK_EN_R {
        RNG_CLK_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RNG_DATE")
            .field("rng_date", &self.rng_date())
            .field("rng_clk_en", &self.rng_clk_en())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:30 - RNG_DATE"]
    #[inline(always)]
    pub fn rng_date(&mut self) -> RNG_DATE_W<'_, RNG_DATE_SPEC> {
        RNG_DATE_W::new(self, 0)
    }
    #[doc = "Bit 31 - RNG_CLK_EN"]
    #[inline(always)]
    pub fn rng_clk_en(&mut self) -> RNG_CLK_EN_W<'_, RNG_DATE_SPEC> {
        RNG_CLK_EN_W::new(self, 31)
    }
}
#[doc = "RNG_DATE\n\nYou can [`read`](crate::Reg::read) this register and get [`rng_date::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rng_date::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RNG_DATE_SPEC;
impl crate::RegisterSpec for RNG_DATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rng_date::R`](R) reader structure"]
impl crate::Readable for RNG_DATE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rng_date::W`](W) writer structure"]
impl crate::Writable for RNG_DATE_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RNG_DATE to value 0"]
impl crate::Resettable for RNG_DATE_SPEC {}
