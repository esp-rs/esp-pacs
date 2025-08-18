#[doc = "Register `CONF3` reader"]
pub type R = crate::R<CONF3_SPEC>;
#[doc = "Register `CONF3` writer"]
pub type W = crate::W<CONF3_SPEC>;
#[doc = "Field `BLOCK_LENGTH_12LINE` reader - The number of bytes contained in a block 12line"]
pub type BLOCK_LENGTH_12LINE_R = crate::FieldReader<u16>;
#[doc = "Field `BLOCK_LENGTH_12LINE` writer - The number of bytes contained in a block 12line"]
pub type BLOCK_LENGTH_12LINE_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `BLOCK_LENGTH_4LINE` reader - The number of bytes contained in a block 4line"]
pub type BLOCK_LENGTH_4LINE_R = crate::FieldReader<u16>;
#[doc = "Field `BLOCK_LENGTH_4LINE` writer - The number of bytes contained in a block 4line"]
pub type BLOCK_LENGTH_4LINE_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 0:13 - The number of bytes contained in a block 12line"]
    #[inline(always)]
    pub fn block_length_12line(&self) -> BLOCK_LENGTH_12LINE_R {
        BLOCK_LENGTH_12LINE_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bits 14:27 - The number of bytes contained in a block 4line"]
    #[inline(always)]
    pub fn block_length_4line(&self) -> BLOCK_LENGTH_4LINE_R {
        BLOCK_LENGTH_4LINE_R::new(((self.bits >> 14) & 0x3fff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONF3")
            .field("block_length_12line", &self.block_length_12line())
            .field("block_length_4line", &self.block_length_4line())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:13 - The number of bytes contained in a block 12line"]
    #[inline(always)]
    pub fn block_length_12line(&mut self) -> BLOCK_LENGTH_12LINE_W<'_, CONF3_SPEC> {
        BLOCK_LENGTH_12LINE_W::new(self, 0)
    }
    #[doc = "Bits 14:27 - The number of bytes contained in a block 4line"]
    #[inline(always)]
    pub fn block_length_4line(&mut self) -> BLOCK_LENGTH_4LINE_W<'_, CONF3_SPEC> {
        BLOCK_LENGTH_4LINE_W::new(self, 14)
    }
}
#[doc = "RX CH5 config3 register\n\nYou can [`read`](crate::Reg::read) this register and get [`conf3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`conf3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CONF3_SPEC;
impl crate::RegisterSpec for CONF3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`conf3::R`](R) reader structure"]
impl crate::Readable for CONF3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`conf3::W`](W) writer structure"]
impl crate::Writable for CONF3_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CONF3 to value 0x0020_0100"]
impl crate::Resettable for CONF3_SPEC {
    const RESET_VALUE: u32 = 0x0020_0100;
}
