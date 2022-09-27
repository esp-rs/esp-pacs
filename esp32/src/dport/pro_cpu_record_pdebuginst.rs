#[doc = "Register `PRO_CPU_RECORD_PDEBUGINST` reader"]
pub struct R(crate::R<PRO_CPU_RECORD_PDEBUGINST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRO_CPU_RECORD_PDEBUGINST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRO_CPU_RECORD_PDEBUGINST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRO_CPU_RECORD_PDEBUGINST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRO_CPU_RECORD_PDEBUGINST` writer"]
pub struct W(crate::W<PRO_CPU_RECORD_PDEBUGINST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRO_CPU_RECORD_PDEBUGINST_SPEC>;
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
impl From<crate::W<PRO_CPU_RECORD_PDEBUGINST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRO_CPU_RECORD_PDEBUGINST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RECORD_PRO_PDEBUGINST` reader - "]
pub type RECORD_PRO_PDEBUGINST_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RECORD_PDEBUGINST_SZ` reader - "]
pub type RECORD_PDEBUGINST_SZ_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RECORD_PDEBUGINST_SZ` writer - "]
pub type RECORD_PDEBUGINST_SZ_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PRO_CPU_RECORD_PDEBUGINST_SPEC, u8, u8, 8, O>;
#[doc = "Field `RECORD_PDEBUGINST_ISRC` reader - "]
pub type RECORD_PDEBUGINST_ISRC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RECORD_PDEBUGINST_ISRC` writer - "]
pub type RECORD_PDEBUGINST_ISRC_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PRO_CPU_RECORD_PDEBUGINST_SPEC, u8, u8, 3, O>;
#[doc = "Field `RECORD_PDEBUGINST_CINTL` reader - "]
pub type RECORD_PDEBUGINST_CINTL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RECORD_PDEBUGINST_CINTL` writer - "]
pub type RECORD_PDEBUGINST_CINTL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PRO_CPU_RECORD_PDEBUGINST_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn record_pro_pdebuginst(&self) -> RECORD_PRO_PDEBUGINST_R {
        RECORD_PRO_PDEBUGINST_R::new(self.bits)
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn record_pdebuginst_sz(&self) -> RECORD_PDEBUGINST_SZ_R {
        RECORD_PDEBUGINST_SZ_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn record_pdebuginst_isrc(&self) -> RECORD_PDEBUGINST_ISRC_R {
        RECORD_PDEBUGINST_ISRC_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn record_pdebuginst_cintl(&self) -> RECORD_PDEBUGINST_CINTL_R {
        RECORD_PDEBUGINST_CINTL_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn record_pdebuginst_sz(&mut self) -> RECORD_PDEBUGINST_SZ_W<0> {
        RECORD_PDEBUGINST_SZ_W::new(self)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn record_pdebuginst_isrc(&mut self) -> RECORD_PDEBUGINST_ISRC_W<12> {
        RECORD_PDEBUGINST_ISRC_W::new(self)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn record_pdebuginst_cintl(&mut self) -> RECORD_PDEBUGINST_CINTL_W<24> {
        RECORD_PDEBUGINST_CINTL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pro_cpu_record_pdebuginst](index.html) module"]
pub struct PRO_CPU_RECORD_PDEBUGINST_SPEC;
impl crate::RegisterSpec for PRO_CPU_RECORD_PDEBUGINST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pro_cpu_record_pdebuginst::R](R) reader structure"]
impl crate::Readable for PRO_CPU_RECORD_PDEBUGINST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pro_cpu_record_pdebuginst::W](W) writer structure"]
impl crate::Writable for PRO_CPU_RECORD_PDEBUGINST_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PRO_CPU_RECORD_PDEBUGINST to value 0"]
impl crate::Resettable for PRO_CPU_RECORD_PDEBUGINST_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
