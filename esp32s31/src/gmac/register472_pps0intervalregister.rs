#[doc = "Register `REGISTER472_PPS0INTERVALREGISTER` reader"]
pub type R = crate::R<REGISTER472_PPS0INTERVALREGISTER_SPEC>;
#[doc = "Register `REGISTER472_PPS0INTERVALREGISTER` writer"]
pub type W = crate::W<REGISTER472_PPS0INTERVALREGISTER_SPEC>;
#[doc = "Field `PPSINT` reader - PPS0 Output Signal Interval These bits store the interval between the rising edges of PPS0 signal output in terms of units of subsecond increment value You need to program one value less than the required interval For example, if the PTP reference clock is 50 MHz _period of 20ns_, and desired interval between rising edges of PPS0 signal output is 100ns _that is, five units of subsecond increment value_, then you should program value 4 _5 1_ in this register"]
pub type PPSINT_R = crate::FieldReader<u32>;
#[doc = "Field `PPSINT` writer - PPS0 Output Signal Interval These bits store the interval between the rising edges of PPS0 signal output in terms of units of subsecond increment value You need to program one value less than the required interval For example, if the PTP reference clock is 50 MHz _period of 20ns_, and desired interval between rising edges of PPS0 signal output is 100ns _that is, five units of subsecond increment value_, then you should program value 4 _5 1_ in this register"]
pub type PPSINT_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - PPS0 Output Signal Interval These bits store the interval between the rising edges of PPS0 signal output in terms of units of subsecond increment value You need to program one value less than the required interval For example, if the PTP reference clock is 50 MHz _period of 20ns_, and desired interval between rising edges of PPS0 signal output is 100ns _that is, five units of subsecond increment value_, then you should program value 4 _5 1_ in this register"]
    #[inline(always)]
    pub fn ppsint(&self) -> PPSINT_R {
        PPSINT_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REGISTER472_PPS0INTERVALREGISTER")
            .field("ppsint", &self.ppsint())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - PPS0 Output Signal Interval These bits store the interval between the rising edges of PPS0 signal output in terms of units of subsecond increment value You need to program one value less than the required interval For example, if the PTP reference clock is 50 MHz _period of 20ns_, and desired interval between rising edges of PPS0 signal output is 100ns _that is, five units of subsecond increment value_, then you should program value 4 _5 1_ in this register"]
    #[inline(always)]
    pub fn ppsint(&mut self) -> PPSINT_W<'_, REGISTER472_PPS0INTERVALREGISTER_SPEC> {
        PPSINT_W::new(self, 0)
    }
}
#[doc = "Contains the number of units of subsecond increment value between the rising edges of PPS0 signal output This register is available only when the flexible PPS feature is selected\n\nYou can [`read`](crate::Reg::read) this register and get [`register472_pps0intervalregister::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register472_pps0intervalregister::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REGISTER472_PPS0INTERVALREGISTER_SPEC;
impl crate::RegisterSpec for REGISTER472_PPS0INTERVALREGISTER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`register472_pps0intervalregister::R`](R) reader structure"]
impl crate::Readable for REGISTER472_PPS0INTERVALREGISTER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`register472_pps0intervalregister::W`](W) writer structure"]
impl crate::Writable for REGISTER472_PPS0INTERVALREGISTER_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REGISTER472_PPS0INTERVALREGISTER to value 0"]
impl crate::Resettable for REGISTER472_PPS0INTERVALREGISTER_SPEC {}
