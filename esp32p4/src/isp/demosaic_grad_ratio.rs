#[doc = "Register `DEMOSAIC_GRAD_RATIO` reader"]
pub type R = crate::R<DEMOSAIC_GRAD_RATIO_SPEC>;
#[doc = "Register `DEMOSAIC_GRAD_RATIO` writer"]
pub type W = crate::W<DEMOSAIC_GRAD_RATIO_SPEC>;
#[doc = "Field `DEMOSAIC_GRAD_RATIO` reader - this field configures demosaic gradient select ratio"]
pub type DEMOSAIC_GRAD_RATIO_R = crate::FieldReader;
#[doc = "Field `DEMOSAIC_GRAD_RATIO` writer - this field configures demosaic gradient select ratio"]
pub type DEMOSAIC_GRAD_RATIO_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - this field configures demosaic gradient select ratio"]
    #[inline(always)]
    pub fn demosaic_grad_ratio(&self) -> DEMOSAIC_GRAD_RATIO_R {
        DEMOSAIC_GRAD_RATIO_R::new((self.bits & 0x3f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DEMOSAIC_GRAD_RATIO")
            .field(
                "demosaic_grad_ratio",
                &format_args!("{}", self.demosaic_grad_ratio().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DEMOSAIC_GRAD_RATIO_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:5 - this field configures demosaic gradient select ratio"]
    #[inline(always)]
    #[must_use]
    pub fn demosaic_grad_ratio(&mut self) -> DEMOSAIC_GRAD_RATIO_W<DEMOSAIC_GRAD_RATIO_SPEC> {
        DEMOSAIC_GRAD_RATIO_W::new(self, 0)
    }
}
#[doc = "demosaic gradient select ratio\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`demosaic_grad_ratio::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`demosaic_grad_ratio::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DEMOSAIC_GRAD_RATIO_SPEC;
impl crate::RegisterSpec for DEMOSAIC_GRAD_RATIO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`demosaic_grad_ratio::R`](R) reader structure"]
impl crate::Readable for DEMOSAIC_GRAD_RATIO_SPEC {}
#[doc = "`write(|w| ..)` method takes [`demosaic_grad_ratio::W`](W) writer structure"]
impl crate::Writable for DEMOSAIC_GRAD_RATIO_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DEMOSAIC_GRAD_RATIO to value 0x10"]
impl crate::Resettable for DEMOSAIC_GRAD_RATIO_SPEC {
    const RESET_VALUE: u32 = 0x10;
}
