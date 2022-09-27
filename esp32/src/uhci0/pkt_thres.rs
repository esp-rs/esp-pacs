#[doc = "Register `PKT_THRES` reader"]
pub struct R(crate::R<PKT_THRES_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PKT_THRES_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PKT_THRES_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PKT_THRES_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PKT_THRES` writer"]
pub struct W(crate::W<PKT_THRES_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PKT_THRES_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<PKT_THRES_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PKT_THRES_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PKT_THRS` reader - when the amount of packet payload is greater than this value the process of receiving data is done."]
pub type PKT_THRS_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PKT_THRS` writer - when the amount of packet payload is greater than this value the process of receiving data is done."]
pub type PKT_THRS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PKT_THRES_SPEC, u16, u16, 13, O>;
impl R {
    #[doc = "Bits 0:12 - when the amount of packet payload is greater than this value the process of receiving data is done."]
    #[inline(always)]
    pub fn pkt_thrs(&self) -> PKT_THRS_R {
        PKT_THRS_R::new((self.bits & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12 - when the amount of packet payload is greater than this value the process of receiving data is done."]
    #[inline(always)]
    pub fn pkt_thrs(&mut self) -> PKT_THRS_W<0> {
        PKT_THRS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pkt_thres](index.html) module"]
pub struct PKT_THRES_SPEC;
impl crate::RegisterSpec for PKT_THRES_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pkt_thres::R](R) reader structure"]
impl crate::Readable for PKT_THRES_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pkt_thres::W](W) writer structure"]
impl crate::Writable for PKT_THRES_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PKT_THRES to value 0x80"]
impl crate::Resettable for PKT_THRES_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x80
    }
}
