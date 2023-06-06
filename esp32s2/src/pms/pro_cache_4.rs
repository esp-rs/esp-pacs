#[doc = "Register `PRO_CACHE_4` reader"]
pub struct R(crate::R<PRO_CACHE_4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRO_CACHE_4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRO_CACHE_4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRO_CACHE_4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PRO_CACHE_ILG_ST_D` reader - Record the illegitimate information of Dcache to access memory. \\[16\\]: access enable, active low. \\[15:4\\]: store the bits \\[11:0\\] of address. \\[3:0\\]: Dcache bus write byte enables, active low."]
pub type PRO_CACHE_ILG_ST_D_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:16 - Record the illegitimate information of Dcache to access memory. \\[16\\]: access enable, active low. \\[15:4\\]: store the bits \\[11:0\\] of address. \\[3:0\\]: Dcache bus write byte enables, active low."]
    #[inline(always)]
    pub fn pro_cache_ilg_st_d(&self) -> PRO_CACHE_ILG_ST_D_R {
        PRO_CACHE_ILG_ST_D_R::new(self.bits & 0x0001_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRO_CACHE_4")
            .field(
                "pro_cache_ilg_st_d",
                &format_args!("{}", self.pro_cache_ilg_st_d().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PRO_CACHE_4_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Dcache status register.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pro_cache_4](index.html) module"]
pub struct PRO_CACHE_4_SPEC;
impl crate::RegisterSpec for PRO_CACHE_4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pro_cache_4::R](R) reader structure"]
impl crate::Readable for PRO_CACHE_4_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PRO_CACHE_4 to value 0"]
impl crate::Resettable for PRO_CACHE_4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
