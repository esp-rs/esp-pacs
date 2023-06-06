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
pub type RECORD_PRO_PDEBUGLS0STAT_R = crate::FieldReader<u32>;
#[doc = "Field `RECORD_PDEBUGLS0STAT_TYPE` reader - "]
pub type RECORD_PDEBUGLS0STAT_TYPE_R = crate::FieldReader;
#[doc = "Field `RECORD_PDEBUGLS0STAT_TYPE` writer - "]
pub type RECORD_PDEBUGLS0STAT_TYPE_W<'a, const O: u8> =
    crate::FieldWriter<'a, PRO_CPU_RECORD_PDEBUGLS0STAT_SPEC, 4, O>;
#[doc = "Field `RECORD_PDEBUGLS0STAT_SZ` reader - "]
pub type RECORD_PDEBUGLS0STAT_SZ_R = crate::FieldReader;
#[doc = "Field `RECORD_PDEBUGLS0STAT_SZ` writer - "]
pub type RECORD_PDEBUGLS0STAT_SZ_W<'a, const O: u8> =
    crate::FieldWriter<'a, PRO_CPU_RECORD_PDEBUGLS0STAT_SPEC, 4, O>;
#[doc = "Field `RECORD_PDEBUGLS0STAT_DTLBM` reader - "]
pub type RECORD_PDEBUGLS0STAT_DTLBM_R = crate::BitReader;
#[doc = "Field `RECORD_PDEBUGLS0STAT_DTLBM` writer - "]
pub type RECORD_PDEBUGLS0STAT_DTLBM_W<'a, const O: u8> =
    crate::BitWriter<'a, PRO_CPU_RECORD_PDEBUGLS0STAT_SPEC, O>;
#[doc = "Field `RECORD_PDEBUGLS0STAT_DCM` reader - "]
pub type RECORD_PDEBUGLS0STAT_DCM_R = crate::BitReader;
#[doc = "Field `RECORD_PDEBUGLS0STAT_DCM` writer - "]
pub type RECORD_PDEBUGLS0STAT_DCM_W<'a, const O: u8> =
    crate::BitWriter<'a, PRO_CPU_RECORD_PDEBUGLS0STAT_SPEC, O>;
#[doc = "Field `RECORD_PDEBUGLS0STAT_DCH` reader - "]
pub type RECORD_PDEBUGLS0STAT_DCH_R = crate::BitReader;
#[doc = "Field `RECORD_PDEBUGLS0STAT_DCH` writer - "]
pub type RECORD_PDEBUGLS0STAT_DCH_W<'a, const O: u8> =
    crate::BitWriter<'a, PRO_CPU_RECORD_PDEBUGLS0STAT_SPEC, O>;
#[doc = "Field `RECORD_PDEBUGLS0STAT_UC` reader - "]
pub type RECORD_PDEBUGLS0STAT_UC_R = crate::BitReader;
#[doc = "Field `RECORD_PDEBUGLS0STAT_UC` writer - "]
pub type RECORD_PDEBUGLS0STAT_UC_W<'a, const O: u8> =
    crate::BitWriter<'a, PRO_CPU_RECORD_PDEBUGLS0STAT_SPEC, O>;
#[doc = "Field `RECORD_PDEBUGLS0STAT_WB` reader - "]
pub type RECORD_PDEBUGLS0STAT_WB_R = crate::BitReader;
#[doc = "Field `RECORD_PDEBUGLS0STAT_WB` writer - "]
pub type RECORD_PDEBUGLS0STAT_WB_W<'a, const O: u8> =
    crate::BitWriter<'a, PRO_CPU_RECORD_PDEBUGLS0STAT_SPEC, O>;
#[doc = "Field `RECORD_PDEBUGLS0STAT_COH` reader - "]
pub type RECORD_PDEBUGLS0STAT_COH_R = crate::BitReader;
#[doc = "Field `RECORD_PDEBUGLS0STAT_COH` writer - "]
pub type RECORD_PDEBUGLS0STAT_COH_W<'a, const O: u8> =
    crate::BitWriter<'a, PRO_CPU_RECORD_PDEBUGLS0STAT_SPEC, O>;
#[doc = "Field `RECORD_PDEBUGLS0STAT_STCOH` reader - "]
pub type RECORD_PDEBUGLS0STAT_STCOH_R = crate::FieldReader;
#[doc = "Field `RECORD_PDEBUGLS0STAT_STCOH` writer - "]
pub type RECORD_PDEBUGLS0STAT_STCOH_W<'a, const O: u8> =
    crate::FieldWriter<'a, PRO_CPU_RECORD_PDEBUGLS0STAT_SPEC, 2, O>;
