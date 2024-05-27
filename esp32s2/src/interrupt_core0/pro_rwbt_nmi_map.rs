///Register `PRO_RWBT_NMI_MAP` reader
pub type R = crate::R<PRO_RWBT_NMI_MAP_SPEC>;
///Register `PRO_RWBT_NMI_MAP` writer
pub type W = crate::W<PRO_RWBT_NMI_MAP_SPEC>;
///Field `PRO_RWBT_NMI_MAP` reader - This register is used to map RWBT_NMI interrupt signal to one of the CPU interrupts.
pub type PRO_RWBT_NMI_MAP_R = crate::FieldReader;
///Field `PRO_RWBT_NMI_MAP` writer - This register is used to map RWBT_NMI interrupt signal to one of the CPU interrupts.
pub type PRO_RWBT_NMI_MAP_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    ///Bits 0:4 - This register is used to map RWBT_NMI interrupt signal to one of the CPU interrupts.
    #[inline(always)]
    pub fn pro_rwbt_nmi_map(&self) -> PRO_RWBT_NMI_MAP_R {
        PRO_RWBT_NMI_MAP_R::new((self.bits & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRO_RWBT_NMI_MAP")
            .field("pro_rwbt_nmi_map", &self.pro_rwbt_nmi_map())
            .finish()
    }
}
impl W {
    ///Bits 0:4 - This register is used to map RWBT_NMI interrupt signal to one of the CPU interrupts.
    #[inline(always)]
    #[must_use]
    pub fn pro_rwbt_nmi_map(&mut self) -> PRO_RWBT_NMI_MAP_W<PRO_RWBT_NMI_MAP_SPEC> {
        PRO_RWBT_NMI_MAP_W::new(self, 0)
    }
}
/**RWBT_NMI interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`pro_rwbt_nmi_map::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_rwbt_nmi_map::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PRO_RWBT_NMI_MAP_SPEC;
impl crate::RegisterSpec for PRO_RWBT_NMI_MAP_SPEC {
    type Ux = u32;
}
///`read()` method returns [`pro_rwbt_nmi_map::R`](R) reader structure
impl crate::Readable for PRO_RWBT_NMI_MAP_SPEC {}
///`write(|w| ..)` method takes [`pro_rwbt_nmi_map::W`](W) writer structure
impl crate::Writable for PRO_RWBT_NMI_MAP_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets PRO_RWBT_NMI_MAP to value 0x10
impl crate::Resettable for PRO_RWBT_NMI_MAP_SPEC {
    const RESET_VALUE: u32 = 0x10;
}
