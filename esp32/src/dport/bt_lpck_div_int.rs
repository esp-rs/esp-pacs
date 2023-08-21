#[doc = "Register `BT_LPCK_DIV_INT` reader"]
pub type R = crate::R<BT_LPCK_DIV_INT_SPEC>;
#[doc = "Register `BT_LPCK_DIV_INT` writer"]
pub type W = crate::W<BT_LPCK_DIV_INT_SPEC>;
#[doc = "Field `BT_LPCK_DIV_NUM` reader - "]
pub type BT_LPCK_DIV_NUM_R = crate::FieldReader<u16>;
#[doc = "Field `BT_LPCK_DIV_NUM` writer - "]
pub type BT_LPCK_DIV_NUM_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 12, O, u16>;
#[doc = "Field `BTEXTWAKEUP_REQ` reader - "]
pub type BTEXTWAKEUP_REQ_R = crate::BitReader;
#[doc = "Field `BTEXTWAKEUP_REQ` writer - "]
pub type BTEXTWAKEUP_REQ_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn bt_lpck_div_num(&self) -> BT_LPCK_DIV_NUM_R {
        BT_LPCK_DIV_NUM_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn btextwakeup_req(&self) -> BTEXTWAKEUP_REQ_R {
        BTEXTWAKEUP_REQ_R::new(((self.bits >> 12) & 1) != 0)
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
            .field(
                "btextwakeup_req",
                &format_args!("{}", self.btextwakeup_req().bit()),
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
    #[doc = "Bits 0:11"]
    #[inline(always)]
    #[must_use]
    pub fn bt_lpck_div_num(&mut self) -> BT_LPCK_DIV_NUM_W<BT_LPCK_DIV_INT_SPEC, 0> {
        BT_LPCK_DIV_NUM_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn btextwakeup_req(&mut self) -> BTEXTWAKEUP_REQ_W<BT_LPCK_DIV_INT_SPEC, 12> {
        BTEXTWAKEUP_REQ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bt_lpck_div_int::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bt_lpck_div_int::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BT_LPCK_DIV_INT_SPEC;
impl crate::RegisterSpec for BT_LPCK_DIV_INT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bt_lpck_div_int::R`](R) reader structure"]
impl crate::Readable for BT_LPCK_DIV_INT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bt_lpck_div_int::W`](W) writer structure"]
impl crate::Writable for BT_LPCK_DIV_INT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BT_LPCK_DIV_INT to value 0xff"]
impl crate::Resettable for BT_LPCK_DIV_INT_SPEC {
    const RESET_VALUE: Self::Ux = 0xff;
}
