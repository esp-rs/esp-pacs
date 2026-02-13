#[doc = "Register `ENTRY%s` reader"]
pub type R = crate::R<ENTRY_SPEC>;
#[doc = "Register `ENTRY%s` writer"]
pub type W = crate::W<ENTRY_SPEC>;
#[doc = "Field `DUTY_INC` reader - Configures whether to increase duty in this fade range."]
pub type DUTY_INC_R = crate::BitReader;
#[doc = "Field `DUTY_INC` writer - Configures whether to increase duty in this fade range."]
pub type DUTY_INC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DUTY_CYCLE` reader - Configures PWM overflows between duty updates in this fade range."]
pub type DUTY_CYCLE_R = crate::FieldReader<u16>;
#[doc = "Field `DUTY_CYCLE` writer - Configures PWM overflows between duty updates in this fade range."]
pub type DUTY_CYCLE_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `SCALE` reader - Configures duty delta per update in this fade range."]
pub type SCALE_R = crate::FieldReader<u16>;
#[doc = "Field `SCALE` writer - Configures duty delta per update in this fade range."]
pub type SCALE_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `DUTY_NUM` reader - Configures number of duty updates in this fade range."]
pub type DUTY_NUM_R = crate::FieldReader<u16>;
#[doc = "Field `DUTY_NUM` writer - Configures number of duty updates in this fade range."]
pub type DUTY_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bit 0 - Configures whether to increase duty in this fade range."]
    #[inline(always)]
    pub fn duty_inc(&self) -> DUTY_INC_R {
        DUTY_INC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:10 - Configures PWM overflows between duty updates in this fade range."]
    #[inline(always)]
    pub fn duty_cycle(&self) -> DUTY_CYCLE_R {
        DUTY_CYCLE_R::new(((self.bits >> 1) & 0x03ff) as u16)
    }
    #[doc = "Bits 11:20 - Configures duty delta per update in this fade range."]
    #[inline(always)]
    pub fn scale(&self) -> SCALE_R {
        SCALE_R::new(((self.bits >> 11) & 0x03ff) as u16)
    }
    #[doc = "Bits 21:30 - Configures number of duty updates in this fade range."]
    #[inline(always)]
    pub fn duty_num(&self) -> DUTY_NUM_R {
        DUTY_NUM_R::new(((self.bits >> 21) & 0x03ff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ENTRY")
            .field("duty_inc", &self.duty_inc())
            .field("duty_cycle", &self.duty_cycle())
            .field("scale", &self.scale())
            .field("duty_num", &self.duty_num())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Configures whether to increase duty in this fade range."]
    #[inline(always)]
    pub fn duty_inc(&mut self) -> DUTY_INC_W<'_, ENTRY_SPEC> {
        DUTY_INC_W::new(self, 0)
    }
    #[doc = "Bits 1:10 - Configures PWM overflows between duty updates in this fade range."]
    #[inline(always)]
    pub fn duty_cycle(&mut self) -> DUTY_CYCLE_W<'_, ENTRY_SPEC> {
        DUTY_CYCLE_W::new(self, 1)
    }
    #[doc = "Bits 11:20 - Configures duty delta per update in this fade range."]
    #[inline(always)]
    pub fn scale(&mut self) -> SCALE_W<'_, ENTRY_SPEC> {
        SCALE_W::new(self, 11)
    }
    #[doc = "Bits 21:30 - Configures number of duty updates in this fade range."]
    #[inline(always)]
    pub fn duty_num(&mut self) -> DUTY_NUM_W<'_, ENTRY_SPEC> {
        DUTY_NUM_W::new(self, 21)
    }
}
#[doc = "LEDC gamma fade configuration RAM entry %s.\n\nYou can [`read`](crate::Reg::read) this register and get [`entry::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`entry::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ENTRY_SPEC;
impl crate::RegisterSpec for ENTRY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`entry::R`](R) reader structure"]
impl crate::Readable for ENTRY_SPEC {}
#[doc = "`write(|w| ..)` method takes [`entry::W`](W) writer structure"]
impl crate::Writable for ENTRY_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ENTRY%s to value 0"]
impl crate::Resettable for ENTRY_SPEC {}
