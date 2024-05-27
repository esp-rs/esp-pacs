///Register `CAP_CH%s_CFG` reader
pub type R = crate::R<CAP_CH_CFG_SPEC>;
///Register `CAP_CH%s_CFG` writer
pub type W = crate::W<CAP_CH_CFG_SPEC>;
///Field `EN` reader - When set, capture on channel 0 is enabled
pub type EN_R = crate::BitReader;
///Field `EN` writer - When set, capture on channel 0 is enabled
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MODE` reader - Edge of capture on channel 0 after prescaling. When bit0 is set to 1: enable capture on the negative edge, When bit1 is set to 1: enable capture on the positive edge.
pub type MODE_R = crate::FieldReader;
///Field `MODE` writer - Edge of capture on channel 0 after prescaling. When bit0 is set to 1: enable capture on the negative edge, When bit1 is set to 1: enable capture on the positive edge.
pub type MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `PRESCALE` reader - Value of prescaling on possitive edge of CAP0. Prescale value = PWM_CAP0_PRESCALE + 1
pub type PRESCALE_R = crate::FieldReader;
///Field `PRESCALE` writer - Value of prescaling on possitive edge of CAP0. Prescale value = PWM_CAP0_PRESCALE + 1
pub type PRESCALE_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `IN_INVERT` reader - when set, CAP0 form GPIO matrix is inverted before prescale
pub type IN_INVERT_R = crate::BitReader;
///Field `IN_INVERT` writer - when set, CAP0 form GPIO matrix is inverted before prescale
pub type IN_INVERT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SW` writer - Write 1 will trigger a software forced capture on channel 0
pub type SW_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - When set, capture on channel 0 is enabled
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    ///Bits 1:2 - Edge of capture on channel 0 after prescaling. When bit0 is set to 1: enable capture on the negative edge, When bit1 is set to 1: enable capture on the positive edge.
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 1) & 3) as u8)
    }
    ///Bits 3:10 - Value of prescaling on possitive edge of CAP0. Prescale value = PWM_CAP0_PRESCALE + 1
    #[inline(always)]
    pub fn prescale(&self) -> PRESCALE_R {
        PRESCALE_R::new(((self.bits >> 3) & 0xff) as u8)
    }
    ///Bit 11 - when set, CAP0 form GPIO matrix is inverted before prescale
    #[inline(always)]
    pub fn in_invert(&self) -> IN_INVERT_R {
        IN_INVERT_R::new(((self.bits >> 11) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CAP_CH_CFG")
            .field("en", &self.en())
            .field("mode", &self.mode())
            .field("prescale", &self.prescale())
            .field("in_invert", &self.in_invert())
            .finish()
    }
}
impl W {
    ///Bit 0 - When set, capture on channel 0 is enabled
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<CAP_CH_CFG_SPEC> {
        EN_W::new(self, 0)
    }
    ///Bits 1:2 - Edge of capture on channel 0 after prescaling. When bit0 is set to 1: enable capture on the negative edge, When bit1 is set to 1: enable capture on the positive edge.
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<CAP_CH_CFG_SPEC> {
        MODE_W::new(self, 1)
    }
    ///Bits 3:10 - Value of prescaling on possitive edge of CAP0. Prescale value = PWM_CAP0_PRESCALE + 1
    #[inline(always)]
    #[must_use]
    pub fn prescale(&mut self) -> PRESCALE_W<CAP_CH_CFG_SPEC> {
        PRESCALE_W::new(self, 3)
    }
    ///Bit 11 - when set, CAP0 form GPIO matrix is inverted before prescale
    #[inline(always)]
    #[must_use]
    pub fn in_invert(&mut self) -> IN_INVERT_W<CAP_CH_CFG_SPEC> {
        IN_INVERT_W::new(self, 11)
    }
    ///Bit 12 - Write 1 will trigger a software forced capture on channel 0
    #[inline(always)]
    #[must_use]
    pub fn sw(&mut self) -> SW_W<CAP_CH_CFG_SPEC> {
        SW_W::new(self, 12)
    }
}
/**Capture channel %s configuration and enable

You can [`read`](crate::generic::Reg::read) this register and get [`cap_ch_cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cap_ch_cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CAP_CH_CFG_SPEC;
impl crate::RegisterSpec for CAP_CH_CFG_SPEC {
    type Ux = u32;
}
///`read()` method returns [`cap_ch_cfg::R`](R) reader structure
impl crate::Readable for CAP_CH_CFG_SPEC {}
///`write(|w| ..)` method takes [`cap_ch_cfg::W`](W) writer structure
impl crate::Writable for CAP_CH_CFG_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CAP_CH%s_CFG to value 0
impl crate::Resettable for CAP_CH_CFG_SPEC {
    const RESET_VALUE: u32 = 0;
}
