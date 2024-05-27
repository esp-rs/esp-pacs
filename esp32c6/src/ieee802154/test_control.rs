///Register `TEST_CONTROL` reader
pub type R = crate::R<TEST_CONTROL_SPEC>;
///Register `TEST_CONTROL` writer
pub type W = crate::W<TEST_CONTROL_SPEC>;
///Field `WRONG_CRC` reader -
pub type WRONG_CRC_R = crate::BitReader;
///Field `WRONG_CRC` writer -
pub type WRONG_CRC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0
    #[inline(always)]
    pub fn wrong_crc(&self) -> WRONG_CRC_R {
        WRONG_CRC_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TEST_CONTROL")
            .field("wrong_crc", &self.wrong_crc())
            .finish()
    }
}
impl W {
    ///Bit 0
    #[inline(always)]
    #[must_use]
    pub fn wrong_crc(&mut self) -> WRONG_CRC_W<TEST_CONTROL_SPEC> {
        WRONG_CRC_W::new(self, 0)
    }
}
/**

You can [`read`](crate::generic::Reg::read) this register and get [`test_control::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`test_control::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct TEST_CONTROL_SPEC;
impl crate::RegisterSpec for TEST_CONTROL_SPEC {
    type Ux = u32;
}
///`read()` method returns [`test_control::R`](R) reader structure
impl crate::Readable for TEST_CONTROL_SPEC {}
///`write(|w| ..)` method takes [`test_control::W`](W) writer structure
impl crate::Writable for TEST_CONTROL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets TEST_CONTROL to value 0
impl crate::Resettable for TEST_CONTROL_SPEC {
    const RESET_VALUE: u32 = 0;
}
