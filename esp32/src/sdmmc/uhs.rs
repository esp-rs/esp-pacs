#[doc = "Register `UHS` reader"]
pub struct R(crate::R<UHS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UHS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UHS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UHS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UHS` writer"]
pub struct W(crate::W<UHS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UHS_SPEC>;
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
impl From<crate::W<UHS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UHS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DDR` reader - DDR mode selecton,1 bit for each card. 0-Non-DDR mdoe. 1-DDR mdoe."]
pub type DDR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DDR` writer - DDR mode selecton,1 bit for each card. 0-Non-DDR mdoe. 1-DDR mdoe."]
pub type DDR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, UHS_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 16:17 - DDR mode selecton,1 bit for each card. 0-Non-DDR mdoe. 1-DDR mdoe."]
    #[inline(always)]
    pub fn ddr(&self) -> DDR_R {
        DDR_R::new(((self.bits >> 16) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 16:17 - DDR mode selecton,1 bit for each card. 0-Non-DDR mdoe. 1-DDR mdoe."]
    #[inline(always)]
    pub fn ddr(&mut self) -> DDR_W<16> {
        DDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UHS-1 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uhs](index.html) module"]
pub struct UHS_SPEC;
impl crate::RegisterSpec for UHS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uhs::R](R) reader structure"]
impl crate::Readable for UHS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uhs::W](W) writer structure"]
impl crate::Writable for UHS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UHS to value 0"]
impl crate::Resettable for UHS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
