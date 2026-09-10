#[doc = "Register `TSF_CTRL` reader"]
pub type R = crate::R<TSF_CTRL_SPEC>;
#[doc = "Register `TSF_CTRL` writer"]
pub type W = crate::W<TSF_CTRL_SPEC>;
#[doc = "Field `LATCH` reader - Bit n latches the TSF counter of interface n into TSF_TIME for reading. Cleared again after the read."]
pub type LATCH_R = crate::FieldReader;
#[doc = "Field `LATCH` writer - Bit n latches the TSF counter of interface n into TSF_TIME for reading. Cleared again after the read."]
pub type LATCH_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `LOAD` reader - Bit n loads the TSF counter of interface n from TSF_LOAD."]
pub type LOAD_R = crate::FieldReader;
#[doc = "Field `LOAD` writer - Bit n loads the TSF counter of interface n from TSF_LOAD."]
pub type LOAD_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `LOAD_TBTT_START` reader - Bit n loads the TBTT start time of interface n from TBTT_START."]
pub type LOAD_TBTT_START_R = crate::FieldReader;
#[doc = "Field `LOAD_TBTT_START` writer - Bit n loads the TBTT start time of interface n from TBTT_START."]
pub type LOAD_TBTT_START_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Bit n latches the TSF counter of interface n into TSF_TIME for reading. Cleared again after the read."]
    #[inline(always)]
    pub fn latch(&self) -> LATCH_R {
        LATCH_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Bit n loads the TSF counter of interface n from TSF_LOAD."]
    #[inline(always)]
    pub fn load(&self) -> LOAD_R {
        LOAD_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Bit n loads the TBTT start time of interface n from TBTT_START."]
    #[inline(always)]
    pub fn load_tbtt_start(&self) -> LOAD_TBTT_START_R {
        LOAD_TBTT_START_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TSF_CTRL")
            .field("latch", &self.latch())
            .field("load", &self.load())
            .field("load_tbtt_start", &self.load_tbtt_start())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - Bit n latches the TSF counter of interface n into TSF_TIME for reading. Cleared again after the read."]
    #[inline(always)]
    pub fn latch(&mut self) -> LATCH_W<'_, TSF_CTRL_SPEC> {
        LATCH_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - Bit n loads the TSF counter of interface n from TSF_LOAD."]
    #[inline(always)]
    pub fn load(&mut self) -> LOAD_W<'_, TSF_CTRL_SPEC> {
        LOAD_W::new(self, 4)
    }
    #[doc = "Bits 8:11 - Bit n loads the TBTT start time of interface n from TBTT_START."]
    #[inline(always)]
    pub fn load_tbtt_start(&mut self) -> LOAD_TBTT_START_W<'_, TSF_CTRL_SPEC> {
        LOAD_TBTT_START_W::new(self, 8)
    }
}
#[doc = "Control for the per-interface TSF counters. Written by tsf_hal_get_counter_value, tsf_hal_set_counter_value and tsf_hal_set_tbtt_start_time.\n\nYou can [`read`](crate::Reg::read) this register and get [`tsf_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tsf_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TSF_CTRL_SPEC;
impl crate::RegisterSpec for TSF_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tsf_ctrl::R`](R) reader structure"]
impl crate::Readable for TSF_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tsf_ctrl::W`](W) writer structure"]
impl crate::Writable for TSF_CTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TSF_CTRL to value 0"]
impl crate::Resettable for TSF_CTRL_SPEC {}
