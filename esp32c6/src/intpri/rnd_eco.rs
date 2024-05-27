///Register `RND_ECO` reader
pub type R = crate::R<RND_ECO_SPEC>;
///Register `RND_ECO` writer
pub type W = crate::W<RND_ECO_SPEC>;
///Field `REDCY_ENA` reader - Only reserved for ECO.
pub type REDCY_ENA_R = crate::BitReader;
///Field `REDCY_ENA` writer - Only reserved for ECO.
pub type REDCY_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `REDCY_RESULT` reader - Only reserved for ECO.
pub type REDCY_RESULT_R = crate::BitReader;
impl R {
    ///Bit 0 - Only reserved for ECO.
    #[inline(always)]
    pub fn redcy_ena(&self) -> REDCY_ENA_R {
        REDCY_ENA_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Only reserved for ECO.
    #[inline(always)]
    pub fn redcy_result(&self) -> REDCY_RESULT_R {
        REDCY_RESULT_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RND_ECO")
            .field("redcy_ena", &self.redcy_ena())
            .field("redcy_result", &self.redcy_result())
            .finish()
    }
}
impl W {
    ///Bit 0 - Only reserved for ECO.
    #[inline(always)]
    #[must_use]
    pub fn redcy_ena(&mut self) -> REDCY_ENA_W<RND_ECO_SPEC> {
        REDCY_ENA_W::new(self, 0)
    }
}
/**redcy eco register.

You can [`read`](crate::generic::Reg::read) this register and get [`rnd_eco::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rnd_eco::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct RND_ECO_SPEC;
impl crate::RegisterSpec for RND_ECO_SPEC {
    type Ux = u32;
}
///`read()` method returns [`rnd_eco::R`](R) reader structure
impl crate::Readable for RND_ECO_SPEC {}
///`write(|w| ..)` method takes [`rnd_eco::W`](W) writer structure
impl crate::Writable for RND_ECO_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets RND_ECO to value 0
impl crate::Resettable for RND_ECO_SPEC {
    const RESET_VALUE: u32 = 0;
}
