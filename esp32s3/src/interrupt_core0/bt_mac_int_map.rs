#[doc = "Register `BT_MAC_INT_MAP` reader"]
pub type R = crate::R<BT_MAC_INT_MAP_SPEC>;
#[doc = "Register `BT_MAC_INT_MAP` writer"]
pub type W = crate::W<BT_MAC_INT_MAP_SPEC>;
#[doc = "Field `BT_MAC_INT_MAP` reader - this register used to map bb_mac interrupt to one of core0's external interrupt"]
pub type BT_MAC_INT_MAP_R = crate::FieldReader;
#[doc = "Field `BT_MAC_INT_MAP` writer - this register used to map bb_mac interrupt to one of core0's external interrupt"]
pub type BT_MAC_INT_MAP_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - this register used to map bb_mac interrupt to one of core0's external interrupt"]
    #[inline(always)]
    pub fn bt_mac_int_map(&self) -> BT_MAC_INT_MAP_R {
        BT_MAC_INT_MAP_R::new((self.bits & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BT_MAC_INT_MAP")
            .field("bt_mac_int_map", &self.bt_mac_int_map().bits())
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<BT_MAC_INT_MAP_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:4 - this register used to map bb_mac interrupt to one of core0's external interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn bt_mac_int_map(&mut self) -> BT_MAC_INT_MAP_W<BT_MAC_INT_MAP_SPEC> {
        BT_MAC_INT_MAP_W::new(self, 0)
    }
}
#[doc = "bb_mac interrupt configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bt_mac_int_map::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bt_mac_int_map::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BT_MAC_INT_MAP_SPEC;
impl crate::RegisterSpec for BT_MAC_INT_MAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bt_mac_int_map::R`](R) reader structure"]
impl crate::Readable for BT_MAC_INT_MAP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bt_mac_int_map::W`](W) writer structure"]
impl crate::Writable for BT_MAC_INT_MAP_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BT_MAC_INT_MAP to value 0x10"]
impl crate::Resettable for BT_MAC_INT_MAP_SPEC {
    const RESET_VALUE: u32 = 0x10;
}
