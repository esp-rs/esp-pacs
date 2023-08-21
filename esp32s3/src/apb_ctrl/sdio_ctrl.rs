#[doc = "Register `SDIO_CTRL` reader"]
pub type R = crate::R<SDIO_CTRL_SPEC>;
#[doc = "Register `SDIO_CTRL` writer"]
pub type W = crate::W<SDIO_CTRL_SPEC>;
#[doc = "Field `SDIO_WIN_ACCESS_EN` reader - ******* Description ***********"]
pub type SDIO_WIN_ACCESS_EN_R = crate::BitReader;
#[doc = "Field `SDIO_WIN_ACCESS_EN` writer - ******* Description ***********"]
pub type SDIO_WIN_ACCESS_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - ******* Description ***********"]
    #[inline(always)]
    pub fn sdio_win_access_en(&self) -> SDIO_WIN_ACCESS_EN_R {
        SDIO_WIN_ACCESS_EN_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SDIO_CTRL")
            .field(
                "sdio_win_access_en",
                &format_args!("{}", self.sdio_win_access_en().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SDIO_CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - ******* Description ***********"]
    #[inline(always)]
    #[must_use]
    pub fn sdio_win_access_en(&mut self) -> SDIO_WIN_ACCESS_EN_W<SDIO_CTRL_SPEC, 0> {
        SDIO_WIN_ACCESS_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdio_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdio_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SDIO_CTRL_SPEC;
impl crate::RegisterSpec for SDIO_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdio_ctrl::R`](R) reader structure"]
impl crate::Readable for SDIO_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sdio_ctrl::W`](W) writer structure"]
impl crate::Writable for SDIO_CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SDIO_CTRL to value 0"]
impl crate::Resettable for SDIO_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
