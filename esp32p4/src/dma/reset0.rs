///Register `RESET0` reader
pub type R = crate::R<RESET0_SPEC>;
///Register `RESET0` writer
pub type W = crate::W<RESET0_SPEC>;
///Field `DMAC_RST` reader - NA
pub type DMAC_RST_R = crate::BitReader;
///Field `DMAC_RST` writer - NA
pub type DMAC_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - NA
    #[inline(always)]
    pub fn dmac_rst(&self) -> DMAC_RST_R {
        DMAC_RST_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RESET0")
            .field("dmac_rst", &self.dmac_rst())
            .finish()
    }
}
impl W {
    ///Bit 0 - NA
    #[inline(always)]
    #[must_use]
    pub fn dmac_rst(&mut self) -> DMAC_RST_W<RESET0_SPEC> {
        DMAC_RST_W::new(self, 0)
    }
}
/**NA

You can [`read`](crate::generic::Reg::read) this register and get [`reset0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reset0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct RESET0_SPEC;
impl crate::RegisterSpec for RESET0_SPEC {
    type Ux = u32;
}
///`read()` method returns [`reset0::R`](R) reader structure
impl crate::Readable for RESET0_SPEC {}
///`write(|w| ..)` method takes [`reset0::W`](W) writer structure
impl crate::Writable for RESET0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets RESET0 to value 0
impl crate::Resettable for RESET0_SPEC {
    const RESET_VALUE: u32 = 0;
}
