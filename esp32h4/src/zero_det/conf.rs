#[doc = "Register `CONF` reader"]
pub type R = crate::R<CONF_SPEC>;
#[doc = "Register `CONF` writer"]
pub type W = crate::W<CONF_SPEC>;
#[doc = "Field `VREF_CHANNEL_SEL` reader - vref pad sel,one hot code and cannot set the same bit with other channels"]
pub type VREF_CHANNEL_SEL_R = crate::FieldReader;
#[doc = "Field `VREF_CHANNEL_SEL` writer - vref pad sel,one hot code and cannot set the same bit with other channels"]
pub type VREF_CHANNEL_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `COMP_CHANNEL_3_SEL` reader - Channel 3 comp pad sel,one hot code and cannot set the same bit with other channels"]
pub type COMP_CHANNEL_3_SEL_R = crate::FieldReader;
#[doc = "Field `COMP_CHANNEL_3_SEL` writer - Channel 3 comp pad sel,one hot code and cannot set the same bit with other channels"]
pub type COMP_CHANNEL_3_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `COMP_CHANNEL_2_SEL` reader - Channel 2 comp pad sel,one hot code and cannot set the same bit with other channels"]
pub type COMP_CHANNEL_2_SEL_R = crate::FieldReader;
#[doc = "Field `COMP_CHANNEL_2_SEL` writer - Channel 2 comp pad sel,one hot code and cannot set the same bit with other channels"]
pub type COMP_CHANNEL_2_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `COMP_CHANNEL_1_SEL` reader - Channel 1 comp pad sel,one hot code and cannot set the same bit with other channels"]
pub type COMP_CHANNEL_1_SEL_R = crate::FieldReader;
#[doc = "Field `COMP_CHANNEL_1_SEL` writer - Channel 1 comp pad sel,one hot code and cannot set the same bit with other channels"]
pub type COMP_CHANNEL_1_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CHANNEL_3_EVENT_TIMER_EN` reader - enable channel 3 event timer to trigger channel 1 event after pad_comp_channel_3_int"]
pub type CHANNEL_3_EVENT_TIMER_EN_R = crate::BitReader;
#[doc = "Field `CHANNEL_3_EVENT_TIMER_EN` writer - enable channel 3 event timer to trigger channel 1 event after pad_comp_channel_3_int"]
pub type CHANNEL_3_EVENT_TIMER_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHANNEL_2_EVENT_TIMER_EN` reader - enable channel 2 event timer to trigger channel 1 event after pad_comp_channel_2_int"]
pub type CHANNEL_2_EVENT_TIMER_EN_R = crate::BitReader;
#[doc = "Field `CHANNEL_2_EVENT_TIMER_EN` writer - enable channel 2 event timer to trigger channel 1 event after pad_comp_channel_2_int"]
pub type CHANNEL_2_EVENT_TIMER_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHANNEL_1_EVENT_TIMER_EN` reader - enable channel 1 event timer to trigger channel 1 event after pad_comp_channel_1_int"]
pub type CHANNEL_1_EVENT_TIMER_EN_R = crate::BitReader;
#[doc = "Field `CHANNEL_1_EVENT_TIMER_EN` writer - enable channel 1 event timer to trigger channel 1 event after pad_comp_channel_1_int"]
pub type CHANNEL_1_EVENT_TIMER_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHANNEL_TIMER_EN` reader - enable timer to record the time between two continuous zero det int in each channel"]
pub type CHANNEL_TIMER_EN_R = crate::BitReader;
#[doc = "Field `CHANNEL_TIMER_EN` writer - enable timer to record the time between two continuous zero det int in each channel"]
pub type CHANNEL_TIMER_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LIMIT_CNT` reader - cfg continuous zero det num to change zero det result"]
pub type LIMIT_CNT_R = crate::FieldReader;
#[doc = "Field `LIMIT_CNT` writer - cfg continuous zero det num to change zero det result"]
pub type LIMIT_CNT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `COMP_POLL_MASK` reader - mask channel to do pad compare and zero det"]
pub type COMP_POLL_MASK_R = crate::FieldReader;
#[doc = "Field `COMP_POLL_MASK` writer - mask channel to do pad compare and zero det"]
pub type COMP_POLL_MASK_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `COMP_POLL_MODE` reader - cfg channel scan mode ,0 means one trigger scan all mask channel, 1 means one trigger scan one mask channel"]
pub type COMP_POLL_MODE_R = crate::BitReader;
#[doc = "Field `COMP_POLL_MODE` writer - cfg channel scan mode ,0 means one trigger scan all mask channel, 1 means one trigger scan one mask channel"]
pub type COMP_POLL_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - vref pad sel,one hot code and cannot set the same bit with other channels"]
    #[inline(always)]
    pub fn vref_channel_sel(&self) -> VREF_CHANNEL_SEL_R {
        VREF_CHANNEL_SEL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Channel 3 comp pad sel,one hot code and cannot set the same bit with other channels"]
    #[inline(always)]
    pub fn comp_channel_3_sel(&self) -> COMP_CHANNEL_3_SEL_R {
        COMP_CHANNEL_3_SEL_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Channel 2 comp pad sel,one hot code and cannot set the same bit with other channels"]
    #[inline(always)]
    pub fn comp_channel_2_sel(&self) -> COMP_CHANNEL_2_SEL_R {
        COMP_CHANNEL_2_SEL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Channel 1 comp pad sel,one hot code and cannot set the same bit with other channels"]
    #[inline(always)]
    pub fn comp_channel_1_sel(&self) -> COMP_CHANNEL_1_SEL_R {
        COMP_CHANNEL_1_SEL_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - enable channel 3 event timer to trigger channel 1 event after pad_comp_channel_3_int"]
    #[inline(always)]
    pub fn channel_3_event_timer_en(&self) -> CHANNEL_3_EVENT_TIMER_EN_R {
        CHANNEL_3_EVENT_TIMER_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - enable channel 2 event timer to trigger channel 1 event after pad_comp_channel_2_int"]
    #[inline(always)]
    pub fn channel_2_event_timer_en(&self) -> CHANNEL_2_EVENT_TIMER_EN_R {
        CHANNEL_2_EVENT_TIMER_EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - enable channel 1 event timer to trigger channel 1 event after pad_comp_channel_1_int"]
    #[inline(always)]
    pub fn channel_1_event_timer_en(&self) -> CHANNEL_1_EVENT_TIMER_EN_R {
        CHANNEL_1_EVENT_TIMER_EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - enable timer to record the time between two continuous zero det int in each channel"]
    #[inline(always)]
    pub fn channel_timer_en(&self) -> CHANNEL_TIMER_EN_R {
        CHANNEL_TIMER_EN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:27 - cfg continuous zero det num to change zero det result"]
    #[inline(always)]
    pub fn limit_cnt(&self) -> LIMIT_CNT_R {
        LIMIT_CNT_R::new(((self.bits >> 20) & 0xff) as u8)
    }
    #[doc = "Bits 28:30 - mask channel to do pad compare and zero det"]
    #[inline(always)]
    pub fn comp_poll_mask(&self) -> COMP_POLL_MASK_R {
        COMP_POLL_MASK_R::new(((self.bits >> 28) & 7) as u8)
    }
    #[doc = "Bit 31 - cfg channel scan mode ,0 means one trigger scan all mask channel, 1 means one trigger scan one mask channel"]
    #[inline(always)]
    pub fn comp_poll_mode(&self) -> COMP_POLL_MODE_R {
        COMP_POLL_MODE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONF")
            .field("vref_channel_sel", &self.vref_channel_sel())
            .field("comp_channel_3_sel", &self.comp_channel_3_sel())
            .field("comp_channel_2_sel", &self.comp_channel_2_sel())
            .field("comp_channel_1_sel", &self.comp_channel_1_sel())
            .field("channel_3_event_timer_en", &self.channel_3_event_timer_en())
            .field("channel_2_event_timer_en", &self.channel_2_event_timer_en())
            .field("channel_1_event_timer_en", &self.channel_1_event_timer_en())
            .field("channel_timer_en", &self.channel_timer_en())
            .field("limit_cnt", &self.limit_cnt())
            .field("comp_poll_mask", &self.comp_poll_mask())
            .field("comp_poll_mode", &self.comp_poll_mode())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - vref pad sel,one hot code and cannot set the same bit with other channels"]
    #[inline(always)]
    pub fn vref_channel_sel(&mut self) -> VREF_CHANNEL_SEL_W<'_, CONF_SPEC> {
        VREF_CHANNEL_SEL_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - Channel 3 comp pad sel,one hot code and cannot set the same bit with other channels"]
    #[inline(always)]
    pub fn comp_channel_3_sel(&mut self) -> COMP_CHANNEL_3_SEL_W<'_, CONF_SPEC> {
        COMP_CHANNEL_3_SEL_W::new(self, 4)
    }
    #[doc = "Bits 8:11 - Channel 2 comp pad sel,one hot code and cannot set the same bit with other channels"]
    #[inline(always)]
    pub fn comp_channel_2_sel(&mut self) -> COMP_CHANNEL_2_SEL_W<'_, CONF_SPEC> {
        COMP_CHANNEL_2_SEL_W::new(self, 8)
    }
    #[doc = "Bits 12:15 - Channel 1 comp pad sel,one hot code and cannot set the same bit with other channels"]
    #[inline(always)]
    pub fn comp_channel_1_sel(&mut self) -> COMP_CHANNEL_1_SEL_W<'_, CONF_SPEC> {
        COMP_CHANNEL_1_SEL_W::new(self, 12)
    }
    #[doc = "Bit 16 - enable channel 3 event timer to trigger channel 1 event after pad_comp_channel_3_int"]
    #[inline(always)]
    pub fn channel_3_event_timer_en(&mut self) -> CHANNEL_3_EVENT_TIMER_EN_W<'_, CONF_SPEC> {
        CHANNEL_3_EVENT_TIMER_EN_W::new(self, 16)
    }
    #[doc = "Bit 17 - enable channel 2 event timer to trigger channel 1 event after pad_comp_channel_2_int"]
    #[inline(always)]
    pub fn channel_2_event_timer_en(&mut self) -> CHANNEL_2_EVENT_TIMER_EN_W<'_, CONF_SPEC> {
        CHANNEL_2_EVENT_TIMER_EN_W::new(self, 17)
    }
    #[doc = "Bit 18 - enable channel 1 event timer to trigger channel 1 event after pad_comp_channel_1_int"]
    #[inline(always)]
    pub fn channel_1_event_timer_en(&mut self) -> CHANNEL_1_EVENT_TIMER_EN_W<'_, CONF_SPEC> {
        CHANNEL_1_EVENT_TIMER_EN_W::new(self, 18)
    }
    #[doc = "Bit 19 - enable timer to record the time between two continuous zero det int in each channel"]
    #[inline(always)]
    pub fn channel_timer_en(&mut self) -> CHANNEL_TIMER_EN_W<'_, CONF_SPEC> {
        CHANNEL_TIMER_EN_W::new(self, 19)
    }
    #[doc = "Bits 20:27 - cfg continuous zero det num to change zero det result"]
    #[inline(always)]
    pub fn limit_cnt(&mut self) -> LIMIT_CNT_W<'_, CONF_SPEC> {
        LIMIT_CNT_W::new(self, 20)
    }
    #[doc = "Bits 28:30 - mask channel to do pad compare and zero det"]
    #[inline(always)]
    pub fn comp_poll_mask(&mut self) -> COMP_POLL_MASK_W<'_, CONF_SPEC> {
        COMP_POLL_MASK_W::new(self, 28)
    }
    #[doc = "Bit 31 - cfg channel scan mode ,0 means one trigger scan all mask channel, 1 means one trigger scan one mask channel"]
    #[inline(always)]
    pub fn comp_poll_mode(&mut self) -> COMP_POLL_MODE_W<'_, CONF_SPEC> {
        COMP_POLL_MODE_W::new(self, 31)
    }
}
#[doc = "zero det cfg reg\n\nYou can [`read`](crate::Reg::read) this register and get [`conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CONF_SPEC;
impl crate::RegisterSpec for CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`conf::R`](R) reader structure"]
impl crate::Readable for CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`conf::W`](W) writer structure"]
impl crate::Writable for CONF_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CONF to value 0x0050_1248"]
impl crate::Resettable for CONF_SPEC {
    const RESET_VALUE: u32 = 0x0050_1248;
}
