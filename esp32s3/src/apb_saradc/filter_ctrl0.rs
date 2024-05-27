///Register `FILTER_CTRL0` reader
pub type R = crate::R<FILTER_CTRL0_SPEC>;
///Register `FILTER_CTRL0` writer
pub type W = crate::W<FILTER_CTRL0_SPEC>;
///Field `FILTER_CHANNEL1` reader - configure the filter1 channel
pub type FILTER_CHANNEL1_R = crate::FieldReader;
///Field `FILTER_CHANNEL1` writer - configure the filter1 channel
pub type FILTER_CHANNEL1_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `FILTER_CHANNEL0` reader - configure the filter0 channel
pub type FILTER_CHANNEL0_R = crate::FieldReader;
///Field `FILTER_CHANNEL0` writer - configure the filter0 channel
pub type FILTER_CHANNEL0_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `FILTER_RESET` reader - enable apb_adc1_filter
pub type FILTER_RESET_R = crate::BitReader;
///Field `FILTER_RESET` writer - enable apb_adc1_filter
pub type FILTER_RESET_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 14:18 - configure the filter1 channel
    #[inline(always)]
    pub fn filter_channel1(&self) -> FILTER_CHANNEL1_R {
        FILTER_CHANNEL1_R::new(((self.bits >> 14) & 0x1f) as u8)
    }
    ///Bits 19:23 - configure the filter0 channel
    #[inline(always)]
    pub fn filter_channel0(&self) -> FILTER_CHANNEL0_R {
        FILTER_CHANNEL0_R::new(((self.bits >> 19) & 0x1f) as u8)
    }
    ///Bit 31 - enable apb_adc1_filter
    #[inline(always)]
    pub fn filter_reset(&self) -> FILTER_RESET_R {
        FILTER_RESET_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FILTER_CTRL0")
            .field("filter_channel1", &self.filter_channel1())
            .field("filter_channel0", &self.filter_channel0())
            .field("filter_reset", &self.filter_reset())
            .finish()
    }
}
impl W {
    ///Bits 14:18 - configure the filter1 channel
    #[inline(always)]
    #[must_use]
    pub fn filter_channel1(&mut self) -> FILTER_CHANNEL1_W<FILTER_CTRL0_SPEC> {
        FILTER_CHANNEL1_W::new(self, 14)
    }
    ///Bits 19:23 - configure the filter0 channel
    #[inline(always)]
    #[must_use]
    pub fn filter_channel0(&mut self) -> FILTER_CHANNEL0_W<FILTER_CTRL0_SPEC> {
        FILTER_CHANNEL0_W::new(self, 19)
    }
    ///Bit 31 - enable apb_adc1_filter
    #[inline(always)]
    #[must_use]
    pub fn filter_reset(&mut self) -> FILTER_RESET_W<FILTER_CTRL0_SPEC> {
        FILTER_RESET_W::new(self, 31)
    }
}
/**configure apb saradc arbit

You can [`read`](crate::generic::Reg::read) this register and get [`filter_ctrl0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`filter_ctrl0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct FILTER_CTRL0_SPEC;
impl crate::RegisterSpec for FILTER_CTRL0_SPEC {
    type Ux = u32;
}
///`read()` method returns [`filter_ctrl0::R`](R) reader structure
impl crate::Readable for FILTER_CTRL0_SPEC {}
///`write(|w| ..)` method takes [`filter_ctrl0::W`](W) writer structure
impl crate::Writable for FILTER_CTRL0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets FILTER_CTRL0 to value 0x006b_4000
impl crate::Resettable for FILTER_CTRL0_SPEC {
    const RESET_VALUE: u32 = 0x006b_4000;
}
