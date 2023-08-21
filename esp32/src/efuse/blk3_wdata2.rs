#[doc = "Register `BLK3_WDATA2` reader"]
pub type R = crate::R<BLK3_WDATA2_SPEC>;
#[doc = "Register `BLK3_WDATA2` writer"]
pub type W = crate::W<BLK3_WDATA2_SPEC>;
#[doc = "Field `BLK3_DIN2` reader - "]
pub type BLK3_DIN2_R = crate::FieldReader<u32>;
#[doc = "Field `BLK3_DIN2` writer - "]
pub type BLK3_DIN2_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn blk3_din2(&self) -> BLK3_DIN2_R {
        BLK3_DIN2_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BLK3_WDATA2")
            .field("blk3_din2", &format_args!("{}", self.blk3_din2().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<BLK3_WDATA2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn blk3_din2(&mut self) -> BLK3_DIN2_W<BLK3_WDATA2_SPEC, 0> {
        BLK3_DIN2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blk3_wdata2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`blk3_wdata2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BLK3_WDATA2_SPEC;
impl crate::RegisterSpec for BLK3_WDATA2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blk3_wdata2::R`](R) reader structure"]
impl crate::Readable for BLK3_WDATA2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`blk3_wdata2::W`](W) writer structure"]
impl crate::Writable for BLK3_WDATA2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BLK3_WDATA2 to value 0"]
impl crate::Resettable for BLK3_WDATA2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
