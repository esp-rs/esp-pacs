#[doc = "Register `APB_PERIPHERAL_ACCESS_1` reader"]
pub type R = crate::R<APB_PERIPHERAL_ACCESS_1_SPEC>;
#[doc = "Register `APB_PERIPHERAL_ACCESS_1` writer"]
pub type W = crate::W<APB_PERIPHERAL_ACCESS_1_SPEC>;
#[doc = "Field `APB_PERIPHERAL_ACCESS_SPLIT_BURST` reader - apb_peripheral_access_split_burst"]
pub type APB_PERIPHERAL_ACCESS_SPLIT_BURST_R = crate::BitReader;
#[doc = "Field `APB_PERIPHERAL_ACCESS_SPLIT_BURST` writer - apb_peripheral_access_split_burst"]
pub type APB_PERIPHERAL_ACCESS_SPLIT_BURST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - apb_peripheral_access_split_burst"]
    #[inline(always)]
    pub fn apb_peripheral_access_split_burst(&self) -> APB_PERIPHERAL_ACCESS_SPLIT_BURST_R {
        APB_PERIPHERAL_ACCESS_SPLIT_BURST_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB_PERIPHERAL_ACCESS_1")
            .field(
                "apb_peripheral_access_split_burst",
                &format_args!("{}", self.apb_peripheral_access_split_burst().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<APB_PERIPHERAL_ACCESS_1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - apb_peripheral_access_split_burst"]
    #[inline(always)]
    #[must_use]
    pub fn apb_peripheral_access_split_burst(
        &mut self,
    ) -> APB_PERIPHERAL_ACCESS_SPLIT_BURST_W<APB_PERIPHERAL_ACCESS_1_SPEC> {
        APB_PERIPHERAL_ACCESS_SPLIT_BURST_W::new(self, 0)
    }
}
#[doc = "SENSITIVE_APB_PERIPHERAL_ACCESS_1_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb_peripheral_access_1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb_peripheral_access_1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APB_PERIPHERAL_ACCESS_1_SPEC;
impl crate::RegisterSpec for APB_PERIPHERAL_ACCESS_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb_peripheral_access_1::R`](R) reader structure"]
impl crate::Readable for APB_PERIPHERAL_ACCESS_1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`apb_peripheral_access_1::W`](W) writer structure"]
impl crate::Writable for APB_PERIPHERAL_ACCESS_1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APB_PERIPHERAL_ACCESS_1 to value 0x01"]
impl crate::Resettable for APB_PERIPHERAL_ACCESS_1_SPEC {
    const RESET_VALUE: u32 = 0x01;
}
