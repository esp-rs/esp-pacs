#[doc = "Register `OUT_SCRAMBLE_CH%s` reader"]
pub type R = crate::R<OUT_SCRAMBLE_CH_SPEC>;
#[doc = "Register `OUT_SCRAMBLE_CH%s` writer"]
pub type W = crate::W<OUT_SCRAMBLE_CH_SPEC>;
#[doc = "Field `OUT_SCRAMBLE_SEL_PRE_CH` reader - Scramble data before color convert : 0 : BYTE2-1-0 1 : BYTE2-0-1 2 : BYTE1-0-2 3 : BYTE1-2-0 4 : BYTE0-2-1 5 : BYTE0-1-2"]
pub type OUT_SCRAMBLE_SEL_PRE_CH_R = crate::FieldReader;
#[doc = "Field `OUT_SCRAMBLE_SEL_PRE_CH` writer - Scramble data before color convert : 0 : BYTE2-1-0 1 : BYTE2-0-1 2 : BYTE1-0-2 3 : BYTE1-2-0 4 : BYTE0-2-1 5 : BYTE0-1-2"]
pub type OUT_SCRAMBLE_SEL_PRE_CH_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - Scramble data before color convert : 0 : BYTE2-1-0 1 : BYTE2-0-1 2 : BYTE1-0-2 3 : BYTE1-2-0 4 : BYTE0-2-1 5 : BYTE0-1-2"]
    #[inline(always)]
    pub fn out_scramble_sel_pre_ch(&self) -> OUT_SCRAMBLE_SEL_PRE_CH_R {
        OUT_SCRAMBLE_SEL_PRE_CH_R::new((self.bits & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OUT_SCRAMBLE_CH")
            .field("out_scramble_sel_pre_ch", &self.out_scramble_sel_pre_ch())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - Scramble data before color convert : 0 : BYTE2-1-0 1 : BYTE2-0-1 2 : BYTE1-0-2 3 : BYTE1-2-0 4 : BYTE0-2-1 5 : BYTE0-1-2"]
    #[inline(always)]
    pub fn out_scramble_sel_pre_ch(
        &mut self,
    ) -> OUT_SCRAMBLE_SEL_PRE_CH_W<'_, OUT_SCRAMBLE_CH_SPEC> {
        OUT_SCRAMBLE_SEL_PRE_CH_W::new(self, 0)
    }
}
#[doc = "Configures the tx scramble of channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`out_scramble_ch::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out_scramble_ch::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OUT_SCRAMBLE_CH_SPEC;
impl crate::RegisterSpec for OUT_SCRAMBLE_CH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`out_scramble_ch::R`](R) reader structure"]
impl crate::Readable for OUT_SCRAMBLE_CH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`out_scramble_ch::W`](W) writer structure"]
impl crate::Writable for OUT_SCRAMBLE_CH_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OUT_SCRAMBLE_CH%s to value 0"]
impl crate::Resettable for OUT_SCRAMBLE_CH_SPEC {}
