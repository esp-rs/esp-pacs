#[doc = "Register `DIN_NUM` reader"]
pub struct R(crate::R<DIN_NUM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIN_NUM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIN_NUM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIN_NUM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DIN0_NUM` reader - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
pub type DIN0_NUM_R = crate::BitReader<bool>;
#[doc = "Field `DIN1_NUM` reader - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
pub type DIN1_NUM_R = crate::BitReader<bool>;
#[doc = "Field `DIN2_NUM` reader - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
pub type DIN2_NUM_R = crate::BitReader<bool>;
#[doc = "Field `DIN3_NUM` reader - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
pub type DIN3_NUM_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
    #[inline(always)]
    pub fn din0_num(&self) -> DIN0_NUM_R {
        DIN0_NUM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
    #[inline(always)]
    pub fn din1_num(&self) -> DIN1_NUM_R {
        DIN1_NUM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
    #[inline(always)]
    pub fn din2_num(&self) -> DIN2_NUM_R {
        DIN2_NUM_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
    #[inline(always)]
    pub fn din3_num(&self) -> DIN3_NUM_R {
        DIN3_NUM_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "SPI0 input delay number control register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [din_num](index.html) module"]
pub struct DIN_NUM_SPEC;
impl crate::RegisterSpec for DIN_NUM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [din_num::R](R) reader structure"]
impl crate::Readable for DIN_NUM_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DIN_NUM to value 0"]
impl crate::Resettable for DIN_NUM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
