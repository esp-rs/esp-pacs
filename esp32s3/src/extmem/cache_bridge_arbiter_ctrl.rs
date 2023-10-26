#[doc = "Register `CACHE_BRIDGE_ARBITER_CTRL` reader"]
pub type R = crate::R<CACHE_BRIDGE_ARBITER_CTRL_SPEC>;
#[doc = "Register `CACHE_BRIDGE_ARBITER_CTRL` writer"]
pub type W = crate::W<CACHE_BRIDGE_ARBITER_CTRL_SPEC>;
#[doc = "Field `ALLOC_WB_HOLD_ARBITER` reader - Reserved"]
pub type ALLOC_WB_HOLD_ARBITER_R = crate::BitReader;
#[doc = "Field `ALLOC_WB_HOLD_ARBITER` writer - Reserved"]
pub type ALLOC_WB_HOLD_ARBITER_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Reserved"]
    #[inline(always)]
    pub fn alloc_wb_hold_arbiter(&self) -> ALLOC_WB_HOLD_ARBITER_R {
        ALLOC_WB_HOLD_ARBITER_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CACHE_BRIDGE_ARBITER_CTRL")
            .field(
                "alloc_wb_hold_arbiter",
                &format_args!("{}", self.alloc_wb_hold_arbiter().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CACHE_BRIDGE_ARBITER_CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn alloc_wb_hold_arbiter(
        &mut self,
    ) -> ALLOC_WB_HOLD_ARBITER_W<CACHE_BRIDGE_ARBITER_CTRL_SPEC, 0> {
        ALLOC_WB_HOLD_ARBITER_W::new(self)
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
#[doc = "******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cache_bridge_arbiter_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cache_bridge_arbiter_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CACHE_BRIDGE_ARBITER_CTRL_SPEC;
impl crate::RegisterSpec for CACHE_BRIDGE_ARBITER_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cache_bridge_arbiter_ctrl::R`](R) reader structure"]
impl crate::Readable for CACHE_BRIDGE_ARBITER_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cache_bridge_arbiter_ctrl::W`](W) writer structure"]
impl crate::Writable for CACHE_BRIDGE_ARBITER_CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CACHE_BRIDGE_ARBITER_CTRL to value 0"]
impl crate::Resettable for CACHE_BRIDGE_ARBITER_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
