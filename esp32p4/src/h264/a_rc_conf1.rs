#[doc = "Register `A_RC_CONF1` reader"]
pub type R = crate::R<A_RC_CONF1_SPEC>;
#[doc = "Register `A_RC_CONF1` writer"]
pub type W = crate::W<A_RC_CONF1_SPEC>;
#[doc = "Field `A_CHROMA_DC_QP_DELTA` reader - Configures video A chroma DC QP offset based on Chroma QP. Chroma DC QP = Chroma QP(after map) + reg_chroma_dc_qp_delta."]
pub type A_CHROMA_DC_QP_DELTA_R = crate::FieldReader;
#[doc = "Field `A_CHROMA_DC_QP_DELTA` writer - Configures video A chroma DC QP offset based on Chroma QP. Chroma DC QP = Chroma QP(after map) + reg_chroma_dc_qp_delta."]
pub type A_CHROMA_DC_QP_DELTA_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `A_CHROMA_QP_DELTA` reader - Configures video A chroma QP offset based on luma QP. Chroma QP(before map) = Luma QP + reg_chroma_qp_delta."]
pub type A_CHROMA_QP_DELTA_R = crate::FieldReader;
#[doc = "Field `A_CHROMA_QP_DELTA` writer - Configures video A chroma QP offset based on luma QP. Chroma QP(before map) = Luma QP + reg_chroma_qp_delta."]
pub type A_CHROMA_QP_DELTA_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `A_QP_MIN` reader - Configures video A allowed luma QP min value."]
pub type A_QP_MIN_R = crate::FieldReader;
#[doc = "Field `A_QP_MIN` writer - Configures video A allowed luma QP min value."]
pub type A_QP_MIN_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `A_QP_MAX` reader - Configures video A allowed luma QP max value."]
pub type A_QP_MAX_R = crate::FieldReader;
#[doc = "Field `A_QP_MAX` writer - Configures video A allowed luma QP max value."]
pub type A_QP_MAX_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `A_MAD_FRAME_PRED` reader - Configures vdieo A frame level predicted MB MAD value."]
pub type A_MAD_FRAME_PRED_R = crate::FieldReader<u16>;
#[doc = "Field `A_MAD_FRAME_PRED` writer - Configures vdieo A frame level predicted MB MAD value."]
pub type A_MAD_FRAME_PRED_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:2 - Configures video A chroma DC QP offset based on Chroma QP. Chroma DC QP = Chroma QP(after map) + reg_chroma_dc_qp_delta."]
    #[inline(always)]
    pub fn a_chroma_dc_qp_delta(&self) -> A_CHROMA_DC_QP_DELTA_R {
        A_CHROMA_DC_QP_DELTA_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:6 - Configures video A chroma QP offset based on luma QP. Chroma QP(before map) = Luma QP + reg_chroma_qp_delta."]
    #[inline(always)]
    pub fn a_chroma_qp_delta(&self) -> A_CHROMA_QP_DELTA_R {
        A_CHROMA_QP_DELTA_R::new(((self.bits >> 3) & 0x0f) as u8)
    }
    #[doc = "Bits 7:12 - Configures video A allowed luma QP min value."]
    #[inline(always)]
    pub fn a_qp_min(&self) -> A_QP_MIN_R {
        A_QP_MIN_R::new(((self.bits >> 7) & 0x3f) as u8)
    }
    #[doc = "Bits 13:18 - Configures video A allowed luma QP max value."]
    #[inline(always)]
    pub fn a_qp_max(&self) -> A_QP_MAX_R {
        A_QP_MAX_R::new(((self.bits >> 13) & 0x3f) as u8)
    }
    #[doc = "Bits 19:30 - Configures vdieo A frame level predicted MB MAD value."]
    #[inline(always)]
    pub fn a_mad_frame_pred(&self) -> A_MAD_FRAME_PRED_R {
        A_MAD_FRAME_PRED_R::new(((self.bits >> 19) & 0x0fff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("A_RC_CONF1")
            .field("a_chroma_dc_qp_delta", &self.a_chroma_dc_qp_delta().bits())
            .field("a_chroma_qp_delta", &self.a_chroma_qp_delta().bits())
            .field("a_qp_min", &self.a_qp_min().bits())
            .field("a_qp_max", &self.a_qp_max().bits())
            .field("a_mad_frame_pred", &self.a_mad_frame_pred().bits())
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<A_RC_CONF1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:2 - Configures video A chroma DC QP offset based on Chroma QP. Chroma DC QP = Chroma QP(after map) + reg_chroma_dc_qp_delta."]
    #[inline(always)]
    #[must_use]
    pub fn a_chroma_dc_qp_delta(&mut self) -> A_CHROMA_DC_QP_DELTA_W<A_RC_CONF1_SPEC> {
        A_CHROMA_DC_QP_DELTA_W::new(self, 0)
    }
    #[doc = "Bits 3:6 - Configures video A chroma QP offset based on luma QP. Chroma QP(before map) = Luma QP + reg_chroma_qp_delta."]
    #[inline(always)]
    #[must_use]
    pub fn a_chroma_qp_delta(&mut self) -> A_CHROMA_QP_DELTA_W<A_RC_CONF1_SPEC> {
        A_CHROMA_QP_DELTA_W::new(self, 3)
    }
    #[doc = "Bits 7:12 - Configures video A allowed luma QP min value."]
    #[inline(always)]
    #[must_use]
    pub fn a_qp_min(&mut self) -> A_QP_MIN_W<A_RC_CONF1_SPEC> {
        A_QP_MIN_W::new(self, 7)
    }
    #[doc = "Bits 13:18 - Configures video A allowed luma QP max value."]
    #[inline(always)]
    #[must_use]
    pub fn a_qp_max(&mut self) -> A_QP_MAX_W<A_RC_CONF1_SPEC> {
        A_QP_MAX_W::new(self, 13)
    }
    #[doc = "Bits 19:30 - Configures vdieo A frame level predicted MB MAD value."]
    #[inline(always)]
    #[must_use]
    pub fn a_mad_frame_pred(&mut self) -> A_MAD_FRAME_PRED_W<A_RC_CONF1_SPEC> {
        A_MAD_FRAME_PRED_W::new(self, 19)
    }
}
#[doc = "Video A rate control configuration register1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`a_rc_conf1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`a_rc_conf1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct A_RC_CONF1_SPEC;
impl crate::RegisterSpec for A_RC_CONF1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`a_rc_conf1::R`](R) reader structure"]
impl crate::Readable for A_RC_CONF1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`a_rc_conf1::W`](W) writer structure"]
impl crate::Writable for A_RC_CONF1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets A_RC_CONF1 to value 0"]
impl crate::Resettable for A_RC_CONF1_SPEC {
    const RESET_VALUE: u32 = 0;
}
