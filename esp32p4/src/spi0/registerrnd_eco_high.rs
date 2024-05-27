///Register `REGISTERRND_ECO_HIGH` reader
pub type R = crate::R<REGISTERRND_ECO_HIGH_SPEC>;
///Register `REGISTERRND_ECO_HIGH` writer
pub type W = crate::W<REGISTERRND_ECO_HIGH_SPEC>;
///Field `REGISTERRND_ECO_HIGH` reader - ECO high register
pub type REGISTERRND_ECO_HIGH_R = crate::FieldReader<u32>;
///Field `REGISTERRND_ECO_HIGH` writer - ECO high register
pub type REGISTERRND_ECO_HIGH_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - ECO high register
    #[inline(always)]
    pub fn registerrnd_eco_high(&self) -> REGISTERRND_ECO_HIGH_R {
        REGISTERRND_ECO_HIGH_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REGISTERRND_ECO_HIGH")
            .field("registerrnd_eco_high", &self.registerrnd_eco_high())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - ECO high register
    #[inline(always)]
    #[must_use]
    pub fn registerrnd_eco_high(&mut self) -> REGISTERRND_ECO_HIGH_W<REGISTERRND_ECO_HIGH_SPEC> {
        REGISTERRND_ECO_HIGH_W::new(self, 0)
    }
}
/**MSPI ECO high register

You can [`read`](crate::generic::Reg::read) this register and get [`registerrnd_eco_high::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`registerrnd_eco_high::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct REGISTERRND_ECO_HIGH_SPEC;
impl crate::RegisterSpec for REGISTERRND_ECO_HIGH_SPEC {
    type Ux = u32;
}
///`read()` method returns [`registerrnd_eco_high::R`](R) reader structure
impl crate::Readable for REGISTERRND_ECO_HIGH_SPEC {}
///`write(|w| ..)` method takes [`registerrnd_eco_high::W`](W) writer structure
impl crate::Writable for REGISTERRND_ECO_HIGH_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets REGISTERRND_ECO_HIGH to value 0x037c
impl crate::Resettable for REGISTERRND_ECO_HIGH_SPEC {
    const RESET_VALUE: u32 = 0x037c;
}
