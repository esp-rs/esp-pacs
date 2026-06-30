#[doc = "Register `REGISTER449_SUBSECONDINCREMENTREGISTER` reader"]
pub type R = crate::R<REGISTER449_SUBSECONDINCREMENTREGISTER_SPEC>;
#[doc = "Register `REGISTER449_SUBSECONDINCREMENTREGISTER` writer"]
pub type W = crate::W<REGISTER449_SUBSECONDINCREMENTREGISTER_SPEC>;
#[doc = "Field `SSINC` reader - Subsecond Increment Value The value programmed in this field is accumulated every clock cycle _of clk_ptp_i_ with the contents of the subsecond register For example, when PTP clock is 50 MHz _period is 20 ns_, you should program 20 _0x14_ when the System Time Nanoseconds register has an accuracy of 1 ns \\[Bit 9 _TSCTRLSSR_ is set in Register 448 _Timestamp Control Register_\\] When TSCTRLSSR is clear, the Nanoseconds register has a resolution of ~0465ns In this case, you should program a value of 43 _0x2B_ that is derived by 20ns/0465"]
pub type SSINC_R = crate::FieldReader;
#[doc = "Field `SSINC` writer - Subsecond Increment Value The value programmed in this field is accumulated every clock cycle _of clk_ptp_i_ with the contents of the subsecond register For example, when PTP clock is 50 MHz _period is 20 ns_, you should program 20 _0x14_ when the System Time Nanoseconds register has an accuracy of 1 ns \\[Bit 9 _TSCTRLSSR_ is set in Register 448 _Timestamp Control Register_\\] When TSCTRLSSR is clear, the Nanoseconds register has a resolution of ~0465ns In this case, you should program a value of 43 _0x2B_ that is derived by 20ns/0465"]
pub type SSINC_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Subsecond Increment Value The value programmed in this field is accumulated every clock cycle _of clk_ptp_i_ with the contents of the subsecond register For example, when PTP clock is 50 MHz _period is 20 ns_, you should program 20 _0x14_ when the System Time Nanoseconds register has an accuracy of 1 ns \\[Bit 9 _TSCTRLSSR_ is set in Register 448 _Timestamp Control Register_\\] When TSCTRLSSR is clear, the Nanoseconds register has a resolution of ~0465ns In this case, you should program a value of 43 _0x2B_ that is derived by 20ns/0465"]
    #[inline(always)]
    pub fn ssinc(&self) -> SSINC_R {
        SSINC_R::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REGISTER449_SUBSECONDINCREMENTREGISTER")
            .field("ssinc", &self.ssinc())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - Subsecond Increment Value The value programmed in this field is accumulated every clock cycle _of clk_ptp_i_ with the contents of the subsecond register For example, when PTP clock is 50 MHz _period is 20 ns_, you should program 20 _0x14_ when the System Time Nanoseconds register has an accuracy of 1 ns \\[Bit 9 _TSCTRLSSR_ is set in Register 448 _Timestamp Control Register_\\] When TSCTRLSSR is clear, the Nanoseconds register has a resolution of ~0465ns In this case, you should program a value of 43 _0x2B_ that is derived by 20ns/0465"]
    #[inline(always)]
    pub fn ssinc(&mut self) -> SSINC_W<'_, REGISTER449_SUBSECONDINCREMENTREGISTER_SPEC> {
        SSINC_W::new(self, 0)
    }
}
#[doc = "Contains the 8bit value by which the SubSecond register is incremented This register is present only when IEEE1588 timestamping is enabled without an external timestamp input\n\nYou can [`read`](crate::Reg::read) this register and get [`register449_subsecondincrementregister::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register449_subsecondincrementregister::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REGISTER449_SUBSECONDINCREMENTREGISTER_SPEC;
impl crate::RegisterSpec for REGISTER449_SUBSECONDINCREMENTREGISTER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`register449_subsecondincrementregister::R`](R) reader structure"]
impl crate::Readable for REGISTER449_SUBSECONDINCREMENTREGISTER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`register449_subsecondincrementregister::W`](W) writer structure"]
impl crate::Writable for REGISTER449_SUBSECONDINCREMENTREGISTER_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REGISTER449_SUBSECONDINCREMENTREGISTER to value 0"]
impl crate::Resettable for REGISTER449_SUBSECONDINCREMENTREGISTER_SPEC {}
