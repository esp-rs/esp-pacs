#[doc = "Register `BT_LPCK_DIV_INT` reader"]
pub type R = crate::R<BT_LPCK_DIV_INT_SPEC>;
#[doc = "Register `BT_LPCK_DIV_INT` writer"]
pub type W = crate::W<BT_LPCK_DIV_INT_SPEC>;
#[doc = "Field `BT_LPCK_DIV_NUM` reader - This field is lower power clock frequent division factor"]
pub type BT_LPCK_DIV_NUM_R = crate::FieldReader<u16>;
#[doc = "Field `BT_LPCK_DIV_NUM` writer - This field is lower power clock frequent division factor"]
pub type BT_LPCK_DIV_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
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
            .field("bt_lpck_div_num", &self.bt_lpck_div_num())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:11 - This field is lower power clock frequent division factor"]
    #[inline(always)]
    pub fn bt_lpck_div_num(&mut self) -> BT_LPCK_DIV_NUM_W<BT_LPCK_DIV_INT_SPEC> {
        BT_LPCK_DIV_NUM_W::new(self, 0)
    }
}
#[doc = "low power clock frequent division factor configuration regsiter\n\nYou can [`read`](crate::Reg::read) this register and get [`bt_lpck_div_int::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bt_lpck_div_int::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BT_LPCK_DIV_INT_SPEC;
impl crate::RegisterSpec for BT_LPCK_DIV_INT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bt_lpck_div_int::R`](R) reader structure"]
impl crate::Readable for BT_LPCK_DIV_INT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bt_lpck_div_int::W`](W) writer structure"]
impl crate::Writable for BT_LPCK_DIV_INT_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BT_LPCK_DIV_INT to value 0xff"]
impl crate::Resettable for BT_LPCK_DIV_INT_SPEC {
    const RESET_VALUE: u32 = 0xff;
}
