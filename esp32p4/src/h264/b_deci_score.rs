///Register `B_DECI_SCORE` reader
pub type R = crate::R<B_DECI_SCORE_SPEC>;
///Register `B_DECI_SCORE` writer
pub type W = crate::W<B_DECI_SCORE_SPEC>;
///Field `B_C_DECI_SCORE` reader - Configures video B chroma MB decimate score. When chroma score is smaller than it, chroma decimate will be enable.
pub type B_C_DECI_SCORE_R = crate::FieldReader<u16>;
///Field `B_C_DECI_SCORE` writer - Configures video B chroma MB decimate score. When chroma score is smaller than it, chroma decimate will be enable.
pub type B_C_DECI_SCORE_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
///Field `B_L_DECI_SCORE` reader - Configures video B luma MB decimate score. When luma score is smaller than it, luma decimate will be enable.
pub type B_L_DECI_SCORE_R = crate::FieldReader<u16>;
///Field `B_L_DECI_SCORE` writer - Configures video B luma MB decimate score. When luma score is smaller than it, luma decimate will be enable.
pub type B_L_DECI_SCORE_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    ///Bits 0:9 - Configures video B chroma MB decimate score. When chroma score is smaller than it, chroma decimate will be enable.
    #[inline(always)]
    pub fn b_c_deci_score(&self) -> B_C_DECI_SCORE_R {
        B_C_DECI_SCORE_R::new((self.bits & 0x03ff) as u16)
    }
    ///Bits 10:19 - Configures video B luma MB decimate score. When luma score is smaller than it, luma decimate will be enable.
    #[inline(always)]
    pub fn b_l_deci_score(&self) -> B_L_DECI_SCORE_R {
        B_L_DECI_SCORE_R::new(((self.bits >> 10) & 0x03ff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("B_DECI_SCORE")
            .field("b_c_deci_score", &self.b_c_deci_score())
            .field("b_l_deci_score", &self.b_l_deci_score())
            .finish()
    }
}
impl W {
    ///Bits 0:9 - Configures video B chroma MB decimate score. When chroma score is smaller than it, chroma decimate will be enable.
    #[inline(always)]
    #[must_use]
    pub fn b_c_deci_score(&mut self) -> B_C_DECI_SCORE_W<B_DECI_SCORE_SPEC> {
        B_C_DECI_SCORE_W::new(self, 0)
    }
    ///Bits 10:19 - Configures video B luma MB decimate score. When luma score is smaller than it, luma decimate will be enable.
    #[inline(always)]
    #[must_use]
    pub fn b_l_deci_score(&mut self) -> B_L_DECI_SCORE_W<B_DECI_SCORE_SPEC> {
        B_L_DECI_SCORE_W::new(self, 10)
    }
}
/**Video B luma and chroma MB decimate score Register.

You can [`read`](crate::generic::Reg::read) this register and get [`b_deci_score::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`b_deci_score::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct B_DECI_SCORE_SPEC;
impl crate::RegisterSpec for B_DECI_SCORE_SPEC {
    type Ux = u32;
}
///`read()` method returns [`b_deci_score::R`](R) reader structure
impl crate::Readable for B_DECI_SCORE_SPEC {}
///`write(|w| ..)` method takes [`b_deci_score::W`](W) writer structure
impl crate::Writable for B_DECI_SCORE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets B_DECI_SCORE to value 0
impl crate::Resettable for B_DECI_SCORE_SPEC {
    const RESET_VALUE: u32 = 0;
}
