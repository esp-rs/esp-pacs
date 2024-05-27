#[doc = "Register `TX_SIM` reader"]
pub type R = crate::R<TX_SIM_SPEC>;
#[doc = "Register `TX_SIM` writer"]
pub type W = crate::W<TX_SIM_SPEC>;
#[doc = "Field `CH0` reader - Set this bit to enable CHANNEL0 to start sending data synchronously with other enabled channels."]
pub type CH0_R = crate::BitReader;
#[doc = "Field `CH0` writer - Set this bit to enable CHANNEL0 to start sending data synchronously with other enabled channels."]
pub type CH0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1` reader - Set this bit to enable CHANNEL1 to start sending data synchronously with other enabled channels."]
pub type CH1_R = crate::BitReader;
#[doc = "Field `CH1` writer - Set this bit to enable CHANNEL1 to start sending data synchronously with other enabled channels."]
pub type CH1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2` reader - Set this bit to enable CHANNEL2 to start sending data synchronously with other enabled channels."]
pub type CH2_R = crate::BitReader;
#[doc = "Field `CH2` writer - Set this bit to enable CHANNEL2 to start sending data synchronously with other enabled channels."]
pub type CH2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH3` reader - Set this bit to enable CHANNEL3 to start sending data synchronously with other enabled channels."]
pub type CH3_R = crate::BitReader;
#[doc = "Field `CH3` writer - Set this bit to enable CHANNEL3 to start sending data synchronously with other enabled channels."]
pub type CH3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EN` reader - This register is used to enable multiple of channels to start sending data synchronously."]
pub type EN_R = crate::BitReader;
#[doc = "Field `EN` writer - This register is used to enable multiple of channels to start sending data synchronously."]
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Set this bit to enable CHANNEL0 to start sending data synchronously with other enabled channels."]
    #[inline(always)]
    pub fn ch0(&self) -> CH0_R {
        CH0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set this bit to enable CHANNEL1 to start sending data synchronously with other enabled channels."]
    #[inline(always)]
    pub fn ch1(&self) -> CH1_R {
        CH1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set this bit to enable CHANNEL2 to start sending data synchronously with other enabled channels."]
    #[inline(always)]
    pub fn ch2(&self) -> CH2_R {
        CH2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Set this bit to enable CHANNEL3 to start sending data synchronously with other enabled channels."]
    #[inline(always)]
    pub fn ch3(&self) -> CH3_R {
        CH3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - This register is used to enable multiple of channels to start sending data synchronously."]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TX_SIM")
            .field("ch0", &self.ch0())
            .field("ch1", &self.ch1())
            .field("ch2", &self.ch2())
            .field("ch3", &self.ch3())
            .field("en", &self.en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to enable CHANNEL0 to start sending data synchronously with other enabled channels."]
    #[inline(always)]
    #[must_use]
    pub fn ch0(&mut self) -> CH0_W<TX_SIM_SPEC> {
        CH0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Set this bit to enable CHANNEL1 to start sending data synchronously with other enabled channels."]
    #[inline(always)]
    #[must_use]
    pub fn ch1(&mut self) -> CH1_W<TX_SIM_SPEC> {
        CH1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Set this bit to enable CHANNEL2 to start sending data synchronously with other enabled channels."]
    #[inline(always)]
    #[must_use]
    pub fn ch2(&mut self) -> CH2_W<TX_SIM_SPEC> {
        CH2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Set this bit to enable CHANNEL3 to start sending data synchronously with other enabled channels."]
    #[inline(always)]
    #[must_use]
    pub fn ch3(&mut self) -> CH3_W<TX_SIM_SPEC> {
        CH3_W::new(self, 3)
    }
    #[doc = "Bit 4 - This register is used to enable multiple of channels to start sending data synchronously."]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<TX_SIM_SPEC> {
        EN_W::new(self, 4)
    }
}
#[doc = "RMT TX synchronous register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_sim::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tx_sim::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TX_SIM_SPEC;
impl crate::RegisterSpec for TX_SIM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_sim::R`](R) reader structure"]
impl crate::Readable for TX_SIM_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tx_sim::W`](W) writer structure"]
impl crate::Writable for TX_SIM_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TX_SIM to value 0"]
impl crate::Resettable for TX_SIM_SPEC {
    const RESET_VALUE: u32 = 0;
}
