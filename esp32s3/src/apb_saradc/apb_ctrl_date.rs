#[doc = "Register `APB_CTRL_DATE` reader"]
pub type R = crate::R<APB_CTRL_DATE_SPEC>;
#[doc = "Register `APB_CTRL_DATE` writer"]
pub type W = crate::W<APB_CTRL_DATE_SPEC>;
#[doc = "Field `APB_CTRL_DATE` reader - version"]
pub type APB_CTRL_DATE_R = crate::FieldReader<u32>;
#[doc = "Field `APB_CTRL_DATE` writer - version"]
pub type APB_CTRL_DATE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - version"]
    #[inline(always)]
    pub fn apb_ctrl_date(&self) -> APB_CTRL_DATE_R {
        APB_CTRL_DATE_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB_CTRL_DATE")
            .field(
                "apb_ctrl_date",
                &format_args!("{}", self.apb_ctrl_date().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<APB_CTRL_DATE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - version"]
    #[inline(always)]
    #[must_use]
    pub fn apb_ctrl_date(&mut self) -> APB_CTRL_DATE_W<APB_CTRL_DATE_SPEC, 0> {
        APB_CTRL_DATE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "version\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb_ctrl_date::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb_ctrl_date::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APB_CTRL_DATE_SPEC;
impl crate::RegisterSpec for APB_CTRL_DATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb_ctrl_date::R`](R) reader structure"]
impl crate::Readable for APB_CTRL_DATE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`apb_ctrl_date::W`](W) writer structure"]
impl crate::Writable for APB_CTRL_DATE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets APB_CTRL_DATE to value 0x0210_1180"]
impl crate::Resettable for APB_CTRL_DATE_SPEC {
    const RESET_VALUE: Self::Ux = 0x0210_1180;
}
