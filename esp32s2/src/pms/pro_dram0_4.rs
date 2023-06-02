#[doc = "Register `PRO_DRAM0_4` reader"]
pub struct R(crate::R<PRO_DRAM0_4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRO_DRAM0_4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRO_DRAM0_4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRO_DRAM0_4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PRO_DRAM0_ILG_ST` reader - Record the illegitimate information of DBUS. \\[25:6\\]: store the bits \\[21:2\\] of DBUS address. \\[5\\]: 1 means atomic access, 0 means nonatomic access. \\[4\\]: 1 means write operation, 0 means read operation. \\[3:0\\]: DBUS0 bus byte enables."]
pub type PRO_DRAM0_ILG_ST_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:25 - Record the illegitimate information of DBUS. \\[25:6\\]: store the bits \\[21:2\\] of DBUS address. \\[5\\]: 1 means atomic access, 0 means nonatomic access. \\[4\\]: 1 means write operation, 0 means read operation. \\[3:0\\]: DBUS0 bus byte enables."]
    #[inline(always)]
    pub fn pro_dram0_ilg_st(&self) -> PRO_DRAM0_ILG_ST_R {
        PRO_DRAM0_ILG_ST_R::new(self.bits & 0x03ff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRO_DRAM0_4")
            .field(
                "pro_dram0_ilg_st",
                &format_args!("{}", self.pro_dram0_ilg_st().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PRO_DRAM0_4_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "DBUS status register.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pro_dram0_4](index.html) module"]
pub struct PRO_DRAM0_4_SPEC;
impl crate::RegisterSpec for PRO_DRAM0_4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pro_dram0_4::R](R) reader structure"]
impl crate::Readable for PRO_DRAM0_4_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PRO_DRAM0_4 to value 0"]
impl crate::Resettable for PRO_DRAM0_4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
