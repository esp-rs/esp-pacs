///Register `DIN_NUM` reader
pub type R = crate::R<DIN_NUM_SPEC>;
///Register `DIN_NUM` writer
pub type W = crate::W<DIN_NUM_SPEC>;
///Field `DIN0_NUM` reader - the input signals are delayed by SPI module clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state.
pub type DIN0_NUM_R = crate::FieldReader;
///Field `DIN0_NUM` writer - the input signals are delayed by SPI module clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state.
pub type DIN0_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `DIN1_NUM` reader - the input signals are delayed by SPI module clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state.
pub type DIN1_NUM_R = crate::FieldReader;
///Field `DIN1_NUM` writer - the input signals are delayed by SPI module clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state.
pub type DIN1_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `DIN2_NUM` reader - the input signals are delayed by SPI module clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state.
pub type DIN2_NUM_R = crate::FieldReader;
///Field `DIN2_NUM` writer - the input signals are delayed by SPI module clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state.
pub type DIN2_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `DIN3_NUM` reader - the input signals are delayed by SPI module clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state.
pub type DIN3_NUM_R = crate::FieldReader;
///Field `DIN3_NUM` writer - the input signals are delayed by SPI module clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state.
pub type DIN3_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:1 - the input signals are delayed by SPI module clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state.
    #[inline(always)]
    pub fn din0_num(&self) -> DIN0_NUM_R {
        DIN0_NUM_R::new((self.bits & 3) as u8)
    }
    ///Bits 2:3 - the input signals are delayed by SPI module clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state.
    #[inline(always)]
    pub fn din1_num(&self) -> DIN1_NUM_R {
        DIN1_NUM_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 4:5 - the input signals are delayed by SPI module clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state.
    #[inline(always)]
    pub fn din2_num(&self) -> DIN2_NUM_R {
        DIN2_NUM_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bits 6:7 - the input signals are delayed by SPI module clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state.
    #[inline(always)]
    pub fn din3_num(&self) -> DIN3_NUM_R {
        DIN3_NUM_R::new(((self.bits >> 6) & 3) as u8)
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
impl W {
    ///Bits 0:1 - the input signals are delayed by SPI module clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state.
    #[inline(always)]
    #[must_use]
    pub fn din0_num(&mut self) -> DIN0_NUM_W<DIN_NUM_SPEC> {
        DIN0_NUM_W::new(self, 0)
    }
    ///Bits 2:3 - the input signals are delayed by SPI module clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state.
    #[inline(always)]
    #[must_use]
    pub fn din1_num(&mut self) -> DIN1_NUM_W<DIN_NUM_SPEC> {
        DIN1_NUM_W::new(self, 2)
    }
    ///Bits 4:5 - the input signals are delayed by SPI module clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state.
    #[inline(always)]
    #[must_use]
    pub fn din2_num(&mut self) -> DIN2_NUM_W<DIN_NUM_SPEC> {
        DIN2_NUM_W::new(self, 4)
    }
    ///Bits 6:7 - the input signals are delayed by SPI module clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state.
    #[inline(always)]
    #[must_use]
    pub fn din3_num(&mut self) -> DIN3_NUM_W<DIN_NUM_SPEC> {
        DIN3_NUM_W::new(self, 6)
    }
}
/**SPI input delay number configuration

You can [`read`](crate::generic::Reg::read) this register and get [`din_num::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`din_num::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DIN_NUM_SPEC;
impl crate::RegisterSpec for DIN_NUM_SPEC {
    type Ux = u32;
}
///`read()` method returns [`din_num::R`](R) reader structure
impl crate::Readable for DIN_NUM_SPEC {}
///`write(|w| ..)` method takes [`din_num::W`](W) writer structure
impl crate::Writable for DIN_NUM_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DIN_NUM to value 0
impl crate::Resettable for DIN_NUM_SPEC {
    const RESET_VALUE: u32 = 0;
}
