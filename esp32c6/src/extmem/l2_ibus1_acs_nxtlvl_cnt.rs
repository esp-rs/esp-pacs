#[doc = "Register `L2_IBUS1_ACS_NXTLVL_CNT` reader"]
pub type R = crate::R<L2_IBUS1_ACS_NXTLVL_CNT_SPEC>;
#[doc = "Field `L2_IBUS1_NXTLVL_CNT` reader - The register records the number of times that L2-Cache accesses external memory due to L1-ICache1 accessing L2-Cache due to bus1 accessing L1-ICache1."]
pub type L2_IBUS1_NXTLVL_CNT_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - The register records the number of times that L2-Cache accesses external memory due to L1-ICache1 accessing L2-Cache due to bus1 accessing L1-ICache1."]
    #[inline(always)]
    pub fn l2_ibus1_nxtlvl_cnt(&self) -> L2_IBUS1_NXTLVL_CNT_R {
        L2_IBUS1_NXTLVL_CNT_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L2_IBUS1_ACS_NXTLVL_CNT")
            .field(
                "l2_ibus1_nxtlvl_cnt",
                &format_args!("{}", self.l2_ibus1_nxtlvl_cnt().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<L2_IBUS1_ACS_NXTLVL_CNT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "L2-Cache bus1 Next-Level-Access Counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l2_ibus1_acs_nxtlvl_cnt::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L2_IBUS1_ACS_NXTLVL_CNT_SPEC;
impl crate::RegisterSpec for L2_IBUS1_ACS_NXTLVL_CNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l2_ibus1_acs_nxtlvl_cnt::R`](R) reader structure"]
impl crate::Readable for L2_IBUS1_ACS_NXTLVL_CNT_SPEC {}
#[doc = "`reset()` method sets L2_IBUS1_ACS_NXTLVL_CNT to value 0"]
impl crate::Resettable for L2_IBUS1_ACS_NXTLVL_CNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
