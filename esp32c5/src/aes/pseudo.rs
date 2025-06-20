#[doc = "Register `PSEUDO` reader"]
pub type R = crate::R<PSEUDO_SPEC>;
#[doc = "Register `PSEUDO` writer"]
pub type W = crate::W<PSEUDO_SPEC>;
#[doc = "Field `EN` reader - This bit decides whether the pseudo round function is enable or not."]
pub type EN_R = crate::BitReader;
#[doc = "Field `EN` writer - This bit decides whether the pseudo round function is enable or not."]
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BASE` reader - Those bits decides the basic number of pseudo round number."]
pub type BASE_R = crate::FieldReader;
#[doc = "Field `BASE` writer - Those bits decides the basic number of pseudo round number."]
pub type BASE_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `INC` reader - Those bits decides the increment number of pseudo round number"]
pub type INC_R = crate::FieldReader;
#[doc = "Field `INC` writer - Those bits decides the increment number of pseudo round number"]
pub type INC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RNG_CNT` reader - Those bits decides the update frequency of the pseudo-key."]
pub type RNG_CNT_R = crate::FieldReader;
#[doc = "Field `RNG_CNT` writer - Those bits decides the update frequency of the pseudo-key."]
pub type RNG_CNT_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0 - This bit decides whether the pseudo round function is enable or not."]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:4 - Those bits decides the basic number of pseudo round number."]
    #[inline(always)]
    pub fn base(&self) -> BASE_R {
        BASE_R::new(((self.bits >> 1) & 0x0f) as u8)
    }
    #[doc = "Bits 5:6 - Those bits decides the increment number of pseudo round number"]
    #[inline(always)]
    pub fn inc(&self) -> INC_R {
        INC_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bits 7:9 - Those bits decides the update frequency of the pseudo-key."]
    #[inline(always)]
    pub fn rng_cnt(&self) -> RNG_CNT_R {
        RNG_CNT_R::new(((self.bits >> 7) & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PSEUDO")
            .field("en", &self.en())
            .field("base", &self.base())
            .field("inc", &self.inc())
            .field("rng_cnt", &self.rng_cnt())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - This bit decides whether the pseudo round function is enable or not."]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W<PSEUDO_SPEC> {
        EN_W::new(self, 0)
    }
    #[doc = "Bits 1:4 - Those bits decides the basic number of pseudo round number."]
    #[inline(always)]
    pub fn base(&mut self) -> BASE_W<PSEUDO_SPEC> {
        BASE_W::new(self, 1)
    }
    #[doc = "Bits 5:6 - Those bits decides the increment number of pseudo round number"]
    #[inline(always)]
    pub fn inc(&mut self) -> INC_W<PSEUDO_SPEC> {
        INC_W::new(self, 5)
    }
    #[doc = "Bits 7:9 - Those bits decides the update frequency of the pseudo-key."]
    #[inline(always)]
    pub fn rng_cnt(&mut self) -> RNG_CNT_W<PSEUDO_SPEC> {
        RNG_CNT_W::new(self, 7)
    }
}
#[doc = "AES PSEUDO function configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`pseudo::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pseudo::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PSEUDO_SPEC;
impl crate::RegisterSpec for PSEUDO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pseudo::R`](R) reader structure"]
impl crate::Readable for PSEUDO_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pseudo::W`](W) writer structure"]
impl crate::Writable for PSEUDO_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PSEUDO to value 0x03c4"]
impl crate::Resettable for PSEUDO_SPEC {
    const RESET_VALUE: u32 = 0x03c4;
}
