#[doc = "Register `PRO_EXTMEM_REG_DATE` reader"]
pub struct R(crate::R<PRO_EXTMEM_REG_DATE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRO_EXTMEM_REG_DATE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRO_EXTMEM_REG_DATE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRO_EXTMEM_REG_DATE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRO_EXTMEM_REG_DATE` writer"]
pub struct W(crate::W<PRO_EXTMEM_REG_DATE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRO_EXTMEM_REG_DATE_SPEC>;
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
impl From<crate::W<PRO_EXTMEM_REG_DATE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRO_EXTMEM_REG_DATE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRO_EXTMEM_REG_DATE` reader - Reserved."]
pub type PRO_EXTMEM_REG_DATE_R = crate::FieldReader<u32, u32>;
#[doc = "Field `PRO_EXTMEM_REG_DATE` writer - Reserved."]
pub type PRO_EXTMEM_REG_DATE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PRO_EXTMEM_REG_DATE_SPEC, u32, u32, 28, O>;
impl R {
    #[doc = "Bits 0:27 - Reserved."]
    #[inline(always)]
    pub fn pro_extmem_reg_date(&self) -> PRO_EXTMEM_REG_DATE_R {
        PRO_EXTMEM_REG_DATE_R::new((self.bits & 0x0fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:27 - Reserved."]
    #[inline(always)]
    pub fn pro_extmem_reg_date(&mut self) -> PRO_EXTMEM_REG_DATE_W<0> {
        PRO_EXTMEM_REG_DATE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "register description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pro_extmem_reg_date](index.html) module"]
pub struct PRO_EXTMEM_REG_DATE_SPEC;
impl crate::RegisterSpec for PRO_EXTMEM_REG_DATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pro_extmem_reg_date::R](R) reader structure"]
impl crate::Readable for PRO_EXTMEM_REG_DATE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pro_extmem_reg_date::W](W) writer structure"]
impl crate::Writable for PRO_EXTMEM_REG_DATE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PRO_EXTMEM_REG_DATE to value 0x0190_4180"]
impl crate::Resettable for PRO_EXTMEM_REG_DATE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0190_4180
    }
}
