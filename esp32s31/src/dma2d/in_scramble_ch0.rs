#[doc = "Register `IN_SCRAMBLE_CH0` reader"]
pub type R = crate::R<IN_SCRAMBLE_CH0_SPEC>;
#[doc = "Register `IN_SCRAMBLE_CH0` writer"]
pub type W = crate::W<IN_SCRAMBLE_CH0_SPEC>;
#[doc = "Field `IN_SCRAMBLE_SEL_PRE_CH` reader - Scramble data before color convert : 0 : BYTE2-1-0 1 : BYTE2-0-1 2 : BYTE1-0-2 3 : BYTE1-2-0 4 : BYTE0-2-1 5 : BYTE0-1-2"]
pub type IN_SCRAMBLE_SEL_PRE_CH_R = crate::FieldReader;
#[doc = "Field `IN_SCRAMBLE_SEL_PRE_CH` writer - Scramble data before color convert : 0 : BYTE2-1-0 1 : BYTE2-0-1 2 : BYTE1-0-2 3 : BYTE1-2-0 4 : BYTE0-2-1 5 : BYTE0-1-2"]
pub type IN_SCRAMBLE_SEL_PRE_CH_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `IN_SCRAMBLE_SEL_POST_CH` reader - Scramble data after color convert : 0 : BYTE2-1-0 1 : BYTE2-0-1 2 : BYTE1-0-2 3 : BYTE1-2-0 4 : BYTE0-2-1 5 : BYTE0-1-2"]
pub type IN_SCRAMBLE_SEL_POST_CH_R = crate::FieldReader;
#[doc = "Field `IN_SCRAMBLE_SEL_POST_CH` writer - Scramble data after color convert : 0 : BYTE2-1-0 1 : BYTE2-0-1 2 : BYTE1-0-2 3 : BYTE1-2-0 4 : BYTE0-2-1 5 : BYTE0-1-2"]
pub type IN_SCRAMBLE_SEL_POST_CH_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - Scramble data before color convert : 0 : BYTE2-1-0 1 : BYTE2-0-1 2 : BYTE1-0-2 3 : BYTE1-2-0 4 : BYTE0-2-1 5 : BYTE0-1-2"]
    #[inline(always)]
    pub fn in_scramble_sel_pre_ch(&self) -> IN_SCRAMBLE_SEL_PRE_CH_R {
        IN_SCRAMBLE_SEL_PRE_CH_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - Scramble data after color convert : 0 : BYTE2-1-0 1 : BYTE2-0-1 2 : BYTE1-0-2 3 : BYTE1-2-0 4 : BYTE0-2-1 5 : BYTE0-1-2"]
    #[inline(always)]
    pub fn in_scramble_sel_post_ch(&self) -> IN_SCRAMBLE_SEL_POST_CH_R {
        IN_SCRAMBLE_SEL_POST_CH_R::new(((self.bits >> 3) & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IN_SCRAMBLE_CH0")
            .field("in_scramble_sel_pre_ch", &self.in_scramble_sel_pre_ch())
            .field("in_scramble_sel_post_ch", &self.in_scramble_sel_post_ch())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - Scramble data before color convert : 0 : BYTE2-1-0 1 : BYTE2-0-1 2 : BYTE1-0-2 3 : BYTE1-2-0 4 : BYTE0-2-1 5 : BYTE0-1-2"]
    #[inline(always)]
    pub fn in_scramble_sel_pre_ch(&mut self) -> IN_SCRAMBLE_SEL_PRE_CH_W<'_, IN_SCRAMBLE_CH0_SPEC> {
        IN_SCRAMBLE_SEL_PRE_CH_W::new(self, 0)
    }
    #[doc = "Bits 3:5 - Scramble data after color convert : 0 : BYTE2-1-0 1 : BYTE2-0-1 2 : BYTE1-0-2 3 : BYTE1-2-0 4 : BYTE0-2-1 5 : BYTE0-1-2"]
    #[inline(always)]
    pub fn in_scramble_sel_post_ch(
        &mut self,
    ) -> IN_SCRAMBLE_SEL_POST_CH_W<'_, IN_SCRAMBLE_CH0_SPEC> {
        IN_SCRAMBLE_SEL_POST_CH_W::new(self, 3)
    }
}
#[doc = "Configures the rx scramble of channel 0\n\nYou can [`read`](crate::Reg::read) this register and get [`in_scramble_ch0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`in_scramble_ch0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IN_SCRAMBLE_CH0_SPEC;
impl crate::RegisterSpec for IN_SCRAMBLE_CH0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`in_scramble_ch0::R`](R) reader structure"]
impl crate::Readable for IN_SCRAMBLE_CH0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`in_scramble_ch0::W`](W) writer structure"]
impl crate::Writable for IN_SCRAMBLE_CH0_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IN_SCRAMBLE_CH0 to value 0"]
impl crate::Resettable for IN_SCRAMBLE_CH0_SPEC {}
