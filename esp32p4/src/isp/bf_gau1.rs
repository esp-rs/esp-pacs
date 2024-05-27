///Register `BF_GAU1` reader
pub type R = crate::R<BF_GAU1_SPEC>;
///Register `BF_GAU1` writer
pub type W = crate::W<BF_GAU1_SPEC>;
///Field `GAU_TEMPLATE22` reader - this field configures index 22 of gausian template
pub type GAU_TEMPLATE22_R = crate::FieldReader;
///Field `GAU_TEMPLATE22` writer - this field configures index 22 of gausian template
pub type GAU_TEMPLATE22_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:3 - this field configures index 22 of gausian template
    #[inline(always)]
    pub fn gau_template22(&self) -> GAU_TEMPLATE22_R {
        GAU_TEMPLATE22_R::new((self.bits & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BF_GAU1")
            .field("gau_template22", &self.gau_template22())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - this field configures index 22 of gausian template
    #[inline(always)]
    #[must_use]
    pub fn gau_template22(&mut self) -> GAU_TEMPLATE22_W<BF_GAU1_SPEC> {
        GAU_TEMPLATE22_W::new(self, 0)
    }
}
/**bf gau template register 1

You can [`read`](crate::generic::Reg::read) this register and get [`bf_gau1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bf_gau1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct BF_GAU1_SPEC;
impl crate::RegisterSpec for BF_GAU1_SPEC {
    type Ux = u32;
}
///`read()` method returns [`bf_gau1::R`](R) reader structure
impl crate::Readable for BF_GAU1_SPEC {}
///`write(|w| ..)` method takes [`bf_gau1::W`](W) writer structure
impl crate::Writable for BF_GAU1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets BF_GAU1 to value 0x0f
impl crate::Resettable for BF_GAU1_SPEC {
    const RESET_VALUE: u32 = 0x0f;
}
