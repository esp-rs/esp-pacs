///Register `CORE_IBUS_TIMEOUT` reader
pub type R = crate::R<CORE_IBUS_TIMEOUT_SPEC>;
///Register `CORE_IBUS_TIMEOUT` writer
pub type W = crate::W<CORE_IBUS_TIMEOUT_SPEC>;
///Field `EN` reader - set this field to 1 to enable hp core0&amp;1 ibus timeout handle
pub type EN_R = crate::BitReader;
///Field `EN` writer - set this field to 1 to enable hp core0&amp;1 ibus timeout handle
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `THRES` reader - This field used to set hp core0&amp;1 ibus timeout threshold
pub type THRES_R = crate::FieldReader<u16>;
///Field `THRES` writer - This field used to set hp core0&amp;1 ibus timeout threshold
pub type THRES_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bit 0 - set this field to 1 to enable hp core0&amp;1 ibus timeout handle
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    ///Bits 1:16 - This field used to set hp core0&amp;1 ibus timeout threshold
    #[inline(always)]
    pub fn thres(&self) -> THRES_R {
        THRES_R::new(((self.bits >> 1) & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORE_IBUS_TIMEOUT")
            .field("en", &self.en())
            .field("thres", &self.thres())
            .finish()
    }
}
impl W {
    ///Bit 0 - set this field to 1 to enable hp core0&amp;1 ibus timeout handle
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<CORE_IBUS_TIMEOUT_SPEC> {
        EN_W::new(self, 0)
    }
    ///Bits 1:16 - This field used to set hp core0&amp;1 ibus timeout threshold
    #[inline(always)]
    #[must_use]
    pub fn thres(&mut self) -> THRES_W<CORE_IBUS_TIMEOUT_SPEC> {
        THRES_W::new(self, 1)
    }
}
/**need_des

You can [`read`](crate::generic::Reg::read) this register and get [`core_ibus_timeout::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_ibus_timeout::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CORE_IBUS_TIMEOUT_SPEC;
impl crate::RegisterSpec for CORE_IBUS_TIMEOUT_SPEC {
    type Ux = u32;
}
///`read()` method returns [`core_ibus_timeout::R`](R) reader structure
impl crate::Readable for CORE_IBUS_TIMEOUT_SPEC {}
///`write(|w| ..)` method takes [`core_ibus_timeout::W`](W) writer structure
impl crate::Writable for CORE_IBUS_TIMEOUT_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CORE_IBUS_TIMEOUT to value 0x0001_ffff
impl crate::Resettable for CORE_IBUS_TIMEOUT_SPEC {
    const RESET_VALUE: u32 = 0x0001_ffff;
}
