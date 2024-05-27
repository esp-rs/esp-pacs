///Register `A_DECI_SCORE` reader
pub type R = crate::R<A_DECI_SCORE_SPEC>;
///Register `A_DECI_SCORE` writer
pub type W = crate::W<A_DECI_SCORE_SPEC>;
///Field `A_C_DECI_SCORE` reader - Configures video A chroma MB decimate score. When chroma score is smaller than it, chroma decimate will be enable.
pub type A_C_DECI_SCORE_R = crate::FieldReader<u16>;
///Field `A_C_DECI_SCORE` writer - Configures video A chroma MB decimate score. When chroma score is smaller than it, chroma decimate will be enable.
pub type A_C_DECI_SCORE_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
///Field `A_L_DECI_SCORE` reader - Configures video A luma MB decimate score. When luma score is smaller than it, luma decimate will be enable.
pub type A_L_DECI_SCORE_R = crate::FieldReader<u16>;
///Field `A_L_DECI_SCORE` writer - Configures video A luma MB decimate score. When luma score is smaller than it, luma decimate will be enable.
pub type A_L_DECI_SCORE_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    ///Bits 0:9 - Configures video A chroma MB decimate score. When chroma score is smaller than it, chroma decimate will be enable.
    #[inline(always)]
    pub fn a_c_deci_score(&self) -> A_C_DECI_SCORE_R {
        A_C_DECI_SCORE_R::new((self.bits & 0x03ff) as u16)
    }
    ///Bits 10:19 - Configures video A luma MB decimate score. When luma score is smaller than it, luma decimate will be enable.
    #[inline(always)]
    pub fn a_l_deci_score(&self) -> A_L_DECI_SCORE_R {
        A_L_DECI_SCORE_R::new(((self.bits >> 10) & 0x03ff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("A_DECI_SCORE")
            .field("a_c_deci_score", &self.a_c_deci_score())
            .field("a_l_deci_score", &self.a_l_deci_score())
            .finish()
    }
}
impl W {
    ///Bits 0:9 - Configures video A chroma MB decimate score. When chroma score is smaller than it, chroma decimate will be enable.
    #[inline(always)]
    #[must_use]
    pub fn a_c_deci_score(&mut self) -> A_C_DECI_SCORE_W<A_DECI_SCORE_SPEC> {
        A_C_DECI_SCORE_W::new(self, 0)
    }
    ///Bits 10:19 - Configures video A luma MB decimate score. When luma score is smaller than it, luma decimate will be enable.
    #[inline(always)]
    #[must_use]
    pub fn a_l_deci_score(&mut self) -> A_L_DECI_SCORE_W<A_DECI_SCORE_SPEC> {
        A_L_DECI_SCORE_W::new(self, 10)
    }
}
/**Video A luma and chroma MB decimate score Register.

You can [`read`](crate::generic::Reg::read) this register and get [`a_deci_score::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`a_deci_score::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct A_DECI_SCORE_SPEC;
impl crate::RegisterSpec for A_DECI_SCORE_SPEC {
    type Ux = u32;
}
///`read()` method returns [`a_deci_score::R`](R) reader structure
impl crate::Readable for A_DECI_SCORE_SPEC {}
///`write(|w| ..)` method takes [`a_deci_score::W`](W) writer structure
impl crate::Writable for A_DECI_SCORE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets A_DECI_SCORE to value 0
impl crate::Resettable for A_DECI_SCORE_SPEC {
    const RESET_VALUE: u32 = 0;
}
