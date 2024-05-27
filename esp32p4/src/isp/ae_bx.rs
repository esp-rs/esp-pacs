///Register `AE_BX` reader
pub type R = crate::R<AE_BX_SPEC>;
///Register `AE_BX` writer
pub type W = crate::W<AE_BX_SPEC>;
///Field `AE_X_BSIZE` reader - this field configures every block x size
pub type AE_X_BSIZE_R = crate::FieldReader<u16>;
///Field `AE_X_BSIZE` writer - this field configures every block x size
pub type AE_X_BSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
///Field `AE_X_START` reader - this field configures first block start x address
pub type AE_X_START_R = crate::FieldReader<u16>;
///Field `AE_X_START` writer - this field configures first block start x address
pub type AE_X_START_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    ///Bits 0:10 - this field configures every block x size
    #[inline(always)]
    pub fn ae_x_bsize(&self) -> AE_X_BSIZE_R {
        AE_X_BSIZE_R::new((self.bits & 0x07ff) as u16)
    }
    ///Bits 11:21 - this field configures first block start x address
    #[inline(always)]
    pub fn ae_x_start(&self) -> AE_X_START_R {
        AE_X_START_R::new(((self.bits >> 11) & 0x07ff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AE_BX")
            .field("ae_x_bsize", &self.ae_x_bsize())
            .field("ae_x_start", &self.ae_x_start())
            .finish()
    }
}
impl W {
    ///Bits 0:10 - this field configures every block x size
    #[inline(always)]
    #[must_use]
    pub fn ae_x_bsize(&mut self) -> AE_X_BSIZE_W<AE_BX_SPEC> {
        AE_X_BSIZE_W::new(self, 0)
    }
    ///Bits 11:21 - this field configures first block start x address
    #[inline(always)]
    #[must_use]
    pub fn ae_x_start(&mut self) -> AE_X_START_W<AE_BX_SPEC> {
        AE_X_START_W::new(self, 11)
    }
}
/**ae window register in x-direction

You can [`read`](crate::generic::Reg::read) this register and get [`ae_bx::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ae_bx::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct AE_BX_SPEC;
impl crate::RegisterSpec for AE_BX_SPEC {
    type Ux = u32;
}
///`read()` method returns [`ae_bx::R`](R) reader structure
impl crate::Readable for AE_BX_SPEC {}
///`write(|w| ..)` method takes [`ae_bx::W`](W) writer structure
impl crate::Writable for AE_BX_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets AE_BX to value 0x0180
impl crate::Resettable for AE_BX_SPEC {
    const RESET_VALUE: u32 = 0x0180;
}
