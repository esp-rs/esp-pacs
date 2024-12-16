#[doc = "Register `A_DECI_SCORE_OFFSET` reader"]
pub type R = crate::R<A_DECI_SCORE_OFFSET_SPEC>;
#[doc = "Register `A_DECI_SCORE_OFFSET` writer"]
pub type W = crate::W<A_DECI_SCORE_OFFSET_SPEC>;
#[doc = "Field `A_I16X16_DECI_SCORE_OFFSET` reader - Configures video A i16x16 MB decimate score offset. This offset will be added to i16x16 MB score."]
pub type A_I16X16_DECI_SCORE_OFFSET_R = crate::FieldReader;
#[doc = "Field `A_I16X16_DECI_SCORE_OFFSET` writer - Configures video A i16x16 MB decimate score offset. This offset will be added to i16x16 MB score."]
pub type A_I16X16_DECI_SCORE_OFFSET_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `A_I_CHROMA_DECI_SCORE_OFFSET` reader - Configures video A I chroma MB decimate score offset. This offset will be added to I chroma MB score."]
pub type A_I_CHROMA_DECI_SCORE_OFFSET_R = crate::FieldReader;
#[doc = "Field `A_I_CHROMA_DECI_SCORE_OFFSET` writer - Configures video A I chroma MB decimate score offset. This offset will be added to I chroma MB score."]
pub type A_I_CHROMA_DECI_SCORE_OFFSET_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `A_P16X16_DECI_SCORE_OFFSET` reader - Configures video A p16x16 MB decimate score offset. This offset will be added to p16x16 MB score."]
pub type A_P16X16_DECI_SCORE_OFFSET_R = crate::FieldReader;
#[doc = "Field `A_P16X16_DECI_SCORE_OFFSET` writer - Configures video A p16x16 MB decimate score offset. This offset will be added to p16x16 MB score."]
pub type A_P16X16_DECI_SCORE_OFFSET_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `A_P_CHROMA_DECI_SCORE_OFFSET` reader - Configures video A p chroma MB decimate score offset. This offset will be added to p chroma MB score."]
pub type A_P_CHROMA_DECI_SCORE_OFFSET_R = crate::FieldReader;
#[doc = "Field `A_P_CHROMA_DECI_SCORE_OFFSET` writer - Configures video A p chroma MB decimate score offset. This offset will be added to p chroma MB score."]
pub type A_P_CHROMA_DECI_SCORE_OFFSET_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - Configures video A i16x16 MB decimate score offset. This offset will be added to i16x16 MB score."]
    #[inline(always)]
    pub fn a_i16x16_deci_score_offset(&self) -> A_I16X16_DECI_SCORE_OFFSET_R {
        A_I16X16_DECI_SCORE_OFFSET_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:11 - Configures video A I chroma MB decimate score offset. This offset will be added to I chroma MB score."]
    #[inline(always)]
    pub fn a_i_chroma_deci_score_offset(&self) -> A_I_CHROMA_DECI_SCORE_OFFSET_R {
        A_I_CHROMA_DECI_SCORE_OFFSET_R::new(((self.bits >> 6) & 0x3f) as u8)
    }
    #[doc = "Bits 12:17 - Configures video A p16x16 MB decimate score offset. This offset will be added to p16x16 MB score."]
    #[inline(always)]
    pub fn a_p16x16_deci_score_offset(&self) -> A_P16X16_DECI_SCORE_OFFSET_R {
        A_P16X16_DECI_SCORE_OFFSET_R::new(((self.bits >> 12) & 0x3f) as u8)
    }
    #[doc = "Bits 18:23 - Configures video A p chroma MB decimate score offset. This offset will be added to p chroma MB score."]
    #[inline(always)]
    pub fn a_p_chroma_deci_score_offset(&self) -> A_P_CHROMA_DECI_SCORE_OFFSET_R {
        A_P_CHROMA_DECI_SCORE_OFFSET_R::new(((self.bits >> 18) & 0x3f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("A_DECI_SCORE_OFFSET")
            .field(
                "a_i16x16_deci_score_offset",
                &self.a_i16x16_deci_score_offset(),
            )
            .field(
                "a_i_chroma_deci_score_offset",
                &self.a_i_chroma_deci_score_offset(),
            )
            .field(
                "a_p16x16_deci_score_offset",
                &self.a_p16x16_deci_score_offset(),
            )
            .field(
                "a_p_chroma_deci_score_offset",
                &self.a_p_chroma_deci_score_offset(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:5 - Configures video A i16x16 MB decimate score offset. This offset will be added to i16x16 MB score."]
    #[inline(always)]
    pub fn a_i16x16_deci_score_offset(
        &mut self,
    ) -> A_I16X16_DECI_SCORE_OFFSET_W<A_DECI_SCORE_OFFSET_SPEC> {
        A_I16X16_DECI_SCORE_OFFSET_W::new(self, 0)
    }
    #[doc = "Bits 6:11 - Configures video A I chroma MB decimate score offset. This offset will be added to I chroma MB score."]
    #[inline(always)]
    pub fn a_i_chroma_deci_score_offset(
        &mut self,
    ) -> A_I_CHROMA_DECI_SCORE_OFFSET_W<A_DECI_SCORE_OFFSET_SPEC> {
        A_I_CHROMA_DECI_SCORE_OFFSET_W::new(self, 6)
    }
    #[doc = "Bits 12:17 - Configures video A p16x16 MB decimate score offset. This offset will be added to p16x16 MB score."]
    #[inline(always)]
    pub fn a_p16x16_deci_score_offset(
        &mut self,
    ) -> A_P16X16_DECI_SCORE_OFFSET_W<A_DECI_SCORE_OFFSET_SPEC> {
        A_P16X16_DECI_SCORE_OFFSET_W::new(self, 12)
    }
    #[doc = "Bits 18:23 - Configures video A p chroma MB decimate score offset. This offset will be added to p chroma MB score."]
    #[inline(always)]
    pub fn a_p_chroma_deci_score_offset(
        &mut self,
    ) -> A_P_CHROMA_DECI_SCORE_OFFSET_W<A_DECI_SCORE_OFFSET_SPEC> {
        A_P_CHROMA_DECI_SCORE_OFFSET_W::new(self, 18)
    }
}
#[doc = "Video A luma and chroma MB decimate score offset Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`a_deci_score_offset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`a_deci_score_offset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct A_DECI_SCORE_OFFSET_SPEC;
impl crate::RegisterSpec for A_DECI_SCORE_OFFSET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`a_deci_score_offset::R`](R) reader structure"]
impl crate::Readable for A_DECI_SCORE_OFFSET_SPEC {}
#[doc = "`write(|w| ..)` method takes [`a_deci_score_offset::W`](W) writer structure"]
impl crate::Writable for A_DECI_SCORE_OFFSET_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets A_DECI_SCORE_OFFSET to value 0"]
impl crate::Resettable for A_DECI_SCORE_OFFSET_SPEC {
    const RESET_VALUE: u32 = 0;
}
