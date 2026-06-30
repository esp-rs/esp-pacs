#[doc = "Register `SAMPLE_WAIT_CFG` reader"]
pub type R = crate::R<SAMPLE_WAIT_CFG_SPEC>;
#[doc = "Register `SAMPLE_WAIT_CFG` writer"]
pub type W = crate::W<SAMPLE_WAIT_CFG_SPEC>;
#[doc = "Field `WAIT_TARGET_SAMPLE_PAD_0` reader - sample wait target for DAC PAD 1"]
pub type WAIT_TARGET_SAMPLE_PAD_0_R = crate::FieldReader<u16>;
#[doc = "Field `WAIT_TARGET_SAMPLE_PAD_0` writer - sample wait target for DAC PAD 1"]
pub type WAIT_TARGET_SAMPLE_PAD_0_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `WAIT_TARGET_SAMPLE_PAD_1` reader - sample wait target for DAC PAD 0"]
pub type WAIT_TARGET_SAMPLE_PAD_1_R = crate::FieldReader<u16>;
#[doc = "Field `WAIT_TARGET_SAMPLE_PAD_1` writer - sample wait target for DAC PAD 0"]
pub type WAIT_TARGET_SAMPLE_PAD_1_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - sample wait target for DAC PAD 1"]
    #[inline(always)]
    pub fn wait_target_sample_pad_0(&self) -> WAIT_TARGET_SAMPLE_PAD_0_R {
        WAIT_TARGET_SAMPLE_PAD_0_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - sample wait target for DAC PAD 0"]
    #[inline(always)]
    pub fn wait_target_sample_pad_1(&self) -> WAIT_TARGET_SAMPLE_PAD_1_R {
        WAIT_TARGET_SAMPLE_PAD_1_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAMPLE_WAIT_CFG")
            .field("wait_target_sample_pad_0", &self.wait_target_sample_pad_0())
            .field("wait_target_sample_pad_1", &self.wait_target_sample_pad_1())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - sample wait target for DAC PAD 1"]
    #[inline(always)]
    pub fn wait_target_sample_pad_0(
        &mut self,
    ) -> WAIT_TARGET_SAMPLE_PAD_0_W<'_, SAMPLE_WAIT_CFG_SPEC> {
        WAIT_TARGET_SAMPLE_PAD_0_W::new(self, 0)
    }
    #[doc = "Bits 16:31 - sample wait target for DAC PAD 0"]
    #[inline(always)]
    pub fn wait_target_sample_pad_1(
        &mut self,
    ) -> WAIT_TARGET_SAMPLE_PAD_1_W<'_, SAMPLE_WAIT_CFG_SPEC> {
        WAIT_TARGET_SAMPLE_PAD_1_W::new(self, 16)
    }
}
#[doc = "cali sample phase duration register\n\nYou can [`read`](crate::Reg::read) this register and get [`sample_wait_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sample_wait_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SAMPLE_WAIT_CFG_SPEC;
impl crate::RegisterSpec for SAMPLE_WAIT_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sample_wait_cfg::R`](R) reader structure"]
impl crate::Readable for SAMPLE_WAIT_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sample_wait_cfg::W`](W) writer structure"]
impl crate::Writable for SAMPLE_WAIT_CFG_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SAMPLE_WAIT_CFG to value 0x0276_0276"]
impl crate::Resettable for SAMPLE_WAIT_CFG_SPEC {
    const RESET_VALUE: u32 = 0x0276_0276;
}
