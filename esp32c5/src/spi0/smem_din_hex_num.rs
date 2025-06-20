#[doc = "Register `SMEM_DIN_HEX_NUM` reader"]
pub type R = crate::R<SMEM_DIN_HEX_NUM_SPEC>;
#[doc = "Field `SMEM_DIN08_NUM` reader - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
pub type SMEM_DIN08_NUM_R = crate::FieldReader;
#[doc = "Field `SMEM_DIN09_NUM` reader - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
pub type SMEM_DIN09_NUM_R = crate::FieldReader;
#[doc = "Field `SMEM_DIN10_NUM` reader - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
pub type SMEM_DIN10_NUM_R = crate::FieldReader;
#[doc = "Field `SMEM_DIN11_NUM` reader - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
pub type SMEM_DIN11_NUM_R = crate::FieldReader;
#[doc = "Field `SMEM_DIN12_NUM` reader - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
pub type SMEM_DIN12_NUM_R = crate::FieldReader;
#[doc = "Field `SMEM_DIN13_NUM` reader - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
pub type SMEM_DIN13_NUM_R = crate::FieldReader;
#[doc = "Field `SMEM_DIN14_NUM` reader - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
pub type SMEM_DIN14_NUM_R = crate::FieldReader;
#[doc = "Field `SMEM_DIN15_NUM` reader - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
pub type SMEM_DIN15_NUM_R = crate::FieldReader;
#[doc = "Field `SMEM_DINS_HEX_NUM` reader - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
pub type SMEM_DINS_HEX_NUM_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:1 - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
    #[inline(always)]
    pub fn smem_din08_num(&self) -> SMEM_DIN08_NUM_R {
        SMEM_DIN08_NUM_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
    #[inline(always)]
    pub fn smem_din09_num(&self) -> SMEM_DIN09_NUM_R {
        SMEM_DIN09_NUM_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
    #[inline(always)]
    pub fn smem_din10_num(&self) -> SMEM_DIN10_NUM_R {
        SMEM_DIN10_NUM_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
    #[inline(always)]
    pub fn smem_din11_num(&self) -> SMEM_DIN11_NUM_R {
        SMEM_DIN11_NUM_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
    #[inline(always)]
    pub fn smem_din12_num(&self) -> SMEM_DIN12_NUM_R {
        SMEM_DIN12_NUM_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
    #[inline(always)]
    pub fn smem_din13_num(&self) -> SMEM_DIN13_NUM_R {
        SMEM_DIN13_NUM_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
    #[inline(always)]
    pub fn smem_din14_num(&self) -> SMEM_DIN14_NUM_R {
        SMEM_DIN14_NUM_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
    #[inline(always)]
    pub fn smem_din15_num(&self) -> SMEM_DIN15_NUM_R {
        SMEM_DIN15_NUM_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
    #[inline(always)]
    pub fn smem_dins_hex_num(&self) -> SMEM_DINS_HEX_NUM_R {
        SMEM_DINS_HEX_NUM_R::new(((self.bits >> 16) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SMEM_DIN_HEX_NUM")
            .field("smem_din08_num", &self.smem_din08_num())
            .field("smem_din09_num", &self.smem_din09_num())
            .field("smem_din10_num", &self.smem_din10_num())
            .field("smem_din11_num", &self.smem_din11_num())
            .field("smem_din12_num", &self.smem_din12_num())
            .field("smem_din13_num", &self.smem_din13_num())
            .field("smem_din14_num", &self.smem_din14_num())
            .field("smem_din15_num", &self.smem_din15_num())
            .field("smem_dins_hex_num", &self.smem_dins_hex_num())
            .finish()
    }
}
#[doc = "MSPI 16x external RAM input timing delay number control register\n\nYou can [`read`](crate::Reg::read) this register and get [`smem_din_hex_num::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SMEM_DIN_HEX_NUM_SPEC;
impl crate::RegisterSpec for SMEM_DIN_HEX_NUM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`smem_din_hex_num::R`](R) reader structure"]
impl crate::Readable for SMEM_DIN_HEX_NUM_SPEC {}
#[doc = "`reset()` method sets SMEM_DIN_HEX_NUM to value 0"]
impl crate::Resettable for SMEM_DIN_HEX_NUM_SPEC {
    const RESET_VALUE: u32 = 0;
}
