#[doc = "Register `PRO_IRAM0_5` reader"]
pub struct R(crate::R<PRO_IRAM0_5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRO_IRAM0_5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRO_IRAM0_5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRO_IRAM0_5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PRO_IRAM0_ILG_ST` reader - Record the illegitimate information of IBUS. \\[21:2\\]: store the bits \\[21:2\\] of IBUS address. \\[1\\]: 1 means data access, 0 means instruction access. \\[0\\]: 1 means write operation, 0 means read operation."]
pub type PRO_IRAM0_ILG_ST_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:21 - Record the illegitimate information of IBUS. \\[21:2\\]: store the bits \\[21:2\\] of IBUS address. \\[1\\]: 1 means data access, 0 means instruction access. \\[0\\]: 1 means write operation, 0 means read operation."]
    #[inline(always)]
    pub fn pro_iram0_ilg_st(&self) -> PRO_IRAM0_ILG_ST_R {
        PRO_IRAM0_ILG_ST_R::new(self.bits & 0x003f_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRO_IRAM0_5")
            .field(
                "pro_iram0_ilg_st",
                &format_args!("{}", self.pro_iram0_ilg_st().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PRO_IRAM0_5_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "IBUS status register.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pro_iram0_5](index.html) module"]
pub struct PRO_IRAM0_5_SPEC;
impl crate::RegisterSpec for PRO_IRAM0_5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pro_iram0_5::R](R) reader structure"]
impl crate::Readable for PRO_IRAM0_5_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PRO_IRAM0_5 to value 0"]
impl crate::Resettable for PRO_IRAM0_5_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
