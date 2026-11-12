#[doc = "Register `RNG_LOOP` reader"]
pub type R = crate::R<RNG_LOOP_SPEC>;
#[doc = "Register `RNG_LOOP` writer"]
pub type W = crate::W<RNG_LOOP_SPEC>;
#[doc = "Field `RNG_SAMPLE_ENABLE` reader - enable rng digital ring count"]
pub type RNG_SAMPLE_ENABLE_R = crate::BitReader;
#[doc = "Field `RNG_SAMPLE_ENABLE` writer - enable rng digital ring count"]
pub type RNG_SAMPLE_ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAMPLE_CNT` reader - This field get rng ring count"]
pub type SAMPLE_CNT_R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - enable rng digital ring count"]
    #[inline(always)]
    pub fn rng_sample_enable(&self) -> RNG_SAMPLE_ENABLE_R {
        RNG_SAMPLE_ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 24:31 - This field get rng ring count"]
    #[inline(always)]
    pub fn sample_cnt(&self) -> SAMPLE_CNT_R {
        SAMPLE_CNT_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RNG_LOOP")
            .field("rng_sample_enable", &self.rng_sample_enable())
            .field("sample_cnt", &self.sample_cnt())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - enable rng digital ring count"]
    #[inline(always)]
    pub fn rng_sample_enable(&mut self) -> RNG_SAMPLE_ENABLE_W<'_, RNG_LOOP_SPEC> {
        RNG_SAMPLE_ENABLE_W::new(self, 0)
    }
}
#[doc = "configure rng_ring\n\nYou can [`read`](crate::Reg::read) this register and get [`rng_loop::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rng_loop::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RNG_LOOP_SPEC;
impl crate::RegisterSpec for RNG_LOOP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rng_loop::R`](R) reader structure"]
impl crate::Readable for RNG_LOOP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rng_loop::W`](W) writer structure"]
impl crate::Writable for RNG_LOOP_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RNG_LOOP to value 0"]
impl crate::Resettable for RNG_LOOP_SPEC {}
