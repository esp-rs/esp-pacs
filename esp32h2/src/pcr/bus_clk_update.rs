#[doc = "Register `BUS_CLK_UPDATE` reader"]
pub type R = crate::R<BUS_CLK_UPDATE_SPEC>;
#[doc = "Register `BUS_CLK_UPDATE` writer"]
pub type W = crate::W<BUS_CLK_UPDATE_SPEC>;
#[doc = "Field `BUS_CLOCK_UPDATE` reader - xxxx"]
pub type BUS_CLOCK_UPDATE_R = crate::BitReader;
#[doc = "Field `BUS_CLOCK_UPDATE` writer - xxxx"]
pub type BUS_CLOCK_UPDATE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - xxxx"]
    #[inline(always)]
    pub fn bus_clock_update(&self) -> BUS_CLOCK_UPDATE_R {
        BUS_CLOCK_UPDATE_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BUS_CLK_UPDATE")
            .field(
                "bus_clock_update",
                &format_args!("{}", self.bus_clock_update().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<BUS_CLK_UPDATE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - xxxx"]
    #[inline(always)]
    #[must_use]
    pub fn bus_clock_update(&mut self) -> BUS_CLOCK_UPDATE_W<BUS_CLK_UPDATE_SPEC, 0> {
        BUS_CLOCK_UPDATE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "xxxx\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bus_clk_update::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bus_clk_update::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BUS_CLK_UPDATE_SPEC;
impl crate::RegisterSpec for BUS_CLK_UPDATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bus_clk_update::R`](R) reader structure"]
impl crate::Readable for BUS_CLK_UPDATE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bus_clk_update::W`](W) writer structure"]
impl crate::Writable for BUS_CLK_UPDATE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BUS_CLK_UPDATE to value 0"]
impl crate::Resettable for BUS_CLK_UPDATE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
