#[doc = "Register `PRO_AHB_4` reader"]
pub struct R(crate::R<PRO_AHB_4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRO_AHB_4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRO_AHB_4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRO_AHB_4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PRO_AHB_ILG_ST` reader - Record the illegitimate information of PeriBus2. \\[31:2\\]: store the bits \\[31:2\\] of PeriBus2 address. \\[1\\]: 1 means data access, 0 means instruction access. \\[0\\]: 1 means write operation, 0 means read operation."]
pub type PRO_AHB_ILG_ST_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Record the illegitimate information of PeriBus2. \\[31:2\\]: store the bits \\[31:2\\] of PeriBus2 address. \\[1\\]: 1 means data access, 0 means instruction access. \\[0\\]: 1 means write operation, 0 means read operation."]
    #[inline(always)]
    pub fn pro_ahb_ilg_st(&self) -> PRO_AHB_ILG_ST_R {
        PRO_AHB_ILG_ST_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRO_AHB_4")
            .field(
                "pro_ahb_ilg_st",
                &format_args!("{}", self.pro_ahb_ilg_st().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PRO_AHB_4_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "PeriBus2 status register.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pro_ahb_4](index.html) module"]
pub struct PRO_AHB_4_SPEC;
impl crate::RegisterSpec for PRO_AHB_4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pro_ahb_4::R](R) reader structure"]
impl crate::Readable for PRO_AHB_4_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PRO_AHB_4 to value 0"]
impl crate::Resettable for PRO_AHB_4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
