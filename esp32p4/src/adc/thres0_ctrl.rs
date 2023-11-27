#[doc = "Register `THRES0_CTRL` reader"]
pub type R = crate::R<THRES0_CTRL_SPEC>;
#[doc = "Register `THRES0_CTRL` writer"]
pub type W = crate::W<THRES0_CTRL_SPEC>;
#[doc = "Field `THRES0_CHANNEL` reader - need_des"]
pub type THRES0_CHANNEL_R = crate::FieldReader;
#[doc = "Field `THRES0_CHANNEL` writer - need_des"]
pub type THRES0_CHANNEL_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `THRES0_HIGH` reader - saradc1's thres0 monitor thres"]
pub type THRES0_HIGH_R = crate::FieldReader<u16>;
#[doc = "Field `THRES0_HIGH` writer - saradc1's thres0 monitor thres"]
pub type THRES0_HIGH_W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
#[doc = "Field `THRES0_LOW` reader - saradc1's thres0 monitor thres"]
pub type THRES0_LOW_R = crate::FieldReader<u16>;
#[doc = "Field `THRES0_LOW` writer - saradc1's thres0 monitor thres"]
pub type THRES0_LOW_W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bits 0:4 - need_des"]
    #[inline(always)]
    pub fn thres0_channel(&self) -> THRES0_CHANNEL_R {
        THRES0_CHANNEL_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:17 - saradc1's thres0 monitor thres"]
    #[inline(always)]
    pub fn thres0_high(&self) -> THRES0_HIGH_R {
        THRES0_HIGH_R::new(((self.bits >> 5) & 0x1fff) as u16)
    }
    #[doc = "Bits 18:30 - saradc1's thres0 monitor thres"]
    #[inline(always)]
    pub fn thres0_low(&self) -> THRES0_LOW_R {
        THRES0_LOW_R::new(((self.bits >> 18) & 0x1fff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("THRES0_CTRL")
            .field(
                "thres0_channel",
                &format_args!("{}", self.thres0_channel().bits()),
            )
            .field(
                "thres0_high",
                &format_args!("{}", self.thres0_high().bits()),
            )
            .field("thres0_low", &format_args!("{}", self.thres0_low().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<THRES0_CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:4 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn thres0_channel(&mut self) -> THRES0_CHANNEL_W<THRES0_CTRL_SPEC> {
        THRES0_CHANNEL_W::new(self, 0)
    }
    #[doc = "Bits 5:17 - saradc1's thres0 monitor thres"]
    #[inline(always)]
    #[must_use]
    pub fn thres0_high(&mut self) -> THRES0_HIGH_W<THRES0_CTRL_SPEC> {
        THRES0_HIGH_W::new(self, 5)
    }
    #[doc = "Bits 18:30 - saradc1's thres0 monitor thres"]
    #[inline(always)]
    #[must_use]
    pub fn thres0_low(&mut self) -> THRES0_LOW_W<THRES0_CTRL_SPEC> {
        THRES0_LOW_W::new(self, 18)
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
#[doc = "Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`thres0_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`thres0_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct THRES0_CTRL_SPEC;
impl crate::RegisterSpec for THRES0_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`thres0_ctrl::R`](R) reader structure"]
impl crate::Readable for THRES0_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`thres0_ctrl::W`](W) writer structure"]
impl crate::Writable for THRES0_CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets THRES0_CTRL to value 0x0003_ffed"]
impl crate::Resettable for THRES0_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0003_ffed;
}
