#[doc = "Register `CONF2` reader"]
pub type R = crate::R<CONF2_SPEC>;
#[doc = "Register `CONF2` writer"]
pub type W = crate::W<CONF2_SPEC>;
#[doc = "Field `BLOCK_ROW_LENGTH_12LINE` reader - The number of bytes contained in a row block 12line in RX channel 5"]
pub type BLOCK_ROW_LENGTH_12LINE_R = crate::FieldReader<u16>;
#[doc = "Field `BLOCK_ROW_LENGTH_12LINE` writer - The number of bytes contained in a row block 12line in RX channel 5"]
pub type BLOCK_ROW_LENGTH_12LINE_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `BLOCK_ROW_LENGTH_4LINE` reader - The number of bytes contained in a row block 4line in RX channel 5"]
pub type BLOCK_ROW_LENGTH_4LINE_R = crate::FieldReader<u16>;
#[doc = "Field `BLOCK_ROW_LENGTH_4LINE` writer - The number of bytes contained in a row block 4line in RX channel 5"]
pub type BLOCK_ROW_LENGTH_4LINE_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - The number of bytes contained in a row block 12line in RX channel 5"]
    #[inline(always)]
    pub fn block_row_length_12line(&self) -> BLOCK_ROW_LENGTH_12LINE_R {
        BLOCK_ROW_LENGTH_12LINE_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - The number of bytes contained in a row block 4line in RX channel 5"]
    #[inline(always)]
    pub fn block_row_length_4line(&self) -> BLOCK_ROW_LENGTH_4LINE_R {
        BLOCK_ROW_LENGTH_4LINE_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONF2")
            .field("block_row_length_12line", &self.block_row_length_12line())
            .field("block_row_length_4line", &self.block_row_length_4line())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - The number of bytes contained in a row block 12line in RX channel 5"]
    #[inline(always)]
    pub fn block_row_length_12line(&mut self) -> BLOCK_ROW_LENGTH_12LINE_W<CONF2_SPEC> {
        BLOCK_ROW_LENGTH_12LINE_W::new(self, 0)
    }
    #[doc = "Bits 16:31 - The number of bytes contained in a row block 4line in RX channel 5"]
    #[inline(always)]
    pub fn block_row_length_4line(&mut self) -> BLOCK_ROW_LENGTH_4LINE_W<CONF2_SPEC> {
        BLOCK_ROW_LENGTH_4LINE_W::new(self, 16)
    }
}
#[doc = "RX CH5 config2 register\n\nYou can [`read`](crate::Reg::read) this register and get [`conf2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`conf2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CONF2_SPEC;
impl crate::RegisterSpec for CONF2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`conf2::R`](R) reader structure"]
impl crate::Readable for CONF2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`conf2::W`](W) writer structure"]
impl crate::Writable for CONF2_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CONF2 to value 0x3c00_7800"]
impl crate::Resettable for CONF2_SPEC {
    const RESET_VALUE: u32 = 0x3c00_7800;
}
