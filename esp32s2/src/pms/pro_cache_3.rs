#[doc = "Register `PRO_CACHE_3` reader"]
pub struct R(crate::R<PRO_CACHE_3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRO_CACHE_3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRO_CACHE_3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRO_CACHE_3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PRO_CACHE_ILG_ST_I` reader - Record the illegitimate information of ICache to access memory. \\[16\\]: access enable, active low. \\[15:4\\]: store the bits \\[11:0\\] of address. \\[3:0\\]: Icache bus write byte enables, active low."]
pub type PRO_CACHE_ILG_ST_I_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:16 - Record the illegitimate information of ICache to access memory. \\[16\\]: access enable, active low. \\[15:4\\]: store the bits \\[11:0\\] of address. \\[3:0\\]: Icache bus write byte enables, active low."]
    #[inline(always)]
    pub fn pro_cache_ilg_st_i(&self) -> PRO_CACHE_ILG_ST_I_R {
        PRO_CACHE_ILG_ST_I_R::new(self.bits & 0x0001_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRO_CACHE_3")
            .field(
                "pro_cache_ilg_st_i",
                &format_args!("{}", self.pro_cache_ilg_st_i().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PRO_CACHE_3_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Icache status register.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pro_cache_3](index.html) module"]
pub struct PRO_CACHE_3_SPEC;
impl crate::RegisterSpec for PRO_CACHE_3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pro_cache_3::R](R) reader structure"]
impl crate::Readable for PRO_CACHE_3_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PRO_CACHE_3 to value 0"]
impl crate::Resettable for PRO_CACHE_3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
