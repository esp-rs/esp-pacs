#[doc = "Register `L1_CACHE_WAY_OBJECT` reader"]
pub struct R(crate::R<L1_CACHE_WAY_OBJECT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<L1_CACHE_WAY_OBJECT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<L1_CACHE_WAY_OBJECT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<L1_CACHE_WAY_OBJECT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `L1_CACHE_WAY_OBJECT` writer"]
pub struct W(crate::W<L1_CACHE_WAY_OBJECT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<L1_CACHE_WAY_OBJECT_SPEC>;
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
impl From<crate::W<L1_CACHE_WAY_OBJECT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<L1_CACHE_WAY_OBJECT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `L1_CACHE_WAY_OBJECT` reader - Set this bits to select which way of the tag-object will be accessed. 0: way0, 1: way1, 2: way2, 3: way3, ?, 7: way7."]
pub type L1_CACHE_WAY_OBJECT_R = crate::FieldReader;
#[doc = "Field `L1_CACHE_WAY_OBJECT` writer - Set this bits to select which way of the tag-object will be accessed. 0: way0, 1: way1, 2: way2, 3: way3, ?, 7: way7."]
pub type L1_CACHE_WAY_OBJECT_W<'a, const O: u8> =
    crate::FieldWriter<'a, L1_CACHE_WAY_OBJECT_SPEC, 3, O>;
impl R {
    #[doc = "Bits 0:2 - Set this bits to select which way of the tag-object will be accessed. 0: way0, 1: way1, 2: way2, 3: way3, ?, 7: way7."]
    #[inline(always)]
    pub fn l1_cache_way_object(&self) -> L1_CACHE_WAY_OBJECT_R {
        L1_CACHE_WAY_OBJECT_R::new((self.bits & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L1_CACHE_WAY_OBJECT")
            .field(
                "l1_cache_way_object",
                &format_args!("{}", self.l1_cache_way_object().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<L1_CACHE_WAY_OBJECT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:2 - Set this bits to select which way of the tag-object will be accessed. 0: way0, 1: way1, 2: way2, 3: way3, ?, 7: way7."]
    #[inline(always)]
    #[must_use]
    pub fn l1_cache_way_object(&mut self) -> L1_CACHE_WAY_OBJECT_W<0> {
        L1_CACHE_WAY_OBJECT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Cache Tag and Data memory way register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [l1_cache_way_object](index.html) module"]
pub struct L1_CACHE_WAY_OBJECT_SPEC;
impl crate::RegisterSpec for L1_CACHE_WAY_OBJECT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [l1_cache_way_object::R](R) reader structure"]
impl crate::Readable for L1_CACHE_WAY_OBJECT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [l1_cache_way_object::W](W) writer structure"]
impl crate::Writable for L1_CACHE_WAY_OBJECT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets L1_CACHE_WAY_OBJECT to value 0"]
impl crate::Resettable for L1_CACHE_WAY_OBJECT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
