///Register `SDA_DUTY` reader
pub type R = crate::R<SDA_DUTY_SPEC>;
///Register `SDA_DUTY` writer
pub type W = crate::W<SDA_DUTY_SPEC>;
///Field `SDA_DUTY` reader - Number of FAST_CLK cycles SDA will switch after falling edge of SCL
pub type SDA_DUTY_R = crate::FieldReader<u32>;
///Field `SDA_DUTY` writer - Number of FAST_CLK cycles SDA will switch after falling edge of SCL
pub type SDA_DUTY_W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    ///Bits 0:19 - Number of FAST_CLK cycles SDA will switch after falling edge of SCL
    #[inline(always)]
    pub fn sda_duty(&self) -> SDA_DUTY_R {
        SDA_DUTY_R::new(self.bits & 0x000f_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SDA_DUTY")
            .field("sda_duty", &self.sda_duty())
            .finish()
    }
}
impl W {
    ///Bits 0:19 - Number of FAST_CLK cycles SDA will switch after falling edge of SCL
    #[inline(always)]
    #[must_use]
    pub fn sda_duty(&mut self) -> SDA_DUTY_W<SDA_DUTY_SPEC> {
        SDA_DUTY_W::new(self, 0)
    }
}
/**

You can [`read`](crate::generic::Reg::read) this register and get [`sda_duty::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sda_duty::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SDA_DUTY_SPEC;
impl crate::RegisterSpec for SDA_DUTY_SPEC {
    type Ux = u32;
}
///`read()` method returns [`sda_duty::R`](R) reader structure
impl crate::Readable for SDA_DUTY_SPEC {}
///`write(|w| ..)` method takes [`sda_duty::W`](W) writer structure
impl crate::Writable for SDA_DUTY_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SDA_DUTY to value 0
impl crate::Resettable for SDA_DUTY_SPEC {
    const RESET_VALUE: u32 = 0;
}
