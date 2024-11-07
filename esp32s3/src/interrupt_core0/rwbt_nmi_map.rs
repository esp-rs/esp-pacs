#[doc = "Register `RWBT_NMI_MAP` reader"]
pub type R = crate::R<RWBT_NMI_MAP_SPEC>;
#[doc = "Register `RWBT_NMI_MAP` writer"]
pub type W = crate::W<RWBT_NMI_MAP_SPEC>;
#[doc = "Field `RWBT_NMI_MAP` reader - this register used to map mac rwbt_nmi to one of core0's external interrupt"]
pub type RWBT_NMI_MAP_R = crate::FieldReader;
#[doc = "Field `RWBT_NMI_MAP` writer - this register used to map mac rwbt_nmi to one of core0's external interrupt"]
pub type RWBT_NMI_MAP_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - this register used to map mac rwbt_nmi to one of core0's external interrupt"]
    #[inline(always)]
    pub fn rwbt_nmi_map(&self) -> RWBT_NMI_MAP_R {
        RWBT_NMI_MAP_R::new((self.bits & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RWBT_NMI_MAP")
            .field("rwbt_nmi_map", &self.rwbt_nmi_map())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:4 - this register used to map mac rwbt_nmi to one of core0's external interrupt"]
    #[inline(always)]
    pub fn rwbt_nmi_map(&mut self) -> RWBT_NMI_MAP_W<RWBT_NMI_MAP_SPEC> {
        RWBT_NMI_MAP_W::new(self, 0)
    }
}
#[doc = "rwbt_nmi interrupt configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`rwbt_nmi_map::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rwbt_nmi_map::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RWBT_NMI_MAP_SPEC;
impl crate::RegisterSpec for RWBT_NMI_MAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rwbt_nmi_map::R`](R) reader structure"]
impl crate::Readable for RWBT_NMI_MAP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rwbt_nmi_map::W`](W) writer structure"]
impl crate::Writable for RWBT_NMI_MAP_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RWBT_NMI_MAP to value 0x10"]
impl crate::Resettable for RWBT_NMI_MAP_SPEC {
    const RESET_VALUE: u32 = 0x10;
}