#[doc = "Field `RECORD_PDEBUGLS0STAT_TGT` reader - "]
pub type RECORD_PDEBUGLS0STAT_TGT_R = crate::FieldReader;
#[doc = "Field `RECORD_PDEBUGLS0STAT_TGT` writer - "]
pub type RECORD_PDEBUGLS0STAT_TGT_W<'a, const O: u8> =
    crate::FieldWriter<'a, PRO_CPU_RECORD_PDEBUGLS0STAT_SPEC, 4, O>;
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
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn record_pdebugls0stat_dtlbm(&self) -> RECORD_PDEBUGLS0STAT_DTLBM_R {
        RECORD_PDEBUGLS0STAT_DTLBM_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn record_pdebugls0stat_dcm(&self) -> RECORD_PDEBUGLS0STAT_DCM_R {
        RECORD_PDEBUGLS0STAT_DCM_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn record_pdebugls0stat_dch(&self) -> RECORD_PDEBUGLS0STAT_DCH_R {
        RECORD_PDEBUGLS0STAT_DCH_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn record_pdebugls0stat_uc(&self) -> RECORD_PDEBUGLS0STAT_UC_R {
        RECORD_PDEBUGLS0STAT_UC_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn record_pdebugls0stat_wb(&self) -> RECORD_PDEBUGLS0STAT_WB_R {
        RECORD_PDEBUGLS0STAT_WB_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn record_pdebugls0stat_coh(&self) -> RECORD_PDEBUGLS0STAT_COH_R {
        RECORD_PDEBUGLS0STAT_COH_R::new(((self.bits >> 16) & 1) != 0)
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
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRO_CPU_RECORD_PDEBUGLS0STAT")
            .field(
                "record_pro_pdebugls0stat",
                &format_args!("{}", self.record_pro_pdebugls0stat().bits()),
            )
            .field(
                "record_pdebugls0stat_type",
                &format_args!("{}", self.record_pdebugls0stat_type().bits()),
            )
            .field(
                "record_pdebugls0stat_sz",
                &format_args!("{}", self.record_pdebugls0stat_sz().bits()),
            )
            .field(
                "record_pdebugls0stat_dtlbm",
                &format_args!("{}", self.record_pdebugls0stat_dtlbm().bit()),
            )
            .field(
                "record_pdebugls0stat_dcm",
                &format_args!("{}", self.record_pdebugls0stat_dcm().bit()),
            )
            .field(
                "record_pdebugls0stat_dch",
                &format_args!("{}", self.record_pdebugls0stat_dch().bit()),
            )
            .field(
                "record_pdebugls0stat_uc",
                &format_args!("{}", self.record_pdebugls0stat_uc().bit()),
            )
            .field(
                "record_pdebugls0stat_wb",
                &format_args!("{}", self.record_pdebugls0stat_wb().bit()),
            )
            .field(
                "record_pdebugls0stat_coh",
                &format_args!("{}", self.record_pdebugls0stat_coh().bit()),
            )
            .field(
                "record_pdebugls0stat_stcoh",
                &format_args!("{}", self.record_pdebugls0stat_stcoh().bits()),
            )
            .field(
                "record_pdebugls0stat_tgt",
                &format_args!("{}", self.record_pdebugls0stat_tgt().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PRO_CPU_RECORD_PDEBUGLS0STAT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    #[must_use]
    pub fn record_pdebugls0stat_type(&mut self) -> RECORD_PDEBUGLS0STAT_TYPE_W<0> {
        RECORD_PDEBUGLS0STAT_TYPE_W::new(self)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    #[must_use]
    pub fn record_pdebugls0stat_sz(&mut self) -> RECORD_PDEBUGLS0STAT_SZ_W<4> {
        RECORD_PDEBUGLS0STAT_SZ_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn record_pdebugls0stat_dtlbm(&mut self) -> RECORD_PDEBUGLS0STAT_DTLBM_W<8> {
        RECORD_PDEBUGLS0STAT_DTLBM_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn record_pdebugls0stat_dcm(&mut self) -> RECORD_PDEBUGLS0STAT_DCM_W<9> {
        RECORD_PDEBUGLS0STAT_DCM_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn record_pdebugls0stat_dch(&mut self) -> RECORD_PDEBUGLS0STAT_DCH_W<10> {
        RECORD_PDEBUGLS0STAT_DCH_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn record_pdebugls0stat_uc(&mut self) -> RECORD_PDEBUGLS0STAT_UC_W<12> {
        RECORD_PDEBUGLS0STAT_UC_W::new(self)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn record_pdebugls0stat_wb(&mut self) -> RECORD_PDEBUGLS0STAT_WB_W<13> {
        RECORD_PDEBUGLS0STAT_WB_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn record_pdebugls0stat_coh(&mut self) -> RECORD_PDEBUGLS0STAT_COH_W<16> {
        RECORD_PDEBUGLS0STAT_COH_W::new(self)
    }
    #[doc = "Bits 17:18"]
    #[inline(always)]
    #[must_use]
    pub fn record_pdebugls0stat_stcoh(&mut self) -> RECORD_PDEBUGLS0STAT_STCOH_W<17> {
        RECORD_PDEBUGLS0STAT_STCOH_W::new(self)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    #[must_use]
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PRO_CPU_RECORD_PDEBUGLS0STAT to value 0"]
impl crate::Resettable for PRO_CPU_RECORD_PDEBUGLS0STAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
