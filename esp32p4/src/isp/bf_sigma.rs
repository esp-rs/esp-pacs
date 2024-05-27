#[doc = "Register `BF_SIGMA` reader"]
pub type R = crate::R<BF_SIGMA_SPEC>;
#[doc = "Register `BF_SIGMA` writer"]
pub type W = crate::W<BF_SIGMA_SPEC>;
#[doc = "Field `SIGMA` reader - this field configures the bayer denoising level, valid data from 2 to 20"]
pub type SIGMA_R = crate::FieldReader;
#[doc = "Field `SIGMA` writer - this field configures the bayer denoising level, valid data from 2 to 20"]
pub type SIGMA_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - this field configures the bayer denoising level, valid data from 2 to 20"]
    #[inline(always)]
    pub fn sigma(&self) -> SIGMA_R {
        SIGMA_R::new((self.bits & 0x3f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BF_SIGMA")
            .field("sigma", &self.sigma())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:5 - this field configures the bayer denoising level, valid data from 2 to 20"]
    #[inline(always)]
    #[must_use]
    pub fn sigma(&mut self) -> SIGMA_W<BF_SIGMA_SPEC> {
        SIGMA_W::new(self, 0)
    }
}
#[doc = "bf denoising level control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bf_sigma::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bf_sigma::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BF_SIGMA_SPEC;
impl crate::RegisterSpec for BF_SIGMA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bf_sigma::R`](R) reader structure"]
impl crate::Readable for BF_SIGMA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bf_sigma::W`](W) writer structure"]
impl crate::Writable for BF_SIGMA_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BF_SIGMA to value 0x02"]
impl crate::Resettable for BF_SIGMA_SPEC {
    const RESET_VALUE: u32 = 0x02;
}
