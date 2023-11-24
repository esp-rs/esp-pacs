#[doc = "Register `PRO_DPORT_7` reader"]
pub type R = crate::R<PRO_DPORT_7_SPEC>;
#[doc = "Field `PRO_DPORT_ILG_ST` reader - Record the illegitimate information of PeriBus1. \\[25:6\\]: store the bits \\[21:2\\] of PeriBus1 address. \\[5\\]: 1 means atomic access, 0 means nonatomic access. \\[4\\]: if bits \\[31:22\\] of PeriBus1 address are 0xfd, then the bit value is 1, otherwise it is 0. \\[3:0\\]: PeriBus1 byte enables."]
pub type PRO_DPORT_ILG_ST_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:25 - Record the illegitimate information of PeriBus1. \\[25:6\\]: store the bits \\[21:2\\] of PeriBus1 address. \\[5\\]: 1 means atomic access, 0 means nonatomic access. \\[4\\]: if bits \\[31:22\\] of PeriBus1 address are 0xfd, then the bit value is 1, otherwise it is 0. \\[3:0\\]: PeriBus1 byte enables."]
    #[inline(always)]
    pub fn pro_dport_ilg_st(&self) -> PRO_DPORT_ILG_ST_R {
        PRO_DPORT_ILG_ST_R::new(self.bits & 0x03ff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRO_DPORT_7")
            .field(
                "pro_dport_ilg_st",
                &format_args!("{}", self.pro_dport_ilg_st().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PRO_DPORT_7_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "PeriBus1 status register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_dport_7::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRO_DPORT_7_SPEC;
impl crate::RegisterSpec for PRO_DPORT_7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pro_dport_7::R`](R) reader structure"]
impl crate::Readable for PRO_DPORT_7_SPEC {}
#[doc = "`reset()` method sets PRO_DPORT_7 to value 0"]
impl crate::Resettable for PRO_DPORT_7_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
