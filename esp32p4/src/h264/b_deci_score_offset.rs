#[doc = "Register `B_DECI_SCORE_OFFSET` reader"]
pub type R = crate::R<B_DECI_SCORE_OFFSET_SPEC>;
#[doc = "Register `B_DECI_SCORE_OFFSET` writer"]
pub type W = crate::W<B_DECI_SCORE_OFFSET_SPEC>;
#[doc = "Field `B_I16X16_DECI_SCORE_OFFSET` reader - Configures video B i16x16 MB decimate score offset. This offset will be added to i16x16 MB score."]
pub type B_I16X16_DECI_SCORE_OFFSET_R = crate::FieldReader;
#[doc = "Field `B_I16X16_DECI_SCORE_OFFSET` writer - Configures video B i16x16 MB decimate score offset. This offset will be added to i16x16 MB score."]
pub type B_I16X16_DECI_SCORE_OFFSET_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `B_I_CHROMA_DECI_SCORE_OFFSET` reader - Configures video B I chroma MB decimate score offset. This offset will be added to I chroma MB score."]
pub type B_I_CHROMA_DECI_SCORE_OFFSET_R = crate::FieldReader;
#[doc = "Field `B_I_CHROMA_DECI_SCORE_OFFSET` writer - Configures video B I chroma MB decimate score offset. This offset will be added to I chroma MB score."]
pub type B_I_CHROMA_DECI_SCORE_OFFSET_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `B_P16X16_DECI_SCORE_OFFSET` reader - Configures video B p16x16 MB decimate score offset. This offset will be added to p16x16 MB score."]
pub type B_P16X16_DECI_SCORE_OFFSET_R = crate::FieldReader;
#[doc = "Field `B_P16X16_DECI_SCORE_OFFSET` writer - Configures video B p16x16 MB decimate score offset. This offset will be added to p16x16 MB score."]
pub type B_P16X16_DECI_SCORE_OFFSET_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `B_P_CHROMA_DECI_SCORE_OFFSET` reader - Configures video B p chroma MB decimate score offset. This offset will be added to p chroma MB score."]
pub type B_P_CHROMA_DECI_SCORE_OFFSET_R = crate::FieldReader;
#[doc = "Field `B_P_CHROMA_DECI_SCORE_OFFSET` writer - Configures video B p chroma MB decimate score offset. This offset will be added to p chroma MB score."]
pub type B_P_CHROMA_DECI_SCORE_OFFSET_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - Configures video B i16x16 MB decimate score offset. This offset will be added to i16x16 MB score."]
    #[inline(always)]
    pub fn b_i16x16_deci_score_offset(&self) -> B_I16X16_DECI_SCORE_OFFSET_R {
        B_I16X16_DECI_SCORE_OFFSET_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:11 - Configures video B I chroma MB decimate score offset. This offset will be added to I chroma MB score."]
    #[inline(always)]
    pub fn b_i_chroma_deci_score_offset(&self) -> B_I_CHROMA_DECI_SCORE_OFFSET_R {
        B_I_CHROMA_DECI_SCORE_OFFSET_R::new(((self.bits >> 6) & 0x3f) as u8)
    }
    #[doc = "Bits 12:17 - Configures video B p16x16 MB decimate score offset. This offset will be added to p16x16 MB score."]
    #[inline(always)]
    pub fn b_p16x16_deci_score_offset(&self) -> B_P16X16_DECI_SCORE_OFFSET_R {
        B_P16X16_DECI_SCORE_OFFSET_R::new(((self.bits >> 12) & 0x3f) as u8)
    }
    #[doc = "Bits 18:23 - Configures video B p chroma MB decimate score offset. This offset will be added to p chroma MB score."]
    #[inline(always)]
    pub fn b_p_chroma_deci_score_offset(&self) -> B_P_CHROMA_DECI_SCORE_OFFSET_R {
        B_P_CHROMA_DECI_SCORE_OFFSET_R::new(((self.bits >> 18) & 0x3f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("B_DECI_SCORE_OFFSET")
            .field(
                "b_i16x16_deci_score_offset",
                &format_args!("{}", self.b_i16x16_deci_score_offset().bits()),
            )
            .field(
                "b_i_chroma_deci_score_offset",
                &format_args!("{}", self.b_i_chroma_deci_score_offset().bits()),
            )
            .field(
                "b_p16x16_deci_score_offset",
                &format_args!("{}", self.b_p16x16_deci_score_offset().bits()),
            )
            .field(
                "b_p_chroma_deci_score_offset",
                &format_args!("{}", self.b_p_chroma_deci_score_offset().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<B_DECI_SCORE_OFFSET_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:5 - Configures video B i16x16 MB decimate score offset. This offset will be added to i16x16 MB score."]
    #[inline(always)]
    #[must_use]
    pub fn b_i16x16_deci_score_offset(
        &mut self,
    ) -> B_I16X16_DECI_SCORE_OFFSET_W<B_DECI_SCORE_OFFSET_SPEC> {
        B_I16X16_DECI_SCORE_OFFSET_W::new(self, 0)
    }
    #[doc = "Bits 6:11 - Configures video B I chroma MB decimate score offset. This offset will be added to I chroma MB score."]
    #[inline(always)]
    #[must_use]
    pub fn b_i_chroma_deci_score_offset(
        &mut self,
    ) -> B_I_CHROMA_DECI_SCORE_OFFSET_W<B_DECI_SCORE_OFFSET_SPEC> {
        B_I_CHROMA_DECI_SCORE_OFFSET_W::new(self, 6)
    }
    #[doc = "Bits 12:17 - Configures video B p16x16 MB decimate score offset. This offset will be added to p16x16 MB score."]
    #[inline(always)]
    #[must_use]
    pub fn b_p16x16_deci_score_offset(
        &mut self,
    ) -> B_P16X16_DECI_SCORE_OFFSET_W<B_DECI_SCORE_OFFSET_SPEC> {
        B_P16X16_DECI_SCORE_OFFSET_W::new(self, 12)
    }
    #[doc = "Bits 18:23 - Configures video B p chroma MB decimate score offset. This offset will be added to p chroma MB score."]
    #[inline(always)]
    #[must_use]
    pub fn b_p_chroma_deci_score_offset(
        &mut self,
    ) -> B_P_CHROMA_DECI_SCORE_OFFSET_W<B_DECI_SCORE_OFFSET_SPEC> {
        B_P_CHROMA_DECI_SCORE_OFFSET_W::new(self, 18)
    }
}
#[doc = "Video B luma and chroma MB decimate score offset Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`b_deci_score_offset::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`b_deci_score_offset::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct B_DECI_SCORE_OFFSET_SPEC;
impl crate::RegisterSpec for B_DECI_SCORE_OFFSET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`b_deci_score_offset::R`](R) reader structure"]
impl crate::Readable for B_DECI_SCORE_OFFSET_SPEC {}
#[doc = "`write(|w| ..)` method takes [`b_deci_score_offset::W`](W) writer structure"]
impl crate::Writable for B_DECI_SCORE_OFFSET_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets B_DECI_SCORE_OFFSET to value 0"]
impl crate::Resettable for B_DECI_SCORE_OFFSET_SPEC {
    const RESET_VALUE: u32 = 0;
}
