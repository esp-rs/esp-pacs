///Register `SAR_DEBUG_CONF` reader
pub type R = crate::R<SAR_DEBUG_CONF_SPEC>;
///Register `SAR_DEBUG_CONF` writer
pub type W = crate::W<SAR_DEBUG_CONF_SPEC>;
///Field `SAR_DEBUG_BIT_SEL` reader - no public
pub type SAR_DEBUG_BIT_SEL_R = crate::FieldReader;
///Field `SAR_DEBUG_BIT_SEL` writer - no public
pub type SAR_DEBUG_BIT_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    ///Bits 0:4 - no public
    #[inline(always)]
    pub fn sar_debug_bit_sel(&self) -> SAR_DEBUG_BIT_SEL_R {
        SAR_DEBUG_BIT_SEL_R::new((self.bits & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAR_DEBUG_CONF")
            .field("sar_debug_bit_sel", &self.sar_debug_bit_sel())
            .finish()
    }
}
impl W {
    ///Bits 0:4 - no public
    #[inline(always)]
    #[must_use]
    pub fn sar_debug_bit_sel(&mut self) -> SAR_DEBUG_BIT_SEL_W<SAR_DEBUG_CONF_SPEC> {
        SAR_DEBUG_BIT_SEL_W::new(self, 0)
    }
}
/**rtc peri debug configure

You can [`read`](crate::generic::Reg::read) this register and get [`sar_debug_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sar_debug_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SAR_DEBUG_CONF_SPEC;
impl crate::RegisterSpec for SAR_DEBUG_CONF_SPEC {
    type Ux = u32;
}
///`read()` method returns [`sar_debug_conf::R`](R) reader structure
impl crate::Readable for SAR_DEBUG_CONF_SPEC {}
///`write(|w| ..)` method takes [`sar_debug_conf::W`](W) writer structure
impl crate::Writable for SAR_DEBUG_CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SAR_DEBUG_CONF to value 0
impl crate::Resettable for SAR_DEBUG_CONF_SPEC {
    const RESET_VALUE: u32 = 0;
}
