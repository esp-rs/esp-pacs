///Register `CONF` reader
pub type R = crate::R<CONF_SPEC>;
///Register `CONF` writer
pub type W = crate::W<CONF_SPEC>;
///Field `MODE` reader - Set this field to choose the huk process. 1: process huk generate mode. 0: process huk recovery mode.
pub type MODE_R = crate::BitReader;
///Field `MODE` writer - Set this field to choose the huk process. 1: process huk generate mode. 0: process huk recovery mode.
pub type MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Set this field to choose the huk process. 1: process huk generate mode. 0: process huk recovery mode.
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONF").field("mode", &self.mode()).finish()
    }
}
impl W {
    ///Bit 0 - Set this field to choose the huk process. 1: process huk generate mode. 0: process huk recovery mode.
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<CONF_SPEC> {
        MODE_W::new(self, 0)
    }
}
/**HUK Generator configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CONF_SPEC;
impl crate::RegisterSpec for CONF_SPEC {
    type Ux = u32;
}
///`read()` method returns [`conf::R`](R) reader structure
impl crate::Readable for CONF_SPEC {}
///`write(|w| ..)` method takes [`conf::W`](W) writer structure
impl crate::Writable for CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CONF to value 0
impl crate::Resettable for CONF_SPEC {
    const RESET_VALUE: u32 = 0;
}
