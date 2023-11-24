#[doc = "Register `TOUCH_TIMEOUT_CTRL` reader"]
pub type R = crate::R<TOUCH_TIMEOUT_CTRL_SPEC>;
#[doc = "Register `TOUCH_TIMEOUT_CTRL` writer"]
pub type W = crate::W<TOUCH_TIMEOUT_CTRL_SPEC>;
#[doc = "Field `TOUCH_TIMEOUT_NUM` reader - Set touch timeout threshold."]
pub type TOUCH_TIMEOUT_NUM_R = crate::FieldReader<u32>;
#[doc = "Field `TOUCH_TIMEOUT_NUM` writer - Set touch timeout threshold."]
pub type TOUCH_TIMEOUT_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 22, u32>;
#[doc = "Field `TOUCH_TIMEOUT_EN` reader - Enable touch timeout."]
pub type TOUCH_TIMEOUT_EN_R = crate::BitReader;
#[doc = "Field `TOUCH_TIMEOUT_EN` writer - Enable touch timeout."]
pub type TOUCH_TIMEOUT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:21 - Set touch timeout threshold."]
    #[inline(always)]
    pub fn touch_timeout_num(&self) -> TOUCH_TIMEOUT_NUM_R {
        TOUCH_TIMEOUT_NUM_R::new(self.bits & 0x003f_ffff)
    }
    #[doc = "Bit 22 - Enable touch timeout."]
    #[inline(always)]
    pub fn touch_timeout_en(&self) -> TOUCH_TIMEOUT_EN_R {
        TOUCH_TIMEOUT_EN_R::new(((self.bits >> 22) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TOUCH_TIMEOUT_CTRL")
            .field(
                "touch_timeout_num",
                &format_args!("{}", self.touch_timeout_num().bits()),
            )
            .field(
                "touch_timeout_en",
                &format_args!("{}", self.touch_timeout_en().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TOUCH_TIMEOUT_CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:21 - Set touch timeout threshold."]
    #[inline(always)]
    #[must_use]
    pub fn touch_timeout_num(&mut self) -> TOUCH_TIMEOUT_NUM_W<TOUCH_TIMEOUT_CTRL_SPEC> {
        TOUCH_TIMEOUT_NUM_W::new(self, 0)
    }
    #[doc = "Bit 22 - Enable touch timeout."]
    #[inline(always)]
    #[must_use]
    pub fn touch_timeout_en(&mut self) -> TOUCH_TIMEOUT_EN_W<TOUCH_TIMEOUT_CTRL_SPEC> {
        TOUCH_TIMEOUT_EN_W::new(self, 22)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Configure touch timeout settings\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`touch_timeout_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`touch_timeout_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TOUCH_TIMEOUT_CTRL_SPEC;
impl crate::RegisterSpec for TOUCH_TIMEOUT_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`touch_timeout_ctrl::R`](R) reader structure"]
impl crate::Readable for TOUCH_TIMEOUT_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`touch_timeout_ctrl::W`](W) writer structure"]
impl crate::Writable for TOUCH_TIMEOUT_CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TOUCH_TIMEOUT_CTRL to value 0x007f_ffff"]
impl crate::Resettable for TOUCH_TIMEOUT_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x007f_ffff;
}
