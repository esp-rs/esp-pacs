#[doc = "Register `BLK3_WDATA1` reader"]
pub struct R(crate::R<BLK3_WDATA1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BLK3_WDATA1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BLK3_WDATA1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BLK3_WDATA1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BLK3_WDATA1` writer"]
pub struct W(crate::W<BLK3_WDATA1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BLK3_WDATA1_SPEC>;
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
impl From<crate::W<BLK3_WDATA1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BLK3_WDATA1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BLK3_DIN1` reader - program for BLOCK3"]
pub type BLK3_DIN1_R = crate::FieldReader<u32, u32>;
#[doc = "Field `BLK3_DIN1` writer - program for BLOCK3"]
pub type BLK3_DIN1_W<'a, const O: u8> = crate::FieldWriter<'a, BLK3_WDATA1_SPEC, 32, O, u32, u32>;
impl R {
    #[doc = "Bits 0:31 - program for BLOCK3"]
    #[inline(always)]
    pub fn blk3_din1(&self) -> BLK3_DIN1_R {
        BLK3_DIN1_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BLK3_WDATA1")
            .field("blk3_din1", &format_args!("{}", self.blk3_din1().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<BLK3_WDATA1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - program for BLOCK3"]
    #[inline(always)]
    #[must_use]
    pub fn blk3_din1(&mut self) -> BLK3_DIN1_W<0> {
        BLK3_DIN1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [blk3_wdata1](index.html) module"]
pub struct BLK3_WDATA1_SPEC;
impl crate::RegisterSpec for BLK3_WDATA1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [blk3_wdata1::R](R) reader structure"]
impl crate::Readable for BLK3_WDATA1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [blk3_wdata1::W](W) writer structure"]
impl crate::Writable for BLK3_WDATA1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BLK3_WDATA1 to value 0"]
impl crate::Resettable for BLK3_WDATA1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
