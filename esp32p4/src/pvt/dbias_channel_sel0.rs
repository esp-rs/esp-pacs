#[doc = "Register `DBIAS_CHANNEL_SEL0` reader"]
pub type R = crate::R<DBIAS_CHANNEL_SEL0_SPEC>;
#[doc = "Register `DBIAS_CHANNEL_SEL0` writer"]
pub type W = crate::W<DBIAS_CHANNEL_SEL0_SPEC>;
#[doc = "Field `DBIAS_CHANNEL3_SEL` reader - needs field desc"]
pub type DBIAS_CHANNEL3_SEL_R = crate::FieldReader;
#[doc = "Field `DBIAS_CHANNEL3_SEL` writer - needs field desc"]
pub type DBIAS_CHANNEL3_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `DBIAS_CHANNEL2_SEL` reader - needs field desc"]
pub type DBIAS_CHANNEL2_SEL_R = crate::FieldReader;
#[doc = "Field `DBIAS_CHANNEL2_SEL` writer - needs field desc"]
pub type DBIAS_CHANNEL2_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `DBIAS_CHANNEL1_SEL` reader - needs field desc"]
pub type DBIAS_CHANNEL1_SEL_R = crate::FieldReader;
#[doc = "Field `DBIAS_CHANNEL1_SEL` writer - needs field desc"]
pub type DBIAS_CHANNEL1_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `DBIAS_CHANNEL0_SEL` reader - needs field desc"]
pub type DBIAS_CHANNEL0_SEL_R = crate::FieldReader;
#[doc = "Field `DBIAS_CHANNEL0_SEL` writer - needs field desc"]
pub type DBIAS_CHANNEL0_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 4:10 - needs field desc"]
    #[inline(always)]
    pub fn dbias_channel3_sel(&self) -> DBIAS_CHANNEL3_SEL_R {
        DBIAS_CHANNEL3_SEL_R::new(((self.bits >> 4) & 0x7f) as u8)
    }
    #[doc = "Bits 11:17 - needs field desc"]
    #[inline(always)]
    pub fn dbias_channel2_sel(&self) -> DBIAS_CHANNEL2_SEL_R {
        DBIAS_CHANNEL2_SEL_R::new(((self.bits >> 11) & 0x7f) as u8)
    }
    #[doc = "Bits 18:24 - needs field desc"]
    #[inline(always)]
    pub fn dbias_channel1_sel(&self) -> DBIAS_CHANNEL1_SEL_R {
        DBIAS_CHANNEL1_SEL_R::new(((self.bits >> 18) & 0x7f) as u8)
    }
    #[doc = "Bits 25:31 - needs field desc"]
    #[inline(always)]
    pub fn dbias_channel0_sel(&self) -> DBIAS_CHANNEL0_SEL_R {
        DBIAS_CHANNEL0_SEL_R::new(((self.bits >> 25) & 0x7f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DBIAS_CHANNEL_SEL0")
            .field(
                "dbias_channel3_sel",
                &format_args!("{}", self.dbias_channel3_sel().bits()),
            )
            .field(
                "dbias_channel2_sel",
                &format_args!("{}", self.dbias_channel2_sel().bits()),
            )
            .field(
                "dbias_channel1_sel",
                &format_args!("{}", self.dbias_channel1_sel().bits()),
            )
            .field(
                "dbias_channel0_sel",
                &format_args!("{}", self.dbias_channel0_sel().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DBIAS_CHANNEL_SEL0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 4:10 - needs field desc"]
    #[inline(always)]
    #[must_use]
    pub fn dbias_channel3_sel(&mut self) -> DBIAS_CHANNEL3_SEL_W<DBIAS_CHANNEL_SEL0_SPEC> {
        DBIAS_CHANNEL3_SEL_W::new(self, 4)
    }
    #[doc = "Bits 11:17 - needs field desc"]
    #[inline(always)]
    #[must_use]
    pub fn dbias_channel2_sel(&mut self) -> DBIAS_CHANNEL2_SEL_W<DBIAS_CHANNEL_SEL0_SPEC> {
        DBIAS_CHANNEL2_SEL_W::new(self, 11)
    }
    #[doc = "Bits 18:24 - needs field desc"]
    #[inline(always)]
    #[must_use]
    pub fn dbias_channel1_sel(&mut self) -> DBIAS_CHANNEL1_SEL_W<DBIAS_CHANNEL_SEL0_SPEC> {
        DBIAS_CHANNEL1_SEL_W::new(self, 18)
    }
    #[doc = "Bits 25:31 - needs field desc"]
    #[inline(always)]
    #[must_use]
    pub fn dbias_channel0_sel(&mut self) -> DBIAS_CHANNEL0_SEL_W<DBIAS_CHANNEL_SEL0_SPEC> {
        DBIAS_CHANNEL0_SEL_W::new(self, 25)
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
#[doc = "needs desc\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dbias_channel_sel0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dbias_channel_sel0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DBIAS_CHANNEL_SEL0_SPEC;
impl crate::RegisterSpec for DBIAS_CHANNEL_SEL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbias_channel_sel0::R`](R) reader structure"]
impl crate::Readable for DBIAS_CHANNEL_SEL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dbias_channel_sel0::W`](W) writer structure"]
impl crate::Writable for DBIAS_CHANNEL_SEL0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DBIAS_CHANNEL_SEL0 to value 0x8102_0400"]
impl crate::Resettable for DBIAS_CHANNEL_SEL0_SPEC {
    const RESET_VALUE: Self::Ux = 0x8102_0400;
}
