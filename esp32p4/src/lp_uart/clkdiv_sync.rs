///Register `CLKDIV_SYNC` reader
pub type R = crate::R<CLKDIV_SYNC_SPEC>;
///Register `CLKDIV_SYNC` writer
pub type W = crate::W<CLKDIV_SYNC_SPEC>;
///Field `CLKDIV` reader - The integral part of the frequency divider factor.
pub type CLKDIV_R = crate::FieldReader<u16>;
///Field `CLKDIV` writer - The integral part of the frequency divider factor.
pub type CLKDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
///Field `CLKDIV_FRAG` reader - The decimal part of the frequency divider factor.
pub type CLKDIV_FRAG_R = crate::FieldReader;
///Field `CLKDIV_FRAG` writer - The decimal part of the frequency divider factor.
pub type CLKDIV_FRAG_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:11 - The integral part of the frequency divider factor.
    #[inline(always)]
    pub fn clkdiv(&self) -> CLKDIV_R {
        CLKDIV_R::new((self.bits & 0x0fff) as u16)
    }
    ///Bits 20:23 - The decimal part of the frequency divider factor.
    #[inline(always)]
    pub fn clkdiv_frag(&self) -> CLKDIV_FRAG_R {
        CLKDIV_FRAG_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLKDIV_SYNC")
            .field("clkdiv", &self.clkdiv())
            .field("clkdiv_frag", &self.clkdiv_frag())
            .finish()
    }
}
impl W {
    ///Bits 0:11 - The integral part of the frequency divider factor.
    #[inline(always)]
    #[must_use]
    pub fn clkdiv(&mut self) -> CLKDIV_W<CLKDIV_SYNC_SPEC> {
        CLKDIV_W::new(self, 0)
    }
    ///Bits 20:23 - The decimal part of the frequency divider factor.
    #[inline(always)]
    #[must_use]
    pub fn clkdiv_frag(&mut self) -> CLKDIV_FRAG_W<CLKDIV_SYNC_SPEC> {
        CLKDIV_FRAG_W::new(self, 20)
    }
}
/**Clock divider configuration

You can [`read`](crate::generic::Reg::read) this register and get [`clkdiv_sync::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkdiv_sync::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CLKDIV_SYNC_SPEC;
impl crate::RegisterSpec for CLKDIV_SYNC_SPEC {
    type Ux = u32;
}
///`read()` method returns [`clkdiv_sync::R`](R) reader structure
impl crate::Readable for CLKDIV_SYNC_SPEC {}
///`write(|w| ..)` method takes [`clkdiv_sync::W`](W) writer structure
impl crate::Writable for CLKDIV_SYNC_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CLKDIV_SYNC to value 0x02b6
impl crate::Resettable for CLKDIV_SYNC_SPEC {
    const RESET_VALUE: u32 = 0x02b6;
}
