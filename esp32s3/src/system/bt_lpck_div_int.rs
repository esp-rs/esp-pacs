#[doc = "Register `BT_LPCK_DIV_INT` reader"]
pub struct R(crate::R<BT_LPCK_DIV_INT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BT_LPCK_DIV_INT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BT_LPCK_DIV_INT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BT_LPCK_DIV_INT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BT_LPCK_DIV_INT` writer"]
pub struct W(crate::W<BT_LPCK_DIV_INT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BT_LPCK_DIV_INT_SPEC>;
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
impl From<crate::W<BT_LPCK_DIV_INT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BT_LPCK_DIV_INT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BT_LPCK_DIV_NUM` reader - This field is lower power clock frequent division factor"]
pub type BT_LPCK_DIV_NUM_R = crate::FieldReader<u16>;
#[doc = "Field `BT_LPCK_DIV_NUM` writer - This field is lower power clock frequent division factor"]
pub type BT_LPCK_DIV_NUM_W<'a, const O: u8> =
    crate::FieldWriter<'a, BT_LPCK_DIV_INT_SPEC, 12, O, u16>;
impl R {
    #[doc = "Bits 0:11 - This field is lower power clock frequent division factor"]
    #[inline(always)]
    pub fn bt_lpck_div_num(&self) -> BT_LPCK_DIV_NUM_R {
        BT_LPCK_DIV_NUM_R::new((self.bits & 0x0fff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BT_LPCK_DIV_INT")
            .field(
                "bt_lpck_div_num",
                &format_args!("{}", self.bt_lpck_div_num().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<BT_LPCK_DIV_INT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:11 - This field is lower power clock frequent division factor"]
    #[inline(always)]
    #[must_use]
    pub fn bt_lpck_div_num(&mut self) -> BT_LPCK_DIV_NUM_W<0> {
        BT_LPCK_DIV_NUM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "low power clock frequent division factor configuration regsiter\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bt_lpck_div_int](index.html) module"]
pub struct BT_LPCK_DIV_INT_SPEC;
impl crate::RegisterSpec for BT_LPCK_DIV_INT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bt_lpck_div_int::R](R) reader structure"]
impl crate::Readable for BT_LPCK_DIV_INT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bt_lpck_div_int::W](W) writer structure"]
impl crate::Writable for BT_LPCK_DIV_INT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BT_LPCK_DIV_INT to value 0xff"]
impl crate::Resettable for BT_LPCK_DIV_INT_SPEC {
    const RESET_VALUE: Self::Ux = 0xff;
}
