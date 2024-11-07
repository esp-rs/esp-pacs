#[doc = "Register `ECO_HIGH_APB` reader"]
pub type R = crate::R<ECO_HIGH_APB_SPEC>;
#[doc = "Register `ECO_HIGH_APB` writer"]
pub type W = crate::W<ECO_HIGH_APB_SPEC>;
#[doc = "Field `RND_ECO_HIGH_APB` reader - Reserved."]
pub type RND_ECO_HIGH_APB_R = crate::FieldReader<u32>;
#[doc = "Field `RND_ECO_HIGH_APB` writer - Reserved."]
pub type RND_ECO_HIGH_APB_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Reserved."]
    #[inline(always)]
    pub fn rnd_eco_high_apb(&self) -> RND_ECO_HIGH_APB_R {
        RND_ECO_HIGH_APB_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ECO_HIGH_APB")
            .field("rnd_eco_high_apb", &self.rnd_eco_high_apb())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Reserved."]
    #[inline(always)]
    pub fn rnd_eco_high_apb(&mut self) -> RND_ECO_HIGH_APB_W<ECO_HIGH_APB_SPEC> {
        RND_ECO_HIGH_APB_W::new(self, 0)
    }
}
#[doc = "Reserved.\n\nYou can [`read`](crate::Reg::read) this register and get [`eco_high_apb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eco_high_apb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ECO_HIGH_APB_SPEC;
impl crate::RegisterSpec for ECO_HIGH_APB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eco_high_apb::R`](R) reader structure"]
impl crate::Readable for ECO_HIGH_APB_SPEC {}
#[doc = "`write(|w| ..)` method takes [`eco_high_apb::W`](W) writer structure"]
impl crate::Writable for ECO_HIGH_APB_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ECO_HIGH_APB to value 0xffff_ffff"]
impl crate::Resettable for ECO_HIGH_APB_SPEC {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
