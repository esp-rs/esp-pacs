#[doc = "Register `REGISTER473_PPS0WIDTHREGISTER` reader"]
pub type R = crate::R<REGISTER473_PPS0WIDTHREGISTER_SPEC>;
#[doc = "Register `REGISTER473_PPS0WIDTHREGISTER` writer"]
pub type W = crate::W<REGISTER473_PPS0WIDTHREGISTER_SPEC>;
#[doc = "Field `PPSWIDTH` reader - PPS0 Output Signal Width These bits store the width between the rising edge and corresponding falling edge of the PPS0 signal output in terms of units of subsecond increment value You need to program one value less than the required interval For example, if PTP reference clock is 50 MHz _period of 20ns_, and desired width between the rising and corresponding falling edges of PPS0 signal output is 80ns _that is, four units of subsecond increment value_, then you should program value 3 _4 1_ in this register Note: The value programmed in this register must be lesser than the value programmed in Register 472 _PPS0 Interval Register_"]
pub type PPSWIDTH_R = crate::FieldReader<u32>;
#[doc = "Field `PPSWIDTH` writer - PPS0 Output Signal Width These bits store the width between the rising edge and corresponding falling edge of the PPS0 signal output in terms of units of subsecond increment value You need to program one value less than the required interval For example, if PTP reference clock is 50 MHz _period of 20ns_, and desired width between the rising and corresponding falling edges of PPS0 signal output is 80ns _that is, four units of subsecond increment value_, then you should program value 3 _4 1_ in this register Note: The value programmed in this register must be lesser than the value programmed in Register 472 _PPS0 Interval Register_"]
pub type PPSWIDTH_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - PPS0 Output Signal Width These bits store the width between the rising edge and corresponding falling edge of the PPS0 signal output in terms of units of subsecond increment value You need to program one value less than the required interval For example, if PTP reference clock is 50 MHz _period of 20ns_, and desired width between the rising and corresponding falling edges of PPS0 signal output is 80ns _that is, four units of subsecond increment value_, then you should program value 3 _4 1_ in this register Note: The value programmed in this register must be lesser than the value programmed in Register 472 _PPS0 Interval Register_"]
    #[inline(always)]
    pub fn ppswidth(&self) -> PPSWIDTH_R {
        PPSWIDTH_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REGISTER473_PPS0WIDTHREGISTER")
            .field("ppswidth", &self.ppswidth())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - PPS0 Output Signal Width These bits store the width between the rising edge and corresponding falling edge of the PPS0 signal output in terms of units of subsecond increment value You need to program one value less than the required interval For example, if PTP reference clock is 50 MHz _period of 20ns_, and desired width between the rising and corresponding falling edges of PPS0 signal output is 80ns _that is, four units of subsecond increment value_, then you should program value 3 _4 1_ in this register Note: The value programmed in this register must be lesser than the value programmed in Register 472 _PPS0 Interval Register_"]
    #[inline(always)]
    pub fn ppswidth(&mut self) -> PPSWIDTH_W<'_, REGISTER473_PPS0WIDTHREGISTER_SPEC> {
        PPSWIDTH_W::new(self, 0)
    }
}
#[doc = "Contains the number of units of subsecond increment value between the rising and corresponding falling edges of PPS0 signal output This register is available only when the flexible PPS feature is selected\n\nYou can [`read`](crate::Reg::read) this register and get [`register473_pps0widthregister::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register473_pps0widthregister::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REGISTER473_PPS0WIDTHREGISTER_SPEC;
impl crate::RegisterSpec for REGISTER473_PPS0WIDTHREGISTER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`register473_pps0widthregister::R`](R) reader structure"]
impl crate::Readable for REGISTER473_PPS0WIDTHREGISTER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`register473_pps0widthregister::W`](W) writer structure"]
impl crate::Writable for REGISTER473_PPS0WIDTHREGISTER_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REGISTER473_PPS0WIDTHREGISTER to value 0"]
impl crate::Resettable for REGISTER473_PPS0WIDTHREGISTER_SPEC {}
