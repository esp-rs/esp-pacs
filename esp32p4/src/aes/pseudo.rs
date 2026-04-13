#[doc = "Register `PSEUDO` reader"]
pub type R = crate::R<PSEUDO_SPEC>;
#[doc = "Register `PSEUDO` writer"]
pub type W = crate::W<PSEUDO_SPEC>;
#[doc = "Field `PSEUDO_EN` reader - "]
pub type PSEUDO_EN_R = crate::BitReader;
#[doc = "Field `PSEUDO_EN` writer - "]
pub type PSEUDO_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PSEUDO_BASE` reader - "]
pub type PSEUDO_BASE_R = crate::FieldReader;
#[doc = "Field `PSEUDO_BASE` writer - "]
pub type PSEUDO_BASE_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PSEUDO_INC` reader - "]
pub type PSEUDO_INC_R = crate::FieldReader;
#[doc = "Field `PSEUDO_INC` writer - "]
pub type PSEUDO_INC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PSEUDO_RNG_CNT` reader - "]
pub type PSEUDO_RNG_CNT_R = crate::FieldReader;
#[doc = "Field `PSEUDO_RNG_CNT` writer - "]
pub type PSEUDO_RNG_CNT_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pseudo_en(&self) -> PSEUDO_EN_R {
        PSEUDO_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:4"]
    #[inline(always)]
    pub fn pseudo_base(&self) -> PSEUDO_BASE_R {
        PSEUDO_BASE_R::new(((self.bits >> 1) & 0x0f) as u8)
    }
    #[doc = "Bits 5:6"]
    #[inline(always)]
    pub fn pseudo_inc(&self) -> PSEUDO_INC_R {
        PSEUDO_INC_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bits 7:9"]
    #[inline(always)]
    pub fn pseudo_rng_cnt(&self) -> PSEUDO_RNG_CNT_R {
        PSEUDO_RNG_CNT_R::new(((self.bits >> 7) & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PSEUDO")
            .field("pseudo_en", &self.pseudo_en())
            .field("pseudo_base", &self.pseudo_base())
            .field("pseudo_inc", &self.pseudo_inc())
            .field("pseudo_rng_cnt", &self.pseudo_rng_cnt())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pseudo_en(&mut self) -> PSEUDO_EN_W<'_, PSEUDO_SPEC> {
        PSEUDO_EN_W::new(self, 0)
    }
    #[doc = "Bits 1:4"]
    #[inline(always)]
    pub fn pseudo_base(&mut self) -> PSEUDO_BASE_W<'_, PSEUDO_SPEC> {
        PSEUDO_BASE_W::new(self, 1)
    }
    #[doc = "Bits 5:6"]
    #[inline(always)]
    pub fn pseudo_inc(&mut self) -> PSEUDO_INC_W<'_, PSEUDO_SPEC> {
        PSEUDO_INC_W::new(self, 5)
    }
    #[doc = "Bits 7:9"]
    #[inline(always)]
    pub fn pseudo_rng_cnt(&mut self) -> PSEUDO_RNG_CNT_W<'_, PSEUDO_SPEC> {
        PSEUDO_RNG_CNT_W::new(self, 7)
    }
}
#[doc = "AES pseudo-round configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`pseudo::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pseudo::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PSEUDO_SPEC;
impl crate::RegisterSpec for PSEUDO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pseudo::R`](R) reader structure"]
impl crate::Readable for PSEUDO_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pseudo::W`](W) writer structure"]
impl crate::Writable for PSEUDO_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PSEUDO to value 0"]
impl crate::Resettable for PSEUDO_SPEC {}
