#[doc = "Register `MAC_NMI_MAP` reader"]
pub type R = crate::R<MAC_NMI_MAP_SPEC>;
#[doc = "Register `MAC_NMI_MAP` writer"]
pub type W = crate::W<MAC_NMI_MAP_SPEC>;
#[doc = "Field `MAC_NMI_MAP` reader - this register used to map_nmi interrupt to one of core0's external interrupt"]
pub type MAC_NMI_MAP_R = crate::FieldReader;
#[doc = "Field `MAC_NMI_MAP` writer - this register used to map_nmi interrupt to one of core0's external interrupt"]
pub type MAC_NMI_MAP_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - this register used to map_nmi interrupt to one of core0's external interrupt"]
    #[inline(always)]
    pub fn mac_nmi_map(&self) -> MAC_NMI_MAP_R {
        MAC_NMI_MAP_R::new((self.bits & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MAC_NMI_MAP")
            .field(
                "mac_nmi_map",
                &format_args!("{}", self.mac_nmi_map().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<MAC_NMI_MAP_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:4 - this register used to map_nmi interrupt to one of core0's external interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn mac_nmi_map(&mut self) -> MAC_NMI_MAP_W<MAC_NMI_MAP_SPEC> {
        MAC_NMI_MAP_W::new(self, 0)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "mac_nmi interrupt configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mac_nmi_map::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mac_nmi_map::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MAC_NMI_MAP_SPEC;
impl crate::RegisterSpec for MAC_NMI_MAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mac_nmi_map::R`](R) reader structure"]
impl crate::Readable for MAC_NMI_MAP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mac_nmi_map::W`](W) writer structure"]
impl crate::Writable for MAC_NMI_MAP_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MAC_NMI_MAP to value 0x10"]
impl crate::Resettable for MAC_NMI_MAP_SPEC {
    const RESET_VALUE: Self::Ux = 0x10;
}
