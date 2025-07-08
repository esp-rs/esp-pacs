#[doc = "Register `DBIAS_CHANNEL_SEL1` reader"]
pub type R = crate::R<DBIAS_CHANNEL_SEL1_SPEC>;
#[doc = "Register `DBIAS_CHANNEL_SEL1` writer"]
pub type W = crate::W<DBIAS_CHANNEL_SEL1_SPEC>;
#[doc = "Field `DBIAS_CHANNEL4_SEL` reader - needs field desc"]
pub type DBIAS_CHANNEL4_SEL_R = crate::FieldReader;
#[doc = "Field `DBIAS_CHANNEL4_SEL` writer - needs field desc"]
pub type DBIAS_CHANNEL4_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 25:31 - needs field desc"]
    #[inline(always)]
    pub fn dbias_channel4_sel(&self) -> DBIAS_CHANNEL4_SEL_R {
        DBIAS_CHANNEL4_SEL_R::new(((self.bits >> 25) & 0x7f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DBIAS_CHANNEL_SEL1")
            .field("dbias_channel4_sel", &self.dbias_channel4_sel())
            .finish()
    }
}
impl W {
    #[doc = "Bits 25:31 - needs field desc"]
    #[inline(always)]
    pub fn dbias_channel4_sel(&mut self) -> DBIAS_CHANNEL4_SEL_W<DBIAS_CHANNEL_SEL1_SPEC> {
        DBIAS_CHANNEL4_SEL_W::new(self, 25)
    }
}
#[doc = "needs desc\n\nYou can [`read`](crate::Reg::read) this register and get [`dbias_channel_sel1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbias_channel_sel1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DBIAS_CHANNEL_SEL1_SPEC;
impl crate::RegisterSpec for DBIAS_CHANNEL_SEL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbias_channel_sel1::R`](R) reader structure"]
impl crate::Readable for DBIAS_CHANNEL_SEL1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dbias_channel_sel1::W`](W) writer structure"]
impl crate::Writable for DBIAS_CHANNEL_SEL1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DBIAS_CHANNEL_SEL1 to value 0x8000_0000"]
impl crate::Resettable for DBIAS_CHANNEL_SEL1_SPEC {
    const RESET_VALUE: u32 = 0x8000_0000;
}
