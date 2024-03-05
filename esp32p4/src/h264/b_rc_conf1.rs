#[doc = "Register `B_RC_CONF1` reader"]
pub type R = crate::R<B_RC_CONF1_SPEC>;
#[doc = "Register `B_RC_CONF1` writer"]
pub type W = crate::W<B_RC_CONF1_SPEC>;
#[doc = "Field `B_CHROMA_DC_QP_DELTA` reader - Configures video B chroma DC QP offset based on Chroma QP. Chroma DC QP = Chroma QP(after map) + reg_chroma_dc_qp_delta."]
pub type B_CHROMA_DC_QP_DELTA_R = crate::FieldReader;
#[doc = "Field `B_CHROMA_DC_QP_DELTA` writer - Configures video B chroma DC QP offset based on Chroma QP. Chroma DC QP = Chroma QP(after map) + reg_chroma_dc_qp_delta."]
pub type B_CHROMA_DC_QP_DELTA_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `B_CHROMA_QP_DELTA` reader - Configures video B chroma QP offset based on luma QP. Chroma QP(before map) = Luma QP + reg_chroma_qp_delta."]
pub type B_CHROMA_QP_DELTA_R = crate::FieldReader;
#[doc = "Field `B_CHROMA_QP_DELTA` writer - Configures video B chroma QP offset based on luma QP. Chroma QP(before map) = Luma QP + reg_chroma_qp_delta."]
pub type B_CHROMA_QP_DELTA_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `B_QP_MIN` reader - Configures video B allowed luma QP min value."]
pub type B_QP_MIN_R = crate::FieldReader;
#[doc = "Field `B_QP_MIN` writer - Configures video B allowed luma QP min value."]
pub type B_QP_MIN_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `B_QP_MAX` reader - Configures video B allowed luma QP max value."]
pub type B_QP_MAX_R = crate::FieldReader;
#[doc = "Field `B_QP_MAX` writer - Configures video B allowed luma QP max value."]
pub type B_QP_MAX_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `B_MAD_FRAME_PRED` reader - Configures vdieo B frame level predicted MB MAD value."]
pub type B_MAD_FRAME_PRED_R = crate::FieldReader<u16>;
#[doc = "Field `B_MAD_FRAME_PRED` writer - Configures vdieo B frame level predicted MB MAD value."]
pub type B_MAD_FRAME_PRED_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:2 - Configures video B chroma DC QP offset based on Chroma QP. Chroma DC QP = Chroma QP(after map) + reg_chroma_dc_qp_delta."]
    #[inline(always)]
    pub fn b_chroma_dc_qp_delta(&self) -> B_CHROMA_DC_QP_DELTA_R {
        B_CHROMA_DC_QP_DELTA_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:6 - Configures video B chroma QP offset based on luma QP. Chroma QP(before map) = Luma QP + reg_chroma_qp_delta."]
    #[inline(always)]
    pub fn b_chroma_qp_delta(&self) -> B_CHROMA_QP_DELTA_R {
        B_CHROMA_QP_DELTA_R::new(((self.bits >> 3) & 0x0f) as u8)
    }
    #[doc = "Bits 7:12 - Configures video B allowed luma QP min value."]
    #[inline(always)]
    pub fn b_qp_min(&self) -> B_QP_MIN_R {
        B_QP_MIN_R::new(((self.bits >> 7) & 0x3f) as u8)
    }
    #[doc = "Bits 13:18 - Configures video B allowed luma QP max value."]
    #[inline(always)]
    pub fn b_qp_max(&self) -> B_QP_MAX_R {
        B_QP_MAX_R::new(((self.bits >> 13) & 0x3f) as u8)
    }
    #[doc = "Bits 19:30 - Configures vdieo B frame level predicted MB MAD value."]
    #[inline(always)]
    pub fn b_mad_frame_pred(&self) -> B_MAD_FRAME_PRED_R {
        B_MAD_FRAME_PRED_R::new(((self.bits >> 19) & 0x0fff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("B_RC_CONF1")
            .field(
                "b_chroma_dc_qp_delta",
                &format_args!("{}", self.b_chroma_dc_qp_delta().bits()),
            )
            .field(
                "b_chroma_qp_delta",
                &format_args!("{}", self.b_chroma_qp_delta().bits()),
            )
            .field("b_qp_min", &format_args!("{}", self.b_qp_min().bits()))
            .field("b_qp_max", &format_args!("{}", self.b_qp_max().bits()))
            .field(
                "b_mad_frame_pred",
                &format_args!("{}", self.b_mad_frame_pred().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<B_RC_CONF1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:2 - Configures video B chroma DC QP offset based on Chroma QP. Chroma DC QP = Chroma QP(after map) + reg_chroma_dc_qp_delta."]
    #[inline(always)]
    #[must_use]
    pub fn b_chroma_dc_qp_delta(&mut self) -> B_CHROMA_DC_QP_DELTA_W<B_RC_CONF1_SPEC> {
        B_CHROMA_DC_QP_DELTA_W::new(self, 0)
    }
    #[doc = "Bits 3:6 - Configures video B chroma QP offset based on luma QP. Chroma QP(before map) = Luma QP + reg_chroma_qp_delta."]
    #[inline(always)]
    #[must_use]
    pub fn b_chroma_qp_delta(&mut self) -> B_CHROMA_QP_DELTA_W<B_RC_CONF1_SPEC> {
        B_CHROMA_QP_DELTA_W::new(self, 3)
    }
    #[doc = "Bits 7:12 - Configures video B allowed luma QP min value."]
    #[inline(always)]
    #[must_use]
    pub fn b_qp_min(&mut self) -> B_QP_MIN_W<B_RC_CONF1_SPEC> {
        B_QP_MIN_W::new(self, 7)
    }
    #[doc = "Bits 13:18 - Configures video B allowed luma QP max value."]
    #[inline(always)]
    #[must_use]
    pub fn b_qp_max(&mut self) -> B_QP_MAX_W<B_RC_CONF1_SPEC> {
        B_QP_MAX_W::new(self, 13)
    }
    #[doc = "Bits 19:30 - Configures vdieo B frame level predicted MB MAD value."]
    #[inline(always)]
    #[must_use]
    pub fn b_mad_frame_pred(&mut self) -> B_MAD_FRAME_PRED_W<B_RC_CONF1_SPEC> {
        B_MAD_FRAME_PRED_W::new(self, 19)
    }
}
#[doc = "Video B rate control configuration register1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`b_rc_conf1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`b_rc_conf1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct B_RC_CONF1_SPEC;
impl crate::RegisterSpec for B_RC_CONF1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`b_rc_conf1::R`](R) reader structure"]
impl crate::Readable for B_RC_CONF1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`b_rc_conf1::W`](W) writer structure"]
impl crate::Writable for B_RC_CONF1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets B_RC_CONF1 to value 0"]
impl crate::Resettable for B_RC_CONF1_SPEC {
    const RESET_VALUE: u32 = 0;
}
