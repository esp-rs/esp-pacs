///Register `VAD_CONF` reader
pub type R = crate::R<VAD_CONF_SPEC>;
///Register `VAD_CONF` writer
pub type W = crate::W<VAD_CONF_SPEC>;
///Field `VAD_EN` reader - VAD enable register
pub type VAD_EN_R = crate::BitReader;
///Field `VAD_EN` writer - VAD enable register
pub type VAD_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VAD_RESET` writer - VAD reset register
pub type VAD_RESET_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VAD_FORCE_START` writer - VAD force start register.
pub type VAD_FORCE_START_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - VAD enable register
    #[inline(always)]
    pub fn vad_en(&self) -> VAD_EN_R {
        VAD_EN_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VAD_CONF")
            .field("vad_en", &self.vad_en())
            .finish()
    }
}
impl W {
    ///Bit 0 - VAD enable register
    #[inline(always)]
    #[must_use]
    pub fn vad_en(&mut self) -> VAD_EN_W<VAD_CONF_SPEC> {
        VAD_EN_W::new(self, 0)
    }
    ///Bit 1 - VAD reset register
    #[inline(always)]
    #[must_use]
    pub fn vad_reset(&mut self) -> VAD_RESET_W<VAD_CONF_SPEC> {
        VAD_RESET_W::new(self, 1)
    }
    ///Bit 2 - VAD force start register.
    #[inline(always)]
    #[must_use]
    pub fn vad_force_start(&mut self) -> VAD_FORCE_START_W<VAD_CONF_SPEC> {
        VAD_FORCE_START_W::new(self, 2)
    }
}
/**I2S VAD Configure register

You can [`read`](crate::generic::Reg::read) this register and get [`vad_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vad_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct VAD_CONF_SPEC;
impl crate::RegisterSpec for VAD_CONF_SPEC {
    type Ux = u32;
}
///`read()` method returns [`vad_conf::R`](R) reader structure
impl crate::Readable for VAD_CONF_SPEC {}
///`write(|w| ..)` method takes [`vad_conf::W`](W) writer structure
impl crate::Writable for VAD_CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets VAD_CONF to value 0
impl crate::Resettable for VAD_CONF_SPEC {
    const RESET_VALUE: u32 = 0;
}
