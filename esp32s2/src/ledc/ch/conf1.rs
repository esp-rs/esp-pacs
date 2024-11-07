#[doc = "Register `CONF1` reader"]
pub type R = crate::R<CONF1_SPEC>;
#[doc = "Register `CONF1` writer"]
pub type W = crate::W<CONF1_SPEC>;
#[doc = "Field `DUTY_SCALE` reader - This register is used to configure the changing step scale of duty on channel %s."]
pub type DUTY_SCALE_R = crate::FieldReader<u16>;
#[doc = "Field `DUTY_SCALE` writer - This register is used to configure the changing step scale of duty on channel %s."]
pub type DUTY_SCALE_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `DUTY_CYCLE` reader - The duty will change every LEDC_DUTY_CYCLE_CH%s on channel %s."]
pub type DUTY_CYCLE_R = crate::FieldReader<u16>;
#[doc = "Field `DUTY_CYCLE` writer - The duty will change every LEDC_DUTY_CYCLE_CH%s on channel %s."]
pub type DUTY_CYCLE_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `DUTY_NUM` reader - This register is used to control the number of times the duty cycle will be changed."]
pub type DUTY_NUM_R = crate::FieldReader<u16>;
#[doc = "Field `DUTY_NUM` writer - This register is used to control the number of times the duty cycle will be changed."]
pub type DUTY_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `DUTY_INC` reader - This register is used to increase or decrease the duty of output signal on channel %s. 1: Increase. 0: Decrease."]
pub type DUTY_INC_R = crate::BitReader;
#[doc = "Field `DUTY_INC` writer - This register is used to increase or decrease the duty of output signal on channel %s. 1: Increase. 0: Decrease."]
pub type DUTY_INC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DUTY_START` reader - Other configured fields in LEDC_CH%s_CONF1_REG will start to take effect when this bit is set to 1."]
pub type DUTY_START_R = crate::BitReader;
#[doc = "Field `DUTY_START` writer - Other configured fields in LEDC_CH%s_CONF1_REG will start to take effect when this bit is set to 1."]
pub type DUTY_START_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:9 - This register is used to configure the changing step scale of duty on channel %s."]
    #[inline(always)]
    pub fn duty_scale(&self) -> DUTY_SCALE_R {
        DUTY_SCALE_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:19 - The duty will change every LEDC_DUTY_CYCLE_CH%s on channel %s."]
    #[inline(always)]
    pub fn duty_cycle(&self) -> DUTY_CYCLE_R {
        DUTY_CYCLE_R::new(((self.bits >> 10) & 0x03ff) as u16)
    }
    #[doc = "Bits 20:29 - This register is used to control the number of times the duty cycle will be changed."]
    #[inline(always)]
    pub fn duty_num(&self) -> DUTY_NUM_R {
        DUTY_NUM_R::new(((self.bits >> 20) & 0x03ff) as u16)
    }
    #[doc = "Bit 30 - This register is used to increase or decrease the duty of output signal on channel %s. 1: Increase. 0: Decrease."]
    #[inline(always)]
    pub fn duty_inc(&self) -> DUTY_INC_R {
        DUTY_INC_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Other configured fields in LEDC_CH%s_CONF1_REG will start to take effect when this bit is set to 1."]
    #[inline(always)]
    pub fn duty_start(&self) -> DUTY_START_R {
        DUTY_START_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONF1")
            .field("duty_scale", &self.duty_scale())
            .field("duty_cycle", &self.duty_cycle())
            .field("duty_num", &self.duty_num())
            .field("duty_inc", &self.duty_inc())
            .field("duty_start", &self.duty_start())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:9 - This register is used to configure the changing step scale of duty on channel %s."]
    #[inline(always)]
    pub fn duty_scale(&mut self) -> DUTY_SCALE_W<CONF1_SPEC> {
        DUTY_SCALE_W::new(self, 0)
    }
    #[doc = "Bits 10:19 - The duty will change every LEDC_DUTY_CYCLE_CH%s on channel %s."]
    #[inline(always)]
    pub fn duty_cycle(&mut self) -> DUTY_CYCLE_W<CONF1_SPEC> {
        DUTY_CYCLE_W::new(self, 10)
    }
    #[doc = "Bits 20:29 - This register is used to control the number of times the duty cycle will be changed."]
    #[inline(always)]
    pub fn duty_num(&mut self) -> DUTY_NUM_W<CONF1_SPEC> {
        DUTY_NUM_W::new(self, 20)
    }
    #[doc = "Bit 30 - This register is used to increase or decrease the duty of output signal on channel %s. 1: Increase. 0: Decrease."]
    #[inline(always)]
    pub fn duty_inc(&mut self) -> DUTY_INC_W<CONF1_SPEC> {
        DUTY_INC_W::new(self, 30)
    }
    #[doc = "Bit 31 - Other configured fields in LEDC_CH%s_CONF1_REG will start to take effect when this bit is set to 1."]
    #[inline(always)]
    pub fn duty_start(&mut self) -> DUTY_START_W<CONF1_SPEC> {
        DUTY_START_W::new(self, 31)
    }
}
#[doc = "Configuration register 1 for channel 0\n\nYou can [`read`](crate::Reg::read) this register and get [`conf1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`conf1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CONF1_SPEC;
impl crate::RegisterSpec for CONF1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`conf1::R`](R) reader structure"]
impl crate::Readable for CONF1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`conf1::W`](W) writer structure"]
impl crate::Writable for CONF1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CONF1 to value 0x4000_0000"]
impl crate::Resettable for CONF1_SPEC {
    const RESET_VALUE: u32 = 0x4000_0000;
}
