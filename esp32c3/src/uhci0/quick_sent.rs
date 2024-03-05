#[doc = "Register `QUICK_SENT` reader"]
pub type R = crate::R<QUICK_SENT_SPEC>;
#[doc = "Register `QUICK_SENT` writer"]
pub type W = crate::W<QUICK_SENT_SPEC>;
#[doc = "Field `SINGLE_SEND_NUM` reader - a"]
pub type SINGLE_SEND_NUM_R = crate::FieldReader;
#[doc = "Field `SINGLE_SEND_NUM` writer - a"]
pub type SINGLE_SEND_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SINGLE_SEND_EN` reader - a"]
pub type SINGLE_SEND_EN_R = crate::BitReader;
#[doc = "Field `SINGLE_SEND_EN` writer - a"]
pub type SINGLE_SEND_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALWAYS_SEND_NUM` reader - a"]
pub type ALWAYS_SEND_NUM_R = crate::FieldReader;
#[doc = "Field `ALWAYS_SEND_NUM` writer - a"]
pub type ALWAYS_SEND_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ALWAYS_SEND_EN` reader - a"]
pub type ALWAYS_SEND_EN_R = crate::BitReader;
#[doc = "Field `ALWAYS_SEND_EN` writer - a"]
pub type ALWAYS_SEND_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - a"]
    #[inline(always)]
    pub fn single_send_num(&self) -> SINGLE_SEND_NUM_R {
        SINGLE_SEND_NUM_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - a"]
    #[inline(always)]
    pub fn single_send_en(&self) -> SINGLE_SEND_EN_R {
        SINGLE_SEND_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - a"]
    #[inline(always)]
    pub fn always_send_num(&self) -> ALWAYS_SEND_NUM_R {
        ALWAYS_SEND_NUM_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - a"]
    #[inline(always)]
    pub fn always_send_en(&self) -> ALWAYS_SEND_EN_R {
        ALWAYS_SEND_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("QUICK_SENT")
            .field(
                "single_send_num",
                &format_args!("{}", self.single_send_num().bits()),
            )
            .field(
                "single_send_en",
                &format_args!("{}", self.single_send_en().bit()),
            )
            .field(
                "always_send_num",
                &format_args!("{}", self.always_send_num().bits()),
            )
            .field(
                "always_send_en",
                &format_args!("{}", self.always_send_en().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<QUICK_SENT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:2 - a"]
    #[inline(always)]
    #[must_use]
    pub fn single_send_num(&mut self) -> SINGLE_SEND_NUM_W<QUICK_SENT_SPEC> {
        SINGLE_SEND_NUM_W::new(self, 0)
    }
    #[doc = "Bit 3 - a"]
    #[inline(always)]
    #[must_use]
    pub fn single_send_en(&mut self) -> SINGLE_SEND_EN_W<QUICK_SENT_SPEC> {
        SINGLE_SEND_EN_W::new(self, 3)
    }
    #[doc = "Bits 4:6 - a"]
    #[inline(always)]
    #[must_use]
    pub fn always_send_num(&mut self) -> ALWAYS_SEND_NUM_W<QUICK_SENT_SPEC> {
        ALWAYS_SEND_NUM_W::new(self, 4)
    }
    #[doc = "Bit 7 - a"]
    #[inline(always)]
    #[must_use]
    pub fn always_send_en(&mut self) -> ALWAYS_SEND_EN_W<QUICK_SENT_SPEC> {
        ALWAYS_SEND_EN_W::new(self, 7)
    }
}
#[doc = "a\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`quick_sent::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`quick_sent::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QUICK_SENT_SPEC;
impl crate::RegisterSpec for QUICK_SENT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`quick_sent::R`](R) reader structure"]
impl crate::Readable for QUICK_SENT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`quick_sent::W`](W) writer structure"]
impl crate::Writable for QUICK_SENT_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets QUICK_SENT to value 0"]
impl crate::Resettable for QUICK_SENT_SPEC {
    const RESET_VALUE: u32 = 0;
}
