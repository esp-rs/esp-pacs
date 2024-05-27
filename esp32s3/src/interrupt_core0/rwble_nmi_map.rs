///Register `RWBLE_NMI_MAP` reader
pub type R = crate::R<RWBLE_NMI_MAP_SPEC>;
///Register `RWBLE_NMI_MAP` writer
pub type W = crate::W<RWBLE_NMI_MAP_SPEC>;
///Field `RWBLE_NMI_MAP` reader - this register used to map rwble_nmi interrupt to one of core0's external interrupt
pub type RWBLE_NMI_MAP_R = crate::FieldReader;
///Field `RWBLE_NMI_MAP` writer - this register used to map rwble_nmi interrupt to one of core0's external interrupt
pub type RWBLE_NMI_MAP_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    ///Bits 0:4 - this register used to map rwble_nmi interrupt to one of core0's external interrupt
    #[inline(always)]
    pub fn rwble_nmi_map(&self) -> RWBLE_NMI_MAP_R {
        RWBLE_NMI_MAP_R::new((self.bits & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RWBLE_NMI_MAP")
            .field("rwble_nmi_map", &self.rwble_nmi_map())
            .finish()
    }
}
impl W {
    ///Bits 0:4 - this register used to map rwble_nmi interrupt to one of core0's external interrupt
    #[inline(always)]
    #[must_use]
    pub fn rwble_nmi_map(&mut self) -> RWBLE_NMI_MAP_W<RWBLE_NMI_MAP_SPEC> {
        RWBLE_NMI_MAP_W::new(self, 0)
    }
}
/**rwble_nmi interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`rwble_nmi_map::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rwble_nmi_map::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct RWBLE_NMI_MAP_SPEC;
impl crate::RegisterSpec for RWBLE_NMI_MAP_SPEC {
    type Ux = u32;
}
///`read()` method returns [`rwble_nmi_map::R`](R) reader structure
impl crate::Readable for RWBLE_NMI_MAP_SPEC {}
///`write(|w| ..)` method takes [`rwble_nmi_map::W`](W) writer structure
impl crate::Writable for RWBLE_NMI_MAP_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets RWBLE_NMI_MAP to value 0x10
impl crate::Resettable for RWBLE_NMI_MAP_SPEC {
    const RESET_VALUE: u32 = 0x10;
}
