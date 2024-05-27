///Register `ECO_LOW_APB` reader
pub type R = crate::R<ECO_LOW_APB_SPEC>;
///Register `ECO_LOW_APB` writer
pub type W = crate::W<ECO_LOW_APB_SPEC>;
///Field `RND_ECO_LOW_APB` reader - Reserved.
pub type RND_ECO_LOW_APB_R = crate::FieldReader<u32>;
///Field `RND_ECO_LOW_APB` writer - Reserved.
pub type RND_ECO_LOW_APB_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Reserved.
    #[inline(always)]
    pub fn rnd_eco_low_apb(&self) -> RND_ECO_LOW_APB_R {
        RND_ECO_LOW_APB_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ECO_LOW_APB")
            .field("rnd_eco_low_apb", &self.rnd_eco_low_apb())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Reserved.
    #[inline(always)]
    #[must_use]
    pub fn rnd_eco_low_apb(&mut self) -> RND_ECO_LOW_APB_W<ECO_LOW_APB_SPEC> {
        RND_ECO_LOW_APB_W::new(self, 0)
    }
}
/**Reserved.

You can [`read`](crate::generic::Reg::read) this register and get [`eco_low_apb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eco_low_apb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ECO_LOW_APB_SPEC;
impl crate::RegisterSpec for ECO_LOW_APB_SPEC {
    type Ux = u32;
}
///`read()` method returns [`eco_low_apb::R`](R) reader structure
impl crate::Readable for ECO_LOW_APB_SPEC {}
///`write(|w| ..)` method takes [`eco_low_apb::W`](W) writer structure
impl crate::Writable for ECO_LOW_APB_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets ECO_LOW_APB to value 0
impl crate::Resettable for ECO_LOW_APB_SPEC {
    const RESET_VALUE: u32 = 0;
}
