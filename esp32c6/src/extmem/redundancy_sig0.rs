///Register `REDUNDANCY_SIG0` reader
pub type R = crate::R<REDUNDANCY_SIG0_SPEC>;
///Register `REDUNDANCY_SIG0` writer
pub type W = crate::W<REDUNDANCY_SIG0_SPEC>;
///Field `CACHE_REDCY_SIG0` reader - Those bits are prepared for ECO.
pub type CACHE_REDCY_SIG0_R = crate::FieldReader<u32>;
///Field `CACHE_REDCY_SIG0` writer - Those bits are prepared for ECO.
pub type CACHE_REDCY_SIG0_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Those bits are prepared for ECO.
    #[inline(always)]
    pub fn cache_redcy_sig0(&self) -> CACHE_REDCY_SIG0_R {
        CACHE_REDCY_SIG0_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REDUNDANCY_SIG0")
            .field("cache_redcy_sig0", &self.cache_redcy_sig0())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Those bits are prepared for ECO.
    #[inline(always)]
    #[must_use]
    pub fn cache_redcy_sig0(&mut self) -> CACHE_REDCY_SIG0_W<REDUNDANCY_SIG0_SPEC> {
        CACHE_REDCY_SIG0_W::new(self, 0)
    }
}
/**Cache redundancy signal 0 register

You can [`read`](crate::generic::Reg::read) this register and get [`redundancy_sig0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`redundancy_sig0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct REDUNDANCY_SIG0_SPEC;
impl crate::RegisterSpec for REDUNDANCY_SIG0_SPEC {
    type Ux = u32;
}
///`read()` method returns [`redundancy_sig0::R`](R) reader structure
impl crate::Readable for REDUNDANCY_SIG0_SPEC {}
///`write(|w| ..)` method takes [`redundancy_sig0::W`](W) writer structure
impl crate::Writable for REDUNDANCY_SIG0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets REDUNDANCY_SIG0 to value 0
impl crate::Resettable for REDUNDANCY_SIG0_SPEC {
    const RESET_VALUE: u32 = 0;
}
