#[doc = "Register `DOUT_NUM` reader"]
pub type R = crate::R<DOUT_NUM_SPEC>;
#[doc = "Register `DOUT_NUM` writer"]
pub type W = crate::W<DOUT_NUM_SPEC>;
#[doc = "Field `DOUT0_NUM` reader - the output signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
pub type DOUT0_NUM_R = crate::FieldReader;
#[doc = "Field `DOUT0_NUM` writer - the output signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
pub type DOUT0_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DOUT1_NUM` reader - the output signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
pub type DOUT1_NUM_R = crate::FieldReader;
#[doc = "Field `DOUT1_NUM` writer - the output signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
pub type DOUT1_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DOUT2_NUM` reader - the output signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
pub type DOUT2_NUM_R = crate::FieldReader;
#[doc = "Field `DOUT2_NUM` writer - the output signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
pub type DOUT2_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DOUT3_NUM` reader - the output signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
pub type DOUT3_NUM_R = crate::FieldReader;
#[doc = "Field `DOUT3_NUM` writer - the output signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
pub type DOUT3_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DOUT4_NUM` reader - the output signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
pub type DOUT4_NUM_R = crate::FieldReader;
#[doc = "Field `DOUT4_NUM` writer - the output signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
pub type DOUT4_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DOUT5_NUM` reader - the output signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
pub type DOUT5_NUM_R = crate::FieldReader;
#[doc = "Field `DOUT5_NUM` writer - the output signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
pub type DOUT5_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DOUT6_NUM` reader - the output signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
pub type DOUT6_NUM_R = crate::FieldReader;
#[doc = "Field `DOUT6_NUM` writer - the output signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
pub type DOUT6_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DOUT7_NUM` reader - the output signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
pub type DOUT7_NUM_R = crate::FieldReader;
#[doc = "Field `DOUT7_NUM` writer - the output signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
pub type DOUT7_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - the output signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
    #[inline(always)]
    pub fn dout0_num(&self) -> DOUT0_NUM_R {
        DOUT0_NUM_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - the output signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
    #[inline(always)]
    pub fn dout1_num(&self) -> DOUT1_NUM_R {
        DOUT1_NUM_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - the output signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
    #[inline(always)]
    pub fn dout2_num(&self) -> DOUT2_NUM_R {
        DOUT2_NUM_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - the output signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
    #[inline(always)]
    pub fn dout3_num(&self) -> DOUT3_NUM_R {
        DOUT3_NUM_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - the output signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
    #[inline(always)]
    pub fn dout4_num(&self) -> DOUT4_NUM_R {
        DOUT4_NUM_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - the output signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
    #[inline(always)]
    pub fn dout5_num(&self) -> DOUT5_NUM_R {
        DOUT5_NUM_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - the output signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
    #[inline(always)]
    pub fn dout6_num(&self) -> DOUT6_NUM_R {
        DOUT6_NUM_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - the output signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
    #[inline(always)]
    pub fn dout7_num(&self) -> DOUT7_NUM_R {
        DOUT7_NUM_R::new(((self.bits >> 14) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DOUT_NUM")
            .field("dout0_num", &self.dout0_num())
            .field("dout1_num", &self.dout1_num())
            .field("dout2_num", &self.dout2_num())
            .field("dout3_num", &self.dout3_num())
            .field("dout4_num", &self.dout4_num())
            .field("dout5_num", &self.dout5_num())
            .field("dout6_num", &self.dout6_num())
            .field("dout7_num", &self.dout7_num())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - the output signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
    #[inline(always)]
    pub fn dout0_num(&mut self) -> DOUT0_NUM_W<'_, DOUT_NUM_SPEC> {
        DOUT0_NUM_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - the output signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
    #[inline(always)]
    pub fn dout1_num(&mut self) -> DOUT1_NUM_W<'_, DOUT_NUM_SPEC> {
        DOUT1_NUM_W::new(self, 2)
    }
    #[doc = "Bits 4:5 - the output signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
    #[inline(always)]
    pub fn dout2_num(&mut self) -> DOUT2_NUM_W<'_, DOUT_NUM_SPEC> {
        DOUT2_NUM_W::new(self, 4)
    }
    #[doc = "Bits 6:7 - the output signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
    #[inline(always)]
    pub fn dout3_num(&mut self) -> DOUT3_NUM_W<'_, DOUT_NUM_SPEC> {
        DOUT3_NUM_W::new(self, 6)
    }
    #[doc = "Bits 8:9 - the output signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
    #[inline(always)]
    pub fn dout4_num(&mut self) -> DOUT4_NUM_W<'_, DOUT_NUM_SPEC> {
        DOUT4_NUM_W::new(self, 8)
    }
    #[doc = "Bits 10:11 - the output signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
    #[inline(always)]
    pub fn dout5_num(&mut self) -> DOUT5_NUM_W<'_, DOUT_NUM_SPEC> {
        DOUT5_NUM_W::new(self, 10)
    }
    #[doc = "Bits 12:13 - the output signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
    #[inline(always)]
    pub fn dout6_num(&mut self) -> DOUT6_NUM_W<'_, DOUT_NUM_SPEC> {
        DOUT6_NUM_W::new(self, 12)
    }
    #[doc = "Bits 14:15 - the output signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
    #[inline(always)]
    pub fn dout7_num(&mut self) -> DOUT7_NUM_W<'_, DOUT_NUM_SPEC> {
        DOUT7_NUM_W::new(self, 14)
    }
}
#[doc = "SPI output delay number configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`dout_num::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dout_num::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DOUT_NUM_SPEC;
impl crate::RegisterSpec for DOUT_NUM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dout_num::R`](R) reader structure"]
impl crate::Readable for DOUT_NUM_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dout_num::W`](W) writer structure"]
impl crate::Writable for DOUT_NUM_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DOUT_NUM to value 0"]
impl crate::Resettable for DOUT_NUM_SPEC {}
