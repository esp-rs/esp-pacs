#[doc = "Register `BT_BB_INT_MAP` reader"]
pub struct R(crate::R<BT_BB_INT_MAP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BT_BB_INT_MAP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BT_BB_INT_MAP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BT_BB_INT_MAP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BT_BB_INT_MAP` writer"]
pub struct W(crate::W<BT_BB_INT_MAP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BT_BB_INT_MAP_SPEC>;
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
impl From<crate::W<BT_BB_INT_MAP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BT_BB_INT_MAP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BT_BB_INT_MAP` reader - Need add description"]
pub type BT_BB_INT_MAP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BT_BB_INT_MAP` writer - Need add description"]
pub type BT_BB_INT_MAP_W<'a> = crate::FieldWriter<'a, u32, BT_BB_INT_MAP_SPEC, u8, u8, 5, 0>;
impl R {
    #[doc = "Bits 0:4 - Need add description"]
    #[inline(always)]
    pub fn bt_bb_int_map(&self) -> BT_BB_INT_MAP_R {
        BT_BB_INT_MAP_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Need add description"]
    #[inline(always)]
    pub fn bt_bb_int_map(&mut self) -> BT_BB_INT_MAP_W {
        BT_BB_INT_MAP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "register description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bt_bb_int_map](index.html) module"]
pub struct BT_BB_INT_MAP_SPEC;
impl crate::RegisterSpec for BT_BB_INT_MAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bt_bb_int_map::R](R) reader structure"]
impl crate::Readable for BT_BB_INT_MAP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bt_bb_int_map::W](W) writer structure"]
impl crate::Writable for BT_BB_INT_MAP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BT_BB_INT_MAP to value 0"]
impl crate::Resettable for BT_BB_INT_MAP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
