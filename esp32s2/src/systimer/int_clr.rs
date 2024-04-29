#[doc = "Register `INT_CLR` writer"]
pub type W = crate::W<INT_CLR_SPEC>;
#[doc = "Field `TARGET(0-2)` writer - Interrupt clear bit of system timer target %s."]
pub type TARGET_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Interrupt clear bit of system timer target (0-2)."]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `TARGET0` field"]
    #[inline(always)]
    #[must_use]
    pub fn target(&mut self, n: u8) -> TARGET_W<INT_CLR_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 3][n as usize];
        TARGET_W::new(self, n)
    }
    #[doc = "Bit 0 - Interrupt clear bit of system timer target 0."]
    #[inline(always)]
    #[must_use]
    pub fn target0(&mut self) -> TARGET_W<INT_CLR_SPEC> {
        TARGET_W::new(self, 0)
    }
    #[doc = "Bit 1 - Interrupt clear bit of system timer target 1."]
    #[inline(always)]
    #[must_use]
    pub fn target1(&mut self) -> TARGET_W<INT_CLR_SPEC> {
        TARGET_W::new(self, 1)
    }
    #[doc = "Bit 2 - Interrupt clear bit of system timer target 2."]
    #[inline(always)]
    #[must_use]
    pub fn target2(&mut self) -> TARGET_W<INT_CLR_SPEC> {
        TARGET_W::new(self, 2)
    }
}
#[doc = "System timer interrupt clear\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_CLR_SPEC;
impl crate::RegisterSpec for INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`int_clr::W`](W) writer structure"]
impl crate::Writable for INT_CLR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x01;
}
#[doc = "`reset()` method sets INT_CLR to value 0"]
impl crate::Resettable for INT_CLR_SPEC {
    const RESET_VALUE: u32 = 0;
}
