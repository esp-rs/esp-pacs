///Register `BF_GAU0` reader
pub type R = crate::R<BF_GAU0_SPEC>;
///Register `BF_GAU0` writer
pub type W = crate::W<BF_GAU0_SPEC>;
///Field `GAU_TEMPLATE21` reader - this field configures index 21 of gausian template
pub type GAU_TEMPLATE21_R = crate::FieldReader;
///Field `GAU_TEMPLATE21` writer - this field configures index 21 of gausian template
pub type GAU_TEMPLATE21_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `GAU_TEMPLATE20` reader - this field configures index 20 of gausian template
pub type GAU_TEMPLATE20_R = crate::FieldReader;
///Field `GAU_TEMPLATE20` writer - this field configures index 20 of gausian template
pub type GAU_TEMPLATE20_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `GAU_TEMPLATE12` reader - this field configures index 12 of gausian template
pub type GAU_TEMPLATE12_R = crate::FieldReader;
///Field `GAU_TEMPLATE12` writer - this field configures index 12 of gausian template
pub type GAU_TEMPLATE12_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `GAU_TEMPLATE11` reader - this field configures index 11 of gausian template
pub type GAU_TEMPLATE11_R = crate::FieldReader;
///Field `GAU_TEMPLATE11` writer - this field configures index 11 of gausian template
pub type GAU_TEMPLATE11_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `GAU_TEMPLATE10` reader - this field configures index 10 of gausian template
pub type GAU_TEMPLATE10_R = crate::FieldReader;
///Field `GAU_TEMPLATE10` writer - this field configures index 10 of gausian template
pub type GAU_TEMPLATE10_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `GAU_TEMPLATE02` reader - this field configures index 02 of gausian template
pub type GAU_TEMPLATE02_R = crate::FieldReader;
///Field `GAU_TEMPLATE02` writer - this field configures index 02 of gausian template
pub type GAU_TEMPLATE02_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `GAU_TEMPLATE01` reader - this field configures index 01 of gausian template
pub type GAU_TEMPLATE01_R = crate::FieldReader;
///Field `GAU_TEMPLATE01` writer - this field configures index 01 of gausian template
pub type GAU_TEMPLATE01_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `GAU_TEMPLATE00` reader - this field configures index 00 of gausian template
pub type GAU_TEMPLATE00_R = crate::FieldReader;
///Field `GAU_TEMPLATE00` writer - this field configures index 00 of gausian template
pub type GAU_TEMPLATE00_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:3 - this field configures index 21 of gausian template
    #[inline(always)]
    pub fn gau_template21(&self) -> GAU_TEMPLATE21_R {
        GAU_TEMPLATE21_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - this field configures index 20 of gausian template
    #[inline(always)]
    pub fn gau_template20(&self) -> GAU_TEMPLATE20_R {
        GAU_TEMPLATE20_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:11 - this field configures index 12 of gausian template
    #[inline(always)]
    pub fn gau_template12(&self) -> GAU_TEMPLATE12_R {
        GAU_TEMPLATE12_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:15 - this field configures index 11 of gausian template
    #[inline(always)]
    pub fn gau_template11(&self) -> GAU_TEMPLATE11_R {
        GAU_TEMPLATE11_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    ///Bits 16:19 - this field configures index 10 of gausian template
    #[inline(always)]
    pub fn gau_template10(&self) -> GAU_TEMPLATE10_R {
        GAU_TEMPLATE10_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bits 20:23 - this field configures index 02 of gausian template
    #[inline(always)]
    pub fn gau_template02(&self) -> GAU_TEMPLATE02_R {
        GAU_TEMPLATE02_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    ///Bits 24:27 - this field configures index 01 of gausian template
    #[inline(always)]
    pub fn gau_template01(&self) -> GAU_TEMPLATE01_R {
        GAU_TEMPLATE01_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    ///Bits 28:31 - this field configures index 00 of gausian template
    #[inline(always)]
    pub fn gau_template00(&self) -> GAU_TEMPLATE00_R {
        GAU_TEMPLATE00_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BF_GAU0")
            .field("gau_template21", &self.gau_template21())
            .field("gau_template20", &self.gau_template20())
            .field("gau_template12", &self.gau_template12())
            .field("gau_template11", &self.gau_template11())
            .field("gau_template10", &self.gau_template10())
            .field("gau_template02", &self.gau_template02())
            .field("gau_template01", &self.gau_template01())
            .field("gau_template00", &self.gau_template00())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - this field configures index 21 of gausian template
    #[inline(always)]
    #[must_use]
    pub fn gau_template21(&mut self) -> GAU_TEMPLATE21_W<BF_GAU0_SPEC> {
        GAU_TEMPLATE21_W::new(self, 0)
    }
    ///Bits 4:7 - this field configures index 20 of gausian template
    #[inline(always)]
    #[must_use]
    pub fn gau_template20(&mut self) -> GAU_TEMPLATE20_W<BF_GAU0_SPEC> {
        GAU_TEMPLATE20_W::new(self, 4)
    }
    ///Bits 8:11 - this field configures index 12 of gausian template
    #[inline(always)]
    #[must_use]
    pub fn gau_template12(&mut self) -> GAU_TEMPLATE12_W<BF_GAU0_SPEC> {
        GAU_TEMPLATE12_W::new(self, 8)
    }
    ///Bits 12:15 - this field configures index 11 of gausian template
    #[inline(always)]
    #[must_use]
    pub fn gau_template11(&mut self) -> GAU_TEMPLATE11_W<BF_GAU0_SPEC> {
        GAU_TEMPLATE11_W::new(self, 12)
    }
    ///Bits 16:19 - this field configures index 10 of gausian template
    #[inline(always)]
    #[must_use]
    pub fn gau_template10(&mut self) -> GAU_TEMPLATE10_W<BF_GAU0_SPEC> {
        GAU_TEMPLATE10_W::new(self, 16)
    }
    ///Bits 20:23 - this field configures index 02 of gausian template
    #[inline(always)]
    #[must_use]
    pub fn gau_template02(&mut self) -> GAU_TEMPLATE02_W<BF_GAU0_SPEC> {
        GAU_TEMPLATE02_W::new(self, 20)
    }
    ///Bits 24:27 - this field configures index 01 of gausian template
    #[inline(always)]
    #[must_use]
    pub fn gau_template01(&mut self) -> GAU_TEMPLATE01_W<BF_GAU0_SPEC> {
        GAU_TEMPLATE01_W::new(self, 24)
    }
    ///Bits 28:31 - this field configures index 00 of gausian template
    #[inline(always)]
    #[must_use]
    pub fn gau_template00(&mut self) -> GAU_TEMPLATE00_W<BF_GAU0_SPEC> {
        GAU_TEMPLATE00_W::new(self, 28)
    }
}
/**bf gau template register 0

You can [`read`](crate::generic::Reg::read) this register and get [`bf_gau0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bf_gau0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct BF_GAU0_SPEC;
impl crate::RegisterSpec for BF_GAU0_SPEC {
    type Ux = u32;
}
///`read()` method returns [`bf_gau0::R`](R) reader structure
impl crate::Readable for BF_GAU0_SPEC {}
///`write(|w| ..)` method takes [`bf_gau0::W`](W) writer structure
impl crate::Writable for BF_GAU0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets BF_GAU0 to value 0xffff_ffff
impl crate::Resettable for BF_GAU0_SPEC {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
