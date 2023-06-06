#[doc = "Register `RD_REPEAT_DATA4` reader"]
pub struct R(crate::R<RD_REPEAT_DATA4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RD_REPEAT_DATA4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RD_REPEAT_DATA4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RD_REPEAT_DATA4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RPT4_RESERVED4_1` reader - Reserved."]
pub type RPT4_RESERVED4_1_R = crate::FieldReader<u32>;
#[doc = "Field `RPT4_RESERVED4_0` reader - Reserved."]
pub type RPT4_RESERVED4_0_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:23 - Reserved."]
    #[inline(always)]
    pub fn rpt4_reserved4_1(&self) -> RPT4_RESERVED4_1_R {
        RPT4_RESERVED4_1_R::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bits 24:31 - Reserved."]
    #[inline(always)]
    pub fn rpt4_reserved4_0(&self) -> RPT4_RESERVED4_0_R {
        RPT4_RESERVED4_0_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RD_REPEAT_DATA4")
            .field(
                "rpt4_reserved4_1",
                &format_args!("{}", self.rpt4_reserved4_1().bits()),
            )
            .field(
                "rpt4_reserved4_0",
                &format_args!("{}", self.rpt4_reserved4_0().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<RD_REPEAT_DATA4_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "BLOCK0 data register 5.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_repeat_data4](index.html) module"]
pub struct RD_REPEAT_DATA4_SPEC;
impl crate::RegisterSpec for RD_REPEAT_DATA4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rd_repeat_data4::R](R) reader structure"]
impl crate::Readable for RD_REPEAT_DATA4_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RD_REPEAT_DATA4 to value 0"]
impl crate::Resettable for RD_REPEAT_DATA4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
