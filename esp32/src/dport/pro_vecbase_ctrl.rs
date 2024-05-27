///Register `PRO_VECBASE_CTRL` reader
pub type R = crate::R<PRO_VECBASE_CTRL_SPEC>;
///Register `PRO_VECBASE_CTRL` writer
pub type W = crate::W<PRO_VECBASE_CTRL_SPEC>;
///Field `PRO_OUT_VECBASE_SEL` reader -
pub type PRO_OUT_VECBASE_SEL_R = crate::FieldReader;
///Field `PRO_OUT_VECBASE_SEL` writer -
pub type PRO_OUT_VECBASE_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:1
    #[inline(always)]
    pub fn pro_out_vecbase_sel(&self) -> PRO_OUT_VECBASE_SEL_R {
        PRO_OUT_VECBASE_SEL_R::new((self.bits & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRO_VECBASE_CTRL")
            .field("pro_out_vecbase_sel", &self.pro_out_vecbase_sel())
            .finish()
    }
}
impl W {
    ///Bits 0:1
    #[inline(always)]
    #[must_use]
    pub fn pro_out_vecbase_sel(&mut self) -> PRO_OUT_VECBASE_SEL_W<PRO_VECBASE_CTRL_SPEC> {
        PRO_OUT_VECBASE_SEL_W::new(self, 0)
    }
}
/**

You can [`read`](crate::generic::Reg::read) this register and get [`pro_vecbase_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_vecbase_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PRO_VECBASE_CTRL_SPEC;
impl crate::RegisterSpec for PRO_VECBASE_CTRL_SPEC {
    type Ux = u32;
}
///`read()` method returns [`pro_vecbase_ctrl::R`](R) reader structure
impl crate::Readable for PRO_VECBASE_CTRL_SPEC {}
///`write(|w| ..)` method takes [`pro_vecbase_ctrl::W`](W) writer structure
impl crate::Writable for PRO_VECBASE_CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets PRO_VECBASE_CTRL to value 0
impl crate::Resettable for PRO_VECBASE_CTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
