#[doc = "Register `PMUP_CHANNEL_CFG` reader"]
pub type R = crate::R<PMUP_CHANNEL_CFG_SPEC>;
#[doc = "Register `PMUP_CHANNEL_CFG` writer"]
pub type W = crate::W<PMUP_CHANNEL_CFG_SPEC>;
#[doc = "Field `PUMP_CHANNEL_CODE4` reader - configure cmd4 code"]
pub type PUMP_CHANNEL_CODE4_R = crate::FieldReader;
#[doc = "Field `PUMP_CHANNEL_CODE4` writer - configure cmd4 code"]
pub type PUMP_CHANNEL_CODE4_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PUMP_CHANNEL_CODE3` reader - configure cmd3 code"]
pub type PUMP_CHANNEL_CODE3_R = crate::FieldReader;
#[doc = "Field `PUMP_CHANNEL_CODE3` writer - configure cmd3 code"]
pub type PUMP_CHANNEL_CODE3_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PUMP_CHANNEL_CODE2` reader - configure cmd2 code"]
pub type PUMP_CHANNEL_CODE2_R = crate::FieldReader;
#[doc = "Field `PUMP_CHANNEL_CODE2` writer - configure cmd2 code"]
pub type PUMP_CHANNEL_CODE2_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PUMP_CHANNEL_CODE1` reader - configure cmd1 code"]
pub type PUMP_CHANNEL_CODE1_R = crate::FieldReader;
#[doc = "Field `PUMP_CHANNEL_CODE1` writer - configure cmd1 code"]
pub type PUMP_CHANNEL_CODE1_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PUMP_CHANNEL_CODE0` reader - configure cmd0 code"]
pub type PUMP_CHANNEL_CODE0_R = crate::FieldReader;
#[doc = "Field `PUMP_CHANNEL_CODE0` writer - configure cmd0 code"]
pub type PUMP_CHANNEL_CODE0_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 7:11 - configure cmd4 code"]
    #[inline(always)]
    pub fn pump_channel_code4(&self) -> PUMP_CHANNEL_CODE4_R {
        PUMP_CHANNEL_CODE4_R::new(((self.bits >> 7) & 0x1f) as u8)
    }
    #[doc = "Bits 12:16 - configure cmd3 code"]
    #[inline(always)]
    pub fn pump_channel_code3(&self) -> PUMP_CHANNEL_CODE3_R {
        PUMP_CHANNEL_CODE3_R::new(((self.bits >> 12) & 0x1f) as u8)
    }
    #[doc = "Bits 17:21 - configure cmd2 code"]
    #[inline(always)]
    pub fn pump_channel_code2(&self) -> PUMP_CHANNEL_CODE2_R {
        PUMP_CHANNEL_CODE2_R::new(((self.bits >> 17) & 0x1f) as u8)
    }
    #[doc = "Bits 22:26 - configure cmd1 code"]
    #[inline(always)]
    pub fn pump_channel_code1(&self) -> PUMP_CHANNEL_CODE1_R {
        PUMP_CHANNEL_CODE1_R::new(((self.bits >> 22) & 0x1f) as u8)
    }
    #[doc = "Bits 27:31 - configure cmd0 code"]
    #[inline(always)]
    pub fn pump_channel_code0(&self) -> PUMP_CHANNEL_CODE0_R {
        PUMP_CHANNEL_CODE0_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PMUP_CHANNEL_CFG")
            .field("pump_channel_code4", &self.pump_channel_code4())
            .field("pump_channel_code3", &self.pump_channel_code3())
            .field("pump_channel_code2", &self.pump_channel_code2())
            .field("pump_channel_code1", &self.pump_channel_code1())
            .field("pump_channel_code0", &self.pump_channel_code0())
            .finish()
    }
}
impl W {
    #[doc = "Bits 7:11 - configure cmd4 code"]
    #[inline(always)]
    pub fn pump_channel_code4(&mut self) -> PUMP_CHANNEL_CODE4_W<PMUP_CHANNEL_CFG_SPEC> {
        PUMP_CHANNEL_CODE4_W::new(self, 7)
    }
    #[doc = "Bits 12:16 - configure cmd3 code"]
    #[inline(always)]
    pub fn pump_channel_code3(&mut self) -> PUMP_CHANNEL_CODE3_W<PMUP_CHANNEL_CFG_SPEC> {
        PUMP_CHANNEL_CODE3_W::new(self, 12)
    }
    #[doc = "Bits 17:21 - configure cmd2 code"]
    #[inline(always)]
    pub fn pump_channel_code2(&mut self) -> PUMP_CHANNEL_CODE2_W<PMUP_CHANNEL_CFG_SPEC> {
        PUMP_CHANNEL_CODE2_W::new(self, 17)
    }
    #[doc = "Bits 22:26 - configure cmd1 code"]
    #[inline(always)]
    pub fn pump_channel_code1(&mut self) -> PUMP_CHANNEL_CODE1_W<PMUP_CHANNEL_CFG_SPEC> {
        PUMP_CHANNEL_CODE1_W::new(self, 22)
    }
    #[doc = "Bits 27:31 - configure cmd0 code"]
    #[inline(always)]
    pub fn pump_channel_code0(&mut self) -> PUMP_CHANNEL_CODE0_W<PMUP_CHANNEL_CFG_SPEC> {
        PUMP_CHANNEL_CODE0_W::new(self, 27)
    }
}
#[doc = "configure the code of valid pump channel code\n\nYou can [`read`](crate::Reg::read) this register and get [`pmup_channel_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmup_channel_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PMUP_CHANNEL_CFG_SPEC;
impl crate::RegisterSpec for PMUP_CHANNEL_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmup_channel_cfg::R`](R) reader structure"]
impl crate::Readable for PMUP_CHANNEL_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pmup_channel_cfg::W`](W) writer structure"]
impl crate::Writable for PMUP_CHANNEL_CFG_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PMUP_CHANNEL_CFG to value 0"]
impl crate::Resettable for PMUP_CHANNEL_CFG_SPEC {}
