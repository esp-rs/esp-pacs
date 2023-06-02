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
#[doc = "Field `PKT_THRS` reader - This register is used to configure the maximum value of the packet length when UHCI_HEAD_EN is 0."]
pub type PKT_THRS_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PKT_THRS` writer - This register is used to configure the maximum value of the packet length when UHCI_HEAD_EN is 0."]
pub type PKT_THRS_W<'a, const O: u8> = crate::FieldWriter<'a, PKT_THRES_SPEC, 13, O, u16, u16>;
impl R {
    #[doc = "Bits 0:12 - This register is used to configure the maximum value of the packet length when UHCI_HEAD_EN is 0."]
    #[inline(always)]
    pub fn pkt_thrs(&self) -> PKT_THRS_R {
        PKT_THRS_R::new((self.bits & 0x1fff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PKT_THRES")
            .field("pkt_thrs", &format_args!("{}", self.pkt_thrs().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PKT_THRES_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:12 - This register is used to configure the maximum value of the packet length when UHCI_HEAD_EN is 0."]
    #[inline(always)]
    #[must_use]
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
#[doc = "Configure register for packet length\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pkt_thres](index.html) module"]
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PKT_THRES to value 0x80"]
impl crate::Resettable for PKT_THRES_SPEC {
    const RESET_VALUE: Self::Ux = 0x80;
}
