#[doc = "Register `BUS_CLK_UPDATE` reader"]
pub struct R(crate::R<BUS_CLK_UPDATE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BUS_CLK_UPDATE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BUS_CLK_UPDATE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BUS_CLK_UPDATE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BUS_CLK_UPDATE` writer"]
pub struct W(crate::W<BUS_CLK_UPDATE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BUS_CLK_UPDATE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<BUS_CLK_UPDATE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BUS_CLK_UPDATE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BUS_CLOCK_UPDATE` reader - xxxx"]
pub type BUS_CLOCK_UPDATE_R = crate::BitReader;
#[doc = "Field `BUS_CLOCK_UPDATE` writer - xxxx"]
pub type BUS_CLOCK_UPDATE_W<'a, const O: u8> = crate::BitWriter<'a, BUS_CLK_UPDATE_SPEC, O>;
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
    pub fn bus_clock_update(&mut self) -> BUS_CLOCK_UPDATE_W<0> {
        BUS_CLOCK_UPDATE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "xxxx\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bus_clk_update](index.html) module"]
pub struct BUS_CLK_UPDATE_SPEC;
impl crate::RegisterSpec for BUS_CLK_UPDATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bus_clk_update::R](R) reader structure"]
impl crate::Readable for BUS_CLK_UPDATE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bus_clk_update::W](W) writer structure"]
impl crate::Writable for BUS_CLK_UPDATE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BUS_CLK_UPDATE to value 0"]
impl crate::Resettable for BUS_CLK_UPDATE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
