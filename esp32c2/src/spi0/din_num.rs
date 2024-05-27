///Register `DIN_NUM` reader
pub type R = crate::R<DIN_NUM_SPEC>;
///Field `DIN0_NUM` reader - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,...
pub type DIN0_NUM_R = crate::BitReader;
///Field `DIN1_NUM` reader - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,...
pub type DIN1_NUM_R = crate::BitReader;
///Field `DIN2_NUM` reader - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,...
pub type DIN2_NUM_R = crate::BitReader;
///Field `DIN3_NUM` reader - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,...
pub type DIN3_NUM_R = crate::BitReader;
impl R {
    ///Bit 0 - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,...
    #[inline(always)]
    pub fn din0_num(&self) -> DIN0_NUM_R {
        DIN0_NUM_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,...
    #[inline(always)]
    pub fn din1_num(&self) -> DIN1_NUM_R {
        DIN1_NUM_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,...
    #[inline(always)]
    pub fn din2_num(&self) -> DIN2_NUM_R {
        DIN2_NUM_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,...
    #[inline(always)]
    pub fn din3_num(&self) -> DIN3_NUM_R {
        DIN3_NUM_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIN_NUM")
            .field("din0_num", &self.din0_num())
            .field("din1_num", &self.din1_num())
            .field("din2_num", &self.din2_num())
            .field("din3_num", &self.din3_num())
            .finish()
    }
}
/**SPI0 input delay number control register

You can [`read`](crate::generic::Reg::read) this register and get [`din_num::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DIN_NUM_SPEC;
impl crate::RegisterSpec for DIN_NUM_SPEC {
    type Ux = u32;
}
///`read()` method returns [`din_num::R`](R) reader structure
impl crate::Readable for DIN_NUM_SPEC {}
///`reset()` method sets DIN_NUM to value 0
impl crate::Resettable for DIN_NUM_SPEC {
    const RESET_VALUE: u32 = 0;
}
