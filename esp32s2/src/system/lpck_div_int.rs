#[doc = "Register `LPCK_DIV_INT` reader"]
pub type R = crate::R<LPCK_DIV_INT_SPEC>;
#[doc = "Register `LPCK_DIV_INT` writer"]
pub type W = crate::W<LPCK_DIV_INT_SPEC>;
#[doc = "Field `LPCK_DIV_NUM` reader - This field is used to set the integer number of the divider value."]
pub type LPCK_DIV_NUM_R = crate::FieldReader<u16>;
#[doc = "Field `LPCK_DIV_NUM` writer - This field is used to set the integer number of the divider value."]
pub type LPCK_DIV_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - This field is used to set the integer number of the divider value."]
    #[inline(always)]
    pub fn lpck_div_num(&self) -> LPCK_DIV_NUM_R {
        LPCK_DIV_NUM_R::new((self.bits & 0x0fff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPCK_DIV_INT")
            .field("lpck_div_num", &self.lpck_div_num())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:11 - This field is used to set the integer number of the divider value."]
    #[inline(always)]
    #[must_use]
    pub fn lpck_div_num(&mut self) -> LPCK_DIV_NUM_W<LPCK_DIV_INT_SPEC> {
        LPCK_DIV_NUM_W::new(self, 0)
    }
}
#[doc = "Low power clock divider integer register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lpck_div_int::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lpck_div_int::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LPCK_DIV_INT_SPEC;
impl crate::RegisterSpec for LPCK_DIV_INT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lpck_div_int::R`](R) reader structure"]
impl crate::Readable for LPCK_DIV_INT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lpck_div_int::W`](W) writer structure"]
impl crate::Writable for LPCK_DIV_INT_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LPCK_DIV_INT to value 0xff"]
impl crate::Resettable for LPCK_DIV_INT_SPEC {
    const RESET_VALUE: u32 = 0xff;
}
