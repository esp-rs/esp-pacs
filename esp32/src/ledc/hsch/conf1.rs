#[doc = "Register `CONF1` reader"]
pub type R = crate::R<CONF1_SPEC>;
#[doc = "Register `CONF1` writer"]
pub type W = crate::W<CONF1_SPEC>;
#[doc = "Field `DUTY_SCALE` reader - This register controls the increase or decrease step scale for high speed channel0."]
pub type DUTY_SCALE_R = crate::FieldReader<u16>;
#[doc = "Field `DUTY_SCALE` writer - This register controls the increase or decrease step scale for high speed channel0."]
pub type DUTY_SCALE_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `DUTY_CYCLE` reader - This register is used to increase or decrease the duty every reg_duty_cycle_hsch0 cycles for high speed channel0."]
pub type DUTY_CYCLE_R = crate::FieldReader<u16>;
#[doc = "Field `DUTY_CYCLE` writer - This register is used to increase or decrease the duty every reg_duty_cycle_hsch0 cycles for high speed channel0."]
pub type DUTY_CYCLE_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `DUTY_NUM` reader - This register is used to control the num of increased or decreased times for high speed channel0."]
pub type DUTY_NUM_R = crate::FieldReader<u16>;
#[doc = "Field `DUTY_NUM` writer - This register is used to control the num of increased or decreased times for high speed channel0."]
pub type DUTY_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `DUTY_INC` reader - This register is used to increase the duty of output signal or decrease the duty of output signal for high speed channel0."]
pub type DUTY_INC_R = crate::BitReader;
#[doc = "Field `DUTY_INC` writer - This register is used to increase the duty of output signal or decrease the duty of output signal for high speed channel0."]
pub type DUTY_INC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DUTY_START` reader - When reg_duty_num_hsch0 reg_duty_cycle_hsch0 and reg_duty_scale_hsch0 has been configured. these register won't take effect until set reg_duty_start_hsch0. this bit is automatically cleared by hardware."]
pub type DUTY_START_R = crate::BitReader;
#[doc = "Field `DUTY_START` writer - When reg_duty_num_hsch0 reg_duty_cycle_hsch0 and reg_duty_scale_hsch0 has been configured. these register won't take effect until set reg_duty_start_hsch0. this bit is automatically cleared by hardware."]
pub type DUTY_START_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:9 - This register controls the increase or decrease step scale for high speed channel0."]
    #[inline(always)]
    pub fn duty_scale(&self) -> DUTY_SCALE_R {
        DUTY_SCALE_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:19 - This register is used to increase or decrease the duty every reg_duty_cycle_hsch0 cycles for high speed channel0."]
    #[inline(always)]
    pub fn duty_cycle(&self) -> DUTY_CYCLE_R {
        DUTY_CYCLE_R::new(((self.bits >> 10) & 0x03ff) as u16)
    }
    #[doc = "Bits 20:29 - This register is used to control the num of increased or decreased times for high speed channel0."]
    #[inline(always)]
    pub fn duty_num(&self) -> DUTY_NUM_R {
        DUTY_NUM_R::new(((self.bits >> 20) & 0x03ff) as u16)
    }
    #[doc = "Bit 30 - This register is used to increase the duty of output signal or decrease the duty of output signal for high speed channel0."]
    #[inline(always)]
    pub fn duty_inc(&self) -> DUTY_INC_R {
        DUTY_INC_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - When reg_duty_num_hsch0 reg_duty_cycle_hsch0 and reg_duty_scale_hsch0 has been configured. these register won't take effect until set reg_duty_start_hsch0. this bit is automatically cleared by hardware."]
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
    #[doc = "Bits 0:9 - This register controls the increase or decrease step scale for high speed channel0."]
    #[inline(always)]
    #[must_use]
    pub fn duty_scale(&mut self) -> DUTY_SCALE_W<CONF1_SPEC> {
        DUTY_SCALE_W::new(self, 0)
    }
    #[doc = "Bits 10:19 - This register is used to increase or decrease the duty every reg_duty_cycle_hsch0 cycles for high speed channel0."]
    #[inline(always)]
    #[must_use]
    pub fn duty_cycle(&mut self) -> DUTY_CYCLE_W<CONF1_SPEC> {
        DUTY_CYCLE_W::new(self, 10)
    }
    #[doc = "Bits 20:29 - This register is used to control the num of increased or decreased times for high speed channel0."]
    #[inline(always)]
    #[must_use]
    pub fn duty_num(&mut self) -> DUTY_NUM_W<CONF1_SPEC> {
        DUTY_NUM_W::new(self, 20)
    }
    #[doc = "Bit 30 - This register is used to increase the duty of output signal or decrease the duty of output signal for high speed channel0."]
    #[inline(always)]
    #[must_use]
    pub fn duty_inc(&mut self) -> DUTY_INC_W<CONF1_SPEC> {
        DUTY_INC_W::new(self, 30)
    }
    #[doc = "Bit 31 - When reg_duty_num_hsch0 reg_duty_cycle_hsch0 and reg_duty_scale_hsch0 has been configured. these register won't take effect until set reg_duty_start_hsch0. this bit is automatically cleared by hardware."]
    #[inline(always)]
    #[must_use]
    pub fn duty_start(&mut self) -> DUTY_START_W<CONF1_SPEC> {
        DUTY_START_W::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`conf1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`conf1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
