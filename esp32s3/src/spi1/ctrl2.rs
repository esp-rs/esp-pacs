///Register `CTRL2` reader
pub type R = crate::R<CTRL2_SPEC>;
///Register `CTRL2` writer
pub type W = crate::W<CTRL2_SPEC>;
///Field `SYNC_RESET` reader - The FSM will be reset.
pub type SYNC_RESET_R = crate::BitReader;
///Field `SYNC_RESET` writer - The FSM will be reset.
pub type SYNC_RESET_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 31 - The FSM will be reset.
    #[inline(always)]
    pub fn sync_reset(&self) -> SYNC_RESET_R {
        SYNC_RESET_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL2")
            .field("sync_reset", &self.sync_reset())
            .finish()
    }
}
impl W {
    ///Bit 31 - The FSM will be reset.
    #[inline(always)]
    #[must_use]
    pub fn sync_reset(&mut self) -> SYNC_RESET_W<CTRL2_SPEC> {
        SYNC_RESET_W::new(self, 31)
    }
}
/**SPI1 control2 register

You can [`read`](crate::generic::Reg::read) this register and get [`ctrl2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CTRL2_SPEC;
impl crate::RegisterSpec for CTRL2_SPEC {
    type Ux = u32;
}
///`read()` method returns [`ctrl2::R`](R) reader structure
impl crate::Readable for CTRL2_SPEC {}
///`write(|w| ..)` method takes [`ctrl2::W`](W) writer structure
impl crate::Writable for CTRL2_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CTRL2 to value 0
impl crate::Resettable for CTRL2_SPEC {
    const RESET_VALUE: u32 = 0;
}
