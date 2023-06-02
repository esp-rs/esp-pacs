#[doc = "Register `PRO_CPU_RECORD_PDEBUGSTATUS` reader"]
pub struct R(crate::R<PRO_CPU_RECORD_PDEBUGSTATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRO_CPU_RECORD_PDEBUGSTATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRO_CPU_RECORD_PDEBUGSTATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRO_CPU_RECORD_PDEBUGSTATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRO_CPU_RECORD_PDEBUGSTATUS` writer"]
pub struct W(crate::W<PRO_CPU_RECORD_PDEBUGSTATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRO_CPU_RECORD_PDEBUGSTATUS_SPEC>;
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
impl From<crate::W<PRO_CPU_RECORD_PDEBUGSTATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRO_CPU_RECORD_PDEBUGSTATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RECORD_PRO_PDEBUGSTATUS` reader - "]
pub type RECORD_PRO_PDEBUGSTATUS_R = crate::FieldReader;
#[doc = "Field `RECORD_PDEBUGSTATUS_BBCAUSE` reader - "]
pub type RECORD_PDEBUGSTATUS_BBCAUSE_R = crate::FieldReader;
#[doc = "Field `RECORD_PDEBUGSTATUS_BBCAUSE` writer - "]
pub type RECORD_PDEBUGSTATUS_BBCAUSE_W<'a, const O: u8> =
    crate::FieldWriter<'a, PRO_CPU_RECORD_PDEBUGSTATUS_SPEC, 6, O>;
#[doc = "Field `RECORD_PDEBUGSTATUS_INSNTYPE` reader - "]
pub type RECORD_PDEBUGSTATUS_INSNTYPE_R = crate::FieldReader;
#[doc = "Field `RECORD_PDEBUGSTATUS_INSNTYPE` writer - "]
pub type RECORD_PDEBUGSTATUS_INSNTYPE_W<'a, const O: u8> =
    crate::FieldWriter<'a, PRO_CPU_RECORD_PDEBUGSTATUS_SPEC, 6, O>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn record_pro_pdebugstatus(&self) -> RECORD_PRO_PDEBUGSTATUS_R {
        RECORD_PRO_PDEBUGSTATUS_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn record_pdebugstatus_bbcause(&self) -> RECORD_PDEBUGSTATUS_BBCAUSE_R {
        RECORD_PDEBUGSTATUS_BBCAUSE_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn record_pdebugstatus_insntype(&self) -> RECORD_PDEBUGSTATUS_INSNTYPE_R {
        RECORD_PDEBUGSTATUS_INSNTYPE_R::new((self.bits & 0x3f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRO_CPU_RECORD_PDEBUGSTATUS")
            .field(
                "record_pro_pdebugstatus",
                &format_args!("{}", self.record_pro_pdebugstatus().bits()),
            )
            .field(
                "record_pdebugstatus_bbcause",
                &format_args!("{}", self.record_pdebugstatus_bbcause().bits()),
            )
            .field(
                "record_pdebugstatus_insntype",
                &format_args!("{}", self.record_pdebugstatus_insntype().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PRO_CPU_RECORD_PDEBUGSTATUS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    #[must_use]
    pub fn record_pdebugstatus_bbcause(&mut self) -> RECORD_PDEBUGSTATUS_BBCAUSE_W<0> {
        RECORD_PDEBUGSTATUS_BBCAUSE_W::new(self)
    }
    #[doc = "Bits 0:5"]
    #[inline(always)]
    #[must_use]
    pub fn record_pdebugstatus_insntype(&mut self) -> RECORD_PDEBUGSTATUS_INSNTYPE_W<0> {
        RECORD_PDEBUGSTATUS_INSNTYPE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pro_cpu_record_pdebugstatus](index.html) module"]
pub struct PRO_CPU_RECORD_PDEBUGSTATUS_SPEC;
impl crate::RegisterSpec for PRO_CPU_RECORD_PDEBUGSTATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pro_cpu_record_pdebugstatus::R](R) reader structure"]
impl crate::Readable for PRO_CPU_RECORD_PDEBUGSTATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pro_cpu_record_pdebugstatus::W](W) writer structure"]
impl crate::Writable for PRO_CPU_RECORD_PDEBUGSTATUS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PRO_CPU_RECORD_PDEBUGSTATUS to value 0"]
impl crate::Resettable for PRO_CPU_RECORD_PDEBUGSTATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
