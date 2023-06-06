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
pub type RECORD_PRO_PDEBUGINST_R = crate::FieldReader<u32>;
#[doc = "Field `RECORD_PDEBUGINST_SZ` reader - "]
pub type RECORD_PDEBUGINST_SZ_R = crate::FieldReader;
#[doc = "Field `RECORD_PDEBUGINST_SZ` writer - "]
pub type RECORD_PDEBUGINST_SZ_W<'a, const O: u8> =
    crate::FieldWriter<'a, PRO_CPU_RECORD_PDEBUGINST_SPEC, 8, O>;
#[doc = "Field `RECORD_PDEBUGINST_ISRC` reader - "]
pub type RECORD_PDEBUGINST_ISRC_R = crate::FieldReader;
#[doc = "Field `RECORD_PDEBUGINST_ISRC` writer - "]
pub type RECORD_PDEBUGINST_ISRC_W<'a, const O: u8> =
    crate::FieldWriter<'a, PRO_CPU_RECORD_PDEBUGINST_SPEC, 3, O>;
#[doc = "Field `RECORD_PDEBUGINST_LOOP_REP` reader - "]
pub type RECORD_PDEBUGINST_LOOP_REP_R = crate::BitReader;
#[doc = "Field `RECORD_PDEBUGINST_LOOP_REP` writer - "]
pub type RECORD_PDEBUGINST_LOOP_REP_W<'a, const O: u8> =
    crate::BitWriter<'a, PRO_CPU_RECORD_PDEBUGINST_SPEC, O>;
#[doc = "Field `RECORD_PDEBUGINST_LOOP` reader - "]
pub type RECORD_PDEBUGINST_LOOP_R = crate::BitReader;
#[doc = "Field `RECORD_PDEBUGINST_LOOP` writer - "]
pub type RECORD_PDEBUGINST_LOOP_W<'a, const O: u8> =
    crate::BitWriter<'a, PRO_CPU_RECORD_PDEBUGINST_SPEC, O>;
#[doc = "Field `RECORD_PDEBUGINST_CINTL` reader - "]
pub type RECORD_PDEBUGINST_CINTL_R = crate::FieldReader;
#[doc = "Field `RECORD_PDEBUGINST_CINTL` writer - "]
pub type RECORD_PDEBUGINST_CINTL_W<'a, const O: u8> =
    crate::FieldWriter<'a, PRO_CPU_RECORD_PDEBUGINST_SPEC, 4, O>;
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
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn record_pdebuginst_loop_rep(&self) -> RECORD_PDEBUGINST_LOOP_REP_R {
        RECORD_PDEBUGINST_LOOP_REP_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn record_pdebuginst_loop(&self) -> RECORD_PDEBUGINST_LOOP_R {
        RECORD_PDEBUGINST_LOOP_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn record_pdebuginst_cintl(&self) -> RECORD_PDEBUGINST_CINTL_R {
        RECORD_PDEBUGINST_CINTL_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRO_CPU_RECORD_PDEBUGINST")
            .field(
                "record_pro_pdebuginst",
                &format_args!("{}", self.record_pro_pdebuginst().bits()),
            )
            .field(
                "record_pdebuginst_sz",
                &format_args!("{}", self.record_pdebuginst_sz().bits()),
            )
            .field(
                "record_pdebuginst_isrc",
                &format_args!("{}", self.record_pdebuginst_isrc().bits()),
            )
            .field(
                "record_pdebuginst_loop_rep",
                &format_args!("{}", self.record_pdebuginst_loop_rep().bit()),
            )
            .field(
                "record_pdebuginst_loop",
                &format_args!("{}", self.record_pdebuginst_loop().bit()),
            )
            .field(
                "record_pdebuginst_cintl",
                &format_args!("{}", self.record_pdebuginst_cintl().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PRO_CPU_RECORD_PDEBUGINST_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn record_pdebuginst_sz(&mut self) -> RECORD_PDEBUGINST_SZ_W<0> {
        RECORD_PDEBUGINST_SZ_W::new(self)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    #[must_use]
    pub fn record_pdebuginst_isrc(&mut self) -> RECORD_PDEBUGINST_ISRC_W<12> {
        RECORD_PDEBUGINST_ISRC_W::new(self)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    #[must_use]
    pub fn record_pdebuginst_loop_rep(&mut self) -> RECORD_PDEBUGINST_LOOP_REP_W<20> {
        RECORD_PDEBUGINST_LOOP_REP_W::new(self)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    #[must_use]
    pub fn record_pdebuginst_loop(&mut self) -> RECORD_PDEBUGINST_LOOP_W<21> {
        RECORD_PDEBUGINST_LOOP_W::new(self)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    #[must_use]
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PRO_CPU_RECORD_PDEBUGINST to value 0"]
impl crate::Resettable for PRO_CPU_RECORD_PDEBUGINST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
