#[doc = "Register `CALI` reader"]
pub type R = crate::R<CALI_SPEC>;
#[doc = "Field `OUT_PAD_0` reader - cali out from DAC PAD 0"]
pub type OUT_PAD_0_R = crate::BitReader;
#[doc = "Field `OUT_PAD_1` reader - cali out from DAC PAD 1"]
pub type OUT_PAD_1_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - cali out from DAC PAD 0"]
    #[inline(always)]
    pub fn out_pad_0(&self) -> OUT_PAD_0_R {
        OUT_PAD_0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - cali out from DAC PAD 1"]
    #[inline(always)]
    pub fn out_pad_1(&self) -> OUT_PAD_1_R {
        OUT_PAD_1_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CALI")
            .field("out_pad_0", &self.out_pad_0())
            .field("out_pad_1", &self.out_pad_1())
            .finish()
    }
}
#[doc = "cali algorithm register for DAC\n\nYou can [`read`](crate::Reg::read) this register and get [`cali::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CALI_SPEC;
impl crate::RegisterSpec for CALI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cali::R`](R) reader structure"]
impl crate::Readable for CALI_SPEC {}
#[doc = "`reset()` method sets CALI to value 0"]
impl crate::Resettable for CALI_SPEC {}
