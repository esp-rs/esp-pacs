///Register `REF_CNT_RST` reader
pub type R = crate::R<REF_CNT_RST_SPEC>;
///Register `REF_CNT_RST` writer
pub type W = crate::W<REF_CNT_RST_SPEC>;
///Field `CH0` reader - This register is used to reset the clock divider of CHANNEL0.
pub type CH0_R = crate::BitReader;
///Field `CH0` writer - This register is used to reset the clock divider of CHANNEL0.
pub type CH0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CH1` reader - This register is used to reset the clock divider of CHANNEL1.
pub type CH1_R = crate::BitReader;
///Field `CH1` writer - This register is used to reset the clock divider of CHANNEL1.
pub type CH1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CH2` reader - This register is used to reset the clock divider of CHANNEL2.
pub type CH2_R = crate::BitReader;
///Field `CH2` writer - This register is used to reset the clock divider of CHANNEL2.
pub type CH2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CH3` reader - This register is used to reset the clock divider of CHANNEL3.
pub type CH3_R = crate::BitReader;
///Field `CH3` writer - This register is used to reset the clock divider of CHANNEL3.
pub type CH3_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - This register is used to reset the clock divider of CHANNEL0.
    #[inline(always)]
    pub fn ch0(&self) -> CH0_R {
        CH0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - This register is used to reset the clock divider of CHANNEL1.
    #[inline(always)]
    pub fn ch1(&self) -> CH1_R {
        CH1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - This register is used to reset the clock divider of CHANNEL2.
    #[inline(always)]
    pub fn ch2(&self) -> CH2_R {
        CH2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - This register is used to reset the clock divider of CHANNEL3.
    #[inline(always)]
    pub fn ch3(&self) -> CH3_R {
        CH3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REF_CNT_RST")
            .field("ch0", &self.ch0())
            .field("ch1", &self.ch1())
            .field("ch2", &self.ch2())
            .field("ch3", &self.ch3())
            .finish()
    }
}
impl W {
    ///Bit 0 - This register is used to reset the clock divider of CHANNEL0.
    #[inline(always)]
    #[must_use]
    pub fn ch0(&mut self) -> CH0_W<REF_CNT_RST_SPEC> {
        CH0_W::new(self, 0)
    }
    ///Bit 1 - This register is used to reset the clock divider of CHANNEL1.
    #[inline(always)]
    #[must_use]
    pub fn ch1(&mut self) -> CH1_W<REF_CNT_RST_SPEC> {
        CH1_W::new(self, 1)
    }
    ///Bit 2 - This register is used to reset the clock divider of CHANNEL2.
    #[inline(always)]
    #[must_use]
    pub fn ch2(&mut self) -> CH2_W<REF_CNT_RST_SPEC> {
        CH2_W::new(self, 2)
    }
    ///Bit 3 - This register is used to reset the clock divider of CHANNEL3.
    #[inline(always)]
    #[must_use]
    pub fn ch3(&mut self) -> CH3_W<REF_CNT_RST_SPEC> {
        CH3_W::new(self, 3)
    }
}
/**RMT clock divider reset register

You can [`read`](crate::generic::Reg::read) this register and get [`ref_cnt_rst::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ref_cnt_rst::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct REF_CNT_RST_SPEC;
impl crate::RegisterSpec for REF_CNT_RST_SPEC {
    type Ux = u32;
}
///`read()` method returns [`ref_cnt_rst::R`](R) reader structure
impl crate::Readable for REF_CNT_RST_SPEC {}
///`write(|w| ..)` method takes [`ref_cnt_rst::W`](W) writer structure
impl crate::Writable for REF_CNT_RST_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets REF_CNT_RST to value 0
impl crate::Resettable for REF_CNT_RST_SPEC {
    const RESET_VALUE: u32 = 0;
}
