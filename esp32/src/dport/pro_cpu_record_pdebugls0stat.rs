#[doc = "Register `PRO_CPU_RECORD_PDEBUGLS0STAT` reader"]
pub struct R(crate::R<PRO_CPU_RECORD_PDEBUGLS0STAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRO_CPU_RECORD_PDEBUGLS0STAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRO_CPU_RECORD_PDEBUGLS0STAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRO_CPU_RECORD_PDEBUGLS0STAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRO_CPU_RECORD_PDEBUGLS0STAT` writer"]
pub struct W(crate::W<PRO_CPU_RECORD_PDEBUGLS0STAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRO_CPU_RECORD_PDEBUGLS0STAT_SPEC>;
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
impl From<crate::W<PRO_CPU_RECORD_PDEBUGLS0STAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRO_CPU_RECORD_PDEBUGLS0STAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RECORD_PRO_PDEBUGLS0STAT` reader - "]
pub type RECORD_PRO_PDEBUGLS0STAT_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RECORD_PDEBUGLS0STAT_TYPE` reader - "]
pub type RECORD_PDEBUGLS0STAT_TYPE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RECORD_PDEBUGLS0STAT_TYPE` writer - "]
pub type RECORD_PDEBUGLS0STAT_TYPE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PRO_CPU_RECORD_PDEBUGLS0STAT_SPEC, u8, u8, 4, O>;
#[doc = "Field `RECORD_PDEBUGLS0STAT_SZ` reader - "]
pub type RECORD_PDEBUGLS0STAT_SZ_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RECORD_PDEBUGLS0STAT_SZ` writer - "]
pub type RECORD_PDEBUGLS0STAT_SZ_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PRO_CPU_RECORD_PDEBUGLS0STAT_SPEC, u8, u8, 4, O>;
#[doc = "Field `RECORD_PDEBUGLS0STAT_STCOH` reader - "]
pub type RECORD_PDEBUGLS0STAT_STCOH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RECORD_PDEBUGLS0STAT_STCOH` writer - "]
pub type RECORD_PDEBUGLS0STAT_STCOH_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PRO_CPU_RECORD_PDEBUGLS0STAT_SPEC, u8, u8, 2, O>;
#[doc = "Field `RECORD_PDEBUGLS0STAT_TGT` reader - "]
pub type RECORD_PDEBUGLS0STAT_TGT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RECORD_PDEBUGLS0STAT_TGT` writer - "]
pub type RECORD_PDEBUGLS0STAT_TGT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PRO_CPU_RECORD_PDEBUGLS0STAT_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn record_pro_pdebugls0stat(&self) -> RECORD_PRO_PDEBUGLS0STAT_R {
        RECORD_PRO_PDEBUGLS0STAT_R::new(self.bits)
    }
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn record_pdebugls0stat_type(&self) -> RECORD_PDEBUGLS0STAT_TYPE_R {
        RECORD_PDEBUGLS0STAT_TYPE_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn record_pdebugls0stat_sz(&self) -> RECORD_PDEBUGLS0STAT_SZ_R {
        RECORD_PDEBUGLS0STAT_SZ_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 17:18"]
    #[inline(always)]
    pub fn record_pdebugls0stat_stcoh(&self) -> RECORD_PDEBUGLS0STAT_STCOH_R {
        RECORD_PDEBUGLS0STAT_STCOH_R::new(((self.bits >> 17) & 3) as u8)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn record_pdebugls0stat_tgt(&self) -> RECORD_PDEBUGLS0STAT_TGT_R {
        RECORD_PDEBUGLS0STAT_TGT_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn record_pdebugls0stat_type(&mut self) -> RECORD_PDEBUGLS0STAT_TYPE_W<0> {
        RECORD_PDEBUGLS0STAT_TYPE_W::new(self)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn record_pdebugls0stat_sz(&mut self) -> RECORD_PDEBUGLS0STAT_SZ_W<4> {
        RECORD_PDEBUGLS0STAT_SZ_W::new(self)
    }
    #[doc = "Bits 17:18"]
    #[inline(always)]
    pub fn record_pdebugls0stat_stcoh(&mut self) -> RECORD_PDEBUGLS0STAT_STCOH_W<17> {
        RECORD_PDEBUGLS0STAT_STCOH_W::new(self)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn record_pdebugls0stat_tgt(&mut self) -> RECORD_PDEBUGLS0STAT_TGT_W<20> {
        RECORD_PDEBUGLS0STAT_TGT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pro_cpu_record_pdebugls0stat](index.html) module"]
pub struct PRO_CPU_RECORD_PDEBUGLS0STAT_SPEC;
impl crate::RegisterSpec for PRO_CPU_RECORD_PDEBUGLS0STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pro_cpu_record_pdebugls0stat::R](R) reader structure"]
impl crate::Readable for PRO_CPU_RECORD_PDEBUGLS0STAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pro_cpu_record_pdebugls0stat::W](W) writer structure"]
impl crate::Writable for PRO_CPU_RECORD_PDEBUGLS0STAT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PRO_CPU_RECORD_PDEBUGLS0STAT to value 0"]
impl crate::Resettable for PRO_CPU_RECORD_PDEBUGLS0STAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
